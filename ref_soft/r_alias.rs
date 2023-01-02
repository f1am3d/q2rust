#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    fn sin(_: libc::c_double) -> libc::c_double;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn VectorInverse(v: *mut vec_t);
    fn R_ConcatTransforms(
        in1: *mut [libc::c_float; 4],
        in2: *mut [libc::c_float; 4],
        out: *mut [libc::c_float; 4],
    );
    fn AngleVectors(
        angles: *mut vec_t,
        forward: *mut vec_t,
        right: *mut vec_t,
        up: *mut vec_t,
    );
    fn R_PolysetUpdateTables();
    static mut vup: vec3_t;
    static mut vpn: vec3_t;
    static mut vright: vec3_t;
    static mut r_lefthand: *mut cvar_t;
    static mut r_lerpmodels: *mut cvar_t;
    static mut r_lightlevel: *mut cvar_t;
    static mut view_clipplanes: [clipplane_t; 4];
    static mut r_origin: vec3_t;
    static mut currentmodel: *mut model_t;
    static mut currententity: *mut entity_t;
    static mut aliasxscale: libc::c_float;
    fn R_AliasClipTriangle(
        index0: *mut finalvert_t,
        index1: *mut finalvert_t,
        index2: *mut finalvert_t,
    );
    fn R_DrawTriangle();
    static mut aliastriangleparms: aliastriangleparms_t;
    static mut aliasycenter: libc::c_float;
    static mut aliasyscale: libc::c_float;
    static mut aliasxcenter: libc::c_float;
    static mut r_refdef: oldrefdef_t;
    fn R_LightPoint(p: *mut vec_t, color: *mut vec_t);
    static mut r_newrefdef: refdef_t;
    static mut ri: refimport_t;
    static mut iractive: byte;
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
pub struct dstvert_t {
    pub s: libc::c_short,
    pub t: libc::c_short,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dtriangle_t {
    pub index_xyz: [libc::c_short; 3],
    pub index_st: [libc::c_short; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dtrivertx_t {
    pub v: [byte; 3],
    pub lightnormalindex: byte,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct daliasframe_t {
    pub scale: [libc::c_float; 3],
    pub translate: [libc::c_float; 3],
    pub name: [libc::c_char; 16],
    pub verts: [dtrivertx_t; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dmdl_t {
    pub ident: libc::c_int,
    pub version: libc::c_int,
    pub skinwidth: libc::c_int,
    pub skinheight: libc::c_int,
    pub framesize: libc::c_int,
    pub num_skins: libc::c_int,
    pub num_xyz: libc::c_int,
    pub num_st: libc::c_int,
    pub num_tris: libc::c_int,
    pub num_glcmds: libc::c_int,
    pub num_frames: libc::c_int,
    pub ofs_skins: libc::c_int,
    pub ofs_st: libc::c_int,
    pub ofs_tris: libc::c_int,
    pub ofs_frames: libc::c_int,
    pub ofs_glcmds: libc::c_int,
    pub ofs_end: libc::c_int,
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
pub struct finalvert_s {
    pub u: libc::c_int,
    pub v: libc::c_int,
    pub s: libc::c_int,
    pub t: libc::c_int,
    pub l: libc::c_int,
    pub zi: libc::c_int,
    pub flags: libc::c_int,
    pub xyz: [libc::c_float; 3],
}
pub type finalvert_t = finalvert_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct affinetridesc_t {
    pub pskin: *mut libc::c_void,
    pub pskindesc: libc::c_int,
    pub skinwidth: libc::c_int,
    pub skinheight: libc::c_int,
    pub ptriangles: *mut dtriangle_t,
    pub pfinalverts: *mut finalvert_t,
    pub numtriangles: libc::c_int,
    pub drawtype: libc::c_int,
    pub seamfixupX16: libc::c_int,
    pub do_vis_thresh: qboolean,
    pub vis_thresh: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct alight_t {
    pub ambientlight: libc::c_int,
    pub shadelight: libc::c_int,
    pub plightvec: *mut libc::c_float,
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aliastriangleparms_t {
    pub a: *mut finalvert_t,
    pub b: *mut finalvert_t,
    pub c: *mut finalvert_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aliasbatchedtransformdata_t {
    pub num_points: libc::c_int,
    pub last_verts: *mut dtrivertx_t,
    pub this_verts: *mut dtrivertx_t,
    pub dest_verts: *mut finalvert_t,
}
#[no_mangle]
pub static mut r_amodels_drawn: libc::c_int = 0;
#[no_mangle]
pub static mut r_affinetridesc: affinetridesc_t = affinetridesc_t {
    pskin: 0 as *const libc::c_void as *mut libc::c_void,
    pskindesc: 0,
    skinwidth: 0,
    skinheight: 0,
    ptriangles: 0 as *const dtriangle_t as *mut dtriangle_t,
    pfinalverts: 0 as *const finalvert_t as *mut finalvert_t,
    numtriangles: 0,
    drawtype: 0,
    seamfixupX16: 0,
    do_vis_thresh: false_0,
    vis_thresh: 0,
};
#[no_mangle]
pub static mut r_plightvec: vec3_t = [0.; 3];
#[no_mangle]
pub static mut r_lerped: [vec3_t; 1024] = [[0.; 3]; 1024];
#[no_mangle]
pub static mut r_lerp_frontv: vec3_t = [0.; 3];
#[no_mangle]
pub static mut r_lerp_backv: vec3_t = [0.; 3];
#[no_mangle]
pub static mut r_lerp_move: vec3_t = [0.; 3];
#[no_mangle]
pub static mut r_ambientlight: libc::c_int = 0;
#[no_mangle]
pub static mut r_aliasblendcolor: libc::c_int = 0;
#[no_mangle]
pub static mut r_shadelight: libc::c_float = 0.;
#[no_mangle]
pub static mut r_thisframe: *mut daliasframe_t = 0 as *const daliasframe_t
    as *mut daliasframe_t;
#[no_mangle]
pub static mut r_lastframe: *mut daliasframe_t = 0 as *const daliasframe_t
    as *mut daliasframe_t;
#[no_mangle]
pub static mut s_pmdl: *mut dmdl_t = 0 as *const dmdl_t as *mut dmdl_t;
#[no_mangle]
pub static mut aliastransform: [[libc::c_float; 4]; 3] = [[0.; 4]; 3];
#[no_mangle]
pub static mut aliasworldtransform: [[libc::c_float; 4]; 3] = [[0.; 4]; 3];
#[no_mangle]
pub static mut aliasoldworldtransform: [[libc::c_float; 4]; 3] = [[0.; 4]; 3];
static mut s_ziscale: libc::c_float = 0.;
static mut s_alias_forward: vec3_t = [0.; 3];
static mut s_alias_right: vec3_t = [0.; 3];
static mut s_alias_up: vec3_t = [0.; 3];
#[no_mangle]
pub static mut r_avertexnormals: [[libc::c_float; 3]; 162] = [
    [
        -0.525731f64 as libc::c_float,
        0.000000f64 as libc::c_float,
        0.850651f64 as libc::c_float,
    ],
    [
        -0.442863f64 as libc::c_float,
        0.238856f64 as libc::c_float,
        0.864188f64 as libc::c_float,
    ],
    [
        -0.295242f64 as libc::c_float,
        0.000000f64 as libc::c_float,
        0.955423f64 as libc::c_float,
    ],
    [
        -0.309017f64 as libc::c_float,
        0.500000f64 as libc::c_float,
        0.809017f64 as libc::c_float,
    ],
    [
        -0.162460f64 as libc::c_float,
        0.262866f64 as libc::c_float,
        0.951056f64 as libc::c_float,
    ],
    [
        0.000000f64 as libc::c_float,
        0.000000f64 as libc::c_float,
        1.000000f64 as libc::c_float,
    ],
    [
        0.000000f64 as libc::c_float,
        0.850651f64 as libc::c_float,
        0.525731f64 as libc::c_float,
    ],
    [
        -0.147621f64 as libc::c_float,
        0.716567f64 as libc::c_float,
        0.681718f64 as libc::c_float,
    ],
    [
        0.147621f64 as libc::c_float,
        0.716567f64 as libc::c_float,
        0.681718f64 as libc::c_float,
    ],
    [
        0.000000f64 as libc::c_float,
        0.525731f64 as libc::c_float,
        0.850651f64 as libc::c_float,
    ],
    [
        0.309017f64 as libc::c_float,
        0.500000f64 as libc::c_float,
        0.809017f64 as libc::c_float,
    ],
    [
        0.525731f64 as libc::c_float,
        0.000000f64 as libc::c_float,
        0.850651f64 as libc::c_float,
    ],
    [
        0.295242f64 as libc::c_float,
        0.000000f64 as libc::c_float,
        0.955423f64 as libc::c_float,
    ],
    [
        0.442863f64 as libc::c_float,
        0.238856f64 as libc::c_float,
        0.864188f64 as libc::c_float,
    ],
    [
        0.162460f64 as libc::c_float,
        0.262866f64 as libc::c_float,
        0.951056f64 as libc::c_float,
    ],
    [
        -0.681718f64 as libc::c_float,
        0.147621f64 as libc::c_float,
        0.716567f64 as libc::c_float,
    ],
    [
        -0.809017f64 as libc::c_float,
        0.309017f64 as libc::c_float,
        0.500000f64 as libc::c_float,
    ],
    [
        -0.587785f64 as libc::c_float,
        0.425325f64 as libc::c_float,
        0.688191f64 as libc::c_float,
    ],
    [
        -0.850651f64 as libc::c_float,
        0.525731f64 as libc::c_float,
        0.000000f64 as libc::c_float,
    ],
    [
        -0.864188f64 as libc::c_float,
        0.442863f64 as libc::c_float,
        0.238856f64 as libc::c_float,
    ],
    [
        -0.716567f64 as libc::c_float,
        0.681718f64 as libc::c_float,
        0.147621f64 as libc::c_float,
    ],
    [
        -0.688191f64 as libc::c_float,
        0.587785f64 as libc::c_float,
        0.425325f64 as libc::c_float,
    ],
    [
        -0.500000f64 as libc::c_float,
        0.809017f64 as libc::c_float,
        0.309017f64 as libc::c_float,
    ],
    [
        -0.238856f64 as libc::c_float,
        0.864188f64 as libc::c_float,
        0.442863f64 as libc::c_float,
    ],
    [
        -0.425325f64 as libc::c_float,
        0.688191f64 as libc::c_float,
        0.587785f64 as libc::c_float,
    ],
    [
        -0.716567f64 as libc::c_float,
        0.681718f64 as libc::c_float,
        -0.147621f64 as libc::c_float,
    ],
    [
        -0.500000f64 as libc::c_float,
        0.809017f64 as libc::c_float,
        -0.309017f64 as libc::c_float,
    ],
    [
        -0.525731f64 as libc::c_float,
        0.850651f64 as libc::c_float,
        0.000000f64 as libc::c_float,
    ],
    [
        0.000000f64 as libc::c_float,
        0.850651f64 as libc::c_float,
        -0.525731f64 as libc::c_float,
    ],
    [
        -0.238856f64 as libc::c_float,
        0.864188f64 as libc::c_float,
        -0.442863f64 as libc::c_float,
    ],
    [
        0.000000f64 as libc::c_float,
        0.955423f64 as libc::c_float,
        -0.295242f64 as libc::c_float,
    ],
    [
        -0.262866f64 as libc::c_float,
        0.951056f64 as libc::c_float,
        -0.162460f64 as libc::c_float,
    ],
    [
        0.000000f64 as libc::c_float,
        1.000000f64 as libc::c_float,
        0.000000f64 as libc::c_float,
    ],
    [
        0.000000f64 as libc::c_float,
        0.955423f64 as libc::c_float,
        0.295242f64 as libc::c_float,
    ],
    [
        -0.262866f64 as libc::c_float,
        0.951056f64 as libc::c_float,
        0.162460f64 as libc::c_float,
    ],
    [
        0.238856f64 as libc::c_float,
        0.864188f64 as libc::c_float,
        0.442863f64 as libc::c_float,
    ],
    [
        0.262866f64 as libc::c_float,
        0.951056f64 as libc::c_float,
        0.162460f64 as libc::c_float,
    ],
    [
        0.500000f64 as libc::c_float,
        0.809017f64 as libc::c_float,
        0.309017f64 as libc::c_float,
    ],
    [
        0.238856f64 as libc::c_float,
        0.864188f64 as libc::c_float,
        -0.442863f64 as libc::c_float,
    ],
    [
        0.262866f64 as libc::c_float,
        0.951056f64 as libc::c_float,
        -0.162460f64 as libc::c_float,
    ],
    [
        0.500000f64 as libc::c_float,
        0.809017f64 as libc::c_float,
        -0.309017f64 as libc::c_float,
    ],
    [
        0.850651f64 as libc::c_float,
        0.525731f64 as libc::c_float,
        0.000000f64 as libc::c_float,
    ],
    [
        0.716567f64 as libc::c_float,
        0.681718f64 as libc::c_float,
        0.147621f64 as libc::c_float,
    ],
    [
        0.716567f64 as libc::c_float,
        0.681718f64 as libc::c_float,
        -0.147621f64 as libc::c_float,
    ],
    [
        0.525731f64 as libc::c_float,
        0.850651f64 as libc::c_float,
        0.000000f64 as libc::c_float,
    ],
    [
        0.425325f64 as libc::c_float,
        0.688191f64 as libc::c_float,
        0.587785f64 as libc::c_float,
    ],
    [
        0.864188f64 as libc::c_float,
        0.442863f64 as libc::c_float,
        0.238856f64 as libc::c_float,
    ],
    [
        0.688191f64 as libc::c_float,
        0.587785f64 as libc::c_float,
        0.425325f64 as libc::c_float,
    ],
    [
        0.809017f64 as libc::c_float,
        0.309017f64 as libc::c_float,
        0.500000f64 as libc::c_float,
    ],
    [
        0.681718f64 as libc::c_float,
        0.147621f64 as libc::c_float,
        0.716567f64 as libc::c_float,
    ],
    [
        0.587785f64 as libc::c_float,
        0.425325f64 as libc::c_float,
        0.688191f64 as libc::c_float,
    ],
    [
        0.955423f64 as libc::c_float,
        0.295242f64 as libc::c_float,
        0.000000f64 as libc::c_float,
    ],
    [
        1.000000f64 as libc::c_float,
        0.000000f64 as libc::c_float,
        0.000000f64 as libc::c_float,
    ],
    [
        0.951056f64 as libc::c_float,
        0.162460f64 as libc::c_float,
        0.262866f64 as libc::c_float,
    ],
    [
        0.850651f64 as libc::c_float,
        -0.525731f64 as libc::c_float,
        0.000000f64 as libc::c_float,
    ],
    [
        0.955423f64 as libc::c_float,
        -0.295242f64 as libc::c_float,
        0.000000f64 as libc::c_float,
    ],
    [
        0.864188f64 as libc::c_float,
        -0.442863f64 as libc::c_float,
        0.238856f64 as libc::c_float,
    ],
    [
        0.951056f64 as libc::c_float,
        -0.162460f64 as libc::c_float,
        0.262866f64 as libc::c_float,
    ],
    [
        0.809017f64 as libc::c_float,
        -0.309017f64 as libc::c_float,
        0.500000f64 as libc::c_float,
    ],
    [
        0.681718f64 as libc::c_float,
        -0.147621f64 as libc::c_float,
        0.716567f64 as libc::c_float,
    ],
    [
        0.850651f64 as libc::c_float,
        0.000000f64 as libc::c_float,
        0.525731f64 as libc::c_float,
    ],
    [
        0.864188f64 as libc::c_float,
        0.442863f64 as libc::c_float,
        -0.238856f64 as libc::c_float,
    ],
    [
        0.809017f64 as libc::c_float,
        0.309017f64 as libc::c_float,
        -0.500000f64 as libc::c_float,
    ],
    [
        0.951056f64 as libc::c_float,
        0.162460f64 as libc::c_float,
        -0.262866f64 as libc::c_float,
    ],
    [
        0.525731f64 as libc::c_float,
        0.000000f64 as libc::c_float,
        -0.850651f64 as libc::c_float,
    ],
    [
        0.681718f64 as libc::c_float,
        0.147621f64 as libc::c_float,
        -0.716567f64 as libc::c_float,
    ],
    [
        0.681718f64 as libc::c_float,
        -0.147621f64 as libc::c_float,
        -0.716567f64 as libc::c_float,
    ],
    [
        0.850651f64 as libc::c_float,
        0.000000f64 as libc::c_float,
        -0.525731f64 as libc::c_float,
    ],
    [
        0.809017f64 as libc::c_float,
        -0.309017f64 as libc::c_float,
        -0.500000f64 as libc::c_float,
    ],
    [
        0.864188f64 as libc::c_float,
        -0.442863f64 as libc::c_float,
        -0.238856f64 as libc::c_float,
    ],
    [
        0.951056f64 as libc::c_float,
        -0.162460f64 as libc::c_float,
        -0.262866f64 as libc::c_float,
    ],
    [
        0.147621f64 as libc::c_float,
        0.716567f64 as libc::c_float,
        -0.681718f64 as libc::c_float,
    ],
    [
        0.309017f64 as libc::c_float,
        0.500000f64 as libc::c_float,
        -0.809017f64 as libc::c_float,
    ],
    [
        0.425325f64 as libc::c_float,
        0.688191f64 as libc::c_float,
        -0.587785f64 as libc::c_float,
    ],
    [
        0.442863f64 as libc::c_float,
        0.238856f64 as libc::c_float,
        -0.864188f64 as libc::c_float,
    ],
    [
        0.587785f64 as libc::c_float,
        0.425325f64 as libc::c_float,
        -0.688191f64 as libc::c_float,
    ],
    [
        0.688191f64 as libc::c_float,
        0.587785f64 as libc::c_float,
        -0.425325f64 as libc::c_float,
    ],
    [
        -0.147621f64 as libc::c_float,
        0.716567f64 as libc::c_float,
        -0.681718f64 as libc::c_float,
    ],
    [
        -0.309017f64 as libc::c_float,
        0.500000f64 as libc::c_float,
        -0.809017f64 as libc::c_float,
    ],
    [
        0.000000f64 as libc::c_float,
        0.525731f64 as libc::c_float,
        -0.850651f64 as libc::c_float,
    ],
    [
        -0.525731f64 as libc::c_float,
        0.000000f64 as libc::c_float,
        -0.850651f64 as libc::c_float,
    ],
    [
        -0.442863f64 as libc::c_float,
        0.238856f64 as libc::c_float,
        -0.864188f64 as libc::c_float,
    ],
    [
        -0.295242f64 as libc::c_float,
        0.000000f64 as libc::c_float,
        -0.955423f64 as libc::c_float,
    ],
    [
        -0.162460f64 as libc::c_float,
        0.262866f64 as libc::c_float,
        -0.951056f64 as libc::c_float,
    ],
    [
        0.000000f64 as libc::c_float,
        0.000000f64 as libc::c_float,
        -1.000000f64 as libc::c_float,
    ],
    [
        0.295242f64 as libc::c_float,
        0.000000f64 as libc::c_float,
        -0.955423f64 as libc::c_float,
    ],
    [
        0.162460f64 as libc::c_float,
        0.262866f64 as libc::c_float,
        -0.951056f64 as libc::c_float,
    ],
    [
        -0.442863f64 as libc::c_float,
        -0.238856f64 as libc::c_float,
        -0.864188f64 as libc::c_float,
    ],
    [
        -0.309017f64 as libc::c_float,
        -0.500000f64 as libc::c_float,
        -0.809017f64 as libc::c_float,
    ],
    [
        -0.162460f64 as libc::c_float,
        -0.262866f64 as libc::c_float,
        -0.951056f64 as libc::c_float,
    ],
    [
        0.000000f64 as libc::c_float,
        -0.850651f64 as libc::c_float,
        -0.525731f64 as libc::c_float,
    ],
    [
        -0.147621f64 as libc::c_float,
        -0.716567f64 as libc::c_float,
        -0.681718f64 as libc::c_float,
    ],
    [
        0.147621f64 as libc::c_float,
        -0.716567f64 as libc::c_float,
        -0.681718f64 as libc::c_float,
    ],
    [
        0.000000f64 as libc::c_float,
        -0.525731f64 as libc::c_float,
        -0.850651f64 as libc::c_float,
    ],
    [
        0.309017f64 as libc::c_float,
        -0.500000f64 as libc::c_float,
        -0.809017f64 as libc::c_float,
    ],
    [
        0.442863f64 as libc::c_float,
        -0.238856f64 as libc::c_float,
        -0.864188f64 as libc::c_float,
    ],
    [
        0.162460f64 as libc::c_float,
        -0.262866f64 as libc::c_float,
        -0.951056f64 as libc::c_float,
    ],
    [
        0.238856f64 as libc::c_float,
        -0.864188f64 as libc::c_float,
        -0.442863f64 as libc::c_float,
    ],
    [
        0.500000f64 as libc::c_float,
        -0.809017f64 as libc::c_float,
        -0.309017f64 as libc::c_float,
    ],
    [
        0.425325f64 as libc::c_float,
        -0.688191f64 as libc::c_float,
        -0.587785f64 as libc::c_float,
    ],
    [
        0.716567f64 as libc::c_float,
        -0.681718f64 as libc::c_float,
        -0.147621f64 as libc::c_float,
    ],
    [
        0.688191f64 as libc::c_float,
        -0.587785f64 as libc::c_float,
        -0.425325f64 as libc::c_float,
    ],
    [
        0.587785f64 as libc::c_float,
        -0.425325f64 as libc::c_float,
        -0.688191f64 as libc::c_float,
    ],
    [
        0.000000f64 as libc::c_float,
        -0.955423f64 as libc::c_float,
        -0.295242f64 as libc::c_float,
    ],
    [
        0.000000f64 as libc::c_float,
        -1.000000f64 as libc::c_float,
        0.000000f64 as libc::c_float,
    ],
    [
        0.262866f64 as libc::c_float,
        -0.951056f64 as libc::c_float,
        -0.162460f64 as libc::c_float,
    ],
    [
        0.000000f64 as libc::c_float,
        -0.850651f64 as libc::c_float,
        0.525731f64 as libc::c_float,
    ],
    [
        0.000000f64 as libc::c_float,
        -0.955423f64 as libc::c_float,
        0.295242f64 as libc::c_float,
    ],
    [
        0.238856f64 as libc::c_float,
        -0.864188f64 as libc::c_float,
        0.442863f64 as libc::c_float,
    ],
    [
        0.262866f64 as libc::c_float,
        -0.951056f64 as libc::c_float,
        0.162460f64 as libc::c_float,
    ],
    [
        0.500000f64 as libc::c_float,
        -0.809017f64 as libc::c_float,
        0.309017f64 as libc::c_float,
    ],
    [
        0.716567f64 as libc::c_float,
        -0.681718f64 as libc::c_float,
        0.147621f64 as libc::c_float,
    ],
    [
        0.525731f64 as libc::c_float,
        -0.850651f64 as libc::c_float,
        0.000000f64 as libc::c_float,
    ],
    [
        -0.238856f64 as libc::c_float,
        -0.864188f64 as libc::c_float,
        -0.442863f64 as libc::c_float,
    ],
    [
        -0.500000f64 as libc::c_float,
        -0.809017f64 as libc::c_float,
        -0.309017f64 as libc::c_float,
    ],
    [
        -0.262866f64 as libc::c_float,
        -0.951056f64 as libc::c_float,
        -0.162460f64 as libc::c_float,
    ],
    [
        -0.850651f64 as libc::c_float,
        -0.525731f64 as libc::c_float,
        0.000000f64 as libc::c_float,
    ],
    [
        -0.716567f64 as libc::c_float,
        -0.681718f64 as libc::c_float,
        -0.147621f64 as libc::c_float,
    ],
    [
        -0.716567f64 as libc::c_float,
        -0.681718f64 as libc::c_float,
        0.147621f64 as libc::c_float,
    ],
    [
        -0.525731f64 as libc::c_float,
        -0.850651f64 as libc::c_float,
        0.000000f64 as libc::c_float,
    ],
    [
        -0.500000f64 as libc::c_float,
        -0.809017f64 as libc::c_float,
        0.309017f64 as libc::c_float,
    ],
    [
        -0.238856f64 as libc::c_float,
        -0.864188f64 as libc::c_float,
        0.442863f64 as libc::c_float,
    ],
    [
        -0.262866f64 as libc::c_float,
        -0.951056f64 as libc::c_float,
        0.162460f64 as libc::c_float,
    ],
    [
        -0.864188f64 as libc::c_float,
        -0.442863f64 as libc::c_float,
        0.238856f64 as libc::c_float,
    ],
    [
        -0.809017f64 as libc::c_float,
        -0.309017f64 as libc::c_float,
        0.500000f64 as libc::c_float,
    ],
    [
        -0.688191f64 as libc::c_float,
        -0.587785f64 as libc::c_float,
        0.425325f64 as libc::c_float,
    ],
    [
        -0.681718f64 as libc::c_float,
        -0.147621f64 as libc::c_float,
        0.716567f64 as libc::c_float,
    ],
    [
        -0.442863f64 as libc::c_float,
        -0.238856f64 as libc::c_float,
        0.864188f64 as libc::c_float,
    ],
    [
        -0.587785f64 as libc::c_float,
        -0.425325f64 as libc::c_float,
        0.688191f64 as libc::c_float,
    ],
    [
        -0.309017f64 as libc::c_float,
        -0.500000f64 as libc::c_float,
        0.809017f64 as libc::c_float,
    ],
    [
        -0.147621f64 as libc::c_float,
        -0.716567f64 as libc::c_float,
        0.681718f64 as libc::c_float,
    ],
    [
        -0.425325f64 as libc::c_float,
        -0.688191f64 as libc::c_float,
        0.587785f64 as libc::c_float,
    ],
    [
        -0.162460f64 as libc::c_float,
        -0.262866f64 as libc::c_float,
        0.951056f64 as libc::c_float,
    ],
    [
        0.442863f64 as libc::c_float,
        -0.238856f64 as libc::c_float,
        0.864188f64 as libc::c_float,
    ],
    [
        0.162460f64 as libc::c_float,
        -0.262866f64 as libc::c_float,
        0.951056f64 as libc::c_float,
    ],
    [
        0.309017f64 as libc::c_float,
        -0.500000f64 as libc::c_float,
        0.809017f64 as libc::c_float,
    ],
    [
        0.147621f64 as libc::c_float,
        -0.716567f64 as libc::c_float,
        0.681718f64 as libc::c_float,
    ],
    [
        0.000000f64 as libc::c_float,
        -0.525731f64 as libc::c_float,
        0.850651f64 as libc::c_float,
    ],
    [
        0.425325f64 as libc::c_float,
        -0.688191f64 as libc::c_float,
        0.587785f64 as libc::c_float,
    ],
    [
        0.587785f64 as libc::c_float,
        -0.425325f64 as libc::c_float,
        0.688191f64 as libc::c_float,
    ],
    [
        0.688191f64 as libc::c_float,
        -0.587785f64 as libc::c_float,
        0.425325f64 as libc::c_float,
    ],
    [
        -0.955423f64 as libc::c_float,
        0.295242f64 as libc::c_float,
        0.000000f64 as libc::c_float,
    ],
    [
        -0.951056f64 as libc::c_float,
        0.162460f64 as libc::c_float,
        0.262866f64 as libc::c_float,
    ],
    [
        -1.000000f64 as libc::c_float,
        0.000000f64 as libc::c_float,
        0.000000f64 as libc::c_float,
    ],
    [
        -0.850651f64 as libc::c_float,
        0.000000f64 as libc::c_float,
        0.525731f64 as libc::c_float,
    ],
    [
        -0.955423f64 as libc::c_float,
        -0.295242f64 as libc::c_float,
        0.000000f64 as libc::c_float,
    ],
    [
        -0.951056f64 as libc::c_float,
        -0.162460f64 as libc::c_float,
        0.262866f64 as libc::c_float,
    ],
    [
        -0.864188f64 as libc::c_float,
        0.442863f64 as libc::c_float,
        -0.238856f64 as libc::c_float,
    ],
    [
        -0.951056f64 as libc::c_float,
        0.162460f64 as libc::c_float,
        -0.262866f64 as libc::c_float,
    ],
    [
        -0.809017f64 as libc::c_float,
        0.309017f64 as libc::c_float,
        -0.500000f64 as libc::c_float,
    ],
    [
        -0.864188f64 as libc::c_float,
        -0.442863f64 as libc::c_float,
        -0.238856f64 as libc::c_float,
    ],
    [
        -0.951056f64 as libc::c_float,
        -0.162460f64 as libc::c_float,
        -0.262866f64 as libc::c_float,
    ],
    [
        -0.809017f64 as libc::c_float,
        -0.309017f64 as libc::c_float,
        -0.500000f64 as libc::c_float,
    ],
    [
        -0.681718f64 as libc::c_float,
        0.147621f64 as libc::c_float,
        -0.716567f64 as libc::c_float,
    ],
    [
        -0.681718f64 as libc::c_float,
        -0.147621f64 as libc::c_float,
        -0.716567f64 as libc::c_float,
    ],
    [
        -0.850651f64 as libc::c_float,
        0.000000f64 as libc::c_float,
        -0.525731f64 as libc::c_float,
    ],
    [
        -0.688191f64 as libc::c_float,
        0.587785f64 as libc::c_float,
        -0.425325f64 as libc::c_float,
    ],
    [
        -0.587785f64 as libc::c_float,
        0.425325f64 as libc::c_float,
        -0.688191f64 as libc::c_float,
    ],
    [
        -0.425325f64 as libc::c_float,
        0.688191f64 as libc::c_float,
        -0.587785f64 as libc::c_float,
    ],
    [
        -0.425325f64 as libc::c_float,
        -0.688191f64 as libc::c_float,
        -0.587785f64 as libc::c_float,
    ],
    [
        -0.587785f64 as libc::c_float,
        -0.425325f64 as libc::c_float,
        -0.688191f64 as libc::c_float,
    ],
    [
        -0.688191f64 as libc::c_float,
        -0.587785f64 as libc::c_float,
        -0.425325f64 as libc::c_float,
    ],
];
#[no_mangle]
pub unsafe extern "C" fn R_AliasCheckFrameBBox(
    mut frame: *mut daliasframe_t,
    mut worldxf: *mut [libc::c_float; 4],
) -> libc::c_ulong {
    let mut aggregate_and_clipcode: libc::c_ulong = !(0 as libc::c_uint)
        as libc::c_ulong;
    let mut aggregate_or_clipcode: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    let mut i: libc::c_int = 0;
    let mut mins: vec3_t = [0.; 3];
    let mut maxs: vec3_t = [0.; 3];
    let mut transformed_min: vec3_t = [0.; 3];
    let mut transformed_max: vec3_t = [0.; 3];
    let mut zclipped: qboolean = false_0;
    let mut zfullyclipped: qboolean = true_0;
    let mut minz: libc::c_float = 9999.0f32;
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        mins[i as usize] = (*frame).translate[i as usize];
        maxs[i
            as usize] = mins[i as usize]
            + (*frame).scale[i as usize] * 255 as libc::c_int as libc::c_float;
        i += 1;
    }
    R_AliasTransformVector(
        mins.as_mut_ptr(),
        transformed_min.as_mut_ptr(),
        aliastransform.as_mut_ptr(),
    );
    R_AliasTransformVector(
        maxs.as_mut_ptr(),
        transformed_max.as_mut_ptr(),
        aliastransform.as_mut_ptr(),
    );
    if transformed_min[2 as libc::c_int as usize] >= 4 as libc::c_int as libc::c_float {
        zfullyclipped = false_0;
    }
    if transformed_max[2 as libc::c_int as usize] >= 4 as libc::c_int as libc::c_float {
        zfullyclipped = false_0;
    }
    if zfullyclipped as u64 != 0 {
        return 8 as libc::c_int as libc::c_ulong;
    }
    if zclipped as u64 != 0 {
        return (1 as libc::c_int | 2 as libc::c_int) as libc::c_ulong;
    }
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        let mut j: libc::c_int = 0;
        let mut tmp: vec3_t = [0.; 3];
        let mut transformed: vec3_t = [0.; 3];
        let mut clipcode: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
        if i & 1 as libc::c_int != 0 {
            tmp[0 as libc::c_int as usize] = mins[0 as libc::c_int as usize];
        } else {
            tmp[0 as libc::c_int as usize] = maxs[0 as libc::c_int as usize];
        }
        if i & 2 as libc::c_int != 0 {
            tmp[1 as libc::c_int as usize] = mins[1 as libc::c_int as usize];
        } else {
            tmp[1 as libc::c_int as usize] = maxs[1 as libc::c_int as usize];
        }
        if i & 4 as libc::c_int != 0 {
            tmp[2 as libc::c_int as usize] = mins[2 as libc::c_int as usize];
        } else {
            tmp[2 as libc::c_int as usize] = maxs[2 as libc::c_int as usize];
        }
        R_AliasTransformVector(tmp.as_mut_ptr(), transformed.as_mut_ptr(), worldxf);
        j = 0 as libc::c_int;
        while j < 4 as libc::c_int {
            let mut dp: libc::c_float = transformed[0 as libc::c_int as usize]
                * view_clipplanes[j as usize].normal[0 as libc::c_int as usize]
                + transformed[1 as libc::c_int as usize]
                    * view_clipplanes[j as usize].normal[1 as libc::c_int as usize]
                + transformed[2 as libc::c_int as usize]
                    * view_clipplanes[j as usize].normal[2 as libc::c_int as usize];
            if dp - view_clipplanes[j as usize].dist < 0.0f32 {
                clipcode |= ((1 as libc::c_int) << j) as libc::c_ulong;
            }
            j += 1;
        }
        aggregate_and_clipcode &= clipcode;
        aggregate_or_clipcode |= clipcode;
        i += 1;
    }
    if aggregate_and_clipcode != 0 {
        return 8 as libc::c_int as libc::c_ulong;
    }
    if aggregate_or_clipcode == 0 {
        return 0 as libc::c_int as libc::c_ulong;
    }
    return 1 as libc::c_int as libc::c_ulong;
}
#[no_mangle]
pub unsafe extern "C" fn R_AliasCheckBBox() -> qboolean {
    let mut ccodes: [libc::c_ulong; 2] = [
        0 as libc::c_int as libc::c_ulong,
        0 as libc::c_int as libc::c_ulong,
    ];
    ccodes[0 as libc::c_int
        as usize] = R_AliasCheckFrameBBox(r_thisframe, aliasworldtransform.as_mut_ptr());
    if (*currententity).backlerp == 0 as libc::c_int as libc::c_float {
        if ccodes[0 as libc::c_int as usize] == 0 as libc::c_int as libc::c_ulong {
            return false_0
        } else if ccodes[0 as libc::c_int as usize] & 8 as libc::c_int as libc::c_ulong
            != 0
        {
            return 8 as qboolean
        } else {
            return (ccodes[0 as libc::c_int as usize]
                & !(8 as libc::c_int) as libc::c_ulong) as qboolean
        }
    }
    ccodes[1 as libc::c_int
        as usize] = R_AliasCheckFrameBBox(
        r_lastframe,
        aliasoldworldtransform.as_mut_ptr(),
    );
    if ccodes[0 as libc::c_int as usize] | ccodes[1 as libc::c_int as usize]
        == 0 as libc::c_int as libc::c_ulong
    {
        return false_0
    } else if ccodes[0 as libc::c_int as usize] & ccodes[1 as libc::c_int as usize]
        & 8 as libc::c_int as libc::c_ulong != 0
    {
        return 8 as qboolean
    } else {
        return ((ccodes[0 as libc::c_int as usize] | ccodes[1 as libc::c_int as usize])
            & !(8 as libc::c_int) as libc::c_ulong) as qboolean
    };
}
#[no_mangle]
pub unsafe extern "C" fn R_AliasTransformVector(
    mut in_0: *mut vec_t,
    mut out: *mut vec_t,
    mut xf: *mut [libc::c_float; 4],
) {
    *out
        .offset(
            0 as libc::c_int as isize,
        ) = *in_0.offset(0 as libc::c_int as isize)
        * (*xf.offset(0 as libc::c_int as isize))[0 as libc::c_int as usize]
        + *in_0.offset(1 as libc::c_int as isize)
            * (*xf.offset(0 as libc::c_int as isize))[1 as libc::c_int as usize]
        + *in_0.offset(2 as libc::c_int as isize)
            * (*xf.offset(0 as libc::c_int as isize))[2 as libc::c_int as usize]
        + (*xf.offset(0 as libc::c_int as isize))[3 as libc::c_int as usize];
    *out
        .offset(
            1 as libc::c_int as isize,
        ) = *in_0.offset(0 as libc::c_int as isize)
        * (*xf.offset(1 as libc::c_int as isize))[0 as libc::c_int as usize]
        + *in_0.offset(1 as libc::c_int as isize)
            * (*xf.offset(1 as libc::c_int as isize))[1 as libc::c_int as usize]
        + *in_0.offset(2 as libc::c_int as isize)
            * (*xf.offset(1 as libc::c_int as isize))[2 as libc::c_int as usize]
        + (*xf.offset(1 as libc::c_int as isize))[3 as libc::c_int as usize];
    *out
        .offset(
            2 as libc::c_int as isize,
        ) = *in_0.offset(0 as libc::c_int as isize)
        * (*xf.offset(2 as libc::c_int as isize))[0 as libc::c_int as usize]
        + *in_0.offset(1 as libc::c_int as isize)
            * (*xf.offset(2 as libc::c_int as isize))[1 as libc::c_int as usize]
        + *in_0.offset(2 as libc::c_int as isize)
            * (*xf.offset(2 as libc::c_int as isize))[2 as libc::c_int as usize]
        + (*xf.offset(2 as libc::c_int as isize))[3 as libc::c_int as usize];
}
#[no_mangle]
pub static mut aliasbatchedtransformdata: aliasbatchedtransformdata_t = aliasbatchedtransformdata_t {
    num_points: 0,
    last_verts: 0 as *const dtrivertx_t as *mut dtrivertx_t,
    this_verts: 0 as *const dtrivertx_t as *mut dtrivertx_t,
    dest_verts: 0 as *const finalvert_t as *mut finalvert_t,
};
#[no_mangle]
pub unsafe extern "C" fn R_AliasPreparePoints() {
    let mut i: libc::c_int = 0;
    let mut pstverts: *mut dstvert_t = 0 as *mut dstvert_t;
    let mut ptri: *mut dtriangle_t = 0 as *mut dtriangle_t;
    let mut pfv: [*mut finalvert_t; 3] = [0 as *mut finalvert_t; 3];
    let mut finalverts: [finalvert_t; 2003] = [finalvert_t {
        u: 0,
        v: 0,
        s: 0,
        t: 0,
        l: 0,
        zi: 0,
        flags: 0,
        xyz: [0.; 3],
    }; 2003];
    let mut pfinalverts: *mut finalvert_t = 0 as *mut finalvert_t;
    iractive = (r_newrefdef.rdflags & 4 as libc::c_int != 0
        && (*currententity).flags & 0x8000 as libc::c_int != 0) as libc::c_int as byte;
    pfinalverts = (&mut *finalverts.as_mut_ptr().offset(0 as libc::c_int as isize)
        as *mut finalvert_t as libc::c_long + 32 as libc::c_int as libc::c_long
        - 1 as libc::c_int as libc::c_long
        & !(32 as libc::c_int - 1 as libc::c_int) as libc::c_long) as *mut finalvert_t;
    aliasbatchedtransformdata.num_points = (*s_pmdl).num_xyz;
    aliasbatchedtransformdata.last_verts = ((*r_lastframe).verts).as_mut_ptr();
    aliasbatchedtransformdata.this_verts = ((*r_thisframe).verts).as_mut_ptr();
    aliasbatchedtransformdata.dest_verts = pfinalverts;
    R_AliasTransformFinalVerts(
        aliasbatchedtransformdata.num_points,
        aliasbatchedtransformdata.dest_verts,
        aliasbatchedtransformdata.last_verts,
        aliasbatchedtransformdata.this_verts,
    );
    pstverts = (s_pmdl as *mut byte).offset((*s_pmdl).ofs_st as isize) as *mut dstvert_t;
    ptri = (s_pmdl as *mut byte).offset((*s_pmdl).ofs_tris as isize) as *mut dtriangle_t;
    if (*currententity).flags & 4 as libc::c_int != 0 && (*r_lefthand).value == 1.0f32 {
        i = 0 as libc::c_int;
        while i < (*s_pmdl).num_tris {
            pfv[0 as libc::c_int
                as usize] = &mut *pfinalverts
                .offset(
                    *((*ptri).index_xyz).as_mut_ptr().offset(0 as libc::c_int as isize)
                        as isize,
                ) as *mut finalvert_t;
            pfv[1 as libc::c_int
                as usize] = &mut *pfinalverts
                .offset(
                    *((*ptri).index_xyz).as_mut_ptr().offset(1 as libc::c_int as isize)
                        as isize,
                ) as *mut finalvert_t;
            pfv[2 as libc::c_int
                as usize] = &mut *pfinalverts
                .offset(
                    *((*ptri).index_xyz).as_mut_ptr().offset(2 as libc::c_int as isize)
                        as isize,
                ) as *mut finalvert_t;
            if !((*pfv[0 as libc::c_int as usize]).flags
                & (*pfv[1 as libc::c_int as usize]).flags
                & (*pfv[2 as libc::c_int as usize]).flags != 0)
            {
                (*pfv[0 as libc::c_int as usize])
                    .s = ((*pstverts
                    .offset((*ptri).index_st[0 as libc::c_int as usize] as isize))
                    .s as libc::c_int) << 16 as libc::c_int;
                (*pfv[0 as libc::c_int as usize])
                    .t = ((*pstverts
                    .offset((*ptri).index_st[0 as libc::c_int as usize] as isize))
                    .t as libc::c_int) << 16 as libc::c_int;
                (*pfv[1 as libc::c_int as usize])
                    .s = ((*pstverts
                    .offset((*ptri).index_st[1 as libc::c_int as usize] as isize))
                    .s as libc::c_int) << 16 as libc::c_int;
                (*pfv[1 as libc::c_int as usize])
                    .t = ((*pstverts
                    .offset((*ptri).index_st[1 as libc::c_int as usize] as isize))
                    .t as libc::c_int) << 16 as libc::c_int;
                (*pfv[2 as libc::c_int as usize])
                    .s = ((*pstverts
                    .offset((*ptri).index_st[2 as libc::c_int as usize] as isize))
                    .s as libc::c_int) << 16 as libc::c_int;
                (*pfv[2 as libc::c_int as usize])
                    .t = ((*pstverts
                    .offset((*ptri).index_st[2 as libc::c_int as usize] as isize))
                    .t as libc::c_int) << 16 as libc::c_int;
                if (*pfv[0 as libc::c_int as usize]).flags
                    | (*pfv[1 as libc::c_int as usize]).flags
                    | (*pfv[2 as libc::c_int as usize]).flags == 0
                {
                    aliastriangleparms.a = pfv[2 as libc::c_int as usize];
                    aliastriangleparms.b = pfv[1 as libc::c_int as usize];
                    aliastriangleparms.c = pfv[0 as libc::c_int as usize];
                    R_DrawTriangle();
                } else {
                    R_AliasClipTriangle(
                        pfv[2 as libc::c_int as usize],
                        pfv[1 as libc::c_int as usize],
                        pfv[0 as libc::c_int as usize],
                    );
                }
            }
            i += 1;
            ptri = ptri.offset(1);
        }
    } else {
        i = 0 as libc::c_int;
        while i < (*s_pmdl).num_tris {
            pfv[0 as libc::c_int
                as usize] = &mut *pfinalverts
                .offset(
                    *((*ptri).index_xyz).as_mut_ptr().offset(0 as libc::c_int as isize)
                        as isize,
                ) as *mut finalvert_t;
            pfv[1 as libc::c_int
                as usize] = &mut *pfinalverts
                .offset(
                    *((*ptri).index_xyz).as_mut_ptr().offset(1 as libc::c_int as isize)
                        as isize,
                ) as *mut finalvert_t;
            pfv[2 as libc::c_int
                as usize] = &mut *pfinalverts
                .offset(
                    *((*ptri).index_xyz).as_mut_ptr().offset(2 as libc::c_int as isize)
                        as isize,
                ) as *mut finalvert_t;
            if !((*pfv[0 as libc::c_int as usize]).flags
                & (*pfv[1 as libc::c_int as usize]).flags
                & (*pfv[2 as libc::c_int as usize]).flags != 0)
            {
                (*pfv[0 as libc::c_int as usize])
                    .s = ((*pstverts
                    .offset((*ptri).index_st[0 as libc::c_int as usize] as isize))
                    .s as libc::c_int) << 16 as libc::c_int;
                (*pfv[0 as libc::c_int as usize])
                    .t = ((*pstverts
                    .offset((*ptri).index_st[0 as libc::c_int as usize] as isize))
                    .t as libc::c_int) << 16 as libc::c_int;
                (*pfv[1 as libc::c_int as usize])
                    .s = ((*pstverts
                    .offset((*ptri).index_st[1 as libc::c_int as usize] as isize))
                    .s as libc::c_int) << 16 as libc::c_int;
                (*pfv[1 as libc::c_int as usize])
                    .t = ((*pstverts
                    .offset((*ptri).index_st[1 as libc::c_int as usize] as isize))
                    .t as libc::c_int) << 16 as libc::c_int;
                (*pfv[2 as libc::c_int as usize])
                    .s = ((*pstverts
                    .offset((*ptri).index_st[2 as libc::c_int as usize] as isize))
                    .s as libc::c_int) << 16 as libc::c_int;
                (*pfv[2 as libc::c_int as usize])
                    .t = ((*pstverts
                    .offset((*ptri).index_st[2 as libc::c_int as usize] as isize))
                    .t as libc::c_int) << 16 as libc::c_int;
                if (*pfv[0 as libc::c_int as usize]).flags
                    | (*pfv[1 as libc::c_int as usize]).flags
                    | (*pfv[2 as libc::c_int as usize]).flags == 0
                {
                    aliastriangleparms.a = pfv[0 as libc::c_int as usize];
                    aliastriangleparms.b = pfv[1 as libc::c_int as usize];
                    aliastriangleparms.c = pfv[2 as libc::c_int as usize];
                    R_DrawTriangle();
                } else {
                    R_AliasClipTriangle(
                        pfv[0 as libc::c_int as usize],
                        pfv[1 as libc::c_int as usize],
                        pfv[2 as libc::c_int as usize],
                    );
                }
            }
            i += 1;
            ptri = ptri.offset(1);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn R_AliasSetUpTransform() {
    let mut i: libc::c_int = 0;
    static mut viewmatrix: [[libc::c_float; 4]; 3] = [[0.; 4]; 3];
    let mut angles: vec3_t = [0.; 3];
    angles[2 as libc::c_int
        as usize] = (*currententity).angles[2 as libc::c_int as usize];
    angles[0 as libc::c_int
        as usize] = (*currententity).angles[0 as libc::c_int as usize];
    angles[1 as libc::c_int
        as usize] = (*currententity).angles[1 as libc::c_int as usize];
    AngleVectors(
        angles.as_mut_ptr(),
        s_alias_forward.as_mut_ptr(),
        s_alias_right.as_mut_ptr(),
        s_alias_up.as_mut_ptr(),
    );
    memset(
        aliasworldtransform.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<[[libc::c_float; 4]; 3]>() as libc::c_ulong,
    );
    memset(
        aliasoldworldtransform.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<[[libc::c_float; 4]; 3]>() as libc::c_ulong,
    );
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        aliasworldtransform[i
            as usize][0 as libc::c_int as usize] = s_alias_forward[i as usize];
        aliasoldworldtransform[i
            as usize][0 as libc::c_int
            as usize] = aliasworldtransform[i as usize][0 as libc::c_int as usize];
        aliasworldtransform[i
            as usize][1 as libc::c_int as usize] = -s_alias_right[i as usize];
        aliasoldworldtransform[i
            as usize][0 as libc::c_int
            as usize] = aliasworldtransform[i as usize][1 as libc::c_int as usize];
        aliasworldtransform[i
            as usize][2 as libc::c_int as usize] = s_alias_up[i as usize];
        aliasoldworldtransform[i
            as usize][0 as libc::c_int
            as usize] = aliasworldtransform[i as usize][2 as libc::c_int as usize];
        i += 1;
    }
    aliasworldtransform[0 as libc::c_int
        as usize][3 as libc::c_int
        as usize] = (*currententity).origin[0 as libc::c_int as usize]
        - r_origin[0 as libc::c_int as usize];
    aliasworldtransform[1 as libc::c_int
        as usize][3 as libc::c_int
        as usize] = (*currententity).origin[1 as libc::c_int as usize]
        - r_origin[1 as libc::c_int as usize];
    aliasworldtransform[2 as libc::c_int
        as usize][3 as libc::c_int
        as usize] = (*currententity).origin[2 as libc::c_int as usize]
        - r_origin[2 as libc::c_int as usize];
    aliasoldworldtransform[0 as libc::c_int
        as usize][3 as libc::c_int
        as usize] = (*currententity).oldorigin[0 as libc::c_int as usize]
        - r_origin[0 as libc::c_int as usize];
    aliasoldworldtransform[1 as libc::c_int
        as usize][3 as libc::c_int
        as usize] = (*currententity).oldorigin[1 as libc::c_int as usize]
        - r_origin[1 as libc::c_int as usize];
    aliasoldworldtransform[2 as libc::c_int
        as usize][3 as libc::c_int
        as usize] = (*currententity).oldorigin[2 as libc::c_int as usize]
        - r_origin[2 as libc::c_int as usize];
    viewmatrix[0 as libc::c_int
        as usize][0 as libc::c_int as usize] = vright[0 as libc::c_int as usize];
    viewmatrix[0 as libc::c_int
        as usize][1 as libc::c_int as usize] = vright[1 as libc::c_int as usize];
    viewmatrix[0 as libc::c_int
        as usize][2 as libc::c_int as usize] = vright[2 as libc::c_int as usize];
    viewmatrix[1 as libc::c_int
        as usize][0 as libc::c_int as usize] = vup[0 as libc::c_int as usize];
    viewmatrix[1 as libc::c_int
        as usize][1 as libc::c_int as usize] = vup[1 as libc::c_int as usize];
    viewmatrix[1 as libc::c_int
        as usize][2 as libc::c_int as usize] = vup[2 as libc::c_int as usize];
    VectorInverse((viewmatrix[1 as libc::c_int as usize]).as_mut_ptr());
    viewmatrix[2 as libc::c_int
        as usize][0 as libc::c_int as usize] = vpn[0 as libc::c_int as usize];
    viewmatrix[2 as libc::c_int
        as usize][1 as libc::c_int as usize] = vpn[1 as libc::c_int as usize];
    viewmatrix[2 as libc::c_int
        as usize][2 as libc::c_int as usize] = vpn[2 as libc::c_int as usize];
    viewmatrix[0 as libc::c_int
        as usize][3 as libc::c_int as usize] = 0 as libc::c_int as libc::c_float;
    viewmatrix[1 as libc::c_int
        as usize][3 as libc::c_int as usize] = 0 as libc::c_int as libc::c_float;
    viewmatrix[2 as libc::c_int
        as usize][3 as libc::c_int as usize] = 0 as libc::c_int as libc::c_float;
    R_ConcatTransforms(
        viewmatrix.as_mut_ptr(),
        aliasworldtransform.as_mut_ptr(),
        aliastransform.as_mut_ptr(),
    );
    aliasworldtransform[0 as libc::c_int
        as usize][3 as libc::c_int
        as usize] = (*currententity).origin[0 as libc::c_int as usize];
    aliasworldtransform[1 as libc::c_int
        as usize][3 as libc::c_int
        as usize] = (*currententity).origin[1 as libc::c_int as usize];
    aliasworldtransform[2 as libc::c_int
        as usize][3 as libc::c_int
        as usize] = (*currententity).origin[2 as libc::c_int as usize];
    aliasoldworldtransform[0 as libc::c_int
        as usize][3 as libc::c_int
        as usize] = (*currententity).oldorigin[0 as libc::c_int as usize];
    aliasoldworldtransform[1 as libc::c_int
        as usize][3 as libc::c_int
        as usize] = (*currententity).oldorigin[1 as libc::c_int as usize];
    aliasoldworldtransform[2 as libc::c_int
        as usize][3 as libc::c_int
        as usize] = (*currententity).oldorigin[2 as libc::c_int as usize];
}
#[no_mangle]
pub unsafe extern "C" fn R_AliasTransformFinalVerts(
    mut numpoints: libc::c_int,
    mut fv: *mut finalvert_t,
    mut oldv: *mut dtrivertx_t,
    mut newv: *mut dtrivertx_t,
) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < numpoints {
        let mut temp: libc::c_int = 0;
        let mut lightcos: libc::c_float = 0.;
        let mut plightnormal: *mut libc::c_float = 0 as *mut libc::c_float;
        let mut lerped_vert: vec3_t = [0.; 3];
        lerped_vert[0 as libc::c_int
            as usize] = r_lerp_move[0 as libc::c_int as usize]
            + (*oldv).v[0 as libc::c_int as usize] as libc::c_int as libc::c_float
                * r_lerp_backv[0 as libc::c_int as usize]
            + (*newv).v[0 as libc::c_int as usize] as libc::c_int as libc::c_float
                * r_lerp_frontv[0 as libc::c_int as usize];
        lerped_vert[1 as libc::c_int
            as usize] = r_lerp_move[1 as libc::c_int as usize]
            + (*oldv).v[1 as libc::c_int as usize] as libc::c_int as libc::c_float
                * r_lerp_backv[1 as libc::c_int as usize]
            + (*newv).v[1 as libc::c_int as usize] as libc::c_int as libc::c_float
                * r_lerp_frontv[1 as libc::c_int as usize];
        lerped_vert[2 as libc::c_int
            as usize] = r_lerp_move[2 as libc::c_int as usize]
            + (*oldv).v[2 as libc::c_int as usize] as libc::c_int as libc::c_float
                * r_lerp_backv[2 as libc::c_int as usize]
            + (*newv).v[2 as libc::c_int as usize] as libc::c_int as libc::c_float
                * r_lerp_frontv[2 as libc::c_int as usize];
        plightnormal = (r_avertexnormals[(*newv).lightnormalindex as usize])
            .as_mut_ptr();
        if (*currententity).flags
            & (1024 as libc::c_int | 2048 as libc::c_int | 4096 as libc::c_int
                | 0x10000 as libc::c_int | 0x20000 as libc::c_int) != 0
        {
            lerped_vert[0 as libc::c_int as usize]
                += *plightnormal.offset(0 as libc::c_int as isize) * 4.0f32;
            lerped_vert[1 as libc::c_int as usize]
                += *plightnormal.offset(1 as libc::c_int as isize) * 4.0f32;
            lerped_vert[2 as libc::c_int as usize]
                += *plightnormal.offset(2 as libc::c_int as isize) * 4.0f32;
        }
        (*fv)
            .xyz[0 as libc::c_int
            as usize] = lerped_vert[0 as libc::c_int as usize]
            * aliastransform[0 as libc::c_int as usize][0 as libc::c_int as usize]
            + lerped_vert[1 as libc::c_int as usize]
                * aliastransform[0 as libc::c_int as usize][1 as libc::c_int as usize]
            + lerped_vert[2 as libc::c_int as usize]
                * aliastransform[0 as libc::c_int as usize][2 as libc::c_int as usize]
            + aliastransform[0 as libc::c_int as usize][3 as libc::c_int as usize];
        (*fv)
            .xyz[1 as libc::c_int
            as usize] = lerped_vert[0 as libc::c_int as usize]
            * aliastransform[1 as libc::c_int as usize][0 as libc::c_int as usize]
            + lerped_vert[1 as libc::c_int as usize]
                * aliastransform[1 as libc::c_int as usize][1 as libc::c_int as usize]
            + lerped_vert[2 as libc::c_int as usize]
                * aliastransform[1 as libc::c_int as usize][2 as libc::c_int as usize]
            + aliastransform[1 as libc::c_int as usize][3 as libc::c_int as usize];
        (*fv)
            .xyz[2 as libc::c_int
            as usize] = lerped_vert[0 as libc::c_int as usize]
            * aliastransform[2 as libc::c_int as usize][0 as libc::c_int as usize]
            + lerped_vert[1 as libc::c_int as usize]
                * aliastransform[2 as libc::c_int as usize][1 as libc::c_int as usize]
            + lerped_vert[2 as libc::c_int as usize]
                * aliastransform[2 as libc::c_int as usize][2 as libc::c_int as usize]
            + aliastransform[2 as libc::c_int as usize][3 as libc::c_int as usize];
        (*fv).flags = 0 as libc::c_int;
        lightcos = *plightnormal.offset(0 as libc::c_int as isize)
            * r_plightvec[0 as libc::c_int as usize]
            + *plightnormal.offset(1 as libc::c_int as isize)
                * r_plightvec[1 as libc::c_int as usize]
            + *plightnormal.offset(2 as libc::c_int as isize)
                * r_plightvec[2 as libc::c_int as usize];
        temp = r_ambientlight;
        if lightcos < 0 as libc::c_int as libc::c_float {
            temp += (r_shadelight * lightcos) as libc::c_int;
            if temp < 0 as libc::c_int {
                temp = 0 as libc::c_int;
            }
        }
        (*fv).l = temp;
        if (*fv).xyz[2 as libc::c_int as usize] < 4 as libc::c_int as libc::c_float {
            (*fv).flags |= 0x10 as libc::c_int;
        } else {
            R_AliasProjectAndClipTestFinalVert(fv);
        }
        i += 1;
        fv = fv.offset(1);
        oldv = oldv.offset(1);
        newv = newv.offset(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn R_AliasProjectAndClipTestFinalVert(mut fv: *mut finalvert_t) {
    let mut zi: libc::c_float = 0.;
    let mut x: libc::c_float = 0.;
    let mut y: libc::c_float = 0.;
    let mut z: libc::c_float = 0.;
    x = (*fv).xyz[0 as libc::c_int as usize];
    y = (*fv).xyz[1 as libc::c_int as usize];
    z = (*fv).xyz[2 as libc::c_int as usize];
    zi = (1.0f64 / z as libc::c_double) as libc::c_float;
    (*fv).zi = (zi * s_ziscale) as libc::c_int;
    (*fv).u = (x * aliasxscale * zi + aliasxcenter) as libc::c_int;
    (*fv).v = (y * aliasyscale * zi + aliasycenter) as libc::c_int;
    if (*fv).u < r_refdef.aliasvrect.x {
        (*fv).flags |= 0x1 as libc::c_int;
    }
    if (*fv).v < r_refdef.aliasvrect.y {
        (*fv).flags |= 0x2 as libc::c_int;
    }
    if (*fv).u > r_refdef.aliasvrectright {
        (*fv).flags |= 0x4 as libc::c_int;
    }
    if (*fv).v > r_refdef.aliasvrectbottom {
        (*fv).flags |= 0x8 as libc::c_int;
    }
}
unsafe extern "C" fn R_AliasSetupSkin() -> qboolean {
    let mut skinnum: libc::c_int = 0;
    let mut pskindesc: *mut image_t = 0 as *mut image_t;
    if !((*currententity).skin).is_null() {
        pskindesc = (*currententity).skin;
    } else {
        skinnum = (*currententity).skinnum;
        if skinnum >= (*s_pmdl).num_skins || skinnum < 0 as libc::c_int {
            (ri.Con_Printf)
                .expect(
                    "non-null function pointer",
                )(
                0 as libc::c_int,
                b"R_AliasSetupSkin %s: no such skin # %d\n\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ((*currentmodel).name).as_mut_ptr(),
                skinnum,
            );
            skinnum = 0 as libc::c_int;
        }
        pskindesc = (*currentmodel).skins[skinnum as usize];
    }
    if pskindesc.is_null() {
        return false_0;
    }
    r_affinetridesc
        .pskin = (*pskindesc).pixels[0 as libc::c_int as usize] as *mut libc::c_void;
    r_affinetridesc.skinwidth = (*pskindesc).width;
    r_affinetridesc.skinheight = (*pskindesc).height;
    R_PolysetUpdateTables();
    return true_0;
}
#[no_mangle]
pub unsafe extern "C" fn R_AliasSetupLighting() {
    let mut lighting: alight_t = alight_t {
        ambientlight: 0,
        shadelight: 0,
        plightvec: 0 as *mut libc::c_float,
    };
    let mut lightvec: [libc::c_float; 3] = [
        -(1 as libc::c_int) as libc::c_float,
        0 as libc::c_int as libc::c_float,
        0 as libc::c_int as libc::c_float,
    ];
    let mut light: vec3_t = [0.; 3];
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    if (*currententity).flags & 8 as libc::c_int != 0 {
        i = 0 as libc::c_int;
        while i < 3 as libc::c_int {
            light[i as usize] = 1.0f64 as vec_t;
            i += 1;
        }
    } else {
        R_LightPoint(((*currententity).origin).as_mut_ptr(), light.as_mut_ptr());
    }
    if (*currententity).flags & 4 as libc::c_int != 0 {
        (*r_lightlevel)
            .value = (150.0f64 * light[0 as libc::c_int as usize] as libc::c_double)
            as libc::c_float;
    }
    if (*currententity).flags & 1 as libc::c_int != 0 {
        i = 0 as libc::c_int;
        while i < 3 as libc::c_int {
            if (light[i as usize] as libc::c_double) < 0.1f64 {
                light[i as usize] = 0.1f64 as vec_t;
            }
            i += 1;
        }
    }
    if (*currententity).flags & 512 as libc::c_int != 0 {
        let mut scale: libc::c_float = 0.;
        let mut min: libc::c_float = 0.;
        scale = (0.1f64
            * sin(
                (r_newrefdef.time * 7 as libc::c_int as libc::c_float) as libc::c_double,
            )) as libc::c_float;
        i = 0 as libc::c_int;
        while i < 3 as libc::c_int {
            min = (light[i as usize] as libc::c_double * 0.8f64) as libc::c_float;
            light[i as usize] += scale;
            if light[i as usize] < min {
                light[i as usize] = min;
            }
            i += 1;
        }
    }
    j = ((light[0 as libc::c_int as usize] + light[1 as libc::c_int as usize]
        + light[2 as libc::c_int as usize]) as libc::c_double * 0.3333f64
        * 255 as libc::c_int as libc::c_double) as libc::c_int;
    lighting.ambientlight = j;
    lighting.shadelight = j;
    lighting.plightvec = lightvec.as_mut_ptr();
    if lighting.ambientlight > 128 as libc::c_int {
        lighting.ambientlight = 128 as libc::c_int;
    }
    if lighting.ambientlight + lighting.shadelight > 192 as libc::c_int {
        lighting.shadelight = 192 as libc::c_int - lighting.ambientlight;
    }
    r_ambientlight = lighting.ambientlight;
    if r_ambientlight < 5 as libc::c_int {
        r_ambientlight = 5 as libc::c_int;
    }
    r_ambientlight = 255 as libc::c_int - r_ambientlight << 6 as libc::c_int;
    if r_ambientlight < 5 as libc::c_int {
        r_ambientlight = 5 as libc::c_int;
    }
    r_shadelight = lighting.shadelight as libc::c_float;
    if r_shadelight < 0 as libc::c_int as libc::c_float {
        r_shadelight = 0 as libc::c_int as libc::c_float;
    }
    r_shadelight *= ((1 as libc::c_int) << 6 as libc::c_int) as libc::c_float;
    r_plightvec[0 as libc::c_int
        as usize] = *(lighting.plightvec).offset(0 as libc::c_int as isize)
        * s_alias_forward[0 as libc::c_int as usize]
        + *(lighting.plightvec).offset(1 as libc::c_int as isize)
            * s_alias_forward[1 as libc::c_int as usize]
        + *(lighting.plightvec).offset(2 as libc::c_int as isize)
            * s_alias_forward[2 as libc::c_int as usize];
    r_plightvec[1 as libc::c_int
        as usize] = -(*(lighting.plightvec).offset(0 as libc::c_int as isize)
        * s_alias_right[0 as libc::c_int as usize]
        + *(lighting.plightvec).offset(1 as libc::c_int as isize)
            * s_alias_right[1 as libc::c_int as usize]
        + *(lighting.plightvec).offset(2 as libc::c_int as isize)
            * s_alias_right[2 as libc::c_int as usize]);
    r_plightvec[2 as libc::c_int
        as usize] = *(lighting.plightvec).offset(0 as libc::c_int as isize)
        * s_alias_up[0 as libc::c_int as usize]
        + *(lighting.plightvec).offset(1 as libc::c_int as isize)
            * s_alias_up[1 as libc::c_int as usize]
        + *(lighting.plightvec).offset(2 as libc::c_int as isize)
            * s_alias_up[2 as libc::c_int as usize];
}
#[no_mangle]
pub unsafe extern "C" fn R_AliasSetupFrames(mut pmdl: *mut dmdl_t) {
    let mut thisframe: libc::c_int = (*currententity).frame;
    let mut lastframe: libc::c_int = (*currententity).oldframe;
    if thisframe >= (*pmdl).num_frames || thisframe < 0 as libc::c_int {
        (ri.Con_Printf)
            .expect(
                "non-null function pointer",
            )(
            0 as libc::c_int,
            b"R_AliasSetupFrames %s: no such thisframe %d\n\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            ((*currentmodel).name).as_mut_ptr(),
            thisframe,
        );
        thisframe = 0 as libc::c_int;
    }
    if lastframe >= (*pmdl).num_frames || lastframe < 0 as libc::c_int {
        (ri.Con_Printf)
            .expect(
                "non-null function pointer",
            )(
            0 as libc::c_int,
            b"R_AliasSetupFrames %s: no such lastframe %d\n\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            ((*currentmodel).name).as_mut_ptr(),
            lastframe,
        );
        lastframe = 0 as libc::c_int;
    }
    r_thisframe = (pmdl as *mut byte)
        .offset((*pmdl).ofs_frames as isize)
        .offset((thisframe * (*pmdl).framesize) as isize) as *mut daliasframe_t;
    r_lastframe = (pmdl as *mut byte)
        .offset((*pmdl).ofs_frames as isize)
        .offset((lastframe * (*pmdl).framesize) as isize) as *mut daliasframe_t;
}
#[no_mangle]
pub unsafe extern "C" fn R_AliasSetUpLerpData(
    mut pmdl: *mut dmdl_t,
    mut backlerp: libc::c_float,
) {
    let mut frontlerp: libc::c_float = 0.;
    let mut translation: vec3_t = [0.; 3];
    let mut vectors: [vec3_t; 3] = [[0.; 3]; 3];
    let mut i: libc::c_int = 0;
    frontlerp = 1.0f32 - backlerp;
    AngleVectors(
        ((*currententity).angles).as_mut_ptr(),
        (vectors[0 as libc::c_int as usize]).as_mut_ptr(),
        (vectors[1 as libc::c_int as usize]).as_mut_ptr(),
        (vectors[2 as libc::c_int as usize]).as_mut_ptr(),
    );
    translation[0 as libc::c_int
        as usize] = (*currententity).oldorigin[0 as libc::c_int as usize]
        - (*currententity).origin[0 as libc::c_int as usize];
    translation[1 as libc::c_int
        as usize] = (*currententity).oldorigin[1 as libc::c_int as usize]
        - (*currententity).origin[1 as libc::c_int as usize];
    translation[2 as libc::c_int
        as usize] = (*currententity).oldorigin[2 as libc::c_int as usize]
        - (*currententity).origin[2 as libc::c_int as usize];
    r_lerp_move[0 as libc::c_int
        as usize] = translation[0 as libc::c_int as usize]
        * vectors[0 as libc::c_int as usize][0 as libc::c_int as usize]
        + translation[1 as libc::c_int as usize]
            * vectors[0 as libc::c_int as usize][1 as libc::c_int as usize]
        + translation[2 as libc::c_int as usize]
            * vectors[0 as libc::c_int as usize][2 as libc::c_int as usize];
    r_lerp_move[1 as libc::c_int
        as usize] = -(translation[0 as libc::c_int as usize]
        * vectors[1 as libc::c_int as usize][0 as libc::c_int as usize]
        + translation[1 as libc::c_int as usize]
            * vectors[1 as libc::c_int as usize][1 as libc::c_int as usize]
        + translation[2 as libc::c_int as usize]
            * vectors[1 as libc::c_int as usize][2 as libc::c_int as usize]);
    r_lerp_move[2 as libc::c_int
        as usize] = translation[0 as libc::c_int as usize]
        * vectors[2 as libc::c_int as usize][0 as libc::c_int as usize]
        + translation[1 as libc::c_int as usize]
            * vectors[2 as libc::c_int as usize][1 as libc::c_int as usize]
        + translation[2 as libc::c_int as usize]
            * vectors[2 as libc::c_int as usize][2 as libc::c_int as usize];
    r_lerp_move[0 as libc::c_int
        as usize] = r_lerp_move[0 as libc::c_int as usize]
        + (*r_lastframe).translate[0 as libc::c_int as usize];
    r_lerp_move[1 as libc::c_int
        as usize] = r_lerp_move[1 as libc::c_int as usize]
        + (*r_lastframe).translate[1 as libc::c_int as usize];
    r_lerp_move[2 as libc::c_int
        as usize] = r_lerp_move[2 as libc::c_int as usize]
        + (*r_lastframe).translate[2 as libc::c_int as usize];
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        r_lerp_move[i
            as usize] = backlerp * r_lerp_move[i as usize]
            + frontlerp * (*r_thisframe).translate[i as usize];
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        r_lerp_frontv[i as usize] = frontlerp * (*r_thisframe).scale[i as usize];
        r_lerp_backv[i as usize] = backlerp * (*r_lastframe).scale[i as usize];
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn R_AliasDrawModel() {
    extern "C" {
        static mut d_pdrawspans: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
    }
    extern "C" {
        #[link_name = "R_PolysetDrawSpans8_Opaque"]
        fn R_PolysetDrawSpans8_Opaque_0(_: *mut libc::c_void);
    }
    extern "C" {
        #[link_name = "R_PolysetDrawSpans8_33"]
        fn R_PolysetDrawSpans8_33_0(_: *mut libc::c_void);
    }
    extern "C" {
        #[link_name = "R_PolysetDrawSpans8_66"]
        fn R_PolysetDrawSpans8_66_0(_: *mut libc::c_void);
    }
    extern "C" {
        #[link_name = "R_PolysetDrawSpansConstant8_33"]
        fn R_PolysetDrawSpansConstant8_33_0(_: *mut libc::c_void);
    }
    extern "C" {
        #[link_name = "R_PolysetDrawSpansConstant8_66"]
        fn R_PolysetDrawSpansConstant8_66_0(_: *mut libc::c_void);
    }
    s_pmdl = (*currentmodel).extradata as *mut dmdl_t;
    if (*r_lerpmodels).value == 0 as libc::c_int as libc::c_float {
        (*currententity).backlerp = 0 as libc::c_int as libc::c_float;
    }
    if (*currententity).flags & 4 as libc::c_int != 0 {
        if (*r_lefthand).value == 1.0f32 {
            aliasxscale = -aliasxscale;
        } else if (*r_lefthand).value == 2.0f32 {
            return
        }
    }
    R_AliasSetupFrames(s_pmdl);
    R_AliasSetUpTransform();
    if R_AliasCheckBBox() as libc::c_uint == 8 as libc::c_int as libc::c_uint {
        if (*currententity).flags & 4 as libc::c_int != 0
            && (*r_lefthand).value == 1.0f32
        {
            aliasxscale = -aliasxscale;
        }
        return;
    }
    if R_AliasSetupSkin() as u64 == 0 {
        (ri.Con_Printf)
            .expect(
                "non-null function pointer",
            )(
            0 as libc::c_int,
            b"R_AliasDrawModel %s: NULL skin found\n\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            ((*currentmodel).name).as_mut_ptr(),
        );
        return;
    }
    r_amodels_drawn += 1;
    R_AliasSetupLighting();
    if (*currententity).flags
        & (1024 as libc::c_int | 2048 as libc::c_int | 4096 as libc::c_int
            | 0x10000 as libc::c_int | 0x20000 as libc::c_int) != 0
    {
        let mut color: libc::c_int = 0;
        color = (*currententity).flags
            & (1024 as libc::c_int | 2048 as libc::c_int | 4096 as libc::c_int
                | 0x10000 as libc::c_int | 0x20000 as libc::c_int);
        if color & 1024 as libc::c_int != 0 {
            if color & 4096 as libc::c_int != 0 && color & 2048 as libc::c_int != 0 {
                r_aliasblendcolor = 0xd7 as libc::c_int;
            } else if color & (4096 as libc::c_int | 0x10000 as libc::c_int) != 0 {
                r_aliasblendcolor = 0x68 as libc::c_int;
            } else {
                r_aliasblendcolor = 0xf2 as libc::c_int;
            }
        } else if color & 4096 as libc::c_int != 0 {
            if color & 0x10000 as libc::c_int != 0 {
                r_aliasblendcolor = 0x72 as libc::c_int;
            } else {
                r_aliasblendcolor = 0xf3 as libc::c_int;
            }
        } else if color & 0x10000 as libc::c_int != 0 {
            r_aliasblendcolor = 0xdf as libc::c_int;
        } else if color & 0x20000 as libc::c_int != 0 {
            r_aliasblendcolor = 0x90 as libc::c_int;
        } else if color & 2048 as libc::c_int != 0 {
            r_aliasblendcolor = 0xd0 as libc::c_int;
        } else {
            r_aliasblendcolor = 0xd7 as libc::c_int;
        }
        if (*currententity).alpha as libc::c_double > 0.33f64 {
            d_pdrawspans = Some(
                R_PolysetDrawSpansConstant8_66_0
                    as unsafe extern "C" fn(*mut libc::c_void) -> (),
            );
        } else {
            d_pdrawspans = Some(
                R_PolysetDrawSpansConstant8_33_0
                    as unsafe extern "C" fn(*mut libc::c_void) -> (),
            );
        }
    } else if (*currententity).flags & 32 as libc::c_int != 0 {
        if (*currententity).alpha as libc::c_double > 0.66f64 {
            d_pdrawspans = Some(
                R_PolysetDrawSpans8_Opaque_0
                    as unsafe extern "C" fn(*mut libc::c_void) -> (),
            );
        } else if (*currententity).alpha as libc::c_double > 0.33f64 {
            d_pdrawspans = Some(
                R_PolysetDrawSpans8_66_0 as unsafe extern "C" fn(*mut libc::c_void) -> (),
            );
        } else {
            d_pdrawspans = Some(
                R_PolysetDrawSpans8_33_0 as unsafe extern "C" fn(*mut libc::c_void) -> (),
            );
        }
    } else {
        d_pdrawspans = Some(
            R_PolysetDrawSpans8_Opaque_0 as unsafe extern "C" fn(*mut libc::c_void) -> (),
        );
    }
    R_AliasSetUpLerpData(s_pmdl, (*currententity).backlerp);
    if (*currententity).flags & 16 as libc::c_int != 0 {
        s_ziscale = ((0x8000 as libc::c_int as libc::c_float
            * 0x10000 as libc::c_int as libc::c_float) as libc::c_double * 3.0f64)
            as libc::c_float;
    } else {
        s_ziscale = 0x8000 as libc::c_int as libc::c_float
            * 0x10000 as libc::c_int as libc::c_float;
    }
    R_AliasPreparePoints();
    if (*currententity).flags & 4 as libc::c_int != 0 && (*r_lefthand).value == 1.0f32 {
        aliasxscale = -aliasxscale;
    }
}
