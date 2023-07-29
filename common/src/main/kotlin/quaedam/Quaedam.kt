package quaedam

import dev.architectury.registry.CreativeTabRegistry
import dev.architectury.registry.registries.DeferredRegister
import dev.architectury.registry.registries.RegistrySupplier
import net.minecraft.core.registries.Registries
import net.minecraft.network.chat.Component
import net.minecraft.resources.ResourceLocation
import net.minecraft.world.item.CreativeModeTab
import net.minecraft.world.item.ItemStack
import org.slf4j.LoggerFactory
import quaedam.config.QuaedamConfig
import quaedam.misc.causality.CausalityAnchor
import quaedam.misc.reality.RealityStabler
import quaedam.projection.ProjectionCommand
import quaedam.projection.ProjectionEffectType
import quaedam.projection.SimpleProjectionUpdate
import quaedam.projection.misc.NoiseProjection
import quaedam.projection.misc.SkylightProjection
import quaedam.projection.misc.SoundProjection
import quaedam.projection.music.MusicProjection
import quaedam.projection.swarm.SwarmProjection
import quaedam.projector.Projector
import quaedam.shell.ProjectionShell

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
    val soundEvents = DeferredRegister.create(ID, Registries.SOUND_EVENT)!!
    val projectionEffects by lazy { DeferredRegister.create(ID, ProjectionEffectType.registryKey)!! }

    val creativeModeTab: RegistrySupplier<CreativeModeTab> = creativeModeTabs.register("quaedam") {
        CreativeTabRegistry.create(Component.translatable("category.quaedam")) {
            ItemStack(Projector.item.get())
        }
    }

    fun init() {
        QuaedamConfig
        Projector
        ProjectionEffectType
        SkylightProjection
        SwarmProjection
        SoundProjection
        NoiseProjection
        MusicProjection
        ProjectionCommand
        SimpleProjectionUpdate
        ProjectionShell
        CausalityAnchor
        RealityStabler

        creativeModeTabs.register()
        items.register()
        blocks.register()
        blockEntities.register()
        entities.register()
        schedules.register()
        memoryTypes.register()
        sensors.register()
        soundEvents.register()
        projectionEffects.register()
    }

    fun resource(path: String) = ResourceLocation(ID, path)

}