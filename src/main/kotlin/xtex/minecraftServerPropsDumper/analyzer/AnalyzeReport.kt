package xtex.minecraftServerPropsDumper.analyzer

import kotlinx.serialization.Serializable

@Serializable
data class AnalyzeReport(
    val version: String,
    val error: String? = null,
    val propertiesClass: String? = null,
    val propertiesClassFingerprints: Int? = null,
    val keys: Set<String>? = null,
)
