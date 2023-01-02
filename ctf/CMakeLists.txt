set(PROJECT_NAME ctf)

################################################################################
# Source groups
################################################################################
set(Header_Files
    "../../../../Projects/PERSONAL/q2-rust/ctf/g_ctf.h"
    "../../../../Projects/PERSONAL/q2-rust/ctf/g_local.h"
    "../../../../Projects/PERSONAL/q2-rust/ctf/game.h"
    "../../../../Projects/PERSONAL/q2-rust/ctf/m_player.h"
    "../../../../Projects/PERSONAL/q2-rust/ctf/p_menu.h"
    "../../../../Projects/PERSONAL/q2-rust/ctf/q_shared.h"
)
source_group("Header Files" FILES ${Header_Files})

set(Resource_Files
    "../../../../Projects/PERSONAL/q2-rust/ctf/ctf.def"
)
source_group("Resource Files" FILES ${Resource_Files})

set(Source_Files
    "../../../../Projects/PERSONAL/q2-rust/ctf/g_ai.c"
    "../../../../Projects/PERSONAL/q2-rust/ctf/g_chase.c"
    "../../../../Projects/PERSONAL/q2-rust/ctf/g_cmds.c"
    "../../../../Projects/PERSONAL/q2-rust/ctf/g_combat.c"
    "../../../../Projects/PERSONAL/q2-rust/ctf/g_ctf.c"
    "../../../../Projects/PERSONAL/q2-rust/ctf/g_func.c"
    "../../../../Projects/PERSONAL/q2-rust/ctf/g_items.c"
    "../../../../Projects/PERSONAL/q2-rust/ctf/g_main.c"
    "../../../../Projects/PERSONAL/q2-rust/ctf/g_misc.c"
    "../../../../Projects/PERSONAL/q2-rust/ctf/g_monster.c"
    "../../../../Projects/PERSONAL/q2-rust/ctf/g_phys.c"
    "../../../../Projects/PERSONAL/q2-rust/ctf/g_save.c"
    "../../../../Projects/PERSONAL/q2-rust/ctf/g_spawn.c"
    "../../../../Projects/PERSONAL/q2-rust/ctf/g_svcmds.c"
    "../../../../Projects/PERSONAL/q2-rust/ctf/g_target.c"
    "../../../../Projects/PERSONAL/q2-rust/ctf/g_trigger.c"
    "../../../../Projects/PERSONAL/q2-rust/ctf/g_utils.c"
    "../../../../Projects/PERSONAL/q2-rust/ctf/g_weapon.c"
    "../../../../Projects/PERSONAL/q2-rust/ctf/m_move.c"
    "../../../../Projects/PERSONAL/q2-rust/ctf/p_client.c"
    "../../../../Projects/PERSONAL/q2-rust/ctf/p_hud.c"
    "../../../../Projects/PERSONAL/q2-rust/ctf/p_menu.c"
    "../../../../Projects/PERSONAL/q2-rust/ctf/p_trail.c"
    "../../../../Projects/PERSONAL/q2-rust/ctf/p_view.c"
    "../../../../Projects/PERSONAL/q2-rust/ctf/p_weapon.c"
    "../../../../Projects/PERSONAL/q2-rust/ctf/q_shared.c"
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
    TARGET_NAME_DEBUGALPHA   "gameaxp"
    TARGET_NAME_DEBUG        "gamex86"
    TARGET_NAME_RELEASEALPHA "gameaxp"
    TARGET_NAME_RELEASE      "gamex86"
)
################################################################################
# Output directory
################################################################################
set_target_properties(${PROJECT_NAME} PROPERTIES
    OUTPUT_DIRECTORY_DEBUGALPHA   "${CMAKE_CURRENT_SOURCE_DIR}/./debugAXP"
    OUTPUT_DIRECTORY_DEBUG        "${CMAKE_CURRENT_SOURCE_DIR}/./debug"
    OUTPUT_DIRECTORY_RELEASEALPHA "${CMAKE_CURRENT_SOURCE_DIR}/./ReleaseAXP"
    OUTPUT_DIRECTORY_RELEASE      "${CMAKE_CURRENT_SOURCE_DIR}/./release"
)
set_target_properties(${PROJECT_NAME} PROPERTIES
    ARCHIVE_OUTPUT_DIRECTORY_DEBUGALPHA   "${CMAKE_CURRENT_SOURCE_DIR}/./../DebugAXP"
    ARCHIVE_OUTPUT_DIRECTORY_DEBUG        "${CMAKE_CURRENT_SOURCE_DIR}/./debug"
    ARCHIVE_OUTPUT_DIRECTORY_RELEASEALPHA "${CMAKE_CURRENT_SOURCE_DIR}/./../ReleaseAXP"
    ARCHIVE_OUTPUT_DIRECTORY_RELEASE      "${CMAKE_CURRENT_SOURCE_DIR}/./release"
)
set_target_properties(${PROJECT_NAME} PROPERTIES
    ARCHIVE_OUTPUT_NAME_DEBUGALPHA   "gameaxp"
    ARCHIVE_OUTPUT_NAME_DEBUG        "gamex86"
    ARCHIVE_OUTPUT_NAME_RELEASEALPHA "gameaxp"
    ARCHIVE_OUTPUT_NAME_RELEASE      "gamex86"
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
        "_WINDOWS"
    ">"
    "$<$<CONFIG:Debug>:"
        "WIN32;"
        "_DEBUG;"
        "_WINDOWS"
    ">"
    "$<$<CONFIG:ReleaseAlpha>:"
        "WIN32;"
        "NDEBUG;"
        "_WINDOWS"
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
            /MTd
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
            ${DEFAULT_CXX_DEBUG_INFORMATION_FORMAT}
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
            "/MTd;/MTd"
        ">"
    )
    source_file_compile_options(../../../../Projects/PERSONAL/q2-rust/ctf/g_ai.c ${FILE_CL_OPTIONS})
    string(CONCAT FILE_CL_OPTIONS
        "$<$<CONFIG:DebugAlpha>:"
            "/MTd;/MTd"
        ">"
    )
    source_file_compile_options(../../../../Projects/PERSONAL/q2-rust/ctf/g_chase.c ${FILE_CL_OPTIONS})
    string(CONCAT FILE_CL_OPTIONS
        "$<$<CONFIG:DebugAlpha>:"
            "/MTd;/MTd"
        ">"
    )
    source_file_compile_options(../../../../Projects/PERSONAL/q2-rust/ctf/g_cmds.c ${FILE_CL_OPTIONS})
    string(CONCAT FILE_CL_OPTIONS
        "$<$<CONFIG:DebugAlpha>:"
            "/MTd;/MTd"
        ">"
    )
    source_file_compile_options(../../../../Projects/PERSONAL/q2-rust/ctf/g_combat.c ${FILE_CL_OPTIONS})
    string(CONCAT FILE_CL_OPTIONS
        "$<$<CONFIG:DebugAlpha>:"
            "/MTd;/MTd"
        ">"
    )
    source_file_compile_options(../../../../Projects/PERSONAL/q2-rust/ctf/g_ctf.c ${FILE_CL_OPTIONS})
    string(CONCAT FILE_CL_OPTIONS
        "$<$<CONFIG:DebugAlpha>:"
            "/MTd;/MTd"
        ">"
    )
    source_file_compile_options(../../../../Projects/PERSONAL/q2-rust/ctf/g_func.c ${FILE_CL_OPTIONS})
    string(CONCAT FILE_CL_OPTIONS
        "$<$<CONFIG:DebugAlpha>:"
            "/MTd;/MTd"
        ">"
    )
    source_file_compile_options(../../../../Projects/PERSONAL/q2-rust/ctf/g_items.c ${FILE_CL_OPTIONS})
    string(CONCAT FILE_CL_OPTIONS
        "$<$<CONFIG:DebugAlpha>:"
            "/MTd;/MTd"
        ">"
    )
    source_file_compile_options(../../../../Projects/PERSONAL/q2-rust/ctf/g_main.c ${FILE_CL_OPTIONS})
    string(CONCAT FILE_CL_OPTIONS
        "$<$<CONFIG:DebugAlpha>:"
            "/MTd;/MTd"
        ">"
    )
    source_file_compile_options(../../../../Projects/PERSONAL/q2-rust/ctf/g_misc.c ${FILE_CL_OPTIONS})
    string(CONCAT FILE_CL_OPTIONS
        "$<$<CONFIG:DebugAlpha>:"
            "/MTd;/MTd"
        ">"
    )
    source_file_compile_options(../../../../Projects/PERSONAL/q2-rust/ctf/g_monster.c ${FILE_CL_OPTIONS})
    string(CONCAT FILE_CL_OPTIONS
        "$<$<CONFIG:DebugAlpha>:"
            "/MTd;/MTd"
        ">"
    )
    source_file_compile_options(../../../../Projects/PERSONAL/q2-rust/ctf/g_phys.c ${FILE_CL_OPTIONS})
    string(CONCAT FILE_CL_OPTIONS
        "$<$<CONFIG:DebugAlpha>:"
            "/MTd;/MTd"
        ">"
    )
    source_file_compile_options(../../../../Projects/PERSONAL/q2-rust/ctf/g_save.c ${FILE_CL_OPTIONS})
    string(CONCAT FILE_CL_OPTIONS
        "$<$<CONFIG:DebugAlpha>:"
            "/MTd;/MTd"
        ">"
    )
    source_file_compile_options(../../../../Projects/PERSONAL/q2-rust/ctf/g_spawn.c ${FILE_CL_OPTIONS})
    string(CONCAT FILE_CL_OPTIONS
        "$<$<CONFIG:DebugAlpha>:"
            "/MTd;/MTd"
        ">"
    )
    source_file_compile_options(../../../../Projects/PERSONAL/q2-rust/ctf/g_svcmds.c ${FILE_CL_OPTIONS})
    string(CONCAT FILE_CL_OPTIONS
        "$<$<CONFIG:DebugAlpha>:"
            "/MTd;/MTd"
        ">"
    )
    source_file_compile_options(../../../../Projects/PERSONAL/q2-rust/ctf/g_target.c ${FILE_CL_OPTIONS})
    string(CONCAT FILE_CL_OPTIONS
        "$<$<CONFIG:DebugAlpha>:"
            "/MTd;/MTd"
        ">"
    )
    source_file_compile_options(../../../../Projects/PERSONAL/q2-rust/ctf/g_trigger.c ${FILE_CL_OPTIONS})
    string(CONCAT FILE_CL_OPTIONS
        "$<$<CONFIG:DebugAlpha>:"
            "/MTd;/MTd"
        ">"
    )
    source_file_compile_options(../../../../Projects/PERSONAL/q2-rust/ctf/g_utils.c ${FILE_CL_OPTIONS})
    string(CONCAT FILE_CL_OPTIONS
        "$<$<CONFIG:DebugAlpha>:"
            "/MTd;/MTd"
        ">"
    )
    source_file_compile_options(../../../../Projects/PERSONAL/q2-rust/ctf/g_weapon.c ${FILE_CL_OPTIONS})
    string(CONCAT FILE_CL_OPTIONS
        "$<$<CONFIG:DebugAlpha>:"
            "/MTd;/MTd"
        ">"
    )
    source_file_compile_options(../../../../Projects/PERSONAL/q2-rust/ctf/m_move.c ${FILE_CL_OPTIONS})
    string(CONCAT FILE_CL_OPTIONS
        "$<$<CONFIG:DebugAlpha>:"
            "/MTd;/MTd"
        ">"
    )
    source_file_compile_options(../../../../Projects/PERSONAL/q2-rust/ctf/p_client.c ${FILE_CL_OPTIONS})
    string(CONCAT FILE_CL_OPTIONS
        "$<$<CONFIG:DebugAlpha>:"
            "/MTd;/MTd"
        ">"
    )
    source_file_compile_options(../../../../Projects/PERSONAL/q2-rust/ctf/p_hud.c ${FILE_CL_OPTIONS})
    string(CONCAT FILE_CL_OPTIONS
        "$<$<CONFIG:DebugAlpha>:"
            "/MTd;/MTd"
        ">"
    )
    source_file_compile_options(../../../../Projects/PERSONAL/q2-rust/ctf/p_menu.c ${FILE_CL_OPTIONS})
    string(CONCAT FILE_CL_OPTIONS
        "$<$<CONFIG:DebugAlpha>:"
            "/MTd;/MTd"
        ">"
    )
    source_file_compile_options(../../../../Projects/PERSONAL/q2-rust/ctf/p_trail.c ${FILE_CL_OPTIONS})
    string(CONCAT FILE_CL_OPTIONS
        "$<$<CONFIG:DebugAlpha>:"
            "/MTd;/MTd"
        ">"
    )
    source_file_compile_options(../../../../Projects/PERSONAL/q2-rust/ctf/p_view.c ${FILE_CL_OPTIONS})
    string(CONCAT FILE_CL_OPTIONS
        "$<$<CONFIG:DebugAlpha>:"
            "/MTd;/MTd"
        ">"
    )
    source_file_compile_options(../../../../Projects/PERSONAL/q2-rust/ctf/p_weapon.c ${FILE_CL_OPTIONS})
    string(CONCAT FILE_CL_OPTIONS
        "$<$<CONFIG:DebugAlpha>:"
            "/MTd;/MTd"
        ">"
    )
    source_file_compile_options(../../../../Projects/PERSONAL/q2-rust/ctf/q_shared.c ${FILE_CL_OPTIONS})
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
            /DEBUG:FULL;
            /SUBSYSTEM:WINDOWS;
            /INCREMENTAL:NO
        >
        $<$<CONFIG:Release>:
            /NOLOGO;
            /DEBUG:FULL;
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

