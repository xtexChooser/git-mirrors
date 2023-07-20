package quaedam.projection

import dev.architectury.registry.registries.DeferredSupplier
import net.minecraft.client.Minecraft
import net.minecraft.core.BlockPos
import net.minecraft.server.level.ServerLevel
import net.minecraft.world.level.Level
import net.minecraft.world.level.block.EntityBlock
import net.minecraft.world.level.block.entity.BlockEntityType
import net.minecraft.world.level.block.state.BlockState
import quaedam.utils.sendBlockUpdated

abstract class EntityProjectionBlock<P : ProjectionEffect>(properties: Properties = createProperties()) :
    ProjectionBlock<P>(properties), EntityBlock {

    companion object {
        fun createProperties(): Properties = ProjectionBlock.createProperties()
    }

    abstract val blockEntity: DeferredSupplier<BlockEntityType<SimpleProjectionEntity<P>>>

    override fun newBlockEntity(pos: BlockPos, state: BlockState) = blockEntity.get().create(pos, state)!!

    @Suppress("UNCHECKED_CAST")
    fun getBlockEntity(level: Level, pos: BlockPos) = (level.getBlockEntity(pos) as SimpleProjectionEntity<P>)

    override fun applyProjectionEffect(level: ServerLevel, state: BlockState, pos: BlockPos) =
        getBlockEntity(level, pos).cloneProjection()

    fun applyChange(level: Level, pos: BlockPos, func: P.() -> Unit) {
        val entity = getBlockEntity(level, pos)
        val projection = entity.projection
        projection.apply(func)
        if (level.isClientSide) {
            check(level == Minecraft.getInstance().player!!.level())
            SimpleProjectionUpdate.send(pos, projection.toNbt())
        } else {
            getBlockEntity(level, pos).sendBlockUpdated()
            sendUpdateToProjectors(level, pos)
        }
    }

}
