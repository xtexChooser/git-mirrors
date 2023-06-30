package quaedam.utils

import net.minecraft.world.level.block.Block
import net.minecraft.world.level.block.entity.BlockEntity

fun BlockEntity.sendBlockUpdated() =
    level!!.sendBlockUpdated(blockPos, blockState, blockState, Block.UPDATE_CLIENTS)
