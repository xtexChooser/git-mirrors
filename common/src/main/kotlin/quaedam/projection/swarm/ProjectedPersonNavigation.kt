package quaedam.projection.swarm

import net.minecraft.core.BlockPos
import net.minecraft.world.entity.ai.navigation.GroundPathNavigation
import net.minecraft.world.level.Level
import net.minecraft.world.level.pathfinder.Path
import quaedam.projector.Projector

class ProjectedPersonNavigation(val entity: ProjectedPersonEntity, level: Level) : GroundPathNavigation(entity, level) {

    override fun createPath(set: MutableSet<BlockPos>, i: Int, bl: Boolean, j: Int, f: Float): Path? {
        if (set.any { Projector.findNearbyProjections(level, it, SwarmProjection.effect.get()).isEmpty() }) {
            return null
        }
        return super.createPath(set, i, bl, j, f)
    }

}