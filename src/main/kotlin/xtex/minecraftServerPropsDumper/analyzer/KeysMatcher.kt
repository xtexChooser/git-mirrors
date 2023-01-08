package xtex.minecraftServerPropsDumper.analyzer

import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.flow.toList
import kotlinx.coroutines.withContext
import okio.utf8Size
import java.io.File
import java.util.jar.JarFile

val KEY_FILTER_PATTERN = "[a-z\\-]+".toRegex()

suspend fun File.extractKeys(klass: String): List<String> =
    withContext(Dispatchers.IO) {
        JarFile(this@extractKeys).use { jf ->
            jf.getInputStream(jf.getJarEntry(klass)).use { it.extractStrings(klass).toList() }
        }
    }

fun Iterable<String>.matchKeys(): List<String> = this
    .filter { it.utf8Size() == it.length.toLong() }
    .filter { it.isNotEmpty() }
    .filter { KEY_FILTER_PATTERN.matches(it) }
