package xtex.minecraftServerPropsDumper.mjapi

import kotlinx.serialization.Serializable
import kotlinx.serialization.decodeFromString
import okhttp3.Request
import okhttp3.executeAsync

var cachedManifest: VersionManifest? = null
suspend fun fetchGameVersions(): VersionManifest {
    if (cachedManifest == null) {
        cachedManifest = MjAPIJson.decodeFromString<VersionManifest>(
            GlobalHTTPClient.newCall(
                Request.Builder()
                    .get()
                    .url("https://piston-meta.mojang.com/mc/game/version_manifest_v2.json")
                    .build()
            ).executeAsync().body.string()
        )
    }
    return cachedManifest!!
}

suspend fun fetchGameVersion(version: String) =
    fetchGameVersions().versions.find { it.id == version } ?: error("Version $version not found")

@Serializable
data class VersionManifest(
    // val latest
    val versions: List<VersionInfo>
)

@Serializable
data class VersionInfo(
    val id: String,
    val type: String,
    val url: String,
    val time: String,
    val releaseTime: String,
    val sha1: String,
    val complianceLevel: Int,
)
