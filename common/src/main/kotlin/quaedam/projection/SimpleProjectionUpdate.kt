package quaedam.projection

import dev.architectury.networking.NetworkManager
import dev.architectury.networking.NetworkManager.PacketContext
import io.netty.buffer.Unpooled
import net.minecraft.core.BlockPos
import net.minecraft.nbt.CompoundTag
import net.minecraft.network.FriendlyByteBuf
import net.minecraft.network.chat.Component
import net.minecraft.resources.ResourceLocation
import net.minecraft.server.level.ServerPlayer
import quaedam.Quaedam
import quaedam.utils.sendBlockUpdated

object SimpleProjectionUpdate {

    val id = ResourceLocation("quaedam", "simple_projection_update")

    init {
        NetworkManager.registerReceiver(NetworkManager.Side.C2S, id, ::handle)
    }

    private fun handle(buf: FriendlyByteBuf, ctx: PacketContext) {
        val player = ctx.player!! as ServerPlayer
        val level = player.level()

        val pos = buf.readBlockPos()
        val data = buf.readNbt()!!

        if (player.blockPosition().distSqr(pos) > 10 * 10) {
            Quaedam.logger.info("Player ${player.name} tried to update a projection block far away")
            if (player.blockPosition().distSqr(pos) > 50 * 50) {
                player.connection.disconnect(Component.literal("[Quaedam] wth r u doing? why not waiting for server?"))
            }
            return
        }

        level.server!!.execute {
            val entity = level.getBlockEntity(pos) ?: return@execute
            val blockEntity = entity as SimpleProjectionEntity<*>
            try {
                blockEntity.projection.fromNbt(data, trusted = false)
            } catch (e: Throwable) {
                Quaedam.logger.error(
                    "Player ${player.name} tried to update projection " +
                            "at $pos but caused error: $data", e
                )
                player.connection.disconnect(Component.literal("[Quaedam] ? wait what did you send to the server?"))
                return@execute
            }
            blockEntity.sendBlockUpdated()
            ProjectionBlock.sendUpdateToProjectors(level, pos)
        }
    }

    fun send(pos: BlockPos, data: CompoundTag) {
        val buf = FriendlyByteBuf(Unpooled.buffer())
        buf.writeBlockPos(pos)
        buf.writeNbt(data)
        NetworkManager.sendToServer(id, buf)
    }

}