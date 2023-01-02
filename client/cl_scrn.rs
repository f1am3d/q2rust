#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(extern_types, register_tool)]

extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type model_s;
    pub type image_s;
    pub type sfx_s;
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn atof(__nptr: *const libc::c_char) -> libc::c_double;
    fn atoi(__nptr: *const libc::c_char) -> libc::c_int;
    fn COM_Parse(data_p: *mut *mut libc::c_char) -> *mut libc::c_char;
    fn Com_sprintf(
        dest: *mut libc::c_char,
        size: libc::c_int,
        fmt: *mut libc::c_char,
        _: ...
    );
    fn va(format: *mut libc::c_char, _: ...) -> *mut libc::c_char;
    fn Sys_Milliseconds() -> libc::c_int;
    fn Com_Printf(msg: *mut libc::c_char, _: ...);
    fn Cmd_AddCommand(cmd_name: *mut libc::c_char, function: xcommand_t);
    fn Cmd_Argc() -> libc::c_int;
    fn Cmd_Argv(arg: libc::c_int) -> *mut libc::c_char;
    fn Cvar_Get(
        var_name: *mut libc::c_char,
        value: *mut libc::c_char,
        flags: libc::c_int,
    ) -> *mut cvar_t;
    fn Cvar_Set(var_name: *mut libc::c_char, value: *mut libc::c_char) -> *mut cvar_t;
    fn Cvar_SetValue(var_name: *mut libc::c_char, value: libc::c_float);
    fn Com_Error(code: libc::c_int, fmt: *mut libc::c_char, _: ...);
    static mut developer: *mut cvar_t;
    static mut cl: client_state_t;
    static mut cls: client_static_t;
    static mut re: refexport_t;
    static mut viddef: viddef_t;
    fn M_Draw();
    fn Con_DrawNotify();
    fn Con_DrawConsole(frac: libc::c_float);
    fn Con_CheckResize();
    static mut cl_paused: *mut cvar_t;
    fn CL_DrawInventory();
    fn DrawAltString(x: libc::c_int, y: libc::c_int, s: *mut libc::c_char);
    fn DrawString(x: libc::c_int, y: libc::c_int, s: *mut libc::c_char);
    fn V_RenderView(stereo_separation: libc::c_float);
    fn SCR_DrawCinematic() -> qboolean;
    static mut cl_stereo_separation: *mut cvar_t;
    static mut cl_stereo: *mut cvar_t;
    static mut con: console_t;
    fn CDAudio_Stop();
    fn S_StopAllSounds();
    fn Con_ClearNotify();
    static mut crosshair: *mut cvar_t;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
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
pub struct graphsamp_t {
    pub value: libc::c_float,
    pub color: libc::c_int,
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
pub struct particle_t {
    pub origin: vec3_t,
    pub color: libc::c_int,
    pub alpha: libc::c_float,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct dlight_t {
    pub origin: vec3_t,
    pub color: vec3_t,
    pub intensity: libc::c_float,
}

pub type entity_t = entity_s;

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

#[derive(Copy, Clone)]
#[repr(C)]
pub struct lightstyle_t {
    pub rgb: [libc::c_float; 3],
    pub white: libc::c_float,
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

pub type keydest_t = libc::c_uint;

pub const key_menu: keydest_t = 3;
pub const key_message: keydest_t = 2;
pub const key_console: keydest_t = 1;
pub const key_game: keydest_t = 0;

pub type connstate_t = libc::c_uint;

pub const ca_active: connstate_t = 4;
pub const ca_connected: connstate_t = 3;
pub const ca_connecting: connstate_t = 2;
pub const ca_disconnected: connstate_t = 1;
pub const ca_uninitialized: connstate_t = 0;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct refexport_t {
    pub api_version: libc::c_int,
    pub Init: Option::<
        unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> qboolean,
    >,
    pub Shutdown: Option::<unsafe extern "C" fn() -> ()>,
    pub BeginRegistration: Option::<unsafe extern "C" fn(*mut libc::c_char) -> ()>,
    pub RegisterModel: Option::<unsafe extern "C" fn(*mut libc::c_char) -> *mut model_s>,
    pub RegisterSkin: Option::<unsafe extern "C" fn(*mut libc::c_char) -> *mut image_s>,
    pub RegisterPic: Option::<unsafe extern "C" fn(*mut libc::c_char) -> *mut image_s>,
    pub SetSky: Option::<
        unsafe extern "C" fn(*mut libc::c_char, libc::c_float, *mut vec_t) -> (),
    >,
    pub EndRegistration: Option::<unsafe extern "C" fn() -> ()>,
    pub RenderFrame: Option::<unsafe extern "C" fn(*mut refdef_t) -> ()>,
    pub DrawGetPicSize: Option::<
        unsafe extern "C" fn(*mut libc::c_int, *mut libc::c_int, *mut libc::c_char) -> (),
    >,
    pub DrawPic: Option::<
        unsafe extern "C" fn(libc::c_int, libc::c_int, *mut libc::c_char) -> (),
    >,
    pub DrawStretchPic: Option::<
        unsafe extern "C" fn(
            libc::c_int,
            libc::c_int,
            libc::c_int,
            libc::c_int,
            *mut libc::c_char,
        ) -> (),
    >,
    pub DrawChar: Option::<
        unsafe extern "C" fn(libc::c_int, libc::c_int, libc::c_int) -> (),
    >,
    pub DrawTileClear: Option::<
        unsafe extern "C" fn(
            libc::c_int,
            libc::c_int,
            libc::c_int,
            libc::c_int,
            *mut libc::c_char,
        ) -> (),
    >,
    pub DrawFill: Option::<
        unsafe extern "C" fn(
            libc::c_int,
            libc::c_int,
            libc::c_int,
            libc::c_int,
            libc::c_int,
        ) -> (),
    >,
    pub DrawFadeScreen: Option::<unsafe extern "C" fn() -> ()>,
    pub DrawStretchRaw: Option::<
        unsafe extern "C" fn(
            libc::c_int,
            libc::c_int,
            libc::c_int,
            libc::c_int,
            libc::c_int,
            libc::c_int,
            *mut byte,
        ) -> (),
    >,
    pub CinematicSetPalette: Option::<unsafe extern "C" fn(*const libc::c_uchar) -> ()>,
    pub BeginFrame: Option::<unsafe extern "C" fn(libc::c_float) -> ()>,
    pub EndFrame: Option::<unsafe extern "C" fn() -> ()>,
    pub AppActivate: Option::<unsafe extern "C" fn(qboolean) -> ()>,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct viddef_t {
    pub width: libc::c_int,
    pub height: libc::c_int,
}

pub type vrect_t = vrect_s;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct vrect_s {
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub width: libc::c_int,
    pub height: libc::c_int,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct dirty_t {
    pub x1: libc::c_int,
    pub y1: libc::c_int,
    pub x2: libc::c_int,
    pub y2: libc::c_int,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct console_t {
    pub initialized: qboolean,
    pub text: [libc::c_char; 32768],
    pub current: libc::c_int,
    pub x: libc::c_int,
    pub display: libc::c_int,
    pub ormask: libc::c_int,
    pub linewidth: libc::c_int,
    pub totallines: libc::c_int,
    pub cursorspeed: libc::c_float,
    pub vislines: libc::c_int,
    pub times: [libc::c_float; 4],
}

#[no_mangle]
pub static mut scr_con_current: libc::c_float = 0.;
#[no_mangle]
pub static mut scr_conlines: libc::c_float = 0.;
#[no_mangle]
pub static mut scr_initialized: qboolean = false_0;
#[no_mangle]
pub static mut scr_draw_loading: libc::c_int = 0;
#[no_mangle]
pub static mut scr_vrect: vrect_t = vrect_t {
    x: 0,
    y: 0,
    width: 0,
    height: 0,
};
#[no_mangle]
pub static mut scr_viewsize: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut scr_conspeed: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut scr_centertime: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut scr_showturtle: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut scr_showpause: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut scr_printspeed: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut scr_netgraph: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut scr_timegraph: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut scr_debuggraph: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut scr_graphheight: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut scr_graphscale: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut scr_graphshift: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut scr_drawall: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut scr_dirty: dirty_t = dirty_t {
    x1: 0,
    y1: 0,
    x2: 0,
    y2: 0,
};
#[no_mangle]
pub static mut scr_old_dirty: [dirty_t; 2] = [dirty_t {
    x1: 0,
    y1: 0,
    x2: 0,
    y2: 0,
}; 2];
#[no_mangle]
pub static mut crosshair_pic: [libc::c_char; 64] = [0; 64];
#[no_mangle]
pub static mut crosshair_width: libc::c_int = 0;
#[no_mangle]
pub static mut crosshair_height: libc::c_int = 0;

#[no_mangle]
pub unsafe extern "C" fn CL_AddNetgraph() {
    let mut i: libc::c_int = 0;
    let mut in_0: libc::c_int = 0;
    let mut ping: libc::c_int = 0;
    if (*scr_debuggraph).value != 0. || (*scr_timegraph).value != 0. {
        return;
    }
    i = 0 as libc::c_int;
    while i < cls.netchan.dropped {
        SCR_DebugGraph(30 as libc::c_int as libc::c_float, 0x40 as libc::c_int);
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < cl.surpressCount {
        SCR_DebugGraph(30 as libc::c_int as libc::c_float, 0xdf as libc::c_int);
        i += 1;
    }
    in_0 = cls.netchan.incoming_acknowledged & 64 as libc::c_int - 1 as libc::c_int;
    ping = cls.realtime - cl.cmd_time[in_0 as usize];
    ping /= 30 as libc::c_int;
    if ping > 30 as libc::c_int {
        ping = 30 as libc::c_int;
    }
    SCR_DebugGraph(ping as libc::c_float, 0xd0 as libc::c_int);
}

static mut current: libc::c_int = 0;
static mut values: [graphsamp_t; 1024] = [graphsamp_t { value: 0., color: 0 }; 1024];

#[no_mangle]
pub unsafe extern "C" fn SCR_DebugGraph(
    mut value: libc::c_float,
    mut color: libc::c_int,
) {
    values[(current & 1023 as libc::c_int) as usize].value = value;
    values[(current & 1023 as libc::c_int) as usize].color = color;
    current += 1;
}

#[no_mangle]
pub unsafe extern "C" fn SCR_DrawDebugGraph() {
    let mut a: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut w: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut h: libc::c_int = 0;
    let mut v: libc::c_float = 0.;
    let mut color: libc::c_int = 0;
    w = scr_vrect.width;
    x = scr_vrect.x;
    y = scr_vrect.y + scr_vrect.height;
    (re.DrawFill)
        .expect(
            "non-null function pointer",
        )(
        x,
        (y as libc::c_float - (*scr_graphheight).value) as libc::c_int,
        w,
        (*scr_graphheight).value as libc::c_int,
        8 as libc::c_int,
    );
    a = 0 as libc::c_int;
    while a < w {
        i = current - 1 as libc::c_int - a + 1024 as libc::c_int & 1023 as libc::c_int;
        v = values[i as usize].value;
        color = values[i as usize].color;
        v = v * (*scr_graphscale).value + (*scr_graphshift).value;
        if v < 0 as libc::c_int as libc::c_float {
            v
                += (*scr_graphheight).value
                * (1 as libc::c_int + (-v / (*scr_graphheight).value) as libc::c_int)
                as libc::c_float;
        }
        h = v as libc::c_int % (*scr_graphheight).value as libc::c_int;
        (re.DrawFill)
            .expect(
                "non-null function pointer",
            )(x + w - 1 as libc::c_int - a, y - h, 1 as libc::c_int, h, color);
        a += 1;
    }
}

#[no_mangle]
pub static mut scr_centerstring: [libc::c_char; 1024] = [0; 1024];
#[no_mangle]
pub static mut scr_centertime_start: libc::c_float = 0.;
#[no_mangle]
pub static mut scr_centertime_off: libc::c_float = 0.;
#[no_mangle]
pub static mut scr_center_lines: libc::c_int = 0;
#[no_mangle]
pub static mut scr_erase_center: libc::c_int = 0;

#[no_mangle]
pub unsafe extern "C" fn SCR_CenterPrint(mut str: *mut libc::c_char) {
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut line: [libc::c_char; 64] = [0; 64];
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    strncpy(
        scr_centerstring.as_mut_ptr(),
        str,
        (::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
    );
    scr_centertime_off = (*scr_centertime).value;
    scr_centertime_start = cl.time as libc::c_float;
    scr_center_lines = 1 as libc::c_int;
    s = str;
    while *s != 0 {
        if *s as libc::c_int == '\n' as i32 {
            scr_center_lines += 1;
        }
        s = s.offset(1);
    }
    Com_Printf(
        b"\n\n\x1D\x1E\x1E\x1E\x1E\x1E\x1E\x1E\x1E\x1E\x1E\x1E\x1E\x1E\x1E\x1E\x1E\x1E\x1E\x1E\x1E\x1E\x1E\x1E\x1E\x1E\x1E\x1E\x1E\x1E\x1E\x1E\x1E\x1E\x1E\x1E\x1F\n\n\0"
            as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    s = str;
    loop {
        l = 0 as libc::c_int;
        while l < 40 as libc::c_int {
            if *s.offset(l as isize) as libc::c_int == '\n' as i32
                || *s.offset(l as isize) == 0
            {
                break;
            }
            l += 1;
        }
        i = 0 as libc::c_int;
        while i < (40 as libc::c_int - l) / 2 as libc::c_int {
            line[i as usize] = ' ' as i32 as libc::c_char;
            i += 1;
        }
        j = 0 as libc::c_int;
        while j < l {
            let fresh0 = i;
            i = i + 1;
            line[fresh0 as usize] = *s.offset(j as isize);
            j += 1;
        }
        line[i as usize] = '\n' as i32 as libc::c_char;
        line[(i + 1 as libc::c_int) as usize] = 0 as libc::c_int as libc::c_char;
        Com_Printf(
            b"%s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            line.as_mut_ptr(),
        );
        while *s as libc::c_int != 0 && *s as libc::c_int != '\n' as i32 {
            s = s.offset(1);
        }
        if *s == 0 {
            break;
        }
        s = s.offset(1);
    }
    Com_Printf(
        b"\n\n\x1D\x1E\x1E\x1E\x1E\x1E\x1E\x1E\x1E\x1E\x1E\x1E\x1E\x1E\x1E\x1E\x1E\x1E\x1E\x1E\x1E\x1E\x1E\x1E\x1E\x1E\x1E\x1E\x1E\x1E\x1E\x1E\x1E\x1E\x1E\x1E\x1F\n\n\0"
            as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    Con_ClearNotify();
}

#[no_mangle]
pub unsafe extern "C" fn SCR_DrawCenterString() {
    let mut start: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut l: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut remaining: libc::c_int = 0;
    remaining = 9999 as libc::c_int;
    scr_erase_center = 0 as libc::c_int;
    start = scr_centerstring.as_mut_ptr();
    if scr_center_lines <= 4 as libc::c_int {
        y = (viddef.height as libc::c_double * 0.35f64) as libc::c_int;
    } else {
        y = 48 as libc::c_int;
    }
    loop {
        l = 0 as libc::c_int;
        while l < 40 as libc::c_int {
            if *start.offset(l as isize) as libc::c_int == '\n' as i32
                || *start.offset(l as isize) == 0
            {
                break;
            }
            l += 1;
        }
        x = (viddef.width - l * 8 as libc::c_int) / 2 as libc::c_int;
        SCR_AddDirtyPoint(x, y);
        j = 0 as libc::c_int;
        while j < l {
            (re.DrawChar)
                .expect(
                    "non-null function pointer",
                )(x, y, *start.offset(j as isize) as libc::c_int);
            let fresh1 = remaining;
            remaining = remaining - 1;
            if fresh1 == 0 {
                return;
            }
            j += 1;
            x += 8 as libc::c_int;
        }
        SCR_AddDirtyPoint(x, y + 8 as libc::c_int);
        y += 8 as libc::c_int;
        while *start as libc::c_int != 0 && *start as libc::c_int != '\n' as i32 {
            start = start.offset(1);
        }
        if *start == 0 {
            break;
        }
        start = start.offset(1);
    };
}

#[no_mangle]
pub unsafe extern "C" fn SCR_CheckDrawCenterString() {
    scr_centertime_off -= cls.frametime;
    if scr_centertime_off <= 0 as libc::c_int as libc::c_float {
        return;
    }
    SCR_DrawCenterString();
}

unsafe extern "C" fn SCR_CalcVrect() {
    let mut size: libc::c_int = 0;
    if (*scr_viewsize).value < 40 as libc::c_int as libc::c_float {
        Cvar_Set(
            b"viewsize\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"40\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
    if (*scr_viewsize).value > 100 as libc::c_int as libc::c_float {
        Cvar_Set(
            b"viewsize\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"100\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
    size = (*scr_viewsize).value as libc::c_int;
    scr_vrect.width = viddef.width * size / 100 as libc::c_int;
    scr_vrect.width &= !(7 as libc::c_int);
    scr_vrect.height = viddef.height * size / 100 as libc::c_int;
    scr_vrect.height &= !(1 as libc::c_int);
    scr_vrect.x = (viddef.width - scr_vrect.width) / 2 as libc::c_int;
    scr_vrect.y = (viddef.height - scr_vrect.height) / 2 as libc::c_int;
}

#[no_mangle]
pub unsafe extern "C" fn SCR_SizeUp_f() {
    Cvar_SetValue(
        b"viewsize\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        (*scr_viewsize).value + 10 as libc::c_int as libc::c_float,
    );
}

#[no_mangle]
pub unsafe extern "C" fn SCR_SizeDown_f() {
    Cvar_SetValue(
        b"viewsize\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        (*scr_viewsize).value - 10 as libc::c_int as libc::c_float,
    );
}

#[no_mangle]
pub unsafe extern "C" fn SCR_Sky_f() {
    let mut rotate: libc::c_float = 0.;
    let mut axis: vec3_t = [0.; 3];
    if Cmd_Argc() < 2 as libc::c_int {
        Com_Printf(
            b"Usage: sky <basename> <rotate> <axis x y z>\n\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        );
        return;
    }
    if Cmd_Argc() > 2 as libc::c_int {
        rotate = atof(Cmd_Argv(2 as libc::c_int)) as libc::c_float;
    } else {
        rotate = 0 as libc::c_int as libc::c_float;
    }
    if Cmd_Argc() == 6 as libc::c_int {
        axis[0 as libc::c_int as usize] = atof(Cmd_Argv(3 as libc::c_int)) as vec_t;
        axis[1 as libc::c_int as usize] = atof(Cmd_Argv(4 as libc::c_int)) as vec_t;
        axis[2 as libc::c_int as usize] = atof(Cmd_Argv(5 as libc::c_int)) as vec_t;
    } else {
        axis[0 as libc::c_int as usize] = 0 as libc::c_int as vec_t;
        axis[1 as libc::c_int as usize] = 0 as libc::c_int as vec_t;
        axis[2 as libc::c_int as usize] = 1 as libc::c_int as vec_t;
    }
    (re.SetSky)
        .expect(
            "non-null function pointer",
        )(Cmd_Argv(1 as libc::c_int), rotate, axis.as_mut_ptr());
}

#[no_mangle]
pub unsafe extern "C" fn SCR_Init() {
    scr_viewsize = Cvar_Get(
        b"viewsize\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"100\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        1 as libc::c_int,
    );
    scr_conspeed = Cvar_Get(
        b"scr_conspeed\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"3\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    scr_showturtle = Cvar_Get(
        b"scr_showturtle\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    scr_showpause = Cvar_Get(
        b"scr_showpause\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    scr_centertime = Cvar_Get(
        b"scr_centertime\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"2.5\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    scr_printspeed = Cvar_Get(
        b"scr_printspeed\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"8\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    scr_netgraph = Cvar_Get(
        b"netgraph\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    scr_timegraph = Cvar_Get(
        b"timegraph\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    scr_debuggraph = Cvar_Get(
        b"debuggraph\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    scr_graphheight = Cvar_Get(
        b"graphheight\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"32\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    scr_graphscale = Cvar_Get(
        b"graphscale\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    scr_graphshift = Cvar_Get(
        b"graphshift\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    scr_drawall = Cvar_Get(
        b"scr_drawall\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    Cmd_AddCommand(
        b"timerefresh\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        Some(SCR_TimeRefresh_f as unsafe extern "C" fn() -> ()),
    );
    Cmd_AddCommand(
        b"loading\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        Some(SCR_Loading_f as unsafe extern "C" fn() -> ()),
    );
    Cmd_AddCommand(
        b"sizeup\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        Some(SCR_SizeUp_f as unsafe extern "C" fn() -> ()),
    );
    Cmd_AddCommand(
        b"sizedown\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        Some(SCR_SizeDown_f as unsafe extern "C" fn() -> ()),
    );
    Cmd_AddCommand(
        b"sky\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        Some(SCR_Sky_f as unsafe extern "C" fn() -> ()),
    );
    scr_initialized = true_0;
}

#[no_mangle]
pub unsafe extern "C" fn SCR_DrawNet() {
    if cls.netchan.outgoing_sequence - cls.netchan.incoming_acknowledged
        < 64 as libc::c_int - 1 as libc::c_int
    {
        return;
    }
    (re.DrawPic)
        .expect(
            "non-null function pointer",
        )(
        scr_vrect.x + 64 as libc::c_int,
        scr_vrect.y,
        b"net\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
}

#[no_mangle]
pub unsafe extern "C" fn SCR_DrawPause() {
    let mut w: libc::c_int = 0;
    let mut h: libc::c_int = 0;
    if (*scr_showpause).value == 0. {
        return;
    }
    if (*cl_paused).value == 0. {
        return;
    }
    (re.DrawGetPicSize)
        .expect(
            "non-null function pointer",
        )(
        &mut w,
        &mut h,
        b"pause\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    (re.DrawPic)
        .expect(
            "non-null function pointer",
        )(
        (viddef.width - w) / 2 as libc::c_int,
        viddef.height / 2 as libc::c_int + 8 as libc::c_int,
        b"pause\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
}

#[no_mangle]
pub unsafe extern "C" fn SCR_DrawLoading() {
    let mut w: libc::c_int = 0;
    let mut h: libc::c_int = 0;
    if scr_draw_loading == 0 {
        return;
    }
    scr_draw_loading = false_0 as libc::c_int;
    (re.DrawGetPicSize)
        .expect(
            "non-null function pointer",
        )(
        &mut w,
        &mut h,
        b"loading\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    (re.DrawPic)
        .expect(
            "non-null function pointer",
        )(
        (viddef.width - w) / 2 as libc::c_int,
        (viddef.height - h) / 2 as libc::c_int,
        b"loading\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
}

#[no_mangle]
pub unsafe extern "C" fn SCR_RunConsole() {
    if cls.key_dest as libc::c_uint == key_console as libc::c_int as libc::c_uint {
        scr_conlines = 0.5f64 as libc::c_float;
    } else {
        scr_conlines = 0 as libc::c_int as libc::c_float;
    }
    if scr_conlines < scr_con_current {
        scr_con_current -= (*scr_conspeed).value * cls.frametime;
        if scr_conlines > scr_con_current {
            scr_con_current = scr_conlines;
        }
    } else if scr_conlines > scr_con_current {
        scr_con_current += (*scr_conspeed).value * cls.frametime;
        if scr_conlines < scr_con_current {
            scr_con_current = scr_conlines;
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn SCR_DrawConsole() {
    Con_CheckResize();
    if cls.state as libc::c_uint == ca_disconnected as libc::c_int as libc::c_uint
        || cls.state as libc::c_uint == ca_connecting as libc::c_int as libc::c_uint
    {
        Con_DrawConsole(1.0f64 as libc::c_float);
        return;
    }
    if cls.state as libc::c_uint != ca_active as libc::c_int as libc::c_uint
        || cl.refresh_prepped as u64 == 0
    {
        Con_DrawConsole(0.5f64 as libc::c_float);
        (re.DrawFill)
            .expect(
                "non-null function pointer",
            )(
            0 as libc::c_int,
            viddef.height / 2 as libc::c_int,
            viddef.width,
            viddef.height / 2 as libc::c_int,
            0 as libc::c_int,
        );
        return;
    }
    if scr_con_current != 0. {
        Con_DrawConsole(scr_con_current);
    } else if cls.key_dest as libc::c_uint == key_game as libc::c_int as libc::c_uint
        || cls.key_dest as libc::c_uint == key_message as libc::c_int as libc::c_uint
    {
        Con_DrawNotify();
    }
}

#[no_mangle]
pub unsafe extern "C" fn SCR_BeginLoadingPlaque() {
    S_StopAllSounds();
    cl.sound_prepped = false_0;
    CDAudio_Stop();
    if cls.disable_screen != 0. {
        return;
    }
    if (*developer).value != 0. {
        return;
    }
    if cls.state as libc::c_uint == ca_disconnected as libc::c_int as libc::c_uint {
        return;
    }
    if cls.key_dest as libc::c_uint == key_console as libc::c_int as libc::c_uint {
        return;
    }
    if cl.cinematictime > 0 as libc::c_int {
        scr_draw_loading = 2 as libc::c_int;
    } else {
        scr_draw_loading = 1 as libc::c_int;
    }
    SCR_UpdateScreen();
    cls.disable_screen = Sys_Milliseconds() as libc::c_float;
    cls.disable_servercount = cl.servercount;
}

#[no_mangle]
pub unsafe extern "C" fn SCR_EndLoadingPlaque() {
    cls.disable_screen = 0 as libc::c_int as libc::c_float;
    Con_ClearNotify();
}

#[no_mangle]
pub unsafe extern "C" fn SCR_Loading_f() {
    SCR_BeginLoadingPlaque();
}

#[no_mangle]
pub unsafe extern "C" fn entitycmpfnc(
    mut a: *const entity_t,
    mut b: *const entity_t,
) -> libc::c_int {
    if (*a).model == (*b).model {
        return (*a).skin as libc::c_int - (*b).skin as libc::c_int;
    } else {
        return (*a).model as libc::c_int - (*b).model as libc::c_int;
    };
}

#[no_mangle]
pub unsafe extern "C" fn SCR_TimeRefresh_f() {
    let mut i: libc::c_int = 0;
    let mut start: libc::c_int = 0;
    let mut stop: libc::c_int = 0;
    let mut time: libc::c_float = 0.;
    if cls.state as libc::c_uint != ca_active as libc::c_int as libc::c_uint {
        return;
    }
    start = Sys_Milliseconds();
    if Cmd_Argc() == 2 as libc::c_int {
        (re.BeginFrame)
            .expect("non-null function pointer")(0 as libc::c_int as libc::c_float);
        i = 0 as libc::c_int;
        while i < 128 as libc::c_int {
            cl
                .refdef
                .viewangles[1 as libc::c_int
                as usize] = (i as libc::c_double / 128.0f64 * 360.0f64) as libc::c_float;
            (re.RenderFrame).expect("non-null function pointer")(&mut cl.refdef);
            i += 1;
        }
        (re.EndFrame).expect("non-null function pointer")();
    } else {
        i = 0 as libc::c_int;
        while i < 128 as libc::c_int {
            cl
                .refdef
                .viewangles[1 as libc::c_int
                as usize] = (i as libc::c_double / 128.0f64 * 360.0f64) as libc::c_float;
            (re.BeginFrame)
                .expect("non-null function pointer")(0 as libc::c_int as libc::c_float);
            (re.RenderFrame).expect("non-null function pointer")(&mut cl.refdef);
            (re.EndFrame).expect("non-null function pointer")();
            i += 1;
        }
    }
    stop = Sys_Milliseconds();
    time = ((stop - start) as libc::c_double / 1000.0f64) as libc::c_float;
    Com_Printf(
        b"%f seconds (%f fps)\n\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        time as libc::c_double,
        (128 as libc::c_int as libc::c_float / time) as libc::c_double,
    );
}

#[no_mangle]
pub unsafe extern "C" fn SCR_AddDirtyPoint(mut x: libc::c_int, mut y: libc::c_int) {
    if x < scr_dirty.x1 {
        scr_dirty.x1 = x;
    }
    if x > scr_dirty.x2 {
        scr_dirty.x2 = x;
    }
    if y < scr_dirty.y1 {
        scr_dirty.y1 = y;
    }
    if y > scr_dirty.y2 {
        scr_dirty.y2 = y;
    }
}

#[no_mangle]
pub unsafe extern "C" fn SCR_DirtyScreen() {
    SCR_AddDirtyPoint(0 as libc::c_int, 0 as libc::c_int);
    SCR_AddDirtyPoint(viddef.width - 1 as libc::c_int, viddef.height - 1 as libc::c_int);
}

#[no_mangle]
pub unsafe extern "C" fn SCR_TileClear() {
    let mut i: libc::c_int = 0;
    let mut top: libc::c_int = 0;
    let mut bottom: libc::c_int = 0;
    let mut left: libc::c_int = 0;
    let mut right: libc::c_int = 0;
    let mut clear: dirty_t = dirty_t {
        x1: 0,
        y1: 0,
        x2: 0,
        y2: 0,
    };
    if (*scr_drawall).value != 0. {
        SCR_DirtyScreen();
    }
    if scr_con_current as libc::c_double == 1.0f64 {
        return;
    }
    if (*scr_viewsize).value == 100 as libc::c_int as libc::c_float {
        return;
    }
    if cl.cinematictime > 0 as libc::c_int {
        return;
    }
    clear = scr_dirty;
    i = 0 as libc::c_int;
    while i < 2 as libc::c_int {
        if scr_old_dirty[i as usize].x1 < clear.x1 {
            clear.x1 = scr_old_dirty[i as usize].x1;
        }
        if scr_old_dirty[i as usize].x2 > clear.x2 {
            clear.x2 = scr_old_dirty[i as usize].x2;
        }
        if scr_old_dirty[i as usize].y1 < clear.y1 {
            clear.y1 = scr_old_dirty[i as usize].y1;
        }
        if scr_old_dirty[i as usize].y2 > clear.y2 {
            clear.y2 = scr_old_dirty[i as usize].y2;
        }
        i += 1;
    }
    scr_old_dirty[1 as libc::c_int as usize] = scr_old_dirty[0 as libc::c_int as usize];
    scr_old_dirty[0 as libc::c_int as usize] = scr_dirty;
    scr_dirty.x1 = 9999 as libc::c_int;
    scr_dirty.x2 = -(9999 as libc::c_int);
    scr_dirty.y1 = 9999 as libc::c_int;
    scr_dirty.y2 = -(9999 as libc::c_int);
    top = (scr_con_current * viddef.height as libc::c_float) as libc::c_int;
    if top >= clear.y1 {
        clear.y1 = top;
    }
    if clear.y2 <= clear.y1 {
        return;
    }
    top = scr_vrect.y;
    bottom = top + scr_vrect.height - 1 as libc::c_int;
    left = scr_vrect.x;
    right = left + scr_vrect.width - 1 as libc::c_int;
    if clear.y1 < top {
        i = if clear.y2 < top - 1 as libc::c_int {
            clear.y2
        } else {
            top - 1 as libc::c_int
        };
        (re.DrawTileClear)
            .expect(
                "non-null function pointer",
            )(
            clear.x1,
            clear.y1,
            clear.x2 - clear.x1 + 1 as libc::c_int,
            i - clear.y1 + 1 as libc::c_int,
            b"backtile\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        clear.y1 = top;
    }
    if clear.y2 > bottom {
        i = if clear.y1 > bottom + 1 as libc::c_int {
            clear.y1
        } else {
            bottom + 1 as libc::c_int
        };
        (re.DrawTileClear)
            .expect(
                "non-null function pointer",
            )(
            clear.x1,
            i,
            clear.x2 - clear.x1 + 1 as libc::c_int,
            clear.y2 - i + 1 as libc::c_int,
            b"backtile\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        clear.y2 = bottom;
    }
    if clear.x1 < left {
        i = if clear.x2 < left - 1 as libc::c_int {
            clear.x2
        } else {
            left - 1 as libc::c_int
        };
        (re.DrawTileClear)
            .expect(
                "non-null function pointer",
            )(
            clear.x1,
            clear.y1,
            i - clear.x1 + 1 as libc::c_int,
            clear.y2 - clear.y1 + 1 as libc::c_int,
            b"backtile\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        clear.x1 = left;
    }
    if clear.x2 > right {
        i = if clear.x1 > right + 1 as libc::c_int {
            clear.x1
        } else {
            right + 1 as libc::c_int
        };
        (re.DrawTileClear)
            .expect(
                "non-null function pointer",
            )(
            i,
            clear.y1,
            clear.x2 - i + 1 as libc::c_int,
            clear.y2 - clear.y1 + 1 as libc::c_int,
            b"backtile\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        clear.x2 = right;
    }
}

#[no_mangle]
pub static mut sb_nums: [[*mut libc::c_char; 11]; 2] = [
    [
        b"num_0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"num_1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"num_2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"num_3\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"num_4\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"num_5\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"num_6\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"num_7\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"num_8\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"num_9\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"num_minus\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ],
    [
        b"anum_0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"anum_1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"anum_2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"anum_3\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"anum_4\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"anum_5\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"anum_6\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"anum_7\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"anum_8\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"anum_9\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"anum_minus\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ],
];

#[no_mangle]
pub unsafe extern "C" fn SizeHUDString(
    mut string: *mut libc::c_char,
    mut w: *mut libc::c_int,
    mut h: *mut libc::c_int,
) {
    let mut lines: libc::c_int = 0;
    let mut width: libc::c_int = 0;
    let mut current_0: libc::c_int = 0;
    lines = 1 as libc::c_int;
    width = 0 as libc::c_int;
    current_0 = 0 as libc::c_int;
    while *string != 0 {
        if *string as libc::c_int == '\n' as i32 {
            lines += 1;
            current_0 = 0 as libc::c_int;
        } else {
            current_0 += 1;
            if current_0 > width {
                width = current_0;
            }
        }
        string = string.offset(1);
    }
    *w = width * 8 as libc::c_int;
    *h = lines * 8 as libc::c_int;
}

#[no_mangle]
pub unsafe extern "C" fn DrawHUDString(
    mut string: *mut libc::c_char,
    mut x: libc::c_int,
    mut y: libc::c_int,
    mut centerwidth: libc::c_int,
    mut xor: libc::c_int,
) {
    let mut margin: libc::c_int = 0;
    let mut line: [libc::c_char; 1024] = [0; 1024];
    let mut width: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    margin = x;
    while *string != 0 {
        width = 0 as libc::c_int;
        while *string as libc::c_int != 0 && *string as libc::c_int != '\n' as i32 {
            let fresh2 = string;
            string = string.offset(1);
            let fresh3 = width;
            width = width + 1;
            line[fresh3 as usize] = *fresh2;
        }
        line[width as usize] = 0 as libc::c_int as libc::c_char;
        if centerwidth != 0 {
            x = margin + (centerwidth - width * 8 as libc::c_int) / 2 as libc::c_int;
        } else {
            x = margin;
        }
        i = 0 as libc::c_int;
        while i < width {
            (re.DrawChar)
                .expect(
                    "non-null function pointer",
                )(x, y, line[i as usize] as libc::c_int ^ xor);
            x += 8 as libc::c_int;
            i += 1;
        }
        if *string != 0 {
            string = string.offset(1);
            x = margin;
            y += 8 as libc::c_int;
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn SCR_DrawField(
    mut x: libc::c_int,
    mut y: libc::c_int,
    mut color: libc::c_int,
    mut width: libc::c_int,
    mut value: libc::c_int,
) {
    let mut num: [libc::c_char; 16] = [0; 16];
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut l: libc::c_int = 0;
    let mut frame: libc::c_int = 0;
    if width < 1 as libc::c_int {
        return;
    }
    if width > 5 as libc::c_int {
        width = 5 as libc::c_int;
    }
    SCR_AddDirtyPoint(x, y);
    SCR_AddDirtyPoint(
        x + width * 16 as libc::c_int + 2 as libc::c_int,
        y + 23 as libc::c_int,
    );
    Com_sprintf(
        num.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong as libc::c_int,
        b"%i\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        value,
    );
    l = strlen(num.as_mut_ptr()) as libc::c_int;
    if l > width {
        l = width;
    }
    x += 2 as libc::c_int + 16 as libc::c_int * (width - l);
    ptr = num.as_mut_ptr();
    while *ptr as libc::c_int != 0 && l != 0 {
        if *ptr as libc::c_int == '-' as i32 {
            frame = 10 as libc::c_int;
        } else {
            frame = *ptr as libc::c_int - '0' as i32;
        }
        (re.DrawPic)
            .expect(
                "non-null function pointer",
            )(x, y, sb_nums[color as usize][frame as usize]);
        x += 16 as libc::c_int;
        ptr = ptr.offset(1);
        l -= 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn SCR_TouchPics() {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 2 as libc::c_int {
        j = 0 as libc::c_int;
        while j < 11 as libc::c_int {
            (re.RegisterPic)
                .expect("non-null function pointer")(sb_nums[i as usize][j as usize]);
            j += 1;
        }
        i += 1;
    }
    if (*crosshair).value != 0. {
        if (*crosshair).value > 3 as libc::c_int as libc::c_float
            || (*crosshair).value < 0 as libc::c_int as libc::c_float
        {
            (*crosshair).value = 3 as libc::c_int as libc::c_float;
        }
        Com_sprintf(
            crosshair_pic.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
            b"ch%i\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            (*crosshair).value as libc::c_int,
        );
        (re.DrawGetPicSize)
            .expect(
                "non-null function pointer",
            )(&mut crosshair_width, &mut crosshair_height, crosshair_pic.as_mut_ptr());
        if crosshair_width == 0 {
            crosshair_pic[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn SCR_ExecuteLayoutString(mut s: *mut libc::c_char) {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut value: libc::c_int = 0;
    let mut token: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut width: libc::c_int = 0;
    let mut index: libc::c_int = 0;
    let mut ci: *mut clientinfo_t = 0 as *mut clientinfo_t;
    if cls.state as libc::c_uint != ca_active as libc::c_int as libc::c_uint
        || cl.refresh_prepped as u64 == 0
    {
        return;
    }
    if *s.offset(0 as libc::c_int as isize) == 0 {
        return;
    }
    x = 0 as libc::c_int;
    y = 0 as libc::c_int;
    width = 3 as libc::c_int;
    while !s.is_null() {
        token = COM_Parse(&mut s);
        if strcmp(token, b"xl\0" as *const u8 as *const libc::c_char) == 0 {
            token = COM_Parse(&mut s);
            x = atoi(token);
        } else if strcmp(token, b"xr\0" as *const u8 as *const libc::c_char) == 0 {
            token = COM_Parse(&mut s);
            x = viddef.width + atoi(token);
        } else if strcmp(token, b"xv\0" as *const u8 as *const libc::c_char) == 0 {
            token = COM_Parse(&mut s);
            x = viddef.width / 2 as libc::c_int - 160 as libc::c_int + atoi(token);
        } else if strcmp(token, b"yt\0" as *const u8 as *const libc::c_char) == 0 {
            token = COM_Parse(&mut s);
            y = atoi(token);
        } else if strcmp(token, b"yb\0" as *const u8 as *const libc::c_char) == 0 {
            token = COM_Parse(&mut s);
            y = viddef.height + atoi(token);
        } else if strcmp(token, b"yv\0" as *const u8 as *const libc::c_char) == 0 {
            token = COM_Parse(&mut s);
            y = viddef.height / 2 as libc::c_int - 120 as libc::c_int + atoi(token);
        } else if strcmp(token, b"pic\0" as *const u8 as *const libc::c_char) == 0 {
            token = COM_Parse(&mut s);
            value = cl.frame.playerstate.stats[atoi(token) as usize] as libc::c_int;
            if value >= 256 as libc::c_int {
                Com_Error(
                    1 as libc::c_int,
                    b"Pic >= MAX_IMAGES\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                );
            }
            if !(cl
                .configstrings[(32 as libc::c_int + 256 as libc::c_int
                + 256 as libc::c_int + value) as usize])
                .as_mut_ptr()
                .is_null()
            {
                SCR_AddDirtyPoint(x, y);
                SCR_AddDirtyPoint(x + 23 as libc::c_int, y + 23 as libc::c_int);
                (re.DrawPic)
                    .expect(
                        "non-null function pointer",
                    )(
                    x,
                    y,
                    (cl
                        .configstrings[(32 as libc::c_int + 256 as libc::c_int
                        + 256 as libc::c_int + value) as usize])
                        .as_mut_ptr(),
                );
            }
        } else if strcmp(token, b"client\0" as *const u8 as *const libc::c_char) == 0 {
            let mut score: libc::c_int = 0;
            let mut ping: libc::c_int = 0;
            let mut time: libc::c_int = 0;
            token = COM_Parse(&mut s);
            x = viddef.width / 2 as libc::c_int - 160 as libc::c_int + atoi(token);
            token = COM_Parse(&mut s);
            y = viddef.height / 2 as libc::c_int - 120 as libc::c_int + atoi(token);
            SCR_AddDirtyPoint(x, y);
            SCR_AddDirtyPoint(x + 159 as libc::c_int, y + 31 as libc::c_int);
            token = COM_Parse(&mut s);
            value = atoi(token);
            if value >= 256 as libc::c_int || value < 0 as libc::c_int {
                Com_Error(
                    1 as libc::c_int,
                    b"client >= MAX_CLIENTS\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                );
            }
            ci = &mut *(cl.clientinfo).as_mut_ptr().offset(value as isize)
                as *mut clientinfo_t;
            token = COM_Parse(&mut s);
            score = atoi(token);
            token = COM_Parse(&mut s);
            ping = atoi(token);
            token = COM_Parse(&mut s);
            time = atoi(token);
            DrawAltString(x + 32 as libc::c_int, y, ((*ci).name).as_mut_ptr());
            DrawString(
                x + 32 as libc::c_int,
                y + 8 as libc::c_int,
                b"Score: \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            DrawAltString(
                x + 32 as libc::c_int + 7 as libc::c_int * 8 as libc::c_int,
                y + 8 as libc::c_int,
                va(
                    b"%i\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    score,
                ),
            );
            DrawString(
                x + 32 as libc::c_int,
                y + 16 as libc::c_int,
                va(
                    b"Ping:  %i\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    ping,
                ),
            );
            DrawString(
                x + 32 as libc::c_int,
                y + 24 as libc::c_int,
                va(
                    b"Time:  %i\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    time,
                ),
            );
            if ((*ci).icon).is_null() {
                ci = &mut cl.baseclientinfo;
            }
            (re.DrawPic)
                .expect(
                    "non-null function pointer",
                )(x, y, ((*ci).iconname).as_mut_ptr());
        } else if strcmp(token, b"ctf\0" as *const u8 as *const libc::c_char) == 0 {
            let mut score_0: libc::c_int = 0;
            let mut ping_0: libc::c_int = 0;
            let mut block: [libc::c_char; 80] = [0; 80];
            token = COM_Parse(&mut s);
            x = viddef.width / 2 as libc::c_int - 160 as libc::c_int + atoi(token);
            token = COM_Parse(&mut s);
            y = viddef.height / 2 as libc::c_int - 120 as libc::c_int + atoi(token);
            SCR_AddDirtyPoint(x, y);
            SCR_AddDirtyPoint(x + 159 as libc::c_int, y + 31 as libc::c_int);
            token = COM_Parse(&mut s);
            value = atoi(token);
            if value >= 256 as libc::c_int || value < 0 as libc::c_int {
                Com_Error(
                    1 as libc::c_int,
                    b"client >= MAX_CLIENTS\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                );
            }
            ci = &mut *(cl.clientinfo).as_mut_ptr().offset(value as isize)
                as *mut clientinfo_t;
            token = COM_Parse(&mut s);
            score_0 = atoi(token);
            token = COM_Parse(&mut s);
            ping_0 = atoi(token);
            if ping_0 > 999 as libc::c_int {
                ping_0 = 999 as libc::c_int;
            }
            sprintf(
                block.as_mut_ptr(),
                b"%3d %3d %-12.12s\0" as *const u8 as *const libc::c_char,
                score_0,
                ping_0,
                ((*ci).name).as_mut_ptr(),
            );
            if value == cl.playernum {
                DrawAltString(x, y, block.as_mut_ptr());
            } else {
                DrawString(x, y, block.as_mut_ptr());
            }
        } else if strcmp(token, b"picn\0" as *const u8 as *const libc::c_char) == 0 {
            token = COM_Parse(&mut s);
            SCR_AddDirtyPoint(x, y);
            SCR_AddDirtyPoint(x + 23 as libc::c_int, y + 23 as libc::c_int);
            (re.DrawPic).expect("non-null function pointer")(x, y, token);
        } else if strcmp(token, b"num\0" as *const u8 as *const libc::c_char) == 0 {
            token = COM_Parse(&mut s);
            width = atoi(token);
            token = COM_Parse(&mut s);
            value = cl.frame.playerstate.stats[atoi(token) as usize] as libc::c_int;
            SCR_DrawField(x, y, 0 as libc::c_int, width, value);
        } else if strcmp(token, b"hnum\0" as *const u8 as *const libc::c_char) == 0 {
            let mut color: libc::c_int = 0;
            width = 3 as libc::c_int;
            value = cl.frame.playerstate.stats[1 as libc::c_int as usize] as libc::c_int;
            if value > 25 as libc::c_int {
                color = 0 as libc::c_int;
            } else if value > 0 as libc::c_int {
                color = cl.frame.serverframe >> 2 as libc::c_int & 1 as libc::c_int;
            } else {
                color = 1 as libc::c_int;
            }
            if cl.frame.playerstate.stats[15 as libc::c_int as usize] as libc::c_int
                & 1 as libc::c_int != 0
            {
                (re.DrawPic)
                    .expect(
                        "non-null function pointer",
                    )(
                    x,
                    y,
                    b"field_3\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                );
            }
            SCR_DrawField(x, y, color, width, value);
        } else if strcmp(token, b"anum\0" as *const u8 as *const libc::c_char) == 0 {
            let mut color_0: libc::c_int = 0;
            width = 3 as libc::c_int;
            value = cl.frame.playerstate.stats[3 as libc::c_int as usize] as libc::c_int;
            if value > 5 as libc::c_int {
                color_0 = 0 as libc::c_int;
            } else {
                if !(value >= 0 as libc::c_int) {
                    continue;
                }
                color_0 = cl.frame.serverframe >> 2 as libc::c_int & 1 as libc::c_int;
            }
            if cl.frame.playerstate.stats[15 as libc::c_int as usize] as libc::c_int
                & 4 as libc::c_int != 0
            {
                (re.DrawPic)
                    .expect(
                        "non-null function pointer",
                    )(
                    x,
                    y,
                    b"field_3\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                );
            }
            SCR_DrawField(x, y, color_0, width, value);
        } else if strcmp(token, b"rnum\0" as *const u8 as *const libc::c_char) == 0 {
            let mut color_1: libc::c_int = 0;
            width = 3 as libc::c_int;
            value = cl.frame.playerstate.stats[5 as libc::c_int as usize] as libc::c_int;
            if value < 1 as libc::c_int {
                continue;
            }
            color_1 = 0 as libc::c_int;
            if cl.frame.playerstate.stats[15 as libc::c_int as usize] as libc::c_int
                & 2 as libc::c_int != 0
            {
                (re.DrawPic)
                    .expect(
                        "non-null function pointer",
                    )(
                    x,
                    y,
                    b"field_3\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                );
            }
            SCR_DrawField(x, y, color_1, width, value);
        } else if strcmp(token, b"stat_string\0" as *const u8 as *const libc::c_char)
            == 0
        {
            token = COM_Parse(&mut s);
            index = atoi(token);
            if index < 0 as libc::c_int
                || index
                >= 32 as libc::c_int + 256 as libc::c_int + 256 as libc::c_int
                + 256 as libc::c_int + 256 as libc::c_int + 256 as libc::c_int
                + 256 as libc::c_int + 256 as libc::c_int * 2 as libc::c_int
            {
                Com_Error(
                    1 as libc::c_int,
                    b"Bad stat_string index\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                );
            }
            index = cl.frame.playerstate.stats[index as usize] as libc::c_int;
            if index < 0 as libc::c_int
                || index
                >= 32 as libc::c_int + 256 as libc::c_int + 256 as libc::c_int
                + 256 as libc::c_int + 256 as libc::c_int + 256 as libc::c_int
                + 256 as libc::c_int + 256 as libc::c_int * 2 as libc::c_int
            {
                Com_Error(
                    1 as libc::c_int,
                    b"Bad stat_string index\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                );
            }
            DrawString(x, y, (cl.configstrings[index as usize]).as_mut_ptr());
        } else if strcmp(token, b"cstring\0" as *const u8 as *const libc::c_char) == 0 {
            token = COM_Parse(&mut s);
            DrawHUDString(token, x, y, 320 as libc::c_int, 0 as libc::c_int);
        } else if strcmp(token, b"string\0" as *const u8 as *const libc::c_char) == 0 {
            token = COM_Parse(&mut s);
            DrawString(x, y, token);
        } else if strcmp(token, b"cstring2\0" as *const u8 as *const libc::c_char) == 0 {
            token = COM_Parse(&mut s);
            DrawHUDString(token, x, y, 320 as libc::c_int, 0x80 as libc::c_int);
        } else if strcmp(token, b"string2\0" as *const u8 as *const libc::c_char) == 0 {
            token = COM_Parse(&mut s);
            DrawAltString(x, y, token);
        } else {
            if !(strcmp(token, b"if\0" as *const u8 as *const libc::c_char) == 0) {
                continue;
            }
            token = COM_Parse(&mut s);
            value = cl.frame.playerstate.stats[atoi(token) as usize] as libc::c_int;
            if value == 0 {
                while !s.is_null()
                    && strcmp(token, b"endif\0" as *const u8 as *const libc::c_char) != 0
                {
                    token = COM_Parse(&mut s);
                }
            }
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn SCR_DrawStats() {
    SCR_ExecuteLayoutString((cl.configstrings[5 as libc::c_int as usize]).as_mut_ptr());
}

#[no_mangle]
pub unsafe extern "C" fn SCR_DrawLayout() {
    if cl.frame.playerstate.stats[13 as libc::c_int as usize] == 0 {
        return;
    }
    SCR_ExecuteLayoutString((cl.layout).as_mut_ptr());
}

#[no_mangle]
pub unsafe extern "C" fn SCR_UpdateScreen() {
    let mut numframes: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut separation: [libc::c_float; 2] = [
        0 as libc::c_int as libc::c_float,
        0 as libc::c_int as libc::c_float,
    ];
    if cls.disable_screen != 0. {
        if Sys_Milliseconds() as libc::c_float - cls.disable_screen
            > 120000 as libc::c_int as libc::c_float
        {
            cls.disable_screen = 0 as libc::c_int as libc::c_float;
            Com_Printf(
                b"Loading plaque timed out.\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
        }
        return;
    }
    if scr_initialized as u64 == 0 || con.initialized as u64 == 0 {
        return;
    }
    if (*cl_stereo_separation).value as libc::c_double > 1.0f64 {
        Cvar_SetValue(
            b"cl_stereo_separation\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            1.0f64 as libc::c_float,
        );
    } else if (*cl_stereo_separation).value < 0 as libc::c_int as libc::c_float {
        Cvar_SetValue(
            b"cl_stereo_separation\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            0.0f64 as libc::c_float,
        );
    }
    if (*cl_stereo).value != 0. {
        numframes = 2 as libc::c_int;
        separation[0 as libc::c_int
            as usize] = -(*cl_stereo_separation).value
            / 2 as libc::c_int as libc::c_float;
        separation[1 as libc::c_int
            as usize] = (*cl_stereo_separation).value
            / 2 as libc::c_int as libc::c_float;
    } else {
        separation[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_float;
        separation[1 as libc::c_int as usize] = 0 as libc::c_int as libc::c_float;
        numframes = 1 as libc::c_int;
    }
    i = 0 as libc::c_int;
    while i < numframes {
        (re.BeginFrame).expect("non-null function pointer")(separation[i as usize]);
        if scr_draw_loading == 2 as libc::c_int {
            let mut w: libc::c_int = 0;
            let mut h: libc::c_int = 0;
            (re.CinematicSetPalette)
                .expect("non-null function pointer")(0 as *const libc::c_uchar);
            scr_draw_loading = false_0 as libc::c_int;
            (re.DrawGetPicSize)
                .expect(
                    "non-null function pointer",
                )(
                &mut w,
                &mut h,
                b"loading\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            (re.DrawPic)
                .expect(
                    "non-null function pointer",
                )(
                (viddef.width - w) / 2 as libc::c_int,
                (viddef.height - h) / 2 as libc::c_int,
                b"loading\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
        } else if cl.cinematictime > 0 as libc::c_int {
            if cls.key_dest as libc::c_uint == key_menu as libc::c_int as libc::c_uint {
                if cl.cinematicpalette_active as u64 != 0 {
                    (re.CinematicSetPalette)
                        .expect("non-null function pointer")(0 as *const libc::c_uchar);
                    cl.cinematicpalette_active = false_0;
                }
                M_Draw();
            } else if cls.key_dest as libc::c_uint
                == key_console as libc::c_int as libc::c_uint
            {
                if cl.cinematicpalette_active as u64 != 0 {
                    (re.CinematicSetPalette)
                        .expect("non-null function pointer")(0 as *const libc::c_uchar);
                    cl.cinematicpalette_active = false_0;
                }
                SCR_DrawConsole();
            } else {
                SCR_DrawCinematic();
            }
        } else {
            if cl.cinematicpalette_active as u64 != 0 {
                (re.CinematicSetPalette)
                    .expect("non-null function pointer")(0 as *const libc::c_uchar);
                cl.cinematicpalette_active = false_0;
            }
            SCR_CalcVrect();
            SCR_TileClear();
            V_RenderView(separation[i as usize]);
            SCR_DrawStats();
            if cl.frame.playerstate.stats[13 as libc::c_int as usize] as libc::c_int
                & 1 as libc::c_int != 0
            {
                SCR_DrawLayout();
            }
            if cl.frame.playerstate.stats[13 as libc::c_int as usize] as libc::c_int
                & 2 as libc::c_int != 0
            {
                CL_DrawInventory();
            }
            SCR_DrawNet();
            SCR_CheckDrawCenterString();
            if (*scr_timegraph).value != 0. {
                SCR_DebugGraph(
                    cls.frametime * 300 as libc::c_int as libc::c_float,
                    0 as libc::c_int,
                );
            }
            if (*scr_debuggraph).value != 0. || (*scr_timegraph).value != 0.
                || (*scr_netgraph).value != 0.
            {
                SCR_DrawDebugGraph();
            }
            SCR_DrawPause();
            SCR_DrawConsole();
            M_Draw();
            SCR_DrawLoading();
        }
        i += 1;
    }
    (re.EndFrame).expect("non-null function pointer")();
}
