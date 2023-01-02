#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    static mut modelorg: vec3_t;
    static mut vec3_origin: vec3_t;
    fn VectorScale(in_0: *mut vec_t, scale: vec_t, out: *mut vec_t);
    static mut r_refdef: oldrefdef_t;
    static mut r_origin: vec3_t;
    static mut currententity: *mut entity_t;
    fn D_DrawZSpans(pspans: *mut espan_t);
    static mut r_screenwidth: libc::c_int;
    static mut d_viewbuffer: *mut pixel_t;
    static mut d_ziorigin: libc::c_float;
    static mut d_zistepv: libc::c_float;
    static mut d_zistepu: libc::c_float;
    static mut base_vright: vec3_t;
    static mut vright: vec3_t;
    static mut base_vup: vec3_t;
    static mut vup: vec3_t;
    static mut base_vpn: vec3_t;
    static mut vpn: vec3_t;
    fn Turbulent8(pspan: *mut espan_t);
    fn NonTurbulent8(pspan: *mut espan_t);
    static mut bbextentt: fixed16_t;
    static mut bbextents: fixed16_t;
    static mut sadjust: fixed16_t;
    static mut tadjust: fixed16_t;
    static mut d_tdivzstepv: libc::c_float;
    static mut ycenter: libc::c_float;
    static mut d_tdivzstepu: libc::c_float;
    static mut xcenter: libc::c_float;
    static mut d_tdivzorigin: libc::c_float;
    static mut d_sdivzstepv: libc::c_float;
    static mut d_sdivzstepu: libc::c_float;
    static mut d_sdivzorigin: libc::c_float;
    static mut yscaleinv: libc::c_float;
    static mut xscaleinv: libc::c_float;
    fn TransformVector(in_0: *mut vec_t, out: *mut vec_t);
    static mut cachewidth: libc::c_int;
    static mut cacheblock: *mut pixel_t;
    static mut sw_clearcolor: *mut cvar_t;
    fn D_DrawSpans16(pspans: *mut espan_t);
    fn D_CacheSurface(
        surface: *mut msurface_t,
        miplevel_0: libc::c_int,
    ) -> *mut surfcache_t;
    static mut d_minmip: libc::c_int;
    static mut d_scalemip: [libc::c_float; 3];
    static mut r_worldentity: entity_t;
    static mut r_drawnpolycount: libc::c_int;
    static mut sw_drawflat: *mut cvar_t;
    static mut sw_draworder: *mut cvar_t;
    fn R_RotateBmodel();
    static mut r_newrefdef: refdef_t;
    static mut r_numallocatededges: libc::c_int;
    fn R_TransformFrustum();
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
pub static mut auxedges: *mut edge_t = 0 as *const edge_t as *mut edge_t;
#[no_mangle]
pub static mut r_edges: *mut edge_t = 0 as *const edge_t as *mut edge_t;
#[no_mangle]
pub static mut edge_p: *mut edge_t = 0 as *const edge_t as *mut edge_t;
#[no_mangle]
pub static mut edge_max: *mut edge_t = 0 as *const edge_t as *mut edge_t;
#[no_mangle]
pub static mut surfaces: *mut surf_t = 0 as *const surf_t as *mut surf_t;
#[no_mangle]
pub static mut surface_p: *mut surf_t = 0 as *const surf_t as *mut surf_t;
#[no_mangle]
pub static mut surf_max: *mut surf_t = 0 as *const surf_t as *mut surf_t;
#[no_mangle]
pub static mut newedges: [*mut edge_t; 1200] = [0 as *const edge_t as *mut edge_t; 1200];
#[no_mangle]
pub static mut removeedges: [*mut edge_t; 1200] = [0 as *const edge_t
    as *mut edge_t; 1200];
#[no_mangle]
pub static mut span_p: *mut espan_t = 0 as *const espan_t as *mut espan_t;
#[no_mangle]
pub static mut max_span_p: *mut espan_t = 0 as *const espan_t as *mut espan_t;
#[no_mangle]
pub static mut r_currentkey: libc::c_int = 0;
#[no_mangle]
pub static mut current_iv: libc::c_int = 0;
#[no_mangle]
pub static mut edge_head_u_shift20: libc::c_int = 0;
#[no_mangle]
pub static mut edge_tail_u_shift20: libc::c_int = 0;
static mut pdrawfunc: Option::<unsafe extern "C" fn() -> ()> = None;
#[no_mangle]
pub static mut edge_head: edge_t = edge_t {
    u: 0,
    u_step: 0,
    prev: 0 as *const edge_s as *mut edge_s,
    next: 0 as *const edge_s as *mut edge_s,
    surfs: [0; 2],
    nextremove: 0 as *const edge_s as *mut edge_s,
    nearzi: 0.,
    owner: 0 as *const medge_t as *mut medge_t,
};
#[no_mangle]
pub static mut edge_tail: edge_t = edge_t {
    u: 0,
    u_step: 0,
    prev: 0 as *const edge_s as *mut edge_s,
    next: 0 as *const edge_s as *mut edge_s,
    surfs: [0; 2],
    nextremove: 0 as *const edge_s as *mut edge_s,
    nearzi: 0.,
    owner: 0 as *const medge_t as *mut medge_t,
};
#[no_mangle]
pub static mut edge_aftertail: edge_t = edge_t {
    u: 0,
    u_step: 0,
    prev: 0 as *const edge_s as *mut edge_s,
    next: 0 as *const edge_s as *mut edge_s,
    surfs: [0; 2],
    nextremove: 0 as *const edge_s as *mut edge_s,
    nearzi: 0.,
    owner: 0 as *const medge_t as *mut medge_t,
};
#[no_mangle]
pub static mut edge_sentinel: edge_t = edge_t {
    u: 0,
    u_step: 0,
    prev: 0 as *const edge_s as *mut edge_s,
    next: 0 as *const edge_s as *mut edge_s,
    surfs: [0; 2],
    nextremove: 0 as *const edge_s as *mut edge_s,
    nearzi: 0.,
    owner: 0 as *const medge_t as *mut medge_t,
};
#[no_mangle]
pub static mut fv: libc::c_float = 0.;
static mut miplevel: libc::c_int = 0;
#[no_mangle]
pub static mut scale_for_mip: libc::c_float = 0.;
#[no_mangle]
pub static mut ubasestep: libc::c_int = 0;
#[no_mangle]
pub static mut errorterm: libc::c_int = 0;
#[no_mangle]
pub static mut erroradjustup: libc::c_int = 0;
#[no_mangle]
pub static mut erroradjustdown: libc::c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn R_BeginEdgeFrame() {
    let mut v: libc::c_int = 0;
    edge_p = r_edges;
    edge_max = &mut *r_edges.offset(r_numallocatededges as isize) as *mut edge_t;
    surface_p = &mut *surfaces.offset(2 as libc::c_int as isize) as *mut surf_t;
    let ref mut fresh0 = (*surfaces.offset(1 as libc::c_int as isize)).spans;
    *fresh0 = 0 as *mut espan_s;
    (*surfaces.offset(1 as libc::c_int as isize)).flags = 0x40 as libc::c_int;
    if (*sw_draworder).value != 0. {
        pdrawfunc = Some(R_GenerateSpansBackward as unsafe extern "C" fn() -> ());
        (*surfaces.offset(1 as libc::c_int as isize)).key = 0 as libc::c_int;
        r_currentkey = 1 as libc::c_int;
    } else {
        pdrawfunc = Some(R_GenerateSpans as unsafe extern "C" fn() -> ());
        (*surfaces.offset(1 as libc::c_int as isize)).key = 0x7fffffff as libc::c_int;
        r_currentkey = 0 as libc::c_int;
    }
    v = r_refdef.vrect.y;
    while v < r_refdef.vrectbottom {
        removeedges[v as usize] = 0 as *mut edge_t;
        newedges[v as usize] = removeedges[v as usize];
        v += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn R_InsertNewEdges(
    mut edgestoadd: *mut edge_t,
    mut edgelist: *mut edge_t,
) {
    let mut next_edge: *mut edge_t = 0 as *mut edge_t;
    loop {
        next_edge = (*edgestoadd).next;
        while !((*edgelist).u >= (*edgestoadd).u) {
            edgelist = (*edgelist).next;
            if (*edgelist).u >= (*edgestoadd).u {
                break;
            }
            edgelist = (*edgelist).next;
            if (*edgelist).u >= (*edgestoadd).u {
                break;
            }
            edgelist = (*edgelist).next;
            if (*edgelist).u >= (*edgestoadd).u {
                break;
            }
            edgelist = (*edgelist).next;
        }
        let ref mut fresh1 = (*edgestoadd).next;
        *fresh1 = edgelist;
        let ref mut fresh2 = (*edgestoadd).prev;
        *fresh2 = (*edgelist).prev;
        let ref mut fresh3 = (*(*edgelist).prev).next;
        *fresh3 = edgestoadd;
        let ref mut fresh4 = (*edgelist).prev;
        *fresh4 = edgestoadd;
        edgestoadd = next_edge;
        if edgestoadd.is_null() {
            break;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn R_RemoveEdges(mut pedge: *mut edge_t) {
    loop {
        let ref mut fresh5 = (*(*pedge).next).prev;
        *fresh5 = (*pedge).prev;
        let ref mut fresh6 = (*(*pedge).prev).next;
        *fresh6 = (*pedge).next;
        pedge = (*pedge).nextremove;
        if pedge.is_null() {
            break;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn R_StepActiveU(mut pedge: *mut edge_t) {
    let mut pnext_edge: *mut edge_t = 0 as *mut edge_t;
    let mut pwedge: *mut edge_t = 0 as *mut edge_t;
    loop {
        let ref mut fresh7 = (*pedge).u;
        *fresh7 += (*pedge).u_step;
        if !((*pedge).u < (*(*pedge).prev).u) {
            pedge = (*pedge).next;
            let ref mut fresh8 = (*pedge).u;
            *fresh8 += (*pedge).u_step;
            if !((*pedge).u < (*(*pedge).prev).u) {
                pedge = (*pedge).next;
                let ref mut fresh9 = (*pedge).u;
                *fresh9 += (*pedge).u_step;
                if !((*pedge).u < (*(*pedge).prev).u) {
                    pedge = (*pedge).next;
                    let ref mut fresh10 = (*pedge).u;
                    *fresh10 += (*pedge).u_step;
                    if !((*pedge).u < (*(*pedge).prev).u) {
                        pedge = (*pedge).next;
                        continue;
                    }
                }
            }
        }
        if pedge == &mut edge_aftertail as *mut edge_t {
            return;
        }
        pnext_edge = (*pedge).next;
        let ref mut fresh11 = (*(*pedge).next).prev;
        *fresh11 = (*pedge).prev;
        let ref mut fresh12 = (*(*pedge).prev).next;
        *fresh12 = (*pedge).next;
        pwedge = (*(*pedge).prev).prev;
        while (*pwedge).u > (*pedge).u {
            pwedge = (*pwedge).prev;
        }
        let ref mut fresh13 = (*pedge).next;
        *fresh13 = (*pwedge).next;
        let ref mut fresh14 = (*pedge).prev;
        *fresh14 = pwedge;
        let ref mut fresh15 = (*(*pedge).next).prev;
        *fresh15 = pedge;
        let ref mut fresh16 = (*pwedge).next;
        *fresh16 = pedge;
        pedge = pnext_edge;
        if pedge == &mut edge_tail as *mut edge_t {
            return;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn R_CleanupSpan() {
    let mut surf: *mut surf_t = 0 as *mut surf_t;
    let mut iu: libc::c_int = 0;
    let mut span: *mut espan_t = 0 as *mut espan_t;
    surf = (*surfaces.offset(1 as libc::c_int as isize)).next;
    iu = edge_tail_u_shift20;
    if iu > (*surf).last_u {
        let fresh17 = span_p;
        span_p = span_p.offset(1);
        span = fresh17;
        (*span).u = (*surf).last_u;
        (*span).count = iu - (*span).u;
        (*span).v = current_iv;
        let ref mut fresh18 = (*span).pnext;
        *fresh18 = (*surf).spans;
        let ref mut fresh19 = (*surf).spans;
        *fresh19 = span;
    }
    loop {
        (*surf).spanstate = 0 as libc::c_int;
        surf = (*surf).next;
        if !(surf != &mut *surfaces.offset(1 as libc::c_int as isize) as *mut surf_t) {
            break;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn R_LeadingEdgeBackwards(mut edge: *mut edge_t) {
    let mut current_block: u64;
    let mut span: *mut espan_t = 0 as *mut espan_t;
    let mut surf: *mut surf_t = 0 as *mut surf_t;
    let mut surf2: *mut surf_t = 0 as *mut surf_t;
    let mut iu: libc::c_int = 0;
    surf = &mut *surfaces
        .offset(*((*edge).surfs).as_mut_ptr().offset(1 as libc::c_int as isize) as isize)
        as *mut surf_t;
    let ref mut fresh20 = (*surf).spanstate;
    *fresh20 += 1;
    if *fresh20 == 1 as libc::c_int {
        surf2 = (*surfaces.offset(1 as libc::c_int as isize)).next;
        if (*surf).key > (*surf2).key {
            current_block = 16903213892611502954;
        } else if (*surf).insubmodel as libc::c_uint != 0 && (*surf).key == (*surf2).key
        {
            current_block = 16903213892611502954;
        } else {
            loop {
                loop {
                    surf2 = (*surf2).next;
                    if !((*surf).key < (*surf2).key) {
                        break;
                    }
                }
                if !((*surf).key == (*surf2).key) {
                    break;
                }
                if !((*surf).insubmodel as u64 == 0) {
                    break;
                }
            }
            current_block = 7995648291273452906;
        }
        match current_block {
            16903213892611502954 => {
                iu = (*edge).u >> 20 as libc::c_int;
                if iu > (*surf2).last_u {
                    let fresh21 = span_p;
                    span_p = span_p.offset(1);
                    span = fresh21;
                    (*span).u = (*surf2).last_u;
                    (*span).count = iu - (*span).u;
                    (*span).v = current_iv;
                    let ref mut fresh22 = (*span).pnext;
                    *fresh22 = (*surf2).spans;
                    let ref mut fresh23 = (*surf2).spans;
                    *fresh23 = span;
                }
                (*surf).last_u = iu;
            }
            _ => {}
        }
        let ref mut fresh24 = (*surf).next;
        *fresh24 = surf2;
        let ref mut fresh25 = (*surf).prev;
        *fresh25 = (*surf2).prev;
        let ref mut fresh26 = (*(*surf2).prev).next;
        *fresh26 = surf;
        let ref mut fresh27 = (*surf2).prev;
        *fresh27 = surf;
    }
}
#[no_mangle]
pub unsafe extern "C" fn R_TrailingEdge(mut surf: *mut surf_t, mut edge: *mut edge_t) {
    let mut span: *mut espan_t = 0 as *mut espan_t;
    let mut iu: libc::c_int = 0;
    let ref mut fresh28 = (*surf).spanstate;
    *fresh28 -= 1;
    if *fresh28 == 0 as libc::c_int {
        if surf == (*surfaces.offset(1 as libc::c_int as isize)).next {
            iu = (*edge).u >> 20 as libc::c_int;
            if iu > (*surf).last_u {
                let fresh29 = span_p;
                span_p = span_p.offset(1);
                span = fresh29;
                (*span).u = (*surf).last_u;
                (*span).count = iu - (*span).u;
                (*span).v = current_iv;
                let ref mut fresh30 = (*span).pnext;
                *fresh30 = (*surf).spans;
                let ref mut fresh31 = (*surf).spans;
                *fresh31 = span;
            }
            (*(*surf).next).last_u = iu;
        }
        let ref mut fresh32 = (*(*surf).prev).next;
        *fresh32 = (*surf).next;
        let ref mut fresh33 = (*(*surf).next).prev;
        *fresh33 = (*surf).prev;
    }
}
#[no_mangle]
pub unsafe extern "C" fn R_LeadingEdge(mut edge: *mut edge_t) {
    let mut current_block: u64;
    let mut span: *mut espan_t = 0 as *mut espan_t;
    let mut surf: *mut surf_t = 0 as *mut surf_t;
    let mut surf2: *mut surf_t = 0 as *mut surf_t;
    let mut iu: libc::c_int = 0;
    let mut fu: libc::c_float = 0.;
    let mut newzi: libc::c_float = 0.;
    let mut testzi: libc::c_float = 0.;
    let mut newzitop: libc::c_float = 0.;
    let mut newzibottom: libc::c_float = 0.;
    if (*edge).surfs[1 as libc::c_int as usize] != 0 {
        surf = &mut *surfaces
            .offset(
                *((*edge).surfs).as_mut_ptr().offset(1 as libc::c_int as isize) as isize,
            ) as *mut surf_t;
        let ref mut fresh34 = (*surf).spanstate;
        *fresh34 += 1;
        if *fresh34 == 1 as libc::c_int {
            surf2 = (*surfaces.offset(1 as libc::c_int as isize)).next;
            if (*surf).key < (*surf2).key {
                current_block = 8234216009147720692;
            } else {
                if (*surf).insubmodel as libc::c_uint != 0 && (*surf).key == (*surf2).key
                {
                    fu = (((*edge).u - 0xfffff as libc::c_int) as libc::c_float
                        as libc::c_double
                        * (1.0f64 / 0x100000 as libc::c_int as libc::c_double))
                        as libc::c_float;
                    newzi = (*surf).d_ziorigin + fv * (*surf).d_zistepv
                        + fu * (*surf).d_zistepu;
                    newzibottom = (newzi as libc::c_double * 0.99f64) as libc::c_float;
                    testzi = (*surf2).d_ziorigin + fv * (*surf2).d_zistepv
                        + fu * (*surf2).d_zistepu;
                    if newzibottom >= testzi {
                        current_block = 8234216009147720692;
                    } else {
                        newzitop = (newzi as libc::c_double * 1.01f64) as libc::c_float;
                        if newzitop >= testzi {
                            if (*surf).d_zistepu >= (*surf2).d_zistepu {
                                current_block = 8234216009147720692;
                            } else {
                                current_block = 6763693791783192250;
                            }
                        } else {
                            current_block = 6763693791783192250;
                        }
                    }
                } else {
                    current_block = 6763693791783192250;
                }
                match current_block {
                    8234216009147720692 => {}
                    _ => {
                        loop {
                            loop {
                                surf2 = (*surf2).next;
                                if !((*surf).key > (*surf2).key) {
                                    break;
                                }
                            }
                            if !((*surf).key == (*surf2).key) {
                                break;
                            }
                            if (*surf).insubmodel as u64 == 0 {
                                continue;
                            }
                            fu = (((*edge).u - 0xfffff as libc::c_int) as libc::c_float
                                as libc::c_double
                                * (1.0f64 / 0x100000 as libc::c_int as libc::c_double))
                                as libc::c_float;
                            newzi = (*surf).d_ziorigin + fv * (*surf).d_zistepv
                                + fu * (*surf).d_zistepu;
                            newzibottom = (newzi as libc::c_double * 0.99f64)
                                as libc::c_float;
                            testzi = (*surf2).d_ziorigin + fv * (*surf2).d_zistepv
                                + fu * (*surf2).d_zistepu;
                            if newzibottom >= testzi {
                                break;
                            }
                            newzitop = (newzi as libc::c_double * 1.01f64)
                                as libc::c_float;
                            if !(newzitop >= testzi) {
                                continue;
                            }
                            if (*surf).d_zistepu >= (*surf2).d_zistepu {
                                break;
                            }
                        }
                        current_block = 7340308794037309040;
                    }
                }
            }
            match current_block {
                8234216009147720692 => {
                    iu = (*edge).u >> 20 as libc::c_int;
                    if iu > (*surf2).last_u {
                        let fresh35 = span_p;
                        span_p = span_p.offset(1);
                        span = fresh35;
                        (*span).u = (*surf2).last_u;
                        (*span).count = iu - (*span).u;
                        (*span).v = current_iv;
                        let ref mut fresh36 = (*span).pnext;
                        *fresh36 = (*surf2).spans;
                        let ref mut fresh37 = (*surf2).spans;
                        *fresh37 = span;
                    }
                    (*surf).last_u = iu;
                }
                _ => {}
            }
            let ref mut fresh38 = (*surf).next;
            *fresh38 = surf2;
            let ref mut fresh39 = (*surf).prev;
            *fresh39 = (*surf2).prev;
            let ref mut fresh40 = (*(*surf2).prev).next;
            *fresh40 = surf;
            let ref mut fresh41 = (*surf2).prev;
            *fresh41 = surf;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn R_GenerateSpans() {
    let mut edge: *mut edge_t = 0 as *mut edge_t;
    let mut surf: *mut surf_t = 0 as *mut surf_t;
    let ref mut fresh42 = (*surfaces.offset(1 as libc::c_int as isize)).prev;
    *fresh42 = &mut *surfaces.offset(1 as libc::c_int as isize) as *mut surf_t;
    let ref mut fresh43 = (*surfaces.offset(1 as libc::c_int as isize)).next;
    *fresh43 = *fresh42;
    (*surfaces.offset(1 as libc::c_int as isize)).last_u = edge_head_u_shift20;
    let mut current_block_5: u64;
    edge = edge_head.next;
    while edge != &mut edge_tail as *mut edge_t {
        if (*edge).surfs[0 as libc::c_int as usize] != 0 {
            surf = &mut *surfaces
                .offset(
                    *((*edge).surfs).as_mut_ptr().offset(0 as libc::c_int as isize)
                        as isize,
                ) as *mut surf_t;
            R_TrailingEdge(surf, edge);
            if (*edge).surfs[1 as libc::c_int as usize] == 0 {
                current_block_5 = 10680521327981672866;
            } else {
                current_block_5 = 5720623009719927633;
            }
        } else {
            current_block_5 = 5720623009719927633;
        }
        match current_block_5 {
            5720623009719927633 => {
                R_LeadingEdge(edge);
            }
            _ => {}
        }
        edge = (*edge).next;
    }
    R_CleanupSpan();
}
#[no_mangle]
pub unsafe extern "C" fn R_GenerateSpansBackward() {
    let mut edge: *mut edge_t = 0 as *mut edge_t;
    let ref mut fresh44 = (*surfaces.offset(1 as libc::c_int as isize)).prev;
    *fresh44 = &mut *surfaces.offset(1 as libc::c_int as isize) as *mut surf_t;
    let ref mut fresh45 = (*surfaces.offset(1 as libc::c_int as isize)).next;
    *fresh45 = *fresh44;
    (*surfaces.offset(1 as libc::c_int as isize)).last_u = edge_head_u_shift20;
    edge = edge_head.next;
    while edge != &mut edge_tail as *mut edge_t {
        if (*edge).surfs[0 as libc::c_int as usize] != 0 {
            R_TrailingEdge(
                &mut *surfaces
                    .offset(
                        *((*edge).surfs).as_mut_ptr().offset(0 as libc::c_int as isize)
                            as isize,
                    ),
                edge,
            );
        }
        if (*edge).surfs[1 as libc::c_int as usize] != 0 {
            R_LeadingEdgeBackwards(edge);
        }
        edge = (*edge).next;
    }
    R_CleanupSpan();
}
#[no_mangle]
pub unsafe extern "C" fn R_ScanEdges() {
    let mut iv: libc::c_int = 0;
    let mut bottom: libc::c_int = 0;
    let mut basespans: [byte; 72032] = [0; 72032];
    let mut basespan_p: *mut espan_t = 0 as *mut espan_t;
    let mut s: *mut surf_t = 0 as *mut surf_t;
    basespan_p = (basespans
        .as_mut_ptr()
        .offset(32 as libc::c_int as isize)
        .offset(-(1 as libc::c_int as isize)) as libc::c_long
        & !(32 as libc::c_int - 1 as libc::c_int) as libc::c_long) as *mut espan_t;
    max_span_p = &mut *basespan_p
        .offset((3000 as libc::c_int - r_refdef.vrect.width) as isize) as *mut espan_t;
    span_p = basespan_p;
    edge_head.u = r_refdef.vrect.x << 20 as libc::c_int;
    edge_head_u_shift20 = edge_head.u >> 20 as libc::c_int;
    edge_head.u_step = 0 as libc::c_int;
    edge_head.prev = 0 as *mut edge_s;
    edge_head.next = &mut edge_tail;
    edge_head.surfs[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_ushort;
    edge_head.surfs[1 as libc::c_int as usize] = 1 as libc::c_int as libc::c_ushort;
    edge_tail.u = (r_refdef.vrectright << 20 as libc::c_int) + 0xfffff as libc::c_int;
    edge_tail_u_shift20 = edge_tail.u >> 20 as libc::c_int;
    edge_tail.u_step = 0 as libc::c_int;
    edge_tail.prev = &mut edge_head;
    edge_tail.next = &mut edge_aftertail;
    edge_tail.surfs[0 as libc::c_int as usize] = 1 as libc::c_int as libc::c_ushort;
    edge_tail.surfs[1 as libc::c_int as usize] = 0 as libc::c_int as libc::c_ushort;
    edge_aftertail.u = -(1 as libc::c_int);
    edge_aftertail.u_step = 0 as libc::c_int;
    edge_aftertail.next = &mut edge_sentinel;
    edge_aftertail.prev = &mut edge_tail;
    edge_sentinel.u = (2000 as libc::c_int) << 24 as libc::c_int;
    edge_sentinel.prev = &mut edge_aftertail;
    bottom = r_refdef.vrectbottom - 1 as libc::c_int;
    iv = r_refdef.vrect.y;
    while iv < bottom {
        current_iv = iv;
        fv = iv as libc::c_float;
        (*surfaces.offset(1 as libc::c_int as isize)).spanstate = 1 as libc::c_int;
        if !(newedges[iv as usize]).is_null() {
            R_InsertNewEdges(newedges[iv as usize], edge_head.next);
        }
        (Some(pdrawfunc.expect("non-null function pointer")))
            .expect("non-null function pointer")();
        if span_p > max_span_p {
            D_DrawSurfaces();
            s = &mut *surfaces.offset(1 as libc::c_int as isize) as *mut surf_t;
            while s < surface_p {
                let ref mut fresh46 = (*s).spans;
                *fresh46 = 0 as *mut espan_s;
                s = s.offset(1);
            }
            span_p = basespan_p;
        }
        if !(removeedges[iv as usize]).is_null() {
            R_RemoveEdges(removeedges[iv as usize]);
        }
        if edge_head.next != &mut edge_tail as *mut edge_t {
            R_StepActiveU(edge_head.next);
        }
        iv += 1;
    }
    current_iv = iv;
    fv = iv as libc::c_float;
    (*surfaces.offset(1 as libc::c_int as isize)).spanstate = 1 as libc::c_int;
    if !(newedges[iv as usize]).is_null() {
        R_InsertNewEdges(newedges[iv as usize], edge_head.next);
    }
    (Some(pdrawfunc.expect("non-null function pointer")))
        .expect("non-null function pointer")();
    D_DrawSurfaces();
}
#[no_mangle]
pub static mut pface: *mut msurface_t = 0 as *const msurface_t as *mut msurface_t;
#[no_mangle]
pub static mut pcurrentcache: *mut surfcache_t = 0 as *const surfcache_t
    as *mut surfcache_t;
#[no_mangle]
pub static mut transformed_modelorg: vec3_t = [0.; 3];
#[no_mangle]
pub static mut world_transformed_modelorg: vec3_t = [0.; 3];
#[no_mangle]
pub static mut local_modelorg: vec3_t = [0.; 3];
#[no_mangle]
pub unsafe extern "C" fn D_MipLevelForScale(mut scale: libc::c_float) -> libc::c_int {
    let mut lmiplevel: libc::c_int = 0;
    if scale >= d_scalemip[0 as libc::c_int as usize] {
        lmiplevel = 0 as libc::c_int;
    } else if scale >= d_scalemip[1 as libc::c_int as usize] {
        lmiplevel = 1 as libc::c_int;
    } else if scale >= d_scalemip[2 as libc::c_int as usize] {
        lmiplevel = 2 as libc::c_int;
    } else {
        lmiplevel = 3 as libc::c_int;
    }
    if lmiplevel < d_minmip {
        lmiplevel = d_minmip;
    }
    return lmiplevel;
}
#[no_mangle]
pub unsafe extern "C" fn D_FlatFillSurface(
    mut surf: *mut surf_t,
    mut color: libc::c_int,
) {
    let mut span: *mut espan_t = 0 as *mut espan_t;
    let mut pdest: *mut byte = 0 as *mut byte;
    let mut u: libc::c_int = 0;
    let mut u2: libc::c_int = 0;
    span = (*surf).spans;
    while !span.is_null() {
        pdest = (d_viewbuffer as *mut byte).offset((r_screenwidth * (*span).v) as isize);
        u = (*span).u;
        u2 = (*span).u + (*span).count - 1 as libc::c_int;
        while u <= u2 {
            *pdest.offset(u as isize) = color as byte;
            u += 1;
        }
        span = (*span).pnext;
    }
}
#[no_mangle]
pub unsafe extern "C" fn D_CalcGradients(mut pface_0: *mut msurface_t) {
    let mut pplane: *mut mplane_t = 0 as *mut mplane_t;
    let mut mipscale: libc::c_float = 0.;
    let mut p_temp1: vec3_t = [0.; 3];
    let mut p_saxis: vec3_t = [0.; 3];
    let mut p_taxis: vec3_t = [0.; 3];
    let mut t: libc::c_float = 0.;
    pplane = (*pface_0).plane;
    mipscale = (1.0f64
        / ((1 as libc::c_int) << miplevel) as libc::c_float as libc::c_double)
        as libc::c_float;
    TransformVector(
        ((*(*pface_0).texinfo).vecs[0 as libc::c_int as usize]).as_mut_ptr(),
        p_saxis.as_mut_ptr(),
    );
    TransformVector(
        ((*(*pface_0).texinfo).vecs[1 as libc::c_int as usize]).as_mut_ptr(),
        p_taxis.as_mut_ptr(),
    );
    t = xscaleinv * mipscale;
    d_sdivzstepu = p_saxis[0 as libc::c_int as usize] * t;
    d_tdivzstepu = p_taxis[0 as libc::c_int as usize] * t;
    t = yscaleinv * mipscale;
    d_sdivzstepv = -p_saxis[1 as libc::c_int as usize] * t;
    d_tdivzstepv = -p_taxis[1 as libc::c_int as usize] * t;
    d_sdivzorigin = p_saxis[2 as libc::c_int as usize] * mipscale
        - xcenter * d_sdivzstepu - ycenter * d_sdivzstepv;
    d_tdivzorigin = p_taxis[2 as libc::c_int as usize] * mipscale
        - xcenter * d_tdivzstepu - ycenter * d_tdivzstepv;
    VectorScale(transformed_modelorg.as_mut_ptr(), mipscale, p_temp1.as_mut_ptr());
    t = 0x10000 as libc::c_int as libc::c_float * mipscale;
    sadjust = (((((p_temp1[0 as libc::c_int as usize]
        * p_saxis[0 as libc::c_int as usize]
        + p_temp1[1 as libc::c_int as usize] * p_saxis[1 as libc::c_int as usize]
        + p_temp1[2 as libc::c_int as usize] * p_saxis[2 as libc::c_int as usize])
        * 0x10000 as libc::c_int as libc::c_float) as libc::c_double + 0.5f64)
        as fixed16_t
        - (((*pface_0).texturemins[0 as libc::c_int as usize] as libc::c_int)
            << 16 as libc::c_int >> miplevel)) as libc::c_float
        + (*(*pface_0).texinfo)
            .vecs[0 as libc::c_int as usize][3 as libc::c_int as usize] * t)
        as fixed16_t;
    tadjust = (((((p_temp1[0 as libc::c_int as usize]
        * p_taxis[0 as libc::c_int as usize]
        + p_temp1[1 as libc::c_int as usize] * p_taxis[1 as libc::c_int as usize]
        + p_temp1[2 as libc::c_int as usize] * p_taxis[2 as libc::c_int as usize])
        * 0x10000 as libc::c_int as libc::c_float) as libc::c_double + 0.5f64)
        as fixed16_t
        - (((*pface_0).texturemins[1 as libc::c_int as usize] as libc::c_int)
            << 16 as libc::c_int >> miplevel)) as libc::c_float
        + (*(*pface_0).texinfo)
            .vecs[1 as libc::c_int as usize][3 as libc::c_int as usize] * t)
        as fixed16_t;
    if (*(*pface_0).texinfo).flags & 0x40 as libc::c_int != 0 {
        if (*(*pface_0).texinfo).flags & 0x8 as libc::c_int != 0 {
            sadjust = (sadjust as libc::c_double
                + 0x10000 as libc::c_int as libc::c_double
                    * (-(128 as libc::c_int) as libc::c_double
                        * (r_newrefdef.time as libc::c_double * 0.25f64
                            - (r_newrefdef.time as libc::c_double * 0.25f64)
                                as libc::c_int as libc::c_double))) as fixed16_t;
        } else {
            sadjust = (sadjust as libc::c_double
                + 0x10000 as libc::c_int as libc::c_double
                    * (-(128 as libc::c_int) as libc::c_double
                        * (r_newrefdef.time as libc::c_double * 0.77f64
                            - (r_newrefdef.time as libc::c_double * 0.77f64)
                                as libc::c_int as libc::c_double))) as fixed16_t;
        }
    }
    bbextents = (((*pface_0).extents[0 as libc::c_int as usize] as libc::c_int)
        << 16 as libc::c_int >> miplevel) - 1 as libc::c_int;
    bbextentt = (((*pface_0).extents[1 as libc::c_int as usize] as libc::c_int)
        << 16 as libc::c_int >> miplevel) - 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn D_BackgroundSurf(mut s: *mut surf_t) {
    d_zistepu = 0 as libc::c_int as libc::c_float;
    d_zistepv = 0 as libc::c_int as libc::c_float;
    d_ziorigin = -0.9f64 as libc::c_float;
    D_FlatFillSurface(s, (*sw_clearcolor).value as libc::c_int & 0xff as libc::c_int);
    D_DrawZSpans((*s).spans);
}
#[no_mangle]
pub unsafe extern "C" fn D_TurbulentSurf(mut s: *mut surf_t) {
    d_zistepu = (*s).d_zistepu;
    d_zistepv = (*s).d_zistepv;
    d_ziorigin = (*s).d_ziorigin;
    pface = (*s).msurf;
    miplevel = 0 as libc::c_int;
    cacheblock = (*(*(*pface).texinfo).image).pixels[0 as libc::c_int as usize];
    cachewidth = 64 as libc::c_int;
    if (*s).insubmodel as u64 != 0 {
        currententity = (*s).entity;
        local_modelorg[0 as libc::c_int
            as usize] = r_origin[0 as libc::c_int as usize]
            - (*currententity).origin[0 as libc::c_int as usize];
        local_modelorg[1 as libc::c_int
            as usize] = r_origin[1 as libc::c_int as usize]
            - (*currententity).origin[1 as libc::c_int as usize];
        local_modelorg[2 as libc::c_int
            as usize] = r_origin[2 as libc::c_int as usize]
            - (*currententity).origin[2 as libc::c_int as usize];
        TransformVector(local_modelorg.as_mut_ptr(), transformed_modelorg.as_mut_ptr());
        R_RotateBmodel();
    }
    D_CalcGradients(pface);
    if (*(*pface).texinfo).flags & 0x8 as libc::c_int == 0 {
        NonTurbulent8((*s).spans);
    } else {
        Turbulent8((*s).spans);
    }
    D_DrawZSpans((*s).spans);
    if (*s).insubmodel as u64 != 0 {
        currententity = 0 as *mut entity_t;
        transformed_modelorg[0 as libc::c_int
            as usize] = world_transformed_modelorg[0 as libc::c_int as usize];
        transformed_modelorg[1 as libc::c_int
            as usize] = world_transformed_modelorg[1 as libc::c_int as usize];
        transformed_modelorg[2 as libc::c_int
            as usize] = world_transformed_modelorg[2 as libc::c_int as usize];
        vpn[0 as libc::c_int as usize] = base_vpn[0 as libc::c_int as usize];
        vpn[1 as libc::c_int as usize] = base_vpn[1 as libc::c_int as usize];
        vpn[2 as libc::c_int as usize] = base_vpn[2 as libc::c_int as usize];
        vup[0 as libc::c_int as usize] = base_vup[0 as libc::c_int as usize];
        vup[1 as libc::c_int as usize] = base_vup[1 as libc::c_int as usize];
        vup[2 as libc::c_int as usize] = base_vup[2 as libc::c_int as usize];
        vright[0 as libc::c_int as usize] = base_vright[0 as libc::c_int as usize];
        vright[1 as libc::c_int as usize] = base_vright[1 as libc::c_int as usize];
        vright[2 as libc::c_int as usize] = base_vright[2 as libc::c_int as usize];
        R_TransformFrustum();
    }
}
#[no_mangle]
pub unsafe extern "C" fn D_SkySurf(mut s: *mut surf_t) {
    pface = (*s).msurf;
    miplevel = 0 as libc::c_int;
    if ((*(*pface).texinfo).image).is_null() {
        return;
    }
    cacheblock = (*(*(*pface).texinfo).image).pixels[0 as libc::c_int as usize];
    cachewidth = 256 as libc::c_int;
    d_zistepu = (*s).d_zistepu;
    d_zistepv = (*s).d_zistepv;
    d_ziorigin = (*s).d_ziorigin;
    D_CalcGradients(pface);
    D_DrawSpans16((*s).spans);
    d_zistepu = 0 as libc::c_int as libc::c_float;
    d_zistepv = 0 as libc::c_int as libc::c_float;
    d_ziorigin = -0.9f64 as libc::c_float;
    D_DrawZSpans((*s).spans);
}
#[no_mangle]
pub unsafe extern "C" fn D_SolidSurf(mut s: *mut surf_t) {
    d_zistepu = (*s).d_zistepu;
    d_zistepv = (*s).d_zistepv;
    d_ziorigin = (*s).d_ziorigin;
    if (*s).insubmodel as u64 != 0 {
        currententity = (*s).entity;
        local_modelorg[0 as libc::c_int
            as usize] = r_origin[0 as libc::c_int as usize]
            - (*currententity).origin[0 as libc::c_int as usize];
        local_modelorg[1 as libc::c_int
            as usize] = r_origin[1 as libc::c_int as usize]
            - (*currententity).origin[1 as libc::c_int as usize];
        local_modelorg[2 as libc::c_int
            as usize] = r_origin[2 as libc::c_int as usize]
            - (*currententity).origin[2 as libc::c_int as usize];
        TransformVector(local_modelorg.as_mut_ptr(), transformed_modelorg.as_mut_ptr());
        R_RotateBmodel();
    } else {
        currententity = &mut r_worldentity;
    }
    pface = (*s).msurf;
    miplevel = D_MipLevelForScale(
        (*s).nearzi * scale_for_mip * (*(*pface).texinfo).mipadjust,
    );
    pcurrentcache = D_CacheSurface(pface, miplevel);
    cacheblock = ((*pcurrentcache).data).as_mut_ptr() as *mut pixel_t;
    cachewidth = (*pcurrentcache).width as libc::c_int;
    D_CalcGradients(pface);
    D_DrawSpans16((*s).spans);
    D_DrawZSpans((*s).spans);
    if (*s).insubmodel as u64 != 0 {
        transformed_modelorg[0 as libc::c_int
            as usize] = world_transformed_modelorg[0 as libc::c_int as usize];
        transformed_modelorg[1 as libc::c_int
            as usize] = world_transformed_modelorg[1 as libc::c_int as usize];
        transformed_modelorg[2 as libc::c_int
            as usize] = world_transformed_modelorg[2 as libc::c_int as usize];
        vpn[0 as libc::c_int as usize] = base_vpn[0 as libc::c_int as usize];
        vpn[1 as libc::c_int as usize] = base_vpn[1 as libc::c_int as usize];
        vpn[2 as libc::c_int as usize] = base_vpn[2 as libc::c_int as usize];
        vup[0 as libc::c_int as usize] = base_vup[0 as libc::c_int as usize];
        vup[1 as libc::c_int as usize] = base_vup[1 as libc::c_int as usize];
        vup[2 as libc::c_int as usize] = base_vup[2 as libc::c_int as usize];
        vright[0 as libc::c_int as usize] = base_vright[0 as libc::c_int as usize];
        vright[1 as libc::c_int as usize] = base_vright[1 as libc::c_int as usize];
        vright[2 as libc::c_int as usize] = base_vright[2 as libc::c_int as usize];
        R_TransformFrustum();
        currententity = 0 as *mut entity_t;
    }
}
#[no_mangle]
pub unsafe extern "C" fn D_DrawflatSurfaces() {
    let mut s: *mut surf_t = 0 as *mut surf_t;
    s = &mut *surfaces.offset(1 as libc::c_int as isize) as *mut surf_t;
    while s < surface_p {
        if !((*s).spans).is_null() {
            d_zistepu = (*s).d_zistepu;
            d_zistepv = (*s).d_zistepv;
            d_ziorigin = (*s).d_ziorigin;
            D_FlatFillSurface(s, (*s).msurf as libc::c_int & 0xff as libc::c_int);
            D_DrawZSpans((*s).spans);
        }
        s = s.offset(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn D_DrawSurfaces() {
    let mut s: *mut surf_t = 0 as *mut surf_t;
    modelorg[0 as libc::c_int
        as usize] = r_origin[0 as libc::c_int as usize]
        - vec3_origin[0 as libc::c_int as usize];
    modelorg[1 as libc::c_int
        as usize] = r_origin[1 as libc::c_int as usize]
        - vec3_origin[1 as libc::c_int as usize];
    modelorg[2 as libc::c_int
        as usize] = r_origin[2 as libc::c_int as usize]
        - vec3_origin[2 as libc::c_int as usize];
    TransformVector(modelorg.as_mut_ptr(), transformed_modelorg.as_mut_ptr());
    world_transformed_modelorg[0 as libc::c_int
        as usize] = transformed_modelorg[0 as libc::c_int as usize];
    world_transformed_modelorg[1 as libc::c_int
        as usize] = transformed_modelorg[1 as libc::c_int as usize];
    world_transformed_modelorg[2 as libc::c_int
        as usize] = transformed_modelorg[2 as libc::c_int as usize];
    if (*sw_drawflat).value == 0. {
        s = &mut *surfaces.offset(1 as libc::c_int as isize) as *mut surf_t;
        while s < surface_p {
            if !((*s).spans).is_null() {
                r_drawnpolycount += 1;
                if (*s).flags
                    & (0x80 as libc::c_int | 0x40 as libc::c_int | 0x10 as libc::c_int)
                    == 0
                {
                    D_SolidSurf(s);
                } else if (*s).flags & 0x80 as libc::c_int != 0 {
                    D_SkySurf(s);
                } else if (*s).flags & 0x40 as libc::c_int != 0 {
                    D_BackgroundSurf(s);
                } else if (*s).flags & 0x10 as libc::c_int != 0 {
                    D_TurbulentSurf(s);
                }
            }
            s = s.offset(1);
        }
    } else {
        D_DrawflatSurfaces();
    }
    currententity = 0 as *mut entity_t;
    modelorg[0 as libc::c_int
        as usize] = r_origin[0 as libc::c_int as usize]
        - vec3_origin[0 as libc::c_int as usize];
    modelorg[1 as libc::c_int
        as usize] = r_origin[1 as libc::c_int as usize]
        - vec3_origin[1 as libc::c_int as usize];
    modelorg[2 as libc::c_int
        as usize] = r_origin[2 as libc::c_int as usize]
        - vec3_origin[2 as libc::c_int as usize];
    R_TransformFrustum();
}
