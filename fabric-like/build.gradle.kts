architectury {
    common("fabric", "quilt")
}

loom {
    accessWidenerPath.set(project(":common").loom.accessWidenerPath)
}

dependencies {
    modImplementation("net.fabricmc:fabric-loader:${rootProject.property("fabric_loader_version")}")
    modApi("net.fabricmc.fabric-api:fabric-api:${rootProject.property("fabric_version")}")
    modApi("dev.architectury:architectury-fabric:${rootProject.property("architectury_version")}")

    compileOnly(project(":common", "namedElements")) {
        isTransitive = false
    }
}
