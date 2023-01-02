#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(extern_types, register_tool)]
extern "C" {
    pub type image_s;
    pub type model_s;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn putenv(__string: *mut libc::c_char) -> libc::c_int;
    fn Com_sprintf(
        dest: *mut libc::c_char,
        size: libc::c_int,
        fmt: *mut libc::c_char,
        _: ...
    );
    fn Cvar_Get(
        var_name: *mut libc::c_char,
        value: *mut libc::c_char,
        flags: libc::c_int,
    ) -> *mut cvar_t;
    fn Cvar_Set(var_name: *mut libc::c_char, value: *mut libc::c_char) -> *mut cvar_t;
    fn Cvar_SetValue(var_name: *mut libc::c_char, value: libc::c_float);
    static mut viddef: viddef_t;
    static mut scr_viewsize: *mut cvar_t;
    static mut re: refexport_t;
    fn M_ForceMenuOff();
    fn Menu_AddItem(menu: *mut menuframework_s, item: *mut libc::c_void);
    fn Menu_AdjustCursor(menu: *mut menuframework_s, dir: libc::c_int);
    fn Menu_Center(menu: *mut menuframework_s);
    fn Menu_Draw(menu: *mut menuframework_s);
    fn Menu_SelectItem(s: *mut menuframework_s) -> qboolean;
    fn Menu_SlideItem(s: *mut menuframework_s, dir: libc::c_int);
    static mut vid_ref: *mut cvar_t;
    static mut vid_fullscreen: *mut cvar_t;
    static mut vid_gamma: *mut cvar_t;
}
pub type byte = libc::c_uchar;
pub type qboolean = libc::c_uint;
pub const true_0: qboolean = 1;
pub const false_0: qboolean = 0;
pub type vec_t = libc::c_float;
pub type vec3_t = [vec_t; 3];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cvar_s {
    pub name: *mut libc::c_char,
    pub string: *mut libc::c_char,
    pub latched_string: *mut libc::c_char,
    pub flags: libc::c_int,
    pub modified: qboolean,
    pub value: libc::c_float,
    pub next: *mut cvar_s,
}
pub type cvar_t = cvar_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct entity_s {
    pub model: *mut model_s,
    pub angles: [libc::c_float; 3],
    pub origin: [libc::c_float; 3],
    pub frame: libc::c_int,
    pub oldorigin: [libc::c_float; 3],
    pub oldframe: libc::c_int,
    pub backlerp: libc::c_float,
    pub skinnum: libc::c_int,
    pub lightstyle: libc::c_int,
    pub alpha: libc::c_float,
    pub skin: *mut image_s,
    pub flags: libc::c_int,
}
pub type entity_t = entity_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dlight_t {
    pub origin: vec3_t,
    pub color: vec3_t,
    pub intensity: libc::c_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct particle_t {
    pub origin: vec3_t,
    pub color: libc::c_int,
    pub alpha: libc::c_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lightstyle_t {
    pub rgb: [libc::c_float; 3],
    pub white: libc::c_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct refdef_t {
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub fov_x: libc::c_float,
    pub fov_y: libc::c_float,
    pub vieworg: [libc::c_float; 3],
    pub viewangles: [libc::c_float; 3],
    pub blend: [libc::c_float; 4],
    pub time: libc::c_float,
    pub rdflags: libc::c_int,
    pub areabits: *mut byte,
    pub lightstyles: *mut lightstyle_t,
    pub num_entities: libc::c_int,
    pub entities: *mut entity_t,
    pub num_dlights: libc::c_int,
    pub dlights: *mut dlight_t,
    pub num_particles: libc::c_int,
    pub particles: *mut particle_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct refexport_t {
    pub api_version: libc::c_int,
    pub Init: Option::<
        unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> qboolean,
    >,
    pub Shutdown: Option::<unsafe extern "C" fn() -> ()>,
    pub BeginRegistration: Option::<unsafe extern "C" fn(*mut libc::c_char) -> ()>,
    pub RegisterModel: Option::<unsafe extern "C" fn(*mut libc::c_char) -> *mut model_s>,
    pub RegisterSkin: Option::<unsafe extern "C" fn(*mut libc::c_char) -> *mut image_s>,
    pub RegisterPic: Option::<unsafe extern "C" fn(*mut libc::c_char) -> *mut image_s>,
    pub SetSky: Option::<
        unsafe extern "C" fn(*mut libc::c_char, libc::c_float, *mut vec_t) -> (),
    >,
    pub EndRegistration: Option::<unsafe extern "C" fn() -> ()>,
    pub RenderFrame: Option::<unsafe extern "C" fn(*mut refdef_t) -> ()>,
    pub DrawGetPicSize: Option::<
        unsafe extern "C" fn(*mut libc::c_int, *mut libc::c_int, *mut libc::c_char) -> (),
    >,
    pub DrawPic: Option::<
        unsafe extern "C" fn(libc::c_int, libc::c_int, *mut libc::c_char) -> (),
    >,
    pub DrawStretchPic: Option::<
        unsafe extern "C" fn(
            libc::c_int,
            libc::c_int,
            libc::c_int,
            libc::c_int,
            *mut libc::c_char,
        ) -> (),
    >,
    pub DrawChar: Option::<
        unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int) -> (),
    >,
    pub DrawTileClear: Option::<
        unsafe extern "C" fn(
            libc::c_int,
            libc::c_int,
            libc::c_int,
            libc::c_int,
            *mut libc::c_char,
        ) -> (),
    >,
    pub DrawFill: Option::<
        unsafe extern "C" fn(
            libc::c_int,
            libc::c_int,
            libc::c_int,
            libc::c_int,
            libc::c_int,
        ) -> (),
    >,
    pub DrawFadeScreen: Option::<unsafe extern "C" fn() -> ()>,
    pub DrawStretchRaw: Option::<
        unsafe extern "C" fn(
            libc::c_int,
            libc::c_int,
            libc::c_int,
            libc::c_int,
            libc::c_int,
            libc::c_int,
            *mut byte,
        ) -> (),
    >,
    pub CinematicSetPalette: Option::<unsafe extern "C" fn(*const libc::c_uchar) -> ()>,
    pub BeginFrame: Option::<unsafe extern "C" fn(libc::c_float) -> ()>,
    pub EndFrame: Option::<unsafe extern "C" fn() -> ()>,
    pub AppActivate: Option::<unsafe extern "C" fn(qboolean) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct viddef_t {
    pub width: libc::c_int,
    pub height: libc::c_int,
}
pub type menuframework_s = _tag_menuframework;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _tag_menuframework {
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub cursor: libc::c_int,
    pub nitems: libc::c_int,
    pub nslots: libc::c_int,
    pub items: [*mut libc::c_void; 64],
    pub statusbar: *const libc::c_char,
    pub cursordraw: Option::<unsafe extern "C" fn(*mut _tag_menuframework) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct menuaction_s {
    pub generic: menucommon_s,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct menucommon_s {
    pub type_0: libc::c_int,
    pub name: *const libc::c_char,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub parent: *mut menuframework_s,
    pub cursor_offset: libc::c_int,
    pub localdata: [libc::c_int; 4],
    pub flags: libc::c_uint,
    pub statusbar: *const libc::c_char,
    pub callback: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub statusbarfunc: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub ownerdraw: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub cursordraw: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct menulist_s {
    pub generic: menucommon_s,
    pub curvalue: libc::c_int,
    pub itemnames: *mut *const libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct menuslider_s {
    pub generic: menucommon_s,
    pub minvalue: libc::c_float,
    pub maxvalue: libc::c_float,
    pub curvalue: libc::c_float,
    pub range: libc::c_float,
}
static mut gl_mode: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
static mut gl_driver: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
static mut gl_picmip: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
static mut gl_ext_palettedtexture: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
static mut gl_finish: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
static mut sw_mode: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
static mut sw_stipplealpha: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
static mut s_software_menu: menuframework_s = menuframework_s {
    x: 0,
    y: 0,
    cursor: 0,
    nitems: 0,
    nslots: 0,
    items: [0 as *const libc::c_void as *mut libc::c_void; 64],
    statusbar: 0 as *const libc::c_char,
    cursordraw: None,
};
static mut s_opengl_menu: menuframework_s = menuframework_s {
    x: 0,
    y: 0,
    cursor: 0,
    nitems: 0,
    nslots: 0,
    items: [0 as *const libc::c_void as *mut libc::c_void; 64],
    statusbar: 0 as *const libc::c_char,
    cursordraw: None,
};
static mut s_current_menu: *mut menuframework_s = 0 as *const menuframework_s
    as *mut menuframework_s;
static mut s_current_menu_index: libc::c_int = 0;
static mut s_mode_list: [menulist_s; 2] = [menulist_s {
    generic: menucommon_s {
        type_0: 0,
        name: 0 as *const libc::c_char,
        x: 0,
        y: 0,
        parent: 0 as *const menuframework_s as *mut menuframework_s,
        cursor_offset: 0,
        localdata: [0; 4],
        flags: 0,
        statusbar: 0 as *const libc::c_char,
        callback: None,
        statusbarfunc: None,
        ownerdraw: None,
        cursordraw: None,
    },
    curvalue: 0,
    itemnames: 0 as *const *const libc::c_char as *mut *const libc::c_char,
}; 2];
static mut s_ref_list: [menulist_s; 2] = [menulist_s {
    generic: menucommon_s {
        type_0: 0,
        name: 0 as *const libc::c_char,
        x: 0,
        y: 0,
        parent: 0 as *const menuframework_s as *mut menuframework_s,
        cursor_offset: 0,
        localdata: [0; 4],
        flags: 0,
        statusbar: 0 as *const libc::c_char,
        callback: None,
        statusbarfunc: None,
        ownerdraw: None,
        cursordraw: None,
    },
    curvalue: 0,
    itemnames: 0 as *const *const libc::c_char as *mut *const libc::c_char,
}; 2];
static mut s_tq_slider: menuslider_s = menuslider_s {
    generic: menucommon_s {
        type_0: 0,
        name: 0 as *const libc::c_char,
        x: 0,
        y: 0,
        parent: 0 as *const menuframework_s as *mut menuframework_s,
        cursor_offset: 0,
        localdata: [0; 4],
        flags: 0,
        statusbar: 0 as *const libc::c_char,
        callback: None,
        statusbarfunc: None,
        ownerdraw: None,
        cursordraw: None,
    },
    minvalue: 0.,
    maxvalue: 0.,
    curvalue: 0.,
    range: 0.,
};
static mut s_screensize_slider: [menuslider_s; 2] = [menuslider_s {
    generic: menucommon_s {
        type_0: 0,
        name: 0 as *const libc::c_char,
        x: 0,
        y: 0,
        parent: 0 as *const menuframework_s as *mut menuframework_s,
        cursor_offset: 0,
        localdata: [0; 4],
        flags: 0,
        statusbar: 0 as *const libc::c_char,
        callback: None,
        statusbarfunc: None,
        ownerdraw: None,
        cursordraw: None,
    },
    minvalue: 0.,
    maxvalue: 0.,
    curvalue: 0.,
    range: 0.,
}; 2];
static mut s_brightness_slider: [menuslider_s; 2] = [menuslider_s {
    generic: menucommon_s {
        type_0: 0,
        name: 0 as *const libc::c_char,
        x: 0,
        y: 0,
        parent: 0 as *const menuframework_s as *mut menuframework_s,
        cursor_offset: 0,
        localdata: [0; 4],
        flags: 0,
        statusbar: 0 as *const libc::c_char,
        callback: None,
        statusbarfunc: None,
        ownerdraw: None,
        cursordraw: None,
    },
    minvalue: 0.,
    maxvalue: 0.,
    curvalue: 0.,
    range: 0.,
}; 2];
static mut s_fs_box: [menulist_s; 2] = [menulist_s {
    generic: menucommon_s {
        type_0: 0,
        name: 0 as *const libc::c_char,
        x: 0,
        y: 0,
        parent: 0 as *const menuframework_s as *mut menuframework_s,
        cursor_offset: 0,
        localdata: [0; 4],
        flags: 0,
        statusbar: 0 as *const libc::c_char,
        callback: None,
        statusbarfunc: None,
        ownerdraw: None,
        cursordraw: None,
    },
    curvalue: 0,
    itemnames: 0 as *const *const libc::c_char as *mut *const libc::c_char,
}; 2];
static mut s_stipple_box: menulist_s = menulist_s {
    generic: menucommon_s {
        type_0: 0,
        name: 0 as *const libc::c_char,
        x: 0,
        y: 0,
        parent: 0 as *const menuframework_s as *mut menuframework_s,
        cursor_offset: 0,
        localdata: [0; 4],
        flags: 0,
        statusbar: 0 as *const libc::c_char,
        callback: None,
        statusbarfunc: None,
        ownerdraw: None,
        cursordraw: None,
    },
    curvalue: 0,
    itemnames: 0 as *const *const libc::c_char as *mut *const libc::c_char,
};
static mut s_paletted_texture_box: menulist_s = menulist_s {
    generic: menucommon_s {
        type_0: 0,
        name: 0 as *const libc::c_char,
        x: 0,
        y: 0,
        parent: 0 as *const menuframework_s as *mut menuframework_s,
        cursor_offset: 0,
        localdata: [0; 4],
        flags: 0,
        statusbar: 0 as *const libc::c_char,
        callback: None,
        statusbarfunc: None,
        ownerdraw: None,
        cursordraw: None,
    },
    curvalue: 0,
    itemnames: 0 as *const *const libc::c_char as *mut *const libc::c_char,
};
static mut s_finish_box: menulist_s = menulist_s {
    generic: menucommon_s {
        type_0: 0,
        name: 0 as *const libc::c_char,
        x: 0,
        y: 0,
        parent: 0 as *const menuframework_s as *mut menuframework_s,
        cursor_offset: 0,
        localdata: [0; 4],
        flags: 0,
        statusbar: 0 as *const libc::c_char,
        callback: None,
        statusbarfunc: None,
        ownerdraw: None,
        cursordraw: None,
    },
    curvalue: 0,
    itemnames: 0 as *const *const libc::c_char as *mut *const libc::c_char,
};
static mut s_cancel_action: [menuaction_s; 2] = [menuaction_s {
    generic: menucommon_s {
        type_0: 0,
        name: 0 as *const libc::c_char,
        x: 0,
        y: 0,
        parent: 0 as *const menuframework_s as *mut menuframework_s,
        cursor_offset: 0,
        localdata: [0; 4],
        flags: 0,
        statusbar: 0 as *const libc::c_char,
        callback: None,
        statusbarfunc: None,
        ownerdraw: None,
        cursordraw: None,
    },
}; 2];
static mut s_defaults_action: [menuaction_s; 2] = [menuaction_s {
    generic: menucommon_s {
        type_0: 0,
        name: 0 as *const libc::c_char,
        x: 0,
        y: 0,
        parent: 0 as *const menuframework_s as *mut menuframework_s,
        cursor_offset: 0,
        localdata: [0; 4],
        flags: 0,
        statusbar: 0 as *const libc::c_char,
        callback: None,
        statusbarfunc: None,
        ownerdraw: None,
        cursordraw: None,
    },
}; 2];
unsafe extern "C" fn DriverCallback(mut unused: *mut libc::c_void) {
    s_ref_list[(s_current_menu_index == 0) as libc::c_int as usize]
        .curvalue = s_ref_list[s_current_menu_index as usize].curvalue;
    if s_ref_list[s_current_menu_index as usize].curvalue == 0 as libc::c_int {
        s_current_menu = &mut s_software_menu;
        s_current_menu_index = 0 as libc::c_int;
    } else {
        s_current_menu = &mut s_opengl_menu;
        s_current_menu_index = 1 as libc::c_int;
    };
}
unsafe extern "C" fn ScreenSizeCallback(mut s: *mut libc::c_void) {
    let mut slider: *mut menuslider_s = s as *mut menuslider_s;
    Cvar_SetValue(
        b"viewsize\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        (*slider).curvalue * 10 as libc::c_int as libc::c_float,
    );
}
unsafe extern "C" fn BrightnessCallback(mut s: *mut libc::c_void) {
    let mut slider: *mut menuslider_s = s as *mut menuslider_s;
    if s_current_menu_index == 0 as libc::c_int {
        s_brightness_slider[1 as libc::c_int as usize]
            .curvalue = s_brightness_slider[0 as libc::c_int as usize].curvalue;
    } else {
        s_brightness_slider[0 as libc::c_int as usize]
            .curvalue = s_brightness_slider[1 as libc::c_int as usize].curvalue;
    }
    if stricmp((*vid_ref).string, b"soft\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        let mut gamma: libc::c_float = (0.8f64
            - ((*slider).curvalue as libc::c_double / 10.0f64 - 0.5f64) + 0.5f64)
            as libc::c_float;
        Cvar_SetValue(
            b"vid_gamma\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            gamma,
        );
    }
}
unsafe extern "C" fn ResetDefaults(mut unused: *mut libc::c_void) {
    VID_MenuInit();
}
unsafe extern "C" fn ApplyChanges(mut unused: *mut libc::c_void) {
    let mut gamma: libc::c_float = 0.;
    s_fs_box[(s_current_menu_index == 0) as libc::c_int as usize]
        .curvalue = s_fs_box[s_current_menu_index as usize].curvalue;
    s_brightness_slider[(s_current_menu_index == 0) as libc::c_int as usize]
        .curvalue = s_brightness_slider[s_current_menu_index as usize].curvalue;
    s_ref_list[(s_current_menu_index == 0) as libc::c_int as usize]
        .curvalue = s_ref_list[s_current_menu_index as usize].curvalue;
    gamma = (0.8f64
        - (s_brightness_slider[s_current_menu_index as usize].curvalue as libc::c_double
            / 10.0f64 - 0.5f64) + 0.5f64) as libc::c_float;
    Cvar_SetValue(
        b"vid_gamma\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        gamma,
    );
    Cvar_SetValue(
        b"sw_stipplealpha\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        s_stipple_box.curvalue as libc::c_float,
    );
    Cvar_SetValue(
        b"gl_picmip\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        3 as libc::c_int as libc::c_float - s_tq_slider.curvalue,
    );
    Cvar_SetValue(
        b"vid_fullscreen\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        s_fs_box[s_current_menu_index as usize].curvalue as libc::c_float,
    );
    Cvar_SetValue(
        b"gl_ext_palettedtexture\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        s_paletted_texture_box.curvalue as libc::c_float,
    );
    Cvar_SetValue(
        b"gl_finish\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        s_finish_box.curvalue as libc::c_float,
    );
    Cvar_SetValue(
        b"sw_mode\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        s_mode_list[0 as libc::c_int as usize].curvalue as libc::c_float,
    );
    Cvar_SetValue(
        b"gl_mode\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        s_mode_list[1 as libc::c_int as usize].curvalue as libc::c_float,
    );
    match s_ref_list[s_current_menu_index as usize].curvalue {
        0 => {
            Cvar_Set(
                b"vid_ref\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                b"soft\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
        }
        1 => {
            Cvar_Set(
                b"vid_ref\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                b"gl\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            Cvar_Set(
                b"gl_driver\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                b"opengl32\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
        }
        2 => {
            Cvar_Set(
                b"vid_ref\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                b"gl\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            Cvar_Set(
                b"gl_driver\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                b"3dfxgl\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
        }
        3 => {
            Cvar_Set(
                b"vid_ref\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                b"gl\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            Cvar_Set(
                b"gl_driver\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                b"pvrgl\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
        }
        4 => {
            Cvar_Set(
                b"vid_ref\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                b"gl\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            Cvar_Set(
                b"gl_driver\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                b"veritegl\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
        }
        _ => {}
    }
    if stricmp((*vid_ref).string, b"gl\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        if (*vid_gamma).modified as u64 != 0 {
            (*vid_ref).modified = true_0;
            if stricmp(
                (*gl_driver).string,
                b"3dfxgl\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
            {
                let mut envbuffer: [libc::c_char; 1024] = [0; 1024];
                let mut g: libc::c_float = 0.;
                (*vid_ref).modified = true_0;
                g = (2.00f64 * (0.8f64 - ((*vid_gamma).value as libc::c_double - 0.5f64))
                    + 1.0f32 as libc::c_double) as libc::c_float;
                Com_sprintf(
                    envbuffer.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong
                        as libc::c_int,
                    b"SSTV2_GAMMA=%f\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    g as libc::c_double,
                );
                putenv(envbuffer.as_mut_ptr());
                Com_sprintf(
                    envbuffer.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong
                        as libc::c_int,
                    b"SST_GAMMA=%f\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    g as libc::c_double,
                );
                putenv(envbuffer.as_mut_ptr());
                (*vid_gamma).modified = false_0;
            }
        }
        if (*gl_driver).modified as u64 != 0 {
            (*vid_ref).modified = true_0;
        }
    }
    M_ForceMenuOff();
}
unsafe extern "C" fn CancelChanges(mut unused: *mut libc::c_void) {
    extern "C" {
        #[link_name = "M_PopMenu"]
        fn M_PopMenu_0();
    }
    M_PopMenu_0();
}
#[no_mangle]
pub unsafe extern "C" fn VID_MenuInit() {
    static mut resolutions: [*const libc::c_char; 11] = [
        b"[320 240  ]\0" as *const u8 as *const libc::c_char,
        b"[400 300  ]\0" as *const u8 as *const libc::c_char,
        b"[512 384  ]\0" as *const u8 as *const libc::c_char,
        b"[640 480  ]\0" as *const u8 as *const libc::c_char,
        b"[800 600  ]\0" as *const u8 as *const libc::c_char,
        b"[960 720  ]\0" as *const u8 as *const libc::c_char,
        b"[1024 768 ]\0" as *const u8 as *const libc::c_char,
        b"[1152 864 ]\0" as *const u8 as *const libc::c_char,
        b"[1280 960 ]\0" as *const u8 as *const libc::c_char,
        b"[1600 1200]\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    static mut refs: [*const libc::c_char; 5] = [
        b"[software      ]\0" as *const u8 as *const libc::c_char,
        b"[default OpenGL]\0" as *const u8 as *const libc::c_char,
        b"[3Dfx OpenGL   ]\0" as *const u8 as *const libc::c_char,
        b"[PowerVR OpenGL]\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    static mut yesno_names: [*const libc::c_char; 3] = [
        b"no\0" as *const u8 as *const libc::c_char,
        b"yes\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut i: libc::c_int = 0;
    if gl_driver.is_null() {
        gl_driver = Cvar_Get(
            b"gl_driver\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"opengl32\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            0 as libc::c_int,
        );
    }
    if gl_picmip.is_null() {
        gl_picmip = Cvar_Get(
            b"gl_picmip\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            0 as libc::c_int,
        );
    }
    if gl_mode.is_null() {
        gl_mode = Cvar_Get(
            b"gl_mode\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"3\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            0 as libc::c_int,
        );
    }
    if sw_mode.is_null() {
        sw_mode = Cvar_Get(
            b"sw_mode\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            0 as libc::c_int,
        );
    }
    if gl_ext_palettedtexture.is_null() {
        gl_ext_palettedtexture = Cvar_Get(
            b"gl_ext_palettedtexture\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            b"1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            1 as libc::c_int,
        );
    }
    if gl_finish.is_null() {
        gl_finish = Cvar_Get(
            b"gl_finish\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            1 as libc::c_int,
        );
    }
    if sw_stipplealpha.is_null() {
        sw_stipplealpha = Cvar_Get(
            b"sw_stipplealpha\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            1 as libc::c_int,
        );
    }
    s_mode_list[0 as libc::c_int as usize].curvalue = (*sw_mode).value as libc::c_int;
    s_mode_list[1 as libc::c_int as usize].curvalue = (*gl_mode).value as libc::c_int;
    if scr_viewsize.is_null() {
        scr_viewsize = Cvar_Get(
            b"viewsize\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"100\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            1 as libc::c_int,
        );
    }
    s_screensize_slider[0 as libc::c_int as usize]
        .curvalue = (*scr_viewsize).value / 10 as libc::c_int as libc::c_float;
    s_screensize_slider[1 as libc::c_int as usize]
        .curvalue = (*scr_viewsize).value / 10 as libc::c_int as libc::c_float;
    if strcmp((*vid_ref).string, b"soft\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        s_current_menu_index = 0 as libc::c_int;
        s_ref_list[1 as libc::c_int as usize].curvalue = 0 as libc::c_int;
        s_ref_list[0 as libc::c_int as usize]
            .curvalue = s_ref_list[1 as libc::c_int as usize].curvalue;
    } else if strcmp((*vid_ref).string, b"gl\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        s_current_menu_index = 1 as libc::c_int;
        if strcmp((*gl_driver).string, b"3dfxgl\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            s_ref_list[s_current_menu_index as usize].curvalue = 2 as libc::c_int;
        } else if strcmp(
            (*gl_driver).string,
            b"pvrgl\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            s_ref_list[s_current_menu_index as usize].curvalue = 3 as libc::c_int;
        } else if strcmp(
            (*gl_driver).string,
            b"opengl32\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            s_ref_list[s_current_menu_index as usize].curvalue = 1 as libc::c_int;
        } else {
            s_ref_list[s_current_menu_index as usize].curvalue = 1 as libc::c_int;
        }
    }
    s_software_menu.x = (viddef.width as libc::c_double * 0.50f64) as libc::c_int;
    s_software_menu.nitems = 0 as libc::c_int;
    s_opengl_menu.x = (viddef.width as libc::c_double * 0.50f64) as libc::c_int;
    s_opengl_menu.nitems = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < 2 as libc::c_int {
        s_ref_list[i as usize].generic.type_0 = 3 as libc::c_int;
        s_ref_list[i as usize]
            .generic
            .name = b"driver\0" as *const u8 as *const libc::c_char;
        s_ref_list[i as usize].generic.x = 0 as libc::c_int;
        s_ref_list[i as usize].generic.y = 0 as libc::c_int;
        s_ref_list[i as usize]
            .generic
            .callback = Some(
            DriverCallback as unsafe extern "C" fn(*mut libc::c_void) -> (),
        );
        s_ref_list[i as usize].itemnames = refs.as_mut_ptr();
        s_mode_list[i as usize].generic.type_0 = 3 as libc::c_int;
        s_mode_list[i as usize]
            .generic
            .name = b"video mode\0" as *const u8 as *const libc::c_char;
        s_mode_list[i as usize].generic.x = 0 as libc::c_int;
        s_mode_list[i as usize].generic.y = 10 as libc::c_int;
        s_mode_list[i as usize].itemnames = resolutions.as_mut_ptr();
        s_screensize_slider[i as usize].generic.type_0 = 0 as libc::c_int;
        s_screensize_slider[i as usize].generic.x = 0 as libc::c_int;
        s_screensize_slider[i as usize].generic.y = 20 as libc::c_int;
        s_screensize_slider[i as usize]
            .generic
            .name = b"screen size\0" as *const u8 as *const libc::c_char;
        s_screensize_slider[i as usize].minvalue = 3 as libc::c_int as libc::c_float;
        s_screensize_slider[i as usize].maxvalue = 12 as libc::c_int as libc::c_float;
        s_screensize_slider[i as usize]
            .generic
            .callback = Some(
            ScreenSizeCallback as unsafe extern "C" fn(*mut libc::c_void) -> (),
        );
        s_brightness_slider[i as usize].generic.type_0 = 0 as libc::c_int;
        s_brightness_slider[i as usize].generic.x = 0 as libc::c_int;
        s_brightness_slider[i as usize].generic.y = 30 as libc::c_int;
        s_brightness_slider[i as usize]
            .generic
            .name = b"brightness\0" as *const u8 as *const libc::c_char;
        s_brightness_slider[i as usize]
            .generic
            .callback = Some(
            BrightnessCallback as unsafe extern "C" fn(*mut libc::c_void) -> (),
        );
        s_brightness_slider[i as usize].minvalue = 5 as libc::c_int as libc::c_float;
        s_brightness_slider[i as usize].maxvalue = 13 as libc::c_int as libc::c_float;
        s_brightness_slider[i as usize]
            .curvalue = ((1.3f64 - (*vid_gamma).value as libc::c_double + 0.5f64)
            * 10 as libc::c_int as libc::c_double) as libc::c_float;
        s_fs_box[i as usize].generic.type_0 = 3 as libc::c_int;
        s_fs_box[i as usize].generic.x = 0 as libc::c_int;
        s_fs_box[i as usize].generic.y = 40 as libc::c_int;
        s_fs_box[i as usize]
            .generic
            .name = b"fullscreen\0" as *const u8 as *const libc::c_char;
        s_fs_box[i as usize].itemnames = yesno_names.as_mut_ptr();
        s_fs_box[i as usize].curvalue = (*vid_fullscreen).value as libc::c_int;
        s_defaults_action[i as usize].generic.type_0 = 2 as libc::c_int;
        s_defaults_action[i as usize]
            .generic
            .name = b"reset to defaults\0" as *const u8 as *const libc::c_char;
        s_defaults_action[i as usize].generic.x = 0 as libc::c_int;
        s_defaults_action[i as usize].generic.y = 90 as libc::c_int;
        s_defaults_action[i as usize]
            .generic
            .callback = Some(
            ResetDefaults as unsafe extern "C" fn(*mut libc::c_void) -> (),
        );
        s_cancel_action[i as usize].generic.type_0 = 2 as libc::c_int;
        s_cancel_action[i as usize]
            .generic
            .name = b"cancel\0" as *const u8 as *const libc::c_char;
        s_cancel_action[i as usize].generic.x = 0 as libc::c_int;
        s_cancel_action[i as usize].generic.y = 100 as libc::c_int;
        s_cancel_action[i as usize]
            .generic
            .callback = Some(
            CancelChanges as unsafe extern "C" fn(*mut libc::c_void) -> (),
        );
        i += 1;
    }
    s_stipple_box.generic.type_0 = 3 as libc::c_int;
    s_stipple_box.generic.x = 0 as libc::c_int;
    s_stipple_box.generic.y = 60 as libc::c_int;
    s_stipple_box.generic.name = b"stipple alpha\0" as *const u8 as *const libc::c_char;
    s_stipple_box.curvalue = (*sw_stipplealpha).value as libc::c_int;
    s_stipple_box.itemnames = yesno_names.as_mut_ptr();
    s_tq_slider.generic.type_0 = 0 as libc::c_int;
    s_tq_slider.generic.x = 0 as libc::c_int;
    s_tq_slider.generic.y = 60 as libc::c_int;
    s_tq_slider.generic.name = b"texture quality\0" as *const u8 as *const libc::c_char;
    s_tq_slider.minvalue = 0 as libc::c_int as libc::c_float;
    s_tq_slider.maxvalue = 3 as libc::c_int as libc::c_float;
    s_tq_slider.curvalue = 3 as libc::c_int as libc::c_float - (*gl_picmip).value;
    s_paletted_texture_box.generic.type_0 = 3 as libc::c_int;
    s_paletted_texture_box.generic.x = 0 as libc::c_int;
    s_paletted_texture_box.generic.y = 70 as libc::c_int;
    s_paletted_texture_box
        .generic
        .name = b"8-bit textures\0" as *const u8 as *const libc::c_char;
    s_paletted_texture_box.itemnames = yesno_names.as_mut_ptr();
    s_paletted_texture_box.curvalue = (*gl_ext_palettedtexture).value as libc::c_int;
    s_finish_box.generic.type_0 = 3 as libc::c_int;
    s_finish_box.generic.x = 0 as libc::c_int;
    s_finish_box.generic.y = 80 as libc::c_int;
    s_finish_box
        .generic
        .name = b"sync every frame\0" as *const u8 as *const libc::c_char;
    s_finish_box.curvalue = (*gl_finish).value as libc::c_int;
    s_finish_box.itemnames = yesno_names.as_mut_ptr();
    Menu_AddItem(
        &mut s_software_menu,
        &mut *s_ref_list.as_mut_ptr().offset(0 as libc::c_int as isize)
            as *mut menulist_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_software_menu,
        &mut *s_mode_list.as_mut_ptr().offset(0 as libc::c_int as isize)
            as *mut menulist_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_software_menu,
        &mut *s_screensize_slider.as_mut_ptr().offset(0 as libc::c_int as isize)
            as *mut menuslider_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_software_menu,
        &mut *s_brightness_slider.as_mut_ptr().offset(0 as libc::c_int as isize)
            as *mut menuslider_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_software_menu,
        &mut *s_fs_box.as_mut_ptr().offset(0 as libc::c_int as isize) as *mut menulist_s
            as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_software_menu,
        &mut s_stipple_box as *mut menulist_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_opengl_menu,
        &mut *s_ref_list.as_mut_ptr().offset(1 as libc::c_int as isize)
            as *mut menulist_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_opengl_menu,
        &mut *s_mode_list.as_mut_ptr().offset(1 as libc::c_int as isize)
            as *mut menulist_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_opengl_menu,
        &mut *s_screensize_slider.as_mut_ptr().offset(1 as libc::c_int as isize)
            as *mut menuslider_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_opengl_menu,
        &mut *s_brightness_slider.as_mut_ptr().offset(1 as libc::c_int as isize)
            as *mut menuslider_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_opengl_menu,
        &mut *s_fs_box.as_mut_ptr().offset(1 as libc::c_int as isize) as *mut menulist_s
            as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_opengl_menu,
        &mut s_tq_slider as *mut menuslider_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_opengl_menu,
        &mut s_paletted_texture_box as *mut menulist_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_opengl_menu,
        &mut s_finish_box as *mut menulist_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_software_menu,
        &mut *s_defaults_action.as_mut_ptr().offset(0 as libc::c_int as isize)
            as *mut menuaction_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_software_menu,
        &mut *s_cancel_action.as_mut_ptr().offset(0 as libc::c_int as isize)
            as *mut menuaction_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_opengl_menu,
        &mut *s_defaults_action.as_mut_ptr().offset(1 as libc::c_int as isize)
            as *mut menuaction_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_opengl_menu,
        &mut *s_cancel_action.as_mut_ptr().offset(1 as libc::c_int as isize)
            as *mut menuaction_s as *mut libc::c_void,
    );
    Menu_Center(&mut s_software_menu);
    Menu_Center(&mut s_opengl_menu);
    s_opengl_menu.x -= 8 as libc::c_int;
    s_software_menu.x -= 8 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn VID_MenuDraw() {
    let mut w: libc::c_int = 0;
    let mut h: libc::c_int = 0;
    if s_current_menu_index == 0 as libc::c_int {
        s_current_menu = &mut s_software_menu;
    } else {
        s_current_menu = &mut s_opengl_menu;
    }
    (re.DrawGetPicSize)
        .expect(
            "non-null function pointer",
        )(
        &mut w,
        &mut h,
        b"m_banner_video\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    (re.DrawPic)
        .expect(
            "non-null function pointer",
        )(
        viddef.width / 2 as libc::c_int - w / 2 as libc::c_int,
        viddef.height / 2 as libc::c_int - 110 as libc::c_int,
        b"m_banner_video\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    Menu_AdjustCursor(s_current_menu, 1 as libc::c_int);
    Menu_Draw(s_current_menu);
}
#[no_mangle]
pub unsafe extern "C" fn VID_MenuKey(mut key: libc::c_int) -> *const libc::c_char {
    let mut m: *mut menuframework_s = s_current_menu;
    static mut sound: *const libc::c_char = b"misc/menu1.wav\0" as *const u8
        as *const libc::c_char;
    match key {
        27 => {
            ApplyChanges(0 as *mut libc::c_void);
            return 0 as *const libc::c_char;
        }
        161 | 128 => {
            let ref mut fresh0 = (*m).cursor;
            *fresh0 -= 1;
            Menu_AdjustCursor(m, -(1 as libc::c_int));
        }
        167 | 129 => {
            let ref mut fresh1 = (*m).cursor;
            *fresh1 += 1;
            Menu_AdjustCursor(m, 1 as libc::c_int);
        }
        163 | 130 => {
            Menu_SlideItem(m, -(1 as libc::c_int));
        }
        165 | 131 => {
            Menu_SlideItem(m, 1 as libc::c_int);
        }
        169 | 13 => {
            if Menu_SelectItem(m) as u64 == 0 {
                ApplyChanges(0 as *mut libc::c_void);
            }
        }
        _ => {}
    }
    return sound;
}
