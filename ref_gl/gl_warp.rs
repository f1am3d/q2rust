#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn floor(_: libc::c_double) -> libc::c_double;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    static mut vec3_origin: vec3_t;
    fn VectorScale(in_0: *mut vec_t, scale: vec_t, out: *mut vec_t);
    fn GL_FindImage(name: *mut libc::c_char, type_0: imagetype_t) -> *mut image_t;
    static mut ri: refimport_t;
    static mut qglEnd: Option::<unsafe extern "C" fn() -> ()>;
    static mut qglPopMatrix: Option::<unsafe extern "C" fn() -> ()>;
    static mut qglPushMatrix: Option::<unsafe extern "C" fn() -> ()>;
    static mut qglRotatef: Option::<
        unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int, libc::c_int) -> (),
    >;
    static mut qglTexCoord2f: Option::<
        unsafe extern "C" fn(libc::c_int, libc::c_int) -> (),
    >;
    static mut qglTranslatef: Option::<
        unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int) -> (),
    >;
    static mut qglVertex3fv: Option::<unsafe extern "C" fn(*const libc::c_int) -> ()>;
    static mut qglColorTableEXT: Option::<
        unsafe extern "C" fn(
            libc::c_int,
            libc::c_int,
            libc::c_int,
            libc::c_int,
            libc::c_int,
            *const libc::c_void,
        ) -> (),
    >;
    static mut r_notexture: *mut image_t;
    static mut r_origin: vec3_t;
    static mut r_newrefdef: refdef_t;
    static mut gl_ext_palettedtexture: *mut cvar_t;
    static mut gl_picmip: *mut cvar_t;
    static mut gl_skymip: *mut cvar_t;
    fn GL_Bind(texnum: libc::c_int);
    fn Com_sprintf(
        dest: *mut libc::c_char,
        size: libc::c_int,
        fmt: *mut libc::c_char,
        _: ...
    );
    fn Hunk_Alloc(size: libc::c_int) -> *mut libc::c_void;
    static mut loadmodel: *mut model_t;
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
pub static mut skyname: [libc::c_char; 64] = [0; 64];
#[no_mangle]
pub static mut skyrotate: libc::c_float = 0.;
#[no_mangle]
pub static mut skyaxis: vec3_t = [0.; 3];
#[no_mangle]
pub static mut sky_images: [*mut image_t; 6] = [0 as *const image_t as *mut image_t; 6];
#[no_mangle]
pub static mut warpface: *mut msurface_t = 0 as *const msurface_t as *mut msurface_t;
#[no_mangle]
pub unsafe extern "C" fn BoundPoly(
    mut numverts: libc::c_int,
    mut verts: *mut libc::c_float,
    mut mins: *mut vec_t,
    mut maxs: *mut vec_t,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut v: *mut libc::c_float = 0 as *mut libc::c_float;
    let ref mut fresh0 = *mins.offset(2 as libc::c_int as isize);
    *fresh0 = 9999 as libc::c_int as vec_t;
    let ref mut fresh1 = *mins.offset(1 as libc::c_int as isize);
    *fresh1 = *fresh0;
    *mins.offset(0 as libc::c_int as isize) = *fresh1;
    let ref mut fresh2 = *maxs.offset(2 as libc::c_int as isize);
    *fresh2 = -(9999 as libc::c_int) as vec_t;
    let ref mut fresh3 = *maxs.offset(1 as libc::c_int as isize);
    *fresh3 = *fresh2;
    *maxs.offset(0 as libc::c_int as isize) = *fresh3;
    v = verts;
    i = 0 as libc::c_int;
    while i < numverts {
        j = 0 as libc::c_int;
        while j < 3 as libc::c_int {
            if *v < *mins.offset(j as isize) {
                *mins.offset(j as isize) = *v;
            }
            if *v > *maxs.offset(j as isize) {
                *maxs.offset(j as isize) = *v;
            }
            j += 1;
            v = v.offset(1);
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn SubdividePolygon(
    mut numverts: libc::c_int,
    mut verts: *mut libc::c_float,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut mins: vec3_t = [0.; 3];
    let mut maxs: vec3_t = [0.; 3];
    let mut m: libc::c_float = 0.;
    let mut v: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut front: [vec3_t; 64] = [[0.; 3]; 64];
    let mut back: [vec3_t; 64] = [[0.; 3]; 64];
    let mut f: libc::c_int = 0;
    let mut b: libc::c_int = 0;
    let mut dist: [libc::c_float; 64] = [0.; 64];
    let mut frac: libc::c_float = 0.;
    let mut poly: *mut glpoly_t = 0 as *mut glpoly_t;
    let mut s: libc::c_float = 0.;
    let mut t: libc::c_float = 0.;
    let mut total: vec3_t = [0.; 3];
    let mut total_s: libc::c_float = 0.;
    let mut total_t: libc::c_float = 0.;
    if numverts > 60 as libc::c_int {
        (ri.Sys_Error)
            .expect(
                "non-null function pointer",
            )(
            1 as libc::c_int,
            b"numverts = %i\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            numverts,
        );
    }
    BoundPoly(numverts, verts, mins.as_mut_ptr(), maxs.as_mut_ptr());
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        m = ((mins[i as usize] + maxs[i as usize]) as libc::c_double * 0.5f64)
            as libc::c_float;
        m = (64 as libc::c_int as libc::c_double
            * floor((m / 64 as libc::c_int as libc::c_float) as libc::c_double + 0.5f64))
            as libc::c_float;
        if !(maxs[i as usize] - m < 8 as libc::c_int as libc::c_float) {
            if !(m - mins[i as usize] < 8 as libc::c_int as libc::c_float) {
                v = verts.offset(i as isize);
                j = 0 as libc::c_int;
                while j < numverts {
                    dist[j as usize] = *v - m;
                    j += 1;
                    v = v.offset(3 as libc::c_int as isize);
                }
                dist[j as usize] = dist[0 as libc::c_int as usize];
                v = v.offset(-(i as isize));
                *v
                    .offset(
                        0 as libc::c_int as isize,
                    ) = *verts.offset(0 as libc::c_int as isize);
                *v
                    .offset(
                        1 as libc::c_int as isize,
                    ) = *verts.offset(1 as libc::c_int as isize);
                *v
                    .offset(
                        2 as libc::c_int as isize,
                    ) = *verts.offset(2 as libc::c_int as isize);
                b = 0 as libc::c_int;
                f = b;
                v = verts;
                j = 0 as libc::c_int;
                while j < numverts {
                    if dist[j as usize] >= 0 as libc::c_int as libc::c_float {
                        front[f
                            as usize][0 as libc::c_int
                            as usize] = *v.offset(0 as libc::c_int as isize);
                        front[f
                            as usize][1 as libc::c_int
                            as usize] = *v.offset(1 as libc::c_int as isize);
                        front[f
                            as usize][2 as libc::c_int
                            as usize] = *v.offset(2 as libc::c_int as isize);
                        f += 1;
                    }
                    if dist[j as usize] <= 0 as libc::c_int as libc::c_float {
                        back[b
                            as usize][0 as libc::c_int
                            as usize] = *v.offset(0 as libc::c_int as isize);
                        back[b
                            as usize][1 as libc::c_int
                            as usize] = *v.offset(1 as libc::c_int as isize);
                        back[b
                            as usize][2 as libc::c_int
                            as usize] = *v.offset(2 as libc::c_int as isize);
                        b += 1;
                    }
                    if !(dist[j as usize] == 0 as libc::c_int as libc::c_float
                        || dist[(j + 1 as libc::c_int) as usize]
                            == 0 as libc::c_int as libc::c_float)
                    {
                        if (dist[j as usize] > 0 as libc::c_int as libc::c_float)
                            as libc::c_int
                            != (dist[(j + 1 as libc::c_int) as usize]
                                > 0 as libc::c_int as libc::c_float) as libc::c_int
                        {
                            frac = dist[j as usize]
                                / (dist[j as usize]
                                    - dist[(j + 1 as libc::c_int) as usize]);
                            k = 0 as libc::c_int;
                            while k < 3 as libc::c_int {
                                back[b
                                    as usize][k
                                    as usize] = *v.offset(k as isize)
                                    + frac
                                        * (*v.offset((3 as libc::c_int + k) as isize)
                                            - *v.offset(k as isize));
                                front[f
                                    as usize][k as usize] = back[b as usize][k as usize];
                                k += 1;
                            }
                            f += 1;
                            b += 1;
                        }
                    }
                    j += 1;
                    v = v.offset(3 as libc::c_int as isize);
                }
                SubdividePolygon(f, (front[0 as libc::c_int as usize]).as_mut_ptr());
                SubdividePolygon(b, (back[0 as libc::c_int as usize]).as_mut_ptr());
                return;
            }
        }
        i += 1;
    }
    poly = Hunk_Alloc(
        (::std::mem::size_of::<glpoly_t>() as libc::c_ulong)
            .wrapping_add(
                (((numverts - 4 as libc::c_int + 2 as libc::c_int) * 7 as libc::c_int)
                    as libc::c_ulong)
                    .wrapping_mul(
                        ::std::mem::size_of::<libc::c_float>() as libc::c_ulong,
                    ),
            ) as libc::c_int,
    ) as *mut glpoly_t;
    let ref mut fresh4 = (*poly).next;
    *fresh4 = (*warpface).polys;
    let ref mut fresh5 = (*warpface).polys;
    *fresh5 = poly;
    (*poly).numverts = numverts + 2 as libc::c_int;
    total[2 as libc::c_int as usize] = 0 as libc::c_int as vec_t;
    total[1 as libc::c_int as usize] = total[2 as libc::c_int as usize];
    total[0 as libc::c_int as usize] = total[1 as libc::c_int as usize];
    total_s = 0 as libc::c_int as libc::c_float;
    total_t = 0 as libc::c_int as libc::c_float;
    i = 0 as libc::c_int;
    while i < numverts {
        (*poly)
            .verts[(i + 1 as libc::c_int)
            as usize][0 as libc::c_int
            as usize] = *verts.offset(0 as libc::c_int as isize);
        (*poly)
            .verts[(i + 1 as libc::c_int)
            as usize][1 as libc::c_int
            as usize] = *verts.offset(1 as libc::c_int as isize);
        (*poly)
            .verts[(i + 1 as libc::c_int)
            as usize][2 as libc::c_int
            as usize] = *verts.offset(2 as libc::c_int as isize);
        s = *verts.offset(0 as libc::c_int as isize)
            * (*(*warpface).texinfo)
                .vecs[0 as libc::c_int as usize][0 as libc::c_int as usize]
            + *verts.offset(1 as libc::c_int as isize)
                * (*(*warpface).texinfo)
                    .vecs[0 as libc::c_int as usize][1 as libc::c_int as usize]
            + *verts.offset(2 as libc::c_int as isize)
                * (*(*warpface).texinfo)
                    .vecs[0 as libc::c_int as usize][2 as libc::c_int as usize];
        t = *verts.offset(0 as libc::c_int as isize)
            * (*(*warpface).texinfo)
                .vecs[1 as libc::c_int as usize][0 as libc::c_int as usize]
            + *verts.offset(1 as libc::c_int as isize)
                * (*(*warpface).texinfo)
                    .vecs[1 as libc::c_int as usize][1 as libc::c_int as usize]
            + *verts.offset(2 as libc::c_int as isize)
                * (*(*warpface).texinfo)
                    .vecs[1 as libc::c_int as usize][2 as libc::c_int as usize];
        total_s += s;
        total_t += t;
        total[0 as libc::c_int
            as usize] = total[0 as libc::c_int as usize]
            + *verts.offset(0 as libc::c_int as isize);
        total[1 as libc::c_int
            as usize] = total[1 as libc::c_int as usize]
            + *verts.offset(1 as libc::c_int as isize);
        total[2 as libc::c_int
            as usize] = total[2 as libc::c_int as usize]
            + *verts.offset(2 as libc::c_int as isize);
        (*poly).verts[(i + 1 as libc::c_int) as usize][3 as libc::c_int as usize] = s;
        (*poly).verts[(i + 1 as libc::c_int) as usize][4 as libc::c_int as usize] = t;
        i += 1;
        verts = verts.offset(3 as libc::c_int as isize);
    }
    VectorScale(
        total.as_mut_ptr(),
        (1.0f64 / numverts as libc::c_double) as vec_t,
        ((*poly).verts[0 as libc::c_int as usize]).as_mut_ptr(),
    );
    (*poly)
        .verts[0 as libc::c_int
        as usize][3 as libc::c_int as usize] = total_s / numverts as libc::c_float;
    (*poly)
        .verts[0 as libc::c_int
        as usize][4 as libc::c_int as usize] = total_t / numverts as libc::c_float;
    memcpy(
        ((*poly).verts[(i + 1 as libc::c_int) as usize]).as_mut_ptr()
            as *mut libc::c_void,
        ((*poly).verts[1 as libc::c_int as usize]).as_mut_ptr() as *const libc::c_void,
        ::std::mem::size_of::<[libc::c_float; 7]>() as libc::c_ulong,
    );
}
#[no_mangle]
pub unsafe extern "C" fn GL_SubdivideSurface(mut fa: *mut msurface_t) {
    let mut verts: [vec3_t; 64] = [[0.; 3]; 64];
    let mut numverts: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut lindex: libc::c_int = 0;
    let mut vec: *mut libc::c_float = 0 as *mut libc::c_float;
    warpface = fa;
    numverts = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < (*fa).numedges {
        lindex = *((*loadmodel).surfedges).offset(((*fa).firstedge + i) as isize);
        if lindex > 0 as libc::c_int {
            vec = ((*((*loadmodel).vertexes)
                .offset(
                    (*((*loadmodel).edges).offset(lindex as isize))
                        .v[0 as libc::c_int as usize] as isize,
                ))
                .position)
                .as_mut_ptr();
        } else {
            vec = ((*((*loadmodel).vertexes)
                .offset(
                    (*((*loadmodel).edges).offset(-lindex as isize))
                        .v[1 as libc::c_int as usize] as isize,
                ))
                .position)
                .as_mut_ptr();
        }
        verts[numverts
            as usize][0 as libc::c_int
            as usize] = *vec.offset(0 as libc::c_int as isize);
        verts[numverts
            as usize][1 as libc::c_int
            as usize] = *vec.offset(1 as libc::c_int as isize);
        verts[numverts
            as usize][2 as libc::c_int
            as usize] = *vec.offset(2 as libc::c_int as isize);
        numverts += 1;
        i += 1;
    }
    SubdividePolygon(numverts, (verts[0 as libc::c_int as usize]).as_mut_ptr());
}
#[no_mangle]
pub static mut r_turbsin: [libc::c_float; 256] = [
    0 as libc::c_int as libc::c_float,
    0.19633f64 as libc::c_float,
    0.392541f64 as libc::c_float,
    0.588517f64 as libc::c_float,
    0.784137f64 as libc::c_float,
    0.979285f64 as libc::c_float,
    1.17384f64 as libc::c_float,
    1.3677f64 as libc::c_float,
    1.56072f64 as libc::c_float,
    1.75281f64 as libc::c_float,
    1.94384f64 as libc::c_float,
    2.1337f64 as libc::c_float,
    2.32228f64 as libc::c_float,
    2.50945f64 as libc::c_float,
    2.69512f64 as libc::c_float,
    2.87916f64 as libc::c_float,
    3.06147f64 as libc::c_float,
    3.24193f64 as libc::c_float,
    3.42044f64 as libc::c_float,
    3.59689f64 as libc::c_float,
    3.77117f64 as libc::c_float,
    3.94319f64 as libc::c_float,
    4.11282f64 as libc::c_float,
    4.27998f64 as libc::c_float,
    4.44456f64 as libc::c_float,
    4.60647f64 as libc::c_float,
    4.76559f64 as libc::c_float,
    4.92185f64 as libc::c_float,
    5.07515f64 as libc::c_float,
    5.22538f64 as libc::c_float,
    5.37247f64 as libc::c_float,
    5.51632f64 as libc::c_float,
    5.65685f64 as libc::c_float,
    5.79398f64 as libc::c_float,
    5.92761f64 as libc::c_float,
    6.05767f64 as libc::c_float,
    6.18408f64 as libc::c_float,
    6.30677f64 as libc::c_float,
    6.42566f64 as libc::c_float,
    6.54068f64 as libc::c_float,
    6.65176f64 as libc::c_float,
    6.75883f64 as libc::c_float,
    6.86183f64 as libc::c_float,
    6.9607f64 as libc::c_float,
    7.05537f64 as libc::c_float,
    7.14579f64 as libc::c_float,
    7.23191f64 as libc::c_float,
    7.31368f64 as libc::c_float,
    7.39104f64 as libc::c_float,
    7.46394f64 as libc::c_float,
    7.53235f64 as libc::c_float,
    7.59623f64 as libc::c_float,
    7.65552f64 as libc::c_float,
    7.71021f64 as libc::c_float,
    7.76025f64 as libc::c_float,
    7.80562f64 as libc::c_float,
    7.84628f64 as libc::c_float,
    7.88222f64 as libc::c_float,
    7.91341f64 as libc::c_float,
    7.93984f64 as libc::c_float,
    7.96148f64 as libc::c_float,
    7.97832f64 as libc::c_float,
    7.99036f64 as libc::c_float,
    7.99759f64 as libc::c_float,
    8 as libc::c_int as libc::c_float,
    7.99759f64 as libc::c_float,
    7.99036f64 as libc::c_float,
    7.97832f64 as libc::c_float,
    7.96148f64 as libc::c_float,
    7.93984f64 as libc::c_float,
    7.91341f64 as libc::c_float,
    7.88222f64 as libc::c_float,
    7.84628f64 as libc::c_float,
    7.80562f64 as libc::c_float,
    7.76025f64 as libc::c_float,
    7.71021f64 as libc::c_float,
    7.65552f64 as libc::c_float,
    7.59623f64 as libc::c_float,
    7.53235f64 as libc::c_float,
    7.46394f64 as libc::c_float,
    7.39104f64 as libc::c_float,
    7.31368f64 as libc::c_float,
    7.23191f64 as libc::c_float,
    7.14579f64 as libc::c_float,
    7.05537f64 as libc::c_float,
    6.9607f64 as libc::c_float,
    6.86183f64 as libc::c_float,
    6.75883f64 as libc::c_float,
    6.65176f64 as libc::c_float,
    6.54068f64 as libc::c_float,
    6.42566f64 as libc::c_float,
    6.30677f64 as libc::c_float,
    6.18408f64 as libc::c_float,
    6.05767f64 as libc::c_float,
    5.92761f64 as libc::c_float,
    5.79398f64 as libc::c_float,
    5.65685f64 as libc::c_float,
    5.51632f64 as libc::c_float,
    5.37247f64 as libc::c_float,
    5.22538f64 as libc::c_float,
    5.07515f64 as libc::c_float,
    4.92185f64 as libc::c_float,
    4.76559f64 as libc::c_float,
    4.60647f64 as libc::c_float,
    4.44456f64 as libc::c_float,
    4.27998f64 as libc::c_float,
    4.11282f64 as libc::c_float,
    3.94319f64 as libc::c_float,
    3.77117f64 as libc::c_float,
    3.59689f64 as libc::c_float,
    3.42044f64 as libc::c_float,
    3.24193f64 as libc::c_float,
    3.06147f64 as libc::c_float,
    2.87916f64 as libc::c_float,
    2.69512f64 as libc::c_float,
    2.50945f64 as libc::c_float,
    2.32228f64 as libc::c_float,
    2.1337f64 as libc::c_float,
    1.94384f64 as libc::c_float,
    1.75281f64 as libc::c_float,
    1.56072f64 as libc::c_float,
    1.3677f64 as libc::c_float,
    1.17384f64 as libc::c_float,
    0.979285f64 as libc::c_float,
    0.784137f64 as libc::c_float,
    0.588517f64 as libc::c_float,
    0.392541f64 as libc::c_float,
    0.19633f64 as libc::c_float,
    9.79717e-16f64 as libc::c_float,
    -0.19633f64 as libc::c_float,
    -0.392541f64 as libc::c_float,
    -0.588517f64 as libc::c_float,
    -0.784137f64 as libc::c_float,
    -0.979285f64 as libc::c_float,
    -1.17384f64 as libc::c_float,
    -1.3677f64 as libc::c_float,
    -1.56072f64 as libc::c_float,
    -1.75281f64 as libc::c_float,
    -1.94384f64 as libc::c_float,
    -2.1337f64 as libc::c_float,
    -2.32228f64 as libc::c_float,
    -2.50945f64 as libc::c_float,
    -2.69512f64 as libc::c_float,
    -2.87916f64 as libc::c_float,
    -3.06147f64 as libc::c_float,
    -3.24193f64 as libc::c_float,
    -3.42044f64 as libc::c_float,
    -3.59689f64 as libc::c_float,
    -3.77117f64 as libc::c_float,
    -3.94319f64 as libc::c_float,
    -4.11282f64 as libc::c_float,
    -4.27998f64 as libc::c_float,
    -4.44456f64 as libc::c_float,
    -4.60647f64 as libc::c_float,
    -4.76559f64 as libc::c_float,
    -4.92185f64 as libc::c_float,
    -5.07515f64 as libc::c_float,
    -5.22538f64 as libc::c_float,
    -5.37247f64 as libc::c_float,
    -5.51632f64 as libc::c_float,
    -5.65685f64 as libc::c_float,
    -5.79398f64 as libc::c_float,
    -5.92761f64 as libc::c_float,
    -6.05767f64 as libc::c_float,
    -6.18408f64 as libc::c_float,
    -6.30677f64 as libc::c_float,
    -6.42566f64 as libc::c_float,
    -6.54068f64 as libc::c_float,
    -6.65176f64 as libc::c_float,
    -6.75883f64 as libc::c_float,
    -6.86183f64 as libc::c_float,
    -6.9607f64 as libc::c_float,
    -7.05537f64 as libc::c_float,
    -7.14579f64 as libc::c_float,
    -7.23191f64 as libc::c_float,
    -7.31368f64 as libc::c_float,
    -7.39104f64 as libc::c_float,
    -7.46394f64 as libc::c_float,
    -7.53235f64 as libc::c_float,
    -7.59623f64 as libc::c_float,
    -7.65552f64 as libc::c_float,
    -7.71021f64 as libc::c_float,
    -7.76025f64 as libc::c_float,
    -7.80562f64 as libc::c_float,
    -7.84628f64 as libc::c_float,
    -7.88222f64 as libc::c_float,
    -7.91341f64 as libc::c_float,
    -7.93984f64 as libc::c_float,
    -7.96148f64 as libc::c_float,
    -7.97832f64 as libc::c_float,
    -7.99036f64 as libc::c_float,
    -7.99759f64 as libc::c_float,
    -(8 as libc::c_int) as libc::c_float,
    -7.99759f64 as libc::c_float,
    -7.99036f64 as libc::c_float,
    -7.97832f64 as libc::c_float,
    -7.96148f64 as libc::c_float,
    -7.93984f64 as libc::c_float,
    -7.91341f64 as libc::c_float,
    -7.88222f64 as libc::c_float,
    -7.84628f64 as libc::c_float,
    -7.80562f64 as libc::c_float,
    -7.76025f64 as libc::c_float,
    -7.71021f64 as libc::c_float,
    -7.65552f64 as libc::c_float,
    -7.59623f64 as libc::c_float,
    -7.53235f64 as libc::c_float,
    -7.46394f64 as libc::c_float,
    -7.39104f64 as libc::c_float,
    -7.31368f64 as libc::c_float,
    -7.23191f64 as libc::c_float,
    -7.14579f64 as libc::c_float,
    -7.05537f64 as libc::c_float,
    -6.9607f64 as libc::c_float,
    -6.86183f64 as libc::c_float,
    -6.75883f64 as libc::c_float,
    -6.65176f64 as libc::c_float,
    -6.54068f64 as libc::c_float,
    -6.42566f64 as libc::c_float,
    -6.30677f64 as libc::c_float,
    -6.18408f64 as libc::c_float,
    -6.05767f64 as libc::c_float,
    -5.92761f64 as libc::c_float,
    -5.79398f64 as libc::c_float,
    -5.65685f64 as libc::c_float,
    -5.51632f64 as libc::c_float,
    -5.37247f64 as libc::c_float,
    -5.22538f64 as libc::c_float,
    -5.07515f64 as libc::c_float,
    -4.92185f64 as libc::c_float,
    -4.76559f64 as libc::c_float,
    -4.60647f64 as libc::c_float,
    -4.44456f64 as libc::c_float,
    -4.27998f64 as libc::c_float,
    -4.11282f64 as libc::c_float,
    -3.94319f64 as libc::c_float,
    -3.77117f64 as libc::c_float,
    -3.59689f64 as libc::c_float,
    -3.42044f64 as libc::c_float,
    -3.24193f64 as libc::c_float,
    -3.06147f64 as libc::c_float,
    -2.87916f64 as libc::c_float,
    -2.69512f64 as libc::c_float,
    -2.50945f64 as libc::c_float,
    -2.32228f64 as libc::c_float,
    -2.1337f64 as libc::c_float,
    -1.94384f64 as libc::c_float,
    -1.75281f64 as libc::c_float,
    -1.56072f64 as libc::c_float,
    -1.3677f64 as libc::c_float,
    -1.17384f64 as libc::c_float,
    -0.979285f64 as libc::c_float,
    -0.784137f64 as libc::c_float,
    -0.588517f64 as libc::c_float,
    -0.392541f64 as libc::c_float,
    -0.19633f64 as libc::c_float,
];
#[no_mangle]
pub unsafe extern "C" fn EmitWaterPolys(mut fa: *mut msurface_t) {
    let mut p: *mut glpoly_t = 0 as *mut glpoly_t;
    let mut bp: *mut glpoly_t = 0 as *mut glpoly_t;
    let mut v: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut i: libc::c_int = 0;
    let mut s: libc::c_float = 0.;
    let mut t: libc::c_float = 0.;
    let mut os: libc::c_float = 0.;
    let mut ot: libc::c_float = 0.;
    let mut scroll: libc::c_float = 0.;
    let mut rdt: libc::c_float = r_newrefdef.time;
    if (*(*fa).texinfo).flags & 0x40 as libc::c_int != 0 {
        scroll = (-(64 as libc::c_int) as libc::c_double
            * (r_newrefdef.time as libc::c_double * 0.5f64
                - (r_newrefdef.time as libc::c_double * 0.5f64) as libc::c_int
                    as libc::c_double)) as libc::c_float;
    } else {
        scroll = 0 as libc::c_int as libc::c_float;
    }
    bp = (*fa).polys;
    while !bp.is_null() {
        p = bp;
        i = 0 as libc::c_int;
        v = ((*p).verts[0 as libc::c_int as usize]).as_mut_ptr();
        while i < (*p).numverts {
            os = *v.offset(3 as libc::c_int as isize);
            ot = *v.offset(4 as libc::c_int as isize);
            s = os
                + r_turbsin[(((ot as libc::c_double * 0.125f64
                    + r_newrefdef.time as libc::c_double)
                    * (256.0f64
                        / (2 as libc::c_int as libc::c_double
                            * 3.14159265358979323846f64))) as libc::c_int
                    & 255 as libc::c_int) as usize];
            s += scroll;
            s = (s as libc::c_double * (1.0f64 / 64 as libc::c_int as libc::c_double))
                as libc::c_float;
            t = ot
                + r_turbsin[(((os as libc::c_double * 0.125f64 + rdt as libc::c_double)
                    * (256.0f64
                        / (2 as libc::c_int as libc::c_double
                            * 3.14159265358979323846f64))) as libc::c_int
                    & 255 as libc::c_int) as usize];
            t = (t as libc::c_double * (1.0f64 / 64 as libc::c_int as libc::c_double))
                as libc::c_float;
            qglTexCoord2f
                .expect("non-null function pointer")(s as libc::c_int, t as libc::c_int);
            qglVertex3fv.expect("non-null function pointer")(v as *const libc::c_int);
            i += 1;
            v = v.offset(7 as libc::c_int as isize);
        }
        qglEnd.expect("non-null function pointer")();
        bp = (*bp).next;
    }
}
#[no_mangle]
pub static mut skyclip: [vec3_t; 6] = [
    [1 as libc::c_int as vec_t, 1 as libc::c_int as vec_t, 0 as libc::c_int as vec_t],
    [1 as libc::c_int as vec_t, -(1 as libc::c_int) as vec_t, 0 as libc::c_int as vec_t],
    [0 as libc::c_int as vec_t, -(1 as libc::c_int) as vec_t, 1 as libc::c_int as vec_t],
    [0 as libc::c_int as vec_t, 1 as libc::c_int as vec_t, 1 as libc::c_int as vec_t],
    [1 as libc::c_int as vec_t, 0 as libc::c_int as vec_t, 1 as libc::c_int as vec_t],
    [-(1 as libc::c_int) as vec_t, 0 as libc::c_int as vec_t, 1 as libc::c_int as vec_t],
];
#[no_mangle]
pub static mut c_sky: libc::c_int = 0;
#[no_mangle]
pub static mut st_to_vec: [[libc::c_int; 3]; 6] = [
    [3 as libc::c_int, -(1 as libc::c_int), 2 as libc::c_int],
    [-(3 as libc::c_int), 1 as libc::c_int, 2 as libc::c_int],
    [1 as libc::c_int, 3 as libc::c_int, 2 as libc::c_int],
    [-(1 as libc::c_int), -(3 as libc::c_int), 2 as libc::c_int],
    [-(2 as libc::c_int), -(1 as libc::c_int), 3 as libc::c_int],
    [2 as libc::c_int, -(1 as libc::c_int), -(3 as libc::c_int)],
];
#[no_mangle]
pub static mut vec_to_st: [[libc::c_int; 3]; 6] = [
    [-(2 as libc::c_int), 3 as libc::c_int, 1 as libc::c_int],
    [2 as libc::c_int, 3 as libc::c_int, -(1 as libc::c_int)],
    [1 as libc::c_int, 3 as libc::c_int, 2 as libc::c_int],
    [-(1 as libc::c_int), 3 as libc::c_int, -(2 as libc::c_int)],
    [-(2 as libc::c_int), -(1 as libc::c_int), 3 as libc::c_int],
    [-(2 as libc::c_int), 1 as libc::c_int, -(3 as libc::c_int)],
];
#[no_mangle]
pub static mut skymins: [[libc::c_float; 6]; 2] = [[0.; 6]; 2];
#[no_mangle]
pub static mut skymaxs: [[libc::c_float; 6]; 2] = [[0.; 6]; 2];
#[no_mangle]
pub static mut sky_min: libc::c_float = 0.;
#[no_mangle]
pub static mut sky_max: libc::c_float = 0.;
#[no_mangle]
pub unsafe extern "C" fn DrawSkyPolygon(mut nump: libc::c_int, mut vecs: *mut vec_t) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut v: vec3_t = [0.; 3];
    let mut av: vec3_t = [0.; 3];
    let mut s: libc::c_float = 0.;
    let mut t: libc::c_float = 0.;
    let mut dv: libc::c_float = 0.;
    let mut axis: libc::c_int = 0;
    let mut vp: *mut libc::c_float = 0 as *mut libc::c_float;
    c_sky += 1;
    v[0 as libc::c_int as usize] = vec3_origin[0 as libc::c_int as usize];
    v[1 as libc::c_int as usize] = vec3_origin[1 as libc::c_int as usize];
    v[2 as libc::c_int as usize] = vec3_origin[2 as libc::c_int as usize];
    i = 0 as libc::c_int;
    vp = vecs;
    while i < nump {
        v[0 as libc::c_int
            as usize] = *vp.offset(0 as libc::c_int as isize)
            + v[0 as libc::c_int as usize];
        v[1 as libc::c_int
            as usize] = *vp.offset(1 as libc::c_int as isize)
            + v[1 as libc::c_int as usize];
        v[2 as libc::c_int
            as usize] = *vp.offset(2 as libc::c_int as isize)
            + v[2 as libc::c_int as usize];
        i += 1;
        vp = vp.offset(3 as libc::c_int as isize);
    }
    av[0 as libc::c_int
        as usize] = fabs(v[0 as libc::c_int as usize] as libc::c_double) as vec_t;
    av[1 as libc::c_int
        as usize] = fabs(v[1 as libc::c_int as usize] as libc::c_double) as vec_t;
    av[2 as libc::c_int
        as usize] = fabs(v[2 as libc::c_int as usize] as libc::c_double) as vec_t;
    if av[0 as libc::c_int as usize] > av[1 as libc::c_int as usize]
        && av[0 as libc::c_int as usize] > av[2 as libc::c_int as usize]
    {
        if v[0 as libc::c_int as usize] < 0 as libc::c_int as libc::c_float {
            axis = 1 as libc::c_int;
        } else {
            axis = 0 as libc::c_int;
        }
    } else if av[1 as libc::c_int as usize] > av[2 as libc::c_int as usize]
        && av[1 as libc::c_int as usize] > av[0 as libc::c_int as usize]
    {
        if v[1 as libc::c_int as usize] < 0 as libc::c_int as libc::c_float {
            axis = 3 as libc::c_int;
        } else {
            axis = 2 as libc::c_int;
        }
    } else if v[2 as libc::c_int as usize] < 0 as libc::c_int as libc::c_float {
        axis = 5 as libc::c_int;
    } else {
        axis = 4 as libc::c_int;
    }
    i = 0 as libc::c_int;
    while i < nump {
        j = vec_to_st[axis as usize][2 as libc::c_int as usize];
        if j > 0 as libc::c_int {
            dv = *vecs.offset((j - 1 as libc::c_int) as isize);
        } else {
            dv = -*vecs.offset((-j - 1 as libc::c_int) as isize);
        }
        if !((dv as libc::c_double) < 0.001f64) {
            j = vec_to_st[axis as usize][0 as libc::c_int as usize];
            if j < 0 as libc::c_int {
                s = -*vecs.offset((-j - 1 as libc::c_int) as isize) / dv;
            } else {
                s = *vecs.offset((j - 1 as libc::c_int) as isize) / dv;
            }
            j = vec_to_st[axis as usize][1 as libc::c_int as usize];
            if j < 0 as libc::c_int {
                t = -*vecs.offset((-j - 1 as libc::c_int) as isize) / dv;
            } else {
                t = *vecs.offset((j - 1 as libc::c_int) as isize) / dv;
            }
            if s < skymins[0 as libc::c_int as usize][axis as usize] {
                skymins[0 as libc::c_int as usize][axis as usize] = s;
            }
            if t < skymins[1 as libc::c_int as usize][axis as usize] {
                skymins[1 as libc::c_int as usize][axis as usize] = t;
            }
            if s > skymaxs[0 as libc::c_int as usize][axis as usize] {
                skymaxs[0 as libc::c_int as usize][axis as usize] = s;
            }
            if t > skymaxs[1 as libc::c_int as usize][axis as usize] {
                skymaxs[1 as libc::c_int as usize][axis as usize] = t;
            }
        }
        i += 1;
        vecs = vecs.offset(3 as libc::c_int as isize);
    }
}
#[no_mangle]
pub unsafe extern "C" fn ClipSkyPolygon(
    mut nump: libc::c_int,
    mut vecs: *mut vec_t,
    mut stage: libc::c_int,
) {
    let mut norm: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut v: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut front: qboolean = false_0;
    let mut back: qboolean = false_0;
    let mut d: libc::c_float = 0.;
    let mut e: libc::c_float = 0.;
    let mut dists: [libc::c_float; 64] = [0.; 64];
    let mut sides: [libc::c_int; 64] = [0; 64];
    let mut newv: [[vec3_t; 64]; 2] = [[[0.; 3]; 64]; 2];
    let mut newc: [libc::c_int; 2] = [0; 2];
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    if nump > 64 as libc::c_int - 2 as libc::c_int {
        (ri.Sys_Error)
            .expect(
                "non-null function pointer",
            )(
            1 as libc::c_int,
            b"ClipSkyPolygon: MAX_CLIP_VERTS\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    if stage == 6 as libc::c_int {
        DrawSkyPolygon(nump, vecs);
        return;
    }
    back = false_0;
    front = back;
    norm = (skyclip[stage as usize]).as_mut_ptr();
    i = 0 as libc::c_int;
    v = vecs;
    while i < nump {
        d = *v.offset(0 as libc::c_int as isize)
            * *norm.offset(0 as libc::c_int as isize)
            + *v.offset(1 as libc::c_int as isize)
                * *norm.offset(1 as libc::c_int as isize)
            + *v.offset(2 as libc::c_int as isize)
                * *norm.offset(2 as libc::c_int as isize);
        if d as libc::c_double > 0.1f64 {
            front = true_0;
            sides[i as usize] = 0 as libc::c_int;
        } else if (d as libc::c_double) < -0.1f64 {
            back = true_0;
            sides[i as usize] = 1 as libc::c_int;
        } else {
            sides[i as usize] = 2 as libc::c_int;
        }
        dists[i as usize] = d;
        i += 1;
        v = v.offset(3 as libc::c_int as isize);
    }
    if front as u64 == 0 || back as u64 == 0 {
        ClipSkyPolygon(nump, vecs, stage + 1 as libc::c_int);
        return;
    }
    sides[i as usize] = sides[0 as libc::c_int as usize];
    dists[i as usize] = dists[0 as libc::c_int as usize];
    *vecs
        .offset((i * 3 as libc::c_int) as isize)
        .offset(0 as libc::c_int as isize) = *vecs.offset(0 as libc::c_int as isize);
    *vecs
        .offset((i * 3 as libc::c_int) as isize)
        .offset(1 as libc::c_int as isize) = *vecs.offset(1 as libc::c_int as isize);
    *vecs
        .offset((i * 3 as libc::c_int) as isize)
        .offset(2 as libc::c_int as isize) = *vecs.offset(2 as libc::c_int as isize);
    newc[1 as libc::c_int as usize] = 0 as libc::c_int;
    newc[0 as libc::c_int as usize] = newc[1 as libc::c_int as usize];
    i = 0 as libc::c_int;
    v = vecs;
    while i < nump {
        match sides[i as usize] {
            0 => {
                newv[0 as libc::c_int
                    as usize][newc[0 as libc::c_int as usize]
                    as usize][0 as libc::c_int
                    as usize] = *v.offset(0 as libc::c_int as isize);
                newv[0 as libc::c_int
                    as usize][newc[0 as libc::c_int as usize]
                    as usize][1 as libc::c_int
                    as usize] = *v.offset(1 as libc::c_int as isize);
                newv[0 as libc::c_int
                    as usize][newc[0 as libc::c_int as usize]
                    as usize][2 as libc::c_int
                    as usize] = *v.offset(2 as libc::c_int as isize);
                newc[0 as libc::c_int as usize] += 1;
            }
            1 => {
                newv[1 as libc::c_int
                    as usize][newc[1 as libc::c_int as usize]
                    as usize][0 as libc::c_int
                    as usize] = *v.offset(0 as libc::c_int as isize);
                newv[1 as libc::c_int
                    as usize][newc[1 as libc::c_int as usize]
                    as usize][1 as libc::c_int
                    as usize] = *v.offset(1 as libc::c_int as isize);
                newv[1 as libc::c_int
                    as usize][newc[1 as libc::c_int as usize]
                    as usize][2 as libc::c_int
                    as usize] = *v.offset(2 as libc::c_int as isize);
                newc[1 as libc::c_int as usize] += 1;
            }
            2 => {
                newv[0 as libc::c_int
                    as usize][newc[0 as libc::c_int as usize]
                    as usize][0 as libc::c_int
                    as usize] = *v.offset(0 as libc::c_int as isize);
                newv[0 as libc::c_int
                    as usize][newc[0 as libc::c_int as usize]
                    as usize][1 as libc::c_int
                    as usize] = *v.offset(1 as libc::c_int as isize);
                newv[0 as libc::c_int
                    as usize][newc[0 as libc::c_int as usize]
                    as usize][2 as libc::c_int
                    as usize] = *v.offset(2 as libc::c_int as isize);
                newc[0 as libc::c_int as usize] += 1;
                newv[1 as libc::c_int
                    as usize][newc[1 as libc::c_int as usize]
                    as usize][0 as libc::c_int
                    as usize] = *v.offset(0 as libc::c_int as isize);
                newv[1 as libc::c_int
                    as usize][newc[1 as libc::c_int as usize]
                    as usize][1 as libc::c_int
                    as usize] = *v.offset(1 as libc::c_int as isize);
                newv[1 as libc::c_int
                    as usize][newc[1 as libc::c_int as usize]
                    as usize][2 as libc::c_int
                    as usize] = *v.offset(2 as libc::c_int as isize);
                newc[1 as libc::c_int as usize] += 1;
            }
            _ => {}
        }
        if !(sides[i as usize] == 2 as libc::c_int
            || sides[(i + 1 as libc::c_int) as usize] == 2 as libc::c_int
            || sides[(i + 1 as libc::c_int) as usize] == sides[i as usize])
        {
            d = dists[i as usize]
                / (dists[i as usize] - dists[(i + 1 as libc::c_int) as usize]);
            j = 0 as libc::c_int;
            while j < 3 as libc::c_int {
                e = *v.offset(j as isize)
                    + d
                        * (*v.offset((j + 3 as libc::c_int) as isize)
                            - *v.offset(j as isize));
                newv[0 as libc::c_int
                    as usize][newc[0 as libc::c_int as usize] as usize][j as usize] = e;
                newv[1 as libc::c_int
                    as usize][newc[1 as libc::c_int as usize] as usize][j as usize] = e;
                j += 1;
            }
            newc[0 as libc::c_int as usize] += 1;
            newc[1 as libc::c_int as usize] += 1;
        }
        i += 1;
        v = v.offset(3 as libc::c_int as isize);
    }
    ClipSkyPolygon(
        newc[0 as libc::c_int as usize],
        (newv[0 as libc::c_int as usize][0 as libc::c_int as usize]).as_mut_ptr(),
        stage + 1 as libc::c_int,
    );
    ClipSkyPolygon(
        newc[1 as libc::c_int as usize],
        (newv[1 as libc::c_int as usize][0 as libc::c_int as usize]).as_mut_ptr(),
        stage + 1 as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn R_AddSkySurface(mut fa: *mut msurface_t) {
    let mut i: libc::c_int = 0;
    let mut verts: [vec3_t; 64] = [[0.; 3]; 64];
    let mut p: *mut glpoly_t = 0 as *mut glpoly_t;
    p = (*fa).polys;
    while !p.is_null() {
        i = 0 as libc::c_int;
        while i < (*p).numverts {
            verts[i
                as usize][0 as libc::c_int
                as usize] = (*p).verts[i as usize][0 as libc::c_int as usize]
                - r_origin[0 as libc::c_int as usize];
            verts[i
                as usize][1 as libc::c_int
                as usize] = (*p).verts[i as usize][1 as libc::c_int as usize]
                - r_origin[1 as libc::c_int as usize];
            verts[i
                as usize][2 as libc::c_int
                as usize] = (*p).verts[i as usize][2 as libc::c_int as usize]
                - r_origin[2 as libc::c_int as usize];
            i += 1;
        }
        ClipSkyPolygon(
            (*p).numverts,
            (verts[0 as libc::c_int as usize]).as_mut_ptr(),
            0 as libc::c_int,
        );
        p = (*p).next;
    }
}
#[no_mangle]
pub unsafe extern "C" fn R_ClearSkyBox() {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 6 as libc::c_int {
        skymins[1 as libc::c_int
            as usize][i as usize] = 9999 as libc::c_int as libc::c_float;
        skymins[0 as libc::c_int
            as usize][i as usize] = skymins[1 as libc::c_int as usize][i as usize];
        skymaxs[1 as libc::c_int
            as usize][i as usize] = -(9999 as libc::c_int) as libc::c_float;
        skymaxs[0 as libc::c_int
            as usize][i as usize] = skymaxs[1 as libc::c_int as usize][i as usize];
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn MakeSkyVec(
    mut s: libc::c_float,
    mut t: libc::c_float,
    mut axis: libc::c_int,
) {
    let mut v: vec3_t = [0.; 3];
    let mut b: vec3_t = [0.; 3];
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    b[0 as libc::c_int as usize] = s * 2300 as libc::c_int as libc::c_float;
    b[1 as libc::c_int as usize] = t * 2300 as libc::c_int as libc::c_float;
    b[2 as libc::c_int as usize] = 2300 as libc::c_int as vec_t;
    j = 0 as libc::c_int;
    while j < 3 as libc::c_int {
        k = st_to_vec[axis as usize][j as usize];
        if k < 0 as libc::c_int {
            v[j as usize] = -b[(-k - 1 as libc::c_int) as usize];
        } else {
            v[j as usize] = b[(k - 1 as libc::c_int) as usize];
        }
        j += 1;
    }
    s = ((s + 1 as libc::c_int as libc::c_float) as libc::c_double * 0.5f64)
        as libc::c_float;
    t = ((t + 1 as libc::c_int as libc::c_float) as libc::c_double * 0.5f64)
        as libc::c_float;
    if s < sky_min {
        s = sky_min;
    } else if s > sky_max {
        s = sky_max;
    }
    if t < sky_min {
        t = sky_min;
    } else if t > sky_max {
        t = sky_max;
    }
    t = (1.0f64 - t as libc::c_double) as libc::c_float;
    qglTexCoord2f
        .expect("non-null function pointer")(s as libc::c_int, t as libc::c_int);
    qglVertex3fv
        .expect("non-null function pointer")(v.as_mut_ptr() as *const libc::c_int);
}
#[no_mangle]
pub static mut skytexorder: [libc::c_int; 6] = [
    0 as libc::c_int,
    2 as libc::c_int,
    1 as libc::c_int,
    3 as libc::c_int,
    4 as libc::c_int,
    5 as libc::c_int,
];
#[no_mangle]
pub unsafe extern "C" fn R_DrawSkyBox() {
    let mut i: libc::c_int = 0;
    if skyrotate != 0. {
        i = 0 as libc::c_int;
        while i < 6 as libc::c_int {
            if skymins[0 as libc::c_int as usize][i as usize]
                < skymaxs[0 as libc::c_int as usize][i as usize]
                && skymins[1 as libc::c_int as usize][i as usize]
                    < skymaxs[1 as libc::c_int as usize][i as usize]
            {
                break;
            }
            i += 1;
        }
        if i == 6 as libc::c_int {
            return;
        }
    }
    qglPushMatrix.expect("non-null function pointer")();
    qglTranslatef
        .expect(
            "non-null function pointer",
        )(
        r_origin[0 as libc::c_int as usize] as libc::c_int,
        r_origin[1 as libc::c_int as usize] as libc::c_int,
        r_origin[2 as libc::c_int as usize] as libc::c_int,
    );
    qglRotatef
        .expect(
            "non-null function pointer",
        )(
        (r_newrefdef.time * skyrotate) as libc::c_int,
        skyaxis[0 as libc::c_int as usize] as libc::c_int,
        skyaxis[1 as libc::c_int as usize] as libc::c_int,
        skyaxis[2 as libc::c_int as usize] as libc::c_int,
    );
    i = 0 as libc::c_int;
    while i < 6 as libc::c_int {
        if skyrotate != 0. {
            skymins[0 as libc::c_int
                as usize][i as usize] = -(1 as libc::c_int) as libc::c_float;
            skymins[1 as libc::c_int
                as usize][i as usize] = -(1 as libc::c_int) as libc::c_float;
            skymaxs[0 as libc::c_int
                as usize][i as usize] = 1 as libc::c_int as libc::c_float;
            skymaxs[1 as libc::c_int
                as usize][i as usize] = 1 as libc::c_int as libc::c_float;
        }
        if !(skymins[0 as libc::c_int as usize][i as usize]
            >= skymaxs[0 as libc::c_int as usize][i as usize]
            || skymins[1 as libc::c_int as usize][i as usize]
                >= skymaxs[1 as libc::c_int as usize][i as usize])
        {
            GL_Bind((*sky_images[skytexorder[i as usize] as usize]).texnum);
            MakeSkyVec(
                skymins[0 as libc::c_int as usize][i as usize],
                skymins[1 as libc::c_int as usize][i as usize],
                i,
            );
            MakeSkyVec(
                skymins[0 as libc::c_int as usize][i as usize],
                skymaxs[1 as libc::c_int as usize][i as usize],
                i,
            );
            MakeSkyVec(
                skymaxs[0 as libc::c_int as usize][i as usize],
                skymaxs[1 as libc::c_int as usize][i as usize],
                i,
            );
            MakeSkyVec(
                skymaxs[0 as libc::c_int as usize][i as usize],
                skymins[1 as libc::c_int as usize][i as usize],
                i,
            );
            qglEnd.expect("non-null function pointer")();
        }
        i += 1;
    }
    qglPopMatrix.expect("non-null function pointer")();
}
#[no_mangle]
pub static mut suf: [*mut libc::c_char; 6] = [
    b"rt\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"bk\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"lf\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"ft\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"up\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"dn\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
];
#[no_mangle]
pub unsafe extern "C" fn R_SetSky(
    mut name: *mut libc::c_char,
    mut rotate: libc::c_float,
    mut axis: *mut vec_t,
) {
    let mut i: libc::c_int = 0;
    let mut pathname: [libc::c_char; 64] = [0; 64];
    strncpy(
        skyname.as_mut_ptr(),
        name,
        (::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
    );
    skyrotate = rotate;
    skyaxis[0 as libc::c_int as usize] = *axis.offset(0 as libc::c_int as isize);
    skyaxis[1 as libc::c_int as usize] = *axis.offset(1 as libc::c_int as isize);
    skyaxis[2 as libc::c_int as usize] = *axis.offset(2 as libc::c_int as isize);
    i = 0 as libc::c_int;
    while i < 6 as libc::c_int {
        if (*gl_skymip).value != 0. || skyrotate != 0. {
            let ref mut fresh6 = (*gl_picmip).value;
            *fresh6 += 1.;
        }
        if qglColorTableEXT.is_some() && (*gl_ext_palettedtexture).value != 0. {
            Com_sprintf(
                pathname.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong
                    as libc::c_int,
                b"env/%s%s.pcx\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                skyname.as_mut_ptr(),
                suf[i as usize],
            );
        } else {
            Com_sprintf(
                pathname.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong
                    as libc::c_int,
                b"env/%s%s.tga\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                skyname.as_mut_ptr(),
                suf[i as usize],
            );
        }
        sky_images[i as usize] = GL_FindImage(pathname.as_mut_ptr(), it_sky);
        if (sky_images[i as usize]).is_null() {
            sky_images[i as usize] = r_notexture;
        }
        if (*gl_skymip).value != 0. || skyrotate != 0. {
            let ref mut fresh7 = (*gl_picmip).value;
            *fresh7 -= 1.;
            sky_min = (1.0f64 / 256 as libc::c_int as libc::c_double) as libc::c_float;
            sky_max = (255.0f64 / 256 as libc::c_int as libc::c_double) as libc::c_float;
        } else {
            sky_min = (1.0f64 / 512 as libc::c_int as libc::c_double) as libc::c_float;
            sky_max = (511.0f64 / 512 as libc::c_int as libc::c_double) as libc::c_float;
        }
        i += 1;
    }
}
