package quaedam.projection.swarm.ai

import net.minecraft.core.BlockPos
import net.minecraft.server.level.ServerLevel
import net.minecraft.world.entity.LivingEntity
import net.minecraft.world.entity.ai.memory.MemoryModuleType
import net.minecraft.world.entity.ai.sensing.Sensor
import net.minecraft.world.entity.ai.sensing.SensorType
import net.minecraft.world.level.block.entity.BaseContainerBlockEntity
import quaedam.Quaedam
import quaedam.utils.getChunksNearby
import java.util.*

class NearestVisibleContainer : Sensor<LivingEntity>() {

    companion object {

        const val ID = "nearest_visible_container"

        val sensor = Quaedam.sensors.register(ID) {
            SensorType(::NearestVisibleContainer)
        }!!

        val memory = Quaedam.memoryTypes.register(ID) {
            MemoryModuleType(Optional.of(BlockPos.CODEC))
        }!!

    }

    override fun requires() = setOf(memory.get())

    override fun doTick(level: ServerLevel, entity: LivingEntity) {
        if (entity.tickCount and 0b11111 == 0) { // 32gt
            val pos = level.getChunksNearby(entity.blockPosition(), 1)
                .flatMap { it.blockEntities.filterValues { be -> be is BaseContainerBlockEntity }.keys }
                .minByOrNull { it.distManhattan(entity.blockPosition()) }
            entity.brain.setMemory(memory.get(), pos)
        }
    }

}