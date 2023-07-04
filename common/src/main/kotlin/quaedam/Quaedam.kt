package quaedam

import dev.architectury.registry.CreativeTabRegistry
import dev.architectury.registry.registries.DeferredRegister
import dev.architectury.registry.registries.RegistrySupplier
import net.minecraft.core.registries.Registries
import net.minecraft.network.chat.Component
import net.minecraft.world.item.CreativeModeTab
import net.minecraft.world.item.ItemStack
import net.minecraft.world.item.Items
import org.slf4j.LoggerFactory
import quaedam.projection.ProjectionCommand
import quaedam.projection.ProjectionEffectType
import quaedam.projection.SkylightProjection
import quaedam.projection.swarm.SwarmProjection
import quaedam.projector.Projector

object Quaedam {

    const val ID = "quaedam"

    val logger = LoggerFactory.getLogger("Quaedam")

    val creativeModeTabs = DeferredRegister.create(ID, Registries.CREATIVE_MODE_TAB)!!
    val items = DeferredRegister.create(ID, Registries.ITEM)!!
    val blocks = DeferredRegister.create(ID, Registries.BLOCK)!!
    val blockEntities = DeferredRegister.create(ID, Registries.BLOCK_ENTITY_TYPE)!!
    val entities = DeferredRegister.create(ID, Registries.ENTITY_TYPE)!!
    val schedules = DeferredRegister.create(ID, Registries.SCHEDULE)!!
    val memoryTypes = DeferredRegister.create(ID, Registries.MEMORY_MODULE_TYPE)!!
    val sensors = DeferredRegister.create(ID, Registries.SENSOR_TYPE)!!
    val projectionEffects = DeferredRegister.create(ID, ProjectionEffectType.registryKey)!!

    val creativeModeTab: RegistrySupplier<CreativeModeTab> = creativeModeTabs.register("quaedam") {
        CreativeTabRegistry.create(Component.translatable("category.quaedam")) {
            ItemStack(Items.TORCH)
        }
    }

    fun init() {
        Projector
        ProjectionEffectType
        SkylightProjection
        SwarmProjection
        ProjectionCommand

        creativeModeTabs.register()
        items.register()
        blocks.register()
        blockEntities.register()
        entities.register()
        schedules.register()
        memoryTypes.register()
        sensors.register()
        projectionEffects.register()
    }

}