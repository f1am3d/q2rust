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
    static mut sensitivity: *mut cvar_t;
    static mut lookstrafe: *mut cvar_t;
    static mut m_side: *mut cvar_t;
    static mut m_yaw: *mut cvar_t;
    static mut in_strafe: kbutton_t;
    static mut m_pitch: *mut cvar_t;
    static mut m_forward: *mut cvar_t;
    static mut in_speed: kbutton_t;
    static mut cl_run: *mut cvar_t;
    static mut cl: client_state_t;
    static mut cls: client_static_t;
    fn Key_Event(key: libc::c_int, down: qboolean, time: libc::c_uint);
    static mut freelook: *mut cvar_t;
    static mut lookspring: *mut cvar_t;
    fn IN_CenterView();
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn Com_Printf(msg: *mut libc::c_char, _: ...);
    fn Cmd_AddCommand(cmd_name: *mut libc::c_char, function: xcommand_t);
    fn Cvar_Get(
        var_name: *mut libc::c_char,
        value: *mut libc::c_char,
        flags: libc::c_int,
    ) -> *mut cvar_t;
    fn Cvar_VariableValue(var_name: *mut libc::c_char) -> libc::c_float;
    static mut ActiveApp: qboolean;
    static mut sys_msg_time: libc::c_uint;
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
pub const key_menu: keydest_t = 3;
pub type keydest_t = libc::c_uint;
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
pub type dltype_t = libc::c_uint;
pub const dl_single: dltype_t = 4;
pub const dl_skin: dltype_t = 3;
pub const dl_sound: dltype_t = 2;
pub const dl_model: dltype_t = 1;
pub const dl_none: dltype_t = 0;
pub type connstate_t = libc::c_uint;
pub const ca_active: connstate_t = 4;
pub const ca_connected: connstate_t = 3;
pub const ca_connecting: connstate_t = 2;
pub const ca_disconnected: connstate_t = 1;
pub const ca_uninitialized: connstate_t = 0;
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
pub struct kbutton_t {
    pub down: [libc::c_int; 2],
    pub downtime: libc::c_uint,
    pub msec: libc::c_uint,
    pub state: libc::c_int,
}
#[no_mangle]
pub static mut dwAxisFlags: [libc::c_int; 6] = [0; 6];
#[no_mangle]
pub static mut dwAxisMap: [libc::c_int; 6] = [0; 6];
#[no_mangle]
pub static mut dwControlMap: [libc::c_int; 6] = [0; 6];
#[no_mangle]
pub static mut pdwRawValue: [libc::c_int; 6] = [0; 6];
#[no_mangle]
pub static mut in_mouse: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut in_joystick: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut joy_name: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut joy_advanced: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut joy_advaxisx: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut joy_advaxisy: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut joy_advaxisz: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut joy_advaxisr: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut joy_advaxisu: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut joy_advaxisv: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut joy_forwardthreshold: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut joy_sidethreshold: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut joy_pitchthreshold: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut joy_yawthreshold: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut joy_forwardsensitivity: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut joy_sidesensitivity: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut joy_pitchsensitivity: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut joy_yawsensitivity: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut joy_upthreshold: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut joy_upsensitivity: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut joy_avail: qboolean = false_0;
#[no_mangle]
pub static mut joy_advancedinit: qboolean = false_0;
#[no_mangle]
pub static mut joy_haspov: qboolean = false_0;
#[no_mangle]
pub static mut joy_oldbuttonstate: libc::c_int = 0;
#[no_mangle]
pub static mut joy_id: libc::c_int = 0;
#[no_mangle]
pub static mut joy_flags: libc::c_int = 0;
#[no_mangle]
pub static mut joy_numbuttons: libc::c_int = 0;
#[no_mangle]
pub static mut in_appactive: qboolean = false_0;
#[no_mangle]
pub static mut m_filter: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut mlooking: qboolean = false_0;
#[no_mangle]
pub unsafe extern "C" fn IN_MLookDown() {
    mlooking = true_0;
}
#[no_mangle]
pub unsafe extern "C" fn IN_MLookUp() {
    mlooking = false_0;
    if (*freelook).value == 0. && (*lookspring).value != 0. {
        IN_CenterView();
    }
}
#[no_mangle]
pub static mut mouse_buttons: libc::c_int = 0;
#[no_mangle]
pub static mut mouse_oldbuttonstate: libc::c_int = 0;
#[no_mangle]
pub static mut current_pos: libc::c_int = 0;
#[no_mangle]
pub static mut mouse_x: libc::c_int = 0;
#[no_mangle]
pub static mut mouse_y: libc::c_int = 0;
#[no_mangle]
pub static mut old_mouse_x: libc::c_int = 0;
#[no_mangle]
pub static mut old_mouse_y: libc::c_int = 0;
#[no_mangle]
pub static mut mx_accum: libc::c_int = 0;
#[no_mangle]
pub static mut my_accum: libc::c_int = 0;
#[no_mangle]
pub static mut old_x: libc::c_int = 0;
#[no_mangle]
pub static mut old_y: libc::c_int = 0;
#[no_mangle]
pub static mut mouseactive: qboolean = false_0;
#[no_mangle]
pub static mut restore_spi: qboolean = false_0;
#[no_mangle]
pub static mut mouseinitialized: qboolean = false_0;
#[no_mangle]
pub static mut originalmouseparms: [libc::c_int; 3] = [0; 3];
#[no_mangle]
pub static mut newmouseparms: [libc::c_int; 3] = [
    0 as libc::c_int,
    0 as libc::c_int,
    1 as libc::c_int,
];
#[no_mangle]
pub static mut mouseparmsvalid: qboolean = false_0;
#[no_mangle]
pub static mut window_center_x: libc::c_int = 0;
#[no_mangle]
pub static mut window_center_y: libc::c_int = 0;
#[no_mangle]
pub static mut window_rect: libc::c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn IN_ActivateMouse() {
    let mut width: libc::c_int = 0;
    let mut height: libc::c_int = 0;
    if mouseinitialized as u64 == 0 {
        return;
    }
    if (*in_mouse).value == 0. {
        mouseactive = false_0;
        return;
    }
    if mouseactive as u64 != 0 {
        return;
    }
    mouseactive = true_0;
    SetCursorPos(window_center_x, window_center_y);
    old_x = window_center_x;
    old_y = window_center_y;
}
#[no_mangle]
pub unsafe extern "C" fn IN_DeactivateMouse() {
    if mouseinitialized as u64 == 0 {
        return;
    }
    if mouseactive as u64 == 0 {
        return;
    }
    mouseactive = false_0;
    ClipCursor(0 as *mut libc::c_void);
    ReleaseCapture();
}
#[no_mangle]
pub unsafe extern "C" fn IN_StartupMouse() {
    let mut cv: *mut cvar_t = 0 as *mut cvar_t;
    cv = Cvar_Get(
        b"in_initmouse\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        8 as libc::c_int,
    );
    if (*cv).value == 0. {
        return;
    }
    mouseinitialized = true_0;
    mouse_buttons = 3 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn IN_MouseEvent(mut mstate: libc::c_int) {
    let mut i: libc::c_int = 0;
    if mouseinitialized as u64 == 0 {
        return;
    }
    i = 0 as libc::c_int;
    while i < mouse_buttons {
        if mstate & (1 as libc::c_int) << i != 0
            && mouse_oldbuttonstate & (1 as libc::c_int) << i == 0
        {
            Key_Event(200 as libc::c_int + i, true_0, sys_msg_time);
        }
        if mstate & (1 as libc::c_int) << i == 0
            && mouse_oldbuttonstate & (1 as libc::c_int) << i != 0
        {
            Key_Event(200 as libc::c_int + i, false_0, sys_msg_time);
        }
        i += 1;
    }
    mouse_oldbuttonstate = mstate;
}
#[no_mangle]
pub static mut v_centermove: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut v_centerspeed: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub unsafe extern "C" fn IN_Init() {
    m_filter = Cvar_Get(
        b"m_filter\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    in_mouse = Cvar_Get(
        b"in_mouse\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        1 as libc::c_int,
    );
    in_joystick = Cvar_Get(
        b"in_joystick\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        1 as libc::c_int,
    );
    joy_name = Cvar_Get(
        b"joy_name\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"joystick\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    joy_advanced = Cvar_Get(
        b"joy_advanced\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    joy_advaxisx = Cvar_Get(
        b"joy_advaxisx\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    joy_advaxisy = Cvar_Get(
        b"joy_advaxisy\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    joy_advaxisz = Cvar_Get(
        b"joy_advaxisz\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    joy_advaxisr = Cvar_Get(
        b"joy_advaxisr\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    joy_advaxisu = Cvar_Get(
        b"joy_advaxisu\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    joy_advaxisv = Cvar_Get(
        b"joy_advaxisv\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    joy_forwardthreshold = Cvar_Get(
        b"joy_forwardthreshold\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        b"0.15\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    joy_sidethreshold = Cvar_Get(
        b"joy_sidethreshold\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"0.15\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    joy_upthreshold = Cvar_Get(
        b"joy_upthreshold\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"0.15\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    joy_pitchthreshold = Cvar_Get(
        b"joy_pitchthreshold\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"0.15\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    joy_yawthreshold = Cvar_Get(
        b"joy_yawthreshold\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"0.15\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    joy_forwardsensitivity = Cvar_Get(
        b"joy_forwardsensitivity\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        b"-1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    joy_sidesensitivity = Cvar_Get(
        b"joy_sidesensitivity\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        b"-1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    joy_upsensitivity = Cvar_Get(
        b"joy_upsensitivity\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"-1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    joy_pitchsensitivity = Cvar_Get(
        b"joy_pitchsensitivity\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        b"1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    joy_yawsensitivity = Cvar_Get(
        b"joy_yawsensitivity\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"-1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    v_centermove = Cvar_Get(
        b"v_centermove\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"0.15\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    v_centerspeed = Cvar_Get(
        b"v_centerspeed\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"500\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    Cmd_AddCommand(
        b"+mlook\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        Some(IN_MLookDown as unsafe extern "C" fn() -> ()),
    );
    Cmd_AddCommand(
        b"-mlook\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        Some(IN_MLookUp as unsafe extern "C" fn() -> ()),
    );
    Cmd_AddCommand(
        b"joy_advancedupdate\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        Some(Joy_AdvancedUpdate_f as unsafe extern "C" fn() -> ()),
    );
    IN_StartupMouse();
    IN_StartupJoystick();
}
#[no_mangle]
pub unsafe extern "C" fn IN_Shutdown() {
    IN_DeactivateMouse();
}
#[no_mangle]
pub unsafe extern "C" fn IN_Activate(mut active: qboolean) {
    in_appactive = active;
    mouseactive = (active as u64 == 0) as libc::c_int as qboolean;
}
#[no_mangle]
pub unsafe extern "C" fn IN_Frame() {
    if mouseinitialized as u64 == 0 {
        return;
    }
    if in_mouse.is_null() || in_appactive as u64 == 0 {
        IN_DeactivateMouse();
        return;
    }
    if cl.refresh_prepped as u64 == 0
        || cls.key_dest as libc::c_uint == key_console as libc::c_int as libc::c_uint
        || cls.key_dest as libc::c_uint == key_menu as libc::c_int as libc::c_uint
    {
        if Cvar_VariableValue(
            b"vid_fullscreen\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) == 0 as libc::c_int as libc::c_float
        {
            IN_DeactivateMouse();
            return;
        }
    }
    IN_ActivateMouse();
}
#[no_mangle]
pub unsafe extern "C" fn IN_Move(mut cmd: *mut usercmd_t) {
    IN_MouseMove(cmd);
    if ActiveApp as u64 != 0 {
        IN_JoyMove(cmd);
    }
}
#[no_mangle]
pub unsafe extern "C" fn IN_ClearStates() {
    mx_accum = 0 as libc::c_int;
    my_accum = 0 as libc::c_int;
    mouse_oldbuttonstate = 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn RawValuePointer(mut axis: libc::c_int) -> libc::c_int {
    match axis {
        0 | 1 | 2 | 3 | 4 | 5 | _ => {}
    }
    panic!("Reached end of non-void function without returning");
}
#[no_mangle]
pub unsafe extern "C" fn IN_JoyMove(mut cmd: *mut usercmd_t) {
    let mut speed: libc::c_float = 0.;
    let mut aspeed: libc::c_float = 0.;
    let mut fAxisValue: libc::c_float = 0.;
    let mut i: libc::c_int = 0;
    if joy_advancedinit as libc::c_uint != true_0 as libc::c_int as libc::c_uint {
        Joy_AdvancedUpdate_f();
        joy_advancedinit = true_0;
    }
    if joy_avail as u64 == 0 || (*in_joystick).value == 0. {
        return;
    }
    if IN_ReadJoystick() as libc::c_uint != true_0 as libc::c_int as libc::c_uint {
        return;
    }
    if in_speed.state & 1 as libc::c_int ^ (*cl_run).value as libc::c_int != 0 {
        speed = 2 as libc::c_int as libc::c_float;
    } else {
        speed = 1 as libc::c_int as libc::c_float;
    }
    aspeed = speed * cls.frametime;
    i = 0 as libc::c_int;
    while i < 6 as libc::c_int {
        fAxisValue = (fAxisValue as libc::c_double - 32768.0f64) as libc::c_float;
        fAxisValue = (fAxisValue as libc::c_double / 32768.0f64) as libc::c_float;
        i += 1;
    }
}
