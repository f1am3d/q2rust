#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(extern_types, register_tool)]
extern "C" {
    pub type edict_s;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    static mut vec3_origin: vec3_t;
    fn VectorMA(
        veca: *mut vec_t,
        scale: libc::c_float,
        vecb: *mut vec_t,
        vecc: *mut vec_t,
    );
    fn VectorLength(v: *mut vec_t) -> vec_t;
    fn CrossProduct(v1: *mut vec_t, v2: *mut vec_t, cross: *mut vec_t);
    fn VectorNormalize(v: *mut vec_t) -> vec_t;
    fn VectorScale(in_0: *mut vec_t, scale: vec_t, out: *mut vec_t);
    fn AngleVectors(
        angles: *mut vec_t,
        forward: *mut vec_t,
        right: *mut vec_t,
        up: *mut vec_t,
    );
    fn Com_DPrintf(fmt: *mut libc::c_char, _: ...);
}
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
pub type cplane_t = cplane_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct csurface_s {
    pub name: [libc::c_char; 16],
    pub flags: libc::c_int,
    pub value: libc::c_int,
}
pub type csurface_t = csurface_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct trace_t {
    pub allsolid: qboolean,
    pub startsolid: qboolean,
    pub fraction: libc::c_float,
    pub endpos: vec3_t,
    pub plane: cplane_t,
    pub surface: *mut csurface_t,
    pub contents: libc::c_int,
    pub ent: *mut edict_s,
}
pub type pmtype_t = libc::c_uint;
pub const PM_FREEZE: pmtype_t = 4;
pub const PM_GIB: pmtype_t = 3;
pub const PM_DEAD: pmtype_t = 2;
pub const PM_SPECTATOR: pmtype_t = 1;
pub const PM_NORMAL: pmtype_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pmove_state_t {
    pub pm_type: pmtype_t,
    pub origin: [libc::c_short; 3],
    pub velocity: [libc::c_short; 3],
    pub pm_flags: byte,
    pub pm_time: byte,
    pub gravity: libc::c_short,
    pub delta_angles: [libc::c_short; 3],
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
pub struct pmove_t {
    pub s: pmove_state_t,
    pub cmd: usercmd_t,
    pub snapinitial: qboolean,
    pub numtouch: libc::c_int,
    pub touchents: [*mut edict_s; 32],
    pub viewangles: vec3_t,
    pub viewheight: libc::c_float,
    pub mins: vec3_t,
    pub maxs: vec3_t,
    pub groundentity: *mut edict_s,
    pub watertype: libc::c_int,
    pub waterlevel: libc::c_int,
    pub trace: Option::<
        unsafe extern "C" fn(*mut vec_t, *mut vec_t, *mut vec_t, *mut vec_t) -> trace_t,
    >,
    pub pointcontents: Option::<unsafe extern "C" fn(*mut vec_t) -> libc::c_int>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pml_t {
    pub origin: vec3_t,
    pub velocity: vec3_t,
    pub forward: vec3_t,
    pub right: vec3_t,
    pub up: vec3_t,
    pub frametime: libc::c_float,
    pub groundsurface: *mut csurface_t,
    pub groundplane: cplane_t,
    pub groundcontents: libc::c_int,
    pub previous_origin: vec3_t,
    pub ladder: qboolean,
}
#[no_mangle]
pub static mut pm: *mut pmove_t = 0 as *const pmove_t as *mut pmove_t;
#[no_mangle]
pub static mut pml: pml_t = pml_t {
    origin: [0.; 3],
    velocity: [0.; 3],
    forward: [0.; 3],
    right: [0.; 3],
    up: [0.; 3],
    frametime: 0.,
    groundsurface: 0 as *const csurface_t as *mut csurface_t,
    groundplane: cplane_t {
        normal: [0.; 3],
        dist: 0.,
        type_0: 0,
        signbits: 0,
        pad: [0; 2],
    },
    groundcontents: 0,
    previous_origin: [0.; 3],
    ladder: false_0,
};
#[no_mangle]
pub static mut pm_stopspeed: libc::c_float = 100 as libc::c_int as libc::c_float;
#[no_mangle]
pub static mut pm_maxspeed: libc::c_float = 300 as libc::c_int as libc::c_float;
#[no_mangle]
pub static mut pm_duckspeed: libc::c_float = 100 as libc::c_int as libc::c_float;
#[no_mangle]
pub static mut pm_accelerate: libc::c_float = 10 as libc::c_int as libc::c_float;
#[no_mangle]
pub static mut pm_airaccelerate: libc::c_float = 0 as libc::c_int as libc::c_float;
#[no_mangle]
pub static mut pm_wateraccelerate: libc::c_float = 10 as libc::c_int as libc::c_float;
#[no_mangle]
pub static mut pm_friction: libc::c_float = 6 as libc::c_int as libc::c_float;
#[no_mangle]
pub static mut pm_waterfriction: libc::c_float = 1 as libc::c_int as libc::c_float;
#[no_mangle]
pub static mut pm_waterspeed: libc::c_float = 400 as libc::c_int as libc::c_float;
#[no_mangle]
pub unsafe extern "C" fn PM_ClipVelocity(
    mut in_0: *mut vec_t,
    mut normal: *mut vec_t,
    mut out: *mut vec_t,
    mut overbounce: libc::c_float,
) {
    let mut backoff: libc::c_float = 0.;
    let mut change: libc::c_float = 0.;
    let mut i: libc::c_int = 0;
    backoff = (*in_0.offset(0 as libc::c_int as isize)
        * *normal.offset(0 as libc::c_int as isize)
        + *in_0.offset(1 as libc::c_int as isize)
            * *normal.offset(1 as libc::c_int as isize)
        + *in_0.offset(2 as libc::c_int as isize)
            * *normal.offset(2 as libc::c_int as isize)) * overbounce;
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        change = *normal.offset(i as isize) * backoff;
        *out.offset(i as isize) = *in_0.offset(i as isize) - change;
        if *out.offset(i as isize) as libc::c_double > -0.1f64
            && (*out.offset(i as isize) as libc::c_double) < 0.1f64
        {
            *out.offset(i as isize) = 0 as libc::c_int as vec_t;
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn PM_StepSlideMove_() {
    let mut bumpcount: libc::c_int = 0;
    let mut numbumps: libc::c_int = 0;
    let mut dir: vec3_t = [0.; 3];
    let mut d: libc::c_float = 0.;
    let mut numplanes: libc::c_int = 0;
    let mut planes: [vec3_t; 5] = [[0.; 3]; 5];
    let mut primal_velocity: vec3_t = [0.; 3];
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut trace: trace_t = trace_t {
        allsolid: false_0,
        startsolid: false_0,
        fraction: 0.,
        endpos: [0.; 3],
        plane: cplane_t {
            normal: [0.; 3],
            dist: 0.,
            type_0: 0,
            signbits: 0,
            pad: [0; 2],
        },
        surface: 0 as *mut csurface_t,
        contents: 0,
        ent: 0 as *mut edict_s,
    };
    let mut end: vec3_t = [0.; 3];
    let mut time_left: libc::c_float = 0.;
    numbumps = 4 as libc::c_int;
    primal_velocity[0 as libc::c_int as usize] = pml.velocity[0 as libc::c_int as usize];
    primal_velocity[1 as libc::c_int as usize] = pml.velocity[1 as libc::c_int as usize];
    primal_velocity[2 as libc::c_int as usize] = pml.velocity[2 as libc::c_int as usize];
    numplanes = 0 as libc::c_int;
    time_left = pml.frametime;
    bumpcount = 0 as libc::c_int;
    while bumpcount < numbumps {
        i = 0 as libc::c_int;
        while i < 3 as libc::c_int {
            end[i
                as usize] = pml.origin[i as usize]
                + time_left * pml.velocity[i as usize];
            i += 1;
        }
        trace = ((*pm).trace)
            .expect(
                "non-null function pointer",
            )(
            (pml.origin).as_mut_ptr(),
            ((*pm).mins).as_mut_ptr(),
            ((*pm).maxs).as_mut_ptr(),
            end.as_mut_ptr(),
        );
        if trace.allsolid as u64 != 0 {
            pml.velocity[2 as libc::c_int as usize] = 0 as libc::c_int as vec_t;
            return;
        }
        if trace.fraction > 0 as libc::c_int as libc::c_float {
            pml
                .origin[0 as libc::c_int
                as usize] = trace.endpos[0 as libc::c_int as usize];
            pml
                .origin[1 as libc::c_int
                as usize] = trace.endpos[1 as libc::c_int as usize];
            pml
                .origin[2 as libc::c_int
                as usize] = trace.endpos[2 as libc::c_int as usize];
            numplanes = 0 as libc::c_int;
        }
        if trace.fraction == 1 as libc::c_int as libc::c_float {
            break;
        }
        if (*pm).numtouch < 32 as libc::c_int && !(trace.ent).is_null() {
            let ref mut fresh0 = (*pm).touchents[(*pm).numtouch as usize];
            *fresh0 = trace.ent;
            let ref mut fresh1 = (*pm).numtouch;
            *fresh1 += 1;
        }
        time_left -= time_left * trace.fraction;
        if numplanes >= 5 as libc::c_int {
            pml
                .velocity[0 as libc::c_int
                as usize] = vec3_origin[0 as libc::c_int as usize];
            pml
                .velocity[1 as libc::c_int
                as usize] = vec3_origin[1 as libc::c_int as usize];
            pml
                .velocity[2 as libc::c_int
                as usize] = vec3_origin[2 as libc::c_int as usize];
            break;
        } else {
            planes[numplanes
                as usize][0 as libc::c_int
                as usize] = trace.plane.normal[0 as libc::c_int as usize];
            planes[numplanes
                as usize][1 as libc::c_int
                as usize] = trace.plane.normal[1 as libc::c_int as usize];
            planes[numplanes
                as usize][2 as libc::c_int
                as usize] = trace.plane.normal[2 as libc::c_int as usize];
            numplanes += 1;
            i = 0 as libc::c_int;
            while i < numplanes {
                PM_ClipVelocity(
                    (pml.velocity).as_mut_ptr(),
                    (planes[i as usize]).as_mut_ptr(),
                    (pml.velocity).as_mut_ptr(),
                    1.01f64 as libc::c_float,
                );
                j = 0 as libc::c_int;
                while j < numplanes {
                    if j != i {
                        if pml.velocity[0 as libc::c_int as usize]
                            * planes[j as usize][0 as libc::c_int as usize]
                            + pml.velocity[1 as libc::c_int as usize]
                                * planes[j as usize][1 as libc::c_int as usize]
                            + pml.velocity[2 as libc::c_int as usize]
                                * planes[j as usize][2 as libc::c_int as usize]
                            < 0 as libc::c_int as libc::c_float
                        {
                            break;
                        }
                    }
                    j += 1;
                }
                if j == numplanes {
                    break;
                }
                i += 1;
            }
            if !(i != numplanes) {
                if numplanes != 2 as libc::c_int {
                    pml
                        .velocity[0 as libc::c_int
                        as usize] = vec3_origin[0 as libc::c_int as usize];
                    pml
                        .velocity[1 as libc::c_int
                        as usize] = vec3_origin[1 as libc::c_int as usize];
                    pml
                        .velocity[2 as libc::c_int
                        as usize] = vec3_origin[2 as libc::c_int as usize];
                    break;
                } else {
                    CrossProduct(
                        (planes[0 as libc::c_int as usize]).as_mut_ptr(),
                        (planes[1 as libc::c_int as usize]).as_mut_ptr(),
                        dir.as_mut_ptr(),
                    );
                    d = dir[0 as libc::c_int as usize]
                        * pml.velocity[0 as libc::c_int as usize]
                        + dir[1 as libc::c_int as usize]
                            * pml.velocity[1 as libc::c_int as usize]
                        + dir[2 as libc::c_int as usize]
                            * pml.velocity[2 as libc::c_int as usize];
                    VectorScale(dir.as_mut_ptr(), d, (pml.velocity).as_mut_ptr());
                }
            }
            if pml.velocity[0 as libc::c_int as usize]
                * primal_velocity[0 as libc::c_int as usize]
                + pml.velocity[1 as libc::c_int as usize]
                    * primal_velocity[1 as libc::c_int as usize]
                + pml.velocity[2 as libc::c_int as usize]
                    * primal_velocity[2 as libc::c_int as usize]
                <= 0 as libc::c_int as libc::c_float
            {
                pml
                    .velocity[0 as libc::c_int
                    as usize] = vec3_origin[0 as libc::c_int as usize];
                pml
                    .velocity[1 as libc::c_int
                    as usize] = vec3_origin[1 as libc::c_int as usize];
                pml
                    .velocity[2 as libc::c_int
                    as usize] = vec3_origin[2 as libc::c_int as usize];
                break;
            } else {
                bumpcount += 1;
            }
        }
    }
    if (*pm).s.pm_time != 0 {
        pml
            .velocity[0 as libc::c_int
            as usize] = primal_velocity[0 as libc::c_int as usize];
        pml
            .velocity[1 as libc::c_int
            as usize] = primal_velocity[1 as libc::c_int as usize];
        pml
            .velocity[2 as libc::c_int
            as usize] = primal_velocity[2 as libc::c_int as usize];
    }
}
#[no_mangle]
pub unsafe extern "C" fn PM_StepSlideMove() {
    let mut start_o: vec3_t = [0.; 3];
    let mut start_v: vec3_t = [0.; 3];
    let mut down_o: vec3_t = [0.; 3];
    let mut down_v: vec3_t = [0.; 3];
    let mut trace: trace_t = trace_t {
        allsolid: false_0,
        startsolid: false_0,
        fraction: 0.,
        endpos: [0.; 3],
        plane: cplane_t {
            normal: [0.; 3],
            dist: 0.,
            type_0: 0,
            signbits: 0,
            pad: [0; 2],
        },
        surface: 0 as *mut csurface_t,
        contents: 0,
        ent: 0 as *mut edict_s,
    };
    let mut down_dist: libc::c_float = 0.;
    let mut up_dist: libc::c_float = 0.;
    let mut up: vec3_t = [0.; 3];
    let mut down: vec3_t = [0.; 3];
    start_o[0 as libc::c_int as usize] = pml.origin[0 as libc::c_int as usize];
    start_o[1 as libc::c_int as usize] = pml.origin[1 as libc::c_int as usize];
    start_o[2 as libc::c_int as usize] = pml.origin[2 as libc::c_int as usize];
    start_v[0 as libc::c_int as usize] = pml.velocity[0 as libc::c_int as usize];
    start_v[1 as libc::c_int as usize] = pml.velocity[1 as libc::c_int as usize];
    start_v[2 as libc::c_int as usize] = pml.velocity[2 as libc::c_int as usize];
    PM_StepSlideMove_();
    down_o[0 as libc::c_int as usize] = pml.origin[0 as libc::c_int as usize];
    down_o[1 as libc::c_int as usize] = pml.origin[1 as libc::c_int as usize];
    down_o[2 as libc::c_int as usize] = pml.origin[2 as libc::c_int as usize];
    down_v[0 as libc::c_int as usize] = pml.velocity[0 as libc::c_int as usize];
    down_v[1 as libc::c_int as usize] = pml.velocity[1 as libc::c_int as usize];
    down_v[2 as libc::c_int as usize] = pml.velocity[2 as libc::c_int as usize];
    up[0 as libc::c_int as usize] = start_o[0 as libc::c_int as usize];
    up[1 as libc::c_int as usize] = start_o[1 as libc::c_int as usize];
    up[2 as libc::c_int as usize] = start_o[2 as libc::c_int as usize];
    up[2 as libc::c_int as usize] += 18 as libc::c_int as libc::c_float;
    trace = ((*pm).trace)
        .expect(
            "non-null function pointer",
        )(
        up.as_mut_ptr(),
        ((*pm).mins).as_mut_ptr(),
        ((*pm).maxs).as_mut_ptr(),
        up.as_mut_ptr(),
    );
    if trace.allsolid as u64 != 0 {
        return;
    }
    pml.origin[0 as libc::c_int as usize] = up[0 as libc::c_int as usize];
    pml.origin[1 as libc::c_int as usize] = up[1 as libc::c_int as usize];
    pml.origin[2 as libc::c_int as usize] = up[2 as libc::c_int as usize];
    pml.velocity[0 as libc::c_int as usize] = start_v[0 as libc::c_int as usize];
    pml.velocity[1 as libc::c_int as usize] = start_v[1 as libc::c_int as usize];
    pml.velocity[2 as libc::c_int as usize] = start_v[2 as libc::c_int as usize];
    PM_StepSlideMove_();
    down[0 as libc::c_int as usize] = pml.origin[0 as libc::c_int as usize];
    down[1 as libc::c_int as usize] = pml.origin[1 as libc::c_int as usize];
    down[2 as libc::c_int as usize] = pml.origin[2 as libc::c_int as usize];
    down[2 as libc::c_int as usize] -= 18 as libc::c_int as libc::c_float;
    trace = ((*pm).trace)
        .expect(
            "non-null function pointer",
        )(
        (pml.origin).as_mut_ptr(),
        ((*pm).mins).as_mut_ptr(),
        ((*pm).maxs).as_mut_ptr(),
        down.as_mut_ptr(),
    );
    if trace.allsolid as u64 == 0 {
        pml.origin[0 as libc::c_int as usize] = trace.endpos[0 as libc::c_int as usize];
        pml.origin[1 as libc::c_int as usize] = trace.endpos[1 as libc::c_int as usize];
        pml.origin[2 as libc::c_int as usize] = trace.endpos[2 as libc::c_int as usize];
    }
    up[0 as libc::c_int as usize] = pml.origin[0 as libc::c_int as usize];
    up[1 as libc::c_int as usize] = pml.origin[1 as libc::c_int as usize];
    up[2 as libc::c_int as usize] = pml.origin[2 as libc::c_int as usize];
    down_dist = (down_o[0 as libc::c_int as usize] - start_o[0 as libc::c_int as usize])
        * (down_o[0 as libc::c_int as usize] - start_o[0 as libc::c_int as usize])
        + (down_o[1 as libc::c_int as usize] - start_o[1 as libc::c_int as usize])
            * (down_o[1 as libc::c_int as usize] - start_o[1 as libc::c_int as usize]);
    up_dist = (up[0 as libc::c_int as usize] - start_o[0 as libc::c_int as usize])
        * (up[0 as libc::c_int as usize] - start_o[0 as libc::c_int as usize])
        + (up[1 as libc::c_int as usize] - start_o[1 as libc::c_int as usize])
            * (up[1 as libc::c_int as usize] - start_o[1 as libc::c_int as usize]);
    if down_dist > up_dist
        || (trace.plane.normal[2 as libc::c_int as usize] as libc::c_double) < 0.7f64
    {
        pml.origin[0 as libc::c_int as usize] = down_o[0 as libc::c_int as usize];
        pml.origin[1 as libc::c_int as usize] = down_o[1 as libc::c_int as usize];
        pml.origin[2 as libc::c_int as usize] = down_o[2 as libc::c_int as usize];
        pml.velocity[0 as libc::c_int as usize] = down_v[0 as libc::c_int as usize];
        pml.velocity[1 as libc::c_int as usize] = down_v[1 as libc::c_int as usize];
        pml.velocity[2 as libc::c_int as usize] = down_v[2 as libc::c_int as usize];
        return;
    }
    pml.velocity[2 as libc::c_int as usize] = down_v[2 as libc::c_int as usize];
}
#[no_mangle]
pub unsafe extern "C" fn PM_Friction() {
    let mut vel: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut speed: libc::c_float = 0.;
    let mut newspeed: libc::c_float = 0.;
    let mut control: libc::c_float = 0.;
    let mut friction: libc::c_float = 0.;
    let mut drop_0: libc::c_float = 0.;
    vel = (pml.velocity).as_mut_ptr();
    speed = sqrt(
        (*vel.offset(0 as libc::c_int as isize) * *vel.offset(0 as libc::c_int as isize)
            + *vel.offset(1 as libc::c_int as isize)
                * *vel.offset(1 as libc::c_int as isize)
            + *vel.offset(2 as libc::c_int as isize)
                * *vel.offset(2 as libc::c_int as isize)) as libc::c_double,
    ) as libc::c_float;
    if speed < 1 as libc::c_int as libc::c_float {
        *vel.offset(0 as libc::c_int as isize) = 0 as libc::c_int as libc::c_float;
        *vel.offset(1 as libc::c_int as isize) = 0 as libc::c_int as libc::c_float;
        return;
    }
    drop_0 = 0 as libc::c_int as libc::c_float;
    if !((*pm).groundentity).is_null() && !(pml.groundsurface).is_null()
        && (*pml.groundsurface).flags & 0x2 as libc::c_int == 0
        || pml.ladder as libc::c_uint != 0
    {
        friction = pm_friction;
        control = if speed < pm_stopspeed { pm_stopspeed } else { speed };
        drop_0 += control * friction * pml.frametime;
    }
    if (*pm).waterlevel != 0 && pml.ladder as u64 == 0 {
        drop_0
            += speed * pm_waterfriction * (*pm).waterlevel as libc::c_float
                * pml.frametime;
    }
    newspeed = speed - drop_0;
    if newspeed < 0 as libc::c_int as libc::c_float {
        newspeed = 0 as libc::c_int as libc::c_float;
    }
    newspeed /= speed;
    *vel
        .offset(
            0 as libc::c_int as isize,
        ) = *vel.offset(0 as libc::c_int as isize) * newspeed;
    *vel
        .offset(
            1 as libc::c_int as isize,
        ) = *vel.offset(1 as libc::c_int as isize) * newspeed;
    *vel
        .offset(
            2 as libc::c_int as isize,
        ) = *vel.offset(2 as libc::c_int as isize) * newspeed;
}
#[no_mangle]
pub unsafe extern "C" fn PM_Accelerate(
    mut wishdir: *mut vec_t,
    mut wishspeed: libc::c_float,
    mut accel: libc::c_float,
) {
    let mut i: libc::c_int = 0;
    let mut addspeed: libc::c_float = 0.;
    let mut accelspeed: libc::c_float = 0.;
    let mut currentspeed: libc::c_float = 0.;
    currentspeed = pml.velocity[0 as libc::c_int as usize]
        * *wishdir.offset(0 as libc::c_int as isize)
        + pml.velocity[1 as libc::c_int as usize]
            * *wishdir.offset(1 as libc::c_int as isize)
        + pml.velocity[2 as libc::c_int as usize]
            * *wishdir.offset(2 as libc::c_int as isize);
    addspeed = wishspeed - currentspeed;
    if addspeed <= 0 as libc::c_int as libc::c_float {
        return;
    }
    accelspeed = accel * pml.frametime * wishspeed;
    if accelspeed > addspeed {
        accelspeed = addspeed;
    }
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        pml.velocity[i as usize] += accelspeed * *wishdir.offset(i as isize);
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn PM_AirAccelerate(
    mut wishdir: *mut vec_t,
    mut wishspeed: libc::c_float,
    mut accel: libc::c_float,
) {
    let mut i: libc::c_int = 0;
    let mut addspeed: libc::c_float = 0.;
    let mut accelspeed: libc::c_float = 0.;
    let mut currentspeed: libc::c_float = 0.;
    let mut wishspd: libc::c_float = wishspeed;
    if wishspd > 30 as libc::c_int as libc::c_float {
        wishspd = 30 as libc::c_int as libc::c_float;
    }
    currentspeed = pml.velocity[0 as libc::c_int as usize]
        * *wishdir.offset(0 as libc::c_int as isize)
        + pml.velocity[1 as libc::c_int as usize]
            * *wishdir.offset(1 as libc::c_int as isize)
        + pml.velocity[2 as libc::c_int as usize]
            * *wishdir.offset(2 as libc::c_int as isize);
    addspeed = wishspd - currentspeed;
    if addspeed <= 0 as libc::c_int as libc::c_float {
        return;
    }
    accelspeed = accel * wishspeed * pml.frametime;
    if accelspeed > addspeed {
        accelspeed = addspeed;
    }
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        pml.velocity[i as usize] += accelspeed * *wishdir.offset(i as isize);
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn PM_AddCurrents(mut wishvel: *mut vec_t) {
    let mut v: vec3_t = [0.; 3];
    let mut s: libc::c_float = 0.;
    if pml.ladder as libc::c_uint != 0
        && fabs(pml.velocity[2 as libc::c_int as usize] as libc::c_double)
            <= 200 as libc::c_int as libc::c_double
    {
        if (*pm).viewangles[0 as libc::c_int as usize]
            <= -(15 as libc::c_int) as libc::c_float
            && (*pm).cmd.forwardmove as libc::c_int > 0 as libc::c_int
        {
            *wishvel.offset(2 as libc::c_int as isize) = 200 as libc::c_int as vec_t;
        } else if (*pm).viewangles[0 as libc::c_int as usize]
            >= 15 as libc::c_int as libc::c_float
            && (*pm).cmd.forwardmove as libc::c_int > 0 as libc::c_int
        {
            *wishvel.offset(2 as libc::c_int as isize) = -(200 as libc::c_int) as vec_t;
        } else if (*pm).cmd.upmove as libc::c_int > 0 as libc::c_int {
            *wishvel.offset(2 as libc::c_int as isize) = 200 as libc::c_int as vec_t;
        } else if ((*pm).cmd.upmove as libc::c_int) < 0 as libc::c_int {
            *wishvel.offset(2 as libc::c_int as isize) = -(200 as libc::c_int) as vec_t;
        } else {
            *wishvel.offset(2 as libc::c_int as isize) = 0 as libc::c_int as vec_t;
        }
        if *wishvel.offset(0 as libc::c_int as isize)
            < -(25 as libc::c_int) as libc::c_float
        {
            *wishvel.offset(0 as libc::c_int as isize) = -(25 as libc::c_int) as vec_t;
        } else if *wishvel.offset(0 as libc::c_int as isize)
            > 25 as libc::c_int as libc::c_float
        {
            *wishvel.offset(0 as libc::c_int as isize) = 25 as libc::c_int as vec_t;
        }
        if *wishvel.offset(1 as libc::c_int as isize)
            < -(25 as libc::c_int) as libc::c_float
        {
            *wishvel.offset(1 as libc::c_int as isize) = -(25 as libc::c_int) as vec_t;
        } else if *wishvel.offset(1 as libc::c_int as isize)
            > 25 as libc::c_int as libc::c_float
        {
            *wishvel.offset(1 as libc::c_int as isize) = 25 as libc::c_int as vec_t;
        }
    }
    if (*pm).watertype
        & (0x40000 as libc::c_int | 0x80000 as libc::c_int | 0x100000 as libc::c_int
            | 0x200000 as libc::c_int | 0x400000 as libc::c_int
            | 0x800000 as libc::c_int) != 0
    {
        v[2 as libc::c_int as usize] = 0 as libc::c_int as vec_t;
        v[1 as libc::c_int as usize] = v[2 as libc::c_int as usize];
        v[0 as libc::c_int as usize] = v[1 as libc::c_int as usize];
        if (*pm).watertype & 0x40000 as libc::c_int != 0 {
            v[0 as libc::c_int as usize] += 1 as libc::c_int as libc::c_float;
        }
        if (*pm).watertype & 0x80000 as libc::c_int != 0 {
            v[1 as libc::c_int as usize] += 1 as libc::c_int as libc::c_float;
        }
        if (*pm).watertype & 0x100000 as libc::c_int != 0 {
            v[0 as libc::c_int as usize] -= 1 as libc::c_int as libc::c_float;
        }
        if (*pm).watertype & 0x200000 as libc::c_int != 0 {
            v[1 as libc::c_int as usize] -= 1 as libc::c_int as libc::c_float;
        }
        if (*pm).watertype & 0x400000 as libc::c_int != 0 {
            v[2 as libc::c_int as usize] += 1 as libc::c_int as libc::c_float;
        }
        if (*pm).watertype & 0x800000 as libc::c_int != 0 {
            v[2 as libc::c_int as usize] -= 1 as libc::c_int as libc::c_float;
        }
        s = pm_waterspeed;
        if (*pm).waterlevel == 1 as libc::c_int && !((*pm).groundentity).is_null() {
            s /= 2 as libc::c_int as libc::c_float;
        }
        VectorMA(wishvel, s, v.as_mut_ptr(), wishvel);
    }
    if !((*pm).groundentity).is_null() {
        v[2 as libc::c_int as usize] = 0 as libc::c_int as vec_t;
        v[1 as libc::c_int as usize] = v[2 as libc::c_int as usize];
        v[0 as libc::c_int as usize] = v[1 as libc::c_int as usize];
        if pml.groundcontents & 0x40000 as libc::c_int != 0 {
            v[0 as libc::c_int as usize] += 1 as libc::c_int as libc::c_float;
        }
        if pml.groundcontents & 0x80000 as libc::c_int != 0 {
            v[1 as libc::c_int as usize] += 1 as libc::c_int as libc::c_float;
        }
        if pml.groundcontents & 0x100000 as libc::c_int != 0 {
            v[0 as libc::c_int as usize] -= 1 as libc::c_int as libc::c_float;
        }
        if pml.groundcontents & 0x200000 as libc::c_int != 0 {
            v[1 as libc::c_int as usize] -= 1 as libc::c_int as libc::c_float;
        }
        if pml.groundcontents & 0x400000 as libc::c_int != 0 {
            v[2 as libc::c_int as usize] += 1 as libc::c_int as libc::c_float;
        }
        if pml.groundcontents & 0x800000 as libc::c_int != 0 {
            v[2 as libc::c_int as usize] -= 1 as libc::c_int as libc::c_float;
        }
        VectorMA(wishvel, 100 as libc::c_int as libc::c_float, v.as_mut_ptr(), wishvel);
    }
}
#[no_mangle]
pub unsafe extern "C" fn PM_WaterMove() {
    let mut i: libc::c_int = 0;
    let mut wishvel: vec3_t = [0.; 3];
    let mut wishspeed: libc::c_float = 0.;
    let mut wishdir: vec3_t = [0.; 3];
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        wishvel[i
            as usize] = pml.forward[i as usize]
            * (*pm).cmd.forwardmove as libc::c_int as libc::c_float
            + pml.right[i as usize] * (*pm).cmd.sidemove as libc::c_int as libc::c_float;
        i += 1;
    }
    if (*pm).cmd.forwardmove == 0 && (*pm).cmd.sidemove == 0 && (*pm).cmd.upmove == 0 {
        wishvel[2 as libc::c_int as usize] -= 60 as libc::c_int as libc::c_float;
    } else {
        wishvel[2 as libc::c_int as usize]
            += (*pm).cmd.upmove as libc::c_int as libc::c_float;
    }
    PM_AddCurrents(wishvel.as_mut_ptr());
    wishdir[0 as libc::c_int as usize] = wishvel[0 as libc::c_int as usize];
    wishdir[1 as libc::c_int as usize] = wishvel[1 as libc::c_int as usize];
    wishdir[2 as libc::c_int as usize] = wishvel[2 as libc::c_int as usize];
    wishspeed = VectorNormalize(wishdir.as_mut_ptr());
    if wishspeed > pm_maxspeed {
        VectorScale(wishvel.as_mut_ptr(), pm_maxspeed / wishspeed, wishvel.as_mut_ptr());
        wishspeed = pm_maxspeed;
    }
    wishspeed = (wishspeed as libc::c_double * 0.5f64) as libc::c_float;
    PM_Accelerate(wishdir.as_mut_ptr(), wishspeed, pm_wateraccelerate);
    PM_StepSlideMove();
}
#[no_mangle]
pub unsafe extern "C" fn PM_AirMove() {
    let mut i: libc::c_int = 0;
    let mut wishvel: vec3_t = [0.; 3];
    let mut fmove: libc::c_float = 0.;
    let mut smove: libc::c_float = 0.;
    let mut wishdir: vec3_t = [0.; 3];
    let mut wishspeed: libc::c_float = 0.;
    let mut maxspeed: libc::c_float = 0.;
    fmove = (*pm).cmd.forwardmove as libc::c_float;
    smove = (*pm).cmd.sidemove as libc::c_float;
    i = 0 as libc::c_int;
    while i < 2 as libc::c_int {
        wishvel[i
            as usize] = pml.forward[i as usize] * fmove + pml.right[i as usize] * smove;
        i += 1;
    }
    wishvel[2 as libc::c_int as usize] = 0 as libc::c_int as vec_t;
    PM_AddCurrents(wishvel.as_mut_ptr());
    wishdir[0 as libc::c_int as usize] = wishvel[0 as libc::c_int as usize];
    wishdir[1 as libc::c_int as usize] = wishvel[1 as libc::c_int as usize];
    wishdir[2 as libc::c_int as usize] = wishvel[2 as libc::c_int as usize];
    wishspeed = VectorNormalize(wishdir.as_mut_ptr());
    maxspeed = if (*pm).s.pm_flags as libc::c_int & 1 as libc::c_int != 0 {
        pm_duckspeed
    } else {
        pm_maxspeed
    };
    if wishspeed > maxspeed {
        VectorScale(wishvel.as_mut_ptr(), maxspeed / wishspeed, wishvel.as_mut_ptr());
        wishspeed = maxspeed;
    }
    if pml.ladder as u64 != 0 {
        PM_Accelerate(wishdir.as_mut_ptr(), wishspeed, pm_accelerate);
        if wishvel[2 as libc::c_int as usize] == 0. {
            if pml.velocity[2 as libc::c_int as usize]
                > 0 as libc::c_int as libc::c_float
            {
                pml.velocity[2 as libc::c_int as usize]
                    -= (*pm).s.gravity as libc::c_int as libc::c_float * pml.frametime;
                if pml.velocity[2 as libc::c_int as usize]
                    < 0 as libc::c_int as libc::c_float
                {
                    pml.velocity[2 as libc::c_int as usize] = 0 as libc::c_int as vec_t;
                }
            } else {
                pml.velocity[2 as libc::c_int as usize]
                    += (*pm).s.gravity as libc::c_int as libc::c_float * pml.frametime;
                if pml.velocity[2 as libc::c_int as usize]
                    > 0 as libc::c_int as libc::c_float
                {
                    pml.velocity[2 as libc::c_int as usize] = 0 as libc::c_int as vec_t;
                }
            }
        }
        PM_StepSlideMove();
    } else if !((*pm).groundentity).is_null() {
        pml.velocity[2 as libc::c_int as usize] = 0 as libc::c_int as vec_t;
        PM_Accelerate(wishdir.as_mut_ptr(), wishspeed, pm_accelerate);
        if (*pm).s.gravity as libc::c_int > 0 as libc::c_int {
            pml.velocity[2 as libc::c_int as usize] = 0 as libc::c_int as vec_t;
        } else {
            pml.velocity[2 as libc::c_int as usize]
                -= (*pm).s.gravity as libc::c_int as libc::c_float * pml.frametime;
        }
        if pml.velocity[0 as libc::c_int as usize] == 0.
            && pml.velocity[1 as libc::c_int as usize] == 0.
        {
            return;
        }
        PM_StepSlideMove();
    } else {
        if pm_airaccelerate != 0. {
            PM_AirAccelerate(wishdir.as_mut_ptr(), wishspeed, pm_accelerate);
        } else {
            PM_Accelerate(
                wishdir.as_mut_ptr(),
                wishspeed,
                1 as libc::c_int as libc::c_float,
            );
        }
        pml.velocity[2 as libc::c_int as usize]
            -= (*pm).s.gravity as libc::c_int as libc::c_float * pml.frametime;
        PM_StepSlideMove();
    };
}
#[no_mangle]
pub unsafe extern "C" fn PM_CatagorizePosition() {
    let mut point: vec3_t = [0.; 3];
    let mut cont: libc::c_int = 0;
    let mut trace: trace_t = trace_t {
        allsolid: false_0,
        startsolid: false_0,
        fraction: 0.,
        endpos: [0.; 3],
        plane: cplane_t {
            normal: [0.; 3],
            dist: 0.,
            type_0: 0,
            signbits: 0,
            pad: [0; 2],
        },
        surface: 0 as *mut csurface_t,
        contents: 0,
        ent: 0 as *mut edict_s,
    };
    let mut sample1: libc::c_int = 0;
    let mut sample2: libc::c_int = 0;
    point[0 as libc::c_int as usize] = pml.origin[0 as libc::c_int as usize];
    point[1 as libc::c_int as usize] = pml.origin[1 as libc::c_int as usize];
    point[2 as libc::c_int
        as usize] = (pml.origin[2 as libc::c_int as usize] as libc::c_double - 0.25f64)
        as vec_t;
    if pml.velocity[2 as libc::c_int as usize] > 180 as libc::c_int as libc::c_float {
        let ref mut fresh2 = (*pm).s.pm_flags;
        *fresh2 = (*fresh2 as libc::c_int & !(4 as libc::c_int)) as byte;
        let ref mut fresh3 = (*pm).groundentity;
        *fresh3 = 0 as *mut edict_s;
    } else {
        trace = ((*pm).trace)
            .expect(
                "non-null function pointer",
            )(
            (pml.origin).as_mut_ptr(),
            ((*pm).mins).as_mut_ptr(),
            ((*pm).maxs).as_mut_ptr(),
            point.as_mut_ptr(),
        );
        pml.groundplane = trace.plane;
        pml.groundsurface = trace.surface;
        pml.groundcontents = trace.contents;
        if (trace.ent).is_null()
            || (trace.plane.normal[2 as libc::c_int as usize] as libc::c_double) < 0.7f64
                && trace.startsolid as u64 == 0
        {
            let ref mut fresh4 = (*pm).groundentity;
            *fresh4 = 0 as *mut edict_s;
            let ref mut fresh5 = (*pm).s.pm_flags;
            *fresh5 = (*fresh5 as libc::c_int & !(4 as libc::c_int)) as byte;
        } else {
            let ref mut fresh6 = (*pm).groundentity;
            *fresh6 = trace.ent;
            if (*pm).s.pm_flags as libc::c_int & 8 as libc::c_int != 0 {
                let ref mut fresh7 = (*pm).s.pm_flags;
                *fresh7 = (*fresh7 as libc::c_int
                    & !(8 as libc::c_int | 16 as libc::c_int | 32 as libc::c_int))
                    as byte;
                (*pm).s.pm_time = 0 as libc::c_int as byte;
            }
            if (*pm).s.pm_flags as libc::c_int & 4 as libc::c_int == 0 {
                let ref mut fresh8 = (*pm).s.pm_flags;
                *fresh8 = (*fresh8 as libc::c_int | 4 as libc::c_int) as byte;
                if pml.velocity[2 as libc::c_int as usize]
                    < -(200 as libc::c_int) as libc::c_float
                {
                    let ref mut fresh9 = (*pm).s.pm_flags;
                    *fresh9 = (*fresh9 as libc::c_int | 16 as libc::c_int) as byte;
                    if pml.velocity[2 as libc::c_int as usize]
                        < -(400 as libc::c_int) as libc::c_float
                    {
                        (*pm).s.pm_time = 25 as libc::c_int as byte;
                    } else {
                        (*pm).s.pm_time = 18 as libc::c_int as byte;
                    }
                }
            }
        }
        if (*pm).numtouch < 32 as libc::c_int && !(trace.ent).is_null() {
            let ref mut fresh10 = (*pm).touchents[(*pm).numtouch as usize];
            *fresh10 = trace.ent;
            let ref mut fresh11 = (*pm).numtouch;
            *fresh11 += 1;
        }
    }
    (*pm).waterlevel = 0 as libc::c_int;
    (*pm).watertype = 0 as libc::c_int;
    sample2 = ((*pm).viewheight - (*pm).mins[2 as libc::c_int as usize]) as libc::c_int;
    sample1 = sample2 / 2 as libc::c_int;
    point[2 as libc::c_int
        as usize] = pml.origin[2 as libc::c_int as usize]
        + (*pm).mins[2 as libc::c_int as usize] + 1 as libc::c_int as libc::c_float;
    cont = ((*pm).pointcontents).expect("non-null function pointer")(point.as_mut_ptr());
    if cont & (32 as libc::c_int | 8 as libc::c_int | 16 as libc::c_int) != 0 {
        (*pm).watertype = cont;
        (*pm).waterlevel = 1 as libc::c_int;
        point[2 as libc::c_int
            as usize] = pml.origin[2 as libc::c_int as usize]
            + (*pm).mins[2 as libc::c_int as usize] + sample1 as libc::c_float;
        cont = ((*pm).pointcontents)
            .expect("non-null function pointer")(point.as_mut_ptr());
        if cont & (32 as libc::c_int | 8 as libc::c_int | 16 as libc::c_int) != 0 {
            (*pm).waterlevel = 2 as libc::c_int;
            point[2 as libc::c_int
                as usize] = pml.origin[2 as libc::c_int as usize]
                + (*pm).mins[2 as libc::c_int as usize] + sample2 as libc::c_float;
            cont = ((*pm).pointcontents)
                .expect("non-null function pointer")(point.as_mut_ptr());
            if cont & (32 as libc::c_int | 8 as libc::c_int | 16 as libc::c_int) != 0 {
                (*pm).waterlevel = 3 as libc::c_int;
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn PM_CheckJump() {
    if (*pm).s.pm_flags as libc::c_int & 16 as libc::c_int != 0 {
        return;
    }
    if ((*pm).cmd.upmove as libc::c_int) < 10 as libc::c_int {
        let ref mut fresh12 = (*pm).s.pm_flags;
        *fresh12 = (*fresh12 as libc::c_int & !(2 as libc::c_int)) as byte;
        return;
    }
    if (*pm).s.pm_flags as libc::c_int & 2 as libc::c_int != 0 {
        return;
    }
    if (*pm).s.pm_type as libc::c_uint == PM_DEAD as libc::c_int as libc::c_uint {
        return;
    }
    if (*pm).waterlevel >= 2 as libc::c_int {
        let ref mut fresh13 = (*pm).groundentity;
        *fresh13 = 0 as *mut edict_s;
        if pml.velocity[2 as libc::c_int as usize]
            <= -(300 as libc::c_int) as libc::c_float
        {
            return;
        }
        if (*pm).watertype == 32 as libc::c_int {
            pml.velocity[2 as libc::c_int as usize] = 100 as libc::c_int as vec_t;
        } else if (*pm).watertype == 16 as libc::c_int {
            pml.velocity[2 as libc::c_int as usize] = 80 as libc::c_int as vec_t;
        } else {
            pml.velocity[2 as libc::c_int as usize] = 50 as libc::c_int as vec_t;
        }
        return;
    }
    if ((*pm).groundentity).is_null() {
        return;
    }
    let ref mut fresh14 = (*pm).s.pm_flags;
    *fresh14 = (*fresh14 as libc::c_int | 2 as libc::c_int) as byte;
    let ref mut fresh15 = (*pm).groundentity;
    *fresh15 = 0 as *mut edict_s;
    pml.velocity[2 as libc::c_int as usize] += 270 as libc::c_int as libc::c_float;
    if pml.velocity[2 as libc::c_int as usize] < 270 as libc::c_int as libc::c_float {
        pml.velocity[2 as libc::c_int as usize] = 270 as libc::c_int as vec_t;
    }
}
#[no_mangle]
pub unsafe extern "C" fn PM_CheckSpecialMovement() {
    let mut spot: vec3_t = [0.; 3];
    let mut cont: libc::c_int = 0;
    let mut flatforward: vec3_t = [0.; 3];
    let mut trace: trace_t = trace_t {
        allsolid: false_0,
        startsolid: false_0,
        fraction: 0.,
        endpos: [0.; 3],
        plane: cplane_t {
            normal: [0.; 3],
            dist: 0.,
            type_0: 0,
            signbits: 0,
            pad: [0; 2],
        },
        surface: 0 as *mut csurface_t,
        contents: 0,
        ent: 0 as *mut edict_s,
    };
    if (*pm).s.pm_time != 0 {
        return;
    }
    pml.ladder = false_0;
    flatforward[0 as libc::c_int as usize] = pml.forward[0 as libc::c_int as usize];
    flatforward[1 as libc::c_int as usize] = pml.forward[1 as libc::c_int as usize];
    flatforward[2 as libc::c_int as usize] = 0 as libc::c_int as vec_t;
    VectorNormalize(flatforward.as_mut_ptr());
    VectorMA(
        (pml.origin).as_mut_ptr(),
        1 as libc::c_int as libc::c_float,
        flatforward.as_mut_ptr(),
        spot.as_mut_ptr(),
    );
    trace = ((*pm).trace)
        .expect(
            "non-null function pointer",
        )(
        (pml.origin).as_mut_ptr(),
        ((*pm).mins).as_mut_ptr(),
        ((*pm).maxs).as_mut_ptr(),
        spot.as_mut_ptr(),
    );
    if trace.fraction < 1 as libc::c_int as libc::c_float
        && trace.contents & 0x20000000 as libc::c_int != 0
    {
        pml.ladder = true_0;
    }
    if (*pm).waterlevel != 2 as libc::c_int {
        return;
    }
    VectorMA(
        (pml.origin).as_mut_ptr(),
        30 as libc::c_int as libc::c_float,
        flatforward.as_mut_ptr(),
        spot.as_mut_ptr(),
    );
    spot[2 as libc::c_int as usize] += 4 as libc::c_int as libc::c_float;
    cont = ((*pm).pointcontents).expect("non-null function pointer")(spot.as_mut_ptr());
    if cont & 1 as libc::c_int == 0 {
        return;
    }
    spot[2 as libc::c_int as usize] += 16 as libc::c_int as libc::c_float;
    cont = ((*pm).pointcontents).expect("non-null function pointer")(spot.as_mut_ptr());
    if cont != 0 {
        return;
    }
    VectorScale(
        flatforward.as_mut_ptr(),
        50 as libc::c_int as vec_t,
        (pml.velocity).as_mut_ptr(),
    );
    pml.velocity[2 as libc::c_int as usize] = 350 as libc::c_int as vec_t;
    let ref mut fresh16 = (*pm).s.pm_flags;
    *fresh16 = (*fresh16 as libc::c_int | 8 as libc::c_int) as byte;
    (*pm).s.pm_time = 255 as libc::c_int as byte;
}
#[no_mangle]
pub unsafe extern "C" fn PM_FlyMove(mut doclip: qboolean) {
    let mut speed: libc::c_float = 0.;
    let mut drop_0: libc::c_float = 0.;
    let mut friction: libc::c_float = 0.;
    let mut control: libc::c_float = 0.;
    let mut newspeed: libc::c_float = 0.;
    let mut currentspeed: libc::c_float = 0.;
    let mut addspeed: libc::c_float = 0.;
    let mut accelspeed: libc::c_float = 0.;
    let mut i: libc::c_int = 0;
    let mut wishvel: vec3_t = [0.; 3];
    let mut fmove: libc::c_float = 0.;
    let mut smove: libc::c_float = 0.;
    let mut wishdir: vec3_t = [0.; 3];
    let mut wishspeed: libc::c_float = 0.;
    let mut end: vec3_t = [0.; 3];
    let mut trace: trace_t = trace_t {
        allsolid: false_0,
        startsolid: false_0,
        fraction: 0.,
        endpos: [0.; 3],
        plane: cplane_t {
            normal: [0.; 3],
            dist: 0.,
            type_0: 0,
            signbits: 0,
            pad: [0; 2],
        },
        surface: 0 as *mut csurface_t,
        contents: 0,
        ent: 0 as *mut edict_s,
    };
    (*pm).viewheight = 22 as libc::c_int as libc::c_float;
    speed = VectorLength((pml.velocity).as_mut_ptr());
    if speed < 1 as libc::c_int as libc::c_float {
        pml.velocity[0 as libc::c_int as usize] = vec3_origin[0 as libc::c_int as usize];
        pml.velocity[1 as libc::c_int as usize] = vec3_origin[1 as libc::c_int as usize];
        pml.velocity[2 as libc::c_int as usize] = vec3_origin[2 as libc::c_int as usize];
    } else {
        drop_0 = 0 as libc::c_int as libc::c_float;
        friction = (pm_friction as libc::c_double * 1.5f64) as libc::c_float;
        control = if speed < pm_stopspeed { pm_stopspeed } else { speed };
        drop_0 += control * friction * pml.frametime;
        newspeed = speed - drop_0;
        if newspeed < 0 as libc::c_int as libc::c_float {
            newspeed = 0 as libc::c_int as libc::c_float;
        }
        newspeed /= speed;
        VectorScale((pml.velocity).as_mut_ptr(), newspeed, (pml.velocity).as_mut_ptr());
    }
    fmove = (*pm).cmd.forwardmove as libc::c_float;
    smove = (*pm).cmd.sidemove as libc::c_float;
    VectorNormalize((pml.forward).as_mut_ptr());
    VectorNormalize((pml.right).as_mut_ptr());
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        wishvel[i
            as usize] = pml.forward[i as usize] * fmove + pml.right[i as usize] * smove;
        i += 1;
    }
    wishvel[2 as libc::c_int as usize]
        += (*pm).cmd.upmove as libc::c_int as libc::c_float;
    wishdir[0 as libc::c_int as usize] = wishvel[0 as libc::c_int as usize];
    wishdir[1 as libc::c_int as usize] = wishvel[1 as libc::c_int as usize];
    wishdir[2 as libc::c_int as usize] = wishvel[2 as libc::c_int as usize];
    wishspeed = VectorNormalize(wishdir.as_mut_ptr());
    if wishspeed > pm_maxspeed {
        VectorScale(wishvel.as_mut_ptr(), pm_maxspeed / wishspeed, wishvel.as_mut_ptr());
        wishspeed = pm_maxspeed;
    }
    currentspeed = pml.velocity[0 as libc::c_int as usize]
        * wishdir[0 as libc::c_int as usize]
        + pml.velocity[1 as libc::c_int as usize] * wishdir[1 as libc::c_int as usize]
        + pml.velocity[2 as libc::c_int as usize] * wishdir[2 as libc::c_int as usize];
    addspeed = wishspeed - currentspeed;
    if addspeed <= 0 as libc::c_int as libc::c_float {
        return;
    }
    accelspeed = pm_accelerate * pml.frametime * wishspeed;
    if accelspeed > addspeed {
        accelspeed = addspeed;
    }
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        pml.velocity[i as usize] += accelspeed * wishdir[i as usize];
        i += 1;
    }
    if doclip as u64 != 0 {
        i = 0 as libc::c_int;
        while i < 3 as libc::c_int {
            end[i
                as usize] = pml.origin[i as usize]
                + pml.frametime * pml.velocity[i as usize];
            i += 1;
        }
        trace = ((*pm).trace)
            .expect(
                "non-null function pointer",
            )(
            (pml.origin).as_mut_ptr(),
            ((*pm).mins).as_mut_ptr(),
            ((*pm).maxs).as_mut_ptr(),
            end.as_mut_ptr(),
        );
        pml.origin[0 as libc::c_int as usize] = trace.endpos[0 as libc::c_int as usize];
        pml.origin[1 as libc::c_int as usize] = trace.endpos[1 as libc::c_int as usize];
        pml.origin[2 as libc::c_int as usize] = trace.endpos[2 as libc::c_int as usize];
    } else {
        VectorMA(
            (pml.origin).as_mut_ptr(),
            pml.frametime,
            (pml.velocity).as_mut_ptr(),
            (pml.origin).as_mut_ptr(),
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn PM_CheckDuck() {
    let mut trace: trace_t = trace_t {
        allsolid: false_0,
        startsolid: false_0,
        fraction: 0.,
        endpos: [0.; 3],
        plane: cplane_t {
            normal: [0.; 3],
            dist: 0.,
            type_0: 0,
            signbits: 0,
            pad: [0; 2],
        },
        surface: 0 as *mut csurface_t,
        contents: 0,
        ent: 0 as *mut edict_s,
    };
    (*pm).mins[0 as libc::c_int as usize] = -(16 as libc::c_int) as vec_t;
    (*pm).mins[1 as libc::c_int as usize] = -(16 as libc::c_int) as vec_t;
    (*pm).maxs[0 as libc::c_int as usize] = 16 as libc::c_int as vec_t;
    (*pm).maxs[1 as libc::c_int as usize] = 16 as libc::c_int as vec_t;
    if (*pm).s.pm_type as libc::c_uint == PM_GIB as libc::c_int as libc::c_uint {
        (*pm).mins[2 as libc::c_int as usize] = 0 as libc::c_int as vec_t;
        (*pm).maxs[2 as libc::c_int as usize] = 16 as libc::c_int as vec_t;
        (*pm).viewheight = 8 as libc::c_int as libc::c_float;
        return;
    }
    (*pm).mins[2 as libc::c_int as usize] = -(24 as libc::c_int) as vec_t;
    if (*pm).s.pm_type as libc::c_uint == PM_DEAD as libc::c_int as libc::c_uint {
        let ref mut fresh17 = (*pm).s.pm_flags;
        *fresh17 = (*fresh17 as libc::c_int | 1 as libc::c_int) as byte;
    } else if ((*pm).cmd.upmove as libc::c_int) < 0 as libc::c_int
        && (*pm).s.pm_flags as libc::c_int & 4 as libc::c_int != 0
    {
        let ref mut fresh18 = (*pm).s.pm_flags;
        *fresh18 = (*fresh18 as libc::c_int | 1 as libc::c_int) as byte;
    } else if (*pm).s.pm_flags as libc::c_int & 1 as libc::c_int != 0 {
        (*pm).maxs[2 as libc::c_int as usize] = 32 as libc::c_int as vec_t;
        trace = ((*pm).trace)
            .expect(
                "non-null function pointer",
            )(
            (pml.origin).as_mut_ptr(),
            ((*pm).mins).as_mut_ptr(),
            ((*pm).maxs).as_mut_ptr(),
            (pml.origin).as_mut_ptr(),
        );
        if trace.allsolid as u64 == 0 {
            let ref mut fresh19 = (*pm).s.pm_flags;
            *fresh19 = (*fresh19 as libc::c_int & !(1 as libc::c_int)) as byte;
        }
    }
    if (*pm).s.pm_flags as libc::c_int & 1 as libc::c_int != 0 {
        (*pm).maxs[2 as libc::c_int as usize] = 4 as libc::c_int as vec_t;
        (*pm).viewheight = -(2 as libc::c_int) as libc::c_float;
    } else {
        (*pm).maxs[2 as libc::c_int as usize] = 32 as libc::c_int as vec_t;
        (*pm).viewheight = 22 as libc::c_int as libc::c_float;
    };
}
#[no_mangle]
pub unsafe extern "C" fn PM_DeadMove() {
    let mut forward: libc::c_float = 0.;
    if ((*pm).groundentity).is_null() {
        return;
    }
    forward = VectorLength((pml.velocity).as_mut_ptr());
    forward -= 20 as libc::c_int as libc::c_float;
    if forward <= 0 as libc::c_int as libc::c_float {
        pml.velocity[2 as libc::c_int as usize] = 0 as libc::c_int as vec_t;
        pml
            .velocity[1 as libc::c_int
            as usize] = pml.velocity[2 as libc::c_int as usize];
        pml
            .velocity[0 as libc::c_int
            as usize] = pml.velocity[1 as libc::c_int as usize];
    } else {
        VectorNormalize((pml.velocity).as_mut_ptr());
        VectorScale((pml.velocity).as_mut_ptr(), forward, (pml.velocity).as_mut_ptr());
    };
}
#[no_mangle]
pub unsafe extern "C" fn PM_GoodPosition() -> qboolean {
    let mut trace: trace_t = trace_t {
        allsolid: false_0,
        startsolid: false_0,
        fraction: 0.,
        endpos: [0.; 3],
        plane: cplane_t {
            normal: [0.; 3],
            dist: 0.,
            type_0: 0,
            signbits: 0,
            pad: [0; 2],
        },
        surface: 0 as *mut csurface_t,
        contents: 0,
        ent: 0 as *mut edict_s,
    };
    let mut origin: vec3_t = [0.; 3];
    let mut end: vec3_t = [0.; 3];
    let mut i: libc::c_int = 0;
    if (*pm).s.pm_type as libc::c_uint == PM_SPECTATOR as libc::c_int as libc::c_uint {
        return true_0;
    }
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        end[i
            as usize] = ((*pm).s.origin[i as usize] as libc::c_int as libc::c_double
            * 0.125f64) as vec_t;
        origin[i as usize] = end[i as usize];
        i += 1;
    }
    trace = ((*pm).trace)
        .expect(
            "non-null function pointer",
        )(
        origin.as_mut_ptr(),
        ((*pm).mins).as_mut_ptr(),
        ((*pm).maxs).as_mut_ptr(),
        end.as_mut_ptr(),
    );
    return (trace.allsolid as u64 == 0) as libc::c_int as qboolean;
}
#[no_mangle]
pub unsafe extern "C" fn PM_SnapPosition() {
    let mut sign: [libc::c_int; 3] = [0; 3];
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut bits: libc::c_int = 0;
    let mut base: [libc::c_short; 3] = [0; 3];
    static mut jitterbits: [libc::c_int; 8] = [
        0 as libc::c_int,
        4 as libc::c_int,
        1 as libc::c_int,
        2 as libc::c_int,
        3 as libc::c_int,
        5 as libc::c_int,
        6 as libc::c_int,
        7 as libc::c_int,
    ];
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        (*pm)
            .s
            .velocity[i
            as usize] = (pml.velocity[i as usize] * 8 as libc::c_int as libc::c_float)
            as libc::c_int as libc::c_short;
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        if pml.origin[i as usize] >= 0 as libc::c_int as libc::c_float {
            sign[i as usize] = 1 as libc::c_int;
        } else {
            sign[i as usize] = -(1 as libc::c_int);
        }
        (*pm)
            .s
            .origin[i
            as usize] = (pml.origin[i as usize] * 8 as libc::c_int as libc::c_float)
            as libc::c_int as libc::c_short;
        if (*pm).s.origin[i as usize] as libc::c_int as libc::c_double * 0.125f64
            == pml.origin[i as usize] as libc::c_double
        {
            sign[i as usize] = 0 as libc::c_int;
        }
        i += 1;
    }
    base[0 as libc::c_int as usize] = (*pm).s.origin[0 as libc::c_int as usize];
    base[1 as libc::c_int as usize] = (*pm).s.origin[1 as libc::c_int as usize];
    base[2 as libc::c_int as usize] = (*pm).s.origin[2 as libc::c_int as usize];
    j = 0 as libc::c_int;
    while j < 8 as libc::c_int {
        bits = jitterbits[j as usize];
        (*pm).s.origin[0 as libc::c_int as usize] = base[0 as libc::c_int as usize];
        (*pm).s.origin[1 as libc::c_int as usize] = base[1 as libc::c_int as usize];
        (*pm).s.origin[2 as libc::c_int as usize] = base[2 as libc::c_int as usize];
        i = 0 as libc::c_int;
        while i < 3 as libc::c_int {
            if bits & (1 as libc::c_int) << i != 0 {
                let ref mut fresh20 = (*pm).s.origin[i as usize];
                *fresh20 = (*fresh20 as libc::c_int + sign[i as usize]) as libc::c_short;
            }
            i += 1;
        }
        if PM_GoodPosition() as u64 != 0 {
            return;
        }
        j += 1;
    }
    (*pm)
        .s
        .origin[0 as libc::c_int
        as usize] = pml.previous_origin[0 as libc::c_int as usize] as libc::c_short;
    (*pm)
        .s
        .origin[1 as libc::c_int
        as usize] = pml.previous_origin[1 as libc::c_int as usize] as libc::c_short;
    (*pm)
        .s
        .origin[2 as libc::c_int
        as usize] = pml.previous_origin[2 as libc::c_int as usize] as libc::c_short;
}
#[no_mangle]
pub unsafe extern "C" fn PM_InitialSnapPosition() {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut z: libc::c_int = 0;
    let mut base: [libc::c_short; 3] = [0; 3];
    static mut offset: [libc::c_int; 3] = [
        0 as libc::c_int,
        -(1 as libc::c_int),
        1 as libc::c_int,
    ];
    base[0 as libc::c_int as usize] = (*pm).s.origin[0 as libc::c_int as usize];
    base[1 as libc::c_int as usize] = (*pm).s.origin[1 as libc::c_int as usize];
    base[2 as libc::c_int as usize] = (*pm).s.origin[2 as libc::c_int as usize];
    z = 0 as libc::c_int;
    while z < 3 as libc::c_int {
        (*pm)
            .s
            .origin[2 as libc::c_int
            as usize] = (base[2 as libc::c_int as usize] as libc::c_int
            + offset[z as usize]) as libc::c_short;
        y = 0 as libc::c_int;
        while y < 3 as libc::c_int {
            (*pm)
                .s
                .origin[1 as libc::c_int
                as usize] = (base[1 as libc::c_int as usize] as libc::c_int
                + offset[y as usize]) as libc::c_short;
            x = 0 as libc::c_int;
            while x < 3 as libc::c_int {
                (*pm)
                    .s
                    .origin[0 as libc::c_int
                    as usize] = (base[0 as libc::c_int as usize] as libc::c_int
                    + offset[x as usize]) as libc::c_short;
                if PM_GoodPosition() as u64 != 0 {
                    pml
                        .origin[0 as libc::c_int
                        as usize] = ((*pm).s.origin[0 as libc::c_int as usize]
                        as libc::c_int as libc::c_double * 0.125f64) as vec_t;
                    pml
                        .origin[1 as libc::c_int
                        as usize] = ((*pm).s.origin[1 as libc::c_int as usize]
                        as libc::c_int as libc::c_double * 0.125f64) as vec_t;
                    pml
                        .origin[2 as libc::c_int
                        as usize] = ((*pm).s.origin[2 as libc::c_int as usize]
                        as libc::c_int as libc::c_double * 0.125f64) as vec_t;
                    pml
                        .previous_origin[0 as libc::c_int
                        as usize] = (*pm).s.origin[0 as libc::c_int as usize] as vec_t;
                    pml
                        .previous_origin[1 as libc::c_int
                        as usize] = (*pm).s.origin[1 as libc::c_int as usize] as vec_t;
                    pml
                        .previous_origin[2 as libc::c_int
                        as usize] = (*pm).s.origin[2 as libc::c_int as usize] as vec_t;
                    return;
                }
                x += 1;
            }
            y += 1;
        }
        z += 1;
    }
    Com_DPrintf(
        b"Bad InitialSnapPosition\n\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
}
#[no_mangle]
pub unsafe extern "C" fn PM_ClampAngles() {
    let mut temp: libc::c_short = 0;
    let mut i: libc::c_int = 0;
    if (*pm).s.pm_flags as libc::c_int & 32 as libc::c_int != 0 {
        (*pm)
            .viewangles[1 as libc::c_int
            as usize] = (((*pm).cmd.angles[1 as libc::c_int as usize] as libc::c_int
            + (*pm).s.delta_angles[1 as libc::c_int as usize] as libc::c_int)
            as libc::c_double * (360.0f64 / 65536 as libc::c_int as libc::c_double))
            as vec_t;
        (*pm).viewangles[0 as libc::c_int as usize] = 0 as libc::c_int as vec_t;
        (*pm).viewangles[2 as libc::c_int as usize] = 0 as libc::c_int as vec_t;
    } else {
        i = 0 as libc::c_int;
        while i < 3 as libc::c_int {
            temp = ((*pm).cmd.angles[i as usize] as libc::c_int
                + (*pm).s.delta_angles[i as usize] as libc::c_int) as libc::c_short;
            (*pm)
                .viewangles[i
                as usize] = (temp as libc::c_int as libc::c_double
                * (360.0f64 / 65536 as libc::c_int as libc::c_double)) as vec_t;
            i += 1;
        }
        if (*pm).viewangles[0 as libc::c_int as usize]
            > 89 as libc::c_int as libc::c_float
            && (*pm).viewangles[0 as libc::c_int as usize]
                < 180 as libc::c_int as libc::c_float
        {
            (*pm).viewangles[0 as libc::c_int as usize] = 89 as libc::c_int as vec_t;
        } else if (*pm).viewangles[0 as libc::c_int as usize]
            < 271 as libc::c_int as libc::c_float
            && (*pm).viewangles[0 as libc::c_int as usize]
                >= 180 as libc::c_int as libc::c_float
        {
            (*pm).viewangles[0 as libc::c_int as usize] = 271 as libc::c_int as vec_t;
        }
    }
    AngleVectors(
        ((*pm).viewangles).as_mut_ptr(),
        (pml.forward).as_mut_ptr(),
        (pml.right).as_mut_ptr(),
        (pml.up).as_mut_ptr(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn Pmove(mut pmove: *mut pmove_t) {
    pm = pmove;
    (*pm).numtouch = 0 as libc::c_int;
    let ref mut fresh21 = (*pm).viewangles[2 as libc::c_int as usize];
    *fresh21 = 0 as libc::c_int as vec_t;
    let ref mut fresh22 = (*pm).viewangles[1 as libc::c_int as usize];
    *fresh22 = *fresh21;
    (*pm).viewangles[0 as libc::c_int as usize] = *fresh22;
    (*pm).viewheight = 0 as libc::c_int as libc::c_float;
    let ref mut fresh23 = (*pm).groundentity;
    *fresh23 = 0 as *mut edict_s;
    (*pm).watertype = 0 as libc::c_int;
    (*pm).waterlevel = 0 as libc::c_int;
    memset(
        &mut pml as *mut pml_t as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<pml_t>() as libc::c_ulong,
    );
    pml
        .origin[0 as libc::c_int
        as usize] = ((*pm).s.origin[0 as libc::c_int as usize] as libc::c_int
        as libc::c_double * 0.125f64) as vec_t;
    pml
        .origin[1 as libc::c_int
        as usize] = ((*pm).s.origin[1 as libc::c_int as usize] as libc::c_int
        as libc::c_double * 0.125f64) as vec_t;
    pml
        .origin[2 as libc::c_int
        as usize] = ((*pm).s.origin[2 as libc::c_int as usize] as libc::c_int
        as libc::c_double * 0.125f64) as vec_t;
    pml
        .velocity[0 as libc::c_int
        as usize] = ((*pm).s.velocity[0 as libc::c_int as usize] as libc::c_int
        as libc::c_double * 0.125f64) as vec_t;
    pml
        .velocity[1 as libc::c_int
        as usize] = ((*pm).s.velocity[1 as libc::c_int as usize] as libc::c_int
        as libc::c_double * 0.125f64) as vec_t;
    pml
        .velocity[2 as libc::c_int
        as usize] = ((*pm).s.velocity[2 as libc::c_int as usize] as libc::c_int
        as libc::c_double * 0.125f64) as vec_t;
    pml
        .previous_origin[0 as libc::c_int
        as usize] = (*pm).s.origin[0 as libc::c_int as usize] as vec_t;
    pml
        .previous_origin[1 as libc::c_int
        as usize] = (*pm).s.origin[1 as libc::c_int as usize] as vec_t;
    pml
        .previous_origin[2 as libc::c_int
        as usize] = (*pm).s.origin[2 as libc::c_int as usize] as vec_t;
    pml
        .frametime = ((*pm).cmd.msec as libc::c_int as libc::c_double * 0.001f64)
        as libc::c_float;
    PM_ClampAngles();
    if (*pm).s.pm_type as libc::c_uint == PM_SPECTATOR as libc::c_int as libc::c_uint {
        PM_FlyMove(false_0);
        PM_SnapPosition();
        return;
    }
    if (*pm).s.pm_type as libc::c_uint >= PM_DEAD as libc::c_int as libc::c_uint {
        (*pm).cmd.forwardmove = 0 as libc::c_int as libc::c_short;
        (*pm).cmd.sidemove = 0 as libc::c_int as libc::c_short;
        (*pm).cmd.upmove = 0 as libc::c_int as libc::c_short;
    }
    if (*pm).s.pm_type as libc::c_uint == PM_FREEZE as libc::c_int as libc::c_uint {
        return;
    }
    PM_CheckDuck();
    if (*pm).snapinitial as u64 != 0 {
        PM_InitialSnapPosition();
    }
    PM_CatagorizePosition();
    if (*pm).s.pm_type as libc::c_uint == PM_DEAD as libc::c_int as libc::c_uint {
        PM_DeadMove();
    }
    PM_CheckSpecialMovement();
    if (*pm).s.pm_time != 0 {
        let mut msec: libc::c_int = 0;
        msec = (*pm).cmd.msec as libc::c_int >> 3 as libc::c_int;
        if msec == 0 {
            msec = 1 as libc::c_int;
        }
        if msec >= (*pm).s.pm_time as libc::c_int {
            let ref mut fresh24 = (*pm).s.pm_flags;
            *fresh24 = (*fresh24 as libc::c_int
                & !(8 as libc::c_int | 16 as libc::c_int | 32 as libc::c_int)) as byte;
            (*pm).s.pm_time = 0 as libc::c_int as byte;
        } else {
            let ref mut fresh25 = (*pm).s.pm_time;
            *fresh25 = (*fresh25 as libc::c_int - msec) as byte;
        }
    }
    if !((*pm).s.pm_flags as libc::c_int & 32 as libc::c_int != 0) {
        if (*pm).s.pm_flags as libc::c_int & 8 as libc::c_int != 0 {
            pml.velocity[2 as libc::c_int as usize]
                -= (*pm).s.gravity as libc::c_int as libc::c_float * pml.frametime;
            if pml.velocity[2 as libc::c_int as usize]
                < 0 as libc::c_int as libc::c_float
            {
                let ref mut fresh26 = (*pm).s.pm_flags;
                *fresh26 = (*fresh26 as libc::c_int
                    & !(8 as libc::c_int | 16 as libc::c_int | 32 as libc::c_int))
                    as byte;
                (*pm).s.pm_time = 0 as libc::c_int as byte;
            }
            PM_StepSlideMove();
        } else {
            PM_CheckJump();
            PM_Friction();
            if (*pm).waterlevel >= 2 as libc::c_int {
                PM_WaterMove();
            } else {
                let mut angles: vec3_t = [0.; 3];
                angles[0 as libc::c_int
                    as usize] = (*pm).viewangles[0 as libc::c_int as usize];
                angles[1 as libc::c_int
                    as usize] = (*pm).viewangles[1 as libc::c_int as usize];
                angles[2 as libc::c_int
                    as usize] = (*pm).viewangles[2 as libc::c_int as usize];
                if angles[0 as libc::c_int as usize]
                    > 180 as libc::c_int as libc::c_float
                {
                    angles[0 as libc::c_int
                        as usize] = angles[0 as libc::c_int as usize]
                        - 360 as libc::c_int as libc::c_float;
                }
                angles[0 as libc::c_int as usize] /= 3 as libc::c_int as libc::c_float;
                AngleVectors(
                    angles.as_mut_ptr(),
                    (pml.forward).as_mut_ptr(),
                    (pml.right).as_mut_ptr(),
                    (pml.up).as_mut_ptr(),
                );
                PM_AirMove();
            }
        }
    }
    PM_CatagorizePosition();
    PM_SnapPosition();
}
