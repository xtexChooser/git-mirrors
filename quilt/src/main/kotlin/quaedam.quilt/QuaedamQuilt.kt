package quaedam.quilt

import net.fabricmc.api.ModInitializer
import quaedam.Quaedam

object QuaedamQuilt: ModInitializer {

    override fun onInitialize() {
        Quaedam.init()
    }

}