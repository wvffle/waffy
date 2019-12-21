cmake_minimum_required(VERSION 3.16)
project(waffy C)

set(CMAKE_C_STANDARD 11)

find_package(PkgConfig REQUIRED)
pkg_check_modules(GTK3_WAYLAND REQUIRED gtk+-wayland-3.0)
pkg_check_modules(GTK3_LAYER_SHELL REQUIRED gtk-layer-shell-0)

pkg_search_module(WAYLAND wayland-client)
pkg_search_module(WAYLAND_SCANNER wayland-scanner)
pkg_search_module(WAYLAND_PROTOCOLS wayland-protocols)


include_directories(${GTK3_INCLUDE_DIRS})
link_directories(${GTK3_LIBRARY_DIRS})

#include_directories(${WAYLAND_INCLUDE_DIR})
#link_directories(${WAYLAND_LIBRARIES})

add_definitions(${GTK3_CFLAGS_OTHER})
#add_definitions(${WAYLAND_DEFINITIONS})

add_executable(waffy src/main.c src/utils.h src/utils.c src/find_desktop.c src/find_desktop.h src/desktop_entry.c src/desktop_entry.h)
target_link_libraries(waffy ${GTK3_LAYER_SHELL_LIBRARIES} ${WAYLAND_LIBRARIES} ${GTK3_LIBRARIES})
