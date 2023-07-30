package quaedam.projection.misc

import net.minecraft.nbt.CompoundTag
import net.minecraft.world.item.BlockItem
import net.minecraft.world.item.Item
import quaedam.Quaedam
import quaedam.config.QuaedamConfig
import quaedam.projection.EntityProjectionBlock
import quaedam.projection.ProjectionEffect
import quaedam.projection.ProjectionEffectType
import quaedam.projection.SimpleProjectionEntity
import quaedam.shell.ProjectionEffectShell
import quaedam.shell.buildProjectionEffectShell
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

}

data class SkylightProjectionEffect(var factor: Double = 2.0) : ProjectionEffect(), ProjectionEffectShell.Provider {

    companion object {
        const val TAG_FACTOR = "Factor"

        val maxFactor get() = QuaedamConfig.current.valuesDouble["projection.skylight.max_factor"] ?: 5.0
    }

    override val type
        get() = SkylightProjection.effect.get()!!

    override fun toNbt(tag: CompoundTag) {
        tag.putDouble(TAG_FACTOR, factor)
    }

    override fun fromNbt(tag: CompoundTag, trusted: Boolean) {
        factor = tag.getDouble(TAG_FACTOR)
        if (!trusted) {
            factor = min(factor, maxFactor)
        }
    }

    override fun createShell() = buildProjectionEffectShell(this) {
        doubleSlider("quaedam.shell.skylight.factor", ::factor, 0.0..maxFactor, 0.1)
    }

}
