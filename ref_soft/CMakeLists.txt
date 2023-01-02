set(PROJECT_NAME ref_soft)

################################################################################
# Source groups
################################################################################
set(Header_Files
    "../../../../Projects/PERSONAL/q2-rust/client/ref.h"
    "../../../../Projects/PERSONAL/q2-rust/game/q_shared.h"
    "../../../../Projects/PERSONAL/q2-rust/qcommon/qcommon.h"
    "../../../../Projects/PERSONAL/q2-rust/qcommon/qfiles.h"
    "../../../../Projects/PERSONAL/q2-rust/ref_soft/adivtab.h"
    "../../../../Projects/PERSONAL/q2-rust/ref_soft/anorms.h"
    "../../../../Projects/PERSONAL/q2-rust/ref_soft/r_local.h"
    "../../../../Projects/PERSONAL/q2-rust/ref_soft/r_model.h"
    "../../../../Projects/PERSONAL/q2-rust/ref_soft/rand1k.h"
    "../../../../Projects/PERSONAL/q2-rust/win32/rw_win.h"
    "../../../../Projects/PERSONAL/q2-rust/win32/winquake.h"
)
source_group("Header Files" FILES ${Header_Files})

set(Resource_Files
    "../../../../Projects/PERSONAL/q2-rust/ref_soft/ref_soft.def"
)
source_group("Resource Files" FILES ${Resource_Files})

set(Source_Files
    "../../../../Projects/PERSONAL/q2-rust/game/q_shared.c"
    "../../../../Projects/PERSONAL/q2-rust/ref_soft/r_aclip.c"
#    "../../../../Projects/PERSONAL/q2-rust/ref_soft/r_aclipa.asm"
    "../../../../Projects/PERSONAL/q2-rust/ref_soft/r_alias.c"
    "../../../../Projects/PERSONAL/q2-rust/ref_soft/r_bsp.c"
    "../../../../Projects/PERSONAL/q2-rust/ref_soft/r_draw.c"
#    "../../../../Projects/PERSONAL/q2-rust/ref_soft/r_draw16.asm"
#    "../../../../Projects/PERSONAL/q2-rust/ref_soft/r_drawa.asm"
    "../../../../Projects/PERSONAL/q2-rust/ref_soft/r_edge.c"
#    "../../../../Projects/PERSONAL/q2-rust/ref_soft/r_edgea.asm"
    "../../../../Projects/PERSONAL/q2-rust/ref_soft/r_image.c"
    "../../../../Projects/PERSONAL/q2-rust/ref_soft/r_light.c"
    "../../../../Projects/PERSONAL/q2-rust/ref_soft/r_main.c"
    "../../../../Projects/PERSONAL/q2-rust/ref_soft/r_misc.c"
    "../../../../Projects/PERSONAL/q2-rust/ref_soft/r_model.c"
    "../../../../Projects/PERSONAL/q2-rust/ref_soft/r_part.c"
    "../../../../Projects/PERSONAL/q2-rust/ref_soft/r_poly.c"
    "../../../../Projects/PERSONAL/q2-rust/ref_soft/r_polysa.asm"
    "../../../../Projects/PERSONAL/q2-rust/ref_soft/r_polyse.c"
    "../../../../Projects/PERSONAL/q2-rust/ref_soft/r_rast.c"
    "../../../../Projects/PERSONAL/q2-rust/ref_soft/r_scan.c"
#    "../../../../Projects/PERSONAL/q2-rust/ref_soft/r_scana.asm"
#    "../../../../Projects/PERSONAL/q2-rust/ref_soft/r_spr8.asm"
    "../../../../Projects/PERSONAL/q2-rust/ref_soft/r_sprite.c"
    "../../../../Projects/PERSONAL/q2-rust/ref_soft/r_surf.c"
#    "../../../../Projects/PERSONAL/q2-rust/ref_soft/r_surf8.asm"
#    "../../../../Projects/PERSONAL/q2-rust/ref_soft/r_varsa.asm"
    "../../../../Projects/PERSONAL/q2-rust/win32/q_shwin.c"
    "../../../../Projects/PERSONAL/q2-rust/win32/rw_ddraw.c"
    "../../../../Projects/PERSONAL/q2-rust/win32/rw_dib.c"
    "../../../../Projects/PERSONAL/q2-rust/win32/rw_imp.c"
)
source_group("Source Files" FILES ${Source_Files})

set(ALL_FILES
    ${Header_Files}
    ${Resource_Files}
    ${Source_Files}
)

################################################################################
# Target
################################################################################
add_library(${PROJECT_NAME} SHARED ${ALL_FILES})

use_props(${PROJECT_NAME} "${CMAKE_CONFIGURATION_TYPES}" "${DEFAULT_CXX_PROPS}")
################################################################################
# Target name
################################################################################
set_target_properties(${PROJECT_NAME} PROPERTIES
    TARGET_NAME_DEBUGALPHA   "ref_soft"
    TARGET_NAME_DEBUG        "ref_soft"
    TARGET_NAME_RELEASEALPHA "ref_soft"
    TARGET_NAME_RELEASE      "ref_soft"
)
################################################################################
# Output directory
################################################################################
set_target_properties(${PROJECT_NAME} PROPERTIES
    OUTPUT_DIRECTORY_DEBUGALPHA   "${CMAKE_CURRENT_SOURCE_DIR}/./../DebugAxp"
    OUTPUT_DIRECTORY_DEBUG        "${CMAKE_CURRENT_SOURCE_DIR}/./../debug"
    OUTPUT_DIRECTORY_RELEASEALPHA "${CMAKE_CURRENT_SOURCE_DIR}/./../ReleaseAXP"
    OUTPUT_DIRECTORY_RELEASE      "${CMAKE_CURRENT_SOURCE_DIR}/./../release"
)
set_target_properties(${PROJECT_NAME} PROPERTIES
    ARCHIVE_OUTPUT_DIRECTORY_DEBUGALPHA   "${CMAKE_CURRENT_SOURCE_DIR}/./../DebugAxp"
    ARCHIVE_OUTPUT_DIRECTORY_DEBUG        "${CMAKE_CURRENT_SOURCE_DIR}/./../debug"
    ARCHIVE_OUTPUT_DIRECTORY_RELEASEALPHA "${CMAKE_CURRENT_SOURCE_DIR}/./../ReleaseAXP"
    ARCHIVE_OUTPUT_DIRECTORY_RELEASE      "${CMAKE_CURRENT_SOURCE_DIR}/./../release"
)
set_target_properties(${PROJECT_NAME} PROPERTIES
    ARCHIVE_OUTPUT_NAME_DEBUGALPHA   "ref_soft"
    ARCHIVE_OUTPUT_NAME_DEBUG        "ref_soft"
    ARCHIVE_OUTPUT_NAME_RELEASEALPHA "ref_soft"
    ARCHIVE_OUTPUT_NAME_RELEASE      "ref_soft"
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
        "_DEBUG;"
        "WIN32;"
        "_WINDOWS;"
        "C_ONLY"
    ">"
    "$<$<CONFIG:Debug>:"
        "_DEBUG;"
        "WIN32;"
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
            /Gm;
            /Od;
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
    source_file_compile_options(../../../../Projects/PERSONAL/q2-rust/game/q_shared.c ${FILE_CL_OPTIONS})
    string(CONCAT FILE_CL_OPTIONS
        "$<$<CONFIG:DebugAlpha>:"
            "/QA21164;/MTd;/QAieee1;/QA21164;/MTd;/QAieee1"
        ">"
        "$<$<CONFIG:ReleaseAlpha>:"
            "/QA21164;/QAieee1;/QA21164;/QAieee1"
        ">"
    )
    source_file_compile_options(../../../../Projects/PERSONAL/q2-rust/win32/q_shwin.c ${FILE_CL_OPTIONS})
    string(CONCAT FILE_CL_OPTIONS
        "$<$<CONFIG:DebugAlpha>:"
            "/QA21164;/MTd;/QAieee1;/QA21164;/MTd;/QAieee1"
        ">"
        "$<$<CONFIG:ReleaseAlpha>:"
            "/QA21164;/QAieee1;/QA21164;/QAieee1"
        ">"
    )
    source_file_compile_options(../../../../Projects/PERSONAL/q2-rust/win32/rw_ddraw.c ${FILE_CL_OPTIONS})
    string(CONCAT FILE_CL_OPTIONS
        "$<$<CONFIG:DebugAlpha>:"
            "/QA21164;/MTd;/QAieee1;/QA21164;/MTd;/QAieee1"
        ">"
        "$<$<CONFIG:ReleaseAlpha>:"
            "/QA21164;/QAieee1;/QA21164;/QAieee1"
        ">"
    )
    source_file_compile_options(../../../../Projects/PERSONAL/q2-rust/win32/rw_dib.c ${FILE_CL_OPTIONS})
    string(CONCAT FILE_CL_OPTIONS
        "$<$<CONFIG:DebugAlpha>:"
            "/QA21164;/MTd;/QAieee1;/QA21164;/MTd;/QAieee1"
        ">"
        "$<$<CONFIG:ReleaseAlpha>:"
            "/QA21164;/QAieee1;/QA21164;/QAieee1"
        ">"
    )
    source_file_compile_options(../../../../Projects/PERSONAL/q2-rust/win32/rw_imp.c ${FILE_CL_OPTIONS})
    string(CONCAT FILE_CL_OPTIONS
        "$<$<CONFIG:DebugAlpha>:"
            "/QA21164;/MTd;/QAieee1;/QA21164;/MTd;/QAieee1"
        ">"
        "$<$<CONFIG:ReleaseAlpha>:"
            "/QA21164;/QAieee1;/QA21164;/QAieee1"
        ">"
    )
    source_file_compile_options(../../../../Projects/PERSONAL/q2-rust/ref_soft/r_aclip.c ${FILE_CL_OPTIONS})
    string(CONCAT FILE_CL_OPTIONS
        "$<$<CONFIG:DebugAlpha>:"
            "/QA21164;/MTd;/QAieee1;/QA21164;/MTd;/QAieee1"
        ">"
        "$<$<CONFIG:ReleaseAlpha>:"
            "/QA21164;/QAieee1;/QA21164;/QAieee1"
        ">"
    )
    source_file_compile_options(../../../../Projects/PERSONAL/q2-rust/ref_soft/r_alias.c ${FILE_CL_OPTIONS})
    string(CONCAT FILE_CL_OPTIONS
        "$<$<CONFIG:DebugAlpha>:"
            "/QA21164;/MTd;/QAieee1;/QA21164;/MTd;/QAieee1"
        ">"
        "$<$<CONFIG:ReleaseAlpha>:"
            "/QA21164;/QAieee1;/QA21164;/QAieee1"
        ">"
    )
    source_file_compile_options(../../../../Projects/PERSONAL/q2-rust/ref_soft/r_bsp.c ${FILE_CL_OPTIONS})
    string(CONCAT FILE_CL_OPTIONS
        "$<$<CONFIG:DebugAlpha>:"
            "/QA21164;/MTd;/QAieee1;/QA21164;/MTd;/QAieee1"
        ">"
        "$<$<CONFIG:ReleaseAlpha>:"
            "/QA21164;/QAieee1;/QA21164;/QAieee1"
        ">"
    )
    source_file_compile_options(../../../../Projects/PERSONAL/q2-rust/ref_soft/r_draw.c ${FILE_CL_OPTIONS})
    string(CONCAT FILE_CL_OPTIONS
        "$<$<CONFIG:DebugAlpha>:"
            "/QA21164;/MTd;/QAieee1;/QA21164;/MTd;/QAieee1"
        ">"
        "$<$<CONFIG:ReleaseAlpha>:"
            "/QA21164;/QAieee1;/QA21164;/QAieee1"
        ">"
    )
    source_file_compile_options(../../../../Projects/PERSONAL/q2-rust/ref_soft/r_edge.c ${FILE_CL_OPTIONS})
    string(CONCAT FILE_CL_OPTIONS
        "$<$<CONFIG:DebugAlpha>:"
            "/QA21164;/MTd;/QAieee1;/QA21164;/MTd;/QAieee1"
        ">"
        "$<$<CONFIG:ReleaseAlpha>:"
            "/QA21164;/QAieee1;/QA21164;/QAieee1"
        ">"
    )
    source_file_compile_options(../../../../Projects/PERSONAL/q2-rust/ref_soft/r_image.c ${FILE_CL_OPTIONS})
    string(CONCAT FILE_CL_OPTIONS
        "$<$<CONFIG:DebugAlpha>:"
            "/QA21164;/MTd;/QAieee1;/QA21164;/MTd;/QAieee1"
        ">"
        "$<$<CONFIG:ReleaseAlpha>:"
            "/QA21164;/QAieee1;/QA21164;/QAieee1"
        ">"
    )
    source_file_compile_options(../../../../Projects/PERSONAL/q2-rust/ref_soft/r_light.c ${FILE_CL_OPTIONS})
    string(CONCAT FILE_CL_OPTIONS
        "$<$<CONFIG:DebugAlpha>:"
            "/QA21164;/MTd;/QAieee1;/QA21164;/MTd;/QAieee1"
        ">"
        "$<$<CONFIG:ReleaseAlpha>:"
            "/QA21164;/QAieee1;/QA21164;/QAieee1"
        ">"
    )
    source_file_compile_options(../../../../Projects/PERSONAL/q2-rust/ref_soft/r_main.c ${FILE_CL_OPTIONS})
    string(CONCAT FILE_CL_OPTIONS
        "$<$<CONFIG:DebugAlpha>:"
            "/QA21164;/MTd;/QAieee1;/QA21164;/MTd;/QAieee1"
        ">"
        "$<$<CONFIG:ReleaseAlpha>:"
            "/QA21164;/QAieee1;/QA21164;/QAieee1"
        ">"
    )
    source_file_compile_options(../../../../Projects/PERSONAL/q2-rust/ref_soft/r_misc.c ${FILE_CL_OPTIONS})
    string(CONCAT FILE_CL_OPTIONS
        "$<$<CONFIG:DebugAlpha>:"
            "/QA21164;/MTd;/QAieee1;/QA21164;/MTd;/QAieee1"
        ">"
        "$<$<CONFIG:ReleaseAlpha>:"
            "/QA21164;/QAieee1;/QA21164;/QAieee1"
        ">"
    )
    source_file_compile_options(../../../../Projects/PERSONAL/q2-rust/ref_soft/r_model.c ${FILE_CL_OPTIONS})
    string(CONCAT FILE_CL_OPTIONS
        "$<$<CONFIG:DebugAlpha>:"
            "/QA21164;/MTd;/QAieee1;/QA21164;/MTd;/QAieee1"
        ">"
        "$<$<CONFIG:ReleaseAlpha>:"
            "/QA21164;/QAieee1;/QA21164;/QAieee1"
        ">"
    )
    source_file_compile_options(../../../../Projects/PERSONAL/q2-rust/ref_soft/r_part.c ${FILE_CL_OPTIONS})
    string(CONCAT FILE_CL_OPTIONS
        "$<$<CONFIG:DebugAlpha>:"
            "/QA21164;/MTd;/QAieee1;/QA21164;/MTd;/QAieee1"
        ">"
        "$<$<CONFIG:ReleaseAlpha>:"
            "/QA21164;/QAieee1;/QA21164;/QAieee1"
        ">"
    )
    source_file_compile_options(../../../../Projects/PERSONAL/q2-rust/ref_soft/r_poly.c ${FILE_CL_OPTIONS})
    string(CONCAT FILE_CL_OPTIONS
        "$<$<CONFIG:DebugAlpha>:"
            "/QA21164;/MTd;/QAieee1;/QA21164;/MTd;/QAieee1"
        ">"
        "$<$<CONFIG:ReleaseAlpha>:"
            "/QA21164;/QAieee1;/QA21164;/QAieee1"
        ">"
    )
    source_file_compile_options(../../../../Projects/PERSONAL/q2-rust/ref_soft/r_polyse.c ${FILE_CL_OPTIONS})
    string(CONCAT FILE_CL_OPTIONS
        "$<$<CONFIG:DebugAlpha>:"
            "/QA21164;/MTd;/QAieee1;/QA21164;/MTd;/QAieee1"
        ">"
        "$<$<CONFIG:ReleaseAlpha>:"
            "/QA21164;/QAieee1;/QA21164;/QAieee1"
        ">"
    )
    source_file_compile_options(../../../../Projects/PERSONAL/q2-rust/ref_soft/r_rast.c ${FILE_CL_OPTIONS})
    string(CONCAT FILE_CL_OPTIONS
        "$<$<CONFIG:DebugAlpha>:"
            "/QA21164;/MTd;/QAieee1;/QA21164;/MTd;/QAieee1"
        ">"
        "$<$<CONFIG:ReleaseAlpha>:"
            "/QA21164;/QAieee1;/QA21164;/QAieee1"
        ">"
    )
    source_file_compile_options(../../../../Projects/PERSONAL/q2-rust/ref_soft/r_scan.c ${FILE_CL_OPTIONS})
    string(CONCAT FILE_CL_OPTIONS
        "$<$<CONFIG:DebugAlpha>:"
            "/QA21164;/MTd;/QAieee1;/QA21164;/MTd;/QAieee1"
        ">"
        "$<$<CONFIG:ReleaseAlpha>:"
            "/QA21164;/QAieee1;/QA21164;/QAieee1"
        ">"
    )
    source_file_compile_options(../../../../Projects/PERSONAL/q2-rust/ref_soft/r_sprite.c ${FILE_CL_OPTIONS})
    string(CONCAT FILE_CL_OPTIONS
        "$<$<CONFIG:DebugAlpha>:"
            "/QA21164;/MTd;/QAieee1;/QA21164;/MTd;/QAieee1"
        ">"
        "$<$<CONFIG:ReleaseAlpha>:"
            "/QA21164;/QAieee1;/QA21164;/QAieee1"
        ">"
    )
    source_file_compile_options(../../../../Projects/PERSONAL/q2-rust/ref_soft/r_surf.c ${FILE_CL_OPTIONS})
    target_link_options(${PROJECT_NAME} PRIVATE
        $<$<CONFIG:DebugAlpha>:
            /NODEFAULTLIB:libc;
            /NOLOGO;
            /DEBUG;
            /SUBSYSTEM:WINDOWS;
            /INCREMENTAL
        >
        $<$<CONFIG:Debug>:
            /NODEFAULTLIB:libc;
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
        "winmm"
    ">"
    "$<$<CONFIG:Debug>:"
        "winmm"
    ">"
    "$<$<CONFIG:ReleaseAlpha>:"
        "winmm"
    ">"
    "$<$<CONFIG:Release>:"
        "winmm"
    ">"
)
target_link_libraries(${PROJECT_NAME} PUBLIC "${ADDITIONAL_LIBRARY_DEPENDENCIES}")

