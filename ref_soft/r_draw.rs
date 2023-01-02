#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn Com_sprintf(
        dest: *mut libc::c_char,
        size: libc::c_int,
        fmt: *mut libc::c_char,
        _: ...
    );
    static mut ri: refimport_t;
    fn R_FindImage(name: *mut libc::c_char, type_0: imagetype_t) -> *mut image_t;
    static mut vid: viddef_t;
}
pub type byte = libc::c_uchar;
pub type qboolean = libc::c_uint;
pub const true_0: qboolean = 1;
pub const false_0: qboolean = 0;
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
pub struct image_s {
    pub name: [libc::c_char; 64],
    pub type_0: imagetype_t,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub transparent: qboolean,
    pub registration_sequence: libc::c_int,
    pub pixels: [*mut byte; 4],
}
pub type imagetype_t = libc::c_uint;
pub const it_sky: imagetype_t = 4;
pub const it_pic: imagetype_t = 3;
pub const it_wall: imagetype_t = 2;
pub const it_sprite: imagetype_t = 1;
pub const it_skin: imagetype_t = 0;
pub type image_t = image_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct refimport_t {
    pub Sys_Error: Option::<
        unsafe extern "C" fn(libc::c_int, *mut libc::c_char, ...) -> (),
    >,
    pub Cmd_AddCommand: Option::<
        unsafe extern "C" fn(
            *mut libc::c_char,
            Option::<unsafe extern "C" fn() -> ()>,
        ) -> (),
    >,
    pub Cmd_RemoveCommand: Option::<unsafe extern "C" fn(*mut libc::c_char) -> ()>,
    pub Cmd_Argc: Option::<unsafe extern "C" fn() -> libc::c_int>,
    pub Cmd_Argv: Option::<unsafe extern "C" fn(libc::c_int) -> *mut libc::c_char>,
    pub Cmd_ExecuteText: Option::<
        unsafe extern "C" fn(libc::c_int, *mut libc::c_char) -> (),
    >,
    pub Con_Printf: Option::<
        unsafe extern "C" fn(libc::c_int, *mut libc::c_char, ...) -> (),
    >,
    pub FS_LoadFile: Option::<
        unsafe extern "C" fn(*mut libc::c_char, *mut *mut libc::c_void) -> libc::c_int,
    >,
    pub FS_FreeFile: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub FS_Gamedir: Option::<unsafe extern "C" fn() -> *mut libc::c_char>,
    pub Cvar_Get: Option::<
        unsafe extern "C" fn(
            *mut libc::c_char,
            *mut libc::c_char,
            libc::c_int,
        ) -> *mut cvar_t,
    >,
    pub Cvar_Set: Option::<
        unsafe extern "C" fn(*mut libc::c_char, *mut libc::c_char) -> *mut cvar_t,
    >,
    pub Cvar_SetValue: Option::<
        unsafe extern "C" fn(*mut libc::c_char, libc::c_float) -> (),
    >,
    pub Vid_GetModeInfo: Option::<
        unsafe extern "C" fn(*mut libc::c_int, *mut libc::c_int, libc::c_int) -> qboolean,
    >,
    pub Vid_MenuInit: Option::<unsafe extern "C" fn() -> ()>,
    pub Vid_NewWindow: Option::<unsafe extern "C" fn(libc::c_int, libc::c_int) -> ()>,
}
pub type pixel_t = libc::c_uchar;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct viddef_t {
    pub buffer: *mut pixel_t,
    pub colormap: *mut pixel_t,
    pub alphamap: *mut pixel_t,
    pub rowbytes: libc::c_int,
    pub width: libc::c_int,
    pub height: libc::c_int,
}
#[no_mangle]
pub static mut draw_chars: *mut image_t = 0 as *const image_t as *mut image_t;
#[no_mangle]
pub unsafe extern "C" fn Draw_FindPic(mut name: *mut libc::c_char) -> *mut image_s {
    let mut image: *mut image_t = 0 as *mut image_t;
    let mut fullname: [libc::c_char; 64] = [0; 64];
    if *name.offset(0 as libc::c_int as isize) as libc::c_int != '/' as i32
        && *name.offset(0 as libc::c_int as isize) as libc::c_int != '\\' as i32
    {
        Com_sprintf(
            fullname.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
            b"pics/%s.pcx\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            name,
        );
        image = R_FindImage(fullname.as_mut_ptr(), it_pic);
    } else {
        image = R_FindImage(name.offset(1 as libc::c_int as isize), it_pic);
    }
    return image;
}
#[no_mangle]
pub unsafe extern "C" fn Draw_InitLocal() {
    draw_chars = Draw_FindPic(
        b"conchars\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
}
#[no_mangle]
pub unsafe extern "C" fn Draw_Char(
    mut x: libc::c_int,
    mut y: libc::c_int,
    mut num: libc::c_int,
) {
    let mut dest: *mut byte = 0 as *mut byte;
    let mut source: *mut byte = 0 as *mut byte;
    let mut drawline: libc::c_int = 0;
    let mut row: libc::c_int = 0;
    let mut col: libc::c_int = 0;
    num &= 255 as libc::c_int;
    if num == 32 as libc::c_int || num == 32 as libc::c_int + 128 as libc::c_int {
        return;
    }
    if y <= -(8 as libc::c_int) {
        return;
    }
    if y + 8 as libc::c_int > vid.height {
        return;
    }
    row = num >> 4 as libc::c_int;
    col = num & 15 as libc::c_int;
    source = ((*draw_chars).pixels[0 as libc::c_int as usize])
        .offset((row << 10 as libc::c_int) as isize)
        .offset((col << 3 as libc::c_int) as isize);
    if y < 0 as libc::c_int {
        drawline = 8 as libc::c_int + y;
        source = source.offset(-((128 as libc::c_int * y) as isize));
        y = 0 as libc::c_int;
    } else {
        drawline = 8 as libc::c_int;
    }
    dest = (vid.buffer).offset((y * vid.rowbytes) as isize).offset(x as isize);
    loop {
        let fresh0 = drawline;
        drawline = drawline - 1;
        if !(fresh0 != 0) {
            break;
        }
        if *source.offset(0 as libc::c_int as isize) as libc::c_int
            != 0xff as libc::c_int
        {
            *dest
                .offset(
                    0 as libc::c_int as isize,
                ) = *source.offset(0 as libc::c_int as isize);
        }
        if *source.offset(1 as libc::c_int as isize) as libc::c_int
            != 0xff as libc::c_int
        {
            *dest
                .offset(
                    1 as libc::c_int as isize,
                ) = *source.offset(1 as libc::c_int as isize);
        }
        if *source.offset(2 as libc::c_int as isize) as libc::c_int
            != 0xff as libc::c_int
        {
            *dest
                .offset(
                    2 as libc::c_int as isize,
                ) = *source.offset(2 as libc::c_int as isize);
        }
        if *source.offset(3 as libc::c_int as isize) as libc::c_int
            != 0xff as libc::c_int
        {
            *dest
                .offset(
                    3 as libc::c_int as isize,
                ) = *source.offset(3 as libc::c_int as isize);
        }
        if *source.offset(4 as libc::c_int as isize) as libc::c_int
            != 0xff as libc::c_int
        {
            *dest
                .offset(
                    4 as libc::c_int as isize,
                ) = *source.offset(4 as libc::c_int as isize);
        }
        if *source.offset(5 as libc::c_int as isize) as libc::c_int
            != 0xff as libc::c_int
        {
            *dest
                .offset(
                    5 as libc::c_int as isize,
                ) = *source.offset(5 as libc::c_int as isize);
        }
        if *source.offset(6 as libc::c_int as isize) as libc::c_int
            != 0xff as libc::c_int
        {
            *dest
                .offset(
                    6 as libc::c_int as isize,
                ) = *source.offset(6 as libc::c_int as isize);
        }
        if *source.offset(7 as libc::c_int as isize) as libc::c_int
            != 0xff as libc::c_int
        {
            *dest
                .offset(
                    7 as libc::c_int as isize,
                ) = *source.offset(7 as libc::c_int as isize);
        }
        source = source.offset(128 as libc::c_int as isize);
        dest = dest.offset(vid.rowbytes as isize);
    };
}
#[no_mangle]
pub unsafe extern "C" fn Draw_GetPicSize(
    mut w: *mut libc::c_int,
    mut h: *mut libc::c_int,
    mut pic: *mut libc::c_char,
) {
    let mut gl: *mut image_t = 0 as *mut image_t;
    gl = Draw_FindPic(pic);
    if gl.is_null() {
        *h = -(1 as libc::c_int);
        *w = *h;
        return;
    }
    *w = (*gl).width;
    *h = (*gl).height;
}
#[no_mangle]
pub unsafe extern "C" fn Draw_StretchPicImplementation(
    mut x: libc::c_int,
    mut y: libc::c_int,
    mut w: libc::c_int,
    mut h: libc::c_int,
    mut pic: *mut image_t,
) {
    let mut dest: *mut byte = 0 as *mut byte;
    let mut source: *mut byte = 0 as *mut byte;
    let mut v: libc::c_int = 0;
    let mut u: libc::c_int = 0;
    let mut sv: libc::c_int = 0;
    let mut height: libc::c_int = 0;
    let mut f: libc::c_int = 0;
    let mut fstep: libc::c_int = 0;
    let mut skip: libc::c_int = 0;
    if x < 0 as libc::c_int || x + w > vid.width || y + h > vid.height {
        (ri.Sys_Error)
            .expect(
                "non-null function pointer",
            )(
            0 as libc::c_int,
            b"Draw_Pic: bad coordinates\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    height = h;
    if y < 0 as libc::c_int {
        skip = -y;
        height += y;
        y = 0 as libc::c_int;
    } else {
        skip = 0 as libc::c_int;
    }
    dest = (vid.buffer).offset((y * vid.rowbytes) as isize).offset(x as isize);
    v = 0 as libc::c_int;
    while v < height {
        sv = (skip + v) * (*pic).height / h;
        source = ((*pic).pixels[0 as libc::c_int as usize])
            .offset((sv * (*pic).width) as isize);
        if w == (*pic).width {
            memcpy(
                dest as *mut libc::c_void,
                source as *const libc::c_void,
                w as libc::c_ulong,
            );
        } else {
            f = 0 as libc::c_int;
            fstep = (*pic).width * 0x10000 as libc::c_int / w;
            u = 0 as libc::c_int;
            while u < w {
                *dest
                    .offset(
                        u as isize,
                    ) = *source.offset((f >> 16 as libc::c_int) as isize);
                f += fstep;
                *dest
                    .offset(
                        (u + 1 as libc::c_int) as isize,
                    ) = *source.offset((f >> 16 as libc::c_int) as isize);
                f += fstep;
                *dest
                    .offset(
                        (u + 2 as libc::c_int) as isize,
                    ) = *source.offset((f >> 16 as libc::c_int) as isize);
                f += fstep;
                *dest
                    .offset(
                        (u + 3 as libc::c_int) as isize,
                    ) = *source.offset((f >> 16 as libc::c_int) as isize);
                f += fstep;
                u += 4 as libc::c_int;
            }
        }
        v += 1;
        dest = dest.offset(vid.rowbytes as isize);
    }
}
#[no_mangle]
pub unsafe extern "C" fn Draw_StretchPic(
    mut x: libc::c_int,
    mut y: libc::c_int,
    mut w: libc::c_int,
    mut h: libc::c_int,
    mut name: *mut libc::c_char,
) {
    let mut pic: *mut image_t = 0 as *mut image_t;
    pic = Draw_FindPic(name);
    if pic.is_null() {
        (ri.Con_Printf)
            .expect(
                "non-null function pointer",
            )(
            0 as libc::c_int,
            b"Can't find pic: %s\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            name,
        );
        return;
    }
    Draw_StretchPicImplementation(x, y, w, h, pic);
}
#[no_mangle]
pub unsafe extern "C" fn Draw_StretchRaw(
    mut x: libc::c_int,
    mut y: libc::c_int,
    mut w: libc::c_int,
    mut h: libc::c_int,
    mut cols: libc::c_int,
    mut rows: libc::c_int,
    mut data: *mut byte,
) {
    let mut pic: image_t = image_t {
        name: [0; 64],
        type_0: it_skin,
        width: 0,
        height: 0,
        transparent: false_0,
        registration_sequence: 0,
        pixels: [0 as *mut byte; 4],
    };
    pic.pixels[0 as libc::c_int as usize] = data;
    pic.width = cols;
    pic.height = rows;
    Draw_StretchPicImplementation(x, y, w, h, &mut pic);
}
#[no_mangle]
pub unsafe extern "C" fn Draw_Pic(
    mut x: libc::c_int,
    mut y: libc::c_int,
    mut name: *mut libc::c_char,
) {
    let mut pic: *mut image_t = 0 as *mut image_t;
    let mut dest: *mut byte = 0 as *mut byte;
    let mut source: *mut byte = 0 as *mut byte;
    let mut v: libc::c_int = 0;
    let mut u: libc::c_int = 0;
    let mut tbyte: libc::c_int = 0;
    let mut height: libc::c_int = 0;
    pic = Draw_FindPic(name);
    if pic.is_null() {
        (ri.Con_Printf)
            .expect(
                "non-null function pointer",
            )(
            0 as libc::c_int,
            b"Can't find pic: %s\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            name,
        );
        return;
    }
    if x < 0 as libc::c_int || x + (*pic).width > vid.width
        || y + (*pic).height > vid.height
    {
        return;
    }
    height = (*pic).height;
    source = (*pic).pixels[0 as libc::c_int as usize];
    if y < 0 as libc::c_int {
        height += y;
        source = source.offset(((*pic).width * -y) as isize);
        y = 0 as libc::c_int;
    }
    dest = (vid.buffer).offset((y * vid.rowbytes) as isize).offset(x as isize);
    if (*pic).transparent as u64 == 0 {
        v = 0 as libc::c_int;
        while v < height {
            memcpy(
                dest as *mut libc::c_void,
                source as *const libc::c_void,
                (*pic).width as libc::c_ulong,
            );
            dest = dest.offset(vid.rowbytes as isize);
            source = source.offset((*pic).width as isize);
            v += 1;
        }
    } else if (*pic).width & 7 as libc::c_int != 0 {
        v = 0 as libc::c_int;
        while v < height {
            u = 0 as libc::c_int;
            while u < (*pic).width {
                tbyte = *source.offset(u as isize) as libc::c_int;
                if tbyte != 0xff as libc::c_int {
                    *dest.offset(u as isize) = tbyte as byte;
                }
                u += 1;
            }
            dest = dest.offset(vid.rowbytes as isize);
            source = source.offset((*pic).width as isize);
            v += 1;
        }
    } else {
        v = 0 as libc::c_int;
        while v < height {
            u = 0 as libc::c_int;
            while u < (*pic).width {
                tbyte = *source.offset(u as isize) as libc::c_int;
                if tbyte != 0xff as libc::c_int {
                    *dest.offset(u as isize) = tbyte as byte;
                }
                tbyte = *source.offset((u + 1 as libc::c_int) as isize) as libc::c_int;
                if tbyte != 0xff as libc::c_int {
                    *dest.offset((u + 1 as libc::c_int) as isize) = tbyte as byte;
                }
                tbyte = *source.offset((u + 2 as libc::c_int) as isize) as libc::c_int;
                if tbyte != 0xff as libc::c_int {
                    *dest.offset((u + 2 as libc::c_int) as isize) = tbyte as byte;
                }
                tbyte = *source.offset((u + 3 as libc::c_int) as isize) as libc::c_int;
                if tbyte != 0xff as libc::c_int {
                    *dest.offset((u + 3 as libc::c_int) as isize) = tbyte as byte;
                }
                tbyte = *source.offset((u + 4 as libc::c_int) as isize) as libc::c_int;
                if tbyte != 0xff as libc::c_int {
                    *dest.offset((u + 4 as libc::c_int) as isize) = tbyte as byte;
                }
                tbyte = *source.offset((u + 5 as libc::c_int) as isize) as libc::c_int;
                if tbyte != 0xff as libc::c_int {
                    *dest.offset((u + 5 as libc::c_int) as isize) = tbyte as byte;
                }
                tbyte = *source.offset((u + 6 as libc::c_int) as isize) as libc::c_int;
                if tbyte != 0xff as libc::c_int {
                    *dest.offset((u + 6 as libc::c_int) as isize) = tbyte as byte;
                }
                tbyte = *source.offset((u + 7 as libc::c_int) as isize) as libc::c_int;
                if tbyte != 0xff as libc::c_int {
                    *dest.offset((u + 7 as libc::c_int) as isize) = tbyte as byte;
                }
                u += 8 as libc::c_int;
            }
            dest = dest.offset(vid.rowbytes as isize);
            source = source.offset((*pic).width as isize);
            v += 1;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn Draw_TileClear(
    mut x: libc::c_int,
    mut y: libc::c_int,
    mut w: libc::c_int,
    mut h: libc::c_int,
    mut name: *mut libc::c_char,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut psrc: *mut byte = 0 as *mut byte;
    let mut pdest: *mut byte = 0 as *mut byte;
    let mut pic: *mut image_t = 0 as *mut image_t;
    let mut x2: libc::c_int = 0;
    if x < 0 as libc::c_int {
        w += x;
        x = 0 as libc::c_int;
    }
    if y < 0 as libc::c_int {
        h += y;
        y = 0 as libc::c_int;
    }
    if x + w > vid.width {
        w = vid.width - x;
    }
    if y + h > vid.height {
        h = vid.height - y;
    }
    if w <= 0 as libc::c_int || h <= 0 as libc::c_int {
        return;
    }
    pic = Draw_FindPic(name);
    if pic.is_null() {
        (ri.Con_Printf)
            .expect(
                "non-null function pointer",
            )(
            0 as libc::c_int,
            b"Can't find pic: %s\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            name,
        );
        return;
    }
    x2 = x + w;
    pdest = (vid.buffer).offset((y * vid.rowbytes) as isize);
    i = 0 as libc::c_int;
    while i < h {
        psrc = ((*pic).pixels[0 as libc::c_int as usize])
            .offset(((*pic).width * (i + y & 63 as libc::c_int)) as isize);
        j = x;
        while j < x2 {
            *pdest.offset(j as isize) = *psrc.offset((j & 63 as libc::c_int) as isize);
            j += 1;
        }
        i += 1;
        pdest = pdest.offset(vid.rowbytes as isize);
    }
}
#[no_mangle]
pub unsafe extern "C" fn Draw_Fill(
    mut x: libc::c_int,
    mut y: libc::c_int,
    mut w: libc::c_int,
    mut h: libc::c_int,
    mut c: libc::c_int,
) {
    let mut dest: *mut byte = 0 as *mut byte;
    let mut u: libc::c_int = 0;
    let mut v: libc::c_int = 0;
    if x + w > vid.width {
        w = vid.width - x;
    }
    if y + h > vid.height {
        h = vid.height - y;
    }
    if x < 0 as libc::c_int {
        w += x;
        x = 0 as libc::c_int;
    }
    if y < 0 as libc::c_int {
        h += y;
        y = 0 as libc::c_int;
    }
    if w < 0 as libc::c_int || h < 0 as libc::c_int {
        return;
    }
    dest = (vid.buffer).offset((y * vid.rowbytes) as isize).offset(x as isize);
    v = 0 as libc::c_int;
    while v < h {
        u = 0 as libc::c_int;
        while u < w {
            *dest.offset(u as isize) = c as byte;
            u += 1;
        }
        v += 1;
        dest = dest.offset(vid.rowbytes as isize);
    }
}
#[no_mangle]
pub unsafe extern "C" fn Draw_FadeScreen() {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut pbuf: *mut byte = 0 as *mut byte;
    let mut t: libc::c_int = 0;
    y = 0 as libc::c_int;
    while y < vid.height {
        pbuf = (vid.buffer).offset((vid.rowbytes * y) as isize) as *mut byte;
        t = (y & 1 as libc::c_int) << 1 as libc::c_int;
        x = 0 as libc::c_int;
        while x < vid.width {
            if x & 3 as libc::c_int != t {
                *pbuf.offset(x as isize) = 0 as libc::c_int as byte;
            }
            x += 1;
        }
        y += 1;
    }
}
