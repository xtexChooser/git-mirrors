package quaedam.projection.swarm

import dev.architectury.platform.Platform
import dev.architectury.registry.ReloadListenerRegistry
import net.fabricmc.api.EnvType
import net.minecraft.nbt.CompoundTag
import net.minecraft.resources.ResourceLocation
import net.minecraft.server.packs.PackType
import net.minecraft.server.packs.resources.PreparableReloadListener
import net.minecraft.server.packs.resources.ResourceManager
import net.minecraft.util.profiling.ProfilerFiller
import quaedam.Quaedam
import java.util.concurrent.CompletableFuture
import java.util.concurrent.Executor
import kotlin.math.abs
import kotlin.random.Random
import kotlin.random.nextInt

data class ProjectedPersonShape(
    val scaleX: Float = 1.0f,
    val scaleY: Float = 1.0f,
    val scaleZ: Float = 1.0f,
    val name: String = "",
    val skin: Int = 0,
    val baby: Boolean = false,
) {

    companion object {

        const val KEY_SCALE_X = "ScaleX"
        const val KEY_SCALE_Y = "ScaleY"
        const val KEY_SCALE_Z = "ScaleZ"
        const val KEY_NAME = "Name"
        const val KEY_SKIN = "Skin"
        const val KEY_BABY = "Baby"

        init {
            Names
            Skins
        }

        fun create(seed: Long) = create(Random(seed))

        fun create(rand: Random) = ProjectedPersonShape(
            scaleX = rand.nextInt(0..6 * 4) * 0.025f + 0.7f,
            scaleY = rand.nextInt(0..6 * 4) * 0.025f + 0.7f,
            scaleZ = rand.nextInt(0..2 * 4) * 0.025f + 0.9f,
            name = Names.random(rand),
            skin = Skins.random(rand),
            baby = rand.nextInt(500) == 1
        )

        fun fromTag(tag: CompoundTag) = ProjectedPersonShape(
            scaleX = tag.getFloat(KEY_SCALE_X),
            scaleY = tag.getFloat(KEY_SCALE_Y),
            scaleZ = tag.getFloat(KEY_SCALE_Z),
            name = tag.getString(KEY_NAME),
            skin = tag.getInt(KEY_SKIN),
            baby = tag.getBoolean(KEY_BABY)
        )

    }

    fun toTag() = CompoundTag().apply {
        putFloat(KEY_SCALE_X, scaleX)
        putFloat(KEY_SCALE_Y, scaleY)
        putFloat(KEY_SCALE_Z, scaleZ)
        putString(KEY_NAME, name)
        putInt(KEY_SKIN, skin)
        putBoolean(KEY_BABY, baby)
    }

    object Names {

        val id = ResourceLocation("quaedam", "projected-person-names")

        var names = emptySet<String>()

        init {
            ReloadListenerRegistry.register(PackType.SERVER_DATA, ReloadListener, id)
        }

        fun random(random: Random) = names.random(random)

        private object ReloadListener : PreparableReloadListener {

            @Suppress("NULLABILITY_MISMATCH_BASED_ON_JAVA_ANNOTATIONS")
            override fun reload(
                preparationBarrier: PreparableReloadListener.PreparationBarrier,
                resourceManager: ResourceManager,
                profilerFiller: ProfilerFiller,
                profilerFiller2: ProfilerFiller,
                executor: Executor,
                executor2: Executor
            ): CompletableFuture<Void> = preparationBarrier.wait(null)
                .thenAcceptAsync({
                    names = resourceManager.getResource(id).get().openAsReader().use { it.readLines() }
                        .filterNot { it.isBlank() }
                        .filterNot { it.startsWith("#") }
                        .toSet()
                    Quaedam.logger.info("Loaded ${names.size} unique projected person names")
                }, executor2)

            override fun getName() = "quaedam:projected_person_names"

        }

    }

    object Skins {

        val id = ResourceLocation("quaedam", "skins")

        var skins = emptyList<ResourceLocation>()

        init {
            if (Platform.getEnv() == EnvType.CLIENT) {
                ReloadListenerRegistry.register(PackType.CLIENT_RESOURCES, ReloadListener, id)
            }
        }

        operator fun get(index: Int) = skins[abs(index) % skins.size]
        fun random(random: Random) = random.nextInt()

        private object ReloadListener : PreparableReloadListener {

            @Suppress("NULLABILITY_MISMATCH_BASED_ON_JAVA_ANNOTATIONS")
            override fun reload(
                preparationBarrier: PreparableReloadListener.PreparationBarrier,
                resourceManager: ResourceManager,
                profilerFiller: ProfilerFiller,
                profilerFiller2: ProfilerFiller,
                executor: Executor,
                executor2: Executor
            ): CompletableFuture<Void> = preparationBarrier.wait(null)
                .thenAcceptAsync({
                    val skins = mutableSetOf<ResourceLocation>()
                    skins.addAll(resourceManager.listResources("textures/entity/player/wide") { it.path.endsWith(".png") }.keys)
                    skins.addAll(resourceManager.listResources("textures/entity/projected_person") { it.namespace == "quaedam" }.keys)
                    Skins.skins = skins.toSet().toList().sorted()
                    Quaedam.logger.info("Loaded ${Skins.skins.size} unique projected person skins")
                    Quaedam.logger.debug("Projected person skins ring: {}", skins)
                }, executor2)

            override fun getName() = "quaedam:projected_person_skins"

        }

    }

}