cmake_minimum_required(VERSION 3.16)
project(Lossy LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

set(ANTLR_RUNTIME_DIR ${CMAKE_SOURCE_DIR}/antlr4-runtime)
set(ANTLR_INCLUDE_DIR ${ANTLR_RUNTIME_DIR}/include/antlr4-runtime)

if(APPLE)
    set(ANTLR_LIB ${ANTLR_RUNTIME_DIR}/macos/libantlr4-runtime.a)
elseif(UNIX)
    set(ANTLR_LIB ${ANTLR_RUNTIME_DIR}/linux/libantlr4-runtime.a)
else()
    message(FATAL_ERROR "Unsupported OS. OS must be either APPLE or UNIX")
endif()

set(CMAKE_RUNTIME_OUTPUT_DIRECTORY ${CMAKE_SOURCE_DIR})

include_directories(${ANTLR_INCLUDE_DIR})
link_directories(${ANTLR_LIB_DIR})

add_executable(lossy src/main.cpp src/lzyLexer.cpp src/lzyParser.cpp)

target_link_libraries(lossy ${ANTLR_LIB})

set_target_properties(lossy PROPERTIES
    BUILD_WITH_INSTALL_RPATH TRUE
    INSTALL_RPATH ${ANTLR_LIB_DIR}
)

add_custom_command(TARGET lossy POST_BUILD
   COMMAND ${CMAKE_COMMAND} -E copy_if_different
   ${ANTLR_LIB}
   ${CMAKE_RUNTIME_OUTPUT_DIRECTORY}
)
