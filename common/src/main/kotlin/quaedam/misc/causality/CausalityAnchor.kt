package quaedam.misc.causality

import net.minecraft.core.BlockPos
import net.minecraft.world.item.BlockItem
import net.minecraft.world.item.Item
import net.minecraft.world.level.Level
import net.minecraft.world.level.block.entity.BlockEntityType
import quaedam.Quaedam

object CausalityAnchor {

    const val ID = "causality_anchor"

    val block = Quaedam.blocks.register(ID) { CABlock }!!

    val item = Quaedam.items.register(ID) {
        BlockItem(
            CABlock, Item.Properties()
                .stacksTo(1)
                .`arch$tab`(Quaedam.creativeModeTab)
        )
    }!!

    val blockEntity = Quaedam.blockEntities.register(ID) {
        BlockEntityType.Builder.of(::CABlockEntity, block.get()).build(null)
    }!!

    fun checkEffect(level: Level, pos: BlockPos) = level.getChunkAt(pos)
        .blockEntities
        .any { (_, v) -> v is CABlockEntity }

}