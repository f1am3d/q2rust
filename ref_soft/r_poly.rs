#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn ceil(_: libc::c_double) -> libc::c_double;
    static mut vec3_origin: vec3_t;
    fn CrossProduct(v1: *mut vec_t, v2: *mut vec_t, cross: *mut vec_t);
    fn VectorNormalize(v: *mut vec_t) -> vec_t;
    static mut vid: viddef_t;
    static mut r_refdef: oldrefdef_t;
    static mut d_sdivzstepu: libc::c_float;
    static mut d_tdivzstepu: libc::c_float;
    static mut d_zistepu: libc::c_float;
    static mut d_sdivzstepv: libc::c_float;
    static mut d_tdivzstepv: libc::c_float;
    static mut d_zistepv: libc::c_float;
    static mut d_sdivzorigin: libc::c_float;
    static mut d_tdivzorigin: libc::c_float;
    static mut d_ziorigin: libc::c_float;
    static mut sadjust: fixed16_t;
    static mut tadjust: fixed16_t;
    static mut bbextents: fixed16_t;
    static mut bbextentt: fixed16_t;
    fn D_CacheSurface(
        surface: *mut msurface_t,
        miplevel: libc::c_int,
    ) -> *mut surfcache_t;
    static mut d_viewbuffer: *mut pixel_t;
    static mut d_pzbuffer: *mut libc::c_short;
    static mut d_zwidth: libc::c_uint;
    static mut d_scantable: [libc::c_int; 1200];
    static mut cachewidth: libc::c_int;
    static mut cacheblock: *mut pixel_t;
    static mut sintable: [libc::c_int; 1280];
    static mut xcenter: libc::c_float;
    static mut ycenter: libc::c_float;
    static mut xscale: libc::c_float;
    static mut yscale: libc::c_float;
    static mut xscaleinv: libc::c_float;
    static mut yscaleinv: libc::c_float;
    fn TransformVector(in_0: *mut vec_t, out: *mut vec_t);
    static mut sw_stipplealpha: *mut cvar_t;
    static mut view_clipplanes: [clipplane_t; 4];
    static mut r_origin: vec3_t;
    static mut currentmodel: *mut model_t;
    static mut modelorg: vec3_t;
    static mut r_worldmodel: *mut model_t;
    static mut r_newrefdef: refdef_t;
    static mut ri: refimport_t;
    static mut r_turb_turb: *mut libc::c_int;
}
pub type byte = libc::c_uchar;
pub type qboolean = libc::c_uint;
pub const true_0: qboolean = 1;
pub const false_0: qboolean = 0;
pub type vec_t = libc::c_float;
pub type vec3_t = [vec_t; 3];
pub type vec5_t = [vec_t; 5];
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
pub struct emitpoint_t {
    pub u: libc::c_float,
    pub v: libc::c_float,
    pub s: libc::c_float,
    pub t: libc::c_float,
    pub zi: libc::c_float,
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
pub type surfcache_t = surfcache_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct espan_s {
    pub u: libc::c_int,
    pub v: libc::c_int,
    pub count: libc::c_int,
    pub pnext: *mut espan_s,
}
pub type espan_t = espan_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct polydesc_t {
    pub nump: libc::c_int,
    pub pverts: *mut emitpoint_t,
    pub pixels: *mut byte,
    pub pixel_width: libc::c_int,
    pub pixel_height: libc::c_int,
    pub vup: vec3_t,
    pub vright: vec3_t,
    pub vpn: vec3_t,
    pub dist: libc::c_float,
    pub s_offset: libc::c_float,
    pub t_offset: libc::c_float,
    pub viewer_position: [libc::c_float; 3],
    pub drawspanlet: Option::<unsafe extern "C" fn() -> ()>,
    pub stipple_parity: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spanletvars_t {
    pub pbase: *mut byte,
    pub pdest: *mut byte,
    pub pz: *mut libc::c_short,
    pub s: fixed16_t,
    pub t: fixed16_t,
    pub sstep: fixed16_t,
    pub tstep: fixed16_t,
    pub izi: libc::c_int,
    pub izistep: libc::c_int,
    pub izistep_times_2: libc::c_int,
    pub spancount: libc::c_int,
    pub u: libc::c_uint,
    pub v: libc::c_uint,
}
#[no_mangle]
pub static mut s_spanletvars: spanletvars_t = spanletvars_t {
    pbase: 0 as *const byte as *mut byte,
    pdest: 0 as *const byte as *mut byte,
    pz: 0 as *const libc::c_short as *mut libc::c_short,
    s: 0,
    t: 0,
    sstep: 0,
    tstep: 0,
    izi: 0,
    izistep: 0,
    izistep_times_2: 0,
    spancount: 0,
    u: 0,
    v: 0,
};
static mut r_polyblendcolor: libc::c_int = 0;
static mut s_polygon_spans: *mut espan_t = 0 as *const espan_t as *mut espan_t;
#[no_mangle]
pub static mut r_polydesc: polydesc_t = polydesc_t {
    nump: 0,
    pverts: 0 as *const emitpoint_t as *mut emitpoint_t,
    pixels: 0 as *const byte as *mut byte,
    pixel_width: 0,
    pixel_height: 0,
    vup: [0.; 3],
    vright: [0.; 3],
    vpn: [0.; 3],
    dist: 0.,
    s_offset: 0.,
    t_offset: 0.,
    viewer_position: [0.; 3],
    drawspanlet: None,
    stipple_parity: 0,
};
#[no_mangle]
pub static mut r_alpha_surfaces: *mut msurface_t = 0 as *const msurface_t
    as *mut msurface_t;
static mut clip_current: libc::c_int = 0;
#[no_mangle]
pub static mut r_clip_verts: [[vec5_t; 70]; 2] = [[[0.; 5]; 70]; 2];
static mut s_minindex: libc::c_int = 0;
static mut s_maxindex: libc::c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn R_DrawSpanletOpaque() {
    let mut btemp: libc::c_uint = 0;
    loop {
        let mut ts: libc::c_uint = 0;
        let mut tt: libc::c_uint = 0;
        ts = (s_spanletvars.s >> 16 as libc::c_int) as libc::c_uint;
        tt = (s_spanletvars.t >> 16 as libc::c_int) as libc::c_uint;
        btemp = *(s_spanletvars.pbase)
            .offset(ts as isize)
            .offset(tt.wrapping_mul(cachewidth as libc::c_uint) as isize)
            as libc::c_uint;
        if btemp != 255 as libc::c_int as libc::c_uint {
            if *s_spanletvars.pz as libc::c_int <= s_spanletvars.izi >> 16 as libc::c_int
            {
                *s_spanletvars
                    .pz = (s_spanletvars.izi >> 16 as libc::c_int) as libc::c_short;
                *s_spanletvars.pdest = btemp as byte;
            }
        }
        s_spanletvars.izi += s_spanletvars.izistep;
        s_spanletvars.pdest = (s_spanletvars.pdest).offset(1);
        s_spanletvars.pz = (s_spanletvars.pz).offset(1);
        s_spanletvars.s += s_spanletvars.sstep;
        s_spanletvars.t += s_spanletvars.tstep;
        s_spanletvars.spancount -= 1;
        if !(s_spanletvars.spancount > 0 as libc::c_int) {
            break;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn R_DrawSpanletTurbulentStipple33() {
    let mut btemp: libc::c_uint = 0;
    let mut sturb: libc::c_int = 0;
    let mut tturb: libc::c_int = 0;
    let mut pdest: *mut byte = s_spanletvars.pdest;
    let mut pz: *mut libc::c_short = s_spanletvars.pz;
    let mut izi: libc::c_int = s_spanletvars.izi;
    if s_spanletvars.v & 1 as libc::c_int as libc::c_uint != 0 {
        s_spanletvars
            .pdest = (s_spanletvars.pdest).offset(s_spanletvars.spancount as isize);
        s_spanletvars.pz = (s_spanletvars.pz).offset(s_spanletvars.spancount as isize);
        if s_spanletvars.spancount == 16 as libc::c_int {
            s_spanletvars.izi += s_spanletvars.izistep << 4 as libc::c_int;
        } else {
            s_spanletvars.izi += s_spanletvars.izistep * s_spanletvars.izistep;
        }
        if s_spanletvars.u & 1 as libc::c_int as libc::c_uint != 0 {
            izi += s_spanletvars.izistep;
            s_spanletvars.s += s_spanletvars.sstep;
            s_spanletvars.t += s_spanletvars.tstep;
            pdest = pdest.offset(1);
            pz = pz.offset(1);
            s_spanletvars.spancount -= 1;
        }
        s_spanletvars.sstep *= 2 as libc::c_int;
        s_spanletvars.tstep *= 2 as libc::c_int;
        while s_spanletvars.spancount > 0 as libc::c_int {
            sturb = s_spanletvars.s
                + *r_turb_turb
                    .offset(
                        (s_spanletvars.t >> 16 as libc::c_int
                            & 128 as libc::c_int - 1 as libc::c_int) as isize,
                    ) >> 16 as libc::c_int & 63 as libc::c_int;
            tturb = s_spanletvars.t
                + *r_turb_turb
                    .offset(
                        (s_spanletvars.s >> 16 as libc::c_int
                            & 128 as libc::c_int - 1 as libc::c_int) as isize,
                    ) >> 16 as libc::c_int & 63 as libc::c_int;
            btemp = *(s_spanletvars.pbase)
                .offset(sturb as isize)
                .offset((tturb << 6 as libc::c_int) as isize) as libc::c_uint;
            if *pz as libc::c_int <= izi >> 16 as libc::c_int {
                *pdest = btemp as byte;
            }
            izi += s_spanletvars.izistep_times_2;
            s_spanletvars.s += s_spanletvars.sstep;
            s_spanletvars.t += s_spanletvars.tstep;
            pdest = pdest.offset(2 as libc::c_int as isize);
            pz = pz.offset(2 as libc::c_int as isize);
            s_spanletvars.spancount -= 2 as libc::c_int;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn R_DrawSpanletTurbulentStipple66() {
    let mut btemp: libc::c_uint = 0;
    let mut sturb: libc::c_int = 0;
    let mut tturb: libc::c_int = 0;
    let mut pdest: *mut byte = s_spanletvars.pdest;
    let mut pz: *mut libc::c_short = s_spanletvars.pz;
    let mut izi: libc::c_int = s_spanletvars.izi;
    if s_spanletvars.v & 1 as libc::c_int as libc::c_uint == 0 {
        s_spanletvars
            .pdest = (s_spanletvars.pdest).offset(s_spanletvars.spancount as isize);
        s_spanletvars.pz = (s_spanletvars.pz).offset(s_spanletvars.spancount as isize);
        if s_spanletvars.spancount == 16 as libc::c_int {
            s_spanletvars.izi += s_spanletvars.izistep << 4 as libc::c_int;
        } else {
            s_spanletvars.izi += s_spanletvars.izistep * s_spanletvars.izistep;
        }
        if s_spanletvars.u & 1 as libc::c_int as libc::c_uint != 0 {
            izi += s_spanletvars.izistep;
            s_spanletvars.s += s_spanletvars.sstep;
            s_spanletvars.t += s_spanletvars.tstep;
            pdest = pdest.offset(1);
            pz = pz.offset(1);
            s_spanletvars.spancount -= 1;
        }
        s_spanletvars.sstep *= 2 as libc::c_int;
        s_spanletvars.tstep *= 2 as libc::c_int;
        while s_spanletvars.spancount > 0 as libc::c_int {
            sturb = s_spanletvars.s
                + *r_turb_turb
                    .offset(
                        (s_spanletvars.t >> 16 as libc::c_int
                            & 128 as libc::c_int - 1 as libc::c_int) as isize,
                    ) >> 16 as libc::c_int & 63 as libc::c_int;
            tturb = s_spanletvars.t
                + *r_turb_turb
                    .offset(
                        (s_spanletvars.s >> 16 as libc::c_int
                            & 128 as libc::c_int - 1 as libc::c_int) as isize,
                    ) >> 16 as libc::c_int & 63 as libc::c_int;
            btemp = *(s_spanletvars.pbase)
                .offset(sturb as isize)
                .offset((tturb << 6 as libc::c_int) as isize) as libc::c_uint;
            if *pz as libc::c_int <= izi >> 16 as libc::c_int {
                *pdest = btemp as byte;
            }
            izi += s_spanletvars.izistep_times_2;
            s_spanletvars.s += s_spanletvars.sstep;
            s_spanletvars.t += s_spanletvars.tstep;
            pdest = pdest.offset(2 as libc::c_int as isize);
            pz = pz.offset(2 as libc::c_int as isize);
            s_spanletvars.spancount -= 2 as libc::c_int;
        }
    } else {
        s_spanletvars
            .pdest = (s_spanletvars.pdest).offset(s_spanletvars.spancount as isize);
        s_spanletvars.pz = (s_spanletvars.pz).offset(s_spanletvars.spancount as isize);
        if s_spanletvars.spancount == 16 as libc::c_int {
            s_spanletvars.izi += s_spanletvars.izistep << 4 as libc::c_int;
        } else {
            s_spanletvars.izi += s_spanletvars.izistep * s_spanletvars.izistep;
        }
        while s_spanletvars.spancount > 0 as libc::c_int {
            sturb = s_spanletvars.s
                + *r_turb_turb
                    .offset(
                        (s_spanletvars.t >> 16 as libc::c_int
                            & 128 as libc::c_int - 1 as libc::c_int) as isize,
                    ) >> 16 as libc::c_int & 63 as libc::c_int;
            tturb = s_spanletvars.t
                + *r_turb_turb
                    .offset(
                        (s_spanletvars.s >> 16 as libc::c_int
                            & 128 as libc::c_int - 1 as libc::c_int) as isize,
                    ) >> 16 as libc::c_int & 63 as libc::c_int;
            btemp = *(s_spanletvars.pbase)
                .offset(sturb as isize)
                .offset((tturb << 6 as libc::c_int) as isize) as libc::c_uint;
            if *pz as libc::c_int <= izi >> 16 as libc::c_int {
                *pdest = btemp as byte;
            }
            izi += s_spanletvars.izistep;
            s_spanletvars.s += s_spanletvars.sstep;
            s_spanletvars.t += s_spanletvars.tstep;
            pdest = pdest.offset(1);
            pz = pz.offset(1);
            s_spanletvars.spancount -= 1;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn R_DrawSpanletTurbulentBlended66() {
    let mut btemp: libc::c_uint = 0;
    let mut sturb: libc::c_int = 0;
    let mut tturb: libc::c_int = 0;
    loop {
        sturb = s_spanletvars.s
            + *r_turb_turb
                .offset(
                    (s_spanletvars.t >> 16 as libc::c_int
                        & 128 as libc::c_int - 1 as libc::c_int) as isize,
                ) >> 16 as libc::c_int & 63 as libc::c_int;
        tturb = s_spanletvars.t
            + *r_turb_turb
                .offset(
                    (s_spanletvars.s >> 16 as libc::c_int
                        & 128 as libc::c_int - 1 as libc::c_int) as isize,
                ) >> 16 as libc::c_int & 63 as libc::c_int;
        btemp = *(s_spanletvars.pbase)
            .offset(sturb as isize)
            .offset((tturb << 6 as libc::c_int) as isize) as libc::c_uint;
        if *s_spanletvars.pz as libc::c_int <= s_spanletvars.izi >> 16 as libc::c_int {
            *s_spanletvars
                .pdest = *(vid.alphamap)
                .offset(
                    btemp
                        .wrapping_mul(256 as libc::c_int as libc::c_uint)
                        .wrapping_add(*s_spanletvars.pdest as libc::c_uint) as isize,
                );
        }
        s_spanletvars.izi += s_spanletvars.izistep;
        s_spanletvars.pdest = (s_spanletvars.pdest).offset(1);
        s_spanletvars.pz = (s_spanletvars.pz).offset(1);
        s_spanletvars.s += s_spanletvars.sstep;
        s_spanletvars.t += s_spanletvars.tstep;
        s_spanletvars.spancount -= 1;
        if !(s_spanletvars.spancount > 0 as libc::c_int) {
            break;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn R_DrawSpanletTurbulentBlended33() {
    let mut btemp: libc::c_uint = 0;
    let mut sturb: libc::c_int = 0;
    let mut tturb: libc::c_int = 0;
    loop {
        sturb = s_spanletvars.s
            + *r_turb_turb
                .offset(
                    (s_spanletvars.t >> 16 as libc::c_int
                        & 128 as libc::c_int - 1 as libc::c_int) as isize,
                ) >> 16 as libc::c_int & 63 as libc::c_int;
        tturb = s_spanletvars.t
            + *r_turb_turb
                .offset(
                    (s_spanletvars.s >> 16 as libc::c_int
                        & 128 as libc::c_int - 1 as libc::c_int) as isize,
                ) >> 16 as libc::c_int & 63 as libc::c_int;
        btemp = *(s_spanletvars.pbase)
            .offset(sturb as isize)
            .offset((tturb << 6 as libc::c_int) as isize) as libc::c_uint;
        if *s_spanletvars.pz as libc::c_int <= s_spanletvars.izi >> 16 as libc::c_int {
            *s_spanletvars
                .pdest = *(vid.alphamap)
                .offset(
                    btemp
                        .wrapping_add(
                            (*s_spanletvars.pdest as libc::c_int * 256 as libc::c_int)
                                as libc::c_uint,
                        ) as isize,
                );
        }
        s_spanletvars.izi += s_spanletvars.izistep;
        s_spanletvars.pdest = (s_spanletvars.pdest).offset(1);
        s_spanletvars.pz = (s_spanletvars.pz).offset(1);
        s_spanletvars.s += s_spanletvars.sstep;
        s_spanletvars.t += s_spanletvars.tstep;
        s_spanletvars.spancount -= 1;
        if !(s_spanletvars.spancount > 0 as libc::c_int) {
            break;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn R_DrawSpanlet33() {
    let mut btemp: libc::c_uint = 0;
    loop {
        let mut ts: libc::c_uint = 0;
        let mut tt: libc::c_uint = 0;
        ts = (s_spanletvars.s >> 16 as libc::c_int) as libc::c_uint;
        tt = (s_spanletvars.t >> 16 as libc::c_int) as libc::c_uint;
        btemp = *(s_spanletvars.pbase)
            .offset(ts as isize)
            .offset(tt.wrapping_mul(cachewidth as libc::c_uint) as isize)
            as libc::c_uint;
        if btemp != 255 as libc::c_int as libc::c_uint {
            if *s_spanletvars.pz as libc::c_int <= s_spanletvars.izi >> 16 as libc::c_int
            {
                *s_spanletvars
                    .pdest = *(vid.alphamap)
                    .offset(
                        btemp
                            .wrapping_add(
                                (*s_spanletvars.pdest as libc::c_int * 256 as libc::c_int)
                                    as libc::c_uint,
                            ) as isize,
                    );
            }
        }
        s_spanletvars.izi += s_spanletvars.izistep;
        s_spanletvars.pdest = (s_spanletvars.pdest).offset(1);
        s_spanletvars.pz = (s_spanletvars.pz).offset(1);
        s_spanletvars.s += s_spanletvars.sstep;
        s_spanletvars.t += s_spanletvars.tstep;
        s_spanletvars.spancount -= 1;
        if !(s_spanletvars.spancount > 0 as libc::c_int) {
            break;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn R_DrawSpanletConstant33() {
    loop {
        if *s_spanletvars.pz as libc::c_int <= s_spanletvars.izi >> 16 as libc::c_int {
            *s_spanletvars
                .pdest = *(vid.alphamap)
                .offset(
                    (r_polyblendcolor
                        + *s_spanletvars.pdest as libc::c_int * 256 as libc::c_int)
                        as isize,
                );
        }
        s_spanletvars.izi += s_spanletvars.izistep;
        s_spanletvars.pdest = (s_spanletvars.pdest).offset(1);
        s_spanletvars.pz = (s_spanletvars.pz).offset(1);
        s_spanletvars.spancount -= 1;
        if !(s_spanletvars.spancount > 0 as libc::c_int) {
            break;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn R_DrawSpanlet66() {
    let mut btemp: libc::c_uint = 0;
    loop {
        let mut ts: libc::c_uint = 0;
        let mut tt: libc::c_uint = 0;
        ts = (s_spanletvars.s >> 16 as libc::c_int) as libc::c_uint;
        tt = (s_spanletvars.t >> 16 as libc::c_int) as libc::c_uint;
        btemp = *(s_spanletvars.pbase)
            .offset(ts as isize)
            .offset(tt.wrapping_mul(cachewidth as libc::c_uint) as isize)
            as libc::c_uint;
        if btemp != 255 as libc::c_int as libc::c_uint {
            if *s_spanletvars.pz as libc::c_int <= s_spanletvars.izi >> 16 as libc::c_int
            {
                *s_spanletvars
                    .pdest = *(vid.alphamap)
                    .offset(
                        btemp
                            .wrapping_mul(256 as libc::c_int as libc::c_uint)
                            .wrapping_add(*s_spanletvars.pdest as libc::c_uint) as isize,
                    );
            }
        }
        s_spanletvars.izi += s_spanletvars.izistep;
        s_spanletvars.pdest = (s_spanletvars.pdest).offset(1);
        s_spanletvars.pz = (s_spanletvars.pz).offset(1);
        s_spanletvars.s += s_spanletvars.sstep;
        s_spanletvars.t += s_spanletvars.tstep;
        s_spanletvars.spancount -= 1;
        if !(s_spanletvars.spancount > 0 as libc::c_int) {
            break;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn R_DrawSpanlet33Stipple() {
    let mut btemp: libc::c_uint = 0;
    let mut pdest: *mut byte = s_spanletvars.pdest;
    let mut pz: *mut libc::c_short = s_spanletvars.pz;
    let mut izi: libc::c_int = s_spanletvars.izi;
    if r_polydesc.stipple_parity as libc::c_uint
        ^ s_spanletvars.v & 1 as libc::c_int as libc::c_uint != 0
    {
        s_spanletvars
            .pdest = (s_spanletvars.pdest).offset(s_spanletvars.spancount as isize);
        s_spanletvars.pz = (s_spanletvars.pz).offset(s_spanletvars.spancount as isize);
        if s_spanletvars.spancount == 16 as libc::c_int {
            s_spanletvars.izi += s_spanletvars.izistep << 4 as libc::c_int;
        } else {
            s_spanletvars.izi += s_spanletvars.izistep * s_spanletvars.izistep;
        }
        if r_polydesc.stipple_parity as libc::c_uint
            ^ s_spanletvars.u & 1 as libc::c_int as libc::c_uint != 0
        {
            izi += s_spanletvars.izistep;
            s_spanletvars.s += s_spanletvars.sstep;
            s_spanletvars.t += s_spanletvars.tstep;
            pdest = pdest.offset(1);
            pz = pz.offset(1);
            s_spanletvars.spancount -= 1;
        }
        s_spanletvars.sstep *= 2 as libc::c_int;
        s_spanletvars.tstep *= 2 as libc::c_int;
        while s_spanletvars.spancount > 0 as libc::c_int {
            let mut s: libc::c_uint = (s_spanletvars.s >> 16 as libc::c_int)
                as libc::c_uint;
            let mut t: libc::c_uint = (s_spanletvars.t >> 16 as libc::c_int)
                as libc::c_uint;
            btemp = *(s_spanletvars.pbase)
                .offset(s as isize)
                .offset(t.wrapping_mul(cachewidth as libc::c_uint) as isize)
                as libc::c_uint;
            if btemp != 255 as libc::c_int as libc::c_uint {
                if *pz as libc::c_int <= izi >> 16 as libc::c_int {
                    *pdest = btemp as byte;
                }
            }
            izi += s_spanletvars.izistep_times_2;
            s_spanletvars.s += s_spanletvars.sstep;
            s_spanletvars.t += s_spanletvars.tstep;
            pdest = pdest.offset(2 as libc::c_int as isize);
            pz = pz.offset(2 as libc::c_int as isize);
            s_spanletvars.spancount -= 2 as libc::c_int;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn R_DrawSpanlet66Stipple() {
    let mut btemp: libc::c_uint = 0;
    let mut pdest: *mut byte = s_spanletvars.pdest;
    let mut pz: *mut libc::c_short = s_spanletvars.pz;
    let mut izi: libc::c_int = s_spanletvars.izi;
    s_spanletvars.pdest = (s_spanletvars.pdest).offset(s_spanletvars.spancount as isize);
    s_spanletvars.pz = (s_spanletvars.pz).offset(s_spanletvars.spancount as isize);
    if s_spanletvars.spancount == 16 as libc::c_int {
        s_spanletvars.izi += s_spanletvars.izistep << 4 as libc::c_int;
    } else {
        s_spanletvars.izi += s_spanletvars.izistep * s_spanletvars.izistep;
    }
    if r_polydesc.stipple_parity as libc::c_uint
        ^ s_spanletvars.v & 1 as libc::c_int as libc::c_uint != 0
    {
        if r_polydesc.stipple_parity as libc::c_uint
            ^ s_spanletvars.u & 1 as libc::c_int as libc::c_uint != 0
        {
            izi += s_spanletvars.izistep;
            s_spanletvars.s += s_spanletvars.sstep;
            s_spanletvars.t += s_spanletvars.tstep;
            pdest = pdest.offset(1);
            pz = pz.offset(1);
            s_spanletvars.spancount -= 1;
        }
        s_spanletvars.sstep *= 2 as libc::c_int;
        s_spanletvars.tstep *= 2 as libc::c_int;
        while s_spanletvars.spancount > 0 as libc::c_int {
            let mut s: libc::c_uint = (s_spanletvars.s >> 16 as libc::c_int)
                as libc::c_uint;
            let mut t: libc::c_uint = (s_spanletvars.t >> 16 as libc::c_int)
                as libc::c_uint;
            btemp = *(s_spanletvars.pbase)
                .offset(s as isize)
                .offset(t.wrapping_mul(cachewidth as libc::c_uint) as isize)
                as libc::c_uint;
            if btemp != 255 as libc::c_int as libc::c_uint {
                if *pz as libc::c_int <= izi >> 16 as libc::c_int {
                    *pdest = btemp as byte;
                }
            }
            izi += s_spanletvars.izistep_times_2;
            s_spanletvars.s += s_spanletvars.sstep;
            s_spanletvars.t += s_spanletvars.tstep;
            pdest = pdest.offset(2 as libc::c_int as isize);
            pz = pz.offset(2 as libc::c_int as isize);
            s_spanletvars.spancount -= 2 as libc::c_int;
        }
    } else {
        while s_spanletvars.spancount > 0 as libc::c_int {
            let mut s_0: libc::c_uint = (s_spanletvars.s >> 16 as libc::c_int)
                as libc::c_uint;
            let mut t_0: libc::c_uint = (s_spanletvars.t >> 16 as libc::c_int)
                as libc::c_uint;
            btemp = *(s_spanletvars.pbase)
                .offset(s_0 as isize)
                .offset(t_0.wrapping_mul(cachewidth as libc::c_uint) as isize)
                as libc::c_uint;
            if btemp != 255 as libc::c_int as libc::c_uint {
                if *pz as libc::c_int <= izi >> 16 as libc::c_int {
                    *pdest = btemp as byte;
                }
            }
            izi += s_spanletvars.izistep;
            s_spanletvars.s += s_spanletvars.sstep;
            s_spanletvars.t += s_spanletvars.tstep;
            pdest = pdest.offset(1);
            pz = pz.offset(1);
            s_spanletvars.spancount -= 1;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn R_ClipPolyFace(
    mut nump: libc::c_int,
    mut pclipplane: *mut clipplane_t,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut outcount: libc::c_int = 0;
    let mut dists: [libc::c_float; 71] = [0.; 71];
    let mut frac: libc::c_float = 0.;
    let mut clipdist: libc::c_float = 0.;
    let mut pclipnormal: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut in_0: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut instep: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut outstep: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut vert2: *mut libc::c_float = 0 as *mut libc::c_float;
    clipdist = (*pclipplane).dist;
    pclipnormal = ((*pclipplane).normal).as_mut_ptr();
    if clip_current != 0 {
        in_0 = (r_clip_verts[1 as libc::c_int as usize][0 as libc::c_int as usize])
            .as_mut_ptr();
        outstep = (r_clip_verts[0 as libc::c_int as usize][0 as libc::c_int as usize])
            .as_mut_ptr();
        clip_current = 0 as libc::c_int;
    } else {
        in_0 = (r_clip_verts[0 as libc::c_int as usize][0 as libc::c_int as usize])
            .as_mut_ptr();
        outstep = (r_clip_verts[1 as libc::c_int as usize][0 as libc::c_int as usize])
            .as_mut_ptr();
        clip_current = 1 as libc::c_int;
    }
    instep = in_0;
    i = 0 as libc::c_int;
    while i < nump {
        dists[i
            as usize] = *instep.offset(0 as libc::c_int as isize)
            * *pclipnormal.offset(0 as libc::c_int as isize)
            + *instep.offset(1 as libc::c_int as isize)
                * *pclipnormal.offset(1 as libc::c_int as isize)
            + *instep.offset(2 as libc::c_int as isize)
                * *pclipnormal.offset(2 as libc::c_int as isize) - clipdist;
        i += 1;
        instep = instep
            .offset(
                (::std::mem::size_of::<vec5_t>() as libc::c_ulong)
                    .wrapping_div(
                        ::std::mem::size_of::<libc::c_float>() as libc::c_ulong,
                    ) as isize,
            );
    }
    dists[nump as usize] = dists[0 as libc::c_int as usize];
    memcpy(
        instep as *mut libc::c_void,
        in_0 as *const libc::c_void,
        ::std::mem::size_of::<vec5_t>() as libc::c_ulong,
    );
    instep = in_0;
    outcount = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < nump {
        if dists[i as usize] >= 0 as libc::c_int as libc::c_float {
            memcpy(
                outstep as *mut libc::c_void,
                instep as *const libc::c_void,
                ::std::mem::size_of::<vec5_t>() as libc::c_ulong,
            );
            outstep = outstep
                .offset(
                    (::std::mem::size_of::<vec5_t>() as libc::c_ulong)
                        .wrapping_div(
                            ::std::mem::size_of::<libc::c_float>() as libc::c_ulong,
                        ) as isize,
                );
            outcount += 1;
        }
        if !(dists[i as usize] == 0 as libc::c_int as libc::c_float
            || dists[(i + 1 as libc::c_int) as usize]
                == 0 as libc::c_int as libc::c_float)
        {
            if !((dists[i as usize] > 0 as libc::c_int as libc::c_float) as libc::c_int
                == (dists[(i + 1 as libc::c_int) as usize]
                    > 0 as libc::c_int as libc::c_float) as libc::c_int)
            {
                frac = dists[i as usize]
                    / (dists[i as usize] - dists[(i + 1 as libc::c_int) as usize]);
                vert2 = instep
                    .offset(
                        (::std::mem::size_of::<vec5_t>() as libc::c_ulong)
                            .wrapping_div(
                                ::std::mem::size_of::<libc::c_float>() as libc::c_ulong,
                            ) as isize,
                    );
                *outstep
                    .offset(
                        0 as libc::c_int as isize,
                    ) = *instep.offset(0 as libc::c_int as isize)
                    + frac
                        * (*vert2.offset(0 as libc::c_int as isize)
                            - *instep.offset(0 as libc::c_int as isize));
                *outstep
                    .offset(
                        1 as libc::c_int as isize,
                    ) = *instep.offset(1 as libc::c_int as isize)
                    + frac
                        * (*vert2.offset(1 as libc::c_int as isize)
                            - *instep.offset(1 as libc::c_int as isize));
                *outstep
                    .offset(
                        2 as libc::c_int as isize,
                    ) = *instep.offset(2 as libc::c_int as isize)
                    + frac
                        * (*vert2.offset(2 as libc::c_int as isize)
                            - *instep.offset(2 as libc::c_int as isize));
                *outstep
                    .offset(
                        3 as libc::c_int as isize,
                    ) = *instep.offset(3 as libc::c_int as isize)
                    + frac
                        * (*vert2.offset(3 as libc::c_int as isize)
                            - *instep.offset(3 as libc::c_int as isize));
                *outstep
                    .offset(
                        4 as libc::c_int as isize,
                    ) = *instep.offset(4 as libc::c_int as isize)
                    + frac
                        * (*vert2.offset(4 as libc::c_int as isize)
                            - *instep.offset(4 as libc::c_int as isize));
                outstep = outstep
                    .offset(
                        (::std::mem::size_of::<vec5_t>() as libc::c_ulong)
                            .wrapping_div(
                                ::std::mem::size_of::<libc::c_float>() as libc::c_ulong,
                            ) as isize,
                    );
                outcount += 1;
            }
        }
        i += 1;
        instep = instep
            .offset(
                (::std::mem::size_of::<vec5_t>() as libc::c_ulong)
                    .wrapping_div(
                        ::std::mem::size_of::<libc::c_float>() as libc::c_ulong,
                    ) as isize,
            );
    }
    return outcount;
}
#[no_mangle]
pub unsafe extern "C" fn R_PolygonDrawSpans(
    mut pspan: *mut espan_t,
    mut iswater: qboolean,
) {
    let mut count: libc::c_int = 0;
    let mut snext: fixed16_t = 0;
    let mut tnext: fixed16_t = 0;
    let mut sdivz: libc::c_float = 0.;
    let mut tdivz: libc::c_float = 0.;
    let mut zi: libc::c_float = 0.;
    let mut z: libc::c_float = 0.;
    let mut du: libc::c_float = 0.;
    let mut dv: libc::c_float = 0.;
    let mut spancountminus1: libc::c_float = 0.;
    let mut sdivzspanletstepu: libc::c_float = 0.;
    let mut tdivzspanletstepu: libc::c_float = 0.;
    let mut zispanletstepu: libc::c_float = 0.;
    s_spanletvars.pbase = cacheblock;
    if iswater as u64 != 0 {
        r_turb_turb = sintable
            .as_mut_ptr()
            .offset(
                ((r_newrefdef.time * 20 as libc::c_int as libc::c_float) as libc::c_int
                    & 128 as libc::c_int - 1 as libc::c_int) as isize,
            );
    }
    sdivzspanletstepu = d_sdivzstepu * 16 as libc::c_int as libc::c_float;
    tdivzspanletstepu = d_tdivzstepu * 16 as libc::c_int as libc::c_float;
    zispanletstepu = d_zistepu * 16 as libc::c_int as libc::c_float;
    s_spanletvars
        .izistep = (d_zistepu * 0x8000 as libc::c_int as libc::c_float
        * 0x10000 as libc::c_int as libc::c_float) as libc::c_int;
    s_spanletvars.izistep_times_2 = s_spanletvars.izistep * 2 as libc::c_int;
    s_spanletvars.pz = 0 as *mut libc::c_short;
    loop {
        s_spanletvars
            .pdest = (d_viewbuffer as *mut byte)
            .offset(d_scantable[(*pspan).v as usize] as isize)
            .offset((*pspan).u as isize);
        s_spanletvars
            .pz = d_pzbuffer
            .offset(d_zwidth.wrapping_mul((*pspan).v as libc::c_uint) as isize)
            .offset((*pspan).u as isize);
        s_spanletvars.u = (*pspan).u as libc::c_uint;
        s_spanletvars.v = (*pspan).v as libc::c_uint;
        count = (*pspan).count;
        if !(count <= 0 as libc::c_int) {
            du = (*pspan).u as libc::c_float;
            dv = (*pspan).v as libc::c_float;
            sdivz = d_sdivzorigin + dv * d_sdivzstepv + du * d_sdivzstepu;
            tdivz = d_tdivzorigin + dv * d_tdivzstepv + du * d_tdivzstepu;
            zi = d_ziorigin + dv * d_zistepv + du * d_zistepu;
            z = 0x10000 as libc::c_int as libc::c_float / zi;
            s_spanletvars
                .izi = (zi * 0x8000 as libc::c_int as libc::c_float
                * 0x10000 as libc::c_int as libc::c_float) as libc::c_int;
            s_spanletvars.s = (sdivz * z) as libc::c_int + sadjust;
            s_spanletvars.t = (tdivz * z) as libc::c_int + tadjust;
            if iswater as u64 == 0 {
                if s_spanletvars.s > bbextents {
                    s_spanletvars.s = bbextents;
                } else if s_spanletvars.s < 0 as libc::c_int {
                    s_spanletvars.s = 0 as libc::c_int;
                }
                if s_spanletvars.t > bbextentt {
                    s_spanletvars.t = bbextentt;
                } else if s_spanletvars.t < 0 as libc::c_int {
                    s_spanletvars.t = 0 as libc::c_int;
                }
            }
            loop {
                if count >= 16 as libc::c_int {
                    s_spanletvars.spancount = 16 as libc::c_int;
                } else {
                    s_spanletvars.spancount = count;
                }
                count -= s_spanletvars.spancount;
                if count != 0 {
                    sdivz += sdivzspanletstepu;
                    tdivz += tdivzspanletstepu;
                    zi += zispanletstepu;
                    z = 0x10000 as libc::c_int as libc::c_float / zi;
                    snext = (sdivz * z) as libc::c_int + sadjust;
                    tnext = (tdivz * z) as libc::c_int + tadjust;
                    if iswater as u64 == 0 {
                        if snext > bbextents {
                            snext = bbextents;
                        } else if snext < 16 as libc::c_int {
                            snext = 16 as libc::c_int;
                        }
                        if tnext > bbextentt {
                            tnext = bbextentt;
                        } else if tnext < 16 as libc::c_int {
                            tnext = 16 as libc::c_int;
                        }
                    }
                    s_spanletvars.sstep = snext - s_spanletvars.s >> 4 as libc::c_int;
                    s_spanletvars.tstep = tnext - s_spanletvars.t >> 4 as libc::c_int;
                } else {
                    spancountminus1 = (s_spanletvars.spancount - 1 as libc::c_int)
                        as libc::c_float;
                    sdivz += d_sdivzstepu * spancountminus1;
                    tdivz += d_tdivzstepu * spancountminus1;
                    zi += d_zistepu * spancountminus1;
                    z = 0x10000 as libc::c_int as libc::c_float / zi;
                    snext = (sdivz * z) as libc::c_int + sadjust;
                    tnext = (tdivz * z) as libc::c_int + tadjust;
                    if iswater as u64 == 0 {
                        if snext > bbextents {
                            snext = bbextents;
                        } else if snext < 16 as libc::c_int {
                            snext = 16 as libc::c_int;
                        }
                        if tnext > bbextentt {
                            tnext = bbextentt;
                        } else if tnext < 16 as libc::c_int {
                            tnext = 16 as libc::c_int;
                        }
                    }
                    if s_spanletvars.spancount > 1 as libc::c_int {
                        s_spanletvars
                            .sstep = (snext - s_spanletvars.s)
                            / (s_spanletvars.spancount - 1 as libc::c_int);
                        s_spanletvars
                            .tstep = (tnext - s_spanletvars.t)
                            / (s_spanletvars.spancount - 1 as libc::c_int);
                    }
                }
                if iswater as u64 != 0 {
                    s_spanletvars
                        .s = s_spanletvars.s
                        & ((128 as libc::c_int) << 16 as libc::c_int) - 1 as libc::c_int;
                    s_spanletvars
                        .t = s_spanletvars.t
                        & ((128 as libc::c_int) << 16 as libc::c_int) - 1 as libc::c_int;
                }
                (r_polydesc.drawspanlet).expect("non-null function pointer")();
                s_spanletvars.s = snext;
                s_spanletvars.t = tnext;
                if !(count > 0 as libc::c_int) {
                    break;
                }
            }
        }
        pspan = pspan.offset(1);
        if !((*pspan).count != -(128 as libc::c_int)) {
            break;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn R_PolygonScanLeftEdge() {
    let mut i: libc::c_int = 0;
    let mut v: libc::c_int = 0;
    let mut itop: libc::c_int = 0;
    let mut ibottom: libc::c_int = 0;
    let mut lmaxindex: libc::c_int = 0;
    let mut pvert: *mut emitpoint_t = 0 as *mut emitpoint_t;
    let mut pnext: *mut emitpoint_t = 0 as *mut emitpoint_t;
    let mut pspan: *mut espan_t = 0 as *mut espan_t;
    let mut du: libc::c_float = 0.;
    let mut dv: libc::c_float = 0.;
    let mut vtop: libc::c_float = 0.;
    let mut vbottom: libc::c_float = 0.;
    let mut slope: libc::c_float = 0.;
    let mut u: fixed16_t = 0;
    let mut u_step: fixed16_t = 0;
    pspan = s_polygon_spans;
    i = s_minindex;
    if i == 0 as libc::c_int {
        i = r_polydesc.nump;
    }
    lmaxindex = s_maxindex;
    if lmaxindex == 0 as libc::c_int {
        lmaxindex = r_polydesc.nump;
    }
    vtop = ceil((*(r_polydesc.pverts).offset(i as isize)).v as libc::c_double)
        as libc::c_float;
    loop {
        pvert = &mut *(r_polydesc.pverts).offset(i as isize) as *mut emitpoint_t;
        pnext = pvert.offset(-(1 as libc::c_int as isize));
        vbottom = ceil((*pnext).v as libc::c_double) as libc::c_float;
        if vtop < vbottom {
            du = (*pnext).u - (*pvert).u;
            dv = (*pnext).v - (*pvert).v;
            slope = du / dv;
            u_step = (slope * 0x10000 as libc::c_int as libc::c_float) as libc::c_int;
            u = (((*pvert).u + slope * (vtop - (*pvert).v))
                * 0x10000 as libc::c_int as libc::c_float) as libc::c_int
                + (0x10000 as libc::c_int - 1 as libc::c_int);
            itop = vtop as libc::c_int;
            ibottom = vbottom as libc::c_int;
            v = itop;
            while v < ibottom {
                (*pspan).u = u >> 16 as libc::c_int;
                (*pspan).v = v;
                u += u_step;
                pspan = pspan.offset(1);
                v += 1;
            }
        }
        vtop = vbottom;
        i -= 1;
        if i == 0 as libc::c_int {
            i = r_polydesc.nump;
        }
        if !(i != lmaxindex) {
            break;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn R_PolygonScanRightEdge() {
    let mut i: libc::c_int = 0;
    let mut v: libc::c_int = 0;
    let mut itop: libc::c_int = 0;
    let mut ibottom: libc::c_int = 0;
    let mut pvert: *mut emitpoint_t = 0 as *mut emitpoint_t;
    let mut pnext: *mut emitpoint_t = 0 as *mut emitpoint_t;
    let mut pspan: *mut espan_t = 0 as *mut espan_t;
    let mut du: libc::c_float = 0.;
    let mut dv: libc::c_float = 0.;
    let mut vtop: libc::c_float = 0.;
    let mut vbottom: libc::c_float = 0.;
    let mut slope: libc::c_float = 0.;
    let mut uvert: libc::c_float = 0.;
    let mut unext: libc::c_float = 0.;
    let mut vvert: libc::c_float = 0.;
    let mut vnext: libc::c_float = 0.;
    let mut u: fixed16_t = 0;
    let mut u_step: fixed16_t = 0;
    pspan = s_polygon_spans;
    i = s_minindex;
    vvert = (*(r_polydesc.pverts).offset(i as isize)).v;
    if vvert < r_refdef.fvrecty_adj {
        vvert = r_refdef.fvrecty_adj;
    }
    if vvert > r_refdef.fvrectbottom_adj {
        vvert = r_refdef.fvrectbottom_adj;
    }
    vtop = ceil(vvert as libc::c_double) as libc::c_float;
    loop {
        pvert = &mut *(r_polydesc.pverts).offset(i as isize) as *mut emitpoint_t;
        pnext = pvert.offset(1 as libc::c_int as isize);
        vnext = (*pnext).v;
        if vnext < r_refdef.fvrecty_adj {
            vnext = r_refdef.fvrecty_adj;
        }
        if vnext > r_refdef.fvrectbottom_adj {
            vnext = r_refdef.fvrectbottom_adj;
        }
        vbottom = ceil(vnext as libc::c_double) as libc::c_float;
        if vtop < vbottom {
            uvert = (*pvert).u;
            if uvert < r_refdef.fvrectx_adj {
                uvert = r_refdef.fvrectx_adj;
            }
            if uvert > r_refdef.fvrectright_adj {
                uvert = r_refdef.fvrectright_adj;
            }
            unext = (*pnext).u;
            if unext < r_refdef.fvrectx_adj {
                unext = r_refdef.fvrectx_adj;
            }
            if unext > r_refdef.fvrectright_adj {
                unext = r_refdef.fvrectright_adj;
            }
            du = unext - uvert;
            dv = vnext - vvert;
            slope = du / dv;
            u_step = (slope * 0x10000 as libc::c_int as libc::c_float) as libc::c_int;
            u = ((uvert + slope * (vtop - vvert))
                * 0x10000 as libc::c_int as libc::c_float) as libc::c_int
                + (0x10000 as libc::c_int - 1 as libc::c_int);
            itop = vtop as libc::c_int;
            ibottom = vbottom as libc::c_int;
            v = itop;
            while v < ibottom {
                (*pspan).count = (u >> 16 as libc::c_int) - (*pspan).u;
                u += u_step;
                pspan = pspan.offset(1);
                v += 1;
            }
        }
        vtop = vbottom;
        vvert = vnext;
        i += 1;
        if i == r_polydesc.nump {
            i = 0 as libc::c_int;
        }
        if !(i != s_maxindex) {
            break;
        }
    }
    (*pspan).count = -(128 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn R_ClipAndDrawPoly(
    mut alpha: libc::c_float,
    mut isturbulent: qboolean,
    mut textured: qboolean,
) {
    let mut outverts: [emitpoint_t; 71] = [emitpoint_t {
        u: 0.,
        v: 0.,
        s: 0.,
        t: 0.,
        zi: 0.,
    }; 71];
    let mut pout: *mut emitpoint_t = 0 as *mut emitpoint_t;
    let mut pv: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut i: libc::c_int = 0;
    let mut nump: libc::c_int = 0;
    let mut scale: libc::c_float = 0.;
    let mut transformed: vec3_t = [0.; 3];
    let mut local: vec3_t = [0.; 3];
    if textured as u64 == 0 {
        r_polydesc
            .drawspanlet = Some(R_DrawSpanletConstant33 as unsafe extern "C" fn() -> ());
    } else if alpha == 1 as libc::c_int as libc::c_float {
        r_polydesc
            .drawspanlet = Some(R_DrawSpanletOpaque as unsafe extern "C" fn() -> ());
    } else if (*sw_stipplealpha).value != 0. {
        if isturbulent as u64 != 0 {
            if alpha as libc::c_double > 0.33f64 {
                r_polydesc
                    .drawspanlet = Some(
                    R_DrawSpanletTurbulentStipple66 as unsafe extern "C" fn() -> (),
                );
            } else {
                r_polydesc
                    .drawspanlet = Some(
                    R_DrawSpanletTurbulentStipple33 as unsafe extern "C" fn() -> (),
                );
            }
        } else if alpha as libc::c_double > 0.33f64 {
            r_polydesc
                .drawspanlet = Some(
                R_DrawSpanlet66Stipple as unsafe extern "C" fn() -> (),
            );
        } else {
            r_polydesc
                .drawspanlet = Some(
                R_DrawSpanlet33Stipple as unsafe extern "C" fn() -> (),
            );
        }
    } else if isturbulent as u64 != 0 {
        if alpha as libc::c_double > 0.33f64 {
            r_polydesc
                .drawspanlet = Some(
                R_DrawSpanletTurbulentBlended66 as unsafe extern "C" fn() -> (),
            );
        } else {
            r_polydesc
                .drawspanlet = Some(
                R_DrawSpanletTurbulentBlended33 as unsafe extern "C" fn() -> (),
            );
        }
    } else if alpha as libc::c_double > 0.33f64 {
        r_polydesc.drawspanlet = Some(R_DrawSpanlet66 as unsafe extern "C" fn() -> ());
    } else {
        r_polydesc.drawspanlet = Some(R_DrawSpanlet33 as unsafe extern "C" fn() -> ());
    }
    nump = r_polydesc.nump;
    clip_current = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        nump = R_ClipPolyFace(
            nump,
            &mut *view_clipplanes.as_mut_ptr().offset(i as isize),
        );
        if nump < 3 as libc::c_int {
            return;
        }
        if nump > 64 as libc::c_int + 4 as libc::c_int {
            (ri.Sys_Error)
                .expect(
                    "non-null function pointer",
                )(
                1 as libc::c_int,
                b"R_ClipAndDrawPoly: too many points: %d\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                nump,
            );
        }
        i += 1;
    }
    pv = &mut *(*(*r_clip_verts.as_mut_ptr().offset(clip_current as isize))
        .as_mut_ptr()
        .offset(0 as libc::c_int as isize))
        .as_mut_ptr()
        .offset(0 as libc::c_int as isize) as *mut vec_t;
    i = 0 as libc::c_int;
    while i < nump {
        local[0 as libc::c_int
            as usize] = *pv.offset(0 as libc::c_int as isize)
            - r_origin[0 as libc::c_int as usize];
        local[1 as libc::c_int
            as usize] = *pv.offset(1 as libc::c_int as isize)
            - r_origin[1 as libc::c_int as usize];
        local[2 as libc::c_int
            as usize] = *pv.offset(2 as libc::c_int as isize)
            - r_origin[2 as libc::c_int as usize];
        TransformVector(local.as_mut_ptr(), transformed.as_mut_ptr());
        if (transformed[2 as libc::c_int as usize] as libc::c_double) < 0.01f64 {
            transformed[2 as libc::c_int as usize] = 0.01f64 as vec_t;
        }
        pout = &mut *outverts.as_mut_ptr().offset(i as isize) as *mut emitpoint_t;
        (*pout)
            .zi = (1.0f64 / transformed[2 as libc::c_int as usize] as libc::c_double)
            as libc::c_float;
        (*pout).s = *pv.offset(3 as libc::c_int as isize);
        (*pout).t = *pv.offset(4 as libc::c_int as isize);
        scale = xscale * (*pout).zi;
        (*pout).u = xcenter + scale * transformed[0 as libc::c_int as usize];
        scale = yscale * (*pout).zi;
        (*pout).v = ycenter - scale * transformed[1 as libc::c_int as usize];
        pv = pv
            .offset(
                (::std::mem::size_of::<vec5_t>() as libc::c_ulong)
                    .wrapping_div(
                        ::std::mem::size_of::<*mut libc::c_float>() as libc::c_ulong,
                    ) as isize,
            );
        i += 1;
    }
    r_polydesc.nump = nump;
    r_polydesc.pverts = outverts.as_mut_ptr();
    R_DrawPoly(isturbulent);
}
#[no_mangle]
pub unsafe extern "C" fn R_BuildPolygonFromSurface(mut fa: *mut msurface_t) {
    let mut i: libc::c_int = 0;
    let mut lindex: libc::c_int = 0;
    let mut lnumverts: libc::c_int = 0;
    let mut pedges: *mut medge_t = 0 as *mut medge_t;
    let mut r_pedge: *mut medge_t = 0 as *mut medge_t;
    let mut vertpage: libc::c_int = 0;
    let mut vec: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut pverts: *mut vec5_t = 0 as *mut vec5_t;
    let mut tmins: [libc::c_float; 2] = [
        0 as libc::c_int as libc::c_float,
        0 as libc::c_int as libc::c_float,
    ];
    r_polydesc.nump = 0 as libc::c_int;
    pedges = (*currentmodel).edges;
    lnumverts = (*fa).numedges;
    vertpage = 0 as libc::c_int;
    pverts = (r_clip_verts[0 as libc::c_int as usize]).as_mut_ptr();
    i = 0 as libc::c_int;
    while i < lnumverts {
        lindex = *((*currentmodel).surfedges).offset(((*fa).firstedge + i) as isize);
        if lindex > 0 as libc::c_int {
            r_pedge = &mut *pedges.offset(lindex as isize) as *mut medge_t;
            vec = ((*((*currentmodel).vertexes)
                .offset((*r_pedge).v[0 as libc::c_int as usize] as isize))
                .position)
                .as_mut_ptr();
        } else {
            r_pedge = &mut *pedges.offset(-lindex as isize) as *mut medge_t;
            vec = ((*((*currentmodel).vertexes)
                .offset((*r_pedge).v[1 as libc::c_int as usize] as isize))
                .position)
                .as_mut_ptr();
        }
        (*pverts
            .offset(
                i as isize,
            ))[0 as libc::c_int as usize] = *vec.offset(0 as libc::c_int as isize);
        (*pverts
            .offset(
                i as isize,
            ))[1 as libc::c_int as usize] = *vec.offset(1 as libc::c_int as isize);
        (*pverts
            .offset(
                i as isize,
            ))[2 as libc::c_int as usize] = *vec.offset(2 as libc::c_int as isize);
        i += 1;
    }
    r_polydesc
        .vright[0 as libc::c_int
        as usize] = (*(*fa).texinfo)
        .vecs[0 as libc::c_int as usize][0 as libc::c_int as usize];
    r_polydesc
        .vright[1 as libc::c_int
        as usize] = (*(*fa).texinfo)
        .vecs[0 as libc::c_int as usize][1 as libc::c_int as usize];
    r_polydesc
        .vright[2 as libc::c_int
        as usize] = (*(*fa).texinfo)
        .vecs[0 as libc::c_int as usize][2 as libc::c_int as usize];
    r_polydesc
        .vup[0 as libc::c_int
        as usize] = (*(*fa).texinfo)
        .vecs[1 as libc::c_int as usize][0 as libc::c_int as usize];
    r_polydesc
        .vup[1 as libc::c_int
        as usize] = (*(*fa).texinfo)
        .vecs[1 as libc::c_int as usize][1 as libc::c_int as usize];
    r_polydesc
        .vup[2 as libc::c_int
        as usize] = (*(*fa).texinfo)
        .vecs[1 as libc::c_int as usize][2 as libc::c_int as usize];
    r_polydesc
        .vpn[0 as libc::c_int
        as usize] = (*(*fa).plane).normal[0 as libc::c_int as usize];
    r_polydesc
        .vpn[1 as libc::c_int
        as usize] = (*(*fa).plane).normal[1 as libc::c_int as usize];
    r_polydesc
        .vpn[2 as libc::c_int
        as usize] = (*(*fa).plane).normal[2 as libc::c_int as usize];
    r_polydesc
        .viewer_position[0 as libc::c_int
        as usize] = r_origin[0 as libc::c_int as usize];
    r_polydesc
        .viewer_position[1 as libc::c_int
        as usize] = r_origin[1 as libc::c_int as usize];
    r_polydesc
        .viewer_position[2 as libc::c_int
        as usize] = r_origin[2 as libc::c_int as usize];
    if (*fa).flags & 2 as libc::c_int != 0 {
        r_polydesc
            .vpn[0 as libc::c_int
            as usize] = vec3_origin[0 as libc::c_int as usize]
            - r_polydesc.vpn[0 as libc::c_int as usize];
        r_polydesc
            .vpn[1 as libc::c_int
            as usize] = vec3_origin[1 as libc::c_int as usize]
            - r_polydesc.vpn[1 as libc::c_int as usize];
        r_polydesc
            .vpn[2 as libc::c_int
            as usize] = vec3_origin[2 as libc::c_int as usize]
            - r_polydesc.vpn[2 as libc::c_int as usize];
    }
    if (*(*fa).texinfo).flags & 0x8 as libc::c_int != 0 {
        r_polydesc.pixels = (*(*(*fa).texinfo).image).pixels[0 as libc::c_int as usize];
        r_polydesc.pixel_width = (*(*(*fa).texinfo).image).width;
        r_polydesc.pixel_height = (*(*(*fa).texinfo).image).height;
    } else {
        let mut scache: *mut surfcache_t = 0 as *mut surfcache_t;
        scache = D_CacheSurface(fa, 0 as libc::c_int);
        r_polydesc.pixels = ((*scache).data).as_mut_ptr();
        r_polydesc.pixel_width = (*scache).width as libc::c_int;
        r_polydesc.pixel_height = (*scache).height as libc::c_int;
        tmins[0 as libc::c_int
            as usize] = (*fa).texturemins[0 as libc::c_int as usize] as libc::c_float;
        tmins[1 as libc::c_int
            as usize] = (*fa).texturemins[1 as libc::c_int as usize] as libc::c_float;
    }
    r_polydesc
        .dist = r_polydesc.vpn[0 as libc::c_int as usize]
        * (*pverts.offset(0 as libc::c_int as isize))[0 as libc::c_int as usize]
        + r_polydesc.vpn[1 as libc::c_int as usize]
            * (*pverts.offset(0 as libc::c_int as isize))[1 as libc::c_int as usize]
        + r_polydesc.vpn[2 as libc::c_int as usize]
            * (*pverts.offset(0 as libc::c_int as isize))[2 as libc::c_int as usize];
    r_polydesc
        .s_offset = (*(*fa).texinfo)
        .vecs[0 as libc::c_int as usize][3 as libc::c_int as usize]
        - tmins[0 as libc::c_int as usize];
    r_polydesc
        .t_offset = (*(*fa).texinfo)
        .vecs[1 as libc::c_int as usize][3 as libc::c_int as usize]
        - tmins[1 as libc::c_int as usize];
    if (*(*fa).texinfo).flags & 0x40 as libc::c_int != 0 {
        r_polydesc
            .s_offset = (r_polydesc.s_offset as libc::c_double
            + -(128 as libc::c_int) as libc::c_double
                * (r_newrefdef.time as libc::c_double * 0.25f64
                    - (r_newrefdef.time as libc::c_double * 0.25f64) as libc::c_int
                        as libc::c_double)) as libc::c_float;
    }
    r_polydesc.nump = lnumverts;
}
#[no_mangle]
pub unsafe extern "C" fn R_PolygonCalculateGradients() {
    let mut p_normal: vec3_t = [0.; 3];
    let mut p_saxis: vec3_t = [0.; 3];
    let mut p_taxis: vec3_t = [0.; 3];
    let mut distinv: libc::c_float = 0.;
    TransformVector((r_polydesc.vpn).as_mut_ptr(), p_normal.as_mut_ptr());
    TransformVector((r_polydesc.vright).as_mut_ptr(), p_saxis.as_mut_ptr());
    TransformVector((r_polydesc.vup).as_mut_ptr(), p_taxis.as_mut_ptr());
    distinv = (1.0f64
        / (-(r_polydesc.viewer_position[0 as libc::c_int as usize]
            * r_polydesc.vpn[0 as libc::c_int as usize]
            + r_polydesc.viewer_position[1 as libc::c_int as usize]
                * r_polydesc.vpn[1 as libc::c_int as usize]
            + r_polydesc.viewer_position[2 as libc::c_int as usize]
                * r_polydesc.vpn[2 as libc::c_int as usize]) + r_polydesc.dist)
            as libc::c_double) as libc::c_float;
    d_sdivzstepu = p_saxis[0 as libc::c_int as usize] * xscaleinv;
    d_sdivzstepv = -p_saxis[1 as libc::c_int as usize] * yscaleinv;
    d_sdivzorigin = p_saxis[2 as libc::c_int as usize] - xcenter * d_sdivzstepu
        - ycenter * d_sdivzstepv;
    d_tdivzstepu = p_taxis[0 as libc::c_int as usize] * xscaleinv;
    d_tdivzstepv = -p_taxis[1 as libc::c_int as usize] * yscaleinv;
    d_tdivzorigin = p_taxis[2 as libc::c_int as usize] - xcenter * d_tdivzstepu
        - ycenter * d_tdivzstepv;
    d_zistepu = p_normal[0 as libc::c_int as usize] * xscaleinv * distinv;
    d_zistepv = -p_normal[1 as libc::c_int as usize] * yscaleinv * distinv;
    d_ziorigin = p_normal[2 as libc::c_int as usize] * distinv - xcenter * d_zistepu
        - ycenter * d_zistepv;
    sadjust = ((r_polydesc.viewer_position[0 as libc::c_int as usize]
        * r_polydesc.vright[0 as libc::c_int as usize]
        + r_polydesc.viewer_position[1 as libc::c_int as usize]
            * r_polydesc.vright[1 as libc::c_int as usize]
        + r_polydesc.viewer_position[2 as libc::c_int as usize]
            * r_polydesc.vright[2 as libc::c_int as usize] + r_polydesc.s_offset)
        * 0x10000 as libc::c_int as libc::c_float) as fixed16_t;
    tadjust = ((r_polydesc.viewer_position[0 as libc::c_int as usize]
        * r_polydesc.vup[0 as libc::c_int as usize]
        + r_polydesc.viewer_position[1 as libc::c_int as usize]
            * r_polydesc.vup[1 as libc::c_int as usize]
        + r_polydesc.viewer_position[2 as libc::c_int as usize]
            * r_polydesc.vup[2 as libc::c_int as usize] + r_polydesc.t_offset)
        * 0x10000 as libc::c_int as libc::c_float) as fixed16_t;
    bbextents = (r_polydesc.pixel_width << 16 as libc::c_int) - 1 as libc::c_int;
    bbextentt = (r_polydesc.pixel_height << 16 as libc::c_int) - 1 as libc::c_int;
}
unsafe extern "C" fn R_DrawPoly(mut iswater: qboolean) {
    let mut i: libc::c_int = 0;
    let mut nump: libc::c_int = 0;
    let mut ymin: libc::c_float = 0.;
    let mut ymax: libc::c_float = 0.;
    let mut pverts: *mut emitpoint_t = 0 as *mut emitpoint_t;
    let mut spans: [espan_t; 1201] = [espan_t {
        u: 0,
        v: 0,
        count: 0,
        pnext: 0 as *mut espan_s,
    }; 1201];
    s_polygon_spans = spans.as_mut_ptr();
    ymin = 999999.9f64 as libc::c_float;
    ymax = -999999.9f64 as libc::c_float;
    pverts = r_polydesc.pverts;
    i = 0 as libc::c_int;
    while i < r_polydesc.nump {
        if (*pverts).v < ymin {
            ymin = (*pverts).v;
            s_minindex = i;
        }
        if (*pverts).v > ymax {
            ymax = (*pverts).v;
            s_maxindex = i;
        }
        pverts = pverts.offset(1);
        i += 1;
    }
    ymin = ceil(ymin as libc::c_double) as libc::c_float;
    ymax = ceil(ymax as libc::c_double) as libc::c_float;
    if ymin >= ymax {
        return;
    }
    cachewidth = r_polydesc.pixel_width;
    cacheblock = r_polydesc.pixels;
    nump = r_polydesc.nump;
    pverts = r_polydesc.pverts;
    *pverts.offset(nump as isize) = *pverts.offset(0 as libc::c_int as isize);
    R_PolygonCalculateGradients();
    R_PolygonScanLeftEdge();
    R_PolygonScanRightEdge();
    R_PolygonDrawSpans(s_polygon_spans, iswater);
}
#[no_mangle]
pub unsafe extern "C" fn R_DrawAlphaSurfaces() {
    let mut s: *mut msurface_t = r_alpha_surfaces;
    currentmodel = r_worldmodel;
    modelorg[0 as libc::c_int as usize] = -r_origin[0 as libc::c_int as usize];
    modelorg[1 as libc::c_int as usize] = -r_origin[1 as libc::c_int as usize];
    modelorg[2 as libc::c_int as usize] = -r_origin[2 as libc::c_int as usize];
    while !s.is_null() {
        R_BuildPolygonFromSurface(s);
        if (*(*s).texinfo).flags & 0x20 as libc::c_int != 0 {
            R_ClipAndDrawPoly(
                0.60f32,
                ((*(*s).texinfo).flags & 0x8 as libc::c_int != 0 as libc::c_int)
                    as libc::c_int as qboolean,
                true_0,
            );
        } else {
            R_ClipAndDrawPoly(
                0.30f32,
                ((*(*s).texinfo).flags & 0x8 as libc::c_int != 0 as libc::c_int)
                    as libc::c_int as qboolean,
                true_0,
            );
        }
        s = (*s).nextalphasurface;
    }
    r_alpha_surfaces = 0 as *mut msurface_t;
}
#[no_mangle]
pub unsafe extern "C" fn R_IMFlatShadedQuad(
    mut a: *mut vec_t,
    mut b: *mut vec_t,
    mut c: *mut vec_t,
    mut d: *mut vec_t,
    mut color: libc::c_int,
    mut alpha: libc::c_float,
) {
    let mut s0: vec3_t = [0.; 3];
    let mut s1: vec3_t = [0.; 3];
    r_polydesc.nump = 4 as libc::c_int;
    r_polydesc
        .viewer_position[0 as libc::c_int
        as usize] = r_origin[0 as libc::c_int as usize];
    r_polydesc
        .viewer_position[1 as libc::c_int
        as usize] = r_origin[1 as libc::c_int as usize];
    r_polydesc
        .viewer_position[2 as libc::c_int
        as usize] = r_origin[2 as libc::c_int as usize];
    r_clip_verts[0 as libc::c_int
        as usize][0 as libc::c_int
        as usize][0 as libc::c_int as usize] = *a.offset(0 as libc::c_int as isize);
    r_clip_verts[0 as libc::c_int
        as usize][0 as libc::c_int
        as usize][1 as libc::c_int as usize] = *a.offset(1 as libc::c_int as isize);
    r_clip_verts[0 as libc::c_int
        as usize][0 as libc::c_int
        as usize][2 as libc::c_int as usize] = *a.offset(2 as libc::c_int as isize);
    r_clip_verts[0 as libc::c_int
        as usize][1 as libc::c_int
        as usize][0 as libc::c_int as usize] = *b.offset(0 as libc::c_int as isize);
    r_clip_verts[0 as libc::c_int
        as usize][1 as libc::c_int
        as usize][1 as libc::c_int as usize] = *b.offset(1 as libc::c_int as isize);
    r_clip_verts[0 as libc::c_int
        as usize][1 as libc::c_int
        as usize][2 as libc::c_int as usize] = *b.offset(2 as libc::c_int as isize);
    r_clip_verts[0 as libc::c_int
        as usize][2 as libc::c_int
        as usize][0 as libc::c_int as usize] = *c.offset(0 as libc::c_int as isize);
    r_clip_verts[0 as libc::c_int
        as usize][2 as libc::c_int
        as usize][1 as libc::c_int as usize] = *c.offset(1 as libc::c_int as isize);
    r_clip_verts[0 as libc::c_int
        as usize][2 as libc::c_int
        as usize][2 as libc::c_int as usize] = *c.offset(2 as libc::c_int as isize);
    r_clip_verts[0 as libc::c_int
        as usize][3 as libc::c_int
        as usize][0 as libc::c_int as usize] = *d.offset(0 as libc::c_int as isize);
    r_clip_verts[0 as libc::c_int
        as usize][3 as libc::c_int
        as usize][1 as libc::c_int as usize] = *d.offset(1 as libc::c_int as isize);
    r_clip_verts[0 as libc::c_int
        as usize][3 as libc::c_int
        as usize][2 as libc::c_int as usize] = *d.offset(2 as libc::c_int as isize);
    r_clip_verts[0 as libc::c_int
        as usize][0 as libc::c_int
        as usize][3 as libc::c_int as usize] = 0 as libc::c_int as vec_t;
    r_clip_verts[0 as libc::c_int
        as usize][1 as libc::c_int
        as usize][3 as libc::c_int as usize] = 0 as libc::c_int as vec_t;
    r_clip_verts[0 as libc::c_int
        as usize][2 as libc::c_int
        as usize][3 as libc::c_int as usize] = 0 as libc::c_int as vec_t;
    r_clip_verts[0 as libc::c_int
        as usize][3 as libc::c_int
        as usize][3 as libc::c_int as usize] = 0 as libc::c_int as vec_t;
    r_clip_verts[0 as libc::c_int
        as usize][0 as libc::c_int
        as usize][4 as libc::c_int as usize] = 0 as libc::c_int as vec_t;
    r_clip_verts[0 as libc::c_int
        as usize][1 as libc::c_int
        as usize][4 as libc::c_int as usize] = 0 as libc::c_int as vec_t;
    r_clip_verts[0 as libc::c_int
        as usize][2 as libc::c_int
        as usize][4 as libc::c_int as usize] = 0 as libc::c_int as vec_t;
    r_clip_verts[0 as libc::c_int
        as usize][3 as libc::c_int
        as usize][4 as libc::c_int as usize] = 0 as libc::c_int as vec_t;
    s0[0 as libc::c_int
        as usize] = *d.offset(0 as libc::c_int as isize)
        - *c.offset(0 as libc::c_int as isize);
    s0[1 as libc::c_int
        as usize] = *d.offset(1 as libc::c_int as isize)
        - *c.offset(1 as libc::c_int as isize);
    s0[2 as libc::c_int
        as usize] = *d.offset(2 as libc::c_int as isize)
        - *c.offset(2 as libc::c_int as isize);
    s1[0 as libc::c_int
        as usize] = *c.offset(0 as libc::c_int as isize)
        - *b.offset(0 as libc::c_int as isize);
    s1[1 as libc::c_int
        as usize] = *c.offset(1 as libc::c_int as isize)
        - *b.offset(1 as libc::c_int as isize);
    s1[2 as libc::c_int
        as usize] = *c.offset(2 as libc::c_int as isize)
        - *b.offset(2 as libc::c_int as isize);
    CrossProduct(s0.as_mut_ptr(), s1.as_mut_ptr(), (r_polydesc.vpn).as_mut_ptr());
    VectorNormalize((r_polydesc.vpn).as_mut_ptr());
    r_polydesc
        .dist = r_polydesc.vpn[0 as libc::c_int as usize]
        * r_clip_verts[0 as libc::c_int
            as usize][0 as libc::c_int as usize][0 as libc::c_int as usize]
        + r_polydesc.vpn[1 as libc::c_int as usize]
            * r_clip_verts[0 as libc::c_int
                as usize][0 as libc::c_int as usize][1 as libc::c_int as usize]
        + r_polydesc.vpn[2 as libc::c_int as usize]
            * r_clip_verts[0 as libc::c_int
                as usize][0 as libc::c_int as usize][2 as libc::c_int as usize];
    r_polyblendcolor = color;
    R_ClipAndDrawPoly(alpha, false_0, false_0);
}
