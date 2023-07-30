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

object SoundProjection {

    const val ID = "sound_projection"
    const val SHORT_ID = "sound"

    val block = Quaedam.blocks.register(ID) { SoundProjectionBlock }!!

    val item = Quaedam.items.register(ID) {
        BlockItem(
            SoundProjectionBlock, Item.Properties()
                .`arch$tab`(Quaedam.creativeModeTab)
        )
    }!!

    val effect = Quaedam.projectionEffects.register(SHORT_ID) {
        ProjectionEffectType { SoundProjectionEffect() }
    }!!

    val blockEntity = Quaedam.blockEntities.register(ID) {
        SimpleProjectionEntity.createBlockEntityType(block) { SoundProjectionEffect() }
    }!!

}

object SoundProjectionBlock : EntityProjectionBlock<SoundProjectionEffect>(createProperties().lightLevel { 3 }) {

    override val blockEntity = SoundProjection.blockEntity

}

data class SoundProjectionEffect(var rate: Int = 60) : ProjectionEffect(), ProjectionEffectShell.Provider {

    companion object {
        const val TAG_RATE = "Rate"

        val maxRate get() = QuaedamConfig.current.valuesInt["projection.sound.max_rate"] ?: 210
    }

    override val type
        get() = SoundProjection.effect.get()!!

    override fun toNbt(tag: CompoundTag) {
        tag.putInt(TAG_RATE, rate)
    }

    override fun fromNbt(tag: CompoundTag, trusted: Boolean) {
        rate = tag.getInt(TAG_RATE)
        if (!trusted) {
            rate = min(rate, maxRate)
        }
    }

    override fun createShell() = buildProjectionEffectShell(this) {
        intSlider("quaedam.shell.sound.rate", ::rate, 0..maxRate)
    }

}
