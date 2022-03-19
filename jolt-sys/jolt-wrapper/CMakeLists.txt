cmake_policy(SET CMP0091 NEW)

cmake_minimum_required(VERSION 3.19)
project(JOLT_WRAPPER CXX)

set(CMAKE_CONFIGURATION_TYPES "Debug;Release;Distribution")

# 64 bit architecture
set(CMAKE_VS_PLATFORM_TOOLSET_HOST_ARCHITECTURE "x64")

# Set runtime library
set(CMAKE_MSVC_RUNTIME_LIBRARY "MultiThreaded$<$<CONFIG:Debug>:Debug>")

# Set general compiler flags
set(CMAKE_CXX_FLAGS "/std:c++17 /Zc:__cplusplus /NODEFAULTLIB /GR- /Gm- /Wall /WX /EHsc /nologo /diagnostics:classic /FC /fp:except- /Zc:inline /Zi /DWIN32 /D_WINDOWS /DUNICODE /D_UNICODE")

# Set compiler flags for various configurations
set(CMAKE_CXX_FLAGS_DEBUG "/D_DEBUG /GS /Od /Ob0 /RTC1 /DJPH_PROFILE_ENABLED /DJPH_DEBUG_RENDERER")
set(CMAKE_CXX_FLAGS_RELEASE "/DNDEBUG /GS- /GL /Gy /O2 /Oi /Ot /DJPH_PROFILE_ENABLED /DJPH_DEBUG_RENDERER")
set(CMAKE_CXX_FLAGS_DISTRIBUTION "/DNDEBUG /GS- /GL /Gy /O2 /Oi /Ot")
set(CMAKE_CXX_FLAGS_RELEASEASAN "/DNDEBUG /DJPH_PROFILE_ENABLED /DJPH_DISABLE_TEMP_ALLOCATOR /DJPH_DEBUG_RENDERER -fsanitize=address /Od")
set(CMAKE_CXX_FLAGS_RELEASEUBSAN "/DNDEBUG /DJPH_PROFILE_ENABLED /DJPH_DEBUG_RENDERER -fsanitize=undefined,implicit-conversion")
set(CMAKE_CXX_FLAGS_RELEASECOVERAGE "/DNDEBUG -fprofile-instr-generate -fcoverage-mapping")

set(CMAKE_EXE_LINKER_FLAGS "/machine:x64 /SUBSYSTEM:WINDOWS /ignore:4221 /DEBUG:FASTLINK")
set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} /MP /fp:fast") # Clang doesn't use fast math because it cannot be turned off inside a single compilation unit
set(CMAKE_EXE_LINKER_FLAGS_RELEASE "/INCREMENTAL:NO /LTCG:incremental /OPT:ICF /OPT:REF")
set(CMAKE_STATIC_LINKER_FLAGS_RELEASE "/LTCG")

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
