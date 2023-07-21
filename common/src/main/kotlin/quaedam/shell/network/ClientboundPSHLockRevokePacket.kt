package quaedam.shell.network

import dev.architectury.networking.NetworkManager.PacketContext
import dev.architectury.utils.GameInstance
import net.minecraft.network.chat.Component
import quaedam.shell.ProjectionShell
import quaedam.shell.ProjectionShellScreen
import java.util.function.Supplier

object ClientboundPSHLockRevokePacket {

    init {
        ProjectionShell.channel.register(
            ClientboundPSHLockRevokePacket::class.java,
            { _, _ -> },
            { ClientboundPSHLockRevokePacket },
            { _, ctx -> apply(ctx) }
        )
    }

    private fun apply(context: Supplier<PacketContext>) {
        val ctx = context.get()
        if (ctx.player.level().isClientSide) {
            ctx.queue {
                val client = GameInstance.getClient()
                if (client.screen is ProjectionShellScreen) {
                    client.setScreen(null)
                    client.gui.setOverlayMessage(
                        Component.translatable("quaedam.screen.projection_shell.lock_revoked"),
                        false
                    )
                }
            }
        }
    }

}