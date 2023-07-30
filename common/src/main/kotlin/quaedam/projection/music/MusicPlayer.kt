package quaedam.projection.music

import dev.architectury.utils.GameInstance
import net.minecraft.client.resources.sounds.SimpleSoundInstance
import net.minecraft.client.resources.sounds.SoundInstance
import net.minecraft.core.BlockPos
import net.minecraft.core.Holder
import net.minecraft.core.particles.ParticleTypes
import net.minecraft.nbt.CompoundTag
import net.minecraft.sounds.SoundEvent
import net.minecraft.sounds.SoundSource
import net.minecraft.util.RandomSource
import net.minecraft.world.level.Level
import net.minecraft.world.level.block.NoteBlock
import net.minecraft.world.level.block.entity.SkullBlockEntity
import net.minecraft.world.level.block.state.properties.BlockStateProperties
import quaedam.projector.Projector
import kotlin.random.Random

class MusicPlayer(val seed: Long, val level: Level, val pos: BlockPos, val startedAt: Long = level.gameTime) {

    companion object {
        const val TAG_SEED = "Seed"
        const val TAG_STARTED_AT = "StartedAt"
    }

    constructor(tag: CompoundTag, level: Level, pos: BlockPos) : this(
        tag.getLong(TAG_SEED),
        level,
        pos,
        tag.getLong(TAG_STARTED_AT)
    )

    var notes = Composer(
        noteRandom = Random(seed),
        rhythmRandom = Random(startedAt / 20),
        instrument = level.getBlockState(pos).getValue(BlockStateProperties.NOTEBLOCK_INSTRUMENT)
    ).composeMusic().toMutableList()
    val totalTime = notes.sumOf { it.time }.toLong()
    var remainingTime = totalTime
    val isEnd get() = remainingTime <= 0 || notes.isEmpty()
    var noteTime = 0

    init {
        val currentRemaining = totalTime - (level.gameTime - startedAt)
        while (remainingTime > currentRemaining) {
            // seek to current position
            remainingTime -= fetchNote().time
        }
    }

    private fun fetchNote() = notes.removeFirst()

    fun tick() {
        if (isEnd)
            return
        if (noteTime <= 0) {
            // start new note
            val note = fetchNote()
            remainingTime -= note.time
            noteTime = note.time
            if (level.isClientSide) {
                // play note
                val projections = Projector.findNearbyProjections(level, pos, MusicProjection.effect.get())
                val volume = 3.0f * projections.maxOf { it.volumeFactor } * note.volume
                val particle = projections.any { it.particle }
                val instrument = level.getBlockState(pos).getValue(BlockStateProperties.NOTEBLOCK_INSTRUMENT)
                val pitch = if (instrument.isTunable) {
                    NoteBlock.getPitchFromNote(note.note)
                } else {
                    1.0f
                }

                val holder = if (instrument.hasCustomSound()) {
                    val entity = level.getBlockEntity(pos.below())
                    (entity as? SkullBlockEntity)?.noteBlockSound?.let {
                        Holder.direct(SoundEvent.createVariableRangeEvent(it))
                    }
                } else {
                    null
                } ?: instrument.soundEvent

                if (particle) {
                    level.addParticle(
                        ParticleTypes.NOTE,
                        pos.x.toDouble() + 0.5,
                        pos.y.toDouble() + 1.2,
                        pos.z.toDouble() + 0.5,
                        note.time.toDouble() / 24.0,
                        0.0,
                        0.0
                    )
                }

                val instance = SimpleSoundInstance(
                    holder.value().location,
                    SoundSource.RECORDS,
                    volume,
                    pitch,
                    RandomSource.create(level.random.nextLong()),
                    false, 0, SoundInstance.Attenuation.LINEAR,
                    pos.x.toDouble() + 0.5,
                    pos.y.toDouble() + 0.5,
                    pos.z.toDouble() + 0.5,
                    false
                )
                GameInstance.getClient().soundManager.play(instance)
            }
        }
        noteTime--
    }

    fun toTag() = CompoundTag().apply {
        putLong(TAG_SEED, seed)
        putLong(TAG_STARTED_AT, startedAt)
    }

}