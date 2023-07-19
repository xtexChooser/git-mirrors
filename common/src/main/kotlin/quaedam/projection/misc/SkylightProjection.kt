package quaedam.projection.misc

import net.minecraft.nbt.CompoundTag
import net.minecraft.world.item.BlockItem
import net.minecraft.world.item.Item
import quaedam.Quaedam
import quaedam.projection.EntityProjectionBlock
import quaedam.projection.ProjectionEffect
import quaedam.projection.ProjectionEffectType
import quaedam.projection.SimpleProjectionEntity

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

    override fun fromNbt(tag: CompoundTag) {
        factor = tag.getDouble(TAG_FACTOR)
    }

}
