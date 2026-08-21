#include "skialin/bridge.h"
#include "skialin/node/RenderNodeContext.h"

namespace skialin {
namespace node {

RenderNodeContext::RenderNodeContext(bool measureDrawBounds, bool snapshotCache)
    : lightGeometry{}, lightInfo{}, measureDrawBounds(measureDrawBounds), snapshotCache(snapshotCache) {}

void RenderNodeContext::setLightingInfo(const LightGeometry& lightGeometry, const LightInfo& lightInfo) {
    this->lightGeometry = lightGeometry;
    this->lightInfo = lightInfo;
}

} // namespace node
} // namespace skialin

skialin::node::RenderNodeContext* skialin_bridge_RenderNodeContext_Make(bool measureDrawBounds, bool snapshotCache) {
    return new skialin::node::RenderNodeContext(measureDrawBounds, snapshotCache);
}

void skialin_bridge_RenderNodeContext_unref(skialin::node::RenderNodeContext* context) {
    SkSafeUnref(context);
}

void skialin_bridge_RenderNodeContext_setLightingInfo(
    skialin::node::RenderNodeContext* context,
    float centerX, float centerY, float centerZ, float radius,
    float ambientShadowAlpha, float spotShadowAlpha) {
    skialin::node::LightGeometry lightGeometry{SkPoint3::Make(centerX, centerY, centerZ), radius};
    skialin::node::LightInfo lightInfo{ambientShadowAlpha, spotShadowAlpha};
    context->setLightingInfo(lightGeometry, lightInfo);
}
