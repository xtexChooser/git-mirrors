package xtex.minecraftServerPropsDumper.analyzer

import java.io.File
import java.io.FileInputStream
import java.io.InputStream
import java.util.jar.JarEntry
import java.util.jar.JarInputStream

fun JarInputStream.extractBundle(): InputStream? {
    var entry: JarEntry? = nextJarEntry
    while (entry != null) {
        if (entry.name.startsWith("META-INF/versions/") && entry.name.endsWith(".jar")) {
            return this
        }
        entry = nextJarEntry
    }
    return null
}

fun File.extractBundle(): JarInputStream {
    return JarInputStream(FileInputStream(this)).extractBundle()?.let { JarInputStream(it) }
        ?: JarInputStream(FileInputStream(this))
}
