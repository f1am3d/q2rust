#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    fn ceil(_: libc::c_double) -> libc::c_double;
    fn floor(_: libc::c_double) -> libc::c_double;
    static mut vid: viddef_t;
    static mut r_affinetridesc: affinetridesc_t;
    static mut d_viewbuffer: *mut pixel_t;
    static mut d_pzbuffer: *mut libc::c_short;
    static mut d_zwidth: libc::c_uint;
    static mut r_screenwidth: libc::c_int;
    static mut ubasestep: libc::c_int;
    static mut errorterm: libc::c_int;
    static mut erroradjustup: libc::c_int;
    static mut erroradjustdown: libc::c_int;
    static mut currententity: *mut entity_t;
    static mut r_aliasblendcolor: libc::c_int;
    static mut r_newrefdef: refdef_t;
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
pub struct dtriangle_t {
    pub index_xyz: [libc::c_short; 3],
    pub index_st: [libc::c_short; 3],
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
pub struct aliastriangleparms_t {
    pub a: *mut finalvert_t,
    pub b: *mut finalvert_t,
    pub c: *mut finalvert_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spanpackage_t {
    pub pdest: *mut libc::c_void,
    pub pz: *mut libc::c_short,
    pub count: libc::c_int,
    pub ptex: *mut byte,
    pub sfrac: libc::c_int,
    pub tfrac: libc::c_int,
    pub light: libc::c_int,
    pub zi: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct adivtab_t {
    pub quotient: libc::c_int,
    pub remainder: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct edgetable {
    pub isflattop: libc::c_int,
    pub numleftedges: libc::c_int,
    pub pleftedgevert0: *mut libc::c_int,
    pub pleftedgevert1: *mut libc::c_int,
    pub pleftedgevert2: *mut libc::c_int,
    pub numrightedges: libc::c_int,
    pub prightedgevert0: *mut libc::c_int,
    pub prightedgevert1: *mut libc::c_int,
    pub prightedgevert2: *mut libc::c_int,
}
#[no_mangle]
pub static mut rand1k: [libc::c_int; 1024] = [
    0 as libc::c_int,
    144 as libc::c_int,
    49 as libc::c_int,
    207 as libc::c_int,
    149 as libc::c_int,
    122 as libc::c_int,
    89 as libc::c_int,
    229 as libc::c_int,
    210 as libc::c_int,
    191 as libc::c_int,
    44 as libc::c_int,
    219 as libc::c_int,
    181 as libc::c_int,
    131 as libc::c_int,
    77 as libc::c_int,
    3 as libc::c_int,
    23 as libc::c_int,
    93 as libc::c_int,
    37 as libc::c_int,
    42 as libc::c_int,
    253 as libc::c_int,
    114 as libc::c_int,
    30 as libc::c_int,
    1 as libc::c_int,
    2 as libc::c_int,
    96 as libc::c_int,
    136 as libc::c_int,
    146 as libc::c_int,
    154 as libc::c_int,
    155 as libc::c_int,
    42 as libc::c_int,
    169 as libc::c_int,
    115 as libc::c_int,
    90 as libc::c_int,
    14 as libc::c_int,
    155 as libc::c_int,
    200 as libc::c_int,
    205 as libc::c_int,
    133 as libc::c_int,
    77 as libc::c_int,
    224 as libc::c_int,
    186 as libc::c_int,
    244 as libc::c_int,
    236 as libc::c_int,
    138 as libc::c_int,
    36 as libc::c_int,
    118 as libc::c_int,
    60 as libc::c_int,
    220 as libc::c_int,
    53 as libc::c_int,
    199 as libc::c_int,
    215 as libc::c_int,
    255 as libc::c_int,
    255 as libc::c_int,
    156 as libc::c_int,
    100 as libc::c_int,
    68 as libc::c_int,
    76 as libc::c_int,
    215 as libc::c_int,
    6 as libc::c_int,
    96 as libc::c_int,
    23 as libc::c_int,
    173 as libc::c_int,
    14 as libc::c_int,
    2 as libc::c_int,
    235 as libc::c_int,
    70 as libc::c_int,
    69 as libc::c_int,
    150 as libc::c_int,
    176 as libc::c_int,
    214 as libc::c_int,
    185 as libc::c_int,
    124 as libc::c_int,
    52 as libc::c_int,
    190 as libc::c_int,
    119 as libc::c_int,
    117 as libc::c_int,
    242 as libc::c_int,
    190 as libc::c_int,
    27 as libc::c_int,
    153 as libc::c_int,
    98 as libc::c_int,
    188 as libc::c_int,
    155 as libc::c_int,
    146 as libc::c_int,
    92 as libc::c_int,
    38 as libc::c_int,
    57 as libc::c_int,
    108 as libc::c_int,
    205 as libc::c_int,
    132 as libc::c_int,
    253 as libc::c_int,
    192 as libc::c_int,
    88 as libc::c_int,
    43 as libc::c_int,
    168 as libc::c_int,
    125 as libc::c_int,
    16 as libc::c_int,
    179 as libc::c_int,
    129 as libc::c_int,
    37 as libc::c_int,
    243 as libc::c_int,
    36 as libc::c_int,
    231 as libc::c_int,
    177 as libc::c_int,
    77 as libc::c_int,
    109 as libc::c_int,
    18 as libc::c_int,
    247 as libc::c_int,
    174 as libc::c_int,
    39 as libc::c_int,
    224 as libc::c_int,
    210 as libc::c_int,
    149 as libc::c_int,
    48 as libc::c_int,
    45 as libc::c_int,
    209 as libc::c_int,
    121 as libc::c_int,
    39 as libc::c_int,
    129 as libc::c_int,
    187 as libc::c_int,
    103 as libc::c_int,
    71 as libc::c_int,
    145 as libc::c_int,
    174 as libc::c_int,
    193 as libc::c_int,
    184 as libc::c_int,
    121 as libc::c_int,
    31 as libc::c_int,
    94 as libc::c_int,
    213 as libc::c_int,
    8 as libc::c_int,
    132 as libc::c_int,
    169 as libc::c_int,
    109 as libc::c_int,
    26 as libc::c_int,
    243 as libc::c_int,
    235 as libc::c_int,
    140 as libc::c_int,
    88 as libc::c_int,
    120 as libc::c_int,
    95 as libc::c_int,
    216 as libc::c_int,
    81 as libc::c_int,
    116 as libc::c_int,
    69 as libc::c_int,
    251 as libc::c_int,
    76 as libc::c_int,
    189 as libc::c_int,
    145 as libc::c_int,
    50 as libc::c_int,
    194 as libc::c_int,
    214 as libc::c_int,
    101 as libc::c_int,
    128 as libc::c_int,
    227 as libc::c_int,
    7 as libc::c_int,
    254 as libc::c_int,
    146 as libc::c_int,
    12 as libc::c_int,
    136 as libc::c_int,
    49 as libc::c_int,
    215 as libc::c_int,
    160 as libc::c_int,
    168 as libc::c_int,
    50 as libc::c_int,
    215 as libc::c_int,
    31 as libc::c_int,
    28 as libc::c_int,
    190 as libc::c_int,
    80 as libc::c_int,
    240 as libc::c_int,
    73 as libc::c_int,
    86 as libc::c_int,
    35 as libc::c_int,
    187 as libc::c_int,
    213 as libc::c_int,
    181 as libc::c_int,
    153 as libc::c_int,
    191 as libc::c_int,
    64 as libc::c_int,
    36 as libc::c_int,
    0 as libc::c_int,
    15 as libc::c_int,
    206 as libc::c_int,
    218 as libc::c_int,
    53 as libc::c_int,
    29 as libc::c_int,
    141 as libc::c_int,
    3 as libc::c_int,
    29 as libc::c_int,
    116 as libc::c_int,
    192 as libc::c_int,
    175 as libc::c_int,
    139 as libc::c_int,
    18 as libc::c_int,
    111 as libc::c_int,
    51 as libc::c_int,
    178 as libc::c_int,
    74 as libc::c_int,
    111 as libc::c_int,
    59 as libc::c_int,
    147 as libc::c_int,
    136 as libc::c_int,
    160 as libc::c_int,
    41 as libc::c_int,
    129 as libc::c_int,
    246 as libc::c_int,
    178 as libc::c_int,
    236 as libc::c_int,
    48 as libc::c_int,
    86 as libc::c_int,
    45 as libc::c_int,
    254 as libc::c_int,
    117 as libc::c_int,
    255 as libc::c_int,
    24 as libc::c_int,
    160 as libc::c_int,
    24 as libc::c_int,
    112 as libc::c_int,
    238 as libc::c_int,
    12 as libc::c_int,
    229 as libc::c_int,
    74 as libc::c_int,
    58 as libc::c_int,
    196 as libc::c_int,
    105 as libc::c_int,
    51 as libc::c_int,
    160 as libc::c_int,
    154 as libc::c_int,
    115 as libc::c_int,
    119 as libc::c_int,
    153 as libc::c_int,
    162 as libc::c_int,
    218 as libc::c_int,
    212 as libc::c_int,
    159 as libc::c_int,
    184 as libc::c_int,
    144 as libc::c_int,
    96 as libc::c_int,
    47 as libc::c_int,
    188 as libc::c_int,
    142 as libc::c_int,
    231 as libc::c_int,
    62 as libc::c_int,
    48 as libc::c_int,
    154 as libc::c_int,
    178 as libc::c_int,
    149 as libc::c_int,
    89 as libc::c_int,
    126 as libc::c_int,
    20 as libc::c_int,
    189 as libc::c_int,
    156 as libc::c_int,
    158 as libc::c_int,
    176 as libc::c_int,
    205 as libc::c_int,
    38 as libc::c_int,
    147 as libc::c_int,
    222 as libc::c_int,
    233 as libc::c_int,
    157 as libc::c_int,
    186 as libc::c_int,
    11 as libc::c_int,
    170 as libc::c_int,
    249 as libc::c_int,
    80 as libc::c_int,
    145 as libc::c_int,
    78 as libc::c_int,
    44 as libc::c_int,
    27 as libc::c_int,
    222 as libc::c_int,
    217 as libc::c_int,
    190 as libc::c_int,
    39 as libc::c_int,
    83 as libc::c_int,
    20 as libc::c_int,
    19 as libc::c_int,
    164 as libc::c_int,
    209 as libc::c_int,
    139 as libc::c_int,
    114 as libc::c_int,
    104 as libc::c_int,
    76 as libc::c_int,
    119 as libc::c_int,
    128 as libc::c_int,
    39 as libc::c_int,
    82 as libc::c_int,
    188 as libc::c_int,
    80 as libc::c_int,
    211 as libc::c_int,
    245 as libc::c_int,
    223 as libc::c_int,
    185 as libc::c_int,
    76 as libc::c_int,
    241 as libc::c_int,
    32 as libc::c_int,
    16 as libc::c_int,
    200 as libc::c_int,
    134 as libc::c_int,
    156 as libc::c_int,
    244 as libc::c_int,
    18 as libc::c_int,
    224 as libc::c_int,
    167 as libc::c_int,
    82 as libc::c_int,
    26 as libc::c_int,
    129 as libc::c_int,
    58 as libc::c_int,
    74 as libc::c_int,
    235 as libc::c_int,
    141 as libc::c_int,
    169 as libc::c_int,
    29 as libc::c_int,
    126 as libc::c_int,
    97 as libc::c_int,
    127 as libc::c_int,
    203 as libc::c_int,
    130 as libc::c_int,
    97 as libc::c_int,
    176 as libc::c_int,
    136 as libc::c_int,
    155 as libc::c_int,
    101 as libc::c_int,
    1 as libc::c_int,
    181 as libc::c_int,
    25 as libc::c_int,
    159 as libc::c_int,
    220 as libc::c_int,
    125 as libc::c_int,
    191 as libc::c_int,
    127 as libc::c_int,
    97 as libc::c_int,
    201 as libc::c_int,
    141 as libc::c_int,
    91 as libc::c_int,
    244 as libc::c_int,
    161 as libc::c_int,
    45 as libc::c_int,
    95 as libc::c_int,
    33 as libc::c_int,
    190 as libc::c_int,
    243 as libc::c_int,
    156 as libc::c_int,
    7 as libc::c_int,
    84 as libc::c_int,
    14 as libc::c_int,
    163 as libc::c_int,
    33 as libc::c_int,
    216 as libc::c_int,
    221 as libc::c_int,
    152 as libc::c_int,
    184 as libc::c_int,
    218 as libc::c_int,
    3 as libc::c_int,
    32 as libc::c_int,
    181 as libc::c_int,
    157 as libc::c_int,
    55 as libc::c_int,
    16 as libc::c_int,
    43 as libc::c_int,
    159 as libc::c_int,
    87 as libc::c_int,
    81 as libc::c_int,
    94 as libc::c_int,
    169 as libc::c_int,
    205 as libc::c_int,
    206 as libc::c_int,
    134 as libc::c_int,
    156 as libc::c_int,
    204 as libc::c_int,
    230 as libc::c_int,
    37 as libc::c_int,
    161 as libc::c_int,
    103 as libc::c_int,
    64 as libc::c_int,
    34 as libc::c_int,
    218 as libc::c_int,
    16 as libc::c_int,
    109 as libc::c_int,
    146 as libc::c_int,
    77 as libc::c_int,
    140 as libc::c_int,
    57 as libc::c_int,
    79 as libc::c_int,
    28 as libc::c_int,
    206 as libc::c_int,
    34 as libc::c_int,
    72 as libc::c_int,
    201 as libc::c_int,
    229 as libc::c_int,
    202 as libc::c_int,
    190 as libc::c_int,
    157 as libc::c_int,
    92 as libc::c_int,
    219 as libc::c_int,
    58 as libc::c_int,
    221 as libc::c_int,
    58 as libc::c_int,
    63 as libc::c_int,
    138 as libc::c_int,
    252 as libc::c_int,
    13 as libc::c_int,
    20 as libc::c_int,
    134 as libc::c_int,
    109 as libc::c_int,
    24 as libc::c_int,
    66 as libc::c_int,
    228 as libc::c_int,
    59 as libc::c_int,
    37 as libc::c_int,
    32 as libc::c_int,
    238 as libc::c_int,
    20 as libc::c_int,
    12 as libc::c_int,
    15 as libc::c_int,
    86 as libc::c_int,
    234 as libc::c_int,
    102 as libc::c_int,
    110 as libc::c_int,
    242 as libc::c_int,
    214 as libc::c_int,
    136 as libc::c_int,
    215 as libc::c_int,
    177 as libc::c_int,
    101 as libc::c_int,
    66 as libc::c_int,
    1 as libc::c_int,
    134 as libc::c_int,
    244 as libc::c_int,
    102 as libc::c_int,
    61 as libc::c_int,
    149 as libc::c_int,
    65 as libc::c_int,
    175 as libc::c_int,
    241 as libc::c_int,
    111 as libc::c_int,
    227 as libc::c_int,
    1 as libc::c_int,
    240 as libc::c_int,
    153 as libc::c_int,
    201 as libc::c_int,
    147 as libc::c_int,
    36 as libc::c_int,
    56 as libc::c_int,
    98 as libc::c_int,
    1 as libc::c_int,
    106 as libc::c_int,
    21 as libc::c_int,
    168 as libc::c_int,
    218 as libc::c_int,
    16 as libc::c_int,
    207 as libc::c_int,
    169 as libc::c_int,
    177 as libc::c_int,
    205 as libc::c_int,
    135 as libc::c_int,
    175 as libc::c_int,
    36 as libc::c_int,
    176 as libc::c_int,
    186 as libc::c_int,
    199 as libc::c_int,
    7 as libc::c_int,
    222 as libc::c_int,
    164 as libc::c_int,
    180 as libc::c_int,
    21 as libc::c_int,
    141 as libc::c_int,
    242 as libc::c_int,
    15 as libc::c_int,
    70 as libc::c_int,
    37 as libc::c_int,
    251 as libc::c_int,
    158 as libc::c_int,
    74 as libc::c_int,
    236 as libc::c_int,
    94 as libc::c_int,
    177 as libc::c_int,
    55 as libc::c_int,
    39 as libc::c_int,
    61 as libc::c_int,
    133 as libc::c_int,
    230 as libc::c_int,
    27 as libc::c_int,
    231 as libc::c_int,
    113 as libc::c_int,
    20 as libc::c_int,
    200 as libc::c_int,
    43 as libc::c_int,
    249 as libc::c_int,
    198 as libc::c_int,
    222 as libc::c_int,
    53 as libc::c_int,
    116 as libc::c_int,
    0 as libc::c_int,
    192 as libc::c_int,
    29 as libc::c_int,
    103 as libc::c_int,
    79 as libc::c_int,
    254 as libc::c_int,
    9 as libc::c_int,
    64 as libc::c_int,
    48 as libc::c_int,
    63 as libc::c_int,
    39 as libc::c_int,
    158 as libc::c_int,
    226 as libc::c_int,
    240 as libc::c_int,
    50 as libc::c_int,
    199 as libc::c_int,
    165 as libc::c_int,
    168 as libc::c_int,
    232 as libc::c_int,
    116 as libc::c_int,
    235 as libc::c_int,
    170 as libc::c_int,
    38 as libc::c_int,
    162 as libc::c_int,
    145 as libc::c_int,
    108 as libc::c_int,
    241 as libc::c_int,
    138 as libc::c_int,
    148 as libc::c_int,
    137 as libc::c_int,
    65 as libc::c_int,
    101 as libc::c_int,
    89 as libc::c_int,
    9 as libc::c_int,
    203 as libc::c_int,
    50 as libc::c_int,
    17 as libc::c_int,
    99 as libc::c_int,
    151 as libc::c_int,
    18 as libc::c_int,
    50 as libc::c_int,
    39 as libc::c_int,
    164 as libc::c_int,
    116 as libc::c_int,
    154 as libc::c_int,
    178 as libc::c_int,
    112 as libc::c_int,
    175 as libc::c_int,
    101 as libc::c_int,
    213 as libc::c_int,
    151 as libc::c_int,
    51 as libc::c_int,
    243 as libc::c_int,
    224 as libc::c_int,
    100 as libc::c_int,
    252 as libc::c_int,
    47 as libc::c_int,
    229 as libc::c_int,
    147 as libc::c_int,
    113 as libc::c_int,
    160 as libc::c_int,
    181 as libc::c_int,
    12 as libc::c_int,
    73 as libc::c_int,
    66 as libc::c_int,
    104 as libc::c_int,
    229 as libc::c_int,
    181 as libc::c_int,
    186 as libc::c_int,
    229 as libc::c_int,
    100 as libc::c_int,
    101 as libc::c_int,
    231 as libc::c_int,
    79 as libc::c_int,
    99 as libc::c_int,
    146 as libc::c_int,
    90 as libc::c_int,
    187 as libc::c_int,
    190 as libc::c_int,
    188 as libc::c_int,
    189 as libc::c_int,
    35 as libc::c_int,
    51 as libc::c_int,
    69 as libc::c_int,
    174 as libc::c_int,
    233 as libc::c_int,
    94 as libc::c_int,
    132 as libc::c_int,
    28 as libc::c_int,
    232 as libc::c_int,
    51 as libc::c_int,
    132 as libc::c_int,
    167 as libc::c_int,
    112 as libc::c_int,
    176 as libc::c_int,
    23 as libc::c_int,
    20 as libc::c_int,
    19 as libc::c_int,
    7 as libc::c_int,
    90 as libc::c_int,
    78 as libc::c_int,
    178 as libc::c_int,
    36 as libc::c_int,
    101 as libc::c_int,
    17 as libc::c_int,
    172 as libc::c_int,
    185 as libc::c_int,
    50 as libc::c_int,
    177 as libc::c_int,
    157 as libc::c_int,
    167 as libc::c_int,
    139 as libc::c_int,
    25 as libc::c_int,
    139 as libc::c_int,
    12 as libc::c_int,
    249 as libc::c_int,
    118 as libc::c_int,
    248 as libc::c_int,
    186 as libc::c_int,
    135 as libc::c_int,
    174 as libc::c_int,
    177 as libc::c_int,
    95 as libc::c_int,
    99 as libc::c_int,
    12 as libc::c_int,
    207 as libc::c_int,
    43 as libc::c_int,
    15 as libc::c_int,
    79 as libc::c_int,
    200 as libc::c_int,
    54 as libc::c_int,
    82 as libc::c_int,
    124 as libc::c_int,
    2 as libc::c_int,
    112 as libc::c_int,
    130 as libc::c_int,
    155 as libc::c_int,
    194 as libc::c_int,
    102 as libc::c_int,
    89 as libc::c_int,
    215 as libc::c_int,
    241 as libc::c_int,
    159 as libc::c_int,
    255 as libc::c_int,
    13 as libc::c_int,
    144 as libc::c_int,
    221 as libc::c_int,
    99 as libc::c_int,
    78 as libc::c_int,
    72 as libc::c_int,
    6 as libc::c_int,
    156 as libc::c_int,
    100 as libc::c_int,
    4 as libc::c_int,
    7 as libc::c_int,
    116 as libc::c_int,
    219 as libc::c_int,
    239 as libc::c_int,
    102 as libc::c_int,
    186 as libc::c_int,
    156 as libc::c_int,
    206 as libc::c_int,
    224 as libc::c_int,
    149 as libc::c_int,
    152 as libc::c_int,
    20 as libc::c_int,
    203 as libc::c_int,
    118 as libc::c_int,
    151 as libc::c_int,
    150 as libc::c_int,
    145 as libc::c_int,
    208 as libc::c_int,
    172 as libc::c_int,
    87 as libc::c_int,
    2 as libc::c_int,
    68 as libc::c_int,
    87 as libc::c_int,
    59 as libc::c_int,
    197 as libc::c_int,
    95 as libc::c_int,
    222 as libc::c_int,
    29 as libc::c_int,
    185 as libc::c_int,
    161 as libc::c_int,
    228 as libc::c_int,
    46 as libc::c_int,
    137 as libc::c_int,
    230 as libc::c_int,
    199 as libc::c_int,
    247 as libc::c_int,
    50 as libc::c_int,
    230 as libc::c_int,
    204 as libc::c_int,
    244 as libc::c_int,
    217 as libc::c_int,
    227 as libc::c_int,
    160 as libc::c_int,
    47 as libc::c_int,
    157 as libc::c_int,
    67 as libc::c_int,
    64 as libc::c_int,
    187 as libc::c_int,
    201 as libc::c_int,
    43 as libc::c_int,
    182 as libc::c_int,
    123 as libc::c_int,
    20 as libc::c_int,
    206 as libc::c_int,
    218 as libc::c_int,
    31 as libc::c_int,
    78 as libc::c_int,
    146 as libc::c_int,
    121 as libc::c_int,
    195 as libc::c_int,
    49 as libc::c_int,
    186 as libc::c_int,
    254 as libc::c_int,
    3 as libc::c_int,
    165 as libc::c_int,
    177 as libc::c_int,
    44 as libc::c_int,
    18 as libc::c_int,
    70 as libc::c_int,
    173 as libc::c_int,
    214 as libc::c_int,
    142 as libc::c_int,
    95 as libc::c_int,
    199 as libc::c_int,
    59 as libc::c_int,
    163 as libc::c_int,
    59 as libc::c_int,
    52 as libc::c_int,
    248 as libc::c_int,
    72 as libc::c_int,
    5 as libc::c_int,
    196 as libc::c_int,
    38 as libc::c_int,
    12 as libc::c_int,
    2 as libc::c_int,
    89 as libc::c_int,
    164 as libc::c_int,
    87 as libc::c_int,
    106 as libc::c_int,
    106 as libc::c_int,
    23 as libc::c_int,
    139 as libc::c_int,
    179 as libc::c_int,
    86 as libc::c_int,
    168 as libc::c_int,
    224 as libc::c_int,
    137 as libc::c_int,
    145 as libc::c_int,
    13 as libc::c_int,
    119 as libc::c_int,
    66 as libc::c_int,
    109 as libc::c_int,
    221 as libc::c_int,
    124 as libc::c_int,
    22 as libc::c_int,
    144 as libc::c_int,
    181 as libc::c_int,
    199 as libc::c_int,
    221 as libc::c_int,
    217 as libc::c_int,
    75 as libc::c_int,
    221 as libc::c_int,
    165 as libc::c_int,
    191 as libc::c_int,
    212 as libc::c_int,
    195 as libc::c_int,
    223 as libc::c_int,
    232 as libc::c_int,
    233 as libc::c_int,
    133 as libc::c_int,
    112 as libc::c_int,
    27 as libc::c_int,
    90 as libc::c_int,
    210 as libc::c_int,
    109 as libc::c_int,
    43 as libc::c_int,
    0 as libc::c_int,
    168 as libc::c_int,
    198 as libc::c_int,
    16 as libc::c_int,
    22 as libc::c_int,
    98 as libc::c_int,
    175 as libc::c_int,
    206 as libc::c_int,
    39 as libc::c_int,
    36 as libc::c_int,
    12 as libc::c_int,
    88 as libc::c_int,
    4 as libc::c_int,
    250 as libc::c_int,
    165 as libc::c_int,
    13 as libc::c_int,
    234 as libc::c_int,
    163 as libc::c_int,
    110 as libc::c_int,
    5 as libc::c_int,
    62 as libc::c_int,
    100 as libc::c_int,
    167 as libc::c_int,
    200 as libc::c_int,
    5 as libc::c_int,
    211 as libc::c_int,
    35 as libc::c_int,
    162 as libc::c_int,
    140 as libc::c_int,
    251 as libc::c_int,
    118 as libc::c_int,
    54 as libc::c_int,
    76 as libc::c_int,
    200 as libc::c_int,
    87 as libc::c_int,
    123 as libc::c_int,
    155 as libc::c_int,
    26 as libc::c_int,
    252 as libc::c_int,
    193 as libc::c_int,
    38 as libc::c_int,
    116 as libc::c_int,
    182 as libc::c_int,
    255 as libc::c_int,
    198 as libc::c_int,
    164 as libc::c_int,
    159 as libc::c_int,
    242 as libc::c_int,
    176 as libc::c_int,
    74 as libc::c_int,
    145 as libc::c_int,
    74 as libc::c_int,
    140 as libc::c_int,
    182 as libc::c_int,
    63 as libc::c_int,
    139 as libc::c_int,
    126 as libc::c_int,
    243 as libc::c_int,
    171 as libc::c_int,
    195 as libc::c_int,
    159 as libc::c_int,
    114 as libc::c_int,
    204 as libc::c_int,
    190 as libc::c_int,
    253 as libc::c_int,
    52 as libc::c_int,
    161 as libc::c_int,
    232 as libc::c_int,
    151 as libc::c_int,
    235 as libc::c_int,
    129 as libc::c_int,
    125 as libc::c_int,
    115 as libc::c_int,
    227 as libc::c_int,
    240 as libc::c_int,
    46 as libc::c_int,
    64 as libc::c_int,
    51 as libc::c_int,
    187 as libc::c_int,
    240 as libc::c_int,
    160 as libc::c_int,
    10 as libc::c_int,
    164 as libc::c_int,
    8 as libc::c_int,
    142 as libc::c_int,
    139 as libc::c_int,
    114 as libc::c_int,
    15 as libc::c_int,
    254 as libc::c_int,
    32 as libc::c_int,
    153 as libc::c_int,
    12 as libc::c_int,
    44 as libc::c_int,
    169 as libc::c_int,
    85 as libc::c_int,
    80 as libc::c_int,
    167 as libc::c_int,
    105 as libc::c_int,
    109 as libc::c_int,
    56 as libc::c_int,
    173 as libc::c_int,
    42 as libc::c_int,
    127 as libc::c_int,
    129 as libc::c_int,
    205 as libc::c_int,
    111 as libc::c_int,
    1 as libc::c_int,
    86 as libc::c_int,
    96 as libc::c_int,
    32 as libc::c_int,
    211 as libc::c_int,
    187 as libc::c_int,
    228 as libc::c_int,
    164 as libc::c_int,
    166 as libc::c_int,
    131 as libc::c_int,
    187 as libc::c_int,
    188 as libc::c_int,
    245 as libc::c_int,
    119 as libc::c_int,
    92 as libc::c_int,
    28 as libc::c_int,
    231 as libc::c_int,
    210 as libc::c_int,
    116 as libc::c_int,
    27 as libc::c_int,
    222 as libc::c_int,
    194 as libc::c_int,
    10 as libc::c_int,
    106 as libc::c_int,
    239 as libc::c_int,
    17 as libc::c_int,
    42 as libc::c_int,
    54 as libc::c_int,
    29 as libc::c_int,
    151 as libc::c_int,
    30 as libc::c_int,
    158 as libc::c_int,
    148 as libc::c_int,
    176 as libc::c_int,
    187 as libc::c_int,
    234 as libc::c_int,
    171 as libc::c_int,
    76 as libc::c_int,
    207 as libc::c_int,
    96 as libc::c_int,
    255 as libc::c_int,
    197 as libc::c_int,
    52 as libc::c_int,
    43 as libc::c_int,
    99 as libc::c_int,
    46 as libc::c_int,
    148 as libc::c_int,
    50 as libc::c_int,
    245 as libc::c_int,
    48 as libc::c_int,
    97 as libc::c_int,
    77 as libc::c_int,
    30 as libc::c_int,
    50 as libc::c_int,
    11 as libc::c_int,
    197 as libc::c_int,
    194 as libc::c_int,
    225 as libc::c_int,
    0 as libc::c_int,
    114 as libc::c_int,
    109 as libc::c_int,
    205 as libc::c_int,
    118 as libc::c_int,
    126 as libc::c_int,
    191 as libc::c_int,
    61 as libc::c_int,
    143 as libc::c_int,
    23 as libc::c_int,
    236 as libc::c_int,
    228 as libc::c_int,
    219 as libc::c_int,
    15 as libc::c_int,
    125 as libc::c_int,
    161 as libc::c_int,
    191 as libc::c_int,
    193 as libc::c_int,
    65 as libc::c_int,
    232 as libc::c_int,
    202 as libc::c_int,
    51 as libc::c_int,
    141 as libc::c_int,
    13 as libc::c_int,
    133 as libc::c_int,
    202 as libc::c_int,
    180 as libc::c_int,
    6 as libc::c_int,
    187 as libc::c_int,
    141 as libc::c_int,
    234 as libc::c_int,
    224 as libc::c_int,
    204 as libc::c_int,
    78 as libc::c_int,
    101 as libc::c_int,
    123 as libc::c_int,
    13 as libc::c_int,
    166 as libc::c_int,
    0 as libc::c_int,
    196 as libc::c_int,
    193 as libc::c_int,
    56 as libc::c_int,
    39 as libc::c_int,
    14 as libc::c_int,
    171 as libc::c_int,
    8 as libc::c_int,
    88 as libc::c_int,
    178 as libc::c_int,
    204 as libc::c_int,
    111 as libc::c_int,
    251 as libc::c_int,
    162 as libc::c_int,
    75 as libc::c_int,
    122 as libc::c_int,
    223 as libc::c_int,
    20 as libc::c_int,
    25 as libc::c_int,
    36 as libc::c_int,
    36 as libc::c_int,
    235 as libc::c_int,
    79 as libc::c_int,
    95 as libc::c_int,
    208 as libc::c_int,
    11 as libc::c_int,
    208 as libc::c_int,
    61 as libc::c_int,
    229 as libc::c_int,
    65 as libc::c_int,
    68 as libc::c_int,
    53 as libc::c_int,
    58 as libc::c_int,
    216 as libc::c_int,
    223 as libc::c_int,
    227 as libc::c_int,
    216 as libc::c_int,
    155 as libc::c_int,
    10 as libc::c_int,
    44 as libc::c_int,
    47 as libc::c_int,
    91 as libc::c_int,
    115 as libc::c_int,
    47 as libc::c_int,
    228 as libc::c_int,
    159 as libc::c_int,
    139 as libc::c_int,
    233 as libc::c_int,
];
#[no_mangle]
pub static mut rand1k_index: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut aliastriangleparms: aliastriangleparms_t = aliastriangleparms_t {
    a: 0 as *const finalvert_t as *mut finalvert_t,
    b: 0 as *const finalvert_t as *mut finalvert_t,
    c: 0 as *const finalvert_t as *mut finalvert_t,
};
#[no_mangle]
pub static mut r_p0: [libc::c_int; 6] = [0; 6];
#[no_mangle]
pub static mut r_p1: [libc::c_int; 6] = [0; 6];
#[no_mangle]
pub static mut r_p2: [libc::c_int; 6] = [0; 6];
#[no_mangle]
pub static mut d_pcolormap: *mut byte = 0 as *const byte as *mut byte;
#[no_mangle]
pub static mut d_aflatcolor: libc::c_int = 0;
#[no_mangle]
pub static mut d_xdenom: libc::c_int = 0;
#[no_mangle]
pub static mut pedgetable: *mut edgetable = 0 as *const edgetable as *mut edgetable;
#[no_mangle]
pub static mut edgetables: [edgetable; 12] = unsafe {
    [
        {
            let mut init = edgetable {
                isflattop: 0 as libc::c_int,
                numleftedges: 1 as libc::c_int,
                pleftedgevert0: r_p0.as_ptr() as *mut _,
                pleftedgevert1: r_p2.as_ptr() as *mut _,
                pleftedgevert2: 0 as *const libc::c_int as *mut libc::c_int,
                numrightedges: 2 as libc::c_int,
                prightedgevert0: r_p0.as_ptr() as *mut _,
                prightedgevert1: r_p1.as_ptr() as *mut _,
                prightedgevert2: r_p2.as_ptr() as *mut _,
            };
            init
        },
        {
            let mut init = edgetable {
                isflattop: 0 as libc::c_int,
                numleftedges: 2 as libc::c_int,
                pleftedgevert0: r_p1.as_ptr() as *mut _,
                pleftedgevert1: r_p0.as_ptr() as *mut _,
                pleftedgevert2: r_p2.as_ptr() as *mut _,
                numrightedges: 1 as libc::c_int,
                prightedgevert0: r_p1.as_ptr() as *mut _,
                prightedgevert1: r_p2.as_ptr() as *mut _,
                prightedgevert2: 0 as *const libc::c_int as *mut libc::c_int,
            };
            init
        },
        {
            let mut init = edgetable {
                isflattop: 1 as libc::c_int,
                numleftedges: 1 as libc::c_int,
                pleftedgevert0: r_p0.as_ptr() as *mut _,
                pleftedgevert1: r_p2.as_ptr() as *mut _,
                pleftedgevert2: 0 as *const libc::c_int as *mut libc::c_int,
                numrightedges: 1 as libc::c_int,
                prightedgevert0: r_p1.as_ptr() as *mut _,
                prightedgevert1: r_p2.as_ptr() as *mut _,
                prightedgevert2: 0 as *const libc::c_int as *mut libc::c_int,
            };
            init
        },
        {
            let mut init = edgetable {
                isflattop: 0 as libc::c_int,
                numleftedges: 1 as libc::c_int,
                pleftedgevert0: r_p1.as_ptr() as *mut _,
                pleftedgevert1: r_p0.as_ptr() as *mut _,
                pleftedgevert2: 0 as *const libc::c_int as *mut libc::c_int,
                numrightedges: 2 as libc::c_int,
                prightedgevert0: r_p1.as_ptr() as *mut _,
                prightedgevert1: r_p2.as_ptr() as *mut _,
                prightedgevert2: r_p0.as_ptr() as *mut _,
            };
            init
        },
        {
            let mut init = edgetable {
                isflattop: 0 as libc::c_int,
                numleftedges: 2 as libc::c_int,
                pleftedgevert0: r_p0.as_ptr() as *mut _,
                pleftedgevert1: r_p2.as_ptr() as *mut _,
                pleftedgevert2: r_p1.as_ptr() as *mut _,
                numrightedges: 1 as libc::c_int,
                prightedgevert0: r_p0.as_ptr() as *mut _,
                prightedgevert1: r_p1.as_ptr() as *mut _,
                prightedgevert2: 0 as *const libc::c_int as *mut libc::c_int,
            };
            init
        },
        {
            let mut init = edgetable {
                isflattop: 0 as libc::c_int,
                numleftedges: 1 as libc::c_int,
                pleftedgevert0: r_p2.as_ptr() as *mut _,
                pleftedgevert1: r_p1.as_ptr() as *mut _,
                pleftedgevert2: 0 as *const libc::c_int as *mut libc::c_int,
                numrightedges: 1 as libc::c_int,
                prightedgevert0: r_p2.as_ptr() as *mut _,
                prightedgevert1: r_p0.as_ptr() as *mut _,
                prightedgevert2: 0 as *const libc::c_int as *mut libc::c_int,
            };
            init
        },
        {
            let mut init = edgetable {
                isflattop: 0 as libc::c_int,
                numleftedges: 1 as libc::c_int,
                pleftedgevert0: r_p2.as_ptr() as *mut _,
                pleftedgevert1: r_p1.as_ptr() as *mut _,
                pleftedgevert2: 0 as *const libc::c_int as *mut libc::c_int,
                numrightedges: 2 as libc::c_int,
                prightedgevert0: r_p2.as_ptr() as *mut _,
                prightedgevert1: r_p0.as_ptr() as *mut _,
                prightedgevert2: r_p1.as_ptr() as *mut _,
            };
            init
        },
        {
            let mut init = edgetable {
                isflattop: 0 as libc::c_int,
                numleftedges: 2 as libc::c_int,
                pleftedgevert0: r_p2.as_ptr() as *mut _,
                pleftedgevert1: r_p1.as_ptr() as *mut _,
                pleftedgevert2: r_p0.as_ptr() as *mut _,
                numrightedges: 1 as libc::c_int,
                prightedgevert0: r_p2.as_ptr() as *mut _,
                prightedgevert1: r_p0.as_ptr() as *mut _,
                prightedgevert2: 0 as *const libc::c_int as *mut libc::c_int,
            };
            init
        },
        {
            let mut init = edgetable {
                isflattop: 0 as libc::c_int,
                numleftedges: 1 as libc::c_int,
                pleftedgevert0: r_p1.as_ptr() as *mut _,
                pleftedgevert1: r_p0.as_ptr() as *mut _,
                pleftedgevert2: 0 as *const libc::c_int as *mut libc::c_int,
                numrightedges: 1 as libc::c_int,
                prightedgevert0: r_p1.as_ptr() as *mut _,
                prightedgevert1: r_p2.as_ptr() as *mut _,
                prightedgevert2: 0 as *const libc::c_int as *mut libc::c_int,
            };
            init
        },
        {
            let mut init = edgetable {
                isflattop: 1 as libc::c_int,
                numleftedges: 1 as libc::c_int,
                pleftedgevert0: r_p2.as_ptr() as *mut _,
                pleftedgevert1: r_p1.as_ptr() as *mut _,
                pleftedgevert2: 0 as *const libc::c_int as *mut libc::c_int,
                numrightedges: 1 as libc::c_int,
                prightedgevert0: r_p0.as_ptr() as *mut _,
                prightedgevert1: r_p1.as_ptr() as *mut _,
                prightedgevert2: 0 as *const libc::c_int as *mut libc::c_int,
            };
            init
        },
        {
            let mut init = edgetable {
                isflattop: 1 as libc::c_int,
                numleftedges: 1 as libc::c_int,
                pleftedgevert0: r_p1.as_ptr() as *mut _,
                pleftedgevert1: r_p0.as_ptr() as *mut _,
                pleftedgevert2: 0 as *const libc::c_int as *mut libc::c_int,
                numrightedges: 1 as libc::c_int,
                prightedgevert0: r_p2.as_ptr() as *mut _,
                prightedgevert1: r_p0.as_ptr() as *mut _,
                prightedgevert2: 0 as *const libc::c_int as *mut libc::c_int,
            };
            init
        },
        {
            let mut init = edgetable {
                isflattop: 0 as libc::c_int,
                numleftedges: 1 as libc::c_int,
                pleftedgevert0: r_p0.as_ptr() as *mut _,
                pleftedgevert1: r_p2.as_ptr() as *mut _,
                pleftedgevert2: 0 as *const libc::c_int as *mut libc::c_int,
                numrightedges: 1 as libc::c_int,
                prightedgevert0: r_p0.as_ptr() as *mut _,
                prightedgevert1: r_p1.as_ptr() as *mut _,
                prightedgevert2: 0 as *const libc::c_int as *mut libc::c_int,
            };
            init
        },
    ]
};
#[no_mangle]
pub static mut a_sstepxfrac: libc::c_int = 0;
#[no_mangle]
pub static mut a_tstepxfrac: libc::c_int = 0;
#[no_mangle]
pub static mut r_lstepx: libc::c_int = 0;
#[no_mangle]
pub static mut a_ststepxwhole: libc::c_int = 0;
#[no_mangle]
pub static mut r_sstepx: libc::c_int = 0;
#[no_mangle]
pub static mut r_tstepx: libc::c_int = 0;
#[no_mangle]
pub static mut r_lstepy: libc::c_int = 0;
#[no_mangle]
pub static mut r_sstepy: libc::c_int = 0;
#[no_mangle]
pub static mut r_tstepy: libc::c_int = 0;
#[no_mangle]
pub static mut r_zistepx: libc::c_int = 0;
#[no_mangle]
pub static mut r_zistepy: libc::c_int = 0;
#[no_mangle]
pub static mut d_aspancount: libc::c_int = 0;
#[no_mangle]
pub static mut d_countextrastep: libc::c_int = 0;
#[no_mangle]
pub static mut a_spans: *mut spanpackage_t = 0 as *const spanpackage_t
    as *mut spanpackage_t;
#[no_mangle]
pub static mut d_pedgespanpackage: *mut spanpackage_t = 0 as *const spanpackage_t
    as *mut spanpackage_t;
static mut ystart: libc::c_int = 0;
#[no_mangle]
pub static mut d_pdest: *mut byte = 0 as *const byte as *mut byte;
#[no_mangle]
pub static mut d_ptex: *mut byte = 0 as *const byte as *mut byte;
#[no_mangle]
pub static mut d_pz: *mut libc::c_short = 0 as *const libc::c_short
    as *mut libc::c_short;
#[no_mangle]
pub static mut d_sfrac: libc::c_int = 0;
#[no_mangle]
pub static mut d_tfrac: libc::c_int = 0;
#[no_mangle]
pub static mut d_light: libc::c_int = 0;
#[no_mangle]
pub static mut d_zi: libc::c_int = 0;
#[no_mangle]
pub static mut d_ptexextrastep: libc::c_int = 0;
#[no_mangle]
pub static mut d_sfracextrastep: libc::c_int = 0;
#[no_mangle]
pub static mut d_tfracextrastep: libc::c_int = 0;
#[no_mangle]
pub static mut d_lightextrastep: libc::c_int = 0;
#[no_mangle]
pub static mut d_pdestextrastep: libc::c_int = 0;
#[no_mangle]
pub static mut d_lightbasestep: libc::c_int = 0;
#[no_mangle]
pub static mut d_pdestbasestep: libc::c_int = 0;
#[no_mangle]
pub static mut d_ptexbasestep: libc::c_int = 0;
#[no_mangle]
pub static mut d_sfracbasestep: libc::c_int = 0;
#[no_mangle]
pub static mut d_tfracbasestep: libc::c_int = 0;
#[no_mangle]
pub static mut d_ziextrastep: libc::c_int = 0;
#[no_mangle]
pub static mut d_zibasestep: libc::c_int = 0;
#[no_mangle]
pub static mut d_pzextrastep: libc::c_int = 0;
#[no_mangle]
pub static mut d_pzbasestep: libc::c_int = 0;
static mut adivtab: [adivtab_t; 1024] = [
    {
        let mut init = adivtab_t {
            quotient: 1 as libc::c_int,
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 1 as libc::c_int,
            remainder: -(1 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 1 as libc::c_int,
            remainder: -(2 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 1 as libc::c_int,
            remainder: -(3 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 1 as libc::c_int,
            remainder: -(4 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 1 as libc::c_int,
            remainder: -(5 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 1 as libc::c_int,
            remainder: -(6 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 1 as libc::c_int,
            remainder: -(7 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 2 as libc::c_int,
            remainder: -(1 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 2 as libc::c_int,
            remainder: -(3 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 3 as libc::c_int,
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 3 as libc::c_int,
            remainder: -(3 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 5 as libc::c_int,
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 7 as libc::c_int,
            remainder: -(1 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 15 as libc::c_int,
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(15 as libc::c_int),
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(8 as libc::c_int),
            remainder: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(5 as libc::c_int),
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(4 as libc::c_int),
            remainder: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(3 as libc::c_int),
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(3 as libc::c_int),
            remainder: 3 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(3 as libc::c_int),
            remainder: 6 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(2 as libc::c_int),
            remainder: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(2 as libc::c_int),
            remainder: 3 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(2 as libc::c_int),
            remainder: 5 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(2 as libc::c_int),
            remainder: 7 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(2 as libc::c_int),
            remainder: 9 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(2 as libc::c_int),
            remainder: 11 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(2 as libc::c_int),
            remainder: 13 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: -(14 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 1 as libc::c_int,
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 1 as libc::c_int,
            remainder: -(1 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 1 as libc::c_int,
            remainder: -(2 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 1 as libc::c_int,
            remainder: -(3 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 1 as libc::c_int,
            remainder: -(4 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 1 as libc::c_int,
            remainder: -(5 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 1 as libc::c_int,
            remainder: -(6 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 2 as libc::c_int,
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 2 as libc::c_int,
            remainder: -(2 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 2 as libc::c_int,
            remainder: -(4 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 3 as libc::c_int,
            remainder: -(2 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 4 as libc::c_int,
            remainder: -(2 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 7 as libc::c_int,
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 14 as libc::c_int,
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(14 as libc::c_int),
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(7 as libc::c_int),
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(5 as libc::c_int),
            remainder: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(4 as libc::c_int),
            remainder: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(3 as libc::c_int),
            remainder: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(3 as libc::c_int),
            remainder: 4 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(2 as libc::c_int),
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(2 as libc::c_int),
            remainder: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(2 as libc::c_int),
            remainder: 4 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(2 as libc::c_int),
            remainder: 6 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(2 as libc::c_int),
            remainder: 8 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(2 as libc::c_int),
            remainder: 10 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(2 as libc::c_int),
            remainder: 12 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: -(13 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: -(13 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 1 as libc::c_int,
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 1 as libc::c_int,
            remainder: -(1 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 1 as libc::c_int,
            remainder: -(2 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 1 as libc::c_int,
            remainder: -(3 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 1 as libc::c_int,
            remainder: -(4 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 1 as libc::c_int,
            remainder: -(5 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 1 as libc::c_int,
            remainder: -(6 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 2 as libc::c_int,
            remainder: -(1 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 2 as libc::c_int,
            remainder: -(3 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 3 as libc::c_int,
            remainder: -(1 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 4 as libc::c_int,
            remainder: -(1 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 6 as libc::c_int,
            remainder: -(1 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 13 as libc::c_int,
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(13 as libc::c_int),
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(7 as libc::c_int),
            remainder: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(5 as libc::c_int),
            remainder: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(4 as libc::c_int),
            remainder: 3 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(3 as libc::c_int),
            remainder: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(3 as libc::c_int),
            remainder: 5 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(2 as libc::c_int),
            remainder: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(2 as libc::c_int),
            remainder: 3 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(2 as libc::c_int),
            remainder: 5 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(2 as libc::c_int),
            remainder: 7 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(2 as libc::c_int),
            remainder: 9 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(2 as libc::c_int),
            remainder: 11 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: 3 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: -(12 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: -(12 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: -(12 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 1 as libc::c_int,
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 1 as libc::c_int,
            remainder: -(1 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 1 as libc::c_int,
            remainder: -(2 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 1 as libc::c_int,
            remainder: -(3 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 1 as libc::c_int,
            remainder: -(4 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 1 as libc::c_int,
            remainder: -(5 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 2 as libc::c_int,
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 2 as libc::c_int,
            remainder: -(2 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 3 as libc::c_int,
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 4 as libc::c_int,
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 6 as libc::c_int,
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 12 as libc::c_int,
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(12 as libc::c_int),
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(6 as libc::c_int),
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(4 as libc::c_int),
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(3 as libc::c_int),
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(3 as libc::c_int),
            remainder: 3 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(2 as libc::c_int),
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(2 as libc::c_int),
            remainder: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(2 as libc::c_int),
            remainder: 4 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(2 as libc::c_int),
            remainder: 6 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(2 as libc::c_int),
            remainder: 8 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(2 as libc::c_int),
            remainder: 10 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: 3 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: 4 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: -(11 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: -(11 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: -(11 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: -(11 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 1 as libc::c_int,
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 1 as libc::c_int,
            remainder: -(1 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 1 as libc::c_int,
            remainder: -(2 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 1 as libc::c_int,
            remainder: -(3 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 1 as libc::c_int,
            remainder: -(4 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 1 as libc::c_int,
            remainder: -(5 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 2 as libc::c_int,
            remainder: -(1 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 2 as libc::c_int,
            remainder: -(3 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 3 as libc::c_int,
            remainder: -(2 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 5 as libc::c_int,
            remainder: -(1 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 11 as libc::c_int,
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(11 as libc::c_int),
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(6 as libc::c_int),
            remainder: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(4 as libc::c_int),
            remainder: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(3 as libc::c_int),
            remainder: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(3 as libc::c_int),
            remainder: 4 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(2 as libc::c_int),
            remainder: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(2 as libc::c_int),
            remainder: 3 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(2 as libc::c_int),
            remainder: 5 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(2 as libc::c_int),
            remainder: 7 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(2 as libc::c_int),
            remainder: 9 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: 3 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: 4 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: 5 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: -(10 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: -(10 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: -(10 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: -(10 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: -(10 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 1 as libc::c_int,
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 1 as libc::c_int,
            remainder: -(1 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 1 as libc::c_int,
            remainder: -(2 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 1 as libc::c_int,
            remainder: -(3 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 1 as libc::c_int,
            remainder: -(4 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 2 as libc::c_int,
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 2 as libc::c_int,
            remainder: -(2 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 3 as libc::c_int,
            remainder: -(1 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 5 as libc::c_int,
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 10 as libc::c_int,
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(10 as libc::c_int),
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(5 as libc::c_int),
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(4 as libc::c_int),
            remainder: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(3 as libc::c_int),
            remainder: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(2 as libc::c_int),
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(2 as libc::c_int),
            remainder: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(2 as libc::c_int),
            remainder: 4 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(2 as libc::c_int),
            remainder: 6 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(2 as libc::c_int),
            remainder: 8 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: 3 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: 4 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: 5 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: 6 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: -(9 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: -(9 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: -(9 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: -(9 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: -(9 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: -(9 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 1 as libc::c_int,
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 1 as libc::c_int,
            remainder: -(1 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 1 as libc::c_int,
            remainder: -(2 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 1 as libc::c_int,
            remainder: -(3 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 1 as libc::c_int,
            remainder: -(4 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 2 as libc::c_int,
            remainder: -(1 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 3 as libc::c_int,
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 4 as libc::c_int,
            remainder: -(1 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 9 as libc::c_int,
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(9 as libc::c_int),
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(5 as libc::c_int),
            remainder: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(3 as libc::c_int),
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(3 as libc::c_int),
            remainder: 3 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(2 as libc::c_int),
            remainder: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(2 as libc::c_int),
            remainder: 3 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(2 as libc::c_int),
            remainder: 5 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(2 as libc::c_int),
            remainder: 7 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: 3 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: 4 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: 5 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: 6 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: 7 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: -(8 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: -(8 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: -(8 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: -(8 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: -(8 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: -(8 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: -(8 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 1 as libc::c_int,
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 1 as libc::c_int,
            remainder: -(1 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 1 as libc::c_int,
            remainder: -(2 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 1 as libc::c_int,
            remainder: -(3 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 2 as libc::c_int,
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 2 as libc::c_int,
            remainder: -(2 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 4 as libc::c_int,
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 8 as libc::c_int,
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(8 as libc::c_int),
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(4 as libc::c_int),
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(3 as libc::c_int),
            remainder: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(2 as libc::c_int),
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(2 as libc::c_int),
            remainder: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(2 as libc::c_int),
            remainder: 4 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(2 as libc::c_int),
            remainder: 6 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: 3 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: 4 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: 5 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: 6 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: 7 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: 8 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: -(7 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: -(7 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: -(7 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: -(7 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: -(7 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: -(7 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: -(7 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: -(7 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 1 as libc::c_int,
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 1 as libc::c_int,
            remainder: -(1 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 1 as libc::c_int,
            remainder: -(2 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 1 as libc::c_int,
            remainder: -(3 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 2 as libc::c_int,
            remainder: -(1 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 3 as libc::c_int,
            remainder: -(1 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 7 as libc::c_int,
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(7 as libc::c_int),
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(4 as libc::c_int),
            remainder: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(3 as libc::c_int),
            remainder: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(2 as libc::c_int),
            remainder: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(2 as libc::c_int),
            remainder: 3 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(2 as libc::c_int),
            remainder: 5 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: 3 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: 4 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: 5 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: 6 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: 7 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: 8 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: 9 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: -(6 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: -(6 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: -(6 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: -(6 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: -(6 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: -(6 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: -(6 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: -(6 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: -(6 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 1 as libc::c_int,
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 1 as libc::c_int,
            remainder: -(1 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 1 as libc::c_int,
            remainder: -(2 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 2 as libc::c_int,
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 3 as libc::c_int,
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 6 as libc::c_int,
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(6 as libc::c_int),
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(3 as libc::c_int),
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(2 as libc::c_int),
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(2 as libc::c_int),
            remainder: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(2 as libc::c_int),
            remainder: 4 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: 3 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: 4 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: 5 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: 6 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: 7 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: 8 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: 9 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: 10 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: -(5 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: -(5 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: -(5 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: -(5 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: -(5 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: -(5 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: -(5 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: -(5 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: -(5 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: -(5 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 1 as libc::c_int,
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 1 as libc::c_int,
            remainder: -(1 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 1 as libc::c_int,
            remainder: -(2 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 2 as libc::c_int,
            remainder: -(1 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 5 as libc::c_int,
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(5 as libc::c_int),
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(3 as libc::c_int),
            remainder: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(2 as libc::c_int),
            remainder: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(2 as libc::c_int),
            remainder: 3 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: 3 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: 4 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: 5 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: 6 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: 7 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: 8 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: 9 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: 10 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: 11 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: -(4 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: -(4 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: -(4 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: -(4 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: -(4 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: -(4 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: -(4 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: -(4 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: -(4 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: -(4 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: -(4 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 1 as libc::c_int,
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 1 as libc::c_int,
            remainder: -(1 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 2 as libc::c_int,
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 4 as libc::c_int,
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(4 as libc::c_int),
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(2 as libc::c_int),
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(2 as libc::c_int),
            remainder: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: 3 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: 4 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: 5 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: 6 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: 7 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: 8 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: 9 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: 10 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: 11 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: 12 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: -(3 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: -(3 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: -(3 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: -(3 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: -(3 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: -(3 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: -(3 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: -(3 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: -(3 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: -(3 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: -(3 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: -(3 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 1 as libc::c_int,
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 1 as libc::c_int,
            remainder: -(1 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 3 as libc::c_int,
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(3 as libc::c_int),
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(2 as libc::c_int),
            remainder: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: 3 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: 4 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: 5 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: 6 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: 7 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: 8 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: 9 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: 10 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: 11 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: 12 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: 13 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: -(2 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: -(2 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: -(2 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: -(2 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: -(2 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: -(2 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: -(2 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: -(2 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: -(2 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: -(2 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: -(2 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: -(2 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: -(2 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 1 as libc::c_int,
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 2 as libc::c_int,
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(2 as libc::c_int),
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: 3 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: 4 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: 5 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: 6 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: 7 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: 8 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: 9 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: 10 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: 11 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: 12 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: 13 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: 14 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: -(1 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: -(1 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: -(1 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: -(1 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: -(1 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: -(1 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: -(1 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: -(1 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: -(1 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: -(1 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: -(1 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: -(1 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: -(1 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: -(1 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 1 as libc::c_int,
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: 3 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: 4 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: 5 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: 6 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: 7 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: 8 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: 9 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: 10 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: 11 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: 12 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: 13 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: 14 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: 15 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: -(14 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: -(13 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: -(12 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: -(11 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: -(10 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: -(9 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: -(8 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: -(7 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: -(6 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: -(5 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: -(4 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: -(3 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: -(2 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: -(1 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 1 as libc::c_int,
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: -(13 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: -(12 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: -(11 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: -(10 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: -(9 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: -(8 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: -(7 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: -(6 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: -(5 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: -(4 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: -(3 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: -(2 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: -(1 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(2 as libc::c_int),
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 2 as libc::c_int,
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 1 as libc::c_int,
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: -(12 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: -(11 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: -(10 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: -(9 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: -(8 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: -(7 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: -(6 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: -(5 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: -(4 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: -(3 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: -(2 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: -(1 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(2 as libc::c_int),
            remainder: -(1 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(3 as libc::c_int),
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 3 as libc::c_int,
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 1 as libc::c_int,
            remainder: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 1 as libc::c_int,
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 3 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 3 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 3 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 3 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 3 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 3 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 3 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 3 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 3 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 3 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 3 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 3 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 3 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: -(11 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: -(10 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: -(9 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: -(8 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: -(7 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: -(6 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: -(5 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: -(4 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: -(3 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: -(2 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: -(1 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(2 as libc::c_int),
            remainder: -(2 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(2 as libc::c_int),
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(4 as libc::c_int),
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 4 as libc::c_int,
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 2 as libc::c_int,
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 1 as libc::c_int,
            remainder: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 1 as libc::c_int,
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 4 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 4 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 4 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 4 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 4 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 4 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 4 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 4 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 4 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 4 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 4 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 4 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: -(10 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: -(9 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: -(8 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: -(7 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: -(6 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: -(5 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: -(4 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: -(3 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: -(2 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: -(1 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(2 as libc::c_int),
            remainder: -(3 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(2 as libc::c_int),
            remainder: -(1 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(3 as libc::c_int),
            remainder: -(1 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(5 as libc::c_int),
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 5 as libc::c_int,
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 2 as libc::c_int,
            remainder: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 1 as libc::c_int,
            remainder: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 1 as libc::c_int,
            remainder: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 1 as libc::c_int,
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 5 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 5 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 5 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 5 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 5 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 5 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 5 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 5 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 5 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 5 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 5 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: -(9 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: -(8 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: -(7 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: -(6 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: -(5 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: -(4 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: -(3 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: -(2 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: -(1 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(2 as libc::c_int),
            remainder: -(4 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(2 as libc::c_int),
            remainder: -(2 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(2 as libc::c_int),
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(3 as libc::c_int),
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(6 as libc::c_int),
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 6 as libc::c_int,
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 3 as libc::c_int,
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 2 as libc::c_int,
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 1 as libc::c_int,
            remainder: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 1 as libc::c_int,
            remainder: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 1 as libc::c_int,
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 6 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 6 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 6 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 6 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 6 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 6 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 6 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 6 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 6 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 6 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: -(8 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: -(7 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: -(6 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: -(5 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: -(4 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: -(3 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: -(2 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: -(1 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(2 as libc::c_int),
            remainder: -(5 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(2 as libc::c_int),
            remainder: -(3 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(2 as libc::c_int),
            remainder: -(1 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(3 as libc::c_int),
            remainder: -(2 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(4 as libc::c_int),
            remainder: -(1 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(7 as libc::c_int),
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 7 as libc::c_int,
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 3 as libc::c_int,
            remainder: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 2 as libc::c_int,
            remainder: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 1 as libc::c_int,
            remainder: 3 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 1 as libc::c_int,
            remainder: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 1 as libc::c_int,
            remainder: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 1 as libc::c_int,
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 7 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 7 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 7 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 7 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 7 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 7 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 7 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 7 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 7 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: -(7 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: -(6 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: -(5 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: -(4 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: -(3 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: -(2 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: -(1 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(2 as libc::c_int),
            remainder: -(6 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(2 as libc::c_int),
            remainder: -(4 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(2 as libc::c_int),
            remainder: -(2 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(2 as libc::c_int),
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(3 as libc::c_int),
            remainder: -(1 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(4 as libc::c_int),
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(8 as libc::c_int),
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 8 as libc::c_int,
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 4 as libc::c_int,
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 2 as libc::c_int,
            remainder: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 2 as libc::c_int,
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 1 as libc::c_int,
            remainder: 3 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 1 as libc::c_int,
            remainder: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 1 as libc::c_int,
            remainder: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 1 as libc::c_int,
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 8 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 8 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 8 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 8 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 8 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 8 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 8 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 8 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: -(6 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: -(5 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: -(4 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: -(3 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: -(2 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: -(1 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(2 as libc::c_int),
            remainder: -(7 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(2 as libc::c_int),
            remainder: -(5 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(2 as libc::c_int),
            remainder: -(3 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(2 as libc::c_int),
            remainder: -(1 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(3 as libc::c_int),
            remainder: -(3 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(3 as libc::c_int),
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(5 as libc::c_int),
            remainder: -(1 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(9 as libc::c_int),
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 9 as libc::c_int,
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 4 as libc::c_int,
            remainder: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 3 as libc::c_int,
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 2 as libc::c_int,
            remainder: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 1 as libc::c_int,
            remainder: 4 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 1 as libc::c_int,
            remainder: 3 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 1 as libc::c_int,
            remainder: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 1 as libc::c_int,
            remainder: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 1 as libc::c_int,
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 9 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 9 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 9 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 9 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 9 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 9 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 9 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: -(5 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: -(4 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: -(3 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: -(2 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: -(1 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(2 as libc::c_int),
            remainder: -(8 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(2 as libc::c_int),
            remainder: -(6 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(2 as libc::c_int),
            remainder: -(4 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(2 as libc::c_int),
            remainder: -(2 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(2 as libc::c_int),
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(3 as libc::c_int),
            remainder: -(2 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(4 as libc::c_int),
            remainder: -(2 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(5 as libc::c_int),
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(10 as libc::c_int),
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 10 as libc::c_int,
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 5 as libc::c_int,
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 3 as libc::c_int,
            remainder: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 2 as libc::c_int,
            remainder: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 2 as libc::c_int,
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 1 as libc::c_int,
            remainder: 4 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 1 as libc::c_int,
            remainder: 3 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 1 as libc::c_int,
            remainder: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 1 as libc::c_int,
            remainder: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 1 as libc::c_int,
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 10 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 10 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 10 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 10 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 10 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 10 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: -(4 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: -(3 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: -(2 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: -(1 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(2 as libc::c_int),
            remainder: -(9 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(2 as libc::c_int),
            remainder: -(7 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(2 as libc::c_int),
            remainder: -(5 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(2 as libc::c_int),
            remainder: -(3 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(2 as libc::c_int),
            remainder: -(1 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(3 as libc::c_int),
            remainder: -(4 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(3 as libc::c_int),
            remainder: -(1 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(4 as libc::c_int),
            remainder: -(1 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(6 as libc::c_int),
            remainder: -(1 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(11 as libc::c_int),
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 11 as libc::c_int,
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 5 as libc::c_int,
            remainder: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 3 as libc::c_int,
            remainder: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 2 as libc::c_int,
            remainder: 3 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 2 as libc::c_int,
            remainder: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 1 as libc::c_int,
            remainder: 5 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 1 as libc::c_int,
            remainder: 4 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 1 as libc::c_int,
            remainder: 3 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 1 as libc::c_int,
            remainder: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 1 as libc::c_int,
            remainder: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 1 as libc::c_int,
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 11 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 11 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 11 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 11 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 11 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: -(3 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: -(2 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: -(1 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(2 as libc::c_int),
            remainder: -(10 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(2 as libc::c_int),
            remainder: -(8 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(2 as libc::c_int),
            remainder: -(6 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(2 as libc::c_int),
            remainder: -(4 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(2 as libc::c_int),
            remainder: -(2 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(2 as libc::c_int),
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(3 as libc::c_int),
            remainder: -(3 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(3 as libc::c_int),
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(4 as libc::c_int),
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(6 as libc::c_int),
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(12 as libc::c_int),
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 12 as libc::c_int,
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 6 as libc::c_int,
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 4 as libc::c_int,
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 3 as libc::c_int,
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 2 as libc::c_int,
            remainder: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 2 as libc::c_int,
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 1 as libc::c_int,
            remainder: 5 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 1 as libc::c_int,
            remainder: 4 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 1 as libc::c_int,
            remainder: 3 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 1 as libc::c_int,
            remainder: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 1 as libc::c_int,
            remainder: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 1 as libc::c_int,
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 12 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 12 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 12 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 12 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: -(2 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: -(1 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(2 as libc::c_int),
            remainder: -(11 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(2 as libc::c_int),
            remainder: -(9 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(2 as libc::c_int),
            remainder: -(7 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(2 as libc::c_int),
            remainder: -(5 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(2 as libc::c_int),
            remainder: -(3 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(2 as libc::c_int),
            remainder: -(1 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(3 as libc::c_int),
            remainder: -(5 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(3 as libc::c_int),
            remainder: -(2 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(4 as libc::c_int),
            remainder: -(3 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(5 as libc::c_int),
            remainder: -(2 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(7 as libc::c_int),
            remainder: -(1 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(13 as libc::c_int),
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 13 as libc::c_int,
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 6 as libc::c_int,
            remainder: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 4 as libc::c_int,
            remainder: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 3 as libc::c_int,
            remainder: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 2 as libc::c_int,
            remainder: 3 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 2 as libc::c_int,
            remainder: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 1 as libc::c_int,
            remainder: 6 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 1 as libc::c_int,
            remainder: 5 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 1 as libc::c_int,
            remainder: 4 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 1 as libc::c_int,
            remainder: 3 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 1 as libc::c_int,
            remainder: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 1 as libc::c_int,
            remainder: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 1 as libc::c_int,
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 13 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 13 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 13 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: -(1 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(2 as libc::c_int),
            remainder: -(12 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(2 as libc::c_int),
            remainder: -(10 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(2 as libc::c_int),
            remainder: -(8 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(2 as libc::c_int),
            remainder: -(6 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(2 as libc::c_int),
            remainder: -(4 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(2 as libc::c_int),
            remainder: -(2 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(2 as libc::c_int),
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(3 as libc::c_int),
            remainder: -(4 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(3 as libc::c_int),
            remainder: -(1 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(4 as libc::c_int),
            remainder: -(2 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(5 as libc::c_int),
            remainder: -(1 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(7 as libc::c_int),
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(14 as libc::c_int),
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 14 as libc::c_int,
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 7 as libc::c_int,
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 4 as libc::c_int,
            remainder: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 3 as libc::c_int,
            remainder: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 2 as libc::c_int,
            remainder: 4 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 2 as libc::c_int,
            remainder: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 2 as libc::c_int,
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 1 as libc::c_int,
            remainder: 6 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 1 as libc::c_int,
            remainder: 5 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 1 as libc::c_int,
            remainder: 4 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 1 as libc::c_int,
            remainder: 3 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 1 as libc::c_int,
            remainder: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 1 as libc::c_int,
            remainder: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 1 as libc::c_int,
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 14 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 14 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(1 as libc::c_int),
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(2 as libc::c_int),
            remainder: -(13 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(2 as libc::c_int),
            remainder: -(11 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(2 as libc::c_int),
            remainder: -(9 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(2 as libc::c_int),
            remainder: -(7 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(2 as libc::c_int),
            remainder: -(5 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(2 as libc::c_int),
            remainder: -(3 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(2 as libc::c_int),
            remainder: -(1 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(3 as libc::c_int),
            remainder: -(6 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(3 as libc::c_int),
            remainder: -(3 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(3 as libc::c_int),
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(4 as libc::c_int),
            remainder: -(1 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(5 as libc::c_int),
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(8 as libc::c_int),
            remainder: -(1 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(15 as libc::c_int),
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 15 as libc::c_int,
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 7 as libc::c_int,
            remainder: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 5 as libc::c_int,
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 3 as libc::c_int,
            remainder: 3 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 3 as libc::c_int,
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 2 as libc::c_int,
            remainder: 3 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 2 as libc::c_int,
            remainder: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 1 as libc::c_int,
            remainder: 7 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 1 as libc::c_int,
            remainder: 6 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 1 as libc::c_int,
            remainder: 5 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 1 as libc::c_int,
            remainder: 4 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 1 as libc::c_int,
            remainder: 3 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 1 as libc::c_int,
            remainder: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 1 as libc::c_int,
            remainder: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 1 as libc::c_int,
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 15 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(2 as libc::c_int),
            remainder: -(14 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(2 as libc::c_int),
            remainder: -(12 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(2 as libc::c_int),
            remainder: -(10 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(2 as libc::c_int),
            remainder: -(8 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(2 as libc::c_int),
            remainder: -(6 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(2 as libc::c_int),
            remainder: -(4 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(2 as libc::c_int),
            remainder: -(2 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(2 as libc::c_int),
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(3 as libc::c_int),
            remainder: -(5 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(3 as libc::c_int),
            remainder: -(2 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(4 as libc::c_int),
            remainder: -(4 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(4 as libc::c_int),
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(6 as libc::c_int),
            remainder: -(2 as libc::c_int),
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(8 as libc::c_int),
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: -(16 as libc::c_int),
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 0 as libc::c_int,
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 16 as libc::c_int,
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 8 as libc::c_int,
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 5 as libc::c_int,
            remainder: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 4 as libc::c_int,
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 3 as libc::c_int,
            remainder: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 2 as libc::c_int,
            remainder: 4 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 2 as libc::c_int,
            remainder: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 2 as libc::c_int,
            remainder: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 1 as libc::c_int,
            remainder: 7 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 1 as libc::c_int,
            remainder: 6 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 1 as libc::c_int,
            remainder: 5 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 1 as libc::c_int,
            remainder: 4 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 1 as libc::c_int,
            remainder: 3 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 1 as libc::c_int,
            remainder: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 1 as libc::c_int,
            remainder: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = adivtab_t {
            quotient: 1 as libc::c_int,
            remainder: 0 as libc::c_int,
        };
        init
    },
];
#[no_mangle]
pub static mut skintable: [*mut byte; 480] = [0 as *const byte as *mut byte; 480];
#[no_mangle]
pub static mut skinwidth: libc::c_int = 0;
#[no_mangle]
pub static mut skinstart: *mut byte = 0 as *const byte as *mut byte;
#[no_mangle]
pub static mut d_pdrawspans: Option::<unsafe extern "C" fn(*mut spanpackage_t) -> ()> = None;
#[no_mangle]
pub static mut iractive: byte = 0 as libc::c_int as byte;
#[no_mangle]
pub static mut irtable: [byte; 256] = [
    79 as libc::c_int as byte,
    78 as libc::c_int as byte,
    77 as libc::c_int as byte,
    76 as libc::c_int as byte,
    75 as libc::c_int as byte,
    74 as libc::c_int as byte,
    73 as libc::c_int as byte,
    72 as libc::c_int as byte,
    71 as libc::c_int as byte,
    70 as libc::c_int as byte,
    69 as libc::c_int as byte,
    68 as libc::c_int as byte,
    67 as libc::c_int as byte,
    66 as libc::c_int as byte,
    65 as libc::c_int as byte,
    64 as libc::c_int as byte,
    64 as libc::c_int as byte,
    65 as libc::c_int as byte,
    66 as libc::c_int as byte,
    67 as libc::c_int as byte,
    68 as libc::c_int as byte,
    69 as libc::c_int as byte,
    70 as libc::c_int as byte,
    71 as libc::c_int as byte,
    72 as libc::c_int as byte,
    73 as libc::c_int as byte,
    74 as libc::c_int as byte,
    75 as libc::c_int as byte,
    76 as libc::c_int as byte,
    77 as libc::c_int as byte,
    78 as libc::c_int as byte,
    79 as libc::c_int as byte,
    64 as libc::c_int as byte,
    65 as libc::c_int as byte,
    66 as libc::c_int as byte,
    67 as libc::c_int as byte,
    68 as libc::c_int as byte,
    69 as libc::c_int as byte,
    70 as libc::c_int as byte,
    71 as libc::c_int as byte,
    72 as libc::c_int as byte,
    73 as libc::c_int as byte,
    74 as libc::c_int as byte,
    75 as libc::c_int as byte,
    76 as libc::c_int as byte,
    77 as libc::c_int as byte,
    78 as libc::c_int as byte,
    79 as libc::c_int as byte,
    208 as libc::c_int as byte,
    208 as libc::c_int as byte,
    208 as libc::c_int as byte,
    208 as libc::c_int as byte,
    208 as libc::c_int as byte,
    208 as libc::c_int as byte,
    208 as libc::c_int as byte,
    208 as libc::c_int as byte,
    64 as libc::c_int as byte,
    66 as libc::c_int as byte,
    68 as libc::c_int as byte,
    70 as libc::c_int as byte,
    72 as libc::c_int as byte,
    74 as libc::c_int as byte,
    76 as libc::c_int as byte,
    78 as libc::c_int as byte,
    64 as libc::c_int as byte,
    65 as libc::c_int as byte,
    66 as libc::c_int as byte,
    67 as libc::c_int as byte,
    68 as libc::c_int as byte,
    69 as libc::c_int as byte,
    70 as libc::c_int as byte,
    71 as libc::c_int as byte,
    72 as libc::c_int as byte,
    73 as libc::c_int as byte,
    74 as libc::c_int as byte,
    75 as libc::c_int as byte,
    76 as libc::c_int as byte,
    77 as libc::c_int as byte,
    78 as libc::c_int as byte,
    79 as libc::c_int as byte,
    64 as libc::c_int as byte,
    65 as libc::c_int as byte,
    66 as libc::c_int as byte,
    67 as libc::c_int as byte,
    68 as libc::c_int as byte,
    69 as libc::c_int as byte,
    70 as libc::c_int as byte,
    71 as libc::c_int as byte,
    72 as libc::c_int as byte,
    73 as libc::c_int as byte,
    74 as libc::c_int as byte,
    75 as libc::c_int as byte,
    76 as libc::c_int as byte,
    77 as libc::c_int as byte,
    78 as libc::c_int as byte,
    79 as libc::c_int as byte,
    64 as libc::c_int as byte,
    66 as libc::c_int as byte,
    68 as libc::c_int as byte,
    70 as libc::c_int as byte,
    72 as libc::c_int as byte,
    74 as libc::c_int as byte,
    76 as libc::c_int as byte,
    78 as libc::c_int as byte,
    68 as libc::c_int as byte,
    67 as libc::c_int as byte,
    66 as libc::c_int as byte,
    65 as libc::c_int as byte,
    64 as libc::c_int as byte,
    65 as libc::c_int as byte,
    66 as libc::c_int as byte,
    67 as libc::c_int as byte,
    68 as libc::c_int as byte,
    69 as libc::c_int as byte,
    70 as libc::c_int as byte,
    71 as libc::c_int as byte,
    72 as libc::c_int as byte,
    73 as libc::c_int as byte,
    74 as libc::c_int as byte,
    75 as libc::c_int as byte,
    76 as libc::c_int as byte,
    76 as libc::c_int as byte,
    77 as libc::c_int as byte,
    77 as libc::c_int as byte,
    78 as libc::c_int as byte,
    78 as libc::c_int as byte,
    79 as libc::c_int as byte,
    79 as libc::c_int as byte,
    64 as libc::c_int as byte,
    65 as libc::c_int as byte,
    66 as libc::c_int as byte,
    67 as libc::c_int as byte,
    68 as libc::c_int as byte,
    69 as libc::c_int as byte,
    70 as libc::c_int as byte,
    71 as libc::c_int as byte,
    72 as libc::c_int as byte,
    73 as libc::c_int as byte,
    74 as libc::c_int as byte,
    75 as libc::c_int as byte,
    76 as libc::c_int as byte,
    77 as libc::c_int as byte,
    78 as libc::c_int as byte,
    79 as libc::c_int as byte,
    64 as libc::c_int as byte,
    65 as libc::c_int as byte,
    66 as libc::c_int as byte,
    67 as libc::c_int as byte,
    68 as libc::c_int as byte,
    69 as libc::c_int as byte,
    70 as libc::c_int as byte,
    71 as libc::c_int as byte,
    72 as libc::c_int as byte,
    73 as libc::c_int as byte,
    74 as libc::c_int as byte,
    75 as libc::c_int as byte,
    76 as libc::c_int as byte,
    77 as libc::c_int as byte,
    78 as libc::c_int as byte,
    79 as libc::c_int as byte,
    64 as libc::c_int as byte,
    65 as libc::c_int as byte,
    66 as libc::c_int as byte,
    67 as libc::c_int as byte,
    68 as libc::c_int as byte,
    69 as libc::c_int as byte,
    70 as libc::c_int as byte,
    71 as libc::c_int as byte,
    72 as libc::c_int as byte,
    73 as libc::c_int as byte,
    74 as libc::c_int as byte,
    75 as libc::c_int as byte,
    76 as libc::c_int as byte,
    77 as libc::c_int as byte,
    78 as libc::c_int as byte,
    79 as libc::c_int as byte,
    64 as libc::c_int as byte,
    65 as libc::c_int as byte,
    66 as libc::c_int as byte,
    67 as libc::c_int as byte,
    68 as libc::c_int as byte,
    69 as libc::c_int as byte,
    70 as libc::c_int as byte,
    71 as libc::c_int as byte,
    72 as libc::c_int as byte,
    73 as libc::c_int as byte,
    74 as libc::c_int as byte,
    75 as libc::c_int as byte,
    76 as libc::c_int as byte,
    77 as libc::c_int as byte,
    78 as libc::c_int as byte,
    79 as libc::c_int as byte,
    64 as libc::c_int as byte,
    65 as libc::c_int as byte,
    66 as libc::c_int as byte,
    67 as libc::c_int as byte,
    68 as libc::c_int as byte,
    69 as libc::c_int as byte,
    70 as libc::c_int as byte,
    71 as libc::c_int as byte,
    72 as libc::c_int as byte,
    73 as libc::c_int as byte,
    74 as libc::c_int as byte,
    75 as libc::c_int as byte,
    76 as libc::c_int as byte,
    77 as libc::c_int as byte,
    78 as libc::c_int as byte,
    79 as libc::c_int as byte,
    64 as libc::c_int as byte,
    65 as libc::c_int as byte,
    66 as libc::c_int as byte,
    67 as libc::c_int as byte,
    68 as libc::c_int as byte,
    69 as libc::c_int as byte,
    70 as libc::c_int as byte,
    71 as libc::c_int as byte,
    64 as libc::c_int as byte,
    65 as libc::c_int as byte,
    66 as libc::c_int as byte,
    67 as libc::c_int as byte,
    68 as libc::c_int as byte,
    69 as libc::c_int as byte,
    70 as libc::c_int as byte,
    71 as libc::c_int as byte,
    64 as libc::c_int as byte,
    65 as libc::c_int as byte,
    66 as libc::c_int as byte,
    67 as libc::c_int as byte,
    68 as libc::c_int as byte,
    69 as libc::c_int as byte,
    70 as libc::c_int as byte,
    71 as libc::c_int as byte,
    72 as libc::c_int as byte,
    73 as libc::c_int as byte,
    74 as libc::c_int as byte,
    75 as libc::c_int as byte,
    76 as libc::c_int as byte,
    77 as libc::c_int as byte,
    78 as libc::c_int as byte,
    79 as libc::c_int as byte,
    208 as libc::c_int as byte,
    208 as libc::c_int as byte,
    64 as libc::c_int as byte,
    64 as libc::c_int as byte,
    70 as libc::c_int as byte,
    71 as libc::c_int as byte,
    72 as libc::c_int as byte,
    64 as libc::c_int as byte,
    66 as libc::c_int as byte,
    68 as libc::c_int as byte,
    70 as libc::c_int as byte,
    64 as libc::c_int as byte,
    65 as libc::c_int as byte,
    66 as libc::c_int as byte,
    67 as libc::c_int as byte,
    68 as libc::c_int as byte,
];
#[no_mangle]
pub unsafe extern "C" fn R_PolysetUpdateTables() {
    let mut i: libc::c_int = 0;
    let mut s: *mut byte = 0 as *mut byte;
    if r_affinetridesc.skinwidth != skinwidth
        || r_affinetridesc.pskin != skinstart as *mut libc::c_void
    {
        skinwidth = r_affinetridesc.skinwidth;
        skinstart = r_affinetridesc.pskin as *mut byte;
        s = skinstart;
        i = 0 as libc::c_int;
        while i < 480 as libc::c_int {
            skintable[i as usize] = s;
            i += 1;
            s = s.offset(skinwidth as isize);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn R_DrawTriangle() {
    let mut spans: [spanpackage_t; 1201] = [spanpackage_t {
        pdest: 0 as *mut libc::c_void,
        pz: 0 as *mut libc::c_short,
        count: 0,
        ptex: 0 as *mut byte,
        sfrac: 0,
        tfrac: 0,
        light: 0,
        zi: 0,
    }; 1201];
    let mut dv1_ab: libc::c_int = 0;
    let mut dv0_ac: libc::c_int = 0;
    let mut dv0_ab: libc::c_int = 0;
    let mut dv1_ac: libc::c_int = 0;
    dv0_ab = (*aliastriangleparms.a).u - (*aliastriangleparms.b).u;
    dv1_ab = (*aliastriangleparms.a).v - (*aliastriangleparms.b).v;
    if dv0_ab | dv1_ab == 0 {
        return;
    }
    dv0_ac = (*aliastriangleparms.a).u - (*aliastriangleparms.c).u;
    dv1_ac = (*aliastriangleparms.a).v - (*aliastriangleparms.c).v;
    if dv0_ac | dv1_ac == 0 {
        return;
    }
    d_xdenom = dv0_ac * dv1_ab - dv0_ab * dv1_ac;
    if d_xdenom < 0 as libc::c_int {
        a_spans = spans.as_mut_ptr();
        r_p0[0 as libc::c_int as usize] = (*aliastriangleparms.a).u;
        r_p0[1 as libc::c_int as usize] = (*aliastriangleparms.a).v;
        r_p0[2 as libc::c_int as usize] = (*aliastriangleparms.a).s;
        r_p0[3 as libc::c_int as usize] = (*aliastriangleparms.a).t;
        r_p0[4 as libc::c_int as usize] = (*aliastriangleparms.a).l;
        r_p0[5 as libc::c_int as usize] = (*aliastriangleparms.a).zi;
        r_p1[0 as libc::c_int as usize] = (*aliastriangleparms.b).u;
        r_p1[1 as libc::c_int as usize] = (*aliastriangleparms.b).v;
        r_p1[2 as libc::c_int as usize] = (*aliastriangleparms.b).s;
        r_p1[3 as libc::c_int as usize] = (*aliastriangleparms.b).t;
        r_p1[4 as libc::c_int as usize] = (*aliastriangleparms.b).l;
        r_p1[5 as libc::c_int as usize] = (*aliastriangleparms.b).zi;
        r_p2[0 as libc::c_int as usize] = (*aliastriangleparms.c).u;
        r_p2[1 as libc::c_int as usize] = (*aliastriangleparms.c).v;
        r_p2[2 as libc::c_int as usize] = (*aliastriangleparms.c).s;
        r_p2[3 as libc::c_int as usize] = (*aliastriangleparms.c).t;
        r_p2[4 as libc::c_int as usize] = (*aliastriangleparms.c).l;
        r_p2[5 as libc::c_int as usize] = (*aliastriangleparms.c).zi;
        R_PolysetSetEdgeTable();
        R_RasterizeAliasPolySmooth();
    }
}
#[no_mangle]
pub unsafe extern "C" fn R_PolysetScanLeftEdge_C(mut height: libc::c_int) {
    loop {
        let ref mut fresh0 = (*d_pedgespanpackage).pdest;
        *fresh0 = d_pdest as *mut libc::c_void;
        let ref mut fresh1 = (*d_pedgespanpackage).pz;
        *fresh1 = d_pz;
        (*d_pedgespanpackage).count = d_aspancount;
        let ref mut fresh2 = (*d_pedgespanpackage).ptex;
        *fresh2 = d_ptex;
        (*d_pedgespanpackage).sfrac = d_sfrac;
        (*d_pedgespanpackage).tfrac = d_tfrac;
        (*d_pedgespanpackage).light = d_light;
        (*d_pedgespanpackage).zi = d_zi;
        d_pedgespanpackage = d_pedgespanpackage.offset(1);
        errorterm += erroradjustup;
        if errorterm >= 0 as libc::c_int {
            d_pdest = d_pdest.offset(d_pdestextrastep as isize);
            d_pz = d_pz.offset(d_pzextrastep as isize);
            d_aspancount += d_countextrastep;
            d_ptex = d_ptex.offset(d_ptexextrastep as isize);
            d_sfrac += d_sfracextrastep;
            d_ptex = d_ptex.offset((d_sfrac >> 16 as libc::c_int) as isize);
            d_sfrac &= 0xffff as libc::c_int;
            d_tfrac += d_tfracextrastep;
            if d_tfrac & 0x10000 as libc::c_int != 0 {
                d_ptex = d_ptex.offset(r_affinetridesc.skinwidth as isize);
                d_tfrac &= 0xffff as libc::c_int;
            }
            d_light += d_lightextrastep;
            d_zi += d_ziextrastep;
            errorterm -= erroradjustdown;
        } else {
            d_pdest = d_pdest.offset(d_pdestbasestep as isize);
            d_pz = d_pz.offset(d_pzbasestep as isize);
            d_aspancount += ubasestep;
            d_ptex = d_ptex.offset(d_ptexbasestep as isize);
            d_sfrac += d_sfracbasestep;
            d_ptex = d_ptex.offset((d_sfrac >> 16 as libc::c_int) as isize);
            d_sfrac &= 0xffff as libc::c_int;
            d_tfrac += d_tfracbasestep;
            if d_tfrac & 0x10000 as libc::c_int != 0 {
                d_ptex = d_ptex.offset(r_affinetridesc.skinwidth as isize);
                d_tfrac &= 0xffff as libc::c_int;
            }
            d_light += d_lightbasestep;
            d_zi += d_zibasestep;
        }
        height -= 1;
        if !(height != 0) {
            break;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn FloorDivMod(
    mut numer: libc::c_float,
    mut denom: libc::c_float,
    mut quotient: *mut libc::c_int,
    mut rem: *mut libc::c_int,
) {
    let mut q: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    let mut x: libc::c_float = 0.;
    if numer as libc::c_double >= 0.0f64 {
        x = floor((numer / denom) as libc::c_double) as libc::c_float;
        q = x as libc::c_int;
        r = floor((numer - x * denom) as libc::c_double) as libc::c_int;
    } else {
        x = floor((-numer / denom) as libc::c_double) as libc::c_float;
        q = -(x as libc::c_int);
        r = floor((-numer - x * denom) as libc::c_double) as libc::c_int;
        if r != 0 as libc::c_int {
            q -= 1;
            r = denom as libc::c_int - r;
        }
    }
    *quotient = q;
    *rem = r;
}
#[no_mangle]
pub unsafe extern "C" fn R_PolysetSetUpForLineScan(
    mut startvertu: fixed8_t,
    mut startvertv: fixed8_t,
    mut endvertu: fixed8_t,
    mut endvertv: fixed8_t,
) {
    let mut dm: libc::c_float = 0.;
    let mut dn: libc::c_float = 0.;
    let mut tm: libc::c_int = 0;
    let mut tn: libc::c_int = 0;
    let mut ptemp: *mut adivtab_t = 0 as *mut adivtab_t;
    errorterm = -(1 as libc::c_int);
    tm = endvertu - startvertu;
    tn = endvertv - startvertv;
    if tm <= 16 as libc::c_int && tm >= -(15 as libc::c_int)
        && (tn <= 16 as libc::c_int && tn >= -(15 as libc::c_int))
    {
        ptemp = &mut *adivtab
            .as_mut_ptr()
            .offset(
                (((tm + 15 as libc::c_int) << 5 as libc::c_int)
                    + (tn + 15 as libc::c_int)) as isize,
            ) as *mut adivtab_t;
        ubasestep = (*ptemp).quotient;
        erroradjustup = (*ptemp).remainder;
        erroradjustdown = tn;
    } else {
        dm = tm as libc::c_float;
        dn = tn as libc::c_float;
        FloorDivMod(dm, dn, &mut ubasestep, &mut erroradjustup);
        erroradjustdown = dn as libc::c_int;
    };
}
#[no_mangle]
pub unsafe extern "C" fn R_PolysetCalcGradients(mut skinwidth_0: libc::c_int) {
    let mut xstepdenominv: libc::c_float = 0.;
    let mut ystepdenominv: libc::c_float = 0.;
    let mut t0: libc::c_float = 0.;
    let mut t1: libc::c_float = 0.;
    let mut p01_minus_p21: libc::c_float = 0.;
    let mut p11_minus_p21: libc::c_float = 0.;
    let mut p00_minus_p20: libc::c_float = 0.;
    let mut p10_minus_p20: libc::c_float = 0.;
    p00_minus_p20 = (r_p0[0 as libc::c_int as usize] - r_p2[0 as libc::c_int as usize])
        as libc::c_float;
    p01_minus_p21 = (r_p0[1 as libc::c_int as usize] - r_p2[1 as libc::c_int as usize])
        as libc::c_float;
    p10_minus_p20 = (r_p1[0 as libc::c_int as usize] - r_p2[0 as libc::c_int as usize])
        as libc::c_float;
    p11_minus_p21 = (r_p1[1 as libc::c_int as usize] - r_p2[1 as libc::c_int as usize])
        as libc::c_float;
    xstepdenominv = (1.0f64 / d_xdenom as libc::c_float as libc::c_double)
        as libc::c_float;
    ystepdenominv = -xstepdenominv;
    t0 = (r_p0[4 as libc::c_int as usize] - r_p2[4 as libc::c_int as usize])
        as libc::c_float;
    t1 = (r_p1[4 as libc::c_int as usize] - r_p2[4 as libc::c_int as usize])
        as libc::c_float;
    r_lstepx = ceil(
        ((t1 * p01_minus_p21 - t0 * p11_minus_p21) * xstepdenominv) as libc::c_double,
    ) as libc::c_int;
    r_lstepy = ceil(
        ((t1 * p00_minus_p20 - t0 * p10_minus_p20) * ystepdenominv) as libc::c_double,
    ) as libc::c_int;
    t0 = (r_p0[2 as libc::c_int as usize] - r_p2[2 as libc::c_int as usize])
        as libc::c_float;
    t1 = (r_p1[2 as libc::c_int as usize] - r_p2[2 as libc::c_int as usize])
        as libc::c_float;
    r_sstepx = ((t1 * p01_minus_p21 - t0 * p11_minus_p21) * xstepdenominv)
        as libc::c_int;
    r_sstepy = ((t1 * p00_minus_p20 - t0 * p10_minus_p20) * ystepdenominv)
        as libc::c_int;
    t0 = (r_p0[3 as libc::c_int as usize] - r_p2[3 as libc::c_int as usize])
        as libc::c_float;
    t1 = (r_p1[3 as libc::c_int as usize] - r_p2[3 as libc::c_int as usize])
        as libc::c_float;
    r_tstepx = ((t1 * p01_minus_p21 - t0 * p11_minus_p21) * xstepdenominv)
        as libc::c_int;
    r_tstepy = ((t1 * p00_minus_p20 - t0 * p10_minus_p20) * ystepdenominv)
        as libc::c_int;
    t0 = (r_p0[5 as libc::c_int as usize] - r_p2[5 as libc::c_int as usize])
        as libc::c_float;
    t1 = (r_p1[5 as libc::c_int as usize] - r_p2[5 as libc::c_int as usize])
        as libc::c_float;
    r_zistepx = ((t1 * p01_minus_p21 - t0 * p11_minus_p21) * xstepdenominv)
        as libc::c_int;
    r_zistepy = ((t1 * p00_minus_p20 - t0 * p10_minus_p20) * ystepdenominv)
        as libc::c_int;
    a_sstepxfrac = r_sstepx & 0xffff as libc::c_int;
    a_tstepxfrac = r_tstepx & 0xffff as libc::c_int;
    a_ststepxwhole = skinwidth_0 * (r_tstepx >> 16 as libc::c_int)
        + (r_sstepx >> 16 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn R_PolysetDrawThreshSpans8(
    mut pspanpackage: *mut spanpackage_t,
) {
    let mut lcount: libc::c_int = 0;
    let mut lpdest: *mut byte = 0 as *mut byte;
    let mut lptex: *mut byte = 0 as *mut byte;
    let mut lsfrac: libc::c_int = 0;
    let mut ltfrac: libc::c_int = 0;
    let mut llight: libc::c_int = 0;
    let mut lzi: libc::c_int = 0;
    let mut lpz: *mut libc::c_short = 0 as *mut libc::c_short;
    loop {
        lcount = d_aspancount - (*pspanpackage).count;
        errorterm += erroradjustup;
        if errorterm >= 0 as libc::c_int {
            d_aspancount += d_countextrastep;
            errorterm -= erroradjustdown;
        } else {
            d_aspancount += ubasestep;
        }
        if lcount != 0 {
            lpdest = (*pspanpackage).pdest as *mut byte;
            lptex = (*pspanpackage).ptex;
            lpz = (*pspanpackage).pz;
            lsfrac = (*pspanpackage).sfrac;
            ltfrac = (*pspanpackage).tfrac;
            llight = (*pspanpackage).light;
            lzi = (*pspanpackage).zi;
            loop {
                if lzi >> 16 as libc::c_int >= *lpz as libc::c_int {
                    rand1k_index = rand1k_index + 1 as libc::c_int
                        & 0x3ff as libc::c_int;
                    if rand1k[rand1k_index as usize] <= r_affinetridesc.vis_thresh {
                        *lpdest = *(vid.colormap as *mut byte)
                            .offset(
                                (*lptex as libc::c_int + (llight & 0xff00 as libc::c_int))
                                    as isize,
                            );
                        *lpz = (lzi >> 16 as libc::c_int) as libc::c_short;
                    }
                }
                lpdest = lpdest.offset(1);
                lzi += r_zistepx;
                lpz = lpz.offset(1);
                llight += r_lstepx;
                lptex = lptex.offset(a_ststepxwhole as isize);
                lsfrac += a_sstepxfrac;
                lptex = lptex.offset((lsfrac >> 16 as libc::c_int) as isize);
                lsfrac &= 0xffff as libc::c_int;
                ltfrac += a_tstepxfrac;
                if ltfrac & 0x10000 as libc::c_int != 0 {
                    lptex = lptex.offset(r_affinetridesc.skinwidth as isize);
                    ltfrac &= 0xffff as libc::c_int;
                }
                lcount -= 1;
                if !(lcount != 0) {
                    break;
                }
            }
        }
        pspanpackage = pspanpackage.offset(1);
        if !((*pspanpackage).count != -(999999 as libc::c_int)) {
            break;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn R_PolysetDrawSpans8_33(mut pspanpackage: *mut spanpackage_t) {
    let mut lcount: libc::c_int = 0;
    let mut lpdest: *mut byte = 0 as *mut byte;
    let mut lptex: *mut byte = 0 as *mut byte;
    let mut lsfrac: libc::c_int = 0;
    let mut ltfrac: libc::c_int = 0;
    let mut llight: libc::c_int = 0;
    let mut lzi: libc::c_int = 0;
    let mut lpz: *mut libc::c_short = 0 as *mut libc::c_short;
    loop {
        lcount = d_aspancount - (*pspanpackage).count;
        errorterm += erroradjustup;
        if errorterm >= 0 as libc::c_int {
            d_aspancount += d_countextrastep;
            errorterm -= erroradjustdown;
        } else {
            d_aspancount += ubasestep;
        }
        if lcount != 0 {
            lpdest = (*pspanpackage).pdest as *mut byte;
            lptex = (*pspanpackage).ptex;
            lpz = (*pspanpackage).pz;
            lsfrac = (*pspanpackage).sfrac;
            ltfrac = (*pspanpackage).tfrac;
            llight = (*pspanpackage).light;
            lzi = (*pspanpackage).zi;
            loop {
                if lzi >> 16 as libc::c_int >= *lpz as libc::c_int {
                    let mut temp: libc::c_int = *(vid.colormap)
                        .offset(
                            (*lptex as libc::c_int + (llight & 0xff00 as libc::c_int))
                                as isize,
                        ) as libc::c_int;
                    *lpdest = *(vid.alphamap)
                        .offset(
                            (temp + *lpdest as libc::c_int * 256 as libc::c_int) as isize,
                        );
                }
                lpdest = lpdest.offset(1);
                lzi += r_zistepx;
                lpz = lpz.offset(1);
                llight += r_lstepx;
                lptex = lptex.offset(a_ststepxwhole as isize);
                lsfrac += a_sstepxfrac;
                lptex = lptex.offset((lsfrac >> 16 as libc::c_int) as isize);
                lsfrac &= 0xffff as libc::c_int;
                ltfrac += a_tstepxfrac;
                if ltfrac & 0x10000 as libc::c_int != 0 {
                    lptex = lptex.offset(r_affinetridesc.skinwidth as isize);
                    ltfrac &= 0xffff as libc::c_int;
                }
                lcount -= 1;
                if !(lcount != 0) {
                    break;
                }
            }
        }
        pspanpackage = pspanpackage.offset(1);
        if !((*pspanpackage).count != -(999999 as libc::c_int)) {
            break;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn R_PolysetDrawSpansConstant8_33(
    mut pspanpackage: *mut spanpackage_t,
) {
    let mut lcount: libc::c_int = 0;
    let mut lpdest: *mut byte = 0 as *mut byte;
    let mut lzi: libc::c_int = 0;
    let mut lpz: *mut libc::c_short = 0 as *mut libc::c_short;
    loop {
        lcount = d_aspancount - (*pspanpackage).count;
        errorterm += erroradjustup;
        if errorterm >= 0 as libc::c_int {
            d_aspancount += d_countextrastep;
            errorterm -= erroradjustdown;
        } else {
            d_aspancount += ubasestep;
        }
        if lcount != 0 {
            lpdest = (*pspanpackage).pdest as *mut byte;
            lpz = (*pspanpackage).pz;
            lzi = (*pspanpackage).zi;
            loop {
                if lzi >> 16 as libc::c_int >= *lpz as libc::c_int {
                    *lpdest = *(vid.alphamap)
                        .offset(
                            (r_aliasblendcolor
                                + *lpdest as libc::c_int * 256 as libc::c_int) as isize,
                        );
                }
                lpdest = lpdest.offset(1);
                lzi += r_zistepx;
                lpz = lpz.offset(1);
                lcount -= 1;
                if !(lcount != 0) {
                    break;
                }
            }
        }
        pspanpackage = pspanpackage.offset(1);
        if !((*pspanpackage).count != -(999999 as libc::c_int)) {
            break;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn R_PolysetDrawSpans8_66(mut pspanpackage: *mut spanpackage_t) {
    let mut lcount: libc::c_int = 0;
    let mut lpdest: *mut byte = 0 as *mut byte;
    let mut lptex: *mut byte = 0 as *mut byte;
    let mut lsfrac: libc::c_int = 0;
    let mut ltfrac: libc::c_int = 0;
    let mut llight: libc::c_int = 0;
    let mut lzi: libc::c_int = 0;
    let mut lpz: *mut libc::c_short = 0 as *mut libc::c_short;
    loop {
        lcount = d_aspancount - (*pspanpackage).count;
        errorterm += erroradjustup;
        if errorterm >= 0 as libc::c_int {
            d_aspancount += d_countextrastep;
            errorterm -= erroradjustdown;
        } else {
            d_aspancount += ubasestep;
        }
        if lcount != 0 {
            lpdest = (*pspanpackage).pdest as *mut byte;
            lptex = (*pspanpackage).ptex;
            lpz = (*pspanpackage).pz;
            lsfrac = (*pspanpackage).sfrac;
            ltfrac = (*pspanpackage).tfrac;
            llight = (*pspanpackage).light;
            lzi = (*pspanpackage).zi;
            loop {
                if lzi >> 16 as libc::c_int >= *lpz as libc::c_int {
                    let mut temp: libc::c_int = *(vid.colormap)
                        .offset(
                            (*lptex as libc::c_int + (llight & 0xff00 as libc::c_int))
                                as isize,
                        ) as libc::c_int;
                    *lpdest = *(vid.alphamap)
                        .offset(
                            (temp * 256 as libc::c_int + *lpdest as libc::c_int) as isize,
                        );
                    *lpz = (lzi >> 16 as libc::c_int) as libc::c_short;
                }
                lpdest = lpdest.offset(1);
                lzi += r_zistepx;
                lpz = lpz.offset(1);
                llight += r_lstepx;
                lptex = lptex.offset(a_ststepxwhole as isize);
                lsfrac += a_sstepxfrac;
                lptex = lptex.offset((lsfrac >> 16 as libc::c_int) as isize);
                lsfrac &= 0xffff as libc::c_int;
                ltfrac += a_tstepxfrac;
                if ltfrac & 0x10000 as libc::c_int != 0 {
                    lptex = lptex.offset(r_affinetridesc.skinwidth as isize);
                    ltfrac &= 0xffff as libc::c_int;
                }
                lcount -= 1;
                if !(lcount != 0) {
                    break;
                }
            }
        }
        pspanpackage = pspanpackage.offset(1);
        if !((*pspanpackage).count != -(999999 as libc::c_int)) {
            break;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn R_PolysetDrawSpansConstant8_66(
    mut pspanpackage: *mut spanpackage_t,
) {
    let mut lcount: libc::c_int = 0;
    let mut lpdest: *mut byte = 0 as *mut byte;
    let mut lzi: libc::c_int = 0;
    let mut lpz: *mut libc::c_short = 0 as *mut libc::c_short;
    loop {
        lcount = d_aspancount - (*pspanpackage).count;
        errorterm += erroradjustup;
        if errorterm >= 0 as libc::c_int {
            d_aspancount += d_countextrastep;
            errorterm -= erroradjustdown;
        } else {
            d_aspancount += ubasestep;
        }
        if lcount != 0 {
            lpdest = (*pspanpackage).pdest as *mut byte;
            lpz = (*pspanpackage).pz;
            lzi = (*pspanpackage).zi;
            loop {
                if lzi >> 16 as libc::c_int >= *lpz as libc::c_int {
                    *lpdest = *(vid.alphamap)
                        .offset(
                            (r_aliasblendcolor * 256 as libc::c_int
                                + *lpdest as libc::c_int) as isize,
                        );
                }
                lpdest = lpdest.offset(1);
                lzi += r_zistepx;
                lpz = lpz.offset(1);
                lcount -= 1;
                if !(lcount != 0) {
                    break;
                }
            }
        }
        pspanpackage = pspanpackage.offset(1);
        if !((*pspanpackage).count != -(999999 as libc::c_int)) {
            break;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn R_PolysetDrawSpans8_Opaque(
    mut pspanpackage: *mut spanpackage_t,
) {
    let mut lcount: libc::c_int = 0;
    loop {
        lcount = d_aspancount - (*pspanpackage).count;
        errorterm += erroradjustup;
        if errorterm >= 0 as libc::c_int {
            d_aspancount += d_countextrastep;
            errorterm -= erroradjustdown;
        } else {
            d_aspancount += ubasestep;
        }
        if lcount != 0 {
            let mut lsfrac: libc::c_int = 0;
            let mut ltfrac: libc::c_int = 0;
            let mut lpdest: *mut byte = 0 as *mut byte;
            let mut lptex: *mut byte = 0 as *mut byte;
            let mut llight: libc::c_int = 0;
            let mut lzi: libc::c_int = 0;
            let mut lpz: *mut libc::c_short = 0 as *mut libc::c_short;
            lpdest = (*pspanpackage).pdest as *mut byte;
            lptex = (*pspanpackage).ptex;
            lpz = (*pspanpackage).pz;
            lsfrac = (*pspanpackage).sfrac;
            ltfrac = (*pspanpackage).tfrac;
            llight = (*pspanpackage).light;
            lzi = (*pspanpackage).zi;
            loop {
                if lzi >> 16 as libc::c_int >= *lpz as libc::c_int {
                    if r_newrefdef.rdflags & 4 as libc::c_int != 0
                        && (*currententity).flags & 0x8000 as libc::c_int != 0
                    {
                        *lpdest = *(vid.colormap as *mut byte)
                            .offset(irtable[*lptex as usize] as isize);
                    } else {
                        *lpdest = *(vid.colormap as *mut byte)
                            .offset(
                                (*lptex as libc::c_int + (llight & 0xff00 as libc::c_int))
                                    as isize,
                            );
                    }
                    *lpz = (lzi >> 16 as libc::c_int) as libc::c_short;
                }
                lpdest = lpdest.offset(1);
                lzi += r_zistepx;
                lpz = lpz.offset(1);
                llight += r_lstepx;
                lptex = lptex.offset(a_ststepxwhole as isize);
                lsfrac += a_sstepxfrac;
                lptex = lptex.offset((lsfrac >> 16 as libc::c_int) as isize);
                lsfrac &= 0xffff as libc::c_int;
                ltfrac += a_tstepxfrac;
                if ltfrac & 0x10000 as libc::c_int != 0 {
                    lptex = lptex.offset(r_affinetridesc.skinwidth as isize);
                    ltfrac &= 0xffff as libc::c_int;
                }
                lcount -= 1;
                if !(lcount != 0) {
                    break;
                }
            }
        }
        pspanpackage = pspanpackage.offset(1);
        if !((*pspanpackage).count != -(999999 as libc::c_int)) {
            break;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn R_PolysetFillSpans8(mut pspanpackage: *mut spanpackage_t) {
    let mut color: libc::c_int = 0;
    let fresh3 = d_aflatcolor;
    d_aflatcolor = d_aflatcolor + 1;
    color = fresh3;
    loop {
        let mut lcount: libc::c_int = 0;
        let mut lpdest: *mut byte = 0 as *mut byte;
        lcount = (*pspanpackage).count;
        if lcount == -(1 as libc::c_int) {
            return;
        }
        if lcount != 0 {
            lpdest = (*pspanpackage).pdest as *mut byte;
            loop {
                let fresh4 = lpdest;
                lpdest = lpdest.offset(1);
                *fresh4 = color as byte;
                lcount -= 1;
                if !(lcount != 0) {
                    break;
                }
            }
        }
        pspanpackage = pspanpackage.offset(1);
    };
}
#[no_mangle]
pub unsafe extern "C" fn R_RasterizeAliasPolySmooth() {
    let mut initialleftheight: libc::c_int = 0;
    let mut initialrightheight: libc::c_int = 0;
    let mut plefttop: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut prighttop: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut pleftbottom: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut prightbottom: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut working_lstepx: libc::c_int = 0;
    let mut originalcount: libc::c_int = 0;
    plefttop = (*pedgetable).pleftedgevert0;
    prighttop = (*pedgetable).prightedgevert0;
    pleftbottom = (*pedgetable).pleftedgevert1;
    prightbottom = (*pedgetable).prightedgevert1;
    initialleftheight = *pleftbottom.offset(1 as libc::c_int as isize)
        - *plefttop.offset(1 as libc::c_int as isize);
    initialrightheight = *prightbottom.offset(1 as libc::c_int as isize)
        - *prighttop.offset(1 as libc::c_int as isize);
    R_PolysetCalcGradients(r_affinetridesc.skinwidth);
    d_pedgespanpackage = a_spans;
    ystart = *plefttop.offset(1 as libc::c_int as isize);
    d_aspancount = *plefttop.offset(0 as libc::c_int as isize)
        - *prighttop.offset(0 as libc::c_int as isize);
    d_ptex = (r_affinetridesc.pskin as *mut byte)
        .offset(
            (*plefttop.offset(2 as libc::c_int as isize) >> 16 as libc::c_int) as isize,
        )
        .offset(
            ((*plefttop.offset(3 as libc::c_int as isize) >> 16 as libc::c_int)
                * r_affinetridesc.skinwidth) as isize,
        );
    d_sfrac = *plefttop.offset(2 as libc::c_int as isize) & 0xffff as libc::c_int;
    d_tfrac = *plefttop.offset(3 as libc::c_int as isize) & 0xffff as libc::c_int;
    d_light = *plefttop.offset(4 as libc::c_int as isize);
    d_zi = *plefttop.offset(5 as libc::c_int as isize);
    d_pdest = (d_viewbuffer as *mut byte)
        .offset((ystart * r_screenwidth) as isize)
        .offset(*plefttop.offset(0 as libc::c_int as isize) as isize);
    d_pz = d_pzbuffer
        .offset((ystart as libc::c_uint).wrapping_mul(d_zwidth) as isize)
        .offset(*plefttop.offset(0 as libc::c_int as isize) as isize);
    if initialleftheight == 1 as libc::c_int {
        let ref mut fresh5 = (*d_pedgespanpackage).pdest;
        *fresh5 = d_pdest as *mut libc::c_void;
        let ref mut fresh6 = (*d_pedgespanpackage).pz;
        *fresh6 = d_pz;
        (*d_pedgespanpackage).count = d_aspancount;
        let ref mut fresh7 = (*d_pedgespanpackage).ptex;
        *fresh7 = d_ptex;
        (*d_pedgespanpackage).sfrac = d_sfrac;
        (*d_pedgespanpackage).tfrac = d_tfrac;
        (*d_pedgespanpackage).light = d_light;
        (*d_pedgespanpackage).zi = d_zi;
        d_pedgespanpackage = d_pedgespanpackage.offset(1);
    } else {
        R_PolysetSetUpForLineScan(
            *plefttop.offset(0 as libc::c_int as isize),
            *plefttop.offset(1 as libc::c_int as isize),
            *pleftbottom.offset(0 as libc::c_int as isize),
            *pleftbottom.offset(1 as libc::c_int as isize),
        );
        d_pzbasestep = d_zwidth.wrapping_add(ubasestep as libc::c_uint) as libc::c_int;
        d_pzextrastep = d_pzbasestep + 1 as libc::c_int;
        d_pdestbasestep = r_screenwidth + ubasestep;
        d_pdestextrastep = d_pdestbasestep + 1 as libc::c_int;
        if ubasestep < 0 as libc::c_int {
            working_lstepx = r_lstepx - 1 as libc::c_int;
        } else {
            working_lstepx = r_lstepx;
        }
        d_countextrastep = ubasestep + 1 as libc::c_int;
        d_ptexbasestep = (r_sstepy + r_sstepx * ubasestep >> 16 as libc::c_int)
            + (r_tstepy + r_tstepx * ubasestep >> 16 as libc::c_int)
                * r_affinetridesc.skinwidth;
        d_sfracbasestep = r_sstepy + r_sstepx * ubasestep & 0xffff as libc::c_int;
        d_tfracbasestep = r_tstepy + r_tstepx * ubasestep & 0xffff as libc::c_int;
        d_lightbasestep = r_lstepy + working_lstepx * ubasestep;
        d_zibasestep = r_zistepy + r_zistepx * ubasestep;
        d_ptexextrastep = (r_sstepy + r_sstepx * d_countextrastep >> 16 as libc::c_int)
            + (r_tstepy + r_tstepx * d_countextrastep >> 16 as libc::c_int)
                * r_affinetridesc.skinwidth;
        d_sfracextrastep = r_sstepy + r_sstepx * d_countextrastep
            & 0xffff as libc::c_int;
        d_tfracextrastep = r_tstepy + r_tstepx * d_countextrastep
            & 0xffff as libc::c_int;
        d_lightextrastep = d_lightbasestep + working_lstepx;
        d_ziextrastep = d_zibasestep + r_zistepx;
        R_PolysetScanLeftEdge_C(initialleftheight);
    }
    if (*pedgetable).numleftedges == 2 as libc::c_int {
        let mut height: libc::c_int = 0;
        plefttop = pleftbottom;
        pleftbottom = (*pedgetable).pleftedgevert2;
        height = *pleftbottom.offset(1 as libc::c_int as isize)
            - *plefttop.offset(1 as libc::c_int as isize);
        ystart = *plefttop.offset(1 as libc::c_int as isize);
        d_aspancount = *plefttop.offset(0 as libc::c_int as isize)
            - *prighttop.offset(0 as libc::c_int as isize);
        d_ptex = (r_affinetridesc.pskin as *mut byte)
            .offset(
                (*plefttop.offset(2 as libc::c_int as isize) >> 16 as libc::c_int)
                    as isize,
            )
            .offset(
                ((*plefttop.offset(3 as libc::c_int as isize) >> 16 as libc::c_int)
                    * r_affinetridesc.skinwidth) as isize,
            );
        d_sfrac = 0 as libc::c_int;
        d_tfrac = 0 as libc::c_int;
        d_light = *plefttop.offset(4 as libc::c_int as isize);
        d_zi = *plefttop.offset(5 as libc::c_int as isize);
        d_pdest = (d_viewbuffer as *mut byte)
            .offset((ystart * r_screenwidth) as isize)
            .offset(*plefttop.offset(0 as libc::c_int as isize) as isize);
        d_pz = d_pzbuffer
            .offset((ystart as libc::c_uint).wrapping_mul(d_zwidth) as isize)
            .offset(*plefttop.offset(0 as libc::c_int as isize) as isize);
        if height == 1 as libc::c_int {
            let ref mut fresh8 = (*d_pedgespanpackage).pdest;
            *fresh8 = d_pdest as *mut libc::c_void;
            let ref mut fresh9 = (*d_pedgespanpackage).pz;
            *fresh9 = d_pz;
            (*d_pedgespanpackage).count = d_aspancount;
            let ref mut fresh10 = (*d_pedgespanpackage).ptex;
            *fresh10 = d_ptex;
            (*d_pedgespanpackage).sfrac = d_sfrac;
            (*d_pedgespanpackage).tfrac = d_tfrac;
            (*d_pedgespanpackage).light = d_light;
            (*d_pedgespanpackage).zi = d_zi;
            d_pedgespanpackage = d_pedgespanpackage.offset(1);
        } else {
            R_PolysetSetUpForLineScan(
                *plefttop.offset(0 as libc::c_int as isize),
                *plefttop.offset(1 as libc::c_int as isize),
                *pleftbottom.offset(0 as libc::c_int as isize),
                *pleftbottom.offset(1 as libc::c_int as isize),
            );
            d_pdestbasestep = r_screenwidth + ubasestep;
            d_pdestextrastep = d_pdestbasestep + 1 as libc::c_int;
            d_pzbasestep = d_zwidth.wrapping_add(ubasestep as libc::c_uint)
                as libc::c_int;
            d_pzextrastep = d_pzbasestep + 1 as libc::c_int;
            if ubasestep < 0 as libc::c_int {
                working_lstepx = r_lstepx - 1 as libc::c_int;
            } else {
                working_lstepx = r_lstepx;
            }
            d_countextrastep = ubasestep + 1 as libc::c_int;
            d_ptexbasestep = (r_sstepy + r_sstepx * ubasestep >> 16 as libc::c_int)
                + (r_tstepy + r_tstepx * ubasestep >> 16 as libc::c_int)
                    * r_affinetridesc.skinwidth;
            d_sfracbasestep = r_sstepy + r_sstepx * ubasestep & 0xffff as libc::c_int;
            d_tfracbasestep = r_tstepy + r_tstepx * ubasestep & 0xffff as libc::c_int;
            d_lightbasestep = r_lstepy + working_lstepx * ubasestep;
            d_zibasestep = r_zistepy + r_zistepx * ubasestep;
            d_ptexextrastep = (r_sstepy + r_sstepx * d_countextrastep
                >> 16 as libc::c_int)
                + (r_tstepy + r_tstepx * d_countextrastep >> 16 as libc::c_int)
                    * r_affinetridesc.skinwidth;
            d_sfracextrastep = r_sstepy + r_sstepx * d_countextrastep
                & 0xffff as libc::c_int;
            d_tfracextrastep = r_tstepy + r_tstepx * d_countextrastep
                & 0xffff as libc::c_int;
            d_lightextrastep = d_lightbasestep + working_lstepx;
            d_ziextrastep = d_zibasestep + r_zistepx;
            R_PolysetScanLeftEdge_C(height);
        }
    }
    d_pedgespanpackage = a_spans;
    R_PolysetSetUpForLineScan(
        *prighttop.offset(0 as libc::c_int as isize),
        *prighttop.offset(1 as libc::c_int as isize),
        *prightbottom.offset(0 as libc::c_int as isize),
        *prightbottom.offset(1 as libc::c_int as isize),
    );
    d_aspancount = 0 as libc::c_int;
    d_countextrastep = ubasestep + 1 as libc::c_int;
    originalcount = (*a_spans.offset(initialrightheight as isize)).count;
    (*a_spans.offset(initialrightheight as isize)).count = -(999999 as libc::c_int);
    (Some(d_pdrawspans.expect("non-null function pointer")))
        .expect("non-null function pointer")(a_spans);
    if (*pedgetable).numrightedges == 2 as libc::c_int {
        let mut height_0: libc::c_int = 0;
        let mut pstart: *mut spanpackage_t = 0 as *mut spanpackage_t;
        pstart = a_spans.offset(initialrightheight as isize);
        (*pstart).count = originalcount;
        d_aspancount = *prightbottom.offset(0 as libc::c_int as isize)
            - *prighttop.offset(0 as libc::c_int as isize);
        prighttop = prightbottom;
        prightbottom = (*pedgetable).prightedgevert2;
        height_0 = *prightbottom.offset(1 as libc::c_int as isize)
            - *prighttop.offset(1 as libc::c_int as isize);
        R_PolysetSetUpForLineScan(
            *prighttop.offset(0 as libc::c_int as isize),
            *prighttop.offset(1 as libc::c_int as isize),
            *prightbottom.offset(0 as libc::c_int as isize),
            *prightbottom.offset(1 as libc::c_int as isize),
        );
        d_countextrastep = ubasestep + 1 as libc::c_int;
        (*a_spans.offset((initialrightheight + height_0) as isize))
            .count = -(999999 as libc::c_int);
        (Some(d_pdrawspans.expect("non-null function pointer")))
            .expect("non-null function pointer")(pstart);
    }
}
#[no_mangle]
pub unsafe extern "C" fn R_PolysetSetEdgeTable() {
    let mut edgetableindex: libc::c_int = 0;
    edgetableindex = 0 as libc::c_int;
    if r_p0[1 as libc::c_int as usize] >= r_p1[1 as libc::c_int as usize] {
        if r_p0[1 as libc::c_int as usize] == r_p1[1 as libc::c_int as usize] {
            if r_p0[1 as libc::c_int as usize] < r_p2[1 as libc::c_int as usize] {
                pedgetable = &mut *edgetables
                    .as_mut_ptr()
                    .offset(2 as libc::c_int as isize) as *mut edgetable;
            } else {
                pedgetable = &mut *edgetables
                    .as_mut_ptr()
                    .offset(5 as libc::c_int as isize) as *mut edgetable;
            }
            return;
        } else {
            edgetableindex = 1 as libc::c_int;
        }
    }
    if r_p0[1 as libc::c_int as usize] == r_p2[1 as libc::c_int as usize] {
        if edgetableindex != 0 {
            pedgetable = &mut *edgetables.as_mut_ptr().offset(8 as libc::c_int as isize)
                as *mut edgetable;
        } else {
            pedgetable = &mut *edgetables.as_mut_ptr().offset(9 as libc::c_int as isize)
                as *mut edgetable;
        }
        return;
    } else {
        if r_p1[1 as libc::c_int as usize] == r_p2[1 as libc::c_int as usize] {
            if edgetableindex != 0 {
                pedgetable = &mut *edgetables
                    .as_mut_ptr()
                    .offset(10 as libc::c_int as isize) as *mut edgetable;
            } else {
                pedgetable = &mut *edgetables
                    .as_mut_ptr()
                    .offset(11 as libc::c_int as isize) as *mut edgetable;
            }
            return;
        }
    }
    if r_p0[1 as libc::c_int as usize] > r_p2[1 as libc::c_int as usize] {
        edgetableindex += 2 as libc::c_int;
    }
    if r_p1[1 as libc::c_int as usize] > r_p2[1 as libc::c_int as usize] {
        edgetableindex += 4 as libc::c_int;
    }
    pedgetable = &mut *edgetables.as_mut_ptr().offset(edgetableindex as isize)
        as *mut edgetable;
}
