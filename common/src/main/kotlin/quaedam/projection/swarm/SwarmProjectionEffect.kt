package quaedam.projection.swarm

import net.minecraft.core.BlockPos
import net.minecraft.nbt.CompoundTag
import net.minecraft.server.level.ServerLevel
import quaedam.projection.ProjectionEffect

data class SwarmProjectionEffect(
    var maxCount: Int = 10,
    var withPlayer: Boolean = true,
    var withVillager: Boolean = true
) : ProjectionEffect() {

    companion object {
        const val TAG_MAX_COUNT = "MaxCount"
        const val TAG_WITH_PLAYER = "WithPlayer"
        const val TAG_WITH_VILLAGER = "WithVillager"
    }

    override val type
        get() = SwarmProjection.effect.get()!!

    override fun toNbt(tag: CompoundTag) {
        tag.putInt(TAG_MAX_COUNT, maxCount)
        tag.putBoolean(TAG_WITH_PLAYER, withPlayer)
        tag.putBoolean(TAG_WITH_VILLAGER, withVillager)
    }

    override fun fromNbt(tag: CompoundTag) {
        maxCount = tag.getInt(TAG_MAX_COUNT)
        withPlayer = tag.getBoolean(TAG_WITH_PLAYER)
        withVillager = tag.getBoolean(TAG_WITH_VILLAGER)
    }

    override fun randomTick(level: ServerLevel, pos: BlockPos) {
    }

}
