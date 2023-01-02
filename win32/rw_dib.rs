#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    static mut sww_state: swwstate_t;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct swwstate_t {
    pub hInstance: libc::c_int,
    pub wndproc: *mut libc::c_void,
    pub hDC: libc::c_int,
    pub hWnd: libc::c_int,
    pub hdcDIBSection: libc::c_int,
    pub hDIBSection: libc::c_int,
    pub pDIBBase: *mut libc::c_uchar,
    pub hPal: libc::c_int,
    pub hpalOld: libc::c_int,
    pub oldsyscolors: [libc::c_int; 20],
    pub hinstDDRAW: libc::c_int,
    pub lpDirectDraw: libc::c_int,
    pub lpddsFrontBuffer: libc::c_int,
    pub lpddsBackBuffer: libc::c_int,
    pub lpddsOffScreenBuffer: libc::c_int,
    pub lpddpPalette: libc::c_int,
    pub palettized: libc::c_int,
    pub modex: libc::c_int,
    pub initializing: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dibinfo {
    pub header: libc::c_int,
    pub acolors: [libc::c_int; 256],
}
pub type dibinfo_t = dibinfo;
unsafe extern "C" fn DIB_RestoreSystemColors() {}
unsafe extern "C" fn DIB_SaveSystemColors() {
    let mut i: libc::c_int = 0;
}
