/* Bindgen entry point. Real Skia headers first, so bindgen binds the actual
 * C++ classes directly; skialin/bridge.h last for the sk_sp ownership seams
 * bindgen can't cross on its own. See native-shim/include/skialin/bridge.h. */

#include "include/core/SkColor.h"
#include "include/core/SkPoint.h"
#include "include/core/SkRect.h"
#include "include/core/SkRRect.h"
#include "include/core/SkMatrix.h"
#include "include/core/SkPathTypes.h"
#include "include/core/SkPath.h"
#include "include/core/SkPathBuilder.h"
#include "include/core/SkPaint.h"
#include "include/core/SkBlendMode.h"
#include "include/core/SkClipOp.h"
#include "include/core/SkColorType.h"
#include "include/core/SkAlphaType.h"
#include "include/core/SkImageInfo.h"
#include "include/core/SkCanvas.h"
#include "include/core/SkSurface.h"
#include "include/core/SkBitmap.h"
#include "include/core/SkImage.h"
#include "include/core/SkData.h"
#include "include/core/SkColorSpace.h"

#include "skialin/bridge.h"
