register_type({
    id = "cargo",
    name = "Cargo",
    file_name = "target",
    do_clean = function(path)
        print("cleanup cargo ", path)
        error("aa")
    end
})

gradle = {
    name = "Gradle",
    file_name = ".gradle",
    do_clean = function(path)
        print("cleanup gradle ", path)
    end
}
register_type_ref("gradle")
