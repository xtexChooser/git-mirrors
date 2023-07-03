package quaedam.projection.swarm.ai

import net.minecraft.core.GlobalPos
import net.minecraft.server.level.ServerLevel
import net.minecraft.world.entity.Mob
import net.minecraft.world.entity.ai.memory.MemoryModuleType
import net.minecraft.world.entity.ai.memory.MemoryStatus
import net.minecraft.world.entity.ai.sensing.Sensor
import net.minecraft.world.entity.ai.sensing.SensorType
import net.minecraft.world.level.block.BedBlock
import net.minecraft.world.level.block.entity.BedBlockEntity
import net.minecraft.world.level.block.state.properties.BedPart
import quaedam.Quaedam

class BedInChunkSensor : Sensor<Mob>() {

    companion object {

        val sensor = Quaedam.sensors.register("bed_in_chunk") {
            SensorType(::BedInChunkSensor)
        }

    }

    override fun requires() = setOf(MemoryModuleType.NEAREST_BED)

    override fun doTick(level: ServerLevel, entity: Mob) {
        if (entity.tickCount and 0b11111 == 0 && !entity.isSleeping) { // 32gt
            level.getChunkAt(entity.blockPosition()).blockEntities
                .filterValues { it is BedBlockEntity }
                .keys
                .filter { level.getBlockState(it).getValue(BedBlock.PART) == BedPart.HEAD }
                .filter { !level.getBlockState(it).getValue(BedBlock.OCCUPIED) }
                .minByOrNull { it.distManhattan(entity.blockPosition()) }
                ?.also { entity.brain.setMemory(MemoryModuleType.NEAREST_BED, it) }
                ?.also {
                    if (entity.brain.checkMemory(MemoryModuleType.HOME, MemoryStatus.REGISTERED)) {
                        entity.brain.setMemory(MemoryModuleType.HOME, GlobalPos.of(level.dimension(), it))
                    }
                }
        }
    }

}