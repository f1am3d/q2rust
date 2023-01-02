#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    fn cos(_: libc::c_double) -> libc::c_double;
    fn sin(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    static mut vec3_origin: vec3_t;
    fn VectorMA(
        veca: *mut vec_t,
        scale: libc::c_float,
        vecb: *mut vec_t,
        vecc: *mut vec_t,
    );
    fn VectorLength(v: *mut vec_t) -> vec_t;
    fn VectorScale(in_0: *mut vec_t, scale: vec_t, out: *mut vec_t);
    static mut ri: refimport_t;
    static mut r_worldmodel: *mut model_t;
    static mut gl_modulate: *mut cvar_t;
    static mut gl_flashblend: *mut cvar_t;
    static mut gl_monolightmap: *mut cvar_t;
    static mut r_newrefdef: refdef_t;
    static mut r_origin: vec3_t;
    static mut vright: vec3_t;
    static mut vpn: vec3_t;
    static mut vup: vec3_t;
    static mut qglColor3f: Option::<
        unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int) -> (),
    >;
    static mut qglDepthMask: Option::<unsafe extern "C" fn(libc::c_int) -> ()>;
    static mut qglEnd: Option::<unsafe extern "C" fn() -> ()>;
    static mut qglVertex3fv: Option::<unsafe extern "C" fn(*const libc::c_int) -> ()>;
    static mut currententity: *mut entity_t;
    static mut r_framecount: libc::c_int;
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
pub struct model_s {
    pub name: [libc::c_char; 64],
    pub registration_sequence: libc::c_int,
    pub type_0: modtype_t,
    pub numframes: libc::c_int,
    pub flags: libc::c_int,
    pub mins: vec3_t,
    pub maxs: vec3_t,
    pub radius: libc::c_float,
    pub clipbox: qboolean,
    pub clipmins: vec3_t,
    pub clipmaxs: vec3_t,
    pub firstmodelsurface: libc::c_int,
    pub nummodelsurfaces: libc::c_int,
    pub lightmap: libc::c_int,
    pub numsubmodels: libc::c_int,
    pub submodels: *mut mmodel_t,
    pub numplanes: libc::c_int,
    pub planes: *mut cplane_t,
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
    pub extradatasize: libc::c_int,
    pub extradata: *mut libc::c_void,
}
pub type msurface_t = msurface_s;
pub type mnode_t = mnode_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mnode_s {
    pub contents: libc::c_int,
    pub visframe: libc::c_int,
    pub minmaxs: [libc::c_float; 6],
    pub parent: *mut mnode_s,
    pub plane: *mut cplane_t,
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
    pub minmaxs: [libc::c_float; 6],
    pub parent: *mut mnode_s,
    pub cluster: libc::c_int,
    pub area: libc::c_int,
    pub firstmarksurface: *mut *mut msurface_t,
    pub nummarksurfaces: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mmodel_t {
    pub mins: vec3_t,
    pub maxs: vec3_t,
    pub origin: vec3_t,
    pub radius: libc::c_float,
    pub headnode: libc::c_int,
    pub visleafs: libc::c_int,
    pub firstface: libc::c_int,
    pub numfaces: libc::c_int,
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
#[no_mangle]
pub static mut r_dlightframecount: libc::c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn R_RenderDlight(mut light: *mut dlight_t) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut a: libc::c_float = 0.;
    let mut v: vec3_t = [0.; 3];
    let mut rad: libc::c_float = 0.;
    rad = ((*light).intensity as libc::c_double * 0.35f64) as libc::c_float;
    v[0 as libc::c_int
        as usize] = (*light).origin[0 as libc::c_int as usize]
        - r_origin[0 as libc::c_int as usize];
    v[1 as libc::c_int
        as usize] = (*light).origin[1 as libc::c_int as usize]
        - r_origin[1 as libc::c_int as usize];
    v[2 as libc::c_int
        as usize] = (*light).origin[2 as libc::c_int as usize]
        - r_origin[2 as libc::c_int as usize];
    qglColor3f
        .expect(
            "non-null function pointer",
        )(
        ((*light).color[0 as libc::c_int as usize] as libc::c_double * 0.2f64)
            as libc::c_int,
        ((*light).color[1 as libc::c_int as usize] as libc::c_double * 0.2f64)
            as libc::c_int,
        ((*light).color[2 as libc::c_int as usize] as libc::c_double * 0.2f64)
            as libc::c_int,
    );
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        v[i as usize] = (*light).origin[i as usize] - vpn[i as usize] * rad;
        i += 1;
    }
    qglVertex3fv
        .expect("non-null function pointer")(v.as_mut_ptr() as *const libc::c_int);
    qglColor3f
        .expect(
            "non-null function pointer",
        )(0 as libc::c_int, 0 as libc::c_int, 0 as libc::c_int);
    i = 16 as libc::c_int;
    while i >= 0 as libc::c_int {
        a = (i as libc::c_double / 16.0f64 * 3.14159265358979323846f64
            * 2 as libc::c_int as libc::c_double) as libc::c_float;
        j = 0 as libc::c_int;
        while j < 3 as libc::c_int {
            v[j
                as usize] = ((*light).origin[j as usize] as libc::c_double
                + vright[j as usize] as libc::c_double * cos(a as libc::c_double)
                    * rad as libc::c_double
                + vup[j as usize] as libc::c_double * sin(a as libc::c_double)
                    * rad as libc::c_double) as vec_t;
            j += 1;
        }
        qglVertex3fv
            .expect("non-null function pointer")(v.as_mut_ptr() as *const libc::c_int);
        i -= 1;
    }
    qglEnd.expect("non-null function pointer")();
}
#[no_mangle]
pub unsafe extern "C" fn R_RenderDlights() {
    let mut i: libc::c_int = 0;
    let mut l: *mut dlight_t = 0 as *mut dlight_t;
    if (*gl_flashblend).value == 0. {
        return;
    }
    r_dlightframecount = r_framecount + 1 as libc::c_int;
    qglDepthMask.expect("non-null function pointer")(0 as libc::c_int);
    l = r_newrefdef.dlights;
    i = 0 as libc::c_int;
    while i < r_newrefdef.num_dlights {
        R_RenderDlight(l);
        i += 1;
        l = l.offset(1);
    }
    qglColor3f
        .expect(
            "non-null function pointer",
        )(1 as libc::c_int, 1 as libc::c_int, 1 as libc::c_int);
    qglDepthMask.expect("non-null function pointer")(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn R_MarkLights(
    mut light: *mut dlight_t,
    mut bit: libc::c_int,
    mut node: *mut mnode_t,
) {
    let mut splitplane: *mut cplane_t = 0 as *mut cplane_t;
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
    if dist > (*light).intensity - 64 as libc::c_int as libc::c_float {
        R_MarkLights(light, bit, (*node).children[0 as libc::c_int as usize]);
        return;
    }
    if dist < -(*light).intensity + 64 as libc::c_int as libc::c_float {
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
pub unsafe extern "C" fn R_PushDlights() {
    let mut i: libc::c_int = 0;
    let mut l: *mut dlight_t = 0 as *mut dlight_t;
    if (*gl_flashblend).value != 0. {
        return;
    }
    r_dlightframecount = r_framecount + 1 as libc::c_int;
    l = r_newrefdef.dlights;
    i = 0 as libc::c_int;
    while i < r_newrefdef.num_dlights {
        R_MarkLights(l, (1 as libc::c_int) << i, (*r_worldmodel).nodes);
        i += 1;
        l = l.offset(1);
    }
}
#[no_mangle]
pub static mut pointcolor: vec3_t = [0.; 3];
#[no_mangle]
pub static mut lightplane: *mut cplane_t = 0 as *const cplane_t as *mut cplane_t;
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
    let mut plane: *mut cplane_t = 0 as *mut cplane_t;
    let mut mid: vec3_t = [0.; 3];
    let mut surf: *mut msurface_t = 0 as *mut msurface_t;
    let mut s: libc::c_int = 0;
    let mut t: libc::c_int = 0;
    let mut ds: libc::c_int = 0;
    let mut dt: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut tex: *mut mtexinfo_t = 0 as *mut mtexinfo_t;
    let mut lightmap: *mut byte = 0 as *mut byte;
    let mut maps: libc::c_int = 0;
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
                        let mut scale: vec3_t = [0.; 3];
                        lightmap = lightmap
                            .offset(
                                (3 as libc::c_int
                                    * (dt
                                        * (((*surf).extents[0 as libc::c_int as usize]
                                            as libc::c_int >> 4 as libc::c_int) + 1 as libc::c_int)
                                        + ds)) as isize,
                            );
                        maps = 0 as libc::c_int;
                        while maps < 4 as libc::c_int
                            && (*surf).styles[maps as usize] as libc::c_int
                                != 255 as libc::c_int
                        {
                            i = 0 as libc::c_int;
                            while i < 3 as libc::c_int {
                                scale[i
                                    as usize] = (*gl_modulate).value
                                    * (*(r_newrefdef.lightstyles)
                                        .offset((*surf).styles[maps as usize] as isize))
                                        .rgb[i as usize];
                                i += 1;
                            }
                            pointcolor[0 as libc::c_int
                                as usize] = (pointcolor[0 as libc::c_int as usize]
                                as libc::c_double
                                + (*lightmap.offset(0 as libc::c_int as isize)
                                    as libc::c_int as libc::c_float
                                    * scale[0 as libc::c_int as usize]) as libc::c_double
                                    * (1.0f64 / 255 as libc::c_int as libc::c_double)) as vec_t;
                            pointcolor[1 as libc::c_int
                                as usize] = (pointcolor[1 as libc::c_int as usize]
                                as libc::c_double
                                + (*lightmap.offset(1 as libc::c_int as isize)
                                    as libc::c_int as libc::c_float
                                    * scale[1 as libc::c_int as usize]) as libc::c_double
                                    * (1.0f64 / 255 as libc::c_int as libc::c_double)) as vec_t;
                            pointcolor[2 as libc::c_int
                                as usize] = (pointcolor[2 as libc::c_int as usize]
                                as libc::c_double
                                + (*lightmap.offset(2 as libc::c_int as isize)
                                    as libc::c_int as libc::c_float
                                    * scale[2 as libc::c_int as usize]) as libc::c_double
                                    * (1.0f64 / 255 as libc::c_int as libc::c_double)) as vec_t;
                            lightmap = lightmap
                                .offset(
                                    (3 as libc::c_int
                                        * (((*surf).extents[0 as libc::c_int as usize]
                                            as libc::c_int >> 4 as libc::c_int) + 1 as libc::c_int)
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
    dl = r_newrefdef.dlights;
    lnum = 0 as libc::c_int;
    while lnum < r_newrefdef.num_dlights {
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
        dl = dl.offset(1);
    }
    VectorScale(color, (*gl_modulate).value, color);
}
static mut s_blocklights: [libc::c_float; 3468] = [0.; 3468];
#[no_mangle]
pub unsafe extern "C" fn R_AddDynamicLights(mut surf: *mut msurface_t) {
    let mut lnum: libc::c_int = 0;
    let mut sd: libc::c_int = 0;
    let mut td: libc::c_int = 0;
    let mut fdist: libc::c_float = 0.;
    let mut frad: libc::c_float = 0.;
    let mut fminlight: libc::c_float = 0.;
    let mut impact: vec3_t = [0.; 3];
    let mut local: vec3_t = [0.; 3];
    let mut s: libc::c_int = 0;
    let mut t: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut smax: libc::c_int = 0;
    let mut tmax: libc::c_int = 0;
    let mut tex: *mut mtexinfo_t = 0 as *mut mtexinfo_t;
    let mut dl: *mut dlight_t = 0 as *mut dlight_t;
    let mut pfBL: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut fsacc: libc::c_float = 0.;
    let mut ftacc: libc::c_float = 0.;
    smax = ((*surf).extents[0 as libc::c_int as usize] as libc::c_int
        >> 4 as libc::c_int) + 1 as libc::c_int;
    tmax = ((*surf).extents[1 as libc::c_int as usize] as libc::c_int
        >> 4 as libc::c_int) + 1 as libc::c_int;
    tex = (*surf).texinfo;
    lnum = 0 as libc::c_int;
    while lnum < r_newrefdef.num_dlights {
        if !((*surf).dlightbits & (1 as libc::c_int) << lnum == 0) {
            dl = &mut *(r_newrefdef.dlights).offset(lnum as isize) as *mut dlight_t;
            frad = (*dl).intensity;
            fdist = (*dl).origin[0 as libc::c_int as usize]
                * (*(*surf).plane).normal[0 as libc::c_int as usize]
                + (*dl).origin[1 as libc::c_int as usize]
                    * (*(*surf).plane).normal[1 as libc::c_int as usize]
                + (*dl).origin[2 as libc::c_int as usize]
                    * (*(*surf).plane).normal[2 as libc::c_int as usize]
                - (*(*surf).plane).dist;
            frad = (frad as libc::c_double - fabs(fdist as libc::c_double))
                as libc::c_float;
            fminlight = 64 as libc::c_int as libc::c_float;
            if !(frad < fminlight) {
                fminlight = frad - fminlight;
                i = 0 as libc::c_int;
                while i < 3 as libc::c_int {
                    impact[i
                        as usize] = (*dl).origin[i as usize]
                        - (*(*surf).plane).normal[i as usize] * fdist;
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
                    + (*tex).vecs[0 as libc::c_int as usize][3 as libc::c_int as usize]
                    - (*surf).texturemins[0 as libc::c_int as usize] as libc::c_int
                        as libc::c_float;
                local[1 as libc::c_int
                    as usize] = impact[0 as libc::c_int as usize]
                    * (*tex).vecs[1 as libc::c_int as usize][0 as libc::c_int as usize]
                    + impact[1 as libc::c_int as usize]
                        * (*tex)
                            .vecs[1 as libc::c_int as usize][1 as libc::c_int as usize]
                    + impact[2 as libc::c_int as usize]
                        * (*tex)
                            .vecs[1 as libc::c_int as usize][2 as libc::c_int as usize]
                    + (*tex).vecs[1 as libc::c_int as usize][3 as libc::c_int as usize]
                    - (*surf).texturemins[1 as libc::c_int as usize] as libc::c_int
                        as libc::c_float;
                pfBL = s_blocklights.as_mut_ptr();
                t = 0 as libc::c_int;
                ftacc = 0 as libc::c_int as libc::c_float;
                while t < tmax {
                    td = (local[1 as libc::c_int as usize] - ftacc) as libc::c_int;
                    if td < 0 as libc::c_int {
                        td = -td;
                    }
                    s = 0 as libc::c_int;
                    fsacc = 0 as libc::c_int as libc::c_float;
                    while s < smax {
                        sd = (local[0 as libc::c_int as usize] - fsacc) as libc::c_long
                            as libc::c_int;
                        if sd < 0 as libc::c_int {
                            sd = -sd;
                        }
                        if sd > td {
                            fdist = (sd + (td >> 1 as libc::c_int)) as libc::c_float;
                        } else {
                            fdist = (td + (sd >> 1 as libc::c_int)) as libc::c_float;
                        }
                        if fdist < fminlight {
                            *pfBL.offset(0 as libc::c_int as isize)
                                += (frad - fdist) * (*dl).color[0 as libc::c_int as usize];
                            *pfBL.offset(1 as libc::c_int as isize)
                                += (frad - fdist) * (*dl).color[1 as libc::c_int as usize];
                            *pfBL.offset(2 as libc::c_int as isize)
                                += (frad - fdist) * (*dl).color[2 as libc::c_int as usize];
                        }
                        s += 1;
                        fsacc += 16 as libc::c_int as libc::c_float;
                        pfBL = pfBL.offset(3 as libc::c_int as isize);
                    }
                    t += 1;
                    ftacc += 16 as libc::c_int as libc::c_float;
                }
            }
        }
        lnum += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn R_SetCacheState(mut surf: *mut msurface_t) {
    let mut maps: libc::c_int = 0;
    maps = 0 as libc::c_int;
    while maps < 4 as libc::c_int
        && (*surf).styles[maps as usize] as libc::c_int != 255 as libc::c_int
    {
        (*surf)
            .cached_light[maps
            as usize] = (*(r_newrefdef.lightstyles)
            .offset((*surf).styles[maps as usize] as isize))
            .white;
        maps += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn R_BuildLightMap(
    mut surf: *mut msurface_t,
    mut dest: *mut byte,
    mut stride: libc::c_int,
) {
    let mut smax: libc::c_int = 0;
    let mut tmax: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    let mut g: libc::c_int = 0;
    let mut b: libc::c_int = 0;
    let mut a: libc::c_int = 0;
    let mut max: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut size: libc::c_int = 0;
    let mut lightmap: *mut byte = 0 as *mut byte;
    let mut scale: [libc::c_float; 4] = [0.; 4];
    let mut nummaps: libc::c_int = 0;
    let mut bl: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut style: *mut lightstyle_t = 0 as *mut lightstyle_t;
    let mut monolightmap: libc::c_int = 0;
    if (*(*surf).texinfo).flags
        & (0x4 as libc::c_int | 0x10 as libc::c_int | 0x20 as libc::c_int
            | 0x8 as libc::c_int) != 0
    {
        (ri.Sys_Error)
            .expect(
                "non-null function pointer",
            )(
            1 as libc::c_int,
            b"R_BuildLightMap called for non-lit surface\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        );
    }
    smax = ((*surf).extents[0 as libc::c_int as usize] as libc::c_int
        >> 4 as libc::c_int) + 1 as libc::c_int;
    tmax = ((*surf).extents[1 as libc::c_int as usize] as libc::c_int
        >> 4 as libc::c_int) + 1 as libc::c_int;
    size = smax * tmax;
    if size as libc::c_ulong
        > ::std::mem::size_of::<[libc::c_float; 3468]>() as libc::c_ulong
            >> 4 as libc::c_int
    {
        (ri.Sys_Error)
            .expect(
                "non-null function pointer",
            )(
            1 as libc::c_int,
            b"Bad s_blocklights size\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    if ((*surf).samples).is_null() {
        let mut maps: libc::c_int = 0;
        i = 0 as libc::c_int;
        while i < size * 3 as libc::c_int {
            s_blocklights[i as usize] = 255 as libc::c_int as libc::c_float;
            i += 1;
        }
        maps = 0 as libc::c_int;
        while maps < 4 as libc::c_int
            && (*surf).styles[maps as usize] as libc::c_int != 255 as libc::c_int
        {
            style = &mut *(r_newrefdef.lightstyles)
                .offset(*((*surf).styles).as_mut_ptr().offset(maps as isize) as isize)
                as *mut lightstyle_t;
            maps += 1;
        }
    } else {
        nummaps = 0 as libc::c_int;
        while nummaps < 4 as libc::c_int
            && (*surf).styles[nummaps as usize] as libc::c_int != 255 as libc::c_int
        {
            nummaps += 1;
        }
        lightmap = (*surf).samples;
        if nummaps == 1 as libc::c_int {
            let mut maps_0: libc::c_int = 0;
            maps_0 = 0 as libc::c_int;
            while maps_0 < 4 as libc::c_int
                && (*surf).styles[maps_0 as usize] as libc::c_int != 255 as libc::c_int
            {
                bl = s_blocklights.as_mut_ptr();
                i = 0 as libc::c_int;
                while i < 3 as libc::c_int {
                    scale[i
                        as usize] = (*gl_modulate).value
                        * (*(r_newrefdef.lightstyles)
                            .offset((*surf).styles[maps_0 as usize] as isize))
                            .rgb[i as usize];
                    i += 1;
                }
                if scale[0 as libc::c_int as usize] == 1.0f32
                    && scale[1 as libc::c_int as usize] == 1.0f32
                    && scale[2 as libc::c_int as usize] == 1.0f32
                {
                    i = 0 as libc::c_int;
                    while i < size {
                        *bl
                            .offset(
                                0 as libc::c_int as isize,
                            ) = *lightmap
                            .offset((i * 3 as libc::c_int + 0 as libc::c_int) as isize)
                            as libc::c_float;
                        *bl
                            .offset(
                                1 as libc::c_int as isize,
                            ) = *lightmap
                            .offset((i * 3 as libc::c_int + 1 as libc::c_int) as isize)
                            as libc::c_float;
                        *bl
                            .offset(
                                2 as libc::c_int as isize,
                            ) = *lightmap
                            .offset((i * 3 as libc::c_int + 2 as libc::c_int) as isize)
                            as libc::c_float;
                        i += 1;
                        bl = bl.offset(3 as libc::c_int as isize);
                    }
                } else {
                    i = 0 as libc::c_int;
                    while i < size {
                        *bl
                            .offset(
                                0 as libc::c_int as isize,
                            ) = *lightmap
                            .offset((i * 3 as libc::c_int + 0 as libc::c_int) as isize)
                            as libc::c_int as libc::c_float
                            * scale[0 as libc::c_int as usize];
                        *bl
                            .offset(
                                1 as libc::c_int as isize,
                            ) = *lightmap
                            .offset((i * 3 as libc::c_int + 1 as libc::c_int) as isize)
                            as libc::c_int as libc::c_float
                            * scale[1 as libc::c_int as usize];
                        *bl
                            .offset(
                                2 as libc::c_int as isize,
                            ) = *lightmap
                            .offset((i * 3 as libc::c_int + 2 as libc::c_int) as isize)
                            as libc::c_int as libc::c_float
                            * scale[2 as libc::c_int as usize];
                        i += 1;
                        bl = bl.offset(3 as libc::c_int as isize);
                    }
                }
                lightmap = lightmap.offset((size * 3 as libc::c_int) as isize);
                maps_0 += 1;
            }
        } else {
            let mut maps_1: libc::c_int = 0;
            memset(
                s_blocklights.as_mut_ptr() as *mut libc::c_void,
                0 as libc::c_int,
                (::std::mem::size_of::<libc::c_float>() as libc::c_ulong)
                    .wrapping_mul(size as libc::c_ulong)
                    .wrapping_mul(3 as libc::c_int as libc::c_ulong),
            );
            maps_1 = 0 as libc::c_int;
            while maps_1 < 4 as libc::c_int
                && (*surf).styles[maps_1 as usize] as libc::c_int != 255 as libc::c_int
            {
                bl = s_blocklights.as_mut_ptr();
                i = 0 as libc::c_int;
                while i < 3 as libc::c_int {
                    scale[i
                        as usize] = (*gl_modulate).value
                        * (*(r_newrefdef.lightstyles)
                            .offset((*surf).styles[maps_1 as usize] as isize))
                            .rgb[i as usize];
                    i += 1;
                }
                if scale[0 as libc::c_int as usize] == 1.0f32
                    && scale[1 as libc::c_int as usize] == 1.0f32
                    && scale[2 as libc::c_int as usize] == 1.0f32
                {
                    i = 0 as libc::c_int;
                    while i < size {
                        *bl.offset(0 as libc::c_int as isize)
                            += *lightmap
                                .offset((i * 3 as libc::c_int + 0 as libc::c_int) as isize)
                                as libc::c_int as libc::c_float;
                        *bl.offset(1 as libc::c_int as isize)
                            += *lightmap
                                .offset((i * 3 as libc::c_int + 1 as libc::c_int) as isize)
                                as libc::c_int as libc::c_float;
                        *bl.offset(2 as libc::c_int as isize)
                            += *lightmap
                                .offset((i * 3 as libc::c_int + 2 as libc::c_int) as isize)
                                as libc::c_int as libc::c_float;
                        i += 1;
                        bl = bl.offset(3 as libc::c_int as isize);
                    }
                } else {
                    i = 0 as libc::c_int;
                    while i < size {
                        *bl.offset(0 as libc::c_int as isize)
                            += *lightmap
                                .offset((i * 3 as libc::c_int + 0 as libc::c_int) as isize)
                                as libc::c_int as libc::c_float
                                * scale[0 as libc::c_int as usize];
                        *bl.offset(1 as libc::c_int as isize)
                            += *lightmap
                                .offset((i * 3 as libc::c_int + 1 as libc::c_int) as isize)
                                as libc::c_int as libc::c_float
                                * scale[1 as libc::c_int as usize];
                        *bl.offset(2 as libc::c_int as isize)
                            += *lightmap
                                .offset((i * 3 as libc::c_int + 2 as libc::c_int) as isize)
                                as libc::c_int as libc::c_float
                                * scale[2 as libc::c_int as usize];
                        i += 1;
                        bl = bl.offset(3 as libc::c_int as isize);
                    }
                }
                lightmap = lightmap.offset((size * 3 as libc::c_int) as isize);
                maps_1 += 1;
            }
        }
        if (*surf).dlightframe == r_framecount {
            R_AddDynamicLights(surf);
        }
    }
    stride -= smax << 2 as libc::c_int;
    bl = s_blocklights.as_mut_ptr();
    monolightmap = *((*gl_monolightmap).string).offset(0 as libc::c_int as isize)
        as libc::c_int;
    if monolightmap == '0' as i32 {
        i = 0 as libc::c_int;
        while i < tmax {
            j = 0 as libc::c_int;
            while j < smax {
                r = *bl.offset(0 as libc::c_int as isize) as libc::c_long as libc::c_int;
                g = *bl.offset(1 as libc::c_int as isize) as libc::c_long as libc::c_int;
                b = *bl.offset(2 as libc::c_int as isize) as libc::c_long as libc::c_int;
                if r < 0 as libc::c_int {
                    r = 0 as libc::c_int;
                }
                if g < 0 as libc::c_int {
                    g = 0 as libc::c_int;
                }
                if b < 0 as libc::c_int {
                    b = 0 as libc::c_int;
                }
                if r > g {
                    max = r;
                } else {
                    max = g;
                }
                if b > max {
                    max = b;
                }
                a = max;
                if max > 255 as libc::c_int {
                    let mut t: libc::c_float = 255.0f32 / max as libc::c_float;
                    r = (r as libc::c_float * t) as libc::c_int;
                    g = (g as libc::c_float * t) as libc::c_int;
                    b = (b as libc::c_float * t) as libc::c_int;
                    a = (a as libc::c_float * t) as libc::c_int;
                }
                *dest.offset(0 as libc::c_int as isize) = r as byte;
                *dest.offset(1 as libc::c_int as isize) = g as byte;
                *dest.offset(2 as libc::c_int as isize) = b as byte;
                *dest.offset(3 as libc::c_int as isize) = a as byte;
                bl = bl.offset(3 as libc::c_int as isize);
                dest = dest.offset(4 as libc::c_int as isize);
                j += 1;
            }
            i += 1;
            dest = dest.offset(stride as isize);
        }
    } else {
        i = 0 as libc::c_int;
        while i < tmax {
            j = 0 as libc::c_int;
            while j < smax {
                r = *bl.offset(0 as libc::c_int as isize) as libc::c_long as libc::c_int;
                g = *bl.offset(1 as libc::c_int as isize) as libc::c_long as libc::c_int;
                b = *bl.offset(2 as libc::c_int as isize) as libc::c_long as libc::c_int;
                if r < 0 as libc::c_int {
                    r = 0 as libc::c_int;
                }
                if g < 0 as libc::c_int {
                    g = 0 as libc::c_int;
                }
                if b < 0 as libc::c_int {
                    b = 0 as libc::c_int;
                }
                if r > g {
                    max = r;
                } else {
                    max = g;
                }
                if b > max {
                    max = b;
                }
                a = max;
                if max > 255 as libc::c_int {
                    let mut t_0: libc::c_float = 255.0f32 / max as libc::c_float;
                    r = (r as libc::c_float * t_0) as libc::c_int;
                    g = (g as libc::c_float * t_0) as libc::c_int;
                    b = (b as libc::c_float * t_0) as libc::c_int;
                    a = (a as libc::c_float * t_0) as libc::c_int;
                }
                match monolightmap {
                    76 | 73 => {
                        r = a;
                        b = 0 as libc::c_int;
                        g = b;
                    }
                    67 => {
                        a = 255 as libc::c_int - (r + g + b) / 3 as libc::c_int;
                        r = (r as libc::c_double * (a as libc::c_double / 255.0f64))
                            as libc::c_int;
                        g = (g as libc::c_double * (a as libc::c_double / 255.0f64))
                            as libc::c_int;
                        b = (b as libc::c_double * (a as libc::c_double / 255.0f64))
                            as libc::c_int;
                    }
                    65 | _ => {
                        b = 0 as libc::c_int;
                        g = b;
                        r = g;
                        a = 255 as libc::c_int - a;
                    }
                }
                *dest.offset(0 as libc::c_int as isize) = r as byte;
                *dest.offset(1 as libc::c_int as isize) = g as byte;
                *dest.offset(2 as libc::c_int as isize) = b as byte;
                *dest.offset(3 as libc::c_int as isize) = a as byte;
                bl = bl.offset(3 as libc::c_int as isize);
                dest = dest.offset(4 as libc::c_int as isize);
                j += 1;
            }
            i += 1;
            dest = dest.offset(stride as isize);
        }
    };
}
