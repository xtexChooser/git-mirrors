package xtex.minecraftServerPropsDumper.mjapi

val ConstantDownloadURLs = mapOf(
    "1.1" to "https://files.betacraft.uk/server-archive/release/1.1/1.1.jar",
    "1.0" to "https://files.betacraft.uk/server-archive/release/1.0/1.0.0.jar",
    "a1.2.6" to "https://files.betacraft.uk/server-archive/alpha/a0.2.8.jar",
    "a1.2.5" to "https://files.betacraft.uk/server-archive/alpha/a0.2.7.jar",
    "a1.2.4_01" to "https://files.betacraft.uk/server-archive/alpha/a0.2.6_02.jar",
    "a1.2.3_04" to "https://files.betacraft.uk/server-archive/alpha/a0.2.5_02.jar",
    "a1.2.3_02" to "https://files.betacraft.uk/server-archive/alpha/a0.2.6_01.jar",
    "a1.2.3_02" to "https://files.betacraft.uk/server-archive/alpha/a0.2.6_01.jar",
    "a1.2.3_01" to "https://files.betacraft.uk/server-archive/alpha/a0.2.6.jar", // need info
    "a1.2.3" to "https://files.betacraft.uk/server-archive/alpha/a0.2.5_02.jar",
    "a1.2.2a" to "https://files.betacraft.uk/server-archive/alpha/a0.2.4.jar",
    "a1.2.2b" to "https://files.betacraft.uk/server-archive/alpha/a0.2.4.jar",
    "a1.2.1" to "https://files.betacraft.uk/server-archive/alpha/a0.2.3.jar",
    "a1.2.1_01" to "https://files.betacraft.uk/server-archive/alpha/a0.2.3.jar",
    "a1.2.0_01" to "https://files.betacraft.uk/server-archive/alpha/a0.2.2.jar",
    "a1.2.0_02" to "https://files.betacraft.uk/server-archive/alpha/a0.2.2.jar",
    "a1.2.0" to "https://files.betacraft.uk/server-archive/alpha/a0.2.2.jar",
    "a1.1.2_01" to "https://files.betacraft.uk/server-archive/alpha/a0.2.1.jar",
    "a1.1.2" to "https://files.betacraft.uk/server-archive/alpha/a0.2.1.jar",
    "a1.1.0" to "https://files.betacraft.uk/server-archive/alpha/a0.2.0_01.jar",
    "a1.0.17_04" to "https://files.betacraft.uk/server-archive/alpha/a0.1.4.jar",
    "a1.0.17_02" to "https://files.betacraft.uk/server-archive/alpha/a0.1.4.jar",
    "a1.0.16" to "https://files.betacraft.uk/server-archive/alpha/a0.1.1-1707.jar",
    "a1.0.15" to "https://files.betacraft.uk/server-archive/alpha/a0.1.0.jar",
)

fun tryResolveArchiveDownloadURL(version: String): String? {
    if (version.startsWith("1.2.")) {
        return "https://files.betacraft.uk/server-archive/release/1.2/$version.jar"
    } else if (version.startsWith("b1.")) {
        return "https://files.betacraft.uk/server-archive/beta/$version.jar"
    }
    return ConstantDownloadURLs[version]
}
