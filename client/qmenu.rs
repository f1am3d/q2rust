#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(extern_types, register_tool)]

extern "C" {
    pub type image_s;
    pub type model_s;
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strtok(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn toupper(_: libc::c_int) -> libc::c_int;
    fn free(__ptr: *mut libc::c_void);
    fn Sys_Milliseconds() -> libc::c_int;
    fn Sys_GetClipboardData() -> *mut libc::c_char;
    static mut viddef: viddef_t;
    static mut re: refexport_t;
}

pub type C2RustUnnamed = libc::c_uint;

pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;

pub type byte = libc::c_uchar;
pub type qboolean = libc::c_uint;

pub const true_0: qboolean = 1;
pub const false_0: qboolean = 0;

pub type vec_t = libc::c_float;
pub type vec3_t = [vec_t; 3];

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

pub type menuframework_s = _tag_menuframework;

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
pub struct menufield_s {
    pub generic: menucommon_s,
    pub buffer: [libc::c_char; 80],
    pub cursor: libc::c_int,
    pub length: libc::c_int,
    pub visible_length: libc::c_int,
    pub visible_offset: libc::c_int,
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

#[derive(Copy, Clone)]
#[repr(C)]
pub struct menulist_s {
    pub generic: menucommon_s,
    pub curvalue: libc::c_int,
    pub itemnames: *mut *const libc::c_char,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct menuaction_s {
    pub generic: menucommon_s,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct menuseparator_s {
    pub generic: menucommon_s,
}

unsafe extern "C" fn Action_DoEnter(mut a: *mut menuaction_s) {
    if ((*a).generic.callback).is_some() {
        ((*a).generic.callback)
            .expect("non-null function pointer")(a as *mut libc::c_void);
    }
}

unsafe extern "C" fn Action_Draw(mut a: *mut menuaction_s) {
    if (*a).generic.flags & 0x1 as libc::c_int as libc::c_uint != 0 {
        if (*a).generic.flags & 0x2 as libc::c_int as libc::c_uint != 0 {
            Menu_DrawStringDark(
                (*a).generic.x + (*(*a).generic.parent).x + -(16 as libc::c_int),
                (*a).generic.y + (*(*a).generic.parent).y,
                (*a).generic.name,
            );
        } else {
            Menu_DrawString(
                (*a).generic.x + (*(*a).generic.parent).x + -(16 as libc::c_int),
                (*a).generic.y + (*(*a).generic.parent).y,
                (*a).generic.name,
            );
        }
    } else if (*a).generic.flags & 0x2 as libc::c_int as libc::c_uint != 0 {
        Menu_DrawStringR2LDark(
            (*a).generic.x + (*(*a).generic.parent).x + -(16 as libc::c_int),
            (*a).generic.y + (*(*a).generic.parent).y,
            (*a).generic.name,
        );
    } else {
        Menu_DrawStringR2L(
            (*a).generic.x + (*(*a).generic.parent).x + -(16 as libc::c_int),
            (*a).generic.y + (*(*a).generic.parent).y,
            (*a).generic.name,
        );
    }
    if ((*a).generic.ownerdraw).is_some() {
        ((*a).generic.ownerdraw)
            .expect("non-null function pointer")(a as *mut libc::c_void);
    }
}

#[no_mangle]
pub unsafe extern "C" fn Field_DoEnter(mut f: *mut menufield_s) -> qboolean {
    if ((*f).generic.callback).is_some() {
        ((*f).generic.callback)
            .expect("non-null function pointer")(f as *mut libc::c_void);
        return true_0;
    }
    return false_0;
}

#[no_mangle]
pub unsafe extern "C" fn Field_Draw(mut f: *mut menufield_s) {
    let mut i: libc::c_int = 0;
    let mut tempbuffer: [libc::c_char; 128] = *::std::mem::transmute::<
        &[u8; 128],
        &mut [libc::c_char; 128],
    >(
        b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
    );
    if !((*f).generic.name).is_null() {
        Menu_DrawStringR2LDark(
            (*f).generic.x + (*(*f).generic.parent).x + -(16 as libc::c_int),
            (*f).generic.y + (*(*f).generic.parent).y,
            (*f).generic.name,
        );
    }
    strncpy(
        tempbuffer.as_mut_ptr(),
        ((*f).buffer).as_mut_ptr().offset((*f).visible_offset as isize),
        (*f).visible_length as libc::c_ulong,
    );
    (re.DrawChar)
        .expect(
            "non-null function pointer",
        )(
        (*f).generic.x + (*(*f).generic.parent).x + 16 as libc::c_int,
        (*f).generic.y + (*(*f).generic.parent).y - 4 as libc::c_int,
        18 as libc::c_int,
    );
    (re.DrawChar)
        .expect(
            "non-null function pointer",
        )(
        (*f).generic.x + (*(*f).generic.parent).x + 16 as libc::c_int,
        (*f).generic.y + (*(*f).generic.parent).y + 4 as libc::c_int,
        24 as libc::c_int,
    );
    (re.DrawChar)
        .expect(
            "non-null function pointer",
        )(
        (*f).generic.x + (*(*f).generic.parent).x + 24 as libc::c_int
            + (*f).visible_length * 8 as libc::c_int,
        (*f).generic.y + (*(*f).generic.parent).y - 4 as libc::c_int,
        20 as libc::c_int,
    );
    (re.DrawChar)
        .expect(
            "non-null function pointer",
        )(
        (*f).generic.x + (*(*f).generic.parent).x + 24 as libc::c_int
            + (*f).visible_length * 8 as libc::c_int,
        (*f).generic.y + (*(*f).generic.parent).y + 4 as libc::c_int,
        26 as libc::c_int,
    );
    i = 0 as libc::c_int;
    while i < (*f).visible_length {
        (re.DrawChar)
            .expect(
                "non-null function pointer",
            )(
            (*f).generic.x + (*(*f).generic.parent).x + 24 as libc::c_int
                + i * 8 as libc::c_int,
            (*f).generic.y + (*(*f).generic.parent).y - 4 as libc::c_int,
            19 as libc::c_int,
        );
        (re.DrawChar)
            .expect(
                "non-null function pointer",
            )(
            (*f).generic.x + (*(*f).generic.parent).x + 24 as libc::c_int
                + i * 8 as libc::c_int,
            (*f).generic.y + (*(*f).generic.parent).y + 4 as libc::c_int,
            25 as libc::c_int,
        );
        i += 1;
    }
    Menu_DrawString(
        (*f).generic.x + (*(*f).generic.parent).x + 24 as libc::c_int,
        (*f).generic.y + (*(*f).generic.parent).y,
        tempbuffer.as_mut_ptr(),
    );
    if Menu_ItemAtCursor((*f).generic.parent) == f as *mut libc::c_void {
        let mut offset: libc::c_int = 0;
        if (*f).visible_offset != 0 {
            offset = (*f).visible_length;
        } else {
            offset = (*f).cursor;
        }
        if Sys_Milliseconds() / 250 as libc::c_int & 1 as libc::c_int != 0 {
            (re.DrawChar)
                .expect(
                    "non-null function pointer",
                )(
                (*f).generic.x + (*(*f).generic.parent).x
                    + (offset + 2 as libc::c_int) * 8 as libc::c_int + 8 as libc::c_int,
                (*f).generic.y + (*(*f).generic.parent).y,
                11 as libc::c_int,
            );
        } else {
            (re.DrawChar)
                .expect(
                    "non-null function pointer",
                )(
                (*f).generic.x + (*(*f).generic.parent).x
                    + (offset + 2 as libc::c_int) * 8 as libc::c_int + 8 as libc::c_int,
                (*f).generic.y + (*(*f).generic.parent).y,
                ' ' as i32,
            );
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn Field_Key(
    mut f: *mut menufield_s,
    mut key: libc::c_int,
) -> qboolean {
    extern "C" {
        static mut keydown: [libc::c_int; 0];
    }
    match key {
        172 => {
            key = '/' as i32;
        }
        173 => {
            key = '-' as i32;
        }
        174 => {
            key = '+' as i32;
        }
        160 => {
            key = '7' as i32;
        }
        161 => {
            key = '8' as i32;
        }
        162 => {
            key = '9' as i32;
        }
        163 => {
            key = '4' as i32;
        }
        164 => {
            key = '5' as i32;
        }
        165 => {
            key = '6' as i32;
        }
        166 => {
            key = '1' as i32;
        }
        167 => {
            key = '2' as i32;
        }
        168 => {
            key = '3' as i32;
        }
        170 => {
            key = '0' as i32;
        }
        171 => {
            key = '.' as i32;
        }
        _ => {}
    }
    if key > 127 as libc::c_int {
        match key {
            148 | _ => {}
        }
        return false_0;
    }
    if toupper(key) == 'V' as i32
        && *keydown.as_mut_ptr().offset(133 as libc::c_int as isize) != 0
        || (key == 147 as libc::c_int || key == 170 as libc::c_int)
        && *keydown.as_mut_ptr().offset(134 as libc::c_int as isize) != 0
    {
        let mut cbd: *mut libc::c_char = 0 as *mut libc::c_char;
        cbd = Sys_GetClipboardData();
        if !cbd.is_null() {
            strtok(cbd, b"\n\r\x08\0" as *const u8 as *const libc::c_char);
            strncpy(
                ((*f).buffer).as_mut_ptr(),
                cbd,
                ((*f).length - 1 as libc::c_int) as libc::c_ulong,
            );
            (*f).cursor = strlen(((*f).buffer).as_mut_ptr()) as libc::c_int;
            (*f).visible_offset = (*f).cursor - (*f).visible_length;
            if (*f).visible_offset < 0 as libc::c_int {
                (*f).visible_offset = 0 as libc::c_int;
            }
            free(cbd as *mut libc::c_void);
        }
        return true_0;
    }
    match key {
        163 | 130 | 127 => {
            if (*f).cursor > 0 as libc::c_int {
                memmove(
                    &mut *((*f).buffer)
                        .as_mut_ptr()
                        .offset(((*f).cursor - 1 as libc::c_int) as isize)
                        as *mut libc::c_char as *mut libc::c_void,
                    &mut *((*f).buffer).as_mut_ptr().offset((*f).cursor as isize)
                        as *mut libc::c_char as *const libc::c_void,
                    (strlen(
                        &mut *((*f).buffer).as_mut_ptr().offset((*f).cursor as isize),
                    ))
                        .wrapping_add(1 as libc::c_int as libc::c_ulong),
                );
                let ref mut fresh0 = (*f).cursor;
                *fresh0 -= 1;
                if (*f).visible_offset != 0 {
                    let ref mut fresh1 = (*f).visible_offset;
                    *fresh1 -= 1;
                }
            }
        }
        171 | 148 => {
            memmove(
                &mut *((*f).buffer).as_mut_ptr().offset((*f).cursor as isize)
                    as *mut libc::c_char as *mut libc::c_void,
                &mut *((*f).buffer)
                    .as_mut_ptr()
                    .offset(((*f).cursor + 1 as libc::c_int) as isize)
                    as *mut libc::c_char as *const libc::c_void,
                (strlen(
                    &mut *((*f).buffer)
                        .as_mut_ptr()
                        .offset(((*f).cursor + 1 as libc::c_int) as isize),
                ))
                    .wrapping_add(1 as libc::c_int as libc::c_ulong),
            );
        }
        169 | 13 | 27 | 9 => return false_0,
        32 | _ => {
            if *(*__ctype_b_loc()).offset(key as isize) as libc::c_int
                & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int == 0
                && (*f).generic.flags & 0x4 as libc::c_int as libc::c_uint != 0
            {
                return false_0;
            }
            if (*f).cursor < (*f).length {
                let ref mut fresh2 = (*f).cursor;
                let fresh3 = *fresh2;
                *fresh2 = *fresh2 + 1;
                (*f).buffer[fresh3 as usize] = key as libc::c_char;
                (*f).buffer[(*f).cursor as usize] = 0 as libc::c_int as libc::c_char;
                if (*f).cursor > (*f).visible_length {
                    let ref mut fresh4 = (*f).visible_offset;
                    *fresh4 += 1;
                }
            }
        }
    }
    return true_0;
}

#[no_mangle]
pub unsafe extern "C" fn Menu_AddItem(
    mut menu: *mut menuframework_s,
    mut item: *mut libc::c_void,
) {
    if (*menu).nitems == 0 as libc::c_int {
        (*menu).nslots = 0 as libc::c_int;
    }
    if (*menu).nitems < 64 as libc::c_int {
        let ref mut fresh5 = (*menu).items[(*menu).nitems as usize];
        *fresh5 = item;
        let ref mut fresh6 = (*((*menu).items[(*menu).nitems as usize]
            as *mut menucommon_s))
            .parent;
        *fresh6 = menu;
        let ref mut fresh7 = (*menu).nitems;
        *fresh7 += 1;
    }
    (*menu).nslots = Menu_TallySlots(menu);
}

#[no_mangle]
pub unsafe extern "C" fn Menu_AdjustCursor(
    mut m: *mut menuframework_s,
    mut dir: libc::c_int,
) {
    let mut citem: *mut menucommon_s = 0 as *mut menucommon_s;
    if (*m).cursor >= 0 as libc::c_int && (*m).cursor < (*m).nitems {
        citem = Menu_ItemAtCursor(m) as *mut menucommon_s;
        if !citem.is_null() {
            if (*citem).type_0 != 4 as libc::c_int {
                return;
            }
        }
    }
    if dir == 1 as libc::c_int {
        loop {
            citem = Menu_ItemAtCursor(m) as *mut menucommon_s;
            if !citem.is_null() {
                if (*citem).type_0 != 4 as libc::c_int {
                    break;
                }
            }
            (*m).cursor += dir;
            if (*m).cursor >= (*m).nitems {
                (*m).cursor = 0 as libc::c_int;
            }
        }
    } else {
        loop {
            citem = Menu_ItemAtCursor(m) as *mut menucommon_s;
            if !citem.is_null() {
                if (*citem).type_0 != 4 as libc::c_int {
                    break;
                }
            }
            (*m).cursor += dir;
            if (*m).cursor < 0 as libc::c_int {
                (*m).cursor = (*m).nitems - 1 as libc::c_int;
            }
        }
    };
}

#[no_mangle]
pub unsafe extern "C" fn Menu_Center(mut menu: *mut menuframework_s) {
    let mut height: libc::c_int = 0;
    height = (*((*menu).items[((*menu).nitems - 1 as libc::c_int) as usize]
        as *mut menucommon_s))
        .y;
    height += 10 as libc::c_int;
    (*menu).y = (viddef.height - height) / 2 as libc::c_int;
}

#[no_mangle]
pub unsafe extern "C" fn Menu_Draw(mut menu: *mut menuframework_s) {
    let mut i: libc::c_int = 0;
    let mut item: *mut menucommon_s = 0 as *mut menucommon_s;
    i = 0 as libc::c_int;
    while i < (*menu).nitems {
        match (*((*menu).items[i as usize] as *mut menucommon_s)).type_0 {
            5 => {
                Field_Draw((*menu).items[i as usize] as *mut menufield_s);
            }
            0 => {
                Slider_Draw((*menu).items[i as usize] as *mut menuslider_s);
            }
            1 => {
                MenuList_Draw((*menu).items[i as usize] as *mut menulist_s);
            }
            3 => {
                SpinControl_Draw((*menu).items[i as usize] as *mut menulist_s);
            }
            2 => {
                Action_Draw((*menu).items[i as usize] as *mut menuaction_s);
            }
            4 => {
                Separator_Draw((*menu).items[i as usize] as *mut menuseparator_s);
            }
            _ => {}
        }
        i += 1;
    }
    item = Menu_ItemAtCursor(menu) as *mut menucommon_s;
    if !item.is_null() && ((*item).cursordraw).is_some() {
        ((*item).cursordraw)
            .expect("non-null function pointer")(item as *mut libc::c_void);
    } else if ((*menu).cursordraw).is_some() {
        ((*menu).cursordraw).expect("non-null function pointer")(menu);
    } else if !item.is_null() && (*item).type_0 != 5 as libc::c_int {
        if (*item).flags & 0x1 as libc::c_int as libc::c_uint != 0 {
            (re.DrawChar)
                .expect(
                    "non-null function pointer",
                )(
                (*menu).x + (*item).x - 24 as libc::c_int + (*item).cursor_offset,
                (*menu).y + (*item).y,
                12 as libc::c_int
                    + (Sys_Milliseconds() / 250 as libc::c_int & 1 as libc::c_int),
            );
        } else {
            (re.DrawChar)
                .expect(
                    "non-null function pointer",
                )(
                (*menu).x + (*item).cursor_offset,
                (*menu).y + (*item).y,
                12 as libc::c_int
                    + (Sys_Milliseconds() / 250 as libc::c_int & 1 as libc::c_int),
            );
        }
    }
    if !item.is_null() {
        if ((*item).statusbarfunc).is_some() {
            ((*item).statusbarfunc)
                .expect("non-null function pointer")(item as *mut libc::c_void);
        } else if !((*item).statusbar).is_null() {
            Menu_DrawStatusBar((*item).statusbar);
        } else {
            Menu_DrawStatusBar((*menu).statusbar);
        }
    } else {
        Menu_DrawStatusBar((*menu).statusbar);
    };
}

unsafe extern "C" fn Menu_DrawStatusBar(mut string: *const libc::c_char) {
    if !string.is_null() {
        let mut l: libc::c_int = strlen(string) as libc::c_int;
        let mut maxrow: libc::c_int = viddef.height / 8 as libc::c_int;
        let mut maxcol: libc::c_int = viddef.width / 8 as libc::c_int;
        let mut col: libc::c_int = maxcol / 2 as libc::c_int - l / 2 as libc::c_int;
        (re.DrawFill)
            .expect(
                "non-null function pointer",
            )(
            0 as libc::c_int,
            viddef.height - 8 as libc::c_int,
            viddef.width,
            8 as libc::c_int,
            4 as libc::c_int,
        );
        Menu_DrawString(
            col * 8 as libc::c_int,
            viddef.height - 8 as libc::c_int,
            string,
        );
    } else {
        (re.DrawFill)
            .expect(
                "non-null function pointer",
            )(
            0 as libc::c_int,
            viddef.height - 8 as libc::c_int,
            viddef.width,
            8 as libc::c_int,
            0 as libc::c_int,
        );
    };
}

#[no_mangle]
pub unsafe extern "C" fn Menu_DrawString(
    mut x: libc::c_int,
    mut y: libc::c_int,
    mut string: *const libc::c_char,
) {
    let mut i: libc::c_uint = 0;
    i = 0 as libc::c_int as libc::c_uint;
    while (i as libc::c_ulong) < strlen(string) {
        (re.DrawChar)
            .expect(
                "non-null function pointer",
            )(
            (x as libc::c_uint)
                .wrapping_add(i.wrapping_mul(8 as libc::c_int as libc::c_uint))
                as libc::c_int,
            y,
            *string.offset(i as isize) as libc::c_int,
        );
        i = i.wrapping_add(1);
    }
}

#[no_mangle]
pub unsafe extern "C" fn Menu_DrawStringDark(
    mut x: libc::c_int,
    mut y: libc::c_int,
    mut string: *const libc::c_char,
) {
    let mut i: libc::c_uint = 0;
    i = 0 as libc::c_int as libc::c_uint;
    while (i as libc::c_ulong) < strlen(string) {
        (re.DrawChar)
            .expect(
                "non-null function pointer",
            )(
            (x as libc::c_uint)
                .wrapping_add(i.wrapping_mul(8 as libc::c_int as libc::c_uint))
                as libc::c_int,
            y,
            *string.offset(i as isize) as libc::c_int + 128 as libc::c_int,
        );
        i = i.wrapping_add(1);
    }
}

#[no_mangle]
pub unsafe extern "C" fn Menu_DrawStringR2L(
    mut x: libc::c_int,
    mut y: libc::c_int,
    mut string: *const libc::c_char,
) {
    let mut i: libc::c_uint = 0;
    i = 0 as libc::c_int as libc::c_uint;
    while (i as libc::c_ulong) < strlen(string) {
        (re.DrawChar)
            .expect(
                "non-null function pointer",
            )(
            (x as libc::c_uint)
                .wrapping_sub(i.wrapping_mul(8 as libc::c_int as libc::c_uint))
                as libc::c_int,
            y,
            *string
                .offset(
                    (strlen(string))
                        .wrapping_sub(i as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                ) as libc::c_int,
        );
        i = i.wrapping_add(1);
    }
}

#[no_mangle]
pub unsafe extern "C" fn Menu_DrawStringR2LDark(
    mut x: libc::c_int,
    mut y: libc::c_int,
    mut string: *const libc::c_char,
) {
    let mut i: libc::c_uint = 0;
    i = 0 as libc::c_int as libc::c_uint;
    while (i as libc::c_ulong) < strlen(string) {
        (re.DrawChar)
            .expect(
                "non-null function pointer",
            )(
            (x as libc::c_uint)
                .wrapping_sub(i.wrapping_mul(8 as libc::c_int as libc::c_uint))
                as libc::c_int,
            y,
            *string
                .offset(
                    (strlen(string))
                        .wrapping_sub(i as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                ) as libc::c_int + 128 as libc::c_int,
        );
        i = i.wrapping_add(1);
    }
}

#[no_mangle]
pub unsafe extern "C" fn Menu_ItemAtCursor(
    mut m: *mut menuframework_s,
) -> *mut libc::c_void {
    if (*m).cursor < 0 as libc::c_int || (*m).cursor >= (*m).nitems {
        return 0 as *mut libc::c_void;
    }
    return (*m).items[(*m).cursor as usize];
}

#[no_mangle]
pub unsafe extern "C" fn Menu_SelectItem(mut s: *mut menuframework_s) -> qboolean {
    let mut item: *mut menucommon_s = Menu_ItemAtCursor(s) as *mut menucommon_s;
    if !item.is_null() {
        match (*item).type_0 {
            5 => return Field_DoEnter(item as *mut menufield_s),
            2 => {
                Action_DoEnter(item as *mut menuaction_s);
                return true_0;
            }
            1 => return false_0,
            3 => return false_0,
            _ => {}
        }
    }
    return false_0;
}

#[no_mangle]
pub unsafe extern "C" fn Menu_SetStatusBar(
    mut m: *mut menuframework_s,
    mut string: *const libc::c_char,
) {
    let ref mut fresh8 = (*m).statusbar;
    *fresh8 = string;
}

#[no_mangle]
pub unsafe extern "C" fn Menu_SlideItem(
    mut s: *mut menuframework_s,
    mut dir: libc::c_int,
) {
    let mut item: *mut menucommon_s = Menu_ItemAtCursor(s) as *mut menucommon_s;
    if !item.is_null() {
        match (*item).type_0 {
            0 => {
                Slider_DoSlide(item as *mut menuslider_s, dir);
            }
            3 => {
                SpinControl_DoSlide(item as *mut menulist_s, dir);
            }
            _ => {}
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn Menu_TallySlots(mut menu: *mut menuframework_s) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut total: libc::c_int = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < (*menu).nitems {
        if (*((*menu).items[i as usize] as *mut menucommon_s)).type_0 == 1 as libc::c_int
        {
            let mut nitems: libc::c_int = 0 as libc::c_int;
            let mut n: *mut *const libc::c_char = (*((*menu).items[i as usize]
                as *mut menulist_s))
                .itemnames;
            while !(*n).is_null() {
                nitems += 1;
                n = n.offset(1);
            }
            total += nitems;
        } else {
            total += 1;
        }
        i += 1;
    }
    return total;
}

unsafe extern "C" fn MenuList_Draw(mut l: *mut menulist_s) {
    let mut n: *mut *const libc::c_char = 0 as *mut *const libc::c_char;
    let mut y: libc::c_int = 0 as libc::c_int;
    Menu_DrawStringR2LDark(
        (*l).generic.x + (*(*l).generic.parent).x + -(16 as libc::c_int),
        (*l).generic.y + (*(*l).generic.parent).y,
        (*l).generic.name,
    );
    n = (*l).itemnames;
    (re.DrawFill)
        .expect(
            "non-null function pointer",
        )(
        (*l).generic.x - 112 as libc::c_int + (*(*l).generic.parent).x,
        (*(*l).generic.parent).y + (*l).generic.y + (*l).curvalue * 10 as libc::c_int
            + 10 as libc::c_int,
        128 as libc::c_int,
        10 as libc::c_int,
        16 as libc::c_int,
    );
    while !(*n).is_null() {
        Menu_DrawStringR2LDark(
            (*l).generic.x + (*(*l).generic.parent).x + -(16 as libc::c_int),
            (*l).generic.y + (*(*l).generic.parent).y + y + 10 as libc::c_int,
            *n,
        );
        n = n.offset(1);
        y += 10 as libc::c_int;
    }
}

unsafe extern "C" fn Separator_Draw(mut s: *mut menuseparator_s) {
    if !((*s).generic.name).is_null() {
        Menu_DrawStringR2LDark(
            (*s).generic.x + (*(*s).generic.parent).x,
            (*s).generic.y + (*(*s).generic.parent).y,
            (*s).generic.name,
        );
    }
}

unsafe extern "C" fn Slider_DoSlide(mut s: *mut menuslider_s, mut dir: libc::c_int) {
    (*s).curvalue += dir as libc::c_float;
    if (*s).curvalue > (*s).maxvalue {
        (*s).curvalue = (*s).maxvalue;
    } else if (*s).curvalue < (*s).minvalue {
        (*s).curvalue = (*s).minvalue;
    }
    if ((*s).generic.callback).is_some() {
        ((*s).generic.callback)
            .expect("non-null function pointer")(s as *mut libc::c_void);
    }
}

unsafe extern "C" fn Slider_Draw(mut s: *mut menuslider_s) {
    let mut i: libc::c_int = 0;
    Menu_DrawStringR2LDark(
        (*s).generic.x + (*(*s).generic.parent).x + -(16 as libc::c_int),
        (*s).generic.y + (*(*s).generic.parent).y,
        (*s).generic.name,
    );
    (*s).range = ((*s).curvalue - (*s).minvalue) / ((*s).maxvalue - (*s).minvalue);
    if (*s).range < 0 as libc::c_int as libc::c_float {
        (*s).range = 0 as libc::c_int as libc::c_float;
    }
    if (*s).range > 1 as libc::c_int as libc::c_float {
        (*s).range = 1 as libc::c_int as libc::c_float;
    }
    (re.DrawChar)
        .expect(
            "non-null function pointer",
        )(
        (*s).generic.x + (*(*s).generic.parent).x + 16 as libc::c_int,
        (*s).generic.y + (*(*s).generic.parent).y,
        128 as libc::c_int,
    );
    i = 0 as libc::c_int;
    while i < 10 as libc::c_int {
        (re.DrawChar)
            .expect(
                "non-null function pointer",
            )(
            16 as libc::c_int + (*s).generic.x + i * 8 as libc::c_int
                + (*(*s).generic.parent).x + 8 as libc::c_int,
            (*s).generic.y + (*(*s).generic.parent).y,
            129 as libc::c_int,
        );
        i += 1;
    }
    (re.DrawChar)
        .expect(
            "non-null function pointer",
        )(
        16 as libc::c_int + (*s).generic.x + i * 8 as libc::c_int
            + (*(*s).generic.parent).x + 8 as libc::c_int,
        (*s).generic.y + (*(*s).generic.parent).y,
        130 as libc::c_int,
    );
    (re.DrawChar)
        .expect(
            "non-null function pointer",
        )(
        ((8 as libc::c_int + 16 as libc::c_int + (*(*s).generic.parent).x
            + (*s).generic.x) as libc::c_float
            + ((10 as libc::c_int - 1 as libc::c_int) * 8 as libc::c_int)
            as libc::c_float * (*s).range) as libc::c_int,
        (*s).generic.y + (*(*s).generic.parent).y,
        131 as libc::c_int,
    );
}

unsafe extern "C" fn SpinControl_DoSlide(mut s: *mut menulist_s, mut dir: libc::c_int) {
    (*s).curvalue += dir;
    if (*s).curvalue < 0 as libc::c_int {
        (*s).curvalue = 0 as libc::c_int;
    } else if (*((*s).itemnames).offset((*s).curvalue as isize)).is_null() {
        let ref mut fresh9 = (*s).curvalue;
        *fresh9 -= 1;
    }
    if ((*s).generic.callback).is_some() {
        ((*s).generic.callback)
            .expect("non-null function pointer")(s as *mut libc::c_void);
    }
}

unsafe extern "C" fn SpinControl_Draw(mut s: *mut menulist_s) {
    let mut buffer: [libc::c_char; 100] = [0; 100];
    if !((*s).generic.name).is_null() {
        Menu_DrawStringR2LDark(
            (*s).generic.x + (*(*s).generic.parent).x + -(16 as libc::c_int),
            (*s).generic.y + (*(*s).generic.parent).y,
            (*s).generic.name,
        );
    }
    if (strchr(*((*s).itemnames).offset((*s).curvalue as isize), '\n' as i32)).is_null()
    {
        Menu_DrawString(
            16 as libc::c_int + (*s).generic.x + (*(*s).generic.parent).x,
            (*s).generic.y + (*(*s).generic.parent).y,
            *((*s).itemnames).offset((*s).curvalue as isize),
        );
    } else {
        strcpy(buffer.as_mut_ptr(), *((*s).itemnames).offset((*s).curvalue as isize));
        *strchr(buffer.as_mut_ptr(), '\n' as i32) = 0 as libc::c_int as libc::c_char;
        Menu_DrawString(
            16 as libc::c_int + (*s).generic.x + (*(*s).generic.parent).x,
            (*s).generic.y + (*(*s).generic.parent).y,
            buffer.as_mut_ptr(),
        );
        strcpy(
            buffer.as_mut_ptr(),
            (strchr(*((*s).itemnames).offset((*s).curvalue as isize), '\n' as i32))
                .offset(1 as libc::c_int as isize),
        );
        Menu_DrawString(
            16 as libc::c_int + (*s).generic.x + (*(*s).generic.parent).x,
            (*s).generic.y + (*(*s).generic.parent).y + 10 as libc::c_int,
            buffer.as_mut_ptr(),
        );
    };
}
