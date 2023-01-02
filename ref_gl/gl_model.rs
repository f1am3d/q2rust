#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    fn atoi(__nptr: *const libc::c_char) -> libc::c_int;
    fn ceil(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn floor(_: libc::c_double) -> libc::c_double;
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
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn VectorLength(v: *mut vec_t) -> vec_t;
    fn Com_sprintf(
        dest: *mut libc::c_char,
        size: libc::c_int,
        fmt: *mut libc::c_char,
        _: ...
    );
    fn LittleShort(l: libc::c_short) -> libc::c_short;
    fn LittleLong(l: libc::c_int) -> libc::c_int;
    fn LittleFloat(l: libc::c_float) -> libc::c_float;
    fn Hunk_Begin(maxsize: libc::c_int) -> *mut libc::c_void;
    fn Hunk_Alloc(size: libc::c_int) -> *mut libc::c_void;
    fn Hunk_Free(buf: *mut libc::c_void);
    fn Hunk_End() -> libc::c_int;
    fn GL_FreeUnusedImages();
    static mut r_oldviewcluster: libc::c_int;
    static mut r_viewcluster: libc::c_int;
    static mut r_worldmodel: *mut model_t;
    fn GL_FindImage(name: *mut libc::c_char, type_0: imagetype_t) -> *mut image_t;
    static mut r_notexture: *mut image_t;
    static mut currentmodel: *mut model_t;
    fn GL_SubdivideSurface(fa: *mut msurface_t);
    static mut ri: refimport_t;
    fn GL_BuildPolygonFromSurface(fa: *mut msurface_t);
    fn GL_CreateSurfaceLightmap(surf: *mut msurface_t);
    fn GL_EndBuildingLightmaps();
    fn GL_BeginBuildingLightmaps(m: *mut model_t);
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
pub struct dsprframe_t {
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub origin_x: libc::c_int,
    pub origin_y: libc::c_int,
    pub name: [libc::c_char; 64],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dsprite_t {
    pub ident: libc::c_int,
    pub version: libc::c_int,
    pub numframes: libc::c_int,
    pub frames: [dsprframe_t; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lump_t {
    pub fileofs: libc::c_int,
    pub filelen: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dheader_t {
    pub ident: libc::c_int,
    pub version: libc::c_int,
    pub lumps: [lump_t; 19],
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
pub struct dvertex_t {
    pub point: [libc::c_float; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dplane_t {
    pub normal: [libc::c_float; 3],
    pub dist: libc::c_float,
    pub type_0: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dnode_t {
    pub planenum: libc::c_int,
    pub children: [libc::c_int; 2],
    pub mins: [libc::c_short; 3],
    pub maxs: [libc::c_short; 3],
    pub firstface: libc::c_ushort,
    pub numfaces: libc::c_ushort,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct texinfo_s {
    pub vecs: [[libc::c_float; 4]; 2],
    pub flags: libc::c_int,
    pub value: libc::c_int,
    pub texture: [libc::c_char; 32],
    pub nexttexinfo: libc::c_int,
}
pub type texinfo_t = texinfo_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dedge_t {
    pub v: [libc::c_ushort; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dface_t {
    pub planenum: libc::c_ushort,
    pub side: libc::c_short,
    pub firstedge: libc::c_int,
    pub numedges: libc::c_short,
    pub texinfo: libc::c_short,
    pub styles: [byte; 4],
    pub lightofs: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dleaf_t {
    pub contents: libc::c_int,
    pub cluster: libc::c_short,
    pub area: libc::c_short,
    pub mins: [libc::c_short; 3],
    pub maxs: [libc::c_short; 3],
    pub firstleafface: libc::c_ushort,
    pub numleaffaces: libc::c_ushort,
    pub firstleafbrush: libc::c_ushort,
    pub numleafbrushes: libc::c_ushort,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dvis_t {
    pub numclusters: libc::c_int,
    pub bitofs: [[libc::c_int; 2]; 8],
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
pub static mut loadmodel: *mut model_t = 0 as *const model_t as *mut model_t;
#[no_mangle]
pub static mut modfilelen: libc::c_int = 0;
#[no_mangle]
pub static mut mod_novis: [byte; 8192] = [0; 8192];
#[no_mangle]
pub static mut mod_known: [model_t; 512] = [model_t {
    name: [0; 64],
    registration_sequence: 0,
    type_0: mod_bad,
    numframes: 0,
    flags: 0,
    mins: [0.; 3],
    maxs: [0.; 3],
    radius: 0.,
    clipbox: false_0,
    clipmins: [0.; 3],
    clipmaxs: [0.; 3],
    firstmodelsurface: 0,
    nummodelsurfaces: 0,
    lightmap: 0,
    numsubmodels: 0,
    submodels: 0 as *const mmodel_t as *mut mmodel_t,
    numplanes: 0,
    planes: 0 as *const cplane_t as *mut cplane_t,
    numleafs: 0,
    leafs: 0 as *const mleaf_t as *mut mleaf_t,
    numvertexes: 0,
    vertexes: 0 as *const mvertex_t as *mut mvertex_t,
    numedges: 0,
    edges: 0 as *const medge_t as *mut medge_t,
    numnodes: 0,
    firstnode: 0,
    nodes: 0 as *const mnode_t as *mut mnode_t,
    numtexinfo: 0,
    texinfo: 0 as *const mtexinfo_t as *mut mtexinfo_t,
    numsurfaces: 0,
    surfaces: 0 as *const msurface_t as *mut msurface_t,
    numsurfedges: 0,
    surfedges: 0 as *const libc::c_int as *mut libc::c_int,
    nummarksurfaces: 0,
    marksurfaces: 0 as *const *mut msurface_t as *mut *mut msurface_t,
    vis: 0 as *const dvis_t as *mut dvis_t,
    lightdata: 0 as *const byte as *mut byte,
    skins: [0 as *const image_t as *mut image_t; 32],
    extradatasize: 0,
    extradata: 0 as *const libc::c_void as *mut libc::c_void,
}; 512];
#[no_mangle]
pub static mut mod_numknown: libc::c_int = 0;
#[no_mangle]
pub static mut mod_inline: [model_t; 512] = [model_t {
    name: [0; 64],
    registration_sequence: 0,
    type_0: mod_bad,
    numframes: 0,
    flags: 0,
    mins: [0.; 3],
    maxs: [0.; 3],
    radius: 0.,
    clipbox: false_0,
    clipmins: [0.; 3],
    clipmaxs: [0.; 3],
    firstmodelsurface: 0,
    nummodelsurfaces: 0,
    lightmap: 0,
    numsubmodels: 0,
    submodels: 0 as *const mmodel_t as *mut mmodel_t,
    numplanes: 0,
    planes: 0 as *const cplane_t as *mut cplane_t,
    numleafs: 0,
    leafs: 0 as *const mleaf_t as *mut mleaf_t,
    numvertexes: 0,
    vertexes: 0 as *const mvertex_t as *mut mvertex_t,
    numedges: 0,
    edges: 0 as *const medge_t as *mut medge_t,
    numnodes: 0,
    firstnode: 0,
    nodes: 0 as *const mnode_t as *mut mnode_t,
    numtexinfo: 0,
    texinfo: 0 as *const mtexinfo_t as *mut mtexinfo_t,
    numsurfaces: 0,
    surfaces: 0 as *const msurface_t as *mut msurface_t,
    numsurfedges: 0,
    surfedges: 0 as *const libc::c_int as *mut libc::c_int,
    nummarksurfaces: 0,
    marksurfaces: 0 as *const *mut msurface_t as *mut *mut msurface_t,
    vis: 0 as *const dvis_t as *mut dvis_t,
    lightdata: 0 as *const byte as *mut byte,
    skins: [0 as *const image_t as *mut image_t; 32],
    extradatasize: 0,
    extradata: 0 as *const libc::c_void as *mut libc::c_void,
}; 512];
#[no_mangle]
pub static mut registration_sequence: libc::c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn Mod_PointInLeaf(
    mut p: *mut vec_t,
    mut model: *mut model_t,
) -> *mut mleaf_t {
    let mut node: *mut mnode_t = 0 as *mut mnode_t;
    let mut d: libc::c_float = 0.;
    let mut plane: *mut cplane_t = 0 as *mut cplane_t;
    if model.is_null() || ((*model).nodes).is_null() {
        (ri.Sys_Error)
            .expect(
                "non-null function pointer",
            )(
            1 as libc::c_int,
            b"Mod_PointInLeaf: bad model\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    node = (*model).nodes;
    loop {
        if (*node).contents != -(1 as libc::c_int) {
            return node as *mut mleaf_t;
        }
        plane = (*node).plane;
        d = *p.offset(0 as libc::c_int as isize)
            * (*plane).normal[0 as libc::c_int as usize]
            + *p.offset(1 as libc::c_int as isize)
                * (*plane).normal[1 as libc::c_int as usize]
            + *p.offset(2 as libc::c_int as isize)
                * (*plane).normal[2 as libc::c_int as usize] - (*plane).dist;
        if d > 0 as libc::c_int as libc::c_float {
            node = (*node).children[0 as libc::c_int as usize];
        } else {
            node = (*node).children[1 as libc::c_int as usize];
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn Mod_DecompressVis(
    mut in_0: *mut byte,
    mut model: *mut model_t,
) -> *mut byte {
    static mut decompressed: [byte; 8192] = [0; 8192];
    let mut c: libc::c_int = 0;
    let mut out: *mut byte = 0 as *mut byte;
    let mut row: libc::c_int = 0;
    row = (*(*model).vis).numclusters + 7 as libc::c_int >> 3 as libc::c_int;
    out = decompressed.as_mut_ptr();
    if in_0.is_null() {
        while row != 0 {
            let fresh0 = out;
            out = out.offset(1);
            *fresh0 = 0xff as libc::c_int as byte;
            row -= 1;
        }
        return decompressed.as_mut_ptr();
    }
    loop {
        if *in_0 != 0 {
            let fresh1 = in_0;
            in_0 = in_0.offset(1);
            let fresh2 = out;
            out = out.offset(1);
            *fresh2 = *fresh1;
        } else {
            c = *in_0.offset(1 as libc::c_int as isize) as libc::c_int;
            in_0 = in_0.offset(2 as libc::c_int as isize);
            while c != 0 {
                let fresh3 = out;
                out = out.offset(1);
                *fresh3 = 0 as libc::c_int as byte;
                c -= 1;
            }
        }
        if !((out.offset_from(decompressed.as_mut_ptr()) as libc::c_long)
            < row as libc::c_long)
        {
            break;
        }
    }
    return decompressed.as_mut_ptr();
}
#[no_mangle]
pub unsafe extern "C" fn Mod_ClusterPVS(
    mut cluster: libc::c_int,
    mut model: *mut model_t,
) -> *mut byte {
    if cluster == -(1 as libc::c_int) || ((*model).vis).is_null() {
        return mod_novis.as_mut_ptr();
    }
    return Mod_DecompressVis(
        ((*model).vis as *mut byte)
            .offset(
                (*(*model).vis).bitofs[cluster as usize][0 as libc::c_int as usize]
                    as isize,
            ),
        model,
    );
}
#[no_mangle]
pub unsafe extern "C" fn Mod_Modellist_f() {
    let mut i: libc::c_int = 0;
    let mut mod_0: *mut model_t = 0 as *mut model_t;
    let mut total: libc::c_int = 0;
    total = 0 as libc::c_int;
    (ri.Con_Printf)
        .expect(
            "non-null function pointer",
        )(
        0 as libc::c_int,
        b"Loaded models:\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    i = 0 as libc::c_int;
    mod_0 = mod_known.as_mut_ptr();
    while i < mod_numknown {
        if !((*mod_0).name[0 as libc::c_int as usize] == 0) {
            (ri.Con_Printf)
                .expect(
                    "non-null function pointer",
                )(
                0 as libc::c_int,
                b"%8i : %s\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                (*mod_0).extradatasize,
                ((*mod_0).name).as_mut_ptr(),
            );
            total += (*mod_0).extradatasize;
        }
        i += 1;
        mod_0 = mod_0.offset(1);
    }
    (ri.Con_Printf)
        .expect(
            "non-null function pointer",
        )(
        0 as libc::c_int,
        b"Total resident: %i\n\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        total,
    );
}
#[no_mangle]
pub unsafe extern "C" fn Mod_Init() {
    memset(
        mod_novis.as_mut_ptr() as *mut libc::c_void,
        0xff as libc::c_int,
        ::std::mem::size_of::<[byte; 8192]>() as libc::c_ulong,
    );
}
#[no_mangle]
pub unsafe extern "C" fn Mod_ForName(
    mut name: *mut libc::c_char,
    mut crash: qboolean,
) -> *mut model_t {
    let mut mod_0: *mut model_t = 0 as *mut model_t;
    let mut buf: *mut libc::c_uint = 0 as *mut libc::c_uint;
    let mut i: libc::c_int = 0;
    if *name.offset(0 as libc::c_int as isize) == 0 {
        (ri.Sys_Error)
            .expect(
                "non-null function pointer",
            )(
            1 as libc::c_int,
            b"Mod_ForName: NULL name\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    if *name.offset(0 as libc::c_int as isize) as libc::c_int == '*' as i32 {
        i = atoi(name.offset(1 as libc::c_int as isize));
        if i < 1 as libc::c_int || r_worldmodel.is_null()
            || i >= (*r_worldmodel).numsubmodels
        {
            (ri.Sys_Error)
                .expect(
                    "non-null function pointer",
                )(
                1 as libc::c_int,
                b"bad inline model number\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
        }
        return &mut *mod_inline.as_mut_ptr().offset(i as isize) as *mut model_t;
    }
    i = 0 as libc::c_int;
    mod_0 = mod_known.as_mut_ptr();
    while i < mod_numknown {
        if !((*mod_0).name[0 as libc::c_int as usize] == 0) {
            if strcmp(((*mod_0).name).as_mut_ptr(), name) == 0 {
                return mod_0;
            }
        }
        i += 1;
        mod_0 = mod_0.offset(1);
    }
    i = 0 as libc::c_int;
    mod_0 = mod_known.as_mut_ptr();
    while i < mod_numknown {
        if (*mod_0).name[0 as libc::c_int as usize] == 0 {
            break;
        }
        i += 1;
        mod_0 = mod_0.offset(1);
    }
    if i == mod_numknown {
        if mod_numknown == 512 as libc::c_int {
            (ri.Sys_Error)
                .expect(
                    "non-null function pointer",
                )(
                1 as libc::c_int,
                b"mod_numknown == MAX_MOD_KNOWN\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
        }
        mod_numknown += 1;
    }
    strcpy(((*mod_0).name).as_mut_ptr(), name);
    modfilelen = (ri.FS_LoadFile)
        .expect(
            "non-null function pointer",
        )(
        ((*mod_0).name).as_mut_ptr(),
        &mut buf as *mut *mut libc::c_uint as *mut *mut libc::c_void,
    );
    if buf.is_null() {
        if crash as u64 != 0 {
            (ri.Sys_Error)
                .expect(
                    "non-null function pointer",
                )(
                1 as libc::c_int,
                b"Mod_NumForName: %s not found\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ((*mod_0).name).as_mut_ptr(),
            );
        }
        memset(
            ((*mod_0).name).as_mut_ptr() as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong,
        );
        return 0 as *mut model_t;
    }
    loadmodel = mod_0;
    match LittleLong(*buf as libc::c_int) {
        844121161 => {
            let ref mut fresh4 = (*loadmodel).extradata;
            *fresh4 = Hunk_Begin(0x200000 as libc::c_int);
            Mod_LoadAliasModel(mod_0, buf as *mut libc::c_void);
        }
        844317769 => {
            let ref mut fresh5 = (*loadmodel).extradata;
            *fresh5 = Hunk_Begin(0x10000 as libc::c_int);
            Mod_LoadSpriteModel(mod_0, buf as *mut libc::c_void);
        }
        1347633737 => {
            let ref mut fresh6 = (*loadmodel).extradata;
            *fresh6 = Hunk_Begin(0x1000000 as libc::c_int);
            Mod_LoadBrushModel(mod_0, buf as *mut libc::c_void);
        }
        _ => {
            (ri.Sys_Error)
                .expect(
                    "non-null function pointer",
                )(
                1 as libc::c_int,
                b"Mod_NumForName: unknown fileid for %s\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ((*mod_0).name).as_mut_ptr(),
            );
        }
    }
    (*loadmodel).extradatasize = Hunk_End();
    (ri.FS_FreeFile).expect("non-null function pointer")(buf as *mut libc::c_void);
    return mod_0;
}
#[no_mangle]
pub static mut mod_base: *mut byte = 0 as *const byte as *mut byte;
#[no_mangle]
pub unsafe extern "C" fn Mod_LoadLighting(mut l: *mut lump_t) {
    if (*l).filelen == 0 {
        let ref mut fresh7 = (*loadmodel).lightdata;
        *fresh7 = 0 as *mut byte;
        return;
    }
    let ref mut fresh8 = (*loadmodel).lightdata;
    *fresh8 = Hunk_Alloc((*l).filelen) as *mut byte;
    memcpy(
        (*loadmodel).lightdata as *mut libc::c_void,
        mod_base.offset((*l).fileofs as isize) as *const libc::c_void,
        (*l).filelen as libc::c_ulong,
    );
}
#[no_mangle]
pub unsafe extern "C" fn Mod_LoadVisibility(mut l: *mut lump_t) {
    let mut i: libc::c_int = 0;
    if (*l).filelen == 0 {
        let ref mut fresh9 = (*loadmodel).vis;
        *fresh9 = 0 as *mut dvis_t;
        return;
    }
    let ref mut fresh10 = (*loadmodel).vis;
    *fresh10 = Hunk_Alloc((*l).filelen) as *mut dvis_t;
    memcpy(
        (*loadmodel).vis as *mut libc::c_void,
        mod_base.offset((*l).fileofs as isize) as *const libc::c_void,
        (*l).filelen as libc::c_ulong,
    );
    (*(*loadmodel).vis).numclusters = LittleLong((*(*loadmodel).vis).numclusters);
    i = 0 as libc::c_int;
    while i < (*(*loadmodel).vis).numclusters {
        (*(*loadmodel).vis)
            .bitofs[i
            as usize][0 as libc::c_int
            as usize] = LittleLong(
            (*(*loadmodel).vis).bitofs[i as usize][0 as libc::c_int as usize],
        );
        (*(*loadmodel).vis)
            .bitofs[i
            as usize][1 as libc::c_int
            as usize] = LittleLong(
            (*(*loadmodel).vis).bitofs[i as usize][1 as libc::c_int as usize],
        );
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn Mod_LoadVertexes(mut l: *mut lump_t) {
    let mut in_0: *mut dvertex_t = 0 as *mut dvertex_t;
    let mut out: *mut mvertex_t = 0 as *mut mvertex_t;
    let mut i: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    in_0 = mod_base.offset((*l).fileofs as isize) as *mut libc::c_void as *mut dvertex_t;
    if ((*l).filelen as libc::c_ulong)
        .wrapping_rem(::std::mem::size_of::<dvertex_t>() as libc::c_ulong) != 0
    {
        (ri.Sys_Error)
            .expect(
                "non-null function pointer",
            )(
            1 as libc::c_int,
            b"MOD_LoadBmodel: funny lump size in %s\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            ((*loadmodel).name).as_mut_ptr(),
        );
    }
    count = ((*l).filelen as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<dvertex_t>() as libc::c_ulong)
        as libc::c_int;
    out = Hunk_Alloc(
        (count as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<mvertex_t>() as libc::c_ulong)
            as libc::c_int,
    ) as *mut mvertex_t;
    let ref mut fresh11 = (*loadmodel).vertexes;
    *fresh11 = out;
    (*loadmodel).numvertexes = count;
    i = 0 as libc::c_int;
    while i < count {
        (*out)
            .position[0 as libc::c_int
            as usize] = LittleFloat((*in_0).point[0 as libc::c_int as usize]);
        (*out)
            .position[1 as libc::c_int
            as usize] = LittleFloat((*in_0).point[1 as libc::c_int as usize]);
        (*out)
            .position[2 as libc::c_int
            as usize] = LittleFloat((*in_0).point[2 as libc::c_int as usize]);
        i += 1;
        in_0 = in_0.offset(1);
        out = out.offset(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn RadiusFromBounds(
    mut mins: *mut vec_t,
    mut maxs: *mut vec_t,
) -> libc::c_float {
    let mut i: libc::c_int = 0;
    let mut corner: vec3_t = [0.; 3];
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        corner[i
            as usize] = (if fabs(*mins.offset(i as isize) as libc::c_double)
            > fabs(*maxs.offset(i as isize) as libc::c_double)
        {
            fabs(*mins.offset(i as isize) as libc::c_double)
        } else {
            fabs(*maxs.offset(i as isize) as libc::c_double)
        }) as vec_t;
        i += 1;
    }
    return VectorLength(corner.as_mut_ptr());
}
#[no_mangle]
pub unsafe extern "C" fn Mod_LoadSubmodels(mut l: *mut lump_t) {
    let mut in_0: *mut dmodel_t = 0 as *mut dmodel_t;
    let mut out: *mut mmodel_t = 0 as *mut mmodel_t;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    in_0 = mod_base.offset((*l).fileofs as isize) as *mut libc::c_void as *mut dmodel_t;
    if ((*l).filelen as libc::c_ulong)
        .wrapping_rem(::std::mem::size_of::<dmodel_t>() as libc::c_ulong) != 0
    {
        (ri.Sys_Error)
            .expect(
                "non-null function pointer",
            )(
            1 as libc::c_int,
            b"MOD_LoadBmodel: funny lump size in %s\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            ((*loadmodel).name).as_mut_ptr(),
        );
    }
    count = ((*l).filelen as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<dmodel_t>() as libc::c_ulong) as libc::c_int;
    out = Hunk_Alloc(
        (count as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<mmodel_t>() as libc::c_ulong)
            as libc::c_int,
    ) as *mut mmodel_t;
    let ref mut fresh12 = (*loadmodel).submodels;
    *fresh12 = out;
    (*loadmodel).numsubmodels = count;
    i = 0 as libc::c_int;
    while i < count {
        j = 0 as libc::c_int;
        while j < 3 as libc::c_int {
            (*out)
                .mins[j
                as usize] = LittleFloat((*in_0).mins[j as usize])
                - 1 as libc::c_int as libc::c_float;
            (*out)
                .maxs[j
                as usize] = LittleFloat((*in_0).maxs[j as usize])
                + 1 as libc::c_int as libc::c_float;
            (*out).origin[j as usize] = LittleFloat((*in_0).origin[j as usize]);
            j += 1;
        }
        (*out)
            .radius = RadiusFromBounds(
            ((*out).mins).as_mut_ptr(),
            ((*out).maxs).as_mut_ptr(),
        );
        (*out).headnode = LittleLong((*in_0).headnode);
        (*out).firstface = LittleLong((*in_0).firstface);
        (*out).numfaces = LittleLong((*in_0).numfaces);
        i += 1;
        in_0 = in_0.offset(1);
        out = out.offset(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn Mod_LoadEdges(mut l: *mut lump_t) {
    let mut in_0: *mut dedge_t = 0 as *mut dedge_t;
    let mut out: *mut medge_t = 0 as *mut medge_t;
    let mut i: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    in_0 = mod_base.offset((*l).fileofs as isize) as *mut libc::c_void as *mut dedge_t;
    if ((*l).filelen as libc::c_ulong)
        .wrapping_rem(::std::mem::size_of::<dedge_t>() as libc::c_ulong) != 0
    {
        (ri.Sys_Error)
            .expect(
                "non-null function pointer",
            )(
            1 as libc::c_int,
            b"MOD_LoadBmodel: funny lump size in %s\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            ((*loadmodel).name).as_mut_ptr(),
        );
    }
    count = ((*l).filelen as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<dedge_t>() as libc::c_ulong) as libc::c_int;
    out = Hunk_Alloc(
        ((count + 1 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<medge_t>() as libc::c_ulong)
            as libc::c_int,
    ) as *mut medge_t;
    let ref mut fresh13 = (*loadmodel).edges;
    *fresh13 = out;
    (*loadmodel).numedges = count;
    i = 0 as libc::c_int;
    while i < count {
        (*out)
            .v[0 as libc::c_int
            as usize] = LittleShort(
            (*in_0).v[0 as libc::c_int as usize] as libc::c_short,
        ) as libc::c_ushort;
        (*out)
            .v[1 as libc::c_int
            as usize] = LittleShort(
            (*in_0).v[1 as libc::c_int as usize] as libc::c_short,
        ) as libc::c_ushort;
        i += 1;
        in_0 = in_0.offset(1);
        out = out.offset(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn Mod_LoadTexinfo(mut l: *mut lump_t) {
    let mut in_0: *mut texinfo_t = 0 as *mut texinfo_t;
    let mut out: *mut mtexinfo_t = 0 as *mut mtexinfo_t;
    let mut step: *mut mtexinfo_t = 0 as *mut mtexinfo_t;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    let mut name: [libc::c_char; 64] = [0; 64];
    let mut next: libc::c_int = 0;
    in_0 = mod_base.offset((*l).fileofs as isize) as *mut libc::c_void as *mut texinfo_t;
    if ((*l).filelen as libc::c_ulong)
        .wrapping_rem(::std::mem::size_of::<texinfo_t>() as libc::c_ulong) != 0
    {
        (ri.Sys_Error)
            .expect(
                "non-null function pointer",
            )(
            1 as libc::c_int,
            b"MOD_LoadBmodel: funny lump size in %s\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            ((*loadmodel).name).as_mut_ptr(),
        );
    }
    count = ((*l).filelen as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<texinfo_t>() as libc::c_ulong)
        as libc::c_int;
    out = Hunk_Alloc(
        (count as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<mtexinfo_t>() as libc::c_ulong)
            as libc::c_int,
    ) as *mut mtexinfo_t;
    let ref mut fresh14 = (*loadmodel).texinfo;
    *fresh14 = out;
    (*loadmodel).numtexinfo = count;
    i = 0 as libc::c_int;
    while i < count {
        j = 0 as libc::c_int;
        while j < 8 as libc::c_int {
            (*out)
                .vecs[0 as libc::c_int
                as usize][j
                as usize] = LittleFloat(
                (*in_0).vecs[0 as libc::c_int as usize][j as usize],
            );
            j += 1;
        }
        (*out).flags = LittleLong((*in_0).flags);
        next = LittleLong((*in_0).nexttexinfo);
        if next > 0 as libc::c_int {
            let ref mut fresh15 = (*out).next;
            *fresh15 = ((*loadmodel).texinfo).offset(next as isize);
        } else {
            let ref mut fresh16 = (*out).next;
            *fresh16 = 0 as *mut mtexinfo_s;
        }
        Com_sprintf(
            name.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
            b"textures/%s.wal\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            ((*in_0).texture).as_mut_ptr(),
        );
        let ref mut fresh17 = (*out).image;
        *fresh17 = GL_FindImage(name.as_mut_ptr(), it_wall);
        if ((*out).image).is_null() {
            (ri.Con_Printf)
                .expect(
                    "non-null function pointer",
                )(
                0 as libc::c_int,
                b"Couldn't load %s\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                name.as_mut_ptr(),
            );
            let ref mut fresh18 = (*out).image;
            *fresh18 = r_notexture;
        }
        i += 1;
        in_0 = in_0.offset(1);
        out = out.offset(1);
    }
    i = 0 as libc::c_int;
    while i < count {
        out = &mut *((*loadmodel).texinfo).offset(i as isize) as *mut mtexinfo_t;
        (*out).numframes = 1 as libc::c_int;
        step = (*out).next;
        while !step.is_null() && step != out {
            let ref mut fresh19 = (*out).numframes;
            *fresh19 += 1;
            step = (*step).next;
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn CalcSurfaceExtents(mut s: *mut msurface_t) {
    let mut mins: [libc::c_float; 2] = [0.; 2];
    let mut maxs: [libc::c_float; 2] = [0.; 2];
    let mut val: libc::c_float = 0.;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut e: libc::c_int = 0;
    let mut v: *mut mvertex_t = 0 as *mut mvertex_t;
    let mut tex: *mut mtexinfo_t = 0 as *mut mtexinfo_t;
    let mut bmins: [libc::c_int; 2] = [0; 2];
    let mut bmaxs: [libc::c_int; 2] = [0; 2];
    mins[1 as libc::c_int as usize] = 999999 as libc::c_int as libc::c_float;
    mins[0 as libc::c_int as usize] = mins[1 as libc::c_int as usize];
    maxs[1 as libc::c_int as usize] = -(99999 as libc::c_int) as libc::c_float;
    maxs[0 as libc::c_int as usize] = maxs[1 as libc::c_int as usize];
    tex = (*s).texinfo;
    i = 0 as libc::c_int;
    while i < (*s).numedges {
        e = *((*loadmodel).surfedges).offset(((*s).firstedge + i) as isize);
        if e >= 0 as libc::c_int {
            v = &mut *((*loadmodel).vertexes)
                .offset(
                    *((*((*loadmodel).edges).offset(e as isize)).v)
                        .as_mut_ptr()
                        .offset(0 as libc::c_int as isize) as isize,
                ) as *mut mvertex_t;
        } else {
            v = &mut *((*loadmodel).vertexes)
                .offset(
                    *((*((*loadmodel).edges).offset(-e as isize)).v)
                        .as_mut_ptr()
                        .offset(1 as libc::c_int as isize) as isize,
                ) as *mut mvertex_t;
        }
        j = 0 as libc::c_int;
        while j < 2 as libc::c_int {
            val = (*v).position[0 as libc::c_int as usize]
                * (*tex).vecs[j as usize][0 as libc::c_int as usize]
                + (*v).position[1 as libc::c_int as usize]
                    * (*tex).vecs[j as usize][1 as libc::c_int as usize]
                + (*v).position[2 as libc::c_int as usize]
                    * (*tex).vecs[j as usize][2 as libc::c_int as usize]
                + (*tex).vecs[j as usize][3 as libc::c_int as usize];
            if val < mins[j as usize] {
                mins[j as usize] = val;
            }
            if val > maxs[j as usize] {
                maxs[j as usize] = val;
            }
            j += 1;
        }
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < 2 as libc::c_int {
        bmins[i
            as usize] = floor(
            (mins[i as usize] / 16 as libc::c_int as libc::c_float) as libc::c_double,
        ) as libc::c_int;
        bmaxs[i
            as usize] = ceil(
            (maxs[i as usize] / 16 as libc::c_int as libc::c_float) as libc::c_double,
        ) as libc::c_int;
        (*s)
            .texturemins[i
            as usize] = (bmins[i as usize] * 16 as libc::c_int) as libc::c_short;
        (*s)
            .extents[i
            as usize] = ((bmaxs[i as usize] - bmins[i as usize]) * 16 as libc::c_int)
            as libc::c_short;
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn Mod_LoadFaces(mut l: *mut lump_t) {
    let mut in_0: *mut dface_t = 0 as *mut dface_t;
    let mut out: *mut msurface_t = 0 as *mut msurface_t;
    let mut i: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    let mut surfnum: libc::c_int = 0;
    let mut planenum: libc::c_int = 0;
    let mut side: libc::c_int = 0;
    let mut ti: libc::c_int = 0;
    in_0 = mod_base.offset((*l).fileofs as isize) as *mut libc::c_void as *mut dface_t;
    if ((*l).filelen as libc::c_ulong)
        .wrapping_rem(::std::mem::size_of::<dface_t>() as libc::c_ulong) != 0
    {
        (ri.Sys_Error)
            .expect(
                "non-null function pointer",
            )(
            1 as libc::c_int,
            b"MOD_LoadBmodel: funny lump size in %s\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            ((*loadmodel).name).as_mut_ptr(),
        );
    }
    count = ((*l).filelen as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<dface_t>() as libc::c_ulong) as libc::c_int;
    out = Hunk_Alloc(
        (count as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<msurface_t>() as libc::c_ulong)
            as libc::c_int,
    ) as *mut msurface_t;
    let ref mut fresh20 = (*loadmodel).surfaces;
    *fresh20 = out;
    (*loadmodel).numsurfaces = count;
    currentmodel = loadmodel;
    GL_BeginBuildingLightmaps(loadmodel);
    surfnum = 0 as libc::c_int;
    while surfnum < count {
        (*out).firstedge = LittleLong((*in_0).firstedge);
        (*out).numedges = LittleShort((*in_0).numedges) as libc::c_int;
        (*out).flags = 0 as libc::c_int;
        let ref mut fresh21 = (*out).polys;
        *fresh21 = 0 as *mut glpoly_t;
        planenum = LittleShort((*in_0).planenum as libc::c_short) as libc::c_int;
        side = LittleShort((*in_0).side) as libc::c_int;
        if side != 0 {
            (*out).flags |= 2 as libc::c_int;
        }
        let ref mut fresh22 = (*out).plane;
        *fresh22 = ((*loadmodel).planes).offset(planenum as isize);
        ti = LittleShort((*in_0).texinfo) as libc::c_int;
        if ti < 0 as libc::c_int || ti >= (*loadmodel).numtexinfo {
            (ri.Sys_Error)
                .expect(
                    "non-null function pointer",
                )(
                1 as libc::c_int,
                b"MOD_LoadBmodel: bad texinfo number\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
            );
        }
        let ref mut fresh23 = (*out).texinfo;
        *fresh23 = ((*loadmodel).texinfo).offset(ti as isize);
        CalcSurfaceExtents(out);
        i = 0 as libc::c_int;
        while i < 4 as libc::c_int {
            (*out).styles[i as usize] = (*in_0).styles[i as usize];
            i += 1;
        }
        i = LittleLong((*in_0).lightofs);
        if i == -(1 as libc::c_int) {
            let ref mut fresh24 = (*out).samples;
            *fresh24 = 0 as *mut byte;
        } else {
            let ref mut fresh25 = (*out).samples;
            *fresh25 = ((*loadmodel).lightdata).offset(i as isize);
        }
        if (*(*out).texinfo).flags & 0x8 as libc::c_int != 0 {
            (*out).flags |= 0x10 as libc::c_int;
            i = 0 as libc::c_int;
            while i < 2 as libc::c_int {
                (*out).extents[i as usize] = 16384 as libc::c_int as libc::c_short;
                (*out).texturemins[i as usize] = -(8192 as libc::c_int) as libc::c_short;
                i += 1;
            }
            GL_SubdivideSurface(out);
        }
        if (*(*out).texinfo).flags
            & (0x4 as libc::c_int | 0x10 as libc::c_int | 0x20 as libc::c_int
                | 0x8 as libc::c_int) == 0
        {
            GL_CreateSurfaceLightmap(out);
        }
        if (*(*out).texinfo).flags & 0x8 as libc::c_int == 0 {
            GL_BuildPolygonFromSurface(out);
        }
        surfnum += 1;
        in_0 = in_0.offset(1);
        out = out.offset(1);
    }
    GL_EndBuildingLightmaps();
}
#[no_mangle]
pub unsafe extern "C" fn Mod_SetParent(
    mut node: *mut mnode_t,
    mut parent: *mut mnode_t,
) {
    let ref mut fresh26 = (*node).parent;
    *fresh26 = parent;
    if (*node).contents != -(1 as libc::c_int) {
        return;
    }
    Mod_SetParent((*node).children[0 as libc::c_int as usize], node);
    Mod_SetParent((*node).children[1 as libc::c_int as usize], node);
}
#[no_mangle]
pub unsafe extern "C" fn Mod_LoadNodes(mut l: *mut lump_t) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    let mut p: libc::c_int = 0;
    let mut in_0: *mut dnode_t = 0 as *mut dnode_t;
    let mut out: *mut mnode_t = 0 as *mut mnode_t;
    in_0 = mod_base.offset((*l).fileofs as isize) as *mut libc::c_void as *mut dnode_t;
    if ((*l).filelen as libc::c_ulong)
        .wrapping_rem(::std::mem::size_of::<dnode_t>() as libc::c_ulong) != 0
    {
        (ri.Sys_Error)
            .expect(
                "non-null function pointer",
            )(
            1 as libc::c_int,
            b"MOD_LoadBmodel: funny lump size in %s\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            ((*loadmodel).name).as_mut_ptr(),
        );
    }
    count = ((*l).filelen as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<dnode_t>() as libc::c_ulong) as libc::c_int;
    out = Hunk_Alloc(
        (count as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<mnode_t>() as libc::c_ulong)
            as libc::c_int,
    ) as *mut mnode_t;
    let ref mut fresh27 = (*loadmodel).nodes;
    *fresh27 = out;
    (*loadmodel).numnodes = count;
    i = 0 as libc::c_int;
    while i < count {
        j = 0 as libc::c_int;
        while j < 3 as libc::c_int {
            (*out)
                .minmaxs[j
                as usize] = LittleShort((*in_0).mins[j as usize]) as libc::c_float;
            (*out)
                .minmaxs[(3 as libc::c_int + j)
                as usize] = LittleShort((*in_0).maxs[j as usize]) as libc::c_float;
            j += 1;
        }
        p = LittleLong((*in_0).planenum);
        let ref mut fresh28 = (*out).plane;
        *fresh28 = ((*loadmodel).planes).offset(p as isize);
        (*out)
            .firstsurface = LittleShort((*in_0).firstface as libc::c_short)
            as libc::c_ushort;
        (*out)
            .numsurfaces = LittleShort((*in_0).numfaces as libc::c_short)
            as libc::c_ushort;
        (*out).contents = -(1 as libc::c_int);
        j = 0 as libc::c_int;
        while j < 2 as libc::c_int {
            p = LittleLong((*in_0).children[j as usize]);
            if p >= 0 as libc::c_int {
                let ref mut fresh29 = (*out).children[j as usize];
                *fresh29 = ((*loadmodel).nodes).offset(p as isize);
            } else {
                let ref mut fresh30 = (*out).children[j as usize];
                *fresh30 = ((*loadmodel).leafs)
                    .offset((-(1 as libc::c_int) - p) as isize) as *mut mnode_t;
            }
            j += 1;
        }
        i += 1;
        in_0 = in_0.offset(1);
        out = out.offset(1);
    }
    Mod_SetParent((*loadmodel).nodes, 0 as *mut mnode_t);
}
#[no_mangle]
pub unsafe extern "C" fn Mod_LoadLeafs(mut l: *mut lump_t) {
    let mut in_0: *mut dleaf_t = 0 as *mut dleaf_t;
    let mut out: *mut mleaf_t = 0 as *mut mleaf_t;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    let mut p: libc::c_int = 0;
    in_0 = mod_base.offset((*l).fileofs as isize) as *mut libc::c_void as *mut dleaf_t;
    if ((*l).filelen as libc::c_ulong)
        .wrapping_rem(::std::mem::size_of::<dleaf_t>() as libc::c_ulong) != 0
    {
        (ri.Sys_Error)
            .expect(
                "non-null function pointer",
            )(
            1 as libc::c_int,
            b"MOD_LoadBmodel: funny lump size in %s\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            ((*loadmodel).name).as_mut_ptr(),
        );
    }
    count = ((*l).filelen as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<dleaf_t>() as libc::c_ulong) as libc::c_int;
    out = Hunk_Alloc(
        (count as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<mleaf_t>() as libc::c_ulong)
            as libc::c_int,
    ) as *mut mleaf_t;
    let ref mut fresh31 = (*loadmodel).leafs;
    *fresh31 = out;
    (*loadmodel).numleafs = count;
    i = 0 as libc::c_int;
    while i < count {
        j = 0 as libc::c_int;
        while j < 3 as libc::c_int {
            (*out)
                .minmaxs[j
                as usize] = LittleShort((*in_0).mins[j as usize]) as libc::c_float;
            (*out)
                .minmaxs[(3 as libc::c_int + j)
                as usize] = LittleShort((*in_0).maxs[j as usize]) as libc::c_float;
            j += 1;
        }
        p = LittleLong((*in_0).contents);
        (*out).contents = p;
        (*out).cluster = LittleShort((*in_0).cluster) as libc::c_int;
        (*out).area = LittleShort((*in_0).area) as libc::c_int;
        let ref mut fresh32 = (*out).firstmarksurface;
        *fresh32 = ((*loadmodel).marksurfaces)
            .offset(
                LittleShort((*in_0).firstleafface as libc::c_short) as libc::c_int
                    as isize,
            );
        (*out)
            .nummarksurfaces = LittleShort((*in_0).numleaffaces as libc::c_short)
            as libc::c_int;
        i += 1;
        in_0 = in_0.offset(1);
        out = out.offset(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn Mod_LoadMarksurfaces(mut l: *mut lump_t) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    let mut in_0: *mut libc::c_short = 0 as *mut libc::c_short;
    let mut out: *mut *mut msurface_t = 0 as *mut *mut msurface_t;
    in_0 = mod_base.offset((*l).fileofs as isize) as *mut libc::c_void
        as *mut libc::c_short;
    if ((*l).filelen as libc::c_ulong)
        .wrapping_rem(::std::mem::size_of::<libc::c_short>() as libc::c_ulong) != 0
    {
        (ri.Sys_Error)
            .expect(
                "non-null function pointer",
            )(
            1 as libc::c_int,
            b"MOD_LoadBmodel: funny lump size in %s\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            ((*loadmodel).name).as_mut_ptr(),
        );
    }
    count = ((*l).filelen as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<libc::c_short>() as libc::c_ulong)
        as libc::c_int;
    out = Hunk_Alloc(
        (count as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<*mut msurface_t>() as libc::c_ulong)
            as libc::c_int,
    ) as *mut *mut msurface_t;
    let ref mut fresh33 = (*loadmodel).marksurfaces;
    *fresh33 = out;
    (*loadmodel).nummarksurfaces = count;
    i = 0 as libc::c_int;
    while i < count {
        j = LittleShort(*in_0.offset(i as isize)) as libc::c_int;
        if j < 0 as libc::c_int || j >= (*loadmodel).numsurfaces {
            (ri.Sys_Error)
                .expect(
                    "non-null function pointer",
                )(
                1 as libc::c_int,
                b"Mod_ParseMarksurfaces: bad surface number\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
            );
        }
        let ref mut fresh34 = *out.offset(i as isize);
        *fresh34 = ((*loadmodel).surfaces).offset(j as isize);
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn Mod_LoadSurfedges(mut l: *mut lump_t) {
    let mut i: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    let mut in_0: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut out: *mut libc::c_int = 0 as *mut libc::c_int;
    in_0 = mod_base.offset((*l).fileofs as isize) as *mut libc::c_void
        as *mut libc::c_int;
    if ((*l).filelen as libc::c_ulong)
        .wrapping_rem(::std::mem::size_of::<libc::c_int>() as libc::c_ulong) != 0
    {
        (ri.Sys_Error)
            .expect(
                "non-null function pointer",
            )(
            1 as libc::c_int,
            b"MOD_LoadBmodel: funny lump size in %s\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            ((*loadmodel).name).as_mut_ptr(),
        );
    }
    count = ((*l).filelen as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
        as libc::c_int;
    if count < 1 as libc::c_int || count >= 256000 as libc::c_int {
        (ri.Sys_Error)
            .expect(
                "non-null function pointer",
            )(
            1 as libc::c_int,
            b"MOD_LoadBmodel: bad surfedges count in %s: %i\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            ((*loadmodel).name).as_mut_ptr(),
            count,
        );
    }
    out = Hunk_Alloc(
        (count as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            as libc::c_int,
    ) as *mut libc::c_int;
    let ref mut fresh35 = (*loadmodel).surfedges;
    *fresh35 = out;
    (*loadmodel).numsurfedges = count;
    i = 0 as libc::c_int;
    while i < count {
        *out.offset(i as isize) = LittleLong(*in_0.offset(i as isize));
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn Mod_LoadPlanes(mut l: *mut lump_t) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut out: *mut cplane_t = 0 as *mut cplane_t;
    let mut in_0: *mut dplane_t = 0 as *mut dplane_t;
    let mut count: libc::c_int = 0;
    let mut bits: libc::c_int = 0;
    in_0 = mod_base.offset((*l).fileofs as isize) as *mut libc::c_void as *mut dplane_t;
    if ((*l).filelen as libc::c_ulong)
        .wrapping_rem(::std::mem::size_of::<dplane_t>() as libc::c_ulong) != 0
    {
        (ri.Sys_Error)
            .expect(
                "non-null function pointer",
            )(
            1 as libc::c_int,
            b"MOD_LoadBmodel: funny lump size in %s\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            ((*loadmodel).name).as_mut_ptr(),
        );
    }
    count = ((*l).filelen as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<dplane_t>() as libc::c_ulong) as libc::c_int;
    out = Hunk_Alloc(
        ((count * 2 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<cplane_t>() as libc::c_ulong)
            as libc::c_int,
    ) as *mut cplane_t;
    let ref mut fresh36 = (*loadmodel).planes;
    *fresh36 = out;
    (*loadmodel).numplanes = count;
    i = 0 as libc::c_int;
    while i < count {
        bits = 0 as libc::c_int;
        j = 0 as libc::c_int;
        while j < 3 as libc::c_int {
            (*out).normal[j as usize] = LittleFloat((*in_0).normal[j as usize]);
            if (*out).normal[j as usize] < 0 as libc::c_int as libc::c_float {
                bits |= (1 as libc::c_int) << j;
            }
            j += 1;
        }
        (*out).dist = LittleFloat((*in_0).dist);
        (*out).type_0 = LittleLong((*in_0).type_0) as byte;
        (*out).signbits = bits as byte;
        i += 1;
        in_0 = in_0.offset(1);
        out = out.offset(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn Mod_LoadBrushModel(
    mut mod_0: *mut model_t,
    mut buffer: *mut libc::c_void,
) {
    let mut i: libc::c_int = 0;
    let mut header: *mut dheader_t = 0 as *mut dheader_t;
    let mut bm: *mut mmodel_t = 0 as *mut mmodel_t;
    (*loadmodel).type_0 = mod_brush;
    if loadmodel != mod_known.as_mut_ptr() {
        (ri.Sys_Error)
            .expect(
                "non-null function pointer",
            )(
            1 as libc::c_int,
            b"Loaded a brush model after the world\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    header = buffer as *mut dheader_t;
    i = LittleLong((*header).version);
    if i != 38 as libc::c_int {
        (ri.Sys_Error)
            .expect(
                "non-null function pointer",
            )(
            1 as libc::c_int,
            b"Mod_LoadBrushModel: %s has wrong version number (%i should be %i)\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            ((*mod_0).name).as_mut_ptr(),
            i,
            38 as libc::c_int,
        );
    }
    mod_base = header as *mut byte;
    i = 0 as libc::c_int;
    while (i as libc::c_ulong)
        < (::std::mem::size_of::<dheader_t>() as libc::c_ulong)
            .wrapping_div(4 as libc::c_int as libc::c_ulong)
    {
        *(header as *mut libc::c_int)
            .offset(
                i as isize,
            ) = LittleLong(*(header as *mut libc::c_int).offset(i as isize));
        i += 1;
    }
    Mod_LoadVertexes(
        &mut *((*header).lumps).as_mut_ptr().offset(2 as libc::c_int as isize),
    );
    Mod_LoadEdges(
        &mut *((*header).lumps).as_mut_ptr().offset(11 as libc::c_int as isize),
    );
    Mod_LoadSurfedges(
        &mut *((*header).lumps).as_mut_ptr().offset(12 as libc::c_int as isize),
    );
    Mod_LoadLighting(
        &mut *((*header).lumps).as_mut_ptr().offset(7 as libc::c_int as isize),
    );
    Mod_LoadPlanes(
        &mut *((*header).lumps).as_mut_ptr().offset(1 as libc::c_int as isize),
    );
    Mod_LoadTexinfo(
        &mut *((*header).lumps).as_mut_ptr().offset(5 as libc::c_int as isize),
    );
    Mod_LoadFaces(
        &mut *((*header).lumps).as_mut_ptr().offset(6 as libc::c_int as isize),
    );
    Mod_LoadMarksurfaces(
        &mut *((*header).lumps).as_mut_ptr().offset(9 as libc::c_int as isize),
    );
    Mod_LoadVisibility(
        &mut *((*header).lumps).as_mut_ptr().offset(3 as libc::c_int as isize),
    );
    Mod_LoadLeafs(
        &mut *((*header).lumps).as_mut_ptr().offset(8 as libc::c_int as isize),
    );
    Mod_LoadNodes(
        &mut *((*header).lumps).as_mut_ptr().offset(4 as libc::c_int as isize),
    );
    Mod_LoadSubmodels(
        &mut *((*header).lumps).as_mut_ptr().offset(13 as libc::c_int as isize),
    );
    (*mod_0).numframes = 2 as libc::c_int;
    i = 0 as libc::c_int;
    while i < (*mod_0).numsubmodels {
        let mut starmod: *mut model_t = 0 as *mut model_t;
        bm = &mut *((*mod_0).submodels).offset(i as isize) as *mut mmodel_t;
        starmod = &mut *mod_inline.as_mut_ptr().offset(i as isize) as *mut model_t;
        *starmod = *loadmodel;
        (*starmod).firstmodelsurface = (*bm).firstface;
        (*starmod).nummodelsurfaces = (*bm).numfaces;
        (*starmod).firstnode = (*bm).headnode;
        if (*starmod).firstnode >= (*loadmodel).numnodes {
            (ri.Sys_Error)
                .expect(
                    "non-null function pointer",
                )(
                1 as libc::c_int,
                b"Inline model %i has bad firstnode\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                i,
            );
        }
        (*starmod)
            .maxs[0 as libc::c_int as usize] = (*bm).maxs[0 as libc::c_int as usize];
        (*starmod)
            .maxs[1 as libc::c_int as usize] = (*bm).maxs[1 as libc::c_int as usize];
        (*starmod)
            .maxs[2 as libc::c_int as usize] = (*bm).maxs[2 as libc::c_int as usize];
        (*starmod)
            .mins[0 as libc::c_int as usize] = (*bm).mins[0 as libc::c_int as usize];
        (*starmod)
            .mins[1 as libc::c_int as usize] = (*bm).mins[1 as libc::c_int as usize];
        (*starmod)
            .mins[2 as libc::c_int as usize] = (*bm).mins[2 as libc::c_int as usize];
        (*starmod).radius = (*bm).radius;
        if i == 0 as libc::c_int {
            *loadmodel = *starmod;
        }
        (*starmod).numleafs = (*bm).visleafs;
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn Mod_LoadAliasModel(
    mut mod_0: *mut model_t,
    mut buffer: *mut libc::c_void,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut pinmodel: *mut dmdl_t = 0 as *mut dmdl_t;
    let mut pheader: *mut dmdl_t = 0 as *mut dmdl_t;
    let mut pinst: *mut dstvert_t = 0 as *mut dstvert_t;
    let mut poutst: *mut dstvert_t = 0 as *mut dstvert_t;
    let mut pintri: *mut dtriangle_t = 0 as *mut dtriangle_t;
    let mut pouttri: *mut dtriangle_t = 0 as *mut dtriangle_t;
    let mut pinframe: *mut daliasframe_t = 0 as *mut daliasframe_t;
    let mut poutframe: *mut daliasframe_t = 0 as *mut daliasframe_t;
    let mut pincmd: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut poutcmd: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut version: libc::c_int = 0;
    pinmodel = buffer as *mut dmdl_t;
    version = LittleLong((*pinmodel).version);
    if version != 8 as libc::c_int {
        (ri.Sys_Error)
            .expect(
                "non-null function pointer",
            )(
            1 as libc::c_int,
            b"%s has wrong version number (%i should be %i)\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            ((*mod_0).name).as_mut_ptr(),
            version,
            8 as libc::c_int,
        );
    }
    pheader = Hunk_Alloc(LittleLong((*pinmodel).ofs_end)) as *mut dmdl_t;
    i = 0 as libc::c_int;
    while (i as libc::c_ulong)
        < (::std::mem::size_of::<dmdl_t>() as libc::c_ulong)
            .wrapping_div(4 as libc::c_int as libc::c_ulong)
    {
        *(pheader as *mut libc::c_int)
            .offset(
                i as isize,
            ) = LittleLong(*(buffer as *mut libc::c_int).offset(i as isize));
        i += 1;
    }
    if (*pheader).skinheight > 480 as libc::c_int {
        (ri.Sys_Error)
            .expect(
                "non-null function pointer",
            )(
            1 as libc::c_int,
            b"model %s has a skin taller than %d\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            ((*mod_0).name).as_mut_ptr(),
            480 as libc::c_int,
        );
    }
    if (*pheader).num_xyz <= 0 as libc::c_int {
        (ri.Sys_Error)
            .expect(
                "non-null function pointer",
            )(
            1 as libc::c_int,
            b"model %s has no vertices\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            ((*mod_0).name).as_mut_ptr(),
        );
    }
    if (*pheader).num_xyz > 2048 as libc::c_int {
        (ri.Sys_Error)
            .expect(
                "non-null function pointer",
            )(
            1 as libc::c_int,
            b"model %s has too many vertices\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            ((*mod_0).name).as_mut_ptr(),
        );
    }
    if (*pheader).num_st <= 0 as libc::c_int {
        (ri.Sys_Error)
            .expect(
                "non-null function pointer",
            )(
            1 as libc::c_int,
            b"model %s has no st vertices\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            ((*mod_0).name).as_mut_ptr(),
        );
    }
    if (*pheader).num_tris <= 0 as libc::c_int {
        (ri.Sys_Error)
            .expect(
                "non-null function pointer",
            )(
            1 as libc::c_int,
            b"model %s has no triangles\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            ((*mod_0).name).as_mut_ptr(),
        );
    }
    if (*pheader).num_frames <= 0 as libc::c_int {
        (ri.Sys_Error)
            .expect(
                "non-null function pointer",
            )(
            1 as libc::c_int,
            b"model %s has no frames\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            ((*mod_0).name).as_mut_ptr(),
        );
    }
    pinst = (pinmodel as *mut byte).offset((*pheader).ofs_st as isize) as *mut dstvert_t;
    poutst = (pheader as *mut byte).offset((*pheader).ofs_st as isize) as *mut dstvert_t;
    i = 0 as libc::c_int;
    while i < (*pheader).num_st {
        (*poutst.offset(i as isize)).s = LittleShort((*pinst.offset(i as isize)).s);
        (*poutst.offset(i as isize)).t = LittleShort((*pinst.offset(i as isize)).t);
        i += 1;
    }
    pintri = (pinmodel as *mut byte).offset((*pheader).ofs_tris as isize)
        as *mut dtriangle_t;
    pouttri = (pheader as *mut byte).offset((*pheader).ofs_tris as isize)
        as *mut dtriangle_t;
    i = 0 as libc::c_int;
    while i < (*pheader).num_tris {
        j = 0 as libc::c_int;
        while j < 3 as libc::c_int {
            (*pouttri.offset(i as isize))
                .index_xyz[j
                as usize] = LittleShort(
                (*pintri.offset(i as isize)).index_xyz[j as usize],
            );
            (*pouttri.offset(i as isize))
                .index_st[j
                as usize] = LittleShort(
                (*pintri.offset(i as isize)).index_st[j as usize],
            );
            j += 1;
        }
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < (*pheader).num_frames {
        pinframe = (pinmodel as *mut byte)
            .offset((*pheader).ofs_frames as isize)
            .offset((i * (*pheader).framesize) as isize) as *mut daliasframe_t;
        poutframe = (pheader as *mut byte)
            .offset((*pheader).ofs_frames as isize)
            .offset((i * (*pheader).framesize) as isize) as *mut daliasframe_t;
        memcpy(
            ((*poutframe).name).as_mut_ptr() as *mut libc::c_void,
            ((*pinframe).name).as_mut_ptr() as *const libc::c_void,
            ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong,
        );
        j = 0 as libc::c_int;
        while j < 3 as libc::c_int {
            (*poutframe).scale[j as usize] = LittleFloat((*pinframe).scale[j as usize]);
            (*poutframe)
                .translate[j as usize] = LittleFloat((*pinframe).translate[j as usize]);
            j += 1;
        }
        memcpy(
            ((*poutframe).verts).as_mut_ptr() as *mut libc::c_void,
            ((*pinframe).verts).as_mut_ptr() as *const libc::c_void,
            ((*pheader).num_xyz as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<dtrivertx_t>() as libc::c_ulong),
        );
        i += 1;
    }
    (*mod_0).type_0 = mod_alias;
    pincmd = (pinmodel as *mut byte).offset((*pheader).ofs_glcmds as isize)
        as *mut libc::c_int;
    poutcmd = (pheader as *mut byte).offset((*pheader).ofs_glcmds as isize)
        as *mut libc::c_int;
    i = 0 as libc::c_int;
    while i < (*pheader).num_glcmds {
        *poutcmd.offset(i as isize) = LittleLong(*pincmd.offset(i as isize));
        i += 1;
    }
    memcpy(
        (pheader as *mut libc::c_char).offset((*pheader).ofs_skins as isize)
            as *mut libc::c_void,
        (pinmodel as *mut libc::c_char).offset((*pheader).ofs_skins as isize)
            as *const libc::c_void,
        ((*pheader).num_skins * 64 as libc::c_int) as libc::c_ulong,
    );
    i = 0 as libc::c_int;
    while i < (*pheader).num_skins {
        let ref mut fresh37 = (*mod_0).skins[i as usize];
        *fresh37 = GL_FindImage(
            (pheader as *mut libc::c_char)
                .offset((*pheader).ofs_skins as isize)
                .offset((i * 64 as libc::c_int) as isize),
            it_skin,
        );
        i += 1;
    }
    (*mod_0).mins[0 as libc::c_int as usize] = -(32 as libc::c_int) as vec_t;
    (*mod_0).mins[1 as libc::c_int as usize] = -(32 as libc::c_int) as vec_t;
    (*mod_0).mins[2 as libc::c_int as usize] = -(32 as libc::c_int) as vec_t;
    (*mod_0).maxs[0 as libc::c_int as usize] = 32 as libc::c_int as vec_t;
    (*mod_0).maxs[1 as libc::c_int as usize] = 32 as libc::c_int as vec_t;
    (*mod_0).maxs[2 as libc::c_int as usize] = 32 as libc::c_int as vec_t;
}
#[no_mangle]
pub unsafe extern "C" fn Mod_LoadSpriteModel(
    mut mod_0: *mut model_t,
    mut buffer: *mut libc::c_void,
) {
    let mut sprin: *mut dsprite_t = 0 as *mut dsprite_t;
    let mut sprout: *mut dsprite_t = 0 as *mut dsprite_t;
    let mut i: libc::c_int = 0;
    sprin = buffer as *mut dsprite_t;
    sprout = Hunk_Alloc(modfilelen) as *mut dsprite_t;
    (*sprout).ident = LittleLong((*sprin).ident);
    (*sprout).version = LittleLong((*sprin).version);
    (*sprout).numframes = LittleLong((*sprin).numframes);
    if (*sprout).version != 2 as libc::c_int {
        (ri.Sys_Error)
            .expect(
                "non-null function pointer",
            )(
            1 as libc::c_int,
            b"%s has wrong version number (%i should be %i)\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            ((*mod_0).name).as_mut_ptr(),
            (*sprout).version,
            2 as libc::c_int,
        );
    }
    if (*sprout).numframes > 32 as libc::c_int {
        (ri.Sys_Error)
            .expect(
                "non-null function pointer",
            )(
            1 as libc::c_int,
            b"%s has too many frames (%i > %i)\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            ((*mod_0).name).as_mut_ptr(),
            (*sprout).numframes,
            32 as libc::c_int,
        );
    }
    i = 0 as libc::c_int;
    while i < (*sprout).numframes {
        (*((*sprout).frames).as_mut_ptr().offset(i as isize))
            .width = LittleLong(
            (*((*sprin).frames).as_mut_ptr().offset(i as isize)).width,
        );
        (*((*sprout).frames).as_mut_ptr().offset(i as isize))
            .height = LittleLong(
            (*((*sprin).frames).as_mut_ptr().offset(i as isize)).height,
        );
        (*((*sprout).frames).as_mut_ptr().offset(i as isize))
            .origin_x = LittleLong(
            (*((*sprin).frames).as_mut_ptr().offset(i as isize)).origin_x,
        );
        (*((*sprout).frames).as_mut_ptr().offset(i as isize))
            .origin_y = LittleLong(
            (*((*sprin).frames).as_mut_ptr().offset(i as isize)).origin_y,
        );
        memcpy(
            ((*((*sprout).frames).as_mut_ptr().offset(i as isize)).name).as_mut_ptr()
                as *mut libc::c_void,
            ((*((*sprin).frames).as_mut_ptr().offset(i as isize)).name).as_mut_ptr()
                as *const libc::c_void,
            64 as libc::c_int as libc::c_ulong,
        );
        let ref mut fresh38 = (*mod_0).skins[i as usize];
        *fresh38 = GL_FindImage(
            ((*((*sprout).frames).as_mut_ptr().offset(i as isize)).name).as_mut_ptr(),
            it_sprite,
        );
        i += 1;
    }
    (*mod_0).type_0 = mod_sprite;
}
#[no_mangle]
pub unsafe extern "C" fn R_BeginRegistration(mut model: *mut libc::c_char) {
    let mut fullname: [libc::c_char; 64] = [0; 64];
    let mut flushmap: *mut cvar_t = 0 as *mut cvar_t;
    registration_sequence += 1;
    r_oldviewcluster = -(1 as libc::c_int);
    Com_sprintf(
        fullname.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
        b"maps/%s.bsp\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        model,
    );
    flushmap = (ri.Cvar_Get)
        .expect(
            "non-null function pointer",
        )(
        b"flushmap\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    if strcmp(
        (mod_known[0 as libc::c_int as usize].name).as_mut_ptr(),
        fullname.as_mut_ptr(),
    ) != 0 || (*flushmap).value != 0.
    {
        Mod_Free(&mut *mod_known.as_mut_ptr().offset(0 as libc::c_int as isize));
    }
    r_worldmodel = Mod_ForName(fullname.as_mut_ptr(), true_0);
    r_viewcluster = -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn R_RegisterModel(mut name: *mut libc::c_char) -> *mut model_s {
    let mut mod_0: *mut model_t = 0 as *mut model_t;
    let mut i: libc::c_int = 0;
    let mut sprout: *mut dsprite_t = 0 as *mut dsprite_t;
    let mut pheader: *mut dmdl_t = 0 as *mut dmdl_t;
    mod_0 = Mod_ForName(name, false_0);
    if !mod_0.is_null() {
        (*mod_0).registration_sequence = registration_sequence;
        if (*mod_0).type_0 as libc::c_uint == mod_sprite as libc::c_int as libc::c_uint {
            sprout = (*mod_0).extradata as *mut dsprite_t;
            i = 0 as libc::c_int;
            while i < (*sprout).numframes {
                let ref mut fresh39 = (*mod_0).skins[i as usize];
                *fresh39 = GL_FindImage(
                    ((*((*sprout).frames).as_mut_ptr().offset(i as isize)).name)
                        .as_mut_ptr(),
                    it_sprite,
                );
                i += 1;
            }
        } else if (*mod_0).type_0 as libc::c_uint
            == mod_alias as libc::c_int as libc::c_uint
        {
            pheader = (*mod_0).extradata as *mut dmdl_t;
            i = 0 as libc::c_int;
            while i < (*pheader).num_skins {
                let ref mut fresh40 = (*mod_0).skins[i as usize];
                *fresh40 = GL_FindImage(
                    (pheader as *mut libc::c_char)
                        .offset((*pheader).ofs_skins as isize)
                        .offset((i * 64 as libc::c_int) as isize),
                    it_skin,
                );
                i += 1;
            }
            (*mod_0).numframes = (*pheader).num_frames;
        } else if (*mod_0).type_0 as libc::c_uint
            == mod_brush as libc::c_int as libc::c_uint
        {
            i = 0 as libc::c_int;
            while i < (*mod_0).numtexinfo {
                (*(*((*mod_0).texinfo).offset(i as isize)).image)
                    .registration_sequence = registration_sequence;
                i += 1;
            }
        }
    }
    return mod_0;
}
#[no_mangle]
pub unsafe extern "C" fn R_EndRegistration() {
    let mut i: libc::c_int = 0;
    let mut mod_0: *mut model_t = 0 as *mut model_t;
    i = 0 as libc::c_int;
    mod_0 = mod_known.as_mut_ptr();
    while i < mod_numknown {
        if !((*mod_0).name[0 as libc::c_int as usize] == 0) {
            if (*mod_0).registration_sequence != registration_sequence {
                Mod_Free(mod_0);
            }
        }
        i += 1;
        mod_0 = mod_0.offset(1);
    }
    GL_FreeUnusedImages();
}
#[no_mangle]
pub unsafe extern "C" fn Mod_Free(mut mod_0: *mut model_t) {
    Hunk_Free((*mod_0).extradata);
    memset(
        mod_0 as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<model_t>() as libc::c_ulong,
    );
}
#[no_mangle]
pub unsafe extern "C" fn Mod_FreeAll() {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < mod_numknown {
        if mod_known[i as usize].extradatasize != 0 {
            Mod_Free(&mut *mod_known.as_mut_ptr().offset(i as isize));
        }
        i += 1;
    }
}
