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
    fn atoi(__nptr: *const libc::c_char) -> libc::c_int;
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
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn rename(__old: *const libc::c_char, __new: *const libc::c_char) -> libc::c_int;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fwrite(
        _: *const libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn fseek(
        __stream: *mut FILE,
        __off: libc::c_long,
        __whence: libc::c_int,
    ) -> libc::c_int;
    fn ftell(__stream: *mut FILE) -> libc::c_long;
    fn COM_StripExtension(in_0: *mut libc::c_char, out: *mut libc::c_char);
    fn Com_sprintf(
        dest: *mut libc::c_char,
        size: libc::c_int,
        fmt: *mut libc::c_char,
        _: ...
    );
    fn Q_stricmp(s1: *mut libc::c_char, s2: *mut libc::c_char) -> libc::c_int;
    fn va(format: *mut libc::c_char, _: ...) -> *mut libc::c_char;
    fn Com_Printf(msg: *mut libc::c_char, _: ...);
    fn SZ_Print(buf: *mut sizebuf_t, data: *mut libc::c_char);
    fn MSG_WriteByte(sb: *mut sizebuf_t, c: libc::c_int);
    fn MSG_WriteString(sb: *mut sizebuf_t, s: *mut libc::c_char);
    fn MSG_ReadByte(sb: *mut sizebuf_t) -> libc::c_int;
    fn MSG_ReadShort(sb: *mut sizebuf_t) -> libc::c_int;
    fn MSG_ReadLong(sb: *mut sizebuf_t) -> libc::c_int;
    fn MSG_ReadString(sb: *mut sizebuf_t) -> *mut libc::c_char;
    fn MSG_ReadPos(sb: *mut sizebuf_t, pos: *mut vec_t);
    fn Cbuf_AddText(text: *mut libc::c_char);
    fn Cbuf_Execute();
    fn Cmd_Argc() -> libc::c_int;
    fn Cmd_Argv(arg: libc::c_int) -> *mut libc::c_char;
    fn Cvar_Set(var_name: *mut libc::c_char, value: *mut libc::c_char) -> *mut cvar_t;
    static mut net_message: sizebuf_t;
    fn CM_InlineModel(name: *mut libc::c_char) -> *mut cmodel_t;
    fn FS_Gamedir() -> *mut libc::c_char;
    fn FS_LoadFile(
        path: *mut libc::c_char,
        buffer: *mut *mut libc::c_void,
    ) -> libc::c_int;
    fn FS_CreatePath(path: *mut libc::c_char);
    fn Com_DPrintf(fmt: *mut libc::c_char, _: ...);
    fn Com_Error(code: libc::c_int, fmt: *mut libc::c_char, _: ...);
    fn Com_ServerState() -> libc::c_int;
    fn Sys_SendKeyEvents();
    fn SCR_CenterPrint(str: *mut libc::c_char);
    fn SCR_PlayCinematic(name: *mut libc::c_char);
    fn S_StartSound(
        origin: *mut vec_t,
        entnum: libc::c_int,
        entchannel: libc::c_int,
        sfx: *mut sfx_s,
        fvol: libc::c_float,
        attenuation: libc::c_float,
        timeofs: libc::c_float,
    );
    fn S_StartLocalSound(s: *mut libc::c_char);
    fn S_BeginRegistration();
    fn S_RegisterSound(sample: *mut libc::c_char) -> *mut sfx_s;
    fn S_EndRegistration();
    static mut con: console_t;
    fn CDAudio_Play(track: libc::c_int, looping: qboolean);
    static mut cl_weaponmodels: [[libc::c_char; 64]; 20];
    static mut num_cl_weaponmodels: libc::c_int;
    static mut cl: client_state_t;
    static mut cls: client_static_t;
    static mut cl_noskins: *mut cvar_t;
    static mut cl_shownet: *mut cvar_t;
    static mut cl_vwep: *mut cvar_t;
    static mut cl_entities: [centity_t; 1024];
    fn CL_AddNetgraph();
    fn CL_ParseEntityBits(bits: *mut libc::c_uint) -> libc::c_int;
    fn CL_ParseDelta(
        from: *mut entity_state_t,
        to: *mut entity_state_t,
        number: libc::c_int,
        bits: libc::c_int,
    );
    fn CL_ParseFrame();
    fn CL_ParseTEnt();
    fn CL_SetLightstyle(i: libc::c_int);
    fn CL_ParseMuzzleFlash();
    fn CL_ParseMuzzleFlash2();
    fn CL_RequestNextDownload();
    fn CL_ClearState();
    static mut re: refexport_t;
    fn CL_RegisterTEntSounds();
    fn CL_WriteDemoMessage();
    fn CL_ParseInventory();
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

pub type cmodel_t = cmodel_s;
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

pub type clc_ops_e = libc::c_uint;

pub const clc_stringcmd: clc_ops_e = 4;
pub const clc_userinfo: clc_ops_e = 3;
pub const clc_move: clc_ops_e = 2;
pub const clc_nop: clc_ops_e = 1;
pub const clc_bad: clc_ops_e = 0;

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

#[no_mangle]
pub static mut svc_strings: [*mut libc::c_char; 256] = [
    b"svc_bad\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"svc_muzzleflash\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"svc_muzzlflash2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"svc_temp_entity\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"svc_layout\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"svc_inventory\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"svc_nop\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"svc_disconnect\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"svc_reconnect\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"svc_sound\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"svc_print\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"svc_stufftext\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"svc_serverdata\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"svc_configstring\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"svc_spawnbaseline\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"svc_centerprint\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"svc_download\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"svc_playerinfo\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"svc_packetentities\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"svc_deltapacketentities\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"svc_frame\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
];

#[no_mangle]
pub unsafe extern "C" fn CL_DownloadFileName(
    mut dest: *mut libc::c_char,
    mut destlen: libc::c_int,
    mut fn_0: *mut libc::c_char,
) {
    if strncmp(
        fn_0,
        b"players\0" as *const u8 as *const libc::c_char,
        7 as libc::c_int as libc::c_ulong,
    ) == 0 as libc::c_int
    {
        Com_sprintf(
            dest,
            destlen,
            b"%s/%s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"baseq2\0" as *const u8 as *const libc::c_char,
            fn_0,
        );
    } else {
        Com_sprintf(
            dest,
            destlen,
            b"%s/%s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            FS_Gamedir(),
            fn_0,
        );
    };
}

#[no_mangle]
pub unsafe extern "C" fn CL_CheckOrDownloadFile(
    mut filename: *mut libc::c_char,
) -> qboolean {
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut name: [libc::c_char; 128] = [0; 128];
    if !(strstr(filename, b"..\0" as *const u8 as *const libc::c_char)).is_null() {
        Com_Printf(
            b"Refusing to download a path with ..\n\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        );
        return true_0;
    }
    if FS_LoadFile(filename, 0 as *mut *mut libc::c_void) != -(1 as libc::c_int) {
        return true_0;
    }
    strcpy((cls.downloadname).as_mut_ptr(), filename);
    COM_StripExtension(
        (cls.downloadname).as_mut_ptr(),
        (cls.downloadtempname).as_mut_ptr(),
    );
    strcat(
        (cls.downloadtempname).as_mut_ptr(),
        b".tmp\0" as *const u8 as *const libc::c_char,
    );
    CL_DownloadFileName(
        name.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong as libc::c_int,
        (cls.downloadtempname).as_mut_ptr(),
    );
    fp = fopen(name.as_mut_ptr(), b"r+b\0" as *const u8 as *const libc::c_char);
    if !fp.is_null() {
        let mut len: libc::c_int = 0;
        fseek(fp, 0 as libc::c_int as libc::c_long, 2 as libc::c_int);
        len = ftell(fp) as libc::c_int;
        cls.download = fp;
        Com_Printf(
            b"Resuming %s\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            (cls.downloadname).as_mut_ptr(),
        );
        MSG_WriteByte(&mut cls.netchan.message, clc_stringcmd as libc::c_int);
        MSG_WriteString(
            &mut cls.netchan.message,
            va(
                b"download %s %i\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                (cls.downloadname).as_mut_ptr(),
                len,
            ),
        );
    } else {
        Com_Printf(
            b"Downloading %s\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            (cls.downloadname).as_mut_ptr(),
        );
        MSG_WriteByte(&mut cls.netchan.message, clc_stringcmd as libc::c_int);
        MSG_WriteString(
            &mut cls.netchan.message,
            va(
                b"download %s\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                (cls.downloadname).as_mut_ptr(),
            ),
        );
    }
    cls.downloadnumber += 1;
    return false_0;
}

#[no_mangle]
pub unsafe extern "C" fn CL_Download_f() {
    let mut filename: [libc::c_char; 128] = [0; 128];
    if Cmd_Argc() != 2 as libc::c_int {
        Com_Printf(
            b"Usage: download <filename>\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return;
    }
    Com_sprintf(
        filename.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong as libc::c_int,
        b"%s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        Cmd_Argv(1 as libc::c_int),
    );
    if !(strstr(filename.as_mut_ptr(), b"..\0" as *const u8 as *const libc::c_char))
        .is_null()
    {
        Com_Printf(
            b"Refusing to download a path with ..\n\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        );
        return;
    }
    if FS_LoadFile(filename.as_mut_ptr(), 0 as *mut *mut libc::c_void)
        != -(1 as libc::c_int)
    {
        Com_Printf(
            b"File already exists.\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return;
    }
    strcpy((cls.downloadname).as_mut_ptr(), filename.as_mut_ptr());
    Com_Printf(
        b"Downloading %s\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        (cls.downloadname).as_mut_ptr(),
    );
    COM_StripExtension(
        (cls.downloadname).as_mut_ptr(),
        (cls.downloadtempname).as_mut_ptr(),
    );
    strcat(
        (cls.downloadtempname).as_mut_ptr(),
        b".tmp\0" as *const u8 as *const libc::c_char,
    );
    MSG_WriteByte(&mut cls.netchan.message, clc_stringcmd as libc::c_int);
    MSG_WriteString(
        &mut cls.netchan.message,
        va(
            b"download %s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            (cls.downloadname).as_mut_ptr(),
        ),
    );
    cls.downloadnumber += 1;
}

#[no_mangle]
pub unsafe extern "C" fn CL_RegisterSounds() {
    let mut i: libc::c_int = 0;
    S_BeginRegistration();
    CL_RegisterTEntSounds();
    i = 1 as libc::c_int;
    while i < 256 as libc::c_int {
        if cl
            .configstrings[(32 as libc::c_int + 256 as libc::c_int + i)
            as usize][0 as libc::c_int as usize] == 0
        {
            break;
        }
        cl
            .sound_precache[i
            as usize] = S_RegisterSound(
            (cl.configstrings[(32 as libc::c_int + 256 as libc::c_int + i) as usize])
                .as_mut_ptr(),
        );
        Sys_SendKeyEvents();
        i += 1;
    }
    S_EndRegistration();
}

#[no_mangle]
pub unsafe extern "C" fn CL_ParseDownload() {
    let mut size: libc::c_int = 0;
    let mut percent: libc::c_int = 0;
    let mut name: [libc::c_char; 128] = [0; 128];
    let mut r: libc::c_int = 0;
    size = MSG_ReadShort(&mut net_message);
    percent = MSG_ReadByte(&mut net_message);
    if size == -(1 as libc::c_int) {
        Com_Printf(
            b"Server does not have this file.\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        if !(cls.download).is_null() {
            fclose(cls.download);
            cls.download = 0 as *mut FILE;
        }
        CL_RequestNextDownload();
        return;
    }
    if (cls.download).is_null() {
        CL_DownloadFileName(
            name.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong as libc::c_int,
            (cls.downloadtempname).as_mut_ptr(),
        );
        FS_CreatePath(name.as_mut_ptr());
        cls
            .download = fopen(
            name.as_mut_ptr(),
            b"wb\0" as *const u8 as *const libc::c_char,
        );
        if (cls.download).is_null() {
            net_message.readcount += size;
            Com_Printf(
                b"Failed to open %s\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                (cls.downloadtempname).as_mut_ptr(),
            );
            CL_RequestNextDownload();
            return;
        }
    }
    fwrite(
        (net_message.data).offset(net_message.readcount as isize) as *const libc::c_void,
        1 as libc::c_int as libc::c_ulong,
        size as libc::c_ulong,
        cls.download,
    );
    net_message.readcount += size;
    if percent != 100 as libc::c_int {
        cls.downloadpercent = percent;
        MSG_WriteByte(&mut cls.netchan.message, clc_stringcmd as libc::c_int);
        SZ_Print(
            &mut cls.netchan.message,
            b"nextdl\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    } else {
        let mut oldn: [libc::c_char; 128] = [0; 128];
        let mut newn: [libc::c_char; 128] = [0; 128];
        fclose(cls.download);
        CL_DownloadFileName(
            oldn.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong as libc::c_int,
            (cls.downloadtempname).as_mut_ptr(),
        );
        CL_DownloadFileName(
            newn.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong as libc::c_int,
            (cls.downloadname).as_mut_ptr(),
        );
        r = rename(oldn.as_mut_ptr(), newn.as_mut_ptr());
        if r != 0 {
            Com_Printf(
                b"failed to rename.\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
        }
        cls.download = 0 as *mut FILE;
        cls.downloadpercent = 0 as libc::c_int;
        CL_RequestNextDownload();
    };
}

#[no_mangle]
pub unsafe extern "C" fn CL_ParseServerData() {
    extern "C" {
        static mut fs_gamedirvar: *mut cvar_t;
    }
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    Com_DPrintf(
        b"Serverdata packet received.\n\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    CL_ClearState();
    cls.state = ca_connected;
    i = MSG_ReadLong(&mut net_message);
    cls.serverProtocol = i;
    if !(Com_ServerState() != 0 && 34 as libc::c_int == 34 as libc::c_int) {
        if i != 34 as libc::c_int {
            Com_Error(
                1 as libc::c_int,
                b"Server returned version %i, not %i\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                i,
                34 as libc::c_int,
            );
        }
    }
    cl.servercount = MSG_ReadLong(&mut net_message);
    cl.attractloop = MSG_ReadByte(&mut net_message) as qboolean;
    str = MSG_ReadString(&mut net_message);
    strncpy(
        (cl.gamedir).as_mut_ptr(),
        str,
        (::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
    );
    if *str as libc::c_int != 0
        && (((*fs_gamedirvar).string).is_null() || *(*fs_gamedirvar).string == 0
        || strcmp((*fs_gamedirvar).string, str) != 0)
        || *str == 0
        && (!((*fs_gamedirvar).string).is_null()
        || *(*fs_gamedirvar).string as libc::c_int != 0)
    {
        Cvar_Set(
            b"game\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            str,
        );
    }
    cl.playernum = MSG_ReadShort(&mut net_message);
    str = MSG_ReadString(&mut net_message);
    if cl.playernum == -(1 as libc::c_int) {
        SCR_PlayCinematic(str);
    } else {
        Com_Printf(
            b"\n\n\x1D\x1E\x1E\x1E\x1E\x1E\x1E\x1E\x1E\x1E\x1E\x1E\x1E\x1E\x1E\x1E\x1E\x1E\x1E\x1E\x1E\x1E\x1E\x1E\x1E\x1E\x1E\x1E\x1E\x1E\x1E\x1E\x1E\x1E\x1E\x1E\x1F\n\n\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        Com_Printf(
            b"%c%s\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            2 as libc::c_int,
            str,
        );
        cl.refresh_prepped = false_0;
    };
}

#[no_mangle]
pub unsafe extern "C" fn CL_ParseBaseline() {
    let mut es: *mut entity_state_t = 0 as *mut entity_state_t;
    let mut bits: libc::c_int = 0;
    let mut newnum: libc::c_int = 0;
    let mut nullstate: entity_state_t = entity_state_t {
        number: 0,
        origin: [0.; 3],
        angles: [0.; 3],
        old_origin: [0.; 3],
        modelindex: 0,
        modelindex2: 0,
        modelindex3: 0,
        modelindex4: 0,
        frame: 0,
        skinnum: 0,
        effects: 0,
        renderfx: 0,
        solid: 0,
        sound: 0,
        event: 0,
    };
    memset(
        &mut nullstate as *mut entity_state_t as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<entity_state_t>() as libc::c_ulong,
    );
    newnum = CL_ParseEntityBits(&mut bits as *mut libc::c_int as *mut libc::c_uint);
    es = &mut (*cl_entities.as_mut_ptr().offset(newnum as isize)).baseline;
    CL_ParseDelta(&mut nullstate, es, newnum, bits);
}

#[no_mangle]
pub unsafe extern "C" fn CL_LoadClientinfo(
    mut ci: *mut clientinfo_t,
    mut s: *mut libc::c_char,
) {
    let mut i: libc::c_int = 0;
    let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut model_name: [libc::c_char; 64] = [0; 64];
    let mut skin_name: [libc::c_char; 64] = [0; 64];
    let mut model_filename: [libc::c_char; 64] = [0; 64];
    let mut skin_filename: [libc::c_char; 64] = [0; 64];
    let mut weapon_filename: [libc::c_char; 64] = [0; 64];
    strncpy(
        ((*ci).cinfo).as_mut_ptr(),
        s,
        ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong,
    );
    (*ci)
        .cinfo[(::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        as usize] = 0 as libc::c_int as libc::c_char;
    strncpy(
        ((*ci).name).as_mut_ptr(),
        s,
        ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong,
    );
    (*ci)
        .name[(::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        as usize] = 0 as libc::c_int as libc::c_char;
    t = strstr(s, b"\\\0" as *const u8 as *const libc::c_char);
    if !t.is_null() {
        (*ci)
            .name[t.offset_from(s) as libc::c_long
            as usize] = 0 as libc::c_int as libc::c_char;
        s = t.offset(1 as libc::c_int as isize);
    }
    if (*cl_noskins).value != 0. || *s as libc::c_int == 0 as libc::c_int {
        Com_sprintf(
            model_filename.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
            b"players/male/tris.md2\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        Com_sprintf(
            weapon_filename.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
            b"players/male/weapon.md2\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        Com_sprintf(
            skin_filename.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
            b"players/male/grunt.pcx\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        Com_sprintf(
            ((*ci).iconname).as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
            b"/players/male/grunt_i.pcx\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        let ref mut fresh0 = (*ci).model;
        *fresh0 = (re.RegisterModel)
            .expect("non-null function pointer")(model_filename.as_mut_ptr());
        memset(
            ((*ci).weaponmodel).as_mut_ptr() as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<[*mut model_s; 20]>() as libc::c_ulong,
        );
        let ref mut fresh1 = (*ci).weaponmodel[0 as libc::c_int as usize];
        *fresh1 = (re.RegisterModel)
            .expect("non-null function pointer")(weapon_filename.as_mut_ptr());
        let ref mut fresh2 = (*ci).skin;
        *fresh2 = (re.RegisterSkin)
            .expect("non-null function pointer")(skin_filename.as_mut_ptr());
        let ref mut fresh3 = (*ci).icon;
        *fresh3 = (re.RegisterPic)
            .expect("non-null function pointer")(((*ci).iconname).as_mut_ptr());
    } else {
        strcpy(model_name.as_mut_ptr(), s);
        t = strstr(model_name.as_mut_ptr(), b"/\0" as *const u8 as *const libc::c_char);
        if t.is_null() {
            t = strstr(
                model_name.as_mut_ptr(),
                b"\\\0" as *const u8 as *const libc::c_char,
            );
        }
        if t.is_null() {
            t = model_name.as_mut_ptr();
        }
        *t = 0 as libc::c_int as libc::c_char;
        strcpy(
            skin_name.as_mut_ptr(),
            s
                .offset(strlen(model_name.as_mut_ptr()) as isize)
                .offset(1 as libc::c_int as isize),
        );
        Com_sprintf(
            model_filename.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
            b"players/%s/tris.md2\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            model_name.as_mut_ptr(),
        );
        let ref mut fresh4 = (*ci).model;
        *fresh4 = (re.RegisterModel)
            .expect("non-null function pointer")(model_filename.as_mut_ptr());
        if ((*ci).model).is_null() {
            strcpy(
                model_name.as_mut_ptr(),
                b"male\0" as *const u8 as *const libc::c_char,
            );
            Com_sprintf(
                model_filename.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong
                    as libc::c_int,
                b"players/male/tris.md2\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
            let ref mut fresh5 = (*ci).model;
            *fresh5 = (re.RegisterModel)
                .expect("non-null function pointer")(model_filename.as_mut_ptr());
        }
        Com_sprintf(
            skin_filename.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
            b"players/%s/%s.pcx\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            model_name.as_mut_ptr(),
            skin_name.as_mut_ptr(),
        );
        let ref mut fresh6 = (*ci).skin;
        *fresh6 = (re.RegisterSkin)
            .expect("non-null function pointer")(skin_filename.as_mut_ptr());
        if ((*ci).skin).is_null()
            && Q_stricmp(
            model_name.as_mut_ptr(),
            b"male\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) != 0
        {
            strcpy(
                model_name.as_mut_ptr(),
                b"male\0" as *const u8 as *const libc::c_char,
            );
            Com_sprintf(
                model_filename.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong
                    as libc::c_int,
                b"players/male/tris.md2\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
            let ref mut fresh7 = (*ci).model;
            *fresh7 = (re.RegisterModel)
                .expect("non-null function pointer")(model_filename.as_mut_ptr());
            Com_sprintf(
                skin_filename.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong
                    as libc::c_int,
                b"players/%s/%s.pcx\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                model_name.as_mut_ptr(),
                skin_name.as_mut_ptr(),
            );
            let ref mut fresh8 = (*ci).skin;
            *fresh8 = (re.RegisterSkin)
                .expect("non-null function pointer")(skin_filename.as_mut_ptr());
        }
        if ((*ci).skin).is_null() {
            Com_sprintf(
                skin_filename.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong
                    as libc::c_int,
                b"players/%s/grunt.pcx\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                model_name.as_mut_ptr(),
                skin_name.as_mut_ptr(),
            );
            let ref mut fresh9 = (*ci).skin;
            *fresh9 = (re.RegisterSkin)
                .expect("non-null function pointer")(skin_filename.as_mut_ptr());
        }
        i = 0 as libc::c_int;
        while i < num_cl_weaponmodels {
            Com_sprintf(
                weapon_filename.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong
                    as libc::c_int,
                b"players/%s/%s\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                model_name.as_mut_ptr(),
                (cl_weaponmodels[i as usize]).as_mut_ptr(),
            );
            let ref mut fresh10 = (*ci).weaponmodel[i as usize];
            *fresh10 = (re.RegisterModel)
                .expect("non-null function pointer")(weapon_filename.as_mut_ptr());
            if ((*ci).weaponmodel[i as usize]).is_null()
                && strcmp(
                model_name.as_mut_ptr(),
                b"cyborg\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
            {
                Com_sprintf(
                    weapon_filename.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong
                        as libc::c_int,
                    b"players/male/%s\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    (cl_weaponmodels[i as usize]).as_mut_ptr(),
                );
                let ref mut fresh11 = (*ci).weaponmodel[i as usize];
                *fresh11 = (re.RegisterModel)
                    .expect("non-null function pointer")(weapon_filename.as_mut_ptr());
            }
            if (*cl_vwep).value == 0. {
                break;
            }
            i += 1;
        }
        Com_sprintf(
            ((*ci).iconname).as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
            b"/players/%s/%s_i.pcx\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            model_name.as_mut_ptr(),
            skin_name.as_mut_ptr(),
        );
        let ref mut fresh12 = (*ci).icon;
        *fresh12 = (re.RegisterPic)
            .expect("non-null function pointer")(((*ci).iconname).as_mut_ptr());
    }
    if ((*ci).skin).is_null() || ((*ci).icon).is_null() || ((*ci).model).is_null()
        || ((*ci).weaponmodel[0 as libc::c_int as usize]).is_null()
    {
        let ref mut fresh13 = (*ci).skin;
        *fresh13 = 0 as *mut image_s;
        let ref mut fresh14 = (*ci).icon;
        *fresh14 = 0 as *mut image_s;
        let ref mut fresh15 = (*ci).model;
        *fresh15 = 0 as *mut model_s;
        let ref mut fresh16 = (*ci).weaponmodel[0 as libc::c_int as usize];
        *fresh16 = 0 as *mut model_s;
        return;
    }
}

#[no_mangle]
pub unsafe extern "C" fn CL_ParseClientinfo(mut player: libc::c_int) {
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ci: *mut clientinfo_t = 0 as *mut clientinfo_t;
    s = (cl
        .configstrings[(player
        + (32 as libc::c_int + 256 as libc::c_int + 256 as libc::c_int
        + 256 as libc::c_int + 256 as libc::c_int + 256 as libc::c_int)) as usize])
        .as_mut_ptr();
    ci = &mut *(cl.clientinfo).as_mut_ptr().offset(player as isize) as *mut clientinfo_t;
    CL_LoadClientinfo(ci, s);
}

#[no_mangle]
pub unsafe extern "C" fn CL_ParseConfigString() {
    let mut i: libc::c_int = 0;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    i = MSG_ReadShort(&mut net_message);
    if i < 0 as libc::c_int
        || i
        >= 32 as libc::c_int + 256 as libc::c_int + 256 as libc::c_int
        + 256 as libc::c_int + 256 as libc::c_int + 256 as libc::c_int
        + 256 as libc::c_int + 256 as libc::c_int * 2 as libc::c_int
    {
        Com_Error(
            1 as libc::c_int,
            b"configstring > MAX_CONFIGSTRINGS\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    s = MSG_ReadString(&mut net_message);
    strcpy((cl.configstrings[i as usize]).as_mut_ptr(), s);
    if i
        >= 32 as libc::c_int + 256 as libc::c_int + 256 as libc::c_int
        + 256 as libc::c_int
        && i
        < 32 as libc::c_int + 256 as libc::c_int + 256 as libc::c_int
        + 256 as libc::c_int + 256 as libc::c_int
    {
        CL_SetLightstyle(
            i
                - (32 as libc::c_int + 256 as libc::c_int + 256 as libc::c_int
                + 256 as libc::c_int),
        );
    } else if i == 1 as libc::c_int {
        if cl.refresh_prepped as u64 != 0 {
            CDAudio_Play(
                atoi((cl.configstrings[1 as libc::c_int as usize]).as_mut_ptr()),
                true_0,
            );
        }
    } else if i >= 32 as libc::c_int && i < 32 as libc::c_int + 256 as libc::c_int {
        if cl.refresh_prepped as u64 != 0 {
            cl
                .model_draw[(i - 32 as libc::c_int)
                as usize] = (re.RegisterModel)
                .expect(
                    "non-null function pointer",
                )((cl.configstrings[i as usize]).as_mut_ptr());
            if cl.configstrings[i as usize][0 as libc::c_int as usize] as libc::c_int
                == '*' as i32
            {
                cl
                    .model_clip[(i - 32 as libc::c_int)
                    as usize] = CM_InlineModel(
                    (cl.configstrings[i as usize]).as_mut_ptr(),
                );
            } else {
                cl.model_clip[(i - 32 as libc::c_int) as usize] = 0 as *mut cmodel_s;
            }
        }
    } else if i >= 32 as libc::c_int + 256 as libc::c_int
        && i < 32 as libc::c_int + 256 as libc::c_int + 256 as libc::c_int
    {
        if cl.refresh_prepped as u64 != 0 {
            cl
                .sound_precache[(i - (32 as libc::c_int + 256 as libc::c_int))
                as usize] = S_RegisterSound((cl.configstrings[i as usize]).as_mut_ptr());
        }
    } else if i >= 32 as libc::c_int + 256 as libc::c_int + 256 as libc::c_int
        && i
        < 32 as libc::c_int + 256 as libc::c_int + 256 as libc::c_int
        + 256 as libc::c_int
    {
        if cl.refresh_prepped as u64 != 0 {
            cl
                .image_precache[(i
                - (32 as libc::c_int + 256 as libc::c_int + 256 as libc::c_int))
                as usize] = (re.RegisterPic)
                .expect(
                    "non-null function pointer",
                )((cl.configstrings[i as usize]).as_mut_ptr());
        }
    } else if i
        >= 32 as libc::c_int + 256 as libc::c_int + 256 as libc::c_int
        + 256 as libc::c_int + 256 as libc::c_int + 256 as libc::c_int
        && i
        < 32 as libc::c_int + 256 as libc::c_int + 256 as libc::c_int
        + 256 as libc::c_int + 256 as libc::c_int + 256 as libc::c_int
        + 256 as libc::c_int
    {
        if cl.refresh_prepped as u64 != 0 {
            CL_ParseClientinfo(
                i
                    - (32 as libc::c_int + 256 as libc::c_int + 256 as libc::c_int
                    + 256 as libc::c_int + 256 as libc::c_int + 256 as libc::c_int),
            );
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn CL_ParseStartSoundPacket() {
    let mut pos_v: vec3_t = [0.; 3];
    let mut pos: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut channel: libc::c_int = 0;
    let mut ent: libc::c_int = 0;
    let mut sound_num: libc::c_int = 0;
    let mut volume: libc::c_float = 0.;
    let mut attenuation: libc::c_float = 0.;
    let mut flags: libc::c_int = 0;
    let mut ofs: libc::c_float = 0.;
    flags = MSG_ReadByte(&mut net_message);
    sound_num = MSG_ReadByte(&mut net_message);
    if flags & (1 as libc::c_int) << 0 as libc::c_int != 0 {
        volume = (MSG_ReadByte(&mut net_message) as libc::c_double / 255.0f64)
            as libc::c_float;
    } else {
        volume = 1.0f64 as libc::c_float;
    }
    if flags & (1 as libc::c_int) << 1 as libc::c_int != 0 {
        attenuation = (MSG_ReadByte(&mut net_message) as libc::c_double / 64.0f64)
            as libc::c_float;
    } else {
        attenuation = 1.0f64 as libc::c_float;
    }
    if flags & (1 as libc::c_int) << 4 as libc::c_int != 0 {
        ofs = (MSG_ReadByte(&mut net_message) as libc::c_double / 1000.0f64)
            as libc::c_float;
    } else {
        ofs = 0 as libc::c_int as libc::c_float;
    }
    if flags & (1 as libc::c_int) << 3 as libc::c_int != 0 {
        channel = MSG_ReadShort(&mut net_message);
        ent = channel >> 3 as libc::c_int;
        if ent > 1024 as libc::c_int {
            Com_Error(
                1 as libc::c_int,
                b"CL_ParseStartSoundPacket: ent = %i\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ent,
            );
        }
        channel &= 7 as libc::c_int;
    } else {
        ent = 0 as libc::c_int;
        channel = 0 as libc::c_int;
    }
    if flags & (1 as libc::c_int) << 2 as libc::c_int != 0 {
        MSG_ReadPos(&mut net_message, pos_v.as_mut_ptr());
        pos = pos_v.as_mut_ptr();
    } else {
        pos = 0 as *mut libc::c_float;
    }
    if (cl.sound_precache[sound_num as usize]).is_null() {
        return;
    }
    S_StartSound(
        pos,
        ent,
        channel,
        cl.sound_precache[sound_num as usize],
        volume,
        attenuation,
        ofs,
    );
}

#[no_mangle]
pub unsafe extern "C" fn SHOWNET(mut s: *mut libc::c_char) {
    if (*cl_shownet).value >= 2 as libc::c_int as libc::c_float {
        Com_Printf(
            b"%3i:%s\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            net_message.readcount - 1 as libc::c_int,
            s,
        );
    }
}

#[no_mangle]
pub unsafe extern "C" fn CL_ParseServerMessage() {
    let mut cmd: libc::c_int = 0;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    if (*cl_shownet).value == 1 as libc::c_int as libc::c_float {
        Com_Printf(
            b"%i \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            net_message.cursize,
        );
    } else if (*cl_shownet).value >= 2 as libc::c_int as libc::c_float {
        Com_Printf(
            b"------------------\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    loop {
        if net_message.readcount > net_message.cursize {
            Com_Error(
                1 as libc::c_int,
                b"CL_ParseServerMessage: Bad server message\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
            );
            break;
        } else {
            cmd = MSG_ReadByte(&mut net_message);
            if cmd == -(1 as libc::c_int) {
                SHOWNET(
                    b"END OF MESSAGE\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                );
                break;
            } else {
                if (*cl_shownet).value >= 2 as libc::c_int as libc::c_float {
                    if (svc_strings[cmd as usize]).is_null() {
                        Com_Printf(
                            b"%3i:BAD CMD %i\n\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char,
                            net_message.readcount - 1 as libc::c_int,
                            cmd,
                        );
                    } else {
                        SHOWNET(svc_strings[cmd as usize]);
                    }
                }
                match cmd {
                    6 => {}
                    7 => {
                        Com_Error(
                            2 as libc::c_int,
                            b"Server disconnected\n\0" as *const u8
                                as *const libc::c_char as *mut libc::c_char,
                        );
                    }
                    8 => {
                        Com_Printf(
                            b"Server disconnected, reconnecting\n\0" as *const u8
                                as *const libc::c_char as *mut libc::c_char,
                        );
                        if !(cls.download).is_null() {
                            fclose(cls.download);
                            cls.download = 0 as *mut FILE;
                        }
                        cls.state = ca_connecting;
                        cls.connect_time = -(99999 as libc::c_int) as libc::c_float;
                    }
                    10 => {
                        i = MSG_ReadByte(&mut net_message);
                        if i == 3 as libc::c_int {
                            S_StartLocalSound(
                                b"misc/talk.wav\0" as *const u8 as *const libc::c_char
                                    as *mut libc::c_char,
                            );
                            con.ormask = 128 as libc::c_int;
                        }
                        Com_Printf(
                            b"%s\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char,
                            MSG_ReadString(&mut net_message),
                        );
                        con.ormask = 0 as libc::c_int;
                    }
                    15 => {
                        SCR_CenterPrint(MSG_ReadString(&mut net_message));
                    }
                    11 => {
                        s = MSG_ReadString(&mut net_message);
                        Com_DPrintf(
                            b"stufftext: %s\n\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char,
                            s,
                        );
                        Cbuf_AddText(s);
                    }
                    12 => {
                        Cbuf_Execute();
                        CL_ParseServerData();
                    }
                    13 => {
                        CL_ParseConfigString();
                    }
                    9 => {
                        CL_ParseStartSoundPacket();
                    }
                    14 => {
                        CL_ParseBaseline();
                    }
                    3 => {
                        CL_ParseTEnt();
                    }
                    1 => {
                        CL_ParseMuzzleFlash();
                    }
                    2 => {
                        CL_ParseMuzzleFlash2();
                    }
                    16 => {
                        CL_ParseDownload();
                    }
                    20 => {
                        CL_ParseFrame();
                    }
                    5 => {
                        CL_ParseInventory();
                    }
                    4 => {
                        s = MSG_ReadString(&mut net_message);
                        strncpy(
                            (cl.layout).as_mut_ptr(),
                            s,
                            (::std::mem::size_of::<[libc::c_char; 1024]>()
                                as libc::c_ulong)
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                        );
                    }
                    17 | 18 | 19 => {
                        Com_Error(
                            1 as libc::c_int,
                            b"Out of place frame data\0" as *const u8
                                as *const libc::c_char as *mut libc::c_char,
                        );
                    }
                    _ => {
                        Com_Error(
                            1 as libc::c_int,
                            b"CL_ParseServerMessage: Illegible server message\n\0"
                                as *const u8 as *const libc::c_char as *mut libc::c_char,
                        );
                    }
                }
            }
        }
    }
    CL_AddNetgraph();
    if cls.demorecording as libc::c_uint != 0 && cls.demowaiting as u64 == 0 {
        CL_WriteDemoMessage();
    }
}
