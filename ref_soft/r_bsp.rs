#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    fn cos(_: libc::c_double) -> libc::c_double;
    fn sin(_: libc::c_double) -> libc::c_double;
    fn R_ConcatRotations(
        in1: *mut [libc::c_float; 3],
        in2: *mut [libc::c_float; 3],
        out: *mut [libc::c_float; 3],
    );
    static mut r_framecount: libc::c_int;
    static mut vup: vec3_t;
    static mut vpn: vec3_t;
    static mut vright: vec3_t;
    static mut r_drawworld: *mut cvar_t;
    static mut view_clipplanes: [clipplane_t; 4];
    static mut pfrustum_indexes: [*mut libc::c_int; 4];
    static mut currentmodel: *mut model_t;
    static mut r_currentkey: libc::c_int;
    fn R_RenderFace(fa: *mut msurface_t, clipflags: libc::c_int);
    static mut r_worldmodel: *mut model_t;
    static mut r_newrefdef: refdef_t;
    static mut r_visframecount: libc::c_int;
    static mut r_pcurrentvertbase: *mut mvertex_t;
    static mut r_origin: vec3_t;
    static mut r_worldentity: entity_t;
    fn R_RenderBmodelFace(pedges: *mut bedge_t, psurf: *mut msurface_t);
    fn R_TransformFrustum();
    static mut ri: refimport_t;
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
pub type model_t = model_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bedge_s {
    pub v: [*mut mvertex_t; 2],
    pub pnext: *mut bedge_s,
}
pub type bedge_t = bedge_s;
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
#[no_mangle]
pub static mut insubmodel: qboolean = false_0;
#[no_mangle]
pub static mut currententity: *mut entity_t = 0 as *const entity_t as *mut entity_t;
#[no_mangle]
pub static mut modelorg: vec3_t = [0.; 3];
#[no_mangle]
pub static mut r_entorigin: vec3_t = [0.; 3];
#[no_mangle]
pub static mut entity_rotation: [[libc::c_float; 3]; 3] = [[0.; 3]; 3];
#[no_mangle]
pub static mut r_currentbkey: libc::c_int = 0;
static mut pbverts: *mut mvertex_t = 0 as *const mvertex_t as *mut mvertex_t;
static mut pbedges: *mut bedge_t = 0 as *const bedge_t as *mut bedge_t;
static mut numbverts: libc::c_int = 0;
static mut numbedges: libc::c_int = 0;
static mut pfrontenter: *mut mvertex_t = 0 as *const mvertex_t as *mut mvertex_t;
static mut pfrontexit: *mut mvertex_t = 0 as *const mvertex_t as *mut mvertex_t;
static mut makeclippededge: qboolean = false_0;
#[no_mangle]
pub unsafe extern "C" fn R_EntityRotate(mut vec: *mut vec_t) {
    let mut tvec: vec3_t = [0.; 3];
    tvec[0 as libc::c_int as usize] = *vec.offset(0 as libc::c_int as isize);
    tvec[1 as libc::c_int as usize] = *vec.offset(1 as libc::c_int as isize);
    tvec[2 as libc::c_int as usize] = *vec.offset(2 as libc::c_int as isize);
    *vec
        .offset(
            0 as libc::c_int as isize,
        ) = entity_rotation[0 as libc::c_int as usize][0 as libc::c_int as usize]
        * tvec[0 as libc::c_int as usize]
        + entity_rotation[0 as libc::c_int as usize][1 as libc::c_int as usize]
            * tvec[1 as libc::c_int as usize]
        + entity_rotation[0 as libc::c_int as usize][2 as libc::c_int as usize]
            * tvec[2 as libc::c_int as usize];
    *vec
        .offset(
            1 as libc::c_int as isize,
        ) = entity_rotation[1 as libc::c_int as usize][0 as libc::c_int as usize]
        * tvec[0 as libc::c_int as usize]
        + entity_rotation[1 as libc::c_int as usize][1 as libc::c_int as usize]
            * tvec[1 as libc::c_int as usize]
        + entity_rotation[1 as libc::c_int as usize][2 as libc::c_int as usize]
            * tvec[2 as libc::c_int as usize];
    *vec
        .offset(
            2 as libc::c_int as isize,
        ) = entity_rotation[2 as libc::c_int as usize][0 as libc::c_int as usize]
        * tvec[0 as libc::c_int as usize]
        + entity_rotation[2 as libc::c_int as usize][1 as libc::c_int as usize]
            * tvec[1 as libc::c_int as usize]
        + entity_rotation[2 as libc::c_int as usize][2 as libc::c_int as usize]
            * tvec[2 as libc::c_int as usize];
}
#[no_mangle]
pub unsafe extern "C" fn R_RotateBmodel() {
    let mut angle: libc::c_float = 0.;
    let mut s: libc::c_float = 0.;
    let mut c: libc::c_float = 0.;
    let mut temp1: [[libc::c_float; 3]; 3] = [[0.; 3]; 3];
    let mut temp2: [[libc::c_float; 3]; 3] = [[0.; 3]; 3];
    let mut temp3: [[libc::c_float; 3]; 3] = [[0.; 3]; 3];
    angle = (*currententity).angles[1 as libc::c_int as usize];
    angle = (angle as libc::c_double * 3.14159265358979323846f64
        * 2 as libc::c_int as libc::c_double / 360 as libc::c_int as libc::c_double)
        as libc::c_float;
    s = sin(angle as libc::c_double) as libc::c_float;
    c = cos(angle as libc::c_double) as libc::c_float;
    temp1[0 as libc::c_int as usize][0 as libc::c_int as usize] = c;
    temp1[0 as libc::c_int as usize][1 as libc::c_int as usize] = s;
    temp1[0 as libc::c_int
        as usize][2 as libc::c_int as usize] = 0 as libc::c_int as libc::c_float;
    temp1[1 as libc::c_int as usize][0 as libc::c_int as usize] = -s;
    temp1[1 as libc::c_int as usize][1 as libc::c_int as usize] = c;
    temp1[1 as libc::c_int
        as usize][2 as libc::c_int as usize] = 0 as libc::c_int as libc::c_float;
    temp1[2 as libc::c_int
        as usize][0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_float;
    temp1[2 as libc::c_int
        as usize][1 as libc::c_int as usize] = 0 as libc::c_int as libc::c_float;
    temp1[2 as libc::c_int
        as usize][2 as libc::c_int as usize] = 1 as libc::c_int as libc::c_float;
    angle = (*currententity).angles[0 as libc::c_int as usize];
    angle = (angle as libc::c_double * 3.14159265358979323846f64
        * 2 as libc::c_int as libc::c_double / 360 as libc::c_int as libc::c_double)
        as libc::c_float;
    s = sin(angle as libc::c_double) as libc::c_float;
    c = cos(angle as libc::c_double) as libc::c_float;
    temp2[0 as libc::c_int as usize][0 as libc::c_int as usize] = c;
    temp2[0 as libc::c_int
        as usize][1 as libc::c_int as usize] = 0 as libc::c_int as libc::c_float;
    temp2[0 as libc::c_int as usize][2 as libc::c_int as usize] = -s;
    temp2[1 as libc::c_int
        as usize][0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_float;
    temp2[1 as libc::c_int
        as usize][1 as libc::c_int as usize] = 1 as libc::c_int as libc::c_float;
    temp2[1 as libc::c_int
        as usize][2 as libc::c_int as usize] = 0 as libc::c_int as libc::c_float;
    temp2[2 as libc::c_int as usize][0 as libc::c_int as usize] = s;
    temp2[2 as libc::c_int
        as usize][1 as libc::c_int as usize] = 0 as libc::c_int as libc::c_float;
    temp2[2 as libc::c_int as usize][2 as libc::c_int as usize] = c;
    R_ConcatRotations(temp2.as_mut_ptr(), temp1.as_mut_ptr(), temp3.as_mut_ptr());
    angle = (*currententity).angles[2 as libc::c_int as usize];
    angle = (angle as libc::c_double * 3.14159265358979323846f64
        * 2 as libc::c_int as libc::c_double / 360 as libc::c_int as libc::c_double)
        as libc::c_float;
    s = sin(angle as libc::c_double) as libc::c_float;
    c = cos(angle as libc::c_double) as libc::c_float;
    temp1[0 as libc::c_int
        as usize][0 as libc::c_int as usize] = 1 as libc::c_int as libc::c_float;
    temp1[0 as libc::c_int
        as usize][1 as libc::c_int as usize] = 0 as libc::c_int as libc::c_float;
    temp1[0 as libc::c_int
        as usize][2 as libc::c_int as usize] = 0 as libc::c_int as libc::c_float;
    temp1[1 as libc::c_int
        as usize][0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_float;
    temp1[1 as libc::c_int as usize][1 as libc::c_int as usize] = c;
    temp1[1 as libc::c_int as usize][2 as libc::c_int as usize] = s;
    temp1[2 as libc::c_int
        as usize][0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_float;
    temp1[2 as libc::c_int as usize][1 as libc::c_int as usize] = -s;
    temp1[2 as libc::c_int as usize][2 as libc::c_int as usize] = c;
    R_ConcatRotations(
        temp1.as_mut_ptr(),
        temp3.as_mut_ptr(),
        entity_rotation.as_mut_ptr(),
    );
    R_EntityRotate(modelorg.as_mut_ptr());
    R_EntityRotate(vpn.as_mut_ptr());
    R_EntityRotate(vright.as_mut_ptr());
    R_EntityRotate(vup.as_mut_ptr());
    R_TransformFrustum();
}
#[no_mangle]
pub unsafe extern "C" fn R_RecursiveClipBPoly(
    mut pedges: *mut bedge_t,
    mut pnode: *mut mnode_t,
    mut psurf: *mut msurface_t,
) {
    let mut psideedges: [*mut bedge_t; 2] = [0 as *mut bedge_t; 2];
    let mut pnextedge: *mut bedge_t = 0 as *mut bedge_t;
    let mut ptedge: *mut bedge_t = 0 as *mut bedge_t;
    let mut i: libc::c_int = 0;
    let mut side: libc::c_int = 0;
    let mut lastside: libc::c_int = 0;
    let mut dist: libc::c_float = 0.;
    let mut frac: libc::c_float = 0.;
    let mut lastdist: libc::c_float = 0.;
    let mut splitplane: *mut mplane_t = 0 as *mut mplane_t;
    let mut tplane: mplane_t = mplane_t {
        normal: [0.; 3],
        dist: 0.,
        type_0: 0,
        signbits: 0,
        pad: [0; 2],
    };
    let mut pvert: *mut mvertex_t = 0 as *mut mvertex_t;
    let mut plastvert: *mut mvertex_t = 0 as *mut mvertex_t;
    let mut ptvert: *mut mvertex_t = 0 as *mut mvertex_t;
    let mut pn: *mut mnode_t = 0 as *mut mnode_t;
    let mut area: libc::c_int = 0;
    psideedges[1 as libc::c_int as usize] = 0 as *mut bedge_t;
    psideedges[0 as libc::c_int as usize] = psideedges[1 as libc::c_int as usize];
    makeclippededge = false_0;
    splitplane = (*pnode).plane;
    tplane
        .dist = (*splitplane).dist
        - (r_entorigin[0 as libc::c_int as usize]
            * (*splitplane).normal[0 as libc::c_int as usize]
            + r_entorigin[1 as libc::c_int as usize]
                * (*splitplane).normal[1 as libc::c_int as usize]
            + r_entorigin[2 as libc::c_int as usize]
                * (*splitplane).normal[2 as libc::c_int as usize]);
    tplane
        .normal[0 as libc::c_int
        as usize] = entity_rotation[0 as libc::c_int as usize][0 as libc::c_int as usize]
        * (*splitplane).normal[0 as libc::c_int as usize]
        + entity_rotation[0 as libc::c_int as usize][1 as libc::c_int as usize]
            * (*splitplane).normal[1 as libc::c_int as usize]
        + entity_rotation[0 as libc::c_int as usize][2 as libc::c_int as usize]
            * (*splitplane).normal[2 as libc::c_int as usize];
    tplane
        .normal[1 as libc::c_int
        as usize] = entity_rotation[1 as libc::c_int as usize][0 as libc::c_int as usize]
        * (*splitplane).normal[0 as libc::c_int as usize]
        + entity_rotation[1 as libc::c_int as usize][1 as libc::c_int as usize]
            * (*splitplane).normal[1 as libc::c_int as usize]
        + entity_rotation[1 as libc::c_int as usize][2 as libc::c_int as usize]
            * (*splitplane).normal[2 as libc::c_int as usize];
    tplane
        .normal[2 as libc::c_int
        as usize] = entity_rotation[2 as libc::c_int as usize][0 as libc::c_int as usize]
        * (*splitplane).normal[0 as libc::c_int as usize]
        + entity_rotation[2 as libc::c_int as usize][1 as libc::c_int as usize]
            * (*splitplane).normal[1 as libc::c_int as usize]
        + entity_rotation[2 as libc::c_int as usize][2 as libc::c_int as usize]
            * (*splitplane).normal[2 as libc::c_int as usize];
    while !pedges.is_null() {
        pnextedge = (*pedges).pnext;
        plastvert = (*pedges).v[0 as libc::c_int as usize];
        lastdist = (*plastvert).position[0 as libc::c_int as usize]
            * tplane.normal[0 as libc::c_int as usize]
            + (*plastvert).position[1 as libc::c_int as usize]
                * tplane.normal[1 as libc::c_int as usize]
            + (*plastvert).position[2 as libc::c_int as usize]
                * tplane.normal[2 as libc::c_int as usize] - tplane.dist;
        if lastdist > 0 as libc::c_int as libc::c_float {
            lastside = 0 as libc::c_int;
        } else {
            lastside = 1 as libc::c_int;
        }
        pvert = (*pedges).v[1 as libc::c_int as usize];
        dist = (*pvert).position[0 as libc::c_int as usize]
            * tplane.normal[0 as libc::c_int as usize]
            + (*pvert).position[1 as libc::c_int as usize]
                * tplane.normal[1 as libc::c_int as usize]
            + (*pvert).position[2 as libc::c_int as usize]
                * tplane.normal[2 as libc::c_int as usize] - tplane.dist;
        if dist > 0 as libc::c_int as libc::c_float {
            side = 0 as libc::c_int;
        } else {
            side = 1 as libc::c_int;
        }
        if side != lastside {
            if numbverts >= 500 as libc::c_int {
                return;
            }
            frac = lastdist / (lastdist - dist);
            let fresh0 = numbverts;
            numbverts = numbverts + 1;
            ptvert = &mut *pbverts.offset(fresh0 as isize) as *mut mvertex_t;
            (*ptvert)
                .position[0 as libc::c_int
                as usize] = (*plastvert).position[0 as libc::c_int as usize]
                + frac
                    * ((*pvert).position[0 as libc::c_int as usize]
                        - (*plastvert).position[0 as libc::c_int as usize]);
            (*ptvert)
                .position[1 as libc::c_int
                as usize] = (*plastvert).position[1 as libc::c_int as usize]
                + frac
                    * ((*pvert).position[1 as libc::c_int as usize]
                        - (*plastvert).position[1 as libc::c_int as usize]);
            (*ptvert)
                .position[2 as libc::c_int
                as usize] = (*plastvert).position[2 as libc::c_int as usize]
                + frac
                    * ((*pvert).position[2 as libc::c_int as usize]
                        - (*plastvert).position[2 as libc::c_int as usize]);
            if numbedges >= 1000 as libc::c_int - 1 as libc::c_int {
                (ri.Con_Printf)
                    .expect(
                        "non-null function pointer",
                    )(
                    0 as libc::c_int,
                    b"Out of edges for bmodel\n\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                );
                return;
            }
            ptedge = &mut *pbedges.offset(numbedges as isize) as *mut bedge_t;
            let ref mut fresh1 = (*ptedge).pnext;
            *fresh1 = psideedges[lastside as usize];
            psideedges[lastside as usize] = ptedge;
            let ref mut fresh2 = (*ptedge).v[0 as libc::c_int as usize];
            *fresh2 = plastvert;
            let ref mut fresh3 = (*ptedge).v[1 as libc::c_int as usize];
            *fresh3 = ptvert;
            ptedge = &mut *pbedges.offset((numbedges + 1 as libc::c_int) as isize)
                as *mut bedge_t;
            let ref mut fresh4 = (*ptedge).pnext;
            *fresh4 = psideedges[side as usize];
            psideedges[side as usize] = ptedge;
            let ref mut fresh5 = (*ptedge).v[0 as libc::c_int as usize];
            *fresh5 = ptvert;
            let ref mut fresh6 = (*ptedge).v[1 as libc::c_int as usize];
            *fresh6 = pvert;
            numbedges += 2 as libc::c_int;
            if side == 0 as libc::c_int {
                pfrontenter = ptvert;
                makeclippededge = true_0;
            } else {
                pfrontexit = ptvert;
                makeclippededge = true_0;
            }
        } else {
            let ref mut fresh7 = (*pedges).pnext;
            *fresh7 = psideedges[side as usize];
            psideedges[side as usize] = pedges;
        }
        pedges = pnextedge;
    }
    if makeclippededge as u64 != 0 {
        if numbedges >= 1000 as libc::c_int - 2 as libc::c_int {
            (ri.Con_Printf)
                .expect(
                    "non-null function pointer",
                )(
                0 as libc::c_int,
                b"Out of edges for bmodel\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
            return;
        }
        ptedge = &mut *pbedges.offset(numbedges as isize) as *mut bedge_t;
        let ref mut fresh8 = (*ptedge).pnext;
        *fresh8 = psideedges[0 as libc::c_int as usize];
        psideedges[0 as libc::c_int as usize] = ptedge;
        let ref mut fresh9 = (*ptedge).v[0 as libc::c_int as usize];
        *fresh9 = pfrontexit;
        let ref mut fresh10 = (*ptedge).v[1 as libc::c_int as usize];
        *fresh10 = pfrontenter;
        ptedge = &mut *pbedges.offset((numbedges + 1 as libc::c_int) as isize)
            as *mut bedge_t;
        let ref mut fresh11 = (*ptedge).pnext;
        *fresh11 = psideedges[1 as libc::c_int as usize];
        psideedges[1 as libc::c_int as usize] = ptedge;
        let ref mut fresh12 = (*ptedge).v[0 as libc::c_int as usize];
        *fresh12 = pfrontenter;
        let ref mut fresh13 = (*ptedge).v[1 as libc::c_int as usize];
        *fresh13 = pfrontexit;
        numbedges += 2 as libc::c_int;
    }
    let mut current_block_77: u64;
    i = 0 as libc::c_int;
    while i < 2 as libc::c_int {
        if !(psideedges[i as usize]).is_null() {
            pn = (*pnode).children[i as usize];
            if (*pn).visframe == r_visframecount {
                if (*pn).contents != -(1 as libc::c_int) {
                    if (*pn).contents != 1 as libc::c_int {
                        if !(r_newrefdef.areabits).is_null() {
                            area = (*(pn as *mut mleaf_t)).area;
                            if *(r_newrefdef.areabits)
                                .offset((area >> 3 as libc::c_int) as isize) as libc::c_int
                                & (1 as libc::c_int) << (area & 7 as libc::c_int) == 0
                            {
                                current_block_77 = 1134115459065347084;
                            } else {
                                current_block_77 = 7385833325316299293;
                            }
                        } else {
                            current_block_77 = 7385833325316299293;
                        }
                        match current_block_77 {
                            1134115459065347084 => {}
                            _ => {
                                r_currentbkey = (*(pn as *mut mleaf_t)).key;
                                R_RenderBmodelFace(psideedges[i as usize], psurf);
                            }
                        }
                    }
                } else {
                    R_RecursiveClipBPoly(
                        psideedges[i as usize],
                        (*pnode).children[i as usize],
                        psurf,
                    );
                }
            }
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn R_DrawSolidClippedSubmodelPolygons(
    mut pmodel: *mut model_t,
    mut topnode: *mut mnode_t,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut lindex: libc::c_int = 0;
    let mut dot: vec_t = 0.;
    let mut psurf: *mut msurface_t = 0 as *mut msurface_t;
    let mut numsurfaces: libc::c_int = 0;
    let mut pplane: *mut mplane_t = 0 as *mut mplane_t;
    let mut bverts: [mvertex_t; 500] = [mvertex_t { position: [0.; 3] }; 500];
    let mut bedges: [bedge_t; 1000] = [bedge_t {
        v: [0 as *mut mvertex_t; 2],
        pnext: 0 as *mut bedge_s,
    }; 1000];
    let mut pbedge: *mut bedge_t = 0 as *mut bedge_t;
    let mut pedge: *mut medge_t = 0 as *mut medge_t;
    let mut pedges: *mut medge_t = 0 as *mut medge_t;
    psurf = &mut *((*pmodel).surfaces).offset((*pmodel).firstmodelsurface as isize)
        as *mut msurface_t;
    numsurfaces = (*pmodel).nummodelsurfaces;
    pedges = (*pmodel).edges;
    i = 0 as libc::c_int;
    while i < numsurfaces {
        pplane = (*psurf).plane;
        dot = modelorg[0 as libc::c_int as usize]
            * (*pplane).normal[0 as libc::c_int as usize]
            + modelorg[1 as libc::c_int as usize]
                * (*pplane).normal[1 as libc::c_int as usize]
            + modelorg[2 as libc::c_int as usize]
                * (*pplane).normal[2 as libc::c_int as usize] - (*pplane).dist;
        if !((*psurf).flags & 2 as libc::c_int == 0 && (dot as libc::c_double) < -0.01f64
            || (*psurf).flags & 2 as libc::c_int != 0 && dot as libc::c_double > 0.01f64)
        {
            pbverts = bverts.as_mut_ptr();
            pbedges = bedges.as_mut_ptr();
            numbedges = 0 as libc::c_int;
            numbverts = numbedges;
            pbedge = &mut *bedges.as_mut_ptr().offset(numbedges as isize)
                as *mut bedge_t;
            numbedges += (*psurf).numedges;
            j = 0 as libc::c_int;
            while j < (*psurf).numedges {
                lindex = *((*pmodel).surfedges)
                    .offset(((*psurf).firstedge + j) as isize);
                if lindex > 0 as libc::c_int {
                    pedge = &mut *pedges.offset(lindex as isize) as *mut medge_t;
                    let ref mut fresh14 = (*pbedge.offset(j as isize))
                        .v[0 as libc::c_int as usize];
                    *fresh14 = &mut *r_pcurrentvertbase
                        .offset(
                            *((*pedge).v).as_mut_ptr().offset(0 as libc::c_int as isize)
                                as isize,
                        ) as *mut mvertex_t;
                    let ref mut fresh15 = (*pbedge.offset(j as isize))
                        .v[1 as libc::c_int as usize];
                    *fresh15 = &mut *r_pcurrentvertbase
                        .offset(
                            *((*pedge).v).as_mut_ptr().offset(1 as libc::c_int as isize)
                                as isize,
                        ) as *mut mvertex_t;
                } else {
                    lindex = -lindex;
                    pedge = &mut *pedges.offset(lindex as isize) as *mut medge_t;
                    let ref mut fresh16 = (*pbedge.offset(j as isize))
                        .v[0 as libc::c_int as usize];
                    *fresh16 = &mut *r_pcurrentvertbase
                        .offset(
                            *((*pedge).v).as_mut_ptr().offset(1 as libc::c_int as isize)
                                as isize,
                        ) as *mut mvertex_t;
                    let ref mut fresh17 = (*pbedge.offset(j as isize))
                        .v[1 as libc::c_int as usize];
                    *fresh17 = &mut *r_pcurrentvertbase
                        .offset(
                            *((*pedge).v).as_mut_ptr().offset(0 as libc::c_int as isize)
                                as isize,
                        ) as *mut mvertex_t;
                }
                let ref mut fresh18 = (*pbedge.offset(j as isize)).pnext;
                *fresh18 = &mut *pbedge.offset((j + 1 as libc::c_int) as isize)
                    as *mut bedge_t;
                j += 1;
            }
            let ref mut fresh19 = (*pbedge.offset((j - 1 as libc::c_int) as isize))
                .pnext;
            *fresh19 = 0 as *mut bedge_s;
            if (*(*psurf).texinfo).flags & (0x20 as libc::c_int | 0x10 as libc::c_int)
                == 0
            {
                R_RecursiveClipBPoly(pbedge, topnode, psurf);
            } else {
                R_RenderBmodelFace(pbedge, psurf);
            }
        }
        i += 1;
        psurf = psurf.offset(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn R_DrawSubmodelPolygons(
    mut pmodel: *mut model_t,
    mut clipflags: libc::c_int,
    mut topnode: *mut mnode_t,
) {
    let mut i: libc::c_int = 0;
    let mut dot: vec_t = 0.;
    let mut psurf: *mut msurface_t = 0 as *mut msurface_t;
    let mut numsurfaces: libc::c_int = 0;
    let mut pplane: *mut mplane_t = 0 as *mut mplane_t;
    psurf = &mut *((*pmodel).surfaces).offset((*pmodel).firstmodelsurface as isize)
        as *mut msurface_t;
    numsurfaces = (*pmodel).nummodelsurfaces;
    i = 0 as libc::c_int;
    while i < numsurfaces {
        pplane = (*psurf).plane;
        dot = modelorg[0 as libc::c_int as usize]
            * (*pplane).normal[0 as libc::c_int as usize]
            + modelorg[1 as libc::c_int as usize]
                * (*pplane).normal[1 as libc::c_int as usize]
            + modelorg[2 as libc::c_int as usize]
                * (*pplane).normal[2 as libc::c_int as usize] - (*pplane).dist;
        if (*psurf).flags & 2 as libc::c_int != 0 && (dot as libc::c_double) < -0.01f64
            || (*psurf).flags & 2 as libc::c_int == 0 && dot as libc::c_double > 0.01f64
        {
            r_currentkey = (*(topnode as *mut mleaf_t)).key;
            R_RenderFace(psurf, clipflags);
        }
        i += 1;
        psurf = psurf.offset(1);
    }
}
#[no_mangle]
pub static mut c_drawnode: libc::c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn R_RecursiveWorldNode(
    mut node: *mut mnode_t,
    mut clipflags: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut side: libc::c_int = 0;
    let mut pindex: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut acceptpt: vec3_t = [0.; 3];
    let mut rejectpt: vec3_t = [0.; 3];
    let mut plane: *mut mplane_t = 0 as *mut mplane_t;
    let mut surf: *mut msurface_t = 0 as *mut msurface_t;
    let mut mark: *mut *mut msurface_t = 0 as *mut *mut msurface_t;
    let mut d: libc::c_float = 0.;
    let mut dot: libc::c_float = 0.;
    let mut pleaf: *mut mleaf_t = 0 as *mut mleaf_t;
    if (*node).contents == 1 as libc::c_int {
        return;
    }
    if (*node).visframe != r_visframecount {
        return;
    }
    if clipflags != 0 {
        i = 0 as libc::c_int;
        while i < 4 as libc::c_int {
            if !(clipflags & (1 as libc::c_int) << i == 0) {
                pindex = pfrustum_indexes[i as usize];
                rejectpt[0 as libc::c_int
                    as usize] = (*node)
                    .minmaxs[*pindex.offset(0 as libc::c_int as isize) as usize]
                    as libc::c_float;
                rejectpt[1 as libc::c_int
                    as usize] = (*node)
                    .minmaxs[*pindex.offset(1 as libc::c_int as isize) as usize]
                    as libc::c_float;
                rejectpt[2 as libc::c_int
                    as usize] = (*node)
                    .minmaxs[*pindex.offset(2 as libc::c_int as isize) as usize]
                    as libc::c_float;
                d = rejectpt[0 as libc::c_int as usize]
                    * view_clipplanes[i as usize].normal[0 as libc::c_int as usize]
                    + rejectpt[1 as libc::c_int as usize]
                        * view_clipplanes[i as usize].normal[1 as libc::c_int as usize]
                    + rejectpt[2 as libc::c_int as usize]
                        * view_clipplanes[i as usize].normal[2 as libc::c_int as usize];
                d -= view_clipplanes[i as usize].dist;
                if d <= 0 as libc::c_int as libc::c_float {
                    return;
                }
                acceptpt[0 as libc::c_int
                    as usize] = (*node)
                    .minmaxs[*pindex
                    .offset((3 as libc::c_int + 0 as libc::c_int) as isize) as usize]
                    as libc::c_float;
                acceptpt[1 as libc::c_int
                    as usize] = (*node)
                    .minmaxs[*pindex
                    .offset((3 as libc::c_int + 1 as libc::c_int) as isize) as usize]
                    as libc::c_float;
                acceptpt[2 as libc::c_int
                    as usize] = (*node)
                    .minmaxs[*pindex
                    .offset((3 as libc::c_int + 2 as libc::c_int) as isize) as usize]
                    as libc::c_float;
                d = acceptpt[0 as libc::c_int as usize]
                    * view_clipplanes[i as usize].normal[0 as libc::c_int as usize]
                    + acceptpt[1 as libc::c_int as usize]
                        * view_clipplanes[i as usize].normal[1 as libc::c_int as usize]
                    + acceptpt[2 as libc::c_int as usize]
                        * view_clipplanes[i as usize].normal[2 as libc::c_int as usize];
                d -= view_clipplanes[i as usize].dist;
                if d >= 0 as libc::c_int as libc::c_float {
                    clipflags &= !((1 as libc::c_int) << i);
                }
            }
            i += 1;
        }
    }
    c_drawnode += 1;
    if (*node).contents != -(1 as libc::c_int) {
        pleaf = node as *mut mleaf_t;
        if !(r_newrefdef.areabits).is_null() {
            if *(r_newrefdef.areabits)
                .offset(((*pleaf).area >> 3 as libc::c_int) as isize) as libc::c_int
                & (1 as libc::c_int) << ((*pleaf).area & 7 as libc::c_int) == 0
            {
                return;
            }
        }
        mark = (*pleaf).firstmarksurface;
        c = (*pleaf).nummarksurfaces;
        if c != 0 {
            loop {
                (**mark).visframe = r_framecount;
                mark = mark.offset(1);
                c -= 1;
                if !(c != 0) {
                    break;
                }
            }
        }
        (*pleaf).key = r_currentkey;
        r_currentkey += 1;
    } else {
        plane = (*node).plane;
        match (*plane).type_0 as libc::c_int {
            0 => {
                dot = modelorg[0 as libc::c_int as usize] - (*plane).dist;
            }
            1 => {
                dot = modelorg[1 as libc::c_int as usize] - (*plane).dist;
            }
            2 => {
                dot = modelorg[2 as libc::c_int as usize] - (*plane).dist;
            }
            _ => {
                dot = modelorg[0 as libc::c_int as usize]
                    * (*plane).normal[0 as libc::c_int as usize]
                    + modelorg[1 as libc::c_int as usize]
                        * (*plane).normal[1 as libc::c_int as usize]
                    + modelorg[2 as libc::c_int as usize]
                        * (*plane).normal[2 as libc::c_int as usize] - (*plane).dist;
            }
        }
        if dot >= 0 as libc::c_int as libc::c_float {
            side = 0 as libc::c_int;
        } else {
            side = 1 as libc::c_int;
        }
        R_RecursiveWorldNode((*node).children[side as usize], clipflags);
        c = (*node).numsurfaces as libc::c_int;
        if c != 0 {
            surf = ((*r_worldmodel).surfaces)
                .offset((*node).firstsurface as libc::c_int as isize);
            if (dot as libc::c_double) < -0.01f64 {
                loop {
                    if (*surf).flags & 2 as libc::c_int != 0
                        && (*surf).visframe == r_framecount
                    {
                        R_RenderFace(surf, clipflags);
                    }
                    surf = surf.offset(1);
                    c -= 1;
                    if !(c != 0) {
                        break;
                    }
                }
            } else if dot as libc::c_double > 0.01f64 {
                loop {
                    if (*surf).flags & 2 as libc::c_int == 0
                        && (*surf).visframe == r_framecount
                    {
                        R_RenderFace(surf, clipflags);
                    }
                    surf = surf.offset(1);
                    c -= 1;
                    if !(c != 0) {
                        break;
                    }
                }
            }
            r_currentkey += 1;
        }
        R_RecursiveWorldNode(
            (*node).children[(side == 0) as libc::c_int as usize],
            clipflags,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn R_RenderWorld() {
    if (*r_drawworld).value == 0. {
        return;
    }
    if r_newrefdef.rdflags & 2 as libc::c_int != 0 {
        return;
    }
    c_drawnode = 0 as libc::c_int;
    r_worldentity
        .frame = (r_newrefdef.time * 2 as libc::c_int as libc::c_float) as libc::c_int;
    currententity = &mut r_worldentity;
    modelorg[0 as libc::c_int as usize] = r_origin[0 as libc::c_int as usize];
    modelorg[1 as libc::c_int as usize] = r_origin[1 as libc::c_int as usize];
    modelorg[2 as libc::c_int as usize] = r_origin[2 as libc::c_int as usize];
    currentmodel = r_worldmodel;
    r_pcurrentvertbase = (*currentmodel).vertexes;
    R_RecursiveWorldNode((*currentmodel).nodes, 15 as libc::c_int);
}
