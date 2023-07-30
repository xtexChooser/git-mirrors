package quaedam.projection.music

import kotlin.math.abs
import kotlin.random.Random
import kotlin.random.nextInt

/**
 * The composer for music.
 * rhythmRandom is used for a better rhythm sync between different instruments.
 */
class Composer(val noteRandom: Random, val rhythmRandom: Random) {

    data class Note(val note: Int, val volume: Float, val time: Int)

    val baseTime = arrayOf(5, 5, 3, 3, 4, 4, 2, 2, 10).random(rhythmRandom)
    val baseNote = noteRandom.nextInt(5..19)

    fun composeMusic() = decorate(
        (0..rhythmRandom.nextInt(5)).flatMap { composeSection() }
    )

    fun decorate(notes: List<Note>) = notes.map {
        if (noteRandom.nextInt(4) == 0) {
            doDecorate(it)
        } else {
            it
        }
    }

    fun doDecorate(note: Note): Note {
        var noteVal = note.note
        if (noteRandom.nextInt(4) == 0) {
            if (noteRandom.nextBoolean()) {
                noteVal += 1
            } else {
                noteVal -= 1
            }
        }
        var volume = note.volume
        if (noteRandom.nextInt(4) == 0) {
            volume *= noteRandom.nextFloat() * 0.8f + 0.6f
        }
        return Note(noteVal, volume, note.time)
    }

    fun composeSection(depth: Int = 0): List<Note> {
        if (depth < 3 && rhythmRandom.nextBoolean()) {
            val notes = (0..rhythmRandom.nextInt(3 - depth)).flatMap { composeSection(depth + 1) }
            if (depth == 2) {
                return (0..rhythmRandom.nextInt(3)).flatMap { notes }
            } else {
                return notes
            }
        } else {
            var notePointer = baseNote + noteRandom.nextInt(-3..3)
            var direction = -1
            var directionCounter = 0
            return (0..rhythmRandom.nextInt(4..16)).map {
                if (directionCounter == 0) {
                    // start new direction
                    directionCounter = rhythmRandom.nextInt(2..6)
                    direction = if (directionCounter % 2 == 0) {
                        rhythmRandom.nextInt(-2..2)
                    } else {
                        noteRandom.nextInt(-3..3)
                    }
                }
                notePointer = abs(notePointer + direction) % 25
                directionCounter--
                Note(notePointer, 1.0f, baseTime + rhythmRandom.nextInt(-1..1))
            }
        }
    }

}