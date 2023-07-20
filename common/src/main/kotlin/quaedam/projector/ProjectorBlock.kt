package quaedam.projector

import net.minecraft.core.BlockPos
import net.minecraft.server.level.ServerLevel
import net.minecraft.util.RandomSource
import net.minecraft.world.InteractionHand
import net.minecraft.world.InteractionResult
import net.minecraft.world.entity.LivingEntity
import net.minecraft.world.entity.player.Player
import net.minecraft.world.item.ItemStack
import net.minecraft.world.level.Level
import net.minecraft.world.level.block.Block
import net.minecraft.world.level.block.EntityBlock
import net.minecraft.world.level.block.state.BlockState
import net.minecraft.world.level.material.MapColor
import net.minecraft.world.phys.BlockHitResult

object ProjectorBlock : Block(Properties.of()
    .jumpFactor(0.8f)
    .lightLevel { 3 }
    .mapColor(MapColor.COLOR_BLACK)
    .randomTicks()
    .strength(4.0f)
    .requiresCorrectToolForDrops()), EntityBlock {

    fun checkUpdate(level: Level, pos: BlockPos) {
        if (!level.isClientSide) {
            (level.getBlockEntity(pos) as ProjectorBlockEntity).checkUpdate()
        }
    }

    @Suppress("OVERRIDE_DEPRECATION")
    override fun use(
        blockState: BlockState,
        level: Level,
        blockPos: BlockPos,
        player: Player,
        interactionHand: InteractionHand,
        blockHitResult: BlockHitResult
    ): InteractionResult {
        checkUpdate(level, blockPos)
        return InteractionResult.SUCCESS
    }

    override fun newBlockEntity(pos: BlockPos, state: BlockState) = ProjectorBlockEntity(pos, state)

    @Suppress("OVERRIDE_DEPRECATION")
    override fun randomTick(
        state: BlockState,
        level: ServerLevel,
        pos: BlockPos,
        random: RandomSource
    ) {
        checkUpdate(level, pos)
        (level.getBlockEntity(pos) as ProjectorBlockEntity).effects.values.forEach { it.randomTick(level, pos) }
    }

    @Suppress("DEPRECATION", "OVERRIDE_DEPRECATION")
    override fun neighborChanged(
        state: BlockState,
        level: Level,
        pos: BlockPos,
        sourceBlock: Block,
        sourcePos: BlockPos,
        notify: Boolean
    ) {
        super.neighborChanged(state, level, pos, sourceBlock, sourcePos, notify)
        checkUpdate(level, pos)
    }

    override fun setPlacedBy(
        level: Level,
        pos: BlockPos,
        state: BlockState,
        placer: LivingEntity?,
        itemStack: ItemStack
    ) {
        super.setPlacedBy(level, pos, state, placer, itemStack)
        checkUpdate(level, pos)
    }

}
