#pragma once
#include "include/core/SkPoint3.h"

// Adoption of frameworks/base/libs/hwui/Lighting.h (also mirrors skiko's node/Lighting.h).

namespace skialin {
namespace node {

struct LightGeometry {
    SkPoint3 center;
    float radius;
};

struct LightInfo {
    float ambientShadowAlpha;
    float spotShadowAlpha;
};

} // namespace node
} // namespace skialin
