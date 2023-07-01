package quaedam

import dev.architectury.registry.CreativeTabRegistry
import dev.architectury.registry.registries.DeferredRegister
import dev.architectury.registry.registries.RegistrySupplier
import net.minecraft.core.registries.Registries
import net.minecraft.network.chat.Component
import net.minecraft.world.item.CreativeModeTab
import net.minecraft.world.item.ItemStack
import net.minecraft.world.item.Items
import quaedam.projection.ProjectionEffectType
import quaedam.projection.SkylightProjection
import quaedam.projection.swarm.SwarmProjection
import quaedam.projector.Projector

object Quaedam {

    const val ID = "quaedam"

    val creativeModeTabs = DeferredRegister.create(ID, Registries.CREATIVE_MODE_TAB)!!
    val items = DeferredRegister.create(ID, Registries.ITEM)!!
    val blocks = DeferredRegister.create(ID, Registries.BLOCK)!!
    val blockEntities = DeferredRegister.create(ID, Registries.BLOCK_ENTITY_TYPE)!!
    val projectionEffects = DeferredRegister.create(ID, ProjectionEffectType.registryKey)!!

    val creativeModeTab: RegistrySupplier<CreativeModeTab> = creativeModeTabs.register("quaedam") {
        CreativeTabRegistry.create(Component.translatable("category.quaedam")) {
            ItemStack(Items.TORCH)
        }
    }

    fun init() {
        Projector
        SkylightProjection
        SwarmProjection

        creativeModeTabs.register()
        items.register()
        blocks.register()
        blockEntities.register()
        projectionEffects.register()
    }

}