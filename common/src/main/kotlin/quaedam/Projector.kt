package quaedam

import net.minecraft.core.BlockPos
import net.minecraft.world.InteractionHand
import net.minecraft.world.InteractionResult
import net.minecraft.world.entity.player.Player
import net.minecraft.world.item.BlockItem
import net.minecraft.world.item.Item
import net.minecraft.world.level.Level
import net.minecraft.world.level.block.Block
import net.minecraft.world.level.block.state.BlockState
import net.minecraft.world.level.material.MapColor
import net.minecraft.world.phys.BlockHitResult

object Projector {

    const val ID = "projector"

    val block = Quaedam.blocks.register(ID) { ProjectorBlock }

    val item = Quaedam.items.register(ID) {
        BlockItem(
            ProjectorBlock, Item.Properties()
                .stacksTo(1)
                .`arch$tab`(Quaedam.creativeModeTab)
        )
    }!!

}

object ProjectorBlock : Block(Properties.of()
    .jumpFactor(0.8f)
    .lightLevel { 3 }
    .mapColor(MapColor.COLOR_BLACK)
    .randomTicks()
    .destroyTime(4.0f)
    .requiresCorrectToolForDrops()) {

    override fun use(
        blockState: BlockState,
        level: Level,
        blockPos: BlockPos,
        player: Player,
        interactionHand: InteractionHand,
        blockHitResult: BlockHitResult
    ): InteractionResult {
        return InteractionResult.SUCCESS
    }

}
