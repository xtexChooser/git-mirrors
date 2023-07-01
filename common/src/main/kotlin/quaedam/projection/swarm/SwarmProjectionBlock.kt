package quaedam.projection.swarm

import net.minecraft.core.BlockPos
import net.minecraft.server.level.ServerLevel
import net.minecraft.world.level.block.state.BlockState
import quaedam.projection.ProjectionBlock

object SwarmProjectionBlock : ProjectionBlock<SwarmProjectionEffect>() {

    override fun createProjectionEffect(
        level: ServerLevel,
        state: BlockState,
        pos: BlockPos
    ) = SwarmProjectionEffect()

}
