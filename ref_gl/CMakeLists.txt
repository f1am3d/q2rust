set(PROJECT_NAME ref_gl)

################################################################################
# Source groups
################################################################################
set(Header_Files
    "../../../../Projects/PERSONAL/q2-rust/client/ref.h"
    "../../../../Projects/PERSONAL/q2-rust/game/q_shared.h"
    "../../../../Projects/PERSONAL/q2-rust/qcommon/qcommon.h"
    "../../../../Projects/PERSONAL/q2-rust/qcommon/qfiles.h"
    "../../../../Projects/PERSONAL/q2-rust/ref_gl/anorms.h"
    "../../../../Projects/PERSONAL/q2-rust/ref_gl/anormtab.h"
    "../../../../Projects/PERSONAL/q2-rust/ref_gl/gl_local.h"
    "../../../../Projects/PERSONAL/q2-rust/ref_gl/gl_model.h"
    "../../../../Projects/PERSONAL/q2-rust/ref_gl/qgl.h"
    "../../../../Projects/PERSONAL/q2-rust/ref_gl/warpsin.h"
    "../../../../Projects/PERSONAL/q2-rust/win32/glw_win.h"
    "../../../../Projects/PERSONAL/q2-rust/win32/winquake.h"
)
source_group("Header Files" FILES ${Header_Files})

set(Resource_Files
    "../../../../Projects/PERSONAL/q2-rust/ref_gl/ref_gl.def"
)
source_group("Resource Files" FILES ${Resource_Files})

set(Source_Files
    "../../../../Projects/PERSONAL/q2-rust/game/q_shared.c"
    "../../../../Projects/PERSONAL/q2-rust/ref_gl/gl_draw.c"
    "../../../../Projects/PERSONAL/q2-rust/ref_gl/gl_image.c"
    "../../../../Projects/PERSONAL/q2-rust/ref_gl/gl_light.c"
    "../../../../Projects/PERSONAL/q2-rust/ref_gl/gl_mesh.c"
    "../../../../Projects/PERSONAL/q2-rust/ref_gl/gl_model.c"
    "../../../../Projects/PERSONAL/q2-rust/ref_gl/gl_rmain.c"
    "../../../../Projects/PERSONAL/q2-rust/ref_gl/gl_rmisc.c"
    "../../../../Projects/PERSONAL/q2-rust/ref_gl/gl_rsurf.c"
    "../../../../Projects/PERSONAL/q2-rust/ref_gl/gl_warp.c"
    "../../../../Projects/PERSONAL/q2-rust/win32/glw_imp.c"
    "../../../../Projects/PERSONAL/q2-rust/win32/q_shwin.c"
    "../../../../Projects/PERSONAL/q2-rust/win32/qgl_win.c"
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
    TARGET_NAME_DEBUGALPHA   "ref_gl"
    TARGET_NAME_DEBUG        "ref_gl"
    TARGET_NAME_RELEASEALPHA "ref_gl"
    TARGET_NAME_RELEASE      "ref_gl"
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
    ARCHIVE_OUTPUT_NAME_DEBUGALPHA   "ref_gl"
    ARCHIVE_OUTPUT_NAME_DEBUG        "ref_gl"
    ARCHIVE_OUTPUT_NAME_RELEASEALPHA "ref_gl"
    ARCHIVE_OUTPUT_NAME_RELEASE      "ref_gl"
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
        "NDEBUG;"
        "WIN32;"
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
    source_file_compile_options(../../../../Projects/PERSONAL/q2-rust/game/q_shared.c ${FILE_CL_OPTIONS})
    string(CONCAT FILE_CL_OPTIONS
        "$<$<CONFIG:DebugAlpha>:"
            "/QA21164;/MTd;/QAieee1;/QA21164;/MTd;/QAieee1"
        ">"
        "$<$<CONFIG:ReleaseAlpha>:"
            "/QA21164;/QAieee1;/QA21164;/QAieee1"
        ">"
    )
    source_file_compile_options(../../../../Projects/PERSONAL/q2-rust/win32/glw_imp.c ${FILE_CL_OPTIONS})
    string(CONCAT FILE_CL_OPTIONS
        "$<$<CONFIG:DebugAlpha>:"
            "/QA21164;/MTd;/QAieee1;/QA21164;/MTd;/QAieee1"
        ">"
        "$<$<CONFIG:ReleaseAlpha>:"
            "/QA21164;/QAieee1;/QA21164;/QAieee1"
        ">"
    )
    source_file_compile_options(../../../../Projects/PERSONAL/q2-rust/win32/qgl_win.c ${FILE_CL_OPTIONS})
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
    source_file_compile_options(../../../../Projects/PERSONAL/q2-rust/ref_gl/gl_draw.c ${FILE_CL_OPTIONS})
    string(CONCAT FILE_CL_OPTIONS
        "$<$<CONFIG:DebugAlpha>:"
            "/QA21164;/MTd;/QAieee1;/QA21164;/MTd;/QAieee1"
        ">"
        "$<$<CONFIG:ReleaseAlpha>:"
            "/QA21164;/QAieee1;/QA21164;/QAieee1"
        ">"
    )
    source_file_compile_options(../../../../Projects/PERSONAL/q2-rust/ref_gl/gl_image.c ${FILE_CL_OPTIONS})
    string(CONCAT FILE_CL_OPTIONS
        "$<$<CONFIG:DebugAlpha>:"
            "/QA21164;/MTd;/QAieee1;/QA21164;/MTd;/QAieee1"
        ">"
        "$<$<CONFIG:ReleaseAlpha>:"
            "/QA21164;/QAieee1;/QA21164;/QAieee1"
        ">"
    )
    source_file_compile_options(../../../../Projects/PERSONAL/q2-rust/ref_gl/gl_light.c ${FILE_CL_OPTIONS})
    string(CONCAT FILE_CL_OPTIONS
        "$<$<CONFIG:DebugAlpha>:"
            "/QA21164;/MTd;/QAieee1;/QA21164;/MTd;/QAieee1"
        ">"
        "$<$<CONFIG:ReleaseAlpha>:"
            "/QA21164;/QAieee1;/QA21164;/QAieee1"
        ">"
    )
    source_file_compile_options(../../../../Projects/PERSONAL/q2-rust/ref_gl/gl_mesh.c ${FILE_CL_OPTIONS})
    string(CONCAT FILE_CL_OPTIONS
        "$<$<CONFIG:DebugAlpha>:"
            "/QA21164;/MTd;/QAieee1;/QA21164;/MTd;/QAieee1"
        ">"
        "$<$<CONFIG:ReleaseAlpha>:"
            "/QA21164;/QAieee1;/QA21164;/QAieee1"
        ">"
    )
    source_file_compile_options(../../../../Projects/PERSONAL/q2-rust/ref_gl/gl_model.c ${FILE_CL_OPTIONS})
    string(CONCAT FILE_CL_OPTIONS
        "$<$<CONFIG:DebugAlpha>:"
            "/QA21164;/MTd;/QAieee1;/QA21164;/MTd;/QAieee1"
        ">"
        "$<$<CONFIG:ReleaseAlpha>:"
            "/QA21164;/QAieee1;/QA21164;/QAieee1"
        ">"
    )
    source_file_compile_options(../../../../Projects/PERSONAL/q2-rust/ref_gl/gl_rmain.c ${FILE_CL_OPTIONS})
    string(CONCAT FILE_CL_OPTIONS
        "$<$<CONFIG:DebugAlpha>:"
            "/QA21164;/MTd;/QAieee1;/QA21164;/MTd;/QAieee1"
        ">"
        "$<$<CONFIG:ReleaseAlpha>:"
            "/QA21164;/QAieee1;/QA21164;/QAieee1"
        ">"
    )
    source_file_compile_options(../../../../Projects/PERSONAL/q2-rust/ref_gl/gl_rmisc.c ${FILE_CL_OPTIONS})
    string(CONCAT FILE_CL_OPTIONS
        "$<$<CONFIG:DebugAlpha>:"
            "/QA21164;/MTd;/QAieee1;/QA21164;/MTd;/QAieee1"
        ">"
        "$<$<CONFIG:ReleaseAlpha>:"
            "/QA21164;/QAieee1;/QA21164;/QAieee1"
        ">"
    )
    source_file_compile_options(../../../../Projects/PERSONAL/q2-rust/ref_gl/gl_rsurf.c ${FILE_CL_OPTIONS})
    string(CONCAT FILE_CL_OPTIONS
        "$<$<CONFIG:DebugAlpha>:"
            "/QA21164;/MTd;/QAieee1;/QA21164;/MTd;/QAieee1"
        ">"
        "$<$<CONFIG:ReleaseAlpha>:"
            "/QA21164;/QAieee1;/QA21164;/QAieee1"
        ">"
    )
    source_file_compile_options(../../../../Projects/PERSONAL/q2-rust/ref_gl/gl_warp.c ${FILE_CL_OPTIONS})
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
        "opengl32;"
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

