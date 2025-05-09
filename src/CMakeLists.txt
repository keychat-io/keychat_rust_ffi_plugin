# The Flutter tooling requires that developers have CMake 3.10 or later
# installed. You should not increase this version, as doing so will cause
# the plugin to fail to compile for some customers of the plugin.
cmake_minimum_required(VERSION 3.10)

set(PROJECT_NAME "keychat_rust_ffi_plugin")

project(keychat_rust_ffi_plugin_library VERSION 0.0.1 LANGUAGES C)

add_library(${PROJECT_NAME} SHARED
  "keychat_rust_ffi_plugin.c"
)

set_target_properties(${PROJECT_NAME} PROPERTIES
  PUBLIC_HEADER keychat_rust_ffi_plugin.h
  OUTPUT_NAME ${PROJECT_NAME}
)

target_compile_definitions(${PROJECT_NAME} PUBLIC DART_SHARED_LIB)
