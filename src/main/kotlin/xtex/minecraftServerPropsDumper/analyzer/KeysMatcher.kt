package xtex.minecraftServerPropsDumper.analyzer

import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.flow.toList
import kotlinx.coroutines.withContext
import okio.utf8Size
import java.io.File
import java.util.jar.JarEntry
import java.util.jar.JarInputStream

val KEY_FILTER_PATTERN = "[a-z\\-]+".toRegex()
val KEY_DENYLIST = setOf(
    "true",
    "false",
    "default", // 1.18.x
    "nogui", // 1.2.5
    "world", // all ver, default value of level-name
    "vanilla", // 1.2.5, 1.13.x
    "save-all", // b1.2_01- command
    "save-off", // b1.2_01- command
    "save-on", // b1.2_01- command
    "stop", // b1.2_01- command
)

suspend fun File.extractKeys(klass: String): List<String> =
    withContext(Dispatchers.IO) {
        val jis = JarInputStream(extractBundle())
        var entry: JarEntry?
        do {
            entry = jis.nextJarEntry
            if (entry?.name == klass) {
                return@withContext jis.extractStrings(klass).toList()
            }
        } while (entry != null)
        return@withContext null
    } ?: error("Jar Entry $klass not found")

fun Iterable<String>.matchKeys(): Set<String> = this
    .asSequence()
    .filter { it.utf8Size() == it.length.toLong() }
    .filter { it.isNotEmpty() }
    .filter { KEY_FILTER_PATTERN.matches(it) }
    .filter { it !in KEY_DENYLIST }
    .sorted()
    .toSet()
