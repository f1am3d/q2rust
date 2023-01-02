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
    fn cos(_: libc::c_double) -> libc::c_double;
    fn sin(_: libc::c_double) -> libc::c_double;
    fn floor(_: libc::c_double) -> libc::c_double;
    fn V_AddParticle(org: *mut vec_t, color: libc::c_int, alpha: libc::c_float);
    fn V_AddLightStyle(
        style: libc::c_int,
        r: libc::c_float,
        g: libc::c_float,
        b: libc::c_float,
    );
    fn V_AddLight(
        org: *mut vec_t,
        intensity: libc::c_float,
        r: libc::c_float,
        g: libc::c_float,
        b: libc::c_float,
    );
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn rand() -> libc::c_int;
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
    fn Com_sprintf(
        dest: *mut libc::c_char,
        size: libc::c_int,
        fmt: *mut libc::c_char,
        _: ...
    );
    static mut monster_flash_offset: [vec3_t; 0];
    static mut vidref_val: libc::c_int;
    fn MSG_ReadByte(sb: *mut sizebuf_t) -> libc::c_int;
    fn MSG_ReadShort(sb: *mut sizebuf_t) -> libc::c_int;
    static mut net_message: sizebuf_t;
    fn Com_Error(code: libc::c_int, fmt: *mut libc::c_char, _: ...);
    fn frand() -> libc::c_float;
    fn crand() -> libc::c_float;
    static mut bytedirs: [vec3_t; 162];
    fn S_StartSound(
        origin: *mut vec_t,
        entnum: libc::c_int,
        entchannel: libc::c_int,
        sfx: *mut sfx_s,
        fvol: libc::c_float,
        attenuation: libc::c_float,
        timeofs: libc::c_float,
    );
    fn S_RegisterSound(sample: *mut libc::c_char) -> *mut sfx_s;
    static mut cl: client_state_t;
    static mut cls: client_static_t;
    static mut cl_footsteps: *mut cvar_t;
    static mut cl_entities: [centity_t; 1024];
    fn CL_SmokeAndFlash(origin: *mut vec_t);
    static mut cl_sfx_footsteps: [*mut sfx_s; 4];
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
pub struct cvar_s {
    pub name: *mut libc::c_char,
    pub string: *mut libc::c_char,
    pub latched_string: *mut libc::c_char,
    pub flags: libc::c_int,
    pub modified: qboolean,
    pub value: libc::c_float,
    pub next: *mut cvar_s,
}

pub type cvar_t = cvar_s;

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
pub type C2RustUnnamed = libc::c_uint;

pub const EV_OTHER_TELEPORT: C2RustUnnamed = 7;
pub const EV_PLAYER_TELEPORT: C2RustUnnamed = 6;
pub const EV_FALLFAR: C2RustUnnamed = 5;
pub const EV_FALL: C2RustUnnamed = 4;
pub const EV_FALLSHORT: C2RustUnnamed = 3;
pub const EV_FOOTSTEP: C2RustUnnamed = 2;
pub const EV_ITEM_RESPAWN: C2RustUnnamed = 1;
pub const EV_NONE: C2RustUnnamed = 0;

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
pub struct sizebuf_s {
    pub allowoverflow: qboolean,
    pub overflowed: qboolean,
    pub data: *mut byte,
    pub maxsize: libc::c_int,
    pub cursize: libc::c_int,
    pub readcount: libc::c_int,
}

pub type sizebuf_t = sizebuf_s;
pub type netadrtype_t = libc::c_uint;

pub const NA_BROADCAST_IPX: netadrtype_t = 4;
pub const NA_IPX: netadrtype_t = 3;
pub const NA_IP: netadrtype_t = 2;
pub const NA_BROADCAST: netadrtype_t = 1;
pub const NA_LOOPBACK: netadrtype_t = 0;

pub type netsrc_t = libc::c_uint;

pub const NS_SERVER: netsrc_t = 1;
pub const NS_CLIENT: netsrc_t = 0;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct netadr_t {
    pub type_0: netadrtype_t,
    pub ip: [byte; 4],
    pub ipx: [byte; 10],
    pub port: libc::c_ushort,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct netchan_t {
    pub fatal_error: qboolean,
    pub sock: netsrc_t,
    pub dropped: libc::c_int,
    pub last_received: libc::c_int,
    pub last_sent: libc::c_int,
    pub remote_address: netadr_t,
    pub qport: libc::c_int,
    pub incoming_sequence: libc::c_int,
    pub incoming_acknowledged: libc::c_int,
    pub incoming_reliable_acknowledged: libc::c_int,
    pub incoming_reliable_sequence: libc::c_int,
    pub outgoing_sequence: libc::c_int,
    pub reliable_sequence: libc::c_int,
    pub last_reliable_sequence: libc::c_int,
    pub message: sizebuf_t,
    pub message_buf: [byte; 1384],
    pub reliable_length: libc::c_int,
    pub reliable_buf: [byte; 1384],
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

pub type connstate_t = libc::c_uint;

pub const ca_active: connstate_t = 4;
pub const ca_connected: connstate_t = 3;
pub const ca_connecting: connstate_t = 2;
pub const ca_disconnected: connstate_t = 1;
pub const ca_uninitialized: connstate_t = 0;

pub type dltype_t = libc::c_uint;

pub const dl_single: dltype_t = 4;
pub const dl_skin: dltype_t = 3;
pub const dl_sound: dltype_t = 2;
pub const dl_model: dltype_t = 1;
pub const dl_none: dltype_t = 0;

pub type keydest_t = libc::c_uint;

pub const key_menu: keydest_t = 3;
pub const key_message: keydest_t = 2;
pub const key_console: keydest_t = 1;
pub const key_game: keydest_t = 0;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct client_static_t {
    pub state: connstate_t,
    pub key_dest: keydest_t,
    pub framecount: libc::c_int,
    pub realtime: libc::c_int,
    pub frametime: libc::c_float,
    pub disable_screen: libc::c_float,
    pub disable_servercount: libc::c_int,
    pub servername: [libc::c_char; 128],
    pub connect_time: libc::c_float,
    pub quakePort: libc::c_int,
    pub netchan: netchan_t,
    pub serverProtocol: libc::c_int,
    pub challenge: libc::c_int,
    pub download: *mut FILE,
    pub downloadtempname: [libc::c_char; 128],
    pub downloadname: [libc::c_char; 128],
    pub downloadnumber: libc::c_int,
    pub downloadtype: dltype_t,
    pub downloadpercent: libc::c_int,
    pub demorecording: qboolean,
    pub demowaiting: qboolean,
    pub demofile: *mut FILE,
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

#[derive(Copy, Clone)]
#[repr(C)]
pub struct clightstyle_t {
    pub length: libc::c_int,
    pub value: [libc::c_float; 3],
    pub map: [libc::c_float; 64],
}

static mut avelocities: [vec3_t; 162] = [[0.; 3]; 162];
#[no_mangle]
pub static mut cl_lightstyle: [clightstyle_t; 256] = [clightstyle_t {
    length: 0,
    value: [0.; 3],
    map: [0.; 64],
}; 256];
#[no_mangle]
pub static mut lastofs: libc::c_int = 0;

#[no_mangle]
pub unsafe extern "C" fn CL_ClearLightStyles() {
    memset(
        cl_lightstyle.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<[clightstyle_t; 256]>() as libc::c_ulong,
    );
    lastofs = -(1 as libc::c_int);
}

#[no_mangle]
pub unsafe extern "C" fn CL_RunLightStyles() {
    let mut ofs: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut ls: *mut clightstyle_t = 0 as *mut clightstyle_t;
    ofs = cl.time / 100 as libc::c_int;
    if ofs == lastofs {
        return;
    }
    lastofs = ofs;
    i = 0 as libc::c_int;
    ls = cl_lightstyle.as_mut_ptr();
    while i < 256 as libc::c_int {
        if (*ls).length == 0 {
            let ref mut fresh0 = (*ls).value[2 as libc::c_int as usize];
            *fresh0 = 1.0f64 as libc::c_float;
            let ref mut fresh1 = (*ls).value[1 as libc::c_int as usize];
            *fresh1 = *fresh0;
            (*ls).value[0 as libc::c_int as usize] = *fresh1;
        } else if (*ls).length == 1 as libc::c_int {
            let ref mut fresh2 = (*ls).value[2 as libc::c_int as usize];
            *fresh2 = (*ls).map[0 as libc::c_int as usize];
            let ref mut fresh3 = (*ls).value[1 as libc::c_int as usize];
            *fresh3 = *fresh2;
            (*ls).value[0 as libc::c_int as usize] = *fresh3;
        } else {
            let ref mut fresh4 = (*ls).value[2 as libc::c_int as usize];
            *fresh4 = (*ls).map[(ofs % (*ls).length) as usize];
            let ref mut fresh5 = (*ls).value[1 as libc::c_int as usize];
            *fresh5 = *fresh4;
            (*ls).value[0 as libc::c_int as usize] = *fresh5;
        }
        i += 1;
        ls = ls.offset(1);
    }
}

#[no_mangle]
pub unsafe extern "C" fn CL_SetLightstyle(mut i: libc::c_int) {
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    s = (cl
        .configstrings[(i
        + (32 as libc::c_int + 256 as libc::c_int + 256 as libc::c_int
        + 256 as libc::c_int)) as usize])
        .as_mut_ptr();
    j = strlen(s) as libc::c_int;
    if j >= 64 as libc::c_int {
        Com_Error(
            1 as libc::c_int,
            b"svc_lightstyle length=%i\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            j,
        );
    }
    cl_lightstyle[i as usize].length = j;
    k = 0 as libc::c_int;
    while k < j {
        cl_lightstyle[i as usize]
            .map[k
            as usize] = (*s.offset(k as isize) as libc::c_int - 'a' as i32)
            as libc::c_float / ('m' as i32 - 'a' as i32) as libc::c_float;
        k += 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn CL_AddLightStyles() {
    let mut i: libc::c_int = 0;
    let mut ls: *mut clightstyle_t = 0 as *mut clightstyle_t;
    i = 0 as libc::c_int;
    ls = cl_lightstyle.as_mut_ptr();
    while i < 256 as libc::c_int {
        V_AddLightStyle(
            i,
            (*ls).value[0 as libc::c_int as usize],
            (*ls).value[1 as libc::c_int as usize],
            (*ls).value[2 as libc::c_int as usize],
        );
        i += 1;
        ls = ls.offset(1);
    }
}

#[no_mangle]
pub static mut cl_dlights: [cdlight_t; 32] = [cdlight_t {
    key: 0,
    color: [0.; 3],
    origin: [0.; 3],
    radius: 0.,
    die: 0.,
    decay: 0.,
    minlight: 0.,
}; 32];

#[no_mangle]
pub unsafe extern "C" fn CL_ClearDlights() {
    memset(
        cl_dlights.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<[cdlight_t; 32]>() as libc::c_ulong,
    );
}

#[no_mangle]
pub unsafe extern "C" fn CL_AllocDlight(mut key: libc::c_int) -> *mut cdlight_t {
    let mut i: libc::c_int = 0;
    let mut dl: *mut cdlight_t = 0 as *mut cdlight_t;
    if key != 0 {
        dl = cl_dlights.as_mut_ptr();
        i = 0 as libc::c_int;
        while i < 32 as libc::c_int {
            if (*dl).key == key {
                memset(
                    dl as *mut libc::c_void,
                    0 as libc::c_int,
                    ::std::mem::size_of::<cdlight_t>() as libc::c_ulong,
                );
                (*dl).key = key;
                return dl;
            }
            i += 1;
            dl = dl.offset(1);
        }
    }
    dl = cl_dlights.as_mut_ptr();
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        if (*dl).die < cl.time as libc::c_float {
            memset(
                dl as *mut libc::c_void,
                0 as libc::c_int,
                ::std::mem::size_of::<cdlight_t>() as libc::c_ulong,
            );
            (*dl).key = key;
            return dl;
        }
        i += 1;
        dl = dl.offset(1);
    }
    dl = &mut *cl_dlights.as_mut_ptr().offset(0 as libc::c_int as isize)
        as *mut cdlight_t;
    memset(
        dl as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<cdlight_t>() as libc::c_ulong,
    );
    (*dl).key = key;
    return dl;
}

#[no_mangle]
pub unsafe extern "C" fn CL_NewDlight(
    mut key: libc::c_int,
    mut x: libc::c_float,
    mut y: libc::c_float,
    mut z: libc::c_float,
    mut radius: libc::c_float,
    mut time: libc::c_float,
) {
    let mut dl: *mut cdlight_t = 0 as *mut cdlight_t;
    dl = CL_AllocDlight(key);
    (*dl).origin[0 as libc::c_int as usize] = x;
    (*dl).origin[1 as libc::c_int as usize] = y;
    (*dl).origin[2 as libc::c_int as usize] = z;
    (*dl).radius = radius;
    (*dl).die = cl.time as libc::c_float + time;
}

#[no_mangle]
pub unsafe extern "C" fn CL_RunDLights() {
    let mut i: libc::c_int = 0;
    let mut dl: *mut cdlight_t = 0 as *mut cdlight_t;
    dl = cl_dlights.as_mut_ptr();
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        if !((*dl).radius == 0.) {
            if (*dl).die < cl.time as libc::c_float {
                (*dl).radius = 0 as libc::c_int as libc::c_float;
                return;
            }
            (*dl).radius -= cls.frametime * (*dl).decay;
            if (*dl).radius < 0 as libc::c_int as libc::c_float {
                (*dl).radius = 0 as libc::c_int as libc::c_float;
            }
        }
        i += 1;
        dl = dl.offset(1);
    }
}

#[no_mangle]
pub unsafe extern "C" fn CL_ParseMuzzleFlash() {
    let mut fv: vec3_t = [0.; 3];
    let mut rv: vec3_t = [0.; 3];
    let mut dl: *mut cdlight_t = 0 as *mut cdlight_t;
    let mut i: libc::c_int = 0;
    let mut weapon: libc::c_int = 0;
    let mut pl: *mut centity_t = 0 as *mut centity_t;
    let mut silenced: libc::c_int = 0;
    let mut volume: libc::c_float = 0.;
    let mut soundname: [libc::c_char; 64] = [0; 64];
    i = MSG_ReadShort(&mut net_message);
    if i < 1 as libc::c_int || i >= 1024 as libc::c_int {
        Com_Error(
            1 as libc::c_int,
            b"CL_ParseMuzzleFlash: bad entity\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    weapon = MSG_ReadByte(&mut net_message);
    silenced = weapon & 128 as libc::c_int;
    weapon &= !(128 as libc::c_int);
    pl = &mut *cl_entities.as_mut_ptr().offset(i as isize) as *mut centity_t;
    dl = CL_AllocDlight(i);
    (*dl)
        .origin[0 as libc::c_int
        as usize] = (*pl).current.origin[0 as libc::c_int as usize];
    (*dl)
        .origin[1 as libc::c_int
        as usize] = (*pl).current.origin[1 as libc::c_int as usize];
    (*dl)
        .origin[2 as libc::c_int
        as usize] = (*pl).current.origin[2 as libc::c_int as usize];
    AngleVectors(
        ((*pl).current.angles).as_mut_ptr(),
        fv.as_mut_ptr(),
        rv.as_mut_ptr(),
        0 as *mut vec_t,
    );
    VectorMA(
        ((*dl).origin).as_mut_ptr(),
        18 as libc::c_int as libc::c_float,
        fv.as_mut_ptr(),
        ((*dl).origin).as_mut_ptr(),
    );
    VectorMA(
        ((*dl).origin).as_mut_ptr(),
        16 as libc::c_int as libc::c_float,
        rv.as_mut_ptr(),
        ((*dl).origin).as_mut_ptr(),
    );
    if silenced != 0 {
        (*dl)
            .radius = (100 as libc::c_int + (rand() & 31 as libc::c_int))
            as libc::c_float;
    } else {
        (*dl)
            .radius = (200 as libc::c_int + (rand() & 31 as libc::c_int))
            as libc::c_float;
    }
    (*dl).minlight = 32 as libc::c_int as libc::c_float;
    (*dl).die = cl.time as libc::c_float;
    if silenced != 0 {
        volume = 0.2f64 as libc::c_float;
    } else {
        volume = 1 as libc::c_int as libc::c_float;
    }
    match weapon {
        0 => {
            (*dl).color[0 as libc::c_int as usize] = 1 as libc::c_int as vec_t;
            (*dl).color[1 as libc::c_int as usize] = 1 as libc::c_int as vec_t;
            (*dl).color[2 as libc::c_int as usize] = 0 as libc::c_int as vec_t;
            S_StartSound(
                0 as *mut vec_t,
                i,
                1 as libc::c_int,
                S_RegisterSound(
                    b"weapons/blastf1a.wav\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                ),
                volume,
                1 as libc::c_int as libc::c_float,
                0 as libc::c_int as libc::c_float,
            );
        }
        17 => {
            (*dl).color[0 as libc::c_int as usize] = 0 as libc::c_int as vec_t;
            (*dl).color[1 as libc::c_int as usize] = 0 as libc::c_int as vec_t;
            (*dl).color[2 as libc::c_int as usize] = 1 as libc::c_int as vec_t;
            S_StartSound(
                0 as *mut vec_t,
                i,
                1 as libc::c_int,
                S_RegisterSound(
                    b"weapons/hyprbf1a.wav\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                ),
                volume,
                1 as libc::c_int as libc::c_float,
                0 as libc::c_int as libc::c_float,
            );
        }
        14 => {
            (*dl).color[0 as libc::c_int as usize] = 1 as libc::c_int as vec_t;
            (*dl).color[1 as libc::c_int as usize] = 1 as libc::c_int as vec_t;
            (*dl).color[2 as libc::c_int as usize] = 0 as libc::c_int as vec_t;
            S_StartSound(
                0 as *mut vec_t,
                i,
                1 as libc::c_int,
                S_RegisterSound(
                    b"weapons/hyprbf1a.wav\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                ),
                volume,
                1 as libc::c_int as libc::c_float,
                0 as libc::c_int as libc::c_float,
            );
        }
        1 => {
            (*dl).color[0 as libc::c_int as usize] = 1 as libc::c_int as vec_t;
            (*dl).color[1 as libc::c_int as usize] = 1 as libc::c_int as vec_t;
            (*dl).color[2 as libc::c_int as usize] = 0 as libc::c_int as vec_t;
            Com_sprintf(
                soundname.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong
                    as libc::c_int,
                b"weapons/machgf%ib.wav\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                rand() % 5 as libc::c_int + 1 as libc::c_int,
            );
            S_StartSound(
                0 as *mut vec_t,
                i,
                1 as libc::c_int,
                S_RegisterSound(soundname.as_mut_ptr()),
                volume,
                1 as libc::c_int as libc::c_float,
                0 as libc::c_int as libc::c_float,
            );
        }
        2 => {
            (*dl).color[0 as libc::c_int as usize] = 1 as libc::c_int as vec_t;
            (*dl).color[1 as libc::c_int as usize] = 1 as libc::c_int as vec_t;
            (*dl).color[2 as libc::c_int as usize] = 0 as libc::c_int as vec_t;
            S_StartSound(
                0 as *mut vec_t,
                i,
                1 as libc::c_int,
                S_RegisterSound(
                    b"weapons/shotgf1b.wav\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                ),
                volume,
                1 as libc::c_int as libc::c_float,
                0 as libc::c_int as libc::c_float,
            );
            S_StartSound(
                0 as *mut vec_t,
                i,
                0 as libc::c_int,
                S_RegisterSound(
                    b"weapons/shotgr1b.wav\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                ),
                volume,
                1 as libc::c_int as libc::c_float,
                0.1f64 as libc::c_float,
            );
        }
        13 => {
            (*dl).color[0 as libc::c_int as usize] = 1 as libc::c_int as vec_t;
            (*dl).color[1 as libc::c_int as usize] = 1 as libc::c_int as vec_t;
            (*dl).color[2 as libc::c_int as usize] = 0 as libc::c_int as vec_t;
            S_StartSound(
                0 as *mut vec_t,
                i,
                1 as libc::c_int,
                S_RegisterSound(
                    b"weapons/sshotf1b.wav\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                ),
                volume,
                1 as libc::c_int as libc::c_float,
                0 as libc::c_int as libc::c_float,
            );
        }
        3 => {
            (*dl)
                .radius = (200 as libc::c_int + (rand() & 31 as libc::c_int))
                as libc::c_float;
            (*dl).color[0 as libc::c_int as usize] = 1 as libc::c_int as vec_t;
            (*dl).color[1 as libc::c_int as usize] = 0.25f64 as vec_t;
            (*dl).color[2 as libc::c_int as usize] = 0 as libc::c_int as vec_t;
            Com_sprintf(
                soundname.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong
                    as libc::c_int,
                b"weapons/machgf%ib.wav\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                rand() % 5 as libc::c_int + 1 as libc::c_int,
            );
            S_StartSound(
                0 as *mut vec_t,
                i,
                1 as libc::c_int,
                S_RegisterSound(soundname.as_mut_ptr()),
                volume,
                1 as libc::c_int as libc::c_float,
                0 as libc::c_int as libc::c_float,
            );
        }
        4 => {
            (*dl)
                .radius = (225 as libc::c_int + (rand() & 31 as libc::c_int))
                as libc::c_float;
            (*dl).color[0 as libc::c_int as usize] = 1 as libc::c_int as vec_t;
            (*dl).color[1 as libc::c_int as usize] = 0.5f64 as vec_t;
            (*dl).color[2 as libc::c_int as usize] = 0 as libc::c_int as vec_t;
            (*dl).die = (cl.time as libc::c_double + 0.1f64) as libc::c_float;
            Com_sprintf(
                soundname.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong
                    as libc::c_int,
                b"weapons/machgf%ib.wav\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                rand() % 5 as libc::c_int + 1 as libc::c_int,
            );
            S_StartSound(
                0 as *mut vec_t,
                i,
                1 as libc::c_int,
                S_RegisterSound(soundname.as_mut_ptr()),
                volume,
                1 as libc::c_int as libc::c_float,
                0 as libc::c_int as libc::c_float,
            );
            Com_sprintf(
                soundname.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong
                    as libc::c_int,
                b"weapons/machgf%ib.wav\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                rand() % 5 as libc::c_int + 1 as libc::c_int,
            );
            S_StartSound(
                0 as *mut vec_t,
                i,
                1 as libc::c_int,
                S_RegisterSound(soundname.as_mut_ptr()),
                volume,
                1 as libc::c_int as libc::c_float,
                0.05f64 as libc::c_float,
            );
        }
        5 => {
            (*dl)
                .radius = (250 as libc::c_int + (rand() & 31 as libc::c_int))
                as libc::c_float;
            (*dl).color[0 as libc::c_int as usize] = 1 as libc::c_int as vec_t;
            (*dl).color[1 as libc::c_int as usize] = 1 as libc::c_int as vec_t;
            (*dl).color[2 as libc::c_int as usize] = 0 as libc::c_int as vec_t;
            (*dl).die = (cl.time as libc::c_double + 0.1f64) as libc::c_float;
            Com_sprintf(
                soundname.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong
                    as libc::c_int,
                b"weapons/machgf%ib.wav\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                rand() % 5 as libc::c_int + 1 as libc::c_int,
            );
            S_StartSound(
                0 as *mut vec_t,
                i,
                1 as libc::c_int,
                S_RegisterSound(soundname.as_mut_ptr()),
                volume,
                1 as libc::c_int as libc::c_float,
                0 as libc::c_int as libc::c_float,
            );
            Com_sprintf(
                soundname.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong
                    as libc::c_int,
                b"weapons/machgf%ib.wav\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                rand() % 5 as libc::c_int + 1 as libc::c_int,
            );
            S_StartSound(
                0 as *mut vec_t,
                i,
                1 as libc::c_int,
                S_RegisterSound(soundname.as_mut_ptr()),
                volume,
                1 as libc::c_int as libc::c_float,
                0.033f64 as libc::c_float,
            );
            Com_sprintf(
                soundname.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong
                    as libc::c_int,
                b"weapons/machgf%ib.wav\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                rand() % 5 as libc::c_int + 1 as libc::c_int,
            );
            S_StartSound(
                0 as *mut vec_t,
                i,
                1 as libc::c_int,
                S_RegisterSound(soundname.as_mut_ptr()),
                volume,
                1 as libc::c_int as libc::c_float,
                0.066f64 as libc::c_float,
            );
        }
        6 => {
            (*dl).color[0 as libc::c_int as usize] = 0.5f64 as vec_t;
            (*dl).color[1 as libc::c_int as usize] = 0.5f64 as vec_t;
            (*dl).color[2 as libc::c_int as usize] = 1.0f64 as vec_t;
            S_StartSound(
                0 as *mut vec_t,
                i,
                1 as libc::c_int,
                S_RegisterSound(
                    b"weapons/railgf1a.wav\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                ),
                volume,
                1 as libc::c_int as libc::c_float,
                0 as libc::c_int as libc::c_float,
            );
        }
        7 => {
            (*dl).color[0 as libc::c_int as usize] = 1 as libc::c_int as vec_t;
            (*dl).color[1 as libc::c_int as usize] = 0.5f64 as vec_t;
            (*dl).color[2 as libc::c_int as usize] = 0.2f64 as vec_t;
            S_StartSound(
                0 as *mut vec_t,
                i,
                1 as libc::c_int,
                S_RegisterSound(
                    b"weapons/rocklf1a.wav\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                ),
                volume,
                1 as libc::c_int as libc::c_float,
                0 as libc::c_int as libc::c_float,
            );
            S_StartSound(
                0 as *mut vec_t,
                i,
                0 as libc::c_int,
                S_RegisterSound(
                    b"weapons/rocklr1b.wav\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                ),
                volume,
                1 as libc::c_int as libc::c_float,
                0.1f64 as libc::c_float,
            );
        }
        8 => {
            (*dl).color[0 as libc::c_int as usize] = 1 as libc::c_int as vec_t;
            (*dl).color[1 as libc::c_int as usize] = 0.5f64 as vec_t;
            (*dl).color[2 as libc::c_int as usize] = 0 as libc::c_int as vec_t;
            S_StartSound(
                0 as *mut vec_t,
                i,
                1 as libc::c_int,
                S_RegisterSound(
                    b"weapons/grenlf1a.wav\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                ),
                volume,
                1 as libc::c_int as libc::c_float,
                0 as libc::c_int as libc::c_float,
            );
            S_StartSound(
                0 as *mut vec_t,
                i,
                0 as libc::c_int,
                S_RegisterSound(
                    b"weapons/grenlr1b.wav\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                ),
                volume,
                1 as libc::c_int as libc::c_float,
                0.1f64 as libc::c_float,
            );
        }
        12 => {
            (*dl).color[0 as libc::c_int as usize] = 0 as libc::c_int as vec_t;
            (*dl).color[1 as libc::c_int as usize] = 1 as libc::c_int as vec_t;
            (*dl).color[2 as libc::c_int as usize] = 0 as libc::c_int as vec_t;
            S_StartSound(
                0 as *mut vec_t,
                i,
                1 as libc::c_int,
                S_RegisterSound(
                    b"weapons/bfg__f1y.wav\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                ),
                volume,
                1 as libc::c_int as libc::c_float,
                0 as libc::c_int as libc::c_float,
            );
        }
        9 => {
            (*dl).color[0 as libc::c_int as usize] = 0 as libc::c_int as vec_t;
            (*dl).color[1 as libc::c_int as usize] = 1 as libc::c_int as vec_t;
            (*dl).color[2 as libc::c_int as usize] = 0 as libc::c_int as vec_t;
            (*dl).die = (cl.time as libc::c_double + 1.0f64) as libc::c_float;
            S_StartSound(
                0 as *mut vec_t,
                i,
                1 as libc::c_int,
                S_RegisterSound(
                    b"weapons/grenlf1a.wav\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                ),
                1 as libc::c_int as libc::c_float,
                1 as libc::c_int as libc::c_float,
                0 as libc::c_int as libc::c_float,
            );
            CL_LogoutEffect(((*pl).current.origin).as_mut_ptr(), weapon);
        }
        10 => {
            (*dl).color[0 as libc::c_int as usize] = 1 as libc::c_int as vec_t;
            (*dl).color[1 as libc::c_int as usize] = 0 as libc::c_int as vec_t;
            (*dl).color[2 as libc::c_int as usize] = 0 as libc::c_int as vec_t;
            (*dl).die = (cl.time as libc::c_double + 1.0f64) as libc::c_float;
            S_StartSound(
                0 as *mut vec_t,
                i,
                1 as libc::c_int,
                S_RegisterSound(
                    b"weapons/grenlf1a.wav\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                ),
                1 as libc::c_int as libc::c_float,
                1 as libc::c_int as libc::c_float,
                0 as libc::c_int as libc::c_float,
            );
            CL_LogoutEffect(((*pl).current.origin).as_mut_ptr(), weapon);
        }
        11 => {
            (*dl).color[0 as libc::c_int as usize] = 1 as libc::c_int as vec_t;
            (*dl).color[1 as libc::c_int as usize] = 1 as libc::c_int as vec_t;
            (*dl).color[2 as libc::c_int as usize] = 0 as libc::c_int as vec_t;
            (*dl).die = (cl.time as libc::c_double + 1.0f64) as libc::c_float;
            S_StartSound(
                0 as *mut vec_t,
                i,
                1 as libc::c_int,
                S_RegisterSound(
                    b"weapons/grenlf1a.wav\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                ),
                1 as libc::c_int as libc::c_float,
                1 as libc::c_int as libc::c_float,
                0 as libc::c_int as libc::c_float,
            );
            CL_LogoutEffect(((*pl).current.origin).as_mut_ptr(), weapon);
        }
        18 => {
            (*dl).color[0 as libc::c_int as usize] = 1 as libc::c_int as vec_t;
            (*dl).color[1 as libc::c_int as usize] = 0.5f64 as vec_t;
            (*dl).color[2 as libc::c_int as usize] = 0.5f64 as vec_t;
            S_StartSound(
                0 as *mut vec_t,
                i,
                1 as libc::c_int,
                S_RegisterSound(
                    b"weapons/plasshot.wav\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                ),
                volume,
                1 as libc::c_int as libc::c_float,
                0 as libc::c_int as libc::c_float,
            );
        }
        16 => {
            (*dl).color[0 as libc::c_int as usize] = 1 as libc::c_int as vec_t;
            (*dl).color[1 as libc::c_int as usize] = 0.5f64 as vec_t;
            (*dl).color[2 as libc::c_int as usize] = 0.5f64 as vec_t;
            S_StartSound(
                0 as *mut vec_t,
                i,
                1 as libc::c_int,
                S_RegisterSound(
                    b"weapons/rippfire.wav\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                ),
                volume,
                1 as libc::c_int as libc::c_float,
                0 as libc::c_int as libc::c_float,
            );
        }
        30 => {
            (*dl).color[0 as libc::c_int as usize] = 0.9f64 as vec_t;
            (*dl).color[1 as libc::c_int as usize] = 0.7f64 as vec_t;
            (*dl).color[2 as libc::c_int as usize] = 0 as libc::c_int as vec_t;
            S_StartSound(
                0 as *mut vec_t,
                i,
                1 as libc::c_int,
                S_RegisterSound(
                    b"weapons/nail1.wav\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                ),
                volume,
                1 as libc::c_int as libc::c_float,
                0 as libc::c_int as libc::c_float,
            );
        }
        32 => {
            (*dl).color[0 as libc::c_int as usize] = 1 as libc::c_int as vec_t;
            (*dl).color[1 as libc::c_int as usize] = 1 as libc::c_int as vec_t;
            (*dl).color[2 as libc::c_int as usize] = 0 as libc::c_int as vec_t;
            S_StartSound(
                0 as *mut vec_t,
                i,
                1 as libc::c_int,
                S_RegisterSound(
                    b"weapons/shotg2.wav\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                ),
                volume,
                1 as libc::c_int as libc::c_float,
                0 as libc::c_int as libc::c_float,
            );
        }
        33 => {
            (*dl).color[0 as libc::c_int as usize] = 1 as libc::c_int as vec_t;
            (*dl).color[1 as libc::c_int as usize] = 1 as libc::c_int as vec_t;
            (*dl).color[2 as libc::c_int as usize] = 0 as libc::c_int as vec_t;
            (*dl).die = (cl.time + 100 as libc::c_int) as libc::c_float;
        }
        34 => {
            (*dl).color[0 as libc::c_int as usize] = 0 as libc::c_int as vec_t;
            (*dl).color[1 as libc::c_int as usize] = 1 as libc::c_int as vec_t;
            (*dl).color[2 as libc::c_int as usize] = 0 as libc::c_int as vec_t;
            S_StartSound(
                0 as *mut vec_t,
                i,
                1 as libc::c_int,
                S_RegisterSound(
                    b"weapons/blastf1a.wav\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                ),
                volume,
                1 as libc::c_int as libc::c_float,
                0 as libc::c_int as libc::c_float,
            );
        }
        35 => {
            (*dl).color[0 as libc::c_int as usize] = -(1 as libc::c_int) as vec_t;
            (*dl).color[1 as libc::c_int as usize] = -(1 as libc::c_int) as vec_t;
            (*dl).color[2 as libc::c_int as usize] = -(1 as libc::c_int) as vec_t;
            S_StartSound(
                0 as *mut vec_t,
                i,
                1 as libc::c_int,
                S_RegisterSound(
                    b"weapons/disint2.wav\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                ),
                volume,
                1 as libc::c_int as libc::c_float,
                0 as libc::c_int as libc::c_float,
            );
        }
        36 => {
            (*dl).color[0 as libc::c_int as usize] = 1 as libc::c_int as vec_t;
            (*dl).color[1 as libc::c_int as usize] = 0 as libc::c_int as vec_t;
            (*dl).color[2 as libc::c_int as usize] = 0 as libc::c_int as vec_t;
            (*dl).die = (cl.time + 100 as libc::c_int) as libc::c_float;
        }
        37 => {
            (*dl).color[0 as libc::c_int as usize] = 1 as libc::c_int as vec_t;
            (*dl).color[1 as libc::c_int as usize] = 1 as libc::c_int as vec_t;
            (*dl).color[2 as libc::c_int as usize] = 0 as libc::c_int as vec_t;
            (*dl).die = (cl.time + 100 as libc::c_int) as libc::c_float;
        }
        38 => {
            (*dl).color[0 as libc::c_int as usize] = 0 as libc::c_int as vec_t;
            (*dl).color[1 as libc::c_int as usize] = 0 as libc::c_int as vec_t;
            (*dl).color[2 as libc::c_int as usize] = 1 as libc::c_int as vec_t;
            (*dl).die = (cl.time + 100 as libc::c_int) as libc::c_float;
        }
        39 => {
            (*dl).color[0 as libc::c_int as usize] = 0 as libc::c_int as vec_t;
            (*dl).color[1 as libc::c_int as usize] = 1 as libc::c_int as vec_t;
            (*dl).color[2 as libc::c_int as usize] = 1 as libc::c_int as vec_t;
            (*dl).die = (cl.time + 100 as libc::c_int) as libc::c_float;
        }
        _ => {}
    };
}

#[no_mangle]
pub unsafe extern "C" fn CL_ParseMuzzleFlash2() {
    let mut ent: libc::c_int = 0;
    let mut origin: vec3_t = [0.; 3];
    let mut flash_number: libc::c_int = 0;
    let mut dl: *mut cdlight_t = 0 as *mut cdlight_t;
    let mut forward: vec3_t = [0.; 3];
    let mut right: vec3_t = [0.; 3];
    let mut soundname: [libc::c_char; 64] = [0; 64];
    ent = MSG_ReadShort(&mut net_message);
    if ent < 1 as libc::c_int || ent >= 1024 as libc::c_int {
        Com_Error(
            1 as libc::c_int,
            b"CL_ParseMuzzleFlash2: bad entity\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    flash_number = MSG_ReadByte(&mut net_message);
    AngleVectors(
        (cl_entities[ent as usize].current.angles).as_mut_ptr(),
        forward.as_mut_ptr(),
        right.as_mut_ptr(),
        0 as *mut vec_t,
    );
    origin[0 as libc::c_int
        as usize] = cl_entities[ent as usize].current.origin[0 as libc::c_int as usize]
        + forward[0 as libc::c_int as usize]
        * (*monster_flash_offset
        .as_mut_ptr()
        .offset(flash_number as isize))[0 as libc::c_int as usize]
        + right[0 as libc::c_int as usize]
        * (*monster_flash_offset
        .as_mut_ptr()
        .offset(flash_number as isize))[1 as libc::c_int as usize];
    origin[1 as libc::c_int
        as usize] = cl_entities[ent as usize].current.origin[1 as libc::c_int as usize]
        + forward[1 as libc::c_int as usize]
        * (*monster_flash_offset
        .as_mut_ptr()
        .offset(flash_number as isize))[0 as libc::c_int as usize]
        + right[1 as libc::c_int as usize]
        * (*monster_flash_offset
        .as_mut_ptr()
        .offset(flash_number as isize))[1 as libc::c_int as usize];
    origin[2 as libc::c_int
        as usize] = cl_entities[ent as usize].current.origin[2 as libc::c_int as usize]
        + forward[2 as libc::c_int as usize]
        * (*monster_flash_offset
        .as_mut_ptr()
        .offset(flash_number as isize))[0 as libc::c_int as usize]
        + right[2 as libc::c_int as usize]
        * (*monster_flash_offset
        .as_mut_ptr()
        .offset(flash_number as isize))[1 as libc::c_int as usize]
        + (*monster_flash_offset
        .as_mut_ptr()
        .offset(flash_number as isize))[2 as libc::c_int as usize];
    dl = CL_AllocDlight(ent);
    (*dl).origin[0 as libc::c_int as usize] = origin[0 as libc::c_int as usize];
    (*dl).origin[1 as libc::c_int as usize] = origin[1 as libc::c_int as usize];
    (*dl).origin[2 as libc::c_int as usize] = origin[2 as libc::c_int as usize];
    (*dl).radius = (200 as libc::c_int + (rand() & 31 as libc::c_int)) as libc::c_float;
    (*dl).minlight = 32 as libc::c_int as libc::c_float;
    (*dl).die = cl.time as libc::c_float;
    match flash_number {
        26 | 27 | 28 | 29 | 30 | 31 | 32 | 33 | 34 | 35 | 36 | 37 | 38 => {
            (*dl).color[0 as libc::c_int as usize] = 1 as libc::c_int as vec_t;
            (*dl).color[1 as libc::c_int as usize] = 1 as libc::c_int as vec_t;
            (*dl).color[2 as libc::c_int as usize] = 0 as libc::c_int as vec_t;
            CL_ParticleEffect(
                origin.as_mut_ptr(),
                vec3_origin.as_mut_ptr(),
                0 as libc::c_int,
                40 as libc::c_int,
            );
            CL_SmokeAndFlash(origin.as_mut_ptr());
            S_StartSound(
                0 as *mut vec_t,
                ent,
                1 as libc::c_int,
                S_RegisterSound(
                    b"infantry/infatck1.wav\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                ),
                1 as libc::c_int as libc::c_float,
                1 as libc::c_int as libc::c_float,
                0 as libc::c_int as libc::c_float,
            );
        }
        43 | 44 | 85 | 88 | 91 | 94 | 97 | 100 => {
            (*dl).color[0 as libc::c_int as usize] = 1 as libc::c_int as vec_t;
            (*dl).color[1 as libc::c_int as usize] = 1 as libc::c_int as vec_t;
            (*dl).color[2 as libc::c_int as usize] = 0 as libc::c_int as vec_t;
            CL_ParticleEffect(
                origin.as_mut_ptr(),
                vec3_origin.as_mut_ptr(),
                0 as libc::c_int,
                40 as libc::c_int,
            );
            CL_SmokeAndFlash(origin.as_mut_ptr());
            S_StartSound(
                0 as *mut vec_t,
                ent,
                1 as libc::c_int,
                S_RegisterSound(
                    b"soldier/solatck3.wav\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                ),
                1 as libc::c_int as libc::c_float,
                1 as libc::c_int as libc::c_float,
                0 as libc::c_int as libc::c_float,
            );
        }
        45 | 46 | 47 | 48 | 49 | 50 | 51 | 52 => {
            (*dl).color[0 as libc::c_int as usize] = 1 as libc::c_int as vec_t;
            (*dl).color[1 as libc::c_int as usize] = 1 as libc::c_int as vec_t;
            (*dl).color[2 as libc::c_int as usize] = 0 as libc::c_int as vec_t;
            CL_ParticleEffect(
                origin.as_mut_ptr(),
                vec3_origin.as_mut_ptr(),
                0 as libc::c_int,
                40 as libc::c_int,
            );
            CL_SmokeAndFlash(origin.as_mut_ptr());
            S_StartSound(
                0 as *mut vec_t,
                ent,
                1 as libc::c_int,
                S_RegisterSound(
                    b"gunner/gunatck2.wav\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                ),
                1 as libc::c_int as libc::c_float,
                1 as libc::c_int as libc::c_float,
                0 as libc::c_int as libc::c_float,
            );
        }
        63 | 64 | 65 | 66 | 67 | 68 | 69 | 141 => {
            (*dl).color[0 as libc::c_int as usize] = 1 as libc::c_int as vec_t;
            (*dl).color[1 as libc::c_int as usize] = 1 as libc::c_int as vec_t;
            (*dl).color[2 as libc::c_int as usize] = 0 as libc::c_int as vec_t;
            CL_ParticleEffect(
                origin.as_mut_ptr(),
                vec3_origin.as_mut_ptr(),
                0 as libc::c_int,
                40 as libc::c_int,
            );
            CL_SmokeAndFlash(origin.as_mut_ptr());
            S_StartSound(
                0 as *mut vec_t,
                ent,
                1 as libc::c_int,
                S_RegisterSound(
                    b"infantry/infatck1.wav\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                ),
                1 as libc::c_int as libc::c_float,
                1 as libc::c_int as libc::c_float,
                0 as libc::c_int as libc::c_float,
            );
        }
        73 | 74 | 75 | 76 | 77 | 138 | 152 => {
            (*dl).color[0 as libc::c_int as usize] = 1 as libc::c_int as vec_t;
            (*dl).color[1 as libc::c_int as usize] = 1 as libc::c_int as vec_t;
            (*dl).color[2 as libc::c_int as usize] = 0 as libc::c_int as vec_t;
            CL_ParticleEffect(
                origin.as_mut_ptr(),
                vec3_origin.as_mut_ptr(),
                0 as libc::c_int,
                40 as libc::c_int,
            );
            CL_SmokeAndFlash(origin.as_mut_ptr());
            S_StartSound(
                0 as *mut vec_t,
                ent,
                1 as libc::c_int,
                S_RegisterSound(
                    b"infantry/infatck1.wav\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                ),
                1 as libc::c_int as libc::c_float,
                0 as libc::c_int as libc::c_float,
                0 as libc::c_int as libc::c_float,
            );
        }
        39 | 40 | 83 | 86 | 89 | 92 | 95 | 98 | 143 => {
            (*dl).color[0 as libc::c_int as usize] = 1 as libc::c_int as vec_t;
            (*dl).color[1 as libc::c_int as usize] = 1 as libc::c_int as vec_t;
            (*dl).color[2 as libc::c_int as usize] = 0 as libc::c_int as vec_t;
            S_StartSound(
                0 as *mut vec_t,
                ent,
                1 as libc::c_int,
                S_RegisterSound(
                    b"soldier/solatck2.wav\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                ),
                1 as libc::c_int as libc::c_float,
                1 as libc::c_int as libc::c_float,
                0 as libc::c_int as libc::c_float,
            );
        }
        58 | 59 => {
            (*dl).color[0 as libc::c_int as usize] = 1 as libc::c_int as vec_t;
            (*dl).color[1 as libc::c_int as usize] = 1 as libc::c_int as vec_t;
            (*dl).color[2 as libc::c_int as usize] = 0 as libc::c_int as vec_t;
            S_StartSound(
                0 as *mut vec_t,
                ent,
                1 as libc::c_int,
                S_RegisterSound(
                    b"flyer/flyatck3.wav\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                ),
                1 as libc::c_int as libc::c_float,
                1 as libc::c_int as libc::c_float,
                0 as libc::c_int as libc::c_float,
            );
        }
        60 => {
            (*dl).color[0 as libc::c_int as usize] = 1 as libc::c_int as vec_t;
            (*dl).color[1 as libc::c_int as usize] = 1 as libc::c_int as vec_t;
            (*dl).color[2 as libc::c_int as usize] = 0 as libc::c_int as vec_t;
            S_StartSound(
                0 as *mut vec_t,
                ent,
                1 as libc::c_int,
                S_RegisterSound(
                    b"medic/medatck1.wav\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                ),
                1 as libc::c_int as libc::c_float,
                1 as libc::c_int as libc::c_float,
                0 as libc::c_int as libc::c_float,
            );
        }
        62 => {
            (*dl).color[0 as libc::c_int as usize] = 1 as libc::c_int as vec_t;
            (*dl).color[1 as libc::c_int as usize] = 1 as libc::c_int as vec_t;
            (*dl).color[2 as libc::c_int as usize] = 0 as libc::c_int as vec_t;
            S_StartSound(
                0 as *mut vec_t,
                ent,
                1 as libc::c_int,
                S_RegisterSound(
                    b"hover/hovatck1.wav\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                ),
                1 as libc::c_int as libc::c_float,
                1 as libc::c_int as libc::c_float,
                0 as libc::c_int as libc::c_float,
            );
        }
        82 => {
            (*dl).color[0 as libc::c_int as usize] = 1 as libc::c_int as vec_t;
            (*dl).color[1 as libc::c_int as usize] = 1 as libc::c_int as vec_t;
            (*dl).color[2 as libc::c_int as usize] = 0 as libc::c_int as vec_t;
            S_StartSound(
                0 as *mut vec_t,
                ent,
                1 as libc::c_int,
                S_RegisterSound(
                    b"floater/fltatck1.wav\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                ),
                1 as libc::c_int as libc::c_float,
                1 as libc::c_int as libc::c_float,
                0 as libc::c_int as libc::c_float,
            );
        }
        41 | 42 | 84 | 87 | 90 | 93 | 96 | 99 => {
            (*dl).color[0 as libc::c_int as usize] = 1 as libc::c_int as vec_t;
            (*dl).color[1 as libc::c_int as usize] = 1 as libc::c_int as vec_t;
            (*dl).color[2 as libc::c_int as usize] = 0 as libc::c_int as vec_t;
            CL_SmokeAndFlash(origin.as_mut_ptr());
            S_StartSound(
                0 as *mut vec_t,
                ent,
                1 as libc::c_int,
                S_RegisterSound(
                    b"soldier/solatck1.wav\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                ),
                1 as libc::c_int as libc::c_float,
                1 as libc::c_int as libc::c_float,
                0 as libc::c_int as libc::c_float,
            );
        }
        1 | 2 | 3 => {
            (*dl).color[0 as libc::c_int as usize] = 1 as libc::c_int as vec_t;
            (*dl).color[1 as libc::c_int as usize] = 1 as libc::c_int as vec_t;
            (*dl).color[2 as libc::c_int as usize] = 0 as libc::c_int as vec_t;
            S_StartSound(
                0 as *mut vec_t,
                ent,
                1 as libc::c_int,
                S_RegisterSound(
                    b"tank/tnkatck3.wav\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                ),
                1 as libc::c_int as libc::c_float,
                1 as libc::c_int as libc::c_float,
                0 as libc::c_int as libc::c_float,
            );
        }
        4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 | 21
        | 22 => {
            (*dl).color[0 as libc::c_int as usize] = 1 as libc::c_int as vec_t;
            (*dl).color[1 as libc::c_int as usize] = 1 as libc::c_int as vec_t;
            (*dl).color[2 as libc::c_int as usize] = 0 as libc::c_int as vec_t;
            CL_ParticleEffect(
                origin.as_mut_ptr(),
                vec3_origin.as_mut_ptr(),
                0 as libc::c_int,
                40 as libc::c_int,
            );
            CL_SmokeAndFlash(origin.as_mut_ptr());
            Com_sprintf(
                soundname.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong
                    as libc::c_int,
                b"tank/tnkatk2%c.wav\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                'a' as i32 + rand() % 5 as libc::c_int,
            );
            S_StartSound(
                0 as *mut vec_t,
                ent,
                1 as libc::c_int,
                S_RegisterSound(soundname.as_mut_ptr()),
                1 as libc::c_int as libc::c_float,
                1 as libc::c_int as libc::c_float,
                0 as libc::c_int as libc::c_float,
            );
        }
        57 | 142 => {
            (*dl).color[0 as libc::c_int as usize] = 1 as libc::c_int as vec_t;
            (*dl).color[1 as libc::c_int as usize] = 0.5f64 as vec_t;
            (*dl).color[2 as libc::c_int as usize] = 0.2f64 as vec_t;
            S_StartSound(
                0 as *mut vec_t,
                ent,
                1 as libc::c_int,
                S_RegisterSound(
                    b"chick/chkatck2.wav\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                ),
                1 as libc::c_int as libc::c_float,
                1 as libc::c_int as libc::c_float,
                0 as libc::c_int as libc::c_float,
            );
        }
        23 | 24 | 25 => {
            (*dl).color[0 as libc::c_int as usize] = 1 as libc::c_int as vec_t;
            (*dl).color[1 as libc::c_int as usize] = 0.5f64 as vec_t;
            (*dl).color[2 as libc::c_int as usize] = 0.2f64 as vec_t;
            S_StartSound(
                0 as *mut vec_t,
                ent,
                1 as libc::c_int,
                S_RegisterSound(
                    b"tank/tnkatck1.wav\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                ),
                1 as libc::c_int as libc::c_float,
                1 as libc::c_int as libc::c_float,
                0 as libc::c_int as libc::c_float,
            );
        }
        70 | 71 | 72 | 78 | 79 | 80 | 81 | 191 => {
            (*dl).color[0 as libc::c_int as usize] = 1 as libc::c_int as vec_t;
            (*dl).color[1 as libc::c_int as usize] = 0.5f64 as vec_t;
            (*dl).color[2 as libc::c_int as usize] = 0.2f64 as vec_t;
            S_StartSound(
                0 as *mut vec_t,
                ent,
                1 as libc::c_int,
                S_RegisterSound(
                    b"tank/rocket.wav\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                ),
                1 as libc::c_int as libc::c_float,
                1 as libc::c_int as libc::c_float,
                0 as libc::c_int as libc::c_float,
            );
        }
        53 | 54 | 55 | 56 => {
            (*dl).color[0 as libc::c_int as usize] = 1 as libc::c_int as vec_t;
            (*dl).color[1 as libc::c_int as usize] = 0.5f64 as vec_t;
            (*dl).color[2 as libc::c_int as usize] = 0 as libc::c_int as vec_t;
            S_StartSound(
                0 as *mut vec_t,
                ent,
                1 as libc::c_int,
                S_RegisterSound(
                    b"gunner/gunatck3.wav\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                ),
                1 as libc::c_int as libc::c_float,
                1 as libc::c_int as libc::c_float,
                0 as libc::c_int as libc::c_float,
            );
        }
        61 | 147 | 150 => {
            (*dl).color[0 as libc::c_int as usize] = 0.5f64 as vec_t;
            (*dl).color[1 as libc::c_int as usize] = 0.5f64 as vec_t;
            (*dl).color[2 as libc::c_int as usize] = 1.0f64 as vec_t;
        }
        101 => {
            (*dl).color[0 as libc::c_int as usize] = 0.5f64 as vec_t;
            (*dl).color[1 as libc::c_int as usize] = 1 as libc::c_int as vec_t;
            (*dl).color[2 as libc::c_int as usize] = 0.5f64 as vec_t;
        }
        102 | 103 | 104 | 105 | 106 | 107 | 108 | 109 | 110 | 111 | 112 | 113 | 114 | 115
        | 116 | 117 | 118 => {
            (*dl).color[0 as libc::c_int as usize] = 1 as libc::c_int as vec_t;
            (*dl).color[1 as libc::c_int as usize] = 1 as libc::c_int as vec_t;
            (*dl).color[2 as libc::c_int as usize] = 0 as libc::c_int as vec_t;
            S_StartSound(
                0 as *mut vec_t,
                ent,
                1 as libc::c_int,
                S_RegisterSound(
                    b"makron/blaster.wav\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                ),
                1 as libc::c_int as libc::c_float,
                1 as libc::c_int as libc::c_float,
                0 as libc::c_int as libc::c_float,
            );
        }
        120 | 121 | 122 | 123 | 124 | 125 => {
            (*dl).color[0 as libc::c_int as usize] = 1 as libc::c_int as vec_t;
            (*dl).color[1 as libc::c_int as usize] = 1 as libc::c_int as vec_t;
            (*dl).color[2 as libc::c_int as usize] = 0 as libc::c_int as vec_t;
            CL_ParticleEffect(
                origin.as_mut_ptr(),
                vec3_origin.as_mut_ptr(),
                0 as libc::c_int,
                40 as libc::c_int,
            );
            CL_SmokeAndFlash(origin.as_mut_ptr());
            S_StartSound(
                0 as *mut vec_t,
                ent,
                1 as libc::c_int,
                S_RegisterSound(
                    b"boss3/xfire.wav\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                ),
                1 as libc::c_int as libc::c_float,
                1 as libc::c_int as libc::c_float,
                0 as libc::c_int as libc::c_float,
            );
        }
        126 | 127 | 128 | 129 | 130 | 131 => {
            (*dl).color[0 as libc::c_int as usize] = 1 as libc::c_int as vec_t;
            (*dl).color[1 as libc::c_int as usize] = 1 as libc::c_int as vec_t;
            (*dl).color[2 as libc::c_int as usize] = 0 as libc::c_int as vec_t;
            CL_ParticleEffect(
                origin.as_mut_ptr(),
                vec3_origin.as_mut_ptr(),
                0 as libc::c_int,
                40 as libc::c_int,
            );
            CL_SmokeAndFlash(origin.as_mut_ptr());
        }
        132 => {
            (*dl).color[0 as libc::c_int as usize] = 0.5f64 as vec_t;
            (*dl).color[1 as libc::c_int as usize] = 1 as libc::c_int as vec_t;
            (*dl).color[2 as libc::c_int as usize] = 0.5f64 as vec_t;
        }
        133 | 134 | 135 | 136 | 137 | 139 | 153 => {
            (*dl).color[0 as libc::c_int as usize] = 1 as libc::c_int as vec_t;
            (*dl).color[1 as libc::c_int as usize] = 1 as libc::c_int as vec_t;
            (*dl).color[2 as libc::c_int as usize] = 0 as libc::c_int as vec_t;
            CL_ParticleEffect(
                origin.as_mut_ptr(),
                vec3_origin.as_mut_ptr(),
                0 as libc::c_int,
                40 as libc::c_int,
            );
            CL_SmokeAndFlash(origin.as_mut_ptr());
        }
        144 | 145 | 146 | 149 | 156 | 157 | 158 | 159 | 160 | 161 | 162 | 163 | 164 | 165
        | 166 | 167 | 168 | 169 | 170 | 171 | 172 | 173 | 174 | 175 | 176 | 177 | 178
        | 179 | 180 | 181 | 182 | 183 | 184 | 185 | 186 | 187 | 188 | 189 | 190 => {
            (*dl).color[0 as libc::c_int as usize] = 0 as libc::c_int as vec_t;
            (*dl).color[1 as libc::c_int as usize] = 1 as libc::c_int as vec_t;
            (*dl).color[2 as libc::c_int as usize] = 0 as libc::c_int as vec_t;
            S_StartSound(
                0 as *mut vec_t,
                ent,
                1 as libc::c_int,
                S_RegisterSound(
                    b"tank/tnkatck3.wav\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                ),
                1 as libc::c_int as libc::c_float,
                1 as libc::c_int as libc::c_float,
                0 as libc::c_int as libc::c_float,
            );
        }
        148 => {
            (*dl).color[0 as libc::c_int as usize] = -(1 as libc::c_int) as vec_t;
            (*dl).color[1 as libc::c_int as usize] = -(1 as libc::c_int) as vec_t;
            (*dl).color[2 as libc::c_int as usize] = -(1 as libc::c_int) as vec_t;
            S_StartSound(
                0 as *mut vec_t,
                ent,
                1 as libc::c_int,
                S_RegisterSound(
                    b"weapons/disint2.wav\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                ),
                1 as libc::c_int as libc::c_float,
                1 as libc::c_int as libc::c_float,
                0 as libc::c_int as libc::c_float,
            );
        }
        151 | 195 | 196 | 197 | 198 | 199 | 200 | 201 | 202 | 203 | 204 | 205 | 206 | 207
        | 208 | 209 | 210 => {
            (*dl)
                .radius = (300 as libc::c_int + (rand() & 100 as libc::c_int))
                as libc::c_float;
            (*dl).color[0 as libc::c_int as usize] = 1 as libc::c_int as vec_t;
            (*dl).color[1 as libc::c_int as usize] = 1 as libc::c_int as vec_t;
            (*dl).color[2 as libc::c_int as usize] = 0 as libc::c_int as vec_t;
            (*dl).die = (cl.time + 200 as libc::c_int) as libc::c_float;
        }
        _ => {}
    };
}

#[no_mangle]
pub unsafe extern "C" fn CL_AddDLights() {
    let mut i: libc::c_int = 0;
    let mut dl: *mut cdlight_t = 0 as *mut cdlight_t;
    dl = cl_dlights.as_mut_ptr();
    if vidref_val == 1 as libc::c_int {
        i = 0 as libc::c_int;
        while i < 32 as libc::c_int {
            if !((*dl).radius == 0.) {
                V_AddLight(
                    ((*dl).origin).as_mut_ptr(),
                    (*dl).radius,
                    (*dl).color[0 as libc::c_int as usize],
                    (*dl).color[1 as libc::c_int as usize],
                    (*dl).color[2 as libc::c_int as usize],
                );
            }
            i += 1;
            dl = dl.offset(1);
        }
    } else {
        i = 0 as libc::c_int;
        while i < 32 as libc::c_int {
            if !((*dl).radius == 0.) {
                if (*dl).color[0 as libc::c_int as usize]
                    < 0 as libc::c_int as libc::c_float
                    || (*dl).color[1 as libc::c_int as usize]
                    < 0 as libc::c_int as libc::c_float
                    || (*dl).color[2 as libc::c_int as usize]
                    < 0 as libc::c_int as libc::c_float
                {
                    (*dl).radius = -(*dl).radius;
                    (*dl).color[0 as libc::c_int as usize] = 1 as libc::c_int as vec_t;
                    (*dl).color[1 as libc::c_int as usize] = 1 as libc::c_int as vec_t;
                    (*dl).color[2 as libc::c_int as usize] = 1 as libc::c_int as vec_t;
                }
                V_AddLight(
                    ((*dl).origin).as_mut_ptr(),
                    (*dl).radius,
                    (*dl).color[0 as libc::c_int as usize],
                    (*dl).color[1 as libc::c_int as usize],
                    (*dl).color[2 as libc::c_int as usize],
                );
            }
            i += 1;
            dl = dl.offset(1);
        }
    };
}

#[no_mangle]
pub static mut active_particles: *mut cparticle_t = 0 as *const cparticle_t
    as *mut cparticle_t;
#[no_mangle]
pub static mut free_particles: *mut cparticle_t = 0 as *const cparticle_t
    as *mut cparticle_t;
#[no_mangle]
pub static mut particles: [cparticle_t; 4096] = [cparticle_t {
    next: 0 as *const particle_s as *mut particle_s,
    time: 0.,
    org: [0.; 3],
    vel: [0.; 3],
    accel: [0.; 3],
    color: 0.,
    colorvel: 0.,
    alpha: 0.,
    alphavel: 0.,
}; 4096];
#[no_mangle]
pub static mut cl_numparticles: libc::c_int = 4096 as libc::c_int;

#[no_mangle]
pub unsafe extern "C" fn CL_ClearParticles() {
    let mut i: libc::c_int = 0;
    free_particles = &mut *particles.as_mut_ptr().offset(0 as libc::c_int as isize)
        as *mut cparticle_t;
    active_particles = 0 as *mut cparticle_t;
    i = 0 as libc::c_int;
    while i < cl_numparticles {
        particles[i as usize]
            .next = &mut *particles.as_mut_ptr().offset((i + 1 as libc::c_int) as isize)
            as *mut cparticle_t;
        i += 1;
    }
    particles[(cl_numparticles - 1 as libc::c_int) as usize].next = 0 as *mut particle_s;
}

#[no_mangle]
pub unsafe extern "C" fn CL_ParticleEffect(
    mut org: *mut vec_t,
    mut dir: *mut vec_t,
    mut color: libc::c_int,
    mut count: libc::c_int,
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
        let ref mut fresh6 = (*p).next;
        *fresh6 = active_particles;
        active_particles = p;
        (*p).time = cl.time as libc::c_float;
        (*p).color = (color + (rand() & 7 as libc::c_int)) as libc::c_float;
        d = (rand() & 31 as libc::c_int) as libc::c_float;
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
        let ref mut fresh7 = (*p).accel[1 as libc::c_int as usize];
        *fresh7 = 0 as libc::c_int as vec_t;
        (*p).accel[0 as libc::c_int as usize] = *fresh7;
        (*p).accel[2 as libc::c_int as usize] = -(40 as libc::c_int) as vec_t;
        (*p).alpha = 1.0f64 as libc::c_float;
        (*p)
            .alphavel = (-1.0f64 / (0.5f64 + frand() as libc::c_double * 0.3f64))
            as libc::c_float;
        i += 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn CL_ParticleEffect2(
    mut org: *mut vec_t,
    mut dir: *mut vec_t,
    mut color: libc::c_int,
    mut count: libc::c_int,
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
        let ref mut fresh8 = (*p).next;
        *fresh8 = active_particles;
        active_particles = p;
        (*p).time = cl.time as libc::c_float;
        (*p).color = color as libc::c_float;
        d = (rand() & 7 as libc::c_int) as libc::c_float;
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
        let ref mut fresh9 = (*p).accel[1 as libc::c_int as usize];
        *fresh9 = 0 as libc::c_int as vec_t;
        (*p).accel[0 as libc::c_int as usize] = *fresh9;
        (*p).accel[2 as libc::c_int as usize] = -(40 as libc::c_int) as vec_t;
        (*p).alpha = 1.0f64 as libc::c_float;
        (*p)
            .alphavel = (-1.0f64 / (0.5f64 + frand() as libc::c_double * 0.3f64))
            as libc::c_float;
        i += 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn CL_ParticleEffect3(
    mut org: *mut vec_t,
    mut dir: *mut vec_t,
    mut color: libc::c_int,
    mut count: libc::c_int,
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
        let ref mut fresh10 = (*p).next;
        *fresh10 = active_particles;
        active_particles = p;
        (*p).time = cl.time as libc::c_float;
        (*p).color = color as libc::c_float;
        d = (rand() & 7 as libc::c_int) as libc::c_float;
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
        let ref mut fresh11 = (*p).accel[1 as libc::c_int as usize];
        *fresh11 = 0 as libc::c_int as vec_t;
        (*p).accel[0 as libc::c_int as usize] = *fresh11;
        (*p).accel[2 as libc::c_int as usize] = 40 as libc::c_int as vec_t;
        (*p).alpha = 1.0f64 as libc::c_float;
        (*p)
            .alphavel = (-1.0f64 / (0.5f64 + frand() as libc::c_double * 0.3f64))
            as libc::c_float;
        i += 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn CL_TeleporterParticles(mut ent: *mut entity_state_t) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut p: *mut cparticle_t = 0 as *mut cparticle_t;
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        if free_particles.is_null() {
            return;
        }
        p = free_particles;
        free_particles = (*p).next;
        let ref mut fresh12 = (*p).next;
        *fresh12 = active_particles;
        active_particles = p;
        (*p).time = cl.time as libc::c_float;
        (*p).color = 0xdb as libc::c_int as libc::c_float;
        j = 0 as libc::c_int;
        while j < 2 as libc::c_int {
            (*p)
                .org[j
                as usize] = (*ent).origin[j as usize]
                - 16 as libc::c_int as libc::c_float
                + (rand() & 31 as libc::c_int) as libc::c_float;
            (*p).vel[j as usize] = crand() * 14 as libc::c_int as libc::c_float;
            j += 1;
        }
        (*p)
            .org[2 as libc::c_int
            as usize] = (*ent).origin[2 as libc::c_int as usize]
            - 8 as libc::c_int as libc::c_float
            + (rand() & 7 as libc::c_int) as libc::c_float;
        (*p)
            .vel[2 as libc::c_int
            as usize] = (80 as libc::c_int + (rand() & 7 as libc::c_int)) as vec_t;
        let ref mut fresh13 = (*p).accel[1 as libc::c_int as usize];
        *fresh13 = 0 as libc::c_int as vec_t;
        (*p).accel[0 as libc::c_int as usize] = *fresh13;
        (*p).accel[2 as libc::c_int as usize] = -(40 as libc::c_int) as vec_t;
        (*p).alpha = 1.0f64 as libc::c_float;
        (*p).alphavel = -0.5f64 as libc::c_float;
        i += 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn CL_LogoutEffect(mut org: *mut vec_t, mut type_0: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut p: *mut cparticle_t = 0 as *mut cparticle_t;
    i = 0 as libc::c_int;
    while i < 500 as libc::c_int {
        if free_particles.is_null() {
            return;
        }
        p = free_particles;
        free_particles = (*p).next;
        let ref mut fresh14 = (*p).next;
        *fresh14 = active_particles;
        active_particles = p;
        (*p).time = cl.time as libc::c_float;
        if type_0 == 9 as libc::c_int {
            (*p)
                .color = (0xd0 as libc::c_int + (rand() & 7 as libc::c_int))
                as libc::c_float;
        } else if type_0 == 10 as libc::c_int {
            (*p)
                .color = (0x40 as libc::c_int + (rand() & 7 as libc::c_int))
                as libc::c_float;
        } else {
            (*p)
                .color = (0xe0 as libc::c_int + (rand() & 7 as libc::c_int))
                as libc::c_float;
        }
        (*p)
            .org[0 as libc::c_int
            as usize] = *org.offset(0 as libc::c_int as isize)
            - 16 as libc::c_int as libc::c_float
            + frand() * 32 as libc::c_int as libc::c_float;
        (*p)
            .org[1 as libc::c_int
            as usize] = *org.offset(1 as libc::c_int as isize)
            - 16 as libc::c_int as libc::c_float
            + frand() * 32 as libc::c_int as libc::c_float;
        (*p)
            .org[2 as libc::c_int
            as usize] = *org.offset(2 as libc::c_int as isize)
            - 24 as libc::c_int as libc::c_float
            + frand() * 56 as libc::c_int as libc::c_float;
        j = 0 as libc::c_int;
        while j < 3 as libc::c_int {
            (*p).vel[j as usize] = crand() * 20 as libc::c_int as libc::c_float;
            j += 1;
        }
        let ref mut fresh15 = (*p).accel[1 as libc::c_int as usize];
        *fresh15 = 0 as libc::c_int as vec_t;
        (*p).accel[0 as libc::c_int as usize] = *fresh15;
        (*p).accel[2 as libc::c_int as usize] = -(40 as libc::c_int) as vec_t;
        (*p).alpha = 1.0f64 as libc::c_float;
        (*p)
            .alphavel = (-1.0f64 / (1.0f64 + frand() as libc::c_double * 0.3f64))
            as libc::c_float;
        i += 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn CL_ItemRespawnParticles(mut org: *mut vec_t) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut p: *mut cparticle_t = 0 as *mut cparticle_t;
    i = 0 as libc::c_int;
    while i < 64 as libc::c_int {
        if free_particles.is_null() {
            return;
        }
        p = free_particles;
        free_particles = (*p).next;
        let ref mut fresh16 = (*p).next;
        *fresh16 = active_particles;
        active_particles = p;
        (*p).time = cl.time as libc::c_float;
        (*p)
            .color = (0xd4 as libc::c_int + (rand() & 3 as libc::c_int))
            as libc::c_float;
        (*p)
            .org[0 as libc::c_int
            as usize] = *org.offset(0 as libc::c_int as isize)
            + crand() * 8 as libc::c_int as libc::c_float;
        (*p)
            .org[1 as libc::c_int
            as usize] = *org.offset(1 as libc::c_int as isize)
            + crand() * 8 as libc::c_int as libc::c_float;
        (*p)
            .org[2 as libc::c_int
            as usize] = *org.offset(2 as libc::c_int as isize)
            + crand() * 8 as libc::c_int as libc::c_float;
        j = 0 as libc::c_int;
        while j < 3 as libc::c_int {
            (*p).vel[j as usize] = crand() * 8 as libc::c_int as libc::c_float;
            j += 1;
        }
        let ref mut fresh17 = (*p).accel[1 as libc::c_int as usize];
        *fresh17 = 0 as libc::c_int as vec_t;
        (*p).accel[0 as libc::c_int as usize] = *fresh17;
        (*p)
            .accel[2 as libc::c_int
            as usize] = (-(40 as libc::c_int) as libc::c_double * 0.2f64) as vec_t;
        (*p).alpha = 1.0f64 as libc::c_float;
        (*p)
            .alphavel = (-1.0f64 / (1.0f64 + frand() as libc::c_double * 0.3f64))
            as libc::c_float;
        i += 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn CL_ExplosionParticles(mut org: *mut vec_t) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut p: *mut cparticle_t = 0 as *mut cparticle_t;
    i = 0 as libc::c_int;
    while i < 256 as libc::c_int {
        if free_particles.is_null() {
            return;
        }
        p = free_particles;
        free_particles = (*p).next;
        let ref mut fresh18 = (*p).next;
        *fresh18 = active_particles;
        active_particles = p;
        (*p).time = cl.time as libc::c_float;
        (*p)
            .color = (0xe0 as libc::c_int + (rand() & 7 as libc::c_int))
            as libc::c_float;
        j = 0 as libc::c_int;
        while j < 3 as libc::c_int {
            (*p)
                .org[j
                as usize] = *org.offset(j as isize)
                + (rand() % 32 as libc::c_int - 16 as libc::c_int) as libc::c_float;
            (*p)
                .vel[j
                as usize] = (rand() % 384 as libc::c_int - 192 as libc::c_int) as vec_t;
            j += 1;
        }
        let ref mut fresh19 = (*p).accel[1 as libc::c_int as usize];
        *fresh19 = 0 as libc::c_int as vec_t;
        (*p).accel[0 as libc::c_int as usize] = *fresh19;
        (*p).accel[2 as libc::c_int as usize] = -(40 as libc::c_int) as vec_t;
        (*p).alpha = 1.0f64 as libc::c_float;
        (*p)
            .alphavel = (-0.8f64 / (0.5f64 + frand() as libc::c_double * 0.3f64))
            as libc::c_float;
        i += 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn CL_BigTeleportParticles(mut org: *mut vec_t) {
    let mut i: libc::c_int = 0;
    let mut p: *mut cparticle_t = 0 as *mut cparticle_t;
    let mut angle: libc::c_float = 0.;
    let mut dist: libc::c_float = 0.;
    static mut colortable: [libc::c_int; 4] = [
        2 as libc::c_int * 8 as libc::c_int,
        13 as libc::c_int * 8 as libc::c_int,
        21 as libc::c_int * 8 as libc::c_int,
        18 as libc::c_int * 8 as libc::c_int,
    ];
    i = 0 as libc::c_int;
    while i < 4096 as libc::c_int {
        if free_particles.is_null() {
            return;
        }
        p = free_particles;
        free_particles = (*p).next;
        let ref mut fresh20 = (*p).next;
        *fresh20 = active_particles;
        active_particles = p;
        (*p).time = cl.time as libc::c_float;
        (*p).color = colortable[(rand() & 3 as libc::c_int) as usize] as libc::c_float;
        angle = (3.14159265358979323846f64 * 2 as libc::c_int as libc::c_double
            * (rand() & 1023 as libc::c_int) as libc::c_double / 1023.0f64)
            as libc::c_float;
        dist = (rand() & 31 as libc::c_int) as libc::c_float;
        (*p)
            .org[0 as libc::c_int
            as usize] = (*org.offset(0 as libc::c_int as isize) as libc::c_double
            + cos(angle as libc::c_double) * dist as libc::c_double) as vec_t;
        (*p)
            .vel[0 as libc::c_int
            as usize] = (cos(angle as libc::c_double)
            * (70 as libc::c_int + (rand() & 63 as libc::c_int)) as libc::c_double)
            as vec_t;
        (*p)
            .accel[0 as libc::c_int
            as usize] = (-cos(angle as libc::c_double)
            * 100 as libc::c_int as libc::c_double) as vec_t;
        (*p)
            .org[1 as libc::c_int
            as usize] = (*org.offset(1 as libc::c_int as isize) as libc::c_double
            + sin(angle as libc::c_double) * dist as libc::c_double) as vec_t;
        (*p)
            .vel[1 as libc::c_int
            as usize] = (sin(angle as libc::c_double)
            * (70 as libc::c_int + (rand() & 63 as libc::c_int)) as libc::c_double)
            as vec_t;
        (*p)
            .accel[1 as libc::c_int
            as usize] = (-sin(angle as libc::c_double)
            * 100 as libc::c_int as libc::c_double) as vec_t;
        (*p)
            .org[2 as libc::c_int
            as usize] = *org.offset(2 as libc::c_int as isize)
            + 8 as libc::c_int as libc::c_float
            + (rand() % 90 as libc::c_int) as libc::c_float;
        (*p)
            .vel[2 as libc::c_int
            as usize] = (-(100 as libc::c_int) + (rand() & 31 as libc::c_int)) as vec_t;
        (*p)
            .accel[2 as libc::c_int
            as usize] = (40 as libc::c_int * 4 as libc::c_int) as vec_t;
        (*p).alpha = 1.0f64 as libc::c_float;
        (*p)
            .alphavel = (-0.3f64 / (0.5f64 + frand() as libc::c_double * 0.3f64))
            as libc::c_float;
        i += 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn CL_BlasterParticles(mut org: *mut vec_t, mut dir: *mut vec_t) {
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
        let ref mut fresh21 = (*p).next;
        *fresh21 = active_particles;
        active_particles = p;
        (*p).time = cl.time as libc::c_float;
        (*p)
            .color = (0xe0 as libc::c_int + (rand() & 7 as libc::c_int))
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
        let ref mut fresh22 = (*p).accel[1 as libc::c_int as usize];
        *fresh22 = 0 as libc::c_int as vec_t;
        (*p).accel[0 as libc::c_int as usize] = *fresh22;
        (*p).accel[2 as libc::c_int as usize] = -(40 as libc::c_int) as vec_t;
        (*p).alpha = 1.0f64 as libc::c_float;
        (*p)
            .alphavel = (-1.0f64 / (0.5f64 + frand() as libc::c_double * 0.3f64))
            as libc::c_float;
        i += 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn CL_BlasterTrail(mut start: *mut vec_t, mut end: *mut vec_t) {
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
        let ref mut fresh23 = (*p).next;
        *fresh23 = active_particles;
        active_particles = p;
        let ref mut fresh24 = (*p).accel[2 as libc::c_int as usize];
        *fresh24 = 0 as libc::c_int as vec_t;
        let ref mut fresh25 = (*p).accel[1 as libc::c_int as usize];
        *fresh25 = *fresh24;
        (*p).accel[0 as libc::c_int as usize] = *fresh25;
        (*p).time = cl.time as libc::c_float;
        (*p).alpha = 1.0f64 as libc::c_float;
        (*p)
            .alphavel = (-1.0f64 / (0.3f64 + frand() as libc::c_double * 0.2f64))
            as libc::c_float;
        (*p).color = 0xe0 as libc::c_int as libc::c_float;
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

#[no_mangle]
pub unsafe extern "C" fn CL_QuadTrail(mut start: *mut vec_t, mut end: *mut vec_t) {
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
        let ref mut fresh26 = (*p).next;
        *fresh26 = active_particles;
        active_particles = p;
        let ref mut fresh27 = (*p).accel[2 as libc::c_int as usize];
        *fresh27 = 0 as libc::c_int as vec_t;
        let ref mut fresh28 = (*p).accel[1 as libc::c_int as usize];
        *fresh28 = *fresh27;
        (*p).accel[0 as libc::c_int as usize] = *fresh28;
        (*p).time = cl.time as libc::c_float;
        (*p).alpha = 1.0f64 as libc::c_float;
        (*p)
            .alphavel = (-1.0f64 / (0.8f64 + frand() as libc::c_double * 0.2f64))
            as libc::c_float;
        (*p).color = 115 as libc::c_int as libc::c_float;
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
pub unsafe extern "C" fn CL_FlagTrail(
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
    while len > 0 as libc::c_int as libc::c_float {
        len -= dec as libc::c_float;
        if free_particles.is_null() {
            return;
        }
        p = free_particles;
        free_particles = (*p).next;
        let ref mut fresh29 = (*p).next;
        *fresh29 = active_particles;
        active_particles = p;
        let ref mut fresh30 = (*p).accel[2 as libc::c_int as usize];
        *fresh30 = 0 as libc::c_int as vec_t;
        let ref mut fresh31 = (*p).accel[1 as libc::c_int as usize];
        *fresh31 = *fresh30;
        (*p).accel[0 as libc::c_int as usize] = *fresh31;
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
pub unsafe extern "C" fn CL_DiminishingTrail(
    mut start: *mut vec_t,
    mut end: *mut vec_t,
    mut old: *mut centity_t,
    mut flags: libc::c_int,
) {
    let mut move_0: vec3_t = [0.; 3];
    let mut vec: vec3_t = [0.; 3];
    let mut len: libc::c_float = 0.;
    let mut j: libc::c_int = 0;
    let mut p: *mut cparticle_t = 0 as *mut cparticle_t;
    let mut dec: libc::c_float = 0.;
    let mut orgscale: libc::c_float = 0.;
    let mut velscale: libc::c_float = 0.;
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
    dec = 0.5f64 as libc::c_float;
    VectorScale(vec.as_mut_ptr(), dec, vec.as_mut_ptr());
    if (*old).trailcount > 900 as libc::c_int {
        orgscale = 4 as libc::c_int as libc::c_float;
        velscale = 15 as libc::c_int as libc::c_float;
    } else if (*old).trailcount > 800 as libc::c_int {
        orgscale = 2 as libc::c_int as libc::c_float;
        velscale = 10 as libc::c_int as libc::c_float;
    } else {
        orgscale = 1 as libc::c_int as libc::c_float;
        velscale = 5 as libc::c_int as libc::c_float;
    }
    while len > 0 as libc::c_int as libc::c_float {
        len -= dec;
        if free_particles.is_null() {
            return;
        }
        if (rand() & 1023 as libc::c_int) < (*old).trailcount {
            p = free_particles;
            free_particles = (*p).next;
            let ref mut fresh32 = (*p).next;
            *fresh32 = active_particles;
            active_particles = p;
            let ref mut fresh33 = (*p).accel[2 as libc::c_int as usize];
            *fresh33 = 0 as libc::c_int as vec_t;
            let ref mut fresh34 = (*p).accel[1 as libc::c_int as usize];
            *fresh34 = *fresh33;
            (*p).accel[0 as libc::c_int as usize] = *fresh34;
            (*p).time = cl.time as libc::c_float;
            if flags & 0x2 as libc::c_int != 0 {
                (*p).alpha = 1.0f64 as libc::c_float;
                (*p)
                    .alphavel = (-1.0f64
                    / (1 as libc::c_int as libc::c_double
                    + frand() as libc::c_double * 0.4f64)) as libc::c_float;
                (*p)
                    .color = (0xe8 as libc::c_int + (rand() & 7 as libc::c_int))
                    as libc::c_float;
                j = 0 as libc::c_int;
                while j < 3 as libc::c_int {
                    (*p).org[j as usize] = move_0[j as usize] + crand() * orgscale;
                    (*p).vel[j as usize] = crand() * velscale;
                    (*p).accel[j as usize] = 0 as libc::c_int as vec_t;
                    j += 1;
                }
                let ref mut fresh35 = (*p).vel[2 as libc::c_int as usize];
                *fresh35 -= 40 as libc::c_int as libc::c_float;
            } else if flags & 0x200000 as libc::c_int != 0 {
                (*p).alpha = 1.0f64 as libc::c_float;
                (*p)
                    .alphavel = (-1.0f64
                    / (1 as libc::c_int as libc::c_double
                    + frand() as libc::c_double * 0.4f64)) as libc::c_float;
                (*p)
                    .color = (0xdb as libc::c_int + (rand() & 7 as libc::c_int))
                    as libc::c_float;
                j = 0 as libc::c_int;
                while j < 3 as libc::c_int {
                    (*p).org[j as usize] = move_0[j as usize] + crand() * orgscale;
                    (*p).vel[j as usize] = crand() * velscale;
                    (*p).accel[j as usize] = 0 as libc::c_int as vec_t;
                    j += 1;
                }
                let ref mut fresh36 = (*p).vel[2 as libc::c_int as usize];
                *fresh36 -= 40 as libc::c_int as libc::c_float;
            } else {
                (*p).alpha = 1.0f64 as libc::c_float;
                (*p)
                    .alphavel = (-1.0f64
                    / (1 as libc::c_int as libc::c_double
                    + frand() as libc::c_double * 0.2f64)) as libc::c_float;
                (*p)
                    .color = (4 as libc::c_int + (rand() & 7 as libc::c_int))
                    as libc::c_float;
                j = 0 as libc::c_int;
                while j < 3 as libc::c_int {
                    (*p).org[j as usize] = move_0[j as usize] + crand() * orgscale;
                    (*p).vel[j as usize] = crand() * velscale;
                    j += 1;
                }
                (*p).accel[2 as libc::c_int as usize] = 20 as libc::c_int as vec_t;
            }
        }
        (*old).trailcount -= 5 as libc::c_int;
        if (*old).trailcount < 100 as libc::c_int {
            (*old).trailcount = 100 as libc::c_int;
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
pub unsafe extern "C" fn MakeNormalVectors(
    mut forward: *mut vec_t,
    mut right: *mut vec_t,
    mut up: *mut vec_t,
) {
    let mut d: libc::c_float = 0.;
    *right
        .offset(1 as libc::c_int as isize) = -*forward.offset(0 as libc::c_int as isize);
    *right
        .offset(2 as libc::c_int as isize) = *forward.offset(1 as libc::c_int as isize);
    *right
        .offset(0 as libc::c_int as isize) = *forward.offset(2 as libc::c_int as isize);
    d = *right.offset(0 as libc::c_int as isize)
        * *forward.offset(0 as libc::c_int as isize)
        + *right.offset(1 as libc::c_int as isize)
        * *forward.offset(1 as libc::c_int as isize)
        + *right.offset(2 as libc::c_int as isize)
        * *forward.offset(2 as libc::c_int as isize);
    VectorMA(right, -d, forward, right);
    VectorNormalize(right);
    CrossProduct(right, forward, up);
}

#[no_mangle]
pub unsafe extern "C" fn CL_RocketTrail(
    mut start: *mut vec_t,
    mut end: *mut vec_t,
    mut old: *mut centity_t,
) {
    let mut move_0: vec3_t = [0.; 3];
    let mut vec: vec3_t = [0.; 3];
    let mut len: libc::c_float = 0.;
    let mut j: libc::c_int = 0;
    let mut p: *mut cparticle_t = 0 as *mut cparticle_t;
    let mut dec: libc::c_float = 0.;
    CL_DiminishingTrail(start, end, old, 0x10 as libc::c_int);
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
    dec = 1 as libc::c_int as libc::c_float;
    VectorScale(vec.as_mut_ptr(), dec, vec.as_mut_ptr());
    while len > 0 as libc::c_int as libc::c_float {
        len -= dec;
        if free_particles.is_null() {
            return;
        }
        if rand() & 7 as libc::c_int == 0 as libc::c_int {
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
            (*p)
                .alphavel = (-1.0f64
                / (1 as libc::c_int as libc::c_double
                + frand() as libc::c_double * 0.2f64)) as libc::c_float;
            (*p)
                .color = (0xdc as libc::c_int + (rand() & 3 as libc::c_int))
                as libc::c_float;
            j = 0 as libc::c_int;
            while j < 3 as libc::c_int {
                (*p)
                    .org[j
                    as usize] = move_0[j as usize]
                    + crand() * 5 as libc::c_int as libc::c_float;
                (*p).vel[j as usize] = crand() * 20 as libc::c_int as libc::c_float;
                j += 1;
            }
            (*p).accel[2 as libc::c_int as usize] = -(40 as libc::c_int) as vec_t;
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
pub unsafe extern "C" fn CL_RailTrail(mut start: *mut vec_t, mut end: *mut vec_t) {
    let mut move_0: vec3_t = [0.; 3];
    let mut vec: vec3_t = [0.; 3];
    let mut len: libc::c_float = 0.;
    let mut j: libc::c_int = 0;
    let mut p: *mut cparticle_t = 0 as *mut cparticle_t;
    let mut dec: libc::c_float = 0.;
    let mut right: vec3_t = [0.; 3];
    let mut up: vec3_t = [0.; 3];
    let mut i: libc::c_int = 0;
    let mut d: libc::c_float = 0.;
    let mut c: libc::c_float = 0.;
    let mut s: libc::c_float = 0.;
    let mut dir: vec3_t = [0.; 3];
    let mut clr: byte = 0x74 as libc::c_int as byte;
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
    i = 0 as libc::c_int;
    while (i as libc::c_float) < len {
        if free_particles.is_null() {
            return;
        }
        p = free_particles;
        free_particles = (*p).next;
        let ref mut fresh40 = (*p).next;
        *fresh40 = active_particles;
        active_particles = p;
        (*p).time = cl.time as libc::c_float;
        let ref mut fresh41 = (*p).accel[2 as libc::c_int as usize];
        *fresh41 = 0 as libc::c_int as vec_t;
        let ref mut fresh42 = (*p).accel[1 as libc::c_int as usize];
        *fresh42 = *fresh41;
        (*p).accel[0 as libc::c_int as usize] = *fresh42;
        d = (i as libc::c_double * 0.1f64) as libc::c_float;
        c = cos(d as libc::c_double) as libc::c_float;
        s = sin(d as libc::c_double) as libc::c_float;
        VectorScale(right.as_mut_ptr(), c, dir.as_mut_ptr());
        VectorMA(dir.as_mut_ptr(), s, up.as_mut_ptr(), dir.as_mut_ptr());
        (*p).alpha = 1.0f64 as libc::c_float;
        (*p)
            .alphavel = (-1.0f64
            / (1 as libc::c_int as libc::c_double + frand() as libc::c_double * 0.2f64))
            as libc::c_float;
        (*p).color = (clr as libc::c_int + (rand() & 7 as libc::c_int)) as libc::c_float;
        j = 0 as libc::c_int;
        while j < 3 as libc::c_int {
            (*p)
                .org[j
                as usize] = move_0[j as usize]
                + dir[j as usize] * 3 as libc::c_int as libc::c_float;
            (*p).vel[j as usize] = dir[j as usize] * 6 as libc::c_int as libc::c_float;
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
        i += 1;
    }
    dec = 0.75f64 as libc::c_float;
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
        let ref mut fresh43 = (*p).next;
        *fresh43 = active_particles;
        active_particles = p;
        (*p).time = cl.time as libc::c_float;
        let ref mut fresh44 = (*p).accel[2 as libc::c_int as usize];
        *fresh44 = 0 as libc::c_int as vec_t;
        let ref mut fresh45 = (*p).accel[1 as libc::c_int as usize];
        *fresh45 = *fresh44;
        (*p).accel[0 as libc::c_int as usize] = *fresh45;
        (*p).alpha = 1.0f64 as libc::c_float;
        (*p)
            .alphavel = (-1.0f64 / (0.6f64 + frand() as libc::c_double * 0.2f64))
            as libc::c_float;
        (*p).color = (0 as libc::c_int + rand() & 15 as libc::c_int) as libc::c_float;
        j = 0 as libc::c_int;
        while j < 3 as libc::c_int {
            (*p)
                .org[j
                as usize] = move_0[j as usize]
                + crand() * 3 as libc::c_int as libc::c_float;
            (*p).vel[j as usize] = crand() * 3 as libc::c_int as libc::c_float;
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
pub unsafe extern "C" fn CL_IonripperTrail(mut start: *mut vec_t, mut ent: *mut vec_t) {
    let mut move_0: vec3_t = [0.; 3];
    let mut vec: vec3_t = [0.; 3];
    let mut len: libc::c_float = 0.;
    let mut j: libc::c_int = 0;
    let mut p: *mut cparticle_t = 0 as *mut cparticle_t;
    let mut dec: libc::c_int = 0;
    let mut left: libc::c_int = 0 as libc::c_int;
    move_0[0 as libc::c_int as usize] = *start.offset(0 as libc::c_int as isize);
    move_0[1 as libc::c_int as usize] = *start.offset(1 as libc::c_int as isize);
    move_0[2 as libc::c_int as usize] = *start.offset(2 as libc::c_int as isize);
    vec[0 as libc::c_int
        as usize] = *ent.offset(0 as libc::c_int as isize)
        - *start.offset(0 as libc::c_int as isize);
    vec[1 as libc::c_int
        as usize] = *ent.offset(1 as libc::c_int as isize)
        - *start.offset(1 as libc::c_int as isize);
    vec[2 as libc::c_int
        as usize] = *ent.offset(2 as libc::c_int as isize)
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
        let ref mut fresh46 = (*p).next;
        *fresh46 = active_particles;
        active_particles = p;
        let ref mut fresh47 = (*p).accel[2 as libc::c_int as usize];
        *fresh47 = 0 as libc::c_int as vec_t;
        let ref mut fresh48 = (*p).accel[1 as libc::c_int as usize];
        *fresh48 = *fresh47;
        (*p).accel[0 as libc::c_int as usize] = *fresh48;
        (*p).time = cl.time as libc::c_float;
        (*p).alpha = 0.5f64 as libc::c_float;
        (*p)
            .alphavel = (-1.0f64 / (0.3f64 + frand() as libc::c_double * 0.2f64))
            as libc::c_float;
        (*p)
            .color = (0xe4 as libc::c_int + (rand() & 3 as libc::c_int))
            as libc::c_float;
        j = 0 as libc::c_int;
        while j < 3 as libc::c_int {
            (*p).org[j as usize] = move_0[j as usize];
            (*p).accel[j as usize] = 0 as libc::c_int as vec_t;
            j += 1;
        }
        if left != 0 {
            left = 0 as libc::c_int;
            (*p).vel[0 as libc::c_int as usize] = 10 as libc::c_int as vec_t;
        } else {
            left = 1 as libc::c_int;
            (*p).vel[0 as libc::c_int as usize] = -(10 as libc::c_int) as vec_t;
        }
        (*p).vel[1 as libc::c_int as usize] = 0 as libc::c_int as vec_t;
        (*p).vel[2 as libc::c_int as usize] = 0 as libc::c_int as vec_t;
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
pub unsafe extern "C" fn CL_BubbleTrail(mut start: *mut vec_t, mut end: *mut vec_t) {
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
    dec = 32 as libc::c_int as libc::c_float;
    VectorScale(vec.as_mut_ptr(), dec, vec.as_mut_ptr());
    i = 0 as libc::c_int;
    while (i as libc::c_float) < len {
        if free_particles.is_null() {
            return;
        }
        p = free_particles;
        free_particles = (*p).next;
        let ref mut fresh49 = (*p).next;
        *fresh49 = active_particles;
        active_particles = p;
        let ref mut fresh50 = (*p).accel[2 as libc::c_int as usize];
        *fresh50 = 0 as libc::c_int as vec_t;
        let ref mut fresh51 = (*p).accel[1 as libc::c_int as usize];
        *fresh51 = *fresh50;
        (*p).accel[0 as libc::c_int as usize] = *fresh51;
        (*p).time = cl.time as libc::c_float;
        (*p).alpha = 1.0f64 as libc::c_float;
        (*p)
            .alphavel = (-1.0f64
            / (1 as libc::c_int as libc::c_double + frand() as libc::c_double * 0.2f64))
            as libc::c_float;
        (*p).color = (4 as libc::c_int + (rand() & 7 as libc::c_int)) as libc::c_float;
        j = 0 as libc::c_int;
        while j < 3 as libc::c_int {
            (*p)
                .org[j
                as usize] = move_0[j as usize]
                + crand() * 2 as libc::c_int as libc::c_float;
            (*p).vel[j as usize] = crand() * 5 as libc::c_int as libc::c_float;
            j += 1;
        }
        let ref mut fresh52 = (*p).vel[2 as libc::c_int as usize];
        *fresh52 += 6 as libc::c_int as libc::c_float;
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
pub unsafe extern "C" fn CL_FlyParticles(
    mut origin: *mut vec_t,
    mut count: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut p: *mut cparticle_t = 0 as *mut cparticle_t;
    let mut angle: libc::c_float = 0.;
    let mut sr: libc::c_float = 0.;
    let mut sp: libc::c_float = 0.;
    let mut sy: libc::c_float = 0.;
    let mut cr: libc::c_float = 0.;
    let mut cp: libc::c_float = 0.;
    let mut cy: libc::c_float = 0.;
    let mut forward: vec3_t = [0.; 3];
    let mut dist: libc::c_float = 64 as libc::c_int as libc::c_float;
    let mut ltime: libc::c_float = 0.;
    if count > 162 as libc::c_int {
        count = 162 as libc::c_int;
    }
    if avelocities[0 as libc::c_int as usize][0 as libc::c_int as usize] == 0. {
        i = 0 as libc::c_int;
        while i < 162 as libc::c_int * 3 as libc::c_int {
            avelocities[0 as libc::c_int
                as usize][i
                as usize] = ((rand() & 255 as libc::c_int) as libc::c_double * 0.01f64)
                as vec_t;
            i += 1;
        }
    }
    ltime = (cl.time as libc::c_float as libc::c_double / 1000.0f64) as libc::c_float;
    i = 0 as libc::c_int;
    while i < count {
        angle = ltime * avelocities[i as usize][0 as libc::c_int as usize];
        sy = sin(angle as libc::c_double) as libc::c_float;
        cy = cos(angle as libc::c_double) as libc::c_float;
        angle = ltime * avelocities[i as usize][1 as libc::c_int as usize];
        sp = sin(angle as libc::c_double) as libc::c_float;
        cp = cos(angle as libc::c_double) as libc::c_float;
        angle = ltime * avelocities[i as usize][2 as libc::c_int as usize];
        sr = sin(angle as libc::c_double) as libc::c_float;
        cr = cos(angle as libc::c_double) as libc::c_float;
        forward[0 as libc::c_int as usize] = cp * cy;
        forward[1 as libc::c_int as usize] = cp * sy;
        forward[2 as libc::c_int as usize] = -sp;
        if free_particles.is_null() {
            return;
        }
        p = free_particles;
        free_particles = (*p).next;
        let ref mut fresh53 = (*p).next;
        *fresh53 = active_particles;
        active_particles = p;
        (*p).time = cl.time as libc::c_float;
        dist = (sin((ltime + i as libc::c_float) as libc::c_double)
            * 64 as libc::c_int as libc::c_double) as libc::c_float;
        (*p)
            .org[0 as libc::c_int
            as usize] = *origin.offset(0 as libc::c_int as isize)
            + bytedirs[i as usize][0 as libc::c_int as usize] * dist
            + forward[0 as libc::c_int as usize] * 16 as libc::c_int as libc::c_float;
        (*p)
            .org[1 as libc::c_int
            as usize] = *origin.offset(1 as libc::c_int as isize)
            + bytedirs[i as usize][1 as libc::c_int as usize] * dist
            + forward[1 as libc::c_int as usize] * 16 as libc::c_int as libc::c_float;
        (*p)
            .org[2 as libc::c_int
            as usize] = *origin.offset(2 as libc::c_int as isize)
            + bytedirs[i as usize][2 as libc::c_int as usize] * dist
            + forward[2 as libc::c_int as usize] * 16 as libc::c_int as libc::c_float;
        let ref mut fresh54 = (*p).vel[2 as libc::c_int as usize];
        *fresh54 = 0 as libc::c_int as vec_t;
        let ref mut fresh55 = (*p).vel[1 as libc::c_int as usize];
        *fresh55 = *fresh54;
        (*p).vel[0 as libc::c_int as usize] = *fresh55;
        let ref mut fresh56 = (*p).accel[2 as libc::c_int as usize];
        *fresh56 = 0 as libc::c_int as vec_t;
        let ref mut fresh57 = (*p).accel[1 as libc::c_int as usize];
        *fresh57 = *fresh56;
        (*p).accel[0 as libc::c_int as usize] = *fresh57;
        (*p).color = 0 as libc::c_int as libc::c_float;
        (*p).colorvel = 0 as libc::c_int as libc::c_float;
        (*p).alpha = 1 as libc::c_int as libc::c_float;
        (*p).alphavel = -(100 as libc::c_int) as libc::c_float;
        i += 2 as libc::c_int;
    }
}

#[no_mangle]
pub unsafe extern "C" fn CL_FlyEffect(mut ent: *mut centity_t, mut origin: *mut vec_t) {
    let mut n: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    let mut starttime: libc::c_int = 0;
    if (*ent).fly_stoptime < cl.time {
        starttime = cl.time;
        (*ent).fly_stoptime = cl.time + 60000 as libc::c_int;
    } else {
        starttime = (*ent).fly_stoptime - 60000 as libc::c_int;
    }
    n = cl.time - starttime;
    if n < 20000 as libc::c_int {
        count = ((n * 162 as libc::c_int) as libc::c_double / 20000.0f64) as libc::c_int;
    } else {
        n = (*ent).fly_stoptime - cl.time;
        if n < 20000 as libc::c_int {
            count = ((n * 162 as libc::c_int) as libc::c_double / 20000.0f64)
                as libc::c_int;
        } else {
            count = 162 as libc::c_int;
        }
    }
    CL_FlyParticles(origin, count);
}

#[no_mangle]
pub unsafe extern "C" fn CL_BfgParticles(mut ent: *mut entity_t) {
    let mut i: libc::c_int = 0;
    let mut p: *mut cparticle_t = 0 as *mut cparticle_t;
    let mut angle: libc::c_float = 0.;
    let mut sr: libc::c_float = 0.;
    let mut sp: libc::c_float = 0.;
    let mut sy: libc::c_float = 0.;
    let mut cr: libc::c_float = 0.;
    let mut cp: libc::c_float = 0.;
    let mut cy: libc::c_float = 0.;
    let mut forward: vec3_t = [0.; 3];
    let mut dist: libc::c_float = 64 as libc::c_int as libc::c_float;
    let mut v: vec3_t = [0.; 3];
    let mut ltime: libc::c_float = 0.;
    if avelocities[0 as libc::c_int as usize][0 as libc::c_int as usize] == 0. {
        i = 0 as libc::c_int;
        while i < 162 as libc::c_int * 3 as libc::c_int {
            avelocities[0 as libc::c_int
                as usize][i
                as usize] = ((rand() & 255 as libc::c_int) as libc::c_double * 0.01f64)
                as vec_t;
            i += 1;
        }
    }
    ltime = (cl.time as libc::c_float as libc::c_double / 1000.0f64) as libc::c_float;
    i = 0 as libc::c_int;
    while i < 162 as libc::c_int {
        angle = ltime * avelocities[i as usize][0 as libc::c_int as usize];
        sy = sin(angle as libc::c_double) as libc::c_float;
        cy = cos(angle as libc::c_double) as libc::c_float;
        angle = ltime * avelocities[i as usize][1 as libc::c_int as usize];
        sp = sin(angle as libc::c_double) as libc::c_float;
        cp = cos(angle as libc::c_double) as libc::c_float;
        angle = ltime * avelocities[i as usize][2 as libc::c_int as usize];
        sr = sin(angle as libc::c_double) as libc::c_float;
        cr = cos(angle as libc::c_double) as libc::c_float;
        forward[0 as libc::c_int as usize] = cp * cy;
        forward[1 as libc::c_int as usize] = cp * sy;
        forward[2 as libc::c_int as usize] = -sp;
        if free_particles.is_null() {
            return;
        }
        p = free_particles;
        free_particles = (*p).next;
        let ref mut fresh58 = (*p).next;
        *fresh58 = active_particles;
        active_particles = p;
        (*p).time = cl.time as libc::c_float;
        dist = (sin((ltime + i as libc::c_float) as libc::c_double)
            * 64 as libc::c_int as libc::c_double) as libc::c_float;
        (*p)
            .org[0 as libc::c_int
            as usize] = (*ent).origin[0 as libc::c_int as usize]
            + bytedirs[i as usize][0 as libc::c_int as usize] * dist
            + forward[0 as libc::c_int as usize] * 16 as libc::c_int as libc::c_float;
        (*p)
            .org[1 as libc::c_int
            as usize] = (*ent).origin[1 as libc::c_int as usize]
            + bytedirs[i as usize][1 as libc::c_int as usize] * dist
            + forward[1 as libc::c_int as usize] * 16 as libc::c_int as libc::c_float;
        (*p)
            .org[2 as libc::c_int
            as usize] = (*ent).origin[2 as libc::c_int as usize]
            + bytedirs[i as usize][2 as libc::c_int as usize] * dist
            + forward[2 as libc::c_int as usize] * 16 as libc::c_int as libc::c_float;
        let ref mut fresh59 = (*p).vel[2 as libc::c_int as usize];
        *fresh59 = 0 as libc::c_int as vec_t;
        let ref mut fresh60 = (*p).vel[1 as libc::c_int as usize];
        *fresh60 = *fresh59;
        (*p).vel[0 as libc::c_int as usize] = *fresh60;
        let ref mut fresh61 = (*p).accel[2 as libc::c_int as usize];
        *fresh61 = 0 as libc::c_int as vec_t;
        let ref mut fresh62 = (*p).accel[1 as libc::c_int as usize];
        *fresh62 = *fresh61;
        (*p).accel[0 as libc::c_int as usize] = *fresh62;
        v[0 as libc::c_int
            as usize] = (*p).org[0 as libc::c_int as usize]
            - (*ent).origin[0 as libc::c_int as usize];
        v[1 as libc::c_int
            as usize] = (*p).org[1 as libc::c_int as usize]
            - (*ent).origin[1 as libc::c_int as usize];
        v[2 as libc::c_int
            as usize] = (*p).org[2 as libc::c_int as usize]
            - (*ent).origin[2 as libc::c_int as usize];
        dist = (VectorLength(v.as_mut_ptr()) as libc::c_double / 90.0f64)
            as libc::c_float;
        (*p)
            .color = floor(
            (0xd0 as libc::c_int as libc::c_float
                + dist * 7 as libc::c_int as libc::c_float) as libc::c_double,
        ) as libc::c_float;
        (*p).colorvel = 0 as libc::c_int as libc::c_float;
        (*p).alpha = (1.0f64 - dist as libc::c_double) as libc::c_float;
        (*p).alphavel = -(100 as libc::c_int) as libc::c_float;
        i += 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn CL_TrapParticles(mut ent: *mut entity_t) {
    let mut move_0: vec3_t = [0.; 3];
    let mut vec: vec3_t = [0.; 3];
    let mut start: vec3_t = [0.; 3];
    let mut end: vec3_t = [0.; 3];
    let mut len: libc::c_float = 0.;
    let mut j: libc::c_int = 0;
    let mut p: *mut cparticle_t = 0 as *mut cparticle_t;
    let mut dec: libc::c_int = 0;
    (*ent).origin[2 as libc::c_int as usize] -= 14 as libc::c_int as libc::c_float;
    start[0 as libc::c_int as usize] = (*ent).origin[0 as libc::c_int as usize];
    start[1 as libc::c_int as usize] = (*ent).origin[1 as libc::c_int as usize];
    start[2 as libc::c_int as usize] = (*ent).origin[2 as libc::c_int as usize];
    end[0 as libc::c_int as usize] = (*ent).origin[0 as libc::c_int as usize];
    end[1 as libc::c_int as usize] = (*ent).origin[1 as libc::c_int as usize];
    end[2 as libc::c_int as usize] = (*ent).origin[2 as libc::c_int as usize];
    end[2 as libc::c_int as usize] += 64 as libc::c_int as libc::c_float;
    move_0[0 as libc::c_int as usize] = start[0 as libc::c_int as usize];
    move_0[1 as libc::c_int as usize] = start[1 as libc::c_int as usize];
    move_0[2 as libc::c_int as usize] = start[2 as libc::c_int as usize];
    vec[0 as libc::c_int
        as usize] = end[0 as libc::c_int as usize] - start[0 as libc::c_int as usize];
    vec[1 as libc::c_int
        as usize] = end[1 as libc::c_int as usize] - start[1 as libc::c_int as usize];
    vec[2 as libc::c_int
        as usize] = end[2 as libc::c_int as usize] - start[2 as libc::c_int as usize];
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
        let ref mut fresh63 = (*p).next;
        *fresh63 = active_particles;
        active_particles = p;
        let ref mut fresh64 = (*p).accel[2 as libc::c_int as usize];
        *fresh64 = 0 as libc::c_int as vec_t;
        let ref mut fresh65 = (*p).accel[1 as libc::c_int as usize];
        *fresh65 = *fresh64;
        (*p).accel[0 as libc::c_int as usize] = *fresh65;
        (*p).time = cl.time as libc::c_float;
        (*p).alpha = 1.0f64 as libc::c_float;
        (*p)
            .alphavel = (-1.0f64 / (0.3f64 + frand() as libc::c_double * 0.2f64))
            as libc::c_float;
        (*p).color = 0xe0 as libc::c_int as libc::c_float;
        j = 0 as libc::c_int;
        while j < 3 as libc::c_int {
            (*p).org[j as usize] = move_0[j as usize] + crand();
            (*p).vel[j as usize] = crand() * 15 as libc::c_int as libc::c_float;
            (*p).accel[j as usize] = 0 as libc::c_int as vec_t;
            j += 1;
        }
        (*p).accel[2 as libc::c_int as usize] = 40 as libc::c_int as vec_t;
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
    let mut i: libc::c_int = 0;
    let mut j_0: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut p_0: *mut cparticle_t = 0 as *mut cparticle_t;
    let mut vel: libc::c_float = 0.;
    let mut dir: vec3_t = [0.; 3];
    let mut org: vec3_t = [0.; 3];
    (*ent).origin[2 as libc::c_int as usize] += 14 as libc::c_int as libc::c_float;
    org[0 as libc::c_int as usize] = (*ent).origin[0 as libc::c_int as usize];
    org[1 as libc::c_int as usize] = (*ent).origin[1 as libc::c_int as usize];
    org[2 as libc::c_int as usize] = (*ent).origin[2 as libc::c_int as usize];
    i = -(2 as libc::c_int);
    while i <= 2 as libc::c_int {
        j_0 = -(2 as libc::c_int);
        while j_0 <= 2 as libc::c_int {
            k = -(2 as libc::c_int);
            while k <= 4 as libc::c_int {
                if free_particles.is_null() {
                    return;
                }
                p_0 = free_particles;
                free_particles = (*p_0).next;
                let ref mut fresh66 = (*p_0).next;
                *fresh66 = active_particles;
                active_particles = p_0;
                (*p_0).time = cl.time as libc::c_float;
                (*p_0)
                    .color = (0xe0 as libc::c_int + (rand() & 3 as libc::c_int))
                    as libc::c_float;
                (*p_0).alpha = 1.0f64 as libc::c_float;
                (*p_0)
                    .alphavel = (-1.0f64
                    / (0.3f64 + (rand() & 7 as libc::c_int) as libc::c_double * 0.02f64))
                    as libc::c_float;
                (*p_0)
                    .org[0 as libc::c_int
                    as usize] = org[0 as libc::c_int as usize] + i as libc::c_float
                    + (rand() & 23 as libc::c_int) as libc::c_float * crand();
                (*p_0)
                    .org[1 as libc::c_int
                    as usize] = org[1 as libc::c_int as usize] + j_0 as libc::c_float
                    + (rand() & 23 as libc::c_int) as libc::c_float * crand();
                (*p_0)
                    .org[2 as libc::c_int
                    as usize] = org[2 as libc::c_int as usize] + k as libc::c_float
                    + (rand() & 23 as libc::c_int) as libc::c_float * crand();
                dir[0 as libc::c_int as usize] = (j_0 * 8 as libc::c_int) as vec_t;
                dir[1 as libc::c_int as usize] = (i * 8 as libc::c_int) as vec_t;
                dir[2 as libc::c_int as usize] = (k * 8 as libc::c_int) as vec_t;
                VectorNormalize(dir.as_mut_ptr());
                vel = (50 as libc::c_int + rand() & 63 as libc::c_int) as libc::c_float;
                VectorScale(dir.as_mut_ptr(), vel, ((*p_0).vel).as_mut_ptr());
                let ref mut fresh67 = (*p_0).accel[1 as libc::c_int as usize];
                *fresh67 = 0 as libc::c_int as vec_t;
                (*p_0).accel[0 as libc::c_int as usize] = *fresh67;
                (*p_0).accel[2 as libc::c_int as usize] = -(40 as libc::c_int) as vec_t;
                k += 4 as libc::c_int;
            }
            j_0 += 4 as libc::c_int;
        }
        i += 4 as libc::c_int;
    }
}

#[no_mangle]
pub unsafe extern "C" fn CL_BFGExplosionParticles(mut org: *mut vec_t) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut p: *mut cparticle_t = 0 as *mut cparticle_t;
    i = 0 as libc::c_int;
    while i < 256 as libc::c_int {
        if free_particles.is_null() {
            return;
        }
        p = free_particles;
        free_particles = (*p).next;
        let ref mut fresh68 = (*p).next;
        *fresh68 = active_particles;
        active_particles = p;
        (*p).time = cl.time as libc::c_float;
        (*p)
            .color = (0xd0 as libc::c_int + (rand() & 7 as libc::c_int))
            as libc::c_float;
        j = 0 as libc::c_int;
        while j < 3 as libc::c_int {
            (*p)
                .org[j
                as usize] = *org.offset(j as isize)
                + (rand() % 32 as libc::c_int - 16 as libc::c_int) as libc::c_float;
            (*p)
                .vel[j
                as usize] = (rand() % 384 as libc::c_int - 192 as libc::c_int) as vec_t;
            j += 1;
        }
        let ref mut fresh69 = (*p).accel[1 as libc::c_int as usize];
        *fresh69 = 0 as libc::c_int as vec_t;
        (*p).accel[0 as libc::c_int as usize] = *fresh69;
        (*p).accel[2 as libc::c_int as usize] = -(40 as libc::c_int) as vec_t;
        (*p).alpha = 1.0f64 as libc::c_float;
        (*p)
            .alphavel = (-0.8f64 / (0.5f64 + frand() as libc::c_double * 0.3f64))
            as libc::c_float;
        i += 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn CL_TeleportParticles(mut org: *mut vec_t) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut p: *mut cparticle_t = 0 as *mut cparticle_t;
    let mut vel: libc::c_float = 0.;
    let mut dir: vec3_t = [0.; 3];
    i = -(16 as libc::c_int);
    while i <= 16 as libc::c_int {
        j = -(16 as libc::c_int);
        while j <= 16 as libc::c_int {
            k = -(16 as libc::c_int);
            while k <= 32 as libc::c_int {
                if free_particles.is_null() {
                    return;
                }
                p = free_particles;
                free_particles = (*p).next;
                let ref mut fresh70 = (*p).next;
                *fresh70 = active_particles;
                active_particles = p;
                (*p).time = cl.time as libc::c_float;
                (*p)
                    .color = (7 as libc::c_int + (rand() & 7 as libc::c_int))
                    as libc::c_float;
                (*p).alpha = 1.0f64 as libc::c_float;
                (*p)
                    .alphavel = (-1.0f64
                    / (0.3f64 + (rand() & 7 as libc::c_int) as libc::c_double * 0.02f64))
                    as libc::c_float;
                (*p)
                    .org[0 as libc::c_int
                    as usize] = *org.offset(0 as libc::c_int as isize)
                    + i as libc::c_float + (rand() & 3 as libc::c_int) as libc::c_float;
                (*p)
                    .org[1 as libc::c_int
                    as usize] = *org.offset(1 as libc::c_int as isize)
                    + j as libc::c_float + (rand() & 3 as libc::c_int) as libc::c_float;
                (*p)
                    .org[2 as libc::c_int
                    as usize] = *org.offset(2 as libc::c_int as isize)
                    + k as libc::c_float + (rand() & 3 as libc::c_int) as libc::c_float;
                dir[0 as libc::c_int as usize] = (j * 8 as libc::c_int) as vec_t;
                dir[1 as libc::c_int as usize] = (i * 8 as libc::c_int) as vec_t;
                dir[2 as libc::c_int as usize] = (k * 8 as libc::c_int) as vec_t;
                VectorNormalize(dir.as_mut_ptr());
                vel = (50 as libc::c_int + (rand() & 63 as libc::c_int))
                    as libc::c_float;
                VectorScale(dir.as_mut_ptr(), vel, ((*p).vel).as_mut_ptr());
                let ref mut fresh71 = (*p).accel[1 as libc::c_int as usize];
                *fresh71 = 0 as libc::c_int as vec_t;
                (*p).accel[0 as libc::c_int as usize] = *fresh71;
                (*p).accel[2 as libc::c_int as usize] = -(40 as libc::c_int) as vec_t;
                k += 4 as libc::c_int;
            }
            j += 4 as libc::c_int;
        }
        i += 4 as libc::c_int;
    }
}

#[no_mangle]
pub unsafe extern "C" fn CL_AddParticles() {
    let mut p: *mut cparticle_t = 0 as *mut cparticle_t;
    let mut next: *mut cparticle_t = 0 as *mut cparticle_t;
    let mut alpha: libc::c_float = 0.;
    let mut time: libc::c_float = 0.;
    let mut time2: libc::c_float = 0.;
    let mut org: vec3_t = [0.; 3];
    let mut color: libc::c_int = 0;
    let mut active: *mut cparticle_t = 0 as *mut cparticle_t;
    let mut tail: *mut cparticle_t = 0 as *mut cparticle_t;
    active = 0 as *mut cparticle_t;
    tail = 0 as *mut cparticle_t;
    let mut current_block_27: u64;
    p = active_particles;
    while !p.is_null() {
        next = (*p).next;
        if (*p).alphavel as libc::c_double != -10000.0f64 {
            time = ((cl.time as libc::c_float - (*p).time) as libc::c_double * 0.001f64)
                as libc::c_float;
            alpha = (*p).alpha + time * (*p).alphavel;
            if alpha <= 0 as libc::c_int as libc::c_float {
                let ref mut fresh72 = (*p).next;
                *fresh72 = free_particles;
                free_particles = p;
                current_block_27 = 11875828834189669668;
            } else {
                current_block_27 = 8831408221741692167;
            }
        } else {
            alpha = (*p).alpha;
            current_block_27 = 8831408221741692167;
        }
        match current_block_27 {
            8831408221741692167 => {
                let ref mut fresh73 = (*p).next;
                *fresh73 = 0 as *mut particle_s;
                if tail.is_null() {
                    tail = p;
                    active = tail;
                } else {
                    let ref mut fresh74 = (*tail).next;
                    *fresh74 = p;
                    tail = p;
                }
                if alpha as libc::c_double > 1.0f64 {
                    alpha = 1 as libc::c_int as libc::c_float;
                }
                color = (*p).color as libc::c_int;
                time2 = time * time;
                org[0 as libc::c_int
                    as usize] = (*p).org[0 as libc::c_int as usize]
                    + (*p).vel[0 as libc::c_int as usize] * time
                    + (*p).accel[0 as libc::c_int as usize] * time2;
                org[1 as libc::c_int
                    as usize] = (*p).org[1 as libc::c_int as usize]
                    + (*p).vel[1 as libc::c_int as usize] * time
                    + (*p).accel[1 as libc::c_int as usize] * time2;
                org[2 as libc::c_int
                    as usize] = (*p).org[2 as libc::c_int as usize]
                    + (*p).vel[2 as libc::c_int as usize] * time
                    + (*p).accel[2 as libc::c_int as usize] * time2;
                V_AddParticle(org.as_mut_ptr(), color, alpha);
                if (*p).alphavel as libc::c_double == -10000.0f64 {
                    (*p).alphavel = 0.0f64 as libc::c_float;
                    (*p).alpha = 0.0f64 as libc::c_float;
                }
            }
            _ => {}
        }
        p = next;
    }
    active_particles = active;
}

#[no_mangle]
pub unsafe extern "C" fn CL_EntityEvent(mut ent: *mut entity_state_t) {
    match (*ent).event {
        1 => {
            S_StartSound(
                0 as *mut vec_t,
                (*ent).number,
                1 as libc::c_int,
                S_RegisterSound(
                    b"items/respawn1.wav\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                ),
                1 as libc::c_int as libc::c_float,
                2 as libc::c_int as libc::c_float,
                0 as libc::c_int as libc::c_float,
            );
            CL_ItemRespawnParticles(((*ent).origin).as_mut_ptr());
        }
        6 => {
            S_StartSound(
                0 as *mut vec_t,
                (*ent).number,
                1 as libc::c_int,
                S_RegisterSound(
                    b"misc/tele1.wav\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                ),
                1 as libc::c_int as libc::c_float,
                2 as libc::c_int as libc::c_float,
                0 as libc::c_int as libc::c_float,
            );
            CL_TeleportParticles(((*ent).origin).as_mut_ptr());
        }
        2 => {
            if (*cl_footsteps).value != 0. {
                S_StartSound(
                    0 as *mut vec_t,
                    (*ent).number,
                    4 as libc::c_int,
                    cl_sfx_footsteps[(rand() & 3 as libc::c_int) as usize],
                    1 as libc::c_int as libc::c_float,
                    1 as libc::c_int as libc::c_float,
                    0 as libc::c_int as libc::c_float,
                );
            }
        }
        3 => {
            S_StartSound(
                0 as *mut vec_t,
                (*ent).number,
                0 as libc::c_int,
                S_RegisterSound(
                    b"player/land1.wav\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                ),
                1 as libc::c_int as libc::c_float,
                1 as libc::c_int as libc::c_float,
                0 as libc::c_int as libc::c_float,
            );
        }
        4 => {
            S_StartSound(
                0 as *mut vec_t,
                (*ent).number,
                0 as libc::c_int,
                S_RegisterSound(
                    b"*fall2.wav\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                ),
                1 as libc::c_int as libc::c_float,
                1 as libc::c_int as libc::c_float,
                0 as libc::c_int as libc::c_float,
            );
        }
        5 => {
            S_StartSound(
                0 as *mut vec_t,
                (*ent).number,
                0 as libc::c_int,
                S_RegisterSound(
                    b"*fall1.wav\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                ),
                1 as libc::c_int as libc::c_float,
                1 as libc::c_int as libc::c_float,
                0 as libc::c_int as libc::c_float,
            );
        }
        _ => {}
    };
}

#[no_mangle]
pub unsafe extern "C" fn CL_ClearEffects() {
    CL_ClearParticles();
    CL_ClearDlights();
    CL_ClearLightStyles();
}
