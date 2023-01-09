package xtex.minecraftServerPropsDumper.mjapi

import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.ExperimentalCoroutinesApi
import kotlinx.coroutines.delay
import kotlinx.coroutines.sync.Mutex
import kotlinx.coroutines.sync.withLock
import kotlinx.coroutines.withContext
import kotlinx.serialization.json.Json
import okhttp3.OkHttpClient
import okhttp3.Request
import okhttp3.executeAsync
import org.apache.commons.io.FileUtils
import xtex.minecraftServerPropsDumper.util.ensureFile
import java.io.File
import java.io.IOException

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

val DownloadHTTPClient = GlobalHTTPClient.newBuilder()
    .retryOnConnectionFailure(true)
    .build()

@OptIn(ExperimentalCoroutinesApi::class)
val DownloadCoroutineScope = Dispatchers.IO.limitedParallelism(5)

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

val downloadDelayController = Mutex()

suspend fun ensureServerJar(version: String) = ensureFile("$version-server.jar") {
    val url = fetchGameVersion(version).fetchClientJson().downloads.server?.url
        ?: (tryResolveArchiveDownloadURL(version)
            ?.apply { println("Resolved archived server jar: $version $this") })
        ?: error("Version $version is too old(<= 1.2.4), no public server URL found")
    println("Downloading: $url")
    var retries = 0
    while (true) {
        try {
            downloadDelayController.withLock(url) { }
            downloadFileTo(url, it.absolutePath)
            break
        } catch (e: IOException) {
            println(e.toString())
            retries++
            if (retries > 3) {
                println("Global download delay $retries from $version")
                downloadDelayController.withLock { delay(retries * 3000L) }
            } else {
                println("Local download delay $retries for $version")
                delay(retries * 1000L)
            }
        }
    }
}

fun String.useMirror() = replace("launcher.mojang.com", LAUNCHER_MOJANG_COM_MIRRORS.random())
