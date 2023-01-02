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
    fn IN_Frame();
    fn M_AddToServerList(adr: netadr_t, info: *mut libc::c_char);
    fn CL_ParseServerMessage();
    fn IN_Commands();
    fn CL_SendCmd();
    fn CL_PredictMovement();
    fn VID_CheckChanges();
    fn S_Update(
        origin: *mut vec_t,
        v_forward: *mut vec_t,
        v_right: *mut vec_t,
        v_up: *mut vec_t,
    );
    fn CDAudio_Update();
    fn CL_RunDLights();
    fn CL_RunLightStyles();
    fn SCR_RunCinematic();
    fn SCR_RunConsole();
    fn Key_WriteBindings(f: *mut FILE);
    fn CDAudio_Shutdown();
    fn IN_Shutdown();
    fn VID_Shutdown();
    fn SCR_EndLoadingPlaque();
    fn Con_Init();
    fn VID_Init();
    fn V_Init();
    fn M_Init();
    fn SCR_Init();
    fn CDAudio_Init() -> libc::c_int;
    fn CL_InitInput();
    static mut cl_upspeed: *mut cvar_t;
    static mut cl_forwardspeed: *mut cvar_t;
    static mut cl_sidespeed: *mut cvar_t;
    static mut cl_yawspeed: *mut cvar_t;
    static mut cl_pitchspeed: *mut cvar_t;
    static mut cl_anglespeedkey: *mut cvar_t;
    static mut cl_run: *mut cvar_t;
    fn SCR_UpdateScreen();
    fn CL_ParseClientinfo(player: libc::c_int);
    fn S_Shutdown();
    fn S_Init();
    fn SCR_BeginLoadingPlaque();
    fn SV_Shutdown(finalmsg: *mut libc::c_char, reconnect: qboolean);
    static mut re: refexport_t;
    fn M_ForceMenuOff();
    fn SCR_StopCinematic();
    fn S_StopAllSounds();
    fn CL_ClearEffects();
    fn CL_ClearTEnts();
    fn CL_CheckOrDownloadFile(filename: *mut libc::c_char) -> qboolean;
    fn CL_RegisterSounds();
    fn CL_PrepRefresh();
    fn CL_Download_f();
    fn IN_Init();
    fn Sys_SendKeyEvents();
    fn Sys_AppActivate();
    static mut time_after_ref: libc::c_int;
    static mut time_before_ref: libc::c_int;
    static mut log_stats_file: *mut FILE;
    static mut log_stats: *mut cvar_t;
    static mut host_speeds: *mut cvar_t;
    static mut dedicated: *mut cvar_t;
    fn Com_ServerState() -> libc::c_int;
    fn Com_Quit();
    fn Com_Error(code: libc::c_int, fmt: *mut libc::c_char, _: ...);
    fn Com_DPrintf(fmt: *mut libc::c_char, _: ...);
    fn FS_CreatePath(path: *mut libc::c_char);
    fn FS_FreeFile(buffer: *mut libc::c_void);
    fn FS_LoadFile(
        path: *mut libc::c_char,
        buffer: *mut *mut libc::c_void,
    ) -> libc::c_int;
    fn FS_ExecAutoexec();
    fn FS_Gamedir() -> *mut libc::c_char;
    fn CM_LoadMap(
        name_0: *mut libc::c_char,
        clientload: qboolean,
        checksum: *mut libc::c_uint,
    ) -> *mut cmodel_t;
    fn Netchan_Process(chan: *mut netchan_t, msg_0: *mut sizebuf_t) -> qboolean;
    fn Netchan_OutOfBandPrint(
        net_socket: libc::c_int,
        adr: netadr_t,
        format: *mut libc::c_char,
        _: ...
    );
    fn Netchan_Transmit(chan: *mut netchan_t, length: libc::c_int, data: *mut byte);
    fn Netchan_Setup(
        sock: netsrc_t,
        chan: *mut netchan_t,
        adr: netadr_t,
        qport: libc::c_int,
    );
    static mut net_message_buffer: [byte; 1400];
    static mut net_message: sizebuf_t;
    static mut net_from: netadr_t;
    fn NET_StringToAdr(s: *mut libc::c_char, a: *mut netadr_t) -> qboolean;
    fn NET_AdrToString(a: netadr_t) -> *mut libc::c_char;
    fn NET_IsLocalAddress(adr: netadr_t) -> qboolean;
    fn NET_CompareAdr(a: netadr_t, b: netadr_t) -> qboolean;
    fn NET_SendPacket(
        sock: netsrc_t,
        length: libc::c_int,
        data: *mut libc::c_void,
        to: netadr_t,
    );
    fn NET_GetPacket(
        sock: netsrc_t,
        net_from_0: *mut netadr_t,
        net_message_0: *mut sizebuf_t,
    ) -> qboolean;
    fn NET_Config(multiplayer: qboolean);
    static mut userinfo_modified: qboolean;
    fn Cvar_Userinfo() -> *mut libc::c_char;
    fn Cvar_WriteVariables(path: *mut libc::c_char);
    fn Cvar_VariableString(var_name: *mut libc::c_char) -> *mut libc::c_char;
    fn Cvar_VariableValue(var_name: *mut libc::c_char) -> libc::c_float;
    fn Cvar_SetValue(var_name: *mut libc::c_char, value: libc::c_float);
    fn Cvar_Set(var_name: *mut libc::c_char, value: *mut libc::c_char) -> *mut cvar_t;
    fn Cvar_Get(
        var_name: *mut libc::c_char,
        value: *mut libc::c_char,
        flags: libc::c_int,
    ) -> *mut cvar_t;
    fn Cmd_TokenizeString(text: *mut libc::c_char, macroExpand: qboolean);
    fn Cmd_Args() -> *mut libc::c_char;
    fn Cmd_Argv(arg: libc::c_int) -> *mut libc::c_char;
    fn Cmd_Argc() -> libc::c_int;
    fn Cmd_AddCommand(cmd_name: *mut libc::c_char, function: xcommand_t);
    fn Cbuf_Execute();
    fn Cbuf_AddText(text: *mut libc::c_char);
    fn Info_Print(s: *mut libc::c_char);
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
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fwrite(
        _: *const libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn atoi(__nptr: *const libc::c_char) -> libc::c_int;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn putenv(__string: *mut libc::c_char) -> libc::c_int;
    fn Com_sprintf(
        dest: *mut libc::c_char,
        size: libc::c_int,
        fmt: *mut libc::c_char,
        _: ...
    );
    fn Q_stricmp(s1: *mut libc::c_char, s2: *mut libc::c_char) -> libc::c_int;
    fn BigShort(l: libc::c_short) -> libc::c_short;
    fn LittleLong(l: libc::c_int) -> libc::c_int;
    fn va(format: *mut libc::c_char, _: ...) -> *mut libc::c_char;
    static mut curtime: libc::c_int;
    fn Sys_Milliseconds() -> libc::c_int;
    fn Com_Printf(msg_0: *mut libc::c_char, _: ...);
    fn SZ_Init(buf: *mut sizebuf_t, data: *mut byte, length: libc::c_int);
    fn SZ_Clear(buf: *mut sizebuf_t);
    fn SZ_Print(buf: *mut sizebuf_t, data: *mut libc::c_char);
    fn MSG_WriteChar(sb: *mut sizebuf_t, c: libc::c_int);
    fn MSG_WriteByte(sb: *mut sizebuf_t, c: libc::c_int);
    fn MSG_WriteShort(sb: *mut sizebuf_t, c: libc::c_int);
    fn MSG_WriteLong(sb: *mut sizebuf_t, c: libc::c_int);
    fn MSG_WriteString(sb: *mut sizebuf_t, s: *mut libc::c_char);
    fn MSG_WriteDeltaEntity(
        from: *mut entity_state_s,
        to: *mut entity_state_s,
        msg_0: *mut sizebuf_t,
        force: qboolean,
        newentity: qboolean,
    );
    fn MSG_BeginReading(sb: *mut sizebuf_t);
    fn MSG_ReadLong(sb: *mut sizebuf_t) -> libc::c_int;
    fn MSG_ReadString(sb: *mut sizebuf_t) -> *mut libc::c_char;
    fn MSG_ReadStringLine(sb: *mut sizebuf_t) -> *mut libc::c_char;
    static mut allow_download: *mut cvar_t;
    static mut allow_download_players: *mut cvar_t;
    static mut allow_download_models: *mut cvar_t;
    static mut allow_download_sounds: *mut cvar_t;
    static mut allow_download_maps: *mut cvar_t;
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
pub struct mapsurface_s {
    pub c: csurface_t,
    pub rname: [libc::c_char; 32],
}

pub type mapsurface_t = mapsurface_s;
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

pub type xcommand_t = Option::<unsafe extern "C" fn() -> ()>;

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
pub struct netadr_t {
    pub type_0: netadrtype_t,
    pub ip: [byte; 4],
    pub ipx: [byte; 10],
    pub port: libc::c_ushort,
}

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
pub struct dmdl_t {
    pub ident: libc::c_int,
    pub version: libc::c_int,
    pub skinwidth: libc::c_int,
    pub skinheight: libc::c_int,
    pub framesize: libc::c_int,
    pub num_skins: libc::c_int,
    pub num_xyz: libc::c_int,
    pub num_st: libc::c_int,
    pub num_tris: libc::c_int,
    pub num_glcmds: libc::c_int,
    pub num_frames: libc::c_int,
    pub ofs_skins: libc::c_int,
    pub ofs_st: libc::c_int,
    pub ofs_tris: libc::c_int,
    pub ofs_frames: libc::c_int,
    pub ofs_glcmds: libc::c_int,
    pub ofs_end: libc::c_int,
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
pub struct cheatvar_t {
    pub name: *mut libc::c_char,
    pub value: *mut libc::c_char,
    pub var: *mut cvar_t,
}

#[no_mangle]
pub static mut freelook: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut adr0: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut adr1: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut adr2: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut adr3: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut adr4: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut adr5: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut adr6: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut adr7: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut adr8: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut cl_stereo_separation: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut cl_stereo: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut rcon_client_password: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut rcon_address: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut cl_noskins: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut cl_autoskins: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut cl_footsteps: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut cl_timeout: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut cl_predict: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut cl_maxfps: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut cl_gun: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut cl_add_particles: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut cl_add_lights: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut cl_add_entities: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut cl_add_blend: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut cl_shownet: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut cl_showmiss: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut cl_showclamp: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut cl_paused: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut cl_timedemo: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut lookspring: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut lookstrafe: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut sensitivity: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut m_pitch: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut m_yaw: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut m_forward: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut m_side: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut cl_lightlevel: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut info_password: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut info_spectator: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut name: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut skin: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut rate: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut fov: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut msg: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut hand: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut gender: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut gender_auto: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut cl_vwep: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut cls: client_static_t = client_static_t {
    state: ca_uninitialized,
    key_dest: key_game,
    framecount: 0,
    realtime: 0,
    frametime: 0.,
    disable_screen: 0.,
    disable_servercount: 0,
    servername: [0; 128],
    connect_time: 0.,
    quakePort: 0,
    netchan: netchan_t {
        fatal_error: false_0,
        sock: NS_CLIENT,
        dropped: 0,
        last_received: 0,
        last_sent: 0,
        remote_address: netadr_t {
            type_0: NA_LOOPBACK,
            ip: [0; 4],
            ipx: [0; 10],
            port: 0,
        },
        qport: 0,
        incoming_sequence: 0,
        incoming_acknowledged: 0,
        incoming_reliable_acknowledged: 0,
        incoming_reliable_sequence: 0,
        outgoing_sequence: 0,
        reliable_sequence: 0,
        last_reliable_sequence: 0,
        message: sizebuf_t {
            allowoverflow: false_0,
            overflowed: false_0,
            data: 0 as *const byte as *mut byte,
            maxsize: 0,
            cursize: 0,
            readcount: 0,
        },
        message_buf: [0; 1384],
        reliable_length: 0,
        reliable_buf: [0; 1384],
    },
    serverProtocol: 0,
    challenge: 0,
    download: 0 as *const FILE as *mut FILE,
    downloadtempname: [0; 128],
    downloadname: [0; 128],
    downloadnumber: 0,
    downloadtype: dl_none,
    downloadpercent: 0,
    demorecording: false_0,
    demowaiting: false_0,
    demofile: 0 as *const FILE as *mut FILE,
};
#[no_mangle]
pub static mut cl: client_state_t = client_state_t {
    timeoutcount: 0,
    timedemo_frames: 0,
    timedemo_start: 0,
    refresh_prepped: false_0,
    sound_prepped: false_0,
    force_refdef: false_0,
    parse_entities: 0,
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
    cmds: [usercmd_t {
        msec: 0,
        buttons: 0,
        angles: [0; 3],
        forwardmove: 0,
        sidemove: 0,
        upmove: 0,
        impulse: 0,
        lightlevel: 0,
    }; 64],
    cmd_time: [0; 64],
    predicted_origins: [[0; 3]; 64],
    predicted_step: 0.,
    predicted_step_time: 0,
    predicted_origin: [0.; 3],
    predicted_angles: [0.; 3],
    prediction_error: [0.; 3],
    frame: frame_t {
        valid: false_0,
        serverframe: 0,
        servertime: 0,
        deltaframe: 0,
        areabits: [0; 32],
        playerstate: player_state_t {
            pmove: pmove_state_t {
                pm_type: PM_NORMAL,
                origin: [0; 3],
                velocity: [0; 3],
                pm_flags: 0,
                pm_time: 0,
                gravity: 0,
                delta_angles: [0; 3],
            },
            viewangles: [0.; 3],
            viewoffset: [0.; 3],
            kick_angles: [0.; 3],
            gunangles: [0.; 3],
            gunoffset: [0.; 3],
            gunindex: 0,
            gunframe: 0,
            blend: [0.; 4],
            fov: 0.,
            rdflags: 0,
            stats: [0; 32],
        },
        num_entities: 0,
        parse_entities: 0,
    },
    surpressCount: 0,
    frames: [frame_t {
        valid: false_0,
        serverframe: 0,
        servertime: 0,
        deltaframe: 0,
        areabits: [0; 32],
        playerstate: player_state_t {
            pmove: pmove_state_t {
                pm_type: PM_NORMAL,
                origin: [0; 3],
                velocity: [0; 3],
                pm_flags: 0,
                pm_time: 0,
                gravity: 0,
                delta_angles: [0; 3],
            },
            viewangles: [0.; 3],
            viewoffset: [0.; 3],
            kick_angles: [0.; 3],
            gunangles: [0.; 3],
            gunoffset: [0.; 3],
            gunindex: 0,
            gunframe: 0,
            blend: [0.; 4],
            fov: 0.,
            rdflags: 0,
            stats: [0; 32],
        },
        num_entities: 0,
        parse_entities: 0,
    }; 16],
    viewangles: [0.; 3],
    time: 0,
    lerpfrac: 0.,
    refdef: refdef_t {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
        fov_x: 0.,
        fov_y: 0.,
        vieworg: [0.; 3],
        viewangles: [0.; 3],
        blend: [0.; 4],
        time: 0.,
        rdflags: 0,
        areabits: 0 as *const byte as *mut byte,
        lightstyles: 0 as *const lightstyle_t as *mut lightstyle_t,
        num_entities: 0,
        entities: 0 as *const entity_t as *mut entity_t,
        num_dlights: 0,
        dlights: 0 as *const dlight_t as *mut dlight_t,
        num_particles: 0,
        particles: 0 as *const particle_t as *mut particle_t,
    },
    v_forward: [0.; 3],
    v_right: [0.; 3],
    v_up: [0.; 3],
    layout: [0; 1024],
    inventory: [0; 256],
    cinematic_file: 0 as *const FILE as *mut FILE,
    cinematictime: 0,
    cinematicframe: 0,
    cinematicpalette: [0; 768],
    cinematicpalette_active: false_0,
    attractloop: false_0,
    servercount: 0,
    gamedir: [0; 64],
    playernum: 0,
    configstrings: [[0; 64]; 2080],
    model_draw: [0 as *const model_s as *mut model_s; 256],
    model_clip: [0 as *const cmodel_s as *mut cmodel_s; 256],
    sound_precache: [0 as *const sfx_s as *mut sfx_s; 256],
    image_precache: [0 as *const image_s as *mut image_s; 256],
    clientinfo: [clientinfo_t {
        name: [0; 64],
        cinfo: [0; 64],
        skin: 0 as *const image_s as *mut image_s,
        icon: 0 as *const image_s as *mut image_s,
        iconname: [0; 64],
        model: 0 as *const model_s as *mut model_s,
        weaponmodel: [0 as *const model_s as *mut model_s; 20],
    }; 256],
    baseclientinfo: clientinfo_t {
        name: [0; 64],
        cinfo: [0; 64],
        skin: 0 as *const image_s as *mut image_s,
        icon: 0 as *const image_s as *mut image_s,
        iconname: [0; 64],
        model: 0 as *const model_s as *mut model_s,
        weaponmodel: [0 as *const model_s as *mut model_s; 20],
    },
};
#[no_mangle]
pub static mut cl_entities: [centity_t; 1024] = [centity_t {
    baseline: entity_state_t {
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
    },
    current: entity_state_t {
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
    },
    prev: entity_state_t {
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
    },
    serverframe: 0,
    trailcount: 0,
    lerp_origin: [0.; 3],
    fly_stoptime: 0,
}; 1024];
#[no_mangle]
pub static mut cl_parse_entities: [entity_state_t; 1024] = [entity_state_t {
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
}; 1024];

#[no_mangle]
pub unsafe extern "C" fn CL_WriteDemoMessage() {
    let mut len: libc::c_int = 0;
    let mut swlen: libc::c_int = 0;
    len = net_message.cursize - 8 as libc::c_int;
    swlen = LittleLong(len);
    fwrite(
        &mut swlen as *mut libc::c_int as *const libc::c_void,
        4 as libc::c_int as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        cls.demofile,
    );
    fwrite(
        (net_message.data).offset(8 as libc::c_int as isize) as *const libc::c_void,
        len as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        cls.demofile,
    );
}

#[no_mangle]
pub unsafe extern "C" fn CL_Stop_f() {
    let mut len: libc::c_int = 0;
    if cls.demorecording as u64 == 0 {
        Com_Printf(
            b"Not recording a demo.\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return;
    }
    len = -(1 as libc::c_int);
    fwrite(
        &mut len as *mut libc::c_int as *const libc::c_void,
        4 as libc::c_int as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        cls.demofile,
    );
    fclose(cls.demofile);
    cls.demofile = 0 as *mut FILE;
    cls.demorecording = false_0;
    Com_Printf(
        b"Stopped demo.\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
}

#[no_mangle]
pub unsafe extern "C" fn CL_Record_f() {
    let mut name_0: [libc::c_char; 128] = [0; 128];
    let mut buf_data: [libc::c_char; 1400] = [0; 1400];
    let mut buf: sizebuf_t = sizebuf_t {
        allowoverflow: false_0,
        overflowed: false_0,
        data: 0 as *const byte as *mut byte,
        maxsize: 0,
        cursize: 0,
        readcount: 0,
    };
    let mut i: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut ent: *mut entity_state_t = 0 as *mut entity_state_t;
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
    if Cmd_Argc() != 2 as libc::c_int {
        Com_Printf(
            b"record <demoname>\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return;
    }
    if cls.demorecording as u64 != 0 {
        Com_Printf(
            b"Already recording.\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return;
    }
    if cls.state as libc::c_uint != ca_active as libc::c_int as libc::c_uint {
        Com_Printf(
            b"You must be in a level to record.\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return;
    }
    Com_sprintf(
        name_0.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong as libc::c_int,
        b"%s/demos/%s.dm2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        FS_Gamedir(),
        Cmd_Argv(1 as libc::c_int),
    );
    Com_Printf(
        b"recording to %s.\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        name_0.as_mut_ptr(),
    );
    FS_CreatePath(name_0.as_mut_ptr());
    cls
        .demofile = fopen(
        name_0.as_mut_ptr(),
        b"wb\0" as *const u8 as *const libc::c_char,
    );
    if (cls.demofile).is_null() {
        Com_Printf(
            b"ERROR: couldn't open.\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return;
    }
    cls.demorecording = true_0;
    cls.demowaiting = true_0;
    SZ_Init(
        &mut buf,
        buf_data.as_mut_ptr() as *mut byte,
        ::std::mem::size_of::<[libc::c_char; 1400]>() as libc::c_ulong as libc::c_int,
    );
    MSG_WriteByte(&mut buf, svc_serverdata as libc::c_int);
    MSG_WriteLong(&mut buf, 34 as libc::c_int);
    MSG_WriteLong(&mut buf, 0x10000 as libc::c_int + cl.servercount);
    MSG_WriteByte(&mut buf, 1 as libc::c_int);
    MSG_WriteString(&mut buf, (cl.gamedir).as_mut_ptr());
    MSG_WriteShort(&mut buf, cl.playernum);
    MSG_WriteString(
        &mut buf,
        (cl.configstrings[0 as libc::c_int as usize]).as_mut_ptr(),
    );
    i = 0 as libc::c_int;
    while i
        < 32 as libc::c_int + 256 as libc::c_int + 256 as libc::c_int
        + 256 as libc::c_int + 256 as libc::c_int + 256 as libc::c_int
        + 256 as libc::c_int + 256 as libc::c_int * 2 as libc::c_int
    {
        if cl.configstrings[i as usize][0 as libc::c_int as usize] != 0 {
            if (buf.cursize as libc::c_ulong)
                .wrapping_add(strlen((cl.configstrings[i as usize]).as_mut_ptr()))
                .wrapping_add(32 as libc::c_int as libc::c_ulong)
                > buf.maxsize as libc::c_ulong
            {
                len = LittleLong(buf.cursize);
                fwrite(
                    &mut len as *mut libc::c_int as *const libc::c_void,
                    4 as libc::c_int as libc::c_ulong,
                    1 as libc::c_int as libc::c_ulong,
                    cls.demofile,
                );
                fwrite(
                    buf.data as *const libc::c_void,
                    buf.cursize as libc::c_ulong,
                    1 as libc::c_int as libc::c_ulong,
                    cls.demofile,
                );
                buf.cursize = 0 as libc::c_int;
            }
            MSG_WriteByte(&mut buf, svc_configstring as libc::c_int);
            MSG_WriteShort(&mut buf, i);
            MSG_WriteString(&mut buf, (cl.configstrings[i as usize]).as_mut_ptr());
        }
        i += 1;
    }
    memset(
        &mut nullstate as *mut entity_state_t as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<entity_state_t>() as libc::c_ulong,
    );
    i = 0 as libc::c_int;
    while i < 1024 as libc::c_int {
        ent = &mut (*cl_entities.as_mut_ptr().offset(i as isize)).baseline;
        if !((*ent).modelindex == 0) {
            if buf.cursize + 64 as libc::c_int > buf.maxsize {
                len = LittleLong(buf.cursize);
                fwrite(
                    &mut len as *mut libc::c_int as *const libc::c_void,
                    4 as libc::c_int as libc::c_ulong,
                    1 as libc::c_int as libc::c_ulong,
                    cls.demofile,
                );
                fwrite(
                    buf.data as *const libc::c_void,
                    buf.cursize as libc::c_ulong,
                    1 as libc::c_int as libc::c_ulong,
                    cls.demofile,
                );
                buf.cursize = 0 as libc::c_int;
            }
            MSG_WriteByte(&mut buf, svc_spawnbaseline as libc::c_int);
            MSG_WriteDeltaEntity(
                &mut nullstate,
                &mut (*cl_entities.as_mut_ptr().offset(i as isize)).baseline,
                &mut buf,
                true_0,
                true_0,
            );
        }
        i += 1;
    }
    MSG_WriteByte(&mut buf, svc_stufftext as libc::c_int);
    MSG_WriteString(
        &mut buf,
        b"precache\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    len = LittleLong(buf.cursize);
    fwrite(
        &mut len as *mut libc::c_int as *const libc::c_void,
        4 as libc::c_int as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        cls.demofile,
    );
    fwrite(
        buf.data as *const libc::c_void,
        buf.cursize as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        cls.demofile,
    );
}

#[no_mangle]
pub unsafe extern "C" fn Cmd_ForwardToServer() {
    let mut cmd: *mut libc::c_char = 0 as *mut libc::c_char;
    cmd = Cmd_Argv(0 as libc::c_int);
    if cls.state as libc::c_uint <= ca_connected as libc::c_int as libc::c_uint
        || *cmd as libc::c_int == '-' as i32 || *cmd as libc::c_int == '+' as i32
    {
        Com_Printf(
            b"Unknown command \"%s\"\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            cmd,
        );
        return;
    }
    MSG_WriteByte(&mut cls.netchan.message, clc_stringcmd as libc::c_int);
    SZ_Print(&mut cls.netchan.message, cmd);
    if Cmd_Argc() > 1 as libc::c_int {
        SZ_Print(
            &mut cls.netchan.message,
            b" \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        SZ_Print(&mut cls.netchan.message, Cmd_Args());
    }
}

#[no_mangle]
pub unsafe extern "C" fn CL_Setenv_f() {
    let mut argc: libc::c_int = Cmd_Argc();
    if argc > 2 as libc::c_int {
        let mut buffer: [libc::c_char; 1000] = [0; 1000];
        let mut i: libc::c_int = 0;
        strcpy(buffer.as_mut_ptr(), Cmd_Argv(1 as libc::c_int));
        strcat(buffer.as_mut_ptr(), b"=\0" as *const u8 as *const libc::c_char);
        i = 2 as libc::c_int;
        while i < argc {
            strcat(buffer.as_mut_ptr(), Cmd_Argv(i));
            strcat(buffer.as_mut_ptr(), b" \0" as *const u8 as *const libc::c_char);
            i += 1;
        }
        putenv(buffer.as_mut_ptr());
    } else if argc == 2 as libc::c_int {
        let mut env: *mut libc::c_char = getenv(Cmd_Argv(1 as libc::c_int));
        if !env.is_null() {
            Com_Printf(
                b"%s=%s\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                Cmd_Argv(1 as libc::c_int),
                env,
            );
        } else {
            Com_Printf(
                b"%s undefined\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                Cmd_Argv(1 as libc::c_int),
                env,
            );
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn CL_ForwardToServer_f() {
    if cls.state as libc::c_uint != ca_connected as libc::c_int as libc::c_uint
        && cls.state as libc::c_uint != ca_active as libc::c_int as libc::c_uint
    {
        Com_Printf(
            b"Can't \"%s\", not connected\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            Cmd_Argv(0 as libc::c_int),
        );
        return;
    }
    if Cmd_Argc() > 1 as libc::c_int {
        MSG_WriteByte(&mut cls.netchan.message, clc_stringcmd as libc::c_int);
        SZ_Print(&mut cls.netchan.message, Cmd_Args());
    }
}

#[no_mangle]
pub unsafe extern "C" fn CL_Pause_f() {
    if Cvar_VariableValue(
        b"maxclients\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) > 1 as libc::c_int as libc::c_float || Com_ServerState() == 0
    {
        Cvar_SetValue(
            b"paused\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            0 as libc::c_int as libc::c_float,
        );
        return;
    }
    Cvar_SetValue(
        b"paused\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ((*cl_paused).value == 0.) as libc::c_int as libc::c_float,
    );
}

#[no_mangle]
pub unsafe extern "C" fn CL_Quit_f() {
    CL_Disconnect();
    Com_Quit();
}

#[no_mangle]
pub unsafe extern "C" fn CL_Drop() {
    if cls.state as libc::c_uint == ca_uninitialized as libc::c_int as libc::c_uint {
        return;
    }
    if cls.state as libc::c_uint == ca_disconnected as libc::c_int as libc::c_uint {
        return;
    }
    CL_Disconnect();
    if cls.disable_servercount != -(1 as libc::c_int) {
        SCR_EndLoadingPlaque();
    }
}

#[no_mangle]
pub unsafe extern "C" fn CL_SendConnectPacket() {
    let mut adr: netadr_t = netadr_t {
        type_0: NA_LOOPBACK,
        ip: [0; 4],
        ipx: [0; 10],
        port: 0,
    };
    let mut port: libc::c_int = 0;
    if NET_StringToAdr((cls.servername).as_mut_ptr(), &mut adr) as u64 == 0 {
        Com_Printf(
            b"Bad server address\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        cls.connect_time = 0 as libc::c_int as libc::c_float;
        return;
    }
    if adr.port as libc::c_int == 0 as libc::c_int {
        adr.port = BigShort(27910 as libc::c_int as libc::c_short) as libc::c_ushort;
    }
    port = Cvar_VariableValue(
        b"qport\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as libc::c_int;
    userinfo_modified = false_0;
    Netchan_OutOfBandPrint(
        NS_CLIENT as libc::c_int,
        adr,
        b"connect %i %i %i \"%s\"\n\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        34 as libc::c_int,
        port,
        cls.challenge,
        Cvar_Userinfo(),
    );
}

#[no_mangle]
pub unsafe extern "C" fn CL_CheckForResend() {
    let mut adr: netadr_t = netadr_t {
        type_0: NA_LOOPBACK,
        ip: [0; 4],
        ipx: [0; 10],
        port: 0,
    };
    if cls.state as libc::c_uint == ca_disconnected as libc::c_int as libc::c_uint
        && Com_ServerState() != 0
    {
        cls.state = ca_connecting;
        strncpy(
            (cls.servername).as_mut_ptr(),
            b"localhost\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        );
        CL_SendConnectPacket();
        return;
    }
    if cls.state as libc::c_uint != ca_connecting as libc::c_int as libc::c_uint {
        return;
    }
    if cls.realtime as libc::c_float - cls.connect_time
        < 3000 as libc::c_int as libc::c_float
    {
        return;
    }
    if NET_StringToAdr((cls.servername).as_mut_ptr(), &mut adr) as u64 == 0 {
        Com_Printf(
            b"Bad server address\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        cls.state = ca_disconnected;
        return;
    }
    if adr.port as libc::c_int == 0 as libc::c_int {
        adr.port = BigShort(27910 as libc::c_int as libc::c_short) as libc::c_ushort;
    }
    cls.connect_time = cls.realtime as libc::c_float;
    Com_Printf(
        b"Connecting to %s...\n\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        (cls.servername).as_mut_ptr(),
    );
    Netchan_OutOfBandPrint(
        NS_CLIENT as libc::c_int,
        adr,
        b"getchallenge\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
}

#[no_mangle]
pub unsafe extern "C" fn CL_Connect_f() {
    let mut server: *mut libc::c_char = 0 as *mut libc::c_char;
    if Cmd_Argc() != 2 as libc::c_int {
        Com_Printf(
            b"usage: connect <server>\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return;
    }
    if Com_ServerState() != 0 {
        SV_Shutdown(
            va(
                b"Server quit\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                msg,
            ),
            false_0,
        );
    } else {
        CL_Disconnect();
    }
    server = Cmd_Argv(1 as libc::c_int);
    NET_Config(true_0);
    CL_Disconnect();
    cls.state = ca_connecting;
    strncpy(
        (cls.servername).as_mut_ptr(),
        server,
        (::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
    );
    cls.connect_time = -(99999 as libc::c_int) as libc::c_float;
}

#[no_mangle]
pub unsafe extern "C" fn CL_Rcon_f() {
    let mut message: [libc::c_char; 1024] = [0; 1024];
    let mut i: libc::c_int = 0;
    let mut to: netadr_t = netadr_t {
        type_0: NA_LOOPBACK,
        ip: [0; 4],
        ipx: [0; 10],
        port: 0,
    };
    if ((*rcon_client_password).string).is_null() {
        Com_Printf(
            b"You must set 'rcon_password' before\nissuing an rcon command.\n\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        return;
    }
    message[0 as libc::c_int as usize] = 255 as libc::c_int as libc::c_char;
    message[1 as libc::c_int as usize] = 255 as libc::c_int as libc::c_char;
    message[2 as libc::c_int as usize] = 255 as libc::c_int as libc::c_char;
    message[3 as libc::c_int as usize] = 255 as libc::c_int as libc::c_char;
    message[4 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    NET_Config(true_0);
    strcat(message.as_mut_ptr(), b"rcon \0" as *const u8 as *const libc::c_char);
    strcat(message.as_mut_ptr(), (*rcon_client_password).string);
    strcat(message.as_mut_ptr(), b" \0" as *const u8 as *const libc::c_char);
    i = 1 as libc::c_int;
    while i < Cmd_Argc() {
        strcat(message.as_mut_ptr(), Cmd_Argv(i));
        strcat(message.as_mut_ptr(), b" \0" as *const u8 as *const libc::c_char);
        i += 1;
    }
    if cls.state as libc::c_uint >= ca_connected as libc::c_int as libc::c_uint {
        to = cls.netchan.remote_address;
    } else {
        if strlen((*rcon_address).string) == 0 {
            Com_Printf(
                b"You must either be connected,\nor set the 'rcon_address' cvar\nto issue rcon commands\n\0"
                    as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            return;
        }
        NET_StringToAdr((*rcon_address).string, &mut to);
        if to.port as libc::c_int == 0 as libc::c_int {
            to.port = BigShort(27910 as libc::c_int as libc::c_short) as libc::c_ushort;
        }
    }
    NET_SendPacket(
        NS_CLIENT,
        (strlen(message.as_mut_ptr())).wrapping_add(1 as libc::c_int as libc::c_ulong)
            as libc::c_int,
        message.as_mut_ptr() as *mut libc::c_void,
        to,
    );
}

#[no_mangle]
pub unsafe extern "C" fn CL_ClearState() {
    S_StopAllSounds();
    CL_ClearEffects();
    CL_ClearTEnts();
    memset(
        &mut cl as *mut client_state_t as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<client_state_t>() as libc::c_ulong,
    );
    memset(
        &mut cl_entities as *mut [centity_t; 1024] as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<[centity_t; 1024]>() as libc::c_ulong,
    );
    SZ_Clear(&mut cls.netchan.message);
}

#[no_mangle]
pub unsafe extern "C" fn CL_Disconnect() {
    let mut final_0: [byte; 32] = [0; 32];
    if cls.state as libc::c_uint == ca_disconnected as libc::c_int as libc::c_uint {
        return;
    }
    if !cl_timedemo.is_null() && (*cl_timedemo).value != 0. {
        let mut time: libc::c_int = 0;
        time = Sys_Milliseconds() - cl.timedemo_start;
        if time > 0 as libc::c_int {
            Com_Printf(
                b"%i frames, %3.1f seconds: %3.1f fps\n\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                cl.timedemo_frames,
                time as libc::c_double / 1000.0f64,
                cl.timedemo_frames as libc::c_double * 1000.0f64 / time as libc::c_double,
            );
        }
    }
    cl.refdef.blend[2 as libc::c_int as usize] = 0 as libc::c_int as libc::c_float;
    cl
        .refdef
        .blend[1 as libc::c_int as usize] = cl.refdef.blend[2 as libc::c_int as usize];
    cl
        .refdef
        .blend[0 as libc::c_int as usize] = cl.refdef.blend[1 as libc::c_int as usize];
    (re.CinematicSetPalette)
        .expect("non-null function pointer")(0 as *const libc::c_uchar);
    M_ForceMenuOff();
    cls.connect_time = 0 as libc::c_int as libc::c_float;
    SCR_StopCinematic();
    if cls.demorecording as u64 != 0 {
        CL_Stop_f();
    }
    final_0[0 as libc::c_int as usize] = clc_stringcmd as libc::c_int as byte;
    strcpy(
        (final_0.as_mut_ptr() as *mut libc::c_char).offset(1 as libc::c_int as isize),
        b"disconnect\0" as *const u8 as *const libc::c_char,
    );
    Netchan_Transmit(
        &mut cls.netchan,
        strlen(final_0.as_mut_ptr() as *const libc::c_char) as libc::c_int,
        final_0.as_mut_ptr(),
    );
    Netchan_Transmit(
        &mut cls.netchan,
        strlen(final_0.as_mut_ptr() as *const libc::c_char) as libc::c_int,
        final_0.as_mut_ptr(),
    );
    Netchan_Transmit(
        &mut cls.netchan,
        strlen(final_0.as_mut_ptr() as *const libc::c_char) as libc::c_int,
        final_0.as_mut_ptr(),
    );
    CL_ClearState();
    if !(cls.download).is_null() {
        fclose(cls.download);
        cls.download = 0 as *mut FILE;
    }
    cls.state = ca_disconnected;
}

#[no_mangle]
pub unsafe extern "C" fn CL_Disconnect_f() {
    Com_Error(
        1 as libc::c_int,
        b"Disconnected from server\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
}

#[no_mangle]
pub unsafe extern "C" fn CL_Packet_f() {
    let mut send: [libc::c_char; 2048] = [0; 2048];
    let mut i: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut in_0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut out: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut adr: netadr_t = netadr_t {
        type_0: NA_LOOPBACK,
        ip: [0; 4],
        ipx: [0; 10],
        port: 0,
    };
    if Cmd_Argc() != 3 as libc::c_int {
        Com_Printf(
            b"packet <destination> <contents>\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return;
    }
    NET_Config(true_0);
    if NET_StringToAdr(Cmd_Argv(1 as libc::c_int), &mut adr) as u64 == 0 {
        Com_Printf(
            b"Bad address\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        return;
    }
    if adr.port == 0 {
        adr.port = BigShort(27910 as libc::c_int as libc::c_short) as libc::c_ushort;
    }
    in_0 = Cmd_Argv(2 as libc::c_int);
    out = send.as_mut_ptr().offset(4 as libc::c_int as isize);
    send[3 as libc::c_int as usize] = 0xff as libc::c_int as libc::c_char;
    send[2 as libc::c_int as usize] = send[3 as libc::c_int as usize];
    send[1 as libc::c_int as usize] = send[2 as libc::c_int as usize];
    send[0 as libc::c_int as usize] = send[1 as libc::c_int as usize];
    l = strlen(in_0) as libc::c_int;
    i = 0 as libc::c_int;
    while i < l {
        if *in_0.offset(i as isize) as libc::c_int == '\\' as i32
            && *in_0.offset((i + 1 as libc::c_int) as isize) as libc::c_int == 'n' as i32
        {
            let fresh0 = out;
            out = out.offset(1);
            *fresh0 = '\n' as i32 as libc::c_char;
            i += 1;
        } else {
            let fresh1 = out;
            out = out.offset(1);
            *fresh1 = *in_0.offset(i as isize);
        }
        i += 1;
    }
    *out = 0 as libc::c_int as libc::c_char;
    NET_SendPacket(
        NS_CLIENT,
        out.offset_from(send.as_mut_ptr()) as libc::c_long as libc::c_int,
        send.as_mut_ptr() as *mut libc::c_void,
        adr,
    );
}

#[no_mangle]
pub unsafe extern "C" fn CL_Changing_f() {
    if !(cls.download).is_null() {
        return;
    }
    SCR_BeginLoadingPlaque();
    cls.state = ca_connected;
    Com_Printf(
        b"\nChanging map...\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
}

#[no_mangle]
pub unsafe extern "C" fn CL_Reconnect_f() {
    if !(cls.download).is_null() {
        return;
    }
    S_StopAllSounds();
    if cls.state as libc::c_uint == ca_connected as libc::c_int as libc::c_uint {
        Com_Printf(
            b"reconnecting...\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        cls.state = ca_connected;
        MSG_WriteChar(&mut cls.netchan.message, clc_stringcmd as libc::c_int);
        MSG_WriteString(
            &mut cls.netchan.message,
            b"new\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        return;
    }
    if *(cls.servername).as_mut_ptr() != 0 {
        if cls.state as libc::c_uint >= ca_connected as libc::c_int as libc::c_uint {
            CL_Disconnect();
            cls.connect_time = (cls.realtime - 1500 as libc::c_int) as libc::c_float;
        } else {
            cls.connect_time = -(99999 as libc::c_int) as libc::c_float;
        }
        cls.state = ca_connecting;
        Com_Printf(
            b"reconnecting...\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
}

#[no_mangle]
pub unsafe extern "C" fn CL_ParseStatusMessage() {
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    s = MSG_ReadString(&mut net_message);
    Com_Printf(b"%s\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char, s);
    M_AddToServerList(net_from, s);
}

#[no_mangle]
pub unsafe extern "C" fn CL_PingServers_f() {
    let mut i: libc::c_int = 0;
    let mut adr: netadr_t = netadr_t {
        type_0: NA_LOOPBACK,
        ip: [0; 4],
        ipx: [0; 10],
        port: 0,
    };
    let mut name_0: [libc::c_char; 32] = [0; 32];
    let mut adrstring: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut noudp: *mut cvar_t = 0 as *mut cvar_t;
    let mut noipx: *mut cvar_t = 0 as *mut cvar_t;
    NET_Config(true_0);
    Com_Printf(
        b"pinging broadcast...\n\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    noudp = Cvar_Get(
        b"noudp\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        8 as libc::c_int,
    );
    if (*noudp).value == 0. {
        adr.type_0 = NA_BROADCAST;
        adr.port = BigShort(27910 as libc::c_int as libc::c_short) as libc::c_ushort;
        Netchan_OutOfBandPrint(
            NS_CLIENT as libc::c_int,
            adr,
            va(
                b"info %i\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                34 as libc::c_int,
            ),
        );
    }
    noipx = Cvar_Get(
        b"noipx\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        8 as libc::c_int,
    );
    if (*noipx).value == 0. {
        adr.type_0 = NA_BROADCAST_IPX;
        adr.port = BigShort(27910 as libc::c_int as libc::c_short) as libc::c_ushort;
        Netchan_OutOfBandPrint(
            NS_CLIENT as libc::c_int,
            adr,
            va(
                b"info %i\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                34 as libc::c_int,
            ),
        );
    }
    i = 0 as libc::c_int;
    while i < 16 as libc::c_int {
        Com_sprintf(
            name_0.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong as libc::c_int,
            b"adr%i\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            i,
        );
        adrstring = Cvar_VariableString(name_0.as_mut_ptr());
        if !(adrstring.is_null() || *adrstring.offset(0 as libc::c_int as isize) == 0) {
            Com_Printf(
                b"pinging %s...\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                adrstring,
            );
            if NET_StringToAdr(adrstring, &mut adr) as u64 == 0 {
                Com_Printf(
                    b"Bad address: %s\n\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    adrstring,
                );
            } else {
                if adr.port == 0 {
                    adr
                        .port = BigShort(27910 as libc::c_int as libc::c_short)
                        as libc::c_ushort;
                }
                Netchan_OutOfBandPrint(
                    NS_CLIENT as libc::c_int,
                    adr,
                    va(
                        b"info %i\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                        34 as libc::c_int,
                    ),
                );
            }
        }
        i += 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn CL_Skins_f() {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 256 as libc::c_int {
        if !(cl
            .configstrings[(32 as libc::c_int + 256 as libc::c_int + 256 as libc::c_int
            + 256 as libc::c_int + 256 as libc::c_int + 256 as libc::c_int + i)
            as usize][0 as libc::c_int as usize] == 0)
        {
            Com_Printf(
                b"client %i: %s\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                i,
                (cl
                    .configstrings[(32 as libc::c_int + 256 as libc::c_int
                    + 256 as libc::c_int + 256 as libc::c_int + 256 as libc::c_int
                    + 256 as libc::c_int + i) as usize])
                    .as_mut_ptr(),
            );
            SCR_UpdateScreen();
            Sys_SendKeyEvents();
            CL_ParseClientinfo(i);
        }
        i += 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn CL_ConnectionlessPacket() {
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut c: *mut libc::c_char = 0 as *mut libc::c_char;
    MSG_BeginReading(&mut net_message);
    MSG_ReadLong(&mut net_message);
    s = MSG_ReadStringLine(&mut net_message);
    Cmd_TokenizeString(s, false_0);
    c = Cmd_Argv(0 as libc::c_int);
    Com_Printf(
        b"%s: %s\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        NET_AdrToString(net_from),
        c,
    );
    if strcmp(c, b"client_connect\0" as *const u8 as *const libc::c_char) == 0 {
        if cls.state as libc::c_uint == ca_connected as libc::c_int as libc::c_uint {
            Com_Printf(
                b"Dup connect received.  Ignored.\n\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
            );
            return;
        }
        Netchan_Setup(NS_CLIENT, &mut cls.netchan, net_from, cls.quakePort);
        MSG_WriteChar(&mut cls.netchan.message, clc_stringcmd as libc::c_int);
        MSG_WriteString(
            &mut cls.netchan.message,
            b"new\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        cls.state = ca_connected;
        return;
    }
    if strcmp(c, b"info\0" as *const u8 as *const libc::c_char) == 0 {
        CL_ParseStatusMessage();
        return;
    }
    if strcmp(c, b"cmd\0" as *const u8 as *const libc::c_char) == 0 {
        if NET_IsLocalAddress(net_from) as u64 == 0 {
            Com_Printf(
                b"Command packet from remote host.  Ignored.\n\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
            );
            return;
        }
        Sys_AppActivate();
        s = MSG_ReadString(&mut net_message);
        Cbuf_AddText(s);
        Cbuf_AddText(b"\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
        return;
    }
    if strcmp(c, b"print\0" as *const u8 as *const libc::c_char) == 0 {
        s = MSG_ReadString(&mut net_message);
        Com_Printf(b"%s\0" as *const u8 as *const libc::c_char as *mut libc::c_char, s);
        return;
    }
    if strcmp(c, b"ping\0" as *const u8 as *const libc::c_char) == 0 {
        Netchan_OutOfBandPrint(
            NS_CLIENT as libc::c_int,
            net_from,
            b"ack\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        return;
    }
    if strcmp(c, b"challenge\0" as *const u8 as *const libc::c_char) == 0 {
        cls.challenge = atoi(Cmd_Argv(1 as libc::c_int));
        CL_SendConnectPacket();
        return;
    }
    if strcmp(c, b"echo\0" as *const u8 as *const libc::c_char) == 0 {
        Netchan_OutOfBandPrint(
            NS_CLIENT as libc::c_int,
            net_from,
            b"%s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            Cmd_Argv(1 as libc::c_int),
        );
        return;
    }
    Com_Printf(
        b"Unknown command.\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
}

#[no_mangle]
pub unsafe extern "C" fn CL_DumpPackets() {
    while NET_GetPacket(NS_CLIENT, &mut net_from, &mut net_message) as u64 != 0 {
        Com_Printf(
            b"dumnping a packet\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
}

#[no_mangle]
pub unsafe extern "C" fn CL_ReadPackets() {
    while NET_GetPacket(NS_CLIENT, &mut net_from, &mut net_message) as u64 != 0 {
        if *(net_message.data as *mut libc::c_int) == -(1 as libc::c_int) {
            CL_ConnectionlessPacket();
        } else {
            if cls.state as libc::c_uint
                == ca_disconnected as libc::c_int as libc::c_uint
                || cls.state as libc::c_uint
                == ca_connecting as libc::c_int as libc::c_uint
            {
                continue;
            }
            if net_message.cursize < 8 as libc::c_int {
                Com_Printf(
                    b"%s: Runt packet\n\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    NET_AdrToString(net_from),
                );
            } else if NET_CompareAdr(net_from, cls.netchan.remote_address) as u64 == 0 {
                Com_DPrintf(
                    b"%s:sequenced packet without connection\n\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    NET_AdrToString(net_from),
                );
            } else {
                if Netchan_Process(&mut cls.netchan, &mut net_message) as u64 == 0 {
                    continue;
                }
                CL_ParseServerMessage();
            }
        }
    }
    if cls.state as libc::c_uint >= ca_connected as libc::c_int as libc::c_uint
        && (cls.realtime - cls.netchan.last_received) as libc::c_float
        > (*cl_timeout).value * 1000 as libc::c_int as libc::c_float
    {
        cl.timeoutcount += 1;
        if cl.timeoutcount > 5 as libc::c_int {
            Com_Printf(
                b"\nServer connection timed out.\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
            CL_Disconnect();
            return;
        }
    } else {
        cl.timeoutcount = 0 as libc::c_int;
    };
}

#[no_mangle]
pub unsafe extern "C" fn CL_FixUpGender() {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut sk: [libc::c_char; 80] = [0; 80];
    if (*gender_auto).value != 0. {
        if (*gender).modified as u64 != 0 {
            (*gender).modified = false_0;
            return;
        }
        strncpy(
            sk.as_mut_ptr(),
            (*skin).string,
            (::std::mem::size_of::<[libc::c_char; 80]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        );
        p = strchr(sk.as_mut_ptr(), '/' as i32);
        if !p.is_null() {
            *p = 0 as libc::c_int as libc::c_char;
        }
        if Q_stricmp(
            sk.as_mut_ptr(),
            b"male\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) == 0 as libc::c_int
            || Q_stricmp(
            sk.as_mut_ptr(),
            b"cyborg\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) == 0 as libc::c_int
        {
            Cvar_Set(
                b"gender\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                b"male\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
        } else if Q_stricmp(
            sk.as_mut_ptr(),
            b"female\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) == 0 as libc::c_int
            || Q_stricmp(
            sk.as_mut_ptr(),
            b"crackhor\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) == 0 as libc::c_int
        {
            Cvar_Set(
                b"gender\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                b"female\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
        } else {
            Cvar_Set(
                b"gender\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                b"none\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
        }
        (*gender).modified = false_0;
    }
}

#[no_mangle]
pub unsafe extern "C" fn CL_Userinfo_f() {
    Com_Printf(
        b"User info settings:\n\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    Info_Print(Cvar_Userinfo());
}

#[no_mangle]
pub unsafe extern "C" fn CL_Snd_Restart_f() {
    S_Shutdown();
    S_Init();
    CL_RegisterSounds();
}

#[no_mangle]
pub static mut precache_check: libc::c_int = 0;
#[no_mangle]
pub static mut precache_spawncount: libc::c_int = 0;
#[no_mangle]
pub static mut precache_tex: libc::c_int = 0;
#[no_mangle]
pub static mut precache_model_skin: libc::c_int = 0;
#[no_mangle]
pub static mut precache_model: *mut byte = 0 as *const byte as *mut byte;
static mut env_suf: [*const libc::c_char; 6] = [
    b"rt\0" as *const u8 as *const libc::c_char,
    b"bk\0" as *const u8 as *const libc::c_char,
    b"lf\0" as *const u8 as *const libc::c_char,
    b"ft\0" as *const u8 as *const libc::c_char,
    b"up\0" as *const u8 as *const libc::c_char,
    b"dn\0" as *const u8 as *const libc::c_char,
];

#[no_mangle]
pub unsafe extern "C" fn CL_RequestNextDownload() {
    let mut map_checksum: libc::c_uint = 0;
    let mut fn_0: [libc::c_char; 128] = [0; 128];
    let mut pheader: *mut dmdl_t = 0 as *mut dmdl_t;
    if cls.state as libc::c_uint != ca_connected as libc::c_int as libc::c_uint {
        return;
    }
    if (*allow_download).value == 0.
        && precache_check
        < 32 as libc::c_int + 256 as libc::c_int + 256 as libc::c_int
        + 256 as libc::c_int + 256 as libc::c_int + 256 as libc::c_int
        + 256 as libc::c_int * 5 as libc::c_int
    {
        precache_check = 32 as libc::c_int + 256 as libc::c_int + 256 as libc::c_int
            + 256 as libc::c_int + 256 as libc::c_int + 256 as libc::c_int
            + 256 as libc::c_int * 5 as libc::c_int;
    }
    if precache_check == 32 as libc::c_int {
        precache_check = 32 as libc::c_int + 2 as libc::c_int;
        if (*allow_download_maps).value != 0. {
            if CL_CheckOrDownloadFile(
                (cl.configstrings[(32 as libc::c_int + 1 as libc::c_int) as usize])
                    .as_mut_ptr(),
            ) as u64 == 0
            {
                return;
            }
        }
    }
    if precache_check >= 32 as libc::c_int
        && precache_check < 32 as libc::c_int + 256 as libc::c_int
    {
        if (*allow_download_models).value != 0. {
            while precache_check < 32 as libc::c_int + 256 as libc::c_int
                && cl.configstrings[precache_check as usize][0 as libc::c_int as usize]
                as libc::c_int != 0
            {
                if cl.configstrings[precache_check as usize][0 as libc::c_int as usize]
                    as libc::c_int == '*' as i32
                    || cl
                    .configstrings[precache_check
                    as usize][0 as libc::c_int as usize] as libc::c_int == '#' as i32
                {
                    precache_check += 1;
                } else {
                    if precache_model_skin == 0 as libc::c_int {
                        if CL_CheckOrDownloadFile(
                            (cl.configstrings[precache_check as usize]).as_mut_ptr(),
                        ) as u64 == 0
                        {
                            precache_model_skin = 1 as libc::c_int;
                            return;
                        }
                        precache_model_skin = 1 as libc::c_int;
                    }
                    if precache_model.is_null() {
                        FS_LoadFile(
                            (cl.configstrings[precache_check as usize]).as_mut_ptr(),
                            &mut precache_model as *mut *mut byte
                                as *mut *mut libc::c_void,
                        );
                        if precache_model.is_null() {
                            precache_model_skin = 0 as libc::c_int;
                            precache_check += 1;
                            continue;
                        } else if LittleLong(
                            *(precache_model as *mut libc::c_uint) as libc::c_int,
                        )
                            != (('2' as i32) << 24 as libc::c_int)
                            + (('P' as i32) << 16 as libc::c_int)
                            + (('D' as i32) << 8 as libc::c_int) + 'I' as i32
                        {
                            FS_FreeFile(precache_model as *mut libc::c_void);
                            precache_model = 0 as *mut byte;
                            precache_model_skin = 0 as libc::c_int;
                            precache_check += 1;
                            continue;
                        } else {
                            pheader = precache_model as *mut dmdl_t;
                            if LittleLong((*pheader).version) != 8 as libc::c_int {
                                precache_check += 1;
                                precache_model_skin = 0 as libc::c_int;
                                continue;
                            }
                        }
                    }
                    pheader = precache_model as *mut dmdl_t;
                    while (precache_model_skin - 1 as libc::c_int)
                        < LittleLong((*pheader).num_skins)
                    {
                        if CL_CheckOrDownloadFile(
                            (precache_model as *mut libc::c_char)
                                .offset(LittleLong((*pheader).ofs_skins) as isize)
                                .offset(
                                    ((precache_model_skin - 1 as libc::c_int)
                                        * 64 as libc::c_int) as isize,
                                ),
                        ) as u64 == 0
                        {
                            precache_model_skin += 1;
                            return;
                        }
                        precache_model_skin += 1;
                    }
                    if !precache_model.is_null() {
                        FS_FreeFile(precache_model as *mut libc::c_void);
                        precache_model = 0 as *mut byte;
                    }
                    precache_model_skin = 0 as libc::c_int;
                    precache_check += 1;
                }
            }
        }
        precache_check = 32 as libc::c_int + 256 as libc::c_int;
    }
    if precache_check >= 32 as libc::c_int + 256 as libc::c_int
        && precache_check < 32 as libc::c_int + 256 as libc::c_int + 256 as libc::c_int
    {
        if (*allow_download_sounds).value != 0. {
            if precache_check == 32 as libc::c_int + 256 as libc::c_int {
                precache_check += 1;
            }
            while precache_check
                < 32 as libc::c_int + 256 as libc::c_int + 256 as libc::c_int
                && cl.configstrings[precache_check as usize][0 as libc::c_int as usize]
                as libc::c_int != 0
            {
                if cl.configstrings[precache_check as usize][0 as libc::c_int as usize]
                    as libc::c_int == '*' as i32
                {
                    precache_check += 1;
                } else {
                    let fresh2 = precache_check;
                    precache_check = precache_check + 1;
                    Com_sprintf(
                        fn_0.as_mut_ptr(),
                        ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong
                            as libc::c_int,
                        b"sound/%s\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                        (cl.configstrings[fresh2 as usize]).as_mut_ptr(),
                    );
                    if CL_CheckOrDownloadFile(fn_0.as_mut_ptr()) as u64 == 0 {
                        return;
                    }
                }
            }
        }
        precache_check = 32 as libc::c_int + 256 as libc::c_int + 256 as libc::c_int;
    }
    if precache_check >= 32 as libc::c_int + 256 as libc::c_int + 256 as libc::c_int
        && precache_check
        < 32 as libc::c_int + 256 as libc::c_int + 256 as libc::c_int
        + 256 as libc::c_int
    {
        if precache_check == 32 as libc::c_int + 256 as libc::c_int + 256 as libc::c_int
        {
            precache_check += 1;
        }
        while precache_check
            < 32 as libc::c_int + 256 as libc::c_int + 256 as libc::c_int
            + 256 as libc::c_int
            && cl.configstrings[precache_check as usize][0 as libc::c_int as usize]
            as libc::c_int != 0
        {
            let fresh3 = precache_check;
            precache_check = precache_check + 1;
            Com_sprintf(
                fn_0.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong
                    as libc::c_int,
                b"pics/%s.pcx\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                (cl.configstrings[fresh3 as usize]).as_mut_ptr(),
            );
            if CL_CheckOrDownloadFile(fn_0.as_mut_ptr()) as u64 == 0 {
                return;
            }
        }
        precache_check = 32 as libc::c_int + 256 as libc::c_int + 256 as libc::c_int
            + 256 as libc::c_int + 256 as libc::c_int + 256 as libc::c_int;
    }
    if precache_check
        >= 32 as libc::c_int + 256 as libc::c_int + 256 as libc::c_int
        + 256 as libc::c_int + 256 as libc::c_int + 256 as libc::c_int
        && precache_check
        < 32 as libc::c_int + 256 as libc::c_int + 256 as libc::c_int
        + 256 as libc::c_int + 256 as libc::c_int + 256 as libc::c_int
        + 256 as libc::c_int * 5 as libc::c_int
    {
        if (*allow_download_players).value != 0. {
            while precache_check
                < 32 as libc::c_int + 256 as libc::c_int + 256 as libc::c_int
                + 256 as libc::c_int + 256 as libc::c_int + 256 as libc::c_int
                + 256 as libc::c_int * 5 as libc::c_int
            {
                let mut i: libc::c_int = 0;
                let mut n: libc::c_int = 0;
                let mut model: [libc::c_char; 64] = [0; 64];
                let mut skin_0: [libc::c_char; 64] = [0; 64];
                let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
                i = (precache_check
                    - (32 as libc::c_int + 256 as libc::c_int + 256 as libc::c_int
                    + 256 as libc::c_int + 256 as libc::c_int + 256 as libc::c_int))
                    / 5 as libc::c_int;
                n = (precache_check
                    - (32 as libc::c_int + 256 as libc::c_int + 256 as libc::c_int
                    + 256 as libc::c_int + 256 as libc::c_int + 256 as libc::c_int))
                    % 5 as libc::c_int;
                if cl
                    .configstrings[(32 as libc::c_int + 256 as libc::c_int
                    + 256 as libc::c_int + 256 as libc::c_int + 256 as libc::c_int
                    + 256 as libc::c_int + i) as usize][0 as libc::c_int as usize] == 0
                {
                    precache_check = 32 as libc::c_int + 256 as libc::c_int
                        + 256 as libc::c_int + 256 as libc::c_int + 256 as libc::c_int
                        + 256 as libc::c_int + (i + 1 as libc::c_int) * 5 as libc::c_int;
                } else {
                    p = strchr(
                        (cl
                            .configstrings[(32 as libc::c_int + 256 as libc::c_int
                            + 256 as libc::c_int + 256 as libc::c_int
                            + 256 as libc::c_int + 256 as libc::c_int + i) as usize])
                            .as_mut_ptr(),
                        '\\' as i32,
                    );
                    if !p.is_null() {
                        p = p.offset(1);
                    } else {
                        p = (cl
                            .configstrings[(32 as libc::c_int + 256 as libc::c_int
                            + 256 as libc::c_int + 256 as libc::c_int
                            + 256 as libc::c_int + 256 as libc::c_int + i) as usize])
                            .as_mut_ptr();
                    }
                    strcpy(model.as_mut_ptr(), p);
                    p = strchr(model.as_mut_ptr(), '/' as i32);
                    if p.is_null() {
                        p = strchr(model.as_mut_ptr(), '\\' as i32);
                    }
                    if !p.is_null() {
                        let fresh4 = p;
                        p = p.offset(1);
                        *fresh4 = 0 as libc::c_int as libc::c_char;
                        strcpy(skin_0.as_mut_ptr(), p);
                    } else {
                        *skin_0.as_mut_ptr() = 0 as libc::c_int as libc::c_char;
                    }
                    let mut current_block_114: u64;
                    match n {
                        0 => {
                            Com_sprintf(
                                fn_0.as_mut_ptr(),
                                ::std::mem::size_of::<[libc::c_char; 128]>()
                                    as libc::c_ulong as libc::c_int,
                                b"players/%s/tris.md2\0" as *const u8 as *const libc::c_char
                                    as *mut libc::c_char,
                                model.as_mut_ptr(),
                            );
                            if CL_CheckOrDownloadFile(fn_0.as_mut_ptr()) as u64 == 0 {
                                precache_check = 32 as libc::c_int + 256 as libc::c_int
                                    + 256 as libc::c_int + 256 as libc::c_int
                                    + 256 as libc::c_int + 256 as libc::c_int
                                    + i * 5 as libc::c_int + 1 as libc::c_int;
                                return;
                            }
                            n += 1;
                            current_block_114 = 17722635354708076050;
                        }
                        1 => {
                            current_block_114 = 17722635354708076050;
                        }
                        2 => {
                            current_block_114 = 13186411831301246546;
                        }
                        3 => {
                            current_block_114 = 5338232980553584439;
                        }
                        4 => {
                            current_block_114 = 16850026184477240658;
                        }
                        _ => {
                            current_block_114 = 11162283542402356847;
                        }
                    }
                    match current_block_114 {
                        17722635354708076050 => {
                            Com_sprintf(
                                fn_0.as_mut_ptr(),
                                ::std::mem::size_of::<[libc::c_char; 128]>()
                                    as libc::c_ulong as libc::c_int,
                                b"players/%s/weapon.md2\0" as *const u8
                                    as *const libc::c_char as *mut libc::c_char,
                                model.as_mut_ptr(),
                            );
                            if CL_CheckOrDownloadFile(fn_0.as_mut_ptr()) as u64 == 0 {
                                precache_check = 32 as libc::c_int + 256 as libc::c_int
                                    + 256 as libc::c_int + 256 as libc::c_int
                                    + 256 as libc::c_int + 256 as libc::c_int
                                    + i * 5 as libc::c_int + 2 as libc::c_int;
                                return;
                            }
                            n += 1;
                            current_block_114 = 13186411831301246546;
                        }
                        _ => {}
                    }
                    match current_block_114 {
                        13186411831301246546 => {
                            Com_sprintf(
                                fn_0.as_mut_ptr(),
                                ::std::mem::size_of::<[libc::c_char; 128]>()
                                    as libc::c_ulong as libc::c_int,
                                b"players/%s/weapon.pcx\0" as *const u8
                                    as *const libc::c_char as *mut libc::c_char,
                                model.as_mut_ptr(),
                            );
                            if CL_CheckOrDownloadFile(fn_0.as_mut_ptr()) as u64 == 0 {
                                precache_check = 32 as libc::c_int + 256 as libc::c_int
                                    + 256 as libc::c_int + 256 as libc::c_int
                                    + 256 as libc::c_int + 256 as libc::c_int
                                    + i * 5 as libc::c_int + 3 as libc::c_int;
                                return;
                            }
                            n += 1;
                            current_block_114 = 5338232980553584439;
                        }
                        _ => {}
                    }
                    match current_block_114 {
                        5338232980553584439 => {
                            Com_sprintf(
                                fn_0.as_mut_ptr(),
                                ::std::mem::size_of::<[libc::c_char; 128]>()
                                    as libc::c_ulong as libc::c_int,
                                b"players/%s/%s.pcx\0" as *const u8 as *const libc::c_char
                                    as *mut libc::c_char,
                                model.as_mut_ptr(),
                                skin_0.as_mut_ptr(),
                            );
                            if CL_CheckOrDownloadFile(fn_0.as_mut_ptr()) as u64 == 0 {
                                precache_check = 32 as libc::c_int + 256 as libc::c_int
                                    + 256 as libc::c_int + 256 as libc::c_int
                                    + 256 as libc::c_int + 256 as libc::c_int
                                    + i * 5 as libc::c_int + 4 as libc::c_int;
                                return;
                            }
                            n += 1;
                            current_block_114 = 16850026184477240658;
                        }
                        _ => {}
                    }
                    match current_block_114 {
                        16850026184477240658 => {
                            Com_sprintf(
                                fn_0.as_mut_ptr(),
                                ::std::mem::size_of::<[libc::c_char; 128]>()
                                    as libc::c_ulong as libc::c_int,
                                b"players/%s/%s_i.pcx\0" as *const u8 as *const libc::c_char
                                    as *mut libc::c_char,
                                model.as_mut_ptr(),
                                skin_0.as_mut_ptr(),
                            );
                            if CL_CheckOrDownloadFile(fn_0.as_mut_ptr()) as u64 == 0 {
                                precache_check = 32 as libc::c_int + 256 as libc::c_int
                                    + 256 as libc::c_int + 256 as libc::c_int
                                    + 256 as libc::c_int + 256 as libc::c_int
                                    + i * 5 as libc::c_int + 5 as libc::c_int;
                                return;
                            }
                            precache_check = 32 as libc::c_int + 256 as libc::c_int
                                + 256 as libc::c_int + 256 as libc::c_int
                                + 256 as libc::c_int + 256 as libc::c_int
                                + (i + 1 as libc::c_int) * 5 as libc::c_int;
                        }
                        _ => {}
                    }
                }
            }
        }
        precache_check = 32 as libc::c_int + 256 as libc::c_int + 256 as libc::c_int
            + 256 as libc::c_int + 256 as libc::c_int + 256 as libc::c_int
            + 256 as libc::c_int * 5 as libc::c_int;
    }
    if precache_check
        == 32 as libc::c_int + 256 as libc::c_int + 256 as libc::c_int
        + 256 as libc::c_int + 256 as libc::c_int + 256 as libc::c_int
        + 256 as libc::c_int * 5 as libc::c_int
    {
        precache_check = 32 as libc::c_int + 256 as libc::c_int + 256 as libc::c_int
            + 256 as libc::c_int + 256 as libc::c_int + 256 as libc::c_int
            + 256 as libc::c_int * 5 as libc::c_int + 1 as libc::c_int;
        CM_LoadMap(
            (cl.configstrings[(32 as libc::c_int + 1 as libc::c_int) as usize])
                .as_mut_ptr(),
            true_0,
            &mut map_checksum,
        );
        if map_checksum
            != atoi((cl.configstrings[31 as libc::c_int as usize]).as_mut_ptr())
            as libc::c_uint
        {
            Com_Error(
                1 as libc::c_int,
                b"Local map version differs from server: %i != '%s'\n\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                map_checksum,
                (cl.configstrings[31 as libc::c_int as usize]).as_mut_ptr(),
            );
            return;
        }
    }
    if precache_check
        > 32 as libc::c_int + 256 as libc::c_int + 256 as libc::c_int
        + 256 as libc::c_int + 256 as libc::c_int + 256 as libc::c_int
        + 256 as libc::c_int * 5 as libc::c_int
        && precache_check
        < 32 as libc::c_int + 256 as libc::c_int + 256 as libc::c_int
        + 256 as libc::c_int + 256 as libc::c_int + 256 as libc::c_int
        + 256 as libc::c_int * 5 as libc::c_int + 13 as libc::c_int
    {
        if (*allow_download).value != 0. && (*allow_download_maps).value != 0. {
            while precache_check
                < 32 as libc::c_int + 256 as libc::c_int + 256 as libc::c_int
                + 256 as libc::c_int + 256 as libc::c_int + 256 as libc::c_int
                + 256 as libc::c_int * 5 as libc::c_int + 13 as libc::c_int
            {
                let fresh5 = precache_check;
                precache_check = precache_check + 1;
                let mut n_0: libc::c_int = fresh5
                    - (32 as libc::c_int + 256 as libc::c_int + 256 as libc::c_int
                    + 256 as libc::c_int + 256 as libc::c_int + 256 as libc::c_int
                    + 256 as libc::c_int * 5 as libc::c_int) - 1 as libc::c_int;
                if n_0 & 1 as libc::c_int != 0 {
                    Com_sprintf(
                        fn_0.as_mut_ptr(),
                        ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong
                            as libc::c_int,
                        b"env/%s%s.pcx\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                        (cl.configstrings[2 as libc::c_int as usize]).as_mut_ptr(),
                        env_suf[(n_0 / 2 as libc::c_int) as usize],
                    );
                } else {
                    Com_sprintf(
                        fn_0.as_mut_ptr(),
                        ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong
                            as libc::c_int,
                        b"env/%s%s.tga\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                        (cl.configstrings[2 as libc::c_int as usize]).as_mut_ptr(),
                        env_suf[(n_0 / 2 as libc::c_int) as usize],
                    );
                }
                if CL_CheckOrDownloadFile(fn_0.as_mut_ptr()) as u64 == 0 {
                    return;
                }
            }
        }
        precache_check = 32 as libc::c_int + 256 as libc::c_int + 256 as libc::c_int
            + 256 as libc::c_int + 256 as libc::c_int + 256 as libc::c_int
            + 256 as libc::c_int * 5 as libc::c_int + 13 as libc::c_int;
    }
    if precache_check
        == 32 as libc::c_int + 256 as libc::c_int + 256 as libc::c_int
        + 256 as libc::c_int + 256 as libc::c_int + 256 as libc::c_int
        + 256 as libc::c_int * 5 as libc::c_int + 13 as libc::c_int
    {
        precache_check = 32 as libc::c_int + 256 as libc::c_int + 256 as libc::c_int
            + 256 as libc::c_int + 256 as libc::c_int + 256 as libc::c_int
            + 256 as libc::c_int * 5 as libc::c_int + 13 as libc::c_int
            + 1 as libc::c_int;
        precache_tex = 0 as libc::c_int;
    }
    if precache_check
        == 32 as libc::c_int + 256 as libc::c_int + 256 as libc::c_int
        + 256 as libc::c_int + 256 as libc::c_int + 256 as libc::c_int
        + 256 as libc::c_int * 5 as libc::c_int + 13 as libc::c_int
        + 1 as libc::c_int
    {
        extern "C" {
            static mut numtexinfo: libc::c_int;
        }
        extern "C" {
            static mut map_surfaces: [mapsurface_t; 0];
        }
        if (*allow_download).value != 0. && (*allow_download_maps).value != 0. {
            while precache_tex < numtexinfo {
                let mut fn_1: [libc::c_char; 128] = [0; 128];
                let fresh6 = precache_tex;
                precache_tex = precache_tex + 1;
                sprintf(
                    fn_1.as_mut_ptr(),
                    b"textures/%s.wal\0" as *const u8 as *const libc::c_char,
                    ((*map_surfaces.as_mut_ptr().offset(fresh6 as isize)).rname)
                        .as_mut_ptr(),
                );
                if CL_CheckOrDownloadFile(fn_1.as_mut_ptr()) as u64 == 0 {
                    return;
                }
            }
        }
        precache_check = 32 as libc::c_int + 256 as libc::c_int + 256 as libc::c_int
            + 256 as libc::c_int + 256 as libc::c_int + 256 as libc::c_int
            + 256 as libc::c_int * 5 as libc::c_int + 13 as libc::c_int
            + 999 as libc::c_int;
    }
    CL_RegisterSounds();
    CL_PrepRefresh();
    MSG_WriteByte(&mut cls.netchan.message, clc_stringcmd as libc::c_int);
    MSG_WriteString(
        &mut cls.netchan.message,
        va(
            b"begin %i\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            precache_spawncount,
        ),
    );
}

#[no_mangle]
pub unsafe extern "C" fn CL_Precache_f() {
    if Cmd_Argc() < 2 as libc::c_int {
        let mut map_checksum: libc::c_uint = 0;
        CM_LoadMap(
            (cl.configstrings[(32 as libc::c_int + 1 as libc::c_int) as usize])
                .as_mut_ptr(),
            true_0,
            &mut map_checksum,
        );
        CL_RegisterSounds();
        CL_PrepRefresh();
        return;
    }
    precache_check = 32 as libc::c_int;
    precache_spawncount = atoi(Cmd_Argv(1 as libc::c_int));
    precache_model = 0 as *mut byte;
    precache_model_skin = 0 as libc::c_int;
    CL_RequestNextDownload();
}

#[no_mangle]
pub unsafe extern "C" fn CL_InitLocal() {
    cls.state = ca_disconnected;
    cls.realtime = Sys_Milliseconds();
    CL_InitInput();
    adr0 = Cvar_Get(
        b"adr0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        1 as libc::c_int,
    );
    adr1 = Cvar_Get(
        b"adr1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        1 as libc::c_int,
    );
    adr2 = Cvar_Get(
        b"adr2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        1 as libc::c_int,
    );
    adr3 = Cvar_Get(
        b"adr3\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        1 as libc::c_int,
    );
    adr4 = Cvar_Get(
        b"adr4\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        1 as libc::c_int,
    );
    adr5 = Cvar_Get(
        b"adr5\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        1 as libc::c_int,
    );
    adr6 = Cvar_Get(
        b"adr6\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        1 as libc::c_int,
    );
    adr7 = Cvar_Get(
        b"adr7\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        1 as libc::c_int,
    );
    adr8 = Cvar_Get(
        b"adr8\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        1 as libc::c_int,
    );
    cl_stereo_separation = Cvar_Get(
        b"cl_stereo_separation\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        b"0.4\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        1 as libc::c_int,
    );
    cl_stereo = Cvar_Get(
        b"cl_stereo\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    cl_add_blend = Cvar_Get(
        b"cl_blend\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    cl_add_lights = Cvar_Get(
        b"cl_lights\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    cl_add_particles = Cvar_Get(
        b"cl_particles\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    cl_add_entities = Cvar_Get(
        b"cl_entities\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    cl_gun = Cvar_Get(
        b"cl_gun\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    cl_footsteps = Cvar_Get(
        b"cl_footsteps\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    cl_noskins = Cvar_Get(
        b"cl_noskins\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    cl_autoskins = Cvar_Get(
        b"cl_autoskins\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    cl_predict = Cvar_Get(
        b"cl_predict\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    cl_maxfps = Cvar_Get(
        b"cl_maxfps\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"90\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    cl_upspeed = Cvar_Get(
        b"cl_upspeed\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"200\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    cl_forwardspeed = Cvar_Get(
        b"cl_forwardspeed\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"200\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    cl_sidespeed = Cvar_Get(
        b"cl_sidespeed\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"200\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    cl_yawspeed = Cvar_Get(
        b"cl_yawspeed\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"140\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    cl_pitchspeed = Cvar_Get(
        b"cl_pitchspeed\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"150\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    cl_anglespeedkey = Cvar_Get(
        b"cl_anglespeedkey\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"1.5\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    cl_run = Cvar_Get(
        b"cl_run\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        1 as libc::c_int,
    );
    freelook = Cvar_Get(
        b"freelook\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        1 as libc::c_int,
    );
    lookspring = Cvar_Get(
        b"lookspring\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        1 as libc::c_int,
    );
    lookstrafe = Cvar_Get(
        b"lookstrafe\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        1 as libc::c_int,
    );
    sensitivity = Cvar_Get(
        b"sensitivity\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"3\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        1 as libc::c_int,
    );
    m_pitch = Cvar_Get(
        b"m_pitch\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"0.022\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        1 as libc::c_int,
    );
    m_yaw = Cvar_Get(
        b"m_yaw\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"0.022\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    m_forward = Cvar_Get(
        b"m_forward\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    m_side = Cvar_Get(
        b"m_side\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    cl_shownet = Cvar_Get(
        b"cl_shownet\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    cl_showmiss = Cvar_Get(
        b"cl_showmiss\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    cl_showclamp = Cvar_Get(
        b"showclamp\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    cl_timeout = Cvar_Get(
        b"cl_timeout\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"120\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    cl_paused = Cvar_Get(
        b"paused\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    cl_timedemo = Cvar_Get(
        b"timedemo\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    rcon_client_password = Cvar_Get(
        b"rcon_password\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    rcon_address = Cvar_Get(
        b"rcon_address\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    cl_lightlevel = Cvar_Get(
        b"r_lightlevel\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    info_password = Cvar_Get(
        b"password\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        2 as libc::c_int,
    );
    info_spectator = Cvar_Get(
        b"spectator\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        2 as libc::c_int,
    );
    name = Cvar_Get(
        b"name\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"unnamed\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        2 as libc::c_int | 1 as libc::c_int,
    );
    skin = Cvar_Get(
        b"skin\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"male/grunt\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        2 as libc::c_int | 1 as libc::c_int,
    );
    rate = Cvar_Get(
        b"rate\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"25000\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        2 as libc::c_int | 1 as libc::c_int,
    );
    msg = Cvar_Get(
        b"msg\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        2 as libc::c_int | 1 as libc::c_int,
    );
    hand = Cvar_Get(
        b"hand\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        2 as libc::c_int | 1 as libc::c_int,
    );
    fov = Cvar_Get(
        b"fov\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"90\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        2 as libc::c_int | 1 as libc::c_int,
    );
    gender = Cvar_Get(
        b"gender\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"male\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        2 as libc::c_int | 1 as libc::c_int,
    );
    gender_auto = Cvar_Get(
        b"gender_auto\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        1 as libc::c_int,
    );
    (*gender).modified = false_0;
    cl_vwep = Cvar_Get(
        b"cl_vwep\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        1 as libc::c_int,
    );
    Cmd_AddCommand(
        b"cmd\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        Some(CL_ForwardToServer_f as unsafe extern "C" fn() -> ()),
    );
    Cmd_AddCommand(
        b"pause\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        Some(CL_Pause_f as unsafe extern "C" fn() -> ()),
    );
    Cmd_AddCommand(
        b"pingservers\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        Some(CL_PingServers_f as unsafe extern "C" fn() -> ()),
    );
    Cmd_AddCommand(
        b"skins\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        Some(CL_Skins_f as unsafe extern "C" fn() -> ()),
    );
    Cmd_AddCommand(
        b"userinfo\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        Some(CL_Userinfo_f as unsafe extern "C" fn() -> ()),
    );
    Cmd_AddCommand(
        b"snd_restart\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        Some(CL_Snd_Restart_f as unsafe extern "C" fn() -> ()),
    );
    Cmd_AddCommand(
        b"changing\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        Some(CL_Changing_f as unsafe extern "C" fn() -> ()),
    );
    Cmd_AddCommand(
        b"disconnect\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        Some(CL_Disconnect_f as unsafe extern "C" fn() -> ()),
    );
    Cmd_AddCommand(
        b"record\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        Some(CL_Record_f as unsafe extern "C" fn() -> ()),
    );
    Cmd_AddCommand(
        b"stop\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        Some(CL_Stop_f as unsafe extern "C" fn() -> ()),
    );
    Cmd_AddCommand(
        b"quit\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        Some(CL_Quit_f as unsafe extern "C" fn() -> ()),
    );
    Cmd_AddCommand(
        b"connect\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        Some(CL_Connect_f as unsafe extern "C" fn() -> ()),
    );
    Cmd_AddCommand(
        b"reconnect\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        Some(CL_Reconnect_f as unsafe extern "C" fn() -> ()),
    );
    Cmd_AddCommand(
        b"rcon\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        Some(CL_Rcon_f as unsafe extern "C" fn() -> ()),
    );
    Cmd_AddCommand(
        b"setenv\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        Some(CL_Setenv_f as unsafe extern "C" fn() -> ()),
    );
    Cmd_AddCommand(
        b"precache\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        Some(CL_Precache_f as unsafe extern "C" fn() -> ()),
    );
    Cmd_AddCommand(
        b"download\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        Some(CL_Download_f as unsafe extern "C" fn() -> ()),
    );
    Cmd_AddCommand(
        b"wave\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        None,
    );
    Cmd_AddCommand(
        b"inven\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        None,
    );
    Cmd_AddCommand(
        b"kill\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        None,
    );
    Cmd_AddCommand(
        b"use\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        None,
    );
    Cmd_AddCommand(
        b"drop\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        None,
    );
    Cmd_AddCommand(
        b"say\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        None,
    );
    Cmd_AddCommand(
        b"say_team\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        None,
    );
    Cmd_AddCommand(
        b"info\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        None,
    );
    Cmd_AddCommand(
        b"prog\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        None,
    );
    Cmd_AddCommand(
        b"give\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        None,
    );
    Cmd_AddCommand(
        b"god\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        None,
    );
    Cmd_AddCommand(
        b"notarget\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        None,
    );
    Cmd_AddCommand(
        b"noclip\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        None,
    );
    Cmd_AddCommand(
        b"invuse\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        None,
    );
    Cmd_AddCommand(
        b"invprev\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        None,
    );
    Cmd_AddCommand(
        b"invnext\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        None,
    );
    Cmd_AddCommand(
        b"invdrop\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        None,
    );
    Cmd_AddCommand(
        b"weapnext\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        None,
    );
    Cmd_AddCommand(
        b"weapprev\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        None,
    );
}

#[no_mangle]
pub unsafe extern "C" fn CL_WriteConfiguration() {
    let mut f: *mut FILE = 0 as *mut FILE;
    let mut path: [libc::c_char; 64] = [0; 64];
    if cls.state as libc::c_uint == ca_uninitialized as libc::c_int as libc::c_uint {
        return;
    }
    Com_sprintf(
        path.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
        b"%s/config.cfg\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        FS_Gamedir(),
    );
    f = fopen(path.as_mut_ptr(), b"w\0" as *const u8 as *const libc::c_char);
    if f.is_null() {
        Com_Printf(
            b"Couldn't write config.cfg.\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return;
    }
    fprintf(
        f,
        b"// generated by quake, do not modify\n\0" as *const u8 as *const libc::c_char,
    );
    Key_WriteBindings(f);
    fclose(f);
    Cvar_WriteVariables(path.as_mut_ptr());
}

#[no_mangle]
pub static mut cheatvars: [cheatvar_t; 12] = [
    {
        let mut init = cheatvar_t {
            name: b"timescale\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            value: b"1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            var: 0 as *const cvar_t as *mut cvar_t,
        };
        init
    },
    {
        let mut init = cheatvar_t {
            name: b"timedemo\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            var: 0 as *const cvar_t as *mut cvar_t,
        };
        init
    },
    {
        let mut init = cheatvar_t {
            name: b"r_drawworld\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            value: b"1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            var: 0 as *const cvar_t as *mut cvar_t,
        };
        init
    },
    {
        let mut init = cheatvar_t {
            name: b"cl_testlights\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            value: b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            var: 0 as *const cvar_t as *mut cvar_t,
        };
        init
    },
    {
        let mut init = cheatvar_t {
            name: b"r_fullbright\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            value: b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            var: 0 as *const cvar_t as *mut cvar_t,
        };
        init
    },
    {
        let mut init = cheatvar_t {
            name: b"r_drawflat\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            value: b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            var: 0 as *const cvar_t as *mut cvar_t,
        };
        init
    },
    {
        let mut init = cheatvar_t {
            name: b"paused\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value: b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            var: 0 as *const cvar_t as *mut cvar_t,
        };
        init
    },
    {
        let mut init = cheatvar_t {
            name: b"fixedtime\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            value: b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            var: 0 as *const cvar_t as *mut cvar_t,
        };
        init
    },
    {
        let mut init = cheatvar_t {
            name: b"sw_draworder\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            value: b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            var: 0 as *const cvar_t as *mut cvar_t,
        };
        init
    },
    {
        let mut init = cheatvar_t {
            name: b"gl_lightmap\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            value: b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            var: 0 as *const cvar_t as *mut cvar_t,
        };
        init
    },
    {
        let mut init = cheatvar_t {
            name: b"gl_saturatelighting\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            value: b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            var: 0 as *const cvar_t as *mut cvar_t,
        };
        init
    },
    {
        let mut init = cheatvar_t {
            name: 0 as *const libc::c_char as *mut libc::c_char,
            value: 0 as *const libc::c_char as *mut libc::c_char,
            var: 0 as *const cvar_t as *mut cvar_t,
        };
        init
    },
];
#[no_mangle]
pub static mut numcheatvars: libc::c_int = 0;

#[no_mangle]
pub unsafe extern "C" fn CL_FixCvarCheats() {
    let mut i: libc::c_int = 0;
    let mut var: *mut cheatvar_t = 0 as *mut cheatvar_t;
    if strcmp(
        (cl.configstrings[30 as libc::c_int as usize]).as_mut_ptr(),
        b"1\0" as *const u8 as *const libc::c_char,
    ) == 0
        || cl.configstrings[30 as libc::c_int as usize][0 as libc::c_int as usize] == 0
    {
        return;
    }
    if numcheatvars == 0 {
        while !(cheatvars[numcheatvars as usize].name).is_null() {
            cheatvars[numcheatvars as usize]
                .var = Cvar_Get(
                cheatvars[numcheatvars as usize].name,
                cheatvars[numcheatvars as usize].value,
                0 as libc::c_int,
            );
            numcheatvars += 1;
        }
    }
    i = 0 as libc::c_int;
    var = cheatvars.as_mut_ptr();
    while i < numcheatvars {
        if strcmp((*(*var).var).string, (*var).value) != 0 {
            Cvar_Set((*var).name, (*var).value);
        }
        i += 1;
        var = var.offset(1);
    }
}

#[no_mangle]
pub unsafe extern "C" fn CL_SendCommand() {
    Sys_SendKeyEvents();
    IN_Commands();
    Cbuf_Execute();
    CL_FixCvarCheats();
    CL_SendCmd();
    CL_CheckForResend();
}

#[no_mangle]
pub unsafe extern "C" fn CL_Frame(mut msec: libc::c_int) {
    static mut extratime: libc::c_int = 0;
    static mut lasttimecalled: libc::c_int = 0;
    if (*dedicated).value != 0. {
        return;
    }
    extratime += msec;
    if (*cl_timedemo).value == 0. {
        if cls.state as libc::c_uint == ca_connected as libc::c_int as libc::c_uint
            && extratime < 100 as libc::c_int
        {
            return;
        }
        if (extratime as libc::c_float)
            < 1000 as libc::c_int as libc::c_float / (*cl_maxfps).value
        {
            return;
        }
    }
    IN_Frame();
    cls.frametime = (extratime as libc::c_double / 1000.0f64) as libc::c_float;
    cl.time += extratime;
    cls.realtime = curtime;
    extratime = 0 as libc::c_int;
    if cls.frametime as libc::c_double > 1.0f64 / 5 as libc::c_int as libc::c_double {
        cls.frametime = (1.0f64 / 5 as libc::c_int as libc::c_double) as libc::c_float;
    }
    if msec > 5000 as libc::c_int {
        cls.netchan.last_received = Sys_Milliseconds();
    }
    CL_ReadPackets();
    CL_SendCommand();
    CL_PredictMovement();
    VID_CheckChanges();
    if cl.refresh_prepped as u64 == 0
        && cls.state as libc::c_uint == ca_active as libc::c_int as libc::c_uint
    {
        CL_PrepRefresh();
    }
    if (*host_speeds).value != 0. {
        time_before_ref = Sys_Milliseconds();
    }
    SCR_UpdateScreen();
    if (*host_speeds).value != 0. {
        time_after_ref = Sys_Milliseconds();
    }
    S_Update(
        (cl.refdef.vieworg).as_mut_ptr(),
        (cl.v_forward).as_mut_ptr(),
        (cl.v_right).as_mut_ptr(),
        (cl.v_up).as_mut_ptr(),
    );
    CDAudio_Update();
    CL_RunDLights();
    CL_RunLightStyles();
    SCR_RunCinematic();
    SCR_RunConsole();
    cls.framecount += 1;
    if (*log_stats).value != 0. {
        if cls.state as libc::c_uint == ca_active as libc::c_int as libc::c_uint {
            if lasttimecalled == 0 {
                lasttimecalled = Sys_Milliseconds();
                if !log_stats_file.is_null() {
                    fprintf(
                        log_stats_file,
                        b"0\n\0" as *const u8 as *const libc::c_char,
                    );
                }
            } else {
                let mut now: libc::c_int = Sys_Milliseconds();
                if !log_stats_file.is_null() {
                    fprintf(
                        log_stats_file,
                        b"%d\n\0" as *const u8 as *const libc::c_char,
                        now - lasttimecalled,
                    );
                }
                lasttimecalled = now;
            }
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn CL_Init() {
    if (*dedicated).value != 0. {
        return;
    }
    Con_Init();
    S_Init();
    VID_Init();
    V_Init();
    net_message.data = net_message_buffer.as_mut_ptr();
    net_message
        .maxsize = ::std::mem::size_of::<[byte; 1400]>() as libc::c_ulong as libc::c_int;
    M_Init();
    SCR_Init();
    cls.disable_screen = true_0 as libc::c_int as libc::c_float;
    CDAudio_Init();
    CL_InitLocal();
    IN_Init();
    FS_ExecAutoexec();
    Cbuf_Execute();
}

#[no_mangle]
pub unsafe extern "C" fn CL_Shutdown() {
    static mut isdown: qboolean = false_0;
    if isdown as u64 != 0 {
        printf(b"recursive shutdown\n\0" as *const u8 as *const libc::c_char);
        return;
    }
    isdown = true_0;
    CL_WriteConfiguration();
    CDAudio_Shutdown();
    S_Shutdown();
    IN_Shutdown();
    VID_Shutdown();
}
