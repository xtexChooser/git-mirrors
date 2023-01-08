package xtex.minecraftServerPropsDumper.analyzer

import kotlinx.serialization.encodeToString
import kotlinx.serialization.json.Json
import xtex.minecraftServerPropsDumper.mjapi.ensureServerJar
import xtex.minecraftServerPropsDumper.util.ensureFile

suspend fun analyze(version: String): AnalyzeReport {
    try {
        val file = ensureServerJar(version)
        val (propClass, propCount) = file.findPropertiesClass()
        val strings = file.extractKeys(propClass).toSet()
        val keys = strings.matchKeys()
        return AnalyzeReport(
            version = version,
            propertiesClass = propClass,
            propertiesClassFingerprints = propCount,
            keys = keys.toSet(),
        )
    } catch (e: Throwable) {
        return AnalyzeReport(
            version = version,
            error = if (e is IllegalStateException) e.message else e.stackTraceToString()
        )
    }
}

val REPORT_SERIALIZER = Json { prettyPrint = true }

suspend fun doReport(version: String) = ensureFile("$version-report.json") {
    ensureServerJar(version)
    it.writeText(REPORT_SERIALIZER.encodeToString(analyze(version)))
    ensureServerJar(version).delete()
}
