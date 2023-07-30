package quaedam.projection.swarm.ai

import net.minecraft.core.GlobalPos
import net.minecraft.world.entity.ai.behavior.AcquirePoi
import net.minecraft.world.entity.ai.behavior.StrollAroundPoi
import net.minecraft.world.entity.ai.behavior.StrollToPoi
import net.minecraft.world.entity.ai.memory.MemoryModuleType
import net.minecraft.world.entity.ai.village.poi.PoiType
import net.minecraft.world.entity.ai.village.poi.PoiTypes
import net.minecraft.world.level.block.Blocks
import quaedam.Quaedam
import quaedam.projection.music.SmartInstrumentBlock
import java.util.*

object AmusementAI {

    const val ID = "amusement"

    val poiType = Quaedam.poiTypes.register(ID) {
        PoiType(
            setOf(
                Blocks.NOTE_BLOCK,
                SmartInstrumentBlock,
                Blocks.HONEY_BLOCK,
                Blocks.TARGET,
            ).flatMap { it.stateDefinition.possibleStates }.toSet(),
            16, 10
        )
    }!!

    val poiTypes by lazy {
        setOf(
            poiType.key,
            PoiTypes.LIBRARIAN,
            PoiTypes.MEETING,
        )
    }

    val memory = Quaedam.memoryTypes.register(ID) {
        MemoryModuleType(Optional.of(GlobalPos.CODEC))
    }!!

    fun createAcquirePoi() =
        AcquirePoi.create({ it.`is` { key -> key in poiTypes } }, memory.get(), false, Optional.empty())

    fun createStrollToPoi() =
        StrollToPoi.create(memory.get(), 0.4f, 7, 15)

    fun createStrollToPoiBaby() =
        StrollToPoi.create(memory.get(), 0.7f, 5, 10)

    fun createStrollAroundPoi() =
        StrollAroundPoi.create(memory.get(), 0.4f, 10)

    fun createStrollAroundPoiBaby() =
        StrollAroundPoi.create(memory.get(), 0.55f, 8)

}