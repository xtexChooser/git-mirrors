package quaedam.projection.swarm

import net.minecraft.core.BlockPos
import net.minecraft.nbt.CompoundTag
import net.minecraft.server.level.ServerLevel
import quaedam.projection.ProjectionEffect

data class SwarmProjectionEffect(
    var maxCount: Int = 10,
) : ProjectionEffect() {

    companion object {
        const val TAG_MAX_COUNT = "MaxCount"
    }

    override val type
        get() = SwarmProjection.effect.get()!!

    override fun toNbt(tag: CompoundTag) {
        tag.putInt(TAG_MAX_COUNT, maxCount)
    }

    override fun fromNbt(tag: CompoundTag) {
        maxCount = tag.getInt(TAG_MAX_COUNT)
    }

    override fun randomTick(level: ServerLevel, pos: BlockPos) {
    }

}
