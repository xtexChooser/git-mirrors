package quaedam.projection

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
import quaedam.Quaedam

object SkylightProjection {

    const val ID = "skylight_projection"

    val block = Quaedam.blocks.register(ID) { SkylightProjectionBlock }

    val item = Quaedam.items.register(ID) {
        BlockItem(
            SkylightProjectionBlock, Item.Properties()
                .`arch$tab`(Quaedam.creativeModeTab)
        )
    }!!

}

object SkylightProjectionBlock : ProjectionBlock(createProperties().lightLevel { 3 }) {

}
