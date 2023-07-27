package quaedam.config

import dev.architectury.networking.NetworkManager
import io.netty.buffer.Unpooled
import net.minecraft.nbt.CompoundTag
import net.minecraft.network.FriendlyByteBuf
import net.minecraft.server.level.ServerPlayer
import quaedam.Quaedam

object SimpleQuaedamConfigPush {

    val id = Quaedam.resource("simple_config_push")

    init {
        NetworkManager.registerReceiver(NetworkManager.Side.S2C, id, ::handle)
    }

    private fun handle(buf: FriendlyByteBuf, ctx: NetworkManager.PacketContext) {
        val data = buf.readNbt()!!
        val config = QuaedamConfig.fromPushNbt(data)
        QuaedamConfig.applyRemoteConfig(config)
    }

    fun sendCurrent(player: ServerPlayer) = send(player, QuaedamConfig.current)

    fun send(player: ServerPlayer, config: QuaedamConfig) = send(player, config.toPushNbt(forPush = true))

    private fun send(player: ServerPlayer, data: CompoundTag) {
        val buf = FriendlyByteBuf(Unpooled.buffer())
        buf.writeNbt(data)
        NetworkManager.sendToPlayer(player, id, buf)
    }

}