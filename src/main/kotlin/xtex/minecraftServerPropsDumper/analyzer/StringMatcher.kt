package xtex.minecraftServerPropsDumper.analyzer

import kotlinx.coroutines.flow.Flow
import java.io.File
import java.util.jar.JarInputStream

val FINGERPRINT_STRINGS = arrayOf(
    "port",
    "max-players",
    "server-name",
    "motd",
    "pvp",
    "view-distance",
    "max-connections",
    "grow-trees",
    "white-list",
    "use-native-transport",
    "sync-chunk-writes",
    "spawn-protection",
    "spawn-npcs",
    "spawn-monsters",
    "online-mode",
    "prevent-proxy-connections",
)

suspend fun Flow<String>.matchStrings(): Int {
    var count = 0
    collect {
        if (it in FINGERPRINT_STRINGS) {
            count++
        }
    }
    return count
}

suspend fun File.findPropertiesClass(): Pair<String, Int> {
    var maxClasses = mutableListOf<String>()
    var maxCount = -1;
    JarInputStream(extractBundle()).extractClasses { name, input ->
        val count = input.extractStrings(name).matchStrings()
        if (count > maxCount) {
            maxCount = count
            maxClasses = mutableListOf(name)
        } else if (count == maxCount) {
            maxClasses += name
        }
    }
    if (maxClasses.size > 1) {
        error("Too many matches: $maxCount $maxClasses")
    }
    return maxClasses.first() to maxCount
}
