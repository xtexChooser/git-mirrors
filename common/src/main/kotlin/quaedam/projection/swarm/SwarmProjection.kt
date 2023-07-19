package quaedam.projection.swarm

import net.minecraft.world.item.BlockItem
import net.minecraft.world.item.Item
import quaedam.Quaedam
import quaedam.projection.ProjectionEffectType
import quaedam.projection.SimpleProjectionEntity

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

    val blockEntity = Quaedam.blockEntities.register(ID) {
        SimpleProjectionEntity.createBlockEntityType(block, ::SwarmProjectionEffect)
    }!!

    init {
        ProjectedPersonEntity
    }

}