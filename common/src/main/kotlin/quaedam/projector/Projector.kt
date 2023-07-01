package quaedam.projector

import net.minecraft.core.BlockPos
import net.minecraft.world.item.BlockItem
import net.minecraft.world.item.Item
import net.minecraft.world.level.Level
import net.minecraft.world.level.block.entity.BlockEntityType
import quaedam.Quaedam
import quaedam.projection.ProjectionEffect
import quaedam.projection.ProjectionEffectType
import quaedam.utils.getChunksNearby

object Projector {

    const val ID = "projector"
    const val EFFECT_RADIUS = 4

    val block = Quaedam.blocks.register(ID) { ProjectorBlock }!!

    val item = Quaedam.items.register(ID) {
        BlockItem(
            ProjectorBlock, Item.Properties()
                .stacksTo(1)
                .`arch$tab`(Quaedam.creativeModeTab)
        )
    }!!

    val blockEntity = Quaedam.blockEntities.register(ID) {
        BlockEntityType.Builder.of(::ProjectorBlockEntity, block.get()).build(null)
    }!!

    fun findNearbyProjectors(level: Level, pos: BlockPos) = level.getChunksNearby(pos, EFFECT_RADIUS)
        .flatMap {
            it.blockEntities.filter { (_, v) -> v is ProjectorBlockEntity }
                .keys
                .filterNotNull()
        }
        .toSet()

    @Suppress("UNCHECKED_CAST")
    fun <T : ProjectionEffect> findNearbyProjections(level: Level, pos: BlockPos, type: ProjectionEffectType<T>) =
        findNearbyProjectors(level, pos)
            .map { level.getBlockEntity(it) as ProjectorBlockEntity }
            .mapNotNull { it.effects[type] as T? }

}
