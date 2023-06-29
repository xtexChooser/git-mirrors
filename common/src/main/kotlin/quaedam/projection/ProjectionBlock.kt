package quaedam.projection

import net.minecraft.core.BlockPos
import net.minecraft.server.level.ServerLevel
import net.minecraft.util.RandomSource
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

    fun projectionActivated(level: ServerLevel, projectorPos: BlockPos, projectionPos: BlockPos) {
    }

    fun projectionDeactivated(level: ServerLevel, projectorPos: BlockPos, projectionPos: BlockPos) {
    }

    fun projectorRandomTick(level: ServerLevel, projectorPos: BlockPos, projectionPos: BlockPos, random: RandomSource) {
    }

}