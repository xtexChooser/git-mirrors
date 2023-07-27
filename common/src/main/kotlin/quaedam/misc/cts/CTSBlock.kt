package quaedam.misc.cts

import net.minecraft.core.BlockPos
import net.minecraft.core.Direction
import net.minecraft.world.item.context.BlockPlaceContext
import net.minecraft.world.level.BlockGetter
import net.minecraft.world.level.LevelAccessor
import net.minecraft.world.level.block.*
import net.minecraft.world.level.block.state.BlockState
import net.minecraft.world.level.block.state.StateDefinition
import net.minecraft.world.level.block.state.properties.BlockStateProperties
import net.minecraft.world.level.material.FluidState
import net.minecraft.world.level.material.Fluids
import net.minecraft.world.phys.shapes.CollisionContext
import net.minecraft.world.phys.shapes.Shapes
import net.minecraft.world.phys.shapes.VoxelShape

object CTSBlock : HorizontalDirectionalBlock(
    Properties.of()
        .lightLevel { 2 }
        .noOcclusion()
), EntityBlock, SimpleWaterloggedBlock {

    val shapes = getShapeForEachState(::createVoxelShape)

    init {
        registerDefaultState(
            defaultBlockState()
                .setValue(FACING, Direction.EAST)
                .setValue(BlockStateProperties.WATERLOGGED, false)
        )
    }

    override fun newBlockEntity(pos: BlockPos, state: BlockState) = CTSBlockEntity(pos, state)

    override fun createBlockStateDefinition(builder: StateDefinition.Builder<Block, BlockState>) {
        super.createBlockStateDefinition(builder)
        builder.add(FACING, BlockStateProperties.WATERLOGGED)
    }

    override fun getStateForPlacement(context: BlockPlaceContext): BlockState? {
        if (context.level.getBlockState(context.clickedPos.below()).isAir) return null
        return super.defaultBlockState().setValue(FACING, context.horizontalDirection)
    }

    @Suppress("OVERRIDE_DEPRECATION")
    override fun getRenderShape(state: BlockState) = RenderShape.MODEL

    @Suppress("OVERRIDE_DEPRECATION")
    override fun getShape(state: BlockState, level: BlockGetter, pos: BlockPos, context: CollisionContext) =
        shapes[state]!!

    private fun createVoxelShape(state: BlockState): VoxelShape =
        when (state.getValue(FACING)) {
            Direction.WEST, Direction.EAST -> Shapes.or(
                box(0.0, 0.0, 0.0, 16.0, 12.0, 16.0),
                box(7.0, 13.0, 6.0, 9.0, 15.0, 10.0),
            )

            Direction.SOUTH, Direction.NORTH -> Shapes.or(
                box(0.0, 0.0, 0.0, 16.0, 12.0, 16.0),
                box(6.0, 13.0, 7.0, 10.0, 15.0, 9.0),
            )

            else -> throw IllegalStateException(state.getValue(FACING).name)
        }

    @Suppress("OVERRIDE_DEPRECATION", "DEPRECATION")
    override fun updateShape(
        state: BlockState,
        direction: Direction,
        neighborState: BlockState,
        level: LevelAccessor,
        pos: BlockPos,
        neighborPos: BlockPos
    ): BlockState {
        if (state.getValue(BlockStateProperties.WATERLOGGED)) {
            level.scheduleTick(pos, Fluids.WATER, Fluids.WATER.getTickDelay(level))
        }
        return super.updateShape(state, direction, neighborState, level, pos, neighborPos)
    }

    @Suppress("OVERRIDE_DEPRECATION", "DEPRECATION")
    override fun getFluidState(state: BlockState): FluidState = if (state.getValue(BlockStateProperties.WATERLOGGED)) {
        Fluids.WATER.getSource(false)
    } else {
        super.getFluidState(state)
    }

}