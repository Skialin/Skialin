package org.skialin

/** Rows of ITU-T H.273 Table 2 that [ColorSpace.makeCICP] accepts. */
enum class CicpPrimaries(internal val code: Int) {
    REC709(1), REC470_SYSTEM_M(4), REC470_SYSTEM_BG(5), REC601(6), SMPTE_ST_240(7),
    GENERIC_FILM(8), REC2020(9), SMPTE_ST_428_1(10), SMPTE_RP_431_2(11), SMPTE_EG_432_1(12),
}
