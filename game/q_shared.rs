#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(c_variadic, label_break_value, register_tool)]
extern "C" {
    fn cos(_: libc::c_double) -> libc::c_double;
    fn sin(_: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn vsprintf(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: ::std::ffi::VaList,
    ) -> libc::c_int;
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
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn Com_Printf(msg: *mut libc::c_char, _: ...);
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
pub union C2RustUnnamed {
    pub f: libc::c_float,
    pub b: [byte; 4],
}
#[no_mangle]
pub static mut vec3_origin: vec3_t = [
    0 as libc::c_int as vec_t,
    0 as libc::c_int as vec_t,
    0 as libc::c_int as vec_t,
];
#[no_mangle]
pub unsafe extern "C" fn RotatePointAroundVector(
    mut dst: *mut vec_t,
    mut dir: *const vec_t,
    mut point: *const vec_t,
    mut degrees: libc::c_float,
) {
    let mut m: [[libc::c_float; 3]; 3] = [[0.; 3]; 3];
    let mut im: [[libc::c_float; 3]; 3] = [[0.; 3]; 3];
    let mut zrot: [[libc::c_float; 3]; 3] = [[0.; 3]; 3];
    let mut tmpmat: [[libc::c_float; 3]; 3] = [[0.; 3]; 3];
    let mut rot: [[libc::c_float; 3]; 3] = [[0.; 3]; 3];
    let mut i_0: libc::c_int = 0;
    let mut vr: vec3_t = [0.; 3];
    let mut vup: vec3_t = [0.; 3];
    let mut vf: vec3_t = [0.; 3];
    vf[0 as libc::c_int as usize] = *dir.offset(0 as libc::c_int as isize);
    vf[1 as libc::c_int as usize] = *dir.offset(1 as libc::c_int as isize);
    vf[2 as libc::c_int as usize] = *dir.offset(2 as libc::c_int as isize);
    PerpendicularVector(vr.as_mut_ptr(), dir);
    CrossProduct(vr.as_mut_ptr(), vf.as_mut_ptr(), vup.as_mut_ptr());
    m[0 as libc::c_int
        as usize][0 as libc::c_int as usize] = vr[0 as libc::c_int as usize];
    m[1 as libc::c_int
        as usize][0 as libc::c_int as usize] = vr[1 as libc::c_int as usize];
    m[2 as libc::c_int
        as usize][0 as libc::c_int as usize] = vr[2 as libc::c_int as usize];
    m[0 as libc::c_int
        as usize][1 as libc::c_int as usize] = vup[0 as libc::c_int as usize];
    m[1 as libc::c_int
        as usize][1 as libc::c_int as usize] = vup[1 as libc::c_int as usize];
    m[2 as libc::c_int
        as usize][1 as libc::c_int as usize] = vup[2 as libc::c_int as usize];
    m[0 as libc::c_int
        as usize][2 as libc::c_int as usize] = vf[0 as libc::c_int as usize];
    m[1 as libc::c_int
        as usize][2 as libc::c_int as usize] = vf[1 as libc::c_int as usize];
    m[2 as libc::c_int
        as usize][2 as libc::c_int as usize] = vf[2 as libc::c_int as usize];
    memcpy(
        im.as_mut_ptr() as *mut libc::c_void,
        m.as_mut_ptr() as *const libc::c_void,
        ::std::mem::size_of::<[[libc::c_float; 3]; 3]>() as libc::c_ulong,
    );
    im[0 as libc::c_int
        as usize][1 as libc::c_int
        as usize] = m[1 as libc::c_int as usize][0 as libc::c_int as usize];
    im[0 as libc::c_int
        as usize][2 as libc::c_int
        as usize] = m[2 as libc::c_int as usize][0 as libc::c_int as usize];
    im[1 as libc::c_int
        as usize][0 as libc::c_int
        as usize] = m[0 as libc::c_int as usize][1 as libc::c_int as usize];
    im[1 as libc::c_int
        as usize][2 as libc::c_int
        as usize] = m[2 as libc::c_int as usize][1 as libc::c_int as usize];
    im[2 as libc::c_int
        as usize][0 as libc::c_int
        as usize] = m[0 as libc::c_int as usize][2 as libc::c_int as usize];
    im[2 as libc::c_int
        as usize][1 as libc::c_int
        as usize] = m[1 as libc::c_int as usize][2 as libc::c_int as usize];
    memset(
        zrot.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<[[libc::c_float; 3]; 3]>() as libc::c_ulong,
    );
    zrot[2 as libc::c_int as usize][2 as libc::c_int as usize] = 1.0f32;
    zrot[1 as libc::c_int
        as usize][1 as libc::c_int
        as usize] = zrot[2 as libc::c_int as usize][2 as libc::c_int as usize];
    zrot[0 as libc::c_int
        as usize][0 as libc::c_int
        as usize] = zrot[1 as libc::c_int as usize][1 as libc::c_int as usize];
    zrot[0 as libc::c_int
        as usize][0 as libc::c_int
        as usize] = cos(
        degrees as libc::c_double * 3.14159265358979323846f64
            / 180.0f32 as libc::c_double,
    ) as libc::c_float;
    zrot[0 as libc::c_int
        as usize][1 as libc::c_int
        as usize] = sin(
        degrees as libc::c_double * 3.14159265358979323846f64
            / 180.0f32 as libc::c_double,
    ) as libc::c_float;
    zrot[1 as libc::c_int
        as usize][0 as libc::c_int
        as usize] = -sin(
        degrees as libc::c_double * 3.14159265358979323846f64
            / 180.0f32 as libc::c_double,
    ) as libc::c_float;
    zrot[1 as libc::c_int
        as usize][1 as libc::c_int
        as usize] = cos(
        degrees as libc::c_double * 3.14159265358979323846f64
            / 180.0f32 as libc::c_double,
    ) as libc::c_float;
    R_ConcatRotations(m.as_mut_ptr(), zrot.as_mut_ptr(), tmpmat.as_mut_ptr());
    R_ConcatRotations(tmpmat.as_mut_ptr(), im.as_mut_ptr(), rot.as_mut_ptr());
    i_0 = 0 as libc::c_int;
    while i_0 < 3 as libc::c_int {
        *dst
            .offset(
                i_0 as isize,
            ) = rot[i_0 as usize][0 as libc::c_int as usize]
            * *point.offset(0 as libc::c_int as isize)
            + rot[i_0 as usize][1 as libc::c_int as usize]
                * *point.offset(1 as libc::c_int as isize)
            + rot[i_0 as usize][2 as libc::c_int as usize]
                * *point.offset(2 as libc::c_int as isize);
        i_0 += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn AngleVectors(
    mut angles: *mut vec_t,
    mut forward: *mut vec_t,
    mut right: *mut vec_t,
    mut up: *mut vec_t,
) {
    let mut angle: libc::c_float = 0.;
    static mut sr: libc::c_float = 0.;
    static mut sp: libc::c_float = 0.;
    static mut sy: libc::c_float = 0.;
    static mut cr: libc::c_float = 0.;
    static mut cp: libc::c_float = 0.;
    static mut cy: libc::c_float = 0.;
    angle = (*angles.offset(1 as libc::c_int as isize) as libc::c_double
        * (3.14159265358979323846f64 * 2 as libc::c_int as libc::c_double
            / 360 as libc::c_int as libc::c_double)) as libc::c_float;
    sy = sin(angle as libc::c_double) as libc::c_float;
    cy = cos(angle as libc::c_double) as libc::c_float;
    angle = (*angles.offset(0 as libc::c_int as isize) as libc::c_double
        * (3.14159265358979323846f64 * 2 as libc::c_int as libc::c_double
            / 360 as libc::c_int as libc::c_double)) as libc::c_float;
    sp = sin(angle as libc::c_double) as libc::c_float;
    cp = cos(angle as libc::c_double) as libc::c_float;
    angle = (*angles.offset(2 as libc::c_int as isize) as libc::c_double
        * (3.14159265358979323846f64 * 2 as libc::c_int as libc::c_double
            / 360 as libc::c_int as libc::c_double)) as libc::c_float;
    sr = sin(angle as libc::c_double) as libc::c_float;
    cr = cos(angle as libc::c_double) as libc::c_float;
    if !forward.is_null() {
        *forward.offset(0 as libc::c_int as isize) = cp * cy;
        *forward.offset(1 as libc::c_int as isize) = cp * sy;
        *forward.offset(2 as libc::c_int as isize) = -sp;
    }
    if !right.is_null() {
        *right
            .offset(
                0 as libc::c_int as isize,
            ) = -(1 as libc::c_int) as libc::c_float * sr * sp * cy
            + -(1 as libc::c_int) as libc::c_float * cr * -sy;
        *right
            .offset(
                1 as libc::c_int as isize,
            ) = -(1 as libc::c_int) as libc::c_float * sr * sp * sy
            + -(1 as libc::c_int) as libc::c_float * cr * cy;
        *right
            .offset(
                2 as libc::c_int as isize,
            ) = -(1 as libc::c_int) as libc::c_float * sr * cp;
    }
    if !up.is_null() {
        *up.offset(0 as libc::c_int as isize) = cr * sp * cy + -sr * -sy;
        *up.offset(1 as libc::c_int as isize) = cr * sp * sy + -sr * cy;
        *up.offset(2 as libc::c_int as isize) = cr * cp;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ProjectPointOnPlane(
    mut dst: *mut vec_t,
    mut p: *const vec_t,
    mut normal: *const vec_t,
) {
    let mut d: libc::c_float = 0.;
    let mut n: vec3_t = [0.; 3];
    let mut inv_denom: libc::c_float = 0.;
    inv_denom = 1.0f32
        / (*normal.offset(0 as libc::c_int as isize)
            * *normal.offset(0 as libc::c_int as isize)
            + *normal.offset(1 as libc::c_int as isize)
                * *normal.offset(1 as libc::c_int as isize)
            + *normal.offset(2 as libc::c_int as isize)
                * *normal.offset(2 as libc::c_int as isize));
    d = (*normal.offset(0 as libc::c_int as isize) * *p.offset(0 as libc::c_int as isize)
        + *normal.offset(1 as libc::c_int as isize)
            * *p.offset(1 as libc::c_int as isize)
        + *normal.offset(2 as libc::c_int as isize)
            * *p.offset(2 as libc::c_int as isize)) * inv_denom;
    n[0 as libc::c_int as usize] = *normal.offset(0 as libc::c_int as isize) * inv_denom;
    n[1 as libc::c_int as usize] = *normal.offset(1 as libc::c_int as isize) * inv_denom;
    n[2 as libc::c_int as usize] = *normal.offset(2 as libc::c_int as isize) * inv_denom;
    *dst
        .offset(
            0 as libc::c_int as isize,
        ) = *p.offset(0 as libc::c_int as isize) - d * n[0 as libc::c_int as usize];
    *dst
        .offset(
            1 as libc::c_int as isize,
        ) = *p.offset(1 as libc::c_int as isize) - d * n[1 as libc::c_int as usize];
    *dst
        .offset(
            2 as libc::c_int as isize,
        ) = *p.offset(2 as libc::c_int as isize) - d * n[2 as libc::c_int as usize];
}
#[no_mangle]
pub unsafe extern "C" fn PerpendicularVector(
    mut dst: *mut vec_t,
    mut src: *const vec_t,
) {
    let mut pos: libc::c_int = 0;
    let mut i_0: libc::c_int = 0;
    let mut minelem: libc::c_float = 1.0f32;
    let mut tempvec: vec3_t = [0.; 3];
    pos = 0 as libc::c_int;
    i_0 = 0 as libc::c_int;
    while i_0 < 3 as libc::c_int {
        if fabs(*src.offset(i_0 as isize) as libc::c_double) < minelem as libc::c_double
        {
            pos = i_0;
            minelem = fabs(*src.offset(i_0 as isize) as libc::c_double) as libc::c_float;
        }
        i_0 += 1;
    }
    tempvec[2 as libc::c_int as usize] = 0.0f32;
    tempvec[1 as libc::c_int as usize] = tempvec[2 as libc::c_int as usize];
    tempvec[0 as libc::c_int as usize] = tempvec[1 as libc::c_int as usize];
    tempvec[pos as usize] = 1.0f32;
    ProjectPointOnPlane(dst, tempvec.as_mut_ptr() as *const vec_t, src);
    VectorNormalize(dst);
}
#[no_mangle]
pub unsafe extern "C" fn R_ConcatRotations(
    mut in1: *mut [libc::c_float; 3],
    mut in2: *mut [libc::c_float; 3],
    mut out: *mut [libc::c_float; 3],
) {
    (*out
        .offset(
            0 as libc::c_int as isize,
        ))[0 as libc::c_int
        as usize] = (*in1.offset(0 as libc::c_int as isize))[0 as libc::c_int as usize]
        * (*in2.offset(0 as libc::c_int as isize))[0 as libc::c_int as usize]
        + (*in1.offset(0 as libc::c_int as isize))[1 as libc::c_int as usize]
            * (*in2.offset(1 as libc::c_int as isize))[0 as libc::c_int as usize]
        + (*in1.offset(0 as libc::c_int as isize))[2 as libc::c_int as usize]
            * (*in2.offset(2 as libc::c_int as isize))[0 as libc::c_int as usize];
    (*out
        .offset(
            0 as libc::c_int as isize,
        ))[1 as libc::c_int
        as usize] = (*in1.offset(0 as libc::c_int as isize))[0 as libc::c_int as usize]
        * (*in2.offset(0 as libc::c_int as isize))[1 as libc::c_int as usize]
        + (*in1.offset(0 as libc::c_int as isize))[1 as libc::c_int as usize]
            * (*in2.offset(1 as libc::c_int as isize))[1 as libc::c_int as usize]
        + (*in1.offset(0 as libc::c_int as isize))[2 as libc::c_int as usize]
            * (*in2.offset(2 as libc::c_int as isize))[1 as libc::c_int as usize];
    (*out
        .offset(
            0 as libc::c_int as isize,
        ))[2 as libc::c_int
        as usize] = (*in1.offset(0 as libc::c_int as isize))[0 as libc::c_int as usize]
        * (*in2.offset(0 as libc::c_int as isize))[2 as libc::c_int as usize]
        + (*in1.offset(0 as libc::c_int as isize))[1 as libc::c_int as usize]
            * (*in2.offset(1 as libc::c_int as isize))[2 as libc::c_int as usize]
        + (*in1.offset(0 as libc::c_int as isize))[2 as libc::c_int as usize]
            * (*in2.offset(2 as libc::c_int as isize))[2 as libc::c_int as usize];
    (*out
        .offset(
            1 as libc::c_int as isize,
        ))[0 as libc::c_int
        as usize] = (*in1.offset(1 as libc::c_int as isize))[0 as libc::c_int as usize]
        * (*in2.offset(0 as libc::c_int as isize))[0 as libc::c_int as usize]
        + (*in1.offset(1 as libc::c_int as isize))[1 as libc::c_int as usize]
            * (*in2.offset(1 as libc::c_int as isize))[0 as libc::c_int as usize]
        + (*in1.offset(1 as libc::c_int as isize))[2 as libc::c_int as usize]
            * (*in2.offset(2 as libc::c_int as isize))[0 as libc::c_int as usize];
    (*out
        .offset(
            1 as libc::c_int as isize,
        ))[1 as libc::c_int
        as usize] = (*in1.offset(1 as libc::c_int as isize))[0 as libc::c_int as usize]
        * (*in2.offset(0 as libc::c_int as isize))[1 as libc::c_int as usize]
        + (*in1.offset(1 as libc::c_int as isize))[1 as libc::c_int as usize]
            * (*in2.offset(1 as libc::c_int as isize))[1 as libc::c_int as usize]
        + (*in1.offset(1 as libc::c_int as isize))[2 as libc::c_int as usize]
            * (*in2.offset(2 as libc::c_int as isize))[1 as libc::c_int as usize];
    (*out
        .offset(
            1 as libc::c_int as isize,
        ))[2 as libc::c_int
        as usize] = (*in1.offset(1 as libc::c_int as isize))[0 as libc::c_int as usize]
        * (*in2.offset(0 as libc::c_int as isize))[2 as libc::c_int as usize]
        + (*in1.offset(1 as libc::c_int as isize))[1 as libc::c_int as usize]
            * (*in2.offset(1 as libc::c_int as isize))[2 as libc::c_int as usize]
        + (*in1.offset(1 as libc::c_int as isize))[2 as libc::c_int as usize]
            * (*in2.offset(2 as libc::c_int as isize))[2 as libc::c_int as usize];
    (*out
        .offset(
            2 as libc::c_int as isize,
        ))[0 as libc::c_int
        as usize] = (*in1.offset(2 as libc::c_int as isize))[0 as libc::c_int as usize]
        * (*in2.offset(0 as libc::c_int as isize))[0 as libc::c_int as usize]
        + (*in1.offset(2 as libc::c_int as isize))[1 as libc::c_int as usize]
            * (*in2.offset(1 as libc::c_int as isize))[0 as libc::c_int as usize]
        + (*in1.offset(2 as libc::c_int as isize))[2 as libc::c_int as usize]
            * (*in2.offset(2 as libc::c_int as isize))[0 as libc::c_int as usize];
    (*out
        .offset(
            2 as libc::c_int as isize,
        ))[1 as libc::c_int
        as usize] = (*in1.offset(2 as libc::c_int as isize))[0 as libc::c_int as usize]
        * (*in2.offset(0 as libc::c_int as isize))[1 as libc::c_int as usize]
        + (*in1.offset(2 as libc::c_int as isize))[1 as libc::c_int as usize]
            * (*in2.offset(1 as libc::c_int as isize))[1 as libc::c_int as usize]
        + (*in1.offset(2 as libc::c_int as isize))[2 as libc::c_int as usize]
            * (*in2.offset(2 as libc::c_int as isize))[1 as libc::c_int as usize];
    (*out
        .offset(
            2 as libc::c_int as isize,
        ))[2 as libc::c_int
        as usize] = (*in1.offset(2 as libc::c_int as isize))[0 as libc::c_int as usize]
        * (*in2.offset(0 as libc::c_int as isize))[2 as libc::c_int as usize]
        + (*in1.offset(2 as libc::c_int as isize))[1 as libc::c_int as usize]
            * (*in2.offset(1 as libc::c_int as isize))[2 as libc::c_int as usize]
        + (*in1.offset(2 as libc::c_int as isize))[2 as libc::c_int as usize]
            * (*in2.offset(2 as libc::c_int as isize))[2 as libc::c_int as usize];
}
#[no_mangle]
pub unsafe extern "C" fn R_ConcatTransforms(
    mut in1: *mut [libc::c_float; 4],
    mut in2: *mut [libc::c_float; 4],
    mut out: *mut [libc::c_float; 4],
) {
    (*out
        .offset(
            0 as libc::c_int as isize,
        ))[0 as libc::c_int
        as usize] = (*in1.offset(0 as libc::c_int as isize))[0 as libc::c_int as usize]
        * (*in2.offset(0 as libc::c_int as isize))[0 as libc::c_int as usize]
        + (*in1.offset(0 as libc::c_int as isize))[1 as libc::c_int as usize]
            * (*in2.offset(1 as libc::c_int as isize))[0 as libc::c_int as usize]
        + (*in1.offset(0 as libc::c_int as isize))[2 as libc::c_int as usize]
            * (*in2.offset(2 as libc::c_int as isize))[0 as libc::c_int as usize];
    (*out
        .offset(
            0 as libc::c_int as isize,
        ))[1 as libc::c_int
        as usize] = (*in1.offset(0 as libc::c_int as isize))[0 as libc::c_int as usize]
        * (*in2.offset(0 as libc::c_int as isize))[1 as libc::c_int as usize]
        + (*in1.offset(0 as libc::c_int as isize))[1 as libc::c_int as usize]
            * (*in2.offset(1 as libc::c_int as isize))[1 as libc::c_int as usize]
        + (*in1.offset(0 as libc::c_int as isize))[2 as libc::c_int as usize]
            * (*in2.offset(2 as libc::c_int as isize))[1 as libc::c_int as usize];
    (*out
        .offset(
            0 as libc::c_int as isize,
        ))[2 as libc::c_int
        as usize] = (*in1.offset(0 as libc::c_int as isize))[0 as libc::c_int as usize]
        * (*in2.offset(0 as libc::c_int as isize))[2 as libc::c_int as usize]
        + (*in1.offset(0 as libc::c_int as isize))[1 as libc::c_int as usize]
            * (*in2.offset(1 as libc::c_int as isize))[2 as libc::c_int as usize]
        + (*in1.offset(0 as libc::c_int as isize))[2 as libc::c_int as usize]
            * (*in2.offset(2 as libc::c_int as isize))[2 as libc::c_int as usize];
    (*out
        .offset(
            0 as libc::c_int as isize,
        ))[3 as libc::c_int
        as usize] = (*in1.offset(0 as libc::c_int as isize))[0 as libc::c_int as usize]
        * (*in2.offset(0 as libc::c_int as isize))[3 as libc::c_int as usize]
        + (*in1.offset(0 as libc::c_int as isize))[1 as libc::c_int as usize]
            * (*in2.offset(1 as libc::c_int as isize))[3 as libc::c_int as usize]
        + (*in1.offset(0 as libc::c_int as isize))[2 as libc::c_int as usize]
            * (*in2.offset(2 as libc::c_int as isize))[3 as libc::c_int as usize]
        + (*in1.offset(0 as libc::c_int as isize))[3 as libc::c_int as usize];
    (*out
        .offset(
            1 as libc::c_int as isize,
        ))[0 as libc::c_int
        as usize] = (*in1.offset(1 as libc::c_int as isize))[0 as libc::c_int as usize]
        * (*in2.offset(0 as libc::c_int as isize))[0 as libc::c_int as usize]
        + (*in1.offset(1 as libc::c_int as isize))[1 as libc::c_int as usize]
            * (*in2.offset(1 as libc::c_int as isize))[0 as libc::c_int as usize]
        + (*in1.offset(1 as libc::c_int as isize))[2 as libc::c_int as usize]
            * (*in2.offset(2 as libc::c_int as isize))[0 as libc::c_int as usize];
    (*out
        .offset(
            1 as libc::c_int as isize,
        ))[1 as libc::c_int
        as usize] = (*in1.offset(1 as libc::c_int as isize))[0 as libc::c_int as usize]
        * (*in2.offset(0 as libc::c_int as isize))[1 as libc::c_int as usize]
        + (*in1.offset(1 as libc::c_int as isize))[1 as libc::c_int as usize]
            * (*in2.offset(1 as libc::c_int as isize))[1 as libc::c_int as usize]
        + (*in1.offset(1 as libc::c_int as isize))[2 as libc::c_int as usize]
            * (*in2.offset(2 as libc::c_int as isize))[1 as libc::c_int as usize];
    (*out
        .offset(
            1 as libc::c_int as isize,
        ))[2 as libc::c_int
        as usize] = (*in1.offset(1 as libc::c_int as isize))[0 as libc::c_int as usize]
        * (*in2.offset(0 as libc::c_int as isize))[2 as libc::c_int as usize]
        + (*in1.offset(1 as libc::c_int as isize))[1 as libc::c_int as usize]
            * (*in2.offset(1 as libc::c_int as isize))[2 as libc::c_int as usize]
        + (*in1.offset(1 as libc::c_int as isize))[2 as libc::c_int as usize]
            * (*in2.offset(2 as libc::c_int as isize))[2 as libc::c_int as usize];
    (*out
        .offset(
            1 as libc::c_int as isize,
        ))[3 as libc::c_int
        as usize] = (*in1.offset(1 as libc::c_int as isize))[0 as libc::c_int as usize]
        * (*in2.offset(0 as libc::c_int as isize))[3 as libc::c_int as usize]
        + (*in1.offset(1 as libc::c_int as isize))[1 as libc::c_int as usize]
            * (*in2.offset(1 as libc::c_int as isize))[3 as libc::c_int as usize]
        + (*in1.offset(1 as libc::c_int as isize))[2 as libc::c_int as usize]
            * (*in2.offset(2 as libc::c_int as isize))[3 as libc::c_int as usize]
        + (*in1.offset(1 as libc::c_int as isize))[3 as libc::c_int as usize];
    (*out
        .offset(
            2 as libc::c_int as isize,
        ))[0 as libc::c_int
        as usize] = (*in1.offset(2 as libc::c_int as isize))[0 as libc::c_int as usize]
        * (*in2.offset(0 as libc::c_int as isize))[0 as libc::c_int as usize]
        + (*in1.offset(2 as libc::c_int as isize))[1 as libc::c_int as usize]
            * (*in2.offset(1 as libc::c_int as isize))[0 as libc::c_int as usize]
        + (*in1.offset(2 as libc::c_int as isize))[2 as libc::c_int as usize]
            * (*in2.offset(2 as libc::c_int as isize))[0 as libc::c_int as usize];
    (*out
        .offset(
            2 as libc::c_int as isize,
        ))[1 as libc::c_int
        as usize] = (*in1.offset(2 as libc::c_int as isize))[0 as libc::c_int as usize]
        * (*in2.offset(0 as libc::c_int as isize))[1 as libc::c_int as usize]
        + (*in1.offset(2 as libc::c_int as isize))[1 as libc::c_int as usize]
            * (*in2.offset(1 as libc::c_int as isize))[1 as libc::c_int as usize]
        + (*in1.offset(2 as libc::c_int as isize))[2 as libc::c_int as usize]
            * (*in2.offset(2 as libc::c_int as isize))[1 as libc::c_int as usize];
    (*out
        .offset(
            2 as libc::c_int as isize,
        ))[2 as libc::c_int
        as usize] = (*in1.offset(2 as libc::c_int as isize))[0 as libc::c_int as usize]
        * (*in2.offset(0 as libc::c_int as isize))[2 as libc::c_int as usize]
        + (*in1.offset(2 as libc::c_int as isize))[1 as libc::c_int as usize]
            * (*in2.offset(1 as libc::c_int as isize))[2 as libc::c_int as usize]
        + (*in1.offset(2 as libc::c_int as isize))[2 as libc::c_int as usize]
            * (*in2.offset(2 as libc::c_int as isize))[2 as libc::c_int as usize];
    (*out
        .offset(
            2 as libc::c_int as isize,
        ))[3 as libc::c_int
        as usize] = (*in1.offset(2 as libc::c_int as isize))[0 as libc::c_int as usize]
        * (*in2.offset(0 as libc::c_int as isize))[3 as libc::c_int as usize]
        + (*in1.offset(2 as libc::c_int as isize))[1 as libc::c_int as usize]
            * (*in2.offset(1 as libc::c_int as isize))[3 as libc::c_int as usize]
        + (*in1.offset(2 as libc::c_int as isize))[2 as libc::c_int as usize]
            * (*in2.offset(2 as libc::c_int as isize))[3 as libc::c_int as usize]
        + (*in1.offset(2 as libc::c_int as isize))[3 as libc::c_int as usize];
}
#[no_mangle]
pub unsafe extern "C" fn Q_fabs(mut f: libc::c_float) -> libc::c_float {
    let mut tmp: libc::c_int = *(&mut f as *mut libc::c_float as *mut libc::c_int);
    tmp &= 0x7fffffff as libc::c_int;
    return *(&mut tmp as *mut libc::c_int as *mut libc::c_float);
}
#[no_mangle]
pub unsafe extern "C" fn LerpAngle(
    mut a2: libc::c_float,
    mut a1: libc::c_float,
    mut frac: libc::c_float,
) -> libc::c_float {
    if a1 - a2 > 180 as libc::c_int as libc::c_float {
        a1 -= 360 as libc::c_int as libc::c_float;
    }
    if a1 - a2 < -(180 as libc::c_int) as libc::c_float {
        a1 += 360 as libc::c_int as libc::c_float;
    }
    return a2 + frac * (a1 - a2);
}
#[no_mangle]
pub unsafe extern "C" fn anglemod(mut a: libc::c_float) -> libc::c_float {
    a = (360.0f64 / 65536 as libc::c_int as libc::c_double
        * ((a as libc::c_double * (65536 as libc::c_int as libc::c_double / 360.0f64))
            as libc::c_int & 65535 as libc::c_int) as libc::c_double) as libc::c_float;
    return a;
}
#[no_mangle]
pub static mut i: libc::c_int = 0;
#[no_mangle]
pub static mut corners: [vec3_t; 2] = [[0.; 3]; 2];
#[no_mangle]
pub unsafe extern "C" fn BoxOnPlaneSide2(
    mut emins: *mut vec_t,
    mut emaxs: *mut vec_t,
    mut p: *mut cplane_s,
) -> libc::c_int {
    let mut i_0: libc::c_int = 0;
    let mut dist1: libc::c_float = 0.;
    let mut dist2: libc::c_float = 0.;
    let mut sides: libc::c_int = 0;
    let mut corners_0: [vec3_t; 2] = [[0.; 3]; 2];
    i_0 = 0 as libc::c_int;
    while i_0 < 3 as libc::c_int {
        if (*p).normal[i_0 as usize] < 0 as libc::c_int as libc::c_float {
            corners_0[0 as libc::c_int
                as usize][i_0 as usize] = *emins.offset(i_0 as isize);
            corners_0[1 as libc::c_int
                as usize][i_0 as usize] = *emaxs.offset(i_0 as isize);
        } else {
            corners_0[1 as libc::c_int
                as usize][i_0 as usize] = *emins.offset(i_0 as isize);
            corners_0[0 as libc::c_int
                as usize][i_0 as usize] = *emaxs.offset(i_0 as isize);
        }
        i_0 += 1;
    }
    dist1 = (*p).normal[0 as libc::c_int as usize]
        * corners_0[0 as libc::c_int as usize][0 as libc::c_int as usize]
        + (*p).normal[1 as libc::c_int as usize]
            * corners_0[0 as libc::c_int as usize][1 as libc::c_int as usize]
        + (*p).normal[2 as libc::c_int as usize]
            * corners_0[0 as libc::c_int as usize][2 as libc::c_int as usize]
        - (*p).dist;
    dist2 = (*p).normal[0 as libc::c_int as usize]
        * corners_0[1 as libc::c_int as usize][0 as libc::c_int as usize]
        + (*p).normal[1 as libc::c_int as usize]
            * corners_0[1 as libc::c_int as usize][1 as libc::c_int as usize]
        + (*p).normal[2 as libc::c_int as usize]
            * corners_0[1 as libc::c_int as usize][2 as libc::c_int as usize]
        - (*p).dist;
    sides = 0 as libc::c_int;
    if dist1 >= 0 as libc::c_int as libc::c_float {
        sides = 1 as libc::c_int;
    }
    if dist2 < 0 as libc::c_int as libc::c_float {
        sides |= 2 as libc::c_int;
    }
    return sides;
}
#[no_mangle]
pub unsafe extern "C" fn BoxOnPlaneSide(
    mut emins: *mut vec_t,
    mut emaxs: *mut vec_t,
    mut p: *mut cplane_s,
) -> libc::c_int {
    let mut dist1: libc::c_float = 0.;
    let mut dist2: libc::c_float = 0.;
    let mut sides: libc::c_int = 0;
    if ((*p).type_0 as libc::c_int) < 3 as libc::c_int {
        if (*p).dist <= *emins.offset((*p).type_0 as isize) {
            return 1 as libc::c_int;
        }
        if (*p).dist >= *emaxs.offset((*p).type_0 as isize) {
            return 2 as libc::c_int;
        }
        return 3 as libc::c_int;
    }
    match (*p).signbits as libc::c_int {
        0 => {
            dist1 = (*p).normal[0 as libc::c_int as usize]
                * *emaxs.offset(0 as libc::c_int as isize)
                + (*p).normal[1 as libc::c_int as usize]
                    * *emaxs.offset(1 as libc::c_int as isize)
                + (*p).normal[2 as libc::c_int as usize]
                    * *emaxs.offset(2 as libc::c_int as isize);
            dist2 = (*p).normal[0 as libc::c_int as usize]
                * *emins.offset(0 as libc::c_int as isize)
                + (*p).normal[1 as libc::c_int as usize]
                    * *emins.offset(1 as libc::c_int as isize)
                + (*p).normal[2 as libc::c_int as usize]
                    * *emins.offset(2 as libc::c_int as isize);
        }
        1 => {
            dist1 = (*p).normal[0 as libc::c_int as usize]
                * *emins.offset(0 as libc::c_int as isize)
                + (*p).normal[1 as libc::c_int as usize]
                    * *emaxs.offset(1 as libc::c_int as isize)
                + (*p).normal[2 as libc::c_int as usize]
                    * *emaxs.offset(2 as libc::c_int as isize);
            dist2 = (*p).normal[0 as libc::c_int as usize]
                * *emaxs.offset(0 as libc::c_int as isize)
                + (*p).normal[1 as libc::c_int as usize]
                    * *emins.offset(1 as libc::c_int as isize)
                + (*p).normal[2 as libc::c_int as usize]
                    * *emins.offset(2 as libc::c_int as isize);
        }
        2 => {
            dist1 = (*p).normal[0 as libc::c_int as usize]
                * *emaxs.offset(0 as libc::c_int as isize)
                + (*p).normal[1 as libc::c_int as usize]
                    * *emins.offset(1 as libc::c_int as isize)
                + (*p).normal[2 as libc::c_int as usize]
                    * *emaxs.offset(2 as libc::c_int as isize);
            dist2 = (*p).normal[0 as libc::c_int as usize]
                * *emins.offset(0 as libc::c_int as isize)
                + (*p).normal[1 as libc::c_int as usize]
                    * *emaxs.offset(1 as libc::c_int as isize)
                + (*p).normal[2 as libc::c_int as usize]
                    * *emins.offset(2 as libc::c_int as isize);
        }
        3 => {
            dist1 = (*p).normal[0 as libc::c_int as usize]
                * *emins.offset(0 as libc::c_int as isize)
                + (*p).normal[1 as libc::c_int as usize]
                    * *emins.offset(1 as libc::c_int as isize)
                + (*p).normal[2 as libc::c_int as usize]
                    * *emaxs.offset(2 as libc::c_int as isize);
            dist2 = (*p).normal[0 as libc::c_int as usize]
                * *emaxs.offset(0 as libc::c_int as isize)
                + (*p).normal[1 as libc::c_int as usize]
                    * *emaxs.offset(1 as libc::c_int as isize)
                + (*p).normal[2 as libc::c_int as usize]
                    * *emins.offset(2 as libc::c_int as isize);
        }
        4 => {
            dist1 = (*p).normal[0 as libc::c_int as usize]
                * *emaxs.offset(0 as libc::c_int as isize)
                + (*p).normal[1 as libc::c_int as usize]
                    * *emaxs.offset(1 as libc::c_int as isize)
                + (*p).normal[2 as libc::c_int as usize]
                    * *emins.offset(2 as libc::c_int as isize);
            dist2 = (*p).normal[0 as libc::c_int as usize]
                * *emins.offset(0 as libc::c_int as isize)
                + (*p).normal[1 as libc::c_int as usize]
                    * *emins.offset(1 as libc::c_int as isize)
                + (*p).normal[2 as libc::c_int as usize]
                    * *emaxs.offset(2 as libc::c_int as isize);
        }
        5 => {
            dist1 = (*p).normal[0 as libc::c_int as usize]
                * *emins.offset(0 as libc::c_int as isize)
                + (*p).normal[1 as libc::c_int as usize]
                    * *emaxs.offset(1 as libc::c_int as isize)
                + (*p).normal[2 as libc::c_int as usize]
                    * *emins.offset(2 as libc::c_int as isize);
            dist2 = (*p).normal[0 as libc::c_int as usize]
                * *emaxs.offset(0 as libc::c_int as isize)
                + (*p).normal[1 as libc::c_int as usize]
                    * *emins.offset(1 as libc::c_int as isize)
                + (*p).normal[2 as libc::c_int as usize]
                    * *emaxs.offset(2 as libc::c_int as isize);
        }
        6 => {
            dist1 = (*p).normal[0 as libc::c_int as usize]
                * *emaxs.offset(0 as libc::c_int as isize)
                + (*p).normal[1 as libc::c_int as usize]
                    * *emins.offset(1 as libc::c_int as isize)
                + (*p).normal[2 as libc::c_int as usize]
                    * *emins.offset(2 as libc::c_int as isize);
            dist2 = (*p).normal[0 as libc::c_int as usize]
                * *emins.offset(0 as libc::c_int as isize)
                + (*p).normal[1 as libc::c_int as usize]
                    * *emaxs.offset(1 as libc::c_int as isize)
                + (*p).normal[2 as libc::c_int as usize]
                    * *emaxs.offset(2 as libc::c_int as isize);
        }
        7 => {
            dist1 = (*p).normal[0 as libc::c_int as usize]
                * *emins.offset(0 as libc::c_int as isize)
                + (*p).normal[1 as libc::c_int as usize]
                    * *emins.offset(1 as libc::c_int as isize)
                + (*p).normal[2 as libc::c_int as usize]
                    * *emins.offset(2 as libc::c_int as isize);
            dist2 = (*p).normal[0 as libc::c_int as usize]
                * *emaxs.offset(0 as libc::c_int as isize)
                + (*p).normal[1 as libc::c_int as usize]
                    * *emaxs.offset(1 as libc::c_int as isize)
                + (*p).normal[2 as libc::c_int as usize]
                    * *emaxs.offset(2 as libc::c_int as isize);
        }
        _ => {
            dist2 = 0 as libc::c_int as libc::c_float;
            dist1 = dist2;
            __assert_fail(
                b"0\0" as *const u8 as *const libc::c_char,
                b"./q_shared.c\0"
                    as *const u8 as *const libc::c_char,
                401 as libc::c_int as libc::c_uint,
                (*::std::mem::transmute::<
                    &[u8; 56],
                    &[libc::c_char; 56],
                >(b"int BoxOnPlaneSide(vec_t *, vec_t *, struct cplane_s *)\0"))
                    .as_ptr(),
            );
        }
    }
    sides = 0 as libc::c_int;
    if dist1 >= (*p).dist {
        sides = 1 as libc::c_int;
    }
    if dist2 < (*p).dist {
        sides |= 2 as libc::c_int;
    }
    if sides != 0 as libc::c_int {} else {
        __assert_fail(
            b"sides != 0\0" as *const u8 as *const libc::c_char,
            b"./q_shared.c\0"
                as *const u8 as *const libc::c_char,
            411 as libc::c_int as libc::c_uint,
            (*::std::mem::transmute::<
                &[u8; 56],
                &[libc::c_char; 56],
            >(b"int BoxOnPlaneSide(vec_t *, vec_t *, struct cplane_s *)\0"))
                .as_ptr(),
        );
    }
    return sides;
}
#[no_mangle]
pub unsafe extern "C" fn ClearBounds(mut mins: *mut vec_t, mut maxs: *mut vec_t) {
    let ref mut fresh0 = *mins.offset(2 as libc::c_int as isize);
    *fresh0 = 99999 as libc::c_int as vec_t;
    let ref mut fresh1 = *mins.offset(1 as libc::c_int as isize);
    *fresh1 = *fresh0;
    *mins.offset(0 as libc::c_int as isize) = *fresh1;
    let ref mut fresh2 = *maxs.offset(2 as libc::c_int as isize);
    *fresh2 = -(99999 as libc::c_int) as vec_t;
    let ref mut fresh3 = *maxs.offset(1 as libc::c_int as isize);
    *fresh3 = *fresh2;
    *maxs.offset(0 as libc::c_int as isize) = *fresh3;
}
#[no_mangle]
pub unsafe extern "C" fn AddPointToBounds(
    mut v: *mut vec_t,
    mut mins: *mut vec_t,
    mut maxs: *mut vec_t,
) {
    let mut i_0: libc::c_int = 0;
    let mut val: vec_t = 0.;
    i_0 = 0 as libc::c_int;
    while i_0 < 3 as libc::c_int {
        val = *v.offset(i_0 as isize);
        if val < *mins.offset(i_0 as isize) {
            *mins.offset(i_0 as isize) = val;
        }
        if val > *maxs.offset(i_0 as isize) {
            *maxs.offset(i_0 as isize) = val;
        }
        i_0 += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn VectorCompare(
    mut v1: *mut vec_t,
    mut v2: *mut vec_t,
) -> libc::c_int {
    if *v1.offset(0 as libc::c_int as isize) != *v2.offset(0 as libc::c_int as isize)
        || *v1.offset(1 as libc::c_int as isize) != *v2.offset(1 as libc::c_int as isize)
        || *v1.offset(2 as libc::c_int as isize) != *v2.offset(2 as libc::c_int as isize)
    {
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn VectorNormalize(mut v: *mut vec_t) -> vec_t {
    let mut length: libc::c_float = 0.;
    let mut ilength: libc::c_float = 0.;
    length = *v.offset(0 as libc::c_int as isize) * *v.offset(0 as libc::c_int as isize)
        + *v.offset(1 as libc::c_int as isize) * *v.offset(1 as libc::c_int as isize)
        + *v.offset(2 as libc::c_int as isize) * *v.offset(2 as libc::c_int as isize);
    length = sqrt(length as libc::c_double) as libc::c_float;
    if length != 0. {
        ilength = 1 as libc::c_int as libc::c_float / length;
        let ref mut fresh4 = *v.offset(0 as libc::c_int as isize);
        *fresh4 *= ilength;
        let ref mut fresh5 = *v.offset(1 as libc::c_int as isize);
        *fresh5 *= ilength;
        let ref mut fresh6 = *v.offset(2 as libc::c_int as isize);
        *fresh6 *= ilength;
    }
    return length;
}
#[no_mangle]
pub unsafe extern "C" fn VectorNormalize2(
    mut v: *mut vec_t,
    mut out: *mut vec_t,
) -> vec_t {
    let mut length: libc::c_float = 0.;
    let mut ilength: libc::c_float = 0.;
    length = *v.offset(0 as libc::c_int as isize) * *v.offset(0 as libc::c_int as isize)
        + *v.offset(1 as libc::c_int as isize) * *v.offset(1 as libc::c_int as isize)
        + *v.offset(2 as libc::c_int as isize) * *v.offset(2 as libc::c_int as isize);
    length = sqrt(length as libc::c_double) as libc::c_float;
    if length != 0. {
        ilength = 1 as libc::c_int as libc::c_float / length;
        *out
            .offset(
                0 as libc::c_int as isize,
            ) = *v.offset(0 as libc::c_int as isize) * ilength;
        *out
            .offset(
                1 as libc::c_int as isize,
            ) = *v.offset(1 as libc::c_int as isize) * ilength;
        *out
            .offset(
                2 as libc::c_int as isize,
            ) = *v.offset(2 as libc::c_int as isize) * ilength;
    }
    return length;
}
#[no_mangle]
pub unsafe extern "C" fn VectorMA(
    mut veca: *mut vec_t,
    mut scale: libc::c_float,
    mut vecb: *mut vec_t,
    mut vecc: *mut vec_t,
) {
    *vecc
        .offset(
            0 as libc::c_int as isize,
        ) = *veca.offset(0 as libc::c_int as isize)
        + scale * *vecb.offset(0 as libc::c_int as isize);
    *vecc
        .offset(
            1 as libc::c_int as isize,
        ) = *veca.offset(1 as libc::c_int as isize)
        + scale * *vecb.offset(1 as libc::c_int as isize);
    *vecc
        .offset(
            2 as libc::c_int as isize,
        ) = *veca.offset(2 as libc::c_int as isize)
        + scale * *vecb.offset(2 as libc::c_int as isize);
}
#[no_mangle]
pub unsafe extern "C" fn _DotProduct(mut v1: *mut vec_t, mut v2: *mut vec_t) -> vec_t {
    return *v1.offset(0 as libc::c_int as isize) * *v2.offset(0 as libc::c_int as isize)
        + *v1.offset(1 as libc::c_int as isize) * *v2.offset(1 as libc::c_int as isize)
        + *v1.offset(2 as libc::c_int as isize) * *v2.offset(2 as libc::c_int as isize);
}
#[no_mangle]
pub unsafe extern "C" fn _VectorSubtract(
    mut veca: *mut vec_t,
    mut vecb: *mut vec_t,
    mut out: *mut vec_t,
) {
    *out
        .offset(
            0 as libc::c_int as isize,
        ) = *veca.offset(0 as libc::c_int as isize)
        - *vecb.offset(0 as libc::c_int as isize);
    *out
        .offset(
            1 as libc::c_int as isize,
        ) = *veca.offset(1 as libc::c_int as isize)
        - *vecb.offset(1 as libc::c_int as isize);
    *out
        .offset(
            2 as libc::c_int as isize,
        ) = *veca.offset(2 as libc::c_int as isize)
        - *vecb.offset(2 as libc::c_int as isize);
}
#[no_mangle]
pub unsafe extern "C" fn _VectorAdd(
    mut veca: *mut vec_t,
    mut vecb: *mut vec_t,
    mut out: *mut vec_t,
) {
    *out
        .offset(
            0 as libc::c_int as isize,
        ) = *veca.offset(0 as libc::c_int as isize)
        + *vecb.offset(0 as libc::c_int as isize);
    *out
        .offset(
            1 as libc::c_int as isize,
        ) = *veca.offset(1 as libc::c_int as isize)
        + *vecb.offset(1 as libc::c_int as isize);
    *out
        .offset(
            2 as libc::c_int as isize,
        ) = *veca.offset(2 as libc::c_int as isize)
        + *vecb.offset(2 as libc::c_int as isize);
}
#[no_mangle]
pub unsafe extern "C" fn _VectorCopy(mut in_0: *mut vec_t, mut out: *mut vec_t) {
    *out.offset(0 as libc::c_int as isize) = *in_0.offset(0 as libc::c_int as isize);
    *out.offset(1 as libc::c_int as isize) = *in_0.offset(1 as libc::c_int as isize);
    *out.offset(2 as libc::c_int as isize) = *in_0.offset(2 as libc::c_int as isize);
}
#[no_mangle]
pub unsafe extern "C" fn CrossProduct(
    mut v1: *mut vec_t,
    mut v2: *mut vec_t,
    mut cross: *mut vec_t,
) {
    *cross
        .offset(
            0 as libc::c_int as isize,
        ) = *v1.offset(1 as libc::c_int as isize) * *v2.offset(2 as libc::c_int as isize)
        - *v1.offset(2 as libc::c_int as isize) * *v2.offset(1 as libc::c_int as isize);
    *cross
        .offset(
            1 as libc::c_int as isize,
        ) = *v1.offset(2 as libc::c_int as isize) * *v2.offset(0 as libc::c_int as isize)
        - *v1.offset(0 as libc::c_int as isize) * *v2.offset(2 as libc::c_int as isize);
    *cross
        .offset(
            2 as libc::c_int as isize,
        ) = *v1.offset(0 as libc::c_int as isize) * *v2.offset(1 as libc::c_int as isize)
        - *v1.offset(1 as libc::c_int as isize) * *v2.offset(0 as libc::c_int as isize);
}
#[no_mangle]
pub unsafe extern "C" fn VectorLength(mut v: *mut vec_t) -> vec_t {
    let mut i_0: libc::c_int = 0;
    let mut length: libc::c_float = 0.;
    length = 0 as libc::c_int as libc::c_float;
    i_0 = 0 as libc::c_int;
    while i_0 < 3 as libc::c_int {
        length += *v.offset(i_0 as isize) * *v.offset(i_0 as isize);
        i_0 += 1;
    }
    length = sqrt(length as libc::c_double) as libc::c_float;
    return length;
}
#[no_mangle]
pub unsafe extern "C" fn VectorInverse(mut v: *mut vec_t) {
    *v.offset(0 as libc::c_int as isize) = -*v.offset(0 as libc::c_int as isize);
    *v.offset(1 as libc::c_int as isize) = -*v.offset(1 as libc::c_int as isize);
    *v.offset(2 as libc::c_int as isize) = -*v.offset(2 as libc::c_int as isize);
}
#[no_mangle]
pub unsafe extern "C" fn VectorScale(
    mut in_0: *mut vec_t,
    mut scale: vec_t,
    mut out: *mut vec_t,
) {
    *out
        .offset(
            0 as libc::c_int as isize,
        ) = *in_0.offset(0 as libc::c_int as isize) * scale;
    *out
        .offset(
            1 as libc::c_int as isize,
        ) = *in_0.offset(1 as libc::c_int as isize) * scale;
    *out
        .offset(
            2 as libc::c_int as isize,
        ) = *in_0.offset(2 as libc::c_int as isize) * scale;
}
#[no_mangle]
pub unsafe extern "C" fn Q_log2(mut val: libc::c_int) -> libc::c_int {
    let mut answer: libc::c_int = 0 as libc::c_int;
    loop {
        val >>= 1 as libc::c_int;
        if !(val != 0) {
            break;
        }
        answer += 1;
    }
    return answer;
}
#[no_mangle]
pub unsafe extern "C" fn COM_SkipPath(
    mut pathname: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut last: *mut libc::c_char = 0 as *mut libc::c_char;
    last = pathname;
    while *pathname != 0 {
        if *pathname as libc::c_int == '/' as i32 {
            last = pathname.offset(1 as libc::c_int as isize);
        }
        pathname = pathname.offset(1);
    }
    return last;
}
#[no_mangle]
pub unsafe extern "C" fn COM_StripExtension(
    mut in_0: *mut libc::c_char,
    mut out: *mut libc::c_char,
) {
    while *in_0 as libc::c_int != 0 && *in_0 as libc::c_int != '.' as i32 {
        let fresh7 = in_0;
        in_0 = in_0.offset(1);
        let fresh8 = out;
        out = out.offset(1);
        *fresh8 = *fresh7;
    }
    *out = 0 as libc::c_int as libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn COM_FileExtension(
    mut in_0: *mut libc::c_char,
) -> *mut libc::c_char {
    static mut exten: [libc::c_char; 8] = [0; 8];
    let mut i_0: libc::c_int = 0;
    while *in_0 as libc::c_int != 0 && *in_0 as libc::c_int != '.' as i32 {
        in_0 = in_0.offset(1);
    }
    if *in_0 == 0 {
        return b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    in_0 = in_0.offset(1);
    i_0 = 0 as libc::c_int;
    while i_0 < 7 as libc::c_int && *in_0 as libc::c_int != 0 {
        exten[i_0 as usize] = *in_0;
        i_0 += 1;
        in_0 = in_0.offset(1);
    }
    exten[i_0 as usize] = 0 as libc::c_int as libc::c_char;
    return exten.as_mut_ptr();
}
#[no_mangle]
pub unsafe extern "C" fn COM_FileBase(
    mut in_0: *mut libc::c_char,
    mut out: *mut libc::c_char,
) {
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut s2: *mut libc::c_char = 0 as *mut libc::c_char;
    s = in_0.offset(strlen(in_0) as isize).offset(-(1 as libc::c_int as isize));
    while s != in_0 && *s as libc::c_int != '.' as i32 {
        s = s.offset(-1);
    }
    s2 = s;
    while s2 != in_0 && *s2 as libc::c_int != '/' as i32 {
        s2 = s2.offset(-1);
    }
    if (s.offset_from(s2) as libc::c_long) < 2 as libc::c_int as libc::c_long {
        *out.offset(0 as libc::c_int as isize) = 0 as libc::c_int as libc::c_char;
    } else {
        s = s.offset(-1);
        strncpy(
            out,
            s2.offset(1 as libc::c_int as isize),
            s.offset_from(s2) as libc::c_long as libc::c_ulong,
        );
        *out
            .offset(
                s.offset_from(s2) as libc::c_long as isize,
            ) = 0 as libc::c_int as libc::c_char;
    };
}
#[no_mangle]
pub unsafe extern "C" fn COM_FilePath(
    mut in_0: *mut libc::c_char,
    mut out: *mut libc::c_char,
) {
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    s = in_0.offset(strlen(in_0) as isize).offset(-(1 as libc::c_int as isize));
    while s != in_0 && *s as libc::c_int != '/' as i32 {
        s = s.offset(-1);
    }
    strncpy(out, in_0, s.offset_from(in_0) as libc::c_long as libc::c_ulong);
    *out
        .offset(
            s.offset_from(in_0) as libc::c_long as isize,
        ) = 0 as libc::c_int as libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn COM_DefaultExtension(
    mut path: *mut libc::c_char,
    mut extension: *mut libc::c_char,
) {
    let mut src: *mut libc::c_char = 0 as *mut libc::c_char;
    src = path.offset(strlen(path) as isize).offset(-(1 as libc::c_int as isize));
    while *src as libc::c_int != '/' as i32 && src != path {
        if *src as libc::c_int == '.' as i32 {
            return;
        }
        src = src.offset(-1);
    }
    strcat(path, extension);
}
#[no_mangle]
pub static mut bigendien: qboolean = false_0;
#[no_mangle]
pub static mut _BigShort: Option::<
    unsafe extern "C" fn(libc::c_short) -> libc::c_short,
> = None;
#[no_mangle]
pub static mut _LittleShort: Option::<
    unsafe extern "C" fn(libc::c_short) -> libc::c_short,
> = None;
#[no_mangle]
pub static mut _BigLong: Option::<unsafe extern "C" fn(libc::c_int) -> libc::c_int> = None;
#[no_mangle]
pub static mut _LittleLong: Option::<unsafe extern "C" fn(libc::c_int) -> libc::c_int> = None;
#[no_mangle]
pub static mut _BigFloat: Option::<
    unsafe extern "C" fn(libc::c_float) -> libc::c_float,
> = None;
#[no_mangle]
pub static mut _LittleFloat: Option::<
    unsafe extern "C" fn(libc::c_float) -> libc::c_float,
> = None;
#[no_mangle]
pub unsafe extern "C" fn BigShort(mut l: libc::c_short) -> libc::c_short {
    return _BigShort.expect("non-null function pointer")(l);
}
#[no_mangle]
pub unsafe extern "C" fn LittleShort(mut l: libc::c_short) -> libc::c_short {
    return _LittleShort.expect("non-null function pointer")(l);
}
#[no_mangle]
pub unsafe extern "C" fn BigLong(mut l: libc::c_int) -> libc::c_int {
    return _BigLong.expect("non-null function pointer")(l);
}
#[no_mangle]
pub unsafe extern "C" fn LittleLong(mut l: libc::c_int) -> libc::c_int {
    return _LittleLong.expect("non-null function pointer")(l);
}
#[no_mangle]
pub unsafe extern "C" fn BigFloat(mut l: libc::c_float) -> libc::c_float {
    return _BigFloat.expect("non-null function pointer")(l);
}
#[no_mangle]
pub unsafe extern "C" fn LittleFloat(mut l: libc::c_float) -> libc::c_float {
    return _LittleFloat.expect("non-null function pointer")(l);
}
#[no_mangle]
pub unsafe extern "C" fn ShortSwap(mut l: libc::c_short) -> libc::c_short {
    let mut b1: byte = 0;
    let mut b2: byte = 0;
    b1 = (l as libc::c_int & 255 as libc::c_int) as byte;
    b2 = (l as libc::c_int >> 8 as libc::c_int & 255 as libc::c_int) as byte;
    return (((b1 as libc::c_int) << 8 as libc::c_int) + b2 as libc::c_int)
        as libc::c_short;
}
#[no_mangle]
pub unsafe extern "C" fn ShortNoSwap(mut l: libc::c_short) -> libc::c_short {
    return l;
}
#[no_mangle]
pub unsafe extern "C" fn LongSwap(mut l: libc::c_int) -> libc::c_int {
    let mut b1: byte = 0;
    let mut b2: byte = 0;
    let mut b3: byte = 0;
    let mut b4: byte = 0;
    b1 = (l & 255 as libc::c_int) as byte;
    b2 = (l >> 8 as libc::c_int & 255 as libc::c_int) as byte;
    b3 = (l >> 16 as libc::c_int & 255 as libc::c_int) as byte;
    b4 = (l >> 24 as libc::c_int & 255 as libc::c_int) as byte;
    return ((b1 as libc::c_int) << 24 as libc::c_int)
        + ((b2 as libc::c_int) << 16 as libc::c_int)
        + ((b3 as libc::c_int) << 8 as libc::c_int) + b4 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn LongNoSwap(mut l: libc::c_int) -> libc::c_int {
    return l;
}
#[no_mangle]
pub unsafe extern "C" fn FloatSwap(mut f: libc::c_float) -> libc::c_float {
    let mut dat1: C2RustUnnamed = C2RustUnnamed { f: 0. };
    let mut dat2: C2RustUnnamed = C2RustUnnamed { f: 0. };
    dat1.f = f;
    dat2.b[0 as libc::c_int as usize] = dat1.b[3 as libc::c_int as usize];
    dat2.b[1 as libc::c_int as usize] = dat1.b[2 as libc::c_int as usize];
    dat2.b[2 as libc::c_int as usize] = dat1.b[1 as libc::c_int as usize];
    dat2.b[3 as libc::c_int as usize] = dat1.b[0 as libc::c_int as usize];
    return dat2.f;
}
#[no_mangle]
pub unsafe extern "C" fn FloatNoSwap(mut f: libc::c_float) -> libc::c_float {
    return f;
}
#[no_mangle]
pub unsafe extern "C" fn Swap_Init() {
    let mut swaptest: [byte; 2] = [1 as libc::c_int as byte, 0 as libc::c_int as byte];
    if *(swaptest.as_mut_ptr() as *mut libc::c_short) as libc::c_int == 1 as libc::c_int
    {
        bigendien = false_0;
        _BigShort = Some(
            ShortSwap as unsafe extern "C" fn(libc::c_short) -> libc::c_short,
        );
        _LittleShort = Some(
            ShortNoSwap as unsafe extern "C" fn(libc::c_short) -> libc::c_short,
        );
        _BigLong = Some(LongSwap as unsafe extern "C" fn(libc::c_int) -> libc::c_int);
        _LittleLong = Some(
            LongNoSwap as unsafe extern "C" fn(libc::c_int) -> libc::c_int,
        );
        _BigFloat = Some(
            FloatSwap as unsafe extern "C" fn(libc::c_float) -> libc::c_float,
        );
        _LittleFloat = Some(
            FloatNoSwap as unsafe extern "C" fn(libc::c_float) -> libc::c_float,
        );
    } else {
        bigendien = true_0;
        _BigShort = Some(
            ShortNoSwap as unsafe extern "C" fn(libc::c_short) -> libc::c_short,
        );
        _LittleShort = Some(
            ShortSwap as unsafe extern "C" fn(libc::c_short) -> libc::c_short,
        );
        _BigLong = Some(LongNoSwap as unsafe extern "C" fn(libc::c_int) -> libc::c_int);
        _LittleLong = Some(LongSwap as unsafe extern "C" fn(libc::c_int) -> libc::c_int);
        _BigFloat = Some(
            FloatNoSwap as unsafe extern "C" fn(libc::c_float) -> libc::c_float,
        );
        _LittleFloat = Some(
            FloatSwap as unsafe extern "C" fn(libc::c_float) -> libc::c_float,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn va(
    mut format: *mut libc::c_char,
    mut args: ...
) -> *mut libc::c_char {
    let mut argptr: ::std::ffi::VaListImpl;
    static mut string: [libc::c_char; 1024] = [0; 1024];
    argptr = args.clone();
    vsprintf(string.as_mut_ptr(), format, argptr.as_va_list());
    return string.as_mut_ptr();
}
#[no_mangle]
pub static mut com_token: [libc::c_char; 128] = [0; 128];
#[no_mangle]
pub unsafe extern "C" fn COM_Parse(
    mut data_p: *mut *mut libc::c_char,
) -> *mut libc::c_char {
    let mut c: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut data: *mut libc::c_char = 0 as *mut libc::c_char;
    data = *data_p;
    len = 0 as libc::c_int;
    com_token[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    if data.is_null() {
        *data_p = 0 as *mut libc::c_char;
        return b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    loop {
        loop {
            c = *data as libc::c_int;
            if !(c <= ' ' as i32) {
                break;
            }
            if c == 0 as libc::c_int {
                *data_p = 0 as *mut libc::c_char;
                return b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
            }
            data = data.offset(1);
        }
        if !(c == '/' as i32
            && *data.offset(1 as libc::c_int as isize) as libc::c_int == '/' as i32)
        {
            break;
        }
        while *data as libc::c_int != 0 && *data as libc::c_int != '\n' as i32 {
            data = data.offset(1);
        }
    }
    if c == '"' as i32 {
        data = data.offset(1);
        loop {
            let fresh9 = data;
            data = data.offset(1);
            c = *fresh9 as libc::c_int;
            if c == '"' as i32 || c == 0 {
                com_token[len as usize] = 0 as libc::c_int as libc::c_char;
                *data_p = data;
                return com_token.as_mut_ptr();
            }
            if len < 128 as libc::c_int {
                com_token[len as usize] = c as libc::c_char;
                len += 1;
            }
        }
    }
    loop {
        if len < 128 as libc::c_int {
            com_token[len as usize] = c as libc::c_char;
            len += 1;
        }
        data = data.offset(1);
        c = *data as libc::c_int;
        if !(c > 32 as libc::c_int) {
            break;
        }
    }
    if len == 128 as libc::c_int {
        len = 0 as libc::c_int;
    }
    com_token[len as usize] = 0 as libc::c_int as libc::c_char;
    *data_p = data;
    return com_token.as_mut_ptr();
}
#[no_mangle]
pub static mut paged_total: libc::c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn Com_PageInMemory(mut buffer: *mut byte, mut size: libc::c_int) {
    let mut i_0: libc::c_int = 0;
    i_0 = size - 1 as libc::c_int;
    while i_0 > 0 as libc::c_int {
        paged_total += *buffer.offset(i_0 as isize) as libc::c_int;
        i_0 -= 4096 as libc::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn Q_stricmp(
    mut s1: *mut libc::c_char,
    mut s2: *mut libc::c_char,
) -> libc::c_int {
    return strcasecmp(s1, s2);
}
#[no_mangle]
pub unsafe extern "C" fn Q_strncasecmp(
    mut s1: *mut libc::c_char,
    mut s2: *mut libc::c_char,
    mut n: libc::c_int,
) -> libc::c_int {
    let mut c1: libc::c_int = 0;
    let mut c2: libc::c_int = 0;
    loop {
        let fresh10 = s1;
        s1 = s1.offset(1);
        c1 = *fresh10 as libc::c_int;
        let fresh11 = s2;
        s2 = s2.offset(1);
        c2 = *fresh11 as libc::c_int;
        let fresh12 = n;
        n = n - 1;
        if fresh12 == 0 {
            return 0 as libc::c_int;
        }
        if c1 != c2 {
            if c1 >= 'a' as i32 && c1 <= 'z' as i32 {
                c1 -= 'a' as i32 - 'A' as i32;
            }
            if c2 >= 'a' as i32 && c2 <= 'z' as i32 {
                c2 -= 'a' as i32 - 'A' as i32;
            }
            if c1 != c2 {
                return -(1 as libc::c_int);
            }
        }
        if !(c1 != 0) {
            break;
        }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn Q_strcasecmp(
    mut s1: *mut libc::c_char,
    mut s2: *mut libc::c_char,
) -> libc::c_int {
    return Q_strncasecmp(s1, s2, 99999 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn Com_sprintf(
    mut dest: *mut libc::c_char,
    mut size: libc::c_int,
    mut fmt: *mut libc::c_char,
    mut args: ...
) {
    let mut len: libc::c_int = 0;
    let mut argptr: ::std::ffi::VaListImpl;
    let mut bigbuffer: [libc::c_char; 65536] = [0; 65536];
    argptr = args.clone();
    len = vsprintf(bigbuffer.as_mut_ptr(), fmt, argptr.as_va_list());
    if len >= size {
        Com_Printf(
            b"Com_sprintf: overflow of %i in %i\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            len,
            size,
        );
    }
    strncpy(dest, bigbuffer.as_mut_ptr(), (size - 1 as libc::c_int) as libc::c_ulong);
}
#[no_mangle]
pub unsafe extern "C" fn Info_ValueForKey(
    mut s: *mut libc::c_char,
    mut key: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut pkey: [libc::c_char; 512] = [0; 512];
    static mut value: [[libc::c_char; 512]; 2] = [[0; 512]; 2];
    static mut valueindex: libc::c_int = 0;
    let mut o: *mut libc::c_char = 0 as *mut libc::c_char;
    valueindex ^= 1 as libc::c_int;
    if *s as libc::c_int == '\\' as i32 {
        s = s.offset(1);
    }
    loop {
        o = pkey.as_mut_ptr();
        while *s as libc::c_int != '\\' as i32 {
            if *s == 0 {
                return b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
            }
            let fresh13 = s;
            s = s.offset(1);
            let fresh14 = o;
            o = o.offset(1);
            *fresh14 = *fresh13;
        }
        *o = 0 as libc::c_int as libc::c_char;
        s = s.offset(1);
        o = (value[valueindex as usize]).as_mut_ptr();
        while *s as libc::c_int != '\\' as i32 && *s as libc::c_int != 0 {
            if *s == 0 {
                return b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
            }
            let fresh15 = s;
            s = s.offset(1);
            let fresh16 = o;
            o = o.offset(1);
            *fresh16 = *fresh15;
        }
        *o = 0 as libc::c_int as libc::c_char;
        if strcmp(key, pkey.as_mut_ptr()) == 0 {
            return (value[valueindex as usize]).as_mut_ptr();
        }
        if *s == 0 {
            return b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        }
        s = s.offset(1);
    };
}
#[no_mangle]
pub unsafe extern "C" fn Info_RemoveKey(
    mut s: *mut libc::c_char,
    mut key: *mut libc::c_char,
) {
    let mut start: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pkey: [libc::c_char; 512] = [0; 512];
    let mut value: [libc::c_char; 512] = [0; 512];
    let mut o: *mut libc::c_char = 0 as *mut libc::c_char;
    if !(strstr(key, b"\\\0" as *const u8 as *const libc::c_char)).is_null() {
        return;
    }
    loop {
        start = s;
        if *s as libc::c_int == '\\' as i32 {
            s = s.offset(1);
        }
        o = pkey.as_mut_ptr();
        while *s as libc::c_int != '\\' as i32 {
            if *s == 0 {
                return;
            }
            let fresh17 = s;
            s = s.offset(1);
            let fresh18 = o;
            o = o.offset(1);
            *fresh18 = *fresh17;
        }
        *o = 0 as libc::c_int as libc::c_char;
        s = s.offset(1);
        o = value.as_mut_ptr();
        while *s as libc::c_int != '\\' as i32 && *s as libc::c_int != 0 {
            if *s == 0 {
                return;
            }
            let fresh19 = s;
            s = s.offset(1);
            let fresh20 = o;
            o = o.offset(1);
            *fresh20 = *fresh19;
        }
        *o = 0 as libc::c_int as libc::c_char;
        if strcmp(key, pkey.as_mut_ptr()) == 0 {
            strcpy(start, s);
            return;
        }
        if *s == 0 {
            return;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn Info_Validate(mut s: *mut libc::c_char) -> qboolean {
    if !(strstr(s, b"\"\0" as *const u8 as *const libc::c_char)).is_null() {
        return false_0;
    }
    if !(strstr(s, b";\0" as *const u8 as *const libc::c_char)).is_null() {
        return false_0;
    }
    return true_0;
}
#[no_mangle]
pub unsafe extern "C" fn Info_SetValueForKey(
    mut s: *mut libc::c_char,
    mut key: *mut libc::c_char,
    mut value: *mut libc::c_char,
) {
    let mut newi: [libc::c_char; 512] = [0; 512];
    let mut v: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut c: libc::c_int = 0;
    let mut maxsize: libc::c_int = 512 as libc::c_int;
    if !(strstr(key, b"\\\0" as *const u8 as *const libc::c_char)).is_null()
        || !(strstr(value, b"\\\0" as *const u8 as *const libc::c_char)).is_null()
    {
        Com_Printf(
            b"Can't use keys or values with a \\\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return;
    }
    if !(strstr(key, b";\0" as *const u8 as *const libc::c_char)).is_null() {
        Com_Printf(
            b"Can't use keys or values with a semicolon\n\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        );
        return;
    }
    if !(strstr(key, b"\"\0" as *const u8 as *const libc::c_char)).is_null()
        || !(strstr(value, b"\"\0" as *const u8 as *const libc::c_char)).is_null()
    {
        Com_Printf(
            b"Can't use keys or values with a \"\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return;
    }
    if strlen(key) > (64 as libc::c_int - 1 as libc::c_int) as libc::c_ulong
        || strlen(value) > (64 as libc::c_int - 1 as libc::c_int) as libc::c_ulong
    {
        Com_Printf(
            b"Keys and values must be < 64 characters.\n\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        );
        return;
    }
    Info_RemoveKey(s, key);
    if value.is_null() || strlen(value) == 0 {
        return;
    }
    Com_sprintf(
        newi.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong as libc::c_int,
        b"\\%s\\%s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        key,
        value,
    );
    if (strlen(newi.as_mut_ptr())).wrapping_add(strlen(s)) > maxsize as libc::c_ulong {
        Com_Printf(
            b"Info string length exceeded\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return;
    }
    s = s.offset(strlen(s) as isize);
    v = newi.as_mut_ptr();
    while *v != 0 {
        let fresh21 = v;
        v = v.offset(1);
        c = *fresh21 as libc::c_int;
        c &= 127 as libc::c_int;
        if c >= 32 as libc::c_int && c < 127 as libc::c_int {
            let fresh22 = s;
            s = s.offset(1);
            *fresh22 = c as libc::c_char;
        }
    }
    *s = 0 as libc::c_int as libc::c_char;
}
