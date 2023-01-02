#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(extern_types, register_tool)]

extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type edict_s;
    pub type image_s;
    pub type model_s;
    pub type sfx_s;
    fn atof(__nptr: *const libc::c_char) -> libc::c_double;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn abs(_: libc::c_int) -> libc::c_int;
    static mut vec3_origin: vec3_t;
    fn Com_Printf(msg: *mut libc::c_char, _: ...);
    fn CM_HeadnodeForBox(mins: *mut vec_t, maxs: *mut vec_t) -> libc::c_int;
    fn CM_PointContents(p: *mut vec_t, headnode: libc::c_int) -> libc::c_int;
    fn CM_TransformedPointContents(
        p: *mut vec_t,
        headnode: libc::c_int,
        origin: *mut vec_t,
        angles: *mut vec_t,
    ) -> libc::c_int;
    fn CM_BoxTrace(
        start: *mut vec_t,
        end: *mut vec_t,
        mins: *mut vec_t,
        maxs: *mut vec_t,
        headnode: libc::c_int,
        brushmask: libc::c_int,
    ) -> trace_t;
    fn CM_TransformedBoxTrace(
        start: *mut vec_t,
        end: *mut vec_t,
        mins: *mut vec_t,
        maxs: *mut vec_t,
        headnode: libc::c_int,
        brushmask: libc::c_int,
        origin: *mut vec_t,
        angles: *mut vec_t,
    ) -> trace_t;
    static mut pm_airaccelerate: libc::c_float;
    fn Pmove(pmove: *mut pmove_t);
    static mut cl: client_state_t;
    static mut cls: client_static_t;
    static mut cl_predict: *mut cvar_t;
    static mut cl_showmiss: *mut cvar_t;
    static mut cl_paused: *mut cvar_t;
    static mut cl_parse_entities: [entity_state_t; 1024];
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
pub struct cplane_s {
    pub normal: vec3_t,
    pub dist: libc::c_float,
    pub type_0: byte,
    pub signbits: byte,
    pub pad: [byte; 2],
}

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
pub type cplane_t = cplane_s;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct cmodel_s {
    pub mins: vec3_t,
    pub maxs: vec3_t,
    pub origin: vec3_t,
    pub headnode: libc::c_int,
}

pub type cmodel_t = cmodel_s;

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

#[no_mangle]
pub unsafe extern "C" fn CL_CheckPredictionError() {
    let mut frame: libc::c_int = 0;
    let mut delta: [libc::c_int; 3] = [0; 3];
    let mut i: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    if (*cl_predict).value == 0.
        || cl.frame.playerstate.pmove.pm_flags as libc::c_int & 64 as libc::c_int != 0
    {
        return;
    }
    frame = cls.netchan.incoming_acknowledged;
    frame &= 64 as libc::c_int - 1 as libc::c_int;
    delta[0 as libc::c_int
        as usize] = cl.frame.playerstate.pmove.origin[0 as libc::c_int as usize]
        as libc::c_int
        - cl.predicted_origins[frame as usize][0 as libc::c_int as usize] as libc::c_int;
    delta[1 as libc::c_int
        as usize] = cl.frame.playerstate.pmove.origin[1 as libc::c_int as usize]
        as libc::c_int
        - cl.predicted_origins[frame as usize][1 as libc::c_int as usize] as libc::c_int;
    delta[2 as libc::c_int
        as usize] = cl.frame.playerstate.pmove.origin[2 as libc::c_int as usize]
        as libc::c_int
        - cl.predicted_origins[frame as usize][2 as libc::c_int as usize] as libc::c_int;
    len = abs(delta[0 as libc::c_int as usize]) + abs(delta[1 as libc::c_int as usize])
        + abs(delta[2 as libc::c_int as usize]);
    if len > 640 as libc::c_int {
        cl.prediction_error[2 as libc::c_int as usize] = 0 as libc::c_int as vec_t;
        cl
            .prediction_error[1 as libc::c_int
            as usize] = cl.prediction_error[2 as libc::c_int as usize];
        cl
            .prediction_error[0 as libc::c_int
            as usize] = cl.prediction_error[1 as libc::c_int as usize];
    } else {
        if (*cl_showmiss).value != 0.
            && (delta[0 as libc::c_int as usize] != 0
            || delta[1 as libc::c_int as usize] != 0
            || delta[2 as libc::c_int as usize] != 0)
        {
            Com_Printf(
                b"prediction miss on %i: %i\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                cl.frame.serverframe,
                delta[0 as libc::c_int as usize] + delta[1 as libc::c_int as usize]
                    + delta[2 as libc::c_int as usize],
            );
        }
        cl
            .predicted_origins[frame
            as usize][0 as libc::c_int
            as usize] = cl.frame.playerstate.pmove.origin[0 as libc::c_int as usize];
        cl
            .predicted_origins[frame
            as usize][1 as libc::c_int
            as usize] = cl.frame.playerstate.pmove.origin[1 as libc::c_int as usize];
        cl
            .predicted_origins[frame
            as usize][2 as libc::c_int
            as usize] = cl.frame.playerstate.pmove.origin[2 as libc::c_int as usize];
        i = 0 as libc::c_int;
        while i < 3 as libc::c_int {
            cl
                .prediction_error[i
                as usize] = (delta[i as usize] as libc::c_double * 0.125f64) as vec_t;
            i += 1;
        }
    };
}

#[no_mangle]
pub unsafe extern "C" fn CL_ClipMoveToEntities(
    mut start: *mut vec_t,
    mut mins: *mut vec_t,
    mut maxs: *mut vec_t,
    mut end: *mut vec_t,
    mut tr: *mut trace_t,
) {
    let mut i: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut zd: libc::c_int = 0;
    let mut zu: libc::c_int = 0;
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
    let mut headnode: libc::c_int = 0;
    let mut angles: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut ent: *mut entity_state_t = 0 as *mut entity_state_t;
    let mut num: libc::c_int = 0;
    let mut cmodel: *mut cmodel_t = 0 as *mut cmodel_t;
    let mut bmins: vec3_t = [0.; 3];
    let mut bmaxs: vec3_t = [0.; 3];
    let mut current_block_28: u64;
    i = 0 as libc::c_int;
    while i < cl.frame.num_entities {
        num = cl.frame.parse_entities + i & 1024 as libc::c_int - 1 as libc::c_int;
        ent = &mut *cl_parse_entities.as_mut_ptr().offset(num as isize)
            as *mut entity_state_t;
        if !((*ent).solid == 0) {
            if !((*ent).number == cl.playernum + 1 as libc::c_int) {
                if (*ent).solid == 31 as libc::c_int {
                    cmodel = cl.model_clip[(*ent).modelindex as usize];
                    if cmodel.is_null() {
                        current_block_28 = 15427931788582360902;
                    } else {
                        headnode = (*cmodel).headnode;
                        angles = ((*ent).angles).as_mut_ptr();
                        current_block_28 = 11194104282611034094;
                    }
                } else {
                    x = 8 as libc::c_int * ((*ent).solid & 31 as libc::c_int);
                    zd = 8 as libc::c_int
                        * ((*ent).solid >> 5 as libc::c_int & 31 as libc::c_int);
                    zu = 8 as libc::c_int
                        * ((*ent).solid >> 10 as libc::c_int & 63 as libc::c_int)
                        - 32 as libc::c_int;
                    bmins[1 as libc::c_int as usize] = -x as vec_t;
                    bmins[0 as libc::c_int as usize] = bmins[1 as libc::c_int as usize];
                    bmaxs[1 as libc::c_int as usize] = x as vec_t;
                    bmaxs[0 as libc::c_int as usize] = bmaxs[1 as libc::c_int as usize];
                    bmins[2 as libc::c_int as usize] = -zd as vec_t;
                    bmaxs[2 as libc::c_int as usize] = zu as vec_t;
                    headnode = CM_HeadnodeForBox(bmins.as_mut_ptr(), bmaxs.as_mut_ptr());
                    angles = vec3_origin.as_mut_ptr();
                    current_block_28 = 11194104282611034094;
                }
                match current_block_28 {
                    15427931788582360902 => {}
                    _ => {
                        if (*tr).allsolid as u64 != 0 {
                            return;
                        }
                        trace = CM_TransformedBoxTrace(
                            start,
                            end,
                            mins,
                            maxs,
                            headnode,
                            1 as libc::c_int | 0x10000 as libc::c_int | 2 as libc::c_int
                                | 0x2000000 as libc::c_int,
                            ((*ent).origin).as_mut_ptr(),
                            angles,
                        );
                        if trace.allsolid as libc::c_uint != 0
                            || trace.startsolid as libc::c_uint != 0
                            || trace.fraction < (*tr).fraction
                        {
                            trace.ent = ent as *mut edict_s;
                            if (*tr).startsolid as u64 != 0 {
                                *tr = trace;
                                (*tr).startsolid = true_0;
                            } else {
                                *tr = trace;
                            }
                        } else if trace.startsolid as u64 != 0 {
                            (*tr).startsolid = true_0;
                        }
                    }
                }
            }
        }
        i += 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn CL_PMTrace(
    mut start: *mut vec_t,
    mut mins: *mut vec_t,
    mut maxs: *mut vec_t,
    mut end: *mut vec_t,
) -> trace_t {
    let mut t: trace_t = trace_t {
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
    t = CM_BoxTrace(
        start,
        end,
        mins,
        maxs,
        0 as libc::c_int,
        1 as libc::c_int | 0x10000 as libc::c_int | 2 as libc::c_int
            | 0x2000000 as libc::c_int,
    );
    if (t.fraction as libc::c_double) < 1.0f64 {
        t.ent = 1 as libc::c_int as *mut edict_s;
    }
    CL_ClipMoveToEntities(start, mins, maxs, end, &mut t);
    return t;
}

#[no_mangle]
pub unsafe extern "C" fn CL_PMpointcontents(mut point: *mut vec_t) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut ent: *mut entity_state_t = 0 as *mut entity_state_t;
    let mut num: libc::c_int = 0;
    let mut cmodel: *mut cmodel_t = 0 as *mut cmodel_t;
    let mut contents: libc::c_int = 0;
    contents = CM_PointContents(point, 0 as libc::c_int);
    i = 0 as libc::c_int;
    while i < cl.frame.num_entities {
        num = cl.frame.parse_entities + i & 1024 as libc::c_int - 1 as libc::c_int;
        ent = &mut *cl_parse_entities.as_mut_ptr().offset(num as isize)
            as *mut entity_state_t;
        if !((*ent).solid != 31 as libc::c_int) {
            cmodel = cl.model_clip[(*ent).modelindex as usize];
            if !cmodel.is_null() {
                contents
                    |= CM_TransformedPointContents(
                    point,
                    (*cmodel).headnode,
                    ((*ent).origin).as_mut_ptr(),
                    ((*ent).angles).as_mut_ptr(),
                );
            }
        }
        i += 1;
    }
    return contents;
}

#[no_mangle]
pub unsafe extern "C" fn CL_PredictMovement() {
    let mut ack: libc::c_int = 0;
    let mut current: libc::c_int = 0;
    let mut frame: libc::c_int = 0;
    let mut oldframe: libc::c_int = 0;
    let mut cmd: *mut usercmd_t = 0 as *mut usercmd_t;
    let mut pm: pmove_t = pmove_t {
        s: pmove_state_t {
            pm_type: PM_NORMAL,
            origin: [0; 3],
            velocity: [0; 3],
            pm_flags: 0,
            pm_time: 0,
            gravity: 0,
            delta_angles: [0; 3],
        },
        cmd: usercmd_t {
            msec: 0,
            buttons: 0,
            angles: [0; 3],
            forwardmove: 0,
            sidemove: 0,
            upmove: 0,
            impulse: 0,
            lightlevel: 0,
        },
        snapinitial: false_0,
        numtouch: 0,
        touchents: [0 as *mut edict_s; 32],
        viewangles: [0.; 3],
        viewheight: 0.,
        mins: [0.; 3],
        maxs: [0.; 3],
        groundentity: 0 as *mut edict_s,
        watertype: 0,
        waterlevel: 0,
        trace: None,
        pointcontents: None,
    };
    let mut i: libc::c_int = 0;
    let mut step: libc::c_int = 0;
    let mut oldz: libc::c_int = 0;
    if cls.state as libc::c_uint != ca_active as libc::c_int as libc::c_uint {
        return;
    }
    if (*cl_paused).value != 0. {
        return;
    }
    if (*cl_predict).value == 0.
        || cl.frame.playerstate.pmove.pm_flags as libc::c_int & 64 as libc::c_int != 0
    {
        i = 0 as libc::c_int;
        while i < 3 as libc::c_int {
            cl
                .predicted_angles[i
                as usize] = (cl.viewangles[i as usize] as libc::c_double
                + cl.frame.playerstate.pmove.delta_angles[i as usize] as libc::c_int
                as libc::c_double
                * (360.0f64 / 65536 as libc::c_int as libc::c_double)) as vec_t;
            i += 1;
        }
        return;
    }
    ack = cls.netchan.incoming_acknowledged;
    current = cls.netchan.outgoing_sequence;
    if current - ack >= 64 as libc::c_int {
        if (*cl_showmiss).value != 0. {
            Com_Printf(
                b"exceeded CMD_BACKUP\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
        }
        return;
    }
    memset(
        &mut pm as *mut pmove_t as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<pmove_t>() as libc::c_ulong,
    );
    pm
        .trace = Some(
        CL_PMTrace
            as unsafe extern "C" fn(
            *mut vec_t,
            *mut vec_t,
            *mut vec_t,
            *mut vec_t,
        ) -> trace_t,
    );
    pm
        .pointcontents = Some(
        CL_PMpointcontents as unsafe extern "C" fn(*mut vec_t) -> libc::c_int,
    );
    pm_airaccelerate = atof((cl.configstrings[29 as libc::c_int as usize]).as_mut_ptr())
        as libc::c_float;
    pm.s = cl.frame.playerstate.pmove;
    frame = 0 as libc::c_int;
    loop {
        ack += 1;
        if !(ack < current) {
            break;
        }
        frame = ack & 64 as libc::c_int - 1 as libc::c_int;
        cmd = &mut *(cl.cmds).as_mut_ptr().offset(frame as isize) as *mut usercmd_t;
        pm.cmd = *cmd;
        Pmove(&mut pm);
        cl
            .predicted_origins[frame
            as usize][0 as libc::c_int
            as usize] = pm.s.origin[0 as libc::c_int as usize];
        cl
            .predicted_origins[frame
            as usize][1 as libc::c_int
            as usize] = pm.s.origin[1 as libc::c_int as usize];
        cl
            .predicted_origins[frame
            as usize][2 as libc::c_int
            as usize] = pm.s.origin[2 as libc::c_int as usize];
    }
    oldframe = ack - 2 as libc::c_int & 64 as libc::c_int - 1 as libc::c_int;
    oldz = cl.predicted_origins[oldframe as usize][2 as libc::c_int as usize]
        as libc::c_int;
    step = pm.s.origin[2 as libc::c_int as usize] as libc::c_int - oldz;
    if step > 63 as libc::c_int && step < 160 as libc::c_int
        && pm.s.pm_flags as libc::c_int & 4 as libc::c_int != 0
    {
        cl.predicted_step = (step as libc::c_double * 0.125f64) as libc::c_float;
        cl
            .predicted_step_time = (cls.realtime as libc::c_float
            - cls.frametime * 500 as libc::c_int as libc::c_float) as libc::c_uint;
    }
    cl
        .predicted_origin[0 as libc::c_int
        as usize] = (pm.s.origin[0 as libc::c_int as usize] as libc::c_int
        as libc::c_double * 0.125f64) as vec_t;
    cl
        .predicted_origin[1 as libc::c_int
        as usize] = (pm.s.origin[1 as libc::c_int as usize] as libc::c_int
        as libc::c_double * 0.125f64) as vec_t;
    cl
        .predicted_origin[2 as libc::c_int
        as usize] = (pm.s.origin[2 as libc::c_int as usize] as libc::c_int
        as libc::c_double * 0.125f64) as vec_t;
    cl
        .predicted_angles[0 as libc::c_int
        as usize] = pm.viewangles[0 as libc::c_int as usize];
    cl
        .predicted_angles[1 as libc::c_int
        as usize] = pm.viewangles[1 as libc::c_int as usize];
    cl
        .predicted_angles[2 as libc::c_int
        as usize] = pm.viewangles[2 as libc::c_int as usize];
}
