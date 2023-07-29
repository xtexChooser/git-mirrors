package quaedam.projection.music

import net.minecraft.nbt.CompoundTag
import net.minecraft.world.item.BlockItem
import net.minecraft.world.item.Item
import quaedam.Quaedam
import quaedam.projection.EntityProjectionBlock
import quaedam.projection.ProjectionEffect
import quaedam.projection.ProjectionEffectType
import quaedam.projection.SimpleProjectionEntity
import quaedam.projection.misc.SoundProjection
import quaedam.shell.ProjectionEffectShell
import quaedam.shell.buildProjectionEffectShell
import kotlin.math.min

object MusicProjection {

    const val ID = "music_projection"
    const val SHORT_ID = "projection"

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

}

object MusicProjectionBlock : EntityProjectionBlock<MusicProjectionEffect>(createProperties().lightLevel { 3 }) {

    override val blockEntity = MusicProjection.blockEntity

}

data class MusicProjectionEffect(var volumeFactor: Float = 1.0f, var multiTracks: Boolean = true) : ProjectionEffect(),
    ProjectionEffectShell.Provider {

    companion object {
        const val TAG_VOLUME_FACTOR = "VolumeFactor"
        const val TAG_MULTI_TRACKS = "MultiTracks"
    }

    override val type
        get() = MusicProjection.effect.get()!!

    override fun toNbt(tag: CompoundTag) {
        tag.putFloat(TAG_VOLUME_FACTOR, volumeFactor)
        tag.putBoolean(TAG_MULTI_TRACKS, multiTracks)
    }

    override fun fromNbt(tag: CompoundTag, trusted: Boolean) {
        volumeFactor = tag.getFloat(TAG_VOLUME_FACTOR)
        multiTracks = tag.getBoolean(TAG_MULTI_TRACKS)
        if (!trusted) {
            volumeFactor = min(volumeFactor, 5.0f)
        }
    }

    override fun createShell() = buildProjectionEffectShell(this) {
        floatSlider("quaedam.shell.music.volume_factor", ::volumeFactor, 0.0f..1.0f, 0.1f)
        boolean("quaedam.shell.music.multi_tracks", ::multiTracks)
    }

}
