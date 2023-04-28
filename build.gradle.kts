import com.github.jengelman.gradle.plugins.shadow.tasks.ShadowJar

plugins {
    id("java")
    id("com.github.johnrengelman.shadow") version "8.1.1"
}

group = "xtex"
version = "1.0-SNAPSHOT"

repositories {
    mavenCentral()
}

dependencies {
    implementation("org.apache.bcel:bcel:6.7.0")
}

tasks.withType<Jar> {
    manifest {
        attributes("Premain-Class" to "mcms.Premain",
            "Agent-Class" to "mcms.Premain",
            "Can-Retransform-Classes" to "true")
    }
}

java {
    sourceCompatibility = JavaVersion.VERSION_18
    targetCompatibility = JavaVersion.VERSION_18
}

tasks.shadowJar {
    isEnableRelocation = true
    relocationPrefix = "mcms.lib"
    minimize()
    isPreserveFileTimestamps = false
    isReproducibleFileOrder = true
}

tasks.build {
    dependsOn(tasks.shadowJar)
}
