package quaedam.shell

import dev.architectury.networking.NetworkChannel
import quaedam.Quaedam
import quaedam.shell.network.ClientboundPSHLockResultPacket
import quaedam.shell.network.ClientboundPSHLockRevokePacket
import quaedam.shell.network.ServerboundPSHLockAcquirePacket
import quaedam.shell.network.ServerboundPSHLockReleasePacket

object ProjectionShell {

    const val ID = "projection_shell"

    val item = Quaedam.items.register(ID) { ProjectionShellItem }!!

    val channel: NetworkChannel = NetworkChannel.create(Quaedam.resource(ID))

    init {
        ServerboundPSHLockAcquirePacket
        ServerboundPSHLockReleasePacket
        ClientboundPSHLockRevokePacket
        ClientboundPSHLockResultPacket
    }

}