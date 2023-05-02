registry:create({
    id = "cargo",
    name = "Cargo",
    file_name = "Cargo.toml",
    filter = function(path)
        return fs:exists(fs:side(path, "target"))
    end,
    do_clean = function(path)
        fs:rmrf(fs:side(path, "target"))
    end
})

registry:create({
    id = "gradle",
    name = "Gradle",
    file_name = ".gradle",
    do_clean = function(path)
        print("cleanup gradle ", path)
    end
})
