#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(c_variadic, register_tool)]
#[no_mangle]
pub static mut re: libc::c_int = 0;
#[no_mangle]
pub static mut win_noalttab: *mut libc::c_int = 0 as *const libc::c_int
    as *mut libc::c_int;
#[no_mangle]
pub static mut vid_gamma: *mut libc::c_int = 0 as *const libc::c_int as *mut libc::c_int;
#[no_mangle]
pub static mut vid_ref: *mut libc::c_int = 0 as *const libc::c_int as *mut libc::c_int;
#[no_mangle]
pub static mut vid_xpos: *mut libc::c_int = 0 as *const libc::c_int as *mut libc::c_int;
#[no_mangle]
pub static mut vid_ypos: *mut libc::c_int = 0 as *const libc::c_int as *mut libc::c_int;
#[no_mangle]
pub static mut vid_fullscreen: *mut libc::c_int = 0 as *const libc::c_int
    as *mut libc::c_int;
#[no_mangle]
pub static mut viddef: libc::c_int = 0;
#[no_mangle]
pub static mut reflib_library: libc::c_int = 0;
#[no_mangle]
pub static mut reflib_active: libc::c_int = 0;
#[no_mangle]
pub static mut cl_hwnd: libc::c_int = 0;
#[no_mangle]
pub static mut WINAPI: libc::c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn VID_Error(
    mut err_level: libc::c_int,
    mut fmt: *mut libc::c_char,
    mut args: ...
) {
    let mut msg: [libc::c_char; 4096] = [0; 4096];
    static mut inupdate: libc::c_int = 0;
    Com_Error(err_level, b"%s\0" as *const u8 as *const libc::c_char, msg.as_mut_ptr());
}
#[no_mangle]
pub static mut scantokey: [libc::c_int; 128] = [0; 128];
#[export_name = "WINAPI"]
pub static mut WINAPI_0: libc::c_int = 0;
#[no_mangle]
pub static mut vid_modes: [libc::c_int; 0] = [];
#[no_mangle]
pub unsafe extern "C" fn VID_GetModeInfo(
    mut width: *mut libc::c_int,
    mut height: *mut libc::c_int,
    mut mode: libc::c_int,
) -> libc::c_int {
    panic!("Reached end of non-void function without returning");
}
#[no_mangle]
pub unsafe extern "C" fn VID_UpdateWindowPosAndSize(
    mut x: libc::c_int,
    mut y: libc::c_int,
) {
    let mut style: libc::c_int = 0;
    let mut w: libc::c_int = 0;
    let mut h: libc::c_int = 0;
}
#[no_mangle]
pub unsafe extern "C" fn VID_NewWindow(mut width: libc::c_int, mut height: libc::c_int) {}
#[no_mangle]
pub unsafe extern "C" fn VID_FreeReflib() {}
#[no_mangle]
pub unsafe extern "C" fn VID_Init() {
    putenv(b"FX_GLIDE_NO_SPLASH=0\0" as *const u8 as *const libc::c_char);
    VID_CheckChanges();
}
