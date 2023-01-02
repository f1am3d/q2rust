#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(c_variadic, register_tool)]
extern "C" {
    fn sin(_: libc::c_double) -> libc::c_double;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn SWimp_AppActivate(active: qboolean);
    fn R_FindImage(name: *mut libc::c_char, type_0: imagetype_t) -> *mut image_t;
    fn SWimp_EndFrame();
    fn Draw_FadeScreen();
    fn Draw_Fill(
        x: libc::c_int,
        y: libc::c_int,
        w: libc::c_int,
        h: libc::c_int,
        c: libc::c_int,
    );
    fn Draw_TileClear(
        x: libc::c_int,
        y: libc::c_int,
        w: libc::c_int,
        h: libc::c_int,
        name: *mut libc::c_char,
    );
    fn Draw_Char(x: libc::c_int, y: libc::c_int, c: libc::c_int);
    fn Draw_StretchRaw(
        x: libc::c_int,
        y: libc::c_int,
        w: libc::c_int,
        h: libc::c_int,
        cols: libc::c_int,
        rows: libc::c_int,
        data: *mut byte,
    );
    fn Draw_StretchPic(
        x: libc::c_int,
        y: libc::c_int,
        w: libc::c_int,
        h: libc::c_int,
        name: *mut libc::c_char,
    );
    fn Draw_Pic(x: libc::c_int, y: libc::c_int, name: *mut libc::c_char);
    fn Draw_GetPicSize(
        w: *mut libc::c_int,
        h: *mut libc::c_int,
        name: *mut libc::c_char,
    );
    fn Draw_FindPic(name: *mut libc::c_char) -> *mut image_s;
    fn R_EndRegistration();
    fn R_RegisterModel(name: *mut libc::c_char) -> *mut model_s;
    fn R_BeginRegistration(map: *mut libc::c_char);
    fn R_ShutdownImages();
    fn SWimp_Shutdown();
    fn R_InitImages();
    fn LoadPCX(
        filename: *mut libc::c_char,
        pic: *mut *mut byte,
        palette: *mut *mut byte,
        width: *mut libc::c_int,
        height: *mut libc::c_int,
    );
    fn SWimp_Init(
        hInstance: *mut libc::c_void,
        wndProc: *mut libc::c_void,
    ) -> libc::c_int;
    fn SWimp_SetMode(
        pwidth: *mut libc::c_int,
        pheight: *mut libc::c_int,
        mode: libc::c_int,
        fullscreen: qboolean,
    ) -> rserr_t;
    fn D_FlushCaches();
    fn R_InitCaches();
    fn SWimp_SetPalette(palette: *const libc::c_uchar);
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn vsprintf(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: ::std::ffi::VaList,
    ) -> libc::c_int;
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    static mut vec3_origin: vec3_t;
    fn VectorMA(
        veca: *mut vec_t,
        scale: libc::c_float,
        vecb: *mut vec_t,
        vecc: *mut vec_t,
    );
    fn VectorNormalize(v: *mut vec_t) -> vec_t;
    fn VectorScale(in_0: *mut vec_t, scale: vec_t, out: *mut vec_t);
    fn AngleVectors(
        angles: *mut vec_t,
        forward: *mut vec_t,
        right: *mut vec_t,
        up: *mut vec_t,
    );
    fn BoxOnPlaneSide(
        emins: *mut vec_t,
        emaxs: *mut vec_t,
        plane: *mut cplane_s,
    ) -> libc::c_int;
    fn PerpendicularVector(dst: *mut vec_t, src: *const vec_t);
    fn RotatePointAroundVector(
        dst: *mut vec_t,
        dir: *const vec_t,
        point: *const vec_t,
        degrees: libc::c_float,
    );
    fn Com_sprintf(
        dest: *mut libc::c_char,
        size: libc::c_int,
        fmt: *mut libc::c_char,
        _: ...
    );
    fn Swap_Init();
    fn Sys_Milliseconds() -> libc::c_int;
    fn Draw_InitLocal();
    fn R_ScreenShot_f();
    fn Mod_Init();
    fn Mod_ClusterPVS(cluster: libc::c_int, model: *mut model_t) -> *mut byte;
    fn Mod_Modellist_f();
    fn Mod_FreeAll();
    fn D_WarpScreen();
    static mut sc_base: *mut surfcache_t;
    fn R_SetupFrame();
    fn R_LightPoint(p: *mut vec_t, color: *mut vec_t);
    fn R_PrintDSpeeds();
    fn R_PrintTimes();
    fn R_PrintAliasStats();
    static mut r_dlightframecount: libc::c_int;
    static mut sintable: [libc::c_int; 1280];
    static mut intsintable: [libc::c_int; 1280];
    static mut blanktable: [libc::c_int; 1280];
    static mut surfaces: *mut surf_t;
    static mut surface_p: *mut surf_t;
    static mut surf_max: *mut surf_t;
    static mut r_edges: *mut edge_t;
    static mut auxedges: *mut edge_t;
    fn R_SurfacePatch();
    fn R_DrawParticles();
    static mut sw_mipcap: *mut cvar_t;
    static mut sw_mipscale: *mut cvar_t;
    fn R_RotateBmodel();
    fn R_PushDlights(model: *mut model_t);
    fn R_ScanEdges();
    fn R_BeginEdgeFrame();
    fn R_AliasDrawModel();
    static mut view_clipplanes: [clipplane_t; 4];
    fn R_RenderWorld();
    fn R_DrawSolidClippedSubmodelPolygons(pmodel: *mut model_t, topnode: *mut mnode_t);
    fn R_DrawSubmodelPolygons(
        pmodel: *mut model_t,
        clipflags: libc::c_int,
        topnode: *mut mnode_t,
    );
    static mut currententity: *mut entity_t;
    static mut modelorg: vec3_t;
    static mut r_entorigin: vec3_t;
    static mut insubmodel: qboolean;
    fn R_DrawAlphaSurfaces();
    fn R_DrawSprite();
    fn R_TransformFrustum();
    fn R_IMFlatShadedQuad(
        a: *mut vec_t,
        b: *mut vec_t,
        c: *mut vec_t,
        d: *mut vec_t,
        color: libc::c_int,
        alpha: libc::c_float,
    );
    fn free(__ptr: *mut libc::c_void);
    fn abs(_: libc::c_int) -> libc::c_int;
    fn R_ImageList_f();
    static mut r_skytexinfo: [mtexinfo_t; 6];
    fn R_RegisterSkin(name: *mut libc::c_char) -> *mut image_s;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type va_list = __builtin_va_list;
pub type byte = libc::c_uchar;
pub type qboolean = libc::c_uint;
pub const true_0: qboolean = 1;
pub const false_0: qboolean = 0;
pub type vec_t = libc::c_float;
pub type vec3_t = [vec_t; 3];
pub type fixed16_t = libc::c_int;
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
pub type cvar_t = cvar_s;
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
pub type cplane_t = cplane_s;
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
pub struct refexport_t {
    pub api_version: libc::c_int,
    pub Init: Option::<
        unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> qboolean,
    >,
    pub Shutdown: Option::<unsafe extern "C" fn() -> ()>,
    pub BeginRegistration: Option::<unsafe extern "C" fn(*mut libc::c_char) -> ()>,
    pub RegisterModel: Option::<unsafe extern "C" fn(*mut libc::c_char) -> *mut model_s>,
    pub RegisterSkin: Option::<unsafe extern "C" fn(*mut libc::c_char) -> *mut image_s>,
    pub RegisterPic: Option::<unsafe extern "C" fn(*mut libc::c_char) -> *mut image_s>,
    pub SetSky: Option::<
        unsafe extern "C" fn(*mut libc::c_char, libc::c_float, *mut vec_t) -> (),
    >,
    pub EndRegistration: Option::<unsafe extern "C" fn() -> ()>,
    pub RenderFrame: Option::<unsafe extern "C" fn(*mut refdef_t) -> ()>,
    pub DrawGetPicSize: Option::<
        unsafe extern "C" fn(*mut libc::c_int, *mut libc::c_int, *mut libc::c_char) -> (),
    >,
    pub DrawPic: Option::<
        unsafe extern "C" fn(libc::c_int, libc::c_int, *mut libc::c_char) -> (),
    >,
    pub DrawStretchPic: Option::<
        unsafe extern "C" fn(
            libc::c_int,
            libc::c_int,
            libc::c_int,
            libc::c_int,
            *mut libc::c_char,
        ) -> (),
    >,
    pub DrawChar: Option::<
        unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int) -> (),
    >,
    pub DrawTileClear: Option::<
        unsafe extern "C" fn(
            libc::c_int,
            libc::c_int,
            libc::c_int,
            libc::c_int,
            *mut libc::c_char,
        ) -> (),
    >,
    pub DrawFill: Option::<
        unsafe extern "C" fn(
            libc::c_int,
            libc::c_int,
            libc::c_int,
            libc::c_int,
            libc::c_int,
        ) -> (),
    >,
    pub DrawFadeScreen: Option::<unsafe extern "C" fn() -> ()>,
    pub DrawStretchRaw: Option::<
        unsafe extern "C" fn(
            libc::c_int,
            libc::c_int,
            libc::c_int,
            libc::c_int,
            libc::c_int,
            libc::c_int,
            *mut byte,
        ) -> (),
    >,
    pub CinematicSetPalette: Option::<unsafe extern "C" fn(*const libc::c_uchar) -> ()>,
    pub BeginFrame: Option::<unsafe extern "C" fn(libc::c_float) -> ()>,
    pub EndFrame: Option::<unsafe extern "C" fn() -> ()>,
    pub AppActivate: Option::<unsafe extern "C" fn(qboolean) -> ()>,
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
pub type rserr_t = libc::c_uint;
pub const rserr_unknown: rserr_t = 3;
pub const rserr_invalid_mode: rserr_t = 2;
pub const rserr_invalid_fullscreen: rserr_t = 1;
pub const rserr_ok: rserr_t = 0;
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
pub struct alight_t {
    pub ambientlight: libc::c_int,
    pub shadelight: libc::c_int,
    pub plightvec: *mut libc::c_float,
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
pub type swstate_t = swstate_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct swstate_s {
    pub fullscreen: qboolean,
    pub prev_mode: libc::c_int,
    pub gammatable: [byte; 256],
    pub currentpalette: [byte; 1024],
}
#[no_mangle]
pub static mut vid: viddef_t = viddef_t {
    buffer: 0 as *const pixel_t as *mut pixel_t,
    colormap: 0 as *const pixel_t as *mut pixel_t,
    alphamap: 0 as *const pixel_t as *mut pixel_t,
    rowbytes: 0,
    width: 0,
    height: 0,
};
#[no_mangle]
pub static mut ri: refimport_t = refimport_t {
    Sys_Error: None,
    Cmd_AddCommand: None,
    Cmd_RemoveCommand: None,
    Cmd_Argc: None,
    Cmd_Argv: None,
    Cmd_ExecuteText: None,
    Con_Printf: None,
    FS_LoadFile: None,
    FS_FreeFile: None,
    FS_Gamedir: None,
    Cvar_Get: None,
    Cvar_Set: None,
    Cvar_SetValue: None,
    Vid_GetModeInfo: None,
    Vid_MenuInit: None,
    Vid_NewWindow: None,
};
#[no_mangle]
pub static mut d_8to24table: [libc::c_uint; 256] = [0; 256];
#[no_mangle]
pub static mut r_worldentity: entity_t = entity_t {
    model: 0 as *const model_s as *mut model_s,
    angles: [0.; 3],
    origin: [0.; 3],
    frame: 0,
    oldorigin: [0.; 3],
    oldframe: 0,
    backlerp: 0.,
    skinnum: 0,
    lightstyle: 0,
    alpha: 0.,
    skin: 0 as *const image_s as *mut image_s,
    flags: 0,
};
#[no_mangle]
pub static mut skyname: [libc::c_char; 64] = [0; 64];
#[no_mangle]
pub static mut skyrotate: libc::c_float = 0.;
#[no_mangle]
pub static mut skyaxis: vec3_t = [0.; 3];
#[no_mangle]
pub static mut sky_images: [*mut image_t; 6] = [0 as *const image_t as *mut image_t; 6];
#[no_mangle]
pub static mut r_newrefdef: refdef_t = refdef_t {
    x: 0,
    y: 0,
    width: 0,
    height: 0,
    fov_x: 0.,
    fov_y: 0.,
    vieworg: [0.; 3],
    viewangles: [0.; 3],
    blend: [0.; 4],
    time: 0.,
    rdflags: 0,
    areabits: 0 as *const byte as *mut byte,
    lightstyles: 0 as *const lightstyle_t as *mut lightstyle_t,
    num_entities: 0,
    entities: 0 as *const entity_t as *mut entity_t,
    num_dlights: 0,
    dlights: 0 as *const dlight_t as *mut dlight_t,
    num_particles: 0,
    particles: 0 as *const particle_t as *mut particle_t,
};
#[no_mangle]
pub static mut currentmodel: *mut model_t = 0 as *const model_t as *mut model_t;
#[no_mangle]
pub static mut r_worldmodel: *mut model_t = 0 as *const model_t as *mut model_t;
#[no_mangle]
pub static mut r_warpbuffer: [byte; 76800] = [0; 76800];
#[no_mangle]
pub static mut sw_state: swstate_t = swstate_t {
    fullscreen: false_0,
    prev_mode: 0,
    gammatable: [0; 256],
    currentpalette: [0; 1024],
};
#[no_mangle]
pub static mut colormap: *mut libc::c_void = 0 as *const libc::c_void
    as *mut libc::c_void;
#[no_mangle]
pub static mut viewlightvec: vec3_t = [0.; 3];
#[no_mangle]
pub static mut r_viewlighting: alight_t = unsafe {
    {
        let mut init = alight_t {
            ambientlight: 128 as libc::c_int,
            shadelight: 192 as libc::c_int,
            plightvec: viewlightvec.as_ptr() as *mut _,
        };
        init
    }
};
#[no_mangle]
pub static mut r_time1: libc::c_float = 0.;
#[no_mangle]
pub static mut r_numallocatededges: libc::c_int = 0;
#[no_mangle]
pub static mut r_aliasuvscale: libc::c_float = 1.0f64 as libc::c_float;
#[no_mangle]
pub static mut r_outofsurfaces: libc::c_int = 0;
#[no_mangle]
pub static mut r_outofedges: libc::c_int = 0;
#[no_mangle]
pub static mut r_dowarp: qboolean = false_0;
#[no_mangle]
pub static mut r_pcurrentvertbase: *mut mvertex_t = 0 as *const mvertex_t
    as *mut mvertex_t;
#[no_mangle]
pub static mut c_surf: libc::c_int = 0;
#[no_mangle]
pub static mut r_maxsurfsseen: libc::c_int = 0;
#[no_mangle]
pub static mut r_maxedgesseen: libc::c_int = 0;
#[no_mangle]
pub static mut r_cnumsurfs: libc::c_int = 0;
#[no_mangle]
pub static mut r_surfsonstack: qboolean = false_0;
#[no_mangle]
pub static mut r_clipflags: libc::c_int = 0;
#[no_mangle]
pub static mut vup: vec3_t = [0.; 3];
#[no_mangle]
pub static mut base_vup: vec3_t = [0.; 3];
#[no_mangle]
pub static mut vpn: vec3_t = [0.; 3];
#[no_mangle]
pub static mut base_vpn: vec3_t = [0.; 3];
#[no_mangle]
pub static mut vright: vec3_t = [0.; 3];
#[no_mangle]
pub static mut base_vright: vec3_t = [0.; 3];
#[no_mangle]
pub static mut r_origin: vec3_t = [0.; 3];
#[no_mangle]
pub static mut r_refdef: oldrefdef_t = oldrefdef_t {
    vrect: vrect_t {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
        pnext: 0 as *const vrect_s as *mut vrect_s,
    },
    aliasvrect: vrect_t {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
        pnext: 0 as *const vrect_s as *mut vrect_s,
    },
    vrectright: 0,
    vrectbottom: 0,
    aliasvrectright: 0,
    aliasvrectbottom: 0,
    vrectrightedge: 0.,
    fvrectx: 0.,
    fvrecty: 0.,
    fvrectx_adj: 0.,
    fvrecty_adj: 0.,
    vrect_x_adj_shift20: 0,
    vrectright_adj_shift20: 0,
    fvrectright_adj: 0.,
    fvrectbottom_adj: 0.,
    fvrectright: 0.,
    fvrectbottom: 0.,
    horizontalFieldOfView: 0.,
    xOrigin: 0.,
    yOrigin: 0.,
    vieworg: [0.; 3],
    viewangles: [0.; 3],
    ambientlight: 0,
};
#[no_mangle]
pub static mut xcenter: libc::c_float = 0.;
#[no_mangle]
pub static mut ycenter: libc::c_float = 0.;
#[no_mangle]
pub static mut xscale: libc::c_float = 0.;
#[no_mangle]
pub static mut yscale: libc::c_float = 0.;
#[no_mangle]
pub static mut xscaleinv: libc::c_float = 0.;
#[no_mangle]
pub static mut yscaleinv: libc::c_float = 0.;
#[no_mangle]
pub static mut xscaleshrink: libc::c_float = 0.;
#[no_mangle]
pub static mut yscaleshrink: libc::c_float = 0.;
#[no_mangle]
pub static mut aliasxscale: libc::c_float = 0.;
#[no_mangle]
pub static mut aliasyscale: libc::c_float = 0.;
#[no_mangle]
pub static mut aliasxcenter: libc::c_float = 0.;
#[no_mangle]
pub static mut aliasycenter: libc::c_float = 0.;
#[no_mangle]
pub static mut r_screenwidth: libc::c_int = 0;
#[no_mangle]
pub static mut verticalFieldOfView: libc::c_float = 0.;
#[no_mangle]
pub static mut xOrigin: libc::c_float = 0.;
#[no_mangle]
pub static mut yOrigin: libc::c_float = 0.;
#[no_mangle]
pub static mut screenedge: [mplane_t; 4] = [mplane_t {
    normal: [0.; 3],
    dist: 0.,
    type_0: 0,
    signbits: 0,
    pad: [0; 2],
}; 4];
#[no_mangle]
pub static mut r_framecount: libc::c_int = 1 as libc::c_int;
#[no_mangle]
pub static mut r_visframecount: libc::c_int = 0;
#[no_mangle]
pub static mut d_spanpixcount: libc::c_int = 0;
#[no_mangle]
pub static mut r_polycount: libc::c_int = 0;
#[no_mangle]
pub static mut r_drawnpolycount: libc::c_int = 0;
#[no_mangle]
pub static mut r_wholepolycount: libc::c_int = 0;
#[no_mangle]
pub static mut pfrustum_indexes: [*mut libc::c_int; 4] = [0 as *const libc::c_int
    as *mut libc::c_int; 4];
#[no_mangle]
pub static mut r_frustum_indexes: [libc::c_int; 24] = [0; 24];
#[no_mangle]
pub static mut r_viewleaf: *mut mleaf_t = 0 as *const mleaf_t as *mut mleaf_t;
#[no_mangle]
pub static mut r_viewcluster: libc::c_int = 0;
#[no_mangle]
pub static mut r_oldviewcluster: libc::c_int = 0;
#[no_mangle]
pub static mut r_notexture_mip: *mut image_t = 0 as *const image_t as *mut image_t;
#[no_mangle]
pub static mut da_time1: libc::c_float = 0.;
#[no_mangle]
pub static mut da_time2: libc::c_float = 0.;
#[no_mangle]
pub static mut dp_time1: libc::c_float = 0.;
#[no_mangle]
pub static mut dp_time2: libc::c_float = 0.;
#[no_mangle]
pub static mut db_time1: libc::c_float = 0.;
#[no_mangle]
pub static mut db_time2: libc::c_float = 0.;
#[no_mangle]
pub static mut rw_time1: libc::c_float = 0.;
#[no_mangle]
pub static mut rw_time2: libc::c_float = 0.;
#[no_mangle]
pub static mut se_time1: libc::c_float = 0.;
#[no_mangle]
pub static mut se_time2: libc::c_float = 0.;
#[no_mangle]
pub static mut de_time1: libc::c_float = 0.;
#[no_mangle]
pub static mut de_time2: libc::c_float = 0.;
#[no_mangle]
pub static mut r_lefthand: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut sw_aliasstats: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut sw_allow_modex: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut sw_clearcolor: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut sw_drawflat: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut sw_draworder: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut sw_maxedges: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut sw_maxsurfs: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut sw_mode: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut sw_reportedgeout: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut sw_reportsurfout: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut sw_stipplealpha: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut sw_surfcacheoverride: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut sw_waterwarp: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut r_drawworld: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut r_drawentities: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut r_dspeeds: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut r_fullbright: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut r_lerpmodels: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut r_novis: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut r_speeds: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut r_lightlevel: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut vid_fullscreen: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut vid_gamma: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut sw_lockpvs: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut d_sdivzstepu: libc::c_float = 0.;
#[no_mangle]
pub static mut d_tdivzstepu: libc::c_float = 0.;
#[no_mangle]
pub static mut d_zistepu: libc::c_float = 0.;
#[no_mangle]
pub static mut d_sdivzstepv: libc::c_float = 0.;
#[no_mangle]
pub static mut d_tdivzstepv: libc::c_float = 0.;
#[no_mangle]
pub static mut d_zistepv: libc::c_float = 0.;
#[no_mangle]
pub static mut d_sdivzorigin: libc::c_float = 0.;
#[no_mangle]
pub static mut d_tdivzorigin: libc::c_float = 0.;
#[no_mangle]
pub static mut d_ziorigin: libc::c_float = 0.;
#[no_mangle]
pub static mut sadjust: fixed16_t = 0;
#[no_mangle]
pub static mut tadjust: fixed16_t = 0;
#[no_mangle]
pub static mut bbextents: fixed16_t = 0;
#[no_mangle]
pub static mut bbextentt: fixed16_t = 0;
#[no_mangle]
pub static mut cacheblock: *mut pixel_t = 0 as *const pixel_t as *mut pixel_t;
#[no_mangle]
pub static mut cachewidth: libc::c_int = 0;
#[no_mangle]
pub static mut d_viewbuffer: *mut pixel_t = 0 as *const pixel_t as *mut pixel_t;
#[no_mangle]
pub static mut d_pzbuffer: *mut libc::c_short = 0 as *const libc::c_short
    as *mut libc::c_short;
#[no_mangle]
pub static mut d_zrowbytes: libc::c_uint = 0;
#[no_mangle]
pub static mut d_zwidth: libc::c_uint = 0;
#[no_mangle]
pub static mut r_notexture_buffer: [byte; 1024] = [0; 1024];
#[no_mangle]
pub unsafe extern "C" fn R_InitTextures() {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut m: libc::c_int = 0;
    let mut dest: *mut byte = 0 as *mut byte;
    r_notexture_mip = &mut r_notexture_buffer as *mut [byte; 1024] as *mut image_t;
    let ref mut fresh0 = (*r_notexture_mip).height;
    *fresh0 = 16 as libc::c_int;
    (*r_notexture_mip).width = *fresh0;
    let ref mut fresh1 = (*r_notexture_mip).pixels[0 as libc::c_int as usize];
    *fresh1 = &mut *r_notexture_buffer
        .as_mut_ptr()
        .offset(::std::mem::size_of::<image_t>() as libc::c_ulong as isize) as *mut byte;
    let ref mut fresh2 = (*r_notexture_mip).pixels[1 as libc::c_int as usize];
    *fresh2 = ((*r_notexture_mip).pixels[0 as libc::c_int as usize])
        .offset((16 as libc::c_int * 16 as libc::c_int) as isize);
    let ref mut fresh3 = (*r_notexture_mip).pixels[2 as libc::c_int as usize];
    *fresh3 = ((*r_notexture_mip).pixels[1 as libc::c_int as usize])
        .offset((8 as libc::c_int * 8 as libc::c_int) as isize);
    let ref mut fresh4 = (*r_notexture_mip).pixels[3 as libc::c_int as usize];
    *fresh4 = ((*r_notexture_mip).pixels[2 as libc::c_int as usize])
        .offset((4 as libc::c_int * 4 as libc::c_int) as isize);
    m = 0 as libc::c_int;
    while m < 4 as libc::c_int {
        dest = (*r_notexture_mip).pixels[m as usize];
        y = 0 as libc::c_int;
        while y < 16 as libc::c_int >> m {
            x = 0 as libc::c_int;
            while x < 16 as libc::c_int >> m {
                if (y < 8 as libc::c_int >> m) as libc::c_int
                    ^ (x < 8 as libc::c_int >> m) as libc::c_int != 0
                {
                    let fresh5 = dest;
                    dest = dest.offset(1);
                    *fresh5 = 0 as libc::c_int as byte;
                } else {
                    let fresh6 = dest;
                    dest = dest.offset(1);
                    *fresh6 = 0xff as libc::c_int as byte;
                }
                x += 1;
            }
            y += 1;
        }
        m += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn R_InitTurb() {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 1280 as libc::c_int {
        sintable[i
            as usize] = ((8 as libc::c_int * 0x10000 as libc::c_int) as libc::c_double
            + sin(
                i as libc::c_double * 3.14159f64 * 2 as libc::c_int as libc::c_double
                    / 128 as libc::c_int as libc::c_double,
            ) * 8 as libc::c_int as libc::c_double
                * 0x10000 as libc::c_int as libc::c_double) as libc::c_int;
        intsintable[i
            as usize] = (3 as libc::c_int as libc::c_double
            + sin(
                i as libc::c_double * 3.14159f64 * 2 as libc::c_int as libc::c_double
                    / 128 as libc::c_int as libc::c_double,
            ) * 3 as libc::c_int as libc::c_double) as libc::c_int;
        blanktable[i as usize] = 0 as libc::c_int;
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn R_Register() {
    sw_aliasstats = (ri.Cvar_Get)
        .expect(
            "non-null function pointer",
        )(
        b"sw_polymodelstats\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    sw_allow_modex = (ri.Cvar_Get)
        .expect(
            "non-null function pointer",
        )(
        b"sw_allow_modex\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        1 as libc::c_int,
    );
    sw_clearcolor = (ri.Cvar_Get)
        .expect(
            "non-null function pointer",
        )(
        b"sw_clearcolor\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    sw_drawflat = (ri.Cvar_Get)
        .expect(
            "non-null function pointer",
        )(
        b"sw_drawflat\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    sw_draworder = (ri.Cvar_Get)
        .expect(
            "non-null function pointer",
        )(
        b"sw_draworder\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    sw_maxedges = (ri.Cvar_Get)
        .expect(
            "non-null function pointer",
        )(
        b"sw_maxedges\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"x\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    sw_maxsurfs = (ri.Cvar_Get)
        .expect(
            "non-null function pointer",
        )(
        b"sw_maxsurfs\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    sw_mipcap = (ri.Cvar_Get)
        .expect(
            "non-null function pointer",
        )(
        b"sw_mipcap\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    sw_mipscale = (ri.Cvar_Get)
        .expect(
            "non-null function pointer",
        )(
        b"sw_mipscale\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    sw_reportedgeout = (ri.Cvar_Get)
        .expect(
            "non-null function pointer",
        )(
        b"sw_reportedgeout\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    sw_reportsurfout = (ri.Cvar_Get)
        .expect(
            "non-null function pointer",
        )(
        b"sw_reportsurfout\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    sw_stipplealpha = (ri.Cvar_Get)
        .expect(
            "non-null function pointer",
        )(
        b"sw_stipplealpha\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        1 as libc::c_int,
    );
    sw_surfcacheoverride = (ri.Cvar_Get)
        .expect(
            "non-null function pointer",
        )(
        b"sw_surfcacheoverride\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    sw_waterwarp = (ri.Cvar_Get)
        .expect(
            "non-null function pointer",
        )(
        b"sw_waterwarp\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    sw_mode = (ri.Cvar_Get)
        .expect(
            "non-null function pointer",
        )(
        b"sw_mode\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        1 as libc::c_int,
    );
    r_lefthand = (ri.Cvar_Get)
        .expect(
            "non-null function pointer",
        )(
        b"hand\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        2 as libc::c_int | 1 as libc::c_int,
    );
    r_speeds = (ri.Cvar_Get)
        .expect(
            "non-null function pointer",
        )(
        b"r_speeds\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    r_fullbright = (ri.Cvar_Get)
        .expect(
            "non-null function pointer",
        )(
        b"r_fullbright\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    r_drawentities = (ri.Cvar_Get)
        .expect(
            "non-null function pointer",
        )(
        b"r_drawentities\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    r_drawworld = (ri.Cvar_Get)
        .expect(
            "non-null function pointer",
        )(
        b"r_drawworld\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    r_dspeeds = (ri.Cvar_Get)
        .expect(
            "non-null function pointer",
        )(
        b"r_dspeeds\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    r_lightlevel = (ri.Cvar_Get)
        .expect(
            "non-null function pointer",
        )(
        b"r_lightlevel\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    r_lerpmodels = (ri.Cvar_Get)
        .expect(
            "non-null function pointer",
        )(
        b"r_lerpmodels\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    r_novis = (ri.Cvar_Get)
        .expect(
            "non-null function pointer",
        )(
        b"r_novis\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    vid_fullscreen = (ri.Cvar_Get)
        .expect(
            "non-null function pointer",
        )(
        b"vid_fullscreen\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        1 as libc::c_int,
    );
    vid_gamma = (ri.Cvar_Get)
        .expect(
            "non-null function pointer",
        )(
        b"vid_gamma\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"1.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        1 as libc::c_int,
    );
    (ri.Cmd_AddCommand)
        .expect(
            "non-null function pointer",
        )(
        b"modellist\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        Some(Mod_Modellist_f as unsafe extern "C" fn() -> ()),
    );
    (ri.Cmd_AddCommand)
        .expect(
            "non-null function pointer",
        )(
        b"screenshot\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        Some(R_ScreenShot_f as unsafe extern "C" fn() -> ()),
    );
    (ri.Cmd_AddCommand)
        .expect(
            "non-null function pointer",
        )(
        b"imagelist\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        Some(R_ImageList_f as unsafe extern "C" fn() -> ()),
    );
    (*sw_mode).modified = true_0;
    (*vid_gamma).modified = true_0;
    sw_lockpvs = (ri.Cvar_Get)
        .expect(
            "non-null function pointer",
        )(
        b"sw_lockpvs\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn R_UnRegister() {
    (ri.Cmd_RemoveCommand)
        .expect(
            "non-null function pointer",
        )(b"screenshot\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    (ri.Cmd_RemoveCommand)
        .expect(
            "non-null function pointer",
        )(b"modellist\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    (ri.Cmd_RemoveCommand)
        .expect(
            "non-null function pointer",
        )(b"imagelist\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn R_Init(
    mut hInstance: *mut libc::c_void,
    mut wndProc: *mut libc::c_void,
) -> qboolean {
    R_InitImages();
    Mod_Init();
    Draw_InitLocal();
    R_InitTextures();
    R_InitTurb();
    view_clipplanes[0 as libc::c_int as usize].leftedge = true_0 as libc::c_int as byte;
    view_clipplanes[1 as libc::c_int as usize].rightedge = true_0 as libc::c_int as byte;
    view_clipplanes[3 as libc::c_int as usize].leftedge = false_0 as libc::c_int as byte;
    view_clipplanes[2 as libc::c_int as usize]
        .leftedge = view_clipplanes[3 as libc::c_int as usize].leftedge;
    view_clipplanes[1 as libc::c_int as usize]
        .leftedge = view_clipplanes[2 as libc::c_int as usize].leftedge;
    view_clipplanes[3 as libc::c_int as usize]
        .rightedge = false_0 as libc::c_int as byte;
    view_clipplanes[2 as libc::c_int as usize]
        .rightedge = view_clipplanes[3 as libc::c_int as usize].rightedge;
    view_clipplanes[0 as libc::c_int as usize]
        .rightedge = view_clipplanes[2 as libc::c_int as usize].rightedge;
    r_refdef.xOrigin = (1.0f64 / 2.0f64) as libc::c_float;
    r_refdef.yOrigin = (1.0f64 / 2.0f64) as libc::c_float;
    r_aliasuvscale = 1.0f64 as libc::c_float;
    R_Register();
    Draw_GetPalette();
    SWimp_Init(hInstance, wndProc);
    R_BeginFrame(0 as libc::c_int as libc::c_float);
    (ri.Con_Printf)
        .expect(
            "non-null function pointer",
        )(
        0 as libc::c_int,
        b"ref_soft version: SOFT 0.01\n\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    return true_0;
}
#[no_mangle]
pub unsafe extern "C" fn R_Shutdown() {
    if !d_pzbuffer.is_null() {
        free(d_pzbuffer as *mut libc::c_void);
        d_pzbuffer = 0 as *mut libc::c_short;
    }
    if !sc_base.is_null() {
        D_FlushCaches();
        free(sc_base as *mut libc::c_void);
        sc_base = 0 as *mut surfcache_t;
    }
    if !(vid.colormap).is_null() {
        free(vid.colormap as *mut libc::c_void);
        vid.colormap = 0 as *mut pixel_t;
    }
    R_UnRegister();
    Mod_FreeAll();
    R_ShutdownImages();
    SWimp_Shutdown();
}
#[no_mangle]
pub unsafe extern "C" fn R_NewMap() {
    r_viewcluster = -(1 as libc::c_int);
    r_cnumsurfs = (*sw_maxsurfs).value as libc::c_int;
    if r_cnumsurfs <= 1000 as libc::c_int {
        r_cnumsurfs = 1000 as libc::c_int;
    }
    if r_cnumsurfs > 1000 as libc::c_int {
        surfaces = malloc(
            (r_cnumsurfs as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<surf_t>() as libc::c_ulong),
        ) as *mut surf_t;
        surface_p = surfaces;
        surf_max = &mut *surfaces.offset(r_cnumsurfs as isize) as *mut surf_t;
        r_surfsonstack = false_0;
        surfaces = surfaces.offset(-1);
        R_SurfacePatch();
    } else {
        r_surfsonstack = true_0;
    }
    r_maxedgesseen = 0 as libc::c_int;
    r_maxsurfsseen = 0 as libc::c_int;
    r_numallocatededges = (*sw_maxedges).value as libc::c_int;
    if r_numallocatededges < 2000 as libc::c_int {
        r_numallocatededges = 2000 as libc::c_int;
    }
    if r_numallocatededges <= 2000 as libc::c_int {
        auxedges = 0 as *mut edge_t;
    } else {
        auxedges = malloc(
            (r_numallocatededges as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<edge_t>() as libc::c_ulong),
        ) as *mut edge_t;
    };
}
#[no_mangle]
pub unsafe extern "C" fn R_MarkLeaves() {
    let mut vis: *mut byte = 0 as *mut byte;
    let mut node: *mut mnode_t = 0 as *mut mnode_t;
    let mut i: libc::c_int = 0;
    let mut leaf: *mut mleaf_t = 0 as *mut mleaf_t;
    let mut cluster: libc::c_int = 0;
    if r_oldviewcluster == r_viewcluster && (*r_novis).value == 0.
        && r_viewcluster != -(1 as libc::c_int)
    {
        return;
    }
    if (*sw_lockpvs).value != 0. {
        return;
    }
    r_visframecount += 1;
    r_oldviewcluster = r_viewcluster;
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
#[no_mangle]
pub unsafe extern "C" fn R_DrawNullModel() {}
#[no_mangle]
pub unsafe extern "C" fn R_DrawEntitiesOnList() {
    let mut i: libc::c_int = 0;
    let mut translucent_entities: qboolean = false_0;
    if (*r_drawentities).value == 0. {
        return;
    }
    i = 0 as libc::c_int;
    while i < r_newrefdef.num_entities {
        currententity = &mut *(r_newrefdef.entities).offset(i as isize) as *mut entity_t;
        if (*currententity).flags & 32 as libc::c_int != 0 {
            translucent_entities = true_0;
        } else if (*currententity).flags & 128 as libc::c_int != 0 {
            modelorg[0 as libc::c_int as usize] = -r_origin[0 as libc::c_int as usize];
            modelorg[1 as libc::c_int as usize] = -r_origin[1 as libc::c_int as usize];
            modelorg[2 as libc::c_int as usize] = -r_origin[2 as libc::c_int as usize];
            r_entorigin[0 as libc::c_int
                as usize] = vec3_origin[0 as libc::c_int as usize];
            r_entorigin[1 as libc::c_int
                as usize] = vec3_origin[1 as libc::c_int as usize];
            r_entorigin[2 as libc::c_int
                as usize] = vec3_origin[2 as libc::c_int as usize];
            R_DrawBeam(currententity);
        } else {
            currentmodel = (*currententity).model;
            if currentmodel.is_null() {
                R_DrawNullModel();
            } else {
                r_entorigin[0 as libc::c_int
                    as usize] = (*currententity).origin[0 as libc::c_int as usize];
                r_entorigin[1 as libc::c_int
                    as usize] = (*currententity).origin[1 as libc::c_int as usize];
                r_entorigin[2 as libc::c_int
                    as usize] = (*currententity).origin[2 as libc::c_int as usize];
                modelorg[0 as libc::c_int
                    as usize] = r_origin[0 as libc::c_int as usize]
                    - r_entorigin[0 as libc::c_int as usize];
                modelorg[1 as libc::c_int
                    as usize] = r_origin[1 as libc::c_int as usize]
                    - r_entorigin[1 as libc::c_int as usize];
                modelorg[2 as libc::c_int
                    as usize] = r_origin[2 as libc::c_int as usize]
                    - r_entorigin[2 as libc::c_int as usize];
                match (*currentmodel).type_0 as libc::c_uint {
                    2 => {
                        R_DrawSprite();
                    }
                    3 => {
                        R_AliasDrawModel();
                    }
                    _ => {}
                }
            }
        }
        i += 1;
    }
    if translucent_entities as u64 == 0 {
        return;
    }
    i = 0 as libc::c_int;
    while i < r_newrefdef.num_entities {
        currententity = &mut *(r_newrefdef.entities).offset(i as isize) as *mut entity_t;
        if !((*currententity).flags & 32 as libc::c_int == 0) {
            if (*currententity).flags & 128 as libc::c_int != 0 {
                modelorg[0 as libc::c_int
                    as usize] = -r_origin[0 as libc::c_int as usize];
                modelorg[1 as libc::c_int
                    as usize] = -r_origin[1 as libc::c_int as usize];
                modelorg[2 as libc::c_int
                    as usize] = -r_origin[2 as libc::c_int as usize];
                r_entorigin[0 as libc::c_int
                    as usize] = vec3_origin[0 as libc::c_int as usize];
                r_entorigin[1 as libc::c_int
                    as usize] = vec3_origin[1 as libc::c_int as usize];
                r_entorigin[2 as libc::c_int
                    as usize] = vec3_origin[2 as libc::c_int as usize];
                R_DrawBeam(currententity);
            } else {
                currentmodel = (*currententity).model;
                if currentmodel.is_null() {
                    R_DrawNullModel();
                } else {
                    r_entorigin[0 as libc::c_int
                        as usize] = (*currententity).origin[0 as libc::c_int as usize];
                    r_entorigin[1 as libc::c_int
                        as usize] = (*currententity).origin[1 as libc::c_int as usize];
                    r_entorigin[2 as libc::c_int
                        as usize] = (*currententity).origin[2 as libc::c_int as usize];
                    modelorg[0 as libc::c_int
                        as usize] = r_origin[0 as libc::c_int as usize]
                        - r_entorigin[0 as libc::c_int as usize];
                    modelorg[1 as libc::c_int
                        as usize] = r_origin[1 as libc::c_int as usize]
                        - r_entorigin[1 as libc::c_int as usize];
                    modelorg[2 as libc::c_int
                        as usize] = r_origin[2 as libc::c_int as usize]
                        - r_entorigin[2 as libc::c_int as usize];
                    match (*currentmodel).type_0 as libc::c_uint {
                        2 => {
                            R_DrawSprite();
                        }
                        3 => {
                            R_AliasDrawModel();
                        }
                        _ => {}
                    }
                }
            }
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn R_BmodelCheckBBox(
    mut minmaxs: *mut libc::c_float,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut pindex: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut clipflags: libc::c_int = 0;
    let mut acceptpt: vec3_t = [0.; 3];
    let mut rejectpt: vec3_t = [0.; 3];
    let mut d: libc::c_float = 0.;
    clipflags = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        pindex = pfrustum_indexes[i as usize];
        rejectpt[0 as libc::c_int
            as usize] = *minmaxs
            .offset(*pindex.offset(0 as libc::c_int as isize) as isize);
        rejectpt[1 as libc::c_int
            as usize] = *minmaxs
            .offset(*pindex.offset(1 as libc::c_int as isize) as isize);
        rejectpt[2 as libc::c_int
            as usize] = *minmaxs
            .offset(*pindex.offset(2 as libc::c_int as isize) as isize);
        d = rejectpt[0 as libc::c_int as usize]
            * view_clipplanes[i as usize].normal[0 as libc::c_int as usize]
            + rejectpt[1 as libc::c_int as usize]
                * view_clipplanes[i as usize].normal[1 as libc::c_int as usize]
            + rejectpt[2 as libc::c_int as usize]
                * view_clipplanes[i as usize].normal[2 as libc::c_int as usize];
        d -= view_clipplanes[i as usize].dist;
        if d <= 0 as libc::c_int as libc::c_float {
            return 0x10 as libc::c_int;
        }
        acceptpt[0 as libc::c_int
            as usize] = *minmaxs
            .offset(
                *pindex.offset((3 as libc::c_int + 0 as libc::c_int) as isize) as isize,
            );
        acceptpt[1 as libc::c_int
            as usize] = *minmaxs
            .offset(
                *pindex.offset((3 as libc::c_int + 1 as libc::c_int) as isize) as isize,
            );
        acceptpt[2 as libc::c_int
            as usize] = *minmaxs
            .offset(
                *pindex.offset((3 as libc::c_int + 2 as libc::c_int) as isize) as isize,
            );
        d = acceptpt[0 as libc::c_int as usize]
            * view_clipplanes[i as usize].normal[0 as libc::c_int as usize]
            + acceptpt[1 as libc::c_int as usize]
                * view_clipplanes[i as usize].normal[1 as libc::c_int as usize]
            + acceptpt[2 as libc::c_int as usize]
                * view_clipplanes[i as usize].normal[2 as libc::c_int as usize];
        d -= view_clipplanes[i as usize].dist;
        if d <= 0 as libc::c_int as libc::c_float {
            clipflags |= (1 as libc::c_int) << i;
        }
        i += 1;
    }
    return clipflags;
}
#[no_mangle]
pub unsafe extern "C" fn R_FindTopnode(
    mut mins: *mut vec_t,
    mut maxs: *mut vec_t,
) -> *mut mnode_t {
    let mut splitplane: *mut mplane_t = 0 as *mut mplane_t;
    let mut sides: libc::c_int = 0;
    let mut node: *mut mnode_t = 0 as *mut mnode_t;
    node = (*r_worldmodel).nodes;
    loop {
        if (*node).visframe != r_visframecount {
            return 0 as *mut mnode_t;
        }
        if (*node).contents != -(1 as libc::c_int) {
            if (*node).contents != 1 as libc::c_int {
                return node;
            }
            return 0 as *mut mnode_t;
        }
        splitplane = (*node).plane;
        sides = if ((*(splitplane as *mut cplane_t)).type_0 as libc::c_int)
            < 3 as libc::c_int
        {
            if (*(splitplane as *mut cplane_t)).dist
                <= *mins.offset((*(splitplane as *mut cplane_t)).type_0 as isize)
            {
                1 as libc::c_int
            } else if (*(splitplane as *mut cplane_t)).dist
                >= *maxs.offset((*(splitplane as *mut cplane_t)).type_0 as isize)
            {
                2 as libc::c_int
            } else {
                3 as libc::c_int
            }
        } else {
            BoxOnPlaneSide(mins, maxs, splitplane as *mut cplane_t)
        };
        if sides == 3 as libc::c_int {
            return node;
        }
        if sides & 1 as libc::c_int != 0 {
            node = (*node).children[0 as libc::c_int as usize];
        } else {
            node = (*node).children[1 as libc::c_int as usize];
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn RotatedBBox(
    mut mins: *mut vec_t,
    mut maxs: *mut vec_t,
    mut angles: *mut vec_t,
    mut tmins: *mut vec_t,
    mut tmaxs: *mut vec_t,
) {
    let mut tmp: vec3_t = [0.; 3];
    let mut v: vec3_t = [0.; 3];
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut forward: vec3_t = [0.; 3];
    let mut right: vec3_t = [0.; 3];
    let mut up: vec3_t = [0.; 3];
    if *angles.offset(0 as libc::c_int as isize) == 0.
        && *angles.offset(1 as libc::c_int as isize) == 0.
        && *angles.offset(2 as libc::c_int as isize) == 0.
    {
        *tmins
            .offset(0 as libc::c_int as isize) = *mins.offset(0 as libc::c_int as isize);
        *tmins
            .offset(1 as libc::c_int as isize) = *mins.offset(1 as libc::c_int as isize);
        *tmins
            .offset(2 as libc::c_int as isize) = *mins.offset(2 as libc::c_int as isize);
        *tmaxs
            .offset(0 as libc::c_int as isize) = *maxs.offset(0 as libc::c_int as isize);
        *tmaxs
            .offset(1 as libc::c_int as isize) = *maxs.offset(1 as libc::c_int as isize);
        *tmaxs
            .offset(2 as libc::c_int as isize) = *maxs.offset(2 as libc::c_int as isize);
        return;
    }
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        *tmins.offset(i as isize) = 99999 as libc::c_int as vec_t;
        *tmaxs.offset(i as isize) = -(99999 as libc::c_int) as vec_t;
        i += 1;
    }
    AngleVectors(angles, forward.as_mut_ptr(), right.as_mut_ptr(), up.as_mut_ptr());
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        if i & 1 as libc::c_int != 0 {
            tmp[0 as libc::c_int as usize] = *mins.offset(0 as libc::c_int as isize);
        } else {
            tmp[0 as libc::c_int as usize] = *maxs.offset(0 as libc::c_int as isize);
        }
        if i & 2 as libc::c_int != 0 {
            tmp[1 as libc::c_int as usize] = *mins.offset(1 as libc::c_int as isize);
        } else {
            tmp[1 as libc::c_int as usize] = *maxs.offset(1 as libc::c_int as isize);
        }
        if i & 4 as libc::c_int != 0 {
            tmp[2 as libc::c_int as usize] = *mins.offset(2 as libc::c_int as isize);
        } else {
            tmp[2 as libc::c_int as usize] = *maxs.offset(2 as libc::c_int as isize);
        }
        VectorScale(
            forward.as_mut_ptr(),
            tmp[0 as libc::c_int as usize],
            v.as_mut_ptr(),
        );
        VectorMA(
            v.as_mut_ptr(),
            -tmp[1 as libc::c_int as usize],
            right.as_mut_ptr(),
            v.as_mut_ptr(),
        );
        VectorMA(
            v.as_mut_ptr(),
            tmp[2 as libc::c_int as usize],
            up.as_mut_ptr(),
            v.as_mut_ptr(),
        );
        j = 0 as libc::c_int;
        while j < 3 as libc::c_int {
            if v[j as usize] < *tmins.offset(j as isize) {
                *tmins.offset(j as isize) = v[j as usize];
            }
            if v[j as usize] > *tmaxs.offset(j as isize) {
                *tmaxs.offset(j as isize) = v[j as usize];
            }
            j += 1;
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn R_DrawBEntitiesOnList() {
    let mut i: libc::c_int = 0;
    let mut clipflags: libc::c_int = 0;
    let mut oldorigin: vec3_t = [0.; 3];
    let mut mins: vec3_t = [0.; 3];
    let mut maxs: vec3_t = [0.; 3];
    let mut minmaxs: [libc::c_float; 6] = [0.; 6];
    let mut topnode: *mut mnode_t = 0 as *mut mnode_t;
    if (*r_drawentities).value == 0. {
        return;
    }
    oldorigin[0 as libc::c_int as usize] = modelorg[0 as libc::c_int as usize];
    oldorigin[1 as libc::c_int as usize] = modelorg[1 as libc::c_int as usize];
    oldorigin[2 as libc::c_int as usize] = modelorg[2 as libc::c_int as usize];
    insubmodel = true_0;
    r_dlightframecount = r_framecount;
    i = 0 as libc::c_int;
    while i < r_newrefdef.num_entities {
        currententity = &mut *(r_newrefdef.entities).offset(i as isize) as *mut entity_t;
        currentmodel = (*currententity).model;
        if !currentmodel.is_null() {
            if !((*currentmodel).nummodelsurfaces == 0 as libc::c_int) {
                if !((*currententity).flags & 128 as libc::c_int != 0) {
                    if !((*currentmodel).type_0 as libc::c_uint
                        != mod_brush as libc::c_int as libc::c_uint)
                    {
                        RotatedBBox(
                            ((*currentmodel).mins).as_mut_ptr(),
                            ((*currentmodel).maxs).as_mut_ptr(),
                            ((*currententity).angles).as_mut_ptr(),
                            mins.as_mut_ptr(),
                            maxs.as_mut_ptr(),
                        );
                        minmaxs[0 as libc::c_int
                            as usize] = mins[0 as libc::c_int as usize]
                            + (*currententity).origin[0 as libc::c_int as usize];
                        minmaxs[1 as libc::c_int
                            as usize] = mins[1 as libc::c_int as usize]
                            + (*currententity).origin[1 as libc::c_int as usize];
                        minmaxs[2 as libc::c_int
                            as usize] = mins[2 as libc::c_int as usize]
                            + (*currententity).origin[2 as libc::c_int as usize];
                        *minmaxs
                            .as_mut_ptr()
                            .offset(3 as libc::c_int as isize)
                            .offset(
                                0 as libc::c_int as isize,
                            ) = maxs[0 as libc::c_int as usize]
                            + (*currententity).origin[0 as libc::c_int as usize];
                        *minmaxs
                            .as_mut_ptr()
                            .offset(3 as libc::c_int as isize)
                            .offset(
                                1 as libc::c_int as isize,
                            ) = maxs[1 as libc::c_int as usize]
                            + (*currententity).origin[1 as libc::c_int as usize];
                        *minmaxs
                            .as_mut_ptr()
                            .offset(3 as libc::c_int as isize)
                            .offset(
                                2 as libc::c_int as isize,
                            ) = maxs[2 as libc::c_int as usize]
                            + (*currententity).origin[2 as libc::c_int as usize];
                        clipflags = R_BmodelCheckBBox(minmaxs.as_mut_ptr());
                        if !(clipflags == 0x10 as libc::c_int) {
                            topnode = R_FindTopnode(
                                minmaxs.as_mut_ptr(),
                                minmaxs.as_mut_ptr().offset(3 as libc::c_int as isize),
                            );
                            if !topnode.is_null() {
                                r_entorigin[0 as libc::c_int
                                    as usize] = (*currententity)
                                    .origin[0 as libc::c_int as usize];
                                r_entorigin[1 as libc::c_int
                                    as usize] = (*currententity)
                                    .origin[1 as libc::c_int as usize];
                                r_entorigin[2 as libc::c_int
                                    as usize] = (*currententity)
                                    .origin[2 as libc::c_int as usize];
                                modelorg[0 as libc::c_int
                                    as usize] = r_origin[0 as libc::c_int as usize]
                                    - r_entorigin[0 as libc::c_int as usize];
                                modelorg[1 as libc::c_int
                                    as usize] = r_origin[1 as libc::c_int as usize]
                                    - r_entorigin[1 as libc::c_int as usize];
                                modelorg[2 as libc::c_int
                                    as usize] = r_origin[2 as libc::c_int as usize]
                                    - r_entorigin[2 as libc::c_int as usize];
                                r_pcurrentvertbase = (*currentmodel).vertexes;
                                R_RotateBmodel();
                                R_PushDlights(currentmodel);
                                if (*topnode).contents == -(1 as libc::c_int) {
                                    r_clipflags = clipflags;
                                    R_DrawSolidClippedSubmodelPolygons(currentmodel, topnode);
                                } else {
                                    R_DrawSubmodelPolygons(currentmodel, clipflags, topnode);
                                }
                                vpn[0 as libc::c_int
                                    as usize] = base_vpn[0 as libc::c_int as usize];
                                vpn[1 as libc::c_int
                                    as usize] = base_vpn[1 as libc::c_int as usize];
                                vpn[2 as libc::c_int
                                    as usize] = base_vpn[2 as libc::c_int as usize];
                                vup[0 as libc::c_int
                                    as usize] = base_vup[0 as libc::c_int as usize];
                                vup[1 as libc::c_int
                                    as usize] = base_vup[1 as libc::c_int as usize];
                                vup[2 as libc::c_int
                                    as usize] = base_vup[2 as libc::c_int as usize];
                                vright[0 as libc::c_int
                                    as usize] = base_vright[0 as libc::c_int as usize];
                                vright[1 as libc::c_int
                                    as usize] = base_vright[1 as libc::c_int as usize];
                                vright[2 as libc::c_int
                                    as usize] = base_vright[2 as libc::c_int as usize];
                                modelorg[0 as libc::c_int
                                    as usize] = oldorigin[0 as libc::c_int as usize];
                                modelorg[1 as libc::c_int
                                    as usize] = oldorigin[1 as libc::c_int as usize];
                                modelorg[2 as libc::c_int
                                    as usize] = oldorigin[2 as libc::c_int as usize];
                                R_TransformFrustum();
                            }
                        }
                    }
                }
            }
        }
        i += 1;
    }
    insubmodel = false_0;
}
#[no_mangle]
pub unsafe extern "C" fn R_EdgeDrawing() {
    let mut ledges: [edge_t; 2001] = [edge_t {
        u: 0,
        u_step: 0,
        prev: 0 as *mut edge_s,
        next: 0 as *mut edge_s,
        surfs: [0; 2],
        nextremove: 0 as *mut edge_s,
        nearzi: 0.,
        owner: 0 as *mut medge_t,
    }; 2001];
    let mut lsurfs: [surf_t; 1001] = [surf_t {
        next: 0 as *mut surf_s,
        prev: 0 as *mut surf_s,
        spans: 0 as *mut espan_s,
        key: 0,
        last_u: 0,
        spanstate: 0,
        flags: 0,
        msurf: 0 as *mut msurface_t,
        entity: 0 as *mut entity_t,
        nearzi: 0.,
        insubmodel: false_0,
        d_ziorigin: 0.,
        d_zistepu: 0.,
        d_zistepv: 0.,
        pad: [0; 2],
    }; 1001];
    if r_newrefdef.rdflags & 2 as libc::c_int != 0 {
        return;
    }
    if !auxedges.is_null() {
        r_edges = auxedges;
    } else {
        r_edges = (&mut *ledges.as_mut_ptr().offset(0 as libc::c_int as isize)
            as *mut edge_t as libc::c_long + 32 as libc::c_int as libc::c_long
            - 1 as libc::c_int as libc::c_long
            & !(32 as libc::c_int - 1 as libc::c_int) as libc::c_long) as *mut edge_t;
    }
    if r_surfsonstack as u64 != 0 {
        surfaces = (&mut *lsurfs.as_mut_ptr().offset(0 as libc::c_int as isize)
            as *mut surf_t as libc::c_long + 32 as libc::c_int as libc::c_long
            - 1 as libc::c_int as libc::c_long
            & !(32 as libc::c_int - 1 as libc::c_int) as libc::c_long) as *mut surf_t;
        surf_max = &mut *surfaces.offset(r_cnumsurfs as isize) as *mut surf_t;
        surfaces = surfaces.offset(-1);
        R_SurfacePatch();
    }
    R_BeginEdgeFrame();
    if (*r_dspeeds).value != 0. {
        rw_time1 = Sys_Milliseconds() as libc::c_float;
    }
    R_RenderWorld();
    if (*r_dspeeds).value != 0. {
        rw_time2 = Sys_Milliseconds() as libc::c_float;
        db_time1 = rw_time2;
    }
    R_DrawBEntitiesOnList();
    if (*r_dspeeds).value != 0. {
        db_time2 = Sys_Milliseconds() as libc::c_float;
        se_time1 = db_time2;
    }
    R_ScanEdges();
}
#[no_mangle]
pub unsafe extern "C" fn R_CalcPalette() {
    static mut modified: qboolean = false_0;
    let mut palette: [[byte; 4]; 256] = [[0; 4]; 256];
    let mut in_0: *mut byte = 0 as *mut byte;
    let mut out: *mut byte = 0 as *mut byte;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut alpha: libc::c_float = 0.;
    let mut one_minus_alpha: libc::c_float = 0.;
    let mut premult: vec3_t = [0.; 3];
    let mut v: libc::c_int = 0;
    alpha = r_newrefdef.blend[3 as libc::c_int as usize];
    if alpha <= 0 as libc::c_int as libc::c_float {
        if modified as u64 != 0 {
            modified = false_0;
            R_GammaCorrectAndSetPalette(
                d_8to24table.as_mut_ptr() as *const libc::c_uchar,
            );
            return;
        }
        return;
    }
    modified = true_0;
    if alpha > 1 as libc::c_int as libc::c_float {
        alpha = 1 as libc::c_int as libc::c_float;
    }
    premult[0 as libc::c_int
        as usize] = r_newrefdef.blend[0 as libc::c_int as usize] * alpha
        * 255 as libc::c_int as libc::c_float;
    premult[1 as libc::c_int
        as usize] = r_newrefdef.blend[1 as libc::c_int as usize] * alpha
        * 255 as libc::c_int as libc::c_float;
    premult[2 as libc::c_int
        as usize] = r_newrefdef.blend[2 as libc::c_int as usize] * alpha
        * 255 as libc::c_int as libc::c_float;
    one_minus_alpha = (1.0f64 - alpha as libc::c_double) as libc::c_float;
    in_0 = d_8to24table.as_mut_ptr() as *mut byte;
    out = (palette[0 as libc::c_int as usize]).as_mut_ptr();
    i = 0 as libc::c_int;
    while i < 256 as libc::c_int {
        j = 0 as libc::c_int;
        while j < 3 as libc::c_int {
            v = (premult[j as usize]
                + one_minus_alpha
                    * *in_0.offset(j as isize) as libc::c_int as libc::c_float)
                as libc::c_int;
            if v > 255 as libc::c_int {
                v = 255 as libc::c_int;
            }
            *out.offset(j as isize) = v as byte;
            j += 1;
        }
        *out.offset(3 as libc::c_int as isize) = 255 as libc::c_int as byte;
        i += 1;
        in_0 = in_0.offset(4 as libc::c_int as isize);
        out = out.offset(4 as libc::c_int as isize);
    }
    R_GammaCorrectAndSetPalette(
        (palette[0 as libc::c_int as usize]).as_mut_ptr() as *const libc::c_uchar,
    );
}
#[no_mangle]
pub unsafe extern "C" fn R_SetLightLevel() {
    let mut light: vec3_t = [0.; 3];
    if r_newrefdef.rdflags & 2 as libc::c_int != 0 || (*r_drawentities).value == 0.
        || currententity.is_null()
    {
        (*r_lightlevel).value = 150.0f64 as libc::c_float;
        return;
    }
    R_LightPoint((r_newrefdef.vieworg).as_mut_ptr(), light.as_mut_ptr());
    (*r_lightlevel)
        .value = (150.0f64 * light[0 as libc::c_int as usize] as libc::c_double)
        as libc::c_float;
}
#[no_mangle]
pub unsafe extern "C" fn R_RenderFrame(mut fd: *mut refdef_t) {
    r_newrefdef = *fd;
    if r_worldmodel.is_null() && r_newrefdef.rdflags & 2 as libc::c_int == 0 {
        (ri.Sys_Error)
            .expect(
                "non-null function pointer",
            )(
            0 as libc::c_int,
            b"R_RenderView: NULL worldmodel\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    r_refdef
        .vieworg[0 as libc::c_int as usize] = (*fd).vieworg[0 as libc::c_int as usize];
    r_refdef
        .vieworg[1 as libc::c_int as usize] = (*fd).vieworg[1 as libc::c_int as usize];
    r_refdef
        .vieworg[2 as libc::c_int as usize] = (*fd).vieworg[2 as libc::c_int as usize];
    r_refdef
        .viewangles[0 as libc::c_int
        as usize] = (*fd).viewangles[0 as libc::c_int as usize];
    r_refdef
        .viewangles[1 as libc::c_int
        as usize] = (*fd).viewangles[1 as libc::c_int as usize];
    r_refdef
        .viewangles[2 as libc::c_int
        as usize] = (*fd).viewangles[2 as libc::c_int as usize];
    if (*r_speeds).value != 0. || (*r_dspeeds).value != 0. {
        r_time1 = Sys_Milliseconds() as libc::c_float;
    }
    R_SetupFrame();
    R_MarkLeaves();
    R_PushDlights(r_worldmodel);
    R_EdgeDrawing();
    if (*r_dspeeds).value != 0. {
        se_time2 = Sys_Milliseconds() as libc::c_float;
        de_time1 = se_time2;
    }
    R_DrawEntitiesOnList();
    if (*r_dspeeds).value != 0. {
        de_time2 = Sys_Milliseconds() as libc::c_float;
        dp_time1 = Sys_Milliseconds() as libc::c_float;
    }
    R_DrawParticles();
    if (*r_dspeeds).value != 0. {
        dp_time2 = Sys_Milliseconds() as libc::c_float;
    }
    R_DrawAlphaSurfaces();
    R_SetLightLevel();
    if r_dowarp as u64 != 0 {
        D_WarpScreen();
    }
    if (*r_dspeeds).value != 0. {
        da_time1 = Sys_Milliseconds() as libc::c_float;
    }
    if (*r_dspeeds).value != 0. {
        da_time2 = Sys_Milliseconds() as libc::c_float;
    }
    R_CalcPalette();
    if (*sw_aliasstats).value != 0. {
        R_PrintAliasStats();
    }
    if (*r_speeds).value != 0. {
        R_PrintTimes();
    }
    if (*r_dspeeds).value != 0. {
        R_PrintDSpeeds();
    }
    if (*sw_reportsurfout).value != 0. && r_outofsurfaces != 0 {
        (ri.Con_Printf)
            .expect(
                "non-null function pointer",
            )(
            0 as libc::c_int,
            b"Short %d surfaces\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            r_outofsurfaces,
        );
    }
    if (*sw_reportedgeout).value != 0. && r_outofedges != 0 {
        (ri.Con_Printf)
            .expect(
                "non-null function pointer",
            )(
            0 as libc::c_int,
            b"Short roughly %d edges\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            r_outofedges * 2 as libc::c_int / 3 as libc::c_int,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn R_InitGraphics(
    mut width: libc::c_int,
    mut height: libc::c_int,
) {
    vid.width = width;
    vid.height = height;
    if !d_pzbuffer.is_null() {
        free(d_pzbuffer as *mut libc::c_void);
        d_pzbuffer = 0 as *mut libc::c_short;
    }
    if !sc_base.is_null() {
        D_FlushCaches();
        free(sc_base as *mut libc::c_void);
        sc_base = 0 as *mut surfcache_t;
    }
    d_pzbuffer = malloc((vid.width * vid.height * 2 as libc::c_int) as libc::c_ulong)
        as *mut libc::c_short;
    R_InitCaches();
    R_GammaCorrectAndSetPalette(d_8to24table.as_mut_ptr() as *const libc::c_uchar);
}
#[no_mangle]
pub unsafe extern "C" fn R_BeginFrame(mut camera_separation: libc::c_float) {
    #[export_name = "Draw_BuildGammaTable"]
    pub unsafe extern "C" fn Draw_BuildGammaTable_0() {
        let mut i: libc::c_int = 0;
        let mut inf: libc::c_int = 0;
        let mut g: libc::c_float = 0.;
        g = (*vid_gamma).value;
        if g as libc::c_double == 1.0f64 {
            i = 0 as libc::c_int;
            while i < 256 as libc::c_int {
                sw_state.gammatable[i as usize] = i as byte;
                i += 1;
            }
            return;
        }
        i = 0 as libc::c_int;
        while i < 256 as libc::c_int {
            inf = (255 as libc::c_int as libc::c_double
                * pow((i as libc::c_double + 0.5f64) / 255.5f64, g as libc::c_double)
                + 0.5f64) as libc::c_int;
            if inf < 0 as libc::c_int {
                inf = 0 as libc::c_int;
            }
            if inf > 255 as libc::c_int {
                inf = 255 as libc::c_int;
            }
            sw_state.gammatable[i as usize] = inf as byte;
            i += 1;
        }
    }
    if (*vid_gamma).modified as u64 != 0 {
        Draw_BuildGammaTable_0();
        R_GammaCorrectAndSetPalette(d_8to24table.as_mut_ptr() as *const libc::c_uchar);
        (*vid_gamma).modified = false_0;
    }
    while (*sw_mode).modified as libc::c_uint != 0
        || (*vid_fullscreen).modified as libc::c_uint != 0
    {
        let mut err: rserr_t = rserr_ok;
        err = SWimp_SetMode(
            &mut vid.width,
            &mut vid.height,
            (*sw_mode).value as libc::c_int,
            (*vid_fullscreen).value as qboolean,
        );
        if err as libc::c_uint == rserr_ok as libc::c_int as libc::c_uint {
            R_InitGraphics(vid.width, vid.height);
            sw_state.prev_mode = (*sw_mode).value as libc::c_int;
            (*vid_fullscreen).modified = false_0;
            (*sw_mode).modified = false_0;
        } else if err as libc::c_uint
            == rserr_invalid_mode as libc::c_int as libc::c_uint
        {
            (ri.Cvar_SetValue)
                .expect(
                    "non-null function pointer",
                )(
                b"sw_mode\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                sw_state.prev_mode as libc::c_float,
            );
            (ri.Con_Printf)
                .expect(
                    "non-null function pointer",
                )(
                0 as libc::c_int,
                b"ref_soft::R_BeginFrame() - could not set mode\n\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
            );
        } else if err as libc::c_uint
            == rserr_invalid_fullscreen as libc::c_int as libc::c_uint
        {
            R_InitGraphics(vid.width, vid.height);
            (ri.Cvar_SetValue)
                .expect(
                    "non-null function pointer",
                )(
                b"vid_fullscreen\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                0 as libc::c_int as libc::c_float,
            );
            (ri.Con_Printf)
                .expect(
                    "non-null function pointer",
                )(
                0 as libc::c_int,
                b"ref_soft::R_BeginFrame() - fullscreen unavailable in this mode\n\0"
                    as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            sw_state.prev_mode = (*sw_mode).value as libc::c_int;
        } else {
            (ri.Sys_Error)
                .expect(
                    "non-null function pointer",
                )(
                0 as libc::c_int,
                b"ref_soft::R_BeginFrame() - catastrophic mode change failure\n\0"
                    as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn R_GammaCorrectAndSetPalette(mut palette: *const libc::c_uchar) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 256 as libc::c_int {
        sw_state
            .currentpalette[(i * 4 as libc::c_int + 0 as libc::c_int)
            as usize] = sw_state
            .gammatable[*palette
            .offset((i * 4 as libc::c_int + 0 as libc::c_int) as isize) as usize];
        sw_state
            .currentpalette[(i * 4 as libc::c_int + 1 as libc::c_int)
            as usize] = sw_state
            .gammatable[*palette
            .offset((i * 4 as libc::c_int + 1 as libc::c_int) as isize) as usize];
        sw_state
            .currentpalette[(i * 4 as libc::c_int + 2 as libc::c_int)
            as usize] = sw_state
            .gammatable[*palette
            .offset((i * 4 as libc::c_int + 2 as libc::c_int) as isize) as usize];
        i += 1;
    }
    SWimp_SetPalette((sw_state.currentpalette).as_mut_ptr());
}
#[no_mangle]
pub unsafe extern "C" fn R_CinematicSetPalette(mut palette: *const libc::c_uchar) {
    let mut palette32: [byte; 1024] = [0; 1024];
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut w: libc::c_int = 0;
    let mut d: *mut libc::c_int = 0 as *mut libc::c_int;
    w = abs(vid.rowbytes) >> 2 as libc::c_int;
    i = 0 as libc::c_int;
    while i < vid.height {
        d = (vid.buffer).offset((i * vid.rowbytes) as isize) as *mut libc::c_int;
        j = 0 as libc::c_int;
        while j < w {
            *d.offset(j as isize) = 0 as libc::c_int;
            j += 1;
        }
        i += 1;
        d = d.offset(w as isize);
    }
    SWimp_EndFrame();
    if !palette.is_null() {
        i = 0 as libc::c_int;
        while i < 256 as libc::c_int {
            palette32[(i * 4 as libc::c_int + 0 as libc::c_int)
                as usize] = *palette
                .offset((i * 3 as libc::c_int + 0 as libc::c_int) as isize);
            palette32[(i * 4 as libc::c_int + 1 as libc::c_int)
                as usize] = *palette
                .offset((i * 3 as libc::c_int + 1 as libc::c_int) as isize);
            palette32[(i * 4 as libc::c_int + 2 as libc::c_int)
                as usize] = *palette
                .offset((i * 3 as libc::c_int + 2 as libc::c_int) as isize);
            palette32[(i * 4 as libc::c_int + 3 as libc::c_int)
                as usize] = 0xff as libc::c_int as byte;
            i += 1;
        }
        R_GammaCorrectAndSetPalette(palette32.as_mut_ptr());
    } else {
        R_GammaCorrectAndSetPalette(d_8to24table.as_mut_ptr() as *const libc::c_uchar);
    };
}
#[no_mangle]
pub unsafe extern "C" fn R_DrawBeam(mut e: *mut entity_t) {
    let mut i: libc::c_int = 0;
    let mut perpvec: vec3_t = [0.; 3];
    let mut direction: vec3_t = [0.; 3];
    let mut normalized_direction: vec3_t = [0.; 3];
    let mut start_points: [vec3_t; 6] = [[0.; 3]; 6];
    let mut end_points: [vec3_t; 6] = [[0.; 3]; 6];
    let mut oldorigin: vec3_t = [0.; 3];
    let mut origin: vec3_t = [0.; 3];
    oldorigin[0 as libc::c_int as usize] = (*e).oldorigin[0 as libc::c_int as usize];
    oldorigin[1 as libc::c_int as usize] = (*e).oldorigin[1 as libc::c_int as usize];
    oldorigin[2 as libc::c_int as usize] = (*e).oldorigin[2 as libc::c_int as usize];
    origin[0 as libc::c_int as usize] = (*e).origin[0 as libc::c_int as usize];
    origin[1 as libc::c_int as usize] = (*e).origin[1 as libc::c_int as usize];
    origin[2 as libc::c_int as usize] = (*e).origin[2 as libc::c_int as usize];
    direction[0 as libc::c_int
        as usize] = oldorigin[0 as libc::c_int as usize]
        - origin[0 as libc::c_int as usize];
    normalized_direction[0 as libc::c_int
        as usize] = direction[0 as libc::c_int as usize];
    direction[1 as libc::c_int
        as usize] = oldorigin[1 as libc::c_int as usize]
        - origin[1 as libc::c_int as usize];
    normalized_direction[1 as libc::c_int
        as usize] = direction[1 as libc::c_int as usize];
    direction[2 as libc::c_int
        as usize] = oldorigin[2 as libc::c_int as usize]
        - origin[2 as libc::c_int as usize];
    normalized_direction[2 as libc::c_int
        as usize] = direction[2 as libc::c_int as usize];
    if VectorNormalize(normalized_direction.as_mut_ptr())
        == 0 as libc::c_int as libc::c_float
    {
        return;
    }
    PerpendicularVector(
        perpvec.as_mut_ptr(),
        normalized_direction.as_mut_ptr() as *const vec_t,
    );
    VectorScale(
        perpvec.as_mut_ptr(),
        ((*e).frame / 2 as libc::c_int) as vec_t,
        perpvec.as_mut_ptr(),
    );
    i = 0 as libc::c_int;
    while i < 6 as libc::c_int {
        RotatePointAroundVector(
            (start_points[i as usize]).as_mut_ptr(),
            normalized_direction.as_mut_ptr() as *const vec_t,
            perpvec.as_mut_ptr() as *const vec_t,
            (360.0f64 / 6 as libc::c_int as libc::c_double * i as libc::c_double)
                as libc::c_float,
        );
        start_points[i
            as usize][0 as libc::c_int
            as usize] = start_points[i as usize][0 as libc::c_int as usize]
            + origin[0 as libc::c_int as usize];
        start_points[i
            as usize][1 as libc::c_int
            as usize] = start_points[i as usize][1 as libc::c_int as usize]
            + origin[1 as libc::c_int as usize];
        start_points[i
            as usize][2 as libc::c_int
            as usize] = start_points[i as usize][2 as libc::c_int as usize]
            + origin[2 as libc::c_int as usize];
        end_points[i
            as usize][0 as libc::c_int
            as usize] = start_points[i as usize][0 as libc::c_int as usize]
            + direction[0 as libc::c_int as usize];
        end_points[i
            as usize][1 as libc::c_int
            as usize] = start_points[i as usize][1 as libc::c_int as usize]
            + direction[1 as libc::c_int as usize];
        end_points[i
            as usize][2 as libc::c_int
            as usize] = start_points[i as usize][2 as libc::c_int as usize]
            + direction[2 as libc::c_int as usize];
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < 6 as libc::c_int {
        R_IMFlatShadedQuad(
            (start_points[i as usize]).as_mut_ptr(),
            (end_points[i as usize]).as_mut_ptr(),
            (end_points[((i + 1 as libc::c_int) % 6 as libc::c_int) as usize])
                .as_mut_ptr(),
            (start_points[((i + 1 as libc::c_int) % 6 as libc::c_int) as usize])
                .as_mut_ptr(),
            (*e).skinnum & 0xff as libc::c_int,
            (*e).alpha,
        );
        i += 1;
    }
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
pub static mut r_skysideimage: [libc::c_int; 6] = [
    5 as libc::c_int,
    2 as libc::c_int,
    4 as libc::c_int,
    1 as libc::c_int,
    0 as libc::c_int,
    3 as libc::c_int,
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
        Com_sprintf(
            pathname.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
            b"env/%s%s.pcx\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            skyname.as_mut_ptr(),
            suf[r_skysideimage[i as usize] as usize],
        );
        r_skytexinfo[i as usize].image = R_FindImage(pathname.as_mut_ptr(), it_sky);
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn Draw_GetPalette() {
    let mut pal: *mut byte = 0 as *mut byte;
    let mut out: *mut byte = 0 as *mut byte;
    let mut i: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    let mut g: libc::c_int = 0;
    let mut b: libc::c_int = 0;
    LoadPCX(
        b"pics/colormap.pcx\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        &mut vid.colormap,
        &mut pal,
        0 as *mut libc::c_int,
        0 as *mut libc::c_int,
    );
    if (vid.colormap).is_null() {
        (ri.Sys_Error)
            .expect(
                "non-null function pointer",
            )(
            0 as libc::c_int,
            b"Couldn't load pics/colormap.pcx\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    vid
        .alphamap = (vid.colormap)
        .offset((64 as libc::c_int * 256 as libc::c_int) as isize);
    out = d_8to24table.as_mut_ptr() as *mut byte;
    i = 0 as libc::c_int;
    while i < 256 as libc::c_int {
        r = *pal.offset((i * 3 as libc::c_int + 0 as libc::c_int) as isize)
            as libc::c_int;
        g = *pal.offset((i * 3 as libc::c_int + 1 as libc::c_int) as isize)
            as libc::c_int;
        b = *pal.offset((i * 3 as libc::c_int + 2 as libc::c_int) as isize)
            as libc::c_int;
        *out.offset(0 as libc::c_int as isize) = r as byte;
        *out.offset(1 as libc::c_int as isize) = g as byte;
        *out.offset(2 as libc::c_int as isize) = b as byte;
        i += 1;
        out = out.offset(4 as libc::c_int as isize);
    }
    free(pal as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn GetRefAPI(mut rimp: refimport_t) -> refexport_t {
    let mut re: refexport_t = refexport_t {
        api_version: 0,
        Init: None,
        Shutdown: None,
        BeginRegistration: None,
        RegisterModel: None,
        RegisterSkin: None,
        RegisterPic: None,
        SetSky: None,
        EndRegistration: None,
        RenderFrame: None,
        DrawGetPicSize: None,
        DrawPic: None,
        DrawStretchPic: None,
        DrawChar: None,
        DrawTileClear: None,
        DrawFill: None,
        DrawFadeScreen: None,
        DrawStretchRaw: None,
        CinematicSetPalette: None,
        BeginFrame: None,
        EndFrame: None,
        AppActivate: None,
    };
    ri = rimp;
    re.api_version = 3 as libc::c_int;
    re
        .BeginRegistration = Some(
        R_BeginRegistration as unsafe extern "C" fn(*mut libc::c_char) -> (),
    );
    re
        .RegisterModel = Some(
        R_RegisterModel as unsafe extern "C" fn(*mut libc::c_char) -> *mut model_s,
    );
    re
        .RegisterSkin = Some(
        R_RegisterSkin as unsafe extern "C" fn(*mut libc::c_char) -> *mut image_s,
    );
    re
        .RegisterPic = Some(
        Draw_FindPic as unsafe extern "C" fn(*mut libc::c_char) -> *mut image_s,
    );
    re
        .SetSky = Some(
        R_SetSky
            as unsafe extern "C" fn(*mut libc::c_char, libc::c_float, *mut vec_t) -> (),
    );
    re.EndRegistration = Some(R_EndRegistration as unsafe extern "C" fn() -> ());
    re.RenderFrame = Some(R_RenderFrame as unsafe extern "C" fn(*mut refdef_t) -> ());
    re
        .DrawGetPicSize = Some(
        Draw_GetPicSize
            as unsafe extern "C" fn(
                *mut libc::c_int,
                *mut libc::c_int,
                *mut libc::c_char,
            ) -> (),
    );
    re
        .DrawPic = Some(
        Draw_Pic
            as unsafe extern "C" fn(libc::c_int, libc::c_int, *mut libc::c_char) -> (),
    );
    re
        .DrawStretchPic = Some(
        Draw_StretchPic
            as unsafe extern "C" fn(
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                *mut libc::c_char,
            ) -> (),
    );
    re
        .DrawChar = Some(
        Draw_Char as unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int) -> (),
    );
    re
        .DrawTileClear = Some(
        Draw_TileClear
            as unsafe extern "C" fn(
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                *mut libc::c_char,
            ) -> (),
    );
    re
        .DrawFill = Some(
        Draw_Fill
            as unsafe extern "C" fn(
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
            ) -> (),
    );
    re.DrawFadeScreen = Some(Draw_FadeScreen as unsafe extern "C" fn() -> ());
    re
        .DrawStretchRaw = Some(
        Draw_StretchRaw
            as unsafe extern "C" fn(
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                libc::c_int,
                *mut byte,
            ) -> (),
    );
    re
        .Init = Some(
        R_Init as unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> qboolean,
    );
    re.Shutdown = Some(R_Shutdown as unsafe extern "C" fn() -> ());
    re
        .CinematicSetPalette = Some(
        R_CinematicSetPalette as unsafe extern "C" fn(*const libc::c_uchar) -> (),
    );
    re.BeginFrame = Some(R_BeginFrame as unsafe extern "C" fn(libc::c_float) -> ());
    re.EndFrame = Some(SWimp_EndFrame as unsafe extern "C" fn() -> ());
    re.AppActivate = Some(SWimp_AppActivate as unsafe extern "C" fn(qboolean) -> ());
    Swap_Init();
    return re;
}
#[no_mangle]
pub unsafe extern "C" fn Sys_Error(mut error: *mut libc::c_char, mut args: ...) {
    let mut argptr: ::std::ffi::VaListImpl;
    let mut text: [libc::c_char; 1024] = [0; 1024];
    argptr = args.clone();
    vsprintf(text.as_mut_ptr(), error, argptr.as_va_list());
    (ri.Sys_Error)
        .expect(
            "non-null function pointer",
        )(
        0 as libc::c_int,
        b"%s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        text.as_mut_ptr(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn Com_Printf(mut fmt: *mut libc::c_char, mut args: ...) {
    let mut argptr: ::std::ffi::VaListImpl;
    let mut text: [libc::c_char; 1024] = [0; 1024];
    argptr = args.clone();
    vsprintf(text.as_mut_ptr(), fmt, argptr.as_va_list());
    (ri.Con_Printf)
        .expect(
            "non-null function pointer",
        )(
        0 as libc::c_int,
        b"%s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        text.as_mut_ptr(),
    );
}
