package quaedam.projection.misc

import net.minecraft.nbt.CompoundTag
import net.minecraft.world.item.BlockItem
import net.minecraft.world.item.Item
import quaedam.Quaedam
import quaedam.projection.EntityProjectionBlock
import quaedam.projection.ProjectionEffect
import quaedam.projection.ProjectionEffectType
import quaedam.projection.SimpleProjectionEntity
import kotlin.math.min

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

    val blockEntity = Quaedam.blockEntities.register(ID) {
        SimpleProjectionEntity.createBlockEntityType(block, ::NoiseProjectionEffect)
    }!!

}

object NoiseProjectionBlock : EntityProjectionBlock<NoiseProjectionEffect>(createProperties().lightLevel { 3 }) {

    override val blockEntity = NoiseProjection.blockEntity

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

    override fun fromNbt(tag: CompoundTag, trusted: Boolean) {
        amount = tag.getInt(TAG_AMOUNT)
        if (!trusted) {
            amount = min(amount, 8)
        }
    }

}
