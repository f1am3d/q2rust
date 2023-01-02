#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    fn Com_sprintf(
        dest: *mut libc::c_char,
        size: libc::c_int,
        fmt: *mut libc::c_char,
        _: ...
    );
    static mut qglColor3f: Option::<
        unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int) -> (),
    >;
    static mut ri: refimport_t;
    fn GL_FindImage(name: *mut libc::c_char, type_0: imagetype_t) -> *mut image_t;
    static mut d_8to24table: [libc::c_uint; 256];
    fn GL_Bind(texnum: libc::c_int);
    static mut vid: viddef_t;
    static mut qglTexCoord2f: Option::<
        unsafe extern "C" fn(libc::c_int, libc::c_int) -> (),
    >;
    static mut qglColor4f: Option::<
        unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int, libc::c_int) -> (),
    >;
    static mut qglEnd: Option::<unsafe extern "C" fn() -> ()>;
    static mut qglVertex2f: Option::<
        unsafe extern "C" fn(libc::c_int, libc::c_int) -> (),
    >;
    static mut qglColorTableEXT: Option::<
        unsafe extern "C" fn(
            libc::c_int,
            libc::c_int,
            libc::c_int,
            libc::c_int,
            libc::c_int,
            *const libc::c_void,
        ) -> (),
    >;
    static mut scrap_dirty: qboolean;
    fn Scrap_Upload();
    static mut r_rawpalette: [libc::c_uint; 256];
}
pub type byte = libc::c_uchar;
pub type qboolean = libc::c_uint;
pub const true_0: qboolean = 1;
pub const false_0: qboolean = 0;
pub type vec_t = libc::c_float;
pub type vec3_t = [vec_t; 3];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cplane_s {
    pub normal: vec3_t,
    pub dist: libc::c_float,
    pub type_0: byte,
    pub signbits: byte,
    pub pad: [byte; 2],
}
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
pub type cplane_t = cplane_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct image_s {
    pub name: [libc::c_char; 64],
    pub type_0: imagetype_t,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub upload_width: libc::c_int,
    pub upload_height: libc::c_int,
    pub registration_sequence: libc::c_int,
    pub texturechain: *mut msurface_s,
    pub texnum: libc::c_int,
    pub sl: libc::c_float,
    pub tl: libc::c_float,
    pub sh: libc::c_float,
    pub th: libc::c_float,
    pub scrap: qboolean,
    pub has_alpha: qboolean,
    pub paletted: qboolean,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct msurface_s {
    pub visframe: libc::c_int,
    pub plane: *mut cplane_t,
    pub flags: libc::c_int,
    pub firstedge: libc::c_int,
    pub numedges: libc::c_int,
    pub texturemins: [libc::c_short; 2],
    pub extents: [libc::c_short; 2],
    pub light_s: libc::c_int,
    pub light_t: libc::c_int,
    pub dlight_s: libc::c_int,
    pub dlight_t: libc::c_int,
    pub polys: *mut glpoly_t,
    pub texturechain: *mut msurface_s,
    pub lightmapchain: *mut msurface_s,
    pub texinfo: *mut mtexinfo_t,
    pub dlightframe: libc::c_int,
    pub dlightbits: libc::c_int,
    pub lightmaptexturenum: libc::c_int,
    pub styles: [byte; 4],
    pub cached_light: [libc::c_float; 4],
    pub samples: *mut byte,
}
pub type mtexinfo_t = mtexinfo_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mtexinfo_s {
    pub vecs: [[libc::c_float; 4]; 2],
    pub flags: libc::c_int,
    pub numframes: libc::c_int,
    pub next: *mut mtexinfo_s,
    pub image: *mut image_t,
}
pub type image_t = image_s;
pub type glpoly_t = glpoly_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glpoly_s {
    pub next: *mut glpoly_s,
    pub chain: *mut glpoly_s,
    pub numverts: libc::c_int,
    pub flags: libc::c_int,
    pub verts: [[libc::c_float; 7]; 4],
}
pub type imagetype_t = libc::c_uint;
pub const it_sky: imagetype_t = 4;
pub const it_pic: imagetype_t = 3;
pub const it_wall: imagetype_t = 2;
pub const it_sprite: imagetype_t = 1;
pub const it_skin: imagetype_t = 0;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct viddef_t {
    pub width: libc::c_uint,
    pub height: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub c: libc::c_uint,
    pub v: [byte; 4],
}
#[no_mangle]
pub static mut draw_chars: *mut image_t = 0 as *const image_t as *mut image_t;
#[no_mangle]
pub unsafe extern "C" fn Draw_InitLocal() {
    draw_chars = GL_FindImage(
        b"pics/conchars.pcx\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        it_pic,
    );
    GL_Bind((*draw_chars).texnum);
}
#[no_mangle]
pub unsafe extern "C" fn Draw_Char(
    mut x: libc::c_int,
    mut y: libc::c_int,
    mut num: libc::c_int,
) {
    let mut row: libc::c_int = 0;
    let mut col: libc::c_int = 0;
    let mut frow: libc::c_float = 0.;
    let mut fcol: libc::c_float = 0.;
    let mut size: libc::c_float = 0.;
    num &= 255 as libc::c_int;
    if num & 127 as libc::c_int == 32 as libc::c_int {
        return;
    }
    if y <= -(8 as libc::c_int) {
        return;
    }
    row = num >> 4 as libc::c_int;
    col = num & 15 as libc::c_int;
    frow = (row as libc::c_double * 0.0625f64) as libc::c_float;
    fcol = (col as libc::c_double * 0.0625f64) as libc::c_float;
    size = 0.0625f64 as libc::c_float;
    GL_Bind((*draw_chars).texnum);
    qglTexCoord2f
        .expect("non-null function pointer")(fcol as libc::c_int, frow as libc::c_int);
    qglVertex2f.expect("non-null function pointer")(x, y);
    qglTexCoord2f
        .expect(
            "non-null function pointer",
        )((fcol + size) as libc::c_int, frow as libc::c_int);
    qglVertex2f.expect("non-null function pointer")(x + 8 as libc::c_int, y);
    qglTexCoord2f
        .expect(
            "non-null function pointer",
        )((fcol + size) as libc::c_int, (frow + size) as libc::c_int);
    qglVertex2f
        .expect("non-null function pointer")(x + 8 as libc::c_int, y + 8 as libc::c_int);
    qglTexCoord2f
        .expect(
            "non-null function pointer",
        )(fcol as libc::c_int, (frow + size) as libc::c_int);
    qglVertex2f.expect("non-null function pointer")(x, y + 8 as libc::c_int);
    qglEnd.expect("non-null function pointer")();
}
#[no_mangle]
pub unsafe extern "C" fn Draw_FindPic(mut name: *mut libc::c_char) -> *mut image_t {
    let mut gl: *mut image_t = 0 as *mut image_t;
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
        gl = GL_FindImage(fullname.as_mut_ptr(), it_pic);
    } else {
        gl = GL_FindImage(name.offset(1 as libc::c_int as isize), it_pic);
    }
    return gl;
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
pub unsafe extern "C" fn Draw_StretchPic(
    mut x: libc::c_int,
    mut y: libc::c_int,
    mut w: libc::c_int,
    mut h: libc::c_int,
    mut pic: *mut libc::c_char,
) {
    let mut gl: *mut image_t = 0 as *mut image_t;
    gl = Draw_FindPic(pic);
    if gl.is_null() {
        (ri.Con_Printf)
            .expect(
                "non-null function pointer",
            )(
            0 as libc::c_int,
            b"Can't find pic: %s\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            pic,
        );
        return;
    }
    if scrap_dirty as u64 != 0 {
        Scrap_Upload();
    }
    GL_Bind((*gl).texnum);
    qglTexCoord2f
        .expect(
            "non-null function pointer",
        )((*gl).sl as libc::c_int, (*gl).tl as libc::c_int);
    qglVertex2f.expect("non-null function pointer")(x, y);
    qglTexCoord2f
        .expect(
            "non-null function pointer",
        )((*gl).sh as libc::c_int, (*gl).tl as libc::c_int);
    qglVertex2f.expect("non-null function pointer")(x + w, y);
    qglTexCoord2f
        .expect(
            "non-null function pointer",
        )((*gl).sh as libc::c_int, (*gl).th as libc::c_int);
    qglVertex2f.expect("non-null function pointer")(x + w, y + h);
    qglTexCoord2f
        .expect(
            "non-null function pointer",
        )((*gl).sl as libc::c_int, (*gl).th as libc::c_int);
    qglVertex2f.expect("non-null function pointer")(x, y + h);
    qglEnd.expect("non-null function pointer")();
}
#[no_mangle]
pub unsafe extern "C" fn Draw_Pic(
    mut x: libc::c_int,
    mut y: libc::c_int,
    mut pic: *mut libc::c_char,
) {
    let mut gl: *mut image_t = 0 as *mut image_t;
    gl = Draw_FindPic(pic);
    if gl.is_null() {
        (ri.Con_Printf)
            .expect(
                "non-null function pointer",
            )(
            0 as libc::c_int,
            b"Can't find pic: %s\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            pic,
        );
        return;
    }
    if scrap_dirty as u64 != 0 {
        Scrap_Upload();
    }
    GL_Bind((*gl).texnum);
    qglTexCoord2f
        .expect(
            "non-null function pointer",
        )((*gl).sl as libc::c_int, (*gl).tl as libc::c_int);
    qglVertex2f.expect("non-null function pointer")(x, y);
    qglTexCoord2f
        .expect(
            "non-null function pointer",
        )((*gl).sh as libc::c_int, (*gl).tl as libc::c_int);
    qglVertex2f.expect("non-null function pointer")(x + (*gl).width, y);
    qglTexCoord2f
        .expect(
            "non-null function pointer",
        )((*gl).sh as libc::c_int, (*gl).th as libc::c_int);
    qglVertex2f.expect("non-null function pointer")(x + (*gl).width, y + (*gl).height);
    qglTexCoord2f
        .expect(
            "non-null function pointer",
        )((*gl).sl as libc::c_int, (*gl).th as libc::c_int);
    qglVertex2f.expect("non-null function pointer")(x, y + (*gl).height);
    qglEnd.expect("non-null function pointer")();
}
#[no_mangle]
pub unsafe extern "C" fn Draw_TileClear(
    mut x: libc::c_int,
    mut y: libc::c_int,
    mut w: libc::c_int,
    mut h: libc::c_int,
    mut pic: *mut libc::c_char,
) {
    let mut image: *mut image_t = 0 as *mut image_t;
    image = Draw_FindPic(pic);
    if image.is_null() {
        (ri.Con_Printf)
            .expect(
                "non-null function pointer",
            )(
            0 as libc::c_int,
            b"Can't find pic: %s\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            pic,
        );
        return;
    }
    GL_Bind((*image).texnum);
    qglTexCoord2f
        .expect(
            "non-null function pointer",
        )(
        (x as libc::c_double / 64.0f64) as libc::c_int,
        (y as libc::c_double / 64.0f64) as libc::c_int,
    );
    qglVertex2f.expect("non-null function pointer")(x, y);
    qglTexCoord2f
        .expect(
            "non-null function pointer",
        )(
        ((x + w) as libc::c_double / 64.0f64) as libc::c_int,
        (y as libc::c_double / 64.0f64) as libc::c_int,
    );
    qglVertex2f.expect("non-null function pointer")(x + w, y);
    qglTexCoord2f
        .expect(
            "non-null function pointer",
        )(
        ((x + w) as libc::c_double / 64.0f64) as libc::c_int,
        ((y + h) as libc::c_double / 64.0f64) as libc::c_int,
    );
    qglVertex2f.expect("non-null function pointer")(x + w, y + h);
    qglTexCoord2f
        .expect(
            "non-null function pointer",
        )(
        (x as libc::c_double / 64.0f64) as libc::c_int,
        ((y + h) as libc::c_double / 64.0f64) as libc::c_int,
    );
    qglVertex2f.expect("non-null function pointer")(x, y + h);
    qglEnd.expect("non-null function pointer")();
}
#[no_mangle]
pub unsafe extern "C" fn Draw_Fill(
    mut x: libc::c_int,
    mut y: libc::c_int,
    mut w: libc::c_int,
    mut h: libc::c_int,
    mut c: libc::c_int,
) {
    let mut color: C2RustUnnamed = C2RustUnnamed { c: 0 };
    if c as libc::c_uint > 255 as libc::c_int as libc::c_uint {
        (ri.Sys_Error)
            .expect(
                "non-null function pointer",
            )(
            0 as libc::c_int,
            b"Draw_Fill: bad color\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    color.c = d_8to24table[c as usize];
    qglColor3f
        .expect(
            "non-null function pointer",
        )(
        (color.v[0 as libc::c_int as usize] as libc::c_int as libc::c_double / 255.0f64)
            as libc::c_int,
        (color.v[1 as libc::c_int as usize] as libc::c_int as libc::c_double / 255.0f64)
            as libc::c_int,
        (color.v[2 as libc::c_int as usize] as libc::c_int as libc::c_double / 255.0f64)
            as libc::c_int,
    );
    qglVertex2f.expect("non-null function pointer")(x, y);
    qglVertex2f.expect("non-null function pointer")(x + w, y);
    qglVertex2f.expect("non-null function pointer")(x + w, y + h);
    qglVertex2f.expect("non-null function pointer")(x, y + h);
    qglEnd.expect("non-null function pointer")();
    qglColor3f
        .expect(
            "non-null function pointer",
        )(1 as libc::c_int, 1 as libc::c_int, 1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn Draw_FadeScreen() {
    qglColor4f
        .expect(
            "non-null function pointer",
        )(0 as libc::c_int, 0 as libc::c_int, 0 as libc::c_int, 0.8f64 as libc::c_int);
    qglVertex2f.expect("non-null function pointer")(0 as libc::c_int, 0 as libc::c_int);
    qglVertex2f
        .expect("non-null function pointer")(vid.width as libc::c_int, 0 as libc::c_int);
    qglVertex2f
        .expect(
            "non-null function pointer",
        )(vid.width as libc::c_int, vid.height as libc::c_int);
    qglVertex2f
        .expect(
            "non-null function pointer",
        )(0 as libc::c_int, vid.height as libc::c_int);
    qglEnd.expect("non-null function pointer")();
    qglColor4f
        .expect(
            "non-null function pointer",
        )(1 as libc::c_int, 1 as libc::c_int, 1 as libc::c_int, 1 as libc::c_int);
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
    let mut image32: [libc::c_uint; 65536] = [0; 65536];
    let mut image8: [libc::c_uchar; 65536] = [0; 65536];
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut trows: libc::c_int = 0;
    let mut source: *mut byte = 0 as *mut byte;
    let mut frac: libc::c_int = 0;
    let mut fracstep: libc::c_int = 0;
    let mut hscale: libc::c_float = 0.;
    let mut row: libc::c_int = 0;
    let mut t: libc::c_float = 0.;
    GL_Bind(0 as libc::c_int);
    if rows <= 256 as libc::c_int {
        hscale = 1 as libc::c_int as libc::c_float;
        trows = rows;
    } else {
        hscale = (rows as libc::c_double / 256.0f64) as libc::c_float;
        trows = 256 as libc::c_int;
    }
    t = rows as libc::c_float * hscale / 256 as libc::c_int as libc::c_float;
    if qglColorTableEXT.is_none() {
        let mut dest: *mut libc::c_uint = 0 as *mut libc::c_uint;
        i = 0 as libc::c_int;
        while i < trows {
            row = (i as libc::c_float * hscale) as libc::c_int;
            if row > rows {
                break;
            }
            source = data.offset((cols * row) as isize);
            dest = &mut *image32.as_mut_ptr().offset((i * 256 as libc::c_int) as isize)
                as *mut libc::c_uint;
            fracstep = cols * 0x10000 as libc::c_int / 256 as libc::c_int;
            frac = fracstep >> 1 as libc::c_int;
            j = 0 as libc::c_int;
            while j < 256 as libc::c_int {
                *dest
                    .offset(
                        j as isize,
                    ) = r_rawpalette[*source.offset((frac >> 16 as libc::c_int) as isize)
                    as usize];
                frac += fracstep;
                j += 1;
            }
            i += 1;
        }
    } else {
        let mut dest_0: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
        i = 0 as libc::c_int;
        while i < trows {
            row = (i as libc::c_float * hscale) as libc::c_int;
            if row > rows {
                break;
            }
            source = data.offset((cols * row) as isize);
            dest_0 = &mut *image8.as_mut_ptr().offset((i * 256 as libc::c_int) as isize)
                as *mut libc::c_uchar;
            fracstep = cols * 0x10000 as libc::c_int / 256 as libc::c_int;
            frac = fracstep >> 1 as libc::c_int;
            j = 0 as libc::c_int;
            while j < 256 as libc::c_int {
                *dest_0
                    .offset(
                        j as isize,
                    ) = *source.offset((frac >> 16 as libc::c_int) as isize);
                frac += fracstep;
                j += 1;
            }
            i += 1;
        }
    }
    qglTexCoord2f
        .expect("non-null function pointer")(0 as libc::c_int, 0 as libc::c_int);
    qglVertex2f.expect("non-null function pointer")(x, y);
    qglTexCoord2f
        .expect("non-null function pointer")(1 as libc::c_int, 0 as libc::c_int);
    qglVertex2f.expect("non-null function pointer")(x + w, y);
    qglTexCoord2f
        .expect("non-null function pointer")(1 as libc::c_int, t as libc::c_int);
    qglVertex2f.expect("non-null function pointer")(x + w, y + h);
    qglTexCoord2f
        .expect("non-null function pointer")(0 as libc::c_int, t as libc::c_int);
    qglVertex2f.expect("non-null function pointer")(x, y + h);
    qglEnd.expect("non-null function pointer")();
}
