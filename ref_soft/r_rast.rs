#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    static mut ri: refimport_t;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn ceil(_: libc::c_double) -> libc::c_double;
    static mut r_refdef: oldrefdef_t;
    static mut r_framecount: libc::c_int;
    static mut surfaces: *mut surf_t;
    static mut surface_p: *mut surf_t;
    static mut surf_max: *mut surf_t;
    static mut xcenter: libc::c_float;
    static mut ycenter: libc::c_float;
    static mut xscale: libc::c_float;
    static mut yscale: libc::c_float;
    static mut xscaleinv: libc::c_float;
    static mut yscaleinv: libc::c_float;
    fn TransformVector(in_0: *mut vec_t, out: *mut vec_t);
    static mut r_origin: vec3_t;
    static mut currentmodel: *mut model_t;
    static mut currententity: *mut entity_t;
    static mut modelorg: vec3_t;
    static mut r_alpha_surfaces: *mut msurface_t;
    static mut insubmodel: qboolean;
    static mut r_currentkey: libc::c_int;
    static mut r_polycount: libc::c_int;
    static mut removeedges: [*mut edge_t; 1200];
    static mut newedges: [*mut edge_t; 1200];
    static mut edge_p: *mut edge_t;
    static mut r_pcurrentvertbase: *mut mvertex_t;
    static mut r_edges: *mut edge_t;
    static mut r_outofedges: libc::c_int;
    static mut edge_max: *mut edge_t;
    static mut r_outofsurfaces: libc::c_int;
    static mut r_currentbkey: libc::c_int;
    static mut r_clipflags: libc::c_int;
}
pub type byte = libc::c_uchar;
pub type qboolean = libc::c_uint;
pub const true_0: qboolean = 1;
pub const false_0: qboolean = 0;
pub type vec_t = libc::c_float;
pub type vec3_t = [vec_t; 3];
pub type fixed16_t = libc::c_int;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct espan_s {
    pub u: libc::c_int,
    pub v: libc::c_int,
    pub count: libc::c_int,
    pub pnext: *mut espan_s,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct surf_s {
    pub next: *mut surf_s,
    pub prev: *mut surf_s,
    pub spans: *mut espan_s,
    pub key: libc::c_int,
    pub last_u: libc::c_int,
    pub spanstate: libc::c_int,
    pub flags: libc::c_int,
    pub msurf: *mut msurface_t,
    pub entity: *mut entity_t,
    pub nearzi: libc::c_float,
    pub insubmodel: qboolean,
    pub d_ziorigin: libc::c_float,
    pub d_zistepu: libc::c_float,
    pub d_zistepv: libc::c_float,
    pub pad: [libc::c_int; 2],
}
pub type surf_t = surf_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct edge_s {
    pub u: fixed16_t,
    pub u_step: fixed16_t,
    pub prev: *mut edge_s,
    pub next: *mut edge_s,
    pub surfs: [libc::c_ushort; 2],
    pub nextremove: *mut edge_s,
    pub nearzi: libc::c_float,
    pub owner: *mut medge_t,
}
pub type edge_t = edge_s;
#[no_mangle]
pub static mut cacheoffset: libc::c_uint = 0;
#[no_mangle]
pub static mut c_faceclip: libc::c_int = 0;
#[no_mangle]
pub static mut entity_clipplanes: *mut clipplane_t = 0 as *const clipplane_t
    as *mut clipplane_t;
#[no_mangle]
pub static mut view_clipplanes: [clipplane_t; 4] = [clipplane_t {
    normal: [0.; 3],
    dist: 0.,
    next: 0 as *const clipplane_s as *mut clipplane_s,
    leftedge: 0,
    rightedge: 0,
    reserved: [0; 2],
}; 4];
#[no_mangle]
pub static mut world_clipplanes: [clipplane_t; 16] = [clipplane_t {
    normal: [0.; 3],
    dist: 0.,
    next: 0 as *const clipplane_s as *mut clipplane_s,
    leftedge: 0,
    rightedge: 0,
    reserved: [0; 2],
}; 16];
#[no_mangle]
pub static mut r_pedge: *mut medge_t = 0 as *const medge_t as *mut medge_t;
#[no_mangle]
pub static mut r_leftclipped: qboolean = false_0;
#[no_mangle]
pub static mut r_rightclipped: qboolean = false_0;
static mut makeleftedge: qboolean = false_0;
static mut makerightedge: qboolean = false_0;
#[no_mangle]
pub static mut r_nearzionly: qboolean = false_0;
#[no_mangle]
pub static mut sintable: [libc::c_int; 1280] = [0; 1280];
#[no_mangle]
pub static mut intsintable: [libc::c_int; 1280] = [0; 1280];
#[no_mangle]
pub static mut blanktable: [libc::c_int; 1280] = [0; 1280];
#[no_mangle]
pub static mut r_leftenter: mvertex_t = mvertex_t { position: [0.; 3] };
#[no_mangle]
pub static mut r_leftexit: mvertex_t = mvertex_t { position: [0.; 3] };
#[no_mangle]
pub static mut r_rightenter: mvertex_t = mvertex_t { position: [0.; 3] };
#[no_mangle]
pub static mut r_rightexit: mvertex_t = mvertex_t { position: [0.; 3] };
#[no_mangle]
pub static mut r_emitted: libc::c_int = 0;
#[no_mangle]
pub static mut r_nearzi: libc::c_float = 0.;
#[no_mangle]
pub static mut r_u1: libc::c_float = 0.;
#[no_mangle]
pub static mut r_v1: libc::c_float = 0.;
#[no_mangle]
pub static mut r_lzi1: libc::c_float = 0.;
#[no_mangle]
pub static mut r_ceilv1: libc::c_int = 0;
#[no_mangle]
pub static mut r_lastvertvalid: qboolean = false_0;
#[no_mangle]
pub static mut r_skyframe: libc::c_int = 0;
#[no_mangle]
pub static mut r_skyfaces: *mut msurface_t = 0 as *const msurface_t as *mut msurface_t;
#[no_mangle]
pub static mut r_skyplanes: [mplane_t; 6] = [mplane_t {
    normal: [0.; 3],
    dist: 0.,
    type_0: 0,
    signbits: 0,
    pad: [0; 2],
}; 6];
#[no_mangle]
pub static mut r_skytexinfo: [mtexinfo_t; 6] = [mtexinfo_t {
    vecs: [[0.; 4]; 2],
    mipadjust: 0.,
    image: 0 as *const image_t as *mut image_t,
    flags: 0,
    numframes: 0,
    next: 0 as *const mtexinfo_s as *mut mtexinfo_s,
}; 6];
#[no_mangle]
pub static mut r_skyverts: *mut mvertex_t = 0 as *const mvertex_t as *mut mvertex_t;
#[no_mangle]
pub static mut r_skyedges: *mut medge_t = 0 as *const medge_t as *mut medge_t;
#[no_mangle]
pub static mut r_skysurfedges: *mut libc::c_int = 0 as *const libc::c_int
    as *mut libc::c_int;
#[no_mangle]
pub static mut skybox_planes: [libc::c_int; 12] = [
    2 as libc::c_int,
    -(128 as libc::c_int),
    0 as libc::c_int,
    -(128 as libc::c_int),
    2 as libc::c_int,
    128 as libc::c_int,
    1 as libc::c_int,
    128 as libc::c_int,
    0 as libc::c_int,
    128 as libc::c_int,
    1 as libc::c_int,
    -(128 as libc::c_int),
];
#[no_mangle]
pub static mut box_surfedges: [libc::c_int; 24] = [
    1 as libc::c_int,
    2 as libc::c_int,
    3 as libc::c_int,
    4 as libc::c_int,
    -(1 as libc::c_int),
    5 as libc::c_int,
    6 as libc::c_int,
    7 as libc::c_int,
    8 as libc::c_int,
    9 as libc::c_int,
    -(6 as libc::c_int),
    10 as libc::c_int,
    -(2 as libc::c_int),
    -(7 as libc::c_int),
    -(9 as libc::c_int),
    11 as libc::c_int,
    12 as libc::c_int,
    -(3 as libc::c_int),
    -(11 as libc::c_int),
    -(8 as libc::c_int),
    -(12 as libc::c_int),
    -(10 as libc::c_int),
    -(5 as libc::c_int),
    -(4 as libc::c_int),
];
#[no_mangle]
pub static mut box_edges: [libc::c_int; 24] = [
    1 as libc::c_int,
    2 as libc::c_int,
    2 as libc::c_int,
    3 as libc::c_int,
    3 as libc::c_int,
    4 as libc::c_int,
    4 as libc::c_int,
    1 as libc::c_int,
    1 as libc::c_int,
    5 as libc::c_int,
    5 as libc::c_int,
    6 as libc::c_int,
    6 as libc::c_int,
    2 as libc::c_int,
    7 as libc::c_int,
    8 as libc::c_int,
    8 as libc::c_int,
    6 as libc::c_int,
    5 as libc::c_int,
    7 as libc::c_int,
    8 as libc::c_int,
    3 as libc::c_int,
    7 as libc::c_int,
    4 as libc::c_int,
];
#[no_mangle]
pub static mut box_faces: [libc::c_int; 6] = [
    0 as libc::c_int,
    0 as libc::c_int,
    2 as libc::c_int,
    2 as libc::c_int,
    2 as libc::c_int,
    0 as libc::c_int,
];
#[no_mangle]
pub static mut box_vecs: [[vec3_t; 2]; 6] = [
    [
        [
            0 as libc::c_int as vec_t,
            -(1 as libc::c_int) as vec_t,
            0 as libc::c_int as vec_t,
        ],
        [
            -(1 as libc::c_int) as vec_t,
            0 as libc::c_int as vec_t,
            0 as libc::c_int as vec_t,
        ],
    ],
    [
        [
            0 as libc::c_int as vec_t,
            1 as libc::c_int as vec_t,
            0 as libc::c_int as vec_t,
        ],
        [
            0 as libc::c_int as vec_t,
            0 as libc::c_int as vec_t,
            -(1 as libc::c_int) as vec_t,
        ],
    ],
    [
        [
            0 as libc::c_int as vec_t,
            -(1 as libc::c_int) as vec_t,
            0 as libc::c_int as vec_t,
        ],
        [1 as libc::c_int as vec_t, 0 as libc::c_int as vec_t, 0 as libc::c_int as vec_t],
    ],
    [
        [
            1 as libc::c_int as vec_t,
            0 as libc::c_int as vec_t,
            0 as libc::c_int as vec_t,
        ],
        [
            0 as libc::c_int as vec_t,
            0 as libc::c_int as vec_t,
            -(1 as libc::c_int) as vec_t,
        ],
    ],
    [
        [
            0 as libc::c_int as vec_t,
            -(1 as libc::c_int) as vec_t,
            0 as libc::c_int as vec_t,
        ],
        [
            0 as libc::c_int as vec_t,
            0 as libc::c_int as vec_t,
            -(1 as libc::c_int) as vec_t,
        ],
    ],
    [
        [
            -(1 as libc::c_int) as vec_t,
            0 as libc::c_int as vec_t,
            0 as libc::c_int as vec_t,
        ],
        [
            0 as libc::c_int as vec_t,
            0 as libc::c_int as vec_t,
            -(1 as libc::c_int) as vec_t,
        ],
    ],
];
#[no_mangle]
pub static mut box_verts: [[libc::c_float; 3]; 8] = [
    [
        -(1 as libc::c_int) as libc::c_float,
        -(1 as libc::c_int) as libc::c_float,
        -(1 as libc::c_int) as libc::c_float,
    ],
    [
        -(1 as libc::c_int) as libc::c_float,
        1 as libc::c_int as libc::c_float,
        -(1 as libc::c_int) as libc::c_float,
    ],
    [
        1 as libc::c_int as libc::c_float,
        1 as libc::c_int as libc::c_float,
        -(1 as libc::c_int) as libc::c_float,
    ],
    [
        1 as libc::c_int as libc::c_float,
        -(1 as libc::c_int) as libc::c_float,
        -(1 as libc::c_int) as libc::c_float,
    ],
    [
        -(1 as libc::c_int) as libc::c_float,
        -(1 as libc::c_int) as libc::c_float,
        1 as libc::c_int as libc::c_float,
    ],
    [
        -(1 as libc::c_int) as libc::c_float,
        1 as libc::c_int as libc::c_float,
        1 as libc::c_int as libc::c_float,
    ],
    [
        1 as libc::c_int as libc::c_float,
        -(1 as libc::c_int) as libc::c_float,
        1 as libc::c_int as libc::c_float,
    ],
    [
        1 as libc::c_int as libc::c_float,
        1 as libc::c_int as libc::c_float,
        1 as libc::c_int as libc::c_float,
    ],
];
#[no_mangle]
pub unsafe extern "C" fn R_InitSkyBox() {
    let mut i: libc::c_int = 0;
    extern "C" {
        static mut loadmodel: *mut model_t;
    }
    r_skyfaces = ((*loadmodel).surfaces).offset((*loadmodel).numsurfaces as isize);
    (*loadmodel).numsurfaces += 6 as libc::c_int;
    r_skyverts = ((*loadmodel).vertexes).offset((*loadmodel).numvertexes as isize);
    (*loadmodel).numvertexes += 8 as libc::c_int;
    r_skyedges = ((*loadmodel).edges).offset((*loadmodel).numedges as isize);
    (*loadmodel).numedges += 12 as libc::c_int;
    r_skysurfedges = ((*loadmodel).surfedges).offset((*loadmodel).numsurfedges as isize);
    (*loadmodel).numsurfedges += 24 as libc::c_int;
    if (*loadmodel).numsurfaces > 65536 as libc::c_int
        || (*loadmodel).numvertexes > 65536 as libc::c_int
        || (*loadmodel).numedges > 128000 as libc::c_int
    {
        (ri.Sys_Error)
            .expect(
                "non-null function pointer",
            )(
            1 as libc::c_int,
            b"InitSkyBox: map overflow\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    memset(
        r_skyfaces as *mut libc::c_void,
        0 as libc::c_int,
        (6 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<msurface_t>() as libc::c_ulong),
    );
    i = 0 as libc::c_int;
    while i < 6 as libc::c_int {
        r_skyplanes[i as usize]
            .normal[skybox_planes[(i * 2 as libc::c_int) as usize]
            as usize] = 1 as libc::c_int as vec_t;
        r_skyplanes[i as usize]
            .dist = skybox_planes[(i * 2 as libc::c_int + 1 as libc::c_int) as usize]
            as libc::c_float;
        r_skytexinfo[i as usize]
            .vecs[0 as libc::c_int
            as usize][0 as libc::c_int
            as usize] = box_vecs[i
            as usize][0 as libc::c_int as usize][0 as libc::c_int as usize];
        r_skytexinfo[i as usize]
            .vecs[0 as libc::c_int
            as usize][1 as libc::c_int
            as usize] = box_vecs[i
            as usize][0 as libc::c_int as usize][1 as libc::c_int as usize];
        r_skytexinfo[i as usize]
            .vecs[0 as libc::c_int
            as usize][2 as libc::c_int
            as usize] = box_vecs[i
            as usize][0 as libc::c_int as usize][2 as libc::c_int as usize];
        r_skytexinfo[i as usize]
            .vecs[1 as libc::c_int
            as usize][0 as libc::c_int
            as usize] = box_vecs[i
            as usize][1 as libc::c_int as usize][0 as libc::c_int as usize];
        r_skytexinfo[i as usize]
            .vecs[1 as libc::c_int
            as usize][1 as libc::c_int
            as usize] = box_vecs[i
            as usize][1 as libc::c_int as usize][1 as libc::c_int as usize];
        r_skytexinfo[i as usize]
            .vecs[1 as libc::c_int
            as usize][2 as libc::c_int
            as usize] = box_vecs[i
            as usize][1 as libc::c_int as usize][2 as libc::c_int as usize];
        let ref mut fresh0 = (*r_skyfaces.offset(i as isize)).plane;
        *fresh0 = &mut *r_skyplanes.as_mut_ptr().offset(i as isize) as *mut mplane_t;
        (*r_skyfaces.offset(i as isize)).numedges = 4 as libc::c_int;
        (*r_skyfaces.offset(i as isize))
            .flags = box_faces[i as usize] | 0x80 as libc::c_int;
        (*r_skyfaces.offset(i as isize))
            .firstedge = (*loadmodel).numsurfedges - 24 as libc::c_int
            + i * 4 as libc::c_int;
        let ref mut fresh1 = (*r_skyfaces.offset(i as isize)).texinfo;
        *fresh1 = &mut *r_skytexinfo.as_mut_ptr().offset(i as isize) as *mut mtexinfo_t;
        (*r_skyfaces.offset(i as isize))
            .texturemins[0 as libc::c_int
            as usize] = -(128 as libc::c_int) as libc::c_short;
        (*r_skyfaces.offset(i as isize))
            .texturemins[1 as libc::c_int
            as usize] = -(128 as libc::c_int) as libc::c_short;
        (*r_skyfaces.offset(i as isize))
            .extents[0 as libc::c_int as usize] = 256 as libc::c_int as libc::c_short;
        (*r_skyfaces.offset(i as isize))
            .extents[1 as libc::c_int as usize] = 256 as libc::c_int as libc::c_short;
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < 24 as libc::c_int {
        if box_surfedges[i as usize] > 0 as libc::c_int {
            *r_skysurfedges
                .offset(
                    i as isize,
                ) = (*loadmodel).numedges - 13 as libc::c_int
                + box_surfedges[i as usize];
        } else {
            *r_skysurfedges
                .offset(
                    i as isize,
                ) = -((*loadmodel).numedges - 13 as libc::c_int
                + -box_surfedges[i as usize]);
        }
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < 12 as libc::c_int {
        (*r_skyedges.offset(i as isize))
            .v[0 as libc::c_int
            as usize] = ((*loadmodel).numvertexes - 9 as libc::c_int
            + box_edges[(i * 2 as libc::c_int + 0 as libc::c_int) as usize])
            as libc::c_ushort;
        (*r_skyedges.offset(i as isize))
            .v[1 as libc::c_int
            as usize] = ((*loadmodel).numvertexes - 9 as libc::c_int
            + box_edges[(i * 2 as libc::c_int + 1 as libc::c_int) as usize])
            as libc::c_ushort;
        (*r_skyedges.offset(i as isize))
            .cachededgeoffset = 0 as libc::c_int as libc::c_uint;
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn R_EmitSkyBox() {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut oldkey: libc::c_int = 0;
    if insubmodel as u64 != 0 {
        return;
    }
    if r_skyframe == r_framecount {
        return;
    }
    r_skyframe = r_framecount;
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        j = 0 as libc::c_int;
        while j < 3 as libc::c_int {
            (*r_skyverts.offset(i as isize))
                .position[j
                as usize] = r_origin[j as usize]
                + box_verts[i as usize][j as usize]
                    * 128 as libc::c_int as libc::c_float;
            j += 1;
        }
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < 6 as libc::c_int {
        if skybox_planes[(i * 2 as libc::c_int + 1 as libc::c_int) as usize]
            > 0 as libc::c_int
        {
            r_skyplanes[i as usize]
                .dist = r_origin[skybox_planes[(i * 2 as libc::c_int) as usize] as usize]
                + 128 as libc::c_int as libc::c_float;
        } else {
            r_skyplanes[i as usize]
                .dist = r_origin[skybox_planes[(i * 2 as libc::c_int) as usize] as usize]
                - 128 as libc::c_int as libc::c_float;
        }
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < 6 as libc::c_int {
        r_skytexinfo[i as usize]
            .vecs[0 as libc::c_int
            as usize][3 as libc::c_int
            as usize] = -(r_origin[0 as libc::c_int as usize]
            * r_skytexinfo[i as usize]
                .vecs[0 as libc::c_int as usize][0 as libc::c_int as usize]
            + r_origin[1 as libc::c_int as usize]
                * r_skytexinfo[i as usize]
                    .vecs[0 as libc::c_int as usize][1 as libc::c_int as usize]
            + r_origin[2 as libc::c_int as usize]
                * r_skytexinfo[i as usize]
                    .vecs[0 as libc::c_int as usize][2 as libc::c_int as usize]);
        r_skytexinfo[i as usize]
            .vecs[1 as libc::c_int
            as usize][3 as libc::c_int
            as usize] = -(r_origin[0 as libc::c_int as usize]
            * r_skytexinfo[i as usize]
                .vecs[1 as libc::c_int as usize][0 as libc::c_int as usize]
            + r_origin[1 as libc::c_int as usize]
                * r_skytexinfo[i as usize]
                    .vecs[1 as libc::c_int as usize][1 as libc::c_int as usize]
            + r_origin[2 as libc::c_int as usize]
                * r_skytexinfo[i as usize]
                    .vecs[1 as libc::c_int as usize][2 as libc::c_int as usize]);
        i += 1;
    }
    oldkey = r_currentkey;
    r_currentkey = 0x7ffffff0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < 6 as libc::c_int {
        R_RenderFace(r_skyfaces.offset(i as isize), 15 as libc::c_int);
        i += 1;
    }
    r_currentkey = oldkey;
}
#[no_mangle]
pub unsafe extern "C" fn R_EmitEdge(mut pv0: *mut mvertex_t, mut pv1: *mut mvertex_t) {
    let mut edge: *mut edge_t = 0 as *mut edge_t;
    let mut pcheck: *mut edge_t = 0 as *mut edge_t;
    let mut u_check: libc::c_int = 0;
    let mut u: libc::c_float = 0.;
    let mut u_step: libc::c_float = 0.;
    let mut local: vec3_t = [0.; 3];
    let mut transformed: vec3_t = [0.; 3];
    let mut world: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut v: libc::c_int = 0;
    let mut v2: libc::c_int = 0;
    let mut ceilv0: libc::c_int = 0;
    let mut scale: libc::c_float = 0.;
    let mut lzi0: libc::c_float = 0.;
    let mut u0: libc::c_float = 0.;
    let mut v0: libc::c_float = 0.;
    let mut side: libc::c_int = 0;
    if r_lastvertvalid as u64 != 0 {
        u0 = r_u1;
        v0 = r_v1;
        lzi0 = r_lzi1;
        ceilv0 = r_ceilv1;
    } else {
        world = &mut *((*pv0).position).as_mut_ptr().offset(0 as libc::c_int as isize)
            as *mut vec_t;
        local[0 as libc::c_int
            as usize] = *world.offset(0 as libc::c_int as isize)
            - modelorg[0 as libc::c_int as usize];
        local[1 as libc::c_int
            as usize] = *world.offset(1 as libc::c_int as isize)
            - modelorg[1 as libc::c_int as usize];
        local[2 as libc::c_int
            as usize] = *world.offset(2 as libc::c_int as isize)
            - modelorg[2 as libc::c_int as usize];
        TransformVector(local.as_mut_ptr(), transformed.as_mut_ptr());
        if (transformed[2 as libc::c_int as usize] as libc::c_double) < 0.01f64 {
            transformed[2 as libc::c_int as usize] = 0.01f64 as vec_t;
        }
        lzi0 = (1.0f64 / transformed[2 as libc::c_int as usize] as libc::c_double)
            as libc::c_float;
        scale = xscale * lzi0;
        u0 = xcenter + scale * transformed[0 as libc::c_int as usize];
        if u0 < r_refdef.fvrectx_adj {
            u0 = r_refdef.fvrectx_adj;
        }
        if u0 > r_refdef.fvrectright_adj {
            u0 = r_refdef.fvrectright_adj;
        }
        scale = yscale * lzi0;
        v0 = ycenter - scale * transformed[1 as libc::c_int as usize];
        if v0 < r_refdef.fvrecty_adj {
            v0 = r_refdef.fvrecty_adj;
        }
        if v0 > r_refdef.fvrectbottom_adj {
            v0 = r_refdef.fvrectbottom_adj;
        }
        ceilv0 = ceil(v0 as libc::c_double) as libc::c_int;
    }
    world = &mut *((*pv1).position).as_mut_ptr().offset(0 as libc::c_int as isize)
        as *mut vec_t;
    local[0 as libc::c_int
        as usize] = *world.offset(0 as libc::c_int as isize)
        - modelorg[0 as libc::c_int as usize];
    local[1 as libc::c_int
        as usize] = *world.offset(1 as libc::c_int as isize)
        - modelorg[1 as libc::c_int as usize];
    local[2 as libc::c_int
        as usize] = *world.offset(2 as libc::c_int as isize)
        - modelorg[2 as libc::c_int as usize];
    TransformVector(local.as_mut_ptr(), transformed.as_mut_ptr());
    if (transformed[2 as libc::c_int as usize] as libc::c_double) < 0.01f64 {
        transformed[2 as libc::c_int as usize] = 0.01f64 as vec_t;
    }
    r_lzi1 = (1.0f64 / transformed[2 as libc::c_int as usize] as libc::c_double)
        as libc::c_float;
    scale = xscale * r_lzi1;
    r_u1 = xcenter + scale * transformed[0 as libc::c_int as usize];
    if r_u1 < r_refdef.fvrectx_adj {
        r_u1 = r_refdef.fvrectx_adj;
    }
    if r_u1 > r_refdef.fvrectright_adj {
        r_u1 = r_refdef.fvrectright_adj;
    }
    scale = yscale * r_lzi1;
    r_v1 = ycenter - scale * transformed[1 as libc::c_int as usize];
    if r_v1 < r_refdef.fvrecty_adj {
        r_v1 = r_refdef.fvrecty_adj;
    }
    if r_v1 > r_refdef.fvrectbottom_adj {
        r_v1 = r_refdef.fvrectbottom_adj;
    }
    if r_lzi1 > lzi0 {
        lzi0 = r_lzi1;
    }
    if lzi0 > r_nearzi {
        r_nearzi = lzi0;
    }
    if r_nearzionly as u64 != 0 {
        return;
    }
    r_emitted = 1 as libc::c_int;
    r_ceilv1 = ceil(r_v1 as libc::c_double) as libc::c_int;
    if ceilv0 == r_ceilv1 {
        if cacheoffset != 0x7fffffff as libc::c_int as libc::c_uint {
            cacheoffset = 0x80000000 as libc::c_uint
                | (r_framecount & 0x7fffffff as libc::c_int) as libc::c_uint;
        }
        return;
    }
    side = (ceilv0 > r_ceilv1) as libc::c_int;
    let fresh2 = edge_p;
    edge_p = edge_p.offset(1);
    edge = fresh2;
    let ref mut fresh3 = (*edge).owner;
    *fresh3 = r_pedge;
    (*edge).nearzi = lzi0;
    if side == 0 as libc::c_int {
        v = ceilv0;
        v2 = r_ceilv1 - 1 as libc::c_int;
        (*edge)
            .surfs[0 as libc::c_int
            as usize] = surface_p.offset_from(surfaces) as libc::c_long
            as libc::c_ushort;
        (*edge).surfs[1 as libc::c_int as usize] = 0 as libc::c_int as libc::c_ushort;
        u_step = (r_u1 - u0) / (r_v1 - v0);
        u = u0 + (v as libc::c_float - v0) * u_step;
    } else {
        v2 = ceilv0 - 1 as libc::c_int;
        v = r_ceilv1;
        (*edge).surfs[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_ushort;
        (*edge)
            .surfs[1 as libc::c_int
            as usize] = surface_p.offset_from(surfaces) as libc::c_long
            as libc::c_ushort;
        u_step = (u0 - r_u1) / (v0 - r_v1);
        u = r_u1 + (v as libc::c_float - r_v1) * u_step;
    }
    (*edge).u_step = (u_step * 0x100000 as libc::c_int as libc::c_float) as fixed16_t;
    (*edge)
        .u = (u * 0x100000 as libc::c_int as libc::c_float
        + 0xfffff as libc::c_int as libc::c_float) as fixed16_t;
    if (*edge).u < r_refdef.vrect_x_adj_shift20 {
        (*edge).u = r_refdef.vrect_x_adj_shift20;
    }
    if (*edge).u > r_refdef.vrectright_adj_shift20 {
        (*edge).u = r_refdef.vrectright_adj_shift20;
    }
    u_check = (*edge).u;
    if (*edge).surfs[0 as libc::c_int as usize] != 0 {
        u_check += 1;
    }
    if (newedges[v as usize]).is_null() || (*newedges[v as usize]).u >= u_check {
        let ref mut fresh4 = (*edge).next;
        *fresh4 = newedges[v as usize];
        newedges[v as usize] = edge;
    } else {
        pcheck = newedges[v as usize];
        while !((*pcheck).next).is_null() && (*(*pcheck).next).u < u_check {
            pcheck = (*pcheck).next;
        }
        let ref mut fresh5 = (*edge).next;
        *fresh5 = (*pcheck).next;
        let ref mut fresh6 = (*pcheck).next;
        *fresh6 = edge;
    }
    let ref mut fresh7 = (*edge).nextremove;
    *fresh7 = removeedges[v2 as usize];
    removeedges[v2 as usize] = edge;
}
#[no_mangle]
pub unsafe extern "C" fn R_ClipEdge(
    mut pv0: *mut mvertex_t,
    mut pv1: *mut mvertex_t,
    mut clip: *mut clipplane_t,
) {
    let mut d0: libc::c_float = 0.;
    let mut d1: libc::c_float = 0.;
    let mut f: libc::c_float = 0.;
    let mut clipvert: mvertex_t = mvertex_t { position: [0.; 3] };
    if !clip.is_null() {
        loop {
            d0 = (*pv0).position[0 as libc::c_int as usize]
                * (*clip).normal[0 as libc::c_int as usize]
                + (*pv0).position[1 as libc::c_int as usize]
                    * (*clip).normal[1 as libc::c_int as usize]
                + (*pv0).position[2 as libc::c_int as usize]
                    * (*clip).normal[2 as libc::c_int as usize] - (*clip).dist;
            d1 = (*pv1).position[0 as libc::c_int as usize]
                * (*clip).normal[0 as libc::c_int as usize]
                + (*pv1).position[1 as libc::c_int as usize]
                    * (*clip).normal[1 as libc::c_int as usize]
                + (*pv1).position[2 as libc::c_int as usize]
                    * (*clip).normal[2 as libc::c_int as usize] - (*clip).dist;
            if d0 >= 0 as libc::c_int as libc::c_float {
                if d1 >= 0 as libc::c_int as libc::c_float {
                    clip = (*clip).next;
                    if clip.is_null() {
                        break;
                    }
                } else {
                    cacheoffset = 0x7fffffff as libc::c_int as libc::c_uint;
                    f = d0 / (d0 - d1);
                    clipvert
                        .position[0 as libc::c_int
                        as usize] = (*pv0).position[0 as libc::c_int as usize]
                        + f
                            * ((*pv1).position[0 as libc::c_int as usize]
                                - (*pv0).position[0 as libc::c_int as usize]);
                    clipvert
                        .position[1 as libc::c_int
                        as usize] = (*pv0).position[1 as libc::c_int as usize]
                        + f
                            * ((*pv1).position[1 as libc::c_int as usize]
                                - (*pv0).position[1 as libc::c_int as usize]);
                    clipvert
                        .position[2 as libc::c_int
                        as usize] = (*pv0).position[2 as libc::c_int as usize]
                        + f
                            * ((*pv1).position[2 as libc::c_int as usize]
                                - (*pv0).position[2 as libc::c_int as usize]);
                    if (*clip).leftedge != 0 {
                        r_leftclipped = true_0;
                        r_leftexit = clipvert;
                    } else if (*clip).rightedge != 0 {
                        r_rightclipped = true_0;
                        r_rightexit = clipvert;
                    }
                    R_ClipEdge(pv0, &mut clipvert, (*clip).next);
                    return;
                }
            } else {
                if d1 < 0 as libc::c_int as libc::c_float {
                    if r_leftclipped as u64 == 0 {
                        cacheoffset = 0x80000000 as libc::c_uint
                            | (r_framecount & 0x7fffffff as libc::c_int) as libc::c_uint;
                    }
                    return;
                }
                r_lastvertvalid = false_0;
                cacheoffset = 0x7fffffff as libc::c_int as libc::c_uint;
                f = d0 / (d0 - d1);
                clipvert
                    .position[0 as libc::c_int
                    as usize] = (*pv0).position[0 as libc::c_int as usize]
                    + f
                        * ((*pv1).position[0 as libc::c_int as usize]
                            - (*pv0).position[0 as libc::c_int as usize]);
                clipvert
                    .position[1 as libc::c_int
                    as usize] = (*pv0).position[1 as libc::c_int as usize]
                    + f
                        * ((*pv1).position[1 as libc::c_int as usize]
                            - (*pv0).position[1 as libc::c_int as usize]);
                clipvert
                    .position[2 as libc::c_int
                    as usize] = (*pv0).position[2 as libc::c_int as usize]
                    + f
                        * ((*pv1).position[2 as libc::c_int as usize]
                            - (*pv0).position[2 as libc::c_int as usize]);
                if (*clip).leftedge != 0 {
                    r_leftclipped = true_0;
                    r_leftenter = clipvert;
                } else if (*clip).rightedge != 0 {
                    r_rightclipped = true_0;
                    r_rightenter = clipvert;
                }
                R_ClipEdge(&mut clipvert, pv1, (*clip).next);
                return;
            }
        }
    }
    R_EmitEdge(pv0, pv1);
}
#[no_mangle]
pub unsafe extern "C" fn R_EmitCachedEdge() {
    let mut pedge_t: *mut edge_t = 0 as *mut edge_t;
    pedge_t = (r_edges as libc::c_ulong)
        .wrapping_add((*r_pedge).cachededgeoffset as libc::c_ulong) as *mut edge_t;
    if (*pedge_t).surfs[0 as libc::c_int as usize] == 0 {
        (*pedge_t)
            .surfs[0 as libc::c_int
            as usize] = surface_p.offset_from(surfaces) as libc::c_long
            as libc::c_ushort;
    } else {
        (*pedge_t)
            .surfs[1 as libc::c_int
            as usize] = surface_p.offset_from(surfaces) as libc::c_long
            as libc::c_ushort;
    }
    if (*pedge_t).nearzi > r_nearzi {
        r_nearzi = (*pedge_t).nearzi;
    }
    r_emitted = 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn R_RenderFace(
    mut fa: *mut msurface_t,
    mut clipflags: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut lindex: libc::c_int = 0;
    let mut mask: libc::c_uint = 0;
    let mut pplane: *mut mplane_t = 0 as *mut mplane_t;
    let mut distinv: libc::c_float = 0.;
    let mut p_normal: vec3_t = [0.; 3];
    let mut pedges: *mut medge_t = 0 as *mut medge_t;
    let mut tedge: medge_t = medge_t {
        v: [0; 2],
        cachededgeoffset: 0,
    };
    let mut pclip: *mut clipplane_t = 0 as *mut clipplane_t;
    if (*(*fa).texinfo).flags & (0x10 as libc::c_int | 0x20 as libc::c_int) != 0 {
        let ref mut fresh8 = (*fa).nextalphasurface;
        *fresh8 = r_alpha_surfaces;
        r_alpha_surfaces = fa;
        return;
    }
    if (*(*fa).texinfo).flags & 0x4 as libc::c_int != 0 {
        R_EmitSkyBox();
        return;
    }
    if surface_p >= surf_max {
        r_outofsurfaces += 1;
        return;
    }
    if edge_p.offset((*fa).numedges as isize).offset(4 as libc::c_int as isize)
        >= edge_max
    {
        r_outofedges += (*fa).numedges;
        return;
    }
    c_faceclip += 1;
    pclip = 0 as *mut clipplane_t;
    i = 3 as libc::c_int;
    mask = 0x8 as libc::c_int as libc::c_uint;
    while i >= 0 as libc::c_int {
        if clipflags as libc::c_uint & mask != 0 {
            view_clipplanes[i as usize].next = pclip;
            pclip = &mut *view_clipplanes.as_mut_ptr().offset(i as isize)
                as *mut clipplane_t;
        }
        i -= 1;
        mask >>= 1 as libc::c_int;
    }
    r_emitted = 0 as libc::c_int;
    r_nearzi = 0 as libc::c_int as libc::c_float;
    r_nearzionly = false_0;
    makerightedge = false_0;
    makeleftedge = makerightedge;
    pedges = (*currentmodel).edges;
    r_lastvertvalid = false_0;
    let mut current_block_60: u64;
    i = 0 as libc::c_int;
    while i < (*fa).numedges {
        lindex = *((*currentmodel).surfedges).offset(((*fa).firstedge + i) as isize);
        if lindex > 0 as libc::c_int {
            r_pedge = &mut *pedges.offset(lindex as isize) as *mut medge_t;
            if insubmodel as u64 == 0 {
                if (*r_pedge).cachededgeoffset & 0x80000000 as libc::c_uint != 0 {
                    if (*r_pedge).cachededgeoffset
                        & 0x7fffffff as libc::c_int as libc::c_uint
                        == r_framecount as libc::c_uint
                    {
                        r_lastvertvalid = false_0;
                        current_block_60 = 5494826135382683477;
                    } else {
                        current_block_60 = 16799951812150840583;
                    }
                } else if (edge_p as libc::c_ulong)
                    .wrapping_sub(r_edges as libc::c_ulong)
                    > (*r_pedge).cachededgeoffset as libc::c_ulong
                    && (*((r_edges as libc::c_ulong)
                        .wrapping_add((*r_pedge).cachededgeoffset as libc::c_ulong)
                        as *mut edge_t))
                        .owner == r_pedge
                {
                    R_EmitCachedEdge();
                    r_lastvertvalid = false_0;
                    current_block_60 = 5494826135382683477;
                } else {
                    current_block_60 = 16799951812150840583;
                }
            } else {
                current_block_60 = 16799951812150840583;
            }
            match current_block_60 {
                5494826135382683477 => {}
                _ => {
                    cacheoffset = (edge_p as *mut byte).offset_from(r_edges as *mut byte)
                        as libc::c_long as libc::c_uint;
                    r_rightclipped = false_0;
                    r_leftclipped = r_rightclipped;
                    R_ClipEdge(
                        &mut *r_pcurrentvertbase
                            .offset(
                                *((*r_pedge).v)
                                    .as_mut_ptr()
                                    .offset(0 as libc::c_int as isize) as isize,
                            ),
                        &mut *r_pcurrentvertbase
                            .offset(
                                *((*r_pedge).v)
                                    .as_mut_ptr()
                                    .offset(1 as libc::c_int as isize) as isize,
                            ),
                        pclip,
                    );
                    (*r_pedge).cachededgeoffset = cacheoffset;
                    if r_leftclipped as u64 != 0 {
                        makeleftedge = true_0;
                    }
                    if r_rightclipped as u64 != 0 {
                        makerightedge = true_0;
                    }
                    r_lastvertvalid = true_0;
                }
            }
        } else {
            lindex = -lindex;
            r_pedge = &mut *pedges.offset(lindex as isize) as *mut medge_t;
            if insubmodel as u64 == 0 {
                if (*r_pedge).cachededgeoffset & 0x80000000 as libc::c_uint != 0 {
                    if (*r_pedge).cachededgeoffset
                        & 0x7fffffff as libc::c_int as libc::c_uint
                        == r_framecount as libc::c_uint
                    {
                        r_lastvertvalid = false_0;
                        current_block_60 = 5494826135382683477;
                    } else {
                        current_block_60 = 307447392441238883;
                    }
                } else if (edge_p as libc::c_ulong)
                    .wrapping_sub(r_edges as libc::c_ulong)
                    > (*r_pedge).cachededgeoffset as libc::c_ulong
                    && (*((r_edges as libc::c_ulong)
                        .wrapping_add((*r_pedge).cachededgeoffset as libc::c_ulong)
                        as *mut edge_t))
                        .owner == r_pedge
                {
                    R_EmitCachedEdge();
                    r_lastvertvalid = false_0;
                    current_block_60 = 5494826135382683477;
                } else {
                    current_block_60 = 307447392441238883;
                }
            } else {
                current_block_60 = 307447392441238883;
            }
            match current_block_60 {
                5494826135382683477 => {}
                _ => {
                    cacheoffset = (edge_p as *mut byte).offset_from(r_edges as *mut byte)
                        as libc::c_long as libc::c_uint;
                    r_rightclipped = false_0;
                    r_leftclipped = r_rightclipped;
                    R_ClipEdge(
                        &mut *r_pcurrentvertbase
                            .offset(
                                *((*r_pedge).v)
                                    .as_mut_ptr()
                                    .offset(1 as libc::c_int as isize) as isize,
                            ),
                        &mut *r_pcurrentvertbase
                            .offset(
                                *((*r_pedge).v)
                                    .as_mut_ptr()
                                    .offset(0 as libc::c_int as isize) as isize,
                            ),
                        pclip,
                    );
                    (*r_pedge).cachededgeoffset = cacheoffset;
                    if r_leftclipped as u64 != 0 {
                        makeleftedge = true_0;
                    }
                    if r_rightclipped as u64 != 0 {
                        makerightedge = true_0;
                    }
                    r_lastvertvalid = true_0;
                }
            }
        }
        i += 1;
    }
    if makeleftedge as u64 != 0 {
        r_pedge = &mut tedge;
        r_lastvertvalid = false_0;
        R_ClipEdge(&mut r_leftexit, &mut r_leftenter, (*pclip).next);
    }
    if makerightedge as u64 != 0 {
        r_pedge = &mut tedge;
        r_lastvertvalid = false_0;
        r_nearzionly = true_0;
        R_ClipEdge(
            &mut r_rightexit,
            &mut r_rightenter,
            view_clipplanes[1 as libc::c_int as usize].next,
        );
    }
    if r_emitted == 0 {
        return;
    }
    r_polycount += 1;
    let ref mut fresh9 = (*surface_p).msurf;
    *fresh9 = fa;
    (*surface_p).nearzi = r_nearzi;
    (*surface_p).flags = (*fa).flags;
    (*surface_p).insubmodel = insubmodel;
    (*surface_p).spanstate = 0 as libc::c_int;
    let ref mut fresh10 = (*surface_p).entity;
    *fresh10 = currententity;
    let fresh11 = r_currentkey;
    r_currentkey = r_currentkey + 1;
    (*surface_p).key = fresh11;
    let ref mut fresh12 = (*surface_p).spans;
    *fresh12 = 0 as *mut espan_s;
    pplane = (*fa).plane;
    TransformVector(((*pplane).normal).as_mut_ptr(), p_normal.as_mut_ptr());
    distinv = (1.0f64
        / ((*pplane).dist
            - (modelorg[0 as libc::c_int as usize]
                * (*pplane).normal[0 as libc::c_int as usize]
                + modelorg[1 as libc::c_int as usize]
                    * (*pplane).normal[1 as libc::c_int as usize]
                + modelorg[2 as libc::c_int as usize]
                    * (*pplane).normal[2 as libc::c_int as usize])) as libc::c_double)
        as libc::c_float;
    (*surface_p).d_zistepu = p_normal[0 as libc::c_int as usize] * xscaleinv * distinv;
    (*surface_p).d_zistepv = -p_normal[1 as libc::c_int as usize] * yscaleinv * distinv;
    (*surface_p)
        .d_ziorigin = p_normal[2 as libc::c_int as usize] * distinv
        - xcenter * (*surface_p).d_zistepu - ycenter * (*surface_p).d_zistepv;
    surface_p = surface_p.offset(1);
}
#[no_mangle]
pub unsafe extern "C" fn R_RenderBmodelFace(
    mut pedges: *mut bedge_t,
    mut psurf: *mut msurface_t,
) {
    let mut i: libc::c_int = 0;
    let mut mask: libc::c_uint = 0;
    let mut pplane: *mut mplane_t = 0 as *mut mplane_t;
    let mut distinv: libc::c_float = 0.;
    let mut p_normal: vec3_t = [0.; 3];
    let mut tedge: medge_t = medge_t {
        v: [0; 2],
        cachededgeoffset: 0,
    };
    let mut pclip: *mut clipplane_t = 0 as *mut clipplane_t;
    if (*(*psurf).texinfo).flags & (0x10 as libc::c_int | 0x20 as libc::c_int) != 0 {
        let ref mut fresh13 = (*psurf).nextalphasurface;
        *fresh13 = r_alpha_surfaces;
        r_alpha_surfaces = psurf;
        return;
    }
    if surface_p >= surf_max {
        r_outofsurfaces += 1;
        return;
    }
    if edge_p.offset((*psurf).numedges as isize).offset(4 as libc::c_int as isize)
        >= edge_max
    {
        r_outofedges += (*psurf).numedges;
        return;
    }
    c_faceclip += 1;
    r_pedge = &mut tedge;
    pclip = 0 as *mut clipplane_t;
    i = 3 as libc::c_int;
    mask = 0x8 as libc::c_int as libc::c_uint;
    while i >= 0 as libc::c_int {
        if r_clipflags as libc::c_uint & mask != 0 {
            view_clipplanes[i as usize].next = pclip;
            pclip = &mut *view_clipplanes.as_mut_ptr().offset(i as isize)
                as *mut clipplane_t;
        }
        i -= 1;
        mask >>= 1 as libc::c_int;
    }
    r_emitted = 0 as libc::c_int;
    r_nearzi = 0 as libc::c_int as libc::c_float;
    r_nearzionly = false_0;
    makerightedge = false_0;
    makeleftedge = makerightedge;
    r_lastvertvalid = false_0;
    while !pedges.is_null() {
        r_rightclipped = false_0;
        r_leftclipped = r_rightclipped;
        R_ClipEdge(
            (*pedges).v[0 as libc::c_int as usize],
            (*pedges).v[1 as libc::c_int as usize],
            pclip,
        );
        if r_leftclipped as u64 != 0 {
            makeleftedge = true_0;
        }
        if r_rightclipped as u64 != 0 {
            makerightedge = true_0;
        }
        pedges = (*pedges).pnext;
    }
    if makeleftedge as u64 != 0 {
        r_pedge = &mut tedge;
        R_ClipEdge(&mut r_leftexit, &mut r_leftenter, (*pclip).next);
    }
    if makerightedge as u64 != 0 {
        r_pedge = &mut tedge;
        r_nearzionly = true_0;
        R_ClipEdge(
            &mut r_rightexit,
            &mut r_rightenter,
            view_clipplanes[1 as libc::c_int as usize].next,
        );
    }
    if r_emitted == 0 {
        return;
    }
    r_polycount += 1;
    let ref mut fresh14 = (*surface_p).msurf;
    *fresh14 = psurf;
    (*surface_p).nearzi = r_nearzi;
    (*surface_p).flags = (*psurf).flags;
    (*surface_p).insubmodel = true_0;
    (*surface_p).spanstate = 0 as libc::c_int;
    let ref mut fresh15 = (*surface_p).entity;
    *fresh15 = currententity;
    (*surface_p).key = r_currentbkey;
    let ref mut fresh16 = (*surface_p).spans;
    *fresh16 = 0 as *mut espan_s;
    pplane = (*psurf).plane;
    TransformVector(((*pplane).normal).as_mut_ptr(), p_normal.as_mut_ptr());
    distinv = (1.0f64
        / ((*pplane).dist
            - (modelorg[0 as libc::c_int as usize]
                * (*pplane).normal[0 as libc::c_int as usize]
                + modelorg[1 as libc::c_int as usize]
                    * (*pplane).normal[1 as libc::c_int as usize]
                + modelorg[2 as libc::c_int as usize]
                    * (*pplane).normal[2 as libc::c_int as usize])) as libc::c_double)
        as libc::c_float;
    (*surface_p).d_zistepu = p_normal[0 as libc::c_int as usize] * xscaleinv * distinv;
    (*surface_p).d_zistepv = -p_normal[1 as libc::c_int as usize] * yscaleinv * distinv;
    (*surface_p)
        .d_ziorigin = p_normal[2 as libc::c_int as usize] * distinv
        - xcenter * (*surface_p).d_zistepu - ycenter * (*surface_p).d_zistepv;
    surface_p = surface_p.offset(1);
}
