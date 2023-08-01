package quaedam.fabric

import net.fabricmc.api.ModInitializer
import quaedam.Quaedam

object QuaedamFabric: ModInitializer {

    override fun onInitialize() {
        Quaedam.init()
    }

}