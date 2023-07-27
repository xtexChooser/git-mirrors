package quaedam.misc.cts

import net.minecraft.core.BlockPos
import net.minecraft.world.item.BlockItem
import net.minecraft.world.item.Item
import net.minecraft.world.level.Level
import net.minecraft.world.level.block.entity.BlockEntityType
import quaedam.Quaedam

object ConstantTemporalSink {

    const val ID = "cts"

    val block = Quaedam.blocks.register(ID) { CTSBlock }!!

    val item = Quaedam.items.register(ID) {
        BlockItem(
            CTSBlock, Item.Properties()
                .stacksTo(1)
                .`arch$tab`(Quaedam.creativeModeTab)
        )
    }!!

    val blockEntity = Quaedam.blockEntities.register(ID) {
        BlockEntityType.Builder.of(::CTSBlockEntity, block.get()).build(null)
    }!!

    fun checkEffect(level: Level, pos: BlockPos) = level.getChunkAt(pos)
        .blockEntities
        .any { (_, v) -> v is CTSBlockEntity }

}