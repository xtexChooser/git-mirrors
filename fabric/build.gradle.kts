plugins {
    id("com.github.johnrengelman.shadow")
}

architectury {
    platformSetupLoomIde()
    fabric()
}

loom {
    accessWidenerPath.set(project(":common").loom.accessWidenerPath)
}

val common: Configuration by configurations.creating
val shadowCommon: Configuration by configurations.creating
val developmentFabric: Configuration by configurations.getting

configurations {
    compileOnly.configure { extendsFrom(common) }
    runtimeOnly.configure { extendsFrom(common) }
    developmentFabric.extendsFrom(common)
}

dependencies {
    modImplementation("net.fabricmc:fabric-loader:${rootProject.property("fabric_loader_version")}")
    modApi("net.fabricmc.fabric-api:fabric-api:${rootProject.property("fabric_version")}")
    modApi("dev.architectury:architectury-fabric:${rootProject.property("architectury_version")}")
    modImplementation("net.fabricmc:fabric-language-kotlin:${rootProject.property("fabric_kotlin_version")}")

    common(project(":common", "namedElements")) {
        isTransitive = false
    }
    shadowCommon(project(":common", "transformProductionFabric")){
        isTransitive = false
    }
    common(project(":fabric-like", "namedElements")) {
        isTransitive = false
    }
    shadowCommon(project(":fabric-like", "transformProductionFabric")){
        isTransitive = false
    }
}

tasks.processResources {
    inputs.property("version", project.version)

    filesMatching("fabric.mod.json") {
        expand(
            mapOf(
                "version" to project.version,

                "minecraft_version" to rootProject.property("minecraft_version"),
                "architectury_version" to rootProject.property("architectury_version"),
                "fabric_kotlin_version" to rootProject.property("fabric_kotlin_version")
            )
        )
    }
}

tasks.shadowJar {
    exclude("architectury.common.json")
    configurations = listOf(shadowCommon)
    archiveClassifier.set("dev-shadow")
}

tasks.remapJar {
    injectAccessWidener.set(true)
    inputFile.set(tasks.shadowJar.get().archiveFile)
    dependsOn(tasks.shadowJar)
    archiveClassifier.set(null as String?)
}

tasks.jar {
    archiveClassifier.set("dev")
}

tasks.sourcesJar {
    val commonSources = project(":common").tasks.getByName<Jar>("sourcesJar")
    dependsOn(commonSources)
    from(commonSources.archiveFile.map { zipTree(it) })
}

components.getByName("java") {
    this as AdhocComponentWithVariants
    this.withVariantsFromConfiguration(project.configurations["shadowRuntimeElements"]) {
        skip()
    }
}