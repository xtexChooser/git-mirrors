package xtex.minecraftServerPropsDumper.analyzer

import com.github.miachm.sods.*
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.flow.channelFlow
import kotlinx.coroutines.flow.map
import kotlinx.coroutines.flow.toList
import kotlinx.coroutines.withContext
import kotlinx.serialization.decodeFromString
import kotlinx.serialization.json.Json
import java.io.File

suspend fun doSummarize() {
    doTableSummarize()
    doDiffSummarize()
}

fun collectALlReportFiles() = File(".")
    .listFiles { _, name -> name.endsWith("-report.json") }!!
    .toSet()
    .sortedDescending()

suspend fun collectAllKeys() = collectAllReports()
    .map { it.keys ?: emptySet() }
    .toList()
    .flatten()
    .toSet()

suspend fun collectAllReports() = channelFlow<AnalyzeReport> {
    withContext(Dispatchers.IO) {
        collectALlReportFiles()
            .forEach {
                send(Json.decodeFromString(it.readText()))
            }
    }
}

val STYLE_BORDERS = Borders(true)
val STYLE_FOUND = Style().apply {
    backgroundColor = Color("#00ff00")
    borders = STYLE_BORDERS
}
val STYLE_NOT_FOUND = Style().apply {
    backgroundColor = Color("#ff0000")
    borders = STYLE_BORDERS
}

// @TODO: https://github.com/miachm/SODS/issues/55
suspend fun doTableSummarize() {
    val allKeys = collectAllKeys().sorted()
    println("Summarizing ${allKeys.size} keys as table")
    val allFiles = collectALlReportFiles()

    val tableFile = File("summary.ods")
    val table = SpreadSheet()

    val sheet = Sheet("Report Summary", allFiles.size + 1, allKeys.size + 3)
    // populate headers
    sheet.getRange(0, 0, 1, 3)
        .setValues("Ver", "Prop Class", "Prop Fingerprints")
    sheet.getRange(0, 3, 1, allKeys.size)
        .setValues(*allKeys.toTypedArray())
    sheet.setColumnWidths(0, sheet.maxColumns, 40.0)
    // populate versions column
    sheet.getRange(1, 0, allFiles.size, 1)
        .setValues(*allFiles.map { it.nameWithoutExtension.substringBefore('-') }.toTypedArray())
    collectAllReports().toList()
        .sortedByDescending { it.releaseTime }
        .forEachIndexed { index, report ->
            sheet.getRange(index + 1, 0, 1, 3)
                .setValues(
                    report.version,
                    report.propertiesClass ?: report.error ?: "ERROR: NO ERROR AND NO PROP",
                    report.propertiesClassFingerprints,
                )
            if (report.keys != null)
                sheet.getRange(index + 1, 3, 1, allKeys.size)
                    .setStyles(*(allKeys.map { it in report.keys }
                        .map { if (it) STYLE_FOUND else STYLE_NOT_FOUND }
                        .toTypedArray()))
        }

    table.appendSheet(sheet)
    withContext(Dispatchers.IO) {
        table.save(tableFile)
    }
    println("Table summarized")
}

suspend fun doDiffSummarize() {
    println("Summarizing as diff")
    val summary = buildString {
        val reports = collectAllReports().toList()
            .sortedBy { it.releaseTime }
        reports.forEachIndexed { index, report ->
            if (index == 0)
                return@forEachIndexed
            val lastReport = reports[index - 1]
            if (report.error != null) {
                appendLine("###### ${report.version}").appendLine()
                appendLine("```").appendLine(report.error).appendLine("```").appendLine()
            } else {
                if (lastReport.keys != null && report.keys != null && report.keys != lastReport.keys) {
                    appendLine("###### ${report.version}").appendLine()
                    appendLine(
                        "Ver: `${report.version}` PV: `${lastReport.version}` RT: `${report.releaseTime}` " +
                                "PC: `${report.propertiesClass}` PFC: `${report.propertiesClassFingerprints}`"
                    ).appendLine()
                    appendLine("```diff")
                    appendLine(buildList {
                        report.keys.forEach {
                            if (it !in lastReport.keys) {
                                add("+ $it")
                            }
                        }
                        lastReport.keys.forEach {
                            if (it !in report.keys) {
                                add("- $it")
                            }
                        }
                    }
                        .sortedBy { it.substring(2) }
                        .joinToString(separator = "\n"))
                    appendLine("```").appendLine()
                }
            }
        }
    }

    withContext(Dispatchers.IO) {
        File("summary.md").writeText(summary)
    }
    println("Diff summarized")
}
