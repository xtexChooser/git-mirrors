package xtex.minecraftServerPropsDumper.main

import kotlinx.coroutines.*
import picocli.CommandLine
import picocli.CommandLine.*
import xtex.minecraftServerPropsDumper.analyzer.*
import xtex.minecraftServerPropsDumper.mjapi.ensureServerJar
import xtex.minecraftServerPropsDumper.mjapi.fetchClientJson
import xtex.minecraftServerPropsDumper.mjapi.fetchGameVersion
import xtex.minecraftServerPropsDumper.mjapi.fetchGameVersions
import java.io.File
import java.util.jar.JarInputStream
import kotlin.system.exitProcess
import kotlin.time.ExperimentalTime
import kotlin.time.measureTime

@Command(
    name = "minecraft-server-props-dumper",
    description = ["A tool to analyze history information about server.properties"],
    mixinStandardHelpOptions = true,
    subcommands = [CommandLine.HelpCommand::class]
)
class Main {

    companion object {
        @JvmStatic
        fun main(args: Array<String>) {
            exitProcess(CommandLine(Main()).execute(*args))
        }
    }

    @Command(name = "getVersionManifest")
    fun getVersionManifest(): Int {
        println(runBlocking { fetchGameVersions() })
        return 0
    }

    @Command(name = "listVersions")
    fun listVersions(): Int {
        runBlocking {
            fetchGameVersions().versions.forEach {
                println(it.id)
            }
        }
        return 0
    }

    @Command(name = "allVersionTypes")
    fun allVersionTypes(): Int {
        runBlocking {
            fetchGameVersions().versions.map { it.type }
                .distinct()
                .forEach { println(it) }
        }
        return 0
    }

    @Command(name = "getClientJson")
    fun getClientJson(@Parameters version: String): Int {
        runBlocking {
            println(fetchGameVersion(version).fetchClientJson().toString())
        }
        return 0
    }

    @Command(name = "downloadServer")
    fun downloadServer(@Parameters version: String): Int {
        runBlocking { ensureServerJar(version) }
        return 0
    }

    @Command(name = "extractClasses")
    fun extractClasses(@Parameters version: String): Int {
        runBlocking {
            JarInputStream(ensureServerJar(version).extractBundle()).extractClasses { name, _ -> println(name) }
        }
        return 0
    }

    @OptIn(ExperimentalTime::class)
    @Command(name = "matchClass")
    fun matchClass(@Parameters version: String): Int {
        runBlocking {
            ensureServerJar(version)
            measureTime {
                val (klass, count) = ensureServerJar(version).findPropertiesClass()
                println("$klass for $count fingerprints matched")
            }.let { println("Took $it") }
        }
        return 0
    }

    @OptIn(ExperimentalTime::class)
    @Command(name = "analyze")
    fun runAnalyze(@Parameters version: String): Int {
        runBlocking {
            println("Reported in ${measureTime { println(analyze(version).toString()) }}")
        }
        return 0
    }

    @OptIn(ExperimentalTime::class)
    @Command(name = "report")
    fun report(@Parameters version: String): Int {
        runBlocking {
            println("Reported $version in ${measureTime { doReport(version) }}")
        }
        return 0
    }

    @OptIn(ExperimentalTime::class)
    @Command(name = "summarize")
    fun summarize(): Int {
        runBlocking {
            println("Summarized in ${measureTime { doSummarize() }}")
        }
        return 0
    }

    @OptIn(ExperimentalTime::class, ExperimentalCoroutinesApi::class)
    @Command(name = "reportAll")
    fun reportAll(): Int {
        File(".")
            .listFiles { _, name -> name.endsWith("-report.json") || name.endsWith(".ods") }
            ?.forEach { it.delete() }
        runBlocking {
            val versions = fetchGameVersions().versions.map { it.id }
            coroutineScope {
                withContext(Dispatchers.Default.limitedParallelism(10)) {
                    versions.forEach { version ->
                        launch {
                            for (i in 1..15) {
                                try {
                                    println("Reporting $version")
                                    println("Reported $version in ${measureTime { doReport(version) }}")
                                    break
                                } catch (e: Throwable) {
                                    println("Failed to report $version, retry $i")
                                    if (i == 15) {
                                        println("Give up reporting $version")
                                        e.printStackTrace()
                                    } else {
                                        File(".")
                                            .listFiles { _, name -> name.startsWith("$version-") }
                                            ?.forEach { it.delete() }
                                        delay(5000)
                                    }
                                }
                            }
                        }
                    }
                }
            }
            println("Reported all, summarizing")
            summarize()
        }
        return 0
    }

}
