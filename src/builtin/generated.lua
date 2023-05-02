-- Build-Clean Builtin Database
-- GENERATED FILE, DO NOT MODIFY
registry:create({
    id = "cargo",
    name = "Cargo",
    file_name = "Cargo.toml",
    filter = function(path)
        if (not fs:exists(fs:side(path, "target"))) then
            return false
        end
        return true
    end,
    do_clean = function(path)
        fs:rmrf(fs:side(path, "target"))
    end
})
registry:create({
    id = "gradle",
    name = "Gradle",
    file_name = "build.gradle",
    filter = function(path)
        if (not fs:exists(fs:side(path, "build"))) then
            return false
        end
        return true
    end,
    do_clean = function(path)
        fs:rmrf(fs:side(path, ".gradle"))
        fs:rmrf(fs:side(path, "build"))
    end
})
registry:create({
    id = "npm",
    name = "npm",
    file_name = "package.json",
    filter = function(path)
        if (not fs:exists(fs:side(path, "node_modules"))) then
            return false
        end
        return true
    end,
    do_clean = function(path)
        fs:rmrf(fs:side(path, "node_modules"))
    end
})
