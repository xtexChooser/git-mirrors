package quaedam.shell

import dev.architectury.event.events.common.TickEvent
import net.minecraft.core.BlockPos
import net.minecraft.core.GlobalPos
import net.minecraft.server.level.ServerLevel
import net.minecraft.server.level.ServerPlayer
import quaedam.mixininterface.ProjectionShellMutexAccessor
import quaedam.shell.network.ClientboundPSHLockRevokePacket

object ProjectionShellMutex {

    init {
        TickEvent.SERVER_POST.register { server ->
            val mutex = (server as ProjectionShellMutexAccessor).`quaedam$getProjectionShellMutex`()
            val currentTime = System.currentTimeMillis()
            mutex.forEach { pos, lock ->
                if (currentTime - lock.time > 60 * 1000) {
                    mutex.remove(pos)
                    ProjectionShell.channel.sendToPlayer(lock.player, ClientboundPSHLockRevokePacket)
                }
            }
        }
    }

    fun tryLock(level: ServerLevel, pos: BlockPos, player: ServerPlayer): Boolean {
        val mutex = (level.server as ProjectionShellMutexAccessor).`quaedam$getProjectionShellMutex`()
        val gPos = GlobalPos.of(level.dimension(), pos)
        if (mutex.values.any { it.player == player }) {
            return false
        }
        if (gPos !in mutex) {
            mutex[gPos] = Lock(player, System.currentTimeMillis())
            return true
        }
        return false
    }

    fun release(level: ServerLevel, pos: BlockPos, player: ServerPlayer) {
        val mutex = (level.server as ProjectionShellMutexAccessor).`quaedam$getProjectionShellMutex`()
        val gPos = GlobalPos.of(level.dimension(), pos)
        if (mutex[gPos]?.player == player) {
            mutex.remove(gPos)
        }
    }

    data class Lock(val player: ServerPlayer, val time: Long)

}