#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(c_variadic, register_tool)]
extern "C" {
    fn putenv(__string: *mut libc::c_char) -> libc::c_int;
    fn vsprintf(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: ::std::ffi::VaList,
    ) -> libc::c_int;
    fn cos(_: libc::c_double) -> libc::c_double;
    fn sin(_: libc::c_double) -> libc::c_double;
    fn ceil(_: libc::c_double) -> libc::c_double;
    fn floor(_: libc::c_double) -> libc::c_double;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
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
    fn QGL_Init(dllname: *const libc::c_char) -> qboolean;
    fn QGL_Shutdown();
    static mut qglClearColor: Option::<
        unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int, libc::c_int) -> (),
    >;
    static mut qglColor3f: Option::<
        unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int) -> (),
    >;
    static mut qglColor3fv: Option::<unsafe extern "C" fn(*const libc::c_int) -> ()>;
    static mut qglColor4f: Option::<
        unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int, libc::c_int) -> (),
    >;
    static mut qglColor4fv: Option::<unsafe extern "C" fn(*const libc::c_int) -> ()>;
    static mut qglColor4ubv: Option::<unsafe extern "C" fn(*const libc::c_int) -> ()>;
    static mut qglDepthMask: Option::<unsafe extern "C" fn(libc::c_int) -> ()>;
    static mut qglDepthRange: Option::<
        unsafe extern "C" fn(libc::c_int, libc::c_int) -> (),
    >;
    static mut qglEnd: Option::<unsafe extern "C" fn() -> ()>;
    static mut qglFinish: Option::<unsafe extern "C" fn() -> ()>;
    fn GLimp_AppActivate(active: qboolean);
    fn GLimp_EndFrame();
    fn R_RegisterSkin(name: *mut libc::c_char) -> *mut image_s;
    fn GL_SetTexturePalette(palette: *mut libc::c_uint);
    fn GLimp_EnableLogging(enable: qboolean);
    fn GLimp_LogNewFrame();
    fn GLimp_BeginFrame(camera_separation: libc::c_float);
    fn GL_TextureMode(string: *mut libc::c_char);
    fn GL_TextureAlphaMode(string: *mut libc::c_char);
    fn GL_TextureSolidMode(string: *mut libc::c_char);
    fn Draw_StretchRaw(
        x: libc::c_int,
        y: libc::c_int,
        w: libc::c_int,
        h: libc::c_int,
        cols: libc::c_int,
        rows: libc::c_int,
        data: *mut byte,
    );
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
    fn R_MarkLeaves();
    fn R_DrawWorld();
    fn R_DrawAliasModel(e: *mut entity_t);
    fn R_DrawBrushModel(e: *mut entity_t);
    fn R_RenderDlights();
    fn R_DrawAlphaSurfaces();
    fn GL_ShutdownImages();
    fn GLimp_Shutdown();
    fn Draw_GetPalette() -> libc::c_int;
    fn GL_ImageList_f();
    fn GL_ScreenShot_f();
    fn GLimp_Init(hinstance: *mut libc::c_void, hWnd: *mut libc::c_void) -> libc::c_int;
    fn GLimp_SetMode(
        pwidth: *mut libc::c_int,
        pheight: *mut libc::c_int,
        mode: libc::c_int,
        fullscreen: qboolean,
    ) -> libc::c_int;
    fn GL_InitImages();
    fn R_InitParticleTexture();
    fn Draw_InitLocal();
    static mut d_8to24table: [libc::c_uint; 256];
    fn R_PushDlights();
    fn R_LightPoint(p: *mut vec_t, color: *mut vec_t);
    fn GL_Bind(texnum: libc::c_int);
    static mut c_visible_textures: libc::c_int;
    static mut c_visible_lightmaps: libc::c_int;
    fn GL_UpdateSwapInterval();
    fn GL_SetDefaultState();
    fn Mod_FreeAll();
    fn Mod_Modellist_f();
    fn Mod_PointInLeaf(p: *mut libc::c_float, model: *mut model_t) -> *mut mleaf_t;
    fn Mod_Init();
    static mut qglRotatef: Option::<
        unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int, libc::c_int) -> (),
    >;
    static mut qglScissor: Option::<
        unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int, libc::c_int) -> (),
    >;
    static mut qglTexCoord2f: Option::<
        unsafe extern "C" fn(libc::c_int, libc::c_int) -> (),
    >;
    static mut qglTranslatef: Option::<
        unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int) -> (),
    >;
    static mut qglVertex3f: Option::<
        unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int) -> (),
    >;
    static mut qglVertex3fv: Option::<unsafe extern "C" fn(*const libc::c_int) -> ()>;
    static mut qglPushMatrix: Option::<unsafe extern "C" fn() -> ()>;
    static mut qglPopMatrix: Option::<unsafe extern "C" fn() -> ()>;
    static mut qglPointSize: Option::<unsafe extern "C" fn(libc::c_int) -> ()>;
    static mut qglLoadIdentity: Option::<unsafe extern "C" fn() -> ()>;
    static mut qglOrtho: Option::<
        unsafe extern "C" fn(
            libc::c_int,
            libc::c_int,
            libc::c_int,
            libc::c_int,
            libc::c_int,
            libc::c_int,
        ) -> (),
    >;
    static mut qglPointParameterfEXT: Option::<
        unsafe extern "C" fn(libc::c_int, libc::c_int) -> (),
    >;
    static mut qglViewport: Option::<
        unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int, libc::c_int) -> (),
    >;
    fn GL_Strings_f();
    fn toupper(_: libc::c_int) -> libc::c_int;
    fn R_BeginRegistration(map: *mut libc::c_char);
    fn R_RegisterModel(name: *mut libc::c_char) -> *mut model_s;
    fn R_SetSky(name: *mut libc::c_char, rotate: libc::c_float, axis: *mut vec_t);
    fn R_EndRegistration();
    fn Draw_FindPic(name: *mut libc::c_char) -> *mut image_s;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct viddef_t {
    pub width: libc::c_uint,
    pub height: libc::c_uint,
}
pub type rserr_t = libc::c_uint;
pub const rserr_unknown: rserr_t = 3;
pub const rserr_invalid_mode: rserr_t = 2;
pub const rserr_invalid_fullscreen: rserr_t = 1;
pub const rserr_ok: rserr_t = 0;
pub type model_t = model_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glconfig_t {
    pub renderer: libc::c_int,
    pub renderer_string: *const libc::c_char,
    pub vendor_string: *const libc::c_char,
    pub version_string: *const libc::c_char,
    pub extensions_string: *const libc::c_char,
    pub allow_cds: qboolean,
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
#[no_mangle]
pub static mut vid: viddef_t = viddef_t { width: 0, height: 0 };
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
pub static mut r_worldmodel: *mut model_t = 0 as *const model_t as *mut model_t;
#[no_mangle]
pub static mut gldepthmin: libc::c_float = 0.;
#[no_mangle]
pub static mut gldepthmax: libc::c_float = 0.;
#[no_mangle]
pub static mut gl_config: glconfig_t = glconfig_t {
    renderer: 0,
    renderer_string: 0 as *const libc::c_char,
    vendor_string: 0 as *const libc::c_char,
    version_string: 0 as *const libc::c_char,
    extensions_string: 0 as *const libc::c_char,
    allow_cds: false_0,
};
#[no_mangle]
pub static mut gl_state: glstate_t = glstate_t {
    inverse_intensity: 0.,
    fullscreen: false_0,
    prev_mode: 0,
    d_16to8table: 0 as *const libc::c_uchar as *mut libc::c_uchar,
    lightmap_textures: 0,
    currenttextures: [0; 2],
    currenttmu: 0,
    camera_separation: 0.,
    stereo_enabled: false_0,
    originalRedGammaTable: [0; 256],
    originalGreenGammaTable: [0; 256],
    originalBlueGammaTable: [0; 256],
};
#[no_mangle]
pub static mut r_notexture: *mut image_t = 0 as *const image_t as *mut image_t;
#[no_mangle]
pub static mut r_particletexture: *mut image_t = 0 as *const image_t as *mut image_t;
#[no_mangle]
pub static mut currententity: *mut entity_t = 0 as *const entity_t as *mut entity_t;
#[no_mangle]
pub static mut currentmodel: *mut model_t = 0 as *const model_t as *mut model_t;
#[no_mangle]
pub static mut frustum: [cplane_t; 4] = [cplane_t {
    normal: [0.; 3],
    dist: 0.,
    type_0: 0,
    signbits: 0,
    pad: [0; 2],
}; 4];
#[no_mangle]
pub static mut r_visframecount: libc::c_int = 0;
#[no_mangle]
pub static mut r_framecount: libc::c_int = 0;
#[no_mangle]
pub static mut c_brush_polys: libc::c_int = 0;
#[no_mangle]
pub static mut c_alias_polys: libc::c_int = 0;
#[no_mangle]
pub static mut v_blend: [libc::c_float; 4] = [0.; 4];
#[no_mangle]
pub static mut vup: vec3_t = [0.; 3];
#[no_mangle]
pub static mut vpn: vec3_t = [0.; 3];
#[no_mangle]
pub static mut vright: vec3_t = [0.; 3];
#[no_mangle]
pub static mut r_origin: vec3_t = [0.; 3];
#[no_mangle]
pub static mut r_world_matrix: [libc::c_float; 16] = [0.; 16];
#[no_mangle]
pub static mut r_base_world_matrix: [libc::c_float; 16] = [0.; 16];
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
pub static mut r_viewcluster: libc::c_int = 0;
#[no_mangle]
pub static mut r_viewcluster2: libc::c_int = 0;
#[no_mangle]
pub static mut r_oldviewcluster: libc::c_int = 0;
#[no_mangle]
pub static mut r_oldviewcluster2: libc::c_int = 0;
#[no_mangle]
pub static mut r_norefresh: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut r_drawentities: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut r_drawworld: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut r_speeds: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut r_fullbright: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut r_novis: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut r_nocull: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut r_lerpmodels: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut r_lefthand: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut r_lightlevel: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut gl_nosubimage: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut gl_allow_software: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut gl_vertex_arrays: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut gl_particle_min_size: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut gl_particle_max_size: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut gl_particle_size: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut gl_particle_att_a: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut gl_particle_att_b: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut gl_particle_att_c: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut gl_ext_swapinterval: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut gl_ext_palettedtexture: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut gl_ext_multitexture: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut gl_ext_pointparameters: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut gl_ext_compiled_vertex_array: *mut cvar_t = 0 as *const cvar_t
    as *mut cvar_t;
#[no_mangle]
pub static mut gl_log: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut gl_bitdepth: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut gl_drawbuffer: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut gl_driver: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut gl_lightmap: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut gl_shadows: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut gl_mode: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut gl_dynamic: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut gl_monolightmap: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut gl_modulate: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut gl_nobind: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut gl_round_down: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut gl_picmip: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut gl_skymip: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut gl_showtris: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut gl_ztrick: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut gl_finish: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut gl_clear: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut gl_cull: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut gl_polyblend: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut gl_flashblend: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut gl_playermip: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut gl_saturatelighting: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut gl_swapinterval: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut gl_texturemode: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut gl_texturealphamode: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut gl_texturesolidmode: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut gl_lockpvs: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut gl_3dlabs_broken: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut vid_fullscreen: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut vid_gamma: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut vid_ref: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub unsafe extern "C" fn R_CullBox(
    mut mins: *mut vec_t,
    mut maxs: *mut vec_t,
) -> qboolean {
    let mut i: libc::c_int = 0;
    if (*r_nocull).value != 0. {
        return false_0;
    }
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        if (if (frustum[i as usize].type_0 as libc::c_int) < 3 as libc::c_int {
            (if frustum[i as usize].dist
                <= *mins.offset(frustum[i as usize].type_0 as isize)
            {
                1 as libc::c_int
            } else {
                (if frustum[i as usize].dist
                    >= *maxs.offset(frustum[i as usize].type_0 as isize)
                {
                    2 as libc::c_int
                } else {
                    3 as libc::c_int
                })
            })
        } else {
            BoxOnPlaneSide(mins, maxs, &mut *frustum.as_mut_ptr().offset(i as isize))
        }) == 2 as libc::c_int
        {
            return true_0;
        }
        i += 1;
    }
    return false_0;
}
#[no_mangle]
pub unsafe extern "C" fn R_RotateForEntity(mut e: *mut entity_t) {
    qglTranslatef
        .expect(
            "non-null function pointer",
        )(
        (*e).origin[0 as libc::c_int as usize] as libc::c_int,
        (*e).origin[1 as libc::c_int as usize] as libc::c_int,
        (*e).origin[2 as libc::c_int as usize] as libc::c_int,
    );
    qglRotatef
        .expect(
            "non-null function pointer",
        )(
        (*e).angles[1 as libc::c_int as usize] as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        1 as libc::c_int,
    );
    qglRotatef
        .expect(
            "non-null function pointer",
        )(
        -(*e).angles[0 as libc::c_int as usize] as libc::c_int,
        0 as libc::c_int,
        1 as libc::c_int,
        0 as libc::c_int,
    );
    qglRotatef
        .expect(
            "non-null function pointer",
        )(
        -(*e).angles[2 as libc::c_int as usize] as libc::c_int,
        1 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn R_DrawSpriteModel(mut e: *mut entity_t) {
    let mut alpha: libc::c_float = 1.0f32;
    let mut point: vec3_t = [0.; 3];
    let mut frame: *mut dsprframe_t = 0 as *mut dsprframe_t;
    let mut up: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut right: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut psprite: *mut dsprite_t = 0 as *mut dsprite_t;
    psprite = (*currentmodel).extradata as *mut dsprite_t;
    (*e).frame %= (*psprite).numframes;
    frame = &mut *((*psprite).frames).as_mut_ptr().offset((*e).frame as isize)
        as *mut dsprframe_t;
    up = vup.as_mut_ptr();
    right = vright.as_mut_ptr();
    if (*e).flags & 32 as libc::c_int != 0 {
        alpha = (*e).alpha;
    }
    qglColor4f
        .expect(
            "non-null function pointer",
        )(1 as libc::c_int, 1 as libc::c_int, 1 as libc::c_int, alpha as libc::c_int);
    GL_Bind((*(*currentmodel).skins[(*e).frame as usize]).texnum);
    qglTexCoord2f
        .expect("non-null function pointer")(0 as libc::c_int, 1 as libc::c_int);
    VectorMA(
        ((*e).origin).as_mut_ptr(),
        -(*frame).origin_y as libc::c_float,
        up,
        point.as_mut_ptr(),
    );
    VectorMA(
        point.as_mut_ptr(),
        -(*frame).origin_x as libc::c_float,
        right,
        point.as_mut_ptr(),
    );
    qglVertex3fv
        .expect("non-null function pointer")(point.as_mut_ptr() as *const libc::c_int);
    qglTexCoord2f
        .expect("non-null function pointer")(0 as libc::c_int, 0 as libc::c_int);
    VectorMA(
        ((*e).origin).as_mut_ptr(),
        ((*frame).height - (*frame).origin_y) as libc::c_float,
        up,
        point.as_mut_ptr(),
    );
    VectorMA(
        point.as_mut_ptr(),
        -(*frame).origin_x as libc::c_float,
        right,
        point.as_mut_ptr(),
    );
    qglVertex3fv
        .expect("non-null function pointer")(point.as_mut_ptr() as *const libc::c_int);
    qglTexCoord2f
        .expect("non-null function pointer")(1 as libc::c_int, 0 as libc::c_int);
    VectorMA(
        ((*e).origin).as_mut_ptr(),
        ((*frame).height - (*frame).origin_y) as libc::c_float,
        up,
        point.as_mut_ptr(),
    );
    VectorMA(
        point.as_mut_ptr(),
        ((*frame).width - (*frame).origin_x) as libc::c_float,
        right,
        point.as_mut_ptr(),
    );
    qglVertex3fv
        .expect("non-null function pointer")(point.as_mut_ptr() as *const libc::c_int);
    qglTexCoord2f
        .expect("non-null function pointer")(1 as libc::c_int, 1 as libc::c_int);
    VectorMA(
        ((*e).origin).as_mut_ptr(),
        -(*frame).origin_y as libc::c_float,
        up,
        point.as_mut_ptr(),
    );
    VectorMA(
        point.as_mut_ptr(),
        ((*frame).width - (*frame).origin_x) as libc::c_float,
        right,
        point.as_mut_ptr(),
    );
    qglVertex3fv
        .expect("non-null function pointer")(point.as_mut_ptr() as *const libc::c_int);
    qglEnd.expect("non-null function pointer")();
    qglColor4f
        .expect(
            "non-null function pointer",
        )(1 as libc::c_int, 1 as libc::c_int, 1 as libc::c_int, 1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn R_DrawNullModel() {
    let mut shadelight: vec3_t = [0.; 3];
    let mut i: libc::c_int = 0;
    if (*currententity).flags & 8 as libc::c_int != 0 {
        shadelight[2 as libc::c_int as usize] = 1.0f32;
        shadelight[1 as libc::c_int as usize] = shadelight[2 as libc::c_int as usize];
        shadelight[0 as libc::c_int as usize] = shadelight[1 as libc::c_int as usize];
    } else {
        R_LightPoint(((*currententity).origin).as_mut_ptr(), shadelight.as_mut_ptr());
    }
    qglPushMatrix.expect("non-null function pointer")();
    R_RotateForEntity(currententity);
    qglColor3fv
        .expect(
            "non-null function pointer",
        )(shadelight.as_mut_ptr() as *const libc::c_int);
    qglVertex3f
        .expect(
            "non-null function pointer",
        )(0 as libc::c_int, 0 as libc::c_int, -(16 as libc::c_int));
    i = 0 as libc::c_int;
    while i <= 4 as libc::c_int {
        qglVertex3f
            .expect(
                "non-null function pointer",
            )(
            (16 as libc::c_int as libc::c_double
                * cos(
                    i as libc::c_double * 3.14159265358979323846f64
                        / 2 as libc::c_int as libc::c_double,
                )) as libc::c_int,
            (16 as libc::c_int as libc::c_double
                * sin(
                    i as libc::c_double * 3.14159265358979323846f64
                        / 2 as libc::c_int as libc::c_double,
                )) as libc::c_int,
            0 as libc::c_int,
        );
        i += 1;
    }
    qglEnd.expect("non-null function pointer")();
    qglVertex3f
        .expect(
            "non-null function pointer",
        )(0 as libc::c_int, 0 as libc::c_int, 16 as libc::c_int);
    i = 4 as libc::c_int;
    while i >= 0 as libc::c_int {
        qglVertex3f
            .expect(
                "non-null function pointer",
            )(
            (16 as libc::c_int as libc::c_double
                * cos(
                    i as libc::c_double * 3.14159265358979323846f64
                        / 2 as libc::c_int as libc::c_double,
                )) as libc::c_int,
            (16 as libc::c_int as libc::c_double
                * sin(
                    i as libc::c_double * 3.14159265358979323846f64
                        / 2 as libc::c_int as libc::c_double,
                )) as libc::c_int,
            0 as libc::c_int,
        );
        i -= 1;
    }
    qglEnd.expect("non-null function pointer")();
    qglColor3f
        .expect(
            "non-null function pointer",
        )(1 as libc::c_int, 1 as libc::c_int, 1 as libc::c_int);
    qglPopMatrix.expect("non-null function pointer")();
}
#[no_mangle]
pub unsafe extern "C" fn R_DrawEntitiesOnList() {
    let mut i: libc::c_int = 0;
    if (*r_drawentities).value == 0. {
        return;
    }
    i = 0 as libc::c_int;
    while i < r_newrefdef.num_entities {
        currententity = &mut *(r_newrefdef.entities).offset(i as isize) as *mut entity_t;
        if !((*currententity).flags & 32 as libc::c_int != 0) {
            if (*currententity).flags & 128 as libc::c_int != 0 {
                R_DrawBeam(currententity);
            } else {
                currentmodel = (*currententity).model;
                if currentmodel.is_null() {
                    R_DrawNullModel();
                } else {
                    match (*currentmodel).type_0 as libc::c_uint {
                        3 => {
                            R_DrawAliasModel(currententity);
                        }
                        1 => {
                            R_DrawBrushModel(currententity);
                        }
                        2 => {
                            R_DrawSpriteModel(currententity);
                        }
                        _ => {
                            (ri.Sys_Error)
                                .expect(
                                    "non-null function pointer",
                                )(
                                1 as libc::c_int,
                                b"Bad modeltype\0" as *const u8 as *const libc::c_char
                                    as *mut libc::c_char,
                            );
                        }
                    }
                }
            }
        }
        i += 1;
    }
    qglDepthMask.expect("non-null function pointer")(0 as libc::c_int);
    i = 0 as libc::c_int;
    while i < r_newrefdef.num_entities {
        currententity = &mut *(r_newrefdef.entities).offset(i as isize) as *mut entity_t;
        if !((*currententity).flags & 32 as libc::c_int == 0) {
            if (*currententity).flags & 128 as libc::c_int != 0 {
                R_DrawBeam(currententity);
            } else {
                currentmodel = (*currententity).model;
                if currentmodel.is_null() {
                    R_DrawNullModel();
                } else {
                    match (*currentmodel).type_0 as libc::c_uint {
                        3 => {
                            R_DrawAliasModel(currententity);
                        }
                        1 => {
                            R_DrawBrushModel(currententity);
                        }
                        2 => {
                            R_DrawSpriteModel(currententity);
                        }
                        _ => {
                            (ri.Sys_Error)
                                .expect(
                                    "non-null function pointer",
                                )(
                                1 as libc::c_int,
                                b"Bad modeltype\0" as *const u8 as *const libc::c_char
                                    as *mut libc::c_char,
                            );
                        }
                    }
                }
            }
        }
        i += 1;
    }
    qglDepthMask.expect("non-null function pointer")(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn GL_DrawParticles(
    mut num_particles: libc::c_int,
    mut particles: *const particle_t,
    mut colortable: *const libc::c_uint,
) {
    let mut p: *const particle_t = 0 as *const particle_t;
    let mut i: libc::c_int = 0;
    let mut up: vec3_t = [0.; 3];
    let mut right: vec3_t = [0.; 3];
    let mut scale: libc::c_float = 0.;
    let mut color: [byte; 4] = [0; 4];
    GL_Bind((*r_particletexture).texnum);
    VectorScale(vup.as_mut_ptr(), 1.5f64 as vec_t, up.as_mut_ptr());
    VectorScale(vright.as_mut_ptr(), 1.5f64 as vec_t, right.as_mut_ptr());
    p = particles;
    i = 0 as libc::c_int;
    while i < num_particles {
        scale = ((*p).origin[0 as libc::c_int as usize]
            - r_origin[0 as libc::c_int as usize]) * vpn[0 as libc::c_int as usize]
            + ((*p).origin[1 as libc::c_int as usize]
                - r_origin[1 as libc::c_int as usize]) * vpn[1 as libc::c_int as usize]
            + ((*p).origin[2 as libc::c_int as usize]
                - r_origin[2 as libc::c_int as usize]) * vpn[2 as libc::c_int as usize];
        if scale < 20 as libc::c_int as libc::c_float {
            scale = 1 as libc::c_int as libc::c_float;
        } else {
            scale = (1 as libc::c_int as libc::c_double
                + scale as libc::c_double * 0.004f64) as libc::c_float;
        }
        *(color.as_mut_ptr()
            as *mut libc::c_int) = *colortable.offset((*p).color as isize)
            as libc::c_int;
        color[3 as libc::c_int
            as usize] = ((*p).alpha * 255 as libc::c_int as libc::c_float) as byte;
        qglColor4ubv
            .expect(
                "non-null function pointer",
            )(color.as_mut_ptr() as *const libc::c_int);
        qglTexCoord2f
            .expect(
                "non-null function pointer",
            )(0.0625f64 as libc::c_int, 0.0625f64 as libc::c_int);
        qglVertex3fv
            .expect(
                "non-null function pointer",
            )(((*p).origin).as_ptr() as *const libc::c_int);
        qglTexCoord2f
            .expect(
                "non-null function pointer",
            )(1.0625f64 as libc::c_int, 0.0625f64 as libc::c_int);
        qglVertex3f
            .expect(
                "non-null function pointer",
            )(
            ((*p).origin[0 as libc::c_int as usize]
                + up[0 as libc::c_int as usize] * scale) as libc::c_int,
            ((*p).origin[1 as libc::c_int as usize]
                + up[1 as libc::c_int as usize] * scale) as libc::c_int,
            ((*p).origin[2 as libc::c_int as usize]
                + up[2 as libc::c_int as usize] * scale) as libc::c_int,
        );
        qglTexCoord2f
            .expect(
                "non-null function pointer",
            )(0.0625f64 as libc::c_int, 1.0625f64 as libc::c_int);
        qglVertex3f
            .expect(
                "non-null function pointer",
            )(
            ((*p).origin[0 as libc::c_int as usize]
                + right[0 as libc::c_int as usize] * scale) as libc::c_int,
            ((*p).origin[1 as libc::c_int as usize]
                + right[1 as libc::c_int as usize] * scale) as libc::c_int,
            ((*p).origin[2 as libc::c_int as usize]
                + right[2 as libc::c_int as usize] * scale) as libc::c_int,
        );
        i += 1;
        p = p.offset(1);
    }
    qglEnd.expect("non-null function pointer")();
    qglColor4f
        .expect(
            "non-null function pointer",
        )(1 as libc::c_int, 1 as libc::c_int, 1 as libc::c_int, 1 as libc::c_int);
    qglDepthMask.expect("non-null function pointer")(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn R_DrawParticles() {
    if (*gl_ext_pointparameters).value != 0. && qglPointParameterfEXT.is_some() {
        let mut i: libc::c_int = 0;
        let mut color: [libc::c_uchar; 4] = [0; 4];
        let mut p: *const particle_t = 0 as *const particle_t;
        qglPointSize
            .expect(
                "non-null function pointer",
            )((*gl_particle_size).value as libc::c_int);
        i = 0 as libc::c_int;
        p = r_newrefdef.particles;
        while i < r_newrefdef.num_particles {
            *(color.as_mut_ptr()
                as *mut libc::c_int) = d_8to24table[(*p).color as usize] as libc::c_int;
            color[3 as libc::c_int
                as usize] = ((*p).alpha * 255 as libc::c_int as libc::c_float)
                as libc::c_uchar;
            qglColor4ubv
                .expect(
                    "non-null function pointer",
                )(color.as_mut_ptr() as *const libc::c_int);
            qglVertex3fv
                .expect(
                    "non-null function pointer",
                )(((*p).origin).as_ptr() as *const libc::c_int);
            i += 1;
            p = p.offset(1);
        }
        qglEnd.expect("non-null function pointer")();
        qglColor4f
            .expect(
                "non-null function pointer",
            )(
            1.0f32 as libc::c_int,
            1.0f32 as libc::c_int,
            1.0f32 as libc::c_int,
            1.0f32 as libc::c_int,
        );
    } else {
        GL_DrawParticles(
            r_newrefdef.num_particles,
            r_newrefdef.particles as *const particle_t,
            d_8to24table.as_mut_ptr() as *const libc::c_uint,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn R_PolyBlend() {
    if (*gl_polyblend).value == 0. {
        return;
    }
    if v_blend[3 as libc::c_int as usize] == 0. {
        return;
    }
    qglLoadIdentity.expect("non-null function pointer")();
    qglRotatef
        .expect(
            "non-null function pointer",
        )(-(90 as libc::c_int), 1 as libc::c_int, 0 as libc::c_int, 0 as libc::c_int);
    qglRotatef
        .expect(
            "non-null function pointer",
        )(90 as libc::c_int, 0 as libc::c_int, 0 as libc::c_int, 1 as libc::c_int);
    qglColor4fv
        .expect("non-null function pointer")(v_blend.as_mut_ptr() as *const libc::c_int);
    qglVertex3f
        .expect(
            "non-null function pointer",
        )(10 as libc::c_int, 100 as libc::c_int, 100 as libc::c_int);
    qglVertex3f
        .expect(
            "non-null function pointer",
        )(10 as libc::c_int, -(100 as libc::c_int), 100 as libc::c_int);
    qglVertex3f
        .expect(
            "non-null function pointer",
        )(10 as libc::c_int, -(100 as libc::c_int), -(100 as libc::c_int));
    qglVertex3f
        .expect(
            "non-null function pointer",
        )(10 as libc::c_int, 100 as libc::c_int, -(100 as libc::c_int));
    qglEnd.expect("non-null function pointer")();
    qglColor4f
        .expect(
            "non-null function pointer",
        )(1 as libc::c_int, 1 as libc::c_int, 1 as libc::c_int, 1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn SignbitsForPlane(mut out: *mut cplane_t) -> libc::c_int {
    let mut bits: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    bits = 0 as libc::c_int;
    j = 0 as libc::c_int;
    while j < 3 as libc::c_int {
        if (*out).normal[j as usize] < 0 as libc::c_int as libc::c_float {
            bits |= (1 as libc::c_int) << j;
        }
        j += 1;
    }
    return bits;
}
#[no_mangle]
pub unsafe extern "C" fn R_SetFrustum() {
    let mut i: libc::c_int = 0;
    RotatePointAroundVector(
        (frustum[0 as libc::c_int as usize].normal).as_mut_ptr(),
        vup.as_mut_ptr() as *const vec_t,
        vpn.as_mut_ptr() as *const vec_t,
        -(90 as libc::c_int as libc::c_float
            - r_newrefdef.fov_x / 2 as libc::c_int as libc::c_float),
    );
    RotatePointAroundVector(
        (frustum[1 as libc::c_int as usize].normal).as_mut_ptr(),
        vup.as_mut_ptr() as *const vec_t,
        vpn.as_mut_ptr() as *const vec_t,
        90 as libc::c_int as libc::c_float
            - r_newrefdef.fov_x / 2 as libc::c_int as libc::c_float,
    );
    RotatePointAroundVector(
        (frustum[2 as libc::c_int as usize].normal).as_mut_ptr(),
        vright.as_mut_ptr() as *const vec_t,
        vpn.as_mut_ptr() as *const vec_t,
        90 as libc::c_int as libc::c_float
            - r_newrefdef.fov_y / 2 as libc::c_int as libc::c_float,
    );
    RotatePointAroundVector(
        (frustum[3 as libc::c_int as usize].normal).as_mut_ptr(),
        vright.as_mut_ptr() as *const vec_t,
        vpn.as_mut_ptr() as *const vec_t,
        -(90 as libc::c_int as libc::c_float
            - r_newrefdef.fov_y / 2 as libc::c_int as libc::c_float),
    );
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        frustum[i as usize].type_0 = 5 as libc::c_int as byte;
        frustum[i as usize]
            .dist = r_origin[0 as libc::c_int as usize]
            * frustum[i as usize].normal[0 as libc::c_int as usize]
            + r_origin[1 as libc::c_int as usize]
                * frustum[i as usize].normal[1 as libc::c_int as usize]
            + r_origin[2 as libc::c_int as usize]
                * frustum[i as usize].normal[2 as libc::c_int as usize];
        frustum[i as usize]
            .signbits = SignbitsForPlane(&mut *frustum.as_mut_ptr().offset(i as isize))
            as byte;
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn R_SetupFrame() {
    let mut i: libc::c_int = 0;
    let mut leaf: *mut mleaf_t = 0 as *mut mleaf_t;
    r_framecount += 1;
    r_origin[0 as libc::c_int as usize] = r_newrefdef.vieworg[0 as libc::c_int as usize];
    r_origin[1 as libc::c_int as usize] = r_newrefdef.vieworg[1 as libc::c_int as usize];
    r_origin[2 as libc::c_int as usize] = r_newrefdef.vieworg[2 as libc::c_int as usize];
    AngleVectors(
        (r_newrefdef.viewangles).as_mut_ptr(),
        vpn.as_mut_ptr(),
        vright.as_mut_ptr(),
        vup.as_mut_ptr(),
    );
    if r_newrefdef.rdflags & 2 as libc::c_int == 0 {
        r_oldviewcluster = r_viewcluster;
        r_oldviewcluster2 = r_viewcluster2;
        leaf = Mod_PointInLeaf(r_origin.as_mut_ptr(), r_worldmodel);
        r_viewcluster2 = (*leaf).cluster;
        r_viewcluster = r_viewcluster2;
        if (*leaf).contents == 0 {
            let mut temp: vec3_t = [0.; 3];
            temp[0 as libc::c_int as usize] = r_origin[0 as libc::c_int as usize];
            temp[1 as libc::c_int as usize] = r_origin[1 as libc::c_int as usize];
            temp[2 as libc::c_int as usize] = r_origin[2 as libc::c_int as usize];
            temp[2 as libc::c_int as usize] -= 16 as libc::c_int as libc::c_float;
            leaf = Mod_PointInLeaf(temp.as_mut_ptr(), r_worldmodel);
            if (*leaf).contents & 1 as libc::c_int == 0
                && (*leaf).cluster != r_viewcluster2
            {
                r_viewcluster2 = (*leaf).cluster;
            }
        } else {
            let mut temp_0: vec3_t = [0.; 3];
            temp_0[0 as libc::c_int as usize] = r_origin[0 as libc::c_int as usize];
            temp_0[1 as libc::c_int as usize] = r_origin[1 as libc::c_int as usize];
            temp_0[2 as libc::c_int as usize] = r_origin[2 as libc::c_int as usize];
            temp_0[2 as libc::c_int as usize] += 16 as libc::c_int as libc::c_float;
            leaf = Mod_PointInLeaf(temp_0.as_mut_ptr(), r_worldmodel);
            if (*leaf).contents & 1 as libc::c_int == 0
                && (*leaf).cluster != r_viewcluster2
            {
                r_viewcluster2 = (*leaf).cluster;
            }
        }
    }
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        v_blend[i as usize] = r_newrefdef.blend[i as usize];
        i += 1;
    }
    c_brush_polys = 0 as libc::c_int;
    c_alias_polys = 0 as libc::c_int;
    if r_newrefdef.rdflags & 2 as libc::c_int != 0 {
        qglClearColor
            .expect(
                "non-null function pointer",
            )(
            0.3f64 as libc::c_int,
            0.3f64 as libc::c_int,
            0.3f64 as libc::c_int,
            1 as libc::c_int,
        );
        qglScissor
            .expect(
                "non-null function pointer",
            )(
            r_newrefdef.x,
            (vid.height)
                .wrapping_sub(r_newrefdef.height as libc::c_uint)
                .wrapping_sub(r_newrefdef.y as libc::c_uint) as libc::c_int,
            r_newrefdef.width,
            r_newrefdef.height,
        );
        qglClearColor
            .expect(
                "non-null function pointer",
            )(
            1 as libc::c_int,
            0 as libc::c_int,
            0.5f64 as libc::c_int,
            0.5f64 as libc::c_int,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn MYgluPerspective(
    mut fovy: libc::c_int,
    mut aspect: libc::c_int,
    mut zNear: libc::c_int,
    mut zFar: libc::c_int,
) {}
#[no_mangle]
pub unsafe extern "C" fn R_SetupGL() {
    let mut screenaspect: libc::c_float = 0.;
    let mut x: libc::c_int = 0;
    let mut x2: libc::c_int = 0;
    let mut y2: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut w: libc::c_int = 0;
    let mut h: libc::c_int = 0;
    x = floor(
        (r_newrefdef.x as libc::c_uint).wrapping_mul(vid.width).wrapping_div(vid.width)
            as libc::c_double,
    ) as libc::c_int;
    x2 = ceil(
        ((r_newrefdef.x + r_newrefdef.width) as libc::c_uint)
            .wrapping_mul(vid.width)
            .wrapping_div(vid.width) as libc::c_double,
    ) as libc::c_int;
    y = floor(
        (vid.height)
            .wrapping_sub(
                (r_newrefdef.y as libc::c_uint)
                    .wrapping_mul(vid.height)
                    .wrapping_div(vid.height),
            ) as libc::c_double,
    ) as libc::c_int;
    y2 = ceil(
        (vid.height)
            .wrapping_sub(
                ((r_newrefdef.y + r_newrefdef.height) as libc::c_uint)
                    .wrapping_mul(vid.height)
                    .wrapping_div(vid.height),
            ) as libc::c_double,
    ) as libc::c_int;
    w = x2 - x;
    h = y - y2;
    qglViewport.expect("non-null function pointer")(x, y2, w, h);
    screenaspect = r_newrefdef.width as libc::c_float
        / r_newrefdef.height as libc::c_float;
    qglLoadIdentity.expect("non-null function pointer")();
    qglLoadIdentity.expect("non-null function pointer")();
    qglRotatef
        .expect(
            "non-null function pointer",
        )(-(90 as libc::c_int), 1 as libc::c_int, 0 as libc::c_int, 0 as libc::c_int);
    qglRotatef
        .expect(
            "non-null function pointer",
        )(90 as libc::c_int, 0 as libc::c_int, 0 as libc::c_int, 1 as libc::c_int);
    qglRotatef
        .expect(
            "non-null function pointer",
        )(
        -r_newrefdef.viewangles[2 as libc::c_int as usize] as libc::c_int,
        1 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
    );
    qglRotatef
        .expect(
            "non-null function pointer",
        )(
        -r_newrefdef.viewangles[0 as libc::c_int as usize] as libc::c_int,
        0 as libc::c_int,
        1 as libc::c_int,
        0 as libc::c_int,
    );
    qglRotatef
        .expect(
            "non-null function pointer",
        )(
        -r_newrefdef.viewangles[1 as libc::c_int as usize] as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        1 as libc::c_int,
    );
    qglTranslatef
        .expect(
            "non-null function pointer",
        )(
        -r_newrefdef.vieworg[0 as libc::c_int as usize] as libc::c_int,
        -r_newrefdef.vieworg[1 as libc::c_int as usize] as libc::c_int,
        -r_newrefdef.vieworg[2 as libc::c_int as usize] as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn R_Clear() {
    if (*gl_ztrick).value != 0. {
        static mut trickframe: libc::c_int = 0;
        trickframe += 1;
        if trickframe & 1 as libc::c_int != 0 {
            gldepthmin = 0 as libc::c_int as libc::c_float;
            gldepthmax = 0.49999f64 as libc::c_float;
        } else {
            gldepthmin = 1 as libc::c_int as libc::c_float;
            gldepthmax = 0.5f64 as libc::c_float;
        }
    } else {
        gldepthmin = 0 as libc::c_int as libc::c_float;
        gldepthmax = 1 as libc::c_int as libc::c_float;
    }
    qglDepthRange
        .expect(
            "non-null function pointer",
        )(gldepthmin as libc::c_int, gldepthmax as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn R_Flash() {
    R_PolyBlend();
}
#[no_mangle]
pub unsafe extern "C" fn R_RenderView(mut fd: *mut refdef_t) {
    if (*r_norefresh).value != 0. {
        return;
    }
    r_newrefdef = *fd;
    if r_worldmodel.is_null() && r_newrefdef.rdflags & 2 as libc::c_int == 0 {
        (ri.Sys_Error)
            .expect(
                "non-null function pointer",
            )(
            1 as libc::c_int,
            b"R_RenderView: NULL worldmodel\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    if (*r_speeds).value != 0. {
        c_brush_polys = 0 as libc::c_int;
        c_alias_polys = 0 as libc::c_int;
    }
    R_PushDlights();
    if (*gl_finish).value != 0. {
        qglFinish.expect("non-null function pointer")();
    }
    R_SetupFrame();
    R_SetFrustum();
    R_SetupGL();
    R_MarkLeaves();
    R_DrawWorld();
    R_DrawEntitiesOnList();
    R_RenderDlights();
    R_DrawParticles();
    R_DrawAlphaSurfaces();
    R_Flash();
    if (*r_speeds).value != 0. {
        (ri.Con_Printf)
            .expect(
                "non-null function pointer",
            )(
            0 as libc::c_int,
            b"%4i wpoly %4i epoly %i tex %i lmaps\n\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            c_brush_polys,
            c_alias_polys,
            c_visible_textures,
            c_visible_lightmaps,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn R_SetGL2D() {
    qglViewport
        .expect(
            "non-null function pointer",
        )(
        0 as libc::c_int,
        0 as libc::c_int,
        vid.width as libc::c_int,
        vid.height as libc::c_int,
    );
    qglLoadIdentity.expect("non-null function pointer")();
    qglOrtho
        .expect(
            "non-null function pointer",
        )(
        0 as libc::c_int,
        vid.width as libc::c_int,
        vid.height as libc::c_int,
        0 as libc::c_int,
        -(99999 as libc::c_int),
        99999 as libc::c_int,
    );
    qglLoadIdentity.expect("non-null function pointer")();
    qglColor4f
        .expect(
            "non-null function pointer",
        )(1 as libc::c_int, 1 as libc::c_int, 1 as libc::c_int, 1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn R_SetLightLevel() {
    let mut shadelight: vec3_t = [0.; 3];
    if r_newrefdef.rdflags & 2 as libc::c_int != 0 {
        return;
    }
    R_LightPoint((r_newrefdef.vieworg).as_mut_ptr(), shadelight.as_mut_ptr());
    if shadelight[0 as libc::c_int as usize] > shadelight[1 as libc::c_int as usize] {
        if shadelight[0 as libc::c_int as usize] > shadelight[2 as libc::c_int as usize]
        {
            (*r_lightlevel)
                .value = 150 as libc::c_int as libc::c_float
                * shadelight[0 as libc::c_int as usize];
        } else {
            (*r_lightlevel)
                .value = 150 as libc::c_int as libc::c_float
                * shadelight[2 as libc::c_int as usize];
        }
    } else if shadelight[1 as libc::c_int as usize]
        > shadelight[2 as libc::c_int as usize]
    {
        (*r_lightlevel)
            .value = 150 as libc::c_int as libc::c_float
            * shadelight[1 as libc::c_int as usize];
    } else {
        (*r_lightlevel)
            .value = 150 as libc::c_int as libc::c_float
            * shadelight[2 as libc::c_int as usize];
    };
}
#[no_mangle]
pub unsafe extern "C" fn R_RenderFrame(mut fd: *mut refdef_t) {
    R_RenderView(fd);
    R_SetLightLevel();
    R_SetGL2D();
}
#[no_mangle]
pub unsafe extern "C" fn R_Register() {
    r_lefthand = (ri.Cvar_Get)
        .expect(
            "non-null function pointer",
        )(
        b"hand\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        2 as libc::c_int | 1 as libc::c_int,
    );
    r_norefresh = (ri.Cvar_Get)
        .expect(
            "non-null function pointer",
        )(
        b"r_norefresh\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
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
    r_novis = (ri.Cvar_Get)
        .expect(
            "non-null function pointer",
        )(
        b"r_novis\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    r_nocull = (ri.Cvar_Get)
        .expect(
            "non-null function pointer",
        )(
        b"r_nocull\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
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
    r_speeds = (ri.Cvar_Get)
        .expect(
            "non-null function pointer",
        )(
        b"r_speeds\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
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
    gl_nosubimage = (ri.Cvar_Get)
        .expect(
            "non-null function pointer",
        )(
        b"gl_nosubimage\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    gl_allow_software = (ri.Cvar_Get)
        .expect(
            "non-null function pointer",
        )(
        b"gl_allow_software\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    gl_particle_min_size = (ri.Cvar_Get)
        .expect(
            "non-null function pointer",
        )(
        b"gl_particle_min_size\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        b"2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        1 as libc::c_int,
    );
    gl_particle_max_size = (ri.Cvar_Get)
        .expect(
            "non-null function pointer",
        )(
        b"gl_particle_max_size\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        b"40\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        1 as libc::c_int,
    );
    gl_particle_size = (ri.Cvar_Get)
        .expect(
            "non-null function pointer",
        )(
        b"gl_particle_size\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"40\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        1 as libc::c_int,
    );
    gl_particle_att_a = (ri.Cvar_Get)
        .expect(
            "non-null function pointer",
        )(
        b"gl_particle_att_a\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"0.01\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        1 as libc::c_int,
    );
    gl_particle_att_b = (ri.Cvar_Get)
        .expect(
            "non-null function pointer",
        )(
        b"gl_particle_att_b\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"0.0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        1 as libc::c_int,
    );
    gl_particle_att_c = (ri.Cvar_Get)
        .expect(
            "non-null function pointer",
        )(
        b"gl_particle_att_c\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"0.01\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        1 as libc::c_int,
    );
    gl_modulate = (ri.Cvar_Get)
        .expect(
            "non-null function pointer",
        )(
        b"gl_modulate\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        1 as libc::c_int,
    );
    gl_log = (ri.Cvar_Get)
        .expect(
            "non-null function pointer",
        )(
        b"gl_log\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    gl_bitdepth = (ri.Cvar_Get)
        .expect(
            "non-null function pointer",
        )(
        b"gl_bitdepth\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    gl_mode = (ri.Cvar_Get)
        .expect(
            "non-null function pointer",
        )(
        b"gl_mode\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"3\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        1 as libc::c_int,
    );
    gl_lightmap = (ri.Cvar_Get)
        .expect(
            "non-null function pointer",
        )(
        b"gl_lightmap\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    gl_shadows = (ri.Cvar_Get)
        .expect(
            "non-null function pointer",
        )(
        b"gl_shadows\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        1 as libc::c_int,
    );
    gl_dynamic = (ri.Cvar_Get)
        .expect(
            "non-null function pointer",
        )(
        b"gl_dynamic\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    gl_nobind = (ri.Cvar_Get)
        .expect(
            "non-null function pointer",
        )(
        b"gl_nobind\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    gl_round_down = (ri.Cvar_Get)
        .expect(
            "non-null function pointer",
        )(
        b"gl_round_down\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    gl_picmip = (ri.Cvar_Get)
        .expect(
            "non-null function pointer",
        )(
        b"gl_picmip\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    gl_skymip = (ri.Cvar_Get)
        .expect(
            "non-null function pointer",
        )(
        b"gl_skymip\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    gl_showtris = (ri.Cvar_Get)
        .expect(
            "non-null function pointer",
        )(
        b"gl_showtris\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    gl_ztrick = (ri.Cvar_Get)
        .expect(
            "non-null function pointer",
        )(
        b"gl_ztrick\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    gl_finish = (ri.Cvar_Get)
        .expect(
            "non-null function pointer",
        )(
        b"gl_finish\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        1 as libc::c_int,
    );
    gl_clear = (ri.Cvar_Get)
        .expect(
            "non-null function pointer",
        )(
        b"gl_clear\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    gl_cull = (ri.Cvar_Get)
        .expect(
            "non-null function pointer",
        )(
        b"gl_cull\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    gl_polyblend = (ri.Cvar_Get)
        .expect(
            "non-null function pointer",
        )(
        b"gl_polyblend\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    gl_flashblend = (ri.Cvar_Get)
        .expect(
            "non-null function pointer",
        )(
        b"gl_flashblend\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    gl_playermip = (ri.Cvar_Get)
        .expect(
            "non-null function pointer",
        )(
        b"gl_playermip\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    gl_monolightmap = (ri.Cvar_Get)
        .expect(
            "non-null function pointer",
        )(
        b"gl_monolightmap\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    gl_driver = (ri.Cvar_Get)
        .expect(
            "non-null function pointer",
        )(
        b"gl_driver\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"opengl32\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        1 as libc::c_int,
    );
    gl_texturemode = (ri.Cvar_Get)
        .expect(
            "non-null function pointer",
        )(
        b"gl_texturemode\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"GL_LINEAR_MIPMAP_NEAREST\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        1 as libc::c_int,
    );
    gl_texturealphamode = (ri.Cvar_Get)
        .expect(
            "non-null function pointer",
        )(
        b"gl_texturealphamode\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        b"default\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        1 as libc::c_int,
    );
    gl_texturesolidmode = (ri.Cvar_Get)
        .expect(
            "non-null function pointer",
        )(
        b"gl_texturesolidmode\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        b"default\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        1 as libc::c_int,
    );
    gl_lockpvs = (ri.Cvar_Get)
        .expect(
            "non-null function pointer",
        )(
        b"gl_lockpvs\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    gl_vertex_arrays = (ri.Cvar_Get)
        .expect(
            "non-null function pointer",
        )(
        b"gl_vertex_arrays\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        1 as libc::c_int,
    );
    gl_ext_swapinterval = (ri.Cvar_Get)
        .expect(
            "non-null function pointer",
        )(
        b"gl_ext_swapinterval\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        b"1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        1 as libc::c_int,
    );
    gl_ext_palettedtexture = (ri.Cvar_Get)
        .expect(
            "non-null function pointer",
        )(
        b"gl_ext_palettedtexture\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        b"1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        1 as libc::c_int,
    );
    gl_ext_multitexture = (ri.Cvar_Get)
        .expect(
            "non-null function pointer",
        )(
        b"gl_ext_multitexture\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        b"1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        1 as libc::c_int,
    );
    gl_ext_pointparameters = (ri.Cvar_Get)
        .expect(
            "non-null function pointer",
        )(
        b"gl_ext_pointparameters\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        b"1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        1 as libc::c_int,
    );
    gl_ext_compiled_vertex_array = (ri.Cvar_Get)
        .expect(
            "non-null function pointer",
        )(
        b"gl_ext_compiled_vertex_array\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        b"1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        1 as libc::c_int,
    );
    gl_drawbuffer = (ri.Cvar_Get)
        .expect(
            "non-null function pointer",
        )(
        b"gl_drawbuffer\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"GL_BACK\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    gl_swapinterval = (ri.Cvar_Get)
        .expect(
            "non-null function pointer",
        )(
        b"gl_swapinterval\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        1 as libc::c_int,
    );
    gl_saturatelighting = (ri.Cvar_Get)
        .expect(
            "non-null function pointer",
        )(
        b"gl_saturatelighting\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    gl_3dlabs_broken = (ri.Cvar_Get)
        .expect(
            "non-null function pointer",
        )(
        b"gl_3dlabs_broken\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        1 as libc::c_int,
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
    vid_ref = (ri.Cvar_Get)
        .expect(
            "non-null function pointer",
        )(
        b"vid_ref\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"soft\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        1 as libc::c_int,
    );
    (ri.Cmd_AddCommand)
        .expect(
            "non-null function pointer",
        )(
        b"imagelist\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        Some(GL_ImageList_f as unsafe extern "C" fn() -> ()),
    );
    (ri.Cmd_AddCommand)
        .expect(
            "non-null function pointer",
        )(
        b"screenshot\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        Some(GL_ScreenShot_f as unsafe extern "C" fn() -> ()),
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
        b"gl_strings\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        Some(GL_Strings_f as unsafe extern "C" fn() -> ()),
    );
}
#[no_mangle]
pub unsafe extern "C" fn R_SetMode() -> qboolean {
    let mut err: rserr_t = rserr_ok;
    let mut fullscreen: qboolean = false_0;
    if (*vid_fullscreen).modified as libc::c_uint != 0 && gl_config.allow_cds as u64 == 0
    {
        (ri.Con_Printf)
            .expect(
                "non-null function pointer",
            )(
            0 as libc::c_int,
            b"R_SetMode() - CDS not allowed with this driver\n\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        );
        (ri.Cvar_SetValue)
            .expect(
                "non-null function pointer",
            )(
            b"vid_fullscreen\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ((*vid_fullscreen).value == 0.) as libc::c_int as libc::c_float,
        );
        (*vid_fullscreen).modified = false_0;
    }
    fullscreen = (*vid_fullscreen).value as qboolean;
    (*vid_fullscreen).modified = false_0;
    (*gl_mode).modified = false_0;
    err = GLimp_SetMode(
        &mut vid.width as *mut libc::c_uint as *mut libc::c_int,
        &mut vid.height as *mut libc::c_uint as *mut libc::c_int,
        (*gl_mode).value as libc::c_int,
        fullscreen,
    ) as rserr_t;
    if err as libc::c_uint == rserr_ok as libc::c_int as libc::c_uint {
        gl_state.prev_mode = (*gl_mode).value as libc::c_int;
    } else {
        if err as libc::c_uint == rserr_invalid_fullscreen as libc::c_int as libc::c_uint
        {
            (ri.Cvar_SetValue)
                .expect(
                    "non-null function pointer",
                )(
                b"vid_fullscreen\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                0 as libc::c_int as libc::c_float,
            );
            (*vid_fullscreen).modified = false_0;
            (ri.Con_Printf)
                .expect(
                    "non-null function pointer",
                )(
                0 as libc::c_int,
                b"ref_gl::R_SetMode() - fullscreen unavailable in this mode\n\0"
                    as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            err = GLimp_SetMode(
                &mut vid.width as *mut libc::c_uint as *mut libc::c_int,
                &mut vid.height as *mut libc::c_uint as *mut libc::c_int,
                (*gl_mode).value as libc::c_int,
                false_0,
            ) as rserr_t;
            if err as libc::c_uint == rserr_ok as libc::c_int as libc::c_uint {
                return true_0;
            }
        } else if err as libc::c_uint
            == rserr_invalid_mode as libc::c_int as libc::c_uint
        {
            (ri.Cvar_SetValue)
                .expect(
                    "non-null function pointer",
                )(
                b"gl_mode\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                gl_state.prev_mode as libc::c_float,
            );
            (*gl_mode).modified = false_0;
            (ri.Con_Printf)
                .expect(
                    "non-null function pointer",
                )(
                0 as libc::c_int,
                b"ref_gl::R_SetMode() - invalid mode\n\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
            );
        }
        err = GLimp_SetMode(
            &mut vid.width as *mut libc::c_uint as *mut libc::c_int,
            &mut vid.height as *mut libc::c_uint as *mut libc::c_int,
            gl_state.prev_mode,
            false_0,
        ) as rserr_t;
        if err as libc::c_uint != rserr_ok as libc::c_int as libc::c_uint {
            (ri.Con_Printf)
                .expect(
                    "non-null function pointer",
                )(
                0 as libc::c_int,
                b"ref_gl::R_SetMode() - could not revert to safe mode\n\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
            );
            return false_0;
        }
    }
    return true_0;
}
#[no_mangle]
pub unsafe extern "C" fn R_Shutdown() {
    (ri.Cmd_RemoveCommand)
        .expect(
            "non-null function pointer",
        )(b"modellist\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    (ri.Cmd_RemoveCommand)
        .expect(
            "non-null function pointer",
        )(b"screenshot\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    (ri.Cmd_RemoveCommand)
        .expect(
            "non-null function pointer",
        )(b"imagelist\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    (ri.Cmd_RemoveCommand)
        .expect(
            "non-null function pointer",
        )(b"gl_strings\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    Mod_FreeAll();
    GL_ShutdownImages();
    GLimp_Shutdown();
    QGL_Shutdown();
}
#[no_mangle]
pub unsafe extern "C" fn R_BeginFrame(mut camera_separation: libc::c_float) {
    gl_state.camera_separation = camera_separation;
    if (*gl_mode).modified as libc::c_uint != 0
        || (*vid_fullscreen).modified as libc::c_uint != 0
    {
        let mut ref_0: *mut cvar_t = 0 as *mut cvar_t;
        ref_0 = (ri.Cvar_Get)
            .expect(
                "non-null function pointer",
            )(
            b"vid_ref\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"gl\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            0 as libc::c_int,
        );
        (*ref_0).modified = true_0;
    }
    if (*gl_log).modified as u64 != 0 {
        GLimp_EnableLogging((*gl_log).value as qboolean);
        (*gl_log).modified = false_0;
    }
    if (*gl_log).value != 0. {
        GLimp_LogNewFrame();
    }
    if (*vid_gamma).modified as u64 != 0 {
        (*vid_gamma).modified = false_0;
        if gl_config.renderer & 0x1 as libc::c_int != 0 {
            let mut envbuffer: [libc::c_char; 1024] = [0; 1024];
            let mut g: libc::c_float = 0.;
            g = (2.00f64 * (0.8f64 - ((*vid_gamma).value as libc::c_double - 0.5f64))
                + 1.0f32 as libc::c_double) as libc::c_float;
            Com_sprintf(
                envbuffer.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong
                    as libc::c_int,
                b"SSTV2_GAMMA=%f\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                g as libc::c_double,
            );
            putenv(envbuffer.as_mut_ptr());
            Com_sprintf(
                envbuffer.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong
                    as libc::c_int,
                b"SST_GAMMA=%f\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                g as libc::c_double,
            );
            putenv(envbuffer.as_mut_ptr());
        }
    }
    GLimp_BeginFrame(camera_separation);
    qglViewport
        .expect(
            "non-null function pointer",
        )(
        0 as libc::c_int,
        0 as libc::c_int,
        vid.width as libc::c_int,
        vid.height as libc::c_int,
    );
    qglLoadIdentity.expect("non-null function pointer")();
    qglOrtho
        .expect(
            "non-null function pointer",
        )(
        0 as libc::c_int,
        vid.width as libc::c_int,
        vid.height as libc::c_int,
        0 as libc::c_int,
        -(99999 as libc::c_int),
        99999 as libc::c_int,
    );
    qglLoadIdentity.expect("non-null function pointer")();
    qglColor4f
        .expect(
            "non-null function pointer",
        )(1 as libc::c_int, 1 as libc::c_int, 1 as libc::c_int, 1 as libc::c_int);
    if (*gl_drawbuffer).modified as u64 != 0 {
        (*gl_drawbuffer).modified = false_0;
        gl_state.camera_separation == 0 as libc::c_int as libc::c_float
            || gl_state.stereo_enabled as u64 == 0;
    }
    if (*gl_texturemode).modified as u64 != 0 {
        GL_TextureMode((*gl_texturemode).string);
        (*gl_texturemode).modified = false_0;
    }
    if (*gl_texturealphamode).modified as u64 != 0 {
        GL_TextureAlphaMode((*gl_texturealphamode).string);
        (*gl_texturealphamode).modified = false_0;
    }
    if (*gl_texturesolidmode).modified as u64 != 0 {
        GL_TextureSolidMode((*gl_texturesolidmode).string);
        (*gl_texturesolidmode).modified = false_0;
    }
    GL_UpdateSwapInterval();
    R_Clear();
}
#[no_mangle]
pub static mut r_rawpalette: [libc::c_uint; 256] = [0; 256];
#[no_mangle]
pub unsafe extern "C" fn R_SetPalette(mut palette: *const libc::c_uchar) {
    let mut i: libc::c_int = 0;
    let mut rp: *mut byte = r_rawpalette.as_mut_ptr() as *mut byte;
    if !palette.is_null() {
        i = 0 as libc::c_int;
        while i < 256 as libc::c_int {
            *rp
                .offset(
                    (i * 4 as libc::c_int + 0 as libc::c_int) as isize,
                ) = *palette.offset((i * 3 as libc::c_int + 0 as libc::c_int) as isize);
            *rp
                .offset(
                    (i * 4 as libc::c_int + 1 as libc::c_int) as isize,
                ) = *palette.offset((i * 3 as libc::c_int + 1 as libc::c_int) as isize);
            *rp
                .offset(
                    (i * 4 as libc::c_int + 2 as libc::c_int) as isize,
                ) = *palette.offset((i * 3 as libc::c_int + 2 as libc::c_int) as isize);
            *rp
                .offset(
                    (i * 4 as libc::c_int + 3 as libc::c_int) as isize,
                ) = 0xff as libc::c_int as byte;
            i += 1;
        }
    } else {
        i = 0 as libc::c_int;
        while i < 256 as libc::c_int {
            *rp
                .offset(
                    (i * 4 as libc::c_int + 0 as libc::c_int) as isize,
                ) = (d_8to24table[i as usize] & 0xff as libc::c_int as libc::c_uint)
                as byte;
            *rp
                .offset(
                    (i * 4 as libc::c_int + 1 as libc::c_int) as isize,
                ) = (d_8to24table[i as usize] >> 8 as libc::c_int
                & 0xff as libc::c_int as libc::c_uint) as byte;
            *rp
                .offset(
                    (i * 4 as libc::c_int + 2 as libc::c_int) as isize,
                ) = (d_8to24table[i as usize] >> 16 as libc::c_int
                & 0xff as libc::c_int as libc::c_uint) as byte;
            *rp
                .offset(
                    (i * 4 as libc::c_int + 3 as libc::c_int) as isize,
                ) = 0xff as libc::c_int as byte;
            i += 1;
        }
    }
    GL_SetTexturePalette(r_rawpalette.as_mut_ptr());
    qglClearColor
        .expect(
            "non-null function pointer",
        )(0 as libc::c_int, 0 as libc::c_int, 0 as libc::c_int, 0 as libc::c_int);
    qglClearColor
        .expect(
            "non-null function pointer",
        )(
        1 as libc::c_int,
        0 as libc::c_int,
        0.5f64 as libc::c_int,
        0.5f64 as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn R_DrawBeam(mut e: *mut entity_t) {
    let mut i: libc::c_int = 0;
    let mut r: libc::c_float = 0.;
    let mut g: libc::c_float = 0.;
    let mut b: libc::c_float = 0.;
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
    r = (d_8to24table[((*e).skinnum & 0xff as libc::c_int) as usize]
        & 0xff as libc::c_int as libc::c_uint) as libc::c_float;
    g = (d_8to24table[((*e).skinnum & 0xff as libc::c_int) as usize] >> 8 as libc::c_int
        & 0xff as libc::c_int as libc::c_uint) as libc::c_float;
    b = (d_8to24table[((*e).skinnum & 0xff as libc::c_int) as usize] >> 16 as libc::c_int
        & 0xff as libc::c_int as libc::c_uint) as libc::c_float;
    r *= 1 as libc::c_int as libc::c_float / 255.0f32;
    g *= 1 as libc::c_int as libc::c_float / 255.0f32;
    b *= 1 as libc::c_int as libc::c_float / 255.0f32;
    qglColor4f
        .expect(
            "non-null function pointer",
        )(
        r as libc::c_int,
        g as libc::c_int,
        b as libc::c_int,
        (*e).alpha as libc::c_int,
    );
    i = 0 as libc::c_int;
    while i < 6 as libc::c_int {
        qglVertex3fv
            .expect(
                "non-null function pointer",
            )((start_points[i as usize]).as_mut_ptr() as *const libc::c_int);
        qglVertex3fv
            .expect(
                "non-null function pointer",
            )((end_points[i as usize]).as_mut_ptr() as *const libc::c_int);
        qglVertex3fv
            .expect(
                "non-null function pointer",
            )(
            (start_points[((i + 1 as libc::c_int) % 6 as libc::c_int) as usize])
                .as_mut_ptr() as *const libc::c_int,
        );
        qglVertex3fv
            .expect(
                "non-null function pointer",
            )(
            (end_points[((i + 1 as libc::c_int) % 6 as libc::c_int) as usize])
                .as_mut_ptr() as *const libc::c_int,
        );
        i += 1;
    }
    qglEnd.expect("non-null function pointer")();
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
        .Init = ::std::mem::transmute::<
        Option::<
            unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> libc::c_int,
        >,
        Option::<unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> qboolean>,
    >(
        Some(
            R_Init
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut libc::c_void,
                ) -> libc::c_int,
        ),
    );
    re.Shutdown = Some(R_Shutdown as unsafe extern "C" fn() -> ());
    re
        .CinematicSetPalette = Some(
        R_SetPalette as unsafe extern "C" fn(*const libc::c_uchar) -> (),
    );
    re.BeginFrame = Some(R_BeginFrame as unsafe extern "C" fn(libc::c_float) -> ());
    re.EndFrame = Some(GLimp_EndFrame as unsafe extern "C" fn() -> ());
    re.AppActivate = Some(GLimp_AppActivate as unsafe extern "C" fn(qboolean) -> ());
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
