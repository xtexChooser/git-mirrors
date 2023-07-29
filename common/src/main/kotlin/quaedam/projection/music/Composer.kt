package quaedam.projection.music

import kotlin.random.Random

object Composer {

    data class Note(val note: Int, val volume: Float, val time: Int)

    fun composeMusic(random: Random) = listOf<Note>(
        Note(0, 1.0f, 3),
        Note(1, 1.0f, 3),
        Note(2, 1.0f, 3),
        Note(3, 1.0f, 3),
        Note(4, 1.0f, 3),
        Note(5, 1.0f, 3),
        Note(6, 1.0f, 3),
        Note(7, 1.0f, 3),
    )

}