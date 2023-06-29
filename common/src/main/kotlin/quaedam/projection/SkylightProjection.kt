package quaedam.projection

import net.minecraft.world.item.BlockItem
import net.minecraft.world.item.Item
import quaedam.Quaedam

object SkylightProjection {

    const val ID = "skylight_projection"

    val block = Quaedam.blocks.register(ID) { SkylightProjectionBlock }

    val item = Quaedam.items.register(ID) {
        BlockItem(
            SkylightProjectionBlock, Item.Properties()
                .`arch$tab`(Quaedam.creativeModeTab)
        )
    }!!

}

object SkylightProjectionBlock : ProjectionBlock(createProperties().lightLevel { 3 })
