package quaedam.misc.reality

import net.minecraft.core.BlockPos
import net.minecraft.core.Direction
import net.minecraft.world.item.context.BlockPlaceContext
import net.minecraft.world.level.BlockGetter
import net.minecraft.world.level.Level
import net.minecraft.world.level.LevelAccessor
import net.minecraft.world.level.block.*
import net.minecraft.world.level.block.state.BlockState
import net.minecraft.world.level.block.state.StateDefinition
import net.minecraft.world.level.block.state.properties.BlockStateProperties
import net.minecraft.world.level.material.FluidState
import net.minecraft.world.level.material.Fluids
import net.minecraft.world.level.material.MapColor
import net.minecraft.world.phys.shapes.CollisionContext
import net.minecraft.world.phys.shapes.Shapes
import net.minecraft.world.phys.shapes.VoxelShape

object RSBlock : HorizontalDirectionalBlock(
    Properties.of()
        .noOcclusion()
        .strength(3f)
        .requiresCorrectToolForDrops()
        .mapColor(MapColor.COLOR_BLUE)
), EntityBlock, SimpleWaterloggedBlock {

    val shape = Shapes.or(
        box(1.0, 0.0, 1.0, 15.0, 1.0, 15.0),
        box(0.0, 1.0, 0.0, 16.0, 14.0, 16.0),
        box(1.0, 14.0, 1.0, 15.0, 15.0, 15.0),
    )

    init {
        registerDefaultState(
            defaultBlockState()
                .setValue(FACING, Direction.EAST)
                .setValue(BlockStateProperties.WATERLOGGED, false)
        )
    }

    override fun newBlockEntity(pos: BlockPos, state: BlockState) = RSBlockEntity(pos, state)

    override fun createBlockStateDefinition(builder: StateDefinition.Builder<Block, BlockState>) {
        super.createBlockStateDefinition(builder)
        builder.add(FACING, BlockStateProperties.WATERLOGGED)
    }

    override fun getStateForPlacement(context: BlockPlaceContext): BlockState? {
        if (!context.level.getBlockState(context.clickedPos.below()).canOcclude()) return null
        return super.defaultBlockState().setValue(FACING, context.horizontalDirection)
    }

    @Suppress("OVERRIDE_DEPRECATION", "DEPRECATION")
    override fun getShape(state: BlockState, level: BlockGetter, pos: BlockPos, context: CollisionContext) = shape

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

    @Suppress("OVERRIDE_DEPRECATION", "DEPRECATION")
    override fun neighborChanged(
        state: BlockState,
        level: Level,
        pos: BlockPos,
        neighborBlock: Block,
        neighborPos: BlockPos,
        movedByPiston: Boolean
    ) {
        super.neighborChanged(state, level, pos, neighborBlock, neighborPos, movedByPiston)
        if (!level.getBlockState(pos.below()).canOcclude()) {
            level.destroyBlock(pos, true)
        }
    }

}