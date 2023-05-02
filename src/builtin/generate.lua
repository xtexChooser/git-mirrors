#!/usr/bin/env lua

local output = io.output("generated.lua")
output:write("-- Build-Clean Builtin Database\n-- GENERATED FILE, DO NOT MODIFY\n")

local current

function flush()
    if (current) then
        local filter = ""
        for i, buildfile in ipairs(current.buildfile) do
            filter = filter .. string.format("        if (not fs:exists(fs:side(path, \"%s\"))) then\
            return false\
        end\
", buildfile)
        end

        local rm = ""
        for path, type in pairs(current.rm) do
            rm = rm .. string.format("        if (fs:exists(fs:side(path, \"%s\"))) then\
            fs:%s(fs:side(path, \"%s\"))\
        end\
", path, type, path)
        end
        output:write(string.format("registry:create({\
    id = \"%s\",\
    name = \"%s\",\
    file_name = \"%s\",\
    filter = function(path)\
%s        return true\
    end,\
    do_clean = function(path)\
%s    end\
})\
", current.id, current.name, current.file, filter, rm))
    end
end

function type(id, name, file)
    flush()
    current = {
        id = id,
        name = name,
        file = file,
        buildfile = {},
        rm = {}
    }
end

function buildfile(name)
    table.insert(current.buildfile, name)
end

function rmrf(name)
    current.rm[name] = "rmrf"
end
function rmd(name)
    current.rm[name] = "rmd"
end
function rm(name)
    current.rm[name] = "rm"
end

require("database")

flush()
io.close(output)
