#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    static mut qglEnd: Option::<unsafe extern "C" fn() -> ()>;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn AngleVectors(
        angles: *mut vec_t,
        forward: *mut vec_t,
        right: *mut vec_t,
        up: *mut vec_t,
    );
    fn Hunk_Alloc(size: libc::c_int) -> *mut libc::c_void;
    static mut qglColor3f: Option::<
        unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int) -> (),
    >;
    static mut qglColor4f: Option::<
        unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int, libc::c_int) -> (),
    >;
    static mut qglDepthMask: Option::<unsafe extern "C" fn(libc::c_int) -> ()>;
    fn R_ClearSkyBox();
    fn R_AddSkySurface(fa: *mut msurface_t);
    fn R_DrawSkyBox();
    fn R_CullBox(mins: *mut vec_t, maxs: *mut vec_t) -> qboolean;
    fn R_RotateForEntity(e: *mut entity_t);
    fn R_MarkLights(light: *mut dlight_t, bit: libc::c_int, node: *mut mnode_t);
    fn EmitWaterPolys(fa: *mut msurface_t);
    static mut gl_state: glstate_t;
    static mut ri: refimport_t;
    static mut r_worldmodel: *mut model_t;
    fn GL_EnableMultitexture(enable: qboolean);
    fn GL_Bind(texnum: libc::c_int);
    static mut r_world_matrix: [libc::c_float; 16];
    static mut gl_tex_alpha_format: libc::c_int;
    static mut gl_tex_solid_format: libc::c_int;
    static mut gl_lockpvs: *mut cvar_t;
    static mut gl_saturatelighting: *mut cvar_t;
    static mut gl_flashblend: *mut cvar_t;
    static mut gl_showtris: *mut cvar_t;
    static mut gl_monolightmap: *mut cvar_t;
    static mut gl_dynamic: *mut cvar_t;
    static mut gl_lightmap: *mut cvar_t;
    static mut r_novis: *mut cvar_t;
    static mut r_fullbright: *mut cvar_t;
    static mut r_drawworld: *mut cvar_t;
    static mut r_oldviewcluster2: libc::c_int;
    static mut r_oldviewcluster: libc::c_int;
    static mut r_viewcluster2: libc::c_int;
    static mut r_viewcluster: libc::c_int;
    static mut r_newrefdef: refdef_t;
    static mut c_brush_polys: libc::c_int;
    static mut r_framecount: libc::c_int;
    static mut r_visframecount: libc::c_int;
    static mut currentmodel: *mut model_t;
    static mut currententity: *mut entity_t;
    static mut numgltextures: libc::c_int;
    static mut gltextures: [image_t; 1024];
    fn Mod_ClusterPVS(cluster: libc::c_int, model: *mut model_t) -> *mut byte;
    static mut qglLoadMatrixf: Option::<unsafe extern "C" fn(*const libc::c_int) -> ()>;
    static mut qglPushMatrix: Option::<unsafe extern "C" fn() -> ()>;
    static mut qglPopMatrix: Option::<unsafe extern "C" fn() -> ()>;
    static mut qglTexCoord2f: Option::<
        unsafe extern "C" fn(libc::c_int, libc::c_int) -> (),
    >;
    static mut qglVertex3fv: Option::<unsafe extern "C" fn(*const libc::c_int) -> ()>;
    fn R_SetCacheState(surf: *mut msurface_t);
    fn R_BuildLightMap(surf: *mut msurface_t, dest: *mut byte, stride: libc::c_int);
    fn toupper(_: libc::c_int) -> libc::c_int;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gllightmapstate_t {
    pub internal_format: libc::c_int,
    pub current_lightmap_texture: libc::c_int,
    pub lightmap_surfaces: [*mut msurface_t; 128],
    pub allocated: [libc::c_int; 128],
    pub lightmap_buffer: [byte; 65536],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glstate_t {
    pub inverse_intensity: libc::c_float,
    pub fullscreen: qboolean,
    pub prev_mode: libc::c_int,
    pub d_16to8table: *mut libc::c_uchar,
    pub lightmap_textures: libc::c_int,
    pub currenttextures: [libc::c_int; 2],
    pub currenttmu: libc::c_int,
    pub camera_separation: libc::c_float,
    pub stereo_enabled: qboolean,
    pub originalRedGammaTable: [libc::c_uchar; 256],
    pub originalGreenGammaTable: [libc::c_uchar; 256],
    pub originalBlueGammaTable: [libc::c_uchar; 256],
}
static mut modelorg: vec3_t = [0.; 3];
#[no_mangle]
pub static mut r_alpha_surfaces: *mut msurface_t = 0 as *const msurface_t
    as *mut msurface_t;
#[no_mangle]
pub static mut c_visible_lightmaps: libc::c_int = 0;
#[no_mangle]
pub static mut c_visible_textures: libc::c_int = 0;
static mut gl_lms: gllightmapstate_t = gllightmapstate_t {
    internal_format: 0,
    current_lightmap_texture: 0,
    lightmap_surfaces: [0 as *const msurface_t as *mut msurface_t; 128],
    allocated: [0; 128],
    lightmap_buffer: [0; 65536],
};
#[no_mangle]
pub unsafe extern "C" fn R_TextureAnimation(mut tex: *mut mtexinfo_t) -> *mut image_t {
    let mut c: libc::c_int = 0;
    if ((*tex).next).is_null() {
        return (*tex).image;
    }
    c = (*currententity).frame % (*tex).numframes;
    while c != 0 {
        tex = (*tex).next;
        c -= 1;
    }
    return (*tex).image;
}
#[no_mangle]
pub unsafe extern "C" fn DrawGLPoly(mut p: *mut glpoly_t) {
    let mut i: libc::c_int = 0;
    let mut v: *mut libc::c_float = 0 as *mut libc::c_float;
    v = ((*p).verts[0 as libc::c_int as usize]).as_mut_ptr();
    i = 0 as libc::c_int;
    while i < (*p).numverts {
        qglTexCoord2f
            .expect(
                "non-null function pointer",
            )(
            *v.offset(3 as libc::c_int as isize) as libc::c_int,
            *v.offset(4 as libc::c_int as isize) as libc::c_int,
        );
        qglVertex3fv.expect("non-null function pointer")(v as *const libc::c_int);
        i += 1;
        v = v.offset(7 as libc::c_int as isize);
    }
    qglEnd.expect("non-null function pointer")();
}
#[no_mangle]
pub unsafe extern "C" fn DrawGLFlowingPoly(mut fa: *mut msurface_t) {
    let mut i: libc::c_int = 0;
    let mut v: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut p: *mut glpoly_t = 0 as *mut glpoly_t;
    let mut scroll: libc::c_float = 0.;
    p = (*fa).polys;
    scroll = (-(64 as libc::c_int) as libc::c_double
        * (r_newrefdef.time as libc::c_double / 40.0f64
            - (r_newrefdef.time as libc::c_double / 40.0f64) as libc::c_int
                as libc::c_double)) as libc::c_float;
    if scroll as libc::c_double == 0.0f64 {
        scroll = -64.0f64 as libc::c_float;
    }
    v = ((*p).verts[0 as libc::c_int as usize]).as_mut_ptr();
    i = 0 as libc::c_int;
    while i < (*p).numverts {
        qglTexCoord2f
            .expect(
                "non-null function pointer",
            )(
            (*v.offset(3 as libc::c_int as isize) + scroll) as libc::c_int,
            *v.offset(4 as libc::c_int as isize) as libc::c_int,
        );
        qglVertex3fv.expect("non-null function pointer")(v as *const libc::c_int);
        i += 1;
        v = v.offset(7 as libc::c_int as isize);
    }
    qglEnd.expect("non-null function pointer")();
}
#[no_mangle]
pub unsafe extern "C" fn R_DrawTriangleOutlines() {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut p: *mut glpoly_t = 0 as *mut glpoly_t;
    if (*gl_showtris).value == 0. {
        return;
    }
    qglColor4f
        .expect(
            "non-null function pointer",
        )(1 as libc::c_int, 1 as libc::c_int, 1 as libc::c_int, 1 as libc::c_int);
    i = 0 as libc::c_int;
    while i < 128 as libc::c_int {
        let mut surf: *mut msurface_t = 0 as *mut msurface_t;
        surf = gl_lms.lightmap_surfaces[i as usize];
        while !surf.is_null() {
            p = (*surf).polys;
            while !p.is_null() {
                j = 2 as libc::c_int;
                while j < (*p).numverts {
                    qglVertex3fv
                        .expect(
                            "non-null function pointer",
                        )(
                        ((*p).verts[0 as libc::c_int as usize]).as_mut_ptr()
                            as *const libc::c_int,
                    );
                    qglVertex3fv
                        .expect(
                            "non-null function pointer",
                        )(
                        ((*p).verts[(j - 1 as libc::c_int) as usize]).as_mut_ptr()
                            as *const libc::c_int,
                    );
                    qglVertex3fv
                        .expect(
                            "non-null function pointer",
                        )(((*p).verts[j as usize]).as_mut_ptr() as *const libc::c_int);
                    qglVertex3fv
                        .expect(
                            "non-null function pointer",
                        )(
                        ((*p).verts[0 as libc::c_int as usize]).as_mut_ptr()
                            as *const libc::c_int,
                    );
                    qglEnd.expect("non-null function pointer")();
                    j += 1;
                }
                p = (*p).chain;
            }
            surf = (*surf).lightmapchain;
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn DrawGLPolyChain(
    mut p: *mut glpoly_t,
    mut soffset: libc::c_float,
    mut toffset: libc::c_float,
) {
    if soffset == 0 as libc::c_int as libc::c_float
        && toffset == 0 as libc::c_int as libc::c_float
    {
        while !p.is_null() {
            let mut v: *mut libc::c_float = 0 as *mut libc::c_float;
            let mut j: libc::c_int = 0;
            v = ((*p).verts[0 as libc::c_int as usize]).as_mut_ptr();
            j = 0 as libc::c_int;
            while j < (*p).numverts {
                qglTexCoord2f
                    .expect(
                        "non-null function pointer",
                    )(
                    *v.offset(5 as libc::c_int as isize) as libc::c_int,
                    *v.offset(6 as libc::c_int as isize) as libc::c_int,
                );
                qglVertex3fv
                    .expect("non-null function pointer")(v as *const libc::c_int);
                j += 1;
                v = v.offset(7 as libc::c_int as isize);
            }
            qglEnd.expect("non-null function pointer")();
            p = (*p).chain;
        }
    } else {
        while !p.is_null() {
            let mut v_0: *mut libc::c_float = 0 as *mut libc::c_float;
            let mut j_0: libc::c_int = 0;
            v_0 = ((*p).verts[0 as libc::c_int as usize]).as_mut_ptr();
            j_0 = 0 as libc::c_int;
            while j_0 < (*p).numverts {
                qglTexCoord2f
                    .expect(
                        "non-null function pointer",
                    )(
                    (*v_0.offset(5 as libc::c_int as isize) - soffset) as libc::c_int,
                    (*v_0.offset(6 as libc::c_int as isize) - toffset) as libc::c_int,
                );
                qglVertex3fv
                    .expect("non-null function pointer")(v_0 as *const libc::c_int);
                j_0 += 1;
                v_0 = v_0.offset(7 as libc::c_int as isize);
            }
            qglEnd.expect("non-null function pointer")();
            p = (*p).chain;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn R_BlendLightmaps() {
    let mut i: libc::c_int = 0;
    let mut surf: *mut msurface_t = 0 as *mut msurface_t;
    let mut newdrawsurf: *mut msurface_t = 0 as *mut msurface_t;
    if (*r_fullbright).value != 0. {
        return;
    }
    if ((*r_worldmodel).lightdata).is_null() {
        return;
    }
    qglDepthMask.expect("non-null function pointer")(0 as libc::c_int);
    if (*gl_lightmap).value == 0. {
        if !((*gl_saturatelighting).value != 0.) {
            if *((*gl_monolightmap).string).offset(0 as libc::c_int as isize)
                as libc::c_int != '0' as i32
            {
                match toupper(
                    *((*gl_monolightmap).string).offset(0 as libc::c_int as isize)
                        as libc::c_int,
                ) {
                    73 | 76 | 65 | _ => {}
                }
            }
        }
    }
    if currentmodel == r_worldmodel {
        c_visible_lightmaps = 0 as libc::c_int;
    }
    i = 1 as libc::c_int;
    while i < 128 as libc::c_int {
        if !(gl_lms.lightmap_surfaces[i as usize]).is_null() {
            if currentmodel == r_worldmodel {
                c_visible_lightmaps += 1;
            }
            GL_Bind(gl_state.lightmap_textures + i);
            surf = gl_lms.lightmap_surfaces[i as usize];
            while !surf.is_null() {
                if !((*surf).polys).is_null() {
                    DrawGLPolyChain(
                        (*surf).polys,
                        0 as libc::c_int as libc::c_float,
                        0 as libc::c_int as libc::c_float,
                    );
                }
                surf = (*surf).lightmapchain;
            }
        }
        i += 1;
    }
    if (*gl_dynamic).value != 0. {
        LM_InitBlock();
        GL_Bind(gl_state.lightmap_textures + 0 as libc::c_int);
        if currentmodel == r_worldmodel {
            c_visible_lightmaps += 1;
        }
        newdrawsurf = gl_lms.lightmap_surfaces[0 as libc::c_int as usize];
        surf = gl_lms.lightmap_surfaces[0 as libc::c_int as usize];
        while !surf.is_null() {
            let mut smax: libc::c_int = 0;
            let mut tmax: libc::c_int = 0;
            let mut base: *mut byte = 0 as *mut byte;
            smax = ((*surf).extents[0 as libc::c_int as usize] as libc::c_int
                >> 4 as libc::c_int) + 1 as libc::c_int;
            tmax = ((*surf).extents[1 as libc::c_int as usize] as libc::c_int
                >> 4 as libc::c_int) + 1 as libc::c_int;
            if LM_AllocBlock(smax, tmax, &mut (*surf).dlight_s, &mut (*surf).dlight_t)
                as u64 != 0
            {
                base = (gl_lms.lightmap_buffer).as_mut_ptr();
                base = base
                    .offset(
                        (((*surf).dlight_t * 128 as libc::c_int + (*surf).dlight_s)
                            * 4 as libc::c_int) as isize,
                    );
                R_BuildLightMap(surf, base, 128 as libc::c_int * 4 as libc::c_int);
            } else {
                let mut drawsurf: *mut msurface_t = 0 as *mut msurface_t;
                LM_UploadBlock(true_0);
                drawsurf = newdrawsurf;
                while drawsurf != surf {
                    if !((*drawsurf).polys).is_null() {
                        DrawGLPolyChain(
                            (*drawsurf).polys,
                            (((*drawsurf).light_s - (*drawsurf).dlight_s)
                                as libc::c_double * (1.0f64 / 128.0f64)) as libc::c_float,
                            (((*drawsurf).light_t - (*drawsurf).dlight_t)
                                as libc::c_double * (1.0f64 / 128.0f64)) as libc::c_float,
                        );
                    }
                    drawsurf = (*drawsurf).lightmapchain;
                }
                newdrawsurf = drawsurf;
                LM_InitBlock();
                if LM_AllocBlock(
                    smax,
                    tmax,
                    &mut (*surf).dlight_s,
                    &mut (*surf).dlight_t,
                ) as u64 == 0
                {
                    (ri.Sys_Error)
                        .expect(
                            "non-null function pointer",
                        )(
                        0 as libc::c_int,
                        b"Consecutive calls to LM_AllocBlock(%d,%d) failed (dynamic)\n\0"
                            as *const u8 as *const libc::c_char as *mut libc::c_char,
                        smax,
                        tmax,
                    );
                }
                base = (gl_lms.lightmap_buffer).as_mut_ptr();
                base = base
                    .offset(
                        (((*surf).dlight_t * 128 as libc::c_int + (*surf).dlight_s)
                            * 4 as libc::c_int) as isize,
                    );
                R_BuildLightMap(surf, base, 128 as libc::c_int * 4 as libc::c_int);
            }
            surf = (*surf).lightmapchain;
        }
        if !newdrawsurf.is_null() {
            LM_UploadBlock(true_0);
        }
        surf = newdrawsurf;
        while !surf.is_null() {
            if !((*surf).polys).is_null() {
                DrawGLPolyChain(
                    (*surf).polys,
                    (((*surf).light_s - (*surf).dlight_s) as libc::c_double
                        * (1.0f64 / 128.0f64)) as libc::c_float,
                    (((*surf).light_t - (*surf).dlight_t) as libc::c_double
                        * (1.0f64 / 128.0f64)) as libc::c_float,
                );
            }
            surf = (*surf).lightmapchain;
        }
    }
    qglDepthMask.expect("non-null function pointer")(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn R_RenderBrushPoly(mut fa: *mut msurface_t) {
    let mut current_block: u64;
    let mut maps: libc::c_int = 0;
    let mut image: *mut image_t = 0 as *mut image_t;
    let mut is_dynamic: qboolean = false_0;
    c_brush_polys += 1;
    image = R_TextureAnimation((*fa).texinfo);
    if (*fa).flags & 0x10 as libc::c_int != 0 {
        GL_Bind((*image).texnum);
        qglColor4f
            .expect(
                "non-null function pointer",
            )(
            gl_state.inverse_intensity as libc::c_int,
            gl_state.inverse_intensity as libc::c_int,
            gl_state.inverse_intensity as libc::c_int,
            1.0f32 as libc::c_int,
        );
        EmitWaterPolys(fa);
        return;
    } else {
        GL_Bind((*image).texnum);
    }
    if (*(*fa).texinfo).flags & 0x40 as libc::c_int != 0 {
        DrawGLFlowingPoly(fa);
    } else {
        DrawGLPoly((*fa).polys);
    }
    maps = 0 as libc::c_int;
    loop {
        if !(maps < 4 as libc::c_int
            && (*fa).styles[maps as usize] as libc::c_int != 255 as libc::c_int)
        {
            current_block = 12039483399334584727;
            break;
        }
        if (*(r_newrefdef.lightstyles).offset((*fa).styles[maps as usize] as isize))
            .white != (*fa).cached_light[maps as usize]
        {
            current_block = 14206553602194855675;
            break;
        }
        maps += 1;
    }
    match current_block {
        12039483399334584727 => {
            if (*fa).dlightframe == r_framecount {
                current_block = 14206553602194855675;
            } else {
                current_block = 5634871135123216486;
            }
        }
        _ => {}
    }
    match current_block {
        14206553602194855675 => {
            if (*gl_dynamic).value != 0. {
                if (*(*fa).texinfo).flags
                    & (0x4 as libc::c_int | 0x10 as libc::c_int | 0x20 as libc::c_int
                        | 0x8 as libc::c_int) == 0
                {
                    is_dynamic = true_0;
                }
            }
        }
        _ => {}
    }
    if is_dynamic as u64 != 0 {
        if ((*fa).styles[maps as usize] as libc::c_int >= 32 as libc::c_int
            || (*fa).styles[maps as usize] as libc::c_int == 0 as libc::c_int)
            && (*fa).dlightframe != r_framecount
        {
            let mut temp: [libc::c_uint; 1156] = [0; 1156];
            let mut smax: libc::c_int = 0;
            let mut tmax: libc::c_int = 0;
            smax = ((*fa).extents[0 as libc::c_int as usize] as libc::c_int
                >> 4 as libc::c_int) + 1 as libc::c_int;
            tmax = ((*fa).extents[1 as libc::c_int as usize] as libc::c_int
                >> 4 as libc::c_int) + 1 as libc::c_int;
            R_BuildLightMap(
                fa,
                temp.as_mut_ptr() as *mut libc::c_void as *mut byte,
                smax * 4 as libc::c_int,
            );
            R_SetCacheState(fa);
            GL_Bind(gl_state.lightmap_textures + (*fa).lightmaptexturenum);
            let ref mut fresh0 = (*fa).lightmapchain;
            *fresh0 = gl_lms.lightmap_surfaces[(*fa).lightmaptexturenum as usize];
            gl_lms.lightmap_surfaces[(*fa).lightmaptexturenum as usize] = fa;
        } else {
            let ref mut fresh1 = (*fa).lightmapchain;
            *fresh1 = gl_lms.lightmap_surfaces[0 as libc::c_int as usize];
            gl_lms.lightmap_surfaces[0 as libc::c_int as usize] = fa;
        }
    } else {
        let ref mut fresh2 = (*fa).lightmapchain;
        *fresh2 = gl_lms.lightmap_surfaces[(*fa).lightmaptexturenum as usize];
        gl_lms.lightmap_surfaces[(*fa).lightmaptexturenum as usize] = fa;
    };
}
#[no_mangle]
pub unsafe extern "C" fn R_DrawAlphaSurfaces() {
    let mut s: *mut msurface_t = 0 as *mut msurface_t;
    let mut intens: libc::c_float = 0.;
    qglLoadMatrixf
        .expect(
            "non-null function pointer",
        )(r_world_matrix.as_mut_ptr() as *const libc::c_int);
    intens = gl_state.inverse_intensity;
    s = r_alpha_surfaces;
    while !s.is_null() {
        GL_Bind((*(*(*s).texinfo).image).texnum);
        c_brush_polys += 1;
        if (*(*s).texinfo).flags & 0x10 as libc::c_int != 0 {
            qglColor4f
                .expect(
                    "non-null function pointer",
                )(
                intens as libc::c_int,
                intens as libc::c_int,
                intens as libc::c_int,
                0.33f64 as libc::c_int,
            );
        } else if (*(*s).texinfo).flags & 0x20 as libc::c_int != 0 {
            qglColor4f
                .expect(
                    "non-null function pointer",
                )(
                intens as libc::c_int,
                intens as libc::c_int,
                intens as libc::c_int,
                0.66f64 as libc::c_int,
            );
        } else {
            qglColor4f
                .expect(
                    "non-null function pointer",
                )(
                intens as libc::c_int,
                intens as libc::c_int,
                intens as libc::c_int,
                1 as libc::c_int,
            );
        }
        if (*s).flags & 0x10 as libc::c_int != 0 {
            EmitWaterPolys(s);
        } else {
            DrawGLPoly((*s).polys);
        }
        s = (*s).texturechain;
    }
    qglColor4f
        .expect(
            "non-null function pointer",
        )(1 as libc::c_int, 1 as libc::c_int, 1 as libc::c_int, 1 as libc::c_int);
    r_alpha_surfaces = 0 as *mut msurface_t;
}
unsafe extern "C" fn GL_RenderLightmappedPoly(mut surf: *mut msurface_t) {
    let mut current_block: u64;
    let mut i: libc::c_int = 0;
    let mut nv: libc::c_int = (*(*surf).polys).numverts;
    let mut map: libc::c_int = 0;
    let mut v: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut image: *mut image_t = R_TextureAnimation((*surf).texinfo);
    let mut is_dynamic: qboolean = false_0;
    let mut lmtex: libc::c_uint = (*surf).lightmaptexturenum as libc::c_uint;
    let mut p: *mut glpoly_t = 0 as *mut glpoly_t;
    map = 0 as libc::c_int;
    loop {
        if !(map < 4 as libc::c_int
            && (*surf).styles[map as usize] as libc::c_int != 255 as libc::c_int)
        {
            current_block = 10886091980245723256;
            break;
        }
        if (*(r_newrefdef.lightstyles).offset((*surf).styles[map as usize] as isize))
            .white != (*surf).cached_light[map as usize]
        {
            current_block = 15560064689176435631;
            break;
        }
        map += 1;
    }
    match current_block {
        10886091980245723256 => {
            if (*surf).dlightframe == r_framecount {
                current_block = 15560064689176435631;
            } else {
                current_block = 8236137900636309791;
            }
        }
        _ => {}
    }
    match current_block {
        15560064689176435631 => {
            if (*gl_dynamic).value != 0. {
                if (*(*surf).texinfo).flags
                    & (0x4 as libc::c_int | 0x10 as libc::c_int | 0x20 as libc::c_int
                        | 0x8 as libc::c_int) == 0
                {
                    is_dynamic = true_0;
                }
            }
        }
        _ => {}
    }
    if is_dynamic as u64 != 0 {
        let mut temp: [libc::c_uint; 16384] = [0; 16384];
        let mut smax: libc::c_int = 0;
        let mut tmax: libc::c_int = 0;
        if ((*surf).styles[map as usize] as libc::c_int >= 32 as libc::c_int
            || (*surf).styles[map as usize] as libc::c_int == 0 as libc::c_int)
            && (*surf).dlightframe != r_framecount
        {
            smax = ((*surf).extents[0 as libc::c_int as usize] as libc::c_int
                >> 4 as libc::c_int) + 1 as libc::c_int;
            tmax = ((*surf).extents[1 as libc::c_int as usize] as libc::c_int
                >> 4 as libc::c_int) + 1 as libc::c_int;
            R_BuildLightMap(
                surf,
                temp.as_mut_ptr() as *mut libc::c_void as *mut byte,
                smax * 4 as libc::c_int,
            );
            R_SetCacheState(surf);
            lmtex = (*surf).lightmaptexturenum as libc::c_uint;
        } else {
            smax = ((*surf).extents[0 as libc::c_int as usize] as libc::c_int
                >> 4 as libc::c_int) + 1 as libc::c_int;
            tmax = ((*surf).extents[1 as libc::c_int as usize] as libc::c_int
                >> 4 as libc::c_int) + 1 as libc::c_int;
            R_BuildLightMap(
                surf,
                temp.as_mut_ptr() as *mut libc::c_void as *mut byte,
                smax * 4 as libc::c_int,
            );
            lmtex = 0 as libc::c_int as libc::c_uint;
        }
        c_brush_polys += 1;
        if (*(*surf).texinfo).flags & 0x40 as libc::c_int != 0 {
            let mut scroll: libc::c_float = 0.;
            scroll = (-(64 as libc::c_int) as libc::c_double
                * (r_newrefdef.time as libc::c_double / 40.0f64
                    - (r_newrefdef.time as libc::c_double / 40.0f64) as libc::c_int
                        as libc::c_double)) as libc::c_float;
            if scroll as libc::c_double == 0.0f64 {
                scroll = -64.0f64 as libc::c_float;
            }
            p = (*surf).polys;
            while !p.is_null() {
                v = ((*p).verts[0 as libc::c_int as usize]).as_mut_ptr();
                i = 0 as libc::c_int;
                while i < nv {
                    qglVertex3fv
                        .expect("non-null function pointer")(v as *const libc::c_int);
                    i += 1;
                    v = v.offset(7 as libc::c_int as isize);
                }
                qglEnd.expect("non-null function pointer")();
                p = (*p).chain;
            }
        } else {
            p = (*surf).polys;
            while !p.is_null() {
                v = ((*p).verts[0 as libc::c_int as usize]).as_mut_ptr();
                i = 0 as libc::c_int;
                while i < nv {
                    qglVertex3fv
                        .expect("non-null function pointer")(v as *const libc::c_int);
                    i += 1;
                    v = v.offset(7 as libc::c_int as isize);
                }
                qglEnd.expect("non-null function pointer")();
                p = (*p).chain;
            }
        }
    } else {
        c_brush_polys += 1;
        if (*(*surf).texinfo).flags & 0x40 as libc::c_int != 0 {
            let mut scroll_0: libc::c_float = 0.;
            scroll_0 = (-(64 as libc::c_int) as libc::c_double
                * (r_newrefdef.time as libc::c_double / 40.0f64
                    - (r_newrefdef.time as libc::c_double / 40.0f64) as libc::c_int
                        as libc::c_double)) as libc::c_float;
            if scroll_0 as libc::c_double == 0.0f64 {
                scroll_0 = -64.0f64 as libc::c_float;
            }
            p = (*surf).polys;
            while !p.is_null() {
                v = ((*p).verts[0 as libc::c_int as usize]).as_mut_ptr();
                i = 0 as libc::c_int;
                while i < nv {
                    qglVertex3fv
                        .expect("non-null function pointer")(v as *const libc::c_int);
                    i += 1;
                    v = v.offset(7 as libc::c_int as isize);
                }
                qglEnd.expect("non-null function pointer")();
                p = (*p).chain;
            }
        } else {
            p = (*surf).polys;
            while !p.is_null() {
                v = ((*p).verts[0 as libc::c_int as usize]).as_mut_ptr();
                i = 0 as libc::c_int;
                while i < nv {
                    qglVertex3fv
                        .expect("non-null function pointer")(v as *const libc::c_int);
                    i += 1;
                    v = v.offset(7 as libc::c_int as isize);
                }
                qglEnd.expect("non-null function pointer")();
                p = (*p).chain;
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn R_DrawBrushModel(mut e: *mut entity_t) {
    let mut mins: vec3_t = [0.; 3];
    let mut maxs: vec3_t = [0.; 3];
    let mut i: libc::c_int = 0;
    let mut rotated: qboolean = false_0;
    if (*currentmodel).nummodelsurfaces == 0 as libc::c_int {
        return;
    }
    currententity = e;
    gl_state.currenttextures[1 as libc::c_int as usize] = -(1 as libc::c_int);
    gl_state
        .currenttextures[0 as libc::c_int
        as usize] = gl_state.currenttextures[1 as libc::c_int as usize];
    if (*e).angles[0 as libc::c_int as usize] != 0.
        || (*e).angles[1 as libc::c_int as usize] != 0.
        || (*e).angles[2 as libc::c_int as usize] != 0.
    {
        rotated = true_0;
        i = 0 as libc::c_int;
        while i < 3 as libc::c_int {
            mins[i as usize] = (*e).origin[i as usize] - (*currentmodel).radius;
            maxs[i as usize] = (*e).origin[i as usize] + (*currentmodel).radius;
            i += 1;
        }
    } else {
        rotated = false_0;
        mins[0 as libc::c_int
            as usize] = (*e).origin[0 as libc::c_int as usize]
            + (*currentmodel).mins[0 as libc::c_int as usize];
        mins[1 as libc::c_int
            as usize] = (*e).origin[1 as libc::c_int as usize]
            + (*currentmodel).mins[1 as libc::c_int as usize];
        mins[2 as libc::c_int
            as usize] = (*e).origin[2 as libc::c_int as usize]
            + (*currentmodel).mins[2 as libc::c_int as usize];
        maxs[0 as libc::c_int
            as usize] = (*e).origin[0 as libc::c_int as usize]
            + (*currentmodel).maxs[0 as libc::c_int as usize];
        maxs[1 as libc::c_int
            as usize] = (*e).origin[1 as libc::c_int as usize]
            + (*currentmodel).maxs[1 as libc::c_int as usize];
        maxs[2 as libc::c_int
            as usize] = (*e).origin[2 as libc::c_int as usize]
            + (*currentmodel).maxs[2 as libc::c_int as usize];
    }
    if R_CullBox(mins.as_mut_ptr(), maxs.as_mut_ptr()) as u64 != 0 {
        return;
    }
    qglColor3f
        .expect(
            "non-null function pointer",
        )(1 as libc::c_int, 1 as libc::c_int, 1 as libc::c_int);
    memset(
        (gl_lms.lightmap_surfaces).as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<[*mut msurface_t; 128]>() as libc::c_ulong,
    );
    modelorg[0 as libc::c_int
        as usize] = r_newrefdef.vieworg[0 as libc::c_int as usize]
        - (*e).origin[0 as libc::c_int as usize];
    modelorg[1 as libc::c_int
        as usize] = r_newrefdef.vieworg[1 as libc::c_int as usize]
        - (*e).origin[1 as libc::c_int as usize];
    modelorg[2 as libc::c_int
        as usize] = r_newrefdef.vieworg[2 as libc::c_int as usize]
        - (*e).origin[2 as libc::c_int as usize];
    if rotated as u64 != 0 {
        let mut temp: vec3_t = [0.; 3];
        let mut forward: vec3_t = [0.; 3];
        let mut right: vec3_t = [0.; 3];
        let mut up: vec3_t = [0.; 3];
        temp[0 as libc::c_int as usize] = modelorg[0 as libc::c_int as usize];
        temp[1 as libc::c_int as usize] = modelorg[1 as libc::c_int as usize];
        temp[2 as libc::c_int as usize] = modelorg[2 as libc::c_int as usize];
        AngleVectors(
            ((*e).angles).as_mut_ptr(),
            forward.as_mut_ptr(),
            right.as_mut_ptr(),
            up.as_mut_ptr(),
        );
        modelorg[0 as libc::c_int
            as usize] = temp[0 as libc::c_int as usize]
            * forward[0 as libc::c_int as usize]
            + temp[1 as libc::c_int as usize] * forward[1 as libc::c_int as usize]
            + temp[2 as libc::c_int as usize] * forward[2 as libc::c_int as usize];
        modelorg[1 as libc::c_int
            as usize] = -(temp[0 as libc::c_int as usize]
            * right[0 as libc::c_int as usize]
            + temp[1 as libc::c_int as usize] * right[1 as libc::c_int as usize]
            + temp[2 as libc::c_int as usize] * right[2 as libc::c_int as usize]);
        modelorg[2 as libc::c_int
            as usize] = temp[0 as libc::c_int as usize] * up[0 as libc::c_int as usize]
            + temp[1 as libc::c_int as usize] * up[1 as libc::c_int as usize]
            + temp[2 as libc::c_int as usize] * up[2 as libc::c_int as usize];
    }
    qglPushMatrix.expect("non-null function pointer")();
    (*e).angles[0 as libc::c_int as usize] = -(*e).angles[0 as libc::c_int as usize];
    (*e).angles[2 as libc::c_int as usize] = -(*e).angles[2 as libc::c_int as usize];
    R_RotateForEntity(e);
    (*e).angles[0 as libc::c_int as usize] = -(*e).angles[0 as libc::c_int as usize];
    (*e).angles[2 as libc::c_int as usize] = -(*e).angles[2 as libc::c_int as usize];
    GL_EnableMultitexture(true_0);
    R_DrawInlineBModel();
    GL_EnableMultitexture(false_0);
    qglPopMatrix.expect("non-null function pointer")();
}
#[no_mangle]
pub unsafe extern "C" fn R_MarkLeaves() {
    let mut vis: *mut byte = 0 as *mut byte;
    let mut fatvis: [byte; 8192] = [0; 8192];
    let mut node: *mut mnode_t = 0 as *mut mnode_t;
    let mut i: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut leaf: *mut mleaf_t = 0 as *mut mleaf_t;
    let mut cluster: libc::c_int = 0;
    if r_oldviewcluster == r_viewcluster && r_oldviewcluster2 == r_viewcluster2
        && (*r_novis).value == 0. && r_viewcluster != -(1 as libc::c_int)
    {
        return;
    }
    if (*gl_lockpvs).value != 0. {
        return;
    }
    r_visframecount += 1;
    r_oldviewcluster = r_viewcluster;
    r_oldviewcluster2 = r_viewcluster2;
    if (*r_novis).value != 0. || r_viewcluster == -(1 as libc::c_int)
        || ((*r_worldmodel).vis).is_null()
    {
        i = 0 as libc::c_int;
        while i < (*r_worldmodel).numleafs {
            (*((*r_worldmodel).leafs).offset(i as isize)).visframe = r_visframecount;
            i += 1;
        }
        i = 0 as libc::c_int;
        while i < (*r_worldmodel).numnodes {
            (*((*r_worldmodel).nodes).offset(i as isize)).visframe = r_visframecount;
            i += 1;
        }
        return;
    }
    vis = Mod_ClusterPVS(r_viewcluster, r_worldmodel);
    if r_viewcluster2 != r_viewcluster {
        memcpy(
            fatvis.as_mut_ptr() as *mut libc::c_void,
            vis as *const libc::c_void,
            (((*r_worldmodel).numleafs + 7 as libc::c_int) / 8 as libc::c_int)
                as libc::c_ulong,
        );
        vis = Mod_ClusterPVS(r_viewcluster2, r_worldmodel);
        c = ((*r_worldmodel).numleafs + 31 as libc::c_int) / 32 as libc::c_int;
        i = 0 as libc::c_int;
        while i < c {
            *(fatvis.as_mut_ptr() as *mut libc::c_int).offset(i as isize)
                |= *(vis as *mut libc::c_int).offset(i as isize);
            i += 1;
        }
        vis = fatvis.as_mut_ptr();
    }
    i = 0 as libc::c_int;
    leaf = (*r_worldmodel).leafs;
    while i < (*r_worldmodel).numleafs {
        cluster = (*leaf).cluster;
        if !(cluster == -(1 as libc::c_int)) {
            if *vis.offset((cluster >> 3 as libc::c_int) as isize) as libc::c_int
                & (1 as libc::c_int) << (cluster & 7 as libc::c_int) != 0
            {
                node = leaf as *mut mnode_t;
                while !((*node).visframe == r_visframecount) {
                    (*node).visframe = r_visframecount;
                    node = (*node).parent;
                    if node.is_null() {
                        break;
                    }
                }
            }
        }
        i += 1;
        leaf = leaf.offset(1);
    }
}
unsafe extern "C" fn LM_InitBlock() {
    memset(
        (gl_lms.allocated).as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<[libc::c_int; 128]>() as libc::c_ulong,
    );
}
unsafe extern "C" fn LM_UploadBlock(mut dynamic: qboolean) {
    let mut texture: libc::c_int = 0;
    let mut height: libc::c_int = 0 as libc::c_int;
    if dynamic as u64 != 0 {
        texture = 0 as libc::c_int;
    } else {
        texture = gl_lms.current_lightmap_texture;
    }
    GL_Bind(gl_state.lightmap_textures + texture);
    if dynamic as u64 != 0 {
        let mut i: libc::c_int = 0;
        i = 0 as libc::c_int;
        while i < 128 as libc::c_int {
            if gl_lms.allocated[i as usize] > height {
                height = gl_lms.allocated[i as usize];
            }
            i += 1;
        }
    } else {
        gl_lms.current_lightmap_texture += 1;
        if gl_lms.current_lightmap_texture == 128 as libc::c_int {
            (ri.Sys_Error)
                .expect(
                    "non-null function pointer",
                )(
                1 as libc::c_int,
                b"LM_UploadBlock() - MAX_LIGHTMAPS exceeded\n\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
            );
        }
    };
}
unsafe extern "C" fn LM_AllocBlock(
    mut w: libc::c_int,
    mut h: libc::c_int,
    mut x: *mut libc::c_int,
    mut y: *mut libc::c_int,
) -> qboolean {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut best: libc::c_int = 0;
    let mut best2: libc::c_int = 0;
    best = 128 as libc::c_int;
    i = 0 as libc::c_int;
    while i < 128 as libc::c_int - w {
        best2 = 0 as libc::c_int;
        j = 0 as libc::c_int;
        while j < w {
            if gl_lms.allocated[(i + j) as usize] >= best {
                break;
            }
            if gl_lms.allocated[(i + j) as usize] > best2 {
                best2 = gl_lms.allocated[(i + j) as usize];
            }
            j += 1;
        }
        if j == w {
            *x = i;
            best = best2;
            *y = best;
        }
        i += 1;
    }
    if best + h > 128 as libc::c_int {
        return false_0;
    }
    i = 0 as libc::c_int;
    while i < w {
        gl_lms.allocated[(*x + i) as usize] = best + h;
        i += 1;
    }
    return true_0;
}
#[no_mangle]
pub unsafe extern "C" fn GL_BuildPolygonFromSurface(mut fa: *mut msurface_t) {
    let mut i: libc::c_int = 0;
    let mut lindex: libc::c_int = 0;
    let mut lnumverts: libc::c_int = 0;
    let mut pedges: *mut medge_t = 0 as *mut medge_t;
    let mut r_pedge: *mut medge_t = 0 as *mut medge_t;
    let mut vertpage: libc::c_int = 0;
    let mut vec: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut s: libc::c_float = 0.;
    let mut t: libc::c_float = 0.;
    let mut poly: *mut glpoly_t = 0 as *mut glpoly_t;
    let mut total: vec3_t = [0.; 3];
    pedges = (*currentmodel).edges;
    lnumverts = (*fa).numedges;
    vertpage = 0 as libc::c_int;
    total[2 as libc::c_int as usize] = 0 as libc::c_int as vec_t;
    total[1 as libc::c_int as usize] = total[2 as libc::c_int as usize];
    total[0 as libc::c_int as usize] = total[1 as libc::c_int as usize];
    poly = Hunk_Alloc(
        (::std::mem::size_of::<glpoly_t>() as libc::c_ulong)
            .wrapping_add(
                (((lnumverts - 4 as libc::c_int) * 7 as libc::c_int) as libc::c_ulong)
                    .wrapping_mul(
                        ::std::mem::size_of::<libc::c_float>() as libc::c_ulong,
                    ),
            ) as libc::c_int,
    ) as *mut glpoly_t;
    let ref mut fresh5 = (*poly).next;
    *fresh5 = (*fa).polys;
    (*poly).flags = (*fa).flags;
    let ref mut fresh6 = (*fa).polys;
    *fresh6 = poly;
    (*poly).numverts = lnumverts;
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
        s = *vec.offset(0 as libc::c_int as isize)
            * (*(*fa).texinfo).vecs[0 as libc::c_int as usize][0 as libc::c_int as usize]
            + *vec.offset(1 as libc::c_int as isize)
                * (*(*fa).texinfo)
                    .vecs[0 as libc::c_int as usize][1 as libc::c_int as usize]
            + *vec.offset(2 as libc::c_int as isize)
                * (*(*fa).texinfo)
                    .vecs[0 as libc::c_int as usize][2 as libc::c_int as usize]
            + (*(*fa).texinfo)
                .vecs[0 as libc::c_int as usize][3 as libc::c_int as usize];
        s /= (*(*(*fa).texinfo).image).width as libc::c_float;
        t = *vec.offset(0 as libc::c_int as isize)
            * (*(*fa).texinfo).vecs[1 as libc::c_int as usize][0 as libc::c_int as usize]
            + *vec.offset(1 as libc::c_int as isize)
                * (*(*fa).texinfo)
                    .vecs[1 as libc::c_int as usize][1 as libc::c_int as usize]
            + *vec.offset(2 as libc::c_int as isize)
                * (*(*fa).texinfo)
                    .vecs[1 as libc::c_int as usize][2 as libc::c_int as usize]
            + (*(*fa).texinfo)
                .vecs[1 as libc::c_int as usize][3 as libc::c_int as usize];
        t /= (*(*(*fa).texinfo).image).height as libc::c_float;
        total[0 as libc::c_int
            as usize] = total[0 as libc::c_int as usize]
            + *vec.offset(0 as libc::c_int as isize);
        total[1 as libc::c_int
            as usize] = total[1 as libc::c_int as usize]
            + *vec.offset(1 as libc::c_int as isize);
        total[2 as libc::c_int
            as usize] = total[2 as libc::c_int as usize]
            + *vec.offset(2 as libc::c_int as isize);
        (*poly)
            .verts[i
            as usize][0 as libc::c_int
            as usize] = *vec.offset(0 as libc::c_int as isize);
        (*poly)
            .verts[i
            as usize][1 as libc::c_int
            as usize] = *vec.offset(1 as libc::c_int as isize);
        (*poly)
            .verts[i
            as usize][2 as libc::c_int
            as usize] = *vec.offset(2 as libc::c_int as isize);
        (*poly).verts[i as usize][3 as libc::c_int as usize] = s;
        (*poly).verts[i as usize][4 as libc::c_int as usize] = t;
        s = *vec.offset(0 as libc::c_int as isize)
            * (*(*fa).texinfo).vecs[0 as libc::c_int as usize][0 as libc::c_int as usize]
            + *vec.offset(1 as libc::c_int as isize)
                * (*(*fa).texinfo)
                    .vecs[0 as libc::c_int as usize][1 as libc::c_int as usize]
            + *vec.offset(2 as libc::c_int as isize)
                * (*(*fa).texinfo)
                    .vecs[0 as libc::c_int as usize][2 as libc::c_int as usize]
            + (*(*fa).texinfo)
                .vecs[0 as libc::c_int as usize][3 as libc::c_int as usize];
        s
            -= (*fa).texturemins[0 as libc::c_int as usize] as libc::c_int
                as libc::c_float;
        s += ((*fa).light_s * 16 as libc::c_int) as libc::c_float;
        s += 8 as libc::c_int as libc::c_float;
        s /= (128 as libc::c_int * 16 as libc::c_int) as libc::c_float;
        t = *vec.offset(0 as libc::c_int as isize)
            * (*(*fa).texinfo).vecs[1 as libc::c_int as usize][0 as libc::c_int as usize]
            + *vec.offset(1 as libc::c_int as isize)
                * (*(*fa).texinfo)
                    .vecs[1 as libc::c_int as usize][1 as libc::c_int as usize]
            + *vec.offset(2 as libc::c_int as isize)
                * (*(*fa).texinfo)
                    .vecs[1 as libc::c_int as usize][2 as libc::c_int as usize]
            + (*(*fa).texinfo)
                .vecs[1 as libc::c_int as usize][3 as libc::c_int as usize];
        t
            -= (*fa).texturemins[1 as libc::c_int as usize] as libc::c_int
                as libc::c_float;
        t += ((*fa).light_t * 16 as libc::c_int) as libc::c_float;
        t += 8 as libc::c_int as libc::c_float;
        t /= (128 as libc::c_int * 16 as libc::c_int) as libc::c_float;
        (*poly).verts[i as usize][5 as libc::c_int as usize] = s;
        (*poly).verts[i as usize][6 as libc::c_int as usize] = t;
        i += 1;
    }
    (*poly).numverts = lnumverts;
}
#[no_mangle]
pub unsafe extern "C" fn GL_CreateSurfaceLightmap(mut surf: *mut msurface_t) {
    let mut smax: libc::c_int = 0;
    let mut tmax: libc::c_int = 0;
    let mut base: *mut byte = 0 as *mut byte;
    if (*surf).flags & (4 as libc::c_int | 0x10 as libc::c_int) != 0 {
        return;
    }
    smax = ((*surf).extents[0 as libc::c_int as usize] as libc::c_int
        >> 4 as libc::c_int) + 1 as libc::c_int;
    tmax = ((*surf).extents[1 as libc::c_int as usize] as libc::c_int
        >> 4 as libc::c_int) + 1 as libc::c_int;
    if LM_AllocBlock(smax, tmax, &mut (*surf).light_s, &mut (*surf).light_t) as u64 == 0
    {
        LM_UploadBlock(false_0);
        LM_InitBlock();
        if LM_AllocBlock(smax, tmax, &mut (*surf).light_s, &mut (*surf).light_t) as u64
            == 0
        {
            (ri.Sys_Error)
                .expect(
                    "non-null function pointer",
                )(
                0 as libc::c_int,
                b"Consecutive calls to LM_AllocBlock(%d,%d) failed\n\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                smax,
                tmax,
            );
        }
    }
    (*surf).lightmaptexturenum = gl_lms.current_lightmap_texture;
    base = (gl_lms.lightmap_buffer).as_mut_ptr();
    base = base
        .offset(
            (((*surf).light_t * 128 as libc::c_int + (*surf).light_s) * 4 as libc::c_int)
                as isize,
        );
    R_SetCacheState(surf);
    R_BuildLightMap(surf, base, 128 as libc::c_int * 4 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn GL_BeginBuildingLightmaps(mut m: *mut model_t) {
    static mut lightstyles: [lightstyle_t; 256] = [lightstyle_t {
        rgb: [0.; 3],
        white: 0.,
    }; 256];
    let mut i: libc::c_int = 0;
    let mut dummy: [libc::c_uint; 16384] = [0; 16384];
    memset(
        (gl_lms.allocated).as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<[libc::c_int; 128]>() as libc::c_ulong,
    );
    r_framecount = 1 as libc::c_int;
    GL_EnableMultitexture(true_0);
    i = 0 as libc::c_int;
    while i < 256 as libc::c_int {
        lightstyles[i as usize]
            .rgb[0 as libc::c_int as usize] = 1 as libc::c_int as libc::c_float;
        lightstyles[i as usize]
            .rgb[1 as libc::c_int as usize] = 1 as libc::c_int as libc::c_float;
        lightstyles[i as usize]
            .rgb[2 as libc::c_int as usize] = 1 as libc::c_int as libc::c_float;
        lightstyles[i as usize].white = 3 as libc::c_int as libc::c_float;
        i += 1;
    }
    r_newrefdef.lightstyles = lightstyles.as_mut_ptr();
    if gl_state.lightmap_textures == 0 {
        gl_state.lightmap_textures = 1024 as libc::c_int;
    }
    gl_lms.current_lightmap_texture = 1 as libc::c_int;
    if toupper(
        *((*gl_monolightmap).string).offset(0 as libc::c_int as isize) as libc::c_int,
    ) == 'A' as i32
    {
        gl_lms.internal_format = gl_tex_alpha_format;
    } else if toupper(
        *((*gl_monolightmap).string).offset(0 as libc::c_int as isize) as libc::c_int,
    ) == 'C' as i32
    {
        gl_lms.internal_format = gl_tex_alpha_format;
    } else if !(toupper(
        *((*gl_monolightmap).string).offset(0 as libc::c_int as isize) as libc::c_int,
    ) == 'I' as i32)
    {
        if !(toupper(
            *((*gl_monolightmap).string).offset(0 as libc::c_int as isize) as libc::c_int,
        ) == 'L' as i32)
        {
            gl_lms.internal_format = gl_tex_solid_format;
        }
    }
    GL_Bind(gl_state.lightmap_textures + 0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn GL_EndBuildingLightmaps() {
    LM_UploadBlock(false_0);
    GL_EnableMultitexture(false_0);
}
