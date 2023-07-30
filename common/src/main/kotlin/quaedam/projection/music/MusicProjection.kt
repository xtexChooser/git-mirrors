package quaedam.projection.music

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

object MusicProjection {

    const val ID = "music_projection"
    const val SHORT_ID = "music"

    val block = Quaedam.blocks.register(ID) { MusicProjectionBlock }!!

    val item = Quaedam.items.register(ID) {
        BlockItem(
            MusicProjectionBlock, Item.Properties()
                .`arch$tab`(Quaedam.creativeModeTab)
        )
    }!!

    val effect = Quaedam.projectionEffects.register(SHORT_ID) {
        ProjectionEffectType { MusicProjectionEffect() }
    }!!

    val blockEntity = Quaedam.blockEntities.register(ID) {
        SimpleProjectionEntity.createBlockEntityType(block) { MusicProjectionEffect() }
    }!!

    init {
        SmartInstrument
    }

}

object MusicProjectionBlock : EntityProjectionBlock<MusicProjectionEffect>(createProperties().lightLevel { 3 }) {

    override val blockEntity = MusicProjection.blockEntity

}

data class MusicProjectionEffect(var volumeFactor: Float = 1.0f, var particle: Boolean = true) : ProjectionEffect(),
    ProjectionEffectShell.Provider {

    companion object {
        const val TAG_VOLUME_FACTOR = "VolumeFactor"
        const val TAG_PARTICLE = "Particle"

        val maxVolumeFactor get() = QuaedamConfig.current.valuesFloat["projection.music.max_volume_factor"] ?: 5.0f
        val enforceParticle get() = QuaedamConfig.current.valuesBoolean["projection.music.enforce_particle"]
    }

    override val type
        get() = MusicProjection.effect.get()!!

    override fun toNbt(tag: CompoundTag) {
        tag.putFloat(TAG_VOLUME_FACTOR, volumeFactor)
        tag.putBoolean(TAG_PARTICLE, particle)
    }

    override fun fromNbt(tag: CompoundTag, trusted: Boolean) {
        volumeFactor = tag.getFloat(TAG_VOLUME_FACTOR)
        particle = tag.getBoolean(TAG_PARTICLE)
        if (!trusted) {
            volumeFactor = min(volumeFactor, maxVolumeFactor)
            particle = enforceParticle ?: particle
        }
    }

    override fun createShell() = buildProjectionEffectShell(this) {
        floatSlider("quaedam.shell.music.volume_factor", ::volumeFactor, 0.0f..maxVolumeFactor, 0.1f)
        if (enforceParticle == null) {
            boolean("quaedam.shell.music.particle", ::particle)
        }
    }

}
