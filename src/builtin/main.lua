cargo = {
    name = "Cargo",
    file_name = "target",
    clean = function(path)
        print("cleanup cargo ", path)
    end
}
register_type("cargo")

gradle = {
    name = "Gradle",
    file_name = ".gradle",
    clean = function(path)
        print("cleanup gradle ", path)
    end
}
register_type("gradle")
