package quaedam.projection

import dev.architectury.registry.registries.DeferredSupplier
import net.minecraft.core.BlockPos
import net.minecraft.server.level.ServerLevel
import net.minecraft.world.level.Level
import net.minecraft.world.level.block.EntityBlock
import net.minecraft.world.level.block.entity.BlockEntityType
import net.minecraft.world.level.block.state.BlockState

abstract class EntityProjectionBlock<P : ProjectionEffect>(properties: Properties = createProperties()) :
    ProjectionBlock<P>(properties), EntityBlock {

    companion object {
        fun createProperties(): Properties = ProjectionBlock.createProperties()
    }

    abstract val blockEntity: DeferredSupplier<BlockEntityType<SimpleProjectionEntity<P>>>

    override fun newBlockEntity(pos: BlockPos, state: BlockState) = blockEntity.get().create(pos, state)

    @Suppress("UNCHECKED_CAST")
    fun getProjection(level: Level, pos: BlockPos) = (level.getBlockEntity(pos) as SimpleProjectionEntity<P>).projection

    override fun applyProjectionEffect(level: ServerLevel, state: BlockState, pos: BlockPos) = getProjection(level, pos)

    inline fun applyChange(level: Level, pos: BlockPos, func: P.() -> Unit) {
        getProjection(level, pos).apply(func)
        sendUpdateToProjectors(level, pos)
    }

}
