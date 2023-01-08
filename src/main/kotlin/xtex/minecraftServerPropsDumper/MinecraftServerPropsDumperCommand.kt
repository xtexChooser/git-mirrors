package xtex.minecraftServerPropsDumper

import io.micronaut.configuration.picocli.PicocliRunner

import picocli.CommandLine.Command
import picocli.CommandLine.Option

@Command(
    name = "minecraft-server-props-dumper",
    description = ["..."],
    mixinStandardHelpOptions = true
)
class MinecraftServerPropsDumperCommand : Runnable {

    @Option(names = ["-v", "--verbose"], description = ["..."])
    private var verbose: Boolean = false

    override fun run() {
        println("Hi!")
    }

    companion object {
        @JvmStatic
        fun main(args: Array<String>) {
            PicocliRunner.run(MinecraftServerPropsDumperCommand::class.java, *args)
        }
    }

}
