package quaedam

import dev.architectury.registry.CreativeTabRegistry
import dev.architectury.registry.registries.DeferredRegister
import dev.architectury.registry.registries.RegistrySupplier
import net.minecraft.core.registries.Registries
import net.minecraft.network.chat.Component
import net.minecraft.world.item.CreativeModeTab
import net.minecraft.world.item.Item
import net.minecraft.world.item.ItemStack
import net.minecraft.world.item.Items
import quaedam.projection.SkylightProjection

object Quaedam {

    const val ID = "quaedam"

    val creativeModeTabs = DeferredRegister.create(ID, Registries.CREATIVE_MODE_TAB)!!
    val items = DeferredRegister.create(ID, Registries.ITEM)!!
    val blocks = DeferredRegister.create(ID, Registries.BLOCK)!!

    val creativeModeTab: RegistrySupplier<CreativeModeTab> = creativeModeTabs.register("quaedam") {
        CreativeTabRegistry.create(Component.translatable("category.quaedam")) {
            ItemStack(Items.TORCH)
        }
    }

    fun init() {
        Projector
        SkylightProjection
        creativeModeTabs.register()
        items.register()
        blocks.register()
    }

}