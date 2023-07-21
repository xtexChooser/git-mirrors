package quaedam.shell.network

import dev.architectury.networking.NetworkManager.PacketContext
import dev.architectury.utils.GameInstance
import net.minecraft.core.BlockPos
import net.minecraft.network.FriendlyByteBuf
import net.minecraft.network.chat.Component
import quaedam.Quaedam
import quaedam.shell.ProjectionShell
import quaedam.shell.ProjectionShellBlock
import quaedam.shell.ProjectionShellScreen
import java.util.function.Supplier

data class ClientboundPSHLockResultPacket(val pos: BlockPos, val result: Boolean) {

    companion object {
        init {
            ProjectionShell.channel.register(
                ClientboundPSHLockResultPacket::class.java,
                ClientboundPSHLockResultPacket::encode,
                ::ClientboundPSHLockResultPacket,
                ClientboundPSHLockResultPacket::apply
            )
        }
    }

    constructor(buf: FriendlyByteBuf) : this(buf.readBlockPos(), buf.readBoolean())

    fun encode(buf: FriendlyByteBuf) {
        buf.writeBlockPos(pos)
        buf.writeBoolean(result)
    }

    fun apply(context: Supplier<PacketContext>) {
        val ctx = context.get()
        if (ctx.player.level().isClientSide) {
            val client = GameInstance.getClient()
            if (result) {
                val level = ctx.player.level()
                val block = level.getBlockState(pos).block
                if (block is ProjectionShellBlock) {
                    ctx.queue {
                        try {
                            client.setScreen(
                                ProjectionShellScreen(
                                    level,
                                    pos,
                                    block.getProjectionEffectForShell(level, pos)
                                )
                            )
                        } catch (e: Throwable) {
                            Quaedam.logger.error("Failed to open projection shell screen", e)
                        }
                    }
                } else {
                    Quaedam.logger.warn("ClientboundPSHLockResultPacket with non-shell-provider block received")
                }
            } else {
                ctx.queue {
                    client.setScreen(null)
                    client.gui.setOverlayMessage(
                        Component.translatable("quaedam.screen.projection_shell.lock_failed"),
                        false
                    )
                }
            }
        }
    }

}