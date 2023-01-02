set(PROJECT_NAME game)

################################################################################
# Source groups
################################################################################
set(Header_Files
    "../../../../Projects/PERSONAL/q2-rust/game/g_local.h"
    "../../../../Projects/PERSONAL/q2-rust/game/game.h"
    "../../../../Projects/PERSONAL/q2-rust/game/m_actor.h"
    "../../../../Projects/PERSONAL/q2-rust/game/m_berserk.h"
    "../../../../Projects/PERSONAL/q2-rust/game/m_boss2.h"
    "../../../../Projects/PERSONAL/q2-rust/game/m_boss31.h"
    "../../../../Projects/PERSONAL/q2-rust/game/m_boss32.h"
    "../../../../Projects/PERSONAL/q2-rust/game/m_brain.h"
    "../../../../Projects/PERSONAL/q2-rust/game/m_chick.h"
    "../../../../Projects/PERSONAL/q2-rust/game/m_flipper.h"
    "../../../../Projects/PERSONAL/q2-rust/game/m_float.h"
    "../../../../Projects/PERSONAL/q2-rust/game/m_flyer.h"
    "../../../../Projects/PERSONAL/q2-rust/game/m_gladiator.h"
    "../../../../Projects/PERSONAL/q2-rust/game/m_gunner.h"
    "../../../../Projects/PERSONAL/q2-rust/game/m_hover.h"
    "../../../../Projects/PERSONAL/q2-rust/game/m_infantry.h"
    "../../../../Projects/PERSONAL/q2-rust/game/m_insane.h"
    "../../../../Projects/PERSONAL/q2-rust/game/m_medic.h"
    "../../../../Projects/PERSONAL/q2-rust/game/m_mutant.h"
    "../../../../Projects/PERSONAL/q2-rust/game/m_parasite.h"
    "../../../../Projects/PERSONAL/q2-rust/game/m_player.h"
    "../../../../Projects/PERSONAL/q2-rust/game/m_soldier.h"
    "../../../../Projects/PERSONAL/q2-rust/game/m_supertank.h"
    "../../../../Projects/PERSONAL/q2-rust/game/m_tank.h"
    "../../../../Projects/PERSONAL/q2-rust/game/q_shared.h"
)
source_group("Header Files" FILES ${Header_Files})

set(Resource_Files
    "../../../../Projects/PERSONAL/q2-rust/game/game.def"
)
source_group("Resource Files" FILES ${Resource_Files})

set(Source_Files
    "../../../../Projects/PERSONAL/q2-rust/game/g_ai.c"
    "../../../../Projects/PERSONAL/q2-rust/game/g_chase.c"
    "../../../../Projects/PERSONAL/q2-rust/game/g_cmds.c"
    "../../../../Projects/PERSONAL/q2-rust/game/g_combat.c"
    "../../../../Projects/PERSONAL/q2-rust/game/g_func.c"
    "../../../../Projects/PERSONAL/q2-rust/game/g_items.c"
    "../../../../Projects/PERSONAL/q2-rust/game/g_main.c"
    "../../../../Projects/PERSONAL/q2-rust/game/g_misc.c"
    "../../../../Projects/PERSONAL/q2-rust/game/g_monster.c"
    "../../../../Projects/PERSONAL/q2-rust/game/g_phys.c"
    "../../../../Projects/PERSONAL/q2-rust/game/g_save.c"
    "../../../../Projects/PERSONAL/q2-rust/game/g_spawn.c"
    "../../../../Projects/PERSONAL/q2-rust/game/g_svcmds.c"
    "../../../../Projects/PERSONAL/q2-rust/game/g_target.c"
    "../../../../Projects/PERSONAL/q2-rust/game/g_trigger.c"
    "../../../../Projects/PERSONAL/q2-rust/game/g_turret.c"
    "../../../../Projects/PERSONAL/q2-rust/game/g_utils.c"
    "../../../../Projects/PERSONAL/q2-rust/game/g_weapon.c"
    "../../../../Projects/PERSONAL/q2-rust/game/m_actor.c"
    "../../../../Projects/PERSONAL/q2-rust/game/m_berserk.c"
    "../../../../Projects/PERSONAL/q2-rust/game/m_boss2.c"
    "../../../../Projects/PERSONAL/q2-rust/game/m_boss3.c"
    "../../../../Projects/PERSONAL/q2-rust/game/m_boss31.c"
    "../../../../Projects/PERSONAL/q2-rust/game/m_boss32.c"
    "../../../../Projects/PERSONAL/q2-rust/game/m_brain.c"
    "../../../../Projects/PERSONAL/q2-rust/game/m_chick.c"
    "../../../../Projects/PERSONAL/q2-rust/game/m_flash.c"
    "../../../../Projects/PERSONAL/q2-rust/game/m_flipper.c"
    "../../../../Projects/PERSONAL/q2-rust/game/m_float.c"
    "../../../../Projects/PERSONAL/q2-rust/game/m_flyer.c"
    "../../../../Projects/PERSONAL/q2-rust/game/m_gladiator.c"
    "../../../../Projects/PERSONAL/q2-rust/game/m_gunner.c"
    "../../../../Projects/PERSONAL/q2-rust/game/m_hover.c"
    "../../../../Projects/PERSONAL/q2-rust/game/m_infantry.c"
    "../../../../Projects/PERSONAL/q2-rust/game/m_insane.c"
    "../../../../Projects/PERSONAL/q2-rust/game/m_medic.c"
    "../../../../Projects/PERSONAL/q2-rust/game/m_move.c"
    "../../../../Projects/PERSONAL/q2-rust/game/m_mutant.c"
    "../../../../Projects/PERSONAL/q2-rust/game/m_parasite.c"
    "../../../../Projects/PERSONAL/q2-rust/game/m_soldier.c"
    "../../../../Projects/PERSONAL/q2-rust/game/m_supertank.c"
    "../../../../Projects/PERSONAL/q2-rust/game/m_tank.c"
    "../../../../Projects/PERSONAL/q2-rust/game/p_client.c"
    "../../../../Projects/PERSONAL/q2-rust/game/p_hud.c"
    "../../../../Projects/PERSONAL/q2-rust/game/p_trail.c"
    "../../../../Projects/PERSONAL/q2-rust/game/p_view.c"
    "../../../../Projects/PERSONAL/q2-rust/game/p_weapon.c"
    "../../../../Projects/PERSONAL/q2-rust/game/q_shared.c"
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
    OUTPUT_DIRECTORY_DEBUGALPHA   "${CMAKE_CURRENT_SOURCE_DIR}/../DebugAxp"
    OUTPUT_DIRECTORY_DEBUG        "${CMAKE_CURRENT_SOURCE_DIR}/../debug"
    OUTPUT_DIRECTORY_RELEASEALPHA "${CMAKE_CURRENT_SOURCE_DIR}/../ReleaseAXP"
    OUTPUT_DIRECTORY_RELEASE      "${CMAKE_CURRENT_SOURCE_DIR}/../release"
)
set_target_properties(${PROJECT_NAME} PROPERTIES
    ARCHIVE_OUTPUT_DIRECTORY_DEBUGALPHA   "${CMAKE_CURRENT_SOURCE_DIR}/./../DebugAxp"
    ARCHIVE_OUTPUT_DIRECTORY_DEBUG        "${CMAKE_CURRENT_SOURCE_DIR}/./../debug"
    ARCHIVE_OUTPUT_DIRECTORY_RELEASEALPHA "${CMAKE_CURRENT_SOURCE_DIR}/./../ReleaseAXP"
    ARCHIVE_OUTPUT_DIRECTORY_RELEASE      "${CMAKE_CURRENT_SOURCE_DIR}/./../release"
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
        "_WINDOWS;"
        "C_ONLY"
    ">"
    "$<$<CONFIG:Debug>:"
        "WIN32;"
        "_DEBUG;"
        "_WINDOWS;"
        "BUILDING_REF_GL"
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
            /MTd
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
            /QA21164
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
            "/QA21164;/MTd;/QA21164;/MTd"
        ">"
        "$<$<CONFIG:ReleaseAlpha>:"
            "/QA21164;/QA21164"
        ">"
    )
    source_file_compile_options(../../../../Projects/PERSONAL/q2-rust/game/g_ai.c ${FILE_CL_OPTIONS})
    string(CONCAT FILE_CL_OPTIONS
        "$<$<CONFIG:DebugAlpha>:"
            "/QA21164;/MTd;/QA21164;/MTd"
        ">"
        "$<$<CONFIG:ReleaseAlpha>:"
            "/QA21164;/QA21164"
        ">"
    )
    source_file_compile_options(../../../../Projects/PERSONAL/q2-rust/game/g_chase.c ${FILE_CL_OPTIONS})
    string(CONCAT FILE_CL_OPTIONS
        "$<$<CONFIG:DebugAlpha>:"
            "/QA21164;/MTd;/QA21164;/MTd"
        ">"
        "$<$<CONFIG:ReleaseAlpha>:"
            "/QA21164;/QA21164"
        ">"
    )
    source_file_compile_options(../../../../Projects/PERSONAL/q2-rust/game/g_cmds.c ${FILE_CL_OPTIONS})
    string(CONCAT FILE_CL_OPTIONS
        "$<$<CONFIG:DebugAlpha>:"
            "/QA21164;/MTd;/QA21164;/MTd"
        ">"
        "$<$<CONFIG:ReleaseAlpha>:"
            "/QA21164;/QA21164"
        ">"
    )
    source_file_compile_options(../../../../Projects/PERSONAL/q2-rust/game/g_combat.c ${FILE_CL_OPTIONS})
    string(CONCAT FILE_CL_OPTIONS
        "$<$<CONFIG:DebugAlpha>:"
            "/QA21164;/MTd;/QA21164;/MTd"
        ">"
        "$<$<CONFIG:ReleaseAlpha>:"
            "/QA21164;/QA21164"
        ">"
    )
    source_file_compile_options(../../../../Projects/PERSONAL/q2-rust/game/g_func.c ${FILE_CL_OPTIONS})
    string(CONCAT FILE_CL_OPTIONS
        "$<$<CONFIG:DebugAlpha>:"
            "/QA21164;/MTd;/QA21164;/MTd"
        ">"
        "$<$<CONFIG:ReleaseAlpha>:"
            "/QA21164;/QA21164"
        ">"
    )
    source_file_compile_options(../../../../Projects/PERSONAL/q2-rust/game/g_items.c ${FILE_CL_OPTIONS})
    string(CONCAT FILE_CL_OPTIONS
        "$<$<CONFIG:DebugAlpha>:"
            "/QA21164;/MTd;/QA21164;/MTd"
        ">"
        "$<$<CONFIG:ReleaseAlpha>:"
            "/QA21164;/QA21164"
        ">"
    )
    source_file_compile_options(../../../../Projects/PERSONAL/q2-rust/game/g_main.c ${FILE_CL_OPTIONS})
    string(CONCAT FILE_CL_OPTIONS
        "$<$<CONFIG:DebugAlpha>:"
            "/QA21164;/MTd;/QA21164;/MTd"
        ">"
        "$<$<CONFIG:ReleaseAlpha>:"
            "/QA21164;/QA21164"
        ">"
    )
    source_file_compile_options(../../../../Projects/PERSONAL/q2-rust/game/g_misc.c ${FILE_CL_OPTIONS})
    string(CONCAT FILE_CL_OPTIONS
        "$<$<CONFIG:DebugAlpha>:"
            "/QA21164;/MTd;/QA21164;/MTd"
        ">"
        "$<$<CONFIG:ReleaseAlpha>:"
            "/QA21164;/QA21164"
        ">"
    )
    source_file_compile_options(../../../../Projects/PERSONAL/q2-rust/game/g_monster.c ${FILE_CL_OPTIONS})
    string(CONCAT FILE_CL_OPTIONS
        "$<$<CONFIG:DebugAlpha>:"
            "/QA21164;/MTd;/QA21164;/MTd"
        ">"
        "$<$<CONFIG:ReleaseAlpha>:"
            "/QA21164;/QA21164"
        ">"
    )
    source_file_compile_options(../../../../Projects/PERSONAL/q2-rust/game/g_phys.c ${FILE_CL_OPTIONS})
    string(CONCAT FILE_CL_OPTIONS
        "$<$<CONFIG:DebugAlpha>:"
            "/QA21164;/MTd;/QA21164;/MTd"
        ">"
        "$<$<CONFIG:ReleaseAlpha>:"
            "/QA21164;/QA21164"
        ">"
    )
    source_file_compile_options(../../../../Projects/PERSONAL/q2-rust/game/g_save.c ${FILE_CL_OPTIONS})
    string(CONCAT FILE_CL_OPTIONS
        "$<$<CONFIG:DebugAlpha>:"
            "/QA21164;/MTd;/QA21164;/MTd"
        ">"
        "$<$<CONFIG:ReleaseAlpha>:"
            "/QA21164;/QA21164"
        ">"
    )
    source_file_compile_options(../../../../Projects/PERSONAL/q2-rust/game/g_spawn.c ${FILE_CL_OPTIONS})
    string(CONCAT FILE_CL_OPTIONS
        "$<$<CONFIG:DebugAlpha>:"
            "/QA21164;/MTd;/QA21164;/MTd"
        ">"
        "$<$<CONFIG:ReleaseAlpha>:"
            "/QA21164;/QA21164"
        ">"
    )
    source_file_compile_options(../../../../Projects/PERSONAL/q2-rust/game/g_svcmds.c ${FILE_CL_OPTIONS})
    string(CONCAT FILE_CL_OPTIONS
        "$<$<CONFIG:DebugAlpha>:"
            "/QA21164;/MTd;/QA21164;/MTd"
        ">"
        "$<$<CONFIG:ReleaseAlpha>:"
            "/QA21164;/QA21164"
        ">"
    )
    source_file_compile_options(../../../../Projects/PERSONAL/q2-rust/game/g_target.c ${FILE_CL_OPTIONS})
    string(CONCAT FILE_CL_OPTIONS
        "$<$<CONFIG:DebugAlpha>:"
            "/QA21164;/MTd;/QA21164;/MTd"
        ">"
        "$<$<CONFIG:ReleaseAlpha>:"
            "/QA21164;/QA21164"
        ">"
    )
    source_file_compile_options(../../../../Projects/PERSONAL/q2-rust/game/g_trigger.c ${FILE_CL_OPTIONS})
    string(CONCAT FILE_CL_OPTIONS
        "$<$<CONFIG:DebugAlpha>:"
            "/QA21164;/MTd;/QA21164;/MTd"
        ">"
        "$<$<CONFIG:ReleaseAlpha>:"
            "/QA21164;/QA21164"
        ">"
    )
    source_file_compile_options(../../../../Projects/PERSONAL/q2-rust/game/g_turret.c ${FILE_CL_OPTIONS})
    string(CONCAT FILE_CL_OPTIONS
        "$<$<CONFIG:DebugAlpha>:"
            "/QA21164;/MTd;/QA21164;/MTd"
        ">"
        "$<$<CONFIG:ReleaseAlpha>:"
            "/QA21164;/QA21164"
        ">"
    )
    source_file_compile_options(../../../../Projects/PERSONAL/q2-rust/game/g_utils.c ${FILE_CL_OPTIONS})
    string(CONCAT FILE_CL_OPTIONS
        "$<$<CONFIG:DebugAlpha>:"
            "/QA21164;/MTd;/QA21164;/MTd"
        ">"
        "$<$<CONFIG:ReleaseAlpha>:"
            "/QA21164;/QA21164"
        ">"
    )
    source_file_compile_options(../../../../Projects/PERSONAL/q2-rust/game/g_weapon.c ${FILE_CL_OPTIONS})
    string(CONCAT FILE_CL_OPTIONS
        "$<$<CONFIG:DebugAlpha>:"
            "/QA21164;/MTd;/QA21164;/MTd"
        ">"
        "$<$<CONFIG:ReleaseAlpha>:"
            "/QA21164;/QA21164"
        ">"
    )
    source_file_compile_options(../../../../Projects/PERSONAL/q2-rust/game/m_actor.c ${FILE_CL_OPTIONS})
    string(CONCAT FILE_CL_OPTIONS
        "$<$<CONFIG:DebugAlpha>:"
            "/QA21164;/MTd;/QA21164;/MTd"
        ">"
        "$<$<CONFIG:ReleaseAlpha>:"
            "/QA21164;/QA21164"
        ">"
    )
    source_file_compile_options(../../../../Projects/PERSONAL/q2-rust/game/m_berserk.c ${FILE_CL_OPTIONS})
    string(CONCAT FILE_CL_OPTIONS
        "$<$<CONFIG:DebugAlpha>:"
            "/QA21164;/MTd;/QA21164;/MTd"
        ">"
        "$<$<CONFIG:ReleaseAlpha>:"
            "/QA21164;/QA21164"
        ">"
    )
    source_file_compile_options(../../../../Projects/PERSONAL/q2-rust/game/m_boss2.c ${FILE_CL_OPTIONS})
    string(CONCAT FILE_CL_OPTIONS
        "$<$<CONFIG:DebugAlpha>:"
            "/QA21164;/MTd;/QA21164;/MTd"
        ">"
        "$<$<CONFIG:ReleaseAlpha>:"
            "/QA21164;/QA21164"
        ">"
    )
    source_file_compile_options(../../../../Projects/PERSONAL/q2-rust/game/m_boss3.c ${FILE_CL_OPTIONS})
    string(CONCAT FILE_CL_OPTIONS
        "$<$<CONFIG:DebugAlpha>:"
            "/QA21164;/MTd;/QA21164;/MTd"
        ">"
        "$<$<CONFIG:ReleaseAlpha>:"
            "/QA21164;/QA21164"
        ">"
    )
    source_file_compile_options(../../../../Projects/PERSONAL/q2-rust/game/m_boss31.c ${FILE_CL_OPTIONS})
    string(CONCAT FILE_CL_OPTIONS
        "$<$<CONFIG:DebugAlpha>:"
            "/QA21164;/MTd;/QA21164;/MTd"
        ">"
        "$<$<CONFIG:ReleaseAlpha>:"
            "/QA21164;/QA21164"
        ">"
    )
    source_file_compile_options(../../../../Projects/PERSONAL/q2-rust/game/m_boss32.c ${FILE_CL_OPTIONS})
    string(CONCAT FILE_CL_OPTIONS
        "$<$<CONFIG:DebugAlpha>:"
            "/QA21164;/MTd;/QA21164;/MTd"
        ">"
        "$<$<CONFIG:ReleaseAlpha>:"
            "/QA21164;/QA21164"
        ">"
    )
    source_file_compile_options(../../../../Projects/PERSONAL/q2-rust/game/m_brain.c ${FILE_CL_OPTIONS})
    string(CONCAT FILE_CL_OPTIONS
        "$<$<CONFIG:DebugAlpha>:"
            "/QA21164;/MTd;/QA21164;/MTd"
        ">"
        "$<$<CONFIG:ReleaseAlpha>:"
            "/QA21164;/QA21164"
        ">"
    )
    source_file_compile_options(../../../../Projects/PERSONAL/q2-rust/game/m_chick.c ${FILE_CL_OPTIONS})
    string(CONCAT FILE_CL_OPTIONS
        "$<$<CONFIG:DebugAlpha>:"
            "/QA21164;/MTd;/QA21164;/MTd"
        ">"
        "$<$<CONFIG:ReleaseAlpha>:"
            "/QA21164;/QA21164"
        ">"
    )
    source_file_compile_options(../../../../Projects/PERSONAL/q2-rust/game/m_flash.c ${FILE_CL_OPTIONS})
    string(CONCAT FILE_CL_OPTIONS
        "$<$<CONFIG:DebugAlpha>:"
            "/QA21164;/MTd;/QA21164;/MTd"
        ">"
        "$<$<CONFIG:ReleaseAlpha>:"
            "/QA21164;/QA21164"
        ">"
    )
    source_file_compile_options(../../../../Projects/PERSONAL/q2-rust/game/m_flipper.c ${FILE_CL_OPTIONS})
    string(CONCAT FILE_CL_OPTIONS
        "$<$<CONFIG:DebugAlpha>:"
            "/QA21164;/MTd;/QA21164;/MTd"
        ">"
        "$<$<CONFIG:ReleaseAlpha>:"
            "/QA21164;/QA21164"
        ">"
    )
    source_file_compile_options(../../../../Projects/PERSONAL/q2-rust/game/m_float.c ${FILE_CL_OPTIONS})
    string(CONCAT FILE_CL_OPTIONS
        "$<$<CONFIG:DebugAlpha>:"
            "/QA21164;/MTd;/QA21164;/MTd"
        ">"
        "$<$<CONFIG:ReleaseAlpha>:"
            "/QA21164;/QA21164"
        ">"
    )
    source_file_compile_options(../../../../Projects/PERSONAL/q2-rust/game/m_flyer.c ${FILE_CL_OPTIONS})
    string(CONCAT FILE_CL_OPTIONS
        "$<$<CONFIG:DebugAlpha>:"
            "/QA21164;/MTd;/QA21164;/MTd"
        ">"
        "$<$<CONFIG:ReleaseAlpha>:"
            "/QA21164;/QA21164"
        ">"
    )
    source_file_compile_options(../../../../Projects/PERSONAL/q2-rust/game/m_gladiator.c ${FILE_CL_OPTIONS})
    string(CONCAT FILE_CL_OPTIONS
        "$<$<CONFIG:DebugAlpha>:"
            "/QA21164;/MTd;/QA21164;/MTd"
        ">"
        "$<$<CONFIG:ReleaseAlpha>:"
            "/QA21164;/QA21164"
        ">"
    )
    source_file_compile_options(../../../../Projects/PERSONAL/q2-rust/game/m_gunner.c ${FILE_CL_OPTIONS})
    string(CONCAT FILE_CL_OPTIONS
        "$<$<CONFIG:DebugAlpha>:"
            "/QA21164;/MTd;/QA21164;/MTd"
        ">"
        "$<$<CONFIG:ReleaseAlpha>:"
            "/QA21164;/QA21164"
        ">"
    )
    source_file_compile_options(../../../../Projects/PERSONAL/q2-rust/game/m_hover.c ${FILE_CL_OPTIONS})
    string(CONCAT FILE_CL_OPTIONS
        "$<$<CONFIG:DebugAlpha>:"
            "/QA21164;/MTd;/QA21164;/MTd"
        ">"
        "$<$<CONFIG:ReleaseAlpha>:"
            "/QA21164;/QA21164"
        ">"
    )
    source_file_compile_options(../../../../Projects/PERSONAL/q2-rust/game/m_infantry.c ${FILE_CL_OPTIONS})
    string(CONCAT FILE_CL_OPTIONS
        "$<$<CONFIG:DebugAlpha>:"
            "/QA21164;/MTd;/QA21164;/MTd"
        ">"
        "$<$<CONFIG:ReleaseAlpha>:"
            "/QA21164;/QA21164"
        ">"
    )
    source_file_compile_options(../../../../Projects/PERSONAL/q2-rust/game/m_insane.c ${FILE_CL_OPTIONS})
    string(CONCAT FILE_CL_OPTIONS
        "$<$<CONFIG:DebugAlpha>:"
            "/QA21164;/MTd;/QA21164;/MTd"
        ">"
        "$<$<CONFIG:ReleaseAlpha>:"
            "/QA21164;/QA21164"
        ">"
    )
    source_file_compile_options(../../../../Projects/PERSONAL/q2-rust/game/m_medic.c ${FILE_CL_OPTIONS})
    string(CONCAT FILE_CL_OPTIONS
        "$<$<CONFIG:DebugAlpha>:"
            "/QA21164;/MTd;/QA21164;/MTd"
        ">"
        "$<$<CONFIG:ReleaseAlpha>:"
            "/QA21164;/QA21164"
        ">"
    )
    source_file_compile_options(../../../../Projects/PERSONAL/q2-rust/game/m_move.c ${FILE_CL_OPTIONS})
    string(CONCAT FILE_CL_OPTIONS
        "$<$<CONFIG:DebugAlpha>:"
            "/QA21164;/MTd;/QA21164;/MTd"
        ">"
        "$<$<CONFIG:ReleaseAlpha>:"
            "/QA21164;/QA21164"
        ">"
    )
    source_file_compile_options(../../../../Projects/PERSONAL/q2-rust/game/m_mutant.c ${FILE_CL_OPTIONS})
    string(CONCAT FILE_CL_OPTIONS
        "$<$<CONFIG:DebugAlpha>:"
            "/QA21164;/MTd;/QA21164;/MTd"
        ">"
        "$<$<CONFIG:ReleaseAlpha>:"
            "/QA21164;/QA21164"
        ">"
    )
    source_file_compile_options(../../../../Projects/PERSONAL/q2-rust/game/m_parasite.c ${FILE_CL_OPTIONS})
    string(CONCAT FILE_CL_OPTIONS
        "$<$<CONFIG:DebugAlpha>:"
            "/QA21164;/MTd;/QA21164;/MTd"
        ">"
        "$<$<CONFIG:ReleaseAlpha>:"
            "/QA21164;/QA21164"
        ">"
    )
    source_file_compile_options(../../../../Projects/PERSONAL/q2-rust/game/m_soldier.c ${FILE_CL_OPTIONS})
    string(CONCAT FILE_CL_OPTIONS
        "$<$<CONFIG:DebugAlpha>:"
            "/QA21164;/MTd;/QA21164;/MTd"
        ">"
        "$<$<CONFIG:ReleaseAlpha>:"
            "/QA21164;/QA21164"
        ">"
    )
    source_file_compile_options(../../../../Projects/PERSONAL/q2-rust/game/m_supertank.c ${FILE_CL_OPTIONS})
    string(CONCAT FILE_CL_OPTIONS
        "$<$<CONFIG:DebugAlpha>:"
            "/QA21164;/MTd;/QA21164;/MTd"
        ">"
        "$<$<CONFIG:ReleaseAlpha>:"
            "/QA21164;/QA21164"
        ">"
    )
    source_file_compile_options(../../../../Projects/PERSONAL/q2-rust/game/m_tank.c ${FILE_CL_OPTIONS})
    string(CONCAT FILE_CL_OPTIONS
        "$<$<CONFIG:DebugAlpha>:"
            "/QA21164;/MTd;/QA21164;/MTd"
        ">"
        "$<$<CONFIG:ReleaseAlpha>:"
            "/QA21164;/QA21164"
        ">"
    )
    source_file_compile_options(../../../../Projects/PERSONAL/q2-rust/game/p_client.c ${FILE_CL_OPTIONS})
    string(CONCAT FILE_CL_OPTIONS
        "$<$<CONFIG:DebugAlpha>:"
            "/QA21164;/MTd;/QA21164;/MTd"
        ">"
        "$<$<CONFIG:ReleaseAlpha>:"
            "/QA21164;/QA21164"
        ">"
    )
    source_file_compile_options(../../../../Projects/PERSONAL/q2-rust/game/p_hud.c ${FILE_CL_OPTIONS})
    string(CONCAT FILE_CL_OPTIONS
        "$<$<CONFIG:DebugAlpha>:"
            "/QA21164;/MTd;/QA21164;/MTd"
        ">"
        "$<$<CONFIG:ReleaseAlpha>:"
            "/QA21164;/QA21164"
        ">"
    )
    source_file_compile_options(../../../../Projects/PERSONAL/q2-rust/game/p_trail.c ${FILE_CL_OPTIONS})
    string(CONCAT FILE_CL_OPTIONS
        "$<$<CONFIG:DebugAlpha>:"
            "/QA21164;/MTd;/QA21164;/MTd"
        ">"
        "$<$<CONFIG:ReleaseAlpha>:"
            "/QA21164;/QA21164"
        ">"
    )
    source_file_compile_options(../../../../Projects/PERSONAL/q2-rust/game/p_view.c ${FILE_CL_OPTIONS})
    string(CONCAT FILE_CL_OPTIONS
        "$<$<CONFIG:DebugAlpha>:"
            "/QA21164;/MTd;/QA21164;/MTd"
        ">"
        "$<$<CONFIG:ReleaseAlpha>:"
            "/QA21164;/QA21164"
        ">"
    )
    source_file_compile_options(../../../../Projects/PERSONAL/q2-rust/game/p_weapon.c ${FILE_CL_OPTIONS})
    string(CONCAT FILE_CL_OPTIONS
        "$<$<CONFIG:DebugAlpha>:"
            "/QA21164;/MTd;/QA21164;/MTd"
        ">"
        "$<$<CONFIG:ReleaseAlpha>:"
            "/QA21164;/QA21164"
        ">"
    )
    source_file_compile_options(../../../../Projects/PERSONAL/q2-rust/game/q_shared.c ${FILE_CL_OPTIONS})
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
        "kernel32;"
        "user32;"
        "gdi32;"
        "winspool;"
        "comdlg32;"
        "advapi32;"
        "shell32;"
        "ole32;"
        "oleaut32;"
        "uuid"
    ">"
    "$<$<CONFIG:Debug>:"
        "winmm"
    ">"
    "$<$<CONFIG:ReleaseAlpha>:"
        "kernel32;"
        "user32;"
        "gdi32"
    ">"
    "$<$<CONFIG:Release>:"
        "winmm"
    ">"
)
target_link_libraries(${PROJECT_NAME} PUBLIC "${ADDITIONAL_LIBRARY_DEPENDENCIES}")

