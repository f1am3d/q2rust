#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    fn fabs(_: libc::c_double) -> libc::c_double;
    static mut vec3_origin: vec3_t;
    fn VectorMA(
        veca: *mut vec_t,
        scale: libc::c_float,
        vecb: *mut vec_t,
        vecc: *mut vec_t,
    );
    fn VectorLength(v: *mut vec_t) -> vec_t;
    static mut r_framecount: libc::c_int;
    static mut r_drawsurf: drawsurf_t;
    static mut r_fullbright: *mut cvar_t;
    static mut currententity: *mut entity_t;
    static mut r_worldmodel: *mut model_t;
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
pub type model_t = model_s;
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
#[no_mangle]
pub static mut r_dlightframecount: libc::c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn R_MarkLights(
    mut light: *mut dlight_t,
    mut bit: libc::c_int,
    mut node: *mut mnode_t,
) {
    let mut splitplane: *mut mplane_t = 0 as *mut mplane_t;
    let mut dist: libc::c_float = 0.;
    let mut surf: *mut msurface_t = 0 as *mut msurface_t;
    let mut i: libc::c_int = 0;
    if (*node).contents != -(1 as libc::c_int) {
        return;
    }
    splitplane = (*node).plane;
    dist = (*light).origin[0 as libc::c_int as usize]
        * (*splitplane).normal[0 as libc::c_int as usize]
        + (*light).origin[1 as libc::c_int as usize]
            * (*splitplane).normal[1 as libc::c_int as usize]
        + (*light).origin[2 as libc::c_int as usize]
            * (*splitplane).normal[2 as libc::c_int as usize] - (*splitplane).dist;
    i = (*light).intensity as libc::c_int;
    if i < 0 as libc::c_int {
        i = -i;
    }
    if dist > i as libc::c_float {
        R_MarkLights(light, bit, (*node).children[0 as libc::c_int as usize]);
        return;
    }
    if dist < -i as libc::c_float {
        R_MarkLights(light, bit, (*node).children[1 as libc::c_int as usize]);
        return;
    }
    surf = ((*r_worldmodel).surfaces)
        .offset((*node).firstsurface as libc::c_int as isize);
    i = 0 as libc::c_int;
    while i < (*node).numsurfaces as libc::c_int {
        if (*surf).dlightframe != r_dlightframecount {
            (*surf).dlightbits = 0 as libc::c_int;
            (*surf).dlightframe = r_dlightframecount;
        }
        (*surf).dlightbits |= bit;
        i += 1;
        surf = surf.offset(1);
    }
    R_MarkLights(light, bit, (*node).children[0 as libc::c_int as usize]);
    R_MarkLights(light, bit, (*node).children[1 as libc::c_int as usize]);
}
#[no_mangle]
pub unsafe extern "C" fn R_PushDlights(mut model: *mut model_t) {
    let mut i: libc::c_int = 0;
    let mut l: *mut dlight_t = 0 as *mut dlight_t;
    r_dlightframecount = r_framecount;
    i = 0 as libc::c_int;
    l = r_newrefdef.dlights;
    while i < r_newrefdef.num_dlights {
        R_MarkLights(
            l,
            (1 as libc::c_int) << i,
            ((*model).nodes).offset((*model).firstnode as isize),
        );
        i += 1;
        l = l.offset(1);
    }
}
#[no_mangle]
pub static mut pointcolor: vec3_t = [0.; 3];
#[no_mangle]
pub static mut lightplane: *mut mplane_t = 0 as *const mplane_t as *mut mplane_t;
#[no_mangle]
pub static mut lightspot: vec3_t = [0.; 3];
#[no_mangle]
pub unsafe extern "C" fn RecursiveLightPoint(
    mut node: *mut mnode_t,
    mut start: *mut vec_t,
    mut end: *mut vec_t,
) -> libc::c_int {
    let mut front: libc::c_float = 0.;
    let mut back: libc::c_float = 0.;
    let mut frac: libc::c_float = 0.;
    let mut side: libc::c_int = 0;
    let mut plane: *mut mplane_t = 0 as *mut mplane_t;
    let mut mid: vec3_t = [0.; 3];
    let mut surf: *mut msurface_t = 0 as *mut msurface_t;
    let mut s: libc::c_int = 0;
    let mut t: libc::c_int = 0;
    let mut ds: libc::c_int = 0;
    let mut dt: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut tex: *mut mtexinfo_t = 0 as *mut mtexinfo_t;
    let mut lightmap: *mut byte = 0 as *mut byte;
    let mut scales: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut maps: libc::c_int = 0;
    let mut samp: libc::c_float = 0.;
    let mut r: libc::c_int = 0;
    if (*node).contents != -(1 as libc::c_int) {
        return -(1 as libc::c_int);
    }
    plane = (*node).plane;
    front = *start.offset(0 as libc::c_int as isize)
        * (*plane).normal[0 as libc::c_int as usize]
        + *start.offset(1 as libc::c_int as isize)
            * (*plane).normal[1 as libc::c_int as usize]
        + *start.offset(2 as libc::c_int as isize)
            * (*plane).normal[2 as libc::c_int as usize] - (*plane).dist;
    back = *end.offset(0 as libc::c_int as isize)
        * (*plane).normal[0 as libc::c_int as usize]
        + *end.offset(1 as libc::c_int as isize)
            * (*plane).normal[1 as libc::c_int as usize]
        + *end.offset(2 as libc::c_int as isize)
            * (*plane).normal[2 as libc::c_int as usize] - (*plane).dist;
    side = (front < 0 as libc::c_int as libc::c_float) as libc::c_int;
    if (back < 0 as libc::c_int as libc::c_float) as libc::c_int == side {
        return RecursiveLightPoint((*node).children[side as usize], start, end);
    }
    frac = front / (front - back);
    mid[0 as libc::c_int
        as usize] = *start.offset(0 as libc::c_int as isize)
        + (*end.offset(0 as libc::c_int as isize)
            - *start.offset(0 as libc::c_int as isize)) * frac;
    mid[1 as libc::c_int
        as usize] = *start.offset(1 as libc::c_int as isize)
        + (*end.offset(1 as libc::c_int as isize)
            - *start.offset(1 as libc::c_int as isize)) * frac;
    mid[2 as libc::c_int
        as usize] = *start.offset(2 as libc::c_int as isize)
        + (*end.offset(2 as libc::c_int as isize)
            - *start.offset(2 as libc::c_int as isize)) * frac;
    if ((*plane).type_0 as libc::c_int) < 3 as libc::c_int {
        mid[(*plane).type_0 as usize] = (*plane).dist;
    }
    r = RecursiveLightPoint((*node).children[side as usize], start, mid.as_mut_ptr());
    if r >= 0 as libc::c_int {
        return r;
    }
    if (back < 0 as libc::c_int as libc::c_float) as libc::c_int == side {
        return -(1 as libc::c_int);
    }
    lightspot[0 as libc::c_int as usize] = mid[0 as libc::c_int as usize];
    lightspot[1 as libc::c_int as usize] = mid[1 as libc::c_int as usize];
    lightspot[2 as libc::c_int as usize] = mid[2 as libc::c_int as usize];
    lightplane = plane;
    surf = ((*r_worldmodel).surfaces)
        .offset((*node).firstsurface as libc::c_int as isize);
    i = 0 as libc::c_int;
    while i < (*node).numsurfaces as libc::c_int {
        if !((*surf).flags & (0x10 as libc::c_int | 4 as libc::c_int) != 0) {
            tex = (*surf).texinfo;
            s = (mid[0 as libc::c_int as usize]
                * (*tex).vecs[0 as libc::c_int as usize][0 as libc::c_int as usize]
                + mid[1 as libc::c_int as usize]
                    * (*tex).vecs[0 as libc::c_int as usize][1 as libc::c_int as usize]
                + mid[2 as libc::c_int as usize]
                    * (*tex).vecs[0 as libc::c_int as usize][2 as libc::c_int as usize]
                + (*tex).vecs[0 as libc::c_int as usize][3 as libc::c_int as usize])
                as libc::c_int;
            t = (mid[0 as libc::c_int as usize]
                * (*tex).vecs[1 as libc::c_int as usize][0 as libc::c_int as usize]
                + mid[1 as libc::c_int as usize]
                    * (*tex).vecs[1 as libc::c_int as usize][1 as libc::c_int as usize]
                + mid[2 as libc::c_int as usize]
                    * (*tex).vecs[1 as libc::c_int as usize][2 as libc::c_int as usize]
                + (*tex).vecs[1 as libc::c_int as usize][3 as libc::c_int as usize])
                as libc::c_int;
            if !(s < (*surf).texturemins[0 as libc::c_int as usize] as libc::c_int
                || t < (*surf).texturemins[1 as libc::c_int as usize] as libc::c_int)
            {
                ds = s - (*surf).texturemins[0 as libc::c_int as usize] as libc::c_int;
                dt = t - (*surf).texturemins[1 as libc::c_int as usize] as libc::c_int;
                if !(ds > (*surf).extents[0 as libc::c_int as usize] as libc::c_int
                    || dt > (*surf).extents[1 as libc::c_int as usize] as libc::c_int)
                {
                    if ((*surf).samples).is_null() {
                        return 0 as libc::c_int;
                    }
                    ds >>= 4 as libc::c_int;
                    dt >>= 4 as libc::c_int;
                    lightmap = (*surf).samples;
                    pointcolor[0 as libc::c_int
                        as usize] = vec3_origin[0 as libc::c_int as usize];
                    pointcolor[1 as libc::c_int
                        as usize] = vec3_origin[1 as libc::c_int as usize];
                    pointcolor[2 as libc::c_int
                        as usize] = vec3_origin[2 as libc::c_int as usize];
                    if !lightmap.is_null() {
                        lightmap = lightmap
                            .offset(
                                (dt
                                    * (((*surf).extents[0 as libc::c_int as usize]
                                        as libc::c_int >> 4 as libc::c_int) + 1 as libc::c_int)
                                    + ds) as isize,
                            );
                        maps = 0 as libc::c_int;
                        while maps < 4 as libc::c_int
                            && (*surf).styles[maps as usize] as libc::c_int
                                != 255 as libc::c_int
                        {
                            samp = (*lightmap as libc::c_int as libc::c_double
                                * (1.0f64 / 255 as libc::c_int as libc::c_double))
                                as libc::c_float;
                            scales = ((*(r_newrefdef.lightstyles)
                                .offset((*surf).styles[maps as usize] as isize))
                                .rgb)
                                .as_mut_ptr();
                            VectorMA(
                                pointcolor.as_mut_ptr(),
                                samp,
                                scales,
                                pointcolor.as_mut_ptr(),
                            );
                            lightmap = lightmap
                                .offset(
                                    ((((*surf).extents[0 as libc::c_int as usize] as libc::c_int
                                        >> 4 as libc::c_int) + 1 as libc::c_int)
                                        * (((*surf).extents[1 as libc::c_int as usize]
                                            as libc::c_int >> 4 as libc::c_int) + 1 as libc::c_int))
                                        as isize,
                                );
                            maps += 1;
                        }
                    }
                    return 1 as libc::c_int;
                }
            }
        }
        i += 1;
        surf = surf.offset(1);
    }
    return RecursiveLightPoint(
        (*node).children[(side == 0) as libc::c_int as usize],
        mid.as_mut_ptr(),
        end,
    );
}
#[no_mangle]
pub unsafe extern "C" fn R_LightPoint(mut p: *mut vec_t, mut color: *mut vec_t) {
    let mut end: vec3_t = [0.; 3];
    let mut r: libc::c_float = 0.;
    let mut lnum: libc::c_int = 0;
    let mut dl: *mut dlight_t = 0 as *mut dlight_t;
    let mut light: libc::c_float = 0.;
    let mut dist: vec3_t = [0.; 3];
    let mut add: libc::c_float = 0.;
    if ((*r_worldmodel).lightdata).is_null() {
        let ref mut fresh0 = *color.offset(2 as libc::c_int as isize);
        *fresh0 = 1.0f64 as vec_t;
        let ref mut fresh1 = *color.offset(1 as libc::c_int as isize);
        *fresh1 = *fresh0;
        *color.offset(0 as libc::c_int as isize) = *fresh1;
        return;
    }
    end[0 as libc::c_int as usize] = *p.offset(0 as libc::c_int as isize);
    end[1 as libc::c_int as usize] = *p.offset(1 as libc::c_int as isize);
    end[2 as libc::c_int
        as usize] = *p.offset(2 as libc::c_int as isize)
        - 2048 as libc::c_int as libc::c_float;
    r = RecursiveLightPoint((*r_worldmodel).nodes, p, end.as_mut_ptr()) as libc::c_float;
    if r == -(1 as libc::c_int) as libc::c_float {
        *color
            .offset(0 as libc::c_int as isize) = vec3_origin[0 as libc::c_int as usize];
        *color
            .offset(1 as libc::c_int as isize) = vec3_origin[1 as libc::c_int as usize];
        *color
            .offset(2 as libc::c_int as isize) = vec3_origin[2 as libc::c_int as usize];
    } else {
        *color.offset(0 as libc::c_int as isize) = pointcolor[0 as libc::c_int as usize];
        *color.offset(1 as libc::c_int as isize) = pointcolor[1 as libc::c_int as usize];
        *color.offset(2 as libc::c_int as isize) = pointcolor[2 as libc::c_int as usize];
    }
    light = 0 as libc::c_int as libc::c_float;
    lnum = 0 as libc::c_int;
    while lnum < r_newrefdef.num_dlights {
        dl = &mut *(r_newrefdef.dlights).offset(lnum as isize) as *mut dlight_t;
        dist[0 as libc::c_int
            as usize] = (*currententity).origin[0 as libc::c_int as usize]
            - (*dl).origin[0 as libc::c_int as usize];
        dist[1 as libc::c_int
            as usize] = (*currententity).origin[1 as libc::c_int as usize]
            - (*dl).origin[1 as libc::c_int as usize];
        dist[2 as libc::c_int
            as usize] = (*currententity).origin[2 as libc::c_int as usize]
            - (*dl).origin[2 as libc::c_int as usize];
        add = (*dl).intensity - VectorLength(dist.as_mut_ptr());
        add = (add as libc::c_double * (1.0f64 / 256 as libc::c_int as libc::c_double))
            as libc::c_float;
        if add > 0 as libc::c_int as libc::c_float {
            VectorMA(color, add, ((*dl).color).as_mut_ptr(), color);
        }
        lnum += 1;
    }
}
#[no_mangle]
pub static mut blocklights: [libc::c_uint; 1024] = [0; 1024];
#[no_mangle]
pub unsafe extern "C" fn R_AddDynamicLights() {
    let mut surf: *mut msurface_t = 0 as *mut msurface_t;
    let mut lnum: libc::c_int = 0;
    let mut sd: libc::c_int = 0;
    let mut td: libc::c_int = 0;
    let mut dist: libc::c_float = 0.;
    let mut rad: libc::c_float = 0.;
    let mut minlight: libc::c_float = 0.;
    let mut impact: vec3_t = [0.; 3];
    let mut local: vec3_t = [0.; 3];
    let mut s: libc::c_int = 0;
    let mut t: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut smax: libc::c_int = 0;
    let mut tmax: libc::c_int = 0;
    let mut tex: *mut mtexinfo_t = 0 as *mut mtexinfo_t;
    let mut dl: *mut dlight_t = 0 as *mut dlight_t;
    let mut negativeLight: libc::c_int = 0;
    surf = r_drawsurf.surf;
    smax = ((*surf).extents[0 as libc::c_int as usize] as libc::c_int
        >> 4 as libc::c_int) + 1 as libc::c_int;
    tmax = ((*surf).extents[1 as libc::c_int as usize] as libc::c_int
        >> 4 as libc::c_int) + 1 as libc::c_int;
    tex = (*surf).texinfo;
    lnum = 0 as libc::c_int;
    while lnum < r_newrefdef.num_dlights {
        if !((*surf).dlightbits & (1 as libc::c_int) << lnum == 0) {
            dl = &mut *(r_newrefdef.dlights).offset(lnum as isize) as *mut dlight_t;
            rad = (*dl).intensity;
            negativeLight = 0 as libc::c_int;
            if rad < 0 as libc::c_int as libc::c_float {
                negativeLight = 1 as libc::c_int;
                rad = -rad;
            }
            dist = (*dl).origin[0 as libc::c_int as usize]
                * (*(*surf).plane).normal[0 as libc::c_int as usize]
                + (*dl).origin[1 as libc::c_int as usize]
                    * (*(*surf).plane).normal[1 as libc::c_int as usize]
                + (*dl).origin[2 as libc::c_int as usize]
                    * (*(*surf).plane).normal[2 as libc::c_int as usize]
                - (*(*surf).plane).dist;
            rad = (rad as libc::c_double - fabs(dist as libc::c_double))
                as libc::c_float;
            minlight = 32 as libc::c_int as libc::c_float;
            if !(rad < minlight) {
                minlight = rad - minlight;
                i = 0 as libc::c_int;
                while i < 3 as libc::c_int {
                    impact[i
                        as usize] = (*dl).origin[i as usize]
                        - (*(*surf).plane).normal[i as usize] * dist;
                    i += 1;
                }
                local[0 as libc::c_int
                    as usize] = impact[0 as libc::c_int as usize]
                    * (*tex).vecs[0 as libc::c_int as usize][0 as libc::c_int as usize]
                    + impact[1 as libc::c_int as usize]
                        * (*tex)
                            .vecs[0 as libc::c_int as usize][1 as libc::c_int as usize]
                    + impact[2 as libc::c_int as usize]
                        * (*tex)
                            .vecs[0 as libc::c_int as usize][2 as libc::c_int as usize]
                    + (*tex).vecs[0 as libc::c_int as usize][3 as libc::c_int as usize];
                local[1 as libc::c_int
                    as usize] = impact[0 as libc::c_int as usize]
                    * (*tex).vecs[1 as libc::c_int as usize][0 as libc::c_int as usize]
                    + impact[1 as libc::c_int as usize]
                        * (*tex)
                            .vecs[1 as libc::c_int as usize][1 as libc::c_int as usize]
                    + impact[2 as libc::c_int as usize]
                        * (*tex)
                            .vecs[1 as libc::c_int as usize][2 as libc::c_int as usize]
                    + (*tex).vecs[1 as libc::c_int as usize][3 as libc::c_int as usize];
                local[0 as libc::c_int as usize]
                    -= (*surf).texturemins[0 as libc::c_int as usize] as libc::c_int
                        as libc::c_float;
                local[1 as libc::c_int as usize]
                    -= (*surf).texturemins[1 as libc::c_int as usize] as libc::c_int
                        as libc::c_float;
                t = 0 as libc::c_int;
                while t < tmax {
                    td = (local[1 as libc::c_int as usize]
                        - (t * 16 as libc::c_int) as libc::c_float) as libc::c_int;
                    if td < 0 as libc::c_int {
                        td = -td;
                    }
                    s = 0 as libc::c_int;
                    while s < smax {
                        sd = (local[0 as libc::c_int as usize]
                            - (s * 16 as libc::c_int) as libc::c_float) as libc::c_int;
                        if sd < 0 as libc::c_int {
                            sd = -sd;
                        }
                        if sd > td {
                            dist = (sd + (td >> 1 as libc::c_int)) as libc::c_float;
                        } else {
                            dist = (td + (sd >> 1 as libc::c_int)) as libc::c_float;
                        }
                        if negativeLight == 0 {
                            if dist < minlight {
                                blocklights[(t * smax + s)
                                    as usize] = (blocklights[(t * smax + s) as usize]
                                    as libc::c_float
                                    + (rad - dist) * 256 as libc::c_int as libc::c_float)
                                    as libc::c_uint;
                            }
                        } else {
                            if dist < minlight {
                                blocklights[(t * smax + s)
                                    as usize] = (blocklights[(t * smax + s) as usize]
                                    as libc::c_float
                                    - (rad - dist) * 256 as libc::c_int as libc::c_float)
                                    as libc::c_uint;
                            }
                            if (blocklights[(t * smax + s) as usize] as libc::c_float)
                                < minlight
                            {
                                blocklights[(t * smax + s)
                                    as usize] = minlight as libc::c_uint;
                            }
                        }
                        s += 1;
                    }
                    t += 1;
                }
            }
        }
        lnum += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn R_BuildLightMap() {
    let mut smax: libc::c_int = 0;
    let mut tmax: libc::c_int = 0;
    let mut t: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut size: libc::c_int = 0;
    let mut lightmap: *mut byte = 0 as *mut byte;
    let mut scale: libc::c_uint = 0;
    let mut maps: libc::c_int = 0;
    let mut surf: *mut msurface_t = 0 as *mut msurface_t;
    surf = r_drawsurf.surf;
    smax = ((*surf).extents[0 as libc::c_int as usize] as libc::c_int
        >> 4 as libc::c_int) + 1 as libc::c_int;
    tmax = ((*surf).extents[1 as libc::c_int as usize] as libc::c_int
        >> 4 as libc::c_int) + 1 as libc::c_int;
    size = smax * tmax;
    if (*r_fullbright).value != 0. || ((*r_worldmodel).lightdata).is_null() {
        i = 0 as libc::c_int;
        while i < size {
            blocklights[i as usize] = 0 as libc::c_int as libc::c_uint;
            i += 1;
        }
        return;
    }
    i = 0 as libc::c_int;
    while i < size {
        blocklights[i as usize] = 0 as libc::c_int as libc::c_uint;
        i += 1;
    }
    lightmap = (*surf).samples;
    if !lightmap.is_null() {
        maps = 0 as libc::c_int;
        while maps < 4 as libc::c_int
            && (*surf).styles[maps as usize] as libc::c_int != 255 as libc::c_int
        {
            scale = r_drawsurf.lightadj[maps as usize] as libc::c_uint;
            i = 0 as libc::c_int;
            while i < size {
                blocklights[i
                    as usize] = (blocklights[i as usize])
                    .wrapping_add(
                        (*lightmap.offset(i as isize) as libc::c_uint)
                            .wrapping_mul(scale),
                    );
                i += 1;
            }
            lightmap = lightmap.offset(size as isize);
            maps += 1;
        }
    }
    if (*surf).dlightframe == r_framecount {
        R_AddDynamicLights();
    }
    i = 0 as libc::c_int;
    while i < size {
        t = blocklights[i as usize] as libc::c_int;
        if t < 0 as libc::c_int {
            t = 0 as libc::c_int;
        }
        t = 255 as libc::c_int * 256 as libc::c_int - t
            >> 8 as libc::c_int - 6 as libc::c_int;
        if t < (1 as libc::c_int) << 6 as libc::c_int {
            t = (1 as libc::c_int) << 6 as libc::c_int;
        }
        blocklights[i as usize] = t as libc::c_uint;
        i += 1;
    }
}
