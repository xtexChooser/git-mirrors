package quaedam.shell

import net.minecraft.world.InteractionResult
import net.minecraft.world.item.Item
import net.minecraft.world.item.context.UseOnContext
import quaedam.Quaedam
import quaedam.shell.network.ServerboundPSHLockAcquirePacket

object ProjectionShellItem : Item(
    Properties()
        .stacksTo(1)
        .`arch$tab`(Quaedam.creativeModeTab)
) {

    override fun useOn(context: UseOnContext): InteractionResult {
        val block = context.level.getBlockState(context.clickedPos).block
        if (block is ProjectionShellBlock && context.level.isClientSide) {
            ProjectionShell.channel.sendToServer(ServerboundPSHLockAcquirePacket(context.clickedPos))
            return InteractionResult.CONSUME
        }
        return InteractionResult.PASS
    }

}