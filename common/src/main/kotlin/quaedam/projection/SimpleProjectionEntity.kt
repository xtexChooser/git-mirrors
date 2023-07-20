package quaedam.projection

import dev.architectury.registry.registries.RegistrySupplier
import net.minecraft.core.BlockPos
import net.minecraft.nbt.CompoundTag
import net.minecraft.network.protocol.Packet
import net.minecraft.network.protocol.game.ClientGamePacketListener
import net.minecraft.network.protocol.game.ClientboundBlockEntityDataPacket
import net.minecraft.world.level.block.entity.BlockEntity
import net.minecraft.world.level.block.entity.BlockEntityType
import net.minecraft.world.level.block.state.BlockState

class SimpleProjectionEntity<P : ProjectionEffect>(
    type: BlockEntityType<SimpleProjectionEntity<P>>,
    pos: BlockPos,
    state: BlockState,
    val projection: P
) : BlockEntity(type, pos, state) {

    companion object {
        const val TAG_PROJECTION_EFFECT = "ProjectionEffect"

        fun <P : ProjectionEffect, B: ProjectionBlock<P>> createBlockEntityType(
            block: RegistrySupplier<B>,
            default: () -> P,
        ): BlockEntityType<SimpleProjectionEntity<P>> {
            val type = ValueContainer<BlockEntityType<SimpleProjectionEntity<P>>>()
            type.inner = BlockEntityType.Builder.of({ pos, state ->
                SimpleProjectionEntity(type.inner!!, pos, state, default())
            }, block.get()).build(null)
            return type.inner!!
        }
    }

    data class ValueContainer<E>(var inner: E? = null)

    override fun saveAdditional(tag: CompoundTag) {
        super.saveAdditional(tag)
        tag.put(TAG_PROJECTION_EFFECT, projection.toNbt())
    }

    override fun load(tag: CompoundTag) {
        super.load(tag)
        projection.fromNbt(tag.getCompound(TAG_PROJECTION_EFFECT), true)
    }

    override fun getUpdateTag(): CompoundTag = saveWithoutMetadata()

    override fun getUpdatePacket(): Packet<ClientGamePacketListener> = ClientboundBlockEntityDataPacket.create(this)

}