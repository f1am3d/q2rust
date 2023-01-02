#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(extern_types, register_tool)]

extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type image_s;
    pub type model_s;
    pub type sfx_s;
    fn atan2(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn cos(_: libc::c_double) -> libc::c_double;
    fn sin(_: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn fmod(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn rand() -> libc::c_int;
    static mut vec3_origin: vec3_t;
    fn VectorMA(
        veca: *mut vec_t,
        scale: libc::c_float,
        vecb: *mut vec_t,
        vecc: *mut vec_t,
    );
    fn VectorNormalize(v: *mut vec_t) -> vec_t;
    fn VectorScale(in_0: *mut vec_t, scale: vec_t, out: *mut vec_t);
    fn AngleVectors(
        angles: *mut vec_t,
        forward: *mut vec_t,
        right: *mut vec_t,
        up: *mut vec_t,
    );
    static mut vidref_val: libc::c_int;
    static mut cl: client_state_t;
    fn CL_AllocDlight(key: libc::c_int) -> *mut cdlight_t;
    fn crand() -> libc::c_float;
    fn frand() -> libc::c_float;
    static mut active_particles: *mut cparticle_t;
    static mut free_particles: *mut cparticle_t;
    fn MakeNormalVectors(forward: *mut vec_t, right: *mut vec_t, up: *mut vec_t);
}

pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type size_t = libc::c_ulong;

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
pub type byte = libc::c_uchar;
pub type qboolean = libc::c_uint;

pub const true_0: qboolean = 1;
pub const false_0: qboolean = 0;

pub type vec_t = libc::c_float;
pub type vec3_t = [vec_t; 3];

#[derive(Copy, Clone)]
#[repr(C)]
pub struct cmodel_s {
    pub mins: vec3_t,
    pub maxs: vec3_t,
    pub origin: vec3_t,
    pub headnode: libc::c_int,
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
pub struct player_state_t {
    pub pmove: pmove_state_t,
    pub viewangles: vec3_t,
    pub viewoffset: vec3_t,
    pub kick_angles: vec3_t,
    pub gunangles: vec3_t,
    pub gunoffset: vec3_t,
    pub gunindex: libc::c_int,
    pub gunframe: libc::c_int,
    pub blend: [libc::c_float; 4],
    pub fov: libc::c_float,
    pub rdflags: libc::c_int,
    pub stats: [libc::c_short; 32],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct entity_s {
    pub model: *mut model_s,
    pub angles: [libc::c_float; 3],
    pub origin: [libc::c_float; 3],
    pub frame: libc::c_int,
    pub oldorigin: [libc::c_float; 3],
    pub oldframe: libc::c_int,
    pub backlerp: libc::c_float,
    pub skinnum: libc::c_int,
    pub lightstyle: libc::c_int,
    pub alpha: libc::c_float,
    pub skin: *mut image_s,
    pub flags: libc::c_int,
}

pub type entity_t = entity_s;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct dlight_t {
    pub origin: vec3_t,
    pub color: vec3_t,
    pub intensity: libc::c_float,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct particle_t {
    pub origin: vec3_t,
    pub color: libc::c_int,
    pub alpha: libc::c_float,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct lightstyle_t {
    pub rgb: [libc::c_float; 3],
    pub white: libc::c_float,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct refdef_t {
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub fov_x: libc::c_float,
    pub fov_y: libc::c_float,
    pub vieworg: [libc::c_float; 3],
    pub viewangles: [libc::c_float; 3],
    pub blend: [libc::c_float; 4],
    pub time: libc::c_float,
    pub rdflags: libc::c_int,
    pub areabits: *mut byte,
    pub lightstyles: *mut lightstyle_t,
    pub num_entities: libc::c_int,
    pub entities: *mut entity_t,
    pub num_dlights: libc::c_int,
    pub dlights: *mut dlight_t,
    pub num_particles: libc::c_int,
    pub particles: *mut particle_t,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct frame_t {
    pub valid: qboolean,
    pub serverframe: libc::c_int,
    pub servertime: libc::c_int,
    pub deltaframe: libc::c_int,
    pub areabits: [byte; 32],
    pub playerstate: player_state_t,
    pub num_entities: libc::c_int,
    pub parse_entities: libc::c_int,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct centity_t {
    pub baseline: entity_state_t,
    pub current: entity_state_t,
    pub prev: entity_state_t,
    pub serverframe: libc::c_int,
    pub trailcount: libc::c_int,
    pub lerp_origin: vec3_t,
    pub fly_stoptime: libc::c_int,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct clientinfo_t {
    pub name: [libc::c_char; 64],
    pub cinfo: [libc::c_char; 64],
    pub skin: *mut image_s,
    pub icon: *mut image_s,
    pub iconname: [libc::c_char; 64],
    pub model: *mut model_s,
    pub weaponmodel: [*mut model_s; 20],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct client_state_t {
    pub timeoutcount: libc::c_int,
    pub timedemo_frames: libc::c_int,
    pub timedemo_start: libc::c_int,
    pub refresh_prepped: qboolean,
    pub sound_prepped: qboolean,
    pub force_refdef: qboolean,
    pub parse_entities: libc::c_int,
    pub cmd: usercmd_t,
    pub cmds: [usercmd_t; 64],
    pub cmd_time: [libc::c_int; 64],
    pub predicted_origins: [[libc::c_short; 3]; 64],
    pub predicted_step: libc::c_float,
    pub predicted_step_time: libc::c_uint,
    pub predicted_origin: vec3_t,
    pub predicted_angles: vec3_t,
    pub prediction_error: vec3_t,
    pub frame: frame_t,
    pub surpressCount: libc::c_int,
    pub frames: [frame_t; 16],
    pub viewangles: vec3_t,
    pub time: libc::c_int,
    pub lerpfrac: libc::c_float,
    pub refdef: refdef_t,
    pub v_forward: vec3_t,
    pub v_right: vec3_t,
    pub v_up: vec3_t,
    pub layout: [libc::c_char; 1024],
    pub inventory: [libc::c_int; 256],
    pub cinematic_file: *mut FILE,
    pub cinematictime: libc::c_int,
    pub cinematicframe: libc::c_int,
    pub cinematicpalette: [libc::c_char; 768],
    pub cinematicpalette_active: qboolean,
    pub attractloop: qboolean,
    pub servercount: libc::c_int,
    pub gamedir: [libc::c_char; 64],
    pub playernum: libc::c_int,
    pub configstrings: [[libc::c_char; 64]; 2080],
    pub model_draw: [*mut model_s; 256],
    pub model_clip: [*mut cmodel_s; 256],
    pub sound_precache: [*mut sfx_s; 256],
    pub image_precache: [*mut image_s; 256],
    pub clientinfo: [clientinfo_t; 256],
    pub baseclientinfo: clientinfo_t,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct cdlight_t {
    pub key: libc::c_int,
    pub color: vec3_t,
    pub origin: vec3_t,
    pub radius: libc::c_float,
    pub die: libc::c_float,
    pub decay: libc::c_float,
    pub minlight: libc::c_float,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct cl_sustain {
    pub id: libc::c_int,
    pub type_0: libc::c_int,
    pub endtime: libc::c_int,
    pub nextthink: libc::c_int,
    pub thinkinterval: libc::c_int,
    pub org: vec3_t,
    pub dir: vec3_t,
    pub color: libc::c_int,
    pub count: libc::c_int,
    pub magnitude: libc::c_int,
    pub think: Option::<unsafe extern "C" fn(*mut cl_sustain) -> ()>,
}

pub type cl_sustain_t = cl_sustain;
pub type cparticle_t = particle_s;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct particle_s {
    pub next: *mut particle_s,
    pub time: libc::c_float,
    pub org: vec3_t,
    pub vel: vec3_t,
    pub accel: vec3_t,
    pub color: libc::c_float,
    pub colorvel: libc::c_float,
    pub alpha: libc::c_float,
    pub alphavel: libc::c_float,
}

#[no_mangle]
pub unsafe extern "C" fn vectoangles2(mut value1: *mut vec_t, mut angles: *mut vec_t) {
    let mut forward: libc::c_float = 0.;
    let mut yaw: libc::c_float = 0.;
    let mut pitch: libc::c_float = 0.;
    if *value1.offset(1 as libc::c_int as isize) == 0 as libc::c_int as libc::c_float
        && *value1.offset(0 as libc::c_int as isize) == 0 as libc::c_int as libc::c_float
    {
        yaw = 0 as libc::c_int as libc::c_float;
        if *value1.offset(2 as libc::c_int as isize) > 0 as libc::c_int as libc::c_float
        {
            pitch = 90 as libc::c_int as libc::c_float;
        } else {
            pitch = 270 as libc::c_int as libc::c_float;
        }
    } else {
        if *value1.offset(0 as libc::c_int as isize) != 0. {
            yaw = (atan2(
                *value1.offset(1 as libc::c_int as isize) as libc::c_double,
                *value1.offset(0 as libc::c_int as isize) as libc::c_double,
            ) * 180 as libc::c_int as libc::c_double / 3.14159265358979323846f64)
                as libc::c_float;
        } else if *value1.offset(1 as libc::c_int as isize)
            > 0 as libc::c_int as libc::c_float
        {
            yaw = 90 as libc::c_int as libc::c_float;
        } else {
            yaw = 270 as libc::c_int as libc::c_float;
        }
        if yaw < 0 as libc::c_int as libc::c_float {
            yaw += 360 as libc::c_int as libc::c_float;
        }
        forward = sqrt(
            (*value1.offset(0 as libc::c_int as isize)
                * *value1.offset(0 as libc::c_int as isize)
                + *value1.offset(1 as libc::c_int as isize)
                * *value1.offset(1 as libc::c_int as isize)) as libc::c_double,
        ) as libc::c_float;
        pitch = (atan2(
            *value1.offset(2 as libc::c_int as isize) as libc::c_double,
            forward as libc::c_double,
        ) * 180 as libc::c_int as libc::c_double / 3.14159265358979323846f64)
            as libc::c_float;
        if pitch < 0 as libc::c_int as libc::c_float {
            pitch += 360 as libc::c_int as libc::c_float;
        }
    }
    *angles.offset(0 as libc::c_int as isize) = -pitch;
    *angles.offset(1 as libc::c_int as isize) = yaw;
    *angles.offset(2 as libc::c_int as isize) = 0 as libc::c_int as vec_t;
}

#[no_mangle]
pub unsafe extern "C" fn CL_Flashlight(mut ent: libc::c_int, mut pos: *mut vec_t) {
    let mut dl: *mut cdlight_t = 0 as *mut cdlight_t;
    dl = CL_AllocDlight(ent);
    (*dl).origin[0 as libc::c_int as usize] = *pos.offset(0 as libc::c_int as isize);
    (*dl).origin[1 as libc::c_int as usize] = *pos.offset(1 as libc::c_int as isize);
    (*dl).origin[2 as libc::c_int as usize] = *pos.offset(2 as libc::c_int as isize);
    (*dl).radius = 400 as libc::c_int as libc::c_float;
    (*dl).minlight = 250 as libc::c_int as libc::c_float;
    (*dl).die = (cl.time + 100 as libc::c_int) as libc::c_float;
    (*dl).color[0 as libc::c_int as usize] = 1 as libc::c_int as vec_t;
    (*dl).color[1 as libc::c_int as usize] = 1 as libc::c_int as vec_t;
    (*dl).color[2 as libc::c_int as usize] = 1 as libc::c_int as vec_t;
}

#[no_mangle]
pub unsafe extern "C" fn CL_ColorFlash(
    mut pos: *mut vec_t,
    mut ent: libc::c_int,
    mut intensity: libc::c_int,
    mut r: libc::c_float,
    mut g: libc::c_float,
    mut b: libc::c_float,
) {
    let mut dl: *mut cdlight_t = 0 as *mut cdlight_t;
    if vidref_val == 2 as libc::c_int
        && (r < 0 as libc::c_int as libc::c_float
        || g < 0 as libc::c_int as libc::c_float
        || b < 0 as libc::c_int as libc::c_float)
    {
        intensity = -intensity;
        r = -r;
        g = -g;
        b = -b;
    }
    dl = CL_AllocDlight(ent);
    (*dl).origin[0 as libc::c_int as usize] = *pos.offset(0 as libc::c_int as isize);
    (*dl).origin[1 as libc::c_int as usize] = *pos.offset(1 as libc::c_int as isize);
    (*dl).origin[2 as libc::c_int as usize] = *pos.offset(2 as libc::c_int as isize);
    (*dl).radius = intensity as libc::c_float;
    (*dl).minlight = 250 as libc::c_int as libc::c_float;
    (*dl).die = (cl.time + 100 as libc::c_int) as libc::c_float;
    (*dl).color[0 as libc::c_int as usize] = r;
    (*dl).color[1 as libc::c_int as usize] = g;
    (*dl).color[2 as libc::c_int as usize] = b;
}

#[no_mangle]
pub unsafe extern "C" fn CL_DebugTrail(mut start: *mut vec_t, mut end: *mut vec_t) {
    let mut move_0: vec3_t = [0.; 3];
    let mut vec: vec3_t = [0.; 3];
    let mut len: libc::c_float = 0.;
    let mut p: *mut cparticle_t = 0 as *mut cparticle_t;
    let mut dec: libc::c_float = 0.;
    let mut right: vec3_t = [0.; 3];
    let mut up: vec3_t = [0.; 3];
    move_0[0 as libc::c_int as usize] = *start.offset(0 as libc::c_int as isize);
    move_0[1 as libc::c_int as usize] = *start.offset(1 as libc::c_int as isize);
    move_0[2 as libc::c_int as usize] = *start.offset(2 as libc::c_int as isize);
    vec[0 as libc::c_int
        as usize] = *end.offset(0 as libc::c_int as isize)
        - *start.offset(0 as libc::c_int as isize);
    vec[1 as libc::c_int
        as usize] = *end.offset(1 as libc::c_int as isize)
        - *start.offset(1 as libc::c_int as isize);
    vec[2 as libc::c_int
        as usize] = *end.offset(2 as libc::c_int as isize)
        - *start.offset(2 as libc::c_int as isize);
    len = VectorNormalize(vec.as_mut_ptr());
    MakeNormalVectors(vec.as_mut_ptr(), right.as_mut_ptr(), up.as_mut_ptr());
    dec = 3 as libc::c_int as libc::c_float;
    VectorScale(vec.as_mut_ptr(), dec, vec.as_mut_ptr());
    move_0[0 as libc::c_int as usize] = *start.offset(0 as libc::c_int as isize);
    move_0[1 as libc::c_int as usize] = *start.offset(1 as libc::c_int as isize);
    move_0[2 as libc::c_int as usize] = *start.offset(2 as libc::c_int as isize);
    while len > 0 as libc::c_int as libc::c_float {
        len -= dec;
        if free_particles.is_null() {
            return;
        }
        p = free_particles;
        free_particles = (*p).next;
        let ref mut fresh0 = (*p).next;
        *fresh0 = active_particles;
        active_particles = p;
        (*p).time = cl.time as libc::c_float;
        let ref mut fresh1 = (*p).accel[2 as libc::c_int as usize];
        *fresh1 = 0 as libc::c_int as vec_t;
        let ref mut fresh2 = (*p).accel[1 as libc::c_int as usize];
        *fresh2 = *fresh1;
        (*p).accel[0 as libc::c_int as usize] = *fresh2;
        let ref mut fresh3 = (*p).vel[2 as libc::c_int as usize];
        *fresh3 = 0 as libc::c_int as vec_t;
        let ref mut fresh4 = (*p).vel[1 as libc::c_int as usize];
        *fresh4 = *fresh3;
        (*p).vel[0 as libc::c_int as usize] = *fresh4;
        (*p).alpha = 1.0f64 as libc::c_float;
        (*p).alphavel = -0.1f64 as libc::c_float;
        (*p)
            .color = (0x74 as libc::c_int + (rand() & 7 as libc::c_int))
            as libc::c_float;
        (*p).org[0 as libc::c_int as usize] = move_0[0 as libc::c_int as usize];
        (*p).org[1 as libc::c_int as usize] = move_0[1 as libc::c_int as usize];
        (*p).org[2 as libc::c_int as usize] = move_0[2 as libc::c_int as usize];
        move_0[0 as libc::c_int
            as usize] = move_0[0 as libc::c_int as usize]
            + vec[0 as libc::c_int as usize];
        move_0[1 as libc::c_int
            as usize] = move_0[1 as libc::c_int as usize]
            + vec[1 as libc::c_int as usize];
        move_0[2 as libc::c_int
            as usize] = move_0[2 as libc::c_int as usize]
            + vec[2 as libc::c_int as usize];
    }
}

#[no_mangle]
pub unsafe extern "C" fn CL_SmokeTrail(
    mut start: *mut vec_t,
    mut end: *mut vec_t,
    mut colorStart: libc::c_int,
    mut colorRun: libc::c_int,
    mut spacing: libc::c_int,
) {
    let mut move_0: vec3_t = [0.; 3];
    let mut vec: vec3_t = [0.; 3];
    let mut len: libc::c_float = 0.;
    let mut j: libc::c_int = 0;
    let mut p: *mut cparticle_t = 0 as *mut cparticle_t;
    move_0[0 as libc::c_int as usize] = *start.offset(0 as libc::c_int as isize);
    move_0[1 as libc::c_int as usize] = *start.offset(1 as libc::c_int as isize);
    move_0[2 as libc::c_int as usize] = *start.offset(2 as libc::c_int as isize);
    vec[0 as libc::c_int
        as usize] = *end.offset(0 as libc::c_int as isize)
        - *start.offset(0 as libc::c_int as isize);
    vec[1 as libc::c_int
        as usize] = *end.offset(1 as libc::c_int as isize)
        - *start.offset(1 as libc::c_int as isize);
    vec[2 as libc::c_int
        as usize] = *end.offset(2 as libc::c_int as isize)
        - *start.offset(2 as libc::c_int as isize);
    len = VectorNormalize(vec.as_mut_ptr());
    VectorScale(vec.as_mut_ptr(), spacing as vec_t, vec.as_mut_ptr());
    while len > 0 as libc::c_int as libc::c_float {
        len -= spacing as libc::c_float;
        if free_particles.is_null() {
            return;
        }
        p = free_particles;
        free_particles = (*p).next;
        let ref mut fresh5 = (*p).next;
        *fresh5 = active_particles;
        active_particles = p;
        let ref mut fresh6 = (*p).accel[2 as libc::c_int as usize];
        *fresh6 = 0 as libc::c_int as vec_t;
        let ref mut fresh7 = (*p).accel[1 as libc::c_int as usize];
        *fresh7 = *fresh6;
        (*p).accel[0 as libc::c_int as usize] = *fresh7;
        (*p).time = cl.time as libc::c_float;
        (*p).alpha = 1.0f64 as libc::c_float;
        (*p)
            .alphavel = (-1.0f64
            / (1 as libc::c_int as libc::c_double + frand() as libc::c_double * 0.5f64))
            as libc::c_float;
        (*p).color = (colorStart + rand() % colorRun) as libc::c_float;
        j = 0 as libc::c_int;
        while j < 3 as libc::c_int {
            (*p)
                .org[j
                as usize] = move_0[j as usize]
                + crand() * 3 as libc::c_int as libc::c_float;
            (*p).accel[j as usize] = 0 as libc::c_int as vec_t;
            j += 1;
        }
        (*p)
            .vel[2 as libc::c_int
            as usize] = 20 as libc::c_int as libc::c_float
            + crand() * 5 as libc::c_int as libc::c_float;
        move_0[0 as libc::c_int
            as usize] = move_0[0 as libc::c_int as usize]
            + vec[0 as libc::c_int as usize];
        move_0[1 as libc::c_int
            as usize] = move_0[1 as libc::c_int as usize]
            + vec[1 as libc::c_int as usize];
        move_0[2 as libc::c_int
            as usize] = move_0[2 as libc::c_int as usize]
            + vec[2 as libc::c_int as usize];
    }
}

#[no_mangle]
pub unsafe extern "C" fn CL_ForceWall(
    mut start: *mut vec_t,
    mut end: *mut vec_t,
    mut color: libc::c_int,
) {
    let mut move_0: vec3_t = [0.; 3];
    let mut vec: vec3_t = [0.; 3];
    let mut len: libc::c_float = 0.;
    let mut j: libc::c_int = 0;
    let mut p: *mut cparticle_t = 0 as *mut cparticle_t;
    move_0[0 as libc::c_int as usize] = *start.offset(0 as libc::c_int as isize);
    move_0[1 as libc::c_int as usize] = *start.offset(1 as libc::c_int as isize);
    move_0[2 as libc::c_int as usize] = *start.offset(2 as libc::c_int as isize);
    vec[0 as libc::c_int
        as usize] = *end.offset(0 as libc::c_int as isize)
        - *start.offset(0 as libc::c_int as isize);
    vec[1 as libc::c_int
        as usize] = *end.offset(1 as libc::c_int as isize)
        - *start.offset(1 as libc::c_int as isize);
    vec[2 as libc::c_int
        as usize] = *end.offset(2 as libc::c_int as isize)
        - *start.offset(2 as libc::c_int as isize);
    len = VectorNormalize(vec.as_mut_ptr());
    VectorScale(vec.as_mut_ptr(), 4 as libc::c_int as vec_t, vec.as_mut_ptr());
    while len > 0 as libc::c_int as libc::c_float {
        len -= 4 as libc::c_int as libc::c_float;
        if free_particles.is_null() {
            return;
        }
        if frand() as libc::c_double > 0.3f64 {
            p = free_particles;
            free_particles = (*p).next;
            let ref mut fresh8 = (*p).next;
            *fresh8 = active_particles;
            active_particles = p;
            let ref mut fresh9 = (*p).accel[2 as libc::c_int as usize];
            *fresh9 = 0 as libc::c_int as vec_t;
            let ref mut fresh10 = (*p).accel[1 as libc::c_int as usize];
            *fresh10 = *fresh9;
            (*p).accel[0 as libc::c_int as usize] = *fresh10;
            (*p).time = cl.time as libc::c_float;
            (*p).alpha = 1.0f64 as libc::c_float;
            (*p)
                .alphavel = (-1.0f64 / (3.0f64 + frand() as libc::c_double * 0.5f64))
                as libc::c_float;
            (*p).color = color as libc::c_float;
            j = 0 as libc::c_int;
            while j < 3 as libc::c_int {
                (*p)
                    .org[j
                    as usize] = move_0[j as usize]
                    + crand() * 3 as libc::c_int as libc::c_float;
                (*p).accel[j as usize] = 0 as libc::c_int as vec_t;
                j += 1;
            }
            (*p).vel[0 as libc::c_int as usize] = 0 as libc::c_int as vec_t;
            (*p).vel[1 as libc::c_int as usize] = 0 as libc::c_int as vec_t;
            (*p)
                .vel[2 as libc::c_int
                as usize] = -(40 as libc::c_int) as libc::c_float
                - crand() * 10 as libc::c_int as libc::c_float;
        }
        move_0[0 as libc::c_int
            as usize] = move_0[0 as libc::c_int as usize]
            + vec[0 as libc::c_int as usize];
        move_0[1 as libc::c_int
            as usize] = move_0[1 as libc::c_int as usize]
            + vec[1 as libc::c_int as usize];
        move_0[2 as libc::c_int
            as usize] = move_0[2 as libc::c_int as usize]
            + vec[2 as libc::c_int as usize];
    }
}

#[no_mangle]
pub unsafe extern "C" fn CL_FlameEffects(
    mut ent: *mut centity_t,
    mut origin: *mut vec_t,
) {
    let mut n: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut p: *mut cparticle_t = 0 as *mut cparticle_t;
    count = rand() & 0xf as libc::c_int;
    n = 0 as libc::c_int;
    while n < count {
        if free_particles.is_null() {
            return;
        }
        p = free_particles;
        free_particles = (*p).next;
        let ref mut fresh11 = (*p).next;
        *fresh11 = active_particles;
        active_particles = p;
        let ref mut fresh12 = (*p).accel[2 as libc::c_int as usize];
        *fresh12 = 0 as libc::c_int as vec_t;
        let ref mut fresh13 = (*p).accel[1 as libc::c_int as usize];
        *fresh13 = *fresh12;
        (*p).accel[0 as libc::c_int as usize] = *fresh13;
        (*p).time = cl.time as libc::c_float;
        (*p).alpha = 1.0f64 as libc::c_float;
        (*p)
            .alphavel = (-1.0f64
            / (1 as libc::c_int as libc::c_double + frand() as libc::c_double * 0.2f64))
            as libc::c_float;
        (*p).color = (226 as libc::c_int + rand() % 4 as libc::c_int) as libc::c_float;
        j = 0 as libc::c_int;
        while j < 3 as libc::c_int {
            (*p)
                .org[j
                as usize] = *origin.offset(j as isize)
                + crand() * 5 as libc::c_int as libc::c_float;
            (*p).vel[j as usize] = crand() * 5 as libc::c_int as libc::c_float;
            j += 1;
        }
        (*p)
            .vel[2 as libc::c_int
            as usize] = crand() * -(10 as libc::c_int) as libc::c_float;
        (*p).accel[2 as libc::c_int as usize] = -(40 as libc::c_int) as vec_t;
        n += 1;
    }
    count = rand() & 0x7 as libc::c_int;
    n = 0 as libc::c_int;
    while n < count {
        if free_particles.is_null() {
            return;
        }
        p = free_particles;
        free_particles = (*p).next;
        let ref mut fresh14 = (*p).next;
        *fresh14 = active_particles;
        active_particles = p;
        let ref mut fresh15 = (*p).accel[2 as libc::c_int as usize];
        *fresh15 = 0 as libc::c_int as vec_t;
        let ref mut fresh16 = (*p).accel[1 as libc::c_int as usize];
        *fresh16 = *fresh15;
        (*p).accel[0 as libc::c_int as usize] = *fresh16;
        (*p).time = cl.time as libc::c_float;
        (*p).alpha = 1.0f64 as libc::c_float;
        (*p)
            .alphavel = (-1.0f64
            / (1 as libc::c_int as libc::c_double + frand() as libc::c_double * 0.5f64))
            as libc::c_float;
        (*p).color = (0 as libc::c_int + rand() % 4 as libc::c_int) as libc::c_float;
        j = 0 as libc::c_int;
        while j < 3 as libc::c_int {
            (*p)
                .org[j
                as usize] = *origin.offset(j as isize)
                + crand() * 3 as libc::c_int as libc::c_float;
            j += 1;
        }
        (*p)
            .vel[2 as libc::c_int
            as usize] = 20 as libc::c_int as libc::c_float
            + crand() * 5 as libc::c_int as libc::c_float;
        n += 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn CL_GenericParticleEffect(
    mut org: *mut vec_t,
    mut dir: *mut vec_t,
    mut color: libc::c_int,
    mut count: libc::c_int,
    mut numcolors: libc::c_int,
    mut dirspread: libc::c_int,
    mut alphavel: libc::c_float,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut p: *mut cparticle_t = 0 as *mut cparticle_t;
    let mut d: libc::c_float = 0.;
    i = 0 as libc::c_int;
    while i < count {
        if free_particles.is_null() {
            return;
        }
        p = free_particles;
        free_particles = (*p).next;
        let ref mut fresh17 = (*p).next;
        *fresh17 = active_particles;
        active_particles = p;
        (*p).time = cl.time as libc::c_float;
        if numcolors > 1 as libc::c_int {
            (*p).color = (color + (rand() & numcolors)) as libc::c_float;
        } else {
            (*p).color = color as libc::c_float;
        }
        d = (rand() & dirspread) as libc::c_float;
        j = 0 as libc::c_int;
        while j < 3 as libc::c_int {
            (*p)
                .org[j
                as usize] = *org.offset(j as isize)
                + ((rand() & 7 as libc::c_int) - 4 as libc::c_int) as libc::c_float
                + d * *dir.offset(j as isize);
            (*p).vel[j as usize] = crand() * 20 as libc::c_int as libc::c_float;
            j += 1;
        }
        let ref mut fresh18 = (*p).accel[1 as libc::c_int as usize];
        *fresh18 = 0 as libc::c_int as vec_t;
        (*p).accel[0 as libc::c_int as usize] = *fresh18;
        (*p).accel[2 as libc::c_int as usize] = -(40 as libc::c_int) as vec_t;
        (*p).alpha = 1.0f64 as libc::c_float;
        (*p)
            .alphavel = (-1.0f64 / (0.5f64 + (frand() * alphavel) as libc::c_double))
            as libc::c_float;
        i += 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn CL_BubbleTrail2(
    mut start: *mut vec_t,
    mut end: *mut vec_t,
    mut dist: libc::c_int,
) {
    let mut move_0: vec3_t = [0.; 3];
    let mut vec: vec3_t = [0.; 3];
    let mut len: libc::c_float = 0.;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut p: *mut cparticle_t = 0 as *mut cparticle_t;
    let mut dec: libc::c_float = 0.;
    move_0[0 as libc::c_int as usize] = *start.offset(0 as libc::c_int as isize);
    move_0[1 as libc::c_int as usize] = *start.offset(1 as libc::c_int as isize);
    move_0[2 as libc::c_int as usize] = *start.offset(2 as libc::c_int as isize);
    vec[0 as libc::c_int
        as usize] = *end.offset(0 as libc::c_int as isize)
        - *start.offset(0 as libc::c_int as isize);
    vec[1 as libc::c_int
        as usize] = *end.offset(1 as libc::c_int as isize)
        - *start.offset(1 as libc::c_int as isize);
    vec[2 as libc::c_int
        as usize] = *end.offset(2 as libc::c_int as isize)
        - *start.offset(2 as libc::c_int as isize);
    len = VectorNormalize(vec.as_mut_ptr());
    dec = dist as libc::c_float;
    VectorScale(vec.as_mut_ptr(), dec, vec.as_mut_ptr());
    i = 0 as libc::c_int;
    while (i as libc::c_float) < len {
        if free_particles.is_null() {
            return;
        }
        p = free_particles;
        free_particles = (*p).next;
        let ref mut fresh19 = (*p).next;
        *fresh19 = active_particles;
        active_particles = p;
        let ref mut fresh20 = (*p).accel[2 as libc::c_int as usize];
        *fresh20 = 0 as libc::c_int as vec_t;
        let ref mut fresh21 = (*p).accel[1 as libc::c_int as usize];
        *fresh21 = *fresh20;
        (*p).accel[0 as libc::c_int as usize] = *fresh21;
        (*p).time = cl.time as libc::c_float;
        (*p).alpha = 1.0f64 as libc::c_float;
        (*p)
            .alphavel = (-1.0f64
            / (1 as libc::c_int as libc::c_double + frand() as libc::c_double * 0.1f64))
            as libc::c_float;
        (*p).color = (4 as libc::c_int + (rand() & 7 as libc::c_int)) as libc::c_float;
        j = 0 as libc::c_int;
        while j < 3 as libc::c_int {
            (*p)
                .org[j
                as usize] = move_0[j as usize]
                + crand() * 2 as libc::c_int as libc::c_float;
            (*p).vel[j as usize] = crand() * 10 as libc::c_int as libc::c_float;
            j += 1;
        }
        let ref mut fresh22 = (*p).org[2 as libc::c_int as usize];
        *fresh22 -= 4 as libc::c_int as libc::c_float;
        let ref mut fresh23 = (*p).vel[2 as libc::c_int as usize];
        *fresh23 += 20 as libc::c_int as libc::c_float;
        move_0[0 as libc::c_int
            as usize] = move_0[0 as libc::c_int as usize]
            + vec[0 as libc::c_int as usize];
        move_0[1 as libc::c_int
            as usize] = move_0[1 as libc::c_int as usize]
            + vec[1 as libc::c_int as usize];
        move_0[2 as libc::c_int
            as usize] = move_0[2 as libc::c_int as usize]
            + vec[2 as libc::c_int as usize];
        i = (i as libc::c_float + dec) as libc::c_int;
    }
}

#[no_mangle]
pub unsafe extern "C" fn CL_Heatbeam(mut start: *mut vec_t, mut forward: *mut vec_t) {
    let mut move_0: vec3_t = [0.; 3];
    let mut vec: vec3_t = [0.; 3];
    let mut len: libc::c_float = 0.;
    let mut j: libc::c_int = 0;
    let mut p: *mut cparticle_t = 0 as *mut cparticle_t;
    let mut right: vec3_t = [0.; 3];
    let mut up: vec3_t = [0.; 3];
    let mut i: libc::c_int = 0;
    let mut c: libc::c_float = 0.;
    let mut s: libc::c_float = 0.;
    let mut dir: vec3_t = [0.; 3];
    let mut ltime: libc::c_float = 0.;
    let mut step: libc::c_float = 32.0f64 as libc::c_float;
    let mut rstep: libc::c_float = 0.;
    let mut start_pt: libc::c_float = 0.;
    let mut rot: libc::c_float = 0.;
    let mut variance: libc::c_float = 0.;
    let mut end: vec3_t = [0.; 3];
    VectorMA(start, 4096 as libc::c_int as libc::c_float, forward, end.as_mut_ptr());
    move_0[0 as libc::c_int as usize] = *start.offset(0 as libc::c_int as isize);
    move_0[1 as libc::c_int as usize] = *start.offset(1 as libc::c_int as isize);
    move_0[2 as libc::c_int as usize] = *start.offset(2 as libc::c_int as isize);
    vec[0 as libc::c_int
        as usize] = end[0 as libc::c_int as usize]
        - *start.offset(0 as libc::c_int as isize);
    vec[1 as libc::c_int
        as usize] = end[1 as libc::c_int as usize]
        - *start.offset(1 as libc::c_int as isize);
    vec[2 as libc::c_int
        as usize] = end[2 as libc::c_int as usize]
        - *start.offset(2 as libc::c_int as isize);
    len = VectorNormalize(vec.as_mut_ptr());
    right[0 as libc::c_int as usize] = cl.v_right[0 as libc::c_int as usize];
    right[1 as libc::c_int as usize] = cl.v_right[1 as libc::c_int as usize];
    right[2 as libc::c_int as usize] = cl.v_right[2 as libc::c_int as usize];
    up[0 as libc::c_int as usize] = cl.v_up[0 as libc::c_int as usize];
    up[1 as libc::c_int as usize] = cl.v_up[1 as libc::c_int as usize];
    up[2 as libc::c_int as usize] = cl.v_up[2 as libc::c_int as usize];
    if vidref_val == 1 as libc::c_int {
        VectorMA(
            move_0.as_mut_ptr(),
            -0.5f64 as libc::c_float,
            right.as_mut_ptr(),
            move_0.as_mut_ptr(),
        );
        VectorMA(
            move_0.as_mut_ptr(),
            -0.5f64 as libc::c_float,
            up.as_mut_ptr(),
            move_0.as_mut_ptr(),
        );
    }
    ltime = (cl.time as libc::c_float as libc::c_double / 1000.0f64) as libc::c_float;
    start_pt = fmod(ltime as libc::c_double * 96.0f64, step as libc::c_double)
        as libc::c_float;
    VectorMA(move_0.as_mut_ptr(), start_pt, vec.as_mut_ptr(), move_0.as_mut_ptr());
    VectorScale(vec.as_mut_ptr(), step, vec.as_mut_ptr());
    rstep = (3.14159265358979323846f64 / 10.0f64) as libc::c_float;
    i = start_pt as libc::c_int;
    while (i as libc::c_float) < len {
        if i as libc::c_float > step * 5 as libc::c_int as libc::c_float {
            break;
        }
        rot = 0 as libc::c_int as libc::c_float;
        while (rot as libc::c_double)
            < 3.14159265358979323846f64 * 2 as libc::c_int as libc::c_double
        {
            if free_particles.is_null() {
                return;
            }
            p = free_particles;
            free_particles = (*p).next;
            let ref mut fresh24 = (*p).next;
            *fresh24 = active_particles;
            active_particles = p;
            (*p).time = cl.time as libc::c_float;
            let ref mut fresh25 = (*p).accel[2 as libc::c_int as usize];
            *fresh25 = 0 as libc::c_int as vec_t;
            let ref mut fresh26 = (*p).accel[1 as libc::c_int as usize];
            *fresh26 = *fresh25;
            (*p).accel[0 as libc::c_int as usize] = *fresh26;
            variance = 0.5f64 as libc::c_float;
            c = (cos(rot as libc::c_double) * variance as libc::c_double)
                as libc::c_float;
            s = (sin(rot as libc::c_double) * variance as libc::c_double)
                as libc::c_float;
            if i < 10 as libc::c_int {
                VectorScale(
                    right.as_mut_ptr(),
                    (c as libc::c_double * (i as libc::c_double / 10.0f64)) as vec_t,
                    dir.as_mut_ptr(),
                );
                VectorMA(
                    dir.as_mut_ptr(),
                    (s as libc::c_double * (i as libc::c_double / 10.0f64))
                        as libc::c_float,
                    up.as_mut_ptr(),
                    dir.as_mut_ptr(),
                );
            } else {
                VectorScale(right.as_mut_ptr(), c, dir.as_mut_ptr());
                VectorMA(dir.as_mut_ptr(), s, up.as_mut_ptr(), dir.as_mut_ptr());
            }
            (*p).alpha = 0.5f64 as libc::c_float;
            (*p).alphavel = -1000.0f64 as libc::c_float;
            (*p)
                .color = (223 as libc::c_int - (rand() & 7 as libc::c_int))
                as libc::c_float;
            j = 0 as libc::c_int;
            while j < 3 as libc::c_int {
                (*p)
                    .org[j
                    as usize] = move_0[j as usize]
                    + dir[j as usize] * 3 as libc::c_int as libc::c_float;
                (*p).vel[j as usize] = 0 as libc::c_int as vec_t;
                j += 1;
            }
            rot += rstep;
        }
        move_0[0 as libc::c_int
            as usize] = move_0[0 as libc::c_int as usize]
            + vec[0 as libc::c_int as usize];
        move_0[1 as libc::c_int
            as usize] = move_0[1 as libc::c_int as usize]
            + vec[1 as libc::c_int as usize];
        move_0[2 as libc::c_int
            as usize] = move_0[2 as libc::c_int as usize]
            + vec[2 as libc::c_int as usize];
        i = (i as libc::c_float + step) as libc::c_int;
    }
}

#[no_mangle]
pub unsafe extern "C" fn CL_ParticleSteamEffect(
    mut org: *mut vec_t,
    mut dir: *mut vec_t,
    mut color: libc::c_int,
    mut count: libc::c_int,
    mut magnitude: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut p: *mut cparticle_t = 0 as *mut cparticle_t;
    let mut d: libc::c_float = 0.;
    let mut r: vec3_t = [0.; 3];
    let mut u: vec3_t = [0.; 3];
    MakeNormalVectors(dir, r.as_mut_ptr(), u.as_mut_ptr());
    i = 0 as libc::c_int;
    while i < count {
        if free_particles.is_null() {
            return;
        }
        p = free_particles;
        free_particles = (*p).next;
        let ref mut fresh27 = (*p).next;
        *fresh27 = active_particles;
        active_particles = p;
        (*p).time = cl.time as libc::c_float;
        (*p).color = (color + (rand() & 7 as libc::c_int)) as libc::c_float;
        j = 0 as libc::c_int;
        while j < 3 as libc::c_int {
            (*p)
                .org[j
                as usize] = (*org.offset(j as isize) as libc::c_double
                + magnitude as libc::c_double * 0.1f64 * crand() as libc::c_double)
                as vec_t;
            j += 1;
        }
        VectorScale(dir, magnitude as vec_t, ((*p).vel).as_mut_ptr());
        d = crand() * magnitude as libc::c_float / 3 as libc::c_int as libc::c_float;
        VectorMA(((*p).vel).as_mut_ptr(), d, r.as_mut_ptr(), ((*p).vel).as_mut_ptr());
        d = crand() * magnitude as libc::c_float / 3 as libc::c_int as libc::c_float;
        VectorMA(((*p).vel).as_mut_ptr(), d, u.as_mut_ptr(), ((*p).vel).as_mut_ptr());
        let ref mut fresh28 = (*p).accel[1 as libc::c_int as usize];
        *fresh28 = 0 as libc::c_int as vec_t;
        (*p).accel[0 as libc::c_int as usize] = *fresh28;
        (*p)
            .accel[2 as libc::c_int
            as usize] = (-(40 as libc::c_int) / 2 as libc::c_int) as vec_t;
        (*p).alpha = 1.0f64 as libc::c_float;
        (*p)
            .alphavel = (-1.0f64 / (0.5f64 + frand() as libc::c_double * 0.3f64))
            as libc::c_float;
        i += 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn CL_ParticleSteamEffect2(mut self_0: *mut cl_sustain_t) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut p: *mut cparticle_t = 0 as *mut cparticle_t;
    let mut d: libc::c_float = 0.;
    let mut r: vec3_t = [0.; 3];
    let mut u: vec3_t = [0.; 3];
    let mut dir: vec3_t = [0.; 3];
    dir[0 as libc::c_int as usize] = (*self_0).dir[0 as libc::c_int as usize];
    dir[1 as libc::c_int as usize] = (*self_0).dir[1 as libc::c_int as usize];
    dir[2 as libc::c_int as usize] = (*self_0).dir[2 as libc::c_int as usize];
    MakeNormalVectors(dir.as_mut_ptr(), r.as_mut_ptr(), u.as_mut_ptr());
    i = 0 as libc::c_int;
    while i < (*self_0).count {
        if free_particles.is_null() {
            return;
        }
        p = free_particles;
        free_particles = (*p).next;
        let ref mut fresh29 = (*p).next;
        *fresh29 = active_particles;
        active_particles = p;
        (*p).time = cl.time as libc::c_float;
        (*p).color = ((*self_0).color + (rand() & 7 as libc::c_int)) as libc::c_float;
        j = 0 as libc::c_int;
        while j < 3 as libc::c_int {
            (*p)
                .org[j
                as usize] = ((*self_0).org[j as usize] as libc::c_double
                + (*self_0).magnitude as libc::c_double * 0.1f64
                * crand() as libc::c_double) as vec_t;
            j += 1;
        }
        VectorScale(
            dir.as_mut_ptr(),
            (*self_0).magnitude as vec_t,
            ((*p).vel).as_mut_ptr(),
        );
        d = crand() * (*self_0).magnitude as libc::c_float
            / 3 as libc::c_int as libc::c_float;
        VectorMA(((*p).vel).as_mut_ptr(), d, r.as_mut_ptr(), ((*p).vel).as_mut_ptr());
        d = crand() * (*self_0).magnitude as libc::c_float
            / 3 as libc::c_int as libc::c_float;
        VectorMA(((*p).vel).as_mut_ptr(), d, u.as_mut_ptr(), ((*p).vel).as_mut_ptr());
        let ref mut fresh30 = (*p).accel[1 as libc::c_int as usize];
        *fresh30 = 0 as libc::c_int as vec_t;
        (*p).accel[0 as libc::c_int as usize] = *fresh30;
        (*p)
            .accel[2 as libc::c_int
            as usize] = (-(40 as libc::c_int) / 2 as libc::c_int) as vec_t;
        (*p).alpha = 1.0f64 as libc::c_float;
        (*p)
            .alphavel = (-1.0f64 / (0.5f64 + frand() as libc::c_double * 0.3f64))
            as libc::c_float;
        i += 1;
    }
    (*self_0).nextthink += (*self_0).thinkinterval;
}

#[no_mangle]
pub unsafe extern "C" fn CL_TrackerTrail(
    mut start: *mut vec_t,
    mut end: *mut vec_t,
    mut particleColor: libc::c_int,
) {
    let mut move_0: vec3_t = [0.; 3];
    let mut vec: vec3_t = [0.; 3];
    let mut forward: vec3_t = [0.; 3];
    let mut right: vec3_t = [0.; 3];
    let mut up: vec3_t = [0.; 3];
    let mut angle_dir: vec3_t = [0.; 3];
    let mut len: libc::c_float = 0.;
    let mut j: libc::c_int = 0;
    let mut p: *mut cparticle_t = 0 as *mut cparticle_t;
    let mut dec: libc::c_int = 0;
    let mut dist: libc::c_float = 0.;
    move_0[0 as libc::c_int as usize] = *start.offset(0 as libc::c_int as isize);
    move_0[1 as libc::c_int as usize] = *start.offset(1 as libc::c_int as isize);
    move_0[2 as libc::c_int as usize] = *start.offset(2 as libc::c_int as isize);
    vec[0 as libc::c_int
        as usize] = *end.offset(0 as libc::c_int as isize)
        - *start.offset(0 as libc::c_int as isize);
    vec[1 as libc::c_int
        as usize] = *end.offset(1 as libc::c_int as isize)
        - *start.offset(1 as libc::c_int as isize);
    vec[2 as libc::c_int
        as usize] = *end.offset(2 as libc::c_int as isize)
        - *start.offset(2 as libc::c_int as isize);
    len = VectorNormalize(vec.as_mut_ptr());
    forward[0 as libc::c_int as usize] = vec[0 as libc::c_int as usize];
    forward[1 as libc::c_int as usize] = vec[1 as libc::c_int as usize];
    forward[2 as libc::c_int as usize] = vec[2 as libc::c_int as usize];
    vectoangles2(forward.as_mut_ptr(), angle_dir.as_mut_ptr());
    AngleVectors(
        angle_dir.as_mut_ptr(),
        forward.as_mut_ptr(),
        right.as_mut_ptr(),
        up.as_mut_ptr(),
    );
    dec = 3 as libc::c_int;
    VectorScale(vec.as_mut_ptr(), 3 as libc::c_int as vec_t, vec.as_mut_ptr());
    while len > 0 as libc::c_int as libc::c_float {
        len -= dec as libc::c_float;
        if free_particles.is_null() {
            return;
        }
        p = free_particles;
        free_particles = (*p).next;
        let ref mut fresh31 = (*p).next;
        *fresh31 = active_particles;
        active_particles = p;
        let ref mut fresh32 = (*p).accel[2 as libc::c_int as usize];
        *fresh32 = 0 as libc::c_int as vec_t;
        let ref mut fresh33 = (*p).accel[1 as libc::c_int as usize];
        *fresh33 = *fresh32;
        (*p).accel[0 as libc::c_int as usize] = *fresh33;
        (*p).time = cl.time as libc::c_float;
        (*p).alpha = 1.0f64 as libc::c_float;
        (*p).alphavel = -2.0f64 as libc::c_float;
        (*p).color = particleColor as libc::c_float;
        dist = move_0[0 as libc::c_int as usize] * forward[0 as libc::c_int as usize]
            + move_0[1 as libc::c_int as usize] * forward[1 as libc::c_int as usize]
            + move_0[2 as libc::c_int as usize] * forward[2 as libc::c_int as usize];
        VectorMA(
            move_0.as_mut_ptr(),
            (8 as libc::c_int as libc::c_double * cos(dist as libc::c_double))
                as libc::c_float,
            up.as_mut_ptr(),
            ((*p).org).as_mut_ptr(),
        );
        j = 0 as libc::c_int;
        while j < 3 as libc::c_int {
            (*p).vel[j as usize] = 0 as libc::c_int as vec_t;
            (*p).accel[j as usize] = 0 as libc::c_int as vec_t;
            j += 1;
        }
        (*p).vel[2 as libc::c_int as usize] = 5 as libc::c_int as vec_t;
        move_0[0 as libc::c_int
            as usize] = move_0[0 as libc::c_int as usize]
            + vec[0 as libc::c_int as usize];
        move_0[1 as libc::c_int
            as usize] = move_0[1 as libc::c_int as usize]
            + vec[1 as libc::c_int as usize];
        move_0[2 as libc::c_int
            as usize] = move_0[2 as libc::c_int as usize]
            + vec[2 as libc::c_int as usize];
    }
}

#[no_mangle]
pub unsafe extern "C" fn CL_Tracker_Shell(mut origin: *mut vec_t) {
    let mut dir: vec3_t = [0.; 3];
    let mut i: libc::c_int = 0;
    let mut p: *mut cparticle_t = 0 as *mut cparticle_t;
    i = 0 as libc::c_int;
    while i < 300 as libc::c_int {
        if free_particles.is_null() {
            return;
        }
        p = free_particles;
        free_particles = (*p).next;
        let ref mut fresh34 = (*p).next;
        *fresh34 = active_particles;
        active_particles = p;
        let ref mut fresh35 = (*p).accel[2 as libc::c_int as usize];
        *fresh35 = 0 as libc::c_int as vec_t;
        let ref mut fresh36 = (*p).accel[1 as libc::c_int as usize];
        *fresh36 = *fresh35;
        (*p).accel[0 as libc::c_int as usize] = *fresh36;
        (*p).time = cl.time as libc::c_float;
        (*p).alpha = 1.0f64 as libc::c_float;
        (*p).alphavel = -10000.0f64 as libc::c_float;
        (*p).color = 0 as libc::c_int as libc::c_float;
        dir[0 as libc::c_int as usize] = crand();
        dir[1 as libc::c_int as usize] = crand();
        dir[2 as libc::c_int as usize] = crand();
        VectorNormalize(dir.as_mut_ptr());
        VectorMA(
            origin,
            40 as libc::c_int as libc::c_float,
            dir.as_mut_ptr(),
            ((*p).org).as_mut_ptr(),
        );
        i += 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn CL_MonsterPlasma_Shell(mut origin: *mut vec_t) {
    let mut dir: vec3_t = [0.; 3];
    let mut i: libc::c_int = 0;
    let mut p: *mut cparticle_t = 0 as *mut cparticle_t;
    i = 0 as libc::c_int;
    while i < 40 as libc::c_int {
        if free_particles.is_null() {
            return;
        }
        p = free_particles;
        free_particles = (*p).next;
        let ref mut fresh37 = (*p).next;
        *fresh37 = active_particles;
        active_particles = p;
        let ref mut fresh38 = (*p).accel[2 as libc::c_int as usize];
        *fresh38 = 0 as libc::c_int as vec_t;
        let ref mut fresh39 = (*p).accel[1 as libc::c_int as usize];
        *fresh39 = *fresh38;
        (*p).accel[0 as libc::c_int as usize] = *fresh39;
        (*p).time = cl.time as libc::c_float;
        (*p).alpha = 1.0f64 as libc::c_float;
        (*p).alphavel = -10000.0f64 as libc::c_float;
        (*p).color = 0xe0 as libc::c_int as libc::c_float;
        dir[0 as libc::c_int as usize] = crand();
        dir[1 as libc::c_int as usize] = crand();
        dir[2 as libc::c_int as usize] = crand();
        VectorNormalize(dir.as_mut_ptr());
        VectorMA(
            origin,
            10 as libc::c_int as libc::c_float,
            dir.as_mut_ptr(),
            ((*p).org).as_mut_ptr(),
        );
        i += 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn CL_Widowbeamout(mut self_0: *mut cl_sustain_t) {
    let mut dir: vec3_t = [0.; 3];
    let mut i: libc::c_int = 0;
    let mut p: *mut cparticle_t = 0 as *mut cparticle_t;
    static mut colortable: [libc::c_int; 4] = [
        2 as libc::c_int * 8 as libc::c_int,
        13 as libc::c_int * 8 as libc::c_int,
        21 as libc::c_int * 8 as libc::c_int,
        18 as libc::c_int * 8 as libc::c_int,
    ];
    let mut ratio: libc::c_float = 0.;
    ratio = (1.0f64
        - ((*self_0).endtime as libc::c_float - cl.time as libc::c_float)
        as libc::c_double / 2100.0f64) as libc::c_float;
    i = 0 as libc::c_int;
    while i < 300 as libc::c_int {
        if free_particles.is_null() {
            return;
        }
        p = free_particles;
        free_particles = (*p).next;
        let ref mut fresh40 = (*p).next;
        *fresh40 = active_particles;
        active_particles = p;
        let ref mut fresh41 = (*p).accel[2 as libc::c_int as usize];
        *fresh41 = 0 as libc::c_int as vec_t;
        let ref mut fresh42 = (*p).accel[1 as libc::c_int as usize];
        *fresh42 = *fresh41;
        (*p).accel[0 as libc::c_int as usize] = *fresh42;
        (*p).time = cl.time as libc::c_float;
        (*p).alpha = 1.0f64 as libc::c_float;
        (*p).alphavel = -10000.0f64 as libc::c_float;
        (*p).color = colortable[(rand() & 3 as libc::c_int) as usize] as libc::c_float;
        dir[0 as libc::c_int as usize] = crand();
        dir[1 as libc::c_int as usize] = crand();
        dir[2 as libc::c_int as usize] = crand();
        VectorNormalize(dir.as_mut_ptr());
        VectorMA(
            ((*self_0).org).as_mut_ptr(),
            (45.0f64 * ratio as libc::c_double) as libc::c_float,
            dir.as_mut_ptr(),
            ((*p).org).as_mut_ptr(),
        );
        i += 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn CL_Nukeblast(mut self_0: *mut cl_sustain_t) {
    let mut dir: vec3_t = [0.; 3];
    let mut i: libc::c_int = 0;
    let mut p: *mut cparticle_t = 0 as *mut cparticle_t;
    static mut colortable: [libc::c_int; 4] = [
        110 as libc::c_int,
        112 as libc::c_int,
        114 as libc::c_int,
        116 as libc::c_int,
    ];
    let mut ratio: libc::c_float = 0.;
    ratio = (1.0f64
        - ((*self_0).endtime as libc::c_float - cl.time as libc::c_float)
        as libc::c_double / 1000.0f64) as libc::c_float;
    i = 0 as libc::c_int;
    while i < 700 as libc::c_int {
        if free_particles.is_null() {
            return;
        }
        p = free_particles;
        free_particles = (*p).next;
        let ref mut fresh43 = (*p).next;
        *fresh43 = active_particles;
        active_particles = p;
        let ref mut fresh44 = (*p).accel[2 as libc::c_int as usize];
        *fresh44 = 0 as libc::c_int as vec_t;
        let ref mut fresh45 = (*p).accel[1 as libc::c_int as usize];
        *fresh45 = *fresh44;
        (*p).accel[0 as libc::c_int as usize] = *fresh45;
        (*p).time = cl.time as libc::c_float;
        (*p).alpha = 1.0f64 as libc::c_float;
        (*p).alphavel = -10000.0f64 as libc::c_float;
        (*p).color = colortable[(rand() & 3 as libc::c_int) as usize] as libc::c_float;
        dir[0 as libc::c_int as usize] = crand();
        dir[1 as libc::c_int as usize] = crand();
        dir[2 as libc::c_int as usize] = crand();
        VectorNormalize(dir.as_mut_ptr());
        VectorMA(
            ((*self_0).org).as_mut_ptr(),
            (200.0f64 * ratio as libc::c_double) as libc::c_float,
            dir.as_mut_ptr(),
            ((*p).org).as_mut_ptr(),
        );
        i += 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn CL_WidowSplash(mut org: *mut vec_t) {
    static mut colortable: [libc::c_int; 4] = [
        2 as libc::c_int * 8 as libc::c_int,
        13 as libc::c_int * 8 as libc::c_int,
        21 as libc::c_int * 8 as libc::c_int,
        18 as libc::c_int * 8 as libc::c_int,
    ];
    let mut i: libc::c_int = 0;
    let mut p: *mut cparticle_t = 0 as *mut cparticle_t;
    let mut dir: vec3_t = [0.; 3];
    i = 0 as libc::c_int;
    while i < 256 as libc::c_int {
        if free_particles.is_null() {
            return;
        }
        p = free_particles;
        free_particles = (*p).next;
        let ref mut fresh46 = (*p).next;
        *fresh46 = active_particles;
        active_particles = p;
        (*p).time = cl.time as libc::c_float;
        (*p).color = colortable[(rand() & 3 as libc::c_int) as usize] as libc::c_float;
        dir[0 as libc::c_int as usize] = crand();
        dir[1 as libc::c_int as usize] = crand();
        dir[2 as libc::c_int as usize] = crand();
        VectorNormalize(dir.as_mut_ptr());
        VectorMA(
            org,
            45.0f64 as libc::c_float,
            dir.as_mut_ptr(),
            ((*p).org).as_mut_ptr(),
        );
        VectorMA(
            vec3_origin.as_mut_ptr(),
            40.0f64 as libc::c_float,
            dir.as_mut_ptr(),
            ((*p).vel).as_mut_ptr(),
        );
        let ref mut fresh47 = (*p).accel[1 as libc::c_int as usize];
        *fresh47 = 0 as libc::c_int as vec_t;
        (*p).accel[0 as libc::c_int as usize] = *fresh47;
        (*p).alpha = 1.0f64 as libc::c_float;
        (*p)
            .alphavel = (-0.8f64 / (0.5f64 + frand() as libc::c_double * 0.3f64))
            as libc::c_float;
        i += 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn CL_Tracker_Explode(mut origin: *mut vec_t) {
    let mut dir: vec3_t = [0.; 3];
    let mut backdir: vec3_t = [0.; 3];
    let mut i: libc::c_int = 0;
    let mut p: *mut cparticle_t = 0 as *mut cparticle_t;
    i = 0 as libc::c_int;
    while i < 300 as libc::c_int {
        if free_particles.is_null() {
            return;
        }
        p = free_particles;
        free_particles = (*p).next;
        let ref mut fresh48 = (*p).next;
        *fresh48 = active_particles;
        active_particles = p;
        let ref mut fresh49 = (*p).accel[2 as libc::c_int as usize];
        *fresh49 = 0 as libc::c_int as vec_t;
        let ref mut fresh50 = (*p).accel[1 as libc::c_int as usize];
        *fresh50 = *fresh49;
        (*p).accel[0 as libc::c_int as usize] = *fresh50;
        (*p).time = cl.time as libc::c_float;
        (*p).alpha = 1.0f64 as libc::c_float;
        (*p).alphavel = -1.0f64 as libc::c_float;
        (*p).color = 0 as libc::c_int as libc::c_float;
        dir[0 as libc::c_int as usize] = crand();
        dir[1 as libc::c_int as usize] = crand();
        dir[2 as libc::c_int as usize] = crand();
        VectorNormalize(dir.as_mut_ptr());
        VectorScale(
            dir.as_mut_ptr(),
            -(1 as libc::c_int) as vec_t,
            backdir.as_mut_ptr(),
        );
        VectorMA(
            origin,
            64 as libc::c_int as libc::c_float,
            dir.as_mut_ptr(),
            ((*p).org).as_mut_ptr(),
        );
        VectorScale(
            backdir.as_mut_ptr(),
            64 as libc::c_int as vec_t,
            ((*p).vel).as_mut_ptr(),
        );
        i += 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn CL_TagTrail(
    mut start: *mut vec_t,
    mut end: *mut vec_t,
    mut color: libc::c_float,
) {
    let mut move_0: vec3_t = [0.; 3];
    let mut vec: vec3_t = [0.; 3];
    let mut len: libc::c_float = 0.;
    let mut j: libc::c_int = 0;
    let mut p: *mut cparticle_t = 0 as *mut cparticle_t;
    let mut dec: libc::c_int = 0;
    move_0[0 as libc::c_int as usize] = *start.offset(0 as libc::c_int as isize);
    move_0[1 as libc::c_int as usize] = *start.offset(1 as libc::c_int as isize);
    move_0[2 as libc::c_int as usize] = *start.offset(2 as libc::c_int as isize);
    vec[0 as libc::c_int
        as usize] = *end.offset(0 as libc::c_int as isize)
        - *start.offset(0 as libc::c_int as isize);
    vec[1 as libc::c_int
        as usize] = *end.offset(1 as libc::c_int as isize)
        - *start.offset(1 as libc::c_int as isize);
    vec[2 as libc::c_int
        as usize] = *end.offset(2 as libc::c_int as isize)
        - *start.offset(2 as libc::c_int as isize);
    len = VectorNormalize(vec.as_mut_ptr());
    dec = 5 as libc::c_int;
    VectorScale(vec.as_mut_ptr(), 5 as libc::c_int as vec_t, vec.as_mut_ptr());
    while len >= 0 as libc::c_int as libc::c_float {
        len -= dec as libc::c_float;
        if free_particles.is_null() {
            return;
        }
        p = free_particles;
        free_particles = (*p).next;
        let ref mut fresh51 = (*p).next;
        *fresh51 = active_particles;
        active_particles = p;
        let ref mut fresh52 = (*p).accel[2 as libc::c_int as usize];
        *fresh52 = 0 as libc::c_int as vec_t;
        let ref mut fresh53 = (*p).accel[1 as libc::c_int as usize];
        *fresh53 = *fresh52;
        (*p).accel[0 as libc::c_int as usize] = *fresh53;
        (*p).time = cl.time as libc::c_float;
        (*p).alpha = 1.0f64 as libc::c_float;
        (*p)
            .alphavel = (-1.0f64 / (0.8f64 + frand() as libc::c_double * 0.2f64))
            as libc::c_float;
        (*p).color = color;
        j = 0 as libc::c_int;
        while j < 3 as libc::c_int {
            (*p)
                .org[j
                as usize] = move_0[j as usize]
                + crand() * 16 as libc::c_int as libc::c_float;
            (*p).vel[j as usize] = crand() * 5 as libc::c_int as libc::c_float;
            (*p).accel[j as usize] = 0 as libc::c_int as vec_t;
            j += 1;
        }
        move_0[0 as libc::c_int
            as usize] = move_0[0 as libc::c_int as usize]
            + vec[0 as libc::c_int as usize];
        move_0[1 as libc::c_int
            as usize] = move_0[1 as libc::c_int as usize]
            + vec[1 as libc::c_int as usize];
        move_0[2 as libc::c_int
            as usize] = move_0[2 as libc::c_int as usize]
            + vec[2 as libc::c_int as usize];
    }
}

#[no_mangle]
pub unsafe extern "C" fn CL_ColorExplosionParticles(
    mut org: *mut vec_t,
    mut color: libc::c_int,
    mut run: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut p: *mut cparticle_t = 0 as *mut cparticle_t;
    i = 0 as libc::c_int;
    while i < 128 as libc::c_int {
        if free_particles.is_null() {
            return;
        }
        p = free_particles;
        free_particles = (*p).next;
        let ref mut fresh54 = (*p).next;
        *fresh54 = active_particles;
        active_particles = p;
        (*p).time = cl.time as libc::c_float;
        (*p).color = (color + rand() % run) as libc::c_float;
        j = 0 as libc::c_int;
        while j < 3 as libc::c_int {
            (*p)
                .org[j
                as usize] = *org.offset(j as isize)
                + (rand() % 32 as libc::c_int - 16 as libc::c_int) as libc::c_float;
            (*p)
                .vel[j
                as usize] = (rand() % 256 as libc::c_int - 128 as libc::c_int) as vec_t;
            j += 1;
        }
        let ref mut fresh55 = (*p).accel[1 as libc::c_int as usize];
        *fresh55 = 0 as libc::c_int as vec_t;
        (*p).accel[0 as libc::c_int as usize] = *fresh55;
        (*p).accel[2 as libc::c_int as usize] = -(40 as libc::c_int) as vec_t;
        (*p).alpha = 1.0f64 as libc::c_float;
        (*p)
            .alphavel = (-0.4f64 / (0.6f64 + frand() as libc::c_double * 0.2f64))
            as libc::c_float;
        i += 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn CL_ParticleSmokeEffect(
    mut org: *mut vec_t,
    mut dir: *mut vec_t,
    mut color: libc::c_int,
    mut count: libc::c_int,
    mut magnitude: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut p: *mut cparticle_t = 0 as *mut cparticle_t;
    let mut d: libc::c_float = 0.;
    let mut r: vec3_t = [0.; 3];
    let mut u: vec3_t = [0.; 3];
    MakeNormalVectors(dir, r.as_mut_ptr(), u.as_mut_ptr());
    i = 0 as libc::c_int;
    while i < count {
        if free_particles.is_null() {
            return;
        }
        p = free_particles;
        free_particles = (*p).next;
        let ref mut fresh56 = (*p).next;
        *fresh56 = active_particles;
        active_particles = p;
        (*p).time = cl.time as libc::c_float;
        (*p).color = (color + (rand() & 7 as libc::c_int)) as libc::c_float;
        j = 0 as libc::c_int;
        while j < 3 as libc::c_int {
            (*p)
                .org[j
                as usize] = (*org.offset(j as isize) as libc::c_double
                + magnitude as libc::c_double * 0.1f64 * crand() as libc::c_double)
                as vec_t;
            j += 1;
        }
        VectorScale(dir, magnitude as vec_t, ((*p).vel).as_mut_ptr());
        d = crand() * magnitude as libc::c_float / 3 as libc::c_int as libc::c_float;
        VectorMA(((*p).vel).as_mut_ptr(), d, r.as_mut_ptr(), ((*p).vel).as_mut_ptr());
        d = crand() * magnitude as libc::c_float / 3 as libc::c_int as libc::c_float;
        VectorMA(((*p).vel).as_mut_ptr(), d, u.as_mut_ptr(), ((*p).vel).as_mut_ptr());
        let ref mut fresh57 = (*p).accel[2 as libc::c_int as usize];
        *fresh57 = 0 as libc::c_int as vec_t;
        let ref mut fresh58 = (*p).accel[1 as libc::c_int as usize];
        *fresh58 = *fresh57;
        (*p).accel[0 as libc::c_int as usize] = *fresh58;
        (*p).alpha = 1.0f64 as libc::c_float;
        (*p)
            .alphavel = (-1.0f64 / (0.5f64 + frand() as libc::c_double * 0.3f64))
            as libc::c_float;
        i += 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn CL_BlasterParticles2(
    mut org: *mut vec_t,
    mut dir: *mut vec_t,
    mut color: libc::c_uint,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut p: *mut cparticle_t = 0 as *mut cparticle_t;
    let mut d: libc::c_float = 0.;
    let mut count: libc::c_int = 0;
    count = 40 as libc::c_int;
    i = 0 as libc::c_int;
    while i < count {
        if free_particles.is_null() {
            return;
        }
        p = free_particles;
        free_particles = (*p).next;
        let ref mut fresh59 = (*p).next;
        *fresh59 = active_particles;
        active_particles = p;
        (*p).time = cl.time as libc::c_float;
        (*p)
            .color = color.wrapping_add((rand() & 7 as libc::c_int) as libc::c_uint)
            as libc::c_float;
        d = (rand() & 15 as libc::c_int) as libc::c_float;
        j = 0 as libc::c_int;
        while j < 3 as libc::c_int {
            (*p)
                .org[j
                as usize] = *org.offset(j as isize)
                + ((rand() & 7 as libc::c_int) - 4 as libc::c_int) as libc::c_float
                + d * *dir.offset(j as isize);
            (*p)
                .vel[j
                as usize] = *dir.offset(j as isize) * 30 as libc::c_int as libc::c_float
                + crand() * 40 as libc::c_int as libc::c_float;
            j += 1;
        }
        let ref mut fresh60 = (*p).accel[1 as libc::c_int as usize];
        *fresh60 = 0 as libc::c_int as vec_t;
        (*p).accel[0 as libc::c_int as usize] = *fresh60;
        (*p).accel[2 as libc::c_int as usize] = -(40 as libc::c_int) as vec_t;
        (*p).alpha = 1.0f64 as libc::c_float;
        (*p)
            .alphavel = (-1.0f64 / (0.5f64 + frand() as libc::c_double * 0.3f64))
            as libc::c_float;
        i += 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn CL_BlasterTrail2(mut start: *mut vec_t, mut end: *mut vec_t) {
    let mut move_0: vec3_t = [0.; 3];
    let mut vec: vec3_t = [0.; 3];
    let mut len: libc::c_float = 0.;
    let mut j: libc::c_int = 0;
    let mut p: *mut cparticle_t = 0 as *mut cparticle_t;
    let mut dec: libc::c_int = 0;
    move_0[0 as libc::c_int as usize] = *start.offset(0 as libc::c_int as isize);
    move_0[1 as libc::c_int as usize] = *start.offset(1 as libc::c_int as isize);
    move_0[2 as libc::c_int as usize] = *start.offset(2 as libc::c_int as isize);
    vec[0 as libc::c_int
        as usize] = *end.offset(0 as libc::c_int as isize)
        - *start.offset(0 as libc::c_int as isize);
    vec[1 as libc::c_int
        as usize] = *end.offset(1 as libc::c_int as isize)
        - *start.offset(1 as libc::c_int as isize);
    vec[2 as libc::c_int
        as usize] = *end.offset(2 as libc::c_int as isize)
        - *start.offset(2 as libc::c_int as isize);
    len = VectorNormalize(vec.as_mut_ptr());
    dec = 5 as libc::c_int;
    VectorScale(vec.as_mut_ptr(), 5 as libc::c_int as vec_t, vec.as_mut_ptr());
    while len > 0 as libc::c_int as libc::c_float {
        len -= dec as libc::c_float;
        if free_particles.is_null() {
            return;
        }
        p = free_particles;
        free_particles = (*p).next;
        let ref mut fresh61 = (*p).next;
        *fresh61 = active_particles;
        active_particles = p;
        let ref mut fresh62 = (*p).accel[2 as libc::c_int as usize];
        *fresh62 = 0 as libc::c_int as vec_t;
        let ref mut fresh63 = (*p).accel[1 as libc::c_int as usize];
        *fresh63 = *fresh62;
        (*p).accel[0 as libc::c_int as usize] = *fresh63;
        (*p).time = cl.time as libc::c_float;
        (*p).alpha = 1.0f64 as libc::c_float;
        (*p)
            .alphavel = (-1.0f64 / (0.3f64 + frand() as libc::c_double * 0.2f64))
            as libc::c_float;
        (*p).color = 0xd0 as libc::c_int as libc::c_float;
        j = 0 as libc::c_int;
        while j < 3 as libc::c_int {
            (*p).org[j as usize] = move_0[j as usize] + crand();
            (*p).vel[j as usize] = crand() * 5 as libc::c_int as libc::c_float;
            (*p).accel[j as usize] = 0 as libc::c_int as vec_t;
            j += 1;
        }
        move_0[0 as libc::c_int
            as usize] = move_0[0 as libc::c_int as usize]
            + vec[0 as libc::c_int as usize];
        move_0[1 as libc::c_int
            as usize] = move_0[1 as libc::c_int as usize]
            + vec[1 as libc::c_int as usize];
        move_0[2 as libc::c_int
            as usize] = move_0[2 as libc::c_int as usize]
            + vec[2 as libc::c_int as usize];
    }
}
