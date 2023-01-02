#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    fn DIB_Shutdown();
    fn DIB_SetPalette(palette: *const libc::c_uchar);
    fn DDRAW_Shutdown();
    fn DDRAW_SetPalette(palette: *const libc::c_uchar);
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
#[no_mangle]
pub static mut sww_state: swwstate_t = swwstate_t {
    hInstance: 0,
    wndproc: 0 as *const libc::c_void as *mut libc::c_void,
    hDC: 0,
    hWnd: 0,
    hdcDIBSection: 0,
    hDIBSection: 0,
    pDIBBase: 0 as *const libc::c_uchar as *mut libc::c_uchar,
    hPal: 0,
    hpalOld: 0,
    oldsyscolors: [0; 20],
    hinstDDRAW: 0,
    lpDirectDraw: 0,
    lpddsFrontBuffer: 0,
    lpddsBackBuffer: 0,
    lpddsOffScreenBuffer: 0,
    lpddpPalette: 0,
    palettized: 0,
    modex: 0,
    initializing: 0,
};
#[no_mangle]
pub unsafe extern "C" fn SWimp_Init(
    mut hInstance: *mut libc::c_void,
    mut wndProc: *mut libc::c_void,
) -> libc::c_int {
    sww_state.wndproc = wndProc;
    panic!("Reached end of non-void function without returning");
}
#[no_mangle]
pub unsafe extern "C" fn Sys_MakeCodeWriteable(
    mut startaddr: libc::c_ulong,
    mut length: libc::c_ulong,
) {}
#[no_mangle]
pub unsafe extern "C" fn Sys_SetFPCW() {}
