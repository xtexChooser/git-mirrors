import org.jetbrains.kotlin.gradle.tasks.KotlinCompile

plugins {
    kotlin("jvm") version "1.8.0"
    kotlin("kapt") version "1.8.0"
    kotlin("plugin.serialization") version "1.8.0"
    id("com.github.johnrengelman.shadow") version "7.1.2"
    id("application")
}

version = "0.1"
group = "xtex.minecraftServerPropsDumper"

repositories {
    mavenCentral()
}

dependencies {
    implementation("org.jetbrains.kotlin:kotlin-stdlib")
    implementation("org.jetbrains.kotlin:kotlin-reflect")

    implementation("info.picocli:picocli:4.7.0")
    kapt("info.picocli:picocli-codegen:4.7.0")

    implementation("org.slf4j:slf4j-api:2.0.6")
    runtimeOnly("org.slf4j:slf4j-simple:2.0.6")

    implementation("com.squareup.okhttp3:okhttp:5.0.0-alpha.11")
    implementation("com.squareup.okhttp3:okhttp-coroutines:5.0.0-alpha.11")

    implementation("org.jetbrains.kotlinx:kotlinx-serialization-core:1.4.1")
    implementation("org.jetbrains.kotlinx:kotlinx-serialization-json:1.4.1")

    implementation("org.jetbrains.kotlinx:kotlinx-coroutines-core:1.6.4")

    implementation("org.apache.bcel:bcel:6.7.0")

    testImplementation("io.kotest:kotest-runner-junit5:5.5.4")
    testImplementation("io.kotest:kotest-assertions-core:5.5.4")

    implementation("commons-io:commons-io:2.11.0")
    implementation("com.github.miachm.sods:SODS:1.5.2")
}

application {
    mainClass.set("$group.main.Main")
}

java {
    sourceCompatibility = JavaVersion.toVersion("17")
}

tasks.withType<KotlinCompile>().configureEach {
    kotlinOptions {
        jvmTarget = "17"
    }
}

tasks.withType<Test>().configureEach {
    useJUnitPlatform()
}

tasks.named<JavaExec>("run").configure {
    mkdir("run")
    workingDir("run")
}
