#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    static mut r_refdef: oldrefdef_t;
    static mut aliastriangleparms: aliastriangleparms_t;
    fn R_DrawTriangle();
    fn R_AliasProjectAndClipTestFinalVert(fv_0: *mut finalvert_t);
}
pub type vec_t = libc::c_float;
pub type vec3_t = [vec_t; 3];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vrect_s {
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub pnext: *mut vrect_s,
}
pub type vrect_t = vrect_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct oldrefdef_t {
    pub vrect: vrect_t,
    pub aliasvrect: vrect_t,
    pub vrectright: libc::c_int,
    pub vrectbottom: libc::c_int,
    pub aliasvrectright: libc::c_int,
    pub aliasvrectbottom: libc::c_int,
    pub vrectrightedge: libc::c_float,
    pub fvrectx: libc::c_float,
    pub fvrecty: libc::c_float,
    pub fvrectx_adj: libc::c_float,
    pub fvrecty_adj: libc::c_float,
    pub vrect_x_adj_shift20: libc::c_int,
    pub vrectright_adj_shift20: libc::c_int,
    pub fvrectright_adj: libc::c_float,
    pub fvrectbottom_adj: libc::c_float,
    pub fvrectright: libc::c_float,
    pub fvrectbottom: libc::c_float,
    pub horizontalFieldOfView: libc::c_float,
    pub xOrigin: libc::c_float,
    pub yOrigin: libc::c_float,
    pub vieworg: vec3_t,
    pub viewangles: vec3_t,
    pub ambientlight: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct finalvert_s {
    pub u: libc::c_int,
    pub v: libc::c_int,
    pub s: libc::c_int,
    pub t: libc::c_int,
    pub l: libc::c_int,
    pub zi: libc::c_int,
    pub flags: libc::c_int,
    pub xyz: [libc::c_float; 3],
}
pub type finalvert_t = finalvert_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aliastriangleparms_t {
    pub a: *mut finalvert_t,
    pub b: *mut finalvert_t,
    pub c: *mut finalvert_t,
}
static mut fv: [[finalvert_t; 8]; 2] = [[finalvert_t {
    u: 0,
    v: 0,
    s: 0,
    t: 0,
    l: 0,
    zi: 0,
    flags: 0,
    xyz: [0.; 3],
}; 8]; 2];
#[no_mangle]
pub unsafe extern "C" fn R_Alias_clip_z(
    mut pfv0: *mut finalvert_t,
    mut pfv1: *mut finalvert_t,
    mut out: *mut finalvert_t,
) {
    let mut scale: libc::c_float = 0.;
    scale = (4 as libc::c_int as libc::c_float - (*pfv0).xyz[2 as libc::c_int as usize])
        / ((*pfv1).xyz[2 as libc::c_int as usize]
            - (*pfv0).xyz[2 as libc::c_int as usize]);
    (*out)
        .xyz[0 as libc::c_int
        as usize] = (*pfv0).xyz[0 as libc::c_int as usize]
        + ((*pfv1).xyz[0 as libc::c_int as usize]
            - (*pfv0).xyz[0 as libc::c_int as usize]) * scale;
    (*out)
        .xyz[1 as libc::c_int
        as usize] = (*pfv0).xyz[1 as libc::c_int as usize]
        + ((*pfv1).xyz[1 as libc::c_int as usize]
            - (*pfv0).xyz[1 as libc::c_int as usize]) * scale;
    (*out).xyz[2 as libc::c_int as usize] = 4 as libc::c_int as libc::c_float;
    (*out)
        .s = ((*pfv0).s as libc::c_float
        + ((*pfv1).s - (*pfv0).s) as libc::c_float * scale) as libc::c_int;
    (*out)
        .t = ((*pfv0).t as libc::c_float
        + ((*pfv1).t - (*pfv0).t) as libc::c_float * scale) as libc::c_int;
    (*out)
        .l = ((*pfv0).l as libc::c_float
        + ((*pfv1).l - (*pfv0).l) as libc::c_float * scale) as libc::c_int;
    R_AliasProjectAndClipTestFinalVert(out);
}
#[no_mangle]
pub unsafe extern "C" fn R_Alias_clip_left(
    mut pfv0: *mut finalvert_t,
    mut pfv1: *mut finalvert_t,
    mut out: *mut finalvert_t,
) {
    let mut scale: libc::c_float = 0.;
    if (*pfv0).v >= (*pfv1).v {
        scale = (r_refdef.aliasvrect.x - (*pfv0).u) as libc::c_float
            / ((*pfv1).u - (*pfv0).u) as libc::c_float;
        (*out)
            .u = (((*pfv0).u as libc::c_float
            + ((*pfv1).u - (*pfv0).u) as libc::c_float * scale) as libc::c_double
            + 0.5f64) as libc::c_int;
        (*out)
            .v = (((*pfv0).v as libc::c_float
            + ((*pfv1).v - (*pfv0).v) as libc::c_float * scale) as libc::c_double
            + 0.5f64) as libc::c_int;
        (*out)
            .s = (((*pfv0).s as libc::c_float
            + ((*pfv1).s - (*pfv0).s) as libc::c_float * scale) as libc::c_double
            + 0.5f64) as libc::c_int;
        (*out)
            .t = (((*pfv0).t as libc::c_float
            + ((*pfv1).t - (*pfv0).t) as libc::c_float * scale) as libc::c_double
            + 0.5f64) as libc::c_int;
        (*out)
            .l = (((*pfv0).l as libc::c_float
            + ((*pfv1).l - (*pfv0).l) as libc::c_float * scale) as libc::c_double
            + 0.5f64) as libc::c_int;
        (*out)
            .zi = (((*pfv0).zi as libc::c_float
            + ((*pfv1).zi - (*pfv0).zi) as libc::c_float * scale) as libc::c_double
            + 0.5f64) as libc::c_int;
    } else {
        scale = (r_refdef.aliasvrect.x - (*pfv1).u) as libc::c_float
            / ((*pfv0).u - (*pfv1).u) as libc::c_float;
        (*out)
            .u = (((*pfv1).u as libc::c_float
            + ((*pfv0).u - (*pfv1).u) as libc::c_float * scale) as libc::c_double
            + 0.5f64) as libc::c_int;
        (*out)
            .v = (((*pfv1).v as libc::c_float
            + ((*pfv0).v - (*pfv1).v) as libc::c_float * scale) as libc::c_double
            + 0.5f64) as libc::c_int;
        (*out)
            .s = (((*pfv1).s as libc::c_float
            + ((*pfv0).s - (*pfv1).s) as libc::c_float * scale) as libc::c_double
            + 0.5f64) as libc::c_int;
        (*out)
            .t = (((*pfv1).t as libc::c_float
            + ((*pfv0).t - (*pfv1).t) as libc::c_float * scale) as libc::c_double
            + 0.5f64) as libc::c_int;
        (*out)
            .l = (((*pfv1).l as libc::c_float
            + ((*pfv0).l - (*pfv1).l) as libc::c_float * scale) as libc::c_double
            + 0.5f64) as libc::c_int;
        (*out)
            .zi = (((*pfv1).zi as libc::c_float
            + ((*pfv0).zi - (*pfv1).zi) as libc::c_float * scale) as libc::c_double
            + 0.5f64) as libc::c_int;
    };
}
#[no_mangle]
pub unsafe extern "C" fn R_Alias_clip_right(
    mut pfv0: *mut finalvert_t,
    mut pfv1: *mut finalvert_t,
    mut out: *mut finalvert_t,
) {
    let mut scale: libc::c_float = 0.;
    if (*pfv0).v >= (*pfv1).v {
        scale = (r_refdef.aliasvrectright - (*pfv0).u) as libc::c_float
            / ((*pfv1).u - (*pfv0).u) as libc::c_float;
        (*out)
            .u = (((*pfv0).u as libc::c_float
            + ((*pfv1).u - (*pfv0).u) as libc::c_float * scale) as libc::c_double
            + 0.5f64) as libc::c_int;
        (*out)
            .v = (((*pfv0).v as libc::c_float
            + ((*pfv1).v - (*pfv0).v) as libc::c_float * scale) as libc::c_double
            + 0.5f64) as libc::c_int;
        (*out)
            .s = (((*pfv0).s as libc::c_float
            + ((*pfv1).s - (*pfv0).s) as libc::c_float * scale) as libc::c_double
            + 0.5f64) as libc::c_int;
        (*out)
            .t = (((*pfv0).t as libc::c_float
            + ((*pfv1).t - (*pfv0).t) as libc::c_float * scale) as libc::c_double
            + 0.5f64) as libc::c_int;
        (*out)
            .l = (((*pfv0).l as libc::c_float
            + ((*pfv1).l - (*pfv0).l) as libc::c_float * scale) as libc::c_double
            + 0.5f64) as libc::c_int;
        (*out)
            .zi = (((*pfv0).zi as libc::c_float
            + ((*pfv1).zi - (*pfv0).zi) as libc::c_float * scale) as libc::c_double
            + 0.5f64) as libc::c_int;
    } else {
        scale = (r_refdef.aliasvrectright - (*pfv1).u) as libc::c_float
            / ((*pfv0).u - (*pfv1).u) as libc::c_float;
        (*out)
            .u = (((*pfv1).u as libc::c_float
            + ((*pfv0).u - (*pfv1).u) as libc::c_float * scale) as libc::c_double
            + 0.5f64) as libc::c_int;
        (*out)
            .v = (((*pfv1).v as libc::c_float
            + ((*pfv0).v - (*pfv1).v) as libc::c_float * scale) as libc::c_double
            + 0.5f64) as libc::c_int;
        (*out)
            .s = (((*pfv1).s as libc::c_float
            + ((*pfv0).s - (*pfv1).s) as libc::c_float * scale) as libc::c_double
            + 0.5f64) as libc::c_int;
        (*out)
            .t = (((*pfv1).t as libc::c_float
            + ((*pfv0).t - (*pfv1).t) as libc::c_float * scale) as libc::c_double
            + 0.5f64) as libc::c_int;
        (*out)
            .l = (((*pfv1).l as libc::c_float
            + ((*pfv0).l - (*pfv1).l) as libc::c_float * scale) as libc::c_double
            + 0.5f64) as libc::c_int;
        (*out)
            .zi = (((*pfv1).zi as libc::c_float
            + ((*pfv0).zi - (*pfv1).zi) as libc::c_float * scale) as libc::c_double
            + 0.5f64) as libc::c_int;
    };
}
#[no_mangle]
pub unsafe extern "C" fn R_Alias_clip_top(
    mut pfv0: *mut finalvert_t,
    mut pfv1: *mut finalvert_t,
    mut out: *mut finalvert_t,
) {
    let mut scale: libc::c_float = 0.;
    if (*pfv0).v >= (*pfv1).v {
        scale = (r_refdef.aliasvrect.y - (*pfv0).v) as libc::c_float
            / ((*pfv1).v - (*pfv0).v) as libc::c_float;
        (*out)
            .u = (((*pfv0).u as libc::c_float
            + ((*pfv1).u - (*pfv0).u) as libc::c_float * scale) as libc::c_double
            + 0.5f64) as libc::c_int;
        (*out)
            .v = (((*pfv0).v as libc::c_float
            + ((*pfv1).v - (*pfv0).v) as libc::c_float * scale) as libc::c_double
            + 0.5f64) as libc::c_int;
        (*out)
            .s = (((*pfv0).s as libc::c_float
            + ((*pfv1).s - (*pfv0).s) as libc::c_float * scale) as libc::c_double
            + 0.5f64) as libc::c_int;
        (*out)
            .t = (((*pfv0).t as libc::c_float
            + ((*pfv1).t - (*pfv0).t) as libc::c_float * scale) as libc::c_double
            + 0.5f64) as libc::c_int;
        (*out)
            .l = (((*pfv0).l as libc::c_float
            + ((*pfv1).l - (*pfv0).l) as libc::c_float * scale) as libc::c_double
            + 0.5f64) as libc::c_int;
        (*out)
            .zi = (((*pfv0).zi as libc::c_float
            + ((*pfv1).zi - (*pfv0).zi) as libc::c_float * scale) as libc::c_double
            + 0.5f64) as libc::c_int;
    } else {
        scale = (r_refdef.aliasvrect.y - (*pfv1).v) as libc::c_float
            / ((*pfv0).v - (*pfv1).v) as libc::c_float;
        (*out)
            .u = (((*pfv1).u as libc::c_float
            + ((*pfv0).u - (*pfv1).u) as libc::c_float * scale) as libc::c_double
            + 0.5f64) as libc::c_int;
        (*out)
            .v = (((*pfv1).v as libc::c_float
            + ((*pfv0).v - (*pfv1).v) as libc::c_float * scale) as libc::c_double
            + 0.5f64) as libc::c_int;
        (*out)
            .s = (((*pfv1).s as libc::c_float
            + ((*pfv0).s - (*pfv1).s) as libc::c_float * scale) as libc::c_double
            + 0.5f64) as libc::c_int;
        (*out)
            .t = (((*pfv1).t as libc::c_float
            + ((*pfv0).t - (*pfv1).t) as libc::c_float * scale) as libc::c_double
            + 0.5f64) as libc::c_int;
        (*out)
            .l = (((*pfv1).l as libc::c_float
            + ((*pfv0).l - (*pfv1).l) as libc::c_float * scale) as libc::c_double
            + 0.5f64) as libc::c_int;
        (*out)
            .zi = (((*pfv1).zi as libc::c_float
            + ((*pfv0).zi - (*pfv1).zi) as libc::c_float * scale) as libc::c_double
            + 0.5f64) as libc::c_int;
    };
}
#[no_mangle]
pub unsafe extern "C" fn R_Alias_clip_bottom(
    mut pfv0: *mut finalvert_t,
    mut pfv1: *mut finalvert_t,
    mut out: *mut finalvert_t,
) {
    let mut scale: libc::c_float = 0.;
    if (*pfv0).v >= (*pfv1).v {
        scale = (r_refdef.aliasvrectbottom - (*pfv0).v) as libc::c_float
            / ((*pfv1).v - (*pfv0).v) as libc::c_float;
        (*out)
            .u = (((*pfv0).u as libc::c_float
            + ((*pfv1).u - (*pfv0).u) as libc::c_float * scale) as libc::c_double
            + 0.5f64) as libc::c_int;
        (*out)
            .v = (((*pfv0).v as libc::c_float
            + ((*pfv1).v - (*pfv0).v) as libc::c_float * scale) as libc::c_double
            + 0.5f64) as libc::c_int;
        (*out)
            .s = (((*pfv0).s as libc::c_float
            + ((*pfv1).s - (*pfv0).s) as libc::c_float * scale) as libc::c_double
            + 0.5f64) as libc::c_int;
        (*out)
            .t = (((*pfv0).t as libc::c_float
            + ((*pfv1).t - (*pfv0).t) as libc::c_float * scale) as libc::c_double
            + 0.5f64) as libc::c_int;
        (*out)
            .l = (((*pfv0).l as libc::c_float
            + ((*pfv1).l - (*pfv0).l) as libc::c_float * scale) as libc::c_double
            + 0.5f64) as libc::c_int;
        (*out)
            .zi = (((*pfv0).zi as libc::c_float
            + ((*pfv1).zi - (*pfv0).zi) as libc::c_float * scale) as libc::c_double
            + 0.5f64) as libc::c_int;
    } else {
        scale = (r_refdef.aliasvrectbottom - (*pfv1).v) as libc::c_float
            / ((*pfv0).v - (*pfv1).v) as libc::c_float;
        (*out)
            .u = (((*pfv1).u as libc::c_float
            + ((*pfv0).u - (*pfv1).u) as libc::c_float * scale) as libc::c_double
            + 0.5f64) as libc::c_int;
        (*out)
            .v = (((*pfv1).v as libc::c_float
            + ((*pfv0).v - (*pfv1).v) as libc::c_float * scale) as libc::c_double
            + 0.5f64) as libc::c_int;
        (*out)
            .s = (((*pfv1).s as libc::c_float
            + ((*pfv0).s - (*pfv1).s) as libc::c_float * scale) as libc::c_double
            + 0.5f64) as libc::c_int;
        (*out)
            .t = (((*pfv1).t as libc::c_float
            + ((*pfv0).t - (*pfv1).t) as libc::c_float * scale) as libc::c_double
            + 0.5f64) as libc::c_int;
        (*out)
            .l = (((*pfv1).l as libc::c_float
            + ((*pfv0).l - (*pfv1).l) as libc::c_float * scale) as libc::c_double
            + 0.5f64) as libc::c_int;
        (*out)
            .zi = (((*pfv1).zi as libc::c_float
            + ((*pfv0).zi - (*pfv1).zi) as libc::c_float * scale) as libc::c_double
            + 0.5f64) as libc::c_int;
    };
}
#[no_mangle]
pub unsafe extern "C" fn R_AliasClip(
    mut in_0: *mut finalvert_t,
    mut out: *mut finalvert_t,
    mut flag: libc::c_int,
    mut count: libc::c_int,
    mut clip: Option::<
        unsafe extern "C" fn(*mut finalvert_t, *mut finalvert_t, *mut finalvert_t) -> (),
    >,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut flags: libc::c_int = 0;
    let mut oldflags: libc::c_int = 0;
    j = count - 1 as libc::c_int;
    k = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < count {
        oldflags = (*in_0.offset(j as isize)).flags & flag;
        flags = (*in_0.offset(i as isize)).flags & flag;
        if !(flags != 0 && oldflags != 0) {
            if oldflags ^ flags != 0 {
                clip
                    .expect(
                        "non-null function pointer",
                    )(
                    &mut *in_0.offset(j as isize),
                    &mut *in_0.offset(i as isize),
                    &mut *out.offset(k as isize),
                );
                (*out.offset(k as isize)).flags = 0 as libc::c_int;
                if (*out.offset(k as isize)).u < r_refdef.aliasvrect.x {
                    (*out.offset(k as isize)).flags |= 0x1 as libc::c_int;
                }
                if (*out.offset(k as isize)).v < r_refdef.aliasvrect.y {
                    (*out.offset(k as isize)).flags |= 0x2 as libc::c_int;
                }
                if (*out.offset(k as isize)).u > r_refdef.aliasvrectright {
                    (*out.offset(k as isize)).flags |= 0x4 as libc::c_int;
                }
                if (*out.offset(k as isize)).v > r_refdef.aliasvrectbottom {
                    (*out.offset(k as isize)).flags |= 0x8 as libc::c_int;
                }
                k += 1;
            }
            if flags == 0 {
                *out.offset(k as isize) = *in_0.offset(i as isize);
                k += 1;
            }
        }
        j = i;
        i += 1;
    }
    return k;
}
#[no_mangle]
pub unsafe extern "C" fn R_AliasClipTriangle(
    mut index0: *mut finalvert_t,
    mut index1: *mut finalvert_t,
    mut index2: *mut finalvert_t,
) {
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut pingpong: libc::c_int = 0;
    let mut clipflags: libc::c_uint = 0;
    fv[0 as libc::c_int as usize][0 as libc::c_int as usize] = *index0;
    fv[0 as libc::c_int as usize][1 as libc::c_int as usize] = *index1;
    fv[0 as libc::c_int as usize][2 as libc::c_int as usize] = *index2;
    clipflags = (fv[0 as libc::c_int as usize][0 as libc::c_int as usize].flags
        | fv[0 as libc::c_int as usize][1 as libc::c_int as usize].flags
        | fv[0 as libc::c_int as usize][2 as libc::c_int as usize].flags)
        as libc::c_uint;
    if clipflags & 0x10 as libc::c_int as libc::c_uint != 0 {
        k = R_AliasClip(
            (fv[0 as libc::c_int as usize]).as_mut_ptr(),
            (fv[1 as libc::c_int as usize]).as_mut_ptr(),
            0x10 as libc::c_int,
            3 as libc::c_int,
            Some(
                R_Alias_clip_z
                    as unsafe extern "C" fn(
                        *mut finalvert_t,
                        *mut finalvert_t,
                        *mut finalvert_t,
                    ) -> (),
            ),
        );
        if k == 0 as libc::c_int {
            return;
        }
        pingpong = 1 as libc::c_int;
        clipflags = (fv[1 as libc::c_int as usize][0 as libc::c_int as usize].flags
            | fv[1 as libc::c_int as usize][1 as libc::c_int as usize].flags
            | fv[1 as libc::c_int as usize][2 as libc::c_int as usize].flags)
            as libc::c_uint;
    } else {
        pingpong = 0 as libc::c_int;
        k = 3 as libc::c_int;
    }
    if clipflags & 0x1 as libc::c_int as libc::c_uint != 0 {
        k = R_AliasClip(
            (fv[pingpong as usize]).as_mut_ptr(),
            (fv[(pingpong ^ 1 as libc::c_int) as usize]).as_mut_ptr(),
            0x1 as libc::c_int,
            k,
            Some(
                R_Alias_clip_left
                    as unsafe extern "C" fn(
                        *mut finalvert_t,
                        *mut finalvert_t,
                        *mut finalvert_t,
                    ) -> (),
            ),
        );
        if k == 0 as libc::c_int {
            return;
        }
        pingpong ^= 1 as libc::c_int;
    }
    if clipflags & 0x4 as libc::c_int as libc::c_uint != 0 {
        k = R_AliasClip(
            (fv[pingpong as usize]).as_mut_ptr(),
            (fv[(pingpong ^ 1 as libc::c_int) as usize]).as_mut_ptr(),
            0x4 as libc::c_int,
            k,
            Some(
                R_Alias_clip_right
                    as unsafe extern "C" fn(
                        *mut finalvert_t,
                        *mut finalvert_t,
                        *mut finalvert_t,
                    ) -> (),
            ),
        );
        if k == 0 as libc::c_int {
            return;
        }
        pingpong ^= 1 as libc::c_int;
    }
    if clipflags & 0x8 as libc::c_int as libc::c_uint != 0 {
        k = R_AliasClip(
            (fv[pingpong as usize]).as_mut_ptr(),
            (fv[(pingpong ^ 1 as libc::c_int) as usize]).as_mut_ptr(),
            0x8 as libc::c_int,
            k,
            Some(
                R_Alias_clip_bottom
                    as unsafe extern "C" fn(
                        *mut finalvert_t,
                        *mut finalvert_t,
                        *mut finalvert_t,
                    ) -> (),
            ),
        );
        if k == 0 as libc::c_int {
            return;
        }
        pingpong ^= 1 as libc::c_int;
    }
    if clipflags & 0x2 as libc::c_int as libc::c_uint != 0 {
        k = R_AliasClip(
            (fv[pingpong as usize]).as_mut_ptr(),
            (fv[(pingpong ^ 1 as libc::c_int) as usize]).as_mut_ptr(),
            0x2 as libc::c_int,
            k,
            Some(
                R_Alias_clip_top
                    as unsafe extern "C" fn(
                        *mut finalvert_t,
                        *mut finalvert_t,
                        *mut finalvert_t,
                    ) -> (),
            ),
        );
        if k == 0 as libc::c_int {
            return;
        }
        pingpong ^= 1 as libc::c_int;
    }
    i = 0 as libc::c_int;
    while i < k {
        if fv[pingpong as usize][i as usize].u < r_refdef.aliasvrect.x {
            fv[pingpong as usize][i as usize].u = r_refdef.aliasvrect.x;
        } else if fv[pingpong as usize][i as usize].u > r_refdef.aliasvrectright {
            fv[pingpong as usize][i as usize].u = r_refdef.aliasvrectright;
        }
        if fv[pingpong as usize][i as usize].v < r_refdef.aliasvrect.y {
            fv[pingpong as usize][i as usize].v = r_refdef.aliasvrect.y;
        } else if fv[pingpong as usize][i as usize].v > r_refdef.aliasvrectbottom {
            fv[pingpong as usize][i as usize].v = r_refdef.aliasvrectbottom;
        }
        fv[pingpong as usize][i as usize].flags = 0 as libc::c_int;
        i += 1;
    }
    i = 1 as libc::c_int;
    while i < k - 1 as libc::c_int {
        aliastriangleparms
            .a = &mut *(*fv.as_mut_ptr().offset(pingpong as isize))
            .as_mut_ptr()
            .offset(0 as libc::c_int as isize) as *mut finalvert_t;
        aliastriangleparms
            .b = &mut *(*fv.as_mut_ptr().offset(pingpong as isize))
            .as_mut_ptr()
            .offset(i as isize) as *mut finalvert_t;
        aliastriangleparms
            .c = &mut *(*fv.as_mut_ptr().offset(pingpong as isize))
            .as_mut_ptr()
            .offset((i + 1 as libc::c_int) as isize) as *mut finalvert_t;
        R_DrawTriangle();
        i += 1;
    }
}
