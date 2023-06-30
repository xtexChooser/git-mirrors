package quaedam.projection

import net.minecraft.core.BlockPos
import net.minecraft.world.entity.LivingEntity
import net.minecraft.world.item.ItemStack
import net.minecraft.world.level.Level
import net.minecraft.world.level.LevelAccessor
import net.minecraft.world.level.block.Block
import net.minecraft.world.level.block.state.BlockState
import net.minecraft.world.level.material.MapColor
import net.minecraft.world.level.storage.loot.LootParams
import quaedam.projector.ProjectorBlockEntity
import quaedam.utils.getChunksNearby

abstract class ProjectionBlock<P : ProjectionEffect>(properties: Properties = createProperties()) : Block(properties),
    ProjectionProvider<P> {

    companion object {
        fun createProperties(): Properties = Properties.of()
            .strength(3.5f)
            .requiresCorrectToolForDrops()
            .mapColor(MapColor.COLOR_GRAY)

        fun findNearbyProjectors(level: Level, pos: BlockPos) = level.getChunksNearby(pos, 1)
            .flatMap {
                it.blockEntities.filter { (k, v) -> v is ProjectorBlockEntity }
                    .keys
                    .filterNotNull()
            }
            .toSet()

    }

    @Suppress("OVERRIDE_DEPRECATION")
    override fun getDrops(blockState: BlockState, builder: LootParams.Builder) = listOf(ItemStack(asItem()))

    override fun setPlacedBy(
        level: Level,
        pos: BlockPos,
        state: BlockState,
        placer: LivingEntity?,
        itemStack: ItemStack
    ) {
        super.setPlacedBy(level, pos, state, placer, itemStack)
        if (!level.isClientSide) {
            findNearbyProjectors(level, pos)
                .forEach { (level.getBlockEntity(it) as ProjectorBlockEntity).checkUpdate() }
        }
    }

    override fun destroy(level: LevelAccessor, pos: BlockPos, state: BlockState) {
        super.destroy(level, pos, state)
        if (level is Level && !level.isClientSide) {
            findNearbyProjectors(level, pos)
                .forEach { (level.getBlockEntity(it) as ProjectorBlockEntity).checkUpdate() }
        }
    }

}
