package quaedam.projection.misc

import net.minecraft.nbt.CompoundTag
import net.minecraft.world.item.BlockItem
import net.minecraft.world.item.Item
import quaedam.Quaedam
import quaedam.projection.EntityProjectionBlock
import quaedam.projection.ProjectionEffect
import quaedam.projection.ProjectionEffectType
import quaedam.projection.SimpleProjectionEntity

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

    val blockEntity = Quaedam.blockEntities.register(ID) {
        SimpleProjectionEntity.createBlockEntityType(block) { SoundProjectionEffect }
    }!!

}

object SoundProjectionBlock : EntityProjectionBlock<SoundProjectionEffect>(createProperties().lightLevel { 3 }) {

    override val blockEntity = SoundProjection.blockEntity

}

object SoundProjectionEffect : ProjectionEffect() {

    override val type
        get() = SoundProjection.effect.get()!!

    override fun toNbt(tag: CompoundTag) {
    }

    override fun fromNbt(tag: CompoundTag) {
    }

}
