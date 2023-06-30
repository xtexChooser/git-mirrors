package quaedam.projector

import net.minecraft.world.item.BlockItem
import net.minecraft.world.item.Item
import net.minecraft.world.level.block.entity.BlockEntityType
import quaedam.Quaedam

object Projector {

    const val ID = "projector"

    val block = Quaedam.blocks.register(ID) { ProjectorBlock }!!

    val item = Quaedam.items.register(ID) {
        BlockItem(
            ProjectorBlock, Item.Properties()
                .stacksTo(1)
                .`arch$tab`(Quaedam.creativeModeTab)
        )
    }!!

    val blockEntity = Quaedam.blockEntities.register(ID) {
        BlockEntityType.Builder.of(::ProjectorBlockEntity, block.get()).build(null)
    }!!

}
