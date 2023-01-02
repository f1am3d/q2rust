#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(c_variadic, extern_types, register_tool)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
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
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn vsprintf(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: ::std::ffi::VaList,
    ) -> libc::c_int;
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn rand() -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn Com_sprintf(
        dest: *mut libc::c_char,
        size: libc::c_int,
        fmt: *mut libc::c_char,
        _: ...
    );
    fn LittleLong(l: libc::c_int) -> libc::c_int;
    fn Swap_Init();
    fn va(format: *mut libc::c_char, _: ...) -> *mut libc::c_char;
    fn Sys_Milliseconds() -> libc::c_int;
    fn Sys_Error(error: *mut libc::c_char, _: ...);
    fn FS_Gamedir() -> *mut libc::c_char;
    fn Sys_ConsoleOutput(string: *mut libc::c_char);
    fn Con_Print(text: *mut libc::c_char);
    fn CL_Shutdown();
    fn SV_Shutdown(finalmsg: *mut libc::c_char, reconnect: qboolean);
    fn CL_Drop();
    fn Sys_ConsoleInput() -> *mut libc::c_char;
    fn SV_Frame(msec: libc::c_int);
    fn CL_Frame(msec: libc::c_int);
    fn Sys_Init();
    fn SV_Init();
    fn CL_Init();
    fn Sys_Quit();
    fn FS_InitFilesystem();
    fn CRC_Block(start: *mut byte, count: libc::c_int) -> libc::c_ushort;
    fn Cbuf_Init();
    fn Cbuf_AddText(text: *mut libc::c_char);
    fn Cbuf_AddEarlyCommands(clear: qboolean);
    fn Cbuf_AddLateCommands() -> qboolean;
    fn Cbuf_Execute();
    fn Cmd_Init();
    fn Cmd_AddCommand(cmd_name: *mut libc::c_char, function: xcommand_t);
    fn Cmd_Argv(arg: libc::c_int) -> *mut libc::c_char;
    fn Cvar_Get(
        var_name: *mut libc::c_char,
        value: *mut libc::c_char,
        flags: libc::c_int,
    ) -> *mut cvar_t;
    fn Cvar_Init();
    fn NET_Init();
    fn Netchan_Init();
    fn longjmp(_: *mut __jmp_buf_tag, _: libc::c_int) -> !;
    fn _setjmp(_: *mut __jmp_buf_tag) -> libc::c_int;
    fn Key_Init();
    fn SCR_EndLoadingPlaque();
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
pub type byte = libc::c_uchar;
pub type qboolean = libc::c_uint;
pub const true_0: qboolean = 1;
pub const false_0: qboolean = 0;
pub type vec_t = libc::c_float;
pub type vec3_t = [vec_t; 3];
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct usercmd_s {
    pub msec: byte,
    pub buttons: byte,
    pub angles: [libc::c_short; 3],
    pub forwardmove: libc::c_short,
    pub sidemove: libc::c_short,
    pub upmove: libc::c_short,
    pub impulse: byte,
    pub lightlevel: byte,
}
pub type usercmd_t = usercmd_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct entity_state_s {
    pub number: libc::c_int,
    pub origin: vec3_t,
    pub angles: vec3_t,
    pub old_origin: vec3_t,
    pub modelindex: libc::c_int,
    pub modelindex2: libc::c_int,
    pub modelindex3: libc::c_int,
    pub modelindex4: libc::c_int,
    pub frame: libc::c_int,
    pub skinnum: libc::c_int,
    pub effects: libc::c_uint,
    pub renderfx: libc::c_int,
    pub solid: libc::c_int,
    pub sound: libc::c_int,
    pub event: libc::c_int,
}
pub type entity_state_t = entity_state_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sizebuf_s {
    pub allowoverflow: qboolean,
    pub overflowed: qboolean,
    pub data: *mut byte,
    pub maxsize: libc::c_int,
    pub cursize: libc::c_int,
    pub readcount: libc::c_int,
}
pub type sizebuf_t = sizebuf_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __jmp_buf_tag {
    pub __jmpbuf: __jmp_buf,
    pub __mask_was_saved: libc::c_int,
    pub __saved_mask: __sigset_t,
}
pub type __jmp_buf = [libc::c_long; 8];
pub type jmp_buf = [__jmp_buf_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub f: libc::c_float,
    pub l: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub b: [byte; 4],
    pub f: libc::c_float,
    pub l: libc::c_int,
}
pub type zhead_t = zhead_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct zhead_s {
    pub prev: *mut zhead_s,
    pub next: *mut zhead_s,
    pub magic: libc::c_short,
    pub tag: libc::c_short,
    pub size: libc::c_int,
}
pub type xcommand_t = Option::<unsafe extern "C" fn() -> ()>;
#[no_mangle]
pub static mut com_argc: libc::c_int = 0;
#[no_mangle]
pub static mut com_argv: [*mut libc::c_char; 51] = [0 as *const libc::c_char
    as *mut libc::c_char; 51];
#[no_mangle]
pub static mut realtime: libc::c_int = 0;
#[no_mangle]
pub static mut abortframe: jmp_buf = [__jmp_buf_tag {
    __jmpbuf: [0; 8],
    __mask_was_saved: 0,
    __saved_mask: __sigset_t { __val: [0; 16] },
}; 1];
#[no_mangle]
pub static mut log_stats_file: *mut FILE = 0 as *const FILE as *mut FILE;
#[no_mangle]
pub static mut host_speeds: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut log_stats: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut developer: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut timescale: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut fixedtime: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut logfile_active: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut showtrace: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut dedicated: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut logfile: *mut FILE = 0 as *const FILE as *mut FILE;
#[no_mangle]
pub static mut server_state: libc::c_int = 0;
#[no_mangle]
pub static mut time_before_game: libc::c_int = 0;
#[no_mangle]
pub static mut time_after_game: libc::c_int = 0;
#[no_mangle]
pub static mut time_before_ref: libc::c_int = 0;
#[no_mangle]
pub static mut time_after_ref: libc::c_int = 0;
static mut rd_target: libc::c_int = 0;
static mut rd_buffer: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut rd_buffersize: libc::c_int = 0;
static mut rd_flush: Option::<
    unsafe extern "C" fn(libc::c_int, *mut libc::c_char) -> (),
> = None;
#[no_mangle]
pub unsafe extern "C" fn Com_BeginRedirect(
    mut target: libc::c_int,
    mut buffer: *mut libc::c_char,
    mut buffersize: libc::c_int,
    mut flush: *mut libc::c_void,
) {
    if target == 0 || buffer.is_null() || buffersize == 0 || flush.is_null() {
        return;
    }
    rd_target = target;
    rd_buffer = buffer;
    rd_buffersize = buffersize;
    rd_flush = ::std::mem::transmute::<
        *mut libc::c_void,
        Option::<unsafe extern "C" fn(libc::c_int, *mut libc::c_char) -> ()>,
    >(flush);
    *rd_buffer = 0 as libc::c_int as libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn Com_EndRedirect() {
    rd_flush.expect("non-null function pointer")(rd_target, rd_buffer);
    rd_target = 0 as libc::c_int;
    rd_buffer = 0 as *mut libc::c_char;
    rd_buffersize = 0 as libc::c_int;
    rd_flush = None;
}
#[no_mangle]
pub unsafe extern "C" fn Com_Printf(mut fmt: *mut libc::c_char, mut args: ...) {
    let mut argptr: ::std::ffi::VaListImpl;
    let mut msg: [libc::c_char; 4096] = [0; 4096];
    argptr = args.clone();
    vsprintf(msg.as_mut_ptr(), fmt, argptr.as_va_list());
    if rd_target != 0 {
        if (strlen(msg.as_mut_ptr())).wrapping_add(strlen(rd_buffer))
            > (rd_buffersize - 1 as libc::c_int) as libc::c_ulong
        {
            rd_flush.expect("non-null function pointer")(rd_target, rd_buffer);
            *rd_buffer = 0 as libc::c_int as libc::c_char;
        }
        strcat(rd_buffer, msg.as_mut_ptr());
        return;
    }
    Con_Print(msg.as_mut_ptr());
    Sys_ConsoleOutput(msg.as_mut_ptr());
    if !logfile_active.is_null() && (*logfile_active).value != 0. {
        let mut name: [libc::c_char; 64] = [0; 64];
        if logfile.is_null() {
            Com_sprintf(
                name.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong
                    as libc::c_int,
                b"%s/qconsole.log\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                FS_Gamedir(),
            );
            logfile = fopen(
                name.as_mut_ptr(),
                b"w\0" as *const u8 as *const libc::c_char,
            );
        }
        if !logfile.is_null() {
            fprintf(
                logfile,
                b"%s\0" as *const u8 as *const libc::c_char,
                msg.as_mut_ptr(),
            );
        }
        if (*logfile_active).value > 1 as libc::c_int as libc::c_float {
            fflush(logfile);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn Com_DPrintf(mut fmt: *mut libc::c_char, mut args: ...) {
    let mut argptr: ::std::ffi::VaListImpl;
    let mut msg: [libc::c_char; 4096] = [0; 4096];
    if developer.is_null() || (*developer).value == 0. {
        return;
    }
    argptr = args.clone();
    vsprintf(msg.as_mut_ptr(), fmt, argptr.as_va_list());
    Com_Printf(
        b"%s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        msg.as_mut_ptr(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn Com_Error(
    mut code: libc::c_int,
    mut fmt: *mut libc::c_char,
    mut args: ...
) {
    let mut argptr: ::std::ffi::VaListImpl;
    static mut msg: [libc::c_char; 4096] = [0; 4096];
    static mut recursive: qboolean = false_0;
    if recursive as u64 != 0 {
        Sys_Error(
            b"recursive error after: %s\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            msg.as_mut_ptr(),
        );
    }
    recursive = true_0;
    argptr = args.clone();
    vsprintf(msg.as_mut_ptr(), fmt, argptr.as_va_list());
    if code == 2 as libc::c_int {
        CL_Drop();
        recursive = false_0;
        longjmp(abortframe.as_mut_ptr(), -(1 as libc::c_int));
    } else {
        if code == 1 as libc::c_int {
            Com_Printf(
                b"********************\nERROR: %s\n********************\n\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                msg.as_mut_ptr(),
            );
            SV_Shutdown(
                va(
                    b"Server crashed: %s\n\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    msg.as_mut_ptr(),
                ),
                false_0,
            );
            CL_Drop();
            recursive = false_0;
            longjmp(abortframe.as_mut_ptr(), -(1 as libc::c_int));
        } else {
            SV_Shutdown(
                va(
                    b"Server fatal crashed: %s\n\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    msg.as_mut_ptr(),
                ),
                false_0,
            );
            CL_Shutdown();
        }
    }
    if !logfile.is_null() {
        fclose(logfile);
        logfile = 0 as *mut FILE;
    }
    Sys_Error(
        b"%s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        msg.as_mut_ptr(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn Com_Quit() {
    SV_Shutdown(
        b"Server quit\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        false_0,
    );
    CL_Shutdown();
    if !logfile.is_null() {
        fclose(logfile);
        logfile = 0 as *mut FILE;
    }
    Sys_Quit();
}
#[no_mangle]
pub unsafe extern "C" fn Com_ServerState() -> libc::c_int {
    return server_state;
}
#[no_mangle]
pub unsafe extern "C" fn Com_SetServerState(mut state: libc::c_int) {
    server_state = state;
}
#[no_mangle]
pub static mut bytedirs: [vec3_t; 162] = [
    [-0.525731f64 as vec_t, 0.000000f64 as vec_t, 0.850651f64 as vec_t],
    [-0.442863f64 as vec_t, 0.238856f64 as vec_t, 0.864188f64 as vec_t],
    [-0.295242f64 as vec_t, 0.000000f64 as vec_t, 0.955423f64 as vec_t],
    [-0.309017f64 as vec_t, 0.500000f64 as vec_t, 0.809017f64 as vec_t],
    [-0.162460f64 as vec_t, 0.262866f64 as vec_t, 0.951056f64 as vec_t],
    [0.000000f64 as vec_t, 0.000000f64 as vec_t, 1.000000f64 as vec_t],
    [0.000000f64 as vec_t, 0.850651f64 as vec_t, 0.525731f64 as vec_t],
    [-0.147621f64 as vec_t, 0.716567f64 as vec_t, 0.681718f64 as vec_t],
    [0.147621f64 as vec_t, 0.716567f64 as vec_t, 0.681718f64 as vec_t],
    [0.000000f64 as vec_t, 0.525731f64 as vec_t, 0.850651f64 as vec_t],
    [0.309017f64 as vec_t, 0.500000f64 as vec_t, 0.809017f64 as vec_t],
    [0.525731f64 as vec_t, 0.000000f64 as vec_t, 0.850651f64 as vec_t],
    [0.295242f64 as vec_t, 0.000000f64 as vec_t, 0.955423f64 as vec_t],
    [0.442863f64 as vec_t, 0.238856f64 as vec_t, 0.864188f64 as vec_t],
    [0.162460f64 as vec_t, 0.262866f64 as vec_t, 0.951056f64 as vec_t],
    [-0.681718f64 as vec_t, 0.147621f64 as vec_t, 0.716567f64 as vec_t],
    [-0.809017f64 as vec_t, 0.309017f64 as vec_t, 0.500000f64 as vec_t],
    [-0.587785f64 as vec_t, 0.425325f64 as vec_t, 0.688191f64 as vec_t],
    [-0.850651f64 as vec_t, 0.525731f64 as vec_t, 0.000000f64 as vec_t],
    [-0.864188f64 as vec_t, 0.442863f64 as vec_t, 0.238856f64 as vec_t],
    [-0.716567f64 as vec_t, 0.681718f64 as vec_t, 0.147621f64 as vec_t],
    [-0.688191f64 as vec_t, 0.587785f64 as vec_t, 0.425325f64 as vec_t],
    [-0.500000f64 as vec_t, 0.809017f64 as vec_t, 0.309017f64 as vec_t],
    [-0.238856f64 as vec_t, 0.864188f64 as vec_t, 0.442863f64 as vec_t],
    [-0.425325f64 as vec_t, 0.688191f64 as vec_t, 0.587785f64 as vec_t],
    [-0.716567f64 as vec_t, 0.681718f64 as vec_t, -0.147621f64 as vec_t],
    [-0.500000f64 as vec_t, 0.809017f64 as vec_t, -0.309017f64 as vec_t],
    [-0.525731f64 as vec_t, 0.850651f64 as vec_t, 0.000000f64 as vec_t],
    [0.000000f64 as vec_t, 0.850651f64 as vec_t, -0.525731f64 as vec_t],
    [-0.238856f64 as vec_t, 0.864188f64 as vec_t, -0.442863f64 as vec_t],
    [0.000000f64 as vec_t, 0.955423f64 as vec_t, -0.295242f64 as vec_t],
    [-0.262866f64 as vec_t, 0.951056f64 as vec_t, -0.162460f64 as vec_t],
    [0.000000f64 as vec_t, 1.000000f64 as vec_t, 0.000000f64 as vec_t],
    [0.000000f64 as vec_t, 0.955423f64 as vec_t, 0.295242f64 as vec_t],
    [-0.262866f64 as vec_t, 0.951056f64 as vec_t, 0.162460f64 as vec_t],
    [0.238856f64 as vec_t, 0.864188f64 as vec_t, 0.442863f64 as vec_t],
    [0.262866f64 as vec_t, 0.951056f64 as vec_t, 0.162460f64 as vec_t],
    [0.500000f64 as vec_t, 0.809017f64 as vec_t, 0.309017f64 as vec_t],
    [0.238856f64 as vec_t, 0.864188f64 as vec_t, -0.442863f64 as vec_t],
    [0.262866f64 as vec_t, 0.951056f64 as vec_t, -0.162460f64 as vec_t],
    [0.500000f64 as vec_t, 0.809017f64 as vec_t, -0.309017f64 as vec_t],
    [0.850651f64 as vec_t, 0.525731f64 as vec_t, 0.000000f64 as vec_t],
    [0.716567f64 as vec_t, 0.681718f64 as vec_t, 0.147621f64 as vec_t],
    [0.716567f64 as vec_t, 0.681718f64 as vec_t, -0.147621f64 as vec_t],
    [0.525731f64 as vec_t, 0.850651f64 as vec_t, 0.000000f64 as vec_t],
    [0.425325f64 as vec_t, 0.688191f64 as vec_t, 0.587785f64 as vec_t],
    [0.864188f64 as vec_t, 0.442863f64 as vec_t, 0.238856f64 as vec_t],
    [0.688191f64 as vec_t, 0.587785f64 as vec_t, 0.425325f64 as vec_t],
    [0.809017f64 as vec_t, 0.309017f64 as vec_t, 0.500000f64 as vec_t],
    [0.681718f64 as vec_t, 0.147621f64 as vec_t, 0.716567f64 as vec_t],
    [0.587785f64 as vec_t, 0.425325f64 as vec_t, 0.688191f64 as vec_t],
    [0.955423f64 as vec_t, 0.295242f64 as vec_t, 0.000000f64 as vec_t],
    [1.000000f64 as vec_t, 0.000000f64 as vec_t, 0.000000f64 as vec_t],
    [0.951056f64 as vec_t, 0.162460f64 as vec_t, 0.262866f64 as vec_t],
    [0.850651f64 as vec_t, -0.525731f64 as vec_t, 0.000000f64 as vec_t],
    [0.955423f64 as vec_t, -0.295242f64 as vec_t, 0.000000f64 as vec_t],
    [0.864188f64 as vec_t, -0.442863f64 as vec_t, 0.238856f64 as vec_t],
    [0.951056f64 as vec_t, -0.162460f64 as vec_t, 0.262866f64 as vec_t],
    [0.809017f64 as vec_t, -0.309017f64 as vec_t, 0.500000f64 as vec_t],
    [0.681718f64 as vec_t, -0.147621f64 as vec_t, 0.716567f64 as vec_t],
    [0.850651f64 as vec_t, 0.000000f64 as vec_t, 0.525731f64 as vec_t],
    [0.864188f64 as vec_t, 0.442863f64 as vec_t, -0.238856f64 as vec_t],
    [0.809017f64 as vec_t, 0.309017f64 as vec_t, -0.500000f64 as vec_t],
    [0.951056f64 as vec_t, 0.162460f64 as vec_t, -0.262866f64 as vec_t],
    [0.525731f64 as vec_t, 0.000000f64 as vec_t, -0.850651f64 as vec_t],
    [0.681718f64 as vec_t, 0.147621f64 as vec_t, -0.716567f64 as vec_t],
    [0.681718f64 as vec_t, -0.147621f64 as vec_t, -0.716567f64 as vec_t],
    [0.850651f64 as vec_t, 0.000000f64 as vec_t, -0.525731f64 as vec_t],
    [0.809017f64 as vec_t, -0.309017f64 as vec_t, -0.500000f64 as vec_t],
    [0.864188f64 as vec_t, -0.442863f64 as vec_t, -0.238856f64 as vec_t],
    [0.951056f64 as vec_t, -0.162460f64 as vec_t, -0.262866f64 as vec_t],
    [0.147621f64 as vec_t, 0.716567f64 as vec_t, -0.681718f64 as vec_t],
    [0.309017f64 as vec_t, 0.500000f64 as vec_t, -0.809017f64 as vec_t],
    [0.425325f64 as vec_t, 0.688191f64 as vec_t, -0.587785f64 as vec_t],
    [0.442863f64 as vec_t, 0.238856f64 as vec_t, -0.864188f64 as vec_t],
    [0.587785f64 as vec_t, 0.425325f64 as vec_t, -0.688191f64 as vec_t],
    [0.688191f64 as vec_t, 0.587785f64 as vec_t, -0.425325f64 as vec_t],
    [-0.147621f64 as vec_t, 0.716567f64 as vec_t, -0.681718f64 as vec_t],
    [-0.309017f64 as vec_t, 0.500000f64 as vec_t, -0.809017f64 as vec_t],
    [0.000000f64 as vec_t, 0.525731f64 as vec_t, -0.850651f64 as vec_t],
    [-0.525731f64 as vec_t, 0.000000f64 as vec_t, -0.850651f64 as vec_t],
    [-0.442863f64 as vec_t, 0.238856f64 as vec_t, -0.864188f64 as vec_t],
    [-0.295242f64 as vec_t, 0.000000f64 as vec_t, -0.955423f64 as vec_t],
    [-0.162460f64 as vec_t, 0.262866f64 as vec_t, -0.951056f64 as vec_t],
    [0.000000f64 as vec_t, 0.000000f64 as vec_t, -1.000000f64 as vec_t],
    [0.295242f64 as vec_t, 0.000000f64 as vec_t, -0.955423f64 as vec_t],
    [0.162460f64 as vec_t, 0.262866f64 as vec_t, -0.951056f64 as vec_t],
    [-0.442863f64 as vec_t, -0.238856f64 as vec_t, -0.864188f64 as vec_t],
    [-0.309017f64 as vec_t, -0.500000f64 as vec_t, -0.809017f64 as vec_t],
    [-0.162460f64 as vec_t, -0.262866f64 as vec_t, -0.951056f64 as vec_t],
    [0.000000f64 as vec_t, -0.850651f64 as vec_t, -0.525731f64 as vec_t],
    [-0.147621f64 as vec_t, -0.716567f64 as vec_t, -0.681718f64 as vec_t],
    [0.147621f64 as vec_t, -0.716567f64 as vec_t, -0.681718f64 as vec_t],
    [0.000000f64 as vec_t, -0.525731f64 as vec_t, -0.850651f64 as vec_t],
    [0.309017f64 as vec_t, -0.500000f64 as vec_t, -0.809017f64 as vec_t],
    [0.442863f64 as vec_t, -0.238856f64 as vec_t, -0.864188f64 as vec_t],
    [0.162460f64 as vec_t, -0.262866f64 as vec_t, -0.951056f64 as vec_t],
    [0.238856f64 as vec_t, -0.864188f64 as vec_t, -0.442863f64 as vec_t],
    [0.500000f64 as vec_t, -0.809017f64 as vec_t, -0.309017f64 as vec_t],
    [0.425325f64 as vec_t, -0.688191f64 as vec_t, -0.587785f64 as vec_t],
    [0.716567f64 as vec_t, -0.681718f64 as vec_t, -0.147621f64 as vec_t],
    [0.688191f64 as vec_t, -0.587785f64 as vec_t, -0.425325f64 as vec_t],
    [0.587785f64 as vec_t, -0.425325f64 as vec_t, -0.688191f64 as vec_t],
    [0.000000f64 as vec_t, -0.955423f64 as vec_t, -0.295242f64 as vec_t],
    [0.000000f64 as vec_t, -1.000000f64 as vec_t, 0.000000f64 as vec_t],
    [0.262866f64 as vec_t, -0.951056f64 as vec_t, -0.162460f64 as vec_t],
    [0.000000f64 as vec_t, -0.850651f64 as vec_t, 0.525731f64 as vec_t],
    [0.000000f64 as vec_t, -0.955423f64 as vec_t, 0.295242f64 as vec_t],
    [0.238856f64 as vec_t, -0.864188f64 as vec_t, 0.442863f64 as vec_t],
    [0.262866f64 as vec_t, -0.951056f64 as vec_t, 0.162460f64 as vec_t],
    [0.500000f64 as vec_t, -0.809017f64 as vec_t, 0.309017f64 as vec_t],
    [0.716567f64 as vec_t, -0.681718f64 as vec_t, 0.147621f64 as vec_t],
    [0.525731f64 as vec_t, -0.850651f64 as vec_t, 0.000000f64 as vec_t],
    [-0.238856f64 as vec_t, -0.864188f64 as vec_t, -0.442863f64 as vec_t],
    [-0.500000f64 as vec_t, -0.809017f64 as vec_t, -0.309017f64 as vec_t],
    [-0.262866f64 as vec_t, -0.951056f64 as vec_t, -0.162460f64 as vec_t],
    [-0.850651f64 as vec_t, -0.525731f64 as vec_t, 0.000000f64 as vec_t],
    [-0.716567f64 as vec_t, -0.681718f64 as vec_t, -0.147621f64 as vec_t],
    [-0.716567f64 as vec_t, -0.681718f64 as vec_t, 0.147621f64 as vec_t],
    [-0.525731f64 as vec_t, -0.850651f64 as vec_t, 0.000000f64 as vec_t],
    [-0.500000f64 as vec_t, -0.809017f64 as vec_t, 0.309017f64 as vec_t],
    [-0.238856f64 as vec_t, -0.864188f64 as vec_t, 0.442863f64 as vec_t],
    [-0.262866f64 as vec_t, -0.951056f64 as vec_t, 0.162460f64 as vec_t],
    [-0.864188f64 as vec_t, -0.442863f64 as vec_t, 0.238856f64 as vec_t],
    [-0.809017f64 as vec_t, -0.309017f64 as vec_t, 0.500000f64 as vec_t],
    [-0.688191f64 as vec_t, -0.587785f64 as vec_t, 0.425325f64 as vec_t],
    [-0.681718f64 as vec_t, -0.147621f64 as vec_t, 0.716567f64 as vec_t],
    [-0.442863f64 as vec_t, -0.238856f64 as vec_t, 0.864188f64 as vec_t],
    [-0.587785f64 as vec_t, -0.425325f64 as vec_t, 0.688191f64 as vec_t],
    [-0.309017f64 as vec_t, -0.500000f64 as vec_t, 0.809017f64 as vec_t],
    [-0.147621f64 as vec_t, -0.716567f64 as vec_t, 0.681718f64 as vec_t],
    [-0.425325f64 as vec_t, -0.688191f64 as vec_t, 0.587785f64 as vec_t],
    [-0.162460f64 as vec_t, -0.262866f64 as vec_t, 0.951056f64 as vec_t],
    [0.442863f64 as vec_t, -0.238856f64 as vec_t, 0.864188f64 as vec_t],
    [0.162460f64 as vec_t, -0.262866f64 as vec_t, 0.951056f64 as vec_t],
    [0.309017f64 as vec_t, -0.500000f64 as vec_t, 0.809017f64 as vec_t],
    [0.147621f64 as vec_t, -0.716567f64 as vec_t, 0.681718f64 as vec_t],
    [0.000000f64 as vec_t, -0.525731f64 as vec_t, 0.850651f64 as vec_t],
    [0.425325f64 as vec_t, -0.688191f64 as vec_t, 0.587785f64 as vec_t],
    [0.587785f64 as vec_t, -0.425325f64 as vec_t, 0.688191f64 as vec_t],
    [0.688191f64 as vec_t, -0.587785f64 as vec_t, 0.425325f64 as vec_t],
    [-0.955423f64 as vec_t, 0.295242f64 as vec_t, 0.000000f64 as vec_t],
    [-0.951056f64 as vec_t, 0.162460f64 as vec_t, 0.262866f64 as vec_t],
    [-1.000000f64 as vec_t, 0.000000f64 as vec_t, 0.000000f64 as vec_t],
    [-0.850651f64 as vec_t, 0.000000f64 as vec_t, 0.525731f64 as vec_t],
    [-0.955423f64 as vec_t, -0.295242f64 as vec_t, 0.000000f64 as vec_t],
    [-0.951056f64 as vec_t, -0.162460f64 as vec_t, 0.262866f64 as vec_t],
    [-0.864188f64 as vec_t, 0.442863f64 as vec_t, -0.238856f64 as vec_t],
    [-0.951056f64 as vec_t, 0.162460f64 as vec_t, -0.262866f64 as vec_t],
    [-0.809017f64 as vec_t, 0.309017f64 as vec_t, -0.500000f64 as vec_t],
    [-0.864188f64 as vec_t, -0.442863f64 as vec_t, -0.238856f64 as vec_t],
    [-0.951056f64 as vec_t, -0.162460f64 as vec_t, -0.262866f64 as vec_t],
    [-0.809017f64 as vec_t, -0.309017f64 as vec_t, -0.500000f64 as vec_t],
    [-0.681718f64 as vec_t, 0.147621f64 as vec_t, -0.716567f64 as vec_t],
    [-0.681718f64 as vec_t, -0.147621f64 as vec_t, -0.716567f64 as vec_t],
    [-0.850651f64 as vec_t, 0.000000f64 as vec_t, -0.525731f64 as vec_t],
    [-0.688191f64 as vec_t, 0.587785f64 as vec_t, -0.425325f64 as vec_t],
    [-0.587785f64 as vec_t, 0.425325f64 as vec_t, -0.688191f64 as vec_t],
    [-0.425325f64 as vec_t, 0.688191f64 as vec_t, -0.587785f64 as vec_t],
    [-0.425325f64 as vec_t, -0.688191f64 as vec_t, -0.587785f64 as vec_t],
    [-0.587785f64 as vec_t, -0.425325f64 as vec_t, -0.688191f64 as vec_t],
    [-0.688191f64 as vec_t, -0.587785f64 as vec_t, -0.425325f64 as vec_t],
];
#[no_mangle]
pub unsafe extern "C" fn MSG_WriteChar(mut sb: *mut sizebuf_t, mut c: libc::c_int) {
    let mut buf: *mut byte = 0 as *mut byte;
    buf = SZ_GetSpace(sb, 1 as libc::c_int) as *mut byte;
    *buf.offset(0 as libc::c_int as isize) = c as byte;
}
#[no_mangle]
pub unsafe extern "C" fn MSG_WriteByte(mut sb: *mut sizebuf_t, mut c: libc::c_int) {
    let mut buf: *mut byte = 0 as *mut byte;
    buf = SZ_GetSpace(sb, 1 as libc::c_int) as *mut byte;
    *buf.offset(0 as libc::c_int as isize) = c as byte;
}
#[no_mangle]
pub unsafe extern "C" fn MSG_WriteShort(mut sb: *mut sizebuf_t, mut c: libc::c_int) {
    let mut buf: *mut byte = 0 as *mut byte;
    buf = SZ_GetSpace(sb, 2 as libc::c_int) as *mut byte;
    *buf.offset(0 as libc::c_int as isize) = (c & 0xff as libc::c_int) as byte;
    *buf.offset(1 as libc::c_int as isize) = (c >> 8 as libc::c_int) as byte;
}
#[no_mangle]
pub unsafe extern "C" fn MSG_WriteLong(mut sb: *mut sizebuf_t, mut c: libc::c_int) {
    let mut buf: *mut byte = 0 as *mut byte;
    buf = SZ_GetSpace(sb, 4 as libc::c_int) as *mut byte;
    *buf.offset(0 as libc::c_int as isize) = (c & 0xff as libc::c_int) as byte;
    *buf
        .offset(
            1 as libc::c_int as isize,
        ) = (c >> 8 as libc::c_int & 0xff as libc::c_int) as byte;
    *buf
        .offset(
            2 as libc::c_int as isize,
        ) = (c >> 16 as libc::c_int & 0xff as libc::c_int) as byte;
    *buf.offset(3 as libc::c_int as isize) = (c >> 24 as libc::c_int) as byte;
}
#[no_mangle]
pub unsafe extern "C" fn MSG_WriteFloat(mut sb: *mut sizebuf_t, mut f: libc::c_float) {
    let mut dat: C2RustUnnamed = C2RustUnnamed { f: 0. };
    dat.f = f;
    dat.l = LittleLong(dat.l);
    SZ_Write(sb, &mut dat.l as *mut libc::c_int as *mut libc::c_void, 4 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn MSG_WriteString(
    mut sb: *mut sizebuf_t,
    mut s: *mut libc::c_char,
) {
    if s.is_null() {
        SZ_Write(
            sb,
            b"\0" as *const u8 as *const libc::c_char as *mut libc::c_void,
            1 as libc::c_int,
        );
    } else {
        SZ_Write(
            sb,
            s as *mut libc::c_void,
            (strlen(s)).wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn MSG_WriteCoord(mut sb: *mut sizebuf_t, mut f: libc::c_float) {
    MSG_WriteShort(sb, (f * 8 as libc::c_int as libc::c_float) as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn MSG_WritePos(mut sb: *mut sizebuf_t, mut pos: *mut vec_t) {
    MSG_WriteShort(
        sb,
        (*pos.offset(0 as libc::c_int as isize) * 8 as libc::c_int as libc::c_float)
            as libc::c_int,
    );
    MSG_WriteShort(
        sb,
        (*pos.offset(1 as libc::c_int as isize) * 8 as libc::c_int as libc::c_float)
            as libc::c_int,
    );
    MSG_WriteShort(
        sb,
        (*pos.offset(2 as libc::c_int as isize) * 8 as libc::c_int as libc::c_float)
            as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn MSG_WriteAngle(mut sb: *mut sizebuf_t, mut f: libc::c_float) {
    MSG_WriteByte(
        sb,
        (f * 256 as libc::c_int as libc::c_float / 360 as libc::c_int as libc::c_float)
            as libc::c_int & 255 as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn MSG_WriteAngle16(mut sb: *mut sizebuf_t, mut f: libc::c_float) {
    MSG_WriteShort(
        sb,
        (f * 65536 as libc::c_int as libc::c_float / 360 as libc::c_int as libc::c_float)
            as libc::c_int & 65535 as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn MSG_WriteDeltaUsercmd(
    mut buf: *mut sizebuf_t,
    mut from: *mut usercmd_t,
    mut cmd: *mut usercmd_t,
) {
    let mut bits: libc::c_int = 0;
    bits = 0 as libc::c_int;
    if (*cmd).angles[0 as libc::c_int as usize] as libc::c_int
        != (*from).angles[0 as libc::c_int as usize] as libc::c_int
    {
        bits |= (1 as libc::c_int) << 0 as libc::c_int;
    }
    if (*cmd).angles[1 as libc::c_int as usize] as libc::c_int
        != (*from).angles[1 as libc::c_int as usize] as libc::c_int
    {
        bits |= (1 as libc::c_int) << 1 as libc::c_int;
    }
    if (*cmd).angles[2 as libc::c_int as usize] as libc::c_int
        != (*from).angles[2 as libc::c_int as usize] as libc::c_int
    {
        bits |= (1 as libc::c_int) << 2 as libc::c_int;
    }
    if (*cmd).forwardmove as libc::c_int != (*from).forwardmove as libc::c_int {
        bits |= (1 as libc::c_int) << 3 as libc::c_int;
    }
    if (*cmd).sidemove as libc::c_int != (*from).sidemove as libc::c_int {
        bits |= (1 as libc::c_int) << 4 as libc::c_int;
    }
    if (*cmd).upmove as libc::c_int != (*from).upmove as libc::c_int {
        bits |= (1 as libc::c_int) << 5 as libc::c_int;
    }
    if (*cmd).buttons as libc::c_int != (*from).buttons as libc::c_int {
        bits |= (1 as libc::c_int) << 6 as libc::c_int;
    }
    if (*cmd).impulse as libc::c_int != (*from).impulse as libc::c_int {
        bits |= (1 as libc::c_int) << 7 as libc::c_int;
    }
    MSG_WriteByte(buf, bits);
    if bits & (1 as libc::c_int) << 0 as libc::c_int != 0 {
        MSG_WriteShort(buf, (*cmd).angles[0 as libc::c_int as usize] as libc::c_int);
    }
    if bits & (1 as libc::c_int) << 1 as libc::c_int != 0 {
        MSG_WriteShort(buf, (*cmd).angles[1 as libc::c_int as usize] as libc::c_int);
    }
    if bits & (1 as libc::c_int) << 2 as libc::c_int != 0 {
        MSG_WriteShort(buf, (*cmd).angles[2 as libc::c_int as usize] as libc::c_int);
    }
    if bits & (1 as libc::c_int) << 3 as libc::c_int != 0 {
        MSG_WriteShort(buf, (*cmd).forwardmove as libc::c_int);
    }
    if bits & (1 as libc::c_int) << 4 as libc::c_int != 0 {
        MSG_WriteShort(buf, (*cmd).sidemove as libc::c_int);
    }
    if bits & (1 as libc::c_int) << 5 as libc::c_int != 0 {
        MSG_WriteShort(buf, (*cmd).upmove as libc::c_int);
    }
    if bits & (1 as libc::c_int) << 6 as libc::c_int != 0 {
        MSG_WriteByte(buf, (*cmd).buttons as libc::c_int);
    }
    if bits & (1 as libc::c_int) << 7 as libc::c_int != 0 {
        MSG_WriteByte(buf, (*cmd).impulse as libc::c_int);
    }
    MSG_WriteByte(buf, (*cmd).msec as libc::c_int);
    MSG_WriteByte(buf, (*cmd).lightlevel as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn MSG_WriteDir(mut sb: *mut sizebuf_t, mut dir: *mut vec_t) {
    let mut i: libc::c_int = 0;
    let mut best: libc::c_int = 0;
    let mut d: libc::c_float = 0.;
    let mut bestd: libc::c_float = 0.;
    if dir.is_null() {
        MSG_WriteByte(sb, 0 as libc::c_int);
        return;
    }
    bestd = 0 as libc::c_int as libc::c_float;
    best = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < 162 as libc::c_int {
        d = *dir.offset(0 as libc::c_int as isize)
            * bytedirs[i as usize][0 as libc::c_int as usize]
            + *dir.offset(1 as libc::c_int as isize)
                * bytedirs[i as usize][1 as libc::c_int as usize]
            + *dir.offset(2 as libc::c_int as isize)
                * bytedirs[i as usize][2 as libc::c_int as usize];
        if d > bestd {
            bestd = d;
            best = i;
        }
        i += 1;
    }
    MSG_WriteByte(sb, best);
}
#[no_mangle]
pub unsafe extern "C" fn MSG_ReadDir(mut sb: *mut sizebuf_t, mut dir: *mut vec_t) {
    let mut b: libc::c_int = 0;
    b = MSG_ReadByte(sb);
    if b >= 162 as libc::c_int {
        Com_Error(
            1 as libc::c_int,
            b"MSF_ReadDir: out of range\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    *dir
        .offset(
            0 as libc::c_int as isize,
        ) = bytedirs[b as usize][0 as libc::c_int as usize];
    *dir
        .offset(
            1 as libc::c_int as isize,
        ) = bytedirs[b as usize][1 as libc::c_int as usize];
    *dir
        .offset(
            2 as libc::c_int as isize,
        ) = bytedirs[b as usize][2 as libc::c_int as usize];
}
#[no_mangle]
pub unsafe extern "C" fn MSG_WriteDeltaEntity(
    mut from: *mut entity_state_t,
    mut to: *mut entity_state_t,
    mut msg: *mut sizebuf_t,
    mut force: qboolean,
    mut newentity: qboolean,
) {
    let mut bits: libc::c_int = 0;
    if (*to).number == 0 {
        Com_Error(
            0 as libc::c_int,
            b"Unset entity number\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    if (*to).number >= 1024 as libc::c_int {
        Com_Error(
            0 as libc::c_int,
            b"Entity number >= MAX_EDICTS\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    bits = 0 as libc::c_int;
    if (*to).number >= 256 as libc::c_int {
        bits |= (1 as libc::c_int) << 8 as libc::c_int;
    }
    if (*to).origin[0 as libc::c_int as usize]
        != (*from).origin[0 as libc::c_int as usize]
    {
        bits |= (1 as libc::c_int) << 0 as libc::c_int;
    }
    if (*to).origin[1 as libc::c_int as usize]
        != (*from).origin[1 as libc::c_int as usize]
    {
        bits |= (1 as libc::c_int) << 1 as libc::c_int;
    }
    if (*to).origin[2 as libc::c_int as usize]
        != (*from).origin[2 as libc::c_int as usize]
    {
        bits |= (1 as libc::c_int) << 9 as libc::c_int;
    }
    if (*to).angles[0 as libc::c_int as usize]
        != (*from).angles[0 as libc::c_int as usize]
    {
        bits |= (1 as libc::c_int) << 10 as libc::c_int;
    }
    if (*to).angles[1 as libc::c_int as usize]
        != (*from).angles[1 as libc::c_int as usize]
    {
        bits |= (1 as libc::c_int) << 2 as libc::c_int;
    }
    if (*to).angles[2 as libc::c_int as usize]
        != (*from).angles[2 as libc::c_int as usize]
    {
        bits |= (1 as libc::c_int) << 3 as libc::c_int;
    }
    if (*to).skinnum != (*from).skinnum {
        if ((*to).skinnum as libc::c_uint) < 256 as libc::c_int as libc::c_uint {
            bits |= (1 as libc::c_int) << 16 as libc::c_int;
        } else if ((*to).skinnum as libc::c_uint)
            < 0x10000 as libc::c_int as libc::c_uint
        {
            bits |= (1 as libc::c_int) << 25 as libc::c_int;
        } else {
            bits
                |= (1 as libc::c_int) << 16 as libc::c_int
                    | (1 as libc::c_int) << 25 as libc::c_int;
        }
    }
    if (*to).frame != (*from).frame {
        if (*to).frame < 256 as libc::c_int {
            bits |= (1 as libc::c_int) << 4 as libc::c_int;
        } else {
            bits |= (1 as libc::c_int) << 17 as libc::c_int;
        }
    }
    if (*to).effects != (*from).effects {
        if (*to).effects < 256 as libc::c_int as libc::c_uint {
            bits |= (1 as libc::c_int) << 14 as libc::c_int;
        } else if (*to).effects < 0x8000 as libc::c_int as libc::c_uint {
            bits |= (1 as libc::c_int) << 19 as libc::c_int;
        } else {
            bits
                |= (1 as libc::c_int) << 14 as libc::c_int
                    | (1 as libc::c_int) << 19 as libc::c_int;
        }
    }
    if (*to).renderfx != (*from).renderfx {
        if (*to).renderfx < 256 as libc::c_int {
            bits |= (1 as libc::c_int) << 12 as libc::c_int;
        } else if (*to).renderfx < 0x8000 as libc::c_int {
            bits |= (1 as libc::c_int) << 18 as libc::c_int;
        } else {
            bits
                |= (1 as libc::c_int) << 12 as libc::c_int
                    | (1 as libc::c_int) << 18 as libc::c_int;
        }
    }
    if (*to).solid != (*from).solid {
        bits |= (1 as libc::c_int) << 27 as libc::c_int;
    }
    if (*to).event != 0 {
        bits |= (1 as libc::c_int) << 5 as libc::c_int;
    }
    if (*to).modelindex != (*from).modelindex {
        bits |= (1 as libc::c_int) << 11 as libc::c_int;
    }
    if (*to).modelindex2 != (*from).modelindex2 {
        bits |= (1 as libc::c_int) << 20 as libc::c_int;
    }
    if (*to).modelindex3 != (*from).modelindex3 {
        bits |= (1 as libc::c_int) << 21 as libc::c_int;
    }
    if (*to).modelindex4 != (*from).modelindex4 {
        bits |= (1 as libc::c_int) << 22 as libc::c_int;
    }
    if (*to).sound != (*from).sound {
        bits |= (1 as libc::c_int) << 26 as libc::c_int;
    }
    if newentity as libc::c_uint != 0 || (*to).renderfx & 128 as libc::c_int != 0 {
        bits |= (1 as libc::c_int) << 24 as libc::c_int;
    }
    if bits == 0 && force as u64 == 0 {
        return;
    }
    if bits as libc::c_uint & 0xff000000 as libc::c_uint != 0 {
        bits
            |= (1 as libc::c_int) << 23 as libc::c_int
                | (1 as libc::c_int) << 15 as libc::c_int
                | (1 as libc::c_int) << 7 as libc::c_int;
    } else if bits & 0xff0000 as libc::c_int != 0 {
        bits
            |= (1 as libc::c_int) << 15 as libc::c_int
                | (1 as libc::c_int) << 7 as libc::c_int;
    } else if bits & 0xff00 as libc::c_int != 0 {
        bits |= (1 as libc::c_int) << 7 as libc::c_int;
    }
    MSG_WriteByte(msg, bits & 255 as libc::c_int);
    if bits as libc::c_uint & 0xff000000 as libc::c_uint != 0 {
        MSG_WriteByte(msg, bits >> 8 as libc::c_int & 255 as libc::c_int);
        MSG_WriteByte(msg, bits >> 16 as libc::c_int & 255 as libc::c_int);
        MSG_WriteByte(msg, bits >> 24 as libc::c_int & 255 as libc::c_int);
    } else if bits & 0xff0000 as libc::c_int != 0 {
        MSG_WriteByte(msg, bits >> 8 as libc::c_int & 255 as libc::c_int);
        MSG_WriteByte(msg, bits >> 16 as libc::c_int & 255 as libc::c_int);
    } else if bits & 0xff00 as libc::c_int != 0 {
        MSG_WriteByte(msg, bits >> 8 as libc::c_int & 255 as libc::c_int);
    }
    if bits & (1 as libc::c_int) << 8 as libc::c_int != 0 {
        MSG_WriteShort(msg, (*to).number);
    } else {
        MSG_WriteByte(msg, (*to).number);
    }
    if bits & (1 as libc::c_int) << 11 as libc::c_int != 0 {
        MSG_WriteByte(msg, (*to).modelindex);
    }
    if bits & (1 as libc::c_int) << 20 as libc::c_int != 0 {
        MSG_WriteByte(msg, (*to).modelindex2);
    }
    if bits & (1 as libc::c_int) << 21 as libc::c_int != 0 {
        MSG_WriteByte(msg, (*to).modelindex3);
    }
    if bits & (1 as libc::c_int) << 22 as libc::c_int != 0 {
        MSG_WriteByte(msg, (*to).modelindex4);
    }
    if bits & (1 as libc::c_int) << 4 as libc::c_int != 0 {
        MSG_WriteByte(msg, (*to).frame);
    }
    if bits & (1 as libc::c_int) << 17 as libc::c_int != 0 {
        MSG_WriteShort(msg, (*to).frame);
    }
    if bits & (1 as libc::c_int) << 16 as libc::c_int != 0
        && bits & (1 as libc::c_int) << 25 as libc::c_int != 0
    {
        MSG_WriteLong(msg, (*to).skinnum);
    } else if bits & (1 as libc::c_int) << 16 as libc::c_int != 0 {
        MSG_WriteByte(msg, (*to).skinnum);
    } else if bits & (1 as libc::c_int) << 25 as libc::c_int != 0 {
        MSG_WriteShort(msg, (*to).skinnum);
    }
    if bits
        & ((1 as libc::c_int) << 14 as libc::c_int
            | (1 as libc::c_int) << 19 as libc::c_int)
        == (1 as libc::c_int) << 14 as libc::c_int
            | (1 as libc::c_int) << 19 as libc::c_int
    {
        MSG_WriteLong(msg, (*to).effects as libc::c_int);
    } else if bits & (1 as libc::c_int) << 14 as libc::c_int != 0 {
        MSG_WriteByte(msg, (*to).effects as libc::c_int);
    } else if bits & (1 as libc::c_int) << 19 as libc::c_int != 0 {
        MSG_WriteShort(msg, (*to).effects as libc::c_int);
    }
    if bits
        & ((1 as libc::c_int) << 12 as libc::c_int
            | (1 as libc::c_int) << 18 as libc::c_int)
        == (1 as libc::c_int) << 12 as libc::c_int
            | (1 as libc::c_int) << 18 as libc::c_int
    {
        MSG_WriteLong(msg, (*to).renderfx);
    } else if bits & (1 as libc::c_int) << 12 as libc::c_int != 0 {
        MSG_WriteByte(msg, (*to).renderfx);
    } else if bits & (1 as libc::c_int) << 18 as libc::c_int != 0 {
        MSG_WriteShort(msg, (*to).renderfx);
    }
    if bits & (1 as libc::c_int) << 0 as libc::c_int != 0 {
        MSG_WriteCoord(msg, (*to).origin[0 as libc::c_int as usize]);
    }
    if bits & (1 as libc::c_int) << 1 as libc::c_int != 0 {
        MSG_WriteCoord(msg, (*to).origin[1 as libc::c_int as usize]);
    }
    if bits & (1 as libc::c_int) << 9 as libc::c_int != 0 {
        MSG_WriteCoord(msg, (*to).origin[2 as libc::c_int as usize]);
    }
    if bits & (1 as libc::c_int) << 10 as libc::c_int != 0 {
        MSG_WriteAngle(msg, (*to).angles[0 as libc::c_int as usize]);
    }
    if bits & (1 as libc::c_int) << 2 as libc::c_int != 0 {
        MSG_WriteAngle(msg, (*to).angles[1 as libc::c_int as usize]);
    }
    if bits & (1 as libc::c_int) << 3 as libc::c_int != 0 {
        MSG_WriteAngle(msg, (*to).angles[2 as libc::c_int as usize]);
    }
    if bits & (1 as libc::c_int) << 24 as libc::c_int != 0 {
        MSG_WriteCoord(msg, (*to).old_origin[0 as libc::c_int as usize]);
        MSG_WriteCoord(msg, (*to).old_origin[1 as libc::c_int as usize]);
        MSG_WriteCoord(msg, (*to).old_origin[2 as libc::c_int as usize]);
    }
    if bits & (1 as libc::c_int) << 26 as libc::c_int != 0 {
        MSG_WriteByte(msg, (*to).sound);
    }
    if bits & (1 as libc::c_int) << 5 as libc::c_int != 0 {
        MSG_WriteByte(msg, (*to).event);
    }
    if bits & (1 as libc::c_int) << 27 as libc::c_int != 0 {
        MSG_WriteShort(msg, (*to).solid);
    }
}
#[no_mangle]
pub unsafe extern "C" fn MSG_BeginReading(mut msg: *mut sizebuf_t) {
    (*msg).readcount = 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn MSG_ReadChar(mut msg_read: *mut sizebuf_t) -> libc::c_int {
    let mut c: libc::c_int = 0;
    if (*msg_read).readcount + 1 as libc::c_int > (*msg_read).cursize {
        c = -(1 as libc::c_int);
    } else {
        c = *((*msg_read).data).offset((*msg_read).readcount as isize) as libc::c_schar
            as libc::c_int;
    }
    let ref mut fresh0 = (*msg_read).readcount;
    *fresh0 += 1;
    return c;
}
#[no_mangle]
pub unsafe extern "C" fn MSG_ReadByte(mut msg_read: *mut sizebuf_t) -> libc::c_int {
    let mut c: libc::c_int = 0;
    if (*msg_read).readcount + 1 as libc::c_int > (*msg_read).cursize {
        c = -(1 as libc::c_int);
    } else {
        c = *((*msg_read).data).offset((*msg_read).readcount as isize) as libc::c_int;
    }
    let ref mut fresh1 = (*msg_read).readcount;
    *fresh1 += 1;
    return c;
}
#[no_mangle]
pub unsafe extern "C" fn MSG_ReadShort(mut msg_read: *mut sizebuf_t) -> libc::c_int {
    let mut c: libc::c_int = 0;
    if (*msg_read).readcount + 2 as libc::c_int > (*msg_read).cursize {
        c = -(1 as libc::c_int);
    } else {
        c = (*((*msg_read).data).offset((*msg_read).readcount as isize) as libc::c_int
            + ((*((*msg_read).data)
                .offset(((*msg_read).readcount + 1 as libc::c_int) as isize)
                as libc::c_int) << 8 as libc::c_int)) as libc::c_short as libc::c_int;
    }
    (*msg_read).readcount += 2 as libc::c_int;
    return c;
}
#[no_mangle]
pub unsafe extern "C" fn MSG_ReadLong(mut msg_read: *mut sizebuf_t) -> libc::c_int {
    let mut c: libc::c_int = 0;
    if (*msg_read).readcount + 4 as libc::c_int > (*msg_read).cursize {
        c = -(1 as libc::c_int);
    } else {
        c = *((*msg_read).data).offset((*msg_read).readcount as isize) as libc::c_int
            + ((*((*msg_read).data)
                .offset(((*msg_read).readcount + 1 as libc::c_int) as isize)
                as libc::c_int) << 8 as libc::c_int)
            + ((*((*msg_read).data)
                .offset(((*msg_read).readcount + 2 as libc::c_int) as isize)
                as libc::c_int) << 16 as libc::c_int)
            + ((*((*msg_read).data)
                .offset(((*msg_read).readcount + 3 as libc::c_int) as isize)
                as libc::c_int) << 24 as libc::c_int);
    }
    (*msg_read).readcount += 4 as libc::c_int;
    return c;
}
#[no_mangle]
pub unsafe extern "C" fn MSG_ReadFloat(mut msg_read: *mut sizebuf_t) -> libc::c_float {
    let mut dat: C2RustUnnamed_0 = C2RustUnnamed_0 { b: [0; 4] };
    if (*msg_read).readcount + 4 as libc::c_int > (*msg_read).cursize {
        dat.f = -(1 as libc::c_int) as libc::c_float;
    } else {
        dat
            .b[0 as libc::c_int
            as usize] = *((*msg_read).data).offset((*msg_read).readcount as isize);
        dat
            .b[1 as libc::c_int
            as usize] = *((*msg_read).data)
            .offset(((*msg_read).readcount + 1 as libc::c_int) as isize);
        dat
            .b[2 as libc::c_int
            as usize] = *((*msg_read).data)
            .offset(((*msg_read).readcount + 2 as libc::c_int) as isize);
        dat
            .b[3 as libc::c_int
            as usize] = *((*msg_read).data)
            .offset(((*msg_read).readcount + 3 as libc::c_int) as isize);
    }
    (*msg_read).readcount += 4 as libc::c_int;
    dat.l = LittleLong(dat.l);
    return dat.f;
}
#[no_mangle]
pub unsafe extern "C" fn MSG_ReadString(
    mut msg_read: *mut sizebuf_t,
) -> *mut libc::c_char {
    static mut string: [libc::c_char; 2048] = [0; 2048];
    let mut l: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    l = 0 as libc::c_int;
    loop {
        c = MSG_ReadChar(msg_read);
        if c == -(1 as libc::c_int) || c == 0 as libc::c_int {
            break;
        }
        string[l as usize] = c as libc::c_char;
        l += 1;
        if !((l as libc::c_ulong)
            < (::std::mem::size_of::<[libc::c_char; 2048]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong))
        {
            break;
        }
    }
    string[l as usize] = 0 as libc::c_int as libc::c_char;
    return string.as_mut_ptr();
}
#[no_mangle]
pub unsafe extern "C" fn MSG_ReadStringLine(
    mut msg_read: *mut sizebuf_t,
) -> *mut libc::c_char {
    static mut string: [libc::c_char; 2048] = [0; 2048];
    let mut l: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    l = 0 as libc::c_int;
    loop {
        c = MSG_ReadChar(msg_read);
        if c == -(1 as libc::c_int) || c == 0 as libc::c_int || c == '\n' as i32 {
            break;
        }
        string[l as usize] = c as libc::c_char;
        l += 1;
        if !((l as libc::c_ulong)
            < (::std::mem::size_of::<[libc::c_char; 2048]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong))
        {
            break;
        }
    }
    string[l as usize] = 0 as libc::c_int as libc::c_char;
    return string.as_mut_ptr();
}
#[no_mangle]
pub unsafe extern "C" fn MSG_ReadCoord(mut msg_read: *mut sizebuf_t) -> libc::c_float {
    return (MSG_ReadShort(msg_read) as libc::c_double
        * (1.0f64 / 8 as libc::c_int as libc::c_double)) as libc::c_float;
}
#[no_mangle]
pub unsafe extern "C" fn MSG_ReadPos(mut msg_read: *mut sizebuf_t, mut pos: *mut vec_t) {
    *pos
        .offset(
            0 as libc::c_int as isize,
        ) = (MSG_ReadShort(msg_read) as libc::c_double
        * (1.0f64 / 8 as libc::c_int as libc::c_double)) as vec_t;
    *pos
        .offset(
            1 as libc::c_int as isize,
        ) = (MSG_ReadShort(msg_read) as libc::c_double
        * (1.0f64 / 8 as libc::c_int as libc::c_double)) as vec_t;
    *pos
        .offset(
            2 as libc::c_int as isize,
        ) = (MSG_ReadShort(msg_read) as libc::c_double
        * (1.0f64 / 8 as libc::c_int as libc::c_double)) as vec_t;
}
#[no_mangle]
pub unsafe extern "C" fn MSG_ReadAngle(mut msg_read: *mut sizebuf_t) -> libc::c_float {
    return (MSG_ReadChar(msg_read) as libc::c_double
        * (360.0f64 / 256 as libc::c_int as libc::c_double)) as libc::c_float;
}
#[no_mangle]
pub unsafe extern "C" fn MSG_ReadAngle16(mut msg_read: *mut sizebuf_t) -> libc::c_float {
    return (MSG_ReadShort(msg_read) as libc::c_double
        * (360.0f64 / 65536 as libc::c_int as libc::c_double)) as libc::c_float;
}
#[no_mangle]
pub unsafe extern "C" fn MSG_ReadDeltaUsercmd(
    mut msg_read: *mut sizebuf_t,
    mut from: *mut usercmd_t,
    mut move_0: *mut usercmd_t,
) {
    let mut bits: libc::c_int = 0;
    memcpy(
        move_0 as *mut libc::c_void,
        from as *const libc::c_void,
        ::std::mem::size_of::<usercmd_t>() as libc::c_ulong,
    );
    bits = MSG_ReadByte(msg_read);
    if bits & (1 as libc::c_int) << 0 as libc::c_int != 0 {
        (*move_0)
            .angles[0 as libc::c_int
            as usize] = MSG_ReadShort(msg_read) as libc::c_short;
    }
    if bits & (1 as libc::c_int) << 1 as libc::c_int != 0 {
        (*move_0)
            .angles[1 as libc::c_int
            as usize] = MSG_ReadShort(msg_read) as libc::c_short;
    }
    if bits & (1 as libc::c_int) << 2 as libc::c_int != 0 {
        (*move_0)
            .angles[2 as libc::c_int
            as usize] = MSG_ReadShort(msg_read) as libc::c_short;
    }
    if bits & (1 as libc::c_int) << 3 as libc::c_int != 0 {
        (*move_0).forwardmove = MSG_ReadShort(msg_read) as libc::c_short;
    }
    if bits & (1 as libc::c_int) << 4 as libc::c_int != 0 {
        (*move_0).sidemove = MSG_ReadShort(msg_read) as libc::c_short;
    }
    if bits & (1 as libc::c_int) << 5 as libc::c_int != 0 {
        (*move_0).upmove = MSG_ReadShort(msg_read) as libc::c_short;
    }
    if bits & (1 as libc::c_int) << 6 as libc::c_int != 0 {
        (*move_0).buttons = MSG_ReadByte(msg_read) as byte;
    }
    if bits & (1 as libc::c_int) << 7 as libc::c_int != 0 {
        (*move_0).impulse = MSG_ReadByte(msg_read) as byte;
    }
    (*move_0).msec = MSG_ReadByte(msg_read) as byte;
    (*move_0).lightlevel = MSG_ReadByte(msg_read) as byte;
}
#[no_mangle]
pub unsafe extern "C" fn MSG_ReadData(
    mut msg_read: *mut sizebuf_t,
    mut data: *mut libc::c_void,
    mut len: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < len {
        *(data as *mut byte).offset(i as isize) = MSG_ReadByte(msg_read) as byte;
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn SZ_Init(
    mut buf: *mut sizebuf_t,
    mut data: *mut byte,
    mut length: libc::c_int,
) {
    memset(
        buf as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<sizebuf_t>() as libc::c_ulong,
    );
    let ref mut fresh2 = (*buf).data;
    *fresh2 = data;
    (*buf).maxsize = length;
}
#[no_mangle]
pub unsafe extern "C" fn SZ_Clear(mut buf: *mut sizebuf_t) {
    (*buf).cursize = 0 as libc::c_int;
    (*buf).overflowed = false_0;
}
#[no_mangle]
pub unsafe extern "C" fn SZ_GetSpace(
    mut buf: *mut sizebuf_t,
    mut length: libc::c_int,
) -> *mut libc::c_void {
    let mut data: *mut libc::c_void = 0 as *mut libc::c_void;
    if (*buf).cursize + length > (*buf).maxsize {
        if (*buf).allowoverflow as u64 == 0 {
            Com_Error(
                0 as libc::c_int,
                b"SZ_GetSpace: overflow without allowoverflow set\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
            );
        }
        if length > (*buf).maxsize {
            Com_Error(
                0 as libc::c_int,
                b"SZ_GetSpace: %i is > full buffer size\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                length,
            );
        }
        Com_Printf(
            b"SZ_GetSpace: overflow\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        SZ_Clear(buf);
        (*buf).overflowed = true_0;
    }
    data = ((*buf).data).offset((*buf).cursize as isize) as *mut libc::c_void;
    (*buf).cursize += length;
    return data;
}
#[no_mangle]
pub unsafe extern "C" fn SZ_Write(
    mut buf: *mut sizebuf_t,
    mut data: *mut libc::c_void,
    mut length: libc::c_int,
) {
    memcpy(SZ_GetSpace(buf, length), data, length as libc::c_ulong);
}
#[no_mangle]
pub unsafe extern "C" fn SZ_Print(mut buf: *mut sizebuf_t, mut data: *mut libc::c_char) {
    let mut len: libc::c_int = 0;
    len = (strlen(data)).wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int;
    if (*buf).cursize != 0 {
        if *((*buf).data).offset(((*buf).cursize - 1 as libc::c_int) as isize) != 0 {
            memcpy(
                SZ_GetSpace(buf, len) as *mut byte as *mut libc::c_void,
                data as *const libc::c_void,
                len as libc::c_ulong,
            );
        } else {
            memcpy(
                (SZ_GetSpace(buf, len - 1 as libc::c_int) as *mut byte)
                    .offset(-(1 as libc::c_int as isize)) as *mut libc::c_void,
                data as *const libc::c_void,
                len as libc::c_ulong,
            );
        }
    } else {
        memcpy(
            SZ_GetSpace(buf, len) as *mut byte as *mut libc::c_void,
            data as *const libc::c_void,
            len as libc::c_ulong,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn COM_CheckParm(mut parm: *mut libc::c_char) -> libc::c_int {
    let mut i: libc::c_int = 0;
    i = 1 as libc::c_int;
    while i < com_argc {
        if strcmp(parm, com_argv[i as usize]) == 0 {
            return i;
        }
        i += 1;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn COM_Argc() -> libc::c_int {
    return com_argc;
}
#[no_mangle]
pub unsafe extern "C" fn COM_Argv(mut arg: libc::c_int) -> *mut libc::c_char {
    if arg < 0 as libc::c_int || arg >= com_argc || (com_argv[arg as usize]).is_null() {
        return b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    return com_argv[arg as usize];
}
#[no_mangle]
pub unsafe extern "C" fn COM_ClearArgv(mut arg: libc::c_int) {
    if arg < 0 as libc::c_int || arg >= com_argc || (com_argv[arg as usize]).is_null() {
        return;
    }
    com_argv[arg
        as usize] = b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn COM_InitArgv(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) {
    let mut i: libc::c_int = 0;
    if argc > 50 as libc::c_int {
        Com_Error(
            0 as libc::c_int,
            b"argc > MAX_NUM_ARGVS\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    com_argc = argc;
    i = 0 as libc::c_int;
    while i < argc {
        if (*argv.offset(i as isize)).is_null()
            || strlen(*argv.offset(i as isize)) >= 128 as libc::c_int as libc::c_ulong
        {
            com_argv[i
                as usize] = b"\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char;
        } else {
            com_argv[i as usize] = *argv.offset(i as isize);
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn COM_AddParm(mut parm: *mut libc::c_char) {
    if com_argc == 50 as libc::c_int {
        Com_Error(
            0 as libc::c_int,
            b"COM_AddParm: MAX_NUM)ARGS\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    let fresh3 = com_argc;
    com_argc = com_argc + 1;
    com_argv[fresh3 as usize] = parm;
}
#[no_mangle]
pub unsafe extern "C" fn memsearch(
    mut start: *mut byte,
    mut count: libc::c_int,
    mut search: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < count {
        if *start.offset(i as isize) as libc::c_int == search {
            return i;
        }
        i += 1;
    }
    return -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn CopyString(mut in_0: *mut libc::c_char) -> *mut libc::c_char {
    let mut out: *mut libc::c_char = 0 as *mut libc::c_char;
    out = Z_Malloc(
        (strlen(in_0)).wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int,
    ) as *mut libc::c_char;
    strcpy(out, in_0);
    return out;
}
#[no_mangle]
pub unsafe extern "C" fn Info_Print(mut s: *mut libc::c_char) {
    let mut key: [libc::c_char; 512] = [0; 512];
    let mut value: [libc::c_char; 512] = [0; 512];
    let mut o: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut l: libc::c_int = 0;
    if *s as libc::c_int == '\\' as i32 {
        s = s.offset(1);
    }
    while *s != 0 {
        o = key.as_mut_ptr();
        while *s as libc::c_int != 0 && *s as libc::c_int != '\\' as i32 {
            let fresh4 = s;
            s = s.offset(1);
            let fresh5 = o;
            o = o.offset(1);
            *fresh5 = *fresh4;
        }
        l = o.offset_from(key.as_mut_ptr()) as libc::c_long as libc::c_int;
        if l < 20 as libc::c_int {
            memset(
                o as *mut libc::c_void,
                ' ' as i32,
                (20 as libc::c_int - l) as libc::c_ulong,
            );
            key[20 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        } else {
            *o = 0 as libc::c_int as libc::c_char;
        }
        Com_Printf(
            b"%s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            key.as_mut_ptr(),
        );
        if *s == 0 {
            Com_Printf(
                b"MISSING VALUE\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
            return;
        }
        o = value.as_mut_ptr();
        s = s.offset(1);
        while *s as libc::c_int != 0 && *s as libc::c_int != '\\' as i32 {
            let fresh6 = s;
            s = s.offset(1);
            let fresh7 = o;
            o = o.offset(1);
            *fresh7 = *fresh6;
        }
        *o = 0 as libc::c_int as libc::c_char;
        if *s != 0 {
            s = s.offset(1);
        }
        Com_Printf(
            b"%s\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value.as_mut_ptr(),
        );
    }
}
#[no_mangle]
pub static mut z_chain: zhead_t = zhead_t {
    prev: 0 as *const zhead_s as *mut zhead_s,
    next: 0 as *const zhead_s as *mut zhead_s,
    magic: 0,
    tag: 0,
    size: 0,
};
#[no_mangle]
pub static mut z_count: libc::c_int = 0;
#[no_mangle]
pub static mut z_bytes: libc::c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn Z_Free(mut ptr: *mut libc::c_void) {
    let mut z: *mut zhead_t = 0 as *mut zhead_t;
    z = (ptr as *mut zhead_t).offset(-(1 as libc::c_int as isize));
    if (*z).magic as libc::c_int != 0x1d1d as libc::c_int {
        Com_Error(
            0 as libc::c_int,
            b"Z_Free: bad magic\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    let ref mut fresh8 = (*(*z).prev).next;
    *fresh8 = (*z).next;
    let ref mut fresh9 = (*(*z).next).prev;
    *fresh9 = (*z).prev;
    z_count -= 1;
    z_bytes -= (*z).size;
    free(z as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn Z_Stats_f() {
    Com_Printf(
        b"%i bytes in %i blocks\n\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        z_bytes,
        z_count,
    );
}
#[no_mangle]
pub unsafe extern "C" fn Z_FreeTags(mut tag: libc::c_int) {
    let mut z: *mut zhead_t = 0 as *mut zhead_t;
    let mut next: *mut zhead_t = 0 as *mut zhead_t;
    z = z_chain.next;
    while z != &mut z_chain as *mut zhead_t {
        next = (*z).next;
        if (*z).tag as libc::c_int == tag {
            Z_Free(z.offset(1 as libc::c_int as isize) as *mut libc::c_void);
        }
        z = next;
    }
}
#[no_mangle]
pub unsafe extern "C" fn Z_TagMalloc(
    mut size: libc::c_int,
    mut tag: libc::c_int,
) -> *mut libc::c_void {
    let mut z: *mut zhead_t = 0 as *mut zhead_t;
    size = (size as libc::c_ulong)
        .wrapping_add(::std::mem::size_of::<zhead_t>() as libc::c_ulong) as libc::c_int;
    z = malloc(size as libc::c_ulong) as *mut zhead_t;
    if z.is_null() {
        Com_Error(
            0 as libc::c_int,
            b"Z_Malloc: failed on allocation of %i bytes\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            size,
        );
    }
    memset(z as *mut libc::c_void, 0 as libc::c_int, size as libc::c_ulong);
    z_count += 1;
    z_bytes += size;
    (*z).magic = 0x1d1d as libc::c_int as libc::c_short;
    (*z).tag = tag as libc::c_short;
    (*z).size = size;
    let ref mut fresh10 = (*z).next;
    *fresh10 = z_chain.next;
    let ref mut fresh11 = (*z).prev;
    *fresh11 = &mut z_chain;
    let ref mut fresh12 = (*z_chain.next).prev;
    *fresh12 = z;
    z_chain.next = z;
    return z.offset(1 as libc::c_int as isize) as *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn Z_Malloc(mut size: libc::c_int) -> *mut libc::c_void {
    return Z_TagMalloc(size, 0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn COM_BlockSequenceCheckByte(
    mut base: *mut byte,
    mut length: libc::c_int,
    mut sequence: libc::c_int,
    mut challenge: libc::c_int,
) -> byte {
    Sys_Error(
        b"COM_BlockSequenceCheckByte called\n\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    return 0 as libc::c_int as byte;
}
static mut chktbl: [byte; 1024] = [
    0x84 as libc::c_int as byte,
    0x47 as libc::c_int as byte,
    0x51 as libc::c_int as byte,
    0xc1 as libc::c_int as byte,
    0x93 as libc::c_int as byte,
    0x22 as libc::c_int as byte,
    0x21 as libc::c_int as byte,
    0x24 as libc::c_int as byte,
    0x2f as libc::c_int as byte,
    0x66 as libc::c_int as byte,
    0x60 as libc::c_int as byte,
    0x4d as libc::c_int as byte,
    0xb0 as libc::c_int as byte,
    0x7c as libc::c_int as byte,
    0xda as libc::c_int as byte,
    0x88 as libc::c_int as byte,
    0x54 as libc::c_int as byte,
    0x15 as libc::c_int as byte,
    0x2b as libc::c_int as byte,
    0xc6 as libc::c_int as byte,
    0x6c as libc::c_int as byte,
    0x89 as libc::c_int as byte,
    0xc5 as libc::c_int as byte,
    0x9d as libc::c_int as byte,
    0x48 as libc::c_int as byte,
    0xee as libc::c_int as byte,
    0xe6 as libc::c_int as byte,
    0x8a as libc::c_int as byte,
    0xb5 as libc::c_int as byte,
    0xf4 as libc::c_int as byte,
    0xcb as libc::c_int as byte,
    0xfb as libc::c_int as byte,
    0xf1 as libc::c_int as byte,
    0xc as libc::c_int as byte,
    0x2e as libc::c_int as byte,
    0xa0 as libc::c_int as byte,
    0xd7 as libc::c_int as byte,
    0xc9 as libc::c_int as byte,
    0x1f as libc::c_int as byte,
    0xd6 as libc::c_int as byte,
    0x6 as libc::c_int as byte,
    0x9a as libc::c_int as byte,
    0x9 as libc::c_int as byte,
    0x41 as libc::c_int as byte,
    0x54 as libc::c_int as byte,
    0x67 as libc::c_int as byte,
    0x46 as libc::c_int as byte,
    0xc7 as libc::c_int as byte,
    0x74 as libc::c_int as byte,
    0xe3 as libc::c_int as byte,
    0xc8 as libc::c_int as byte,
    0xb6 as libc::c_int as byte,
    0x5d as libc::c_int as byte,
    0xa6 as libc::c_int as byte,
    0x36 as libc::c_int as byte,
    0xc4 as libc::c_int as byte,
    0xab as libc::c_int as byte,
    0x2c as libc::c_int as byte,
    0x7e as libc::c_int as byte,
    0x85 as libc::c_int as byte,
    0xa8 as libc::c_int as byte,
    0xa4 as libc::c_int as byte,
    0xa6 as libc::c_int as byte,
    0x4d as libc::c_int as byte,
    0x96 as libc::c_int as byte,
    0x19 as libc::c_int as byte,
    0x19 as libc::c_int as byte,
    0x9a as libc::c_int as byte,
    0xcc as libc::c_int as byte,
    0xd8 as libc::c_int as byte,
    0xac as libc::c_int as byte,
    0x39 as libc::c_int as byte,
    0x5e as libc::c_int as byte,
    0x3c as libc::c_int as byte,
    0xf2 as libc::c_int as byte,
    0xf5 as libc::c_int as byte,
    0x5a as libc::c_int as byte,
    0x72 as libc::c_int as byte,
    0xe5 as libc::c_int as byte,
    0xa9 as libc::c_int as byte,
    0xd1 as libc::c_int as byte,
    0xb3 as libc::c_int as byte,
    0x23 as libc::c_int as byte,
    0x82 as libc::c_int as byte,
    0x6f as libc::c_int as byte,
    0x29 as libc::c_int as byte,
    0xcb as libc::c_int as byte,
    0xd1 as libc::c_int as byte,
    0xcc as libc::c_int as byte,
    0x71 as libc::c_int as byte,
    0xfb as libc::c_int as byte,
    0xea as libc::c_int as byte,
    0x92 as libc::c_int as byte,
    0xeb as libc::c_int as byte,
    0x1c as libc::c_int as byte,
    0xca as libc::c_int as byte,
    0x4c as libc::c_int as byte,
    0x70 as libc::c_int as byte,
    0xfe as libc::c_int as byte,
    0x4d as libc::c_int as byte,
    0xc9 as libc::c_int as byte,
    0x67 as libc::c_int as byte,
    0x43 as libc::c_int as byte,
    0x47 as libc::c_int as byte,
    0x94 as libc::c_int as byte,
    0xb9 as libc::c_int as byte,
    0x47 as libc::c_int as byte,
    0xbc as libc::c_int as byte,
    0x3f as libc::c_int as byte,
    0x1 as libc::c_int as byte,
    0xab as libc::c_int as byte,
    0x7b as libc::c_int as byte,
    0xa6 as libc::c_int as byte,
    0xe2 as libc::c_int as byte,
    0x76 as libc::c_int as byte,
    0xef as libc::c_int as byte,
    0x5a as libc::c_int as byte,
    0x7a as libc::c_int as byte,
    0x29 as libc::c_int as byte,
    0xb as libc::c_int as byte,
    0x51 as libc::c_int as byte,
    0x54 as libc::c_int as byte,
    0x67 as libc::c_int as byte,
    0xd8 as libc::c_int as byte,
    0x1c as libc::c_int as byte,
    0x14 as libc::c_int as byte,
    0x3e as libc::c_int as byte,
    0x29 as libc::c_int as byte,
    0xec as libc::c_int as byte,
    0xe9 as libc::c_int as byte,
    0x2d as libc::c_int as byte,
    0x48 as libc::c_int as byte,
    0x67 as libc::c_int as byte,
    0xff as libc::c_int as byte,
    0xed as libc::c_int as byte,
    0x54 as libc::c_int as byte,
    0x4f as libc::c_int as byte,
    0x48 as libc::c_int as byte,
    0xc0 as libc::c_int as byte,
    0xaa as libc::c_int as byte,
    0x61 as libc::c_int as byte,
    0xf7 as libc::c_int as byte,
    0x78 as libc::c_int as byte,
    0x12 as libc::c_int as byte,
    0x3 as libc::c_int as byte,
    0x7a as libc::c_int as byte,
    0x9e as libc::c_int as byte,
    0x8b as libc::c_int as byte,
    0xcf as libc::c_int as byte,
    0x83 as libc::c_int as byte,
    0x7b as libc::c_int as byte,
    0xae as libc::c_int as byte,
    0xca as libc::c_int as byte,
    0x7b as libc::c_int as byte,
    0xd9 as libc::c_int as byte,
    0xe9 as libc::c_int as byte,
    0x53 as libc::c_int as byte,
    0x2a as libc::c_int as byte,
    0xeb as libc::c_int as byte,
    0xd2 as libc::c_int as byte,
    0xd8 as libc::c_int as byte,
    0xcd as libc::c_int as byte,
    0xa3 as libc::c_int as byte,
    0x10 as libc::c_int as byte,
    0x25 as libc::c_int as byte,
    0x78 as libc::c_int as byte,
    0x5a as libc::c_int as byte,
    0xb5 as libc::c_int as byte,
    0x23 as libc::c_int as byte,
    0x6 as libc::c_int as byte,
    0x93 as libc::c_int as byte,
    0xb7 as libc::c_int as byte,
    0x84 as libc::c_int as byte,
    0xd2 as libc::c_int as byte,
    0xbd as libc::c_int as byte,
    0x96 as libc::c_int as byte,
    0x75 as libc::c_int as byte,
    0xa5 as libc::c_int as byte,
    0x5e as libc::c_int as byte,
    0xcf as libc::c_int as byte,
    0x4e as libc::c_int as byte,
    0xe9 as libc::c_int as byte,
    0x50 as libc::c_int as byte,
    0xa1 as libc::c_int as byte,
    0xe6 as libc::c_int as byte,
    0x9d as libc::c_int as byte,
    0xb1 as libc::c_int as byte,
    0xe3 as libc::c_int as byte,
    0x85 as libc::c_int as byte,
    0x66 as libc::c_int as byte,
    0x28 as libc::c_int as byte,
    0x4e as libc::c_int as byte,
    0x43 as libc::c_int as byte,
    0xdc as libc::c_int as byte,
    0x6e as libc::c_int as byte,
    0xbb as libc::c_int as byte,
    0x33 as libc::c_int as byte,
    0x9e as libc::c_int as byte,
    0xf3 as libc::c_int as byte,
    0xd as libc::c_int as byte,
    0 as libc::c_int as byte,
    0xc1 as libc::c_int as byte,
    0xcf as libc::c_int as byte,
    0x67 as libc::c_int as byte,
    0x34 as libc::c_int as byte,
    0x6 as libc::c_int as byte,
    0x7c as libc::c_int as byte,
    0x71 as libc::c_int as byte,
    0xe3 as libc::c_int as byte,
    0x63 as libc::c_int as byte,
    0xb7 as libc::c_int as byte,
    0xb7 as libc::c_int as byte,
    0xdf as libc::c_int as byte,
    0x92 as libc::c_int as byte,
    0xc4 as libc::c_int as byte,
    0xc2 as libc::c_int as byte,
    0x25 as libc::c_int as byte,
    0x5c as libc::c_int as byte,
    0xff as libc::c_int as byte,
    0xc3 as libc::c_int as byte,
    0x6e as libc::c_int as byte,
    0xfc as libc::c_int as byte,
    0xaa as libc::c_int as byte,
    0x1e as libc::c_int as byte,
    0x2a as libc::c_int as byte,
    0x48 as libc::c_int as byte,
    0x11 as libc::c_int as byte,
    0x1c as libc::c_int as byte,
    0x36 as libc::c_int as byte,
    0x68 as libc::c_int as byte,
    0x78 as libc::c_int as byte,
    0x86 as libc::c_int as byte,
    0x79 as libc::c_int as byte,
    0x30 as libc::c_int as byte,
    0xc3 as libc::c_int as byte,
    0xd6 as libc::c_int as byte,
    0xde as libc::c_int as byte,
    0xbc as libc::c_int as byte,
    0x3a as libc::c_int as byte,
    0x2a as libc::c_int as byte,
    0x6d as libc::c_int as byte,
    0x1e as libc::c_int as byte,
    0x46 as libc::c_int as byte,
    0xdd as libc::c_int as byte,
    0xe0 as libc::c_int as byte,
    0x80 as libc::c_int as byte,
    0x1e as libc::c_int as byte,
    0x44 as libc::c_int as byte,
    0x3b as libc::c_int as byte,
    0x6f as libc::c_int as byte,
    0xaf as libc::c_int as byte,
    0x31 as libc::c_int as byte,
    0xda as libc::c_int as byte,
    0xa2 as libc::c_int as byte,
    0xbd as libc::c_int as byte,
    0x77 as libc::c_int as byte,
    0x6 as libc::c_int as byte,
    0x56 as libc::c_int as byte,
    0xc0 as libc::c_int as byte,
    0xb7 as libc::c_int as byte,
    0x92 as libc::c_int as byte,
    0x4b as libc::c_int as byte,
    0x37 as libc::c_int as byte,
    0xc0 as libc::c_int as byte,
    0xfc as libc::c_int as byte,
    0xc2 as libc::c_int as byte,
    0xd5 as libc::c_int as byte,
    0xfb as libc::c_int as byte,
    0xa8 as libc::c_int as byte,
    0xda as libc::c_int as byte,
    0xf5 as libc::c_int as byte,
    0x57 as libc::c_int as byte,
    0xa8 as libc::c_int as byte,
    0x18 as libc::c_int as byte,
    0xc0 as libc::c_int as byte,
    0xdf as libc::c_int as byte,
    0xe7 as libc::c_int as byte,
    0xaa as libc::c_int as byte,
    0x2a as libc::c_int as byte,
    0xe0 as libc::c_int as byte,
    0x7c as libc::c_int as byte,
    0x6f as libc::c_int as byte,
    0x77 as libc::c_int as byte,
    0xb1 as libc::c_int as byte,
    0x26 as libc::c_int as byte,
    0xba as libc::c_int as byte,
    0xf9 as libc::c_int as byte,
    0x2e as libc::c_int as byte,
    0x1d as libc::c_int as byte,
    0x16 as libc::c_int as byte,
    0xcb as libc::c_int as byte,
    0xb8 as libc::c_int as byte,
    0xa2 as libc::c_int as byte,
    0x44 as libc::c_int as byte,
    0xd5 as libc::c_int as byte,
    0x2f as libc::c_int as byte,
    0x1a as libc::c_int as byte,
    0x79 as libc::c_int as byte,
    0x74 as libc::c_int as byte,
    0x87 as libc::c_int as byte,
    0x4b as libc::c_int as byte,
    0 as libc::c_int as byte,
    0xc9 as libc::c_int as byte,
    0x4a as libc::c_int as byte,
    0x3a as libc::c_int as byte,
    0x65 as libc::c_int as byte,
    0x8f as libc::c_int as byte,
    0xe6 as libc::c_int as byte,
    0x5d as libc::c_int as byte,
    0xe5 as libc::c_int as byte,
    0xa as libc::c_int as byte,
    0x77 as libc::c_int as byte,
    0xd8 as libc::c_int as byte,
    0x1a as libc::c_int as byte,
    0x14 as libc::c_int as byte,
    0x41 as libc::c_int as byte,
    0x75 as libc::c_int as byte,
    0xb1 as libc::c_int as byte,
    0xe2 as libc::c_int as byte,
    0x50 as libc::c_int as byte,
    0x2c as libc::c_int as byte,
    0x93 as libc::c_int as byte,
    0x38 as libc::c_int as byte,
    0x2b as libc::c_int as byte,
    0x6d as libc::c_int as byte,
    0xf3 as libc::c_int as byte,
    0xf6 as libc::c_int as byte,
    0xdb as libc::c_int as byte,
    0x1f as libc::c_int as byte,
    0xcd as libc::c_int as byte,
    0xff as libc::c_int as byte,
    0x14 as libc::c_int as byte,
    0x70 as libc::c_int as byte,
    0xe7 as libc::c_int as byte,
    0x16 as libc::c_int as byte,
    0xe8 as libc::c_int as byte,
    0x3d as libc::c_int as byte,
    0xf0 as libc::c_int as byte,
    0xe3 as libc::c_int as byte,
    0xbc as libc::c_int as byte,
    0x5e as libc::c_int as byte,
    0xb6 as libc::c_int as byte,
    0x3f as libc::c_int as byte,
    0xcc as libc::c_int as byte,
    0x81 as libc::c_int as byte,
    0x24 as libc::c_int as byte,
    0x67 as libc::c_int as byte,
    0xf3 as libc::c_int as byte,
    0x97 as libc::c_int as byte,
    0x3b as libc::c_int as byte,
    0xfe as libc::c_int as byte,
    0x3a as libc::c_int as byte,
    0x96 as libc::c_int as byte,
    0x85 as libc::c_int as byte,
    0xdf as libc::c_int as byte,
    0xe4 as libc::c_int as byte,
    0x6e as libc::c_int as byte,
    0x3c as libc::c_int as byte,
    0x85 as libc::c_int as byte,
    0x5 as libc::c_int as byte,
    0xe as libc::c_int as byte,
    0xa3 as libc::c_int as byte,
    0x2b as libc::c_int as byte,
    0x7 as libc::c_int as byte,
    0xc8 as libc::c_int as byte,
    0xbf as libc::c_int as byte,
    0xe5 as libc::c_int as byte,
    0x13 as libc::c_int as byte,
    0x82 as libc::c_int as byte,
    0x62 as libc::c_int as byte,
    0x8 as libc::c_int as byte,
    0x61 as libc::c_int as byte,
    0x69 as libc::c_int as byte,
    0x4b as libc::c_int as byte,
    0x47 as libc::c_int as byte,
    0x62 as libc::c_int as byte,
    0x73 as libc::c_int as byte,
    0x44 as libc::c_int as byte,
    0x64 as libc::c_int as byte,
    0x8e as libc::c_int as byte,
    0xe2 as libc::c_int as byte,
    0x91 as libc::c_int as byte,
    0xa6 as libc::c_int as byte,
    0x9a as libc::c_int as byte,
    0xb7 as libc::c_int as byte,
    0xe9 as libc::c_int as byte,
    0x4 as libc::c_int as byte,
    0xb6 as libc::c_int as byte,
    0x54 as libc::c_int as byte,
    0xc as libc::c_int as byte,
    0xc5 as libc::c_int as byte,
    0xa9 as libc::c_int as byte,
    0x47 as libc::c_int as byte,
    0xa6 as libc::c_int as byte,
    0xc9 as libc::c_int as byte,
    0x8 as libc::c_int as byte,
    0xfe as libc::c_int as byte,
    0x4e as libc::c_int as byte,
    0xa6 as libc::c_int as byte,
    0xcc as libc::c_int as byte,
    0x8a as libc::c_int as byte,
    0x5b as libc::c_int as byte,
    0x90 as libc::c_int as byte,
    0x6f as libc::c_int as byte,
    0x2b as libc::c_int as byte,
    0x3f as libc::c_int as byte,
    0xb6 as libc::c_int as byte,
    0xa as libc::c_int as byte,
    0x96 as libc::c_int as byte,
    0xc0 as libc::c_int as byte,
    0x78 as libc::c_int as byte,
    0x58 as libc::c_int as byte,
    0x3c as libc::c_int as byte,
    0x76 as libc::c_int as byte,
    0x6d as libc::c_int as byte,
    0x94 as libc::c_int as byte,
    0x1a as libc::c_int as byte,
    0xe4 as libc::c_int as byte,
    0x4e as libc::c_int as byte,
    0xb8 as libc::c_int as byte,
    0x38 as libc::c_int as byte,
    0xbb as libc::c_int as byte,
    0xf5 as libc::c_int as byte,
    0xeb as libc::c_int as byte,
    0x29 as libc::c_int as byte,
    0xd8 as libc::c_int as byte,
    0xb0 as libc::c_int as byte,
    0xf3 as libc::c_int as byte,
    0x15 as libc::c_int as byte,
    0x1e as libc::c_int as byte,
    0x99 as libc::c_int as byte,
    0x96 as libc::c_int as byte,
    0x3c as libc::c_int as byte,
    0x5d as libc::c_int as byte,
    0x63 as libc::c_int as byte,
    0xd5 as libc::c_int as byte,
    0xb1 as libc::c_int as byte,
    0xad as libc::c_int as byte,
    0x52 as libc::c_int as byte,
    0xb8 as libc::c_int as byte,
    0x55 as libc::c_int as byte,
    0x70 as libc::c_int as byte,
    0x75 as libc::c_int as byte,
    0x3e as libc::c_int as byte,
    0x1a as libc::c_int as byte,
    0xd5 as libc::c_int as byte,
    0xda as libc::c_int as byte,
    0xf6 as libc::c_int as byte,
    0x7a as libc::c_int as byte,
    0x48 as libc::c_int as byte,
    0x7d as libc::c_int as byte,
    0x44 as libc::c_int as byte,
    0x41 as libc::c_int as byte,
    0xf9 as libc::c_int as byte,
    0x11 as libc::c_int as byte,
    0xce as libc::c_int as byte,
    0xd7 as libc::c_int as byte,
    0xca as libc::c_int as byte,
    0xa5 as libc::c_int as byte,
    0x3d as libc::c_int as byte,
    0x7a as libc::c_int as byte,
    0x79 as libc::c_int as byte,
    0x7e as libc::c_int as byte,
    0x7d as libc::c_int as byte,
    0x25 as libc::c_int as byte,
    0x1b as libc::c_int as byte,
    0x77 as libc::c_int as byte,
    0xbc as libc::c_int as byte,
    0xf7 as libc::c_int as byte,
    0xc7 as libc::c_int as byte,
    0xf as libc::c_int as byte,
    0x84 as libc::c_int as byte,
    0x95 as libc::c_int as byte,
    0x10 as libc::c_int as byte,
    0x92 as libc::c_int as byte,
    0x67 as libc::c_int as byte,
    0x15 as libc::c_int as byte,
    0x11 as libc::c_int as byte,
    0x5a as libc::c_int as byte,
    0x5e as libc::c_int as byte,
    0x41 as libc::c_int as byte,
    0x66 as libc::c_int as byte,
    0xf as libc::c_int as byte,
    0x38 as libc::c_int as byte,
    0x3 as libc::c_int as byte,
    0xb2 as libc::c_int as byte,
    0xf1 as libc::c_int as byte,
    0x5d as libc::c_int as byte,
    0xf8 as libc::c_int as byte,
    0xab as libc::c_int as byte,
    0xc0 as libc::c_int as byte,
    0x2 as libc::c_int as byte,
    0x76 as libc::c_int as byte,
    0x84 as libc::c_int as byte,
    0x28 as libc::c_int as byte,
    0xf4 as libc::c_int as byte,
    0x9d as libc::c_int as byte,
    0x56 as libc::c_int as byte,
    0x46 as libc::c_int as byte,
    0x60 as libc::c_int as byte,
    0x20 as libc::c_int as byte,
    0xdb as libc::c_int as byte,
    0x68 as libc::c_int as byte,
    0xa7 as libc::c_int as byte,
    0xbb as libc::c_int as byte,
    0xee as libc::c_int as byte,
    0xac as libc::c_int as byte,
    0x15 as libc::c_int as byte,
    0x1 as libc::c_int as byte,
    0x2f as libc::c_int as byte,
    0x20 as libc::c_int as byte,
    0x9 as libc::c_int as byte,
    0xdb as libc::c_int as byte,
    0xc0 as libc::c_int as byte,
    0x16 as libc::c_int as byte,
    0xa1 as libc::c_int as byte,
    0x89 as libc::c_int as byte,
    0xf9 as libc::c_int as byte,
    0x94 as libc::c_int as byte,
    0x59 as libc::c_int as byte,
    0 as libc::c_int as byte,
    0xc1 as libc::c_int as byte,
    0x76 as libc::c_int as byte,
    0xbf as libc::c_int as byte,
    0xc1 as libc::c_int as byte,
    0x4d as libc::c_int as byte,
    0x5d as libc::c_int as byte,
    0x2d as libc::c_int as byte,
    0xa9 as libc::c_int as byte,
    0x85 as libc::c_int as byte,
    0x2c as libc::c_int as byte,
    0xd6 as libc::c_int as byte,
    0xd3 as libc::c_int as byte,
    0x14 as libc::c_int as byte,
    0xcc as libc::c_int as byte,
    0x2 as libc::c_int as byte,
    0xc3 as libc::c_int as byte,
    0xc2 as libc::c_int as byte,
    0xfa as libc::c_int as byte,
    0x6b as libc::c_int as byte,
    0xb7 as libc::c_int as byte,
    0xa6 as libc::c_int as byte,
    0xef as libc::c_int as byte,
    0xdd as libc::c_int as byte,
    0x12 as libc::c_int as byte,
    0x26 as libc::c_int as byte,
    0xa4 as libc::c_int as byte,
    0x63 as libc::c_int as byte,
    0xe3 as libc::c_int as byte,
    0x62 as libc::c_int as byte,
    0xbd as libc::c_int as byte,
    0x56 as libc::c_int as byte,
    0x8a as libc::c_int as byte,
    0x52 as libc::c_int as byte,
    0x2b as libc::c_int as byte,
    0xb9 as libc::c_int as byte,
    0xdf as libc::c_int as byte,
    0x9 as libc::c_int as byte,
    0xbc as libc::c_int as byte,
    0xe as libc::c_int as byte,
    0x97 as libc::c_int as byte,
    0xa9 as libc::c_int as byte,
    0xb0 as libc::c_int as byte,
    0x82 as libc::c_int as byte,
    0x46 as libc::c_int as byte,
    0x8 as libc::c_int as byte,
    0xd5 as libc::c_int as byte,
    0x1a as libc::c_int as byte,
    0x8e as libc::c_int as byte,
    0x1b as libc::c_int as byte,
    0xa7 as libc::c_int as byte,
    0x90 as libc::c_int as byte,
    0x98 as libc::c_int as byte,
    0xb9 as libc::c_int as byte,
    0xbb as libc::c_int as byte,
    0x3c as libc::c_int as byte,
    0x17 as libc::c_int as byte,
    0x9a as libc::c_int as byte,
    0xf2 as libc::c_int as byte,
    0x82 as libc::c_int as byte,
    0xba as libc::c_int as byte,
    0x64 as libc::c_int as byte,
    0xa as libc::c_int as byte,
    0x7f as libc::c_int as byte,
    0xca as libc::c_int as byte,
    0x5a as libc::c_int as byte,
    0x8c as libc::c_int as byte,
    0x7c as libc::c_int as byte,
    0xd3 as libc::c_int as byte,
    0x79 as libc::c_int as byte,
    0x9 as libc::c_int as byte,
    0x5b as libc::c_int as byte,
    0x26 as libc::c_int as byte,
    0xbb as libc::c_int as byte,
    0xbd as libc::c_int as byte,
    0x25 as libc::c_int as byte,
    0xdf as libc::c_int as byte,
    0x3d as libc::c_int as byte,
    0x6f as libc::c_int as byte,
    0x9a as libc::c_int as byte,
    0x8f as libc::c_int as byte,
    0xee as libc::c_int as byte,
    0x21 as libc::c_int as byte,
    0x66 as libc::c_int as byte,
    0xb0 as libc::c_int as byte,
    0x8d as libc::c_int as byte,
    0x84 as libc::c_int as byte,
    0x4c as libc::c_int as byte,
    0x91 as libc::c_int as byte,
    0x45 as libc::c_int as byte,
    0xd4 as libc::c_int as byte,
    0x77 as libc::c_int as byte,
    0x4f as libc::c_int as byte,
    0xb3 as libc::c_int as byte,
    0x8c as libc::c_int as byte,
    0xbc as libc::c_int as byte,
    0xa8 as libc::c_int as byte,
    0x99 as libc::c_int as byte,
    0xaa as libc::c_int as byte,
    0x19 as libc::c_int as byte,
    0x53 as libc::c_int as byte,
    0x7c as libc::c_int as byte,
    0x2 as libc::c_int as byte,
    0x87 as libc::c_int as byte,
    0xbb as libc::c_int as byte,
    0xb as libc::c_int as byte,
    0x7c as libc::c_int as byte,
    0x1a as libc::c_int as byte,
    0x2d as libc::c_int as byte,
    0xdf as libc::c_int as byte,
    0x48 as libc::c_int as byte,
    0x44 as libc::c_int as byte,
    0x6 as libc::c_int as byte,
    0xd6 as libc::c_int as byte,
    0x7d as libc::c_int as byte,
    0xc as libc::c_int as byte,
    0x2d as libc::c_int as byte,
    0x35 as libc::c_int as byte,
    0x76 as libc::c_int as byte,
    0xae as libc::c_int as byte,
    0xc4 as libc::c_int as byte,
    0x5f as libc::c_int as byte,
    0x71 as libc::c_int as byte,
    0x85 as libc::c_int as byte,
    0x97 as libc::c_int as byte,
    0xc4 as libc::c_int as byte,
    0x3d as libc::c_int as byte,
    0xef as libc::c_int as byte,
    0x52 as libc::c_int as byte,
    0xbe as libc::c_int as byte,
    0 as libc::c_int as byte,
    0xe4 as libc::c_int as byte,
    0xcd as libc::c_int as byte,
    0x49 as libc::c_int as byte,
    0xd1 as libc::c_int as byte,
    0xd1 as libc::c_int as byte,
    0x1c as libc::c_int as byte,
    0x3c as libc::c_int as byte,
    0xd0 as libc::c_int as byte,
    0x1c as libc::c_int as byte,
    0x42 as libc::c_int as byte,
    0xaf as libc::c_int as byte,
    0xd4 as libc::c_int as byte,
    0xbd as libc::c_int as byte,
    0x58 as libc::c_int as byte,
    0x34 as libc::c_int as byte,
    0x7 as libc::c_int as byte,
    0x32 as libc::c_int as byte,
    0xee as libc::c_int as byte,
    0xb9 as libc::c_int as byte,
    0xb5 as libc::c_int as byte,
    0xea as libc::c_int as byte,
    0xff as libc::c_int as byte,
    0xd7 as libc::c_int as byte,
    0x8c as libc::c_int as byte,
    0xd as libc::c_int as byte,
    0x2e as libc::c_int as byte,
    0x2f as libc::c_int as byte,
    0xaf as libc::c_int as byte,
    0x87 as libc::c_int as byte,
    0xbb as libc::c_int as byte,
    0xe6 as libc::c_int as byte,
    0x52 as libc::c_int as byte,
    0x71 as libc::c_int as byte,
    0x22 as libc::c_int as byte,
    0xf5 as libc::c_int as byte,
    0x25 as libc::c_int as byte,
    0x17 as libc::c_int as byte,
    0xa1 as libc::c_int as byte,
    0x82 as libc::c_int as byte,
    0x4 as libc::c_int as byte,
    0xc2 as libc::c_int as byte,
    0x4a as libc::c_int as byte,
    0xbd as libc::c_int as byte,
    0x57 as libc::c_int as byte,
    0xc6 as libc::c_int as byte,
    0xab as libc::c_int as byte,
    0xc8 as libc::c_int as byte,
    0x35 as libc::c_int as byte,
    0xc as libc::c_int as byte,
    0x3c as libc::c_int as byte,
    0xd9 as libc::c_int as byte,
    0xc2 as libc::c_int as byte,
    0x43 as libc::c_int as byte,
    0xdb as libc::c_int as byte,
    0x27 as libc::c_int as byte,
    0x92 as libc::c_int as byte,
    0xcf as libc::c_int as byte,
    0xb8 as libc::c_int as byte,
    0x25 as libc::c_int as byte,
    0x60 as libc::c_int as byte,
    0xfa as libc::c_int as byte,
    0x21 as libc::c_int as byte,
    0x3b as libc::c_int as byte,
    0x4 as libc::c_int as byte,
    0x52 as libc::c_int as byte,
    0xc8 as libc::c_int as byte,
    0x96 as libc::c_int as byte,
    0xba as libc::c_int as byte,
    0x74 as libc::c_int as byte,
    0xe3 as libc::c_int as byte,
    0x67 as libc::c_int as byte,
    0x3e as libc::c_int as byte,
    0x8e as libc::c_int as byte,
    0x8d as libc::c_int as byte,
    0x61 as libc::c_int as byte,
    0x90 as libc::c_int as byte,
    0x92 as libc::c_int as byte,
    0x59 as libc::c_int as byte,
    0xb6 as libc::c_int as byte,
    0x1a as libc::c_int as byte,
    0x1c as libc::c_int as byte,
    0x5e as libc::c_int as byte,
    0x21 as libc::c_int as byte,
    0xc1 as libc::c_int as byte,
    0x65 as libc::c_int as byte,
    0xe5 as libc::c_int as byte,
    0xa6 as libc::c_int as byte,
    0x34 as libc::c_int as byte,
    0x5 as libc::c_int as byte,
    0x6f as libc::c_int as byte,
    0xc5 as libc::c_int as byte,
    0x60 as libc::c_int as byte,
    0xb1 as libc::c_int as byte,
    0x83 as libc::c_int as byte,
    0xc1 as libc::c_int as byte,
    0xd5 as libc::c_int as byte,
    0xd5 as libc::c_int as byte,
    0xed as libc::c_int as byte,
    0xd9 as libc::c_int as byte,
    0xc7 as libc::c_int as byte,
    0x11 as libc::c_int as byte,
    0x7b as libc::c_int as byte,
    0x49 as libc::c_int as byte,
    0x7a as libc::c_int as byte,
    0xf9 as libc::c_int as byte,
    0xf9 as libc::c_int as byte,
    0x84 as libc::c_int as byte,
    0x47 as libc::c_int as byte,
    0x9b as libc::c_int as byte,
    0xe2 as libc::c_int as byte,
    0xa5 as libc::c_int as byte,
    0x82 as libc::c_int as byte,
    0xe0 as libc::c_int as byte,
    0xc2 as libc::c_int as byte,
    0x88 as libc::c_int as byte,
    0xd0 as libc::c_int as byte,
    0xb2 as libc::c_int as byte,
    0x58 as libc::c_int as byte,
    0x88 as libc::c_int as byte,
    0x7f as libc::c_int as byte,
    0x45 as libc::c_int as byte,
    0x9 as libc::c_int as byte,
    0x67 as libc::c_int as byte,
    0x74 as libc::c_int as byte,
    0x61 as libc::c_int as byte,
    0xbf as libc::c_int as byte,
    0xe6 as libc::c_int as byte,
    0x40 as libc::c_int as byte,
    0xe2 as libc::c_int as byte,
    0x9d as libc::c_int as byte,
    0xc2 as libc::c_int as byte,
    0x47 as libc::c_int as byte,
    0x5 as libc::c_int as byte,
    0x89 as libc::c_int as byte,
    0xed as libc::c_int as byte,
    0xcb as libc::c_int as byte,
    0xbb as libc::c_int as byte,
    0xb7 as libc::c_int as byte,
    0x27 as libc::c_int as byte,
    0xe7 as libc::c_int as byte,
    0xdc as libc::c_int as byte,
    0x7a as libc::c_int as byte,
    0xfd as libc::c_int as byte,
    0xbf as libc::c_int as byte,
    0xa8 as libc::c_int as byte,
    0xd0 as libc::c_int as byte,
    0xaa as libc::c_int as byte,
    0x10 as libc::c_int as byte,
    0x39 as libc::c_int as byte,
    0x3c as libc::c_int as byte,
    0x20 as libc::c_int as byte,
    0xf0 as libc::c_int as byte,
    0xd3 as libc::c_int as byte,
    0x6e as libc::c_int as byte,
    0xb1 as libc::c_int as byte,
    0x72 as libc::c_int as byte,
    0xf8 as libc::c_int as byte,
    0xe6 as libc::c_int as byte,
    0xf as libc::c_int as byte,
    0xef as libc::c_int as byte,
    0x37 as libc::c_int as byte,
    0xe5 as libc::c_int as byte,
    0x9 as libc::c_int as byte,
    0x33 as libc::c_int as byte,
    0x5a as libc::c_int as byte,
    0x83 as libc::c_int as byte,
    0x43 as libc::c_int as byte,
    0x80 as libc::c_int as byte,
    0x4f as libc::c_int as byte,
    0x65 as libc::c_int as byte,
    0x2f as libc::c_int as byte,
    0x7c as libc::c_int as byte,
    0x8c as libc::c_int as byte,
    0x6a as libc::c_int as byte,
    0xa0 as libc::c_int as byte,
    0x82 as libc::c_int as byte,
    0xc as libc::c_int as byte,
    0xd4 as libc::c_int as byte,
    0xd4 as libc::c_int as byte,
    0xfa as libc::c_int as byte,
    0x81 as libc::c_int as byte,
    0x60 as libc::c_int as byte,
    0x3d as libc::c_int as byte,
    0xdf as libc::c_int as byte,
    0x6 as libc::c_int as byte,
    0xf1 as libc::c_int as byte,
    0x5f as libc::c_int as byte,
    0x8 as libc::c_int as byte,
    0xd as libc::c_int as byte,
    0x6d as libc::c_int as byte,
    0x43 as libc::c_int as byte,
    0xf2 as libc::c_int as byte,
    0xe3 as libc::c_int as byte,
    0x11 as libc::c_int as byte,
    0x7d as libc::c_int as byte,
    0x80 as libc::c_int as byte,
    0x32 as libc::c_int as byte,
    0xc5 as libc::c_int as byte,
    0xfb as libc::c_int as byte,
    0xc5 as libc::c_int as byte,
    0xd9 as libc::c_int as byte,
    0x27 as libc::c_int as byte,
    0xec as libc::c_int as byte,
    0xc6 as libc::c_int as byte,
    0x4e as libc::c_int as byte,
    0x65 as libc::c_int as byte,
    0x27 as libc::c_int as byte,
    0x76 as libc::c_int as byte,
    0x87 as libc::c_int as byte,
    0xa6 as libc::c_int as byte,
    0xee as libc::c_int as byte,
    0xee as libc::c_int as byte,
    0xd7 as libc::c_int as byte,
    0x8b as libc::c_int as byte,
    0xd1 as libc::c_int as byte,
    0xa0 as libc::c_int as byte,
    0x5c as libc::c_int as byte,
    0xb0 as libc::c_int as byte,
    0x42 as libc::c_int as byte,
    0x13 as libc::c_int as byte,
    0xe as libc::c_int as byte,
    0x95 as libc::c_int as byte,
    0x4a as libc::c_int as byte,
    0xf2 as libc::c_int as byte,
    0x6 as libc::c_int as byte,
    0xc6 as libc::c_int as byte,
    0x43 as libc::c_int as byte,
    0x33 as libc::c_int as byte,
    0xf4 as libc::c_int as byte,
    0xc7 as libc::c_int as byte,
    0xf8 as libc::c_int as byte,
    0xe7 as libc::c_int as byte,
    0x1f as libc::c_int as byte,
    0xdd as libc::c_int as byte,
    0xe4 as libc::c_int as byte,
    0x46 as libc::c_int as byte,
    0x4a as libc::c_int as byte,
    0x70 as libc::c_int as byte,
    0x39 as libc::c_int as byte,
    0x6c as libc::c_int as byte,
    0xd0 as libc::c_int as byte,
    0xed as libc::c_int as byte,
    0xca as libc::c_int as byte,
    0xbe as libc::c_int as byte,
    0x60 as libc::c_int as byte,
    0x3b as libc::c_int as byte,
    0xd1 as libc::c_int as byte,
    0x7b as libc::c_int as byte,
    0x57 as libc::c_int as byte,
    0x48 as libc::c_int as byte,
    0xe5 as libc::c_int as byte,
    0x3a as libc::c_int as byte,
    0x79 as libc::c_int as byte,
    0xc1 as libc::c_int as byte,
    0x69 as libc::c_int as byte,
    0x33 as libc::c_int as byte,
    0x53 as libc::c_int as byte,
    0x1b as libc::c_int as byte,
    0x80 as libc::c_int as byte,
    0xb8 as libc::c_int as byte,
    0x91 as libc::c_int as byte,
    0x7d as libc::c_int as byte,
    0xb4 as libc::c_int as byte,
    0xf6 as libc::c_int as byte,
    0x17 as libc::c_int as byte,
    0x1a as libc::c_int as byte,
    0x1d as libc::c_int as byte,
    0x5a as libc::c_int as byte,
    0x32 as libc::c_int as byte,
    0xd6 as libc::c_int as byte,
    0xcc as libc::c_int as byte,
    0x71 as libc::c_int as byte,
    0x29 as libc::c_int as byte,
    0x3f as libc::c_int as byte,
    0x28 as libc::c_int as byte,
    0xbb as libc::c_int as byte,
    0xf3 as libc::c_int as byte,
    0x5e as libc::c_int as byte,
    0x71 as libc::c_int as byte,
    0xb8 as libc::c_int as byte,
    0x43 as libc::c_int as byte,
    0xaf as libc::c_int as byte,
    0xf8 as libc::c_int as byte,
    0xb9 as libc::c_int as byte,
    0x64 as libc::c_int as byte,
    0xef as libc::c_int as byte,
    0xc4 as libc::c_int as byte,
    0xa5 as libc::c_int as byte,
    0x6c as libc::c_int as byte,
    0x8 as libc::c_int as byte,
    0x53 as libc::c_int as byte,
    0xc7 as libc::c_int as byte,
    0 as libc::c_int as byte,
    0x10 as libc::c_int as byte,
    0x39 as libc::c_int as byte,
    0x4f as libc::c_int as byte,
    0xdd as libc::c_int as byte,
    0xe4 as libc::c_int as byte,
    0xb6 as libc::c_int as byte,
    0x19 as libc::c_int as byte,
    0x27 as libc::c_int as byte,
    0xfb as libc::c_int as byte,
    0xb8 as libc::c_int as byte,
    0xf5 as libc::c_int as byte,
    0x32 as libc::c_int as byte,
    0x73 as libc::c_int as byte,
    0xe5 as libc::c_int as byte,
    0xcb as libc::c_int as byte,
    0x32 as libc::c_int as byte,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
];
#[no_mangle]
pub unsafe extern "C" fn COM_BlockSequenceCRCByte(
    mut base: *mut byte,
    mut length: libc::c_int,
    mut sequence: libc::c_int,
) -> byte {
    let mut n: libc::c_int = 0;
    let mut p: *mut byte = 0 as *mut byte;
    let mut x: libc::c_int = 0;
    let mut chkb: [byte; 64] = [0; 64];
    let mut crc: libc::c_ushort = 0;
    if sequence < 0 as libc::c_int {
        Sys_Error(
            b"sequence < 0, this shouldn't happen\n\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        );
    }
    p = chktbl
        .as_mut_ptr()
        .offset(
            (sequence as libc::c_ulong)
                .wrapping_rem(
                    (::std::mem::size_of::<[byte; 1024]>() as libc::c_ulong)
                        .wrapping_sub(4 as libc::c_int as libc::c_ulong),
                ) as isize,
        );
    if length > 60 as libc::c_int {
        length = 60 as libc::c_int;
    }
    memcpy(
        chkb.as_mut_ptr() as *mut libc::c_void,
        base as *const libc::c_void,
        length as libc::c_ulong,
    );
    chkb[length as usize] = *p.offset(0 as libc::c_int as isize);
    chkb[(length + 1 as libc::c_int) as usize] = *p.offset(1 as libc::c_int as isize);
    chkb[(length + 2 as libc::c_int) as usize] = *p.offset(2 as libc::c_int as isize);
    chkb[(length + 3 as libc::c_int) as usize] = *p.offset(3 as libc::c_int as isize);
    length += 4 as libc::c_int;
    crc = CRC_Block(chkb.as_mut_ptr(), length);
    x = 0 as libc::c_int;
    n = 0 as libc::c_int;
    while n < length {
        x += chkb[n as usize] as libc::c_int;
        n += 1;
    }
    crc = ((crc as libc::c_int ^ x) & 0xff as libc::c_int) as libc::c_ushort;
    return crc as byte;
}
#[no_mangle]
pub unsafe extern "C" fn frand() -> libc::c_float {
    return ((rand() & 32767 as libc::c_int) as libc::c_double
        * (1.0f64 / 32767 as libc::c_int as libc::c_double)) as libc::c_float;
}
#[no_mangle]
pub unsafe extern "C" fn crand() -> libc::c_float {
    return ((rand() & 32767 as libc::c_int) as libc::c_double
        * (2.0f64 / 32767 as libc::c_int as libc::c_double)
        - 1 as libc::c_int as libc::c_double) as libc::c_float;
}
#[no_mangle]
pub unsafe extern "C" fn Com_Error_f() {
    Com_Error(
        0 as libc::c_int,
        b"%s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        Cmd_Argv(1 as libc::c_int),
    );
}
#[no_mangle]
pub unsafe extern "C" fn Qcommon_Init(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) {
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    if _setjmp(abortframe.as_mut_ptr()) != 0 {
        Sys_Error(
            b"Error during initialization\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    z_chain.prev = &mut z_chain;
    z_chain.next = z_chain.prev;
    COM_InitArgv(argc, argv);
    Swap_Init();
    Cbuf_Init();
    Cmd_Init();
    Cvar_Init();
    Key_Init();
    Cbuf_AddEarlyCommands(false_0);
    Cbuf_Execute();
    FS_InitFilesystem();
    Cbuf_AddText(
        b"exec default.cfg\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    Cbuf_AddText(
        b"exec config.cfg\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    Cbuf_AddEarlyCommands(true_0);
    Cbuf_Execute();
    Cmd_AddCommand(
        b"z_stats\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        Some(Z_Stats_f as unsafe extern "C" fn() -> ()),
    );
    Cmd_AddCommand(
        b"error\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        Some(Com_Error_f as unsafe extern "C" fn() -> ()),
    );
    host_speeds = Cvar_Get(
        b"host_speeds\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    log_stats = Cvar_Get(
        b"log_stats\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    developer = Cvar_Get(
        b"developer\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    timescale = Cvar_Get(
        b"timescale\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    fixedtime = Cvar_Get(
        b"fixedtime\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    logfile_active = Cvar_Get(
        b"logfile\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    showtrace = Cvar_Get(
        b"showtrace\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    dedicated = Cvar_Get(
        b"dedicated\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        8 as libc::c_int,
    );
    s = va(
        b"%4.2f %s %s %s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        3.19f64,
        b"Unknown\0" as *const u8 as *const libc::c_char,
        b"Jan  2 2023\0" as *const u8 as *const libc::c_char,
        b"Linux\0" as *const u8 as *const libc::c_char,
    );
    Cvar_Get(
        b"version\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        s,
        4 as libc::c_int | 8 as libc::c_int,
    );
    if (*dedicated).value != 0. {
        Cmd_AddCommand(
            b"quit\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            Some(Com_Quit as unsafe extern "C" fn() -> ()),
        );
    }
    Sys_Init();
    NET_Init();
    Netchan_Init();
    SV_Init();
    CL_Init();
    if Cbuf_AddLateCommands() as u64 == 0 {
        if (*dedicated).value == 0. {
            Cbuf_AddText(
                b"d1\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
        } else {
            Cbuf_AddText(
                b"dedicated_start\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
        }
        Cbuf_Execute();
    } else {
        SCR_EndLoadingPlaque();
    }
    Com_Printf(
        b"====== Quake2 Initialized ======\n\n\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
}
#[no_mangle]
pub unsafe extern "C" fn Qcommon_Frame(mut msec: libc::c_int) {
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut time_before: libc::c_int = 0;
    let mut time_between: libc::c_int = 0;
    let mut time_after: libc::c_int = 0;
    if _setjmp(abortframe.as_mut_ptr()) != 0 {
        return;
    }
    if (*log_stats).modified as u64 != 0 {
        (*log_stats).modified = false_0;
        if (*log_stats).value != 0. {
            if !log_stats_file.is_null() {
                fclose(log_stats_file);
                log_stats_file = 0 as *mut FILE;
            }
            log_stats_file = fopen(
                b"stats.log\0" as *const u8 as *const libc::c_char,
                b"w\0" as *const u8 as *const libc::c_char,
            );
            if !log_stats_file.is_null() {
                fprintf(
                    log_stats_file,
                    b"entities,dlights,parts,frame time\n\0" as *const u8
                        as *const libc::c_char,
                );
            }
        } else if !log_stats_file.is_null() {
            fclose(log_stats_file);
            log_stats_file = 0 as *mut FILE;
        }
    }
    if (*fixedtime).value != 0. {
        msec = (*fixedtime).value as libc::c_int;
    } else if (*timescale).value != 0. {
        msec = (msec as libc::c_float * (*timescale).value) as libc::c_int;
        if msec < 1 as libc::c_int {
            msec = 1 as libc::c_int;
        }
    }
    if (*showtrace).value != 0. {
        extern "C" {
            static mut c_traces: libc::c_int;
        }
        extern "C" {
            static mut c_brush_traces: libc::c_int;
        }
        extern "C" {
            static mut c_pointcontents: libc::c_int;
        }
        Com_Printf(
            b"%4i traces  %4i points\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            c_traces,
            c_pointcontents,
        );
        c_traces = 0 as libc::c_int;
        c_brush_traces = 0 as libc::c_int;
        c_pointcontents = 0 as libc::c_int;
    }
    loop {
        s = Sys_ConsoleInput();
        if !s.is_null() {
            Cbuf_AddText(
                va(b"%s\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char, s),
            );
        }
        if s.is_null() {
            break;
        }
    }
    Cbuf_Execute();
    if (*host_speeds).value != 0. {
        time_before = Sys_Milliseconds();
    }
    SV_Frame(msec);
    if (*host_speeds).value != 0. {
        time_between = Sys_Milliseconds();
    }
    CL_Frame(msec);
    if (*host_speeds).value != 0. {
        time_after = Sys_Milliseconds();
    }
    if (*host_speeds).value != 0. {
        let mut all: libc::c_int = 0;
        let mut sv: libc::c_int = 0;
        let mut gm: libc::c_int = 0;
        let mut cl: libc::c_int = 0;
        let mut rf: libc::c_int = 0;
        all = time_after - time_before;
        sv = time_between - time_before;
        cl = time_after - time_between;
        gm = time_after_game - time_before_game;
        rf = time_after_ref - time_before_ref;
        sv -= gm;
        cl -= rf;
        Com_Printf(
            b"all:%3i sv:%3i gm:%3i cl:%3i rf:%3i\n\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            all,
            sv,
            gm,
            cl,
            rf,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn Qcommon_Shutdown() {}
