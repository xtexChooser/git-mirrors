package quaedam.forge

import dev.architectury.platform.forge.EventBuses
import net.minecraftforge.fml.common.Mod
import quaedam.Quaedam
import thedarkcolour.kotlinforforge.forge.MOD_BUS

@Mod(Quaedam.MOD_ID)
object QuaedamForge {
    init {
        // Submit our event bus to let architectury register our content on the right time
        EventBuses.registerModEventBus(Quaedam.MOD_ID, MOD_BUS)
        Quaedam.init()
    }
}