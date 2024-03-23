-- Build-Clean Builtin Database
-- GENERATED FILE, DO NOT MODIFY
registry:add_ignore_dir_name(".Trash-1000")
registry:add_ignore_dir_name(".pnpm-store")
registry:add_ignore_dir_name("node_modules")
registry:create({
    id = "cargo",
    name = "Cargo",
    file_name = "Cargo.toml",
    filter = function(path)
        if not fs:exists(fs:side(path, "target")) then
            return false
        end
        return true
    end,
    do_fast_clean = function(path)
        if fs:exists(fs:side(path, "target")) then
            fs:rmrf(fs:side(path, "target"))
        end
    end,
    do_clean = function(path)
        if not os.execute(string.format("cd %s; cargo clean", fs:parent(path))) then
            error("failed to execute fast clean command at " .. path)
        end
    end
})
registry:create({
    id = "gradle-groovy",
    name = "Gradle (Groovy)",
    file_name = "build.gradle",
    filter = function(path)
        if not fs:exists(fs:side(path, "build")) then
            return false
        end
        return true
    end,
    do_fast_clean = function(path)
        if fs:exists(fs:side(path, ".gradle")) then
            fs:rmrf(fs:side(path, ".gradle"))
        end
        if fs:exists(fs:side(path, "build")) then
            fs:rmrf(fs:side(path, "build"))
        end
    end,
})
registry:create({
    id = "gradle-kts",
    name = "Gradle (KTS)",
    file_name = "build.gradle.kts",
    filter = function(path)
        if not fs:exists(fs:side(path, "build")) then
            return false
        end
        return true
    end,
    do_fast_clean = function(path)
        if fs:exists(fs:side(path, ".gradle")) then
            fs:rmrf(fs:side(path, ".gradle"))
        end
        if fs:exists(fs:side(path, "build")) then
            fs:rmrf(fs:side(path, "build"))
        end
    end,
})
registry:create({
    id = "npm",
    name = "npm",
    file_name = "package.json",
    filter = function(path)
        if not fs:exists(fs:side(path, "node_modules")) then
            return false
        end
        return true
    end,
    do_fast_clean = function(path)
        if fs:exists(fs:side(path, "node_modules")) then
            fs:rmrf(fs:side(path, "node_modules"))
        end
    end,
})
registry:create({
    id = "zig",
    name = "Zig",
    file_name = "build.zig",
    filter = function(path)
        if not fs:exists(fs:side(path, "zig-out")) then
            return false
        end
        if not fs:exists(fs:side(path, "zig-cache")) then
            return false
        end
        return true
    end,
    do_fast_clean = function(path)
        if fs:exists(fs:side(path, "zig-cache")) then
            fs:rmrf(fs:side(path, "zig-cache"))
        end
        if fs:exists(fs:side(path, "zig-out")) then
            fs:rmrf(fs:side(path, "zig-out"))
        end
    end,
})
registry:create({
    id = "linux-kernel",
    name = "Linux Kernel",
    file_name = "vmlinux",
    filter = function(path)
        if not fs:exists(fs:side(path, "vmlinux")) then
            return false
        end
        if not fs:exists(fs:side(path, "Makefile")) then
            return false
        end
        if not fs:exists(fs:side(path, "modules.builtin")) then
            return false
        end
        return true
    end,
    do_fast_clean = function(path)
    end,
    do_clean = function(path)
        if not os.execute(string.format("cd %s; make clean", fs:parent(path))) then
            error("failed to execute fast clean command at " .. path)
        end
    end
})
