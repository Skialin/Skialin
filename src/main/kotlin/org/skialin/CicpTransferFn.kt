package org.skialin

/** Rows of ITU-T H.273 Table 3 that [ColorSpace.makeCICP] accepts. */
enum class CicpTransferFn(internal val code: Int) {
    REC709(1), REC470_SYSTEM_M(4), REC470_SYSTEM_BG(5), REC601(6), SMPTE_ST_240(7), LINEAR(8),
    IEC61966_2_4(11), SRGB(13), REC2020_10BIT(14), REC2020_12BIT(15), PQ(16), SMPTE_ST_428_1(17), HLG(18),
}
