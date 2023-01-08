package xtex.minecraftServerPropsDumper.util

import java.io.File

suspend fun ensureFile(name: String, generator: suspend (File) -> Unit): File {
    val file = File(name)
    if (!file.exists()) {
        generator(file)
    }
    return file
}
