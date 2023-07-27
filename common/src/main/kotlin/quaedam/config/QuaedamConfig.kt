package quaedam.config

import com.google.gson.Gson
import com.google.gson.GsonBuilder
import dev.architectury.event.events.client.ClientPlayerEvent
import dev.architectury.platform.Platform
import dev.architectury.utils.GameInstance
import net.fabricmc.api.EnvType
import net.minecraft.nbt.CompoundTag
import quaedam.Quaedam
import java.nio.file.Path
import kotlin.io.path.exists
import kotlin.io.path.notExists
import kotlin.io.path.readText
import kotlin.io.path.writeText

data class QuaedamConfig(
    val projectorEffectRadius: Int = 4
) {

    companion object {
        const val TAG_PROJECTOR_EFFECT_RADIUS = "ProjectorEffectRadius"

        private val localFile: Path = Platform.getConfigFolder().resolve("quaedam.json")
        private var local0 = loadLocalConfig()
        var local
            get() = local0
            set(value) {
                local0 = value
                writeLocalConfig()
            }
        var remote: QuaedamConfig? = null
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
            Gson().fromJson(localFile.readText(), QuaedamConfig::class.java)
        } else {
            QuaedamConfig()
        }

        private fun writeLocalConfig() {
            localFile.writeText(GsonBuilder().serializeNulls().setPrettyPrinting().create().toJson(local0))
        }

        fun applyRemoteConfig(config: QuaedamConfig?) {
            Quaedam.logger.info("Received remote config push: $config")
            remote = config
        }

        fun fromPushNbt(tag: CompoundTag) = QuaedamConfig(
            projectorEffectRadius = tag.getInt(TAG_PROJECTOR_EFFECT_RADIUS)
        )
    }

    fun toPushNbt(tag: CompoundTag) {
        tag.putInt(TAG_PROJECTOR_EFFECT_RADIUS, projectorEffectRadius)
    }

    fun toPushNbt(forPush: Boolean) = CompoundTag().also { toPushNbt(it) }

}
