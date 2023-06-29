package quaedam

import dev.architectury.registry.CreativeTabRegistry
import dev.architectury.registry.registries.DeferredRegister
import dev.architectury.registry.registries.RegistrySupplier
import net.minecraft.core.registries.Registries
import net.minecraft.network.chat.Component
import net.minecraft.world.item.CreativeModeTab
import net.minecraft.world.item.Item
import net.minecraft.world.item.ItemStack
import quaedam.QuaedamExpectPlatform.getConfigDirectory

object Quaedam {
    const val MOD_ID = "quaedam"

    private val createModeTabs = DeferredRegister.create(MOD_ID, Registries.CREATIVE_MODE_TAB)
    val exampleTab: RegistrySupplier<CreativeModeTab> = createModeTabs.register("example_tab") {
        CreativeTabRegistry.create(Component.translatable("category.quaedam")) {
            ItemStack(exampleItem.get())
        }
    }

    private val items = DeferredRegister.create(MOD_ID, Registries.ITEM)
    val exampleItem: RegistrySupplier<Item> = items.register(
        "example_item"
    ) {
        Item(
            Item.Properties().`arch$tab`(exampleTab) // DON'T CALL GET ON exampleTab HERE
        )
    }

    fun init() {
        createModeTabs.register()
        items.register()

        println("CONFIG DIR: ${getConfigDirectory().toAbsolutePath().normalize()}")
    }
}