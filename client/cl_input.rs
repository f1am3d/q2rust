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
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn atoi(__nptr: *const libc::c_char) -> libc::c_int;
    static mut curtime: libc::c_int;
    fn Com_Printf(msg: *mut libc::c_char, _: ...);
    fn SZ_Init(buf: *mut sizebuf_t, data: *mut byte, length: libc::c_int);
    fn MSG_WriteByte(sb: *mut sizebuf_t, c: libc::c_int);
    fn MSG_WriteLong(sb: *mut sizebuf_t, c: libc::c_int);
    fn MSG_WriteString(sb: *mut sizebuf_t, s: *mut libc::c_char);
    fn MSG_WriteDeltaUsercmd(
        sb: *mut sizebuf_t,
        from: *mut usercmd_s,
        cmd: *mut usercmd_s,
    );
    fn Cmd_AddCommand(cmd_name: *mut libc::c_char, function: xcommand_t);
    fn Cmd_Argv(arg: libc::c_int) -> *mut libc::c_char;
    fn Cvar_Get(
        var_name: *mut libc::c_char,
        value: *mut libc::c_char,
        flags: libc::c_int,
    ) -> *mut cvar_t;
    fn Cvar_Userinfo() -> *mut libc::c_char;
    static mut userinfo_modified: qboolean;
    fn Netchan_Transmit(chan: *mut netchan_t, length: libc::c_int, data: *mut byte);
    fn COM_BlockSequenceCRCByte(
        base: *mut byte,
        length: libc::c_int,
        sequence: libc::c_int,
    ) -> byte;
    fn SCR_FinishCinematic();
    fn IN_Move(cmd: *mut usercmd_t);
    static mut anykeydown: libc::c_int;
    static mut cl: client_state_t;
    static mut cls: client_static_t;
    static mut cl_lightlevel: *mut cvar_t;
    fn CL_FixUpGender();
    static mut sys_frame_time: libc::c_uint;
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
pub type clc_ops_e = libc::c_uint;

pub const clc_stringcmd: clc_ops_e = 4;
pub const clc_userinfo: clc_ops_e = 3;
pub const clc_move: clc_ops_e = 2;
pub const clc_nop: clc_ops_e = 1;
pub const clc_bad: clc_ops_e = 0;

pub type xcommand_t = Option::<unsafe extern "C" fn() -> ()>;
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

#[derive(Copy, Clone)]
#[repr(C)]
pub struct kbutton_t {
    pub down: [libc::c_int; 2],
    pub downtime: libc::c_uint,
    pub msec: libc::c_uint,
    pub state: libc::c_int,
}

#[no_mangle]
pub static mut cl_nodelta: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut frame_msec: libc::c_uint = 0;
#[no_mangle]
pub static mut old_sys_frame_time: libc::c_uint = 0;
#[no_mangle]
pub static mut in_klook: kbutton_t = kbutton_t {
    down: [0; 2],
    downtime: 0,
    msec: 0,
    state: 0,
};
#[no_mangle]
pub static mut in_left: kbutton_t = kbutton_t {
    down: [0; 2],
    downtime: 0,
    msec: 0,
    state: 0,
};
#[no_mangle]
pub static mut in_right: kbutton_t = kbutton_t {
    down: [0; 2],
    downtime: 0,
    msec: 0,
    state: 0,
};
#[no_mangle]
pub static mut in_forward: kbutton_t = kbutton_t {
    down: [0; 2],
    downtime: 0,
    msec: 0,
    state: 0,
};
#[no_mangle]
pub static mut in_back: kbutton_t = kbutton_t {
    down: [0; 2],
    downtime: 0,
    msec: 0,
    state: 0,
};
#[no_mangle]
pub static mut in_lookup: kbutton_t = kbutton_t {
    down: [0; 2],
    downtime: 0,
    msec: 0,
    state: 0,
};
#[no_mangle]
pub static mut in_lookdown: kbutton_t = kbutton_t {
    down: [0; 2],
    downtime: 0,
    msec: 0,
    state: 0,
};
#[no_mangle]
pub static mut in_moveleft: kbutton_t = kbutton_t {
    down: [0; 2],
    downtime: 0,
    msec: 0,
    state: 0,
};
#[no_mangle]
pub static mut in_moveright: kbutton_t = kbutton_t {
    down: [0; 2],
    downtime: 0,
    msec: 0,
    state: 0,
};
#[no_mangle]
pub static mut in_strafe: kbutton_t = kbutton_t {
    down: [0; 2],
    downtime: 0,
    msec: 0,
    state: 0,
};
#[no_mangle]
pub static mut in_speed: kbutton_t = kbutton_t {
    down: [0; 2],
    downtime: 0,
    msec: 0,
    state: 0,
};
#[no_mangle]
pub static mut in_use: kbutton_t = kbutton_t {
    down: [0; 2],
    downtime: 0,
    msec: 0,
    state: 0,
};
#[no_mangle]
pub static mut in_attack: kbutton_t = kbutton_t {
    down: [0; 2],
    downtime: 0,
    msec: 0,
    state: 0,
};
#[no_mangle]
pub static mut in_up: kbutton_t = kbutton_t {
    down: [0; 2],
    downtime: 0,
    msec: 0,
    state: 0,
};
#[no_mangle]
pub static mut in_down: kbutton_t = kbutton_t {
    down: [0; 2],
    downtime: 0,
    msec: 0,
    state: 0,
};
#[no_mangle]
pub static mut in_impulse: libc::c_int = 0;

#[no_mangle]
pub unsafe extern "C" fn KeyDown(mut b: *mut kbutton_t) {
    let mut k: libc::c_int = 0;
    let mut c: *mut libc::c_char = 0 as *mut libc::c_char;
    c = Cmd_Argv(1 as libc::c_int);
    if *c.offset(0 as libc::c_int as isize) != 0 {
        k = atoi(c);
    } else {
        k = -(1 as libc::c_int);
    }
    if k == (*b).down[0 as libc::c_int as usize]
        || k == (*b).down[1 as libc::c_int as usize]
    {
        return;
    }
    if (*b).down[0 as libc::c_int as usize] == 0 {
        (*b).down[0 as libc::c_int as usize] = k;
    } else if (*b).down[1 as libc::c_int as usize] == 0 {
        (*b).down[1 as libc::c_int as usize] = k;
    } else {
        Com_Printf(
            b"Three keys down for a button!\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return;
    }
    if (*b).state & 1 as libc::c_int != 0 {
        return;
    }
    c = Cmd_Argv(2 as libc::c_int);
    (*b).downtime = atoi(c) as libc::c_uint;
    if (*b).downtime == 0 {
        (*b).downtime = sys_frame_time.wrapping_sub(100 as libc::c_int as libc::c_uint);
    }
    (*b).state |= 1 as libc::c_int + 2 as libc::c_int;
}

#[no_mangle]
pub unsafe extern "C" fn KeyUp(mut b: *mut kbutton_t) {
    let mut k: libc::c_int = 0;
    let mut c: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut uptime: libc::c_uint = 0;
    c = Cmd_Argv(1 as libc::c_int);
    if *c.offset(0 as libc::c_int as isize) != 0 {
        k = atoi(c);
    } else {
        let ref mut fresh0 = (*b).down[1 as libc::c_int as usize];
        *fresh0 = 0 as libc::c_int;
        (*b).down[0 as libc::c_int as usize] = *fresh0;
        (*b).state = 4 as libc::c_int;
        return;
    }
    if (*b).down[0 as libc::c_int as usize] == k {
        (*b).down[0 as libc::c_int as usize] = 0 as libc::c_int;
    } else if (*b).down[1 as libc::c_int as usize] == k {
        (*b).down[1 as libc::c_int as usize] = 0 as libc::c_int;
    } else {
        return;
    }
    if (*b).down[0 as libc::c_int as usize] != 0
        || (*b).down[1 as libc::c_int as usize] != 0
    {
        return;
    }
    if (*b).state & 1 as libc::c_int == 0 {
        return;
    }
    c = Cmd_Argv(2 as libc::c_int);
    uptime = atoi(c) as libc::c_uint;
    if uptime != 0 {
        let ref mut fresh1 = (*b).msec;
        *fresh1 = (*fresh1).wrapping_add(uptime.wrapping_sub((*b).downtime));
    } else {
        let ref mut fresh2 = (*b).msec;
        *fresh2 = (*fresh2).wrapping_add(10 as libc::c_int as libc::c_uint);
    }
    (*b).state &= !(1 as libc::c_int);
    (*b).state |= 4 as libc::c_int;
}

#[no_mangle]
pub unsafe extern "C" fn IN_KLookDown() {
    KeyDown(&mut in_klook);
}

#[no_mangle]
pub unsafe extern "C" fn IN_KLookUp() {
    KeyUp(&mut in_klook);
}

#[no_mangle]
pub unsafe extern "C" fn IN_UpDown() {
    KeyDown(&mut in_up);
}

#[no_mangle]
pub unsafe extern "C" fn IN_UpUp() {
    KeyUp(&mut in_up);
}

#[no_mangle]
pub unsafe extern "C" fn IN_DownDown() {
    KeyDown(&mut in_down);
}

#[no_mangle]
pub unsafe extern "C" fn IN_DownUp() {
    KeyUp(&mut in_down);
}

#[no_mangle]
pub unsafe extern "C" fn IN_LeftDown() {
    KeyDown(&mut in_left);
}

#[no_mangle]
pub unsafe extern "C" fn IN_LeftUp() {
    KeyUp(&mut in_left);
}

#[no_mangle]
pub unsafe extern "C" fn IN_RightDown() {
    KeyDown(&mut in_right);
}

#[no_mangle]
pub unsafe extern "C" fn IN_RightUp() {
    KeyUp(&mut in_right);
}

#[no_mangle]
pub unsafe extern "C" fn IN_ForwardDown() {
    KeyDown(&mut in_forward);
}

#[no_mangle]
pub unsafe extern "C" fn IN_ForwardUp() {
    KeyUp(&mut in_forward);
}

#[no_mangle]
pub unsafe extern "C" fn IN_BackDown() {
    KeyDown(&mut in_back);
}

#[no_mangle]
pub unsafe extern "C" fn IN_BackUp() {
    KeyUp(&mut in_back);
}

#[no_mangle]
pub unsafe extern "C" fn IN_LookupDown() {
    KeyDown(&mut in_lookup);
}

#[no_mangle]
pub unsafe extern "C" fn IN_LookupUp() {
    KeyUp(&mut in_lookup);
}

#[no_mangle]
pub unsafe extern "C" fn IN_LookdownDown() {
    KeyDown(&mut in_lookdown);
}

#[no_mangle]
pub unsafe extern "C" fn IN_LookdownUp() {
    KeyUp(&mut in_lookdown);
}

#[no_mangle]
pub unsafe extern "C" fn IN_MoveleftDown() {
    KeyDown(&mut in_moveleft);
}

#[no_mangle]
pub unsafe extern "C" fn IN_MoveleftUp() {
    KeyUp(&mut in_moveleft);
}

#[no_mangle]
pub unsafe extern "C" fn IN_MoverightDown() {
    KeyDown(&mut in_moveright);
}

#[no_mangle]
pub unsafe extern "C" fn IN_MoverightUp() {
    KeyUp(&mut in_moveright);
}

#[no_mangle]
pub unsafe extern "C" fn IN_SpeedDown() {
    KeyDown(&mut in_speed);
}

#[no_mangle]
pub unsafe extern "C" fn IN_SpeedUp() {
    KeyUp(&mut in_speed);
}

#[no_mangle]
pub unsafe extern "C" fn IN_StrafeDown() {
    KeyDown(&mut in_strafe);
}

#[no_mangle]
pub unsafe extern "C" fn IN_StrafeUp() {
    KeyUp(&mut in_strafe);
}

#[no_mangle]
pub unsafe extern "C" fn IN_AttackDown() {
    KeyDown(&mut in_attack);
}

#[no_mangle]
pub unsafe extern "C" fn IN_AttackUp() {
    KeyUp(&mut in_attack);
}

#[no_mangle]
pub unsafe extern "C" fn IN_UseDown() {
    KeyDown(&mut in_use);
}

#[no_mangle]
pub unsafe extern "C" fn IN_UseUp() {
    KeyUp(&mut in_use);
}

#[no_mangle]
pub unsafe extern "C" fn IN_Impulse() {
    in_impulse = atoi(Cmd_Argv(1 as libc::c_int));
}

#[no_mangle]
pub unsafe extern "C" fn CL_KeyState(mut key: *mut kbutton_t) -> libc::c_float {
    let mut val: libc::c_float = 0.;
    let mut msec: libc::c_int = 0;
    (*key).state &= 1 as libc::c_int;
    msec = (*key).msec as libc::c_int;
    (*key).msec = 0 as libc::c_int as libc::c_uint;
    if (*key).state != 0 {
        msec = (msec as libc::c_uint)
            .wrapping_add(sys_frame_time.wrapping_sub((*key).downtime)) as libc::c_int
            as libc::c_int;
        (*key).downtime = sys_frame_time;
    }
    val = msec as libc::c_float / frame_msec as libc::c_float;
    if val < 0 as libc::c_int as libc::c_float {
        val = 0 as libc::c_int as libc::c_float;
    }
    if val > 1 as libc::c_int as libc::c_float {
        val = 1 as libc::c_int as libc::c_float;
    }
    return val;
}

#[no_mangle]
pub static mut cl_upspeed: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut cl_forwardspeed: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut cl_sidespeed: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut cl_yawspeed: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut cl_pitchspeed: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut cl_run: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut cl_anglespeedkey: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;

#[no_mangle]
pub unsafe extern "C" fn CL_AdjustAngles() {
    let mut speed: libc::c_float = 0.;
    let mut up: libc::c_float = 0.;
    let mut down: libc::c_float = 0.;
    if in_speed.state & 1 as libc::c_int != 0 {
        speed = cls.frametime * (*cl_anglespeedkey).value;
    } else {
        speed = cls.frametime;
    }
    if in_strafe.state & 1 as libc::c_int == 0 {
        cl.viewangles[1 as libc::c_int as usize]
            -= speed * (*cl_yawspeed).value * CL_KeyState(&mut in_right);
        cl.viewangles[1 as libc::c_int as usize]
            += speed * (*cl_yawspeed).value * CL_KeyState(&mut in_left);
    }
    if in_klook.state & 1 as libc::c_int != 0 {
        cl.viewangles[0 as libc::c_int as usize]
            -= speed * (*cl_pitchspeed).value * CL_KeyState(&mut in_forward);
        cl.viewangles[0 as libc::c_int as usize]
            += speed * (*cl_pitchspeed).value * CL_KeyState(&mut in_back);
    }
    up = CL_KeyState(&mut in_lookup);
    down = CL_KeyState(&mut in_lookdown);
    cl.viewangles[0 as libc::c_int as usize] -= speed * (*cl_pitchspeed).value * up;
    cl.viewangles[0 as libc::c_int as usize] += speed * (*cl_pitchspeed).value * down;
}

#[no_mangle]
pub unsafe extern "C" fn CL_BaseMove(mut cmd: *mut usercmd_t) {
    CL_AdjustAngles();
    memset(
        cmd as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<usercmd_t>() as libc::c_ulong,
    );
    (*cmd)
        .angles[0 as libc::c_int
        as usize] = cl.viewangles[0 as libc::c_int as usize] as libc::c_short;
    (*cmd)
        .angles[1 as libc::c_int
        as usize] = cl.viewangles[1 as libc::c_int as usize] as libc::c_short;
    (*cmd)
        .angles[2 as libc::c_int
        as usize] = cl.viewangles[2 as libc::c_int as usize] as libc::c_short;
    if in_strafe.state & 1 as libc::c_int != 0 {
        let ref mut fresh3 = (*cmd).sidemove;
        *fresh3 = (*fresh3 as libc::c_float
            + (*cl_sidespeed).value * CL_KeyState(&mut in_right)) as libc::c_short;
        let ref mut fresh4 = (*cmd).sidemove;
        *fresh4 = (*fresh4 as libc::c_float
            - (*cl_sidespeed).value * CL_KeyState(&mut in_left)) as libc::c_short;
    }
    let ref mut fresh5 = (*cmd).sidemove;
    *fresh5 = (*fresh5 as libc::c_float
        + (*cl_sidespeed).value * CL_KeyState(&mut in_moveright)) as libc::c_short;
    let ref mut fresh6 = (*cmd).sidemove;
    *fresh6 = (*fresh6 as libc::c_float
        - (*cl_sidespeed).value * CL_KeyState(&mut in_moveleft)) as libc::c_short;
    let ref mut fresh7 = (*cmd).upmove;
    *fresh7 = (*fresh7 as libc::c_float + (*cl_upspeed).value * CL_KeyState(&mut in_up))
        as libc::c_short;
    let ref mut fresh8 = (*cmd).upmove;
    *fresh8 = (*fresh8 as libc::c_float
        - (*cl_upspeed).value * CL_KeyState(&mut in_down)) as libc::c_short;
    if in_klook.state & 1 as libc::c_int == 0 {
        let ref mut fresh9 = (*cmd).forwardmove;
        *fresh9 = (*fresh9 as libc::c_float
            + (*cl_forwardspeed).value * CL_KeyState(&mut in_forward)) as libc::c_short;
        let ref mut fresh10 = (*cmd).forwardmove;
        *fresh10 = (*fresh10 as libc::c_float
            - (*cl_forwardspeed).value * CL_KeyState(&mut in_back)) as libc::c_short;
    }
    if in_speed.state & 1 as libc::c_int ^ (*cl_run).value as libc::c_int != 0 {
        let ref mut fresh11 = (*cmd).forwardmove;
        *fresh11 = (*fresh11 as libc::c_int * 2 as libc::c_int) as libc::c_short;
        let ref mut fresh12 = (*cmd).sidemove;
        *fresh12 = (*fresh12 as libc::c_int * 2 as libc::c_int) as libc::c_short;
        let ref mut fresh13 = (*cmd).upmove;
        *fresh13 = (*fresh13 as libc::c_int * 2 as libc::c_int) as libc::c_short;
    }
}

#[no_mangle]
pub unsafe extern "C" fn CL_ClampPitch() {
    let mut pitch: libc::c_float = 0.;
    pitch = (cl.frame.playerstate.pmove.delta_angles[0 as libc::c_int as usize]
        as libc::c_int as libc::c_double
        * (360.0f64 / 65536 as libc::c_int as libc::c_double)) as libc::c_float;
    if pitch > 180 as libc::c_int as libc::c_float {
        pitch -= 360 as libc::c_int as libc::c_float;
    }
    if cl.viewangles[0 as libc::c_int as usize] + pitch
        > 89 as libc::c_int as libc::c_float
    {
        cl
            .viewangles[0 as libc::c_int
            as usize] = 89 as libc::c_int as libc::c_float - pitch;
    }
    if cl.viewangles[0 as libc::c_int as usize] + pitch
        < -(89 as libc::c_int) as libc::c_float
    {
        cl
            .viewangles[0 as libc::c_int
            as usize] = -(89 as libc::c_int) as libc::c_float - pitch;
    }
}

#[no_mangle]
pub unsafe extern "C" fn CL_FinishMove(mut cmd: *mut usercmd_t) {
    let mut ms: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    if in_attack.state & 3 as libc::c_int != 0 {
        let ref mut fresh14 = (*cmd).buttons;
        *fresh14 = (*fresh14 as libc::c_int | 1 as libc::c_int) as byte;
    }
    in_attack.state &= !(2 as libc::c_int);
    if in_use.state & 3 as libc::c_int != 0 {
        let ref mut fresh15 = (*cmd).buttons;
        *fresh15 = (*fresh15 as libc::c_int | 2 as libc::c_int) as byte;
    }
    in_use.state &= !(2 as libc::c_int);
    if anykeydown != 0
        && cls.key_dest as libc::c_uint == key_game as libc::c_int as libc::c_uint
    {
        let ref mut fresh16 = (*cmd).buttons;
        *fresh16 = (*fresh16 as libc::c_int | 128 as libc::c_int) as byte;
    }
    ms = (cls.frametime * 1000 as libc::c_int as libc::c_float) as libc::c_int;
    if ms > 250 as libc::c_int {
        ms = 100 as libc::c_int;
    }
    (*cmd).msec = ms as byte;
    CL_ClampPitch();
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        (*cmd)
            .angles[i
            as usize] = ((cl.viewangles[i as usize]
            * 65536 as libc::c_int as libc::c_float
            / 360 as libc::c_int as libc::c_float) as libc::c_int & 65535 as libc::c_int)
            as libc::c_short;
        i += 1;
    }
    (*cmd).impulse = in_impulse as byte;
    in_impulse = 0 as libc::c_int;
    (*cmd).lightlevel = (*cl_lightlevel).value as byte;
}

#[no_mangle]
pub unsafe extern "C" fn CL_CreateCmd() -> usercmd_t {
    let mut cmd: usercmd_t = usercmd_t {
        msec: 0,
        buttons: 0,
        angles: [0; 3],
        forwardmove: 0,
        sidemove: 0,
        upmove: 0,
        impulse: 0,
        lightlevel: 0,
    };
    frame_msec = sys_frame_time.wrapping_sub(old_sys_frame_time);
    if frame_msec < 1 as libc::c_int as libc::c_uint {
        frame_msec = 1 as libc::c_int as libc::c_uint;
    }
    if frame_msec > 200 as libc::c_int as libc::c_uint {
        frame_msec = 200 as libc::c_int as libc::c_uint;
    }
    CL_BaseMove(&mut cmd);
    IN_Move(&mut cmd);
    CL_FinishMove(&mut cmd);
    old_sys_frame_time = sys_frame_time;
    return cmd;
}

#[no_mangle]
pub unsafe extern "C" fn IN_CenterView() {
    cl
        .viewangles[0 as libc::c_int
        as usize] = -(cl.frame.playerstate.pmove.delta_angles[0 as libc::c_int as usize]
        as libc::c_int as libc::c_double
        * (360.0f64 / 65536 as libc::c_int as libc::c_double)) as vec_t;
}

#[no_mangle]
pub unsafe extern "C" fn CL_InitInput() {
    Cmd_AddCommand(
        b"centerview\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        Some(IN_CenterView as unsafe extern "C" fn() -> ()),
    );
    Cmd_AddCommand(
        b"+moveup\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        Some(IN_UpDown as unsafe extern "C" fn() -> ()),
    );
    Cmd_AddCommand(
        b"-moveup\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        Some(IN_UpUp as unsafe extern "C" fn() -> ()),
    );
    Cmd_AddCommand(
        b"+movedown\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        Some(IN_DownDown as unsafe extern "C" fn() -> ()),
    );
    Cmd_AddCommand(
        b"-movedown\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        Some(IN_DownUp as unsafe extern "C" fn() -> ()),
    );
    Cmd_AddCommand(
        b"+left\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        Some(IN_LeftDown as unsafe extern "C" fn() -> ()),
    );
    Cmd_AddCommand(
        b"-left\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        Some(IN_LeftUp as unsafe extern "C" fn() -> ()),
    );
    Cmd_AddCommand(
        b"+right\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        Some(IN_RightDown as unsafe extern "C" fn() -> ()),
    );
    Cmd_AddCommand(
        b"-right\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        Some(IN_RightUp as unsafe extern "C" fn() -> ()),
    );
    Cmd_AddCommand(
        b"+forward\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        Some(IN_ForwardDown as unsafe extern "C" fn() -> ()),
    );
    Cmd_AddCommand(
        b"-forward\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        Some(IN_ForwardUp as unsafe extern "C" fn() -> ()),
    );
    Cmd_AddCommand(
        b"+back\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        Some(IN_BackDown as unsafe extern "C" fn() -> ()),
    );
    Cmd_AddCommand(
        b"-back\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        Some(IN_BackUp as unsafe extern "C" fn() -> ()),
    );
    Cmd_AddCommand(
        b"+lookup\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        Some(IN_LookupDown as unsafe extern "C" fn() -> ()),
    );
    Cmd_AddCommand(
        b"-lookup\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        Some(IN_LookupUp as unsafe extern "C" fn() -> ()),
    );
    Cmd_AddCommand(
        b"+lookdown\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        Some(IN_LookdownDown as unsafe extern "C" fn() -> ()),
    );
    Cmd_AddCommand(
        b"-lookdown\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        Some(IN_LookdownUp as unsafe extern "C" fn() -> ()),
    );
    Cmd_AddCommand(
        b"+strafe\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        Some(IN_StrafeDown as unsafe extern "C" fn() -> ()),
    );
    Cmd_AddCommand(
        b"-strafe\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        Some(IN_StrafeUp as unsafe extern "C" fn() -> ()),
    );
    Cmd_AddCommand(
        b"+moveleft\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        Some(IN_MoveleftDown as unsafe extern "C" fn() -> ()),
    );
    Cmd_AddCommand(
        b"-moveleft\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        Some(IN_MoveleftUp as unsafe extern "C" fn() -> ()),
    );
    Cmd_AddCommand(
        b"+moveright\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        Some(IN_MoverightDown as unsafe extern "C" fn() -> ()),
    );
    Cmd_AddCommand(
        b"-moveright\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        Some(IN_MoverightUp as unsafe extern "C" fn() -> ()),
    );
    Cmd_AddCommand(
        b"+speed\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        Some(IN_SpeedDown as unsafe extern "C" fn() -> ()),
    );
    Cmd_AddCommand(
        b"-speed\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        Some(IN_SpeedUp as unsafe extern "C" fn() -> ()),
    );
    Cmd_AddCommand(
        b"+attack\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        Some(IN_AttackDown as unsafe extern "C" fn() -> ()),
    );
    Cmd_AddCommand(
        b"-attack\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        Some(IN_AttackUp as unsafe extern "C" fn() -> ()),
    );
    Cmd_AddCommand(
        b"+use\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        Some(IN_UseDown as unsafe extern "C" fn() -> ()),
    );
    Cmd_AddCommand(
        b"-use\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        Some(IN_UseUp as unsafe extern "C" fn() -> ()),
    );
    Cmd_AddCommand(
        b"impulse\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        Some(IN_Impulse as unsafe extern "C" fn() -> ()),
    );
    Cmd_AddCommand(
        b"+klook\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        Some(IN_KLookDown as unsafe extern "C" fn() -> ()),
    );
    Cmd_AddCommand(
        b"-klook\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        Some(IN_KLookUp as unsafe extern "C" fn() -> ()),
    );
    cl_nodelta = Cvar_Get(
        b"cl_nodelta\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
}

#[no_mangle]
pub unsafe extern "C" fn CL_SendCmd() {
    let mut buf: sizebuf_t = sizebuf_t {
        allowoverflow: false_0,
        overflowed: false_0,
        data: 0 as *const byte as *mut byte,
        maxsize: 0,
        cursize: 0,
        readcount: 0,
    };
    let mut data: [byte; 128] = [0; 128];
    let mut i: libc::c_int = 0;
    let mut cmd: *mut usercmd_t = 0 as *mut usercmd_t;
    let mut oldcmd: *mut usercmd_t = 0 as *mut usercmd_t;
    let mut nullcmd: usercmd_t = usercmd_t {
        msec: 0,
        buttons: 0,
        angles: [0; 3],
        forwardmove: 0,
        sidemove: 0,
        upmove: 0,
        impulse: 0,
        lightlevel: 0,
    };
    let mut checksumIndex: libc::c_int = 0;
    i = cls.netchan.outgoing_sequence & 64 as libc::c_int - 1 as libc::c_int;
    cmd = &mut *(cl.cmds).as_mut_ptr().offset(i as isize) as *mut usercmd_t;
    cl.cmd_time[i as usize] = cls.realtime;
    *cmd = CL_CreateCmd();
    cl.cmd = *cmd;
    if cls.state as libc::c_uint == ca_disconnected as libc::c_int as libc::c_uint
        || cls.state as libc::c_uint == ca_connecting as libc::c_int as libc::c_uint
    {
        return;
    }
    if cls.state as libc::c_uint == ca_connected as libc::c_int as libc::c_uint {
        if cls.netchan.message.cursize != 0
            || curtime - cls.netchan.last_sent > 1000 as libc::c_int
        {
            Netchan_Transmit(&mut cls.netchan, 0 as libc::c_int, buf.data);
        }
        return;
    }
    if userinfo_modified as u64 != 0 {
        CL_FixUpGender();
        userinfo_modified = false_0;
        MSG_WriteByte(&mut cls.netchan.message, clc_userinfo as libc::c_int);
        MSG_WriteString(&mut cls.netchan.message, Cvar_Userinfo());
    }
    SZ_Init(
        &mut buf,
        data.as_mut_ptr(),
        ::std::mem::size_of::<[byte; 128]>() as libc::c_ulong as libc::c_int,
    );
    if (*cmd).buttons as libc::c_int != 0 && cl.cinematictime > 0 as libc::c_int
        && cl.attractloop as u64 == 0
        && cls.realtime - cl.cinematictime > 1000 as libc::c_int
    {
        SCR_FinishCinematic();
    }
    MSG_WriteByte(&mut buf, clc_move as libc::c_int);
    checksumIndex = buf.cursize;
    MSG_WriteByte(&mut buf, 0 as libc::c_int);
    if (*cl_nodelta).value != 0. || cl.frame.valid as u64 == 0
        || cls.demowaiting as libc::c_uint != 0
    {
        MSG_WriteLong(&mut buf, -(1 as libc::c_int));
    } else {
        MSG_WriteLong(&mut buf, cl.frame.serverframe);
    }
    i = cls.netchan.outgoing_sequence - 2 as libc::c_int
        & 64 as libc::c_int - 1 as libc::c_int;
    cmd = &mut *(cl.cmds).as_mut_ptr().offset(i as isize) as *mut usercmd_t;
    memset(
        &mut nullcmd as *mut usercmd_t as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<usercmd_t>() as libc::c_ulong,
    );
    MSG_WriteDeltaUsercmd(&mut buf, &mut nullcmd, cmd);
    oldcmd = cmd;
    i = cls.netchan.outgoing_sequence - 1 as libc::c_int
        & 64 as libc::c_int - 1 as libc::c_int;
    cmd = &mut *(cl.cmds).as_mut_ptr().offset(i as isize) as *mut usercmd_t;
    MSG_WriteDeltaUsercmd(&mut buf, oldcmd, cmd);
    oldcmd = cmd;
    i = cls.netchan.outgoing_sequence & 64 as libc::c_int - 1 as libc::c_int;
    cmd = &mut *(cl.cmds).as_mut_ptr().offset(i as isize) as *mut usercmd_t;
    MSG_WriteDeltaUsercmd(&mut buf, oldcmd, cmd);
    *(buf.data)
        .offset(
            checksumIndex as isize,
        ) = COM_BlockSequenceCRCByte(
        (buf.data).offset(checksumIndex as isize).offset(1 as libc::c_int as isize),
        buf.cursize - checksumIndex - 1 as libc::c_int,
        cls.netchan.outgoing_sequence,
    );
    Netchan_Transmit(&mut cls.netchan, buf.cursize, buf.data);
}
