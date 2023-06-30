package quaedam.projection

import net.minecraft.core.BlockPos
import net.minecraft.core.Registry
import net.minecraft.core.registries.BuiltInRegistries
import net.minecraft.nbt.CompoundTag
import net.minecraft.resources.ResourceKey
import net.minecraft.resources.ResourceLocation
import net.minecraft.server.level.ServerLevel
import net.minecraft.util.RandomSource
import net.minecraft.world.level.block.state.BlockState

abstract class ProjectionEffect {

    abstract val type: ProjectionEffectType<*>

    abstract fun toNbt(tag: CompoundTag)

    abstract fun fromNbt(tag: CompoundTag)

    fun toNbt() = CompoundTag().apply { toNbt(this) }

    override fun equals(other: Any?) = other === this

    override fun hashCode() = type.hashCode()

    fun activated(level: ServerLevel, projectorPos: BlockPos) {
    }

    fun deactivated(level: ServerLevel, projectorPos: BlockPos) {
    }

    fun randomTick(level: ServerLevel, projectorPos: BlockPos, random: RandomSource) {
    }

}

data class ProjectionEffectType<T : ProjectionEffect>(val constructor: () -> T) {

    companion object {

        val registryKey: ResourceKey<Registry<ProjectionEffectType<*>>> =
            ResourceKey.createRegistryKey(ResourceLocation("quaedam", "projection_effect"))
        val registry: Registry<ProjectionEffectType<*>> = BuiltInRegistries.registerSimple(registryKey) { null }

    }

    val id by lazy { registry.getResourceKey(this).get().location()!! }

}

interface ProjectionProvider<P : ProjectionEffect> {
    fun createProjectionEffect(level: ServerLevel, state: BlockState, pos: BlockPos): P?
}
