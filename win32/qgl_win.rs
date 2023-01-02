#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(extern_types, register_tool)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn putenv(__string: *mut libc::c_char) -> libc::c_int;
    fn time(__timer: *mut time_t) -> time_t;
    fn localtime(__timer: *const time_t) -> *mut tm;
    fn asctime(__tp: *const tm) -> *mut libc::c_char;
    fn Com_sprintf(
        dest: *mut libc::c_char,
        size: libc::c_int,
        fmt: *mut libc::c_char,
        _: ...
    );
    static mut gl_config: glconfig_t;
    static mut ri: refimport_t;
    static mut vid_gamma: *mut cvar_t;
    static mut glw_state: glwstate_t;
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type time_t = __time_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tm {
    pub tm_sec: libc::c_int,
    pub tm_min: libc::c_int,
    pub tm_hour: libc::c_int,
    pub tm_mday: libc::c_int,
    pub tm_mon: libc::c_int,
    pub tm_year: libc::c_int,
    pub tm_wday: libc::c_int,
    pub tm_yday: libc::c_int,
    pub tm_isdst: libc::c_int,
    pub tm_gmtoff: libc::c_long,
    pub tm_zone: *const libc::c_char,
}
pub type qboolean = libc::c_uint;
pub const true_0: qboolean = 1;
pub const false_0: qboolean = 0;
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
pub struct glwstate_t {
    pub hInstance: libc::c_int,
    pub wndproc: *mut libc::c_void,
    pub hDC: libc::c_int,
    pub hWnd: libc::c_int,
    pub hGLRC: libc::c_int,
    pub hinstOpenGL: libc::c_int,
    pub minidriver: qboolean,
    pub allowdisplaydepthchange: qboolean,
    pub mcd_accelerated: qboolean,
    pub log_fp: *mut FILE,
}
#[no_mangle]
pub static mut qglAccum: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglAlphaFunc: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglArrayElement: Option::<unsafe extern "C" fn(libc::c_int) -> ()> = None;
#[no_mangle]
pub static mut qglBegin: Option::<unsafe extern "C" fn(libc::c_int) -> ()> = None;
#[no_mangle]
pub static mut qglBindTexture: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglBitmap: Option::<
    unsafe extern "C" fn(
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        *const libc::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut qglBlendFunc: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglCallList: Option::<unsafe extern "C" fn(libc::c_int) -> ()> = None;
#[no_mangle]
pub static mut qglCallLists: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, *const libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglClear: Option::<unsafe extern "C" fn(libc::c_int) -> ()> = None;
#[no_mangle]
pub static mut qglClearAccum: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int, libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglClearColor: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int, libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglClearDepth: Option::<unsafe extern "C" fn(libc::c_int) -> ()> = None;
#[no_mangle]
pub static mut qglClearIndex: Option::<unsafe extern "C" fn(libc::c_int) -> ()> = None;
#[no_mangle]
pub static mut qglClearStencil: Option::<unsafe extern "C" fn(libc::c_int) -> ()> = None;
#[no_mangle]
pub static mut qglClipPlane: Option::<
    unsafe extern "C" fn(libc::c_int, *const libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglColor3b: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglColor3bv: Option::<unsafe extern "C" fn(*const libc::c_int) -> ()> = None;
#[no_mangle]
pub static mut qglColor3d: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglColor3dv: Option::<unsafe extern "C" fn(*const libc::c_int) -> ()> = None;
#[no_mangle]
pub static mut qglColor3f: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglColor3fv: Option::<unsafe extern "C" fn(*const libc::c_int) -> ()> = None;
#[no_mangle]
pub static mut qglColor3i: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglColor3iv: Option::<unsafe extern "C" fn(*const libc::c_int) -> ()> = None;
#[no_mangle]
pub static mut qglColor3s: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglColor3sv: Option::<unsafe extern "C" fn(*const libc::c_int) -> ()> = None;
#[no_mangle]
pub static mut qglColor3ub: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglColor3ubv: Option::<unsafe extern "C" fn(*const libc::c_int) -> ()> = None;
#[no_mangle]
pub static mut qglColor3ui: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglColor3uiv: Option::<unsafe extern "C" fn(*const libc::c_int) -> ()> = None;
#[no_mangle]
pub static mut qglColor3us: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglColor3usv: Option::<unsafe extern "C" fn(*const libc::c_int) -> ()> = None;
#[no_mangle]
pub static mut qglColor4b: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int, libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglColor4bv: Option::<unsafe extern "C" fn(*const libc::c_int) -> ()> = None;
#[no_mangle]
pub static mut qglColor4d: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int, libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglColor4dv: Option::<unsafe extern "C" fn(*const libc::c_int) -> ()> = None;
#[no_mangle]
pub static mut qglColor4f: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int, libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglColor4fv: Option::<unsafe extern "C" fn(*const libc::c_int) -> ()> = None;
#[no_mangle]
pub static mut qglColor4i: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int, libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglColor4iv: Option::<unsafe extern "C" fn(*const libc::c_int) -> ()> = None;
#[no_mangle]
pub static mut qglColor4s: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int, libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglColor4sv: Option::<unsafe extern "C" fn(*const libc::c_int) -> ()> = None;
#[no_mangle]
pub static mut qglColor4ub: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int, libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglColor4ubv: Option::<unsafe extern "C" fn(*const libc::c_int) -> ()> = None;
#[no_mangle]
pub static mut qglColor4ui: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int, libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglColor4uiv: Option::<unsafe extern "C" fn(*const libc::c_int) -> ()> = None;
#[no_mangle]
pub static mut qglColor4us: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int, libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglColor4usv: Option::<unsafe extern "C" fn(*const libc::c_int) -> ()> = None;
#[no_mangle]
pub static mut qglColorMask: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int, libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglColorMaterial: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglColorPointer: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int, *const libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglCopyPixels: Option::<
    unsafe extern "C" fn(
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut qglCopyTexImage1D: Option::<
    unsafe extern "C" fn(
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut qglCopyTexImage2D: Option::<
    unsafe extern "C" fn(
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut qglCopyTexSubImage1D: Option::<
    unsafe extern "C" fn(
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut qglCopyTexSubImage2D: Option::<
    unsafe extern "C" fn(
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut qglCullFace: Option::<unsafe extern "C" fn(libc::c_int) -> ()> = None;
#[no_mangle]
pub static mut qglDeleteLists: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglDeleteTextures: Option::<
    unsafe extern "C" fn(libc::c_int, *const libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglDepthFunc: Option::<unsafe extern "C" fn(libc::c_int) -> ()> = None;
#[no_mangle]
pub static mut qglDepthMask: Option::<unsafe extern "C" fn(libc::c_int) -> ()> = None;
#[no_mangle]
pub static mut qglDepthRange: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglDisable: Option::<unsafe extern "C" fn(libc::c_int) -> ()> = None;
#[no_mangle]
pub static mut qglDisableClientState: Option::<
    unsafe extern "C" fn(libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglDrawArrays: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglDrawBuffer: Option::<unsafe extern "C" fn(libc::c_int) -> ()> = None;
#[no_mangle]
pub static mut qglDrawElements: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int, *const libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglDrawPixels: Option::<
    unsafe extern "C" fn(
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        *const libc::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut qglEdgeFlag: Option::<unsafe extern "C" fn(libc::c_int) -> ()> = None;
#[no_mangle]
pub static mut qglEdgeFlagPointer: Option::<
    unsafe extern "C" fn(libc::c_int, *const libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglEdgeFlagv: Option::<unsafe extern "C" fn(*const libc::c_int) -> ()> = None;
#[no_mangle]
pub static mut qglEnable: Option::<unsafe extern "C" fn(libc::c_int) -> ()> = None;
#[no_mangle]
pub static mut qglEnableClientState: Option::<unsafe extern "C" fn(libc::c_int) -> ()> = None;
#[no_mangle]
pub static mut qglEnd: Option::<unsafe extern "C" fn() -> ()> = None;
#[no_mangle]
pub static mut qglEndList: Option::<unsafe extern "C" fn() -> ()> = None;
#[no_mangle]
pub static mut qglEvalCoord1d: Option::<unsafe extern "C" fn(libc::c_int) -> ()> = None;
#[no_mangle]
pub static mut qglEvalCoord1dv: Option::<
    unsafe extern "C" fn(*const libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglEvalCoord1f: Option::<unsafe extern "C" fn(libc::c_int) -> ()> = None;
#[no_mangle]
pub static mut qglEvalCoord1fv: Option::<
    unsafe extern "C" fn(*const libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglEvalCoord2d: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglEvalCoord2dv: Option::<
    unsafe extern "C" fn(*const libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglEvalCoord2f: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglEvalCoord2fv: Option::<
    unsafe extern "C" fn(*const libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglEvalMesh1: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglEvalMesh2: Option::<
    unsafe extern "C" fn(
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut qglEvalPoint1: Option::<unsafe extern "C" fn(libc::c_int) -> ()> = None;
#[no_mangle]
pub static mut qglEvalPoint2: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglFeedbackBuffer: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, *mut libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglFinish: Option::<unsafe extern "C" fn() -> ()> = None;
#[no_mangle]
pub static mut qglFlush: Option::<unsafe extern "C" fn() -> ()> = None;
#[no_mangle]
pub static mut qglFogf: Option::<unsafe extern "C" fn(libc::c_int, libc::c_int) -> ()> = None;
#[no_mangle]
pub static mut qglFogfv: Option::<
    unsafe extern "C" fn(libc::c_int, *const libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglFogi: Option::<unsafe extern "C" fn(libc::c_int, libc::c_int) -> ()> = None;
#[no_mangle]
pub static mut qglFogiv: Option::<
    unsafe extern "C" fn(libc::c_int, *const libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglFrontFace: Option::<unsafe extern "C" fn(libc::c_int) -> ()> = None;
#[no_mangle]
pub static mut qglFrustum: Option::<
    unsafe extern "C" fn(
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut qglGenTextures: Option::<
    unsafe extern "C" fn(libc::c_int, *mut libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglGetBooleanv: Option::<
    unsafe extern "C" fn(libc::c_int, *mut libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglGetClipPlane: Option::<
    unsafe extern "C" fn(libc::c_int, *mut libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglGetDoublev: Option::<
    unsafe extern "C" fn(libc::c_int, *mut libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglGetFloatv: Option::<
    unsafe extern "C" fn(libc::c_int, *mut libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglGetIntegerv: Option::<
    unsafe extern "C" fn(libc::c_int, *mut libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglGetLightfv: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, *mut libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglGetLightiv: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, *mut libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglGetMapdv: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, *mut libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglGetMapfv: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, *mut libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglGetMapiv: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, *mut libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglGetMaterialfv: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, *mut libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglGetMaterialiv: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, *mut libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglGetPixelMapfv: Option::<
    unsafe extern "C" fn(libc::c_int, *mut libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglGetPixelMapuiv: Option::<
    unsafe extern "C" fn(libc::c_int, *mut libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglGetPixelMapusv: Option::<
    unsafe extern "C" fn(libc::c_int, *mut libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglGetPointerv: Option::<
    unsafe extern "C" fn(libc::c_int, *mut *mut libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglGetPolygonStipple: Option::<
    unsafe extern "C" fn(*mut libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglGetString: Option::<
    unsafe extern "C" fn(libc::c_int) -> *const libc::c_int,
> = None;
#[no_mangle]
pub static mut qglGetTexEnvfv: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, *mut libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglGetTexEnviv: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, *mut libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglGetTexGendv: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, *mut libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglGetTexGenfv: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, *mut libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglGetTexGeniv: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, *mut libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglGetTexImage: Option::<
    unsafe extern "C" fn(
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        *mut libc::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut qglGetTexLevelParameterfv: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int, *mut libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglGetTexLevelParameteriv: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int, *mut libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglGetTexParameterfv: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, *mut libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglGetTexParameteriv: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, *mut libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglHint: Option::<unsafe extern "C" fn(libc::c_int, libc::c_int) -> ()> = None;
#[no_mangle]
pub static mut qglIndexMask: Option::<unsafe extern "C" fn(libc::c_int) -> ()> = None;
#[no_mangle]
pub static mut qglIndexPointer: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, *const libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglIndexd: Option::<unsafe extern "C" fn(libc::c_int) -> ()> = None;
#[no_mangle]
pub static mut qglIndexdv: Option::<unsafe extern "C" fn(*const libc::c_int) -> ()> = None;
#[no_mangle]
pub static mut qglIndexf: Option::<unsafe extern "C" fn(libc::c_int) -> ()> = None;
#[no_mangle]
pub static mut qglIndexfv: Option::<unsafe extern "C" fn(*const libc::c_int) -> ()> = None;
#[no_mangle]
pub static mut qglIndexi: Option::<unsafe extern "C" fn(libc::c_int) -> ()> = None;
#[no_mangle]
pub static mut qglIndexiv: Option::<unsafe extern "C" fn(*const libc::c_int) -> ()> = None;
#[no_mangle]
pub static mut qglIndexs: Option::<unsafe extern "C" fn(libc::c_int) -> ()> = None;
#[no_mangle]
pub static mut qglIndexsv: Option::<unsafe extern "C" fn(*const libc::c_int) -> ()> = None;
#[no_mangle]
pub static mut qglIndexub: Option::<unsafe extern "C" fn(libc::c_int) -> ()> = None;
#[no_mangle]
pub static mut qglIndexubv: Option::<unsafe extern "C" fn(*const libc::c_int) -> ()> = None;
#[no_mangle]
pub static mut qglInitNames: Option::<unsafe extern "C" fn() -> ()> = None;
#[no_mangle]
pub static mut qglInterleavedArrays: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, *const libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglLightModelf: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglLightModelfv: Option::<
    unsafe extern "C" fn(libc::c_int, *const libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglLightModeli: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglLightModeliv: Option::<
    unsafe extern "C" fn(libc::c_int, *const libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglLightf: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglLightfv: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, *const libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglLighti: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglLightiv: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, *const libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglLineStipple: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglLineWidth: Option::<unsafe extern "C" fn(libc::c_int) -> ()> = None;
#[no_mangle]
pub static mut qglListBase: Option::<unsafe extern "C" fn(libc::c_int) -> ()> = None;
#[no_mangle]
pub static mut qglLoadIdentity: Option::<unsafe extern "C" fn() -> ()> = None;
#[no_mangle]
pub static mut qglLoadMatrixd: Option::<
    unsafe extern "C" fn(*const libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglLoadMatrixf: Option::<
    unsafe extern "C" fn(*const libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglLoadName: Option::<unsafe extern "C" fn(libc::c_int) -> ()> = None;
#[no_mangle]
pub static mut qglLogicOp: Option::<unsafe extern "C" fn(libc::c_int) -> ()> = None;
#[no_mangle]
pub static mut qglMap1d: Option::<
    unsafe extern "C" fn(
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        *const libc::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut qglMap1f: Option::<
    unsafe extern "C" fn(
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        *const libc::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut qglMap2d: Option::<
    unsafe extern "C" fn(
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        *const libc::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut qglMap2f: Option::<
    unsafe extern "C" fn(
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        *const libc::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut qglMapGrid1d: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglMapGrid1f: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglMapGrid2d: Option::<
    unsafe extern "C" fn(
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut qglMapGrid2f: Option::<
    unsafe extern "C" fn(
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut qglMaterialf: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglMaterialfv: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, *const libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglMateriali: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglMaterialiv: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, *const libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglMatrixMode: Option::<unsafe extern "C" fn(libc::c_int) -> ()> = None;
#[no_mangle]
pub static mut qglMultMatrixd: Option::<
    unsafe extern "C" fn(*const libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglMultMatrixf: Option::<
    unsafe extern "C" fn(*const libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglNewList: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglNormal3b: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglNormal3bv: Option::<unsafe extern "C" fn(*const libc::c_int) -> ()> = None;
#[no_mangle]
pub static mut qglNormal3d: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglNormal3dv: Option::<unsafe extern "C" fn(*const libc::c_int) -> ()> = None;
#[no_mangle]
pub static mut qglNormal3f: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglNormal3fv: Option::<unsafe extern "C" fn(*const libc::c_int) -> ()> = None;
#[no_mangle]
pub static mut qglNormal3i: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglNormal3iv: Option::<unsafe extern "C" fn(*const libc::c_int) -> ()> = None;
#[no_mangle]
pub static mut qglNormal3s: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglNormal3sv: Option::<unsafe extern "C" fn(*const libc::c_int) -> ()> = None;
#[no_mangle]
pub static mut qglNormalPointer: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, *const libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglOrtho: Option::<
    unsafe extern "C" fn(
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut qglPassThrough: Option::<unsafe extern "C" fn(libc::c_int) -> ()> = None;
#[no_mangle]
pub static mut qglPixelMapfv: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, *const libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglPixelMapuiv: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, *const libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglPixelMapusv: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, *const libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglPixelStoref: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglPixelStorei: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglPixelTransferf: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglPixelTransferi: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglPixelZoom: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglPointSize: Option::<unsafe extern "C" fn(libc::c_int) -> ()> = None;
#[no_mangle]
pub static mut qglPolygonMode: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglPolygonOffset: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglPolygonStipple: Option::<
    unsafe extern "C" fn(*const libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglPopAttrib: Option::<unsafe extern "C" fn() -> ()> = None;
#[no_mangle]
pub static mut qglPopClientAttrib: Option::<unsafe extern "C" fn() -> ()> = None;
#[no_mangle]
pub static mut qglPopMatrix: Option::<unsafe extern "C" fn() -> ()> = None;
#[no_mangle]
pub static mut qglPopName: Option::<unsafe extern "C" fn() -> ()> = None;
#[no_mangle]
pub static mut qglPrioritizeTextures: Option::<
    unsafe extern "C" fn(libc::c_int, *const libc::c_int, *const libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglPushAttrib: Option::<unsafe extern "C" fn(libc::c_int) -> ()> = None;
#[no_mangle]
pub static mut qglPushClientAttrib: Option::<unsafe extern "C" fn(libc::c_int) -> ()> = None;
#[no_mangle]
pub static mut qglPushMatrix: Option::<unsafe extern "C" fn() -> ()> = None;
#[no_mangle]
pub static mut qglPushName: Option::<unsafe extern "C" fn(libc::c_int) -> ()> = None;
#[no_mangle]
pub static mut qglRasterPos2d: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglRasterPos2dv: Option::<
    unsafe extern "C" fn(*const libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglRasterPos2f: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglRasterPos2fv: Option::<
    unsafe extern "C" fn(*const libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglRasterPos2i: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglRasterPos2iv: Option::<
    unsafe extern "C" fn(*const libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglRasterPos2s: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglRasterPos2sv: Option::<
    unsafe extern "C" fn(*const libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglRasterPos3d: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglRasterPos3dv: Option::<
    unsafe extern "C" fn(*const libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglRasterPos3f: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglRasterPos3fv: Option::<
    unsafe extern "C" fn(*const libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglRasterPos3i: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglRasterPos3iv: Option::<
    unsafe extern "C" fn(*const libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglRasterPos3s: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglRasterPos3sv: Option::<
    unsafe extern "C" fn(*const libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglRasterPos4d: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int, libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglRasterPos4dv: Option::<
    unsafe extern "C" fn(*const libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglRasterPos4f: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int, libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglRasterPos4fv: Option::<
    unsafe extern "C" fn(*const libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglRasterPos4i: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int, libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglRasterPos4iv: Option::<
    unsafe extern "C" fn(*const libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglRasterPos4s: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int, libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglRasterPos4sv: Option::<
    unsafe extern "C" fn(*const libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglReadBuffer: Option::<unsafe extern "C" fn(libc::c_int) -> ()> = None;
#[no_mangle]
pub static mut qglReadPixels: Option::<
    unsafe extern "C" fn(
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        *mut libc::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut qglRectd: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int, libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglRectdv: Option::<
    unsafe extern "C" fn(*const libc::c_int, *const libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglRectf: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int, libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglRectfv: Option::<
    unsafe extern "C" fn(*const libc::c_int, *const libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglRecti: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int, libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglRectiv: Option::<
    unsafe extern "C" fn(*const libc::c_int, *const libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglRects: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int, libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglRectsv: Option::<
    unsafe extern "C" fn(*const libc::c_int, *const libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglRotated: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int, libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglRotatef: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int, libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglScaled: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglScalef: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglScissor: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int, libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglSelectBuffer: Option::<
    unsafe extern "C" fn(libc::c_int, *mut libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglShadeModel: Option::<unsafe extern "C" fn(libc::c_int) -> ()> = None;
#[no_mangle]
pub static mut qglStencilFunc: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglStencilMask: Option::<unsafe extern "C" fn(libc::c_int) -> ()> = None;
#[no_mangle]
pub static mut qglStencilOp: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglTexCoord1d: Option::<unsafe extern "C" fn(libc::c_int) -> ()> = None;
#[no_mangle]
pub static mut qglTexCoord1dv: Option::<
    unsafe extern "C" fn(*const libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglTexCoord1f: Option::<unsafe extern "C" fn(libc::c_int) -> ()> = None;
#[no_mangle]
pub static mut qglTexCoord1fv: Option::<
    unsafe extern "C" fn(*const libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglTexCoord1i: Option::<unsafe extern "C" fn(libc::c_int) -> ()> = None;
#[no_mangle]
pub static mut qglTexCoord1iv: Option::<
    unsafe extern "C" fn(*const libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglTexCoord1s: Option::<unsafe extern "C" fn(libc::c_int) -> ()> = None;
#[no_mangle]
pub static mut qglTexCoord1sv: Option::<
    unsafe extern "C" fn(*const libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglTexCoord2d: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglTexCoord2dv: Option::<
    unsafe extern "C" fn(*const libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglTexCoord2f: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglTexCoord2fv: Option::<
    unsafe extern "C" fn(*const libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglTexCoord2i: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglTexCoord2iv: Option::<
    unsafe extern "C" fn(*const libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglTexCoord2s: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglTexCoord2sv: Option::<
    unsafe extern "C" fn(*const libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglTexCoord3d: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglTexCoord3dv: Option::<
    unsafe extern "C" fn(*const libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglTexCoord3f: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglTexCoord3fv: Option::<
    unsafe extern "C" fn(*const libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglTexCoord3i: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglTexCoord3iv: Option::<
    unsafe extern "C" fn(*const libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglTexCoord3s: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglTexCoord3sv: Option::<
    unsafe extern "C" fn(*const libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglTexCoord4d: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int, libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglTexCoord4dv: Option::<
    unsafe extern "C" fn(*const libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglTexCoord4f: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int, libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglTexCoord4fv: Option::<
    unsafe extern "C" fn(*const libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglTexCoord4i: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int, libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglTexCoord4iv: Option::<
    unsafe extern "C" fn(*const libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglTexCoord4s: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int, libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglTexCoord4sv: Option::<
    unsafe extern "C" fn(*const libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglTexCoordPointer: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int, *const libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglTexEnvf: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglTexEnvfv: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, *const libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglTexEnvi: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglTexEnviv: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, *const libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglTexGend: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglTexGendv: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, *const libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglTexGenf: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglTexGenfv: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, *const libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglTexGeni: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglTexGeniv: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, *const libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglTexImage1D: Option::<
    unsafe extern "C" fn(
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        *const libc::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut qglTexImage2D: Option::<
    unsafe extern "C" fn(
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        *const libc::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut qglTexParameterf: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglTexParameterfv: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, *const libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglTexParameteri: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglTexParameteriv: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, *const libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglTexSubImage1D: Option::<
    unsafe extern "C" fn(
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        *const libc::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut qglTexSubImage2D: Option::<
    unsafe extern "C" fn(
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        *const libc::c_int,
    ) -> (),
> = None;
#[no_mangle]
pub static mut qglTranslated: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglTranslatef: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglVertex2d: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglVertex2dv: Option::<unsafe extern "C" fn(*const libc::c_int) -> ()> = None;
#[no_mangle]
pub static mut qglVertex2f: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglVertex2fv: Option::<unsafe extern "C" fn(*const libc::c_int) -> ()> = None;
#[no_mangle]
pub static mut qglVertex2i: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglVertex2iv: Option::<unsafe extern "C" fn(*const libc::c_int) -> ()> = None;
#[no_mangle]
pub static mut qglVertex2s: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglVertex2sv: Option::<unsafe extern "C" fn(*const libc::c_int) -> ()> = None;
#[no_mangle]
pub static mut qglVertex3d: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglVertex3dv: Option::<unsafe extern "C" fn(*const libc::c_int) -> ()> = None;
#[no_mangle]
pub static mut qglVertex3f: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglVertex3fv: Option::<unsafe extern "C" fn(*const libc::c_int) -> ()> = None;
#[no_mangle]
pub static mut qglVertex3i: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglVertex3iv: Option::<unsafe extern "C" fn(*const libc::c_int) -> ()> = None;
#[no_mangle]
pub static mut qglVertex3s: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglVertex3sv: Option::<unsafe extern "C" fn(*const libc::c_int) -> ()> = None;
#[no_mangle]
pub static mut qglVertex4d: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int, libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglVertex4dv: Option::<unsafe extern "C" fn(*const libc::c_int) -> ()> = None;
#[no_mangle]
pub static mut qglVertex4f: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int, libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglVertex4fv: Option::<unsafe extern "C" fn(*const libc::c_int) -> ()> = None;
#[no_mangle]
pub static mut qglVertex4i: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int, libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglVertex4iv: Option::<unsafe extern "C" fn(*const libc::c_int) -> ()> = None;
#[no_mangle]
pub static mut qglVertex4s: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int, libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglVertex4sv: Option::<unsafe extern "C" fn(*const libc::c_int) -> ()> = None;
#[no_mangle]
pub static mut qglVertexPointer: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int, *const libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglViewport: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int, libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglLockArraysEXT: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglUnlockArraysEXT: Option::<unsafe extern "C" fn() -> ()> = None;
#[no_mangle]
pub static mut qglPointParameterfEXT: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglPointParameterfvEXT: Option::<
    unsafe extern "C" fn(libc::c_int, *const libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut qglColorTableEXT: Option::<
    unsafe extern "C" fn(
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        *const libc::c_void,
    ) -> (),
> = None;
#[no_mangle]
pub static mut qglSelectTextureSGIS: Option::<unsafe extern "C" fn() -> ()> = None;
#[no_mangle]
pub static mut qglMTexCoord2fSGIS: Option::<unsafe extern "C" fn() -> ()> = None;
static mut dllAccum: Option::<unsafe extern "C" fn(libc::c_int, libc::c_int) -> ()> = None;
static mut dllAlphaFunc: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int) -> (),
> = None;
static mut dllArrayElement: Option::<unsafe extern "C" fn(libc::c_int) -> ()> = None;
static mut dllBegin: Option::<unsafe extern "C" fn(libc::c_int) -> ()> = None;
static mut dllBindTexture: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int) -> (),
> = None;
static mut dllBitmap: Option::<
    unsafe extern "C" fn(
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        *const libc::c_int,
    ) -> (),
> = None;
static mut dllBlendFunc: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int) -> (),
> = None;
static mut dllCallList: Option::<unsafe extern "C" fn(libc::c_int) -> ()> = None;
static mut dllCallLists: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, *const libc::c_int) -> (),
> = None;
static mut dllClear: Option::<unsafe extern "C" fn(libc::c_int) -> ()> = None;
static mut dllClearAccum: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int, libc::c_int) -> (),
> = None;
static mut dllClearColor: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int, libc::c_int) -> (),
> = None;
static mut dllClearDepth: Option::<unsafe extern "C" fn(libc::c_int) -> ()> = None;
static mut dllClearIndex: Option::<unsafe extern "C" fn(libc::c_int) -> ()> = None;
static mut dllClearStencil: Option::<unsafe extern "C" fn(libc::c_int) -> ()> = None;
static mut dllClipPlane: Option::<
    unsafe extern "C" fn(libc::c_int, *const libc::c_int) -> (),
> = None;
static mut dllColor3b: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int) -> (),
> = None;
static mut dllColor3bv: Option::<unsafe extern "C" fn(*const libc::c_int) -> ()> = None;
static mut dllColor3d: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int) -> (),
> = None;
static mut dllColor3dv: Option::<unsafe extern "C" fn(*const libc::c_int) -> ()> = None;
static mut dllColor3f: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int) -> (),
> = None;
static mut dllColor3fv: Option::<unsafe extern "C" fn(*const libc::c_int) -> ()> = None;
static mut dllColor3i: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int) -> (),
> = None;
static mut dllColor3iv: Option::<unsafe extern "C" fn(*const libc::c_int) -> ()> = None;
static mut dllColor3s: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int) -> (),
> = None;
static mut dllColor3sv: Option::<unsafe extern "C" fn(*const libc::c_int) -> ()> = None;
static mut dllColor3ub: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int) -> (),
> = None;
static mut dllColor3ubv: Option::<unsafe extern "C" fn(*const libc::c_int) -> ()> = None;
static mut dllColor3ui: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int) -> (),
> = None;
static mut dllColor3uiv: Option::<unsafe extern "C" fn(*const libc::c_int) -> ()> = None;
static mut dllColor3us: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int) -> (),
> = None;
static mut dllColor3usv: Option::<unsafe extern "C" fn(*const libc::c_int) -> ()> = None;
static mut dllColor4b: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int, libc::c_int) -> (),
> = None;
static mut dllColor4bv: Option::<unsafe extern "C" fn(*const libc::c_int) -> ()> = None;
static mut dllColor4d: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int, libc::c_int) -> (),
> = None;
static mut dllColor4dv: Option::<unsafe extern "C" fn(*const libc::c_int) -> ()> = None;
static mut dllColor4f: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int, libc::c_int) -> (),
> = None;
static mut dllColor4fv: Option::<unsafe extern "C" fn(*const libc::c_int) -> ()> = None;
static mut dllColor4i: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int, libc::c_int) -> (),
> = None;
static mut dllColor4iv: Option::<unsafe extern "C" fn(*const libc::c_int) -> ()> = None;
static mut dllColor4s: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int, libc::c_int) -> (),
> = None;
static mut dllColor4sv: Option::<unsafe extern "C" fn(*const libc::c_int) -> ()> = None;
static mut dllColor4ub: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int, libc::c_int) -> (),
> = None;
static mut dllColor4ubv: Option::<unsafe extern "C" fn(*const libc::c_int) -> ()> = None;
static mut dllColor4ui: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int, libc::c_int) -> (),
> = None;
static mut dllColor4uiv: Option::<unsafe extern "C" fn(*const libc::c_int) -> ()> = None;
static mut dllColor4us: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int, libc::c_int) -> (),
> = None;
static mut dllColor4usv: Option::<unsafe extern "C" fn(*const libc::c_int) -> ()> = None;
static mut dllColorMask: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int, libc::c_int) -> (),
> = None;
static mut dllColorMaterial: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int) -> (),
> = None;
static mut dllColorPointer: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int, *const libc::c_int) -> (),
> = None;
static mut dllCopyPixels: Option::<
    unsafe extern "C" fn(
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
    ) -> (),
> = None;
static mut dllCopyTexImage1D: Option::<
    unsafe extern "C" fn(
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
    ) -> (),
> = None;
static mut dllCopyTexImage2D: Option::<
    unsafe extern "C" fn(
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
    ) -> (),
> = None;
static mut dllCopyTexSubImage1D: Option::<
    unsafe extern "C" fn(
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
    ) -> (),
> = None;
static mut dllCopyTexSubImage2D: Option::<
    unsafe extern "C" fn(
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
    ) -> (),
> = None;
static mut dllCullFace: Option::<unsafe extern "C" fn(libc::c_int) -> ()> = None;
static mut dllDeleteLists: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int) -> (),
> = None;
static mut dllDeleteTextures: Option::<
    unsafe extern "C" fn(libc::c_int, *const libc::c_int) -> (),
> = None;
static mut dllDepthFunc: Option::<unsafe extern "C" fn(libc::c_int) -> ()> = None;
static mut dllDepthMask: Option::<unsafe extern "C" fn(libc::c_int) -> ()> = None;
static mut dllDepthRange: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int) -> (),
> = None;
static mut dllDisable: Option::<unsafe extern "C" fn(libc::c_int) -> ()> = None;
static mut dllDisableClientState: Option::<unsafe extern "C" fn(libc::c_int) -> ()> = None;
static mut dllDrawArrays: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int) -> (),
> = None;
static mut dllDrawBuffer: Option::<unsafe extern "C" fn(libc::c_int) -> ()> = None;
static mut dllDrawElements: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int, *const libc::c_int) -> (),
> = None;
static mut dllDrawPixels: Option::<
    unsafe extern "C" fn(
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        *const libc::c_int,
    ) -> (),
> = None;
static mut dllEdgeFlag: Option::<unsafe extern "C" fn(libc::c_int) -> ()> = None;
static mut dllEdgeFlagPointer: Option::<
    unsafe extern "C" fn(libc::c_int, *const libc::c_int) -> (),
> = None;
static mut dllEdgeFlagv: Option::<unsafe extern "C" fn(*const libc::c_int) -> ()> = None;
static mut dllEnable: Option::<unsafe extern "C" fn(libc::c_int) -> ()> = None;
static mut dllEnableClientState: Option::<unsafe extern "C" fn(libc::c_int) -> ()> = None;
static mut dllEnd: Option::<unsafe extern "C" fn() -> ()> = None;
static mut dllEndList: Option::<unsafe extern "C" fn() -> ()> = None;
static mut dllEvalCoord1d: Option::<unsafe extern "C" fn(libc::c_int) -> ()> = None;
static mut dllEvalCoord1dv: Option::<unsafe extern "C" fn(*const libc::c_int) -> ()> = None;
static mut dllEvalCoord1f: Option::<unsafe extern "C" fn(libc::c_int) -> ()> = None;
static mut dllEvalCoord1fv: Option::<unsafe extern "C" fn(*const libc::c_int) -> ()> = None;
static mut dllEvalCoord2d: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int) -> (),
> = None;
static mut dllEvalCoord2dv: Option::<unsafe extern "C" fn(*const libc::c_int) -> ()> = None;
static mut dllEvalCoord2f: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int) -> (),
> = None;
static mut dllEvalCoord2fv: Option::<unsafe extern "C" fn(*const libc::c_int) -> ()> = None;
static mut dllEvalMesh1: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int) -> (),
> = None;
static mut dllEvalMesh2: Option::<
    unsafe extern "C" fn(
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
    ) -> (),
> = None;
static mut dllEvalPoint1: Option::<unsafe extern "C" fn(libc::c_int) -> ()> = None;
static mut dllEvalPoint2: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int) -> (),
> = None;
static mut dllFeedbackBuffer: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, *mut libc::c_int) -> (),
> = None;
static mut dllFinish: Option::<unsafe extern "C" fn() -> ()> = None;
static mut dllFlush: Option::<unsafe extern "C" fn() -> ()> = None;
static mut dllFogf: Option::<unsafe extern "C" fn(libc::c_int, libc::c_int) -> ()> = None;
static mut dllFogfv: Option::<
    unsafe extern "C" fn(libc::c_int, *const libc::c_int) -> (),
> = None;
static mut dllFogi: Option::<unsafe extern "C" fn(libc::c_int, libc::c_int) -> ()> = None;
static mut dllFogiv: Option::<
    unsafe extern "C" fn(libc::c_int, *const libc::c_int) -> (),
> = None;
static mut dllFrontFace: Option::<unsafe extern "C" fn(libc::c_int) -> ()> = None;
static mut dllFrustum: Option::<
    unsafe extern "C" fn(
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
    ) -> (),
> = None;
static mut dllGenTextures: Option::<
    unsafe extern "C" fn(libc::c_int, *mut libc::c_int) -> (),
> = None;
static mut dllGetBooleanv: Option::<
    unsafe extern "C" fn(libc::c_int, *mut libc::c_int) -> (),
> = None;
static mut dllGetClipPlane: Option::<
    unsafe extern "C" fn(libc::c_int, *mut libc::c_int) -> (),
> = None;
static mut dllGetDoublev: Option::<
    unsafe extern "C" fn(libc::c_int, *mut libc::c_int) -> (),
> = None;
static mut dllGetFloatv: Option::<
    unsafe extern "C" fn(libc::c_int, *mut libc::c_int) -> (),
> = None;
static mut dllGetIntegerv: Option::<
    unsafe extern "C" fn(libc::c_int, *mut libc::c_int) -> (),
> = None;
static mut dllGetLightfv: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, *mut libc::c_int) -> (),
> = None;
static mut dllGetLightiv: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, *mut libc::c_int) -> (),
> = None;
static mut dllGetMapdv: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, *mut libc::c_int) -> (),
> = None;
static mut dllGetMapfv: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, *mut libc::c_int) -> (),
> = None;
static mut dllGetMapiv: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, *mut libc::c_int) -> (),
> = None;
static mut dllGetMaterialfv: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, *mut libc::c_int) -> (),
> = None;
static mut dllGetMaterialiv: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, *mut libc::c_int) -> (),
> = None;
static mut dllGetPixelMapfv: Option::<
    unsafe extern "C" fn(libc::c_int, *mut libc::c_int) -> (),
> = None;
static mut dllGetPixelMapuiv: Option::<
    unsafe extern "C" fn(libc::c_int, *mut libc::c_int) -> (),
> = None;
static mut dllGetPixelMapusv: Option::<
    unsafe extern "C" fn(libc::c_int, *mut libc::c_int) -> (),
> = None;
static mut dllGetPointerv: Option::<
    unsafe extern "C" fn(libc::c_int, *mut *mut libc::c_int) -> (),
> = None;
static mut dllGetPolygonStipple: Option::<
    unsafe extern "C" fn(*mut libc::c_int) -> (),
> = None;
#[no_mangle]
pub static mut dllGetString: Option::<
    unsafe extern "C" fn(libc::c_int) -> *const libc::c_int,
> = None;
static mut dllGetTexEnvfv: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, *mut libc::c_int) -> (),
> = None;
static mut dllGetTexEnviv: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, *mut libc::c_int) -> (),
> = None;
static mut dllGetTexGendv: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, *mut libc::c_int) -> (),
> = None;
static mut dllGetTexGenfv: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, *mut libc::c_int) -> (),
> = None;
static mut dllGetTexGeniv: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, *mut libc::c_int) -> (),
> = None;
static mut dllGetTexImage: Option::<
    unsafe extern "C" fn(
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        *mut libc::c_int,
    ) -> (),
> = None;
static mut dllGetTexLevelParameterfv: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int, *mut libc::c_int) -> (),
> = None;
static mut dllGetTexLevelParameteriv: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int, *mut libc::c_int) -> (),
> = None;
static mut dllGetTexParameterfv: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, *mut libc::c_int) -> (),
> = None;
static mut dllGetTexParameteriv: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, *mut libc::c_int) -> (),
> = None;
static mut dllHint: Option::<unsafe extern "C" fn(libc::c_int, libc::c_int) -> ()> = None;
static mut dllIndexMask: Option::<unsafe extern "C" fn(libc::c_int) -> ()> = None;
static mut dllIndexPointer: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, *const libc::c_int) -> (),
> = None;
static mut dllIndexd: Option::<unsafe extern "C" fn(libc::c_int) -> ()> = None;
static mut dllIndexdv: Option::<unsafe extern "C" fn(*const libc::c_int) -> ()> = None;
static mut dllIndexf: Option::<unsafe extern "C" fn(libc::c_int) -> ()> = None;
static mut dllIndexfv: Option::<unsafe extern "C" fn(*const libc::c_int) -> ()> = None;
static mut dllIndexi: Option::<unsafe extern "C" fn(libc::c_int) -> ()> = None;
static mut dllIndexiv: Option::<unsafe extern "C" fn(*const libc::c_int) -> ()> = None;
static mut dllIndexs: Option::<unsafe extern "C" fn(libc::c_int) -> ()> = None;
static mut dllIndexsv: Option::<unsafe extern "C" fn(*const libc::c_int) -> ()> = None;
static mut dllIndexub: Option::<unsafe extern "C" fn(libc::c_int) -> ()> = None;
static mut dllIndexubv: Option::<unsafe extern "C" fn(*const libc::c_int) -> ()> = None;
static mut dllInitNames: Option::<unsafe extern "C" fn() -> ()> = None;
static mut dllInterleavedArrays: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, *const libc::c_int) -> (),
> = None;
static mut dllLightModelf: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int) -> (),
> = None;
static mut dllLightModelfv: Option::<
    unsafe extern "C" fn(libc::c_int, *const libc::c_int) -> (),
> = None;
static mut dllLightModeli: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int) -> (),
> = None;
static mut dllLightModeliv: Option::<
    unsafe extern "C" fn(libc::c_int, *const libc::c_int) -> (),
> = None;
static mut dllLightf: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int) -> (),
> = None;
static mut dllLightfv: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, *const libc::c_int) -> (),
> = None;
static mut dllLighti: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int) -> (),
> = None;
static mut dllLightiv: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, *const libc::c_int) -> (),
> = None;
static mut dllLineStipple: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int) -> (),
> = None;
static mut dllLineWidth: Option::<unsafe extern "C" fn(libc::c_int) -> ()> = None;
static mut dllListBase: Option::<unsafe extern "C" fn(libc::c_int) -> ()> = None;
static mut dllLoadIdentity: Option::<unsafe extern "C" fn() -> ()> = None;
static mut dllLoadMatrixd: Option::<unsafe extern "C" fn(*const libc::c_int) -> ()> = None;
static mut dllLoadMatrixf: Option::<unsafe extern "C" fn(*const libc::c_int) -> ()> = None;
static mut dllLoadName: Option::<unsafe extern "C" fn(libc::c_int) -> ()> = None;
static mut dllLogicOp: Option::<unsafe extern "C" fn(libc::c_int) -> ()> = None;
static mut dllMap1d: Option::<
    unsafe extern "C" fn(
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        *const libc::c_int,
    ) -> (),
> = None;
static mut dllMap1f: Option::<
    unsafe extern "C" fn(
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        *const libc::c_int,
    ) -> (),
> = None;
static mut dllMap2d: Option::<
    unsafe extern "C" fn(
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        *const libc::c_int,
    ) -> (),
> = None;
static mut dllMap2f: Option::<
    unsafe extern "C" fn(
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        *const libc::c_int,
    ) -> (),
> = None;
static mut dllMapGrid1d: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int) -> (),
> = None;
static mut dllMapGrid1f: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int) -> (),
> = None;
static mut dllMapGrid2d: Option::<
    unsafe extern "C" fn(
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
    ) -> (),
> = None;
static mut dllMapGrid2f: Option::<
    unsafe extern "C" fn(
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
    ) -> (),
> = None;
static mut dllMaterialf: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int) -> (),
> = None;
static mut dllMaterialfv: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, *const libc::c_int) -> (),
> = None;
static mut dllMateriali: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int) -> (),
> = None;
static mut dllMaterialiv: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, *const libc::c_int) -> (),
> = None;
static mut dllMatrixMode: Option::<unsafe extern "C" fn(libc::c_int) -> ()> = None;
static mut dllMultMatrixd: Option::<unsafe extern "C" fn(*const libc::c_int) -> ()> = None;
static mut dllMultMatrixf: Option::<unsafe extern "C" fn(*const libc::c_int) -> ()> = None;
static mut dllNewList: Option::<unsafe extern "C" fn(libc::c_int, libc::c_int) -> ()> = None;
static mut dllNormal3b: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int) -> (),
> = None;
static mut dllNormal3bv: Option::<unsafe extern "C" fn(*const libc::c_int) -> ()> = None;
static mut dllNormal3d: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int) -> (),
> = None;
static mut dllNormal3dv: Option::<unsafe extern "C" fn(*const libc::c_int) -> ()> = None;
static mut dllNormal3f: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int) -> (),
> = None;
static mut dllNormal3fv: Option::<unsafe extern "C" fn(*const libc::c_int) -> ()> = None;
static mut dllNormal3i: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int) -> (),
> = None;
static mut dllNormal3iv: Option::<unsafe extern "C" fn(*const libc::c_int) -> ()> = None;
static mut dllNormal3s: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int) -> (),
> = None;
static mut dllNormal3sv: Option::<unsafe extern "C" fn(*const libc::c_int) -> ()> = None;
static mut dllNormalPointer: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, *const libc::c_int) -> (),
> = None;
static mut dllOrtho: Option::<
    unsafe extern "C" fn(
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
    ) -> (),
> = None;
static mut dllPassThrough: Option::<unsafe extern "C" fn(libc::c_int) -> ()> = None;
static mut dllPixelMapfv: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, *const libc::c_int) -> (),
> = None;
static mut dllPixelMapuiv: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, *const libc::c_int) -> (),
> = None;
static mut dllPixelMapusv: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, *const libc::c_int) -> (),
> = None;
static mut dllPixelStoref: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int) -> (),
> = None;
static mut dllPixelStorei: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int) -> (),
> = None;
static mut dllPixelTransferf: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int) -> (),
> = None;
static mut dllPixelTransferi: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int) -> (),
> = None;
static mut dllPixelZoom: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int) -> (),
> = None;
static mut dllPointSize: Option::<unsafe extern "C" fn(libc::c_int) -> ()> = None;
static mut dllPolygonMode: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int) -> (),
> = None;
static mut dllPolygonOffset: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int) -> (),
> = None;
static mut dllPolygonStipple: Option::<unsafe extern "C" fn(*const libc::c_int) -> ()> = None;
static mut dllPopAttrib: Option::<unsafe extern "C" fn() -> ()> = None;
static mut dllPopClientAttrib: Option::<unsafe extern "C" fn() -> ()> = None;
static mut dllPopMatrix: Option::<unsafe extern "C" fn() -> ()> = None;
static mut dllPopName: Option::<unsafe extern "C" fn() -> ()> = None;
static mut dllPrioritizeTextures: Option::<
    unsafe extern "C" fn(libc::c_int, *const libc::c_int, *const libc::c_int) -> (),
> = None;
static mut dllPushAttrib: Option::<unsafe extern "C" fn(libc::c_int) -> ()> = None;
static mut dllPushClientAttrib: Option::<unsafe extern "C" fn(libc::c_int) -> ()> = None;
static mut dllPushMatrix: Option::<unsafe extern "C" fn() -> ()> = None;
static mut dllPushName: Option::<unsafe extern "C" fn(libc::c_int) -> ()> = None;
static mut dllRasterPos2d: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int) -> (),
> = None;
static mut dllRasterPos2dv: Option::<unsafe extern "C" fn(*const libc::c_int) -> ()> = None;
static mut dllRasterPos2f: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int) -> (),
> = None;
static mut dllRasterPos2fv: Option::<unsafe extern "C" fn(*const libc::c_int) -> ()> = None;
static mut dllRasterPos2i: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int) -> (),
> = None;
static mut dllRasterPos2iv: Option::<unsafe extern "C" fn(*const libc::c_int) -> ()> = None;
static mut dllRasterPos2s: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int) -> (),
> = None;
static mut dllRasterPos2sv: Option::<unsafe extern "C" fn(*const libc::c_int) -> ()> = None;
static mut dllRasterPos3d: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int) -> (),
> = None;
static mut dllRasterPos3dv: Option::<unsafe extern "C" fn(*const libc::c_int) -> ()> = None;
static mut dllRasterPos3f: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int) -> (),
> = None;
static mut dllRasterPos3fv: Option::<unsafe extern "C" fn(*const libc::c_int) -> ()> = None;
static mut dllRasterPos3i: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int) -> (),
> = None;
static mut dllRasterPos3iv: Option::<unsafe extern "C" fn(*const libc::c_int) -> ()> = None;
static mut dllRasterPos3s: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int) -> (),
> = None;
static mut dllRasterPos3sv: Option::<unsafe extern "C" fn(*const libc::c_int) -> ()> = None;
static mut dllRasterPos4d: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int, libc::c_int) -> (),
> = None;
static mut dllRasterPos4dv: Option::<unsafe extern "C" fn(*const libc::c_int) -> ()> = None;
static mut dllRasterPos4f: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int, libc::c_int) -> (),
> = None;
static mut dllRasterPos4fv: Option::<unsafe extern "C" fn(*const libc::c_int) -> ()> = None;
static mut dllRasterPos4i: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int, libc::c_int) -> (),
> = None;
static mut dllRasterPos4iv: Option::<unsafe extern "C" fn(*const libc::c_int) -> ()> = None;
static mut dllRasterPos4s: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int, libc::c_int) -> (),
> = None;
static mut dllRasterPos4sv: Option::<unsafe extern "C" fn(*const libc::c_int) -> ()> = None;
static mut dllReadBuffer: Option::<unsafe extern "C" fn(libc::c_int) -> ()> = None;
static mut dllReadPixels: Option::<
    unsafe extern "C" fn(
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        *mut libc::c_int,
    ) -> (),
> = None;
static mut dllRectd: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int, libc::c_int) -> (),
> = None;
static mut dllRectdv: Option::<
    unsafe extern "C" fn(*const libc::c_int, *const libc::c_int) -> (),
> = None;
static mut dllRectf: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int, libc::c_int) -> (),
> = None;
static mut dllRectfv: Option::<
    unsafe extern "C" fn(*const libc::c_int, *const libc::c_int) -> (),
> = None;
static mut dllRecti: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int, libc::c_int) -> (),
> = None;
static mut dllRectiv: Option::<
    unsafe extern "C" fn(*const libc::c_int, *const libc::c_int) -> (),
> = None;
static mut dllRects: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int, libc::c_int) -> (),
> = None;
static mut dllRectsv: Option::<
    unsafe extern "C" fn(*const libc::c_int, *const libc::c_int) -> (),
> = None;
static mut dllRotated: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int, libc::c_int) -> (),
> = None;
static mut dllRotatef: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int, libc::c_int) -> (),
> = None;
static mut dllScaled: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int) -> (),
> = None;
static mut dllScalef: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int) -> (),
> = None;
static mut dllScissor: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int, libc::c_int) -> (),
> = None;
static mut dllSelectBuffer: Option::<
    unsafe extern "C" fn(libc::c_int, *mut libc::c_int) -> (),
> = None;
static mut dllShadeModel: Option::<unsafe extern "C" fn(libc::c_int) -> ()> = None;
static mut dllStencilFunc: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int) -> (),
> = None;
static mut dllStencilMask: Option::<unsafe extern "C" fn(libc::c_int) -> ()> = None;
static mut dllStencilOp: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int) -> (),
> = None;
static mut dllTexCoord1d: Option::<unsafe extern "C" fn(libc::c_int) -> ()> = None;
static mut dllTexCoord1dv: Option::<unsafe extern "C" fn(*const libc::c_int) -> ()> = None;
static mut dllTexCoord1f: Option::<unsafe extern "C" fn(libc::c_int) -> ()> = None;
static mut dllTexCoord1fv: Option::<unsafe extern "C" fn(*const libc::c_int) -> ()> = None;
static mut dllTexCoord1i: Option::<unsafe extern "C" fn(libc::c_int) -> ()> = None;
static mut dllTexCoord1iv: Option::<unsafe extern "C" fn(*const libc::c_int) -> ()> = None;
static mut dllTexCoord1s: Option::<unsafe extern "C" fn(libc::c_int) -> ()> = None;
static mut dllTexCoord1sv: Option::<unsafe extern "C" fn(*const libc::c_int) -> ()> = None;
static mut dllTexCoord2d: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int) -> (),
> = None;
static mut dllTexCoord2dv: Option::<unsafe extern "C" fn(*const libc::c_int) -> ()> = None;
static mut dllTexCoord2f: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int) -> (),
> = None;
static mut dllTexCoord2fv: Option::<unsafe extern "C" fn(*const libc::c_int) -> ()> = None;
static mut dllTexCoord2i: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int) -> (),
> = None;
static mut dllTexCoord2iv: Option::<unsafe extern "C" fn(*const libc::c_int) -> ()> = None;
static mut dllTexCoord2s: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int) -> (),
> = None;
static mut dllTexCoord2sv: Option::<unsafe extern "C" fn(*const libc::c_int) -> ()> = None;
static mut dllTexCoord3d: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int) -> (),
> = None;
static mut dllTexCoord3dv: Option::<unsafe extern "C" fn(*const libc::c_int) -> ()> = None;
static mut dllTexCoord3f: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int) -> (),
> = None;
static mut dllTexCoord3fv: Option::<unsafe extern "C" fn(*const libc::c_int) -> ()> = None;
static mut dllTexCoord3i: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int) -> (),
> = None;
static mut dllTexCoord3iv: Option::<unsafe extern "C" fn(*const libc::c_int) -> ()> = None;
static mut dllTexCoord3s: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int) -> (),
> = None;
static mut dllTexCoord3sv: Option::<unsafe extern "C" fn(*const libc::c_int) -> ()> = None;
static mut dllTexCoord4d: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int, libc::c_int) -> (),
> = None;
static mut dllTexCoord4dv: Option::<unsafe extern "C" fn(*const libc::c_int) -> ()> = None;
static mut dllTexCoord4f: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int, libc::c_int) -> (),
> = None;
static mut dllTexCoord4fv: Option::<unsafe extern "C" fn(*const libc::c_int) -> ()> = None;
static mut dllTexCoord4i: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int, libc::c_int) -> (),
> = None;
static mut dllTexCoord4iv: Option::<unsafe extern "C" fn(*const libc::c_int) -> ()> = None;
static mut dllTexCoord4s: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int, libc::c_int) -> (),
> = None;
static mut dllTexCoord4sv: Option::<unsafe extern "C" fn(*const libc::c_int) -> ()> = None;
static mut dllTexCoordPointer: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int, *const libc::c_int) -> (),
> = None;
static mut dllTexEnvf: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int) -> (),
> = None;
static mut dllTexEnvfv: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, *const libc::c_int) -> (),
> = None;
static mut dllTexEnvi: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int) -> (),
> = None;
static mut dllTexEnviv: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, *const libc::c_int) -> (),
> = None;
static mut dllTexGend: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int) -> (),
> = None;
static mut dllTexGendv: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, *const libc::c_int) -> (),
> = None;
static mut dllTexGenf: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int) -> (),
> = None;
static mut dllTexGenfv: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, *const libc::c_int) -> (),
> = None;
static mut dllTexGeni: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int) -> (),
> = None;
static mut dllTexGeniv: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, *const libc::c_int) -> (),
> = None;
static mut dllTexImage1D: Option::<
    unsafe extern "C" fn(
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        *const libc::c_int,
    ) -> (),
> = None;
static mut dllTexImage2D: Option::<
    unsafe extern "C" fn(
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        *const libc::c_int,
    ) -> (),
> = None;
static mut dllTexParameterf: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int) -> (),
> = None;
static mut dllTexParameterfv: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, *const libc::c_int) -> (),
> = None;
static mut dllTexParameteri: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int) -> (),
> = None;
static mut dllTexParameteriv: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, *const libc::c_int) -> (),
> = None;
static mut dllTexSubImage1D: Option::<
    unsafe extern "C" fn(
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        *const libc::c_int,
    ) -> (),
> = None;
static mut dllTexSubImage2D: Option::<
    unsafe extern "C" fn(
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        libc::c_int,
        *const libc::c_int,
    ) -> (),
> = None;
static mut dllTranslated: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int) -> (),
> = None;
static mut dllTranslatef: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int) -> (),
> = None;
static mut dllVertex2d: Option::<unsafe extern "C" fn(libc::c_int, libc::c_int) -> ()> = None;
static mut dllVertex2dv: Option::<unsafe extern "C" fn(*const libc::c_int) -> ()> = None;
static mut dllVertex2f: Option::<unsafe extern "C" fn(libc::c_int, libc::c_int) -> ()> = None;
static mut dllVertex2fv: Option::<unsafe extern "C" fn(*const libc::c_int) -> ()> = None;
static mut dllVertex2i: Option::<unsafe extern "C" fn(libc::c_int, libc::c_int) -> ()> = None;
static mut dllVertex2iv: Option::<unsafe extern "C" fn(*const libc::c_int) -> ()> = None;
static mut dllVertex2s: Option::<unsafe extern "C" fn(libc::c_int, libc::c_int) -> ()> = None;
static mut dllVertex2sv: Option::<unsafe extern "C" fn(*const libc::c_int) -> ()> = None;
static mut dllVertex3d: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int) -> (),
> = None;
static mut dllVertex3dv: Option::<unsafe extern "C" fn(*const libc::c_int) -> ()> = None;
static mut dllVertex3f: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int) -> (),
> = None;
static mut dllVertex3fv: Option::<unsafe extern "C" fn(*const libc::c_int) -> ()> = None;
static mut dllVertex3i: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int) -> (),
> = None;
static mut dllVertex3iv: Option::<unsafe extern "C" fn(*const libc::c_int) -> ()> = None;
static mut dllVertex3s: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int) -> (),
> = None;
static mut dllVertex3sv: Option::<unsafe extern "C" fn(*const libc::c_int) -> ()> = None;
static mut dllVertex4d: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int, libc::c_int) -> (),
> = None;
static mut dllVertex4dv: Option::<unsafe extern "C" fn(*const libc::c_int) -> ()> = None;
static mut dllVertex4f: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int, libc::c_int) -> (),
> = None;
static mut dllVertex4fv: Option::<unsafe extern "C" fn(*const libc::c_int) -> ()> = None;
static mut dllVertex4i: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int, libc::c_int) -> (),
> = None;
static mut dllVertex4iv: Option::<unsafe extern "C" fn(*const libc::c_int) -> ()> = None;
static mut dllVertex4s: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int, libc::c_int) -> (),
> = None;
static mut dllVertex4sv: Option::<unsafe extern "C" fn(*const libc::c_int) -> ()> = None;
static mut dllVertexPointer: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int, *const libc::c_int) -> (),
> = None;
static mut dllViewport: Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int, libc::c_int) -> (),
> = None;
unsafe extern "C" fn logEnd() {
    fprintf(glw_state.log_fp, b"glEnd\n\0" as *const u8 as *const libc::c_char);
    dllEnd.expect("non-null function pointer")();
}
unsafe extern "C" fn logEndList() {
    fprintf(glw_state.log_fp, b"glEndList\n\0" as *const u8 as *const libc::c_char);
    dllEndList.expect("non-null function pointer")();
}
unsafe extern "C" fn logFinish() {
    fprintf(glw_state.log_fp, b"glFinish\n\0" as *const u8 as *const libc::c_char);
    dllFinish.expect("non-null function pointer")();
}
unsafe extern "C" fn logFlush() {
    fprintf(glw_state.log_fp, b"glFlush\n\0" as *const u8 as *const libc::c_char);
    dllFlush.expect("non-null function pointer")();
}
unsafe extern "C" fn logInitNames() {
    fprintf(glw_state.log_fp, b"glInitNames\n\0" as *const u8 as *const libc::c_char);
    dllInitNames.expect("non-null function pointer")();
}
unsafe extern "C" fn logLoadIdentity() {
    fprintf(glw_state.log_fp, b"glLoadIdentity\n\0" as *const u8 as *const libc::c_char);
    dllLoadIdentity.expect("non-null function pointer")();
}
unsafe extern "C" fn logPopAttrib() {
    fprintf(glw_state.log_fp, b"glPopAttrib\n\0" as *const u8 as *const libc::c_char);
    dllPopAttrib.expect("non-null function pointer")();
}
unsafe extern "C" fn logPopClientAttrib() {
    fprintf(
        glw_state.log_fp,
        b"glPopClientAttrib\n\0" as *const u8 as *const libc::c_char,
    );
    dllPopClientAttrib.expect("non-null function pointer")();
}
unsafe extern "C" fn logPopMatrix() {
    fprintf(glw_state.log_fp, b"glPopMatrix\n\0" as *const u8 as *const libc::c_char);
    dllPopMatrix.expect("non-null function pointer")();
}
unsafe extern "C" fn logPopName() {
    fprintf(glw_state.log_fp, b"glPopName\n\0" as *const u8 as *const libc::c_char);
    dllPopName.expect("non-null function pointer")();
}
unsafe extern "C" fn logPushMatrix() {
    fprintf(glw_state.log_fp, b"glPushMatrix\n\0" as *const u8 as *const libc::c_char);
    dllPushMatrix.expect("non-null function pointer")();
}
#[no_mangle]
pub unsafe extern "C" fn GLimp_EnableLogging(mut enable: qboolean) {
    if enable as u64 != 0 {
        if (glw_state.log_fp).is_null() {
            let mut newtime: *mut tm = 0 as *mut tm;
            let mut aclock: time_t = 0;
            let mut buffer: [libc::c_char; 1024] = [0; 1024];
            time(&mut aclock);
            newtime = localtime(&mut aclock);
            asctime(newtime);
            Com_sprintf(
                buffer.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong
                    as libc::c_int,
                b"%s/gl.log\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                (ri.FS_Gamedir).expect("non-null function pointer")(),
            );
            glw_state
                .log_fp = fopen(
                buffer.as_mut_ptr(),
                b"wt\0" as *const u8 as *const libc::c_char,
            );
            fprintf(
                glw_state.log_fp,
                b"%s\n\0" as *const u8 as *const libc::c_char,
                asctime(newtime),
            );
        }
        qglEnd = Some(logEnd as unsafe extern "C" fn() -> ());
        qglEndList = Some(logEndList as unsafe extern "C" fn() -> ());
        qglFinish = Some(logFinish as unsafe extern "C" fn() -> ());
        qglFlush = Some(logFlush as unsafe extern "C" fn() -> ());
        qglInitNames = Some(logInitNames as unsafe extern "C" fn() -> ());
        qglLoadIdentity = Some(logLoadIdentity as unsafe extern "C" fn() -> ());
        qglPopAttrib = Some(logPopAttrib as unsafe extern "C" fn() -> ());
        qglPopClientAttrib = Some(logPopClientAttrib as unsafe extern "C" fn() -> ());
        qglPopMatrix = Some(logPopMatrix as unsafe extern "C" fn() -> ());
        qglPopName = Some(logPopName as unsafe extern "C" fn() -> ());
        qglPushMatrix = Some(logPushMatrix as unsafe extern "C" fn() -> ());
    } else {
        qglAccum = dllAccum;
        qglAlphaFunc = dllAlphaFunc;
        qglArrayElement = dllArrayElement;
        qglBegin = dllBegin;
        qglBindTexture = dllBindTexture;
        qglBitmap = dllBitmap;
        qglBlendFunc = dllBlendFunc;
        qglCallList = dllCallList;
        qglCallLists = dllCallLists;
        qglClear = dllClear;
        qglClearAccum = dllClearAccum;
        qglClearColor = dllClearColor;
        qglClearDepth = dllClearDepth;
        qglClearIndex = dllClearIndex;
        qglClearStencil = dllClearStencil;
        qglClipPlane = dllClipPlane;
        qglColor3b = dllColor3b;
        qglColor3bv = dllColor3bv;
        qglColor3d = dllColor3d;
        qglColor3dv = dllColor3dv;
        qglColor3f = dllColor3f;
        qglColor3fv = dllColor3fv;
        qglColor3i = dllColor3i;
        qglColor3iv = dllColor3iv;
        qglColor3s = dllColor3s;
        qglColor3sv = dllColor3sv;
        qglColor3ub = dllColor3ub;
        qglColor3ubv = dllColor3ubv;
        qglColor3ui = dllColor3ui;
        qglColor3uiv = dllColor3uiv;
        qglColor3us = dllColor3us;
        qglColor3usv = dllColor3usv;
        qglColor4b = dllColor4b;
        qglColor4bv = dllColor4bv;
        qglColor4d = dllColor4d;
        qglColor4dv = dllColor4dv;
        qglColor4f = dllColor4f;
        qglColor4fv = dllColor4fv;
        qglColor4i = dllColor4i;
        qglColor4iv = dllColor4iv;
        qglColor4s = dllColor4s;
        qglColor4sv = dllColor4sv;
        qglColor4ub = dllColor4ub;
        qglColor4ubv = dllColor4ubv;
        qglColor4ui = dllColor4ui;
        qglColor4uiv = dllColor4uiv;
        qglColor4us = dllColor4us;
        qglColor4usv = dllColor4usv;
        qglColorMask = dllColorMask;
        qglColorMaterial = dllColorMaterial;
        qglColorPointer = dllColorPointer;
        qglCopyPixels = dllCopyPixels;
        qglCopyTexImage1D = dllCopyTexImage1D;
        qglCopyTexImage2D = dllCopyTexImage2D;
        qglCopyTexSubImage1D = dllCopyTexSubImage1D;
        qglCopyTexSubImage2D = dllCopyTexSubImage2D;
        qglCullFace = dllCullFace;
        qglDeleteLists = dllDeleteLists;
        qglDeleteTextures = dllDeleteTextures;
        qglDepthFunc = dllDepthFunc;
        qglDepthMask = dllDepthMask;
        qglDepthRange = dllDepthRange;
        qglDisable = dllDisable;
        qglDisableClientState = dllDisableClientState;
        qglDrawArrays = dllDrawArrays;
        qglDrawBuffer = dllDrawBuffer;
        qglDrawElements = dllDrawElements;
        qglDrawPixels = dllDrawPixels;
        qglEdgeFlag = dllEdgeFlag;
        qglEdgeFlagPointer = dllEdgeFlagPointer;
        qglEdgeFlagv = dllEdgeFlagv;
        qglEnable = dllEnable;
        qglEnableClientState = dllEnableClientState;
        qglEnd = dllEnd;
        qglEndList = dllEndList;
        qglEvalCoord1d = dllEvalCoord1d;
        qglEvalCoord1dv = dllEvalCoord1dv;
        qglEvalCoord1f = dllEvalCoord1f;
        qglEvalCoord1fv = dllEvalCoord1fv;
        qglEvalCoord2d = dllEvalCoord2d;
        qglEvalCoord2dv = dllEvalCoord2dv;
        qglEvalCoord2f = dllEvalCoord2f;
        qglEvalCoord2fv = dllEvalCoord2fv;
        qglEvalMesh1 = dllEvalMesh1;
        qglEvalMesh2 = dllEvalMesh2;
        qglEvalPoint1 = dllEvalPoint1;
        qglEvalPoint2 = dllEvalPoint2;
        qglFeedbackBuffer = dllFeedbackBuffer;
        qglFinish = dllFinish;
        qglFlush = dllFlush;
        qglFogf = dllFogf;
        qglFogfv = dllFogfv;
        qglFogi = dllFogi;
        qglFogiv = dllFogiv;
        qglFrontFace = dllFrontFace;
        qglFrustum = dllFrustum;
        qglGenTextures = dllGenTextures;
        qglGetBooleanv = dllGetBooleanv;
        qglGetClipPlane = dllGetClipPlane;
        qglGetDoublev = dllGetDoublev;
        qglGetFloatv = dllGetFloatv;
        qglGetIntegerv = dllGetIntegerv;
        qglGetLightfv = dllGetLightfv;
        qglGetLightiv = dllGetLightiv;
        qglGetMapdv = dllGetMapdv;
        qglGetMapfv = dllGetMapfv;
        qglGetMapiv = dllGetMapiv;
        qglGetMaterialfv = dllGetMaterialfv;
        qglGetMaterialiv = dllGetMaterialiv;
        qglGetPixelMapfv = dllGetPixelMapfv;
        qglGetPixelMapuiv = dllGetPixelMapuiv;
        qglGetPixelMapusv = dllGetPixelMapusv;
        qglGetPointerv = dllGetPointerv;
        qglGetPolygonStipple = dllGetPolygonStipple;
        qglGetTexEnvfv = dllGetTexEnvfv;
        qglGetTexEnviv = dllGetTexEnviv;
        qglGetTexGendv = dllGetTexGendv;
        qglGetTexGenfv = dllGetTexGenfv;
        qglGetTexGeniv = dllGetTexGeniv;
        qglGetTexImage = dllGetTexImage;
        qglGetTexLevelParameterfv = dllGetTexLevelParameterfv;
        qglGetTexLevelParameteriv = dllGetTexLevelParameteriv;
        qglGetTexParameterfv = dllGetTexParameterfv;
        qglGetTexParameteriv = dllGetTexParameteriv;
        qglHint = dllHint;
        qglIndexMask = dllIndexMask;
        qglIndexPointer = dllIndexPointer;
        qglIndexd = dllIndexd;
        qglIndexdv = dllIndexdv;
        qglIndexf = dllIndexf;
        qglIndexfv = dllIndexfv;
        qglIndexi = dllIndexi;
        qglIndexiv = dllIndexiv;
        qglIndexs = dllIndexs;
        qglIndexsv = dllIndexsv;
        qglIndexub = dllIndexub;
        qglIndexubv = dllIndexubv;
        qglInitNames = dllInitNames;
        qglInterleavedArrays = dllInterleavedArrays;
        qglLightModelf = dllLightModelf;
        qglLightModelfv = dllLightModelfv;
        qglLightModeli = dllLightModeli;
        qglLightModeliv = dllLightModeliv;
        qglLightf = dllLightf;
        qglLightfv = dllLightfv;
        qglLighti = dllLighti;
        qglLightiv = dllLightiv;
        qglLineStipple = dllLineStipple;
        qglLineWidth = dllLineWidth;
        qglListBase = dllListBase;
        qglLoadIdentity = dllLoadIdentity;
        qglLoadMatrixd = dllLoadMatrixd;
        qglLoadMatrixf = dllLoadMatrixf;
        qglLoadName = dllLoadName;
        qglLogicOp = dllLogicOp;
        qglMap1d = dllMap1d;
        qglMap1f = dllMap1f;
        qglMap2d = dllMap2d;
        qglMap2f = dllMap2f;
        qglMapGrid1d = dllMapGrid1d;
        qglMapGrid1f = dllMapGrid1f;
        qglMapGrid2d = dllMapGrid2d;
        qglMapGrid2f = dllMapGrid2f;
        qglMaterialf = dllMaterialf;
        qglMaterialfv = dllMaterialfv;
        qglMateriali = dllMateriali;
        qglMaterialiv = dllMaterialiv;
        qglMatrixMode = dllMatrixMode;
        qglMultMatrixd = dllMultMatrixd;
        qglMultMatrixf = dllMultMatrixf;
        qglNewList = dllNewList;
        qglNormal3b = dllNormal3b;
        qglNormal3bv = dllNormal3bv;
        qglNormal3d = dllNormal3d;
        qglNormal3dv = dllNormal3dv;
        qglNormal3f = dllNormal3f;
        qglNormal3fv = dllNormal3fv;
        qglNormal3i = dllNormal3i;
        qglNormal3iv = dllNormal3iv;
        qglNormal3s = dllNormal3s;
        qglNormal3sv = dllNormal3sv;
        qglNormalPointer = dllNormalPointer;
        qglOrtho = dllOrtho;
        qglPassThrough = dllPassThrough;
        qglPixelMapfv = dllPixelMapfv;
        qglPixelMapuiv = dllPixelMapuiv;
        qglPixelMapusv = dllPixelMapusv;
        qglPixelStoref = dllPixelStoref;
        qglPixelStorei = dllPixelStorei;
        qglPixelTransferf = dllPixelTransferf;
        qglPixelTransferi = dllPixelTransferi;
        qglPixelZoom = dllPixelZoom;
        qglPointSize = dllPointSize;
        qglPolygonMode = dllPolygonMode;
        qglPolygonOffset = dllPolygonOffset;
        qglPolygonStipple = dllPolygonStipple;
        qglPopAttrib = dllPopAttrib;
        qglPopClientAttrib = dllPopClientAttrib;
        qglPopMatrix = dllPopMatrix;
        qglPopName = dllPopName;
        qglPrioritizeTextures = dllPrioritizeTextures;
        qglPushAttrib = dllPushAttrib;
        qglPushClientAttrib = dllPushClientAttrib;
        qglPushMatrix = dllPushMatrix;
        qglPushName = dllPushName;
        qglRasterPos2d = dllRasterPos2d;
        qglRasterPos2dv = dllRasterPos2dv;
        qglRasterPos2f = dllRasterPos2f;
        qglRasterPos2fv = dllRasterPos2fv;
        qglRasterPos2i = dllRasterPos2i;
        qglRasterPos2iv = dllRasterPos2iv;
        qglRasterPos2s = dllRasterPos2s;
        qglRasterPos2sv = dllRasterPos2sv;
        qglRasterPos3d = dllRasterPos3d;
        qglRasterPos3dv = dllRasterPos3dv;
        qglRasterPos3f = dllRasterPos3f;
        qglRasterPos3fv = dllRasterPos3fv;
        qglRasterPos3i = dllRasterPos3i;
        qglRasterPos3iv = dllRasterPos3iv;
        qglRasterPos3s = dllRasterPos3s;
        qglRasterPos3sv = dllRasterPos3sv;
        qglRasterPos4d = dllRasterPos4d;
        qglRasterPos4dv = dllRasterPos4dv;
        qglRasterPos4f = dllRasterPos4f;
        qglRasterPos4fv = dllRasterPos4fv;
        qglRasterPos4i = dllRasterPos4i;
        qglRasterPos4iv = dllRasterPos4iv;
        qglRasterPos4s = dllRasterPos4s;
        qglRasterPos4sv = dllRasterPos4sv;
        qglReadBuffer = dllReadBuffer;
        qglReadPixels = dllReadPixels;
        qglRectd = dllRectd;
        qglRectdv = dllRectdv;
        qglRectf = dllRectf;
        qglRectfv = dllRectfv;
        qglRecti = dllRecti;
        qglRectiv = dllRectiv;
        qglRects = dllRects;
        qglRectsv = dllRectsv;
        qglRotated = dllRotated;
        qglRotatef = dllRotatef;
        qglScaled = dllScaled;
        qglScalef = dllScalef;
        qglScissor = dllScissor;
        qglSelectBuffer = dllSelectBuffer;
        qglShadeModel = dllShadeModel;
        qglStencilFunc = dllStencilFunc;
        qglStencilMask = dllStencilMask;
        qglStencilOp = dllStencilOp;
        qglTexCoord1d = dllTexCoord1d;
        qglTexCoord1dv = dllTexCoord1dv;
        qglTexCoord1f = dllTexCoord1f;
        qglTexCoord1fv = dllTexCoord1fv;
        qglTexCoord1i = dllTexCoord1i;
        qglTexCoord1iv = dllTexCoord1iv;
        qglTexCoord1s = dllTexCoord1s;
        qglTexCoord1sv = dllTexCoord1sv;
        qglTexCoord2d = dllTexCoord2d;
        qglTexCoord2dv = dllTexCoord2dv;
        qglTexCoord2f = dllTexCoord2f;
        qglTexCoord2fv = dllTexCoord2fv;
        qglTexCoord2i = dllTexCoord2i;
        qglTexCoord2iv = dllTexCoord2iv;
        qglTexCoord2s = dllTexCoord2s;
        qglTexCoord2sv = dllTexCoord2sv;
        qglTexCoord3d = dllTexCoord3d;
        qglTexCoord3dv = dllTexCoord3dv;
        qglTexCoord3f = dllTexCoord3f;
        qglTexCoord3fv = dllTexCoord3fv;
        qglTexCoord3i = dllTexCoord3i;
        qglTexCoord3iv = dllTexCoord3iv;
        qglTexCoord3s = dllTexCoord3s;
        qglTexCoord3sv = dllTexCoord3sv;
        qglTexCoord4d = dllTexCoord4d;
        qglTexCoord4dv = dllTexCoord4dv;
        qglTexCoord4f = dllTexCoord4f;
        qglTexCoord4fv = dllTexCoord4fv;
        qglTexCoord4i = dllTexCoord4i;
        qglTexCoord4iv = dllTexCoord4iv;
        qglTexCoord4s = dllTexCoord4s;
        qglTexCoord4sv = dllTexCoord4sv;
        qglTexCoordPointer = dllTexCoordPointer;
        qglTexEnvf = dllTexEnvf;
        qglTexEnvfv = dllTexEnvfv;
        qglTexEnvi = dllTexEnvi;
        qglTexEnviv = dllTexEnviv;
        qglTexGend = dllTexGend;
        qglTexGendv = dllTexGendv;
        qglTexGenf = dllTexGenf;
        qglTexGenfv = dllTexGenfv;
        qglTexGeni = dllTexGeni;
        qglTexGeniv = dllTexGeniv;
        qglTexImage1D = dllTexImage1D;
        qglTexImage2D = dllTexImage2D;
        qglTexParameterf = dllTexParameterf;
        qglTexParameterfv = dllTexParameterfv;
        qglTexParameteri = dllTexParameteri;
        qglTexParameteriv = dllTexParameteriv;
        qglTexSubImage1D = dllTexSubImage1D;
        qglTexSubImage2D = dllTexSubImage2D;
        qglTranslated = dllTranslated;
        qglTranslatef = dllTranslatef;
        qglVertex2d = dllVertex2d;
        qglVertex2dv = dllVertex2dv;
        qglVertex2f = dllVertex2f;
        qglVertex2fv = dllVertex2fv;
        qglVertex2i = dllVertex2i;
        qglVertex2iv = dllVertex2iv;
        qglVertex2s = dllVertex2s;
        qglVertex2sv = dllVertex2sv;
        qglVertex3d = dllVertex3d;
        qglVertex3dv = dllVertex3dv;
        qglVertex3f = dllVertex3f;
        qglVertex3fv = dllVertex3fv;
        qglVertex3i = dllVertex3i;
        qglVertex3iv = dllVertex3iv;
        qglVertex3s = dllVertex3s;
        qglVertex3sv = dllVertex3sv;
        qglVertex4d = dllVertex4d;
        qglVertex4dv = dllVertex4dv;
        qglVertex4f = dllVertex4f;
        qglVertex4fv = dllVertex4fv;
        qglVertex4i = dllVertex4i;
        qglVertex4iv = dllVertex4iv;
        qglVertex4s = dllVertex4s;
        qglVertex4sv = dllVertex4sv;
        qglVertexPointer = dllVertexPointer;
        qglViewport = dllViewport;
    };
}
#[no_mangle]
pub unsafe extern "C" fn GLimp_LogNewFrame() {
    fprintf(
        glw_state.log_fp,
        b"*** R_BeginFrame ***\n\0" as *const u8 as *const libc::c_char,
    );
}
