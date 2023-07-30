package quaedam.projection.misc

import dev.architectury.event.events.client.ClientTickEvent
import dev.architectury.platform.Platform
import net.fabricmc.api.EnvType
import net.minecraft.client.Minecraft
import net.minecraft.client.resources.sounds.SimpleSoundInstance
import net.minecraft.client.resources.sounds.SoundInstance
import net.minecraft.nbt.CompoundTag
import net.minecraft.sounds.SoundEvent
import net.minecraft.sounds.SoundSource
import net.minecraft.util.RandomSource
import net.minecraft.world.item.BlockItem
import net.minecraft.world.item.Item
import quaedam.Quaedam
import quaedam.config.QuaedamConfig
import quaedam.projection.EntityProjectionBlock
import quaedam.projection.ProjectionEffect
import quaedam.projection.ProjectionEffectType
import quaedam.projection.SimpleProjectionEntity
import quaedam.projector.Projector
import quaedam.shell.ProjectionEffectShell
import quaedam.shell.buildProjectionEffectShell
import kotlin.math.min

object NoiseProjection {

    const val ID = "noise_projection"
    const val SHORT_ID = "noise"

    val block = Quaedam.blocks.register(ID) { NoiseProjectionBlock }!!

    val item = Quaedam.items.register(ID) {
        BlockItem(
            NoiseProjectionBlock, Item.Properties()
                .`arch$tab`(Quaedam.creativeModeTab)
        )
    }!!

    val effect = Quaedam.projectionEffects.register(SHORT_ID) {
        ProjectionEffectType { NoiseProjectionEffect() }
    }!!

    val blockEntity = Quaedam.blockEntities.register(ID) {
        SimpleProjectionEntity.createBlockEntityType(block, ::NoiseProjectionEffect)
    }!!

    const val SOUND_NOISE_ID = "quaedam.projection.noise"
    val soundEvent = Quaedam.soundEvents.register(SOUND_NOISE_ID) {
        SoundEvent.createVariableRangeEvent(Quaedam.resource(SOUND_NOISE_ID))
    }!!

    init {
        if (Platform.getEnv() == EnvType.CLIENT) {
            ClientTickEvent.CLIENT_POST.register { game ->
                val player = game.player ?: return@register
                val random = (game.level ?: return@register).random
                val projections = Projector.findNearbyProjections(player.level(), player.blockPosition(), effect.get())
                if (projections.isNotEmpty()) {
                    val rate = projections.maxOf { it.rate }
                    val amount = min(projections.sumOf { it.amount }, 12)
                    if (amount != 0 && random.nextInt(1000 / rate) == 1) {
                        for (i in 0 until random.nextInt(amount)) {
                            // play random noise
                            playRandomNoise(random, game)
                        }
                    }
                }
            }
        }
    }

    private fun playRandomNoise(random: RandomSource, game: Minecraft) {
        val volumeFactor = random.nextInt(100)
        val sound = SimpleSoundInstance(
            soundEvent.get().location,
            SoundSource.AMBIENT,
            when (volumeFactor) {
                in 0..8 -> random.nextFloat() * 0.65f
                in 10..15 -> random.nextFloat() * 0.5f + 0.5f
                in 21..50 -> random.nextFloat() * 0.3f
                else -> random.nextFloat() * 0.2f
            },
            random.nextFloat() + 0.4f,
            RandomSource.create(random.nextLong()),
            false,
            0,
            SoundInstance.Attenuation.NONE,
            random.nextFloat() * 28.0 - 14,
            random.nextFloat() * 12.0 - 2,
            random.nextFloat() * 28.0 - 14,
            true
        )
        game.soundManager.playDelayed(sound, random.nextInt(3))
    }

}

object NoiseProjectionBlock : EntityProjectionBlock<NoiseProjectionEffect>(createProperties().lightLevel { 3 }) {

    override val blockEntity = NoiseProjection.blockEntity

}

data class NoiseProjectionEffect(var rate: Int = 250, var amount: Int = 3) : ProjectionEffect(),
    ProjectionEffectShell.Provider {

    companion object {
        const val TAG_RATE = "Rate"
        const val TAG_AMOUNT = "Amount"

        val maxAmount get() = QuaedamConfig.current.valuesInt["projection.noise.max_amount"] ?: 8
        val maxRate get() = QuaedamConfig.current.valuesInt["projection.noise.max_rate"] ?: 300
    }

    override val type
        get() = NoiseProjection.effect.get()!!

    override fun toNbt(tag: CompoundTag) {
        tag.putInt(TAG_RATE, rate)
        tag.putInt(TAG_AMOUNT, amount)
    }

    override fun fromNbt(tag: CompoundTag, trusted: Boolean) {
        rate = tag.getInt(TAG_RATE)
        amount = tag.getInt(TAG_AMOUNT)
        if (!trusted) {
            amount = min(amount, maxAmount)
            rate = min(rate, maxRate)
        }
    }

    override fun createShell() = buildProjectionEffectShell(this) {
        intSlider("quaedam.shell.noise.rate", ::rate, 0..maxRate step 5)
        intSlider("quaedam.shell.noise.amount", ::amount, 0..maxAmount)
    }

}
