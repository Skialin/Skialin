package org.skialin

/** Whether [Canvas.drawImageRect] may sample slightly outside `src`. Mirrors `SkCanvas::SrcRectConstraint`. */
enum class SrcRectConstraint { STRICT, FAST }
