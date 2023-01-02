#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(extern_types, register_tool)]
extern "C" {
    pub type _finddata_t;
    pub type _finddata_t_0;
    fn COM_FilePath(in_0: *mut libc::c_char, out: *mut libc::c_char);
    fn Sys_Error(error: *mut libc::c_char, _: ...);
}
pub type byte = libc::c_uchar;
pub type qboolean = libc::c_uint;
pub const true_0: qboolean = 1;
pub const false_0: qboolean = 0;
#[no_mangle]
pub static mut hunkcount: libc::c_int = 0;
#[no_mangle]
pub static mut membase: *mut byte = 0 as *const byte as *mut byte;
#[no_mangle]
pub static mut hunkmaxsize: libc::c_int = 0;
#[no_mangle]
pub static mut cursize: libc::c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn Hunk_Begin(mut maxsize: libc::c_int) -> *mut libc::c_void {
    cursize = 0 as libc::c_int;
    hunkmaxsize = maxsize;
    if membase.is_null() {
        Sys_Error(
            b"VirtualAlloc reserve failed\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    return membase as *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn Hunk_Alloc(mut size: libc::c_int) -> *mut libc::c_void {
    let mut buf: *mut libc::c_void = 0 as *mut libc::c_void;
    size = size + 31 as libc::c_int & !(31 as libc::c_int);
    if buf.is_null() {
        Sys_Error(
            b"VirtualAlloc commit failed.\n%s\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            buf,
        );
    }
    cursize += size;
    if cursize > hunkmaxsize {
        Sys_Error(
            b"Hunk_Alloc overflow\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    return membase.offset(cursize as isize).offset(-(size as isize))
        as *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn Hunk_End() -> libc::c_int {
    hunkcount += 1;
    return cursize;
}
#[no_mangle]
pub unsafe extern "C" fn Hunk_Free(mut base: *mut libc::c_void) {
    hunkcount -= 1;
}
#[no_mangle]
pub static mut curtime: libc::c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn Sys_Milliseconds() -> libc::c_int {
    static mut base: libc::c_int = 0;
    static mut initialized: qboolean = false_0;
    if initialized as u64 == 0 {
        base = (timeGetTime() as libc::c_uint & 0xffff0000 as libc::c_uint)
            as libc::c_int;
        initialized = true_0;
    }
    curtime = timeGetTime() - base;
    return curtime;
}
#[no_mangle]
pub unsafe extern "C" fn Sys_Mkdir(mut path: *mut libc::c_char) {
    _mkdir(path);
}
#[no_mangle]
pub static mut findbase: [libc::c_char; 128] = [0; 128];
#[no_mangle]
pub static mut findpath: [libc::c_char; 128] = [0; 128];
#[no_mangle]
pub static mut findhandle: libc::c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn Sys_FindClose() {
    if findhandle != -(1 as libc::c_int) {
        _findclose(findhandle);
    }
    findhandle = 0 as libc::c_int;
}
