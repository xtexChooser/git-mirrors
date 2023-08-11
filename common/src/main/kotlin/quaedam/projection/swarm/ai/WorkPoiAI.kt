package quaedam.projection.swarm.ai

import net.minecraft.core.GlobalPos
import net.minecraft.world.entity.ai.behavior.AcquirePoi
import net.minecraft.world.entity.ai.behavior.StrollAroundPoi
import net.minecraft.world.entity.ai.behavior.StrollToPoi
import net.minecraft.world.entity.ai.memory.MemoryModuleType
import net.minecraft.world.entity.ai.village.poi.PoiTypes
import quaedam.Quaedam
import java.util.*

object WorkPoiAI {

    const val ID = "work"

    val poiTypes by lazy {
        setOf(
            PoiTypes.ARMORER,
            PoiTypes.BUTCHER,
            PoiTypes.CARTOGRAPHER,
            PoiTypes.CLERIC,
            PoiTypes.FARMER,
            PoiTypes.FISHERMAN,
            PoiTypes.FLETCHER,
            PoiTypes.LEATHERWORKER,
            PoiTypes.LIBRARIAN,
            PoiTypes.MASON,
            PoiTypes.SHEPHERD,
            PoiTypes.TOOLSMITH,
            PoiTypes.WEAPONSMITH,
            PoiTypes.LODESTONE,
            PoiTypes.LIGHTNING_ROD,
        )
    }

    val memory = Quaedam.memoryTypes.register(ID) {
        MemoryModuleType(Optional.of(GlobalPos.CODEC))
    }!!

    fun createAcquirePoi() =
        AcquirePoi.create({ it.`is` { key -> key in poiTypes } }, memory.get(), false, Optional.empty())

    fun createStrollToPoi() =
        StrollToPoi.create(memory.get(), 0.4f, 7, 4)

    fun createStrollAroundPoi() =
        StrollAroundPoi.create(memory.get(), 0.4f, 5)

}