#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
}
#[no_mangle]
pub static mut heventDone: libc::c_int = 0;
#[no_mangle]
pub static mut hfileBuffer: libc::c_int = 0;
#[no_mangle]
pub static mut heventChildSend: libc::c_int = 0;
#[no_mangle]
pub static mut heventParentSend: libc::c_int = 0;
#[no_mangle]
pub static mut hStdout: libc::c_int = 0;
#[no_mangle]
pub static mut hStdin: libc::c_int = 0;
#[no_mangle]
pub static mut ccom_argc: libc::c_int = 0;
#[no_mangle]
pub static mut ccom_argv: *mut *mut libc::c_char = 0 as *const *mut libc::c_char
    as *mut *mut libc::c_char;
#[no_mangle]
pub unsafe extern "C" fn CCheckParm(mut parm: *mut libc::c_char) -> libc::c_int {
    let mut i: libc::c_int = 0;
    i = 1 as libc::c_int;
    while i < ccom_argc {
        if !(*ccom_argv.offset(i as isize)).is_null() {
            if strcmp(parm, *ccom_argv.offset(i as isize)) == 0 {
                return i;
            }
        }
        i += 1;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn DeinitConProc() {}
#[no_mangle]
pub static mut _stdcall: libc::c_uint = 0;
