package quaedam.projection

import net.minecraft.core.BlockPos
import net.minecraft.nbt.CompoundTag
import net.minecraft.server.level.ServerLevel
import net.minecraft.world.item.BlockItem
import net.minecraft.world.item.Item
import net.minecraft.world.level.block.state.BlockState
import quaedam.Quaedam

object NoiseProjection {

    const val ID = "noise_projection"
    const val SHORT_ID = "noise"

    val block = Quaedam.blocks.register(ID) { NoiseProjectionBlock }!!

    val item = Quaedam.items.register(ID) {
        BlockItem(
            NoiseProjectionBlock, Item.Properties()
                .`arch$tab`(Quaedam.creativeModeTab)
        )
    }!!

    val effect = Quaedam.projectionEffects.register(SHORT_ID) {
        ProjectionEffectType { NoiseProjectionEffect() }
    }!!

}

object NoiseProjectionBlock : ProjectionBlock<NoiseProjectionEffect>(createProperties().lightLevel { 3 }) {

    override fun createProjectionEffect(
        level: ServerLevel,
        state: BlockState,
        pos: BlockPos
    ) = NoiseProjectionEffect()

}

data class NoiseProjectionEffect(var amount: Int = 5) : ProjectionEffect() {

    companion object {
        const val TAG_AMOUNT = "Amount"
    }

    override val type
        get() = SoundProjection.effect.get()!!

    override fun toNbt(tag: CompoundTag) {
        tag.putInt(TAG_AMOUNT, amount)
    }

    override fun fromNbt(tag: CompoundTag) {
        amount = tag.getInt(TAG_AMOUNT)
    }

}
