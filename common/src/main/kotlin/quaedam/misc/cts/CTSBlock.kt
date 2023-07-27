package quaedam.misc.cts

import net.minecraft.core.BlockPos
import net.minecraft.world.level.block.Block
import net.minecraft.world.level.block.EntityBlock
import net.minecraft.world.level.block.state.BlockState

object CTSBlock : Block(
    Properties.of()
        .lightLevel { 2 }
), EntityBlock {

    override fun newBlockEntity(pos: BlockPos, state: BlockState) = CTSBlockEntity(pos, state)

}