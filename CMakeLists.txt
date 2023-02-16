cmake_minimum_required(VERSION 3.18.4)
include(settings.cmake)
project(camkes C CXX ASM)

find_package(camkes-tool REQUIRED)

set(CapDLLoaderMaxObjects 20000 CACHE STRING "")
set(KernelRootCNodeSizeBits 17 CACHE STRING "")
if(KernelArchARM)
    ApplyData61ElfLoaderSettings(${KernelPlatform} ${KernelSel4Arch})
endif()

camkes_tool_setup_camkes_build_environment()

#include(projects/hello-world/CMakeLists.txt)
include(projects/hello-rust/CMakeLists.txt)
#include(projects/client-server/CMakeLists.txt)

GenerateCAmkESRootserver()

include(simulation)
if(SIMULATION)
    GenerateSimulateScript()
endif()
