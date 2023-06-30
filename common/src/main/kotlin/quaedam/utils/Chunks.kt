package quaedam.utils

import net.minecraft.core.BlockPos
import net.minecraft.core.SectionPos
import net.minecraft.world.level.Level
import net.minecraft.world.level.chunk.LevelChunk

fun Level.getChunksNearby(pos: BlockPos, radius: Int): Set<LevelChunk> {
    val chunkX = SectionPos.blockToSectionCoord(pos.x)
    val chunkZ = SectionPos.blockToSectionCoord(pos.z)
    return (chunkX - radius..chunkX + radius).flatMap { x ->
        (chunkZ - radius..chunkZ + radius).map { z ->
            getChunk(x, z)
        }
    }.toSet()
}
