#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(extern_types, register_tool)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    static mut ri: refimport_t;
    static mut gl_state: glstate_t;
    static mut gl_driver: *mut cvar_t;
    static mut gl_drawbuffer: *mut cvar_t;
    static mut gl_bitdepth: *mut cvar_t;
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
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
pub type rserr_t = libc::c_uint;
pub const rserr_unknown: rserr_t = 3;
pub const rserr_invalid_mode: rserr_t = 2;
pub const rserr_invalid_fullscreen: rserr_t = 1;
pub const rserr_ok: rserr_t = 0;
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
pub static mut glw_state: glwstate_t = glwstate_t {
    hInstance: 0,
    wndproc: 0 as *const libc::c_void as *mut libc::c_void,
    hDC: 0,
    hWnd: 0,
    hGLRC: 0,
    hinstOpenGL: 0,
    minidriver: false_0,
    allowdisplaydepthchange: false_0,
    mcd_accelerated: false_0,
    log_fp: 0 as *const FILE as *mut FILE,
};
unsafe extern "C" fn VerifyDriver() -> qboolean {
    let mut buffer: [libc::c_char; 1024] = [0; 1024];
    strlwr(buffer.as_mut_ptr());
    if strcmp(buffer.as_mut_ptr(), b"gdi generic\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        if glw_state.mcd_accelerated as u64 == 0 {
            return false_0;
        }
    }
    return true_0;
}
#[no_mangle]
pub unsafe extern "C" fn GLimp_BeginFrame(mut camera_separation: libc::c_float) {
    if (*gl_bitdepth).modified as u64 != 0 {
        if (*gl_bitdepth).value != 0 as libc::c_int as libc::c_float
            && glw_state.allowdisplaydepthchange as u64 == 0
        {
            (ri.Cvar_SetValue)
                .expect(
                    "non-null function pointer",
                )(
                b"gl_bitdepth\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                0 as libc::c_int as libc::c_float,
            );
            (ri.Con_Printf)
                .expect(
                    "non-null function pointer",
                )(
                0 as libc::c_int,
                b"gl_bitdepth requires Win95 OSR2.x or WinNT 4.x\n\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
            );
        }
        (*gl_bitdepth).modified = false_0;
    }
    if !(camera_separation < 0 as libc::c_int as libc::c_float
        && gl_state.stereo_enabled as libc::c_uint != 0)
    {
        camera_separation > 0 as libc::c_int as libc::c_float
            && gl_state.stereo_enabled as libc::c_uint != 0;
    }
}
#[no_mangle]
pub unsafe extern "C" fn GLimp_AppActivate(mut active: qboolean) {
    active as u64 != 0;
}
