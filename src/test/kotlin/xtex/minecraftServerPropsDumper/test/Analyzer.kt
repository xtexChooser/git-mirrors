package xtex.minecraftServerPropsDumper.test

import io.kotest.core.spec.style.StringSpec
import io.kotest.matchers.shouldBe
import xtex.minecraftServerPropsDumper.analyzer.FINGERPRINT_STRINGS
import xtex.minecraftServerPropsDumper.analyzer.KEY_FILTER_PATTERN

class StringMatcherTests : StringSpec({
    "no repeated fingerprints" {
        FINGERPRINT_STRINGS.distinct().toTypedArray() shouldBe FINGERPRINT_STRINGS
    }
})

class KeysMatcherTests : StringSpec({
    "test regex" {
        KEY_FILTER_PATTERN.matches("test") shouldBe true
        KEY_FILTER_PATTERN.matches("test-key") shouldBe true
        KEY_FILTER_PATTERN.matches("test-Key") shouldBe false
        KEY_FILTER_PATTERN.matches("test_key") shouldBe false
        KEY_FILTER_PATTERN.matches("test!key") shouldBe false
        KEY_FILTER_PATTERN.matches("Test String") shouldBe false
    }
})
