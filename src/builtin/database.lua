type("cargo", "Cargo", "Cargo.toml")
buildfile("target")
rmrf("target")

type("gradle", "Gradle", "build.gradle")
buildfile("build")
rmrf("build")
rmrf(".gradle")

type("npm", "npm", "package.json")
buildfile("node_modules")
rmrf("node_modules")
