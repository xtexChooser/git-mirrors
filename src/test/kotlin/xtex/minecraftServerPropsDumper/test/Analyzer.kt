package xtex.minecraftServerPropsDumper.test

import io.kotest.core.spec.style.StringSpec
import io.kotest.matchers.shouldBe
import xtex.minecraftServerPropsDumper.analyzer.FINGERPRINT_STRINGS

class StringMatcherTests: StringSpec({
    "no repeated fingerprints" {
        FINGERPRINT_STRINGS.distinct().toTypedArray() shouldBe FINGERPRINT_STRINGS
    }
})
