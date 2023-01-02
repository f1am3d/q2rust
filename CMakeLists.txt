cmake_minimum_required(VERSION 3.16.0 FATAL_ERROR)

set(CMAKE_SYSTEM_VERSION 10.0 CACHE STRING "" FORCE)

project(quake2 C)

################################################################################
# Set target arch type if empty. Visual studio solution generator provides it.
################################################################################
if(NOT CMAKE_VS_PLATFORM_NAME)
    set(CMAKE_VS_PLATFORM_NAME "x86")
endif()
message("${CMAKE_VS_PLATFORM_NAME} architecture in use")

if(NOT ("${CMAKE_VS_PLATFORM_NAME}" STREQUAL "x86"))
    message(FATAL_ERROR "${CMAKE_VS_PLATFORM_NAME} arch is not supported!")
endif()

################################################################################
# Global configuration types
################################################################################
set(CMAKE_CONFIGURATION_TYPES
    "Debug"
    "DebugAlpha"
    "Release"
    "ReleaseAlpha"
    "Template"
    CACHE STRING "" FORCE
)

################################################################################
# Global compiler options
################################################################################
if(MSVC)
    # remove default flags provided with CMake for MSVC
    set(CMAKE_C_FLAGS "")
    set(CMAKE_C_FLAGS_DEBUG "")
    set(CMAKE_C_FLAGS_DEBUGALPHA "")
    set(CMAKE_C_FLAGS_RELEASE "")
    set(CMAKE_C_FLAGS_RELEASEALPHA "")
    set(CMAKE_C_FLAGS_TEMPLATE "")
endif()

################################################################################
# Global linker options
################################################################################
if(MSVC)
    # remove default flags provided with CMake for MSVC
    set(CMAKE_EXE_LINKER_FLAGS "")
    set(CMAKE_MODULE_LINKER_FLAGS "")
    set(CMAKE_SHARED_LINKER_FLAGS "")
    set(CMAKE_STATIC_LINKER_FLAGS "")
    set(CMAKE_EXE_LINKER_FLAGS_DEBUG "${CMAKE_EXE_LINKER_FLAGS}")
    set(CMAKE_MODULE_LINKER_FLAGS_DEBUG "${CMAKE_MODULE_LINKER_FLAGS}")
    set(CMAKE_SHARED_LINKER_FLAGS_DEBUG "${CMAKE_SHARED_LINKER_FLAGS}")
    set(CMAKE_STATIC_LINKER_FLAGS_DEBUG "${CMAKE_STATIC_LINKER_FLAGS}")
    set(CMAKE_EXE_LINKER_FLAGS_DEBUGALPHA "${CMAKE_EXE_LINKER_FLAGS}")
    set(CMAKE_MODULE_LINKER_FLAGS_DEBUGALPHA "${CMAKE_MODULE_LINKER_FLAGS}")
    set(CMAKE_SHARED_LINKER_FLAGS_DEBUGALPHA "${CMAKE_SHARED_LINKER_FLAGS}")
    set(CMAKE_STATIC_LINKER_FLAGS_DEBUGALPHA "${CMAKE_STATIC_LINKER_FLAGS}")
    set(CMAKE_EXE_LINKER_FLAGS_RELEASE "${CMAKE_EXE_LINKER_FLAGS}")
    set(CMAKE_MODULE_LINKER_FLAGS_RELEASE "${CMAKE_MODULE_LINKER_FLAGS}")
    set(CMAKE_SHARED_LINKER_FLAGS_RELEASE "${CMAKE_SHARED_LINKER_FLAGS}")
    set(CMAKE_STATIC_LINKER_FLAGS_RELEASE "${CMAKE_STATIC_LINKER_FLAGS}")
    set(CMAKE_EXE_LINKER_FLAGS_RELEASEALPHA "${CMAKE_EXE_LINKER_FLAGS}")
    set(CMAKE_MODULE_LINKER_FLAGS_RELEASEALPHA "${CMAKE_MODULE_LINKER_FLAGS}")
    set(CMAKE_SHARED_LINKER_FLAGS_RELEASEALPHA "${CMAKE_SHARED_LINKER_FLAGS}")
    set(CMAKE_STATIC_LINKER_FLAGS_RELEASEALPHA "${CMAKE_STATIC_LINKER_FLAGS}")
    set(CMAKE_EXE_LINKER_FLAGS_TEMPLATE "${CMAKE_EXE_LINKER_FLAGS}")
    set(CMAKE_MODULE_LINKER_FLAGS_TEMPLATE "${CMAKE_MODULE_LINKER_FLAGS}")
    set(CMAKE_SHARED_LINKER_FLAGS_TEMPLATE "${CMAKE_SHARED_LINKER_FLAGS}")
    set(CMAKE_STATIC_LINKER_FLAGS_TEMPLATE "${CMAKE_STATIC_LINKER_FLAGS}")
endif()

################################################################################
# Nuget packages function stub.
################################################################################
function(use_package TARGET PACKAGE VERSION)
    message(WARNING "No implementation of use_package. Create yours. "
                    "Package \"${PACKAGE}\" with version \"${VERSION}\" "
                    "for target \"${TARGET}\" is ignored!")
endfunction()

################################################################################
# Common utils
################################################################################
include(CMake/Utils.cmake)

################################################################################
# Additional Global Settings(add specific info there)
################################################################################
include(CMake/GlobalSettingsInclude.cmake OPTIONAL)

################################################################################
# Use solution folders feature
################################################################################
set_property(GLOBAL PROPERTY USE_FOLDERS ON)

################################################################################
# Sub-projects
################################################################################
add_subdirectory(ctf)
add_subdirectory(game)
add_subdirectory(ref_gl)
add_subdirectory(ref_soft)



























set(PROJECT_NAME quake2)

################################################################################
# Source groups
################################################################################
set(Header_Files
    "../../../Projects/PERSONAL/q2-rust/client/anorms.h"
    "../../../Projects/PERSONAL/q2-rust/client/cdaudio.h"
    "../../../Projects/PERSONAL/q2-rust/client/client.h"
    "../../../Projects/PERSONAL/q2-rust/client/console.h"
    "../../../Projects/PERSONAL/q2-rust/client/input.h"
    "../../../Projects/PERSONAL/q2-rust/client/keys.h"
    "../../../Projects/PERSONAL/q2-rust/client/qmenu.h"
    "../../../Projects/PERSONAL/q2-rust/client/ref.h"
    "../../../Projects/PERSONAL/q2-rust/client/screen.h"
    "../../../Projects/PERSONAL/q2-rust/client/snd_loc.h"
    "../../../Projects/PERSONAL/q2-rust/client/sound.h"
    "../../../Projects/PERSONAL/q2-rust/client/vid.h"
    "../../../Projects/PERSONAL/q2-rust/game/game.h"
    "../../../Projects/PERSONAL/q2-rust/game/q_shared.h"
    "../../../Projects/PERSONAL/q2-rust/qcommon/qcommon.h"
    "../../../Projects/PERSONAL/q2-rust/qcommon/qfiles.h"
    "../../../Projects/PERSONAL/q2-rust/server/server.h"
    "../../../Projects/PERSONAL/q2-rust/win32/conproc.h"
    "../../../Projects/PERSONAL/q2-rust/win32/winquake.h"
)
source_group("Header Files" FILES ${Header_Files})

set(Source_Files
    "../../../Projects/PERSONAL/q2-rust/client/cl_cin.c"
    "../../../Projects/PERSONAL/q2-rust/client/cl_ents.c"
    "../../../Projects/PERSONAL/q2-rust/client/cl_fx.c"
    "../../../Projects/PERSONAL/q2-rust/client/cl_input.c"
    "../../../Projects/PERSONAL/q2-rust/client/cl_inv.c"
    "../../../Projects/PERSONAL/q2-rust/client/cl_main.c"
    "../../../Projects/PERSONAL/q2-rust/client/cl_newfx.c"
    "../../../Projects/PERSONAL/q2-rust/client/cl_parse.c"
    "../../../Projects/PERSONAL/q2-rust/client/cl_pred.c"
    "../../../Projects/PERSONAL/q2-rust/client/cl_scrn.c"
    "../../../Projects/PERSONAL/q2-rust/client/cl_tent.c"
    "../../../Projects/PERSONAL/q2-rust/client/cl_view.c"
    "../../../Projects/PERSONAL/q2-rust/client/console.c"
    "../../../Projects/PERSONAL/q2-rust/client/keys.c"
    "../../../Projects/PERSONAL/q2-rust/client/menu.c"
    "../../../Projects/PERSONAL/q2-rust/client/qmenu.c"
    "../../../Projects/PERSONAL/q2-rust/client/snd_dma.c"
    "../../../Projects/PERSONAL/q2-rust/client/snd_mem.c"
    "../../../Projects/PERSONAL/q2-rust/client/snd_mix.c"
    "../../../Projects/PERSONAL/q2-rust/client/x86.c"
    "../../../Projects/PERSONAL/q2-rust/game/m_flash.c"
    "../../../Projects/PERSONAL/q2-rust/game/q_shared.c"
    "../../../Projects/PERSONAL/q2-rust/qcommon/cmd.c"
    "../../../Projects/PERSONAL/q2-rust/qcommon/cmodel.c"
    "../../../Projects/PERSONAL/q2-rust/qcommon/common.c"
    "../../../Projects/PERSONAL/q2-rust/qcommon/crc.c"
    "../../../Projects/PERSONAL/q2-rust/qcommon/cvar.c"
    "../../../Projects/PERSONAL/q2-rust/qcommon/files.c"
    "../../../Projects/PERSONAL/q2-rust/qcommon/md4.c"
    "../../../Projects/PERSONAL/q2-rust/qcommon/net_chan.c"
    "../../../Projects/PERSONAL/q2-rust/qcommon/pmove.c"
    "../../../Projects/PERSONAL/q2-rust/server/sv_ccmds.c"
    "../../../Projects/PERSONAL/q2-rust/server/sv_ents.c"
    "../../../Projects/PERSONAL/q2-rust/server/sv_game.c"
    "../../../Projects/PERSONAL/q2-rust/server/sv_init.c"
    "../../../Projects/PERSONAL/q2-rust/server/sv_main.c"
    "../../../Projects/PERSONAL/q2-rust/server/sv_send.c"
    "../../../Projects/PERSONAL/q2-rust/server/sv_user.c"
    "../../../Projects/PERSONAL/q2-rust/server/sv_world.c"
    "../../../Projects/PERSONAL/q2-rust/win32/cd_win.c"
    "../../../Projects/PERSONAL/q2-rust/win32/conproc.c"
    "../../../Projects/PERSONAL/q2-rust/win32/in_win.c"
    "../../../Projects/PERSONAL/q2-rust/win32/net_wins.c"
    "../../../Projects/PERSONAL/q2-rust/win32/q_shwin.c"
    "../../../Projects/PERSONAL/q2-rust/win32/snd_win.c"
    "../../../Projects/PERSONAL/q2-rust/win32/sys_win.c"
    "../../../Projects/PERSONAL/q2-rust/win32/vid_dll.c"
    "../../../Projects/PERSONAL/q2-rust/win32/vid_menu.c"
)
source_group("Source Files" FILES ${Source_Files})

set(ALL_FILES
    ${Header_Files}
    ${Source_Files}
)

################################################################################
# Target
################################################################################
add_executable(${PROJECT_NAME} ${ALL_FILES})

use_props(${PROJECT_NAME} "${CMAKE_CONFIGURATION_TYPES}" "${DEFAULT_CXX_PROPS}")
################################################################################
# Target name
################################################################################
set_target_properties(${PROJECT_NAME} PROPERTIES
    TARGET_NAME_DEBUGALPHA   "quake2"
    TARGET_NAME_DEBUG        "quake2"
    TARGET_NAME_RELEASEALPHA "quake2"
    TARGET_NAME_RELEASE      "quake2"
)
################################################################################
# Output directory
################################################################################
set_target_properties(${PROJECT_NAME} PROPERTIES
    OUTPUT_DIRECTORY_DEBUGALPHA   "${CMAKE_CURRENT_SOURCE_DIR}/./DebugAxp"
    OUTPUT_DIRECTORY_DEBUG        "${CMAKE_CURRENT_SOURCE_DIR}/./debug"
    OUTPUT_DIRECTORY_RELEASEALPHA "${CMAKE_CURRENT_SOURCE_DIR}/./ReleaseAXP"
    OUTPUT_DIRECTORY_RELEASE      "${CMAKE_CURRENT_SOURCE_DIR}/./release"
)
################################################################################
# MSVC runtime library
################################################################################
get_property(MSVC_RUNTIME_LIBRARY_DEFAULT TARGET ${PROJECT_NAME} PROPERTY MSVC_RUNTIME_LIBRARY)
string(CONCAT "MSVC_RUNTIME_LIBRARY_STR"
    $<$<CONFIG:DebugAlpha>:
        MultiThreadedDebug
    >
    $<$<CONFIG:Debug>:
        MultiThreadedDebug
    >
    $<$<CONFIG:ReleaseAlpha>:
        MultiThreaded
    >
    $<$<CONFIG:Release>:
        MultiThreaded
    >
    $<$<NOT:$<OR:$<CONFIG:DebugAlpha>,$<CONFIG:Debug>,$<CONFIG:ReleaseAlpha>,$<CONFIG:Release>>>:${MSVC_RUNTIME_LIBRARY_DEFAULT}>
)
set_target_properties(${PROJECT_NAME} PROPERTIES MSVC_RUNTIME_LIBRARY ${MSVC_RUNTIME_LIBRARY_STR})

################################################################################
# Compile definitions
################################################################################
target_compile_definitions(${PROJECT_NAME} PRIVATE
    "$<$<CONFIG:DebugAlpha>:"
        "WIN32;"
        "_DEBUG;"
        "_WINDOWS;"
        "C_ONLY"
    ">"
    "$<$<CONFIG:Debug>:"
        "WIN32;"
        "_DEBUG;"
        "_WINDOWS"
    ">"
    "$<$<CONFIG:ReleaseAlpha>:"
        "WIN32;"
        "NDEBUG;"
        "_WINDOWS;"
        "C_ONLY"
    ">"
    "$<$<CONFIG:Release>:"
        "WIN32;"
        "NDEBUG;"
        "_WINDOWS"
    ">"
)

################################################################################
# Compile and link options
################################################################################
if(MSVC)
    target_compile_options(${PROJECT_NAME} PRIVATE
        $<$<CONFIG:DebugAlpha>:
            /Od;
            /Gy-;
            /W3;
            ${DEFAULT_CXX_DEBUG_INFORMATION_FORMAT};
            /QA21164;
            /MTd;
            /QAieee1
        >
        $<$<CONFIG:Debug>:
            /Od;
            /RTC1;
            /Gy;
            /W3;
            /ZI
        >
        $<$<CONFIG:ReleaseAlpha>:
            /O2;
            /GF;
            /Gy;
            /W3;
            ${DEFAULT_CXX_DEBUG_INFORMATION_FORMAT};
            /QA21164;
            /QAieee1
        >
        $<$<CONFIG:Release>:
            /O2;
            /GF;
            /Gy;
            /W4;
            ${DEFAULT_CXX_DEBUG_INFORMATION_FORMAT}
        >
        $<$<CONFIG:Template>:
            ${DEFAULT_CXX_DEBUG_INFORMATION_FORMAT}
        >
        ${DEFAULT_CXX_EXCEPTION_HANDLING}
    )
    string(CONCAT FILE_CL_OPTIONS
        "$<$<CONFIG:DebugAlpha>:"
            "/QA21164;/MTd;/QAieee1;/QA21164;/MTd;/QAieee1"
        ">"
        "$<$<CONFIG:ReleaseAlpha>:"
            "/QA21164;/QAieee1;/QA21164;/QAieee1"
        ">"
    )
    source_file_compile_options(../../../Projects/PERSONAL/q2-rust/client/cl_cin.c ${FILE_CL_OPTIONS})
    string(CONCAT FILE_CL_OPTIONS
        "$<$<CONFIG:DebugAlpha>:"
            "/QA21164;/MTd;/QAieee1;/QA21164;/MTd;/QAieee1"
        ">"
        "$<$<CONFIG:ReleaseAlpha>:"
            "/QA21164;/QAieee1;/QA21164;/QAieee1"
        ">"
    )
    source_file_compile_options(../../../Projects/PERSONAL/q2-rust/client/cl_ents.c ${FILE_CL_OPTIONS})
    string(CONCAT FILE_CL_OPTIONS
        "$<$<CONFIG:DebugAlpha>:"
            "/QA21164;/MTd;/QAieee1;/QA21164;/MTd;/QAieee1"
        ">"
        "$<$<CONFIG:ReleaseAlpha>:"
            "/QA21164;/QAieee1;/QA21164;/QAieee1"
        ">"
    )
    source_file_compile_options(../../../Projects/PERSONAL/q2-rust/client/cl_fx.c ${FILE_CL_OPTIONS})
    string(CONCAT FILE_CL_OPTIONS
        "$<$<CONFIG:DebugAlpha>:"
            "/QA21164;/MTd;/QAieee1;/QA21164;/MTd;/QAieee1"
        ">"
        "$<$<CONFIG:ReleaseAlpha>:"
            "/QA21164;/QAieee1;/QA21164;/QAieee1"
        ">"
    )
    source_file_compile_options(../../../Projects/PERSONAL/q2-rust/client/cl_input.c ${FILE_CL_OPTIONS})
    string(CONCAT FILE_CL_OPTIONS
        "$<$<CONFIG:DebugAlpha>:"
            "/QA21164;/MTd;/QAieee1;/QA21164;/MTd;/QAieee1"
        ">"
        "$<$<CONFIG:ReleaseAlpha>:"
            "/QA21164;/QAieee1;/QA21164;/QAieee1"
        ">"
    )
    source_file_compile_options(../../../Projects/PERSONAL/q2-rust/client/cl_inv.c ${FILE_CL_OPTIONS})
    string(CONCAT FILE_CL_OPTIONS
        "$<$<CONFIG:DebugAlpha>:"
            "/QA21164;/MTd;/QAieee1;/QA21164;/MTd;/QAieee1"
        ">"
        "$<$<CONFIG:ReleaseAlpha>:"
            "/QA21164;/QAieee1;/QA21164;/QAieee1"
        ">"
    )
    source_file_compile_options(../../../Projects/PERSONAL/q2-rust/client/cl_main.c ${FILE_CL_OPTIONS})
    string(CONCAT FILE_CL_OPTIONS
        "$<$<CONFIG:DebugAlpha>:"
            "/QA21164;/MTd;/QAieee1;/QA21164;/MTd;/QAieee1"
        ">"
        "$<$<CONFIG:ReleaseAlpha>:"
            "/QA21164;/QAieee1;/QA21164;/QAieee1"
        ">"
    )
    source_file_compile_options(../../../Projects/PERSONAL/q2-rust/client/cl_newfx.c ${FILE_CL_OPTIONS})
    string(CONCAT FILE_CL_OPTIONS
        "$<$<CONFIG:DebugAlpha>:"
            "/QA21164;/MTd;/QAieee1;/QA21164;/MTd;/QAieee1"
        ">"
        "$<$<CONFIG:ReleaseAlpha>:"
            "/QA21164;/QAieee1;/QA21164;/QAieee1"
        ">"
    )
    source_file_compile_options(../../../Projects/PERSONAL/q2-rust/client/cl_parse.c ${FILE_CL_OPTIONS})
    string(CONCAT FILE_CL_OPTIONS
        "$<$<CONFIG:DebugAlpha>:"
            "/QA21164;/MTd;/QAieee1;/QA21164;/MTd;/QAieee1"
        ">"
        "$<$<CONFIG:ReleaseAlpha>:"
            "/QA21164;/QAieee1;/QA21164;/QAieee1"
        ">"
    )
    source_file_compile_options(../../../Projects/PERSONAL/q2-rust/client/cl_pred.c ${FILE_CL_OPTIONS})
    string(CONCAT FILE_CL_OPTIONS
        "$<$<CONFIG:DebugAlpha>:"
            "/QA21164;/MTd;/QAieee1;/QA21164;/MTd;/QAieee1"
        ">"
        "$<$<CONFIG:ReleaseAlpha>:"
            "/QA21164;/QAieee1;/QA21164;/QAieee1"
        ">"
    )
    source_file_compile_options(../../../Projects/PERSONAL/q2-rust/client/cl_scrn.c ${FILE_CL_OPTIONS})
    string(CONCAT FILE_CL_OPTIONS
        "$<$<CONFIG:DebugAlpha>:"
            "/QA21164;/MTd;/QAieee1;/QA21164;/MTd;/QAieee1"
        ">"
        "$<$<CONFIG:ReleaseAlpha>:"
            "/QA21164;/QAieee1;/QA21164;/QAieee1"
        ">"
    )
    source_file_compile_options(../../../Projects/PERSONAL/q2-rust/client/cl_tent.c ${FILE_CL_OPTIONS})
    string(CONCAT FILE_CL_OPTIONS
        "$<$<CONFIG:DebugAlpha>:"
            "/QA21164;/MTd;/QAieee1;/QA21164;/MTd;/QAieee1"
        ">"
        "$<$<CONFIG:ReleaseAlpha>:"
            "/QA21164;/QAieee1;/QA21164;/QAieee1"
        ">"
    )
    source_file_compile_options(../../../Projects/PERSONAL/q2-rust/client/cl_view.c ${FILE_CL_OPTIONS})
    string(CONCAT FILE_CL_OPTIONS
        "$<$<CONFIG:DebugAlpha>:"
            "/QA21164;/MTd;/QAieee1;/QA21164;/MTd;/QAieee1"
        ">"
        "$<$<CONFIG:ReleaseAlpha>:"
            "/QA21164;/QAieee1;/QA21164;/QAieee1"
        ">"
    )
    source_file_compile_options(../../../Projects/PERSONAL/q2-rust/client/console.c ${FILE_CL_OPTIONS})
    string(CONCAT FILE_CL_OPTIONS
        "$<$<CONFIG:DebugAlpha>:"
            "/QA21164;/MTd;/QAieee1;/QA21164;/MTd;/QAieee1"
        ">"
        "$<$<CONFIG:ReleaseAlpha>:"
            "/QA21164;/QAieee1;/QA21164;/QAieee1"
        ">"
    )
    source_file_compile_options(../../../Projects/PERSONAL/q2-rust/client/keys.c ${FILE_CL_OPTIONS})
    string(CONCAT FILE_CL_OPTIONS
        "$<$<CONFIG:DebugAlpha>:"
            "/QA21164;/MTd;/QAieee1;/QA21164;/MTd;/QAieee1"
        ">"
        "$<$<CONFIG:ReleaseAlpha>:"
            "/QA21164;/QAieee1;/QA21164;/QAieee1"
        ">"
    )
    source_file_compile_options(../../../Projects/PERSONAL/q2-rust/client/menu.c ${FILE_CL_OPTIONS})
    string(CONCAT FILE_CL_OPTIONS
        "$<$<CONFIG:DebugAlpha>:"
            "/QA21164;/MTd;/QAieee1;/QA21164;/MTd;/QAieee1"
        ">"
        "$<$<CONFIG:ReleaseAlpha>:"
            "/QA21164;/QAieee1;/QA21164;/QAieee1"
        ">"
    )
    source_file_compile_options(../../../Projects/PERSONAL/q2-rust/client/qmenu.c ${FILE_CL_OPTIONS})
    string(CONCAT FILE_CL_OPTIONS
        "$<$<CONFIG:DebugAlpha>:"
            "/QA21164;/MTd;/QAieee1;/QA21164;/MTd;/QAieee1"
        ">"
        "$<$<CONFIG:ReleaseAlpha>:"
            "/QA21164;/QAieee1;/QA21164;/QAieee1"
        ">"
    )
    source_file_compile_options(../../../Projects/PERSONAL/q2-rust/client/snd_dma.c ${FILE_CL_OPTIONS})
    string(CONCAT FILE_CL_OPTIONS
        "$<$<CONFIG:DebugAlpha>:"
            "/QA21164;/MTd;/QAieee1;/QA21164;/MTd;/QAieee1"
        ">"
        "$<$<CONFIG:ReleaseAlpha>:"
            "/QA21164;/QAieee1;/QA21164;/QAieee1"
        ">"
    )
    source_file_compile_options(../../../Projects/PERSONAL/q2-rust/client/snd_mem.c ${FILE_CL_OPTIONS})
    string(CONCAT FILE_CL_OPTIONS
        "$<$<CONFIG:DebugAlpha>:"
            "/QA21164;/MTd;/QAieee1;/QA21164;/MTd;/QAieee1"
        ">"
        "$<$<CONFIG:ReleaseAlpha>:"
            "/QA21164;/QAieee1;/QA21164;/QAieee1"
        ">"
    )
    source_file_compile_options(../../../Projects/PERSONAL/q2-rust/client/snd_mix.c ${FILE_CL_OPTIONS})
    string(CONCAT FILE_CL_OPTIONS
        "$<$<CONFIG:DebugAlpha>:"
            "/QA21164;/MTd;/QAieee1;/QA21164;/MTd;/QAieee1"
        ">"
        "$<$<CONFIG:ReleaseAlpha>:"
            "/QA21164;/QAieee1;/QA21164;/QAieee1"
        ">"
    )
    source_file_compile_options(../../../Projects/PERSONAL/q2-rust/client/x86.c ${FILE_CL_OPTIONS})
    string(CONCAT FILE_CL_OPTIONS
        "$<$<CONFIG:DebugAlpha>:"
            "/QA21164;/MTd;/QAieee1;/QA21164;/MTd;/QAieee1"
        ">"
        "$<$<CONFIG:ReleaseAlpha>:"
            "/QA21164;/QAieee1;/QA21164;/QAieee1"
        ">"
    )
    source_file_compile_options(../../../Projects/PERSONAL/q2-rust/game/m_flash.c ${FILE_CL_OPTIONS})
    string(CONCAT FILE_CL_OPTIONS
        "$<$<CONFIG:DebugAlpha>:"
            "/QA21164;/MTd;/QAieee1;/QA21164;/MTd;/QAieee1"
        ">"
        "$<$<CONFIG:ReleaseAlpha>:"
            "/QA21164;/QAieee1;/QA21164;/QAieee1"
        ">"
    )
    source_file_compile_options(../../../Projects/PERSONAL/q2-rust/game/q_shared.c ${FILE_CL_OPTIONS})
    string(CONCAT FILE_CL_OPTIONS
        "$<$<CONFIG:DebugAlpha>:"
            "/QA21164;/MTd;/QAieee1;/QA21164;/MTd;/QAieee1"
        ">"
        "$<$<CONFIG:ReleaseAlpha>:"
            "/QA21164;/QAieee1;/QA21164;/QAieee1"
        ">"
    )
    source_file_compile_options(../../../Projects/PERSONAL/q2-rust/qcommon/cmd.c ${FILE_CL_OPTIONS})
    string(CONCAT FILE_CL_OPTIONS
        "$<$<CONFIG:DebugAlpha>:"
            "/QA21164;/MTd;/QAieee1;/QA21164;/MTd;/QAieee1"
        ">"
        "$<$<CONFIG:ReleaseAlpha>:"
            "/QA21164;/QAieee1;/QA21164;/QAieee1"
        ">"
    )
    source_file_compile_options(../../../Projects/PERSONAL/q2-rust/qcommon/cmodel.c ${FILE_CL_OPTIONS})
    string(CONCAT FILE_CL_OPTIONS
        "$<$<CONFIG:DebugAlpha>:"
            "/QA21164;/MTd;/QAieee1;/QA21164;/MTd;/QAieee1"
        ">"
        "$<$<CONFIG:ReleaseAlpha>:"
            "/QA21164;/QAieee1;/QA21164;/QAieee1"
        ">"
    )
    source_file_compile_options(../../../Projects/PERSONAL/q2-rust/qcommon/common.c ${FILE_CL_OPTIONS})
    string(CONCAT FILE_CL_OPTIONS
        "$<$<CONFIG:DebugAlpha>:"
            "/QA21164;/MTd;/QAieee1;/QA21164;/MTd;/QAieee1"
        ">"
        "$<$<CONFIG:ReleaseAlpha>:"
            "/QA21164;/QAieee1;/QA21164;/QAieee1"
        ">"
    )
    source_file_compile_options(../../../Projects/PERSONAL/q2-rust/qcommon/crc.c ${FILE_CL_OPTIONS})
    string(CONCAT FILE_CL_OPTIONS
        "$<$<CONFIG:DebugAlpha>:"
            "/QA21164;/MTd;/QAieee1;/QA21164;/MTd;/QAieee1"
        ">"
        "$<$<CONFIG:ReleaseAlpha>:"
            "/QA21164;/QAieee1;/QA21164;/QAieee1"
        ">"
    )
    source_file_compile_options(../../../Projects/PERSONAL/q2-rust/qcommon/cvar.c ${FILE_CL_OPTIONS})
    string(CONCAT FILE_CL_OPTIONS
        "$<$<CONFIG:DebugAlpha>:"
            "/QA21164;/MTd;/QAieee1;/QA21164;/MTd;/QAieee1"
        ">"
        "$<$<CONFIG:ReleaseAlpha>:"
            "/QA21164;/QAieee1;/QA21164;/QAieee1"
        ">"
    )
    source_file_compile_options(../../../Projects/PERSONAL/q2-rust/qcommon/files.c ${FILE_CL_OPTIONS})
    string(CONCAT FILE_CL_OPTIONS
        "$<$<CONFIG:DebugAlpha>:"
            "/QA21164;/MTd;/QAieee1;/QA21164;/MTd;/QAieee1"
        ">"
        "$<$<CONFIG:ReleaseAlpha>:"
            "/QA21164;/QAieee1;/QA21164;/QAieee1"
        ">"
    )
    source_file_compile_options(../../../Projects/PERSONAL/q2-rust/qcommon/md4.c ${FILE_CL_OPTIONS})
    string(CONCAT FILE_CL_OPTIONS
        "$<$<CONFIG:DebugAlpha>:"
            "/QA21164;/MTd;/QAieee1;/QA21164;/MTd;/QAieee1"
        ">"
        "$<$<CONFIG:ReleaseAlpha>:"
            "/QA21164;/QAieee1;/QA21164;/QAieee1"
        ">"
    )
    source_file_compile_options(../../../Projects/PERSONAL/q2-rust/qcommon/net_chan.c ${FILE_CL_OPTIONS})
    string(CONCAT FILE_CL_OPTIONS
        "$<$<CONFIG:DebugAlpha>:"
            "/QA21164;/MTd;/QAieee1;/QA21164;/MTd;/QAieee1"
        ">"
        "$<$<CONFIG:ReleaseAlpha>:"
            "/QA21164;/QAieee1;/QA21164;/QAieee1"
        ">"
    )
    source_file_compile_options(../../../Projects/PERSONAL/q2-rust/qcommon/pmove.c ${FILE_CL_OPTIONS})
    string(CONCAT FILE_CL_OPTIONS
        "$<$<CONFIG:DebugAlpha>:"
            "/QA21164;/MTd;/QAieee1;/QA21164;/MTd;/QAieee1"
        ">"
        "$<$<CONFIG:ReleaseAlpha>:"
            "/QA21164;/QAieee1;/QA21164;/QAieee1"
        ">"
    )
    source_file_compile_options(../../../Projects/PERSONAL/q2-rust/server/sv_ccmds.c ${FILE_CL_OPTIONS})
    string(CONCAT FILE_CL_OPTIONS
        "$<$<CONFIG:DebugAlpha>:"
            "/QA21164;/MTd;/QAieee1;/QA21164;/MTd;/QAieee1"
        ">"
        "$<$<CONFIG:ReleaseAlpha>:"
            "/QA21164;/QAieee1;/QA21164;/QAieee1"
        ">"
    )
    source_file_compile_options(../../../Projects/PERSONAL/q2-rust/server/sv_ents.c ${FILE_CL_OPTIONS})
    string(CONCAT FILE_CL_OPTIONS
        "$<$<CONFIG:DebugAlpha>:"
            "/QA21164;/MTd;/QAieee1;/QA21164;/MTd;/QAieee1"
        ">"
        "$<$<CONFIG:ReleaseAlpha>:"
            "/QA21164;/QAieee1;/QA21164;/QAieee1"
        ">"
    )
    source_file_compile_options(../../../Projects/PERSONAL/q2-rust/server/sv_game.c ${FILE_CL_OPTIONS})
    string(CONCAT FILE_CL_OPTIONS
        "$<$<CONFIG:DebugAlpha>:"
            "/QA21164;/MTd;/QAieee1;/QA21164;/MTd;/QAieee1"
        ">"
        "$<$<CONFIG:ReleaseAlpha>:"
            "/QA21164;/QAieee1;/QA21164;/QAieee1"
        ">"
    )
    source_file_compile_options(../../../Projects/PERSONAL/q2-rust/server/sv_init.c ${FILE_CL_OPTIONS})
    string(CONCAT FILE_CL_OPTIONS
        "$<$<CONFIG:DebugAlpha>:"
            "/QA21164;/MTd;/QAieee1;/QA21164;/MTd;/QAieee1"
        ">"
        "$<$<CONFIG:ReleaseAlpha>:"
            "/QA21164;/QAieee1;/QA21164;/QAieee1"
        ">"
    )
    source_file_compile_options(../../../Projects/PERSONAL/q2-rust/server/sv_main.c ${FILE_CL_OPTIONS})
    string(CONCAT FILE_CL_OPTIONS
        "$<$<CONFIG:DebugAlpha>:"
            "/QA21164;/MTd;/QAieee1;/QA21164;/MTd;/QAieee1"
        ">"
        "$<$<CONFIG:ReleaseAlpha>:"
            "/QA21164;/QAieee1;/QA21164;/QAieee1"
        ">"
    )
    source_file_compile_options(../../../Projects/PERSONAL/q2-rust/server/sv_send.c ${FILE_CL_OPTIONS})
    string(CONCAT FILE_CL_OPTIONS
        "$<$<CONFIG:DebugAlpha>:"
            "/QA21164;/MTd;/QAieee1;/QA21164;/MTd;/QAieee1"
        ">"
        "$<$<CONFIG:ReleaseAlpha>:"
            "/QA21164;/QAieee1;/QA21164;/QAieee1"
        ">"
    )
    source_file_compile_options(../../../Projects/PERSONAL/q2-rust/server/sv_user.c ${FILE_CL_OPTIONS})
    string(CONCAT FILE_CL_OPTIONS
        "$<$<CONFIG:DebugAlpha>:"
            "/QA21164;/MTd;/QAieee1;/QA21164;/MTd;/QAieee1"
        ">"
        "$<$<CONFIG:ReleaseAlpha>:"
            "/QA21164;/QAieee1;/QA21164;/QAieee1"
        ">"
    )
    source_file_compile_options(../../../Projects/PERSONAL/q2-rust/server/sv_world.c ${FILE_CL_OPTIONS})
    string(CONCAT FILE_CL_OPTIONS
        "$<$<CONFIG:DebugAlpha>:"
            "/QA21164;/MTd;/QAieee1;/QA21164;/MTd;/QAieee1"
        ">"
        "$<$<CONFIG:ReleaseAlpha>:"
            "/QA21164;/QAieee1;/QA21164;/QAieee1"
        ">"
    )
    source_file_compile_options(../../../Projects/PERSONAL/q2-rust/win32/cd_win.c ${FILE_CL_OPTIONS})
    string(CONCAT FILE_CL_OPTIONS
        "$<$<CONFIG:DebugAlpha>:"
            "/QA21164;/MTd;/QAieee1;/QA21164;/MTd;/QAieee1"
        ">"
        "$<$<CONFIG:ReleaseAlpha>:"
            "/QA21164;/QAieee1;/QA21164;/QAieee1"
        ">"
    )
    source_file_compile_options(../../../Projects/PERSONAL/q2-rust/win32/conproc.c ${FILE_CL_OPTIONS})
    string(CONCAT FILE_CL_OPTIONS
        "$<$<CONFIG:DebugAlpha>:"
            "/QA21164;/MTd;/QAieee1;/QA21164;/MTd;/QAieee1"
        ">"
        "$<$<CONFIG:ReleaseAlpha>:"
            "/QA21164;/QAieee1;/QA21164;/QAieee1"
        ">"
    )
    source_file_compile_options(../../../Projects/PERSONAL/q2-rust/win32/in_win.c ${FILE_CL_OPTIONS})
    string(CONCAT FILE_CL_OPTIONS
        "$<$<CONFIG:DebugAlpha>:"
            "/QA21164;/MTd;/QAieee1;/QA21164;/MTd;/QAieee1"
        ">"
        "$<$<CONFIG:ReleaseAlpha>:"
            "/QA21164;/QAieee1;/QA21164;/QAieee1"
        ">"
    )
    source_file_compile_options(../../../Projects/PERSONAL/q2-rust/win32/net_wins.c ${FILE_CL_OPTIONS})
    string(CONCAT FILE_CL_OPTIONS
        "$<$<CONFIG:DebugAlpha>:"
            "/QA21164;/MTd;/QAieee1;/QA21164;/MTd;/QAieee1"
        ">"
        "$<$<CONFIG:ReleaseAlpha>:"
            "/QA21164;/QAieee1;/QA21164;/QAieee1"
        ">"
    )
    source_file_compile_options(../../../Projects/PERSONAL/q2-rust/win32/q_shwin.c ${FILE_CL_OPTIONS})
    string(CONCAT FILE_CL_OPTIONS
        "$<$<CONFIG:DebugAlpha>:"
            "/QA21164;/MTd;/QAieee1;/QA21164;/MTd;/QAieee1"
        ">"
        "$<$<CONFIG:ReleaseAlpha>:"
            "/QA21164;/QAieee1;/QA21164;/QAieee1"
        ">"
    )
    source_file_compile_options(../../../Projects/PERSONAL/q2-rust/win32/snd_win.c ${FILE_CL_OPTIONS})
    string(CONCAT FILE_CL_OPTIONS
        "$<$<CONFIG:DebugAlpha>:"
            "/QA21164;/MTd;/QAieee1;/QA21164;/MTd;/QAieee1"
        ">"
        "$<$<CONFIG:ReleaseAlpha>:"
            "/QA21164;/QAieee1;/QA21164;/QAieee1"
        ">"
    )
    source_file_compile_options(../../../Projects/PERSONAL/q2-rust/win32/sys_win.c ${FILE_CL_OPTIONS})
    string(CONCAT FILE_CL_OPTIONS
        "$<$<CONFIG:DebugAlpha>:"
            "/QA21164;/MTd;/QAieee1;/QA21164;/MTd;/QAieee1"
        ">"
        "$<$<CONFIG:ReleaseAlpha>:"
            "/QA21164;/QAieee1;/QA21164;/QAieee1"
        ">"
    )
    source_file_compile_options(../../../Projects/PERSONAL/q2-rust/win32/vid_dll.c ${FILE_CL_OPTIONS})
    string(CONCAT FILE_CL_OPTIONS
        "$<$<CONFIG:DebugAlpha>:"
            "/QA21164;/MTd;/QAieee1;/QA21164;/MTd;/QAieee1"
        ">"
        "$<$<CONFIG:ReleaseAlpha>:"
            "/QA21164;/QAieee1;/QA21164;/QAieee1"
        ">"
    )
    source_file_compile_options(../../../Projects/PERSONAL/q2-rust/win32/vid_menu.c ${FILE_CL_OPTIONS})
    target_link_options(${PROJECT_NAME} PRIVATE
        $<$<CONFIG:DebugAlpha>:
            /NOLOGO;
            /DEBUG;
            /SUBSYSTEM:WINDOWS;
            /INCREMENTAL
        >
        $<$<CONFIG:Debug>:
            /NOLOGO;
            /DEBUG;
            /SUBSYSTEM:WINDOWS;
            /INCREMENTAL:NO
        >
        $<$<CONFIG:ReleaseAlpha>:
            /NOLOGO;
            /SUBSYSTEM:WINDOWS;
            /INCREMENTAL:NO
        >
        $<$<CONFIG:Release>:
            /NOLOGO;
            /SUBSYSTEM:WINDOWS;
            /INCREMENTAL:NO
        >
        $<$<CONFIG:Template>:
            /DEBUG:FULL
        >
    )
endif()

################################################################################
# Dependencies
################################################################################
set(ADDITIONAL_LIBRARY_DEPENDENCIES
    "$<$<CONFIG:DebugAlpha>:"
        "winmm;"
        "wsock32"
    ">"
    "$<$<CONFIG:Debug>:"
        "winmm;"
        "wsock32"
    ">"
    "$<$<CONFIG:ReleaseAlpha>:"
        "winmm;"
        "wsock32"
    ">"
    "$<$<CONFIG:Release>:"
        "winmm;"
        "wsock32"
    ">"
)
target_link_libraries(${PROJECT_NAME} PRIVATE "${ADDITIONAL_LIBRARY_DEPENDENCIES}")

