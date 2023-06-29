package quaedam

import dev.architectury.injectables.annotations.ExpectPlatform
import java.nio.file.Path

object QuaedamExpectPlatform {
    @JvmStatic // Make sure its Jvm Static
    @ExpectPlatform
    fun getConfigDirectory(): Path {
        // Just throw an error, the content should get replaced at runtime.
        throw AssertionError()
    }
}