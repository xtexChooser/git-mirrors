package quaedam.misc.reality

import net.minecraft.core.BlockPos
import net.minecraft.core.Direction
import net.minecraft.world.level.block.EntityBlock
import net.minecraft.world.level.block.HorizontalDirectionalBlock
import net.minecraft.world.level.block.SimpleWaterloggedBlock
import net.minecraft.world.level.block.entity.BlockEntity
import net.minecraft.world.level.block.state.BlockState
import net.minecraft.world.level.block.state.properties.BlockStateProperties
import net.minecraft.world.level.material.MapColor

object RSBlock : HorizontalDirectionalBlock(
    Properties.of()
        .noOcclusion()
        .strength(3f)
        .requiresCorrectToolForDrops()
        .mapColor(MapColor.COLOR_CYAN)
), EntityBlock, SimpleWaterloggedBlock {

    init {
        registerDefaultState(
            defaultBlockState()
                .setValue(FACING, Direction.EAST)
                .setValue(BlockStateProperties.WATERLOGGED, false)
        )
    }

    override fun newBlockEntity(pos: BlockPos, state: BlockState) = RSBlockEntity(pos, state)

}