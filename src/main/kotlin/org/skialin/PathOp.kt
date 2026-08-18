package org.skialin

/** A boolean operation combining two paths. Mirrors Skia's `SkPathOp`. */
enum class PathOp { DIFFERENCE, INTERSECT, UNION, XOR, REVERSE_DIFFERENCE }
