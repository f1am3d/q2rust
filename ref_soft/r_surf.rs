#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    static mut vid: viddef_t;
    static mut r_framecount: libc::c_int;
    static mut c_surf: libc::c_int;
    static mut d_roverwrapped: qboolean;
    static mut d_initial_rover: *mut surfcache_t;
    static mut ri: refimport_t;
    static mut r_newrefdef: refdef_t;
    static mut currententity: *mut entity_t;
    static mut sw_surfcacheoverride: *mut cvar_t;
    fn R_BuildLightMap();
    static mut blocklights: [libc::c_uint; 1024];
}
pub type byte = libc::c_uchar;
pub type qboolean = libc::c_uint;
pub const true_0: qboolean = 1;
pub const false_0: qboolean = 0;
pub type vec_t = libc::c_float;
pub type vec3_t = [vec_t; 3];
pub type fixed8_t = libc::c_int;
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
pub struct dmodel_t {
    pub mins: [libc::c_float; 3],
    pub maxs: [libc::c_float; 3],
    pub origin: [libc::c_float; 3],
    pub headnode: libc::c_int,
    pub firstface: libc::c_int,
    pub numfaces: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dvis_t {
    pub numclusters: libc::c_int,
    pub bitofs: [[libc::c_int; 2]; 8],
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct model_s {
    pub name: [libc::c_char; 64],
    pub registration_sequence: libc::c_int,
    pub type_0: modtype_t,
    pub numframes: libc::c_int,
    pub flags: libc::c_int,
    pub mins: vec3_t,
    pub maxs: vec3_t,
    pub clipbox: qboolean,
    pub clipmins: vec3_t,
    pub clipmaxs: vec3_t,
    pub firstmodelsurface: libc::c_int,
    pub nummodelsurfaces: libc::c_int,
    pub numsubmodels: libc::c_int,
    pub submodels: *mut dmodel_t,
    pub numplanes: libc::c_int,
    pub planes: *mut mplane_t,
    pub numleafs: libc::c_int,
    pub leafs: *mut mleaf_t,
    pub numvertexes: libc::c_int,
    pub vertexes: *mut mvertex_t,
    pub numedges: libc::c_int,
    pub edges: *mut medge_t,
    pub numnodes: libc::c_int,
    pub firstnode: libc::c_int,
    pub nodes: *mut mnode_t,
    pub numtexinfo: libc::c_int,
    pub texinfo: *mut mtexinfo_t,
    pub numsurfaces: libc::c_int,
    pub surfaces: *mut msurface_t,
    pub numsurfedges: libc::c_int,
    pub surfedges: *mut libc::c_int,
    pub nummarksurfaces: libc::c_int,
    pub marksurfaces: *mut *mut msurface_t,
    pub vis: *mut dvis_t,
    pub lightdata: *mut byte,
    pub skins: [*mut image_t; 32],
    pub extradata: *mut libc::c_void,
    pub extradatasize: libc::c_int,
}
pub type image_t = image_s;
pub type msurface_t = msurface_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct msurface_s {
    pub visframe: libc::c_int,
    pub dlightframe: libc::c_int,
    pub dlightbits: libc::c_int,
    pub plane: *mut mplane_t,
    pub flags: libc::c_int,
    pub firstedge: libc::c_int,
    pub numedges: libc::c_int,
    pub cachespots: [*mut surfcache_s; 4],
    pub texturemins: [libc::c_short; 2],
    pub extents: [libc::c_short; 2],
    pub texinfo: *mut mtexinfo_t,
    pub styles: [byte; 4],
    pub samples: *mut byte,
    pub nextalphasurface: *mut msurface_s,
}
pub type mtexinfo_t = mtexinfo_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mtexinfo_s {
    pub vecs: [[libc::c_float; 4]; 2],
    pub mipadjust: libc::c_float,
    pub image: *mut image_t,
    pub flags: libc::c_int,
    pub numframes: libc::c_int,
    pub next: *mut mtexinfo_s,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct surfcache_s {
    pub next: *mut surfcache_s,
    pub owner: *mut *mut surfcache_s,
    pub lightadj: [libc::c_int; 4],
    pub dlight: libc::c_int,
    pub size: libc::c_int,
    pub width: libc::c_uint,
    pub height: libc::c_uint,
    pub mipscale: libc::c_float,
    pub image: *mut image_t,
    pub data: [byte; 4],
}
pub type mplane_t = mplane_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mplane_s {
    pub normal: vec3_t,
    pub dist: libc::c_float,
    pub type_0: byte,
    pub signbits: byte,
    pub pad: [byte; 2],
}
pub type mnode_t = mnode_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mnode_s {
    pub contents: libc::c_int,
    pub visframe: libc::c_int,
    pub minmaxs: [libc::c_short; 6],
    pub parent: *mut mnode_s,
    pub plane: *mut mplane_t,
    pub children: [*mut mnode_s; 2],
    pub firstsurface: libc::c_ushort,
    pub numsurfaces: libc::c_ushort,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct medge_t {
    pub v: [libc::c_ushort; 2],
    pub cachededgeoffset: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mvertex_t {
    pub position: vec3_t,
}
pub type mleaf_t = mleaf_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mleaf_s {
    pub contents: libc::c_int,
    pub visframe: libc::c_int,
    pub minmaxs: [libc::c_short; 6],
    pub parent: *mut mnode_s,
    pub cluster: libc::c_int,
    pub area: libc::c_int,
    pub firstmarksurface: *mut *mut msurface_t,
    pub nummarksurfaces: libc::c_int,
    pub key: libc::c_int,
}
pub type modtype_t = libc::c_uint;
pub const mod_alias: modtype_t = 3;
pub const mod_sprite: modtype_t = 2;
pub const mod_brush: modtype_t = 1;
pub const mod_bad: modtype_t = 0;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct drawsurf_t {
    pub surfdat: *mut byte,
    pub rowbytes: libc::c_int,
    pub surf: *mut msurface_t,
    pub lightadj: [fixed8_t; 4],
    pub image: *mut image_t,
    pub surfmip: libc::c_int,
    pub surfwidth: libc::c_int,
    pub surfheight: libc::c_int,
}
pub type surfcache_t = surfcache_s;
#[no_mangle]
pub static mut r_drawsurf: drawsurf_t = drawsurf_t {
    surfdat: 0 as *const byte as *mut byte,
    rowbytes: 0,
    surf: 0 as *const msurface_t as *mut msurface_t,
    lightadj: [0; 4],
    image: 0 as *const image_t as *mut image_t,
    surfmip: 0,
    surfwidth: 0,
    surfheight: 0,
};
#[no_mangle]
pub static mut lightleft: libc::c_int = 0;
#[no_mangle]
pub static mut sourcesstep: libc::c_int = 0;
#[no_mangle]
pub static mut blocksize: libc::c_int = 0;
#[no_mangle]
pub static mut sourcetstep: libc::c_int = 0;
#[no_mangle]
pub static mut lightdelta: libc::c_int = 0;
#[no_mangle]
pub static mut lightdeltastep: libc::c_int = 0;
#[no_mangle]
pub static mut lightright: libc::c_int = 0;
#[no_mangle]
pub static mut lightleftstep: libc::c_int = 0;
#[no_mangle]
pub static mut lightrightstep: libc::c_int = 0;
#[no_mangle]
pub static mut blockdivshift: libc::c_int = 0;
#[no_mangle]
pub static mut blockdivmask: libc::c_uint = 0;
#[no_mangle]
pub static mut prowdestbase: *mut libc::c_void = 0 as *const libc::c_void
    as *mut libc::c_void;
#[no_mangle]
pub static mut pbasesource: *mut libc::c_uchar = 0 as *const libc::c_uchar
    as *mut libc::c_uchar;
#[no_mangle]
pub static mut surfrowbytes: libc::c_int = 0;
#[no_mangle]
pub static mut r_lightptr: *mut libc::c_uint = 0 as *const libc::c_uint
    as *mut libc::c_uint;
#[no_mangle]
pub static mut r_stepback: libc::c_int = 0;
#[no_mangle]
pub static mut r_lightwidth: libc::c_int = 0;
#[no_mangle]
pub static mut r_numhblocks: libc::c_int = 0;
#[no_mangle]
pub static mut r_numvblocks: libc::c_int = 0;
#[no_mangle]
pub static mut r_source: *mut libc::c_uchar = 0 as *const libc::c_uchar
    as *mut libc::c_uchar;
#[no_mangle]
pub static mut r_sourcemax: *mut libc::c_uchar = 0 as *const libc::c_uchar
    as *mut libc::c_uchar;
static mut surfmiptable: [Option::<unsafe extern "C" fn() -> ()>; 4] = unsafe {
    [
        Some(R_DrawSurfaceBlock8_mip0 as unsafe extern "C" fn() -> ()),
        Some(R_DrawSurfaceBlock8_mip1 as unsafe extern "C" fn() -> ()),
        Some(R_DrawSurfaceBlock8_mip2 as unsafe extern "C" fn() -> ()),
        Some(R_DrawSurfaceBlock8_mip3 as unsafe extern "C" fn() -> ()),
    ]
};
#[no_mangle]
pub static mut surfscale: libc::c_float = 0.;
#[no_mangle]
pub static mut r_cache_thrash: qboolean = false_0;
#[no_mangle]
pub static mut sc_size: libc::c_int = 0;
#[no_mangle]
pub static mut sc_rover: *mut surfcache_t = 0 as *const surfcache_t as *mut surfcache_t;
#[no_mangle]
pub static mut sc_base: *mut surfcache_t = 0 as *const surfcache_t as *mut surfcache_t;
#[no_mangle]
pub unsafe extern "C" fn R_TextureAnimation(mut tex: *mut mtexinfo_t) -> *mut image_t {
    let mut c: libc::c_int = 0;
    if ((*tex).next).is_null() {
        return (*tex).image;
    }
    c = (*currententity).frame % (*tex).numframes;
    while c != 0 {
        tex = (*tex).next;
        c -= 1;
    }
    return (*tex).image;
}
#[no_mangle]
pub unsafe extern "C" fn R_DrawSurface() {
    let mut basetptr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut smax: libc::c_int = 0;
    let mut tmax: libc::c_int = 0;
    let mut twidth: libc::c_int = 0;
    let mut u: libc::c_int = 0;
    let mut soffset: libc::c_int = 0;
    let mut basetoffset: libc::c_int = 0;
    let mut texwidth: libc::c_int = 0;
    let mut horzblockstep: libc::c_int = 0;
    let mut pcolumndest: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut pblockdrawer: Option::<unsafe extern "C" fn() -> ()> = None;
    let mut mt: *mut image_t = 0 as *mut image_t;
    surfrowbytes = r_drawsurf.rowbytes;
    mt = r_drawsurf.image;
    r_source = (*mt).pixels[r_drawsurf.surfmip as usize];
    texwidth = (*mt).width >> r_drawsurf.surfmip;
    blocksize = 16 as libc::c_int >> r_drawsurf.surfmip;
    blockdivshift = 4 as libc::c_int - r_drawsurf.surfmip;
    blockdivmask = (((1 as libc::c_int) << blockdivshift) - 1 as libc::c_int)
        as libc::c_uint;
    r_lightwidth = ((*r_drawsurf.surf).extents[0 as libc::c_int as usize] as libc::c_int
        >> 4 as libc::c_int) + 1 as libc::c_int;
    r_numhblocks = r_drawsurf.surfwidth >> blockdivshift;
    r_numvblocks = r_drawsurf.surfheight >> blockdivshift;
    pblockdrawer = surfmiptable[r_drawsurf.surfmip as usize];
    horzblockstep = blocksize;
    smax = (*mt).width >> r_drawsurf.surfmip;
    twidth = texwidth;
    tmax = (*mt).height >> r_drawsurf.surfmip;
    sourcetstep = texwidth;
    r_stepback = tmax * twidth;
    r_sourcemax = r_source.offset((tmax * smax) as isize);
    soffset = (*r_drawsurf.surf).texturemins[0 as libc::c_int as usize] as libc::c_int;
    basetoffset = (*r_drawsurf.surf).texturemins[1 as libc::c_int as usize]
        as libc::c_int;
    soffset = ((soffset >> r_drawsurf.surfmip) + (smax << 16 as libc::c_int)) % smax;
    basetptr = &mut *r_source
        .offset(
            (((basetoffset >> r_drawsurf.surfmip) + (tmax << 16 as libc::c_int)) % tmax
                * twidth) as isize,
        ) as *mut libc::c_uchar;
    pcolumndest = r_drawsurf.surfdat;
    u = 0 as libc::c_int;
    while u < r_numhblocks {
        r_lightptr = blocklights.as_mut_ptr().offset(u as isize);
        prowdestbase = pcolumndest as *mut libc::c_void;
        pbasesource = basetptr.offset(soffset as isize);
        (Some(pblockdrawer.expect("non-null function pointer")))
            .expect("non-null function pointer")();
        soffset = soffset + blocksize;
        if soffset >= smax {
            soffset = 0 as libc::c_int;
        }
        pcolumndest = pcolumndest.offset(horzblockstep as isize);
        u += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn R_DrawSurfaceBlock8_mip0() {
    let mut v: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut b: libc::c_int = 0;
    let mut lightstep: libc::c_int = 0;
    let mut lighttemp: libc::c_int = 0;
    let mut light: libc::c_int = 0;
    let mut pix: libc::c_uchar = 0;
    let mut psource: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut prowdest: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    psource = pbasesource;
    prowdest = prowdestbase as *mut libc::c_uchar;
    v = 0 as libc::c_int;
    while v < r_numvblocks {
        lightleft = *r_lightptr.offset(0 as libc::c_int as isize) as libc::c_int;
        lightright = *r_lightptr.offset(1 as libc::c_int as isize) as libc::c_int;
        r_lightptr = r_lightptr.offset(r_lightwidth as isize);
        lightleftstep = ((*r_lightptr.offset(0 as libc::c_int as isize))
            .wrapping_sub(lightleft as libc::c_uint) >> 4 as libc::c_int) as libc::c_int;
        lightrightstep = ((*r_lightptr.offset(1 as libc::c_int as isize))
            .wrapping_sub(lightright as libc::c_uint) >> 4 as libc::c_int)
            as libc::c_int;
        i = 0 as libc::c_int;
        while i < 16 as libc::c_int {
            lighttemp = lightleft - lightright;
            lightstep = lighttemp >> 4 as libc::c_int;
            light = lightright;
            b = 15 as libc::c_int;
            while b >= 0 as libc::c_int {
                pix = *psource.offset(b as isize);
                *prowdest
                    .offset(
                        b as isize,
                    ) = *(vid.colormap as *mut libc::c_uchar)
                    .offset(
                        ((light & 0xff00 as libc::c_int) + pix as libc::c_int) as isize,
                    );
                light += lightstep;
                b -= 1;
            }
            psource = psource.offset(sourcetstep as isize);
            lightright += lightrightstep;
            lightleft += lightleftstep;
            prowdest = prowdest.offset(surfrowbytes as isize);
            i += 1;
        }
        if psource >= r_sourcemax {
            psource = psource.offset(-(r_stepback as isize));
        }
        v += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn R_DrawSurfaceBlock8_mip1() {
    let mut v: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut b: libc::c_int = 0;
    let mut lightstep: libc::c_int = 0;
    let mut lighttemp: libc::c_int = 0;
    let mut light: libc::c_int = 0;
    let mut pix: libc::c_uchar = 0;
    let mut psource: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut prowdest: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    psource = pbasesource;
    prowdest = prowdestbase as *mut libc::c_uchar;
    v = 0 as libc::c_int;
    while v < r_numvblocks {
        lightleft = *r_lightptr.offset(0 as libc::c_int as isize) as libc::c_int;
        lightright = *r_lightptr.offset(1 as libc::c_int as isize) as libc::c_int;
        r_lightptr = r_lightptr.offset(r_lightwidth as isize);
        lightleftstep = ((*r_lightptr.offset(0 as libc::c_int as isize))
            .wrapping_sub(lightleft as libc::c_uint) >> 3 as libc::c_int) as libc::c_int;
        lightrightstep = ((*r_lightptr.offset(1 as libc::c_int as isize))
            .wrapping_sub(lightright as libc::c_uint) >> 3 as libc::c_int)
            as libc::c_int;
        i = 0 as libc::c_int;
        while i < 8 as libc::c_int {
            lighttemp = lightleft - lightright;
            lightstep = lighttemp >> 3 as libc::c_int;
            light = lightright;
            b = 7 as libc::c_int;
            while b >= 0 as libc::c_int {
                pix = *psource.offset(b as isize);
                *prowdest
                    .offset(
                        b as isize,
                    ) = *(vid.colormap as *mut libc::c_uchar)
                    .offset(
                        ((light & 0xff00 as libc::c_int) + pix as libc::c_int) as isize,
                    );
                light += lightstep;
                b -= 1;
            }
            psource = psource.offset(sourcetstep as isize);
            lightright += lightrightstep;
            lightleft += lightleftstep;
            prowdest = prowdest.offset(surfrowbytes as isize);
            i += 1;
        }
        if psource >= r_sourcemax {
            psource = psource.offset(-(r_stepback as isize));
        }
        v += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn R_DrawSurfaceBlock8_mip2() {
    let mut v: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut b: libc::c_int = 0;
    let mut lightstep: libc::c_int = 0;
    let mut lighttemp: libc::c_int = 0;
    let mut light: libc::c_int = 0;
    let mut pix: libc::c_uchar = 0;
    let mut psource: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut prowdest: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    psource = pbasesource;
    prowdest = prowdestbase as *mut libc::c_uchar;
    v = 0 as libc::c_int;
    while v < r_numvblocks {
        lightleft = *r_lightptr.offset(0 as libc::c_int as isize) as libc::c_int;
        lightright = *r_lightptr.offset(1 as libc::c_int as isize) as libc::c_int;
        r_lightptr = r_lightptr.offset(r_lightwidth as isize);
        lightleftstep = ((*r_lightptr.offset(0 as libc::c_int as isize))
            .wrapping_sub(lightleft as libc::c_uint) >> 2 as libc::c_int) as libc::c_int;
        lightrightstep = ((*r_lightptr.offset(1 as libc::c_int as isize))
            .wrapping_sub(lightright as libc::c_uint) >> 2 as libc::c_int)
            as libc::c_int;
        i = 0 as libc::c_int;
        while i < 4 as libc::c_int {
            lighttemp = lightleft - lightright;
            lightstep = lighttemp >> 2 as libc::c_int;
            light = lightright;
            b = 3 as libc::c_int;
            while b >= 0 as libc::c_int {
                pix = *psource.offset(b as isize);
                *prowdest
                    .offset(
                        b as isize,
                    ) = *(vid.colormap as *mut libc::c_uchar)
                    .offset(
                        ((light & 0xff00 as libc::c_int) + pix as libc::c_int) as isize,
                    );
                light += lightstep;
                b -= 1;
            }
            psource = psource.offset(sourcetstep as isize);
            lightright += lightrightstep;
            lightleft += lightleftstep;
            prowdest = prowdest.offset(surfrowbytes as isize);
            i += 1;
        }
        if psource >= r_sourcemax {
            psource = psource.offset(-(r_stepback as isize));
        }
        v += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn R_DrawSurfaceBlock8_mip3() {
    let mut v: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut b: libc::c_int = 0;
    let mut lightstep: libc::c_int = 0;
    let mut lighttemp: libc::c_int = 0;
    let mut light: libc::c_int = 0;
    let mut pix: libc::c_uchar = 0;
    let mut psource: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut prowdest: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    psource = pbasesource;
    prowdest = prowdestbase as *mut libc::c_uchar;
    v = 0 as libc::c_int;
    while v < r_numvblocks {
        lightleft = *r_lightptr.offset(0 as libc::c_int as isize) as libc::c_int;
        lightright = *r_lightptr.offset(1 as libc::c_int as isize) as libc::c_int;
        r_lightptr = r_lightptr.offset(r_lightwidth as isize);
        lightleftstep = ((*r_lightptr.offset(0 as libc::c_int as isize))
            .wrapping_sub(lightleft as libc::c_uint) >> 1 as libc::c_int) as libc::c_int;
        lightrightstep = ((*r_lightptr.offset(1 as libc::c_int as isize))
            .wrapping_sub(lightright as libc::c_uint) >> 1 as libc::c_int)
            as libc::c_int;
        i = 0 as libc::c_int;
        while i < 2 as libc::c_int {
            lighttemp = lightleft - lightright;
            lightstep = lighttemp >> 1 as libc::c_int;
            light = lightright;
            b = 1 as libc::c_int;
            while b >= 0 as libc::c_int {
                pix = *psource.offset(b as isize);
                *prowdest
                    .offset(
                        b as isize,
                    ) = *(vid.colormap as *mut libc::c_uchar)
                    .offset(
                        ((light & 0xff00 as libc::c_int) + pix as libc::c_int) as isize,
                    );
                light += lightstep;
                b -= 1;
            }
            psource = psource.offset(sourcetstep as isize);
            lightright += lightrightstep;
            lightleft += lightleftstep;
            prowdest = prowdest.offset(surfrowbytes as isize);
            i += 1;
        }
        if psource >= r_sourcemax {
            psource = psource.offset(-(r_stepback as isize));
        }
        v += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn R_InitCaches() {
    let mut size: libc::c_int = 0;
    let mut pix: libc::c_int = 0;
    if (*sw_surfcacheoverride).value != 0. {
        size = (*sw_surfcacheoverride).value as libc::c_int;
    } else {
        size = 1024 as libc::c_int * 768 as libc::c_int;
        pix = vid.width * vid.height;
        if pix > 64000 as libc::c_int {
            size += (pix - 64000 as libc::c_int) * 3 as libc::c_int;
        }
    }
    size = size + 8191 as libc::c_int & !(8191 as libc::c_int);
    (ri.Con_Printf)
        .expect(
            "non-null function pointer",
        )(
        0 as libc::c_int,
        b"%ik surface cache\n\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        size / 1024 as libc::c_int,
    );
    sc_size = size;
    sc_base = malloc(size as libc::c_ulong) as *mut surfcache_t;
    sc_rover = sc_base;
    let ref mut fresh0 = (*sc_base).next;
    *fresh0 = 0 as *mut surfcache_s;
    let ref mut fresh1 = (*sc_base).owner;
    *fresh1 = 0 as *mut *mut surfcache_s;
    (*sc_base).size = sc_size;
}
#[no_mangle]
pub unsafe extern "C" fn D_FlushCaches() {
    let mut c: *mut surfcache_t = 0 as *mut surfcache_t;
    if sc_base.is_null() {
        return;
    }
    c = sc_base;
    while !c.is_null() {
        if !((*c).owner).is_null() {
            let ref mut fresh2 = *(*c).owner;
            *fresh2 = 0 as *mut surfcache_s;
        }
        c = (*c).next;
    }
    sc_rover = sc_base;
    let ref mut fresh3 = (*sc_base).next;
    *fresh3 = 0 as *mut surfcache_s;
    let ref mut fresh4 = (*sc_base).owner;
    *fresh4 = 0 as *mut *mut surfcache_s;
    (*sc_base).size = sc_size;
}
#[no_mangle]
pub unsafe extern "C" fn D_SCAlloc(
    mut width: libc::c_int,
    mut size: libc::c_int,
) -> *mut surfcache_t {
    let mut new: *mut surfcache_t = 0 as *mut surfcache_t;
    let mut wrapped_this_time: qboolean = false_0;
    if width < 0 as libc::c_int || width > 256 as libc::c_int {
        (ri.Sys_Error)
            .expect(
                "non-null function pointer",
            )(
            0 as libc::c_int,
            b"D_SCAlloc: bad cache width %d\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            width,
        );
    }
    if size <= 0 as libc::c_int || size > 0x10000 as libc::c_int {
        (ri.Sys_Error)
            .expect(
                "non-null function pointer",
            )(
            0 as libc::c_int,
            b"D_SCAlloc: bad cache size %d\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            size,
        );
    }
    size = &mut *((*(0 as *mut surfcache_t)).data).as_mut_ptr().offset(size as isize)
        as *mut byte as libc::c_int;
    size = size + 3 as libc::c_int & !(3 as libc::c_int);
    if size > sc_size {
        (ri.Sys_Error)
            .expect(
                "non-null function pointer",
            )(
            0 as libc::c_int,
            b"D_SCAlloc: %i > cache size of %i\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            size,
            sc_size,
        );
    }
    wrapped_this_time = false_0;
    if sc_rover.is_null()
        || (sc_rover as *mut byte).offset_from(sc_base as *mut byte) as libc::c_long
            > (sc_size - size) as libc::c_long
    {
        if !sc_rover.is_null() {
            wrapped_this_time = true_0;
        }
        sc_rover = sc_base;
    }
    new = sc_rover;
    if !((*sc_rover).owner).is_null() {
        let ref mut fresh5 = *(*sc_rover).owner;
        *fresh5 = 0 as *mut surfcache_s;
    }
    while (*new).size < size {
        sc_rover = (*sc_rover).next;
        if sc_rover.is_null() {
            (ri.Sys_Error)
                .expect(
                    "non-null function pointer",
                )(
                0 as libc::c_int,
                b"D_SCAlloc: hit the end of memory\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
        }
        if !((*sc_rover).owner).is_null() {
            let ref mut fresh6 = *(*sc_rover).owner;
            *fresh6 = 0 as *mut surfcache_s;
        }
        (*new).size += (*sc_rover).size;
        let ref mut fresh7 = (*new).next;
        *fresh7 = (*sc_rover).next;
    }
    if (*new).size - size > 256 as libc::c_int {
        sc_rover = (new as *mut byte).offset(size as isize) as *mut surfcache_t;
        (*sc_rover).size = (*new).size - size;
        let ref mut fresh8 = (*sc_rover).next;
        *fresh8 = (*new).next;
        (*sc_rover).width = 0 as libc::c_int as libc::c_uint;
        let ref mut fresh9 = (*sc_rover).owner;
        *fresh9 = 0 as *mut *mut surfcache_s;
        let ref mut fresh10 = (*new).next;
        *fresh10 = sc_rover;
        (*new).size = size;
    } else {
        sc_rover = (*new).next;
    }
    (*new).width = width as libc::c_uint;
    if width > 0 as libc::c_int {
        (*new)
            .height = (size as libc::c_ulong)
            .wrapping_sub(::std::mem::size_of::<surfcache_t>() as libc::c_ulong)
            .wrapping_add(::std::mem::size_of::<[byte; 4]>() as libc::c_ulong)
            .wrapping_div(width as libc::c_ulong) as libc::c_uint;
    }
    let ref mut fresh11 = (*new).owner;
    *fresh11 = 0 as *mut *mut surfcache_s;
    if d_roverwrapped as u64 != 0 {
        if wrapped_this_time as libc::c_uint != 0 || sc_rover >= d_initial_rover {
            r_cache_thrash = true_0;
        }
    } else if wrapped_this_time as u64 != 0 {
        d_roverwrapped = true_0;
    }
    return new;
}
#[no_mangle]
pub unsafe extern "C" fn D_SCDump() {
    let mut test: *mut surfcache_t = 0 as *mut surfcache_t;
    test = sc_base;
    while !test.is_null() {
        if test == sc_rover {
            (ri.Con_Printf)
                .expect(
                    "non-null function pointer",
                )(
                0 as libc::c_int,
                b"ROVER:\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
        }
        (ri.Con_Printf)
            .expect(
                "non-null function pointer",
            )(
            0 as libc::c_int,
            b"%p : %i bytes     %i width\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            test,
            (*test).size,
            (*test).width,
        );
        test = (*test).next;
    }
}
#[no_mangle]
pub unsafe extern "C" fn MaskForNum(mut num: libc::c_int) -> libc::c_int {
    if num == 128 as libc::c_int {
        return 127 as libc::c_int;
    }
    if num == 64 as libc::c_int {
        return 63 as libc::c_int;
    }
    if num == 32 as libc::c_int {
        return 31 as libc::c_int;
    }
    if num == 16 as libc::c_int {
        return 15 as libc::c_int;
    }
    return 255 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn D_log2(mut num: libc::c_int) -> libc::c_int {
    let mut c: libc::c_int = 0;
    c = 0 as libc::c_int;
    loop {
        num >>= 1 as libc::c_int;
        if !(num != 0) {
            break;
        }
        c += 1;
    }
    return c;
}
#[no_mangle]
pub unsafe extern "C" fn D_CacheSurface(
    mut surface: *mut msurface_t,
    mut miplevel: libc::c_int,
) -> *mut surfcache_t {
    let mut cache: *mut surfcache_t = 0 as *mut surfcache_t;
    r_drawsurf.image = R_TextureAnimation((*surface).texinfo);
    r_drawsurf
        .lightadj[0 as libc::c_int
        as usize] = ((*(r_newrefdef.lightstyles)
        .offset((*surface).styles[0 as libc::c_int as usize] as isize))
        .white * 128 as libc::c_int as libc::c_float) as fixed8_t;
    r_drawsurf
        .lightadj[1 as libc::c_int
        as usize] = ((*(r_newrefdef.lightstyles)
        .offset((*surface).styles[1 as libc::c_int as usize] as isize))
        .white * 128 as libc::c_int as libc::c_float) as fixed8_t;
    r_drawsurf
        .lightadj[2 as libc::c_int
        as usize] = ((*(r_newrefdef.lightstyles)
        .offset((*surface).styles[2 as libc::c_int as usize] as isize))
        .white * 128 as libc::c_int as libc::c_float) as fixed8_t;
    r_drawsurf
        .lightadj[3 as libc::c_int
        as usize] = ((*(r_newrefdef.lightstyles)
        .offset((*surface).styles[3 as libc::c_int as usize] as isize))
        .white * 128 as libc::c_int as libc::c_float) as fixed8_t;
    cache = (*surface).cachespots[miplevel as usize];
    if !cache.is_null() && (*cache).dlight == 0 && (*surface).dlightframe != r_framecount
        && (*cache).image == r_drawsurf.image
        && (*cache).lightadj[0 as libc::c_int as usize]
            == r_drawsurf.lightadj[0 as libc::c_int as usize]
        && (*cache).lightadj[1 as libc::c_int as usize]
            == r_drawsurf.lightadj[1 as libc::c_int as usize]
        && (*cache).lightadj[2 as libc::c_int as usize]
            == r_drawsurf.lightadj[2 as libc::c_int as usize]
        && (*cache).lightadj[3 as libc::c_int as usize]
            == r_drawsurf.lightadj[3 as libc::c_int as usize]
    {
        return cache;
    }
    surfscale = (1.0f64 / ((1 as libc::c_int) << miplevel) as libc::c_double)
        as libc::c_float;
    r_drawsurf.surfmip = miplevel;
    r_drawsurf
        .surfwidth = (*surface).extents[0 as libc::c_int as usize] as libc::c_int
        >> miplevel;
    r_drawsurf.rowbytes = r_drawsurf.surfwidth;
    r_drawsurf
        .surfheight = (*surface).extents[1 as libc::c_int as usize] as libc::c_int
        >> miplevel;
    if cache.is_null() {
        cache = D_SCAlloc(
            r_drawsurf.surfwidth,
            r_drawsurf.surfwidth * r_drawsurf.surfheight,
        );
        let ref mut fresh12 = (*surface).cachespots[miplevel as usize];
        *fresh12 = cache;
        let ref mut fresh13 = (*cache).owner;
        *fresh13 = &mut *((*surface).cachespots).as_mut_ptr().offset(miplevel as isize)
            as *mut *mut surfcache_s;
        (*cache).mipscale = surfscale;
    }
    if (*surface).dlightframe == r_framecount {
        (*cache).dlight = 1 as libc::c_int;
    } else {
        (*cache).dlight = 0 as libc::c_int;
    }
    r_drawsurf.surfdat = ((*cache).data).as_mut_ptr() as *mut pixel_t;
    let ref mut fresh14 = (*cache).image;
    *fresh14 = r_drawsurf.image;
    (*cache)
        .lightadj[0 as libc::c_int
        as usize] = r_drawsurf.lightadj[0 as libc::c_int as usize];
    (*cache)
        .lightadj[1 as libc::c_int
        as usize] = r_drawsurf.lightadj[1 as libc::c_int as usize];
    (*cache)
        .lightadj[2 as libc::c_int
        as usize] = r_drawsurf.lightadj[2 as libc::c_int as usize];
    (*cache)
        .lightadj[3 as libc::c_int
        as usize] = r_drawsurf.lightadj[3 as libc::c_int as usize];
    r_drawsurf.surf = surface;
    c_surf += 1;
    R_BuildLightMap();
    R_DrawSurface();
    return cache;
}
