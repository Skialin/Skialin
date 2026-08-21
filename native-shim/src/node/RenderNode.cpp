#include "include/utils/SkNWayCanvas.h"
#include "include/utils/SkNoDrawCanvas.h"
#include "include/core/SkPathBuilder.h"
#include "include/utils/SkPaintFilterCanvas.h"
#include "include/core/SkPicture.h"
#include "include/utils/SkShadowUtils.h"
#include "skialin/bridge.h"
#include "skialin/node/RenderNode.h"
#include "skialin/node/RenderNodeContext.h"

namespace skialin {
namespace node {

static void drawDrawableCompat(SkCanvas* canvas, SkCanvas* deviceCanvas, SkDrawable* drawable) {
    if (deviceCanvas->recorder() != nullptr) {
        drawable->draw(canvas);
    } else {
        canvas->drawDrawable(drawable);
    }
}

static void drawDrawableCompat(SkCanvas* canvas, SkDrawable* drawable) {
    drawDrawableCompat(canvas, canvas, drawable);
}

// The goal with selecting the size of the rectangle here is to avoid limiting the
// drawable area as much as possible.
// Due to https://issuetracker.google.com/issues/324465764 we have to
// leave room for scale between the values we specify here and Float.MAX_VALUE.
// The maximum possible scale that can be applied to the canvas will be
// Float.MAX_VALUE divided by the largest value below.
// 2^30 was chosen because it's big enough, leaves quite a lot of room between it
// and Float.MAX_VALUE, and also lets the width and height fit into int32 (just in
// case).
static const float UNKNOWN_BOUNDS_MIN_VALUE = static_cast<float>(-(1L << 30));
static const float UNKNOWN_BOUNDS_MAX_VALUE = static_cast<float>((1L << 30) - 1);
static SkRect UNKNOWN_BOUNDS {
    UNKNOWN_BOUNDS_MIN_VALUE,
    UNKNOWN_BOUNDS_MIN_VALUE,
    UNKNOWN_BOUNDS_MAX_VALUE,
    UNKNOWN_BOUNDS_MAX_VALUE
};

static const float DEFAULT_CAMERA_DISTANCE = 8.0f;
static const float NON_ZERO_EPSILON = 0.001f;

// Since "kSkDrawable_Type" isn't really used anywhere outside of serialization, we can use it
// to identify RenderNode objects without RTTI, like SkRuntimeEffect does (even without adding to original enum).
// The value has to stay outside the range Skia assigns, or a node is mistaken for a
// flattenable of that type. kSkShader_Type is the largest Skia declares.
static const SkFlattenable::Type kRenderNode_Type = static_cast<SkFlattenable::Type>(0x2d2595b7);

/**
 * Check for floats that are close enough to zero.
 */
inline static bool isZero(float value) {
    return fabsf(value) <= NON_ZERO_EPSILON;
}

static SkColor multiplyAlpha(SkColor color, float alpha) {
    return SkColorSetA(color, alpha * SkColorGetA(color));
}

class UnrollDrawableCanvas : public SkNWayCanvas {
public:
    UnrollDrawableCanvas(SkCanvas* canvas)
        : SkNWayCanvas(canvas->imageInfo().width(), canvas->imageInfo().height()) {
        this->addCanvas(canvas);
    }

protected:
    void onDrawDrawable(SkDrawable* drawable, const SkMatrix* matrix) override {
        drawable->draw(this, matrix);
    }
};

class AlphaModulateFilterCanvas : public SkPaintFilterCanvas {
public:
    AlphaModulateFilterCanvas(SkCanvas* canvas, float alpha)
        : SkPaintFilterCanvas(canvas), alpha(alpha) {}

protected:
    bool onFilter(SkPaint& paint) const override {
        paint.setAlphaf(paint.getAlphaf() * alpha);
        return true;
    }

    void onDrawDrawable(SkDrawable* drawable, const SkMatrix* matrix) override {
        drawable->draw(this, matrix);
    }

private:
    float alpha;
};

// Collects the render nodes drawn by a recording, taking a reference to each.
// The set is expected to be empty on entry.
class DependencyTrackerCanvas : public SkNoDrawCanvas {
public:
    DependencyTrackerCanvas(std::unordered_set<RenderNode *>& dependencies)
        : SkNoDrawCanvas(INT_MAX, INT_MAX), dependencies(dependencies) {}

protected:
    void onDrawDrawable(SkDrawable* drawable, const SkMatrix* matrix) override {
        if (drawable->getFlattenableType() == kRenderNode_Type) {
            auto renderNode = static_cast<RenderNode *>(drawable);
            if (dependencies.insert(renderNode).second) {
                renderNode->ref();
            }
        } else {
            drawable->draw(this, matrix);
        }
    }

private:
    std::unordered_set<RenderNode *>& dependencies;
};

RenderNode::RenderNode(const sk_sp<RenderNodeContext>& context)
    : context(context),
      bbhFactory(context->shouldMeasureDrawBounds() ? new SkRTreeFactory() : nullptr),
      recorder(),
      contentCache(),
      contentSnapshot(),
      contentTransformInvariant(true),
      observersNotified(false),
      dependencies(),
      observers(),
      layerPaint(),
      bounds { 0.0f, 0.0f, 0.0f, 0.0f },
      pivot { SK_FloatNaN, SK_FloatNaN },
      alpha(1.0f),
      scaleX(1.0f),
      scaleY(1.0f),
      translationX(0.0f),
      translationY(0.0f),
      shadowElevation(0.0f),
      ambientShadowColor(SK_ColorBLACK),
      spotShadowColor(SK_ColorBLACK),
      rotationX(0.0f),
      rotationY(0.0f),
      rotationZ(0.0f),
      clipRect(),
      clipRRect(),
      clipPath(),
      clipOp(SkClipOp::kIntersect),
      clipAntiAlias(false),
      clip(false),
      transformMatrix(),
      transformCamera(),
      matrixIdentity(true),
      matrixDirty(false) {
    this->setCameraLocation(0.0f, 0.0f, DEFAULT_CAMERA_DISTANCE);
}

RenderNode::~RenderNode() {
    if (this->bbhFactory) {
        delete this->bbhFactory;
        this->bbhFactory = nullptr;
    }
    // The observers set is necessarily empty: an observer holds a reference to this
    // node through its own dependencies, so it cannot outlive it.
    this->releaseDependencies();
}

void RenderNode::setLayerPaint(const std::optional<SkPaint>& layerPaint) {
    this->layerPaint = layerPaint;
    this->invalidateSnapshot(kAppearance);
}

void RenderNode::setBounds(const SkRect& bounds) {
    this->bounds = bounds;
    this->matrixDirty = true;
    // The bounds size is the cull rect of a clipping node's recording.
    this->invalidateSnapshot(kContent);
}

void RenderNode::setPivot(const SkPoint& pivot) {
    this->pivot = pivot;
    this->matrixDirty = true;
    this->invalidateSnapshot(kAppearance);
}

void RenderNode::setAlpha(float alpha) {
    this->alpha = alpha;
    this->invalidateSnapshot(kAppearance);
}

void RenderNode::setScaleX(float scaleX) {
    this->scaleX = scaleX;
    this->matrixDirty = true;
    this->invalidateSnapshot(kAppearance);
}

void RenderNode::setScaleY(float scaleY) {
    this->scaleY = scaleY;
    this->matrixDirty = true;
    this->invalidateSnapshot(kAppearance);
}

void RenderNode::setTranslationX(float translationX) {
    this->translationX = translationX;
    this->matrixDirty = true;
    this->invalidateSnapshot(kAppearance);
}

void RenderNode::setTranslationY(float translationY) {
    this->translationY = translationY;
    this->matrixDirty = true;
    this->invalidateSnapshot(kAppearance);
}

void RenderNode::setShadowElevation(float shadowElevation) {
    bool wasPreventingObserverSnapshot = this->preventsObserverSnapshot();
    this->shadowElevation = shadowElevation;
    // A shadow makes this node's drawing transform-dependent, which its observers read
    // when deciding whether they may snapshot, so a change to it has to reach them even
    // if they were already told stale this frame.
    if (this->preventsObserverSnapshot() != wasPreventingObserverSnapshot) {
        this->observersNotified = false;
    }
    // A shadow also lets the node draw outside its bounds, widening the cull rect.
    this->invalidateSnapshot(kContent);
}

void RenderNode::setAmbientShadowColor(SkColor ambientShadowColor) {
    this->ambientShadowColor = ambientShadowColor;
    this->invalidateSnapshot(kAppearance);
}

void RenderNode::setSpotShadowColor(SkColor spotShadowColor) {
    this->spotShadowColor = spotShadowColor;
    this->invalidateSnapshot(kAppearance);
}

void RenderNode::setRotationX(float rotationX) {
    this->rotationX = rotationX;
    this->matrixDirty = true;
    this->invalidateSnapshot(kAppearance);
}

void RenderNode::setRotationY(float rotationY) {
    this->rotationY = rotationY;
    this->matrixDirty = true;
    this->invalidateSnapshot(kAppearance);
}

void RenderNode::setRotationZ(float rotationZ) {
    this->rotationZ = rotationZ;
    this->matrixDirty = true;
    this->invalidateSnapshot(kAppearance);
}

float RenderNode::getCameraDistance() const {
    return this->transformCamera.fLocation.z / 72.0f;
}

void RenderNode::setCameraDistance(float cameraDistance) {
    this->setCameraLocation(0.0f, 0.0f, cameraDistance);
    this->matrixDirty = true;
    this->invalidateSnapshot(kAppearance);
}

void RenderNode::setClipRect(const std::optional<SkRect>& clipRect, SkClipOp op, bool doAntiAlias) {
    this->clipRect = clipRect;
    this->clipRRect.reset();
    this->clipPath.reset();
    this->clipOp = op;
    this->clipAntiAlias = doAntiAlias;
    this->invalidateSnapshot(kAppearance);
}

void RenderNode::setClipRRect(const std::optional<SkRRect>& clipRRect, SkClipOp op, bool doAntiAlias) {
    this->clipRect.reset();
    this->clipRRect = clipRRect;
    this->clipPath.reset();
    this->clipOp = op;
    this->clipAntiAlias = doAntiAlias;
    this->invalidateSnapshot(kAppearance);
}

void RenderNode::setClipPath(const std::optional<SkPath>& clipPath, SkClipOp op, bool doAntiAlias) {
    this->clipRect.reset();
    this->clipRRect.reset();
    this->clipPath = clipPath;
    this->clipOp = op;
    this->clipAntiAlias = doAntiAlias;
    this->invalidateSnapshot(kAppearance);
}

void RenderNode::setClip(bool clip) {
    this->clip = clip;
    // Clipping bounds the recording's cull rect to the node's own bounds.
    this->invalidateSnapshot(kContent);
}

const SkMatrix& RenderNode::getMatrix() {
    this->updateMatrix();
    return this->transformMatrix;
}

bool RenderNode::mayDrawOutOfBounds() const {
    return !this->clip || this->shadowElevation > 0.0f;
}

// The content cache and the snapshot recorded from it have to agree on the cull rect
// and the bounding hierarchy, or the two cull differently and report different bounds.
// They cannot share a recorder: the snapshot is taken while the member one may be
// mid-recording.
SkCanvas *RenderNode::beginRecordingInto(SkPictureRecorder& target) {
    bool measureDrawBounds = this->mayDrawOutOfBounds();
    const SkRect& bounds = measureDrawBounds ? UNKNOWN_BOUNDS : SkRect::MakeWH(this->bounds.width(), this->bounds.height());
    SkBBHFactory* bbhFactory = measureDrawBounds ? this->bbhFactory : nullptr;
    return target.beginRecording(bounds, bbhFactory);
}

SkCanvas *RenderNode::beginRecording() {
    return this->beginRecordingInto(this->recorder);
}

void RenderNode::endRecording() {
    this->contentCache = this->recorder.finishRecordingAsDrawable();
    this->updateDependencies();
    this->invalidateSnapshot(kContent | kDependencies);
}

void RenderNode::drawInto(SkCanvas* canvas) {
    drawDrawableCompat(canvas, this);
}

SkFlattenable::Type RenderNode::getFlattenableType() const {
    return kRenderNode_Type;
}

SkRect RenderNode::onGetBounds() {
    this->updateMatrix();
    SkRect result;
    if (this->contentCache) {
        result = this->contentCache->getBounds().makeOffset(this->bounds.left(), this->bounds.top());
    } else {
        result = this->mayDrawOutOfBounds() ? UNKNOWN_BOUNDS : this->bounds;
    }
    if (!this->matrixIdentity) {
        this->transformMatrix.mapRect(&result);
    }
    return result;
}

size_t RenderNode::onApproximateBytesUsed() {
    size_t contentSize = 0;
    if (this->contentCache) {
        contentSize += this->contentCache->approximateBytesUsed();
    }
    if (this->contentSnapshot) {
        // A snapshot references the snapshots nested in it rather than copying them, and
        // SkPicture already counts those, so the figure reported here covers the subtree
        // and must not be summed with the figures its dependencies report.
        contentSize += this->contentSnapshot->approximateBytesUsed();
    }
    if (this->clipPath) {
        contentSize += this->clipPath->approximateBytesUsed();
    }
    return sizeof(*this) + contentSize;
}

void RenderNode::onDraw(SkCanvas* canvas) {
    this->updateMatrix();
    // Whatever records this draw inlines the node afresh, so its next change must be
    // relayed to the observers again.
    this->observersNotified = false;

    canvas->translate(this->bounds.left(), this->bounds.top());
    if (!this->matrixIdentity) {
        canvas->concat(this->transformMatrix);
    }

    if (this->shadowElevation > 0) {
        auto lightGeometry = this->context->getLightGeometry();
        auto lightInfo = this->context->getLightInfo();
        this->drawShadow(canvas, lightGeometry, lightInfo);
    }

    if (this->clip) {
        canvas->save();
        if (this->clipRect) {
            canvas->clipRect(*this->clipRect, this->clipOp, this->clipAntiAlias);
        } else if (this->clipRRect) {
            canvas->clipRRect(*this->clipRRect, this->clipOp, this->clipAntiAlias);
        } else if (this->clipPath) {
            canvas->clipPath(*this->clipPath, this->clipOp, this->clipAntiAlias);
        } else {
            auto rect = SkRect::MakeWH(this->bounds.width(), this->bounds.height());
            canvas->clipRect(rect, this->clipOp, this->clipAntiAlias);
        }
    }

    if (this->layerPaint) {
        auto rect = SkRect::MakeWH(this->bounds.width(), this->bounds.height());
        canvas->saveLayer(rect, &*this->layerPaint);
    } else {
        canvas->save();
    }

    if (this->alpha < 1.0f && !this->layerPaint) {
        // Modulating alpha at replay time has to reach every recorded paint, which
        // means unrolling the content. A snapshot is only cheap while it is replayed
        // whole, so this path neither records nor uses one.
        AlphaModulateFilterCanvas alphaCanvas(canvas, this->alpha);
        if (this->contentCache) {
            drawDrawableCompat(&alphaCanvas, canvas, this->contentCache.get());
        }
    } else {
        this->updateSnapshot();
        if (this->contentSnapshot) {
            canvas->drawPicture(this->contentSnapshot.get());
        } else if (this->contentCache) {
            drawDrawableCompat(canvas, this->contentCache.get());
        }
    }

    canvas->restore();
    if (this->clip) {
        canvas->restore();
    }
    canvas->restore();
}

// The shadow is drawn against the context's light geometry, which is expressed in the
// coordinate space the resulting picture is replayed in.
sk_sp<SkPicture> RenderNode::onMakePictureSnapshot() {
    SkCanvas* canvas = this->beginRecording();
    UnrollDrawableCanvas unrollCanvas(canvas);
    this->draw(&unrollCanvas);
    return this->recorder.finishRecordingAsPicture();
}

// Adoption from frameworks/base/libs/hwui/RenderProperties.cpp
void RenderNode::updateMatrix() {
    if (!this->matrixDirty) {
        return;
    }
    float pivotX, pivotY;
    if (this->pivot.isFinite()) {
        pivotX = this->pivot.fX;
        pivotY = this->pivot.fY;
    } else {
        pivotX = this->bounds.width() / 2.0f;
        pivotY = this->bounds.height() / 2.0f;
    }
    this->transformMatrix.reset();
    if (isZero(this->rotationX) && isZero(this->rotationY)) {
        this->transformMatrix.setTranslate(this->translationX, this->translationY);
        this->transformMatrix.preRotate(this->rotationZ, pivotX, pivotY);
        this->transformMatrix.preScale(this->scaleX, this->scaleY, pivotX, pivotY);
    } else {
        this->transformMatrix.preScale(this->scaleX, this->scaleY, pivotX, pivotY);

        SkM44 transform3D;
        transform3D.preConcat(SkM44::Rotate({1, 0, 0}, -this->rotationX * SK_ScalarPI / 180));
        transform3D.preConcat(SkM44::Rotate({0,-1, 0}, -this->rotationY * SK_ScalarPI / 180));
        transform3D.preConcat(SkM44::Rotate({0, 0, 1}, -this->rotationZ * SK_ScalarPI / 180));
        SkPatch3D patch3D;
        patch3D.transform(transform3D);

        SkMatrix transform;
        this->transformCamera.patchToMatrix(patch3D, &transform);
        transform.preTranslate(-pivotX, -pivotY);
        transform.postTranslate(pivotX + this->translationX, pivotY + this->translationY);
        this->transformMatrix.postConcat(transform);
    }
    this->matrixDirty = false;
    this->matrixIdentity = this->transformMatrix.isIdentity();
}

// Maintains the invariant `node in dep->observers` iff `dep in node->dependencies`.
// Only dependencies holds references: an observer registration is a bare back
// pointer, so that a parent drawing a child does not form a reference cycle.
void RenderNode::releaseDependencies() {
    for (auto renderNode : this->dependencies) {
        renderNode->observers.erase(this);
        renderNode->unref();
    }
    this->dependencies.clear();
}

void RenderNode::updateDependencies() {
    // Tracking exists to invalidate the snapshots that inline this node, so it is only
    // worth its replay of the recording where snapshots are taken at all.
    if (!this->context->shouldUseSnapshotCache()) {
        return;
    }
    this->releaseDependencies();
    if (this->contentCache) {
        DependencyTrackerCanvas dependencyTracker(this->dependencies);
        this->contentCache->draw(&dependencyTracker);

        for (auto renderNode : this->dependencies) {
            renderNode->observers.insert(this);
        }
    }
}

void RenderNode::updateSnapshot() {
    if (!this->context->shouldUseSnapshotCache() || !this->contentTransformInvariant ||
        !this->contentCache || this->contentSnapshot) {
        return;
    }

    SkPictureRecorder snapshotRecorder;
    SkCanvas* recordingCanvas = this->beginRecordingInto(snapshotRecorder);
    // Force unrolling all drawables to avoid nested snapshotting.
    UnrollDrawableCanvas unrollCanvas(recordingCanvas);
    this->contentCache->draw(&unrollCanvas);
    this->contentSnapshot = snapshotRecorder.finishRecordingAsPicture();
}

bool RenderNode::drawsTransformDependentContent() const {
    return this->shadowElevation > 0.0f;
}

bool RenderNode::preventsObserverSnapshot() const {
    return this->drawsTransformDependentContent() || !this->contentTransformInvariant;
}

// Snapshots hold the recorded content only: the transform, clip, layer, alpha and
// shadow are applied around them in onDraw, so which of those changed decides whether
// this node's own snapshot survives. None of them leaves the snapshots that inlined
// this node's drawing valid, so those are dropped whatever the change was.
void RenderNode::invalidateSnapshot(uint32_t changes) {
    this->notifyDrawingChanged();

    if (changes & kDependencies) {
        bool wasPreventingObserverSnapshot = this->preventsObserverSnapshot();
        this->contentTransformInvariant = true;
        for (auto renderNode : this->dependencies) {
            if (renderNode->preventsObserverSnapshot()) {
                this->contentTransformInvariant = false;
                break;
            }
        }
        // Observers derive their own snapshottability from this bit, so a change to it has
        // to reach them even if they were already told stale this frame.
        if (this->preventsObserverSnapshot() != wasPreventingObserverSnapshot) {
            this->observersNotified = false;
        }
    }
    if (changes & kContent) {
        this->contentSnapshot.reset();
    }

    // Observers inlined this node's drawing when they recorded, so they hold a stale copy
    // until they record again -- which redraws this node and clears the flag below. One
    // notification per draw is enough to invalidate them, and telling them once instead of
    // once per observer path bounds the walk when a node is drawn through several of them.
    if (!this->observersNotified) {
        this->observersNotified = true;
        for (auto renderNode : this->observers) {
            renderNode->invalidateSnapshot(kContent | kDependencies);
        }
    }
}

void RenderNode::drawShadow(SkCanvas *canvas, const LightGeometry& lightGeometry, const LightInfo& lightInfo) {
    SkPath tmpPath, *path = &tmpPath;
    if (this->clipRect) {
        tmpPath = SkPathBuilder().addRect(*this->clipRect).detach();
    } else if (this->clipRRect) {
        tmpPath = SkPathBuilder().addRRect(*this->clipRRect).detach();
    } else if (this->clipPath) {
        path = &*this->clipPath;
    } else {
        return;
    }

    SkPoint3 zParams{0.0f, 0.0f, this->shadowElevation};
    float ambientAlpha = lightInfo.ambientShadowAlpha * this->alpha;
    float spotAlpha = lightInfo.spotShadowAlpha * this->alpha;
    SkColor ambientColor = multiplyAlpha(this->ambientShadowColor, ambientAlpha);
    SkColor spotColor = multiplyAlpha(this->spotShadowColor, spotAlpha);

    SkShadowUtils::DrawShadow(
        canvas,
        *path,
        zParams,
        lightGeometry.center,
        lightGeometry.radius,
        ambientColor,
        spotColor,
        this->alpha < 1.0f ? SkShadowFlags::kTransparentOccluder_ShadowFlag : SkShadowFlags::kNone_ShadowFlag
    );
}

// Adoption from frameworks/base/libs/hwui/RenderProperties.cpp
void RenderNode::setCameraLocation(float x, float y, float z) {
    // the camera location is passed in inches, set in pt
    SkScalar lz = z * 72.0f;
    this->transformCamera.fLocation = {x * 72.0f, y * 72.0f, lz};
    this->transformCamera.fObserver = {0, 0, lz};
    this->transformCamera.update();
}

} // namespace node
} // namespace skialin

using skialin::node::RenderNode;
using skialin::node::RenderNodeContext;

skialin::node::RenderNode* skialin_bridge_RenderNode_Make(skialin::node::RenderNodeContext* context) {
    auto node = sk_make_sp<RenderNode>(sk_ref_sp(context));
    return node.release();
}

void skialin_bridge_RenderNode_unref(skialin::node::RenderNode* node) {
    SkSafeUnref(node);
}

bool skialin_bridge_RenderNode_getLayerPaint(const skialin::node::RenderNode* node, SkPaint* outPaint) {
    // getLayerPaint() aliases the node's own optional<SkPaint>. Rust's Paint owns a
    // bindgen-allocated SkPaint value it can copy-assign into, rather than a pointer it would
    // need to free with a mismatched (C++ `delete` vs. Rust's allocator) ownership model.
    const std::optional<SkPaint>& layerPaint = const_cast<RenderNode*>(node)->getLayerPaint();
    if (!layerPaint) return false;
    *outPaint = *layerPaint;
    return true;
}

void skialin_bridge_RenderNode_setLayerPaint(skialin::node::RenderNode* node, const SkPaint* paint) {
    node->setLayerPaint(paint ? std::optional<SkPaint>{*paint} : std::nullopt);
}

void skialin_bridge_RenderNode_getBounds(const skialin::node::RenderNode* node, SkRect* outBounds) {
    *outBounds = node->getBounds();
}

void skialin_bridge_RenderNode_setBounds(skialin::node::RenderNode* node, const SkRect* bounds) {
    node->setBounds(*bounds);
}

void skialin_bridge_RenderNode_getPivot(const skialin::node::RenderNode* node, SkPoint* outPivot) {
    *outPivot = node->getPivot();
}

void skialin_bridge_RenderNode_setPivot(skialin::node::RenderNode* node, float x, float y) {
    node->setPivot(SkPoint::Make(x, y));
}

float skialin_bridge_RenderNode_getAlpha(const skialin::node::RenderNode* node) {
    return node->getAlpha();
}

void skialin_bridge_RenderNode_setAlpha(skialin::node::RenderNode* node, float alpha) {
    node->setAlpha(alpha);
}

float skialin_bridge_RenderNode_getScaleX(const skialin::node::RenderNode* node) {
    return node->getScaleX();
}

void skialin_bridge_RenderNode_setScaleX(skialin::node::RenderNode* node, float scaleX) {
    node->setScaleX(scaleX);
}

float skialin_bridge_RenderNode_getScaleY(const skialin::node::RenderNode* node) {
    return node->getScaleY();
}

void skialin_bridge_RenderNode_setScaleY(skialin::node::RenderNode* node, float scaleY) {
    node->setScaleY(scaleY);
}

float skialin_bridge_RenderNode_getTranslationX(const skialin::node::RenderNode* node) {
    return node->getTranslationX();
}

void skialin_bridge_RenderNode_setTranslationX(skialin::node::RenderNode* node, float translationX) {
    node->setTranslationX(translationX);
}

float skialin_bridge_RenderNode_getTranslationY(const skialin::node::RenderNode* node) {
    return node->getTranslationY();
}

void skialin_bridge_RenderNode_setTranslationY(skialin::node::RenderNode* node, float translationY) {
    node->setTranslationY(translationY);
}

float skialin_bridge_RenderNode_getShadowElevation(const skialin::node::RenderNode* node) {
    return node->getShadowElevation();
}

void skialin_bridge_RenderNode_setShadowElevation(skialin::node::RenderNode* node, float shadowElevation) {
    node->setShadowElevation(shadowElevation);
}

uint32_t skialin_bridge_RenderNode_getAmbientShadowColor(const skialin::node::RenderNode* node) {
    return node->getAmbientShadowColor();
}

void skialin_bridge_RenderNode_setAmbientShadowColor(skialin::node::RenderNode* node, uint32_t color) {
    node->setAmbientShadowColor(static_cast<SkColor>(color));
}

uint32_t skialin_bridge_RenderNode_getSpotShadowColor(const skialin::node::RenderNode* node) {
    return node->getSpotShadowColor();
}

void skialin_bridge_RenderNode_setSpotShadowColor(skialin::node::RenderNode* node, uint32_t color) {
    node->setSpotShadowColor(static_cast<SkColor>(color));
}

float skialin_bridge_RenderNode_getRotationX(const skialin::node::RenderNode* node) {
    return node->getRotationX();
}

void skialin_bridge_RenderNode_setRotationX(skialin::node::RenderNode* node, float rotationX) {
    node->setRotationX(rotationX);
}

float skialin_bridge_RenderNode_getRotationY(const skialin::node::RenderNode* node) {
    return node->getRotationY();
}

void skialin_bridge_RenderNode_setRotationY(skialin::node::RenderNode* node, float rotationY) {
    node->setRotationY(rotationY);
}

float skialin_bridge_RenderNode_getRotationZ(const skialin::node::RenderNode* node) {
    return node->getRotationZ();
}

void skialin_bridge_RenderNode_setRotationZ(skialin::node::RenderNode* node, float rotationZ) {
    node->setRotationZ(rotationZ);
}

float skialin_bridge_RenderNode_getCameraDistance(const skialin::node::RenderNode* node) {
    return node->getCameraDistance();
}

void skialin_bridge_RenderNode_setCameraDistance(skialin::node::RenderNode* node, float cameraDistance) {
    node->setCameraDistance(cameraDistance);
}

void skialin_bridge_RenderNode_setClipRect(skialin::node::RenderNode* node, const SkRect* rect, SkClipOp op, bool antiAlias) {
    node->setClipRect(rect ? std::optional<SkRect>{*rect} : std::nullopt, static_cast<SkClipOp>(op), antiAlias);
}

void skialin_bridge_RenderNode_setClipRRect(skialin::node::RenderNode* node, const SkRRect* rrect, SkClipOp op, bool antiAlias) {
    node->setClipRRect(rrect ? std::optional<SkRRect>{*rrect} : std::nullopt, static_cast<SkClipOp>(op), antiAlias);
}

void skialin_bridge_RenderNode_setClipPath(skialin::node::RenderNode* node, const SkPath* path, SkClipOp op, bool antiAlias) {
    node->setClipPath(path ? std::optional<SkPath>{*path} : std::nullopt, static_cast<SkClipOp>(op), antiAlias);
}

bool skialin_bridge_RenderNode_getClip(const skialin::node::RenderNode* node) {
    return node->getClip();
}

void skialin_bridge_RenderNode_setClip(skialin::node::RenderNode* node, bool clip) {
    node->setClip(clip);
}

SkCanvas* skialin_bridge_RenderNode_beginRecording(skialin::node::RenderNode* node) {
    return node->beginRecording();
}

void skialin_bridge_RenderNode_endRecording(skialin::node::RenderNode* node) {
    node->endRecording();
}

void skialin_bridge_RenderNode_drawInto(skialin::node::RenderNode* node, SkCanvas* canvas) {
    node->drawInto(canvas);
}
