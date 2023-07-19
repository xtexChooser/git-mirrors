package quaedam.projection.misc

import net.minecraft.core.BlockPos
import net.minecraft.nbt.CompoundTag
import net.minecraft.server.level.ServerLevel
import net.minecraft.world.item.BlockItem
import net.minecraft.world.item.Item
import net.minecraft.world.level.block.state.BlockState
import quaedam.Quaedam
import quaedam.projection.ProjectionBlock
import quaedam.projection.ProjectionEffect
import quaedam.projection.ProjectionEffectType

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
        ProjectionEffectType { SoundProjectionEffect }
    }!!

}

object SoundProjectionBlock : ProjectionBlock<SoundProjectionEffect>(createProperties().lightLevel { 3 }) {

    override fun createProjectionEffect(
        level: ServerLevel,
        state: BlockState,
        pos: BlockPos
    ) = SoundProjectionEffect

}

object SoundProjectionEffect : ProjectionEffect() {

    override val type
        get() = SoundProjection.effect.get()!!

    override fun toNbt(tag: CompoundTag) {
    }

    override fun fromNbt(tag: CompoundTag) {
    }

}
