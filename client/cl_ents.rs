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
    fn sin(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    static mut gun_model: *mut model_s;
    static mut gun_frame: libc::c_int;
    static mut re: refexport_t;
    fn V_AddEntity(ent: *mut entity_t);
    fn CL_RocketTrail(start: *mut vec_t, end: *mut vec_t, old: *mut centity_t);
    fn CL_FlyEffect(ent: *mut centity_t, origin: *mut vec_t);
    fn CL_BfgParticles(ent: *mut entity_t);
    fn CL_TrapParticles(ent: *mut entity_t);
    fn CL_DiminishingTrail(
        start: *mut vec_t,
        end: *mut vec_t,
        old: *mut centity_t,
        flags: libc::c_int,
    );
    fn V_AddLight(
        org: *mut vec_t,
        intensity: libc::c_float,
        r: libc::c_float,
        g: libc::c_float,
        b: libc::c_float,
    );
    fn CL_AddTEnts();
    fn CL_AddParticles();
    fn CL_AddDLights();
    fn CL_AddLightStyles();
    fn SHOWNET(s: *mut libc::c_char);
    static mut svc_strings: [*mut libc::c_char; 256];
    fn CL_EntityEvent(ent: *mut entity_state_t);
    fn CL_CheckPredictionError();
    fn CL_Tracker_Shell(origin: *mut vec_t);
    fn CL_TagTrail(start: *mut vec_t, end: *mut vec_t, color: libc::c_float);
    fn CL_TrackerTrail(start: *mut vec_t, end: *mut vec_t, particleColor: libc::c_int);
    fn CL_BlasterTrail2(start: *mut vec_t, end: *mut vec_t);
    fn CL_IonripperTrail(start: *mut vec_t, end: *mut vec_t);
    fn CL_FlagTrail(start: *mut vec_t, end: *mut vec_t, color: libc::c_float);
    fn CL_BlasterTrail(start: *mut vec_t, end: *mut vec_t);
    fn CL_TeleporterParticles(ent: *mut entity_state_t);
    static mut cl_parse_entities: [entity_state_t; 1024];
    static mut cl_vwep: *mut cvar_t;
    static mut cl_timedemo: *mut cvar_t;
    static mut cl_showclamp: *mut cvar_t;
    static mut cl_shownet: *mut cvar_t;
    static mut cl_predict: *mut cvar_t;
    static mut cl_gun: *mut cvar_t;
    static mut cls: client_static_t;
    static mut cl: client_state_t;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn rand() -> libc::c_int;
    fn abs(_: libc::c_int) -> libc::c_int;
    fn VectorMA(
        veca: *mut vec_t,
        scale: libc::c_float,
        vecb: *mut vec_t,
        vecc: *mut vec_t,
    );
    fn AngleVectors(
        angles: *mut vec_t,
        forward: *mut vec_t,
        right: *mut vec_t,
        up: *mut vec_t,
    );
    fn anglemod(a: libc::c_float) -> libc::c_float;
    fn LerpAngle(
        a1: libc::c_float,
        a2: libc::c_float,
        frac: libc::c_float,
    ) -> libc::c_float;
    fn Com_sprintf(
        dest: *mut libc::c_char,
        size: libc::c_int,
        fmt: *mut libc::c_char,
        _: ...
    );
    fn Com_Printf(msg: *mut libc::c_char, _: ...);
    fn MSG_ReadChar(sb: *mut sizebuf_t) -> libc::c_int;
    fn MSG_ReadByte(sb: *mut sizebuf_t) -> libc::c_int;
    fn MSG_ReadShort(sb: *mut sizebuf_t) -> libc::c_int;
    fn MSG_ReadLong(sb: *mut sizebuf_t) -> libc::c_int;
    fn MSG_ReadCoord(sb: *mut sizebuf_t) -> libc::c_float;
    fn MSG_ReadPos(sb: *mut sizebuf_t, pos: *mut vec_t);
    fn MSG_ReadAngle(sb: *mut sizebuf_t) -> libc::c_float;
    fn MSG_ReadAngle16(sb: *mut sizebuf_t) -> libc::c_float;
    fn MSG_ReadData(sb: *mut sizebuf_t, buffer: *mut libc::c_void, size: libc::c_int);
    static mut net_message: sizebuf_t;
    fn Com_Error(code: libc::c_int, fmt: *mut libc::c_char, _: ...);
    fn SCR_EndLoadingPlaque();
    static mut cl_entities: [centity_t; 1024];
    static mut cl_mod_powerscreen: *mut model_s;
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
pub type svc_ops_e = libc::c_uint;

pub const svc_frame: svc_ops_e = 20;
pub const svc_deltapacketentities: svc_ops_e = 19;
pub const svc_packetentities: svc_ops_e = 18;
pub const svc_playerinfo: svc_ops_e = 17;
pub const svc_download: svc_ops_e = 16;
pub const svc_centerprint: svc_ops_e = 15;
pub const svc_spawnbaseline: svc_ops_e = 14;
pub const svc_configstring: svc_ops_e = 13;
pub const svc_serverdata: svc_ops_e = 12;
pub const svc_stufftext: svc_ops_e = 11;
pub const svc_print: svc_ops_e = 10;
pub const svc_sound: svc_ops_e = 9;
pub const svc_reconnect: svc_ops_e = 8;
pub const svc_disconnect: svc_ops_e = 7;
pub const svc_nop: svc_ops_e = 6;
pub const svc_inventory: svc_ops_e = 5;
pub const svc_layout: svc_ops_e = 4;
pub const svc_temp_entity: svc_ops_e = 3;
pub const svc_muzzleflash2: svc_ops_e = 2;
pub const svc_muzzleflash: svc_ops_e = 1;
pub const svc_bad: svc_ops_e = 0;

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
pub static mut vidref_val: libc::c_int = 0;
#[no_mangle]
pub static mut bitcounts: [libc::c_int; 32] = [0; 32];

#[no_mangle]
pub unsafe extern "C" fn CL_ParseEntityBits(mut bits: *mut libc::c_uint) -> libc::c_int {
    let mut b: libc::c_uint = 0;
    let mut total: libc::c_uint = 0;
    let mut i: libc::c_int = 0;
    let mut number: libc::c_int = 0;
    total = MSG_ReadByte(&mut net_message) as libc::c_uint;
    if total & ((1 as libc::c_int) << 7 as libc::c_int) as libc::c_uint != 0 {
        b = MSG_ReadByte(&mut net_message) as libc::c_uint;
        total |= b << 8 as libc::c_int;
    }
    if total & ((1 as libc::c_int) << 15 as libc::c_int) as libc::c_uint != 0 {
        b = MSG_ReadByte(&mut net_message) as libc::c_uint;
        total |= b << 16 as libc::c_int;
    }
    if total & ((1 as libc::c_int) << 23 as libc::c_int) as libc::c_uint != 0 {
        b = MSG_ReadByte(&mut net_message) as libc::c_uint;
        total |= b << 24 as libc::c_int;
    }
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        if total & ((1 as libc::c_int) << i) as libc::c_uint != 0 {
            bitcounts[i as usize] += 1;
        }
        i += 1;
    }
    if total & ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_uint != 0 {
        number = MSG_ReadShort(&mut net_message);
    } else {
        number = MSG_ReadByte(&mut net_message);
    }
    *bits = total;
    return number;
}

#[no_mangle]
pub unsafe extern "C" fn CL_ParseDelta(
    mut from: *mut entity_state_t,
    mut to: *mut entity_state_t,
    mut number: libc::c_int,
    mut bits: libc::c_int,
) {
    *to = *from;
    (*to)
        .old_origin[0 as libc::c_int
        as usize] = (*from).origin[0 as libc::c_int as usize];
    (*to)
        .old_origin[1 as libc::c_int
        as usize] = (*from).origin[1 as libc::c_int as usize];
    (*to)
        .old_origin[2 as libc::c_int
        as usize] = (*from).origin[2 as libc::c_int as usize];
    (*to).number = number;
    if bits & (1 as libc::c_int) << 11 as libc::c_int != 0 {
        (*to).modelindex = MSG_ReadByte(&mut net_message);
    }
    if bits & (1 as libc::c_int) << 20 as libc::c_int != 0 {
        (*to).modelindex2 = MSG_ReadByte(&mut net_message);
    }
    if bits & (1 as libc::c_int) << 21 as libc::c_int != 0 {
        (*to).modelindex3 = MSG_ReadByte(&mut net_message);
    }
    if bits & (1 as libc::c_int) << 22 as libc::c_int != 0 {
        (*to).modelindex4 = MSG_ReadByte(&mut net_message);
    }
    if bits & (1 as libc::c_int) << 4 as libc::c_int != 0 {
        (*to).frame = MSG_ReadByte(&mut net_message);
    }
    if bits & (1 as libc::c_int) << 17 as libc::c_int != 0 {
        (*to).frame = MSG_ReadShort(&mut net_message);
    }
    if bits & (1 as libc::c_int) << 16 as libc::c_int != 0
        && bits & (1 as libc::c_int) << 25 as libc::c_int != 0
    {
        (*to).skinnum = MSG_ReadLong(&mut net_message);
    } else if bits & (1 as libc::c_int) << 16 as libc::c_int != 0 {
        (*to).skinnum = MSG_ReadByte(&mut net_message);
    } else if bits & (1 as libc::c_int) << 25 as libc::c_int != 0 {
        (*to).skinnum = MSG_ReadShort(&mut net_message);
    }
    if bits
        & ((1 as libc::c_int) << 14 as libc::c_int
        | (1 as libc::c_int) << 19 as libc::c_int)
        == (1 as libc::c_int) << 14 as libc::c_int
        | (1 as libc::c_int) << 19 as libc::c_int
    {
        (*to).effects = MSG_ReadLong(&mut net_message) as libc::c_uint;
    } else if bits & (1 as libc::c_int) << 14 as libc::c_int != 0 {
        (*to).effects = MSG_ReadByte(&mut net_message) as libc::c_uint;
    } else if bits & (1 as libc::c_int) << 19 as libc::c_int != 0 {
        (*to).effects = MSG_ReadShort(&mut net_message) as libc::c_uint;
    }
    if bits
        & ((1 as libc::c_int) << 12 as libc::c_int
        | (1 as libc::c_int) << 18 as libc::c_int)
        == (1 as libc::c_int) << 12 as libc::c_int
        | (1 as libc::c_int) << 18 as libc::c_int
    {
        (*to).renderfx = MSG_ReadLong(&mut net_message);
    } else if bits & (1 as libc::c_int) << 12 as libc::c_int != 0 {
        (*to).renderfx = MSG_ReadByte(&mut net_message);
    } else if bits & (1 as libc::c_int) << 18 as libc::c_int != 0 {
        (*to).renderfx = MSG_ReadShort(&mut net_message);
    }
    if bits & (1 as libc::c_int) << 0 as libc::c_int != 0 {
        (*to).origin[0 as libc::c_int as usize] = MSG_ReadCoord(&mut net_message);
    }
    if bits & (1 as libc::c_int) << 1 as libc::c_int != 0 {
        (*to).origin[1 as libc::c_int as usize] = MSG_ReadCoord(&mut net_message);
    }
    if bits & (1 as libc::c_int) << 9 as libc::c_int != 0 {
        (*to).origin[2 as libc::c_int as usize] = MSG_ReadCoord(&mut net_message);
    }
    if bits & (1 as libc::c_int) << 10 as libc::c_int != 0 {
        (*to).angles[0 as libc::c_int as usize] = MSG_ReadAngle(&mut net_message);
    }
    if bits & (1 as libc::c_int) << 2 as libc::c_int != 0 {
        (*to).angles[1 as libc::c_int as usize] = MSG_ReadAngle(&mut net_message);
    }
    if bits & (1 as libc::c_int) << 3 as libc::c_int != 0 {
        (*to).angles[2 as libc::c_int as usize] = MSG_ReadAngle(&mut net_message);
    }
    if bits & (1 as libc::c_int) << 24 as libc::c_int != 0 {
        MSG_ReadPos(&mut net_message, ((*to).old_origin).as_mut_ptr());
    }
    if bits & (1 as libc::c_int) << 26 as libc::c_int != 0 {
        (*to).sound = MSG_ReadByte(&mut net_message);
    }
    if bits & (1 as libc::c_int) << 5 as libc::c_int != 0 {
        (*to).event = MSG_ReadByte(&mut net_message);
    } else {
        (*to).event = 0 as libc::c_int;
    }
    if bits & (1 as libc::c_int) << 27 as libc::c_int != 0 {
        (*to).solid = MSG_ReadShort(&mut net_message);
    }
}

#[no_mangle]
pub unsafe extern "C" fn CL_DeltaEntity(
    mut frame: *mut frame_t,
    mut newnum: libc::c_int,
    mut old: *mut entity_state_t,
    mut bits: libc::c_int,
) {
    let mut ent: *mut centity_t = 0 as *mut centity_t;
    let mut state: *mut entity_state_t = 0 as *mut entity_state_t;
    ent = &mut *cl_entities.as_mut_ptr().offset(newnum as isize) as *mut centity_t;
    state = &mut *cl_parse_entities
        .as_mut_ptr()
        .offset((cl.parse_entities & 1024 as libc::c_int - 1 as libc::c_int) as isize)
        as *mut entity_state_t;
    cl.parse_entities += 1;
    let ref mut fresh0 = (*frame).num_entities;
    *fresh0 += 1;
    CL_ParseDelta(old, state, newnum, bits);
    if (*state).modelindex != (*ent).current.modelindex
        || (*state).modelindex2 != (*ent).current.modelindex2
        || (*state).modelindex3 != (*ent).current.modelindex3
        || (*state).modelindex4 != (*ent).current.modelindex4
        || abs(
        ((*state).origin[0 as libc::c_int as usize]
            - (*ent).current.origin[0 as libc::c_int as usize]) as libc::c_int,
    ) > 512 as libc::c_int
        || abs(
        ((*state).origin[1 as libc::c_int as usize]
            - (*ent).current.origin[1 as libc::c_int as usize]) as libc::c_int,
    ) > 512 as libc::c_int
        || abs(
        ((*state).origin[2 as libc::c_int as usize]
            - (*ent).current.origin[2 as libc::c_int as usize]) as libc::c_int,
    ) > 512 as libc::c_int || (*state).event == EV_PLAYER_TELEPORT as libc::c_int
        || (*state).event == EV_OTHER_TELEPORT as libc::c_int
    {
        (*ent).serverframe = -(99 as libc::c_int);
    }
    if (*ent).serverframe != cl.frame.serverframe - 1 as libc::c_int {
        (*ent).trailcount = 1024 as libc::c_int;
        (*ent).prev = *state;
        if (*state).event == EV_OTHER_TELEPORT as libc::c_int {
            (*ent)
                .prev
                .origin[0 as libc::c_int
                as usize] = (*state).origin[0 as libc::c_int as usize];
            (*ent)
                .prev
                .origin[1 as libc::c_int
                as usize] = (*state).origin[1 as libc::c_int as usize];
            (*ent)
                .prev
                .origin[2 as libc::c_int
                as usize] = (*state).origin[2 as libc::c_int as usize];
            (*ent)
                .lerp_origin[0 as libc::c_int
                as usize] = (*state).origin[0 as libc::c_int as usize];
            (*ent)
                .lerp_origin[1 as libc::c_int
                as usize] = (*state).origin[1 as libc::c_int as usize];
            (*ent)
                .lerp_origin[2 as libc::c_int
                as usize] = (*state).origin[2 as libc::c_int as usize];
        } else {
            (*ent)
                .prev
                .origin[0 as libc::c_int
                as usize] = (*state).old_origin[0 as libc::c_int as usize];
            (*ent)
                .prev
                .origin[1 as libc::c_int
                as usize] = (*state).old_origin[1 as libc::c_int as usize];
            (*ent)
                .prev
                .origin[2 as libc::c_int
                as usize] = (*state).old_origin[2 as libc::c_int as usize];
            (*ent)
                .lerp_origin[0 as libc::c_int
                as usize] = (*state).old_origin[0 as libc::c_int as usize];
            (*ent)
                .lerp_origin[1 as libc::c_int
                as usize] = (*state).old_origin[1 as libc::c_int as usize];
            (*ent)
                .lerp_origin[2 as libc::c_int
                as usize] = (*state).old_origin[2 as libc::c_int as usize];
        }
    } else {
        (*ent).prev = (*ent).current;
    }
    (*ent).serverframe = cl.frame.serverframe;
    (*ent).current = *state;
}

#[no_mangle]
pub unsafe extern "C" fn CL_ParsePacketEntities(
    mut oldframe: *mut frame_t,
    mut newframe: *mut frame_t,
) {
    let mut newnum: libc::c_int = 0;
    let mut bits: libc::c_int = 0;
    let mut oldstate: *mut entity_state_t = 0 as *mut entity_state_t;
    let mut oldindex: libc::c_int = 0;
    let mut oldnum: libc::c_int = 0;
    (*newframe).parse_entities = cl.parse_entities;
    (*newframe).num_entities = 0 as libc::c_int;
    oldindex = 0 as libc::c_int;
    if oldframe.is_null() {
        oldnum = 99999 as libc::c_int;
    } else if oldindex >= (*oldframe).num_entities {
        oldnum = 99999 as libc::c_int;
    } else {
        oldstate = &mut *cl_parse_entities
            .as_mut_ptr()
            .offset(
                ((*oldframe).parse_entities + oldindex
                    & 1024 as libc::c_int - 1 as libc::c_int) as isize,
            ) as *mut entity_state_t;
        oldnum = (*oldstate).number;
    }
    loop {
        newnum = CL_ParseEntityBits(&mut bits as *mut libc::c_int as *mut libc::c_uint);
        if newnum >= 1024 as libc::c_int {
            Com_Error(
                1 as libc::c_int,
                b"CL_ParsePacketEntities: bad number:%i\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                newnum,
            );
        }
        if net_message.readcount > net_message.cursize {
            Com_Error(
                1 as libc::c_int,
                b"CL_ParsePacketEntities: end of message\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
            );
        }
        if newnum == 0 {
            break;
        }
        while oldnum < newnum {
            if (*cl_shownet).value == 3 as libc::c_int as libc::c_float {
                Com_Printf(
                    b"   unchanged: %i\n\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    oldnum,
                );
            }
            CL_DeltaEntity(newframe, oldnum, oldstate, 0 as libc::c_int);
            oldindex += 1;
            if oldindex >= (*oldframe).num_entities {
                oldnum = 99999 as libc::c_int;
            } else {
                oldstate = &mut *cl_parse_entities
                    .as_mut_ptr()
                    .offset(
                        ((*oldframe).parse_entities + oldindex
                            & 1024 as libc::c_int - 1 as libc::c_int) as isize,
                    ) as *mut entity_state_t;
                oldnum = (*oldstate).number;
            }
        }
        if bits & (1 as libc::c_int) << 6 as libc::c_int != 0 {
            if (*cl_shownet).value == 3 as libc::c_int as libc::c_float {
                Com_Printf(
                    b"   remove: %i\n\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    newnum,
                );
            }
            if oldnum != newnum {
                Com_Printf(
                    b"U_REMOVE: oldnum != newnum\n\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                );
            }
            oldindex += 1;
            if oldindex >= (*oldframe).num_entities {
                oldnum = 99999 as libc::c_int;
            } else {
                oldstate = &mut *cl_parse_entities
                    .as_mut_ptr()
                    .offset(
                        ((*oldframe).parse_entities + oldindex
                            & 1024 as libc::c_int - 1 as libc::c_int) as isize,
                    ) as *mut entity_state_t;
                oldnum = (*oldstate).number;
            }
        } else if oldnum == newnum {
            if (*cl_shownet).value == 3 as libc::c_int as libc::c_float {
                Com_Printf(
                    b"   delta: %i\n\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    newnum,
                );
            }
            CL_DeltaEntity(newframe, newnum, oldstate, bits);
            oldindex += 1;
            if oldindex >= (*oldframe).num_entities {
                oldnum = 99999 as libc::c_int;
            } else {
                oldstate = &mut *cl_parse_entities
                    .as_mut_ptr()
                    .offset(
                        ((*oldframe).parse_entities + oldindex
                            & 1024 as libc::c_int - 1 as libc::c_int) as isize,
                    ) as *mut entity_state_t;
                oldnum = (*oldstate).number;
            }
        } else {
            if !(oldnum > newnum) {
                continue;
            }
            if (*cl_shownet).value == 3 as libc::c_int as libc::c_float {
                Com_Printf(
                    b"   baseline: %i\n\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    newnum,
                );
            }
            CL_DeltaEntity(
                newframe,
                newnum,
                &mut (*cl_entities.as_mut_ptr().offset(newnum as isize)).baseline,
                bits,
            );
        }
    }
    while oldnum != 99999 as libc::c_int {
        if (*cl_shownet).value == 3 as libc::c_int as libc::c_float {
            Com_Printf(
                b"   unchanged: %i\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                oldnum,
            );
        }
        CL_DeltaEntity(newframe, oldnum, oldstate, 0 as libc::c_int);
        oldindex += 1;
        if oldindex >= (*oldframe).num_entities {
            oldnum = 99999 as libc::c_int;
        } else {
            oldstate = &mut *cl_parse_entities
                .as_mut_ptr()
                .offset(
                    ((*oldframe).parse_entities + oldindex
                        & 1024 as libc::c_int - 1 as libc::c_int) as isize,
                ) as *mut entity_state_t;
            oldnum = (*oldstate).number;
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn CL_ParsePlayerstate(
    mut oldframe: *mut frame_t,
    mut newframe: *mut frame_t,
) {
    let mut flags: libc::c_int = 0;
    let mut state: *mut player_state_t = 0 as *mut player_state_t;
    let mut i: libc::c_int = 0;
    let mut statbits: libc::c_int = 0;
    state = &mut (*newframe).playerstate;
    if !oldframe.is_null() {
        *state = (*oldframe).playerstate;
    } else {
        memset(
            state as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<player_state_t>() as libc::c_ulong,
        );
    }
    flags = MSG_ReadShort(&mut net_message);
    if flags & (1 as libc::c_int) << 0 as libc::c_int != 0 {
        (*state).pmove.pm_type = MSG_ReadByte(&mut net_message) as pmtype_t;
    }
    if flags & (1 as libc::c_int) << 1 as libc::c_int != 0 {
        (*state)
            .pmove
            .origin[0 as libc::c_int
            as usize] = MSG_ReadShort(&mut net_message) as libc::c_short;
        (*state)
            .pmove
            .origin[1 as libc::c_int
            as usize] = MSG_ReadShort(&mut net_message) as libc::c_short;
        (*state)
            .pmove
            .origin[2 as libc::c_int
            as usize] = MSG_ReadShort(&mut net_message) as libc::c_short;
    }
    if flags & (1 as libc::c_int) << 2 as libc::c_int != 0 {
        (*state)
            .pmove
            .velocity[0 as libc::c_int
            as usize] = MSG_ReadShort(&mut net_message) as libc::c_short;
        (*state)
            .pmove
            .velocity[1 as libc::c_int
            as usize] = MSG_ReadShort(&mut net_message) as libc::c_short;
        (*state)
            .pmove
            .velocity[2 as libc::c_int
            as usize] = MSG_ReadShort(&mut net_message) as libc::c_short;
    }
    if flags & (1 as libc::c_int) << 3 as libc::c_int != 0 {
        (*state).pmove.pm_time = MSG_ReadByte(&mut net_message) as byte;
    }
    if flags & (1 as libc::c_int) << 4 as libc::c_int != 0 {
        (*state).pmove.pm_flags = MSG_ReadByte(&mut net_message) as byte;
    }
    if flags & (1 as libc::c_int) << 5 as libc::c_int != 0 {
        (*state).pmove.gravity = MSG_ReadShort(&mut net_message) as libc::c_short;
    }
    if flags & (1 as libc::c_int) << 6 as libc::c_int != 0 {
        (*state)
            .pmove
            .delta_angles[0 as libc::c_int
            as usize] = MSG_ReadShort(&mut net_message) as libc::c_short;
        (*state)
            .pmove
            .delta_angles[1 as libc::c_int
            as usize] = MSG_ReadShort(&mut net_message) as libc::c_short;
        (*state)
            .pmove
            .delta_angles[2 as libc::c_int
            as usize] = MSG_ReadShort(&mut net_message) as libc::c_short;
    }
    if cl.attractloop as u64 != 0 {
        (*state).pmove.pm_type = PM_FREEZE;
    }
    if flags & (1 as libc::c_int) << 7 as libc::c_int != 0 {
        (*state)
            .viewoffset[0 as libc::c_int
            as usize] = (MSG_ReadChar(&mut net_message) as libc::c_double * 0.25f64)
            as vec_t;
        (*state)
            .viewoffset[1 as libc::c_int
            as usize] = (MSG_ReadChar(&mut net_message) as libc::c_double * 0.25f64)
            as vec_t;
        (*state)
            .viewoffset[2 as libc::c_int
            as usize] = (MSG_ReadChar(&mut net_message) as libc::c_double * 0.25f64)
            as vec_t;
    }
    if flags & (1 as libc::c_int) << 8 as libc::c_int != 0 {
        (*state)
            .viewangles[0 as libc::c_int as usize] = MSG_ReadAngle16(&mut net_message);
        (*state)
            .viewangles[1 as libc::c_int as usize] = MSG_ReadAngle16(&mut net_message);
        (*state)
            .viewangles[2 as libc::c_int as usize] = MSG_ReadAngle16(&mut net_message);
    }
    if flags & (1 as libc::c_int) << 9 as libc::c_int != 0 {
        (*state)
            .kick_angles[0 as libc::c_int
            as usize] = (MSG_ReadChar(&mut net_message) as libc::c_double * 0.25f64)
            as vec_t;
        (*state)
            .kick_angles[1 as libc::c_int
            as usize] = (MSG_ReadChar(&mut net_message) as libc::c_double * 0.25f64)
            as vec_t;
        (*state)
            .kick_angles[2 as libc::c_int
            as usize] = (MSG_ReadChar(&mut net_message) as libc::c_double * 0.25f64)
            as vec_t;
    }
    if flags & (1 as libc::c_int) << 12 as libc::c_int != 0 {
        (*state).gunindex = MSG_ReadByte(&mut net_message);
    }
    if flags & (1 as libc::c_int) << 13 as libc::c_int != 0 {
        (*state).gunframe = MSG_ReadByte(&mut net_message);
        (*state)
            .gunoffset[0 as libc::c_int
            as usize] = (MSG_ReadChar(&mut net_message) as libc::c_double * 0.25f64)
            as vec_t;
        (*state)
            .gunoffset[1 as libc::c_int
            as usize] = (MSG_ReadChar(&mut net_message) as libc::c_double * 0.25f64)
            as vec_t;
        (*state)
            .gunoffset[2 as libc::c_int
            as usize] = (MSG_ReadChar(&mut net_message) as libc::c_double * 0.25f64)
            as vec_t;
        (*state)
            .gunangles[0 as libc::c_int
            as usize] = (MSG_ReadChar(&mut net_message) as libc::c_double * 0.25f64)
            as vec_t;
        (*state)
            .gunangles[1 as libc::c_int
            as usize] = (MSG_ReadChar(&mut net_message) as libc::c_double * 0.25f64)
            as vec_t;
        (*state)
            .gunangles[2 as libc::c_int
            as usize] = (MSG_ReadChar(&mut net_message) as libc::c_double * 0.25f64)
            as vec_t;
    }
    if flags & (1 as libc::c_int) << 10 as libc::c_int != 0 {
        (*state)
            .blend[0 as libc::c_int
            as usize] = (MSG_ReadByte(&mut net_message) as libc::c_double / 255.0f64)
            as libc::c_float;
        (*state)
            .blend[1 as libc::c_int
            as usize] = (MSG_ReadByte(&mut net_message) as libc::c_double / 255.0f64)
            as libc::c_float;
        (*state)
            .blend[2 as libc::c_int
            as usize] = (MSG_ReadByte(&mut net_message) as libc::c_double / 255.0f64)
            as libc::c_float;
        (*state)
            .blend[3 as libc::c_int
            as usize] = (MSG_ReadByte(&mut net_message) as libc::c_double / 255.0f64)
            as libc::c_float;
    }
    if flags & (1 as libc::c_int) << 11 as libc::c_int != 0 {
        (*state).fov = MSG_ReadByte(&mut net_message) as libc::c_float;
    }
    if flags & (1 as libc::c_int) << 14 as libc::c_int != 0 {
        (*state).rdflags = MSG_ReadByte(&mut net_message);
    }
    statbits = MSG_ReadLong(&mut net_message);
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        if statbits & (1 as libc::c_int) << i != 0 {
            (*state)
                .stats[i as usize] = MSG_ReadShort(&mut net_message) as libc::c_short;
        }
        i += 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn CL_FireEntityEvents(mut frame: *mut frame_t) {
    let mut s1: *mut entity_state_t = 0 as *mut entity_state_t;
    let mut pnum: libc::c_int = 0;
    let mut num: libc::c_int = 0;
    pnum = 0 as libc::c_int;
    while pnum < (*frame).num_entities {
        num = (*frame).parse_entities + pnum & 1024 as libc::c_int - 1 as libc::c_int;
        s1 = &mut *cl_parse_entities.as_mut_ptr().offset(num as isize)
            as *mut entity_state_t;
        if (*s1).event != 0 {
            CL_EntityEvent(s1);
        }
        if (*s1).effects & 0x20000 as libc::c_int as libc::c_uint != 0 {
            CL_TeleporterParticles(s1);
        }
        pnum += 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn CL_ParseFrame() {
    let mut cmd: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut old: *mut frame_t = 0 as *mut frame_t;
    memset(
        &mut cl.frame as *mut frame_t as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<frame_t>() as libc::c_ulong,
    );
    cl.frame.serverframe = MSG_ReadLong(&mut net_message);
    cl.frame.deltaframe = MSG_ReadLong(&mut net_message);
    cl.frame.servertime = cl.frame.serverframe * 100 as libc::c_int;
    if cls.serverProtocol != 26 as libc::c_int {
        cl.surpressCount = MSG_ReadByte(&mut net_message);
    }
    if (*cl_shownet).value == 3 as libc::c_int as libc::c_float {
        Com_Printf(
            b"   frame:%i  delta:%i\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            cl.frame.serverframe,
            cl.frame.deltaframe,
        );
    }
    if cl.frame.deltaframe <= 0 as libc::c_int {
        cl.frame.valid = true_0;
        old = 0 as *mut frame_t;
        cls.demowaiting = false_0;
    } else {
        old = &mut *(cl.frames)
            .as_mut_ptr()
            .offset(
                (cl.frame.deltaframe & 16 as libc::c_int - 1 as libc::c_int) as isize,
            ) as *mut frame_t;
        if (*old).valid as u64 == 0 {
            Com_Printf(
                b"Delta from invalid frame (not supposed to happen!).\n\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
            );
        }
        if (*old).serverframe != cl.frame.deltaframe {
            Com_Printf(
                b"Delta frame too old.\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
        } else if cl.parse_entities - (*old).parse_entities
            > 1024 as libc::c_int - 128 as libc::c_int
        {
            Com_Printf(
                b"Delta parse_entities too old.\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
        } else {
            cl.frame.valid = true_0;
        }
    }
    if cl.time > cl.frame.servertime {
        cl.time = cl.frame.servertime;
    } else if cl.time < cl.frame.servertime - 100 as libc::c_int {
        cl.time = cl.frame.servertime - 100 as libc::c_int;
    }
    len = MSG_ReadByte(&mut net_message);
    MSG_ReadData(
        &mut net_message,
        &mut cl.frame.areabits as *mut [byte; 32] as *mut libc::c_void,
        len,
    );
    cmd = MSG_ReadByte(&mut net_message);
    SHOWNET(svc_strings[cmd as usize]);
    if cmd != svc_playerinfo as libc::c_int {
        Com_Error(
            1 as libc::c_int,
            b"CL_ParseFrame: not playerinfo\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    CL_ParsePlayerstate(old, &mut cl.frame);
    cmd = MSG_ReadByte(&mut net_message);
    SHOWNET(svc_strings[cmd as usize]);
    if cmd != svc_packetentities as libc::c_int {
        Com_Error(
            1 as libc::c_int,
            b"CL_ParseFrame: not packetentities\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    CL_ParsePacketEntities(old, &mut cl.frame);
    cl
        .frames[(cl.frame.serverframe & 16 as libc::c_int - 1 as libc::c_int)
        as usize] = cl.frame;
    if cl.frame.valid as u64 != 0 {
        if cls.state as libc::c_uint != ca_active as libc::c_int as libc::c_uint {
            cls.state = ca_active;
            cl.force_refdef = true_0;
            cl
                .predicted_origin[0 as libc::c_int
                as usize] = (cl.frame.playerstate.pmove.origin[0 as libc::c_int as usize]
                as libc::c_int as libc::c_double * 0.125f64) as vec_t;
            cl
                .predicted_origin[1 as libc::c_int
                as usize] = (cl.frame.playerstate.pmove.origin[1 as libc::c_int as usize]
                as libc::c_int as libc::c_double * 0.125f64) as vec_t;
            cl
                .predicted_origin[2 as libc::c_int
                as usize] = (cl.frame.playerstate.pmove.origin[2 as libc::c_int as usize]
                as libc::c_int as libc::c_double * 0.125f64) as vec_t;
            cl
                .predicted_angles[0 as libc::c_int
                as usize] = cl.frame.playerstate.viewangles[0 as libc::c_int as usize];
            cl
                .predicted_angles[1 as libc::c_int
                as usize] = cl.frame.playerstate.viewangles[1 as libc::c_int as usize];
            cl
                .predicted_angles[2 as libc::c_int
                as usize] = cl.frame.playerstate.viewangles[2 as libc::c_int as usize];
            if cls.disable_servercount != cl.servercount
                && cl.refresh_prepped as libc::c_uint != 0
            {
                SCR_EndLoadingPlaque();
            }
        }
        cl.sound_prepped = true_0;
        CL_FireEntityEvents(&mut cl.frame);
        CL_CheckPredictionError();
    }
}

#[no_mangle]
pub unsafe extern "C" fn S_RegisterSexedModel(
    mut ent: *mut entity_state_t,
    mut base: *mut libc::c_char,
) -> *mut model_s {
    let mut n: libc::c_int = 0;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut mdl: *mut model_s = 0 as *mut model_s;
    let mut model: [libc::c_char; 64] = [0; 64];
    let mut buffer: [libc::c_char; 64] = [0; 64];
    model[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    n = 32 as libc::c_int + 256 as libc::c_int + 256 as libc::c_int + 256 as libc::c_int
        + 256 as libc::c_int + 256 as libc::c_int + (*ent).number - 1 as libc::c_int;
    if cl.configstrings[n as usize][0 as libc::c_int as usize] != 0 {
        p = strchr((cl.configstrings[n as usize]).as_mut_ptr(), '\\' as i32);
        if !p.is_null() {
            p = p.offset(1 as libc::c_int as isize);
            strcpy(model.as_mut_ptr(), p);
            p = strchr(model.as_mut_ptr(), '/' as i32);
            if !p.is_null() {
                *p = 0 as libc::c_int as libc::c_char;
            }
        }
    }
    if model[0 as libc::c_int as usize] == 0 {
        strcpy(model.as_mut_ptr(), b"male\0" as *const u8 as *const libc::c_char);
    }
    Com_sprintf(
        buffer.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
        b"players/%s/%s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        model.as_mut_ptr(),
        base.offset(1 as libc::c_int as isize),
    );
    mdl = (re.RegisterModel).expect("non-null function pointer")(buffer.as_mut_ptr());
    if mdl.is_null() {
        Com_sprintf(
            buffer.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
            b"players/%s/weapon.md2\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            model.as_mut_ptr(),
        );
        mdl = (re.RegisterModel)
            .expect("non-null function pointer")(buffer.as_mut_ptr());
        if mdl.is_null() {
            Com_sprintf(
                buffer.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong
                    as libc::c_int,
                b"players/%s/%s\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                b"male\0" as *const u8 as *const libc::c_char,
                base.offset(1 as libc::c_int as isize),
            );
            mdl = (re.RegisterModel)
                .expect("non-null function pointer")(buffer.as_mut_ptr());
            if mdl.is_null() {
                Com_sprintf(
                    buffer.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong
                        as libc::c_int,
                    b"players/male/weapon.md2\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                );
                mdl = (re.RegisterModel)
                    .expect("non-null function pointer")(buffer.as_mut_ptr());
            }
        }
    }
    return mdl;
}

#[no_mangle]
pub unsafe extern "C" fn CL_AddPacketEntities(mut frame: *mut frame_t) {
    let mut ent: entity_t = entity_t {
        model: 0 as *mut model_s,
        angles: [0.; 3],
        origin: [0.; 3],
        frame: 0,
        oldorigin: [0.; 3],
        oldframe: 0,
        backlerp: 0.,
        skinnum: 0,
        lightstyle: 0,
        alpha: 0.,
        skin: 0 as *mut image_s,
        flags: 0,
    };
    let mut s1: *mut entity_state_t = 0 as *mut entity_state_t;
    let mut autorotate: libc::c_float = 0.;
    let mut i: libc::c_int = 0;
    let mut pnum: libc::c_int = 0;
    let mut cent: *mut centity_t = 0 as *mut centity_t;
    let mut autoanim: libc::c_int = 0;
    let mut ci: *mut clientinfo_t = 0 as *mut clientinfo_t;
    let mut effects: libc::c_uint = 0;
    let mut renderfx: libc::c_uint = 0;
    autorotate = anglemod((cl.time / 10 as libc::c_int) as libc::c_float);
    autoanim = 2 as libc::c_int * cl.time / 1000 as libc::c_int;
    memset(
        &mut ent as *mut entity_t as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<entity_t>() as libc::c_ulong,
    );
    pnum = 0 as libc::c_int;
    while pnum < (*frame).num_entities {
        s1 = &mut *cl_parse_entities
            .as_mut_ptr()
            .offset(
                ((*frame).parse_entities + pnum & 1024 as libc::c_int - 1 as libc::c_int)
                    as isize,
            ) as *mut entity_state_t;
        cent = &mut *cl_entities.as_mut_ptr().offset((*s1).number as isize)
            as *mut centity_t;
        effects = (*s1).effects;
        renderfx = (*s1).renderfx as libc::c_uint;
        if effects & 0x400 as libc::c_int as libc::c_uint != 0 {
            ent.frame = autoanim & 1 as libc::c_int;
        } else if effects & 0x800 as libc::c_int as libc::c_uint != 0 {
            ent.frame = 2 as libc::c_int + (autoanim & 1 as libc::c_int);
        } else if effects & 0x1000 as libc::c_int as libc::c_uint != 0 {
            ent.frame = autoanim;
        } else if effects & 0x2000 as libc::c_int as libc::c_uint != 0 {
            ent.frame = cl.time / 100 as libc::c_int;
        } else {
            ent.frame = (*s1).frame;
        }
        if effects & 0x10000 as libc::c_int as libc::c_uint != 0 {
            effects &= !(0x10000 as libc::c_int) as libc::c_uint;
            effects |= 0x100 as libc::c_int as libc::c_uint;
            renderfx |= 1024 as libc::c_int as libc::c_uint;
        }
        if effects & 0x8000 as libc::c_int as libc::c_uint != 0 {
            effects &= !(0x8000 as libc::c_int) as libc::c_uint;
            effects |= 0x100 as libc::c_int as libc::c_uint;
            renderfx |= 4096 as libc::c_int as libc::c_uint;
        }
        if effects & 0x8000000 as libc::c_int as libc::c_uint != 0 {
            effects &= !(0x8000000 as libc::c_int) as libc::c_uint;
            effects |= 0x100 as libc::c_int as libc::c_uint;
            renderfx |= 0x10000 as libc::c_int as libc::c_uint;
        }
        if effects & 0x40000000 as libc::c_int as libc::c_uint != 0 {
            effects &= !(0x40000000 as libc::c_int) as libc::c_uint;
            effects |= 0x100 as libc::c_int as libc::c_uint;
            renderfx |= 0x20000 as libc::c_int as libc::c_uint;
        }
        ent.oldframe = (*cent).prev.frame;
        ent.backlerp = (1.0f64 - cl.lerpfrac as libc::c_double) as libc::c_float;
        if renderfx & (64 as libc::c_int | 128 as libc::c_int) as libc::c_uint != 0 {
            ent
                .origin[0 as libc::c_int
                as usize] = (*cent).current.origin[0 as libc::c_int as usize];
            ent
                .origin[1 as libc::c_int
                as usize] = (*cent).current.origin[1 as libc::c_int as usize];
            ent
                .origin[2 as libc::c_int
                as usize] = (*cent).current.origin[2 as libc::c_int as usize];
            ent
                .oldorigin[0 as libc::c_int
                as usize] = (*cent).current.old_origin[0 as libc::c_int as usize];
            ent
                .oldorigin[1 as libc::c_int
                as usize] = (*cent).current.old_origin[1 as libc::c_int as usize];
            ent
                .oldorigin[2 as libc::c_int
                as usize] = (*cent).current.old_origin[2 as libc::c_int as usize];
        } else {
            i = 0 as libc::c_int;
            while i < 3 as libc::c_int {
                ent
                    .oldorigin[i
                    as usize] = (*cent).prev.origin[i as usize]
                    + cl.lerpfrac
                    * ((*cent).current.origin[i as usize]
                    - (*cent).prev.origin[i as usize]);
                ent.origin[i as usize] = ent.oldorigin[i as usize];
                i += 1;
            }
        }
        if renderfx & 128 as libc::c_int as libc::c_uint != 0 {
            ent.alpha = 0.30f64 as libc::c_float;
            ent
                .skinnum = (*s1).skinnum >> rand() % 4 as libc::c_int * 8 as libc::c_int
                & 0xff as libc::c_int;
            ent.model = 0 as *mut model_s;
        } else if (*s1).modelindex == 255 as libc::c_int {
            ent.skinnum = 0 as libc::c_int;
            ci = &mut *(cl.clientinfo)
                .as_mut_ptr()
                .offset(((*s1).skinnum & 0xff as libc::c_int) as isize)
                as *mut clientinfo_t;
            ent.skin = (*ci).skin;
            ent.model = (*ci).model;
            if (ent.skin).is_null() || (ent.model).is_null() {
                ent.skin = cl.baseclientinfo.skin;
                ent.model = cl.baseclientinfo.model;
            }
            if renderfx & 0x40000 as libc::c_int as libc::c_uint != 0 {
                if strncmp(
                    ent.skin as *mut libc::c_char,
                    b"players/male\0" as *const u8 as *const libc::c_char,
                    12 as libc::c_int as libc::c_ulong,
                ) == 0
                {
                    ent
                        .skin = (re.RegisterSkin)
                        .expect(
                            "non-null function pointer",
                        )(
                        b"players/male/disguise.pcx\0" as *const u8
                            as *const libc::c_char as *mut libc::c_char,
                    );
                    ent
                        .model = (re.RegisterModel)
                        .expect(
                            "non-null function pointer",
                        )(
                        b"players/male/tris.md2\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    );
                } else if strncmp(
                    ent.skin as *mut libc::c_char,
                    b"players/female\0" as *const u8 as *const libc::c_char,
                    14 as libc::c_int as libc::c_ulong,
                ) == 0
                {
                    ent
                        .skin = (re.RegisterSkin)
                        .expect(
                            "non-null function pointer",
                        )(
                        b"players/female/disguise.pcx\0" as *const u8
                            as *const libc::c_char as *mut libc::c_char,
                    );
                    ent
                        .model = (re.RegisterModel)
                        .expect(
                            "non-null function pointer",
                        )(
                        b"players/female/tris.md2\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    );
                } else if strncmp(
                    ent.skin as *mut libc::c_char,
                    b"players/cyborg\0" as *const u8 as *const libc::c_char,
                    14 as libc::c_int as libc::c_ulong,
                ) == 0
                {
                    ent
                        .skin = (re.RegisterSkin)
                        .expect(
                            "non-null function pointer",
                        )(
                        b"players/cyborg/disguise.pcx\0" as *const u8
                            as *const libc::c_char as *mut libc::c_char,
                    );
                    ent
                        .model = (re.RegisterModel)
                        .expect(
                            "non-null function pointer",
                        )(
                        b"players/cyborg/tris.md2\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    );
                }
            }
        } else {
            ent.skinnum = (*s1).skinnum;
            ent.skin = 0 as *mut image_s;
            ent.model = cl.model_draw[(*s1).modelindex as usize];
        }
        if renderfx == 32 as libc::c_int as libc::c_uint {
            ent.alpha = 0.70f64 as libc::c_float;
        }
        if effects & 0x100 as libc::c_int as libc::c_uint != 0 {
            ent.flags = 0 as libc::c_int;
        } else {
            ent.flags = renderfx as libc::c_int;
        }
        if effects & 0x1 as libc::c_int as libc::c_uint != 0 {
            ent.angles[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_float;
            ent.angles[1 as libc::c_int as usize] = autorotate;
            ent.angles[2 as libc::c_int as usize] = 0 as libc::c_int as libc::c_float;
        } else if effects & 0x800000 as libc::c_int as libc::c_uint != 0 {
            ent.angles[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_float;
            ent
                .angles[1 as libc::c_int
                as usize] = anglemod((cl.time / 2 as libc::c_int) as libc::c_float)
                + (*s1).angles[1 as libc::c_int as usize];
            ent.angles[2 as libc::c_int as usize] = 180 as libc::c_int as libc::c_float;
            let mut forward: vec3_t = [0.; 3];
            let mut start: vec3_t = [0.; 3];
            AngleVectors(
                (ent.angles).as_mut_ptr(),
                forward.as_mut_ptr(),
                0 as *mut vec_t,
                0 as *mut vec_t,
            );
            VectorMA(
                (ent.origin).as_mut_ptr(),
                64 as libc::c_int as libc::c_float,
                forward.as_mut_ptr(),
                start.as_mut_ptr(),
            );
            V_AddLight(
                start.as_mut_ptr(),
                100 as libc::c_int as libc::c_float,
                1 as libc::c_int as libc::c_float,
                0 as libc::c_int as libc::c_float,
                0 as libc::c_int as libc::c_float,
            );
        } else {
            let mut a1: libc::c_float = 0.;
            let mut a2: libc::c_float = 0.;
            i = 0 as libc::c_int;
            while i < 3 as libc::c_int {
                a1 = (*cent).current.angles[i as usize];
                a2 = (*cent).prev.angles[i as usize];
                ent.angles[i as usize] = LerpAngle(a2, a1, cl.lerpfrac);
                i += 1;
            }
        }
        if (*s1).number == cl.playernum + 1 as libc::c_int {
            ent.flags |= 2 as libc::c_int;
            if effects & 0x40000 as libc::c_int as libc::c_uint != 0 {
                V_AddLight(
                    (ent.origin).as_mut_ptr(),
                    225 as libc::c_int as libc::c_float,
                    1.0f64 as libc::c_float,
                    0.1f64 as libc::c_float,
                    0.1f64 as libc::c_float,
                );
            } else if effects & 0x80000 as libc::c_int as libc::c_uint != 0 {
                V_AddLight(
                    (ent.origin).as_mut_ptr(),
                    225 as libc::c_int as libc::c_float,
                    0.1f64 as libc::c_float,
                    0.1f64 as libc::c_float,
                    1.0f64 as libc::c_float,
                );
            } else if effects & 0x20000000 as libc::c_int as libc::c_uint != 0 {
                V_AddLight(
                    (ent.origin).as_mut_ptr(),
                    225 as libc::c_int as libc::c_float,
                    1.0f64 as libc::c_float,
                    1.0f64 as libc::c_float,
                    0.0f64 as libc::c_float,
                );
            } else if effects & 0x80000000 as libc::c_uint != 0 {
                V_AddLight(
                    (ent.origin).as_mut_ptr(),
                    225 as libc::c_int as libc::c_float,
                    -1.0f64 as libc::c_float,
                    -1.0f64 as libc::c_float,
                    -1.0f64 as libc::c_float,
                );
            }
        } else if !((*s1).modelindex == 0) {
            if effects & 0x80 as libc::c_int as libc::c_uint != 0 {
                ent.flags |= 32 as libc::c_int;
                ent.alpha = 0.30f64 as libc::c_float;
            }
            if effects & 0x1000000 as libc::c_int as libc::c_uint != 0 {
                ent.flags |= 32 as libc::c_int;
                ent.alpha = 0.6f64 as libc::c_float;
            }
            if effects & 0x10000000 as libc::c_int as libc::c_uint != 0 {
                ent.flags |= 32 as libc::c_int;
                if effects & 0x80000000 as libc::c_uint != 0 {
                    ent.alpha = 0.6f64 as libc::c_float;
                } else {
                    ent.alpha = 0.3f64 as libc::c_float;
                }
            }
            V_AddEntity(&mut ent);
            if effects & 0x100 as libc::c_int as libc::c_uint != 0 {
                ent
                    .flags = (renderfx | 32 as libc::c_int as libc::c_uint)
                    as libc::c_int;
                ent.alpha = 0.30f64 as libc::c_float;
                V_AddEntity(&mut ent);
            }
            ent.skin = 0 as *mut image_s;
            ent.skinnum = 0 as libc::c_int;
            ent.flags = 0 as libc::c_int;
            ent.alpha = 0 as libc::c_int as libc::c_float;
            if (*s1).modelindex2 != 0 {
                if (*s1).modelindex2 == 255 as libc::c_int {
                    ci = &mut *(cl.clientinfo)
                        .as_mut_ptr()
                        .offset(((*s1).skinnum & 0xff as libc::c_int) as isize)
                        as *mut clientinfo_t;
                    i = (*s1).skinnum >> 8 as libc::c_int;
                    if (*cl_vwep).value == 0. || i > 20 as libc::c_int - 1 as libc::c_int
                    {
                        i = 0 as libc::c_int;
                    }
                    ent.model = (*ci).weaponmodel[i as usize];
                    if (ent.model).is_null() {
                        if i != 0 as libc::c_int {
                            ent.model = (*ci).weaponmodel[0 as libc::c_int as usize];
                        }
                        if (ent.model).is_null() {
                            ent
                                .model = cl
                                .baseclientinfo
                                .weaponmodel[0 as libc::c_int as usize];
                        }
                    }
                } else if (*s1).modelindex2 & 0x80 as libc::c_int != 0 {
                    ent
                        .model = cl
                        .model_draw[((*s1).modelindex2 & 0x7f as libc::c_int) as usize];
                    ent.alpha = 0.32f64 as libc::c_float;
                    ent.flags = 32 as libc::c_int;
                } else {
                    ent.model = cl.model_draw[(*s1).modelindex2 as usize];
                }
                V_AddEntity(&mut ent);
                ent.flags = 0 as libc::c_int;
                ent.alpha = 0 as libc::c_int as libc::c_float;
            }
            if (*s1).modelindex3 != 0 {
                ent.model = cl.model_draw[(*s1).modelindex3 as usize];
                V_AddEntity(&mut ent);
            }
            if (*s1).modelindex4 != 0 {
                ent.model = cl.model_draw[(*s1).modelindex4 as usize];
                V_AddEntity(&mut ent);
            }
            if effects & 0x200 as libc::c_int as libc::c_uint != 0 {
                ent.model = cl_mod_powerscreen;
                ent.oldframe = 0 as libc::c_int;
                ent.frame = 0 as libc::c_int;
                ent.flags |= 32 as libc::c_int | 2048 as libc::c_int;
                ent.alpha = 0.30f64 as libc::c_float;
                V_AddEntity(&mut ent);
            }
            if effects & !(0x1 as libc::c_int) as libc::c_uint != 0 {
                if effects & 0x10 as libc::c_int as libc::c_uint != 0 {
                    CL_RocketTrail(
                        ((*cent).lerp_origin).as_mut_ptr(),
                        (ent.origin).as_mut_ptr(),
                        cent,
                    );
                    V_AddLight(
                        (ent.origin).as_mut_ptr(),
                        200 as libc::c_int as libc::c_float,
                        1 as libc::c_int as libc::c_float,
                        1 as libc::c_int as libc::c_float,
                        0 as libc::c_int as libc::c_float,
                    );
                } else if effects & 0x8 as libc::c_int as libc::c_uint != 0 {
                    if effects & 0x4000000 as libc::c_int as libc::c_uint != 0 {
                        CL_BlasterTrail2(
                            ((*cent).lerp_origin).as_mut_ptr(),
                            (ent.origin).as_mut_ptr(),
                        );
                        V_AddLight(
                            (ent.origin).as_mut_ptr(),
                            200 as libc::c_int as libc::c_float,
                            0 as libc::c_int as libc::c_float,
                            1 as libc::c_int as libc::c_float,
                            0 as libc::c_int as libc::c_float,
                        );
                    } else {
                        CL_BlasterTrail(
                            ((*cent).lerp_origin).as_mut_ptr(),
                            (ent.origin).as_mut_ptr(),
                        );
                        V_AddLight(
                            (ent.origin).as_mut_ptr(),
                            200 as libc::c_int as libc::c_float,
                            1 as libc::c_int as libc::c_float,
                            1 as libc::c_int as libc::c_float,
                            0 as libc::c_int as libc::c_float,
                        );
                    }
                } else if effects & 0x40 as libc::c_int as libc::c_uint != 0 {
                    if effects & 0x4000000 as libc::c_int as libc::c_uint != 0 {
                        V_AddLight(
                            (ent.origin).as_mut_ptr(),
                            200 as libc::c_int as libc::c_float,
                            0 as libc::c_int as libc::c_float,
                            1 as libc::c_int as libc::c_float,
                            0 as libc::c_int as libc::c_float,
                        );
                    } else {
                        V_AddLight(
                            (ent.origin).as_mut_ptr(),
                            200 as libc::c_int as libc::c_float,
                            1 as libc::c_int as libc::c_float,
                            1 as libc::c_int as libc::c_float,
                            0 as libc::c_int as libc::c_float,
                        );
                    }
                } else if effects & 0x2 as libc::c_int as libc::c_uint != 0 {
                    CL_DiminishingTrail(
                        ((*cent).lerp_origin).as_mut_ptr(),
                        (ent.origin).as_mut_ptr(),
                        cent,
                        effects as libc::c_int,
                    );
                } else if effects & 0x20 as libc::c_int as libc::c_uint != 0 {
                    CL_DiminishingTrail(
                        ((*cent).lerp_origin).as_mut_ptr(),
                        (ent.origin).as_mut_ptr(),
                        cent,
                        effects as libc::c_int,
                    );
                } else if effects & 0x4000 as libc::c_int as libc::c_uint != 0 {
                    CL_FlyEffect(cent, (ent.origin).as_mut_ptr());
                } else if effects & 0x80 as libc::c_int as libc::c_uint != 0 {
                    static mut bfg_lightramp: [libc::c_int; 6] = [
                        300 as libc::c_int,
                        400 as libc::c_int,
                        600 as libc::c_int,
                        300 as libc::c_int,
                        150 as libc::c_int,
                        75 as libc::c_int,
                    ];
                    if effects & 0x2000 as libc::c_int as libc::c_uint != 0 {
                        CL_BfgParticles(&mut ent);
                        i = 200 as libc::c_int;
                    } else {
                        i = bfg_lightramp[(*s1).frame as usize];
                    }
                    V_AddLight(
                        (ent.origin).as_mut_ptr(),
                        i as libc::c_float,
                        0 as libc::c_int as libc::c_float,
                        1 as libc::c_int as libc::c_float,
                        0 as libc::c_int as libc::c_float,
                    );
                } else if effects & 0x2000000 as libc::c_int as libc::c_uint != 0 {
                    ent.origin[2 as libc::c_int as usize]
                        += 32 as libc::c_int as libc::c_float;
                    CL_TrapParticles(&mut ent);
                    i = rand() % 100 as libc::c_int + 100 as libc::c_int;
                    V_AddLight(
                        (ent.origin).as_mut_ptr(),
                        i as libc::c_float,
                        1 as libc::c_int as libc::c_float,
                        0.8f64 as libc::c_float,
                        0.1f64 as libc::c_float,
                    );
                } else if effects & 0x40000 as libc::c_int as libc::c_uint != 0 {
                    CL_FlagTrail(
                        ((*cent).lerp_origin).as_mut_ptr(),
                        (ent.origin).as_mut_ptr(),
                        242 as libc::c_int as libc::c_float,
                    );
                    V_AddLight(
                        (ent.origin).as_mut_ptr(),
                        225 as libc::c_int as libc::c_float,
                        1 as libc::c_int as libc::c_float,
                        0.1f64 as libc::c_float,
                        0.1f64 as libc::c_float,
                    );
                } else if effects & 0x80000 as libc::c_int as libc::c_uint != 0 {
                    CL_FlagTrail(
                        ((*cent).lerp_origin).as_mut_ptr(),
                        (ent.origin).as_mut_ptr(),
                        115 as libc::c_int as libc::c_float,
                    );
                    V_AddLight(
                        (ent.origin).as_mut_ptr(),
                        225 as libc::c_int as libc::c_float,
                        0.1f64 as libc::c_float,
                        0.1f64 as libc::c_float,
                        1 as libc::c_int as libc::c_float,
                    );
                } else if effects & 0x20000000 as libc::c_int as libc::c_uint != 0 {
                    CL_TagTrail(
                        ((*cent).lerp_origin).as_mut_ptr(),
                        (ent.origin).as_mut_ptr(),
                        220 as libc::c_int as libc::c_float,
                    );
                    V_AddLight(
                        (ent.origin).as_mut_ptr(),
                        225 as libc::c_int as libc::c_float,
                        1.0f64 as libc::c_float,
                        1.0f64 as libc::c_float,
                        0.0f64 as libc::c_float,
                    );
                } else if effects & 0x80000000 as libc::c_uint != 0 {
                    if effects & 0x4000000 as libc::c_int as libc::c_uint != 0 {
                        let mut intensity: libc::c_float = 0.;
                        intensity = (50 as libc::c_int as libc::c_double
                            + 500 as libc::c_int as libc::c_double
                            * (sin(cl.time as libc::c_double / 500.0f64) + 1.0f64))
                            as libc::c_float;
                        if vidref_val == 1 as libc::c_int {
                            V_AddLight(
                                (ent.origin).as_mut_ptr(),
                                intensity,
                                -1.0f64 as libc::c_float,
                                -1.0f64 as libc::c_float,
                                -1.0f64 as libc::c_float,
                            );
                        } else {
                            V_AddLight(
                                (ent.origin).as_mut_ptr(),
                                (-1.0f64 * intensity as libc::c_double) as libc::c_float,
                                1.0f64 as libc::c_float,
                                1.0f64 as libc::c_float,
                                1.0f64 as libc::c_float,
                            );
                        }
                    } else {
                        CL_Tracker_Shell(((*cent).lerp_origin).as_mut_ptr());
                        V_AddLight(
                            (ent.origin).as_mut_ptr(),
                            155 as libc::c_int as libc::c_float,
                            -1.0f64 as libc::c_float,
                            -1.0f64 as libc::c_float,
                            -1.0f64 as libc::c_float,
                        );
                    }
                } else if effects & 0x4000000 as libc::c_int as libc::c_uint != 0 {
                    CL_TrackerTrail(
                        ((*cent).lerp_origin).as_mut_ptr(),
                        (ent.origin).as_mut_ptr(),
                        0 as libc::c_int,
                    );
                    if vidref_val == 1 as libc::c_int {
                        V_AddLight(
                            (ent.origin).as_mut_ptr(),
                            200 as libc::c_int as libc::c_float,
                            -(1 as libc::c_int) as libc::c_float,
                            -(1 as libc::c_int) as libc::c_float,
                            -(1 as libc::c_int) as libc::c_float,
                        );
                    } else {
                        V_AddLight(
                            (ent.origin).as_mut_ptr(),
                            -(200 as libc::c_int) as libc::c_float,
                            1 as libc::c_int as libc::c_float,
                            1 as libc::c_int as libc::c_float,
                            1 as libc::c_int as libc::c_float,
                        );
                    }
                } else if effects & 0x200000 as libc::c_int as libc::c_uint != 0 {
                    CL_DiminishingTrail(
                        ((*cent).lerp_origin).as_mut_ptr(),
                        (ent.origin).as_mut_ptr(),
                        cent,
                        effects as libc::c_int,
                    );
                } else if effects & 0x100000 as libc::c_int as libc::c_uint != 0 {
                    CL_IonripperTrail(
                        ((*cent).lerp_origin).as_mut_ptr(),
                        (ent.origin).as_mut_ptr(),
                    );
                    V_AddLight(
                        (ent.origin).as_mut_ptr(),
                        100 as libc::c_int as libc::c_float,
                        1 as libc::c_int as libc::c_float,
                        0.5f64 as libc::c_float,
                        0.5f64 as libc::c_float,
                    );
                } else if effects & 0x400000 as libc::c_int as libc::c_uint != 0 {
                    V_AddLight(
                        (ent.origin).as_mut_ptr(),
                        200 as libc::c_int as libc::c_float,
                        0 as libc::c_int as libc::c_float,
                        0 as libc::c_int as libc::c_float,
                        1 as libc::c_int as libc::c_float,
                    );
                } else if effects & 0x1000000 as libc::c_int as libc::c_uint != 0 {
                    if effects & 0x2000 as libc::c_int as libc::c_uint != 0 {
                        CL_BlasterTrail(
                            ((*cent).lerp_origin).as_mut_ptr(),
                            (ent.origin).as_mut_ptr(),
                        );
                    }
                    V_AddLight(
                        (ent.origin).as_mut_ptr(),
                        130 as libc::c_int as libc::c_float,
                        1 as libc::c_int as libc::c_float,
                        0.5f64 as libc::c_float,
                        0.5f64 as libc::c_float,
                    );
                }
            }
            (*cent)
                .lerp_origin[0 as libc::c_int
                as usize] = ent.origin[0 as libc::c_int as usize];
            (*cent)
                .lerp_origin[1 as libc::c_int
                as usize] = ent.origin[1 as libc::c_int as usize];
            (*cent)
                .lerp_origin[2 as libc::c_int
                as usize] = ent.origin[2 as libc::c_int as usize];
        }
        pnum += 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn CL_AddViewWeapon(
    mut ps: *mut player_state_t,
    mut ops: *mut player_state_t,
) {
    let mut gun: entity_t = entity_t {
        model: 0 as *mut model_s,
        angles: [0.; 3],
        origin: [0.; 3],
        frame: 0,
        oldorigin: [0.; 3],
        oldframe: 0,
        backlerp: 0.,
        skinnum: 0,
        lightstyle: 0,
        alpha: 0.,
        skin: 0 as *mut image_s,
        flags: 0,
    };
    let mut i: libc::c_int = 0;
    if (*cl_gun).value == 0. {
        return;
    }
    if (*ps).fov > 90 as libc::c_int as libc::c_float {
        return;
    }
    memset(
        &mut gun as *mut entity_t as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<entity_t>() as libc::c_ulong,
    );
    if !gun_model.is_null() {
        gun.model = gun_model;
    } else {
        gun.model = cl.model_draw[(*ps).gunindex as usize];
    }
    if (gun.model).is_null() {
        return;
    }
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        gun
            .origin[i
            as usize] = cl.refdef.vieworg[i as usize] + (*ops).gunoffset[i as usize]
            + cl.lerpfrac * ((*ps).gunoffset[i as usize] - (*ops).gunoffset[i as usize]);
        gun
            .angles[i
            as usize] = cl.refdef.viewangles[i as usize]
            + LerpAngle(
            (*ops).gunangles[i as usize],
            (*ps).gunangles[i as usize],
            cl.lerpfrac,
        );
        i += 1;
    }
    if gun_frame != 0 {
        gun.frame = gun_frame;
        gun.oldframe = gun_frame;
    } else {
        gun.frame = (*ps).gunframe;
        if gun.frame == 0 as libc::c_int {
            gun.oldframe = 0 as libc::c_int;
        } else {
            gun.oldframe = (*ops).gunframe;
        }
    }
    gun.flags = 1 as libc::c_int | 16 as libc::c_int | 4 as libc::c_int;
    gun.backlerp = (1.0f64 - cl.lerpfrac as libc::c_double) as libc::c_float;
    gun.oldorigin[0 as libc::c_int as usize] = gun.origin[0 as libc::c_int as usize];
    gun.oldorigin[1 as libc::c_int as usize] = gun.origin[1 as libc::c_int as usize];
    gun.oldorigin[2 as libc::c_int as usize] = gun.origin[2 as libc::c_int as usize];
    V_AddEntity(&mut gun);
}

#[no_mangle]
pub unsafe extern "C" fn CL_CalcViewValues() {
    let mut i: libc::c_int = 0;
    let mut lerp: libc::c_float = 0.;
    let mut backlerp: libc::c_float = 0.;
    let mut ent: *mut centity_t = 0 as *mut centity_t;
    let mut oldframe: *mut frame_t = 0 as *mut frame_t;
    let mut ps: *mut player_state_t = 0 as *mut player_state_t;
    let mut ops: *mut player_state_t = 0 as *mut player_state_t;
    ps = &mut cl.frame.playerstate;
    i = cl.frame.serverframe - 1 as libc::c_int & 16 as libc::c_int - 1 as libc::c_int;
    oldframe = &mut *(cl.frames).as_mut_ptr().offset(i as isize) as *mut frame_t;
    if (*oldframe).serverframe != cl.frame.serverframe - 1 as libc::c_int
        || (*oldframe).valid as u64 == 0
    {
        oldframe = &mut cl.frame;
    }
    ops = &mut (*oldframe).playerstate;
    if fabs(
        ((*ops).pmove.origin[0 as libc::c_int as usize] as libc::c_int
            - (*ps).pmove.origin[0 as libc::c_int as usize] as libc::c_int)
            as libc::c_double,
    ) > (256 as libc::c_int * 8 as libc::c_int) as libc::c_double
        || abs(
        (*ops).pmove.origin[1 as libc::c_int as usize] as libc::c_int
            - (*ps).pmove.origin[1 as libc::c_int as usize] as libc::c_int,
    ) > 256 as libc::c_int * 8 as libc::c_int
        || abs(
        (*ops).pmove.origin[2 as libc::c_int as usize] as libc::c_int
            - (*ps).pmove.origin[2 as libc::c_int as usize] as libc::c_int,
    ) > 256 as libc::c_int * 8 as libc::c_int
    {
        ops = ps;
    }
    ent = &mut *cl_entities
        .as_mut_ptr()
        .offset((cl.playernum + 1 as libc::c_int) as isize) as *mut centity_t;
    lerp = cl.lerpfrac;
    if (*cl_predict).value != 0.
        && cl.frame.playerstate.pmove.pm_flags as libc::c_int & 64 as libc::c_int == 0
    {
        let mut delta: libc::c_uint = 0;
        backlerp = (1.0f64 - lerp as libc::c_double) as libc::c_float;
        i = 0 as libc::c_int;
        while i < 3 as libc::c_int {
            cl
                .refdef
                .vieworg[i
                as usize] = cl.predicted_origin[i as usize]
                + (*ops).viewoffset[i as usize]
                + cl.lerpfrac
                * ((*ps).viewoffset[i as usize] - (*ops).viewoffset[i as usize])
                - backlerp * cl.prediction_error[i as usize];
            i += 1;
        }
        delta = (cls.realtime as libc::c_uint).wrapping_sub(cl.predicted_step_time);
        if delta < 100 as libc::c_int as libc::c_uint {
            cl
                .refdef
                .vieworg[2 as libc::c_int
                as usize] = (cl.refdef.vieworg[2 as libc::c_int as usize]
                as libc::c_double
                - (cl.predicted_step
                * (100 as libc::c_int as libc::c_uint).wrapping_sub(delta)
                as libc::c_float) as libc::c_double * 0.01f64) as libc::c_float;
        }
    } else {
        i = 0 as libc::c_int;
        while i < 3 as libc::c_int {
            cl
                .refdef
                .vieworg[i
                as usize] = ((*ops).pmove.origin[i as usize] as libc::c_int
                as libc::c_double * 0.125f64
                + (*ops).viewoffset[i as usize] as libc::c_double
                + lerp as libc::c_double
                * ((*ps).pmove.origin[i as usize] as libc::c_int as libc::c_double
                * 0.125f64 + (*ps).viewoffset[i as usize] as libc::c_double
                - ((*ops).pmove.origin[i as usize] as libc::c_int
                as libc::c_double * 0.125f64
                + (*ops).viewoffset[i as usize] as libc::c_double)))
                as libc::c_float;
            i += 1;
        }
    }
    if (cl.frame.playerstate.pmove.pm_type as libc::c_uint)
        < PM_DEAD as libc::c_int as libc::c_uint
    {
        i = 0 as libc::c_int;
        while i < 3 as libc::c_int {
            cl.refdef.viewangles[i as usize] = cl.predicted_angles[i as usize];
            i += 1;
        }
    } else {
        i = 0 as libc::c_int;
        while i < 3 as libc::c_int {
            cl
                .refdef
                .viewangles[i
                as usize] = LerpAngle(
                (*ops).viewangles[i as usize],
                (*ps).viewangles[i as usize],
                lerp,
            );
            i += 1;
        }
    }
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        cl.refdef.viewangles[i as usize]
            += LerpAngle(
            (*ops).kick_angles[i as usize],
            (*ps).kick_angles[i as usize],
            lerp,
        );
        i += 1;
    }
    AngleVectors(
        (cl.refdef.viewangles).as_mut_ptr(),
        (cl.v_forward).as_mut_ptr(),
        (cl.v_right).as_mut_ptr(),
        (cl.v_up).as_mut_ptr(),
    );
    cl.refdef.fov_x = (*ops).fov + lerp * ((*ps).fov - (*ops).fov);
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        cl.refdef.blend[i as usize] = (*ps).blend[i as usize];
        i += 1;
    }
    CL_AddViewWeapon(ps, ops);
}

#[no_mangle]
pub unsafe extern "C" fn CL_AddEntities() {
    if cls.state as libc::c_uint != ca_active as libc::c_int as libc::c_uint {
        return;
    }
    if cl.time > cl.frame.servertime {
        if (*cl_showclamp).value != 0. {
            Com_Printf(
                b"high clamp %i\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                cl.time - cl.frame.servertime,
            );
        }
        cl.time = cl.frame.servertime;
        cl.lerpfrac = 1.0f64 as libc::c_float;
    } else if cl.time < cl.frame.servertime - 100 as libc::c_int {
        if (*cl_showclamp).value != 0. {
            Com_Printf(
                b"low clamp %i\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                cl.frame.servertime - 100 as libc::c_int - cl.time,
            );
        }
        cl.time = cl.frame.servertime - 100 as libc::c_int;
        cl.lerpfrac = 0 as libc::c_int as libc::c_float;
    } else {
        cl
            .lerpfrac = (1.0f64
            - (cl.frame.servertime - cl.time) as libc::c_double * 0.01f64)
            as libc::c_float;
    }
    if (*cl_timedemo).value != 0. {
        cl.lerpfrac = 1.0f64 as libc::c_float;
    }
    CL_CalcViewValues();
    CL_AddPacketEntities(&mut cl.frame);
    CL_AddTEnts();
    CL_AddParticles();
    CL_AddDLights();
    CL_AddLightStyles();
}

#[no_mangle]
pub unsafe extern "C" fn CL_GetEntitySoundOrigin(
    mut ent: libc::c_int,
    mut org: *mut vec_t,
) {
    let mut old: *mut centity_t = 0 as *mut centity_t;
    if ent < 0 as libc::c_int || ent >= 1024 as libc::c_int {
        Com_Error(
            1 as libc::c_int,
            b"CL_GetEntitySoundOrigin: bad ent\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    old = &mut *cl_entities.as_mut_ptr().offset(ent as isize) as *mut centity_t;
    *org
        .offset(
            0 as libc::c_int as isize,
        ) = (*old).lerp_origin[0 as libc::c_int as usize];
    *org
        .offset(
            1 as libc::c_int as isize,
        ) = (*old).lerp_origin[1 as libc::c_int as usize];
    *org
        .offset(
            2 as libc::c_int as isize,
        ) = (*old).lerp_origin[2 as libc::c_int as usize];
}
