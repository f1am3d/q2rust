#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn LittleShort(l: libc::c_short) -> libc::c_short;
    fn LittleLong(l: libc::c_int) -> libc::c_int;
    static mut qglDeleteTextures: Option::<
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
    static mut r_notexture: *mut image_t;
    static mut r_particletexture: *mut image_t;
    static mut gl_ext_palettedtexture: *mut cvar_t;
    static mut gl_nobind: *mut cvar_t;
    static mut gl_round_down: *mut cvar_t;
    static mut gl_picmip: *mut cvar_t;
    static mut vid_gamma: *mut cvar_t;
    static mut registration_sequence: libc::c_int;
    static mut gl_state: glstate_t;
    static mut ri: refimport_t;
    static mut gl_config: glconfig_t;
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
pub struct pcx_t {
    pub manufacturer: libc::c_char,
    pub version: libc::c_char,
    pub encoding: libc::c_char,
    pub bits_per_pixel: libc::c_char,
    pub xmin: libc::c_ushort,
    pub ymin: libc::c_ushort,
    pub xmax: libc::c_ushort,
    pub ymax: libc::c_ushort,
    pub hres: libc::c_ushort,
    pub vres: libc::c_ushort,
    pub palette: [libc::c_uchar; 48],
    pub reserved: libc::c_char,
    pub color_planes: libc::c_char,
    pub bytes_per_line: libc::c_ushort,
    pub palette_type: libc::c_ushort,
    pub filler: [libc::c_char; 58],
    pub data: libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct miptex_s {
    pub name: [libc::c_char; 32],
    pub width: libc::c_uint,
    pub height: libc::c_uint,
    pub offsets: [libc::c_uint; 4],
    pub animname: [libc::c_char; 32],
    pub flags: libc::c_int,
    pub contents: libc::c_int,
    pub value: libc::c_int,
}
pub type miptex_t = miptex_s;
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
pub struct floodfill_t {
    pub x: libc::c_short,
    pub y: libc::c_short,
}
pub type TargaHeader = _TargaHeader;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _TargaHeader {
    pub id_length: libc::c_uchar,
    pub colormap_type: libc::c_uchar,
    pub image_type: libc::c_uchar,
    pub colormap_index: libc::c_ushort,
    pub colormap_length: libc::c_ushort,
    pub colormap_size: libc::c_uchar,
    pub x_origin: libc::c_ushort,
    pub y_origin: libc::c_ushort,
    pub width: libc::c_ushort,
    pub height: libc::c_ushort,
    pub pixel_size: libc::c_uchar,
    pub attributes: libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glmode_t {
    pub name: *mut libc::c_char,
    pub minimize: libc::c_int,
    pub maximize: libc::c_int,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gltmode_t {
    pub name: *mut libc::c_char,
    pub mode: libc::c_int,
}
#[no_mangle]
pub static mut gltextures: [image_t; 1024] = [image_t {
    name: [0; 64],
    type_0: it_skin,
    width: 0,
    height: 0,
    upload_width: 0,
    upload_height: 0,
    registration_sequence: 0,
    texturechain: 0 as *const msurface_s as *mut msurface_s,
    texnum: 0,
    sl: 0.,
    tl: 0.,
    sh: 0.,
    th: 0.,
    scrap: false_0,
    has_alpha: false_0,
    paletted: false_0,
}; 1024];
#[no_mangle]
pub static mut numgltextures: libc::c_int = 0;
#[no_mangle]
pub static mut base_textureid: libc::c_int = 0;
static mut intensitytable: [byte; 256] = [0; 256];
static mut gammatable: [libc::c_uchar; 256] = [0; 256];
#[no_mangle]
pub static mut intensity: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut d_8to24table: [libc::c_uint; 256] = [0; 256];
#[no_mangle]
pub static mut gl_solid_format: libc::c_int = 3 as libc::c_int;
#[no_mangle]
pub static mut gl_alpha_format: libc::c_int = 4 as libc::c_int;
#[no_mangle]
pub static mut gl_tex_solid_format: libc::c_int = 3 as libc::c_int;
#[no_mangle]
pub static mut gl_tex_alpha_format: libc::c_int = 4 as libc::c_int;
#[no_mangle]
pub static mut gl_filter_min: libc::c_int = 0;
#[no_mangle]
pub static mut gl_filter_max: libc::c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn GL_SetTexturePalette(mut palette: *mut libc::c_uint) {
    let mut i: libc::c_int = 0;
    let mut temptable: [libc::c_uchar; 768] = [0; 768];
    i = 0 as libc::c_int;
    while i < 256 as libc::c_int {
        temptable[(i * 3 as libc::c_int + 0 as libc::c_int)
            as usize] = (*palette.offset(i as isize) >> 0 as libc::c_int
            & 0xff as libc::c_int as libc::c_uint) as libc::c_uchar;
        temptable[(i * 3 as libc::c_int + 1 as libc::c_int)
            as usize] = (*palette.offset(i as isize) >> 8 as libc::c_int
            & 0xff as libc::c_int as libc::c_uint) as libc::c_uchar;
        temptable[(i * 3 as libc::c_int + 2 as libc::c_int)
            as usize] = (*palette.offset(i as isize) >> 16 as libc::c_int
            & 0xff as libc::c_int as libc::c_uint) as libc::c_uchar;
        i += 1;
    }
    qglColorTableEXT.is_some() && (*gl_ext_palettedtexture).value != 0.;
}
#[no_mangle]
pub unsafe extern "C" fn GL_Bind(mut texnum: libc::c_int) {
    extern "C" {
        static mut draw_chars: *mut image_t;
    }
    if (*gl_nobind).value != 0. && !draw_chars.is_null() {
        texnum = (*draw_chars).texnum;
    }
    if gl_state.currenttextures[gl_state.currenttmu as usize] == texnum {
        return;
    }
    gl_state.currenttextures[gl_state.currenttmu as usize] = texnum;
}
#[no_mangle]
pub static mut modes: [glmode_t; 0] = [];
#[no_mangle]
pub static mut gl_alpha_modes: [gltmode_t; 0] = [];
#[no_mangle]
pub static mut gl_solid_modes: [gltmode_t; 0] = [];
#[no_mangle]
pub unsafe extern "C" fn GL_ImageList_f() {
    let mut i: libc::c_int = 0;
    let mut image: *mut image_t = 0 as *mut image_t;
    let mut texels: libc::c_int = 0;
    let mut palstrings: [*const libc::c_char; 2] = [
        b"RGB\0" as *const u8 as *const libc::c_char,
        b"PAL\0" as *const u8 as *const libc::c_char,
    ];
    (ri.Con_Printf)
        .expect(
            "non-null function pointer",
        )(
        0 as libc::c_int,
        b"------------------\n\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    texels = 0 as libc::c_int;
    i = 0 as libc::c_int;
    image = gltextures.as_mut_ptr();
    while i < numgltextures {
        if !((*image).texnum <= 0 as libc::c_int) {
            texels += (*image).upload_width * (*image).upload_height;
            match (*image).type_0 as libc::c_uint {
                0 => {
                    (ri.Con_Printf)
                        .expect(
                            "non-null function pointer",
                        )(
                        0 as libc::c_int,
                        b"M\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    );
                }
                1 => {
                    (ri.Con_Printf)
                        .expect(
                            "non-null function pointer",
                        )(
                        0 as libc::c_int,
                        b"S\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    );
                }
                2 => {
                    (ri.Con_Printf)
                        .expect(
                            "non-null function pointer",
                        )(
                        0 as libc::c_int,
                        b"W\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    );
                }
                3 => {
                    (ri.Con_Printf)
                        .expect(
                            "non-null function pointer",
                        )(
                        0 as libc::c_int,
                        b"P\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    );
                }
                _ => {
                    (ri.Con_Printf)
                        .expect(
                            "non-null function pointer",
                        )(
                        0 as libc::c_int,
                        b" \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    );
                }
            }
            (ri.Con_Printf)
                .expect(
                    "non-null function pointer",
                )(
                0 as libc::c_int,
                b" %3i %3i %s: %s\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                (*image).upload_width,
                (*image).upload_height,
                palstrings[(*image).paletted as usize],
                ((*image).name).as_mut_ptr(),
            );
        }
        i += 1;
        image = image.offset(1);
    }
    (ri.Con_Printf)
        .expect(
            "non-null function pointer",
        )(
        0 as libc::c_int,
        b"Total texel count (not counting mipmaps): %i\n\0" as *const u8
            as *const libc::c_char as *mut libc::c_char,
        texels,
    );
}
#[no_mangle]
pub static mut scrap_allocated: [[libc::c_int; 256]; 1] = [[0; 256]; 1];
#[no_mangle]
pub static mut scrap_texels: [[byte; 65536]; 1] = [[0; 65536]; 1];
#[no_mangle]
pub static mut scrap_dirty: qboolean = false_0;
#[no_mangle]
pub unsafe extern "C" fn Scrap_AllocBlock(
    mut w: libc::c_int,
    mut h: libc::c_int,
    mut x: *mut libc::c_int,
    mut y: *mut libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut best: libc::c_int = 0;
    let mut best2: libc::c_int = 0;
    let mut texnum: libc::c_int = 0;
    texnum = 0 as libc::c_int;
    while texnum < 1 as libc::c_int {
        best = 256 as libc::c_int;
        i = 0 as libc::c_int;
        while i < 256 as libc::c_int - w {
            best2 = 0 as libc::c_int;
            j = 0 as libc::c_int;
            while j < w {
                if scrap_allocated[texnum as usize][(i + j) as usize] >= best {
                    break;
                }
                if scrap_allocated[texnum as usize][(i + j) as usize] > best2 {
                    best2 = scrap_allocated[texnum as usize][(i + j) as usize];
                }
                j += 1;
            }
            if j == w {
                *x = i;
                best = best2;
                *y = best;
            }
            i += 1;
        }
        if best + h > 256 as libc::c_int {
            texnum += 1;
        } else {
            i = 0 as libc::c_int;
            while i < w {
                scrap_allocated[texnum as usize][(*x + i) as usize] = best + h;
                i += 1;
            }
            return texnum;
        }
    }
    return -(1 as libc::c_int);
}
#[no_mangle]
pub static mut scrap_uploads: libc::c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn Scrap_Upload() {
    scrap_uploads += 1;
    GL_Bind(1152 as libc::c_int);
    GL_Upload8(
        (scrap_texels[0 as libc::c_int as usize]).as_mut_ptr(),
        256 as libc::c_int,
        256 as libc::c_int,
        false_0,
        false_0,
    );
    scrap_dirty = false_0;
}
#[no_mangle]
pub unsafe extern "C" fn LoadPCX(
    mut filename: *mut libc::c_char,
    mut pic: *mut *mut byte,
    mut palette: *mut *mut byte,
    mut width: *mut libc::c_int,
    mut height: *mut libc::c_int,
) {
    let mut raw: *mut byte = 0 as *mut byte;
    let mut pcx: *mut pcx_t = 0 as *mut pcx_t;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut dataByte: libc::c_int = 0;
    let mut runLength: libc::c_int = 0;
    let mut out: *mut byte = 0 as *mut byte;
    let mut pix: *mut byte = 0 as *mut byte;
    *pic = 0 as *mut byte;
    *palette = 0 as *mut byte;
    len = (ri.FS_LoadFile)
        .expect(
            "non-null function pointer",
        )(filename, &mut raw as *mut *mut byte as *mut *mut libc::c_void);
    if raw.is_null() {
        (ri.Con_Printf)
            .expect(
                "non-null function pointer",
            )(
            1 as libc::c_int,
            b"Bad pcx file %s\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            filename,
        );
        return;
    }
    pcx = raw as *mut pcx_t;
    (*pcx).xmin = LittleShort((*pcx).xmin as libc::c_short) as libc::c_ushort;
    (*pcx).ymin = LittleShort((*pcx).ymin as libc::c_short) as libc::c_ushort;
    (*pcx).xmax = LittleShort((*pcx).xmax as libc::c_short) as libc::c_ushort;
    (*pcx).ymax = LittleShort((*pcx).ymax as libc::c_short) as libc::c_ushort;
    (*pcx).hres = LittleShort((*pcx).hres as libc::c_short) as libc::c_ushort;
    (*pcx).vres = LittleShort((*pcx).vres as libc::c_short) as libc::c_ushort;
    (*pcx)
        .bytes_per_line = LittleShort((*pcx).bytes_per_line as libc::c_short)
        as libc::c_ushort;
    (*pcx)
        .palette_type = LittleShort((*pcx).palette_type as libc::c_short)
        as libc::c_ushort;
    raw = &mut (*pcx).data;
    if (*pcx).manufacturer as libc::c_int != 0xa as libc::c_int
        || (*pcx).version as libc::c_int != 5 as libc::c_int
        || (*pcx).encoding as libc::c_int != 1 as libc::c_int
        || (*pcx).bits_per_pixel as libc::c_int != 8 as libc::c_int
        || (*pcx).xmax as libc::c_int >= 640 as libc::c_int
        || (*pcx).ymax as libc::c_int >= 480 as libc::c_int
    {
        (ri.Con_Printf)
            .expect(
                "non-null function pointer",
            )(
            0 as libc::c_int,
            b"Bad pcx file %s\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            filename,
        );
        return;
    }
    out = malloc(
        (((*pcx).ymax as libc::c_int + 1 as libc::c_int)
            * ((*pcx).xmax as libc::c_int + 1 as libc::c_int)) as libc::c_ulong,
    ) as *mut byte;
    *pic = out;
    pix = out;
    if !palette.is_null() {
        *palette = malloc(768 as libc::c_int as libc::c_ulong) as *mut byte;
        memcpy(
            *palette as *mut libc::c_void,
            (pcx as *mut byte)
                .offset(len as isize)
                .offset(-(768 as libc::c_int as isize)) as *const libc::c_void,
            768 as libc::c_int as libc::c_ulong,
        );
    }
    if !width.is_null() {
        *width = (*pcx).xmax as libc::c_int + 1 as libc::c_int;
    }
    if !height.is_null() {
        *height = (*pcx).ymax as libc::c_int + 1 as libc::c_int;
    }
    y = 0 as libc::c_int;
    while y <= (*pcx).ymax as libc::c_int {
        x = 0 as libc::c_int;
        while x <= (*pcx).xmax as libc::c_int {
            let fresh0 = raw;
            raw = raw.offset(1);
            dataByte = *fresh0 as libc::c_int;
            if dataByte & 0xc0 as libc::c_int == 0xc0 as libc::c_int {
                runLength = dataByte & 0x3f as libc::c_int;
                let fresh1 = raw;
                raw = raw.offset(1);
                dataByte = *fresh1 as libc::c_int;
            } else {
                runLength = 1 as libc::c_int;
            }
            loop {
                let fresh2 = runLength;
                runLength = runLength - 1;
                if !(fresh2 > 0 as libc::c_int) {
                    break;
                }
                let fresh3 = x;
                x = x + 1;
                *pix.offset(fresh3 as isize) = dataByte as byte;
            }
        }
        y += 1;
        pix = pix.offset(((*pcx).xmax as libc::c_int + 1 as libc::c_int) as isize);
    }
    if raw.offset_from(pcx as *mut byte) as libc::c_long > len as libc::c_long {
        (ri.Con_Printf)
            .expect(
                "non-null function pointer",
            )(
            1 as libc::c_int,
            b"PCX file %s was malformed\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            filename,
        );
        free(*pic as *mut libc::c_void);
        *pic = 0 as *mut byte;
    }
    (ri.FS_FreeFile).expect("non-null function pointer")(pcx as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn LoadTGA(
    mut name: *mut libc::c_char,
    mut pic: *mut *mut byte,
    mut width: *mut libc::c_int,
    mut height: *mut libc::c_int,
) {
    let mut columns: libc::c_int = 0;
    let mut rows: libc::c_int = 0;
    let mut numPixels: libc::c_int = 0;
    let mut pixbuf: *mut byte = 0 as *mut byte;
    let mut row: libc::c_int = 0;
    let mut column: libc::c_int = 0;
    let mut buf_p: *mut byte = 0 as *mut byte;
    let mut buffer: *mut byte = 0 as *mut byte;
    let mut length: libc::c_int = 0;
    let mut targa_header: TargaHeader = TargaHeader {
        id_length: 0,
        colormap_type: 0,
        image_type: 0,
        colormap_index: 0,
        colormap_length: 0,
        colormap_size: 0,
        x_origin: 0,
        y_origin: 0,
        width: 0,
        height: 0,
        pixel_size: 0,
        attributes: 0,
    };
    let mut targa_rgba: *mut byte = 0 as *mut byte;
    let mut tmp: [byte; 2] = [0; 2];
    *pic = 0 as *mut byte;
    length = (ri.FS_LoadFile)
        .expect(
            "non-null function pointer",
        )(name, &mut buffer as *mut *mut byte as *mut *mut libc::c_void);
    if buffer.is_null() {
        (ri.Con_Printf)
            .expect(
                "non-null function pointer",
            )(
            1 as libc::c_int,
            b"Bad tga file %s\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            name,
        );
        return;
    }
    buf_p = buffer;
    let fresh4 = buf_p;
    buf_p = buf_p.offset(1);
    targa_header.id_length = *fresh4;
    let fresh5 = buf_p;
    buf_p = buf_p.offset(1);
    targa_header.colormap_type = *fresh5;
    let fresh6 = buf_p;
    buf_p = buf_p.offset(1);
    targa_header.image_type = *fresh6;
    tmp[0 as libc::c_int as usize] = *buf_p.offset(0 as libc::c_int as isize);
    tmp[1 as libc::c_int as usize] = *buf_p.offset(1 as libc::c_int as isize);
    targa_header
        .colormap_index = LittleShort(*(tmp.as_mut_ptr() as *mut libc::c_short))
        as libc::c_ushort;
    buf_p = buf_p.offset(2 as libc::c_int as isize);
    tmp[0 as libc::c_int as usize] = *buf_p.offset(0 as libc::c_int as isize);
    tmp[1 as libc::c_int as usize] = *buf_p.offset(1 as libc::c_int as isize);
    targa_header
        .colormap_length = LittleShort(*(tmp.as_mut_ptr() as *mut libc::c_short))
        as libc::c_ushort;
    buf_p = buf_p.offset(2 as libc::c_int as isize);
    let fresh7 = buf_p;
    buf_p = buf_p.offset(1);
    targa_header.colormap_size = *fresh7;
    targa_header
        .x_origin = LittleShort(*(buf_p as *mut libc::c_short)) as libc::c_ushort;
    buf_p = buf_p.offset(2 as libc::c_int as isize);
    targa_header
        .y_origin = LittleShort(*(buf_p as *mut libc::c_short)) as libc::c_ushort;
    buf_p = buf_p.offset(2 as libc::c_int as isize);
    targa_header.width = LittleShort(*(buf_p as *mut libc::c_short)) as libc::c_ushort;
    buf_p = buf_p.offset(2 as libc::c_int as isize);
    targa_header.height = LittleShort(*(buf_p as *mut libc::c_short)) as libc::c_ushort;
    buf_p = buf_p.offset(2 as libc::c_int as isize);
    let fresh8 = buf_p;
    buf_p = buf_p.offset(1);
    targa_header.pixel_size = *fresh8;
    let fresh9 = buf_p;
    buf_p = buf_p.offset(1);
    targa_header.attributes = *fresh9;
    if targa_header.image_type as libc::c_int != 2 as libc::c_int
        && targa_header.image_type as libc::c_int != 10 as libc::c_int
    {
        (ri.Sys_Error)
            .expect(
                "non-null function pointer",
            )(
            1 as libc::c_int,
            b"LoadTGA: Only type 2 and 10 targa RGB images supported\n\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        );
    }
    if targa_header.colormap_type as libc::c_int != 0 as libc::c_int
        || targa_header.pixel_size as libc::c_int != 32 as libc::c_int
            && targa_header.pixel_size as libc::c_int != 24 as libc::c_int
    {
        (ri.Sys_Error)
            .expect(
                "non-null function pointer",
            )(
            1 as libc::c_int,
            b"LoadTGA: Only 32 or 24 bit images supported (no colormaps)\n\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
    columns = targa_header.width as libc::c_int;
    rows = targa_header.height as libc::c_int;
    numPixels = columns * rows;
    if !width.is_null() {
        *width = columns;
    }
    if !height.is_null() {
        *height = rows;
    }
    targa_rgba = malloc((numPixels * 4 as libc::c_int) as libc::c_ulong) as *mut byte;
    *pic = targa_rgba;
    if targa_header.id_length as libc::c_int != 0 as libc::c_int {
        buf_p = buf_p.offset(targa_header.id_length as libc::c_int as isize);
    }
    if targa_header.image_type as libc::c_int == 2 as libc::c_int {
        row = rows - 1 as libc::c_int;
        while row >= 0 as libc::c_int {
            pixbuf = targa_rgba.offset((row * columns * 4 as libc::c_int) as isize);
            column = 0 as libc::c_int;
            while column < columns {
                let mut red: libc::c_uchar = 0;
                let mut green: libc::c_uchar = 0;
                let mut blue: libc::c_uchar = 0;
                let mut alphabyte: libc::c_uchar = 0;
                match targa_header.pixel_size as libc::c_int {
                    24 => {
                        let fresh10 = buf_p;
                        buf_p = buf_p.offset(1);
                        blue = *fresh10;
                        let fresh11 = buf_p;
                        buf_p = buf_p.offset(1);
                        green = *fresh11;
                        let fresh12 = buf_p;
                        buf_p = buf_p.offset(1);
                        red = *fresh12;
                        let fresh13 = pixbuf;
                        pixbuf = pixbuf.offset(1);
                        *fresh13 = red;
                        let fresh14 = pixbuf;
                        pixbuf = pixbuf.offset(1);
                        *fresh14 = green;
                        let fresh15 = pixbuf;
                        pixbuf = pixbuf.offset(1);
                        *fresh15 = blue;
                        let fresh16 = pixbuf;
                        pixbuf = pixbuf.offset(1);
                        *fresh16 = 255 as libc::c_int as byte;
                    }
                    32 => {
                        let fresh17 = buf_p;
                        buf_p = buf_p.offset(1);
                        blue = *fresh17;
                        let fresh18 = buf_p;
                        buf_p = buf_p.offset(1);
                        green = *fresh18;
                        let fresh19 = buf_p;
                        buf_p = buf_p.offset(1);
                        red = *fresh19;
                        let fresh20 = buf_p;
                        buf_p = buf_p.offset(1);
                        alphabyte = *fresh20;
                        let fresh21 = pixbuf;
                        pixbuf = pixbuf.offset(1);
                        *fresh21 = red;
                        let fresh22 = pixbuf;
                        pixbuf = pixbuf.offset(1);
                        *fresh22 = green;
                        let fresh23 = pixbuf;
                        pixbuf = pixbuf.offset(1);
                        *fresh23 = blue;
                        let fresh24 = pixbuf;
                        pixbuf = pixbuf.offset(1);
                        *fresh24 = alphabyte;
                    }
                    _ => {}
                }
                column += 1;
            }
            row -= 1;
        }
    } else if targa_header.image_type as libc::c_int == 10 as libc::c_int {
        let mut red_0: libc::c_uchar = 0;
        let mut green_0: libc::c_uchar = 0;
        let mut blue_0: libc::c_uchar = 0;
        let mut alphabyte_0: libc::c_uchar = 0;
        let mut packetHeader: libc::c_uchar = 0;
        let mut packetSize: libc::c_uchar = 0;
        let mut j: libc::c_uchar = 0;
        row = rows - 1 as libc::c_int;
        while row >= 0 as libc::c_int {
            pixbuf = targa_rgba.offset((row * columns * 4 as libc::c_int) as isize);
            column = 0 as libc::c_int;
            's_354: while column < columns {
                let fresh25 = buf_p;
                buf_p = buf_p.offset(1);
                packetHeader = *fresh25;
                packetSize = (1 as libc::c_int
                    + (packetHeader as libc::c_int & 0x7f as libc::c_int))
                    as libc::c_uchar;
                if packetHeader as libc::c_int & 0x80 as libc::c_int != 0 {
                    match targa_header.pixel_size as libc::c_int {
                        24 => {
                            let fresh26 = buf_p;
                            buf_p = buf_p.offset(1);
                            blue_0 = *fresh26;
                            let fresh27 = buf_p;
                            buf_p = buf_p.offset(1);
                            green_0 = *fresh27;
                            let fresh28 = buf_p;
                            buf_p = buf_p.offset(1);
                            red_0 = *fresh28;
                            alphabyte_0 = 255 as libc::c_int as libc::c_uchar;
                        }
                        32 => {
                            let fresh29 = buf_p;
                            buf_p = buf_p.offset(1);
                            blue_0 = *fresh29;
                            let fresh30 = buf_p;
                            buf_p = buf_p.offset(1);
                            green_0 = *fresh30;
                            let fresh31 = buf_p;
                            buf_p = buf_p.offset(1);
                            red_0 = *fresh31;
                            let fresh32 = buf_p;
                            buf_p = buf_p.offset(1);
                            alphabyte_0 = *fresh32;
                        }
                        _ => {}
                    }
                    j = 0 as libc::c_int as libc::c_uchar;
                    while (j as libc::c_int) < packetSize as libc::c_int {
                        let fresh33 = pixbuf;
                        pixbuf = pixbuf.offset(1);
                        *fresh33 = red_0;
                        let fresh34 = pixbuf;
                        pixbuf = pixbuf.offset(1);
                        *fresh34 = green_0;
                        let fresh35 = pixbuf;
                        pixbuf = pixbuf.offset(1);
                        *fresh35 = blue_0;
                        let fresh36 = pixbuf;
                        pixbuf = pixbuf.offset(1);
                        *fresh36 = alphabyte_0;
                        column += 1;
                        if column == columns {
                            column = 0 as libc::c_int;
                            if !(row > 0 as libc::c_int) {
                                break 's_354;
                            }
                            row -= 1;
                            pixbuf = targa_rgba
                                .offset((row * columns * 4 as libc::c_int) as isize);
                        }
                        j = j.wrapping_add(1);
                    }
                } else {
                    j = 0 as libc::c_int as libc::c_uchar;
                    while (j as libc::c_int) < packetSize as libc::c_int {
                        match targa_header.pixel_size as libc::c_int {
                            24 => {
                                let fresh37 = buf_p;
                                buf_p = buf_p.offset(1);
                                blue_0 = *fresh37;
                                let fresh38 = buf_p;
                                buf_p = buf_p.offset(1);
                                green_0 = *fresh38;
                                let fresh39 = buf_p;
                                buf_p = buf_p.offset(1);
                                red_0 = *fresh39;
                                let fresh40 = pixbuf;
                                pixbuf = pixbuf.offset(1);
                                *fresh40 = red_0;
                                let fresh41 = pixbuf;
                                pixbuf = pixbuf.offset(1);
                                *fresh41 = green_0;
                                let fresh42 = pixbuf;
                                pixbuf = pixbuf.offset(1);
                                *fresh42 = blue_0;
                                let fresh43 = pixbuf;
                                pixbuf = pixbuf.offset(1);
                                *fresh43 = 255 as libc::c_int as byte;
                            }
                            32 => {
                                let fresh44 = buf_p;
                                buf_p = buf_p.offset(1);
                                blue_0 = *fresh44;
                                let fresh45 = buf_p;
                                buf_p = buf_p.offset(1);
                                green_0 = *fresh45;
                                let fresh46 = buf_p;
                                buf_p = buf_p.offset(1);
                                red_0 = *fresh46;
                                let fresh47 = buf_p;
                                buf_p = buf_p.offset(1);
                                alphabyte_0 = *fresh47;
                                let fresh48 = pixbuf;
                                pixbuf = pixbuf.offset(1);
                                *fresh48 = red_0;
                                let fresh49 = pixbuf;
                                pixbuf = pixbuf.offset(1);
                                *fresh49 = green_0;
                                let fresh50 = pixbuf;
                                pixbuf = pixbuf.offset(1);
                                *fresh50 = blue_0;
                                let fresh51 = pixbuf;
                                pixbuf = pixbuf.offset(1);
                                *fresh51 = alphabyte_0;
                            }
                            _ => {}
                        }
                        column += 1;
                        if column == columns {
                            column = 0 as libc::c_int;
                            if !(row > 0 as libc::c_int) {
                                break 's_354;
                            }
                            row -= 1;
                            pixbuf = targa_rgba
                                .offset((row * columns * 4 as libc::c_int) as isize);
                        }
                        j = j.wrapping_add(1);
                    }
                }
            }
            row -= 1;
        }
    }
    (ri.FS_FreeFile).expect("non-null function pointer")(buffer as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn R_FloodFillSkin(
    mut skin: *mut byte,
    mut skinwidth: libc::c_int,
    mut skinheight: libc::c_int,
) {
    let mut fillcolor: byte = *skin;
    let mut fifo: [floodfill_t; 4096] = [floodfill_t { x: 0, y: 0 }; 4096];
    let mut inpt: libc::c_int = 0 as libc::c_int;
    let mut outpt: libc::c_int = 0 as libc::c_int;
    let mut filledcolor: libc::c_int = -(1 as libc::c_int);
    let mut i: libc::c_int = 0;
    if filledcolor == -(1 as libc::c_int) {
        filledcolor = 0 as libc::c_int;
        i = 0 as libc::c_int;
        while i < 256 as libc::c_int {
            if d_8to24table[i as usize]
                == ((255 as libc::c_int) << 0 as libc::c_int) as libc::c_uint
            {
                filledcolor = i;
                break;
            } else {
                i += 1;
            }
        }
    }
    if fillcolor as libc::c_int == filledcolor
        || fillcolor as libc::c_int == 255 as libc::c_int
    {
        return;
    }
    fifo[inpt as usize].x = 0 as libc::c_int as libc::c_short;
    fifo[inpt as usize].y = 0 as libc::c_int as libc::c_short;
    inpt = inpt + 1 as libc::c_int & 0x1000 as libc::c_int - 1 as libc::c_int;
    while outpt != inpt {
        let mut x: libc::c_int = fifo[outpt as usize].x as libc::c_int;
        let mut y: libc::c_int = fifo[outpt as usize].y as libc::c_int;
        let mut fdc: libc::c_int = filledcolor;
        let mut pos: *mut byte = &mut *skin.offset((x + skinwidth * y) as isize)
            as *mut byte;
        outpt = outpt + 1 as libc::c_int & 0x1000 as libc::c_int - 1 as libc::c_int;
        if x > 0 as libc::c_int {
            if *pos.offset(-(1 as libc::c_int) as isize) as libc::c_int
                == fillcolor as libc::c_int
            {
                *pos.offset(-(1 as libc::c_int) as isize) = 255 as libc::c_int as byte;
                fifo[inpt as usize].x = (x + -(1 as libc::c_int)) as libc::c_short;
                fifo[inpt as usize].y = (y + 0 as libc::c_int) as libc::c_short;
                inpt = inpt + 1 as libc::c_int
                    & 0x1000 as libc::c_int - 1 as libc::c_int;
            } else if *pos.offset(-(1 as libc::c_int) as isize) as libc::c_int
                != 255 as libc::c_int
            {
                fdc = *pos.offset(-(1 as libc::c_int) as isize) as libc::c_int;
            }
        }
        if x < skinwidth - 1 as libc::c_int {
            if *pos.offset(1 as libc::c_int as isize) as libc::c_int
                == fillcolor as libc::c_int
            {
                *pos.offset(1 as libc::c_int as isize) = 255 as libc::c_int as byte;
                fifo[inpt as usize].x = (x + 1 as libc::c_int) as libc::c_short;
                fifo[inpt as usize].y = (y + 0 as libc::c_int) as libc::c_short;
                inpt = inpt + 1 as libc::c_int
                    & 0x1000 as libc::c_int - 1 as libc::c_int;
            } else if *pos.offset(1 as libc::c_int as isize) as libc::c_int
                != 255 as libc::c_int
            {
                fdc = *pos.offset(1 as libc::c_int as isize) as libc::c_int;
            }
        }
        if y > 0 as libc::c_int {
            if *pos.offset(-skinwidth as isize) as libc::c_int
                == fillcolor as libc::c_int
            {
                *pos.offset(-skinwidth as isize) = 255 as libc::c_int as byte;
                fifo[inpt as usize].x = (x + 0 as libc::c_int) as libc::c_short;
                fifo[inpt as usize].y = (y + -(1 as libc::c_int)) as libc::c_short;
                inpt = inpt + 1 as libc::c_int
                    & 0x1000 as libc::c_int - 1 as libc::c_int;
            } else if *pos.offset(-skinwidth as isize) as libc::c_int
                != 255 as libc::c_int
            {
                fdc = *pos.offset(-skinwidth as isize) as libc::c_int;
            }
        }
        if y < skinheight - 1 as libc::c_int {
            if *pos.offset(skinwidth as isize) as libc::c_int == fillcolor as libc::c_int
            {
                *pos.offset(skinwidth as isize) = 255 as libc::c_int as byte;
                fifo[inpt as usize].x = (x + 0 as libc::c_int) as libc::c_short;
                fifo[inpt as usize].y = (y + 1 as libc::c_int) as libc::c_short;
                inpt = inpt + 1 as libc::c_int
                    & 0x1000 as libc::c_int - 1 as libc::c_int;
            } else if *pos.offset(skinwidth as isize) as libc::c_int
                != 255 as libc::c_int
            {
                fdc = *pos.offset(skinwidth as isize) as libc::c_int;
            }
        }
        *skin.offset((x + skinwidth * y) as isize) = fdc as byte;
    }
}
#[no_mangle]
pub unsafe extern "C" fn GL_ResampleTexture(
    mut in_0: *mut libc::c_uint,
    mut inwidth: libc::c_int,
    mut inheight: libc::c_int,
    mut out: *mut libc::c_uint,
    mut outwidth: libc::c_int,
    mut outheight: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut inrow: *mut libc::c_uint = 0 as *mut libc::c_uint;
    let mut inrow2: *mut libc::c_uint = 0 as *mut libc::c_uint;
    let mut frac: libc::c_uint = 0;
    let mut fracstep: libc::c_uint = 0;
    let mut p1: [libc::c_uint; 1024] = [0; 1024];
    let mut p2: [libc::c_uint; 1024] = [0; 1024];
    let mut pix1: *mut byte = 0 as *mut byte;
    let mut pix2: *mut byte = 0 as *mut byte;
    let mut pix3: *mut byte = 0 as *mut byte;
    let mut pix4: *mut byte = 0 as *mut byte;
    fracstep = (inwidth * 0x10000 as libc::c_int / outwidth) as libc::c_uint;
    frac = fracstep >> 2 as libc::c_int;
    i = 0 as libc::c_int;
    while i < outwidth {
        p1[i
            as usize] = (4 as libc::c_int as libc::c_uint)
            .wrapping_mul(frac >> 16 as libc::c_int);
        frac = frac.wrapping_add(fracstep);
        i += 1;
    }
    frac = (3 as libc::c_int as libc::c_uint).wrapping_mul(fracstep >> 2 as libc::c_int);
    i = 0 as libc::c_int;
    while i < outwidth {
        p2[i
            as usize] = (4 as libc::c_int as libc::c_uint)
            .wrapping_mul(frac >> 16 as libc::c_int);
        frac = frac.wrapping_add(fracstep);
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < outheight {
        inrow = in_0
            .offset(
                (inwidth
                    * ((i as libc::c_double + 0.25f64) * inheight as libc::c_double
                        / outheight as libc::c_double) as libc::c_int) as isize,
            );
        inrow2 = in_0
            .offset(
                (inwidth
                    * ((i as libc::c_double + 0.75f64) * inheight as libc::c_double
                        / outheight as libc::c_double) as libc::c_int) as isize,
            );
        frac = fracstep >> 1 as libc::c_int;
        j = 0 as libc::c_int;
        while j < outwidth {
            pix1 = (inrow as *mut byte).offset(p1[j as usize] as isize);
            pix2 = (inrow as *mut byte).offset(p2[j as usize] as isize);
            pix3 = (inrow2 as *mut byte).offset(p1[j as usize] as isize);
            pix4 = (inrow2 as *mut byte).offset(p2[j as usize] as isize);
            *(out.offset(j as isize) as *mut byte)
                .offset(
                    0 as libc::c_int as isize,
                ) = (*pix1.offset(0 as libc::c_int as isize) as libc::c_int
                + *pix2.offset(0 as libc::c_int as isize) as libc::c_int
                + *pix3.offset(0 as libc::c_int as isize) as libc::c_int
                + *pix4.offset(0 as libc::c_int as isize) as libc::c_int
                >> 2 as libc::c_int) as byte;
            *(out.offset(j as isize) as *mut byte)
                .offset(
                    1 as libc::c_int as isize,
                ) = (*pix1.offset(1 as libc::c_int as isize) as libc::c_int
                + *pix2.offset(1 as libc::c_int as isize) as libc::c_int
                + *pix3.offset(1 as libc::c_int as isize) as libc::c_int
                + *pix4.offset(1 as libc::c_int as isize) as libc::c_int
                >> 2 as libc::c_int) as byte;
            *(out.offset(j as isize) as *mut byte)
                .offset(
                    2 as libc::c_int as isize,
                ) = (*pix1.offset(2 as libc::c_int as isize) as libc::c_int
                + *pix2.offset(2 as libc::c_int as isize) as libc::c_int
                + *pix3.offset(2 as libc::c_int as isize) as libc::c_int
                + *pix4.offset(2 as libc::c_int as isize) as libc::c_int
                >> 2 as libc::c_int) as byte;
            *(out.offset(j as isize) as *mut byte)
                .offset(
                    3 as libc::c_int as isize,
                ) = (*pix1.offset(3 as libc::c_int as isize) as libc::c_int
                + *pix2.offset(3 as libc::c_int as isize) as libc::c_int
                + *pix3.offset(3 as libc::c_int as isize) as libc::c_int
                + *pix4.offset(3 as libc::c_int as isize) as libc::c_int
                >> 2 as libc::c_int) as byte;
            j += 1;
        }
        i += 1;
        out = out.offset(outwidth as isize);
    }
}
#[no_mangle]
pub unsafe extern "C" fn GL_LightScaleTexture(
    mut in_0: *mut libc::c_uint,
    mut inwidth: libc::c_int,
    mut inheight: libc::c_int,
    mut only_gamma: qboolean,
) {
    if only_gamma as u64 != 0 {
        let mut i: libc::c_int = 0;
        let mut c: libc::c_int = 0;
        let mut p: *mut byte = 0 as *mut byte;
        p = in_0 as *mut byte;
        c = inwidth * inheight;
        i = 0 as libc::c_int;
        while i < c {
            *p
                .offset(
                    0 as libc::c_int as isize,
                ) = gammatable[*p.offset(0 as libc::c_int as isize) as usize];
            *p
                .offset(
                    1 as libc::c_int as isize,
                ) = gammatable[*p.offset(1 as libc::c_int as isize) as usize];
            *p
                .offset(
                    2 as libc::c_int as isize,
                ) = gammatable[*p.offset(2 as libc::c_int as isize) as usize];
            i += 1;
            p = p.offset(4 as libc::c_int as isize);
        }
    } else {
        let mut i_0: libc::c_int = 0;
        let mut c_0: libc::c_int = 0;
        let mut p_0: *mut byte = 0 as *mut byte;
        p_0 = in_0 as *mut byte;
        c_0 = inwidth * inheight;
        i_0 = 0 as libc::c_int;
        while i_0 < c_0 {
            *p_0
                .offset(
                    0 as libc::c_int as isize,
                ) = gammatable[intensitytable[*p_0.offset(0 as libc::c_int as isize)
                as usize] as usize];
            *p_0
                .offset(
                    1 as libc::c_int as isize,
                ) = gammatable[intensitytable[*p_0.offset(1 as libc::c_int as isize)
                as usize] as usize];
            *p_0
                .offset(
                    2 as libc::c_int as isize,
                ) = gammatable[intensitytable[*p_0.offset(2 as libc::c_int as isize)
                as usize] as usize];
            i_0 += 1;
            p_0 = p_0.offset(4 as libc::c_int as isize);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn GL_MipMap(
    mut in_0: *mut byte,
    mut width: libc::c_int,
    mut height: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut out: *mut byte = 0 as *mut byte;
    width <<= 2 as libc::c_int;
    height >>= 1 as libc::c_int;
    out = in_0;
    i = 0 as libc::c_int;
    while i < height {
        j = 0 as libc::c_int;
        while j < width {
            *out
                .offset(
                    0 as libc::c_int as isize,
                ) = (*in_0.offset(0 as libc::c_int as isize) as libc::c_int
                + *in_0.offset(4 as libc::c_int as isize) as libc::c_int
                + *in_0.offset((width + 0 as libc::c_int) as isize) as libc::c_int
                + *in_0.offset((width + 4 as libc::c_int) as isize) as libc::c_int
                >> 2 as libc::c_int) as byte;
            *out
                .offset(
                    1 as libc::c_int as isize,
                ) = (*in_0.offset(1 as libc::c_int as isize) as libc::c_int
                + *in_0.offset(5 as libc::c_int as isize) as libc::c_int
                + *in_0.offset((width + 1 as libc::c_int) as isize) as libc::c_int
                + *in_0.offset((width + 5 as libc::c_int) as isize) as libc::c_int
                >> 2 as libc::c_int) as byte;
            *out
                .offset(
                    2 as libc::c_int as isize,
                ) = (*in_0.offset(2 as libc::c_int as isize) as libc::c_int
                + *in_0.offset(6 as libc::c_int as isize) as libc::c_int
                + *in_0.offset((width + 2 as libc::c_int) as isize) as libc::c_int
                + *in_0.offset((width + 6 as libc::c_int) as isize) as libc::c_int
                >> 2 as libc::c_int) as byte;
            *out
                .offset(
                    3 as libc::c_int as isize,
                ) = (*in_0.offset(3 as libc::c_int as isize) as libc::c_int
                + *in_0.offset(7 as libc::c_int as isize) as libc::c_int
                + *in_0.offset((width + 3 as libc::c_int) as isize) as libc::c_int
                + *in_0.offset((width + 7 as libc::c_int) as isize) as libc::c_int
                >> 2 as libc::c_int) as byte;
            j += 8 as libc::c_int;
            out = out.offset(4 as libc::c_int as isize);
            in_0 = in_0.offset(8 as libc::c_int as isize);
        }
        i += 1;
        in_0 = in_0.offset(width as isize);
    }
}
#[no_mangle]
pub unsafe extern "C" fn GL_BuildPalettedTexture(
    mut paletted_texture: *mut libc::c_uchar,
    mut scaled: *mut libc::c_uchar,
    mut scaled_width: libc::c_int,
    mut scaled_height: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < scaled_width * scaled_height {
        let mut r: libc::c_uint = 0;
        let mut g: libc::c_uint = 0;
        let mut b: libc::c_uint = 0;
        let mut c: libc::c_uint = 0;
        r = (*scaled.offset(0 as libc::c_int as isize) as libc::c_int >> 3 as libc::c_int
            & 31 as libc::c_int) as libc::c_uint;
        g = (*scaled.offset(1 as libc::c_int as isize) as libc::c_int >> 2 as libc::c_int
            & 63 as libc::c_int) as libc::c_uint;
        b = (*scaled.offset(2 as libc::c_int as isize) as libc::c_int >> 3 as libc::c_int
            & 31 as libc::c_int) as libc::c_uint;
        c = r | g << 5 as libc::c_int | b << 11 as libc::c_int;
        *paletted_texture
            .offset(i as isize) = *(gl_state.d_16to8table).offset(c as isize);
        scaled = scaled.offset(4 as libc::c_int as isize);
        i += 1;
    }
}
#[no_mangle]
pub static mut upload_width: libc::c_int = 0;
#[no_mangle]
pub static mut upload_height: libc::c_int = 0;
#[no_mangle]
pub static mut uploaded_paletted: qboolean = false_0;
#[no_mangle]
pub unsafe extern "C" fn GL_Upload32(
    mut data: *mut libc::c_uint,
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut mipmap: qboolean,
) -> qboolean {
    let mut current_block: u64;
    let mut samples: libc::c_int = 0;
    let mut scaled: [libc::c_uint; 65536] = [0; 65536];
    let mut paletted_texture: [libc::c_uchar; 65536] = [0; 65536];
    let mut scaled_width: libc::c_int = 0;
    let mut scaled_height: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut scan: *mut byte = 0 as *mut byte;
    let mut comp: libc::c_int = 0;
    uploaded_paletted = false_0;
    scaled_width = 1 as libc::c_int;
    while scaled_width < width {
        scaled_width <<= 1 as libc::c_int;
    }
    if (*gl_round_down).value != 0. && scaled_width > width
        && mipmap as libc::c_uint != 0
    {
        scaled_width >>= 1 as libc::c_int;
    }
    scaled_height = 1 as libc::c_int;
    while scaled_height < height {
        scaled_height <<= 1 as libc::c_int;
    }
    if (*gl_round_down).value != 0. && scaled_height > height
        && mipmap as libc::c_uint != 0
    {
        scaled_height >>= 1 as libc::c_int;
    }
    if mipmap as u64 != 0 {
        scaled_width >>= (*gl_picmip).value as libc::c_int;
        scaled_height >>= (*gl_picmip).value as libc::c_int;
    }
    if scaled_width > 256 as libc::c_int {
        scaled_width = 256 as libc::c_int;
    }
    if scaled_height > 256 as libc::c_int {
        scaled_height = 256 as libc::c_int;
    }
    if scaled_width < 1 as libc::c_int {
        scaled_width = 1 as libc::c_int;
    }
    if scaled_height < 1 as libc::c_int {
        scaled_height = 1 as libc::c_int;
    }
    upload_width = scaled_width;
    upload_height = scaled_height;
    if (scaled_width * scaled_height) as libc::c_ulong
        > (::std::mem::size_of::<[libc::c_uint; 65536]>() as libc::c_ulong)
            .wrapping_div(4 as libc::c_int as libc::c_ulong)
    {
        (ri.Sys_Error)
            .expect(
                "non-null function pointer",
            )(
            1 as libc::c_int,
            b"GL_Upload32: too big\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    c = width * height;
    scan = (data as *mut byte).offset(3 as libc::c_int as isize);
    samples = gl_solid_format;
    i = 0 as libc::c_int;
    while i < c {
        if *scan as libc::c_int != 255 as libc::c_int {
            samples = gl_alpha_format;
            break;
        } else {
            i += 1;
            scan = scan.offset(4 as libc::c_int as isize);
        }
    }
    if samples == gl_solid_format {
        comp = gl_tex_solid_format;
    } else if samples == gl_alpha_format {
        comp = gl_tex_alpha_format;
    } else {
        (ri.Con_Printf)
            .expect(
                "non-null function pointer",
            )(
            0 as libc::c_int,
            b"Unknown number of texture components %i\n\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            samples,
        );
        comp = samples;
    }
    if scaled_width == width && scaled_height == height {
        if mipmap as u64 == 0 {
            if qglColorTableEXT.is_some() && (*gl_ext_palettedtexture).value != 0.
                && samples == gl_solid_format
            {
                uploaded_paletted = true_0;
                GL_BuildPalettedTexture(
                    paletted_texture.as_mut_ptr(),
                    data as *mut libc::c_uchar,
                    scaled_width,
                    scaled_height,
                );
            }
            current_block = 11796148217846552555;
        } else {
            memcpy(
                scaled.as_mut_ptr() as *mut libc::c_void,
                data as *const libc::c_void,
                (width * height * 4 as libc::c_int) as libc::c_ulong,
            );
            current_block = 2516253395664191498;
        }
    } else {
        GL_ResampleTexture(
            data,
            width,
            height,
            scaled.as_mut_ptr(),
            scaled_width,
            scaled_height,
        );
        current_block = 2516253395664191498;
    }
    match current_block {
        2516253395664191498 => {
            GL_LightScaleTexture(
                scaled.as_mut_ptr(),
                scaled_width,
                scaled_height,
                (mipmap as u64 == 0) as libc::c_int as qboolean,
            );
            if qglColorTableEXT.is_some() && (*gl_ext_palettedtexture).value != 0.
                && samples == gl_solid_format
            {
                uploaded_paletted = true_0;
                GL_BuildPalettedTexture(
                    paletted_texture.as_mut_ptr(),
                    scaled.as_mut_ptr() as *mut libc::c_uchar,
                    scaled_width,
                    scaled_height,
                );
            }
            if mipmap as u64 != 0 {
                let mut miplevel: libc::c_int = 0;
                miplevel = 0 as libc::c_int;
                while scaled_width > 1 as libc::c_int || scaled_height > 1 as libc::c_int
                {
                    GL_MipMap(
                        scaled.as_mut_ptr() as *mut byte,
                        scaled_width,
                        scaled_height,
                    );
                    scaled_width >>= 1 as libc::c_int;
                    scaled_height >>= 1 as libc::c_int;
                    if scaled_width < 1 as libc::c_int {
                        scaled_width = 1 as libc::c_int;
                    }
                    if scaled_height < 1 as libc::c_int {
                        scaled_height = 1 as libc::c_int;
                    }
                    miplevel += 1;
                    if qglColorTableEXT.is_some()
                        && (*gl_ext_palettedtexture).value != 0.
                        && samples == gl_solid_format
                    {
                        uploaded_paletted = true_0;
                        GL_BuildPalettedTexture(
                            paletted_texture.as_mut_ptr(),
                            scaled.as_mut_ptr() as *mut libc::c_uchar,
                            scaled_width,
                            scaled_height,
                        );
                    }
                }
            }
        }
        _ => {}
    }
    mipmap as u64 != 0;
    return (samples == gl_alpha_format) as libc::c_int as qboolean;
}
#[no_mangle]
pub unsafe extern "C" fn GL_Upload8(
    mut data: *mut byte,
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut mipmap: qboolean,
    mut is_sky: qboolean,
) -> qboolean {
    let mut trans: [libc::c_uint; 131072] = [0; 131072];
    let mut i: libc::c_int = 0;
    let mut s: libc::c_int = 0;
    let mut p: libc::c_int = 0;
    s = width * height;
    if s as libc::c_ulong
        > (::std::mem::size_of::<[libc::c_uint; 131072]>() as libc::c_ulong)
            .wrapping_div(4 as libc::c_int as libc::c_ulong)
    {
        (ri.Sys_Error)
            .expect(
                "non-null function pointer",
            )(
            1 as libc::c_int,
            b"GL_Upload8: too large\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    if qglColorTableEXT.is_some() && (*gl_ext_palettedtexture).value != 0.
        && is_sky as libc::c_uint != 0
    {} else {
        i = 0 as libc::c_int;
        while i < s {
            p = *data.offset(i as isize) as libc::c_int;
            trans[i as usize] = d_8to24table[p as usize];
            if p == 255 as libc::c_int {
                if i > width
                    && *data.offset((i - width) as isize) as libc::c_int
                        != 255 as libc::c_int
                {
                    p = *data.offset((i - width) as isize) as libc::c_int;
                } else if i < s - width
                    && *data.offset((i + width) as isize) as libc::c_int
                        != 255 as libc::c_int
                {
                    p = *data.offset((i + width) as isize) as libc::c_int;
                } else if i > 0 as libc::c_int
                    && *data.offset((i - 1 as libc::c_int) as isize) as libc::c_int
                        != 255 as libc::c_int
                {
                    p = *data.offset((i - 1 as libc::c_int) as isize) as libc::c_int;
                } else if i < s - 1 as libc::c_int
                    && *data.offset((i + 1 as libc::c_int) as isize) as libc::c_int
                        != 255 as libc::c_int
                {
                    p = *data.offset((i + 1 as libc::c_int) as isize) as libc::c_int;
                } else {
                    p = 0 as libc::c_int;
                }
                *(&mut *trans.as_mut_ptr().offset(i as isize) as *mut libc::c_uint
                    as *mut byte)
                    .offset(
                        0 as libc::c_int as isize,
                    ) = *(&mut *d_8to24table.as_mut_ptr().offset(p as isize)
                    as *mut libc::c_uint as *mut byte)
                    .offset(0 as libc::c_int as isize);
                *(&mut *trans.as_mut_ptr().offset(i as isize) as *mut libc::c_uint
                    as *mut byte)
                    .offset(
                        1 as libc::c_int as isize,
                    ) = *(&mut *d_8to24table.as_mut_ptr().offset(p as isize)
                    as *mut libc::c_uint as *mut byte)
                    .offset(1 as libc::c_int as isize);
                *(&mut *trans.as_mut_ptr().offset(i as isize) as *mut libc::c_uint
                    as *mut byte)
                    .offset(
                        2 as libc::c_int as isize,
                    ) = *(&mut *d_8to24table.as_mut_ptr().offset(p as isize)
                    as *mut libc::c_uint as *mut byte)
                    .offset(2 as libc::c_int as isize);
            }
            i += 1;
        }
        return GL_Upload32(trans.as_mut_ptr(), width, height, mipmap);
    }
    panic!("Reached end of non-void function without returning");
}
#[no_mangle]
pub unsafe extern "C" fn GL_LoadPic(
    mut name: *mut libc::c_char,
    mut pic: *mut byte,
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut type_0: imagetype_t,
    mut bits: libc::c_int,
) -> *mut image_t {
    let mut image: *mut image_t = 0 as *mut image_t;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    image = gltextures.as_mut_ptr();
    while i < numgltextures {
        if (*image).texnum == 0 {
            break;
        }
        i += 1;
        image = image.offset(1);
    }
    if i == numgltextures {
        if numgltextures == 1024 as libc::c_int {
            (ri.Sys_Error)
                .expect(
                    "non-null function pointer",
                )(
                1 as libc::c_int,
                b"MAX_GLTEXTURES\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
        }
        numgltextures += 1;
    }
    image = &mut *gltextures.as_mut_ptr().offset(i as isize) as *mut image_t;
    if strlen(name) >= ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong {
        (ri.Sys_Error)
            .expect(
                "non-null function pointer",
            )(
            1 as libc::c_int,
            b"Draw_LoadPic: \"%s\" is too long\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            name,
        );
    }
    strcpy(((*image).name).as_mut_ptr(), name);
    (*image).registration_sequence = registration_sequence;
    (*image).width = width;
    (*image).height = height;
    (*image).type_0 = type_0;
    if type_0 as libc::c_uint == it_skin as libc::c_int as libc::c_uint
        && bits == 8 as libc::c_int
    {
        R_FloodFillSkin(pic, width, height);
    }
    let mut current_block_44: u64;
    if (*image).type_0 as libc::c_uint == it_pic as libc::c_int as libc::c_uint
        && bits == 8 as libc::c_int && (*image).width < 64 as libc::c_int
        && (*image).height < 64 as libc::c_int
    {
        let mut x: libc::c_int = 0;
        let mut y: libc::c_int = 0;
        let mut i_0: libc::c_int = 0;
        let mut j: libc::c_int = 0;
        let mut k: libc::c_int = 0;
        let mut texnum: libc::c_int = 0;
        texnum = Scrap_AllocBlock((*image).width, (*image).height, &mut x, &mut y);
        if texnum == -(1 as libc::c_int) {
            current_block_44 = 4330951968453410359;
        } else {
            scrap_dirty = true_0;
            k = 0 as libc::c_int;
            i_0 = 0 as libc::c_int;
            while i_0 < (*image).height {
                j = 0 as libc::c_int;
                while j < (*image).width {
                    scrap_texels[texnum
                        as usize][((y + i_0) * 256 as libc::c_int + x + j)
                        as usize] = *pic.offset(k as isize);
                    j += 1;
                    k += 1;
                }
                i_0 += 1;
            }
            (*image).texnum = 1152 as libc::c_int + texnum;
            (*image).scrap = true_0;
            (*image).has_alpha = true_0;
            (*image)
                .sl = ((x as libc::c_double + 0.01f64)
                / 256 as libc::c_int as libc::c_float as libc::c_double)
                as libc::c_float;
            (*image)
                .sh = (((x + (*image).width) as libc::c_double - 0.01f64)
                / 256 as libc::c_int as libc::c_float as libc::c_double)
                as libc::c_float;
            (*image)
                .tl = ((y as libc::c_double + 0.01f64)
                / 256 as libc::c_int as libc::c_float as libc::c_double)
                as libc::c_float;
            (*image)
                .th = (((y + (*image).height) as libc::c_double - 0.01f64)
                / 256 as libc::c_int as libc::c_float as libc::c_double)
                as libc::c_float;
            current_block_44 = 10150597327160359210;
        }
    } else {
        current_block_44 = 4330951968453410359;
    }
    match current_block_44 {
        4330951968453410359 => {
            (*image).scrap = false_0;
            (*image)
                .texnum = (1153 as libc::c_int as libc::c_long
                + image.offset_from(gltextures.as_mut_ptr()) as libc::c_long)
                as libc::c_int;
            GL_Bind((*image).texnum);
            if bits == 8 as libc::c_int {
                (*image)
                    .has_alpha = GL_Upload8(
                    pic,
                    width,
                    height,
                    ((*image).type_0 as libc::c_uint
                        != it_pic as libc::c_int as libc::c_uint
                        && (*image).type_0 as libc::c_uint
                            != it_sky as libc::c_int as libc::c_uint) as libc::c_int
                        as qboolean,
                    ((*image).type_0 as libc::c_uint
                        == it_sky as libc::c_int as libc::c_uint) as libc::c_int
                        as qboolean,
                );
            } else {
                (*image)
                    .has_alpha = GL_Upload32(
                    pic as *mut libc::c_uint,
                    width,
                    height,
                    ((*image).type_0 as libc::c_uint
                        != it_pic as libc::c_int as libc::c_uint
                        && (*image).type_0 as libc::c_uint
                            != it_sky as libc::c_int as libc::c_uint) as libc::c_int
                        as qboolean,
                );
            }
            (*image).upload_width = upload_width;
            (*image).upload_height = upload_height;
            (*image).paletted = uploaded_paletted;
            (*image).sl = 0 as libc::c_int as libc::c_float;
            (*image).sh = 1 as libc::c_int as libc::c_float;
            (*image).tl = 0 as libc::c_int as libc::c_float;
            (*image).th = 1 as libc::c_int as libc::c_float;
        }
        _ => {}
    }
    return image;
}
#[no_mangle]
pub unsafe extern "C" fn GL_LoadWal(mut name: *mut libc::c_char) -> *mut image_t {
    let mut mt: *mut miptex_t = 0 as *mut miptex_t;
    let mut width: libc::c_int = 0;
    let mut height: libc::c_int = 0;
    let mut ofs: libc::c_int = 0;
    let mut image: *mut image_t = 0 as *mut image_t;
    (ri.FS_LoadFile)
        .expect(
            "non-null function pointer",
        )(name, &mut mt as *mut *mut miptex_t as *mut *mut libc::c_void);
    if mt.is_null() {
        (ri.Con_Printf)
            .expect(
                "non-null function pointer",
            )(
            0 as libc::c_int,
            b"GL_FindImage: can't load %s\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            name,
        );
        return r_notexture;
    }
    width = LittleLong((*mt).width as libc::c_int);
    height = LittleLong((*mt).height as libc::c_int);
    ofs = LittleLong((*mt).offsets[0 as libc::c_int as usize] as libc::c_int);
    image = GL_LoadPic(
        name,
        (mt as *mut byte).offset(ofs as isize),
        width,
        height,
        it_wall,
        8 as libc::c_int,
    );
    (ri.FS_FreeFile).expect("non-null function pointer")(mt as *mut libc::c_void);
    return image;
}
#[no_mangle]
pub unsafe extern "C" fn GL_FindImage(
    mut name: *mut libc::c_char,
    mut type_0: imagetype_t,
) -> *mut image_t {
    let mut image: *mut image_t = 0 as *mut image_t;
    let mut i: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut pic: *mut byte = 0 as *mut byte;
    let mut palette: *mut byte = 0 as *mut byte;
    let mut width: libc::c_int = 0;
    let mut height: libc::c_int = 0;
    if name.is_null() {
        return 0 as *mut image_t;
    }
    len = strlen(name) as libc::c_int;
    if len < 5 as libc::c_int {
        return 0 as *mut image_t;
    }
    i = 0 as libc::c_int;
    image = gltextures.as_mut_ptr();
    while i < numgltextures {
        if strcmp(name, ((*image).name).as_mut_ptr()) == 0 {
            (*image).registration_sequence = registration_sequence;
            return image;
        }
        i += 1;
        image = image.offset(1);
    }
    pic = 0 as *mut byte;
    palette = 0 as *mut byte;
    if strcmp(
        name.offset(len as isize).offset(-(4 as libc::c_int as isize)),
        b".pcx\0" as *const u8 as *const libc::c_char,
    ) == 0
    {
        LoadPCX(name, &mut pic, &mut palette, &mut width, &mut height);
        if pic.is_null() {
            return 0 as *mut image_t;
        }
        image = GL_LoadPic(name, pic, width, height, type_0, 8 as libc::c_int);
    } else if strcmp(
        name.offset(len as isize).offset(-(4 as libc::c_int as isize)),
        b".wal\0" as *const u8 as *const libc::c_char,
    ) == 0
    {
        image = GL_LoadWal(name);
    } else if strcmp(
        name.offset(len as isize).offset(-(4 as libc::c_int as isize)),
        b".tga\0" as *const u8 as *const libc::c_char,
    ) == 0
    {
        LoadTGA(name, &mut pic, &mut width, &mut height);
        if pic.is_null() {
            return 0 as *mut image_t;
        }
        image = GL_LoadPic(name, pic, width, height, type_0, 32 as libc::c_int);
    } else {
        return 0 as *mut image_t
    }
    if !pic.is_null() {
        free(pic as *mut libc::c_void);
    }
    if !palette.is_null() {
        free(palette as *mut libc::c_void);
    }
    return image;
}
#[no_mangle]
pub unsafe extern "C" fn R_RegisterSkin(mut name: *mut libc::c_char) -> *mut image_s {
    return GL_FindImage(name, it_skin);
}
#[no_mangle]
pub unsafe extern "C" fn GL_FreeUnusedImages() {
    let mut i: libc::c_int = 0;
    let mut image: *mut image_t = 0 as *mut image_t;
    (*r_notexture).registration_sequence = registration_sequence;
    (*r_particletexture).registration_sequence = registration_sequence;
    i = 0 as libc::c_int;
    image = gltextures.as_mut_ptr();
    while i < numgltextures {
        if !((*image).registration_sequence == registration_sequence) {
            if !((*image).registration_sequence == 0) {
                if !((*image).type_0 as libc::c_uint
                    == it_pic as libc::c_int as libc::c_uint)
                {
                    qglDeleteTextures
                        .expect(
                            "non-null function pointer",
                        )(1 as libc::c_int, &mut (*image).texnum);
                    memset(
                        image as *mut libc::c_void,
                        0 as libc::c_int,
                        ::std::mem::size_of::<image_t>() as libc::c_ulong,
                    );
                }
            }
        }
        i += 1;
        image = image.offset(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn Draw_GetPalette() -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    let mut g: libc::c_int = 0;
    let mut b: libc::c_int = 0;
    let mut v: libc::c_uint = 0;
    let mut pic: *mut byte = 0 as *mut byte;
    let mut pal: *mut byte = 0 as *mut byte;
    let mut width: libc::c_int = 0;
    let mut height: libc::c_int = 0;
    LoadPCX(
        b"pics/colormap.pcx\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        &mut pic,
        &mut pal,
        &mut width,
        &mut height,
    );
    if pal.is_null() {
        (ri.Sys_Error)
            .expect(
                "non-null function pointer",
            )(
            0 as libc::c_int,
            b"Couldn't load pics/colormap.pcx\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    i = 0 as libc::c_int;
    while i < 256 as libc::c_int {
        r = *pal.offset((i * 3 as libc::c_int + 0 as libc::c_int) as isize)
            as libc::c_int;
        g = *pal.offset((i * 3 as libc::c_int + 1 as libc::c_int) as isize)
            as libc::c_int;
        b = *pal.offset((i * 3 as libc::c_int + 2 as libc::c_int) as isize)
            as libc::c_int;
        v = (((255 as libc::c_int) << 24 as libc::c_int) + (r << 0 as libc::c_int)
            + (g << 8 as libc::c_int) + (b << 16 as libc::c_int)) as libc::c_uint;
        d_8to24table[i as usize] = LittleLong(v as libc::c_int) as libc::c_uint;
        i += 1;
    }
    d_8to24table[255 as libc::c_int as usize]
        &= LittleLong(0xffffff as libc::c_int) as libc::c_uint;
    free(pic as *mut libc::c_void);
    free(pal as *mut libc::c_void);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn GL_InitImages() {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut g: libc::c_float = (*vid_gamma).value;
    registration_sequence = 1 as libc::c_int;
    intensity = (ri.Cvar_Get)
        .expect(
            "non-null function pointer",
        )(
        b"intensity\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    if (*intensity).value <= 1 as libc::c_int as libc::c_float {
        (ri.Cvar_Set)
            .expect(
                "non-null function pointer",
            )(
            b"intensity\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
    gl_state.inverse_intensity = 1 as libc::c_int as libc::c_float / (*intensity).value;
    Draw_GetPalette();
    if qglColorTableEXT.is_some() {
        (ri.FS_LoadFile)
            .expect(
                "non-null function pointer",
            )(
            b"pics/16to8.dat\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            &mut gl_state.d_16to8table as *mut *mut libc::c_uchar
                as *mut *mut libc::c_void,
        );
        if (gl_state.d_16to8table).is_null() {
            (ri.Sys_Error)
                .expect(
                    "non-null function pointer",
                )(
                0 as libc::c_int,
                b"Couldn't load pics/16to8.pcx\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
        }
    }
    if gl_config.renderer & (0x1 as libc::c_int | 0x2 as libc::c_int) != 0 {
        g = 1.0f32;
    }
    i = 0 as libc::c_int;
    while i < 256 as libc::c_int {
        if g == 1 as libc::c_int as libc::c_float {
            gammatable[i as usize] = i as libc::c_uchar;
        } else {
            let mut inf: libc::c_float = 0.;
            inf = (255 as libc::c_int as libc::c_double
                * pow((i as libc::c_double + 0.5f64) / 255.5f64, g as libc::c_double)
                + 0.5f64) as libc::c_float;
            if inf < 0 as libc::c_int as libc::c_float {
                inf = 0 as libc::c_int as libc::c_float;
            }
            if inf > 255 as libc::c_int as libc::c_float {
                inf = 255 as libc::c_int as libc::c_float;
            }
            gammatable[i as usize] = inf as libc::c_uchar;
        }
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < 256 as libc::c_int {
        j = (i as libc::c_float * (*intensity).value) as libc::c_int;
        if j > 255 as libc::c_int {
            j = 255 as libc::c_int;
        }
        intensitytable[i as usize] = j as byte;
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn GL_ShutdownImages() {
    let mut i: libc::c_int = 0;
    let mut image: *mut image_t = 0 as *mut image_t;
    i = 0 as libc::c_int;
    image = gltextures.as_mut_ptr();
    while i < numgltextures {
        if !((*image).registration_sequence == 0) {
            qglDeleteTextures
                .expect(
                    "non-null function pointer",
                )(1 as libc::c_int, &mut (*image).texnum);
            memset(
                image as *mut libc::c_void,
                0 as libc::c_int,
                ::std::mem::size_of::<image_t>() as libc::c_ulong,
            );
        }
        i += 1;
        image = image.offset(1);
    }
}
