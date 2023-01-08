package xtex.minecraftServerPropsDumper.analyzer

import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.flow.toList
import kotlinx.coroutines.withContext
import okio.utf8Size
import java.io.File
import java.util.jar.JarEntry
import java.util.jar.JarFile
import java.util.jar.JarInputStream

val KEY_FILTER_PATTERN = "[a-z\\-]+".toRegex()

suspend fun File.extractKeys(klass: String): List<String> {
    withContext(Dispatchers.IO) {
        val jis = JarInputStream(extractBundle())
        var entry: JarEntry?
        do {
            entry = jis.nextJarEntry
            if (entry?.name == klass) {
                return@withContext jis.extractStrings(klass).toList()
            }
        } while (entry != null)
    }
    error("Jar Entry $klass not found")
}

fun Iterable<String>.matchKeys(): List<String> = this
    .filter { it.utf8Size() == it.length.toLong() }
    .filter { it.isNotEmpty() }
    .filter { KEY_FILTER_PATTERN.matches(it) }
