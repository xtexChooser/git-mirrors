package xtex.minecraftServerPropsDumper.test

import io.kotest.common.runBlocking
import io.kotest.core.spec.style.StringSpec
import io.kotest.engine.test.logging.info
import io.kotest.matchers.string.shouldContain
import kotlinx.serialization.decodeFromString
import okhttp3.Request
import okhttp3.executeAsync
import xtex.minecraftServerPropsDumper.mjapi.*
import xtex.minecraftServerPropsDumper.mjapi.VersionManifest

class GlobalHTTPClient : StringSpec({
    "user agent" {
        GlobalHTTPClient.newCall(
            Request.Builder()
                .get()
                .url("https://ipconfig.io/json")
                .build()
        ).executeAsync().body.string() shouldContain USER_AGENT
    }
})

class MjAPIJson : StringSpec({
    "ignore unknown keys" {
        MjAPIJson.decodeFromString<VersionManifest>(
            """
                {"versions":[], "key": "value"}
            """.trimIndent()
        )
    }
})

class VersionManifest : StringSpec({
    "fetch versions" {
        val str = fetchGameVersions().toString()
        info { str }
    }
})

class ClientJson : StringSpec({
    "fetch client json" {
        val version = fetchGameVersions().versions.first()
        info { "Version: ${version.id} ${version.url}" }
        val client = version.fetchClientJson()
        info { client.toString() }
    }
})
