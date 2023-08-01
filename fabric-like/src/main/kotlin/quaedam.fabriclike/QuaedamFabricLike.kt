package quaedam.fabriclike

import net.fabricmc.api.ModInitializer
import quaedam.Quaedam

object QuaedamFabricLike: ModInitializer {

    override fun onInitialize() {
        Quaedam.init()
    }

}