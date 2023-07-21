package quaedam.shell.network

import dev.architectury.networking.NetworkManager.PacketContext
import net.minecraft.core.BlockPos
import net.minecraft.network.FriendlyByteBuf
import net.minecraft.server.level.ServerLevel
import net.minecraft.server.level.ServerPlayer
import quaedam.shell.ProjectionShell
import quaedam.shell.ProjectionShellMutex
import java.util.function.Supplier

data class ServerboundPSHLockAcquirePacket(val pos: BlockPos) {

    companion object {
        init {
            ProjectionShell.channel.register(
                ServerboundPSHLockAcquirePacket::class.java,
                ServerboundPSHLockAcquirePacket::encode,
                ::ServerboundPSHLockAcquirePacket,
                ServerboundPSHLockAcquirePacket::apply
            )
        }
    }

    constructor(buf: FriendlyByteBuf) : this(buf.readBlockPos())

    fun encode(buf: FriendlyByteBuf) {
        buf.writeBlockPos(pos)
    }

    fun apply(context: Supplier<PacketContext>) {
        val ctx = context.get()
        if (!ctx.player.level().isClientSide) {
            ctx.queue {
                val player = ctx.player as ServerPlayer
                val result = ProjectionShellMutex.tryLock(ctx.player.level() as ServerLevel, pos, player)
                ProjectionShell.channel.sendToPlayer(player, ClientboundPSHLockResultPacket(pos, result))
            }
        }
    }

}