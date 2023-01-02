#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(extern_types, register_tool)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn SV_BroadcastPrintf(level: libc::c_int, fmt: *mut libc::c_char, _: ...);
    static mut sv_outputbuf: [libc::c_char; 1384];
    fn SV_FlushRedirect(sv_redirected: libc::c_int, outputbuf: *mut libc::c_char);
    fn SV_ExecuteClientMessage(cl: *mut client_t);
    fn SV_SendClientMessages();
    fn SV_RecordDemoMessage();
    static mut ge: *mut game_export_t;
    fn SV_ShutdownGameProgs();
    static mut sv: server_t;
    static mut svs: server_static_t;
    fn SV_InitOperatorCommands();
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn Z_Free(ptr: *mut libc::c_void);
    static mut time_after_game: libc::c_int;
    static mut time_before_game: libc::c_int;
    static mut host_speeds: *mut cvar_t;
    static mut dedicated: *mut cvar_t;
    fn Com_SetServerState(state: libc::c_int);
    fn Com_DPrintf(fmt: *mut libc::c_char, _: ...);
    fn Com_EndRedirect();
    fn Com_BeginRedirect(
        target: libc::c_int,
        buffer: *mut libc::c_char,
        buffersize: libc::c_int,
        flush: *mut libc::c_void,
    );
    fn FS_FreeFile(buffer: *mut libc::c_void);
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn atoi(__nptr: *const libc::c_char) -> libc::c_int;
    fn rand() -> libc::c_int;
    fn Com_sprintf(
        dest: *mut libc::c_char,
        size: libc::c_int,
        fmt: *mut libc::c_char,
        _: ...
    );
    fn va(format: *mut libc::c_char, _: ...) -> *mut libc::c_char;
    fn Info_ValueForKey(
        s: *mut libc::c_char,
        key: *mut libc::c_char,
    ) -> *mut libc::c_char;
    fn Info_SetValueForKey(
        s: *mut libc::c_char,
        key: *mut libc::c_char,
        value: *mut libc::c_char,
    );
    static mut curtime: libc::c_int;
    fn Sys_Milliseconds() -> libc::c_int;
    fn Com_Printf(msg: *mut libc::c_char, _: ...);
    fn SZ_Init(buf: *mut sizebuf_t, data: *mut byte, length: libc::c_int);
    fn SZ_Clear(buf: *mut sizebuf_t);
    fn MSG_WriteByte(sb: *mut sizebuf_t, c: libc::c_int);
    fn MSG_WriteString(sb: *mut sizebuf_t, s: *mut libc::c_char);
    fn MSG_BeginReading(sb: *mut sizebuf_t);
    fn MSG_ReadShort(sb: *mut sizebuf_t) -> libc::c_int;
    fn MSG_ReadLong(sb: *mut sizebuf_t) -> libc::c_int;
    fn MSG_ReadStringLine(sb: *mut sizebuf_t) -> *mut libc::c_char;
    fn Cmd_Argc() -> libc::c_int;
    fn Cmd_Argv(arg: libc::c_int) -> *mut libc::c_char;
    fn Cmd_TokenizeString(text: *mut libc::c_char, macroExpand: qboolean);
    fn Cmd_ExecuteString(text: *mut libc::c_char);
    fn Cvar_Get(
        var_name: *mut libc::c_char,
        value: *mut libc::c_char,
        flags: libc::c_int,
    ) -> *mut cvar_t;
    fn Cvar_Serverinfo() -> *mut libc::c_char;
    fn NET_GetPacket(
        sock: netsrc_t,
        net_from_0: *mut netadr_t,
        net_message_0: *mut sizebuf_t,
    ) -> qboolean;
    fn NET_CompareBaseAdr(a: netadr_t, b: netadr_t) -> qboolean;
    fn NET_IsLocalAddress(adr: netadr_t) -> qboolean;
    fn NET_AdrToString(a: netadr_t) -> *mut libc::c_char;
    fn NET_Sleep(msec: libc::c_int);
    static mut net_from: netadr_t;
    static mut net_message: sizebuf_t;
    static mut net_message_buffer: [byte; 1400];
    fn Netchan_Setup(
        sock: netsrc_t,
        chan: *mut netchan_t,
        adr: netadr_t,
        qport: libc::c_int,
    );
    fn Netchan_Transmit(chan: *mut netchan_t, length: libc::c_int, data: *mut byte);
    fn Netchan_OutOfBandPrint(
        net_socket: libc::c_int,
        adr: netadr_t,
        format: *mut libc::c_char,
        _: ...
    );
    fn Netchan_Process(chan: *mut netchan_t, msg: *mut sizebuf_t) -> qboolean;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct edict_s {
    pub s: entity_state_t,
    pub client: *mut gclient_s,
    pub inuse: qboolean,
    pub linkcount: libc::c_int,
    pub area: link_t,
    pub num_clusters: libc::c_int,
    pub clusternums: [libc::c_int; 16],
    pub headnode: libc::c_int,
    pub areanum: libc::c_int,
    pub areanum2: libc::c_int,
    pub svflags: libc::c_int,
    pub mins: vec3_t,
    pub maxs: vec3_t,
    pub absmin: vec3_t,
    pub absmax: vec3_t,
    pub size: vec3_t,
    pub solid: solid_t,
    pub clipmask: libc::c_int,
    pub owner: *mut edict_t,
}
pub type edict_t = edict_s;
pub type solid_t = libc::c_uint;
pub const SOLID_BSP: solid_t = 3;
pub const SOLID_BBOX: solid_t = 2;
pub const SOLID_TRIGGER: solid_t = 1;
pub const SOLID_NOT: solid_t = 0;
pub type link_t = link_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct link_s {
    pub prev: *mut link_s,
    pub next: *mut link_s,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gclient_s {
    pub ps: player_state_t,
    pub ping: libc::c_int,
}
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
pub struct pmove_state_t {
    pub pm_type: pmtype_t,
    pub origin: [libc::c_short; 3],
    pub velocity: [libc::c_short; 3],
    pub pm_flags: byte,
    pub pm_time: byte,
    pub gravity: libc::c_short,
    pub delta_angles: [libc::c_short; 3],
}
pub type pmtype_t = libc::c_uint;
pub const PM_FREEZE: pmtype_t = 4;
pub const PM_GIB: pmtype_t = 3;
pub const PM_DEAD: pmtype_t = 2;
pub const PM_SPECTATOR: pmtype_t = 1;
pub const PM_NORMAL: pmtype_t = 0;
pub type entity_state_t = entity_state_s;
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
pub struct server_static_t {
    pub initialized: qboolean,
    pub realtime: libc::c_int,
    pub mapcmd: [libc::c_char; 128],
    pub spawncount: libc::c_int,
    pub clients: *mut client_t,
    pub num_client_entities: libc::c_int,
    pub next_client_entities: libc::c_int,
    pub client_entities: *mut entity_state_t,
    pub last_heartbeat: libc::c_int,
    pub challenges: [challenge_t; 1024],
    pub demofile: *mut FILE,
    pub demo_multicast: sizebuf_t,
    pub demo_multicast_buf: [byte; 1400],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct challenge_t {
    pub adr: netadr_t,
    pub challenge: libc::c_int,
    pub time: libc::c_int,
}
pub type client_t = client_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct client_s {
    pub state: client_state_t,
    pub userinfo: [libc::c_char; 512],
    pub lastframe: libc::c_int,
    pub lastcmd: usercmd_t,
    pub commandMsec: libc::c_int,
    pub frame_latency: [libc::c_int; 16],
    pub ping: libc::c_int,
    pub message_size: [libc::c_int; 10],
    pub rate: libc::c_int,
    pub surpressCount: libc::c_int,
    pub edict: *mut edict_t,
    pub name: [libc::c_char; 32],
    pub messagelevel: libc::c_int,
    pub datagram: sizebuf_t,
    pub datagram_buf: [byte; 1400],
    pub frames: [client_frame_t; 16],
    pub download: *mut byte,
    pub downloadsize: libc::c_int,
    pub downloadcount: libc::c_int,
    pub lastmessage: libc::c_int,
    pub lastconnect: libc::c_int,
    pub challenge: libc::c_int,
    pub netchan: netchan_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct client_frame_t {
    pub areabytes: libc::c_int,
    pub areabits: [byte; 32],
    pub ps: player_state_t,
    pub num_entities: libc::c_int,
    pub first_entity: libc::c_int,
    pub senttime: libc::c_int,
}
pub type client_state_t = libc::c_uint;
pub const cs_spawned: client_state_t = 3;
pub const cs_connected: client_state_t = 2;
pub const cs_zombie: client_state_t = 1;
pub const cs_free: client_state_t = 0;
pub type server_state_t = libc::c_uint;
pub const ss_pic: server_state_t = 5;
pub const ss_demo: server_state_t = 4;
pub const ss_cinematic: server_state_t = 3;
pub const ss_game: server_state_t = 2;
pub const ss_loading: server_state_t = 1;
pub const ss_dead: server_state_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct server_t {
    pub state: server_state_t,
    pub attractloop: qboolean,
    pub loadgame: qboolean,
    pub time: libc::c_uint,
    pub framenum: libc::c_int,
    pub name: [libc::c_char; 64],
    pub models: [*mut cmodel_s; 256],
    pub configstrings: [[libc::c_char; 64]; 2080],
    pub baselines: [entity_state_t; 1024],
    pub multicast: sizebuf_t,
    pub multicast_buf: [byte; 1400],
    pub demofile: *mut FILE,
    pub timedemo: qboolean,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct game_export_t {
    pub apiversion: libc::c_int,
    pub Init: Option::<unsafe extern "C" fn() -> ()>,
    pub Shutdown: Option::<unsafe extern "C" fn() -> ()>,
    pub SpawnEntities: Option::<
        unsafe extern "C" fn(
            *mut libc::c_char,
            *mut libc::c_char,
            *mut libc::c_char,
        ) -> (),
    >,
    pub WriteGame: Option::<unsafe extern "C" fn(*mut libc::c_char, qboolean) -> ()>,
    pub ReadGame: Option::<unsafe extern "C" fn(*mut libc::c_char) -> ()>,
    pub WriteLevel: Option::<unsafe extern "C" fn(*mut libc::c_char) -> ()>,
    pub ReadLevel: Option::<unsafe extern "C" fn(*mut libc::c_char) -> ()>,
    pub ClientConnect: Option::<
        unsafe extern "C" fn(*mut edict_t, *mut libc::c_char) -> qboolean,
    >,
    pub ClientBegin: Option::<unsafe extern "C" fn(*mut edict_t) -> ()>,
    pub ClientUserinfoChanged: Option::<
        unsafe extern "C" fn(*mut edict_t, *mut libc::c_char) -> (),
    >,
    pub ClientDisconnect: Option::<unsafe extern "C" fn(*mut edict_t) -> ()>,
    pub ClientCommand: Option::<unsafe extern "C" fn(*mut edict_t) -> ()>,
    pub ClientThink: Option::<unsafe extern "C" fn(*mut edict_t, *mut usercmd_t) -> ()>,
    pub RunFrame: Option::<unsafe extern "C" fn() -> ()>,
    pub ServerCommand: Option::<unsafe extern "C" fn() -> ()>,
    pub edicts: *mut edict_s,
    pub edict_size: libc::c_int,
    pub num_edicts: libc::c_int,
    pub max_edicts: libc::c_int,
}
pub const RD_PACKET: C2RustUnnamed = 2;
pub type C2RustUnnamed = libc::c_uint;
pub const RD_CLIENT: C2RustUnnamed = 1;
pub const RD_NONE: C2RustUnnamed = 0;
#[no_mangle]
pub static mut master_adr: [netadr_t; 8] = [netadr_t {
    type_0: NA_LOOPBACK,
    ip: [0; 4],
    ipx: [0; 10],
    port: 0,
}; 8];
#[no_mangle]
pub static mut sv_client: *mut client_t = 0 as *const client_t as *mut client_t;
#[no_mangle]
pub static mut sv_paused: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut sv_timedemo: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut sv_enforcetime: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut timeout: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut zombietime: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut rcon_password: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut allow_download: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut allow_download_players: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut allow_download_models: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut allow_download_sounds: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut allow_download_maps: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut sv_airaccelerate: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut sv_noreload: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut maxclients: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut sv_showclamp: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut hostname: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut public_server: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut sv_reconnect_limit: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub unsafe extern "C" fn SV_DropClient(mut drop_0: *mut client_t) {
    MSG_WriteByte(&mut (*drop_0).netchan.message, svc_disconnect as libc::c_int);
    if (*drop_0).state as libc::c_uint == cs_spawned as libc::c_int as libc::c_uint {
        ((*ge).ClientDisconnect).expect("non-null function pointer")((*drop_0).edict);
    }
    if !((*drop_0).download).is_null() {
        FS_FreeFile((*drop_0).download as *mut libc::c_void);
        let ref mut fresh0 = (*drop_0).download;
        *fresh0 = 0 as *mut byte;
    }
    (*drop_0).state = cs_zombie;
    (*drop_0).name[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn SV_StatusString() -> *mut libc::c_char {
    let mut player: [libc::c_char; 1024] = [0; 1024];
    static mut status: [libc::c_char; 1384] = [0; 1384];
    let mut i: libc::c_int = 0;
    let mut cl: *mut client_t = 0 as *mut client_t;
    let mut statusLength: libc::c_int = 0;
    let mut playerLength: libc::c_int = 0;
    strcpy(status.as_mut_ptr(), Cvar_Serverinfo());
    strcat(status.as_mut_ptr(), b"\n\0" as *const u8 as *const libc::c_char);
    statusLength = strlen(status.as_mut_ptr()) as libc::c_int;
    i = 0 as libc::c_int;
    while (i as libc::c_float) < (*maxclients).value {
        cl = &mut *(svs.clients).offset(i as isize) as *mut client_t;
        if (*cl).state as libc::c_uint == cs_connected as libc::c_int as libc::c_uint
            || (*cl).state as libc::c_uint == cs_spawned as libc::c_int as libc::c_uint
        {
            Com_sprintf(
                player.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong
                    as libc::c_int,
                b"%i %i \"%s\"\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                (*(*(*cl).edict).client).ps.stats[14 as libc::c_int as usize]
                    as libc::c_int,
                (*cl).ping,
                ((*cl).name).as_mut_ptr(),
            );
            playerLength = strlen(player.as_mut_ptr()) as libc::c_int;
            if (statusLength + playerLength) as libc::c_ulong
                >= ::std::mem::size_of::<[libc::c_char; 1384]>() as libc::c_ulong
            {
                break;
            }
            strcpy(
                status.as_mut_ptr().offset(statusLength as isize),
                player.as_mut_ptr(),
            );
            statusLength += playerLength;
        }
        i += 1;
    }
    return status.as_mut_ptr();
}
#[no_mangle]
pub unsafe extern "C" fn SVC_Status() {
    Netchan_OutOfBandPrint(
        NS_SERVER as libc::c_int,
        net_from,
        b"print\n%s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        SV_StatusString(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn SVC_Ack() {
    Com_Printf(
        b"Ping acknowledge from %s\n\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        NET_AdrToString(net_from),
    );
}
#[no_mangle]
pub unsafe extern "C" fn SVC_Info() {
    let mut string: [libc::c_char; 64] = [0; 64];
    let mut i: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    let mut version: libc::c_int = 0;
    if (*maxclients).value == 1 as libc::c_int as libc::c_float {
        return;
    }
    version = atoi(Cmd_Argv(1 as libc::c_int));
    if version != 34 as libc::c_int {
        Com_sprintf(
            string.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
            b"%s: wrong version\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            (*hostname).string,
            ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong,
        );
    } else {
        count = 0 as libc::c_int;
        i = 0 as libc::c_int;
        while (i as libc::c_float) < (*maxclients).value {
            if (*(svs.clients).offset(i as isize)).state as libc::c_uint
                >= cs_connected as libc::c_int as libc::c_uint
            {
                count += 1;
            }
            i += 1;
        }
        Com_sprintf(
            string.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
            b"%16s %8s %2i/%2i\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            (*hostname).string,
            (sv.name).as_mut_ptr(),
            count,
            (*maxclients).value as libc::c_int,
        );
    }
    Netchan_OutOfBandPrint(
        NS_SERVER as libc::c_int,
        net_from,
        b"info\n%s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        string.as_mut_ptr(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn SVC_Ping() {
    Netchan_OutOfBandPrint(
        NS_SERVER as libc::c_int,
        net_from,
        b"ack\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
}
#[no_mangle]
pub unsafe extern "C" fn SVC_GetChallenge() {
    let mut i: libc::c_int = 0;
    let mut oldest: libc::c_int = 0;
    let mut oldestTime: libc::c_int = 0;
    oldest = 0 as libc::c_int;
    oldestTime = 0x7fffffff as libc::c_int;
    i = 0 as libc::c_int;
    while i < 1024 as libc::c_int {
        if NET_CompareBaseAdr(net_from, svs.challenges[i as usize].adr) as u64 != 0 {
            break;
        }
        if svs.challenges[i as usize].time < oldestTime {
            oldestTime = svs.challenges[i as usize].time;
            oldest = i;
        }
        i += 1;
    }
    if i == 1024 as libc::c_int {
        svs.challenges[oldest as usize].challenge = rand() & 0x7fff as libc::c_int;
        svs.challenges[oldest as usize].adr = net_from;
        svs.challenges[oldest as usize].time = curtime;
        i = oldest;
    }
    Netchan_OutOfBandPrint(
        NS_SERVER as libc::c_int,
        net_from,
        b"challenge %i\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        svs.challenges[i as usize].challenge,
    );
}
#[no_mangle]
pub unsafe extern "C" fn SVC_DirectConnect() {
    let mut current_block: u64;
    let mut userinfo: [libc::c_char; 512] = [0; 512];
    let mut adr: netadr_t = netadr_t {
        type_0: NA_LOOPBACK,
        ip: [0; 4],
        ipx: [0; 10],
        port: 0,
    };
    let mut i: libc::c_int = 0;
    let mut cl: *mut client_t = 0 as *mut client_t;
    let mut newcl: *mut client_t = 0 as *mut client_t;
    let mut temp: client_t = client_t {
        state: cs_free,
        userinfo: [0; 512],
        lastframe: 0,
        lastcmd: usercmd_t {
            msec: 0,
            buttons: 0,
            angles: [0; 3],
            forwardmove: 0,
            sidemove: 0,
            upmove: 0,
            impulse: 0,
            lightlevel: 0,
        },
        commandMsec: 0,
        frame_latency: [0; 16],
        ping: 0,
        message_size: [0; 10],
        rate: 0,
        surpressCount: 0,
        edict: 0 as *mut edict_t,
        name: [0; 32],
        messagelevel: 0,
        datagram: sizebuf_t {
            allowoverflow: false_0,
            overflowed: false_0,
            data: 0 as *const byte as *mut byte,
            maxsize: 0,
            cursize: 0,
            readcount: 0,
        },
        datagram_buf: [0; 1400],
        frames: [client_frame_t {
            areabytes: 0,
            areabits: [0; 32],
            ps: player_state_t {
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
            first_entity: 0,
            senttime: 0,
        }; 16],
        download: 0 as *mut byte,
        downloadsize: 0,
        downloadcount: 0,
        lastmessage: 0,
        lastconnect: 0,
        challenge: 0,
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
    };
    let mut ent: *mut edict_t = 0 as *mut edict_t;
    let mut edictnum: libc::c_int = 0;
    let mut version: libc::c_int = 0;
    let mut qport: libc::c_int = 0;
    let mut challenge: libc::c_int = 0;
    adr = net_from;
    Com_DPrintf(
        b"SVC_DirectConnect ()\n\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    version = atoi(Cmd_Argv(1 as libc::c_int));
    if version != 34 as libc::c_int {
        Netchan_OutOfBandPrint(
            NS_SERVER as libc::c_int,
            adr,
            b"print\nServer is version %4.2f.\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            3.19f64,
        );
        Com_DPrintf(
            b"    rejected connect from version %i\n\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            version,
        );
        return;
    }
    qport = atoi(Cmd_Argv(2 as libc::c_int));
    challenge = atoi(Cmd_Argv(3 as libc::c_int));
    strncpy(
        userinfo.as_mut_ptr(),
        Cmd_Argv(4 as libc::c_int),
        (::std::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
    );
    userinfo[(::std::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        as usize] = 0 as libc::c_int as libc::c_char;
    Info_SetValueForKey(
        userinfo.as_mut_ptr(),
        b"ip\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        NET_AdrToString(net_from),
    );
    if sv.attractloop as u64 != 0 {
        if NET_IsLocalAddress(adr) as u64 == 0 {
            Com_Printf(
                b"Remote connect in attract loop.  Ignored.\n\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
            );
            Netchan_OutOfBandPrint(
                NS_SERVER as libc::c_int,
                adr,
                b"print\nConnection refused.\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
            return;
        }
    }
    if NET_IsLocalAddress(adr) as u64 == 0 {
        i = 0 as libc::c_int;
        while i < 1024 as libc::c_int {
            if NET_CompareBaseAdr(net_from, svs.challenges[i as usize].adr) as u64 != 0 {
                if challenge == svs.challenges[i as usize].challenge {
                    break;
                }
                Netchan_OutOfBandPrint(
                    NS_SERVER as libc::c_int,
                    adr,
                    b"print\nBad challenge.\n\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                );
                return;
            } else {
                i += 1;
            }
        }
        if i == 1024 as libc::c_int {
            Netchan_OutOfBandPrint(
                NS_SERVER as libc::c_int,
                adr,
                b"print\nNo challenge for address.\n\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
            );
            return;
        }
    }
    newcl = &mut temp;
    memset(
        newcl as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<client_t>() as libc::c_ulong,
    );
    i = 0 as libc::c_int;
    cl = svs.clients;
    loop {
        if !((i as libc::c_float) < (*maxclients).value) {
            current_block = 1345366029464561491;
            break;
        }
        if !((*cl).state as libc::c_uint == cs_free as libc::c_int as libc::c_uint) {
            if NET_CompareBaseAdr(adr, (*cl).netchan.remote_address) as libc::c_uint != 0
                && ((*cl).netchan.qport == qport
                    || adr.port as libc::c_int
                        == (*cl).netchan.remote_address.port as libc::c_int)
            {
                if NET_IsLocalAddress(adr) as u64 == 0
                    && svs.realtime - (*cl).lastconnect
                        < (*sv_reconnect_limit).value as libc::c_int
                            * 1000 as libc::c_int
                {
                    Com_DPrintf(
                        b"%s:reconnect rejected : too soon\n\0" as *const u8
                            as *const libc::c_char as *mut libc::c_char,
                        NET_AdrToString(adr),
                    );
                    return;
                }
                Com_Printf(
                    b"%s:reconnect\n\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    NET_AdrToString(adr),
                );
                newcl = cl;
                current_block = 2380987886157893679;
                break;
            }
        }
        i += 1;
        cl = cl.offset(1);
    }
    match current_block {
        1345366029464561491 => {
            newcl = 0 as *mut client_t;
            i = 0 as libc::c_int;
            cl = svs.clients;
            while (i as libc::c_float) < (*maxclients).value {
                if (*cl).state as libc::c_uint == cs_free as libc::c_int as libc::c_uint
                {
                    newcl = cl;
                    break;
                } else {
                    i += 1;
                    cl = cl.offset(1);
                }
            }
            if newcl.is_null() {
                Netchan_OutOfBandPrint(
                    NS_SERVER as libc::c_int,
                    adr,
                    b"print\nServer is full.\n\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                );
                Com_DPrintf(
                    b"Rejected a connection.\n\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                );
                return;
            }
        }
        _ => {}
    }
    *newcl = temp;
    sv_client = newcl;
    edictnum = (newcl.offset_from(svs.clients) as libc::c_long
        + 1 as libc::c_int as libc::c_long) as libc::c_int;
    ent = ((*ge).edicts as *mut byte).offset(((*ge).edict_size * edictnum) as isize)
        as *mut edict_t;
    let ref mut fresh1 = (*newcl).edict;
    *fresh1 = ent;
    (*newcl).challenge = challenge;
    if ((*ge).ClientConnect)
        .expect("non-null function pointer")(ent, userinfo.as_mut_ptr()) as u64 == 0
    {
        if *Info_ValueForKey(
            userinfo.as_mut_ptr(),
            b"rejmsg\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) != 0
        {
            Netchan_OutOfBandPrint(
                NS_SERVER as libc::c_int,
                adr,
                b"print\n%s\nConnection refused.\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                Info_ValueForKey(
                    userinfo.as_mut_ptr(),
                    b"rejmsg\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                ),
            );
        } else {
            Netchan_OutOfBandPrint(
                NS_SERVER as libc::c_int,
                adr,
                b"print\nConnection refused.\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
        }
        Com_DPrintf(
            b"Game rejected a connection.\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return;
    }
    strncpy(
        ((*newcl).userinfo).as_mut_ptr(),
        userinfo.as_mut_ptr(),
        (::std::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
    );
    SV_UserinfoChanged(newcl);
    Netchan_OutOfBandPrint(
        NS_SERVER as libc::c_int,
        adr,
        b"client_connect\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    Netchan_Setup(NS_SERVER, &mut (*newcl).netchan, adr, qport);
    (*newcl).state = cs_connected;
    SZ_Init(
        &mut (*newcl).datagram,
        ((*newcl).datagram_buf).as_mut_ptr(),
        ::std::mem::size_of::<[byte; 1400]>() as libc::c_ulong as libc::c_int,
    );
    (*newcl).datagram.allowoverflow = true_0;
    (*newcl).lastmessage = svs.realtime;
    (*newcl).lastconnect = svs.realtime;
}
#[no_mangle]
pub unsafe extern "C" fn Rcon_Validate() -> libc::c_int {
    if strlen((*rcon_password).string) == 0 {
        return 0 as libc::c_int;
    }
    if strcmp(Cmd_Argv(1 as libc::c_int), (*rcon_password).string) != 0 {
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn SVC_RemoteCommand() {
    let mut i: libc::c_int = 0;
    let mut remaining: [libc::c_char; 1024] = [0; 1024];
    i = Rcon_Validate();
    if i == 0 as libc::c_int {
        Com_Printf(
            b"Bad rcon from %s:\n%s\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            NET_AdrToString(net_from),
            (net_message.data).offset(4 as libc::c_int as isize),
        );
    } else {
        Com_Printf(
            b"Rcon from %s:\n%s\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            NET_AdrToString(net_from),
            (net_message.data).offset(4 as libc::c_int as isize),
        );
    }
    Com_BeginRedirect(
        RD_PACKET as libc::c_int,
        sv_outputbuf.as_mut_ptr(),
        1400 as libc::c_int - 16 as libc::c_int,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn(libc::c_int, *mut libc::c_char) -> ()>,
            *mut libc::c_void,
        >(
            Some(
                SV_FlushRedirect
                    as unsafe extern "C" fn(libc::c_int, *mut libc::c_char) -> (),
            ),
        ),
    );
    if Rcon_Validate() == 0 {
        Com_Printf(
            b"Bad rcon_password.\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    } else {
        remaining[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        i = 2 as libc::c_int;
        while i < Cmd_Argc() {
            strcat(remaining.as_mut_ptr(), Cmd_Argv(i));
            strcat(remaining.as_mut_ptr(), b" \0" as *const u8 as *const libc::c_char);
            i += 1;
        }
        Cmd_ExecuteString(remaining.as_mut_ptr());
    }
    Com_EndRedirect();
}
#[no_mangle]
pub unsafe extern "C" fn SV_ConnectionlessPacket() {
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut c: *mut libc::c_char = 0 as *mut libc::c_char;
    MSG_BeginReading(&mut net_message);
    MSG_ReadLong(&mut net_message);
    s = MSG_ReadStringLine(&mut net_message);
    Cmd_TokenizeString(s, false_0);
    c = Cmd_Argv(0 as libc::c_int);
    Com_DPrintf(
        b"Packet %s : %s\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        NET_AdrToString(net_from),
        c,
    );
    if strcmp(c, b"ping\0" as *const u8 as *const libc::c_char) == 0 {
        SVC_Ping();
    } else if strcmp(c, b"ack\0" as *const u8 as *const libc::c_char) == 0 {
        SVC_Ack();
    } else if strcmp(c, b"status\0" as *const u8 as *const libc::c_char) == 0 {
        SVC_Status();
    } else if strcmp(c, b"info\0" as *const u8 as *const libc::c_char) == 0 {
        SVC_Info();
    } else if strcmp(c, b"getchallenge\0" as *const u8 as *const libc::c_char) == 0 {
        SVC_GetChallenge();
    } else if strcmp(c, b"connect\0" as *const u8 as *const libc::c_char) == 0 {
        SVC_DirectConnect();
    } else if strcmp(c, b"rcon\0" as *const u8 as *const libc::c_char) == 0 {
        SVC_RemoteCommand();
    } else {
        Com_Printf(
            b"bad connectionless packet from %s:\n%s\n\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            NET_AdrToString(net_from),
            s,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn SV_CalcPings() {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut cl: *mut client_t = 0 as *mut client_t;
    let mut total: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    i = 0 as libc::c_int;
    while (i as libc::c_float) < (*maxclients).value {
        cl = &mut *(svs.clients).offset(i as isize) as *mut client_t;
        if !((*cl).state as libc::c_uint != cs_spawned as libc::c_int as libc::c_uint) {
            total = 0 as libc::c_int;
            count = 0 as libc::c_int;
            j = 0 as libc::c_int;
            while j < 16 as libc::c_int {
                if (*cl).frame_latency[j as usize] > 0 as libc::c_int {
                    count += 1;
                    total += (*cl).frame_latency[j as usize];
                }
                j += 1;
            }
            if count == 0 {
                (*cl).ping = 0 as libc::c_int;
            } else {
                (*cl).ping = total / count;
            }
            (*(*(*cl).edict).client).ping = (*cl).ping;
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn SV_GiveMsec() {
    let mut i: libc::c_int = 0;
    let mut cl: *mut client_t = 0 as *mut client_t;
    if sv.framenum & 15 as libc::c_int != 0 {
        return;
    }
    i = 0 as libc::c_int;
    while (i as libc::c_float) < (*maxclients).value {
        cl = &mut *(svs.clients).offset(i as isize) as *mut client_t;
        if !((*cl).state as libc::c_uint == cs_free as libc::c_int as libc::c_uint) {
            (*cl).commandMsec = 1800 as libc::c_int;
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn SV_ReadPackets() {
    let mut i: libc::c_int = 0;
    let mut cl: *mut client_t = 0 as *mut client_t;
    let mut qport: libc::c_int = 0;
    while NET_GetPacket(NS_SERVER, &mut net_from, &mut net_message) as u64 != 0 {
        if *(net_message.data as *mut libc::c_int) == -(1 as libc::c_int) {
            SV_ConnectionlessPacket();
        } else {
            MSG_BeginReading(&mut net_message);
            MSG_ReadLong(&mut net_message);
            MSG_ReadLong(&mut net_message);
            qport = MSG_ReadShort(&mut net_message) & 0xffff as libc::c_int;
            i = 0 as libc::c_int;
            cl = svs.clients;
            while (i as libc::c_float) < (*maxclients).value {
                if !((*cl).state as libc::c_uint
                    == cs_free as libc::c_int as libc::c_uint)
                {
                    if !(NET_CompareBaseAdr(net_from, (*cl).netchan.remote_address)
                        as u64 == 0)
                    {
                        if !((*cl).netchan.qport != qport) {
                            if (*cl).netchan.remote_address.port as libc::c_int
                                != net_from.port as libc::c_int
                            {
                                Com_Printf(
                                    b"SV_ReadPackets: fixing up a translated port\n\0"
                                        as *const u8 as *const libc::c_char as *mut libc::c_char,
                                );
                                (*cl).netchan.remote_address.port = net_from.port;
                            }
                            if Netchan_Process(&mut (*cl).netchan, &mut net_message)
                                as u64 != 0
                            {
                                if (*cl).state as libc::c_uint
                                    != cs_zombie as libc::c_int as libc::c_uint
                                {
                                    (*cl).lastmessage = svs.realtime;
                                    SV_ExecuteClientMessage(cl);
                                }
                            }
                            break;
                        }
                    }
                }
                i += 1;
                cl = cl.offset(1);
            }
            i as libc::c_float != (*maxclients).value;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn SV_CheckTimeouts() {
    let mut i: libc::c_int = 0;
    let mut cl: *mut client_t = 0 as *mut client_t;
    let mut droppoint: libc::c_int = 0;
    let mut zombiepoint: libc::c_int = 0;
    droppoint = (svs.realtime as libc::c_float
        - 1000 as libc::c_int as libc::c_float * (*timeout).value) as libc::c_int;
    zombiepoint = (svs.realtime as libc::c_float
        - 1000 as libc::c_int as libc::c_float * (*zombietime).value) as libc::c_int;
    i = 0 as libc::c_int;
    cl = svs.clients;
    while (i as libc::c_float) < (*maxclients).value {
        if (*cl).lastmessage > svs.realtime {
            (*cl).lastmessage = svs.realtime;
        }
        if (*cl).state as libc::c_uint == cs_zombie as libc::c_int as libc::c_uint
            && (*cl).lastmessage < zombiepoint
        {
            (*cl).state = cs_free;
        } else if ((*cl).state as libc::c_uint
            == cs_connected as libc::c_int as libc::c_uint
            || (*cl).state as libc::c_uint == cs_spawned as libc::c_int as libc::c_uint)
            && (*cl).lastmessage < droppoint
        {
            SV_BroadcastPrintf(
                2 as libc::c_int,
                b"%s timed out\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ((*cl).name).as_mut_ptr(),
            );
            SV_DropClient(cl);
            (*cl).state = cs_free;
        }
        i += 1;
        cl = cl.offset(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn SV_PrepWorldFrame() {
    let mut ent: *mut edict_t = 0 as *mut edict_t;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*ge).num_edicts {
        ent = ((*ge).edicts as *mut byte).offset(((*ge).edict_size * i) as isize)
            as *mut edict_t;
        (*ent).s.event = 0 as libc::c_int;
        i += 1;
        ent = ent.offset(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn SV_RunGameFrame() {
    if (*host_speeds).value != 0. {
        time_before_game = Sys_Milliseconds();
    }
    sv.framenum += 1;
    sv.time = (sv.framenum * 100 as libc::c_int) as libc::c_uint;
    if (*sv_paused).value == 0.
        || (*maxclients).value > 1 as libc::c_int as libc::c_float
    {
        ((*ge).RunFrame).expect("non-null function pointer")();
        if sv.time < svs.realtime as libc::c_uint {
            if (*sv_showclamp).value != 0. {
                Com_Printf(
                    b"sv highclamp\n\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                );
            }
            svs.realtime = sv.time as libc::c_int;
        }
    }
    if (*host_speeds).value != 0. {
        time_after_game = Sys_Milliseconds();
    }
}
#[no_mangle]
pub unsafe extern "C" fn SV_Frame(mut msec: libc::c_int) {
    time_after_game = 0 as libc::c_int;
    time_before_game = time_after_game;
    if svs.initialized as u64 == 0 {
        return;
    }
    svs.realtime += msec;
    rand();
    SV_CheckTimeouts();
    SV_ReadPackets();
    if (*sv_timedemo).value == 0. && (svs.realtime as libc::c_uint) < sv.time {
        if (sv.time).wrapping_sub(svs.realtime as libc::c_uint)
            > 100 as libc::c_int as libc::c_uint
        {
            if (*sv_showclamp).value != 0. {
                Com_Printf(
                    b"sv lowclamp\n\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                );
            }
            svs
                .realtime = (sv.time).wrapping_sub(100 as libc::c_int as libc::c_uint)
                as libc::c_int;
        }
        NET_Sleep((sv.time).wrapping_sub(svs.realtime as libc::c_uint) as libc::c_int);
        return;
    }
    SV_CalcPings();
    SV_GiveMsec();
    SV_RunGameFrame();
    SV_SendClientMessages();
    SV_RecordDemoMessage();
    Master_Heartbeat();
    SV_PrepWorldFrame();
}
#[no_mangle]
pub unsafe extern "C" fn Master_Heartbeat() {
    let mut string: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    if (*dedicated).value == 0. {
        return;
    }
    if (*public_server).value == 0. {
        return;
    }
    if svs.last_heartbeat > svs.realtime {
        svs.last_heartbeat = svs.realtime;
    }
    if svs.realtime - svs.last_heartbeat < 300 as libc::c_int * 1000 as libc::c_int {
        return;
    }
    svs.last_heartbeat = svs.realtime;
    string = SV_StatusString();
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        if master_adr[i as usize].port != 0 {
            Com_Printf(
                b"Sending heartbeat to %s\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                NET_AdrToString(master_adr[i as usize]),
            );
            Netchan_OutOfBandPrint(
                NS_SERVER as libc::c_int,
                master_adr[i as usize],
                b"heartbeat\n%s\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                string,
            );
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn Master_Shutdown() {
    let mut i: libc::c_int = 0;
    if (*dedicated).value == 0. {
        return;
    }
    if (*public_server).value == 0. {
        return;
    }
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        if master_adr[i as usize].port != 0 {
            if i > 0 as libc::c_int {
                Com_Printf(
                    b"Sending heartbeat to %s\n\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    NET_AdrToString(master_adr[i as usize]),
                );
            }
            Netchan_OutOfBandPrint(
                NS_SERVER as libc::c_int,
                master_adr[i as usize],
                b"shutdown\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn SV_UserinfoChanged(mut cl: *mut client_t) {
    let mut val: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    ((*ge).ClientUserinfoChanged)
        .expect("non-null function pointer")((*cl).edict, ((*cl).userinfo).as_mut_ptr());
    strncpy(
        ((*cl).name).as_mut_ptr(),
        Info_ValueForKey(
            ((*cl).userinfo).as_mut_ptr(),
            b"name\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ),
        (::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
    );
    i = 0 as libc::c_int;
    while (i as libc::c_ulong)
        < ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong
    {
        let ref mut fresh2 = (*cl).name[i as usize];
        *fresh2 = (*fresh2 as libc::c_int & 127 as libc::c_int) as libc::c_char;
        i += 1;
    }
    val = Info_ValueForKey(
        ((*cl).userinfo).as_mut_ptr(),
        b"rate\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if strlen(val) != 0 {
        i = atoi(val);
        (*cl).rate = i;
        if (*cl).rate < 100 as libc::c_int {
            (*cl).rate = 100 as libc::c_int;
        }
        if (*cl).rate > 15000 as libc::c_int {
            (*cl).rate = 15000 as libc::c_int;
        }
    } else {
        (*cl).rate = 5000 as libc::c_int;
    }
    val = Info_ValueForKey(
        ((*cl).userinfo).as_mut_ptr(),
        b"msg\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if strlen(val) != 0 {
        (*cl).messagelevel = atoi(val);
    }
}
#[no_mangle]
pub unsafe extern "C" fn SV_Init() {
    SV_InitOperatorCommands();
    rcon_password = Cvar_Get(
        b"rcon_password\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    Cvar_Get(
        b"skill\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    Cvar_Get(
        b"deathmatch\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        16 as libc::c_int,
    );
    Cvar_Get(
        b"coop\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        16 as libc::c_int,
    );
    Cvar_Get(
        b"dmflags\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        va(
            b"%i\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            0x10 as libc::c_int,
        ),
        4 as libc::c_int,
    );
    Cvar_Get(
        b"fraglimit\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        4 as libc::c_int,
    );
    Cvar_Get(
        b"timelimit\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        4 as libc::c_int,
    );
    Cvar_Get(
        b"cheats\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        4 as libc::c_int | 16 as libc::c_int,
    );
    Cvar_Get(
        b"protocol\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        va(
            b"%i\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            34 as libc::c_int,
        ),
        4 as libc::c_int | 8 as libc::c_int,
    );
    maxclients = Cvar_Get(
        b"maxclients\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        4 as libc::c_int | 16 as libc::c_int,
    );
    hostname = Cvar_Get(
        b"hostname\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"noname\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        4 as libc::c_int | 1 as libc::c_int,
    );
    timeout = Cvar_Get(
        b"timeout\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"125\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    zombietime = Cvar_Get(
        b"zombietime\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    sv_showclamp = Cvar_Get(
        b"showclamp\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    sv_paused = Cvar_Get(
        b"paused\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    sv_timedemo = Cvar_Get(
        b"timedemo\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    sv_enforcetime = Cvar_Get(
        b"sv_enforcetime\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    allow_download = Cvar_Get(
        b"allow_download\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        1 as libc::c_int,
    );
    allow_download_players = Cvar_Get(
        b"allow_download_players\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        1 as libc::c_int,
    );
    allow_download_models = Cvar_Get(
        b"allow_download_models\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        b"1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        1 as libc::c_int,
    );
    allow_download_sounds = Cvar_Get(
        b"allow_download_sounds\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        b"1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        1 as libc::c_int,
    );
    allow_download_maps = Cvar_Get(
        b"allow_download_maps\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        b"1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        1 as libc::c_int,
    );
    sv_noreload = Cvar_Get(
        b"sv_noreload\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    sv_airaccelerate = Cvar_Get(
        b"sv_airaccelerate\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        16 as libc::c_int,
    );
    public_server = Cvar_Get(
        b"public\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    sv_reconnect_limit = Cvar_Get(
        b"sv_reconnect_limit\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"3\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        1 as libc::c_int,
    );
    SZ_Init(
        &mut net_message,
        net_message_buffer.as_mut_ptr(),
        ::std::mem::size_of::<[byte; 1400]>() as libc::c_ulong as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn SV_FinalMessage(
    mut message: *mut libc::c_char,
    mut reconnect: qboolean,
) {
    let mut i: libc::c_int = 0;
    let mut cl: *mut client_t = 0 as *mut client_t;
    SZ_Clear(&mut net_message);
    MSG_WriteByte(&mut net_message, svc_print as libc::c_int);
    MSG_WriteByte(&mut net_message, 2 as libc::c_int);
    MSG_WriteString(&mut net_message, message);
    if reconnect as u64 != 0 {
        MSG_WriteByte(&mut net_message, svc_reconnect as libc::c_int);
    } else {
        MSG_WriteByte(&mut net_message, svc_disconnect as libc::c_int);
    }
    i = 0 as libc::c_int;
    cl = svs.clients;
    while (i as libc::c_float) < (*maxclients).value {
        if (*cl).state as libc::c_uint >= cs_connected as libc::c_int as libc::c_uint {
            Netchan_Transmit(&mut (*cl).netchan, net_message.cursize, net_message.data);
        }
        i += 1;
        cl = cl.offset(1);
    }
    i = 0 as libc::c_int;
    cl = svs.clients;
    while (i as libc::c_float) < (*maxclients).value {
        if (*cl).state as libc::c_uint >= cs_connected as libc::c_int as libc::c_uint {
            Netchan_Transmit(&mut (*cl).netchan, net_message.cursize, net_message.data);
        }
        i += 1;
        cl = cl.offset(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn SV_Shutdown(
    mut finalmsg: *mut libc::c_char,
    mut reconnect: qboolean,
) {
    if !(svs.clients).is_null() {
        SV_FinalMessage(finalmsg, reconnect);
    }
    Master_Shutdown();
    SV_ShutdownGameProgs();
    if !(sv.demofile).is_null() {
        fclose(sv.demofile);
    }
    memset(
        &mut sv as *mut server_t as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<server_t>() as libc::c_ulong,
    );
    Com_SetServerState(sv.state as libc::c_int);
    if !(svs.clients).is_null() {
        Z_Free(svs.clients as *mut libc::c_void);
    }
    if !(svs.client_entities).is_null() {
        Z_Free(svs.client_entities as *mut libc::c_void);
    }
    if !(svs.demofile).is_null() {
        fclose(svs.demofile);
    }
    memset(
        &mut svs as *mut server_static_t as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<server_static_t>() as libc::c_ulong,
    );
}
