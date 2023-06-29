pluginManagement {
    repositories {
        maven { url = uri("https://maven.architectury.dev/") }
        maven { url =uri("https://maven.quiltmc.org/repository/release/") }
        maven { url = uri("https://maven.fabricmc.net/") }
        maven { url = uri("https://maven.minecraftforge.net/") }
        gradlePluginPortal()
    }
}

include("common")
include("forge")

rootProject.name = "quaedam"
