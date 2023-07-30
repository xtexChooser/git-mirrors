package quaedam.config

import dev.architectury.event.events.client.ClientPlayerEvent
import dev.architectury.platform.Platform
import dev.architectury.utils.GameInstance
import kotlinx.serialization.Serializable
import kotlinx.serialization.encodeToString
import kotlinx.serialization.json.Json
import net.fabricmc.api.EnvType
import net.minecraft.nbt.*
import quaedam.Quaedam
import java.nio.file.Path
import kotlin.io.path.exists
import kotlin.io.path.notExists
import kotlin.io.path.readText
import kotlin.io.path.writeText

@Serializable
data class QuaedamConfig(
    val valuesInt: Map<String, Int> = mapOf(),
    val valuesFloat: Map<String, Float> = mapOf(),
    val valuesDouble: Map<String, Double> = mapOf(),
) {

    companion object {

        private val localJson = Json {
            isLenient = true
            prettyPrint = true
            encodeDefaults = true
            ignoreUnknownKeys = true
        }

        private val pushJson = Json {
            encodeDefaults = true
            ignoreUnknownKeys = true
        }

        private val localFile: Path = Platform.getConfigFolder().resolve("quaedam.json")
        private var local0 = loadLocalConfig()
        var local
            get() = local0
            set(value) {
                local0 = value
                writeLocalConfig()
            }
        private var remote: QuaedamConfig? = null
        val current get() = remote ?: local0

        init {
            SimpleQuaedamConfigPush

            if (Platform.getEnv() == EnvType.CLIENT) {
                ClientPlayerEvent.CLIENT_PLAYER_QUIT.register { player ->
                    if (player == GameInstance.getClient().player) {
                        applyRemoteConfig(null)
                    }
                }
            }
            if (localFile.notExists()) {
                writeLocalConfig()
            }
        }

        private fun loadLocalConfig(): QuaedamConfig = if (localFile.exists()) {
            localJson.decodeFromString(localFile.readText())
        } else {
            QuaedamConfig()
        }

        private fun writeLocalConfig() {
            localFile.writeText(localJson.encodeToString(local0))
        }

        fun applyRemoteConfig(config: QuaedamConfig?) {
            Quaedam.logger.info("Received remote config push: $config")
            remote = config
        }

        const val TAG_VALUES_INT = "ValuesInt"
        const val TAG_VALUES_FLOAT = "ValuesFloat"
        const val TAG_VALUES_DOUBLE = "ValuesDouble"

        fun fromPushNbt(tag: CompoundTag): QuaedamConfig {
            return QuaedamConfig(
                valuesInt = pushJson.decodeFromString(tag.getString(TAG_VALUES_INT)),
                valuesFloat = pushJson.decodeFromString(tag.getString(TAG_VALUES_FLOAT)),
                valuesDouble = pushJson.decodeFromString(tag.getString(TAG_VALUES_DOUBLE)),
            )
        }
    }

    fun toPushNbt(tag: CompoundTag) {
        tag.putString(TAG_VALUES_INT, pushJson.encodeToString(valuesInt))
        tag.putString(TAG_VALUES_FLOAT, pushJson.encodeToString(valuesFloat))
        tag.putString(TAG_VALUES_DOUBLE, pushJson.encodeToString(valuesDouble))
    }

    fun toPushNbt() = CompoundTag().also { toPushNbt(it) }

}
