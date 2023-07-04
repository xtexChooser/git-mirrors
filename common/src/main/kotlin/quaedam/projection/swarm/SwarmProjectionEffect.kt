package quaedam.projection.swarm

import net.minecraft.core.BlockPos
import net.minecraft.nbt.CompoundTag
import net.minecraft.server.level.ServerLevel
import net.minecraft.world.entity.MobSpawnType
import net.minecraft.world.level.levelgen.Heightmap
import quaedam.projection.ProjectionEffect
import quaedam.projector.ProjectorBlockEntity
import kotlin.math.min

data class SwarmProjectionEffect(
    var maxCount: Int = 180,
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
        val projector = level.getBlockEntity(pos) as ProjectorBlockEntity
        val entities = level.getEntitiesOfClass(ProjectedPersonEntity::class.java, projector.effectAreaAABB).size
        if (entities < maxCount) {
            val area = projector.effectArea
            for (i in 0..(min(level.random.nextInt(maxCount - entities), 6))) {
                var spawnPos = BlockPos(
                    level.random.nextInt(area.minX(), area.maxX()),
                    level.random.nextInt(area.minY(), area.maxY()),
                    level.random.nextInt(area.minZ(), area.maxZ()),
                )
                spawnPos = spawnPos.atY(level.getHeight(Heightmap.Types.WORLD_SURFACE, spawnPos.x, spawnPos.z))
                if (level.getBlockState(spawnPos.below()).isAir)
                    return
                if (!level.getBlockState(spawnPos).isAir)
                    return
                ProjectedPersonEntity.entity.get().spawn(level, spawnPos, MobSpawnType.TRIGGERED)
            }
        }
    }

}
