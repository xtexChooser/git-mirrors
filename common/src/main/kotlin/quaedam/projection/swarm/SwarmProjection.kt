package quaedam.projection.swarm

import net.minecraft.world.item.BlockItem
import net.minecraft.world.item.Item
import quaedam.Quaedam
import quaedam.projection.ProjectionEffectType

object SwarmProjection {

    const val ID = "swarm_projection"
    const val SHORT_ID = "swarm"

    val block = Quaedam.blocks.register(ID) { SwarmProjectionBlock }!!

    val item = Quaedam.items.register(ID) {
        BlockItem(
            SwarmProjectionBlock, Item.Properties()
                .`arch$tab`(Quaedam.creativeModeTab)
        )
    }!!

    val effect = Quaedam.projectionEffects.register(SHORT_ID) {
        ProjectionEffectType { SwarmProjectionEffect() }
    }!!

}