package xtex.minecraftServerPropsDumper.mjapi

import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.withContext
import kotlinx.serialization.json.Json
import okhttp3.OkHttpClient
import okhttp3.Request
import okhttp3.executeAsync
import org.apache.commons.io.FileUtils
import java.io.File
import java.io.FileOutputStream

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

suspend fun downloadFile(url: String) = DownloadHTTPClient.newCall(
    Request.Builder()
        .get()
        .url(url)
        .build()
).executeAsync().body.byteStream()

suspend fun downloadFileTo(url: String, file: String) =
    withContext(Dispatchers.IO) {
        FileUtils.copyToFile(downloadFile(url), File(file))
    }
