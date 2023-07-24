package quaedam.shell.network

import dev.architectury.networking.NetworkManager.PacketContext
import net.minecraft.core.BlockPos
import net.minecraft.network.FriendlyByteBuf
import net.minecraft.server.level.ServerLevel
import net.minecraft.server.level.ServerPlayer
import quaedam.shell.ProjectionShell
import quaedam.shell.ProjectionShellBlock
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
                val level = ctx.player.level() as ServerLevel
                if (level.getBlockState(pos).block !is ProjectionShellBlock) {
                    ProjectionShell.channel.sendToPlayer(player, ClientboundPSHLockResultPacket(pos, false))
                    return@queue
                }
                val result = ProjectionShellMutex.tryLock(level, pos, player)
                ProjectionShell.channel.sendToPlayer(player, ClientboundPSHLockResultPacket(pos, result))
            }
        }
    }

}