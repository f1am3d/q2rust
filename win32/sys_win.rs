#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(c_variadic, extern_types, register_tool)]


use winapi;

pub const MAX_NUM_ARGVS: i8 = 128;

extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;


    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn vsprintf(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: ::std::ffi::VaList,
    ) -> libc::c_int;
    static mut dedicated: *mut cvar_t;
    fn Com_Error(code: libc::c_int, fmt: *mut libc::c_char, _: ...);
    fn Com_DPrintf(fmt: *mut libc::c_char, _: ...);
    fn FS_NextPath(prevpath: *mut libc::c_char) -> *mut libc::c_char;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn exit(_: libc::c_int) -> !;
    fn Qcommon_Shutdown();
    fn CL_Shutdown();
    fn InitConProc(argc_0: libc::c_int, argv_0: *mut *mut libc::c_char);
    fn DeinitConProc();
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

pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type size_t = libc::c_ulong;
pub type va_list = __builtin_va_list;

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

#[no_mangle]
pub static mut s_win95: qboolean = false_0;
#[no_mangle]
pub static mut starttime: libc::c_int = 0;
#[no_mangle]
pub static mut ActiveApp: libc::c_int = 0;
#[no_mangle]
pub static mut Minimized: qboolean = false_0;
#[no_mangle]
pub static mut sys_msg_time: libc::c_uint = 0;
#[no_mangle]
pub static mut sys_frame_time: libc::c_uint = 0;
#[no_mangle]
pub static mut argc: libc::c_int = 0;
#[no_mangle]
pub static mut argv: [*mut libc::c_char; 128] = [0 as *const libc::c_char
    as *mut libc::c_char; 128];

#[no_mangle]
pub unsafe extern "C" fn Sys_Error(mut error: *mut libc::c_char, mut args: ...) {
    let mut argptr: ::std::ffi::VaListImpl;
    let mut text: [libc::c_char; 1024] = [0; 1024];
    CL_Shutdown();
    Qcommon_Shutdown();

    argptr = args.clone();

    vsprintf(text.as_mut_ptr(), error, argptr.as_va_list());


    winapi::um::winuser::MessageBox(
        0 as *mut libc::c_void,
        text.as_mut_ptr(),
        b"Error\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
    );
    DeinitConProc();
    exit(1 as libc::c_int);
}

#[no_mangle]
pub unsafe extern "C" fn Sys_Quit() {
    winapi::um::timeapi::timeEndPeriod(1 as libc::c_int);
    CL_Shutdown();
    Qcommon_Shutdown();

    if !dedicated.is_null() && (*dedicated).value != 0. {
        winapi::um::wincon::FreeConsole();
    }

    DeinitConProc();
    exit(0 as libc::c_int);
}

#[no_mangle]
pub unsafe extern "C" fn WinError() {}

#[no_mangle]
pub unsafe extern "C" fn Sys_ScanForCD() -> *mut libc::c_char {}

#[no_mangle]
pub unsafe extern "C" fn Sys_CopyProtect() {
    let mut cddir: *mut libc::c_char = 0 as *mut libc::c_char;
    cddir = Sys_ScanForCD();
    if *cddir.offset(0 as libc::c_int as isize) == 0 {
        Com_Error(
            0 as libc::c_int,
            b"You must have the Quake2 CD in the drive to play.\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        );
    }
}

static mut console_text: [libc::c_char; 256] = [0; 256];
static mut console_textlen: libc::c_int = 0;

#[no_mangle]
pub unsafe extern "C" fn Sys_ConsoleOutput(mut string: *mut libc::c_char) {
    let mut dummy: libc::c_int = 0;
    let mut text: [libc::c_char; 256] = [0; 256];
    if dedicated.is_null() || (*dedicated).value == 0. {
        return;
    }
    if console_textlen != 0 {
        text[0 as libc::c_int as usize] = '\r' as i32 as libc::c_char;
        memset(
            &mut *text.as_mut_ptr().offset(1 as libc::c_int as isize)
                as *mut libc::c_char as *mut libc::c_void,
            ' ' as i32,
            console_textlen as libc::c_ulong,
        );
        text[(console_textlen + 1 as libc::c_int)
            as usize] = '\r' as i32 as libc::c_char;
        text[(console_textlen + 2 as libc::c_int)
            as usize] = 0 as libc::c_int as libc::c_char;
    }
}

#[no_mangle]
pub unsafe extern "C" fn Sys_SendKeyEvents() {
    sys_frame_time = winapi::um::timeapi::timeGetTime() as libc::c_uint;
}

#[no_mangle]
pub unsafe extern "C" fn Sys_AppActivate() {}

#[no_mangle]
pub unsafe extern "C" fn ParseCommandLine(mut lpCmdLine: libc::c_int) {
    argc = 1 as libc::c_int;
    argv[0 as libc::c_int
        as usize] = b"exe\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
}

#[no_mangle]
pub static mut global_hInstance: libc::c_int = 0;
#[no_mangle]
pub static mut WINAPI: libc::c_int = 0;
