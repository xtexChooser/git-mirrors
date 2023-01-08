package xtex.minecraftServerPropsDumper.main

import kotlinx.coroutines.runBlocking
import picocli.CommandLine

import picocli.CommandLine.Command
import picocli.CommandLine.Option
import picocli.CommandLine.Parameters
import xtex.minecraftServerPropsDumper.mjapi.downloadFileTo
import xtex.minecraftServerPropsDumper.mjapi.fetchClientJson
import xtex.minecraftServerPropsDumper.mjapi.fetchGameVersion
import xtex.minecraftServerPropsDumper.mjapi.fetchGameVersions
import java.util.concurrent.Callable
import kotlin.system.exitProcess

@Command(
    name = "minecraft-server-props-dumper",
    description = ["A tool to analyze history information about server.properties"],
    mixinStandardHelpOptions = true,
    subcommands = [CommandLine.HelpCommand::class]
)
class Main : Runnable {

    companion object {
        @JvmStatic
        fun main(args: Array<String>) {
            exitProcess(CommandLine(Main()).execute(*args))
        }
    }

    @Option(names = ["-v", "--verbose"], description = ["..."])
    private var verbose: Boolean = false

    override fun run() {
        println("Hi!")
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
        runBlocking {
            val url = fetchGameVersion(version).fetchClientJson().downloads.server.url
            println("URL: $url")
            downloadFileTo(url, "server-$version.jar")
        }
        return 0
    }

}
