package xtex.minecraftServerPropsDumper.analyzer

import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.withContext
import java.io.InputStream
import java.util.jar.JarEntry
import java.util.jar.JarInputStream

suspend fun JarInputStream.extractClasses(handler: suspend (String, InputStream) -> Unit) {
    withContext(Dispatchers.IO) {
        var entry: JarEntry? = nextJarEntry
        while (entry != null) {
            if (entry.name.endsWith(".class") && (!entry.name.contains("/") || entry.name.startsWith("net/"))) {
                handler(entry.name, this@extractClasses)
            }
            entry = nextJarEntry
        }
    }
}
