package quaedam.forge

import dev.architectury.platform.forge.EventBuses
import net.minecraftforge.fml.common.Mod
import quaedam.Quaedam
import thedarkcolour.kotlinforforge.forge.MOD_BUS

@Mod(Quaedam.ID)
object QuaedamForge {

    init {
        EventBuses.registerModEventBus(Quaedam.ID, MOD_BUS)
        Quaedam.init()
    }

}