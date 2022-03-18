cmake_minimum_required(VERSION 3.19)
project(JOLT_WRAPPER CXX)

add_library(jolt-wrapper STATIC
    ${CMAKE_CURRENT_LIST_DIR}/inc/wrapper/jolt.h
    ${CMAKE_CURRENT_LIST_DIR}/src/jolt.cpp
)

# Jolt uses system style includes, kinda evil but whatever.
target_include_directories(jolt-wrapper SYSTEM
    PUBLIC
        ${JOLT_INCLUDE_PATH}/Jolt
)
# Wrapper include files.
target_include_directories(jolt-wrapper
    PUBLIC
        ${CMAKE_CURRENT_LIST_DIR}/inc
)
target_link_libraries(jolt-wrapper
    PUBLIC
        ${JOLT_BINARY_PATH}/Jolt
)
