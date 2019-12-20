cmake_minimum_required(VERSION 3.16)
project(waffy C)

set(CMAKE_C_STANDARD 11)

find_package(PkgConfig REQUIRED)
pkg_check_modules(GTK3 REQUIRED gtk+-3.0)

include_directories(${GTK3_INCLUDE_DIRS})
link_directories(${GTK3_LIBRARY_DIRS})

add_definitions(${GTK3_CFLAGS_OTHER})

add_executable(waffy src/main.c src/utils.h src/utils.c src/find_desktop.c src/find_desktop.h src/desktop_entry.c src/desktop_entry.h)
target_link_libraries(waffy ${GTK3_LIBRARIES})
