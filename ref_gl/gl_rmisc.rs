#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(extern_types, register_tool)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fwrite(
        _: *const libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn Com_sprintf(
        dest: *mut libc::c_char,
        size: libc::c_int,
        fmt: *mut libc::c_char,
        _: ...
    );
    fn Sys_Mkdir(path: *mut libc::c_char);
    static mut qglClearColor: Option::<
        unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int, libc::c_int) -> (),
    >;
    static mut qglColor4f: Option::<
        unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int, libc::c_int) -> (),
    >;
    static mut gl_config: glconfig_t;
    fn GL_LoadPic(
        name: *mut libc::c_char,
        pic: *mut byte,
        width: libc::c_int,
        height: libc::c_int,
        type_0: imagetype_t,
        bits: libc::c_int,
    ) -> *mut image_t;
    static mut ri: refimport_t;
    static mut r_particletexture: *mut image_t;
    static mut r_notexture: *mut image_t;
    fn GL_TextureMode(string: *mut libc::c_char);
    static mut gl_texturemode: *mut cvar_t;
    fn GL_TextureAlphaMode(string: *mut libc::c_char);
    static mut gl_texturealphamode: *mut cvar_t;
    fn GL_TextureSolidMode(string: *mut libc::c_char);
    static mut gl_texturesolidmode: *mut cvar_t;
    static mut gl_particle_att_a: *mut cvar_t;
    static mut gl_particle_att_b: *mut cvar_t;
    static mut gl_particle_att_c: *mut cvar_t;
    static mut gl_particle_min_size: *mut cvar_t;
    static mut gl_particle_max_size: *mut cvar_t;
    static mut gl_ext_palettedtexture: *mut cvar_t;
    fn GL_SetTexturePalette(palette: *mut libc::c_uint);
    static mut d_8to24table: [libc::c_uint; 256];
    static mut gl_swapinterval: *mut cvar_t;
    static mut gl_state: glstate_t;
    static mut vid: viddef_t;
    static mut qglEnable: Option::<unsafe extern "C" fn(libc::c_int) -> ()>;
    static mut qglPointParameterfEXT: Option::<
        unsafe extern "C" fn(libc::c_int, libc::c_int) -> (),
    >;
    static mut qglPointParameterfvEXT: Option::<
        unsafe extern "C" fn(libc::c_int, *const libc::c_int) -> (),
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
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
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
pub struct glstate_t {
    pub inverse_intensity: libc::c_float,
    pub fullscreen: qboolean,
    pub prev_mode: libc::c_int,
    pub d_16to8table: *mut libc::c_uchar,
    pub lightmap_textures: libc::c_int,
    pub currenttextures: [libc::c_int; 2],
    pub currenttmu: libc::c_int,
    pub camera_separation: libc::c_float,
    pub stereo_enabled: qboolean,
    pub originalRedGammaTable: [libc::c_uchar; 256],
    pub originalGreenGammaTable: [libc::c_uchar; 256],
    pub originalBlueGammaTable: [libc::c_uchar; 256],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glconfig_t {
    pub renderer: libc::c_int,
    pub renderer_string: *const libc::c_char,
    pub vendor_string: *const libc::c_char,
    pub version_string: *const libc::c_char,
    pub extensions_string: *const libc::c_char,
    pub allow_cds: qboolean,
}
#[no_mangle]
pub static mut dottexture: [[byte; 8]; 8] = [
    [
        0 as libc::c_int as byte,
        0 as libc::c_int as byte,
        0 as libc::c_int as byte,
        0 as libc::c_int as byte,
        0 as libc::c_int as byte,
        0 as libc::c_int as byte,
        0 as libc::c_int as byte,
        0 as libc::c_int as byte,
    ],
    [
        0 as libc::c_int as byte,
        0 as libc::c_int as byte,
        1 as libc::c_int as byte,
        1 as libc::c_int as byte,
        0 as libc::c_int as byte,
        0 as libc::c_int as byte,
        0 as libc::c_int as byte,
        0 as libc::c_int as byte,
    ],
    [
        0 as libc::c_int as byte,
        1 as libc::c_int as byte,
        1 as libc::c_int as byte,
        1 as libc::c_int as byte,
        1 as libc::c_int as byte,
        0 as libc::c_int as byte,
        0 as libc::c_int as byte,
        0 as libc::c_int as byte,
    ],
    [
        0 as libc::c_int as byte,
        1 as libc::c_int as byte,
        1 as libc::c_int as byte,
        1 as libc::c_int as byte,
        1 as libc::c_int as byte,
        0 as libc::c_int as byte,
        0 as libc::c_int as byte,
        0 as libc::c_int as byte,
    ],
    [
        0 as libc::c_int as byte,
        0 as libc::c_int as byte,
        1 as libc::c_int as byte,
        1 as libc::c_int as byte,
        0 as libc::c_int as byte,
        0 as libc::c_int as byte,
        0 as libc::c_int as byte,
        0 as libc::c_int as byte,
    ],
    [
        0 as libc::c_int as byte,
        0 as libc::c_int as byte,
        0 as libc::c_int as byte,
        0 as libc::c_int as byte,
        0 as libc::c_int as byte,
        0 as libc::c_int as byte,
        0 as libc::c_int as byte,
        0 as libc::c_int as byte,
    ],
    [
        0 as libc::c_int as byte,
        0 as libc::c_int as byte,
        0 as libc::c_int as byte,
        0 as libc::c_int as byte,
        0 as libc::c_int as byte,
        0 as libc::c_int as byte,
        0 as libc::c_int as byte,
        0 as libc::c_int as byte,
    ],
    [
        0 as libc::c_int as byte,
        0 as libc::c_int as byte,
        0 as libc::c_int as byte,
        0 as libc::c_int as byte,
        0 as libc::c_int as byte,
        0 as libc::c_int as byte,
        0 as libc::c_int as byte,
        0 as libc::c_int as byte,
    ],
];
#[no_mangle]
pub unsafe extern "C" fn R_InitParticleTexture() {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut data: [[[byte; 4]; 8]; 8] = [[[0; 4]; 8]; 8];
    x = 0 as libc::c_int;
    while x < 8 as libc::c_int {
        y = 0 as libc::c_int;
        while y < 8 as libc::c_int {
            data[y
                as usize][x
                as usize][0 as libc::c_int as usize] = 255 as libc::c_int as byte;
            data[y
                as usize][x
                as usize][1 as libc::c_int as usize] = 255 as libc::c_int as byte;
            data[y
                as usize][x
                as usize][2 as libc::c_int as usize] = 255 as libc::c_int as byte;
            data[y
                as usize][x
                as usize][3 as libc::c_int
                as usize] = (dottexture[x as usize][y as usize] as libc::c_int
                * 255 as libc::c_int) as byte;
            y += 1;
        }
        x += 1;
    }
    r_particletexture = GL_LoadPic(
        b"***particle***\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        data.as_mut_ptr() as *mut byte,
        8 as libc::c_int,
        8 as libc::c_int,
        it_sprite,
        32 as libc::c_int,
    );
    x = 0 as libc::c_int;
    while x < 8 as libc::c_int {
        y = 0 as libc::c_int;
        while y < 8 as libc::c_int {
            data[y
                as usize][x
                as usize][0 as libc::c_int
                as usize] = (dottexture[(x & 3 as libc::c_int)
                as usize][(y & 3 as libc::c_int) as usize] as libc::c_int
                * 255 as libc::c_int) as byte;
            data[y
                as usize][x
                as usize][1 as libc::c_int as usize] = 0 as libc::c_int as byte;
            data[y
                as usize][x
                as usize][2 as libc::c_int as usize] = 0 as libc::c_int as byte;
            data[y
                as usize][x
                as usize][3 as libc::c_int as usize] = 255 as libc::c_int as byte;
            y += 1;
        }
        x += 1;
    }
    r_notexture = GL_LoadPic(
        b"***r_notexture***\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        data.as_mut_ptr() as *mut byte,
        8 as libc::c_int,
        8 as libc::c_int,
        it_wall,
        32 as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn GL_ScreenShot_f() {
    let mut buffer: *mut byte = 0 as *mut byte;
    let mut picname: [libc::c_char; 80] = [0; 80];
    let mut checkname: [libc::c_char; 128] = [0; 128];
    let mut i: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut temp: libc::c_int = 0;
    let mut f: *mut FILE = 0 as *mut FILE;
    Com_sprintf(
        checkname.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong as libc::c_int,
        b"%s/scrnshot\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        (ri.FS_Gamedir).expect("non-null function pointer")(),
    );
    Sys_Mkdir(checkname.as_mut_ptr());
    strcpy(picname.as_mut_ptr(), b"quake00.tga\0" as *const u8 as *const libc::c_char);
    i = 0 as libc::c_int;
    while i <= 99 as libc::c_int {
        picname[5 as libc::c_int
            as usize] = (i / 10 as libc::c_int + '0' as i32) as libc::c_char;
        picname[6 as libc::c_int
            as usize] = (i % 10 as libc::c_int + '0' as i32) as libc::c_char;
        Com_sprintf(
            checkname.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong as libc::c_int,
            b"%s/scrnshot/%s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            (ri.FS_Gamedir).expect("non-null function pointer")(),
            picname.as_mut_ptr(),
        );
        f = fopen(checkname.as_mut_ptr(), b"rb\0" as *const u8 as *const libc::c_char);
        if f.is_null() {
            break;
        }
        fclose(f);
        i += 1;
    }
    if i == 100 as libc::c_int {
        (ri.Con_Printf)
            .expect(
                "non-null function pointer",
            )(
            0 as libc::c_int,
            b"SCR_ScreenShot_f: Couldn't create a file\n\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        );
        return;
    }
    buffer = malloc(
        (vid.width)
            .wrapping_mul(vid.height)
            .wrapping_mul(3 as libc::c_int as libc::c_uint)
            .wrapping_add(18 as libc::c_int as libc::c_uint) as libc::c_ulong,
    ) as *mut byte;
    memset(
        buffer as *mut libc::c_void,
        0 as libc::c_int,
        18 as libc::c_int as libc::c_ulong,
    );
    *buffer.offset(2 as libc::c_int as isize) = 2 as libc::c_int as byte;
    *buffer
        .offset(
            12 as libc::c_int as isize,
        ) = (vid.width & 255 as libc::c_int as libc::c_uint) as byte;
    *buffer.offset(13 as libc::c_int as isize) = (vid.width >> 8 as libc::c_int) as byte;
    *buffer
        .offset(
            14 as libc::c_int as isize,
        ) = (vid.height & 255 as libc::c_int as libc::c_uint) as byte;
    *buffer
        .offset(15 as libc::c_int as isize) = (vid.height >> 8 as libc::c_int) as byte;
    *buffer.offset(16 as libc::c_int as isize) = 24 as libc::c_int as byte;
    c = (18 as libc::c_int as libc::c_uint)
        .wrapping_add(
            (vid.width)
                .wrapping_mul(vid.height)
                .wrapping_mul(3 as libc::c_int as libc::c_uint),
        ) as libc::c_int;
    i = 18 as libc::c_int;
    while i < c {
        temp = *buffer.offset(i as isize) as libc::c_int;
        *buffer.offset(i as isize) = *buffer.offset((i + 2 as libc::c_int) as isize);
        *buffer.offset((i + 2 as libc::c_int) as isize) = temp as byte;
        i += 3 as libc::c_int;
    }
    f = fopen(checkname.as_mut_ptr(), b"wb\0" as *const u8 as *const libc::c_char);
    fwrite(
        buffer as *const libc::c_void,
        1 as libc::c_int as libc::c_ulong,
        c as libc::c_ulong,
        f,
    );
    fclose(f);
    free(buffer as *mut libc::c_void);
    (ri.Con_Printf)
        .expect(
            "non-null function pointer",
        )(
        0 as libc::c_int,
        b"Wrote %s\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        picname.as_mut_ptr(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn GL_Strings_f() {
    (ri.Con_Printf)
        .expect(
            "non-null function pointer",
        )(
        0 as libc::c_int,
        b"GL_VENDOR: %s\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        gl_config.vendor_string,
    );
    (ri.Con_Printf)
        .expect(
            "non-null function pointer",
        )(
        0 as libc::c_int,
        b"GL_RENDERER: %s\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        gl_config.renderer_string,
    );
    (ri.Con_Printf)
        .expect(
            "non-null function pointer",
        )(
        0 as libc::c_int,
        b"GL_VERSION: %s\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        gl_config.version_string,
    );
    (ri.Con_Printf)
        .expect(
            "non-null function pointer",
        )(
        0 as libc::c_int,
        b"GL_EXTENSIONS: %s\n\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        gl_config.extensions_string,
    );
}
#[no_mangle]
pub unsafe extern "C" fn GL_SetDefaultState() {
    qglClearColor
        .expect(
            "non-null function pointer",
        )(
        1 as libc::c_int,
        0 as libc::c_int,
        0.5f64 as libc::c_int,
        0.5f64 as libc::c_int,
    );
    qglColor4f
        .expect(
            "non-null function pointer",
        )(1 as libc::c_int, 1 as libc::c_int, 1 as libc::c_int, 1 as libc::c_int);
    GL_TextureMode((*gl_texturemode).string);
    GL_TextureAlphaMode((*gl_texturealphamode).string);
    GL_TextureSolidMode((*gl_texturesolidmode).string);
    if qglPointParameterfEXT.is_some() {
        let mut attenuations: [libc::c_float; 3] = [0.; 3];
        attenuations[0 as libc::c_int as usize] = (*gl_particle_att_a).value;
        attenuations[1 as libc::c_int as usize] = (*gl_particle_att_b).value;
        attenuations[2 as libc::c_int as usize] = (*gl_particle_att_c).value;
        qglPointParameterfEXT
            .expect(
                "non-null function pointer",
            )(0x8126 as libc::c_int, (*gl_particle_min_size).value as libc::c_int);
        qglPointParameterfEXT
            .expect(
                "non-null function pointer",
            )(0x8127 as libc::c_int, (*gl_particle_max_size).value as libc::c_int);
        qglPointParameterfvEXT
            .expect(
                "non-null function pointer",
            )(0x8129 as libc::c_int, attenuations.as_mut_ptr() as *const libc::c_int);
    }
    if qglColorTableEXT.is_some() && (*gl_ext_palettedtexture).value != 0. {
        qglEnable.expect("non-null function pointer")(0x81fb as libc::c_int);
        GL_SetTexturePalette(d_8to24table.as_mut_ptr());
    }
    GL_UpdateSwapInterval();
}
#[no_mangle]
pub unsafe extern "C" fn GL_UpdateSwapInterval() {
    if (*gl_swapinterval).modified as u64 != 0 {
        (*gl_swapinterval).modified = false_0;
        gl_state.stereo_enabled as u64 == 0;
    }
}
