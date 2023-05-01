test = {
    name = "test",
    file_name = "target",
    clean = function(path)
        print("cleanup ", path)
    end
}

register_type("test")
