package quaedam.shell.network

import dev.architectury.networking.NetworkManager.PacketContext
import net.minecraft.core.BlockPos
import net.minecraft.network.FriendlyByteBuf
import net.minecraft.server.level.ServerLevel
import net.minecraft.server.level.ServerPlayer
import quaedam.shell.ProjectionShell
import quaedam.shell.ProjectionShellMutex
import java.util.function.Supplier

data class ServerboundPSHLockReleasePacket(val pos: BlockPos) {

    companion object {
        init {
            ProjectionShell.channel.register(
                ServerboundPSHLockReleasePacket::class.java,
                ServerboundPSHLockReleasePacket::encode,
                ::ServerboundPSHLockReleasePacket,
                ServerboundPSHLockReleasePacket::apply
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
                ProjectionShellMutex.release(ctx.player.level() as ServerLevel, pos, player)
            }
        }
    }

}