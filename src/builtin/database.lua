ignore_dir_name(".Trash-1000")
ignore_dir_name(".pnpm-store")
ignore_dir_name("node_modules")

type("cargo", "Cargo", "Cargo.toml")
buildfile("target")
rmrf("target")
safeclean("string.format(\"cd %s; cargo clean\", fs:parent(path))")

type("gradle", "Gradle", "build.gradle")
buildfile("build")
rmrf("build")
rmrf(".gradle")

type("npm", "npm", "package.json")
buildfile("node_modules")
rmrf("node_modules")

type("zig", "Zig", "build.zig")
buildfile("zig-out")
buildfile("zig-cache")
rmrf("zig-out")
rmrf("zig-cache")

type("linux-kernel", "Linux Kernel", "vmlinux")
buildfile("vmlinux")
buildfile("Makefile")
buildfile("modules.builtin")
safeclean("string.format(\"cd %s; make clean\", fs:parent(path))")
