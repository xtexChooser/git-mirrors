package xtex.minecraftServerPropsDumper.mjapi

import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.ExperimentalCoroutinesApi
import kotlinx.coroutines.withContext
import kotlinx.serialization.json.Json
import okhttp3.OkHttpClient
import okhttp3.Request
import okhttp3.executeAsync
import org.apache.commons.io.FileUtils
import xtex.minecraftServerPropsDumper.util.ensureFile
import java.io.File

const val USER_AGENT = "xtex-minecraft-server-props-dumper/1 (https://source.moe/xtex/minecraft-server-props-dumper)"

val GlobalHTTPClient = OkHttpClient.Builder()
    .addInterceptor {
        it.proceed(
            it.request().newBuilder()
                .header("User-Agent", USER_AGENT)
                .build()
        )
    }
    .build()

val MjAPIJson = Json { ignoreUnknownKeys = true }

val DownloadHTTPClient = GlobalHTTPClient.newBuilder().build()

@OptIn(ExperimentalCoroutinesApi::class)
val DownloadCoroutineScope = Dispatchers.IO.limitedParallelism(2)

suspend fun downloadFile(url: String) = withContext(DownloadCoroutineScope) {
    DownloadHTTPClient.newCall(
        Request.Builder()
            .get()
            .url(url)
            .build()
    ).executeAsync().body.byteStream()
}

val LAUNCHER_MOJANG_COM_MIRRORS = arrayOf(
    "launcher.mojang.com",
    "bmclapi2.bangbang93.com",
    "download.mcbbs.net"
)

suspend fun downloadFileTo(url: String, file: String) =
    withContext(Dispatchers.IO) {
        FileUtils.copyToFile(downloadFile(url), File(file))
    }


suspend fun ensureServerJar(version: String) = ensureFile("$version-server.jar") {
    val url = (fetchGameVersion(version).fetchClientJson().downloads.server
        ?: error("Version $version is too old(<= 1.2.4), no public server URL found")).url/*.useMirror()*/
    println("Downloading: $url")
    downloadFileTo(url, it.absolutePath)
}

fun String.useMirror() = replace("launcher.mojang.com", LAUNCHER_MOJANG_COM_MIRRORS.random())
