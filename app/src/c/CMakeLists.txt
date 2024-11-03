cmake_minimum_required(VERSION 3.30)

set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

project(libyzt VERSION 1.0.0)

add_library(yztsec STATIC
    src/sec/sec.cpp
    )
target_include_directories(yztsec PUBLIC include)

install(TARGETS yztsec
    DESTINATION ${CMAKE_INSTALL_PREFIX}
    )
