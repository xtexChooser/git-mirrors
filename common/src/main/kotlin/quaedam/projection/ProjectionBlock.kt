package quaedam.projection

import net.minecraft.world.item.ItemStack
import net.minecraft.world.level.block.Block
import net.minecraft.world.level.block.state.BlockState
import net.minecraft.world.level.material.MapColor
import net.minecraft.world.level.storage.loot.LootParams

abstract class ProjectionBlock(properties: Properties = createProperties()) : Block(properties) {

    companion object {
        fun createProperties(): Properties = Properties.of()
            .strength(3.5f)
            .requiresCorrectToolForDrops()
            .mapColor(MapColor.COLOR_GRAY)

    }

    @Suppress("OVERRIDE_DEPRECATION")
    override fun getDrops(blockState: BlockState, builder: LootParams.Builder) = listOf(ItemStack(asItem()))

}