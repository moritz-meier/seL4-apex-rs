cmake_minimum_required(VERSION 3.18.4)

set(project_dir "${CMAKE_CURRENT_LIST_DIR}")
file(GLOB project_modules "${project_dir}/projects/*")
list(
    APPEND
        CMAKE_MODULE_PATH
        "${project_dir}/kernel"
        "${project_dir}/tools/seL4/cmake-tool/helpers/"
        "${project_dir}/tools/seL4/elfloader-tool/"
        "${project_modules}"
)

set(SEL4_CONFIG_DEFAULT_ADVANCED ON)

set(LibSel4FunctionAttributes public CACHE STRING "" FORCE)

include(application_settings)

if(ARM_HYP)
    set(KernelArmHypervisorSupport ON CACHE BOOL "" FORCE)
endif()

correct_platform_strings()

find_package(seL4 REQUIRED)
sel4_configure_platform_settings()

if(MCS)
	set(KernelIsMCS ON CACHE BOOL "" FORCE)
endif()

if(SIMULATION)
    ApplyCommonSimulationSettings(${KernelSel4Arch})
endif()

ApplyCommonReleaseVerificationSettings(${RELEASE} FALSE)
