package xtex.minecraftServerPropsDumper.mjapi

import kotlinx.serialization.Serializable
import kotlinx.serialization.decodeFromString
import okhttp3.Request
import okhttp3.executeAsync

suspend fun VersionInfo.fetchClientJson() = MjAPIJson.decodeFromString<ClientJson>(
    GlobalHTTPClient.newCall(
        Request.Builder()
            .get()
            .url(url)
            .build()
    ).executeAsync().body.string()
)

@Serializable
data class ClientJson(
    val downloads: Downloads
) {

    @Serializable
    data class Downloads(
        val server: Download? = null,
    )

    @Serializable
    data class Download(
        val sha1: String,
        val size: Int,
        val url: String,
    )

}
