package quaedam.projection

import net.minecraft.core.BlockPos
import net.minecraft.nbt.CompoundTag
import net.minecraft.server.level.ServerLevel
import net.minecraft.world.item.BlockItem
import net.minecraft.world.item.Item
import net.minecraft.world.level.block.state.BlockState
import quaedam.Quaedam

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

}

object SkylightProjectionBlock : ProjectionBlock<SkylightProjectionEffect>(createProperties().lightLevel { 3 }) {

    override fun createProjectionEffect(
        level: ServerLevel,
        state: BlockState,
        pos: BlockPos
    ) = SkylightProjectionEffect()

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
