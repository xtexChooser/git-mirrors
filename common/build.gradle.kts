architectury {
    common("forge", "fabric", "quilt")
}

loom {
    accessWidenerPath.set(file("src/main/resources/quaedam.accesswidener"))
}

dependencies {
    modImplementation("net.fabricmc:fabric-loader:${rootProject.property("fabric_loader_version")}")
    modApi("dev.architectury:architectury:${rootProject.property("architectury_version")}")
}
