package quaedam.projection.misc

import net.minecraft.core.BlockPos
import net.minecraft.nbt.CompoundTag
import net.minecraft.world.InteractionHand
import net.minecraft.world.InteractionResult
import net.minecraft.world.entity.player.Player
import net.minecraft.world.item.BlockItem
import net.minecraft.world.item.Item
import net.minecraft.world.level.Level
import net.minecraft.world.level.block.state.BlockState
import net.minecraft.world.phys.BlockHitResult
import quaedam.Quaedam
import quaedam.projection.EntityProjectionBlock
import quaedam.projection.ProjectionEffect
import quaedam.projection.ProjectionEffectType
import quaedam.projection.SimpleProjectionEntity
import kotlin.math.min

object SkylightProjection {

    const val ID = "skylight_projection"
    const val SHORT_ID = "skylight"

    val block = Quaedam.blocks.register(ID) { SkylightProjectionBlock }!!

    val item = Quaedam.items.register(ID) {
        BlockItem(
            SkylightProjectionBlock, Item.Properties()
                .`arch$tab`(Quaedam.creativeModeTab)
        )
    }!!

    val effect = Quaedam.projectionEffects.register(SHORT_ID) {
        ProjectionEffectType { SkylightProjectionEffect() }
    }!!

    val blockEntity = Quaedam.blockEntities.register(ID) {
        SimpleProjectionEntity.createBlockEntityType(block, ::SkylightProjectionEffect)
    }!!

}

object SkylightProjectionBlock : EntityProjectionBlock<SkylightProjectionEffect>(createProperties().lightLevel { 3 }) {

    override val blockEntity = SkylightProjection.blockEntity

    override fun use(
        blockState: BlockState,
        level: Level,
        blockPos: BlockPos,
        player: Player,
        interactionHand: InteractionHand,
        blockHitResult: BlockHitResult
    ): InteractionResult {
        if (level.isClientSide) {
            println("update")
            applyChange(level, blockPos) {
                factor -= 0.5
                if (factor < 0.5) factor = 2.0
                println("new factor: $factor")
            }
            return InteractionResult.CONSUME
        }
        return super.use(blockState, level, blockPos, player, interactionHand, blockHitResult)
    }

}

data class SkylightProjectionEffect(var factor: Double = 2.0) : ProjectionEffect() {

    companion object {
        const val TAG_FACTOR = "Factor"
    }

    override val type
        get() = SkylightProjection.effect.get()!!

    override fun toNbt(tag: CompoundTag) {
        tag.putDouble(TAG_FACTOR, factor)
    }

    override fun fromNbt(tag: CompoundTag, trusted: Boolean) {
        factor = tag.getDouble(TAG_FACTOR)
        if (!trusted) {
            factor = min(factor, 5.0)
        }
    }

}
