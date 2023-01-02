#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(extern_types, register_tool)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn tan(_: libc::c_double) -> libc::c_double;
    fn fwrite(
        _: *const libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn VectorNormalize(v: *mut vec_t) -> vec_t;
    fn AngleVectors(
        angles: *mut vec_t,
        forward: *mut vec_t,
        right: *mut vec_t,
        up: *mut vec_t,
    );
    fn Com_sprintf(
        dest: *mut libc::c_char,
        size: libc::c_int,
        fmt: *mut libc::c_char,
        _: ...
    );
    fn LittleShort(l: libc::c_short) -> libc::c_short;
    fn Sys_Milliseconds() -> libc::c_int;
    fn Sys_Mkdir(path: *mut libc::c_char);
    static mut vid: viddef_t;
    static mut r_refdef: oldrefdef_t;
    fn Mod_PointInLeaf(p: *mut libc::c_float, model: *mut model_t) -> *mut mleaf_t;
    static mut d_spanpixcount: libc::c_int;
    static mut r_framecount: libc::c_int;
    static mut r_aliasuvscale: libc::c_float;
    static mut r_dowarp: qboolean;
    static mut sw_clearcolor: *mut cvar_t;
    static mut r_newrefdef: refdef_t;
    fn Draw_Fill(
        x: libc::c_int,
        y: libc::c_int,
        w: libc::c_int,
        h: libc::c_int,
        c: libc::c_int,
    );
    static mut d_pzbuffer: *mut libc::c_short;
    static mut d_zwidth: libc::c_uint;
    static mut r_screenwidth: libc::c_int;
    static mut sw_state: swstate_t;
    static mut d_zrowbytes: libc::c_uint;
    static mut yscale: libc::c_float;
    static mut scale_for_mip: libc::c_float;
    static mut xscale: libc::c_float;
    static mut c_surf: libc::c_int;
    static mut r_warpbuffer: [byte; 76800];
    static mut sc_rover: *mut surfcache_t;
    static mut d_viewbuffer: *mut pixel_t;
    static mut r_drawnpolycount: libc::c_int;
    static mut vup: vec3_t;
    static mut base_vup: vec3_t;
    static mut vpn: vec3_t;
    static mut base_vpn: vec3_t;
    static mut vright: vec3_t;
    static mut base_vright: vec3_t;
    static mut xcenter: libc::c_float;
    static mut ycenter: libc::c_float;
    static mut xscaleinv: libc::c_float;
    static mut yscaleinv: libc::c_float;
    static mut xscaleshrink: libc::c_float;
    static mut yscaleshrink: libc::c_float;
    static mut sw_waterwarp: *mut cvar_t;
    static mut r_fullbright: *mut cvar_t;
    static mut view_clipplanes: [clipplane_t; 4];
    static mut pfrustum_indexes: [*mut libc::c_int; 4];
    static mut screenedge: [mplane_t; 4];
    static mut r_origin: vec3_t;
    static mut modelorg: vec3_t;
    static mut verticalFieldOfView: libc::c_float;
    static mut xOrigin: libc::c_float;
    static mut yOrigin: libc::c_float;
    static mut c_faceclip: libc::c_int;
    static mut r_polycount: libc::c_int;
    static mut r_wholepolycount: libc::c_int;
    static mut r_amodels_drawn: libc::c_int;
    static mut aliasxscale: libc::c_float;
    static mut aliasyscale: libc::c_float;
    static mut aliasxcenter: libc::c_float;
    static mut aliasycenter: libc::c_float;
    static mut r_outofsurfaces: libc::c_int;
    static mut r_outofedges: libc::c_int;
    static mut r_time1: libc::c_float;
    static mut da_time1: libc::c_float;
    static mut da_time2: libc::c_float;
    static mut dp_time1: libc::c_float;
    static mut dp_time2: libc::c_float;
    static mut db_time1: libc::c_float;
    static mut db_time2: libc::c_float;
    static mut rw_time1: libc::c_float;
    static mut rw_time2: libc::c_float;
    static mut se_time1: libc::c_float;
    static mut se_time2: libc::c_float;
    static mut de_time1: libc::c_float;
    static mut de_time2: libc::c_float;
    static mut r_frustum_indexes: [libc::c_int; 24];
    static mut r_viewleaf: *mut mleaf_t;
    static mut r_viewcluster: libc::c_int;
    static mut r_worldmodel: *mut model_t;
    static mut ri: refimport_t;
    fn D_FlushCaches();
    static mut d_aflatcolor: libc::c_int;
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
pub struct vrect_s {
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub pnext: *mut vrect_s,
}
pub type vrect_t = vrect_s;
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
pub struct oldrefdef_t {
    pub vrect: vrect_t,
    pub aliasvrect: vrect_t,
    pub vrectright: libc::c_int,
    pub vrectbottom: libc::c_int,
    pub aliasvrectright: libc::c_int,
    pub aliasvrectbottom: libc::c_int,
    pub vrectrightedge: libc::c_float,
    pub fvrectx: libc::c_float,
    pub fvrecty: libc::c_float,
    pub fvrectx_adj: libc::c_float,
    pub fvrecty_adj: libc::c_float,
    pub vrect_x_adj_shift20: libc::c_int,
    pub vrectright_adj_shift20: libc::c_int,
    pub fvrectright_adj: libc::c_float,
    pub fvrectbottom_adj: libc::c_float,
    pub fvrectright: libc::c_float,
    pub fvrectbottom: libc::c_float,
    pub horizontalFieldOfView: libc::c_float,
    pub xOrigin: libc::c_float,
    pub yOrigin: libc::c_float,
    pub vieworg: vec3_t,
    pub viewangles: vec3_t,
    pub ambientlight: libc::c_int,
}
pub type model_t = model_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct clipplane_s {
    pub normal: vec3_t,
    pub dist: libc::c_float,
    pub next: *mut clipplane_s,
    pub leftedge: byte,
    pub rightedge: byte,
    pub reserved: [byte; 2],
}
pub type clipplane_t = clipplane_s;
pub type surfcache_t = surfcache_s;
pub type swstate_t = swstate_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct swstate_s {
    pub fullscreen: qboolean,
    pub prev_mode: libc::c_int,
    pub gammatable: [byte; 256],
    pub currentpalette: [byte; 1024],
}
#[no_mangle]
pub static mut sw_mipcap: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut sw_mipscale: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut d_initial_rover: *mut surfcache_t = 0 as *const surfcache_t
    as *mut surfcache_t;
#[no_mangle]
pub static mut d_roverwrapped: qboolean = false_0;
#[no_mangle]
pub static mut d_minmip: libc::c_int = 0;
#[no_mangle]
pub static mut d_scalemip: [libc::c_float; 3] = [0.; 3];
static mut basemip: [libc::c_float; 3] = [
    1.0f64 as libc::c_float,
    (0.5f64 * 0.8f64) as libc::c_float,
    (0.25f64 * 0.8f64) as libc::c_float,
];
#[no_mangle]
pub static mut d_vrectx: libc::c_int = 0;
#[no_mangle]
pub static mut d_vrecty: libc::c_int = 0;
#[no_mangle]
pub static mut d_vrectright_particle: libc::c_int = 0;
#[no_mangle]
pub static mut d_vrectbottom_particle: libc::c_int = 0;
#[no_mangle]
pub static mut d_pix_min: libc::c_int = 0;
#[no_mangle]
pub static mut d_pix_max: libc::c_int = 0;
#[no_mangle]
pub static mut d_pix_shift: libc::c_int = 0;
#[no_mangle]
pub static mut d_scantable: [libc::c_int; 1200] = [0; 1200];
#[no_mangle]
pub static mut zspantable: [*mut libc::c_short; 1200] = [0 as *const libc::c_short
    as *mut libc::c_short; 1200];
#[no_mangle]
pub unsafe extern "C" fn D_Patch() {}
#[no_mangle]
pub static mut alias_colormap: *mut libc::c_uchar = 0 as *const libc::c_uchar
    as *mut libc::c_uchar;
#[no_mangle]
pub unsafe extern "C" fn D_ViewChanged() {
    let mut i: libc::c_int = 0;
    scale_for_mip = xscale;
    if yscale > xscale {
        scale_for_mip = yscale;
    }
    d_zrowbytes = (vid.width * 2 as libc::c_int) as libc::c_uint;
    d_zwidth = vid.width as libc::c_uint;
    d_pix_min = r_refdef.vrect.width / 320 as libc::c_int;
    if d_pix_min < 1 as libc::c_int {
        d_pix_min = 1 as libc::c_int;
    }
    d_pix_max = (r_refdef.vrect.width as libc::c_float as libc::c_double
        / (320.0f64 / 4.0f64) + 0.5f64) as libc::c_int;
    d_pix_shift = 8 as libc::c_int
        - (r_refdef.vrect.width as libc::c_float as libc::c_double / 320.0f64 + 0.5f64)
            as libc::c_int;
    if d_pix_max < 1 as libc::c_int {
        d_pix_max = 1 as libc::c_int;
    }
    d_vrectx = r_refdef.vrect.x;
    d_vrecty = r_refdef.vrect.y;
    d_vrectright_particle = r_refdef.vrectright - d_pix_max;
    d_vrectbottom_particle = r_refdef.vrectbottom - d_pix_max;
    i = 0 as libc::c_int;
    while i < vid.height {
        d_scantable[i as usize] = i * r_screenwidth;
        zspantable[i
            as usize] = d_pzbuffer
            .offset((i as libc::c_uint).wrapping_mul(d_zwidth) as isize);
        i += 1;
    }
    if r_newrefdef.rdflags & 2 as libc::c_int != 0 {
        memset(
            d_pzbuffer as *mut libc::c_void,
            0xff as libc::c_int,
            ((vid.width * vid.height) as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_short>() as libc::c_ulong),
        );
        Draw_Fill(
            r_newrefdef.x,
            r_newrefdef.y,
            r_newrefdef.width,
            r_newrefdef.height,
            (*sw_clearcolor).value as libc::c_int & 0xff as libc::c_int,
        );
    }
    alias_colormap = vid.colormap;
    D_Patch();
}
#[no_mangle]
pub unsafe extern "C" fn R_PrintTimes() {
    let mut r_time2: libc::c_int = 0;
    let mut ms: libc::c_int = 0;
    r_time2 = Sys_Milliseconds();
    ms = (r_time2 as libc::c_float - r_time1) as libc::c_int;
    (ri.Con_Printf)
        .expect(
            "non-null function pointer",
        )(
        0 as libc::c_int,
        b"%5i ms %3i/%3i/%3i poly %3i surf\n\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        ms,
        c_faceclip,
        r_polycount,
        r_drawnpolycount,
        c_surf,
    );
    c_surf = 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn R_PrintDSpeeds() {
    let mut ms: libc::c_int = 0;
    let mut dp_time: libc::c_int = 0;
    let mut r_time2: libc::c_int = 0;
    let mut rw_time: libc::c_int = 0;
    let mut db_time: libc::c_int = 0;
    let mut se_time: libc::c_int = 0;
    let mut de_time: libc::c_int = 0;
    let mut da_time: libc::c_int = 0;
    r_time2 = Sys_Milliseconds();
    da_time = (da_time2 - da_time1) as libc::c_int;
    dp_time = (dp_time2 - dp_time1) as libc::c_int;
    rw_time = (rw_time2 - rw_time1) as libc::c_int;
    db_time = (db_time2 - db_time1) as libc::c_int;
    se_time = (se_time2 - se_time1) as libc::c_int;
    de_time = (de_time2 - de_time1) as libc::c_int;
    ms = (r_time2 as libc::c_float - r_time1) as libc::c_int;
    (ri.Con_Printf)
        .expect(
            "non-null function pointer",
        )(
        0 as libc::c_int,
        b"%3i %2ip %2iw %2ib %2is %2ie %2ia\n\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        ms,
        dp_time,
        rw_time,
        db_time,
        se_time,
        de_time,
        da_time,
    );
}
#[no_mangle]
pub unsafe extern "C" fn R_PrintAliasStats() {
    (ri.Con_Printf)
        .expect(
            "non-null function pointer",
        )(
        0 as libc::c_int,
        b"%3i polygon model drawn\n\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        r_amodels_drawn,
    );
}
#[no_mangle]
pub unsafe extern "C" fn R_TransformFrustum() {
    let mut i: libc::c_int = 0;
    let mut v: vec3_t = [0.; 3];
    let mut v2: vec3_t = [0.; 3];
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        v[0 as libc::c_int
            as usize] = screenedge[i as usize].normal[2 as libc::c_int as usize];
        v[1 as libc::c_int
            as usize] = -screenedge[i as usize].normal[0 as libc::c_int as usize];
        v[2 as libc::c_int
            as usize] = screenedge[i as usize].normal[1 as libc::c_int as usize];
        v2[0 as libc::c_int
            as usize] = v[1 as libc::c_int as usize] * vright[0 as libc::c_int as usize]
            + v[2 as libc::c_int as usize] * vup[0 as libc::c_int as usize]
            + v[0 as libc::c_int as usize] * vpn[0 as libc::c_int as usize];
        v2[1 as libc::c_int
            as usize] = v[1 as libc::c_int as usize] * vright[1 as libc::c_int as usize]
            + v[2 as libc::c_int as usize] * vup[1 as libc::c_int as usize]
            + v[0 as libc::c_int as usize] * vpn[1 as libc::c_int as usize];
        v2[2 as libc::c_int
            as usize] = v[1 as libc::c_int as usize] * vright[2 as libc::c_int as usize]
            + v[2 as libc::c_int as usize] * vup[2 as libc::c_int as usize]
            + v[0 as libc::c_int as usize] * vpn[2 as libc::c_int as usize];
        view_clipplanes[i as usize]
            .normal[0 as libc::c_int as usize] = v2[0 as libc::c_int as usize];
        view_clipplanes[i as usize]
            .normal[1 as libc::c_int as usize] = v2[1 as libc::c_int as usize];
        view_clipplanes[i as usize]
            .normal[2 as libc::c_int as usize] = v2[2 as libc::c_int as usize];
        view_clipplanes[i as usize]
            .dist = modelorg[0 as libc::c_int as usize] * v2[0 as libc::c_int as usize]
            + modelorg[1 as libc::c_int as usize] * v2[1 as libc::c_int as usize]
            + modelorg[2 as libc::c_int as usize] * v2[2 as libc::c_int as usize];
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn TransformVector(mut in_0: *mut vec_t, mut out: *mut vec_t) {
    *out
        .offset(
            0 as libc::c_int as isize,
        ) = *in_0.offset(0 as libc::c_int as isize) * vright[0 as libc::c_int as usize]
        + *in_0.offset(1 as libc::c_int as isize) * vright[1 as libc::c_int as usize]
        + *in_0.offset(2 as libc::c_int as isize) * vright[2 as libc::c_int as usize];
    *out
        .offset(
            1 as libc::c_int as isize,
        ) = *in_0.offset(0 as libc::c_int as isize) * vup[0 as libc::c_int as usize]
        + *in_0.offset(1 as libc::c_int as isize) * vup[1 as libc::c_int as usize]
        + *in_0.offset(2 as libc::c_int as isize) * vup[2 as libc::c_int as usize];
    *out
        .offset(
            2 as libc::c_int as isize,
        ) = *in_0.offset(0 as libc::c_int as isize) * vpn[0 as libc::c_int as usize]
        + *in_0.offset(1 as libc::c_int as isize) * vpn[1 as libc::c_int as usize]
        + *in_0.offset(2 as libc::c_int as isize) * vpn[2 as libc::c_int as usize];
}
#[no_mangle]
pub unsafe extern "C" fn R_TransformPlane(
    mut p: *mut mplane_t,
    mut normal: *mut libc::c_float,
    mut dist: *mut libc::c_float,
) {
    let mut d: libc::c_float = 0.;
    d = r_origin[0 as libc::c_int as usize] * (*p).normal[0 as libc::c_int as usize]
        + r_origin[1 as libc::c_int as usize] * (*p).normal[1 as libc::c_int as usize]
        + r_origin[2 as libc::c_int as usize] * (*p).normal[2 as libc::c_int as usize];
    *dist = (*p).dist - d;
    TransformVector(((*p).normal).as_mut_ptr(), normal);
}
#[no_mangle]
pub unsafe extern "C" fn R_SetUpFrustumIndexes() {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut pindex: *mut libc::c_int = 0 as *mut libc::c_int;
    pindex = r_frustum_indexes.as_mut_ptr();
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        j = 0 as libc::c_int;
        while j < 3 as libc::c_int {
            if view_clipplanes[i as usize].normal[j as usize]
                < 0 as libc::c_int as libc::c_float
            {
                *pindex.offset(j as isize) = j;
                *pindex.offset((j + 3 as libc::c_int) as isize) = j + 3 as libc::c_int;
            } else {
                *pindex.offset(j as isize) = j + 3 as libc::c_int;
                *pindex.offset((j + 3 as libc::c_int) as isize) = j;
            }
            j += 1;
        }
        pfrustum_indexes[i as usize] = pindex;
        pindex = pindex.offset(6 as libc::c_int as isize);
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn R_ViewChanged(mut vr: *mut vrect_t) {
    let mut i: libc::c_int = 0;
    r_refdef.vrect = *vr;
    r_refdef
        .horizontalFieldOfView = (2 as libc::c_int as libc::c_double
        * tan(
            (r_newrefdef.fov_x / 360 as libc::c_int as libc::c_float) as libc::c_double
                * 3.14159265358979323846f64,
        )) as libc::c_float;
    verticalFieldOfView = (2 as libc::c_int as libc::c_double
        * tan(
            (r_newrefdef.fov_y / 360 as libc::c_int as libc::c_float) as libc::c_double
                * 3.14159265358979323846f64,
        )) as libc::c_float;
    r_refdef.fvrectx = r_refdef.vrect.x as libc::c_float;
    r_refdef
        .fvrectx_adj = (r_refdef.vrect.x as libc::c_float as libc::c_double - 0.5f64)
        as libc::c_float;
    r_refdef
        .vrect_x_adj_shift20 = (r_refdef.vrect.x << 20 as libc::c_int)
        + ((1 as libc::c_int) << 19 as libc::c_int) - 1 as libc::c_int;
    r_refdef.fvrecty = r_refdef.vrect.y as libc::c_float;
    r_refdef
        .fvrecty_adj = (r_refdef.vrect.y as libc::c_float as libc::c_double - 0.5f64)
        as libc::c_float;
    r_refdef.vrectright = r_refdef.vrect.x + r_refdef.vrect.width;
    r_refdef
        .vrectright_adj_shift20 = (r_refdef.vrectright << 20 as libc::c_int)
        + ((1 as libc::c_int) << 19 as libc::c_int) - 1 as libc::c_int;
    r_refdef.fvrectright = r_refdef.vrectright as libc::c_float;
    r_refdef
        .fvrectright_adj = (r_refdef.vrectright as libc::c_float as libc::c_double
        - 0.5f64) as libc::c_float;
    r_refdef
        .vrectrightedge = (r_refdef.vrectright as libc::c_float as libc::c_double
        - 0.99f64) as libc::c_float;
    r_refdef.vrectbottom = r_refdef.vrect.y + r_refdef.vrect.height;
    r_refdef.fvrectbottom = r_refdef.vrectbottom as libc::c_float;
    r_refdef
        .fvrectbottom_adj = (r_refdef.vrectbottom as libc::c_float as libc::c_double
        - 0.5f64) as libc::c_float;
    r_refdef
        .aliasvrect
        .x = (r_refdef.vrect.x as libc::c_float * r_aliasuvscale) as libc::c_int;
    r_refdef
        .aliasvrect
        .y = (r_refdef.vrect.y as libc::c_float * r_aliasuvscale) as libc::c_int;
    r_refdef
        .aliasvrect
        .width = (r_refdef.vrect.width as libc::c_float * r_aliasuvscale) as libc::c_int;
    r_refdef
        .aliasvrect
        .height = (r_refdef.vrect.height as libc::c_float * r_aliasuvscale)
        as libc::c_int;
    r_refdef.aliasvrectright = r_refdef.aliasvrect.x + r_refdef.aliasvrect.width;
    r_refdef.aliasvrectbottom = r_refdef.aliasvrect.y + r_refdef.aliasvrect.height;
    xOrigin = r_refdef.xOrigin;
    yOrigin = r_refdef.yOrigin;
    xcenter = (r_refdef.vrect.width as libc::c_float as libc::c_double
        * (1.0f64 / 2.0f64) + r_refdef.vrect.x as libc::c_double - 0.5f64)
        as libc::c_float;
    aliasxcenter = xcenter * r_aliasuvscale;
    ycenter = (r_refdef.vrect.height as libc::c_float as libc::c_double
        * (1.0f64 / 2.0f64) + r_refdef.vrect.y as libc::c_double - 0.5f64)
        as libc::c_float;
    aliasycenter = ycenter * r_aliasuvscale;
    xscale = r_refdef.vrect.width as libc::c_float / r_refdef.horizontalFieldOfView;
    aliasxscale = xscale * r_aliasuvscale;
    xscaleinv = (1.0f64 / xscale as libc::c_double) as libc::c_float;
    yscale = xscale;
    aliasyscale = yscale * r_aliasuvscale;
    yscaleinv = (1.0f64 / yscale as libc::c_double) as libc::c_float;
    xscaleshrink = (r_refdef.vrect.width - 6 as libc::c_int) as libc::c_float
        / r_refdef.horizontalFieldOfView;
    yscaleshrink = xscaleshrink;
    screenedge[0 as libc::c_int as usize]
        .normal[0 as libc::c_int
        as usize] = (-1.0f64
        / (xOrigin * r_refdef.horizontalFieldOfView) as libc::c_double) as vec_t;
    screenedge[0 as libc::c_int as usize]
        .normal[1 as libc::c_int as usize] = 0 as libc::c_int as vec_t;
    screenedge[0 as libc::c_int as usize]
        .normal[2 as libc::c_int as usize] = 1 as libc::c_int as vec_t;
    screenedge[0 as libc::c_int as usize].type_0 = 5 as libc::c_int as byte;
    screenedge[1 as libc::c_int as usize]
        .normal[0 as libc::c_int
        as usize] = (1.0f64
        / ((1.0f64 - xOrigin as libc::c_double)
            * r_refdef.horizontalFieldOfView as libc::c_double)) as vec_t;
    screenedge[1 as libc::c_int as usize]
        .normal[1 as libc::c_int as usize] = 0 as libc::c_int as vec_t;
    screenedge[1 as libc::c_int as usize]
        .normal[2 as libc::c_int as usize] = 1 as libc::c_int as vec_t;
    screenedge[1 as libc::c_int as usize].type_0 = 5 as libc::c_int as byte;
    screenedge[2 as libc::c_int as usize]
        .normal[0 as libc::c_int as usize] = 0 as libc::c_int as vec_t;
    screenedge[2 as libc::c_int as usize]
        .normal[1 as libc::c_int
        as usize] = (-1.0f64 / (yOrigin * verticalFieldOfView) as libc::c_double)
        as vec_t;
    screenedge[2 as libc::c_int as usize]
        .normal[2 as libc::c_int as usize] = 1 as libc::c_int as vec_t;
    screenedge[2 as libc::c_int as usize].type_0 = 5 as libc::c_int as byte;
    screenedge[3 as libc::c_int as usize]
        .normal[0 as libc::c_int as usize] = 0 as libc::c_int as vec_t;
    screenedge[3 as libc::c_int as usize]
        .normal[1 as libc::c_int
        as usize] = (1.0f64
        / ((1.0f64 - yOrigin as libc::c_double) * verticalFieldOfView as libc::c_double))
        as vec_t;
    screenedge[3 as libc::c_int as usize]
        .normal[2 as libc::c_int as usize] = 1 as libc::c_int as vec_t;
    screenedge[3 as libc::c_int as usize].type_0 = 5 as libc::c_int as byte;
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        VectorNormalize((screenedge[i as usize].normal).as_mut_ptr());
        i += 1;
    }
    D_ViewChanged();
}
#[no_mangle]
pub unsafe extern "C" fn R_SetupFrame() {
    let mut i: libc::c_int = 0;
    let mut vrect: vrect_t = vrect_t {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
        pnext: 0 as *const vrect_s as *mut vrect_s,
    };
    if (*r_fullbright).modified as u64 != 0 {
        (*r_fullbright).modified = false_0;
        D_FlushCaches();
    }
    r_framecount += 1;
    modelorg[0 as libc::c_int as usize] = r_refdef.vieworg[0 as libc::c_int as usize];
    modelorg[1 as libc::c_int as usize] = r_refdef.vieworg[1 as libc::c_int as usize];
    modelorg[2 as libc::c_int as usize] = r_refdef.vieworg[2 as libc::c_int as usize];
    r_origin[0 as libc::c_int as usize] = r_refdef.vieworg[0 as libc::c_int as usize];
    r_origin[1 as libc::c_int as usize] = r_refdef.vieworg[1 as libc::c_int as usize];
    r_origin[2 as libc::c_int as usize] = r_refdef.vieworg[2 as libc::c_int as usize];
    AngleVectors(
        (r_refdef.viewangles).as_mut_ptr(),
        vpn.as_mut_ptr(),
        vright.as_mut_ptr(),
        vup.as_mut_ptr(),
    );
    if r_newrefdef.rdflags & 2 as libc::c_int == 0 {
        r_viewleaf = Mod_PointInLeaf(r_origin.as_mut_ptr(), r_worldmodel);
        r_viewcluster = (*r_viewleaf).cluster;
    }
    if (*sw_waterwarp).value != 0. && r_newrefdef.rdflags & 1 as libc::c_int != 0 {
        r_dowarp = true_0;
    } else {
        r_dowarp = false_0;
    }
    if r_dowarp as u64 != 0 {
        vrect.x = 0 as libc::c_int;
        vrect.y = 0 as libc::c_int;
        vrect
            .width = if r_newrefdef.width < 320 as libc::c_int {
            r_newrefdef.width
        } else {
            320 as libc::c_int
        };
        vrect
            .height = if r_newrefdef.height < 240 as libc::c_int {
            r_newrefdef.height
        } else {
            240 as libc::c_int
        };
        d_viewbuffer = r_warpbuffer.as_mut_ptr();
        r_screenwidth = 320 as libc::c_int;
    } else {
        vrect.x = r_newrefdef.x;
        vrect.y = r_newrefdef.y;
        vrect.width = r_newrefdef.width;
        vrect.height = r_newrefdef.height;
        d_viewbuffer = vid.buffer as *mut libc::c_void as *mut pixel_t;
        r_screenwidth = vid.rowbytes;
    }
    R_ViewChanged(&mut vrect);
    R_TransformFrustum();
    R_SetUpFrustumIndexes();
    base_vpn[0 as libc::c_int as usize] = vpn[0 as libc::c_int as usize];
    base_vpn[1 as libc::c_int as usize] = vpn[1 as libc::c_int as usize];
    base_vpn[2 as libc::c_int as usize] = vpn[2 as libc::c_int as usize];
    base_vright[0 as libc::c_int as usize] = vright[0 as libc::c_int as usize];
    base_vright[1 as libc::c_int as usize] = vright[1 as libc::c_int as usize];
    base_vright[2 as libc::c_int as usize] = vright[2 as libc::c_int as usize];
    base_vup[0 as libc::c_int as usize] = vup[0 as libc::c_int as usize];
    base_vup[1 as libc::c_int as usize] = vup[1 as libc::c_int as usize];
    base_vup[2 as libc::c_int as usize] = vup[2 as libc::c_int as usize];
    c_faceclip = 0 as libc::c_int;
    d_spanpixcount = 0 as libc::c_int;
    r_polycount = 0 as libc::c_int;
    r_drawnpolycount = 0 as libc::c_int;
    r_wholepolycount = 0 as libc::c_int;
    r_amodels_drawn = 0 as libc::c_int;
    r_outofsurfaces = 0 as libc::c_int;
    r_outofedges = 0 as libc::c_int;
    d_roverwrapped = false_0;
    d_initial_rover = sc_rover;
    d_minmip = (*sw_mipcap).value as libc::c_int;
    if d_minmip > 3 as libc::c_int {
        d_minmip = 3 as libc::c_int;
    } else if d_minmip < 0 as libc::c_int {
        d_minmip = 0 as libc::c_int;
    }
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int - 1 as libc::c_int {
        d_scalemip[i as usize] = basemip[i as usize] * (*sw_mipscale).value;
        i += 1;
    }
    d_aflatcolor = 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn R_SurfacePatch() {}
#[no_mangle]
pub unsafe extern "C" fn WritePCXfile(
    mut filename: *mut libc::c_char,
    mut data: *mut byte,
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut rowbytes: libc::c_int,
    mut palette: *mut byte,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut length: libc::c_int = 0;
    let mut pcx: *mut pcx_t = 0 as *mut pcx_t;
    let mut pack: *mut byte = 0 as *mut byte;
    let mut f: *mut FILE = 0 as *mut FILE;
    pcx = malloc(
        (width * height * 2 as libc::c_int + 1000 as libc::c_int) as libc::c_ulong,
    ) as *mut pcx_t;
    if pcx.is_null() {
        return;
    }
    (*pcx).manufacturer = 0xa as libc::c_int as libc::c_char;
    (*pcx).version = 5 as libc::c_int as libc::c_char;
    (*pcx).encoding = 1 as libc::c_int as libc::c_char;
    (*pcx).bits_per_pixel = 8 as libc::c_int as libc::c_char;
    (*pcx).xmin = 0 as libc::c_int as libc::c_ushort;
    (*pcx).ymin = 0 as libc::c_int as libc::c_ushort;
    (*pcx)
        .xmax = LittleShort((width - 1 as libc::c_int) as libc::c_short)
        as libc::c_ushort;
    (*pcx)
        .ymax = LittleShort((height - 1 as libc::c_int) as libc::c_short)
        as libc::c_ushort;
    (*pcx).hres = LittleShort(width as libc::c_short) as libc::c_ushort;
    (*pcx).vres = LittleShort(height as libc::c_short) as libc::c_ushort;
    memset(
        ((*pcx).palette).as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<[libc::c_uchar; 48]>() as libc::c_ulong,
    );
    (*pcx).color_planes = 1 as libc::c_int as libc::c_char;
    (*pcx).bytes_per_line = LittleShort(width as libc::c_short) as libc::c_ushort;
    (*pcx)
        .palette_type = LittleShort(2 as libc::c_int as libc::c_short) as libc::c_ushort;
    memset(
        ((*pcx).filler).as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<[libc::c_char; 58]>() as libc::c_ulong,
    );
    pack = &mut (*pcx).data;
    i = 0 as libc::c_int;
    while i < height {
        j = 0 as libc::c_int;
        while j < width {
            if *data as libc::c_int & 0xc0 as libc::c_int != 0xc0 as libc::c_int {
                let fresh0 = data;
                data = data.offset(1);
                let fresh1 = pack;
                pack = pack.offset(1);
                *fresh1 = *fresh0;
            } else {
                let fresh2 = pack;
                pack = pack.offset(1);
                *fresh2 = 0xc1 as libc::c_int as byte;
                let fresh3 = data;
                data = data.offset(1);
                let fresh4 = pack;
                pack = pack.offset(1);
                *fresh4 = *fresh3;
            }
            j += 1;
        }
        data = data.offset((rowbytes - width) as isize);
        i += 1;
    }
    let fresh5 = pack;
    pack = pack.offset(1);
    *fresh5 = 0xc as libc::c_int as byte;
    i = 0 as libc::c_int;
    while i < 768 as libc::c_int {
        let fresh6 = palette;
        palette = palette.offset(1);
        let fresh7 = pack;
        pack = pack.offset(1);
        *fresh7 = *fresh6;
        i += 1;
    }
    length = pack.offset_from(pcx as *mut byte) as libc::c_long as libc::c_int;
    f = fopen(filename, b"wb\0" as *const u8 as *const libc::c_char);
    if f.is_null() {
        (ri.Con_Printf)
            .expect(
                "non-null function pointer",
            )(
            0 as libc::c_int,
            b"Failed to open to %s\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            filename,
        );
    } else {
        fwrite(
            pcx as *mut libc::c_void,
            1 as libc::c_int as libc::c_ulong,
            length as libc::c_ulong,
            f,
        );
        fclose(f);
    }
    free(pcx as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn R_ScreenShot_f() {
    let mut i: libc::c_int = 0;
    let mut pcxname: [libc::c_char; 80] = [0; 80];
    let mut checkname: [libc::c_char; 128] = [0; 128];
    let mut f: *mut FILE = 0 as *mut FILE;
    let mut palette: [byte; 768] = [0; 768];
    Com_sprintf(
        checkname.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong as libc::c_int,
        b"%s/scrnshot\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        (ri.FS_Gamedir).expect("non-null function pointer")(),
    );
    Sys_Mkdir(checkname.as_mut_ptr());
    strcpy(pcxname.as_mut_ptr(), b"quake00.pcx\0" as *const u8 as *const libc::c_char);
    i = 0 as libc::c_int;
    while i <= 99 as libc::c_int {
        pcxname[5 as libc::c_int
            as usize] = (i / 10 as libc::c_int + '0' as i32) as libc::c_char;
        pcxname[6 as libc::c_int
            as usize] = (i % 10 as libc::c_int + '0' as i32) as libc::c_char;
        Com_sprintf(
            checkname.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong as libc::c_int,
            b"%s/scrnshot/%s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            (ri.FS_Gamedir).expect("non-null function pointer")(),
            pcxname.as_mut_ptr(),
        );
        f = fopen(checkname.as_mut_ptr(), b"r\0" as *const u8 as *const libc::c_char);
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
            b"R_ScreenShot_f: Couldn't create a PCX\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        );
        return;
    }
    i = 0 as libc::c_int;
    while i < 256 as libc::c_int {
        palette[(i * 3 as libc::c_int + 0 as libc::c_int)
            as usize] = sw_state
            .currentpalette[(i * 4 as libc::c_int + 0 as libc::c_int) as usize];
        palette[(i * 3 as libc::c_int + 1 as libc::c_int)
            as usize] = sw_state
            .currentpalette[(i * 4 as libc::c_int + 1 as libc::c_int) as usize];
        palette[(i * 3 as libc::c_int + 2 as libc::c_int)
            as usize] = sw_state
            .currentpalette[(i * 4 as libc::c_int + 2 as libc::c_int) as usize];
        i += 1;
    }
    WritePCXfile(
        checkname.as_mut_ptr(),
        vid.buffer,
        vid.width,
        vid.height,
        vid.rowbytes,
        palette.as_mut_ptr(),
    );
    (ri.Con_Printf)
        .expect(
            "non-null function pointer",
        )(
        0 as libc::c_int,
        b"Wrote %s\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        checkname.as_mut_ptr(),
    );
}
