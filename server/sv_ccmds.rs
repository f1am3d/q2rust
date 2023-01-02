#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(extern_types, register_tool)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn remove(__filename: *const libc::c_char) -> libc::c_int;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fread(
        _: *mut libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn fwrite(
        _: *const libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    static mut dedicated: *mut cvar_t;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strncat(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn atoi(__nptr: *const libc::c_char) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn time(__timer: *mut time_t) -> time_t;
    fn localtime(__timer: *const time_t) -> *mut tm;
    fn Com_sprintf(
        dest: *mut libc::c_char,
        size: libc::c_int,
        fmt: *mut libc::c_char,
        _: ...
    );
    fn BigShort(l: libc::c_short) -> libc::c_short;
    fn LittleLong(l: libc::c_int) -> libc::c_int;
    fn va(format: *mut libc::c_char, _: ...) -> *mut libc::c_char;
    fn Sys_FindFirst(
        path: *mut libc::c_char,
        musthave: libc::c_uint,
        canthave: libc::c_uint,
    ) -> *mut libc::c_char;
    fn Sys_FindNext(musthave: libc::c_uint, canthave: libc::c_uint) -> *mut libc::c_char;
    fn Sys_FindClose();
    fn Com_Printf(msg: *mut libc::c_char, _: ...);
    fn SZ_Init(buf: *mut sizebuf_t, data: *mut byte, length: libc::c_int);
    fn MSG_WriteByte(sb: *mut sizebuf_t, c: libc::c_int);
    fn MSG_WriteShort(sb: *mut sizebuf_t, c: libc::c_int);
    fn MSG_WriteLong(sb: *mut sizebuf_t, c: libc::c_int);
    fn MSG_WriteString(sb: *mut sizebuf_t, s: *mut libc::c_char);
    fn Info_Print(s: *mut libc::c_char);
    fn Cmd_AddCommand(cmd_name: *mut libc::c_char, function: xcommand_t);
    fn Cmd_Argc() -> libc::c_int;
    fn Cmd_Argv(arg: libc::c_int) -> *mut libc::c_char;
    fn Cmd_Args() -> *mut libc::c_char;
    static mut cvar_vars: *mut cvar_t;
    fn Cvar_Set(var_name: *mut libc::c_char, value: *mut libc::c_char) -> *mut cvar_t;
    fn Cvar_ForceSet(
        var_name: *mut libc::c_char,
        value: *mut libc::c_char,
    ) -> *mut cvar_t;
    fn Cvar_VariableValue(var_name: *mut libc::c_char) -> libc::c_float;
    fn Cvar_VariableString(var_name: *mut libc::c_char) -> *mut libc::c_char;
    fn Cvar_Serverinfo() -> *mut libc::c_char;
    fn NET_Config(multiplayer: qboolean);
    fn NET_AdrToString(a: netadr_t) -> *mut libc::c_char;
    fn NET_StringToAdr(s: *mut libc::c_char, a: *mut netadr_t) -> qboolean;
    fn Netchan_OutOfBandPrint(
        net_socket: libc::c_int,
        adr: netadr_t,
        format: *mut libc::c_char,
        _: ...
    );
    fn CM_WritePortalState(f: *mut FILE);
    fn CM_ReadPortalState(f: *mut FILE);
    fn FS_Gamedir() -> *mut libc::c_char;
    fn FS_LoadFile(
        path: *mut libc::c_char,
        buffer: *mut *mut libc::c_void,
    ) -> libc::c_int;
    fn FS_Read(buffer: *mut libc::c_void, len: libc::c_int, f: *mut FILE);
    fn FS_CreatePath(path: *mut libc::c_char);
    fn Com_DPrintf(fmt: *mut libc::c_char, _: ...);
    fn SV_Shutdown(finalmsg: *mut libc::c_char, reconnect: qboolean);
    static mut master_adr: [netadr_t; 8];
    static mut svs: server_static_t;
    static mut sv: server_t;
    static mut maxclients: *mut cvar_t;
    static mut sv_client: *mut client_t;
    static mut sv_player: *mut edict_t;
    fn SV_DropClient(drop_0: *mut client_t);
    static mut ge: *mut game_export_t;
    fn SV_Map(attractloop: qboolean, levelstring: *mut libc::c_char, loadgame: qboolean);
    fn SV_InitGame();
    fn SV_ClientPrintf(
        cl: *mut client_t,
        level: libc::c_int,
        fmt: *mut libc::c_char,
        _: ...
    );
    fn SV_BroadcastPrintf(level: libc::c_int, fmt: *mut libc::c_char, _: ...);
}
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
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
pub type time_t = __time_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tm {
    pub tm_sec: libc::c_int,
    pub tm_min: libc::c_int,
    pub tm_hour: libc::c_int,
    pub tm_mday: libc::c_int,
    pub tm_mon: libc::c_int,
    pub tm_year: libc::c_int,
    pub tm_wday: libc::c_int,
    pub tm_yday: libc::c_int,
    pub tm_isdst: libc::c_int,
    pub tm_gmtoff: libc::c_long,
    pub tm_zone: *const libc::c_char,
}
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
pub type client_state_t = libc::c_uint;
pub const cs_spawned: client_state_t = 3;
pub const cs_connected: client_state_t = 2;
pub const cs_zombie: client_state_t = 1;
pub const cs_free: client_state_t = 0;
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
pub type client_t = client_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct challenge_t {
    pub adr: netadr_t,
    pub challenge: libc::c_int,
    pub time: libc::c_int,
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
#[no_mangle]
pub unsafe extern "C" fn SV_SetMaster_f() {
    let mut i: libc::c_int = 0;
    let mut slot: libc::c_int = 0;
    if (*dedicated).value == 0. {
        Com_Printf(
            b"Only dedicated servers use masters.\n\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        );
        return;
    }
    Cvar_Set(
        b"public\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    i = 1 as libc::c_int;
    while i < 8 as libc::c_int {
        memset(
            &mut *master_adr.as_mut_ptr().offset(i as isize) as *mut netadr_t
                as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<netadr_t>() as libc::c_ulong,
        );
        i += 1;
    }
    slot = 1 as libc::c_int;
    i = 1 as libc::c_int;
    while i < Cmd_Argc() {
        if slot == 8 as libc::c_int {
            break;
        }
        if NET_StringToAdr(Cmd_Argv(i), &mut *master_adr.as_mut_ptr().offset(i as isize))
            as u64 == 0
        {
            Com_Printf(
                b"Bad address: %s\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                Cmd_Argv(i),
            );
        } else {
            if master_adr[slot as usize].port as libc::c_int == 0 as libc::c_int {
                master_adr[slot as usize]
                    .port = BigShort(27900 as libc::c_int as libc::c_short)
                    as libc::c_ushort;
            }
            Com_Printf(
                b"Master server at %s\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                NET_AdrToString(master_adr[slot as usize]),
            );
            Com_Printf(
                b"Sending a ping.\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
            Netchan_OutOfBandPrint(
                NS_SERVER as libc::c_int,
                master_adr[slot as usize],
                b"ping\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            slot += 1;
        }
        i += 1;
    }
    svs.last_heartbeat = -(9999999 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn SV_SetPlayer() -> qboolean {
    let mut cl: *mut client_t = 0 as *mut client_t;
    let mut i: libc::c_int = 0;
    let mut idnum: libc::c_int = 0;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    if Cmd_Argc() < 2 as libc::c_int {
        return false_0;
    }
    s = Cmd_Argv(1 as libc::c_int);
    if *s.offset(0 as libc::c_int as isize) as libc::c_int >= '0' as i32
        && *s.offset(0 as libc::c_int as isize) as libc::c_int <= '9' as i32
    {
        idnum = atoi(Cmd_Argv(1 as libc::c_int));
        if idnum < 0 as libc::c_int || idnum as libc::c_float >= (*maxclients).value {
            Com_Printf(
                b"Bad client slot: %i\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                idnum,
            );
            return false_0;
        }
        sv_client = &mut *(svs.clients).offset(idnum as isize) as *mut client_t;
        sv_player = (*sv_client).edict;
        if (*sv_client).state as u64 == 0 {
            Com_Printf(
                b"Client %i is not active\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                idnum,
            );
            return false_0;
        }
        return true_0;
    }
    i = 0 as libc::c_int;
    cl = svs.clients;
    while (i as libc::c_float) < (*maxclients).value {
        if !((*cl).state as u64 == 0) {
            if strcmp(((*cl).name).as_mut_ptr(), s) == 0 {
                sv_client = cl;
                sv_player = (*sv_client).edict;
                return true_0;
            }
        }
        i += 1;
        cl = cl.offset(1);
    }
    Com_Printf(
        b"Userid %s is not on the server\n\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        s,
    );
    return false_0;
}
#[no_mangle]
pub unsafe extern "C" fn SV_WipeSavegame(mut savename: *mut libc::c_char) {
    let mut name: [libc::c_char; 128] = [0; 128];
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    Com_DPrintf(
        b"SV_WipeSaveGame(%s)\n\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        savename,
    );
    Com_sprintf(
        name.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong as libc::c_int,
        b"%s/save/%s/server.ssv\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        FS_Gamedir(),
        savename,
    );
    remove(name.as_mut_ptr());
    Com_sprintf(
        name.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong as libc::c_int,
        b"%s/save/%s/game.ssv\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        FS_Gamedir(),
        savename,
    );
    remove(name.as_mut_ptr());
    Com_sprintf(
        name.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong as libc::c_int,
        b"%s/save/%s/*.sav\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        FS_Gamedir(),
        savename,
    );
    s = Sys_FindFirst(
        name.as_mut_ptr(),
        0 as libc::c_int as libc::c_uint,
        0 as libc::c_int as libc::c_uint,
    );
    while !s.is_null() {
        remove(s);
        s = Sys_FindNext(
            0 as libc::c_int as libc::c_uint,
            0 as libc::c_int as libc::c_uint,
        );
    }
    Sys_FindClose();
    Com_sprintf(
        name.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong as libc::c_int,
        b"%s/save/%s/*.sv2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        FS_Gamedir(),
        savename,
    );
    s = Sys_FindFirst(
        name.as_mut_ptr(),
        0 as libc::c_int as libc::c_uint,
        0 as libc::c_int as libc::c_uint,
    );
    while !s.is_null() {
        remove(s);
        s = Sys_FindNext(
            0 as libc::c_int as libc::c_uint,
            0 as libc::c_int as libc::c_uint,
        );
    }
    Sys_FindClose();
}
#[no_mangle]
pub unsafe extern "C" fn CopyFile(
    mut src: *mut libc::c_char,
    mut dst: *mut libc::c_char,
) {
    let mut f1: *mut FILE = 0 as *mut FILE;
    let mut f2: *mut FILE = 0 as *mut FILE;
    let mut l: libc::c_int = 0;
    let mut buffer: [byte; 65536] = [0; 65536];
    Com_DPrintf(
        b"CopyFile (%s, %s)\n\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        src,
        dst,
    );
    f1 = fopen(src, b"rb\0" as *const u8 as *const libc::c_char);
    if f1.is_null() {
        return;
    }
    f2 = fopen(dst, b"wb\0" as *const u8 as *const libc::c_char);
    if f2.is_null() {
        fclose(f1);
        return;
    }
    loop {
        l = fread(
            buffer.as_mut_ptr() as *mut libc::c_void,
            1 as libc::c_int as libc::c_ulong,
            ::std::mem::size_of::<[byte; 65536]>() as libc::c_ulong,
            f1,
        ) as libc::c_int;
        if l == 0 {
            break;
        }
        fwrite(
            buffer.as_mut_ptr() as *const libc::c_void,
            1 as libc::c_int as libc::c_ulong,
            l as libc::c_ulong,
            f2,
        );
    }
    fclose(f1);
    fclose(f2);
}
#[no_mangle]
pub unsafe extern "C" fn SV_CopySaveGame(
    mut src: *mut libc::c_char,
    mut dst: *mut libc::c_char,
) {
    let mut name: [libc::c_char; 128] = [0; 128];
    let mut name2: [libc::c_char; 128] = [0; 128];
    let mut l: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut found: *mut libc::c_char = 0 as *mut libc::c_char;
    Com_DPrintf(
        b"SV_CopySaveGame(%s, %s)\n\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        src,
        dst,
    );
    SV_WipeSavegame(dst);
    Com_sprintf(
        name.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong as libc::c_int,
        b"%s/save/%s/server.ssv\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        FS_Gamedir(),
        src,
    );
    Com_sprintf(
        name2.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong as libc::c_int,
        b"%s/save/%s/server.ssv\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        FS_Gamedir(),
        dst,
    );
    FS_CreatePath(name2.as_mut_ptr());
    CopyFile(name.as_mut_ptr(), name2.as_mut_ptr());
    Com_sprintf(
        name.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong as libc::c_int,
        b"%s/save/%s/game.ssv\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        FS_Gamedir(),
        src,
    );
    Com_sprintf(
        name2.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong as libc::c_int,
        b"%s/save/%s/game.ssv\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        FS_Gamedir(),
        dst,
    );
    CopyFile(name.as_mut_ptr(), name2.as_mut_ptr());
    Com_sprintf(
        name.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong as libc::c_int,
        b"%s/save/%s/\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        FS_Gamedir(),
        src,
    );
    len = strlen(name.as_mut_ptr()) as libc::c_int;
    Com_sprintf(
        name.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong as libc::c_int,
        b"%s/save/%s/*.sav\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        FS_Gamedir(),
        src,
    );
    found = Sys_FindFirst(
        name.as_mut_ptr(),
        0 as libc::c_int as libc::c_uint,
        0 as libc::c_int as libc::c_uint,
    );
    while !found.is_null() {
        strcpy(name.as_mut_ptr().offset(len as isize), found.offset(len as isize));
        Com_sprintf(
            name2.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong as libc::c_int,
            b"%s/save/%s/%s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            FS_Gamedir(),
            dst,
            found.offset(len as isize),
        );
        CopyFile(name.as_mut_ptr(), name2.as_mut_ptr());
        l = strlen(name.as_mut_ptr()) as libc::c_int;
        strcpy(
            name.as_mut_ptr().offset(l as isize).offset(-(3 as libc::c_int as isize)),
            b"sv2\0" as *const u8 as *const libc::c_char,
        );
        l = strlen(name2.as_mut_ptr()) as libc::c_int;
        strcpy(
            name2.as_mut_ptr().offset(l as isize).offset(-(3 as libc::c_int as isize)),
            b"sv2\0" as *const u8 as *const libc::c_char,
        );
        CopyFile(name.as_mut_ptr(), name2.as_mut_ptr());
        found = Sys_FindNext(
            0 as libc::c_int as libc::c_uint,
            0 as libc::c_int as libc::c_uint,
        );
    }
    Sys_FindClose();
}
#[no_mangle]
pub unsafe extern "C" fn SV_WriteLevelFile() {
    let mut name: [libc::c_char; 128] = [0; 128];
    let mut f: *mut FILE = 0 as *mut FILE;
    Com_DPrintf(
        b"SV_WriteLevelFile()\n\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    Com_sprintf(
        name.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong as libc::c_int,
        b"%s/save/current/%s.sv2\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        FS_Gamedir(),
        (sv.name).as_mut_ptr(),
    );
    f = fopen(name.as_mut_ptr(), b"wb\0" as *const u8 as *const libc::c_char);
    if f.is_null() {
        Com_Printf(
            b"Failed to open %s\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            name.as_mut_ptr(),
        );
        return;
    }
    fwrite(
        (sv.configstrings).as_mut_ptr() as *const libc::c_void,
        ::std::mem::size_of::<[[libc::c_char; 64]; 2080]>() as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        f,
    );
    CM_WritePortalState(f);
    fclose(f);
    Com_sprintf(
        name.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong as libc::c_int,
        b"%s/save/current/%s.sav\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        FS_Gamedir(),
        (sv.name).as_mut_ptr(),
    );
    ((*ge).WriteLevel).expect("non-null function pointer")(name.as_mut_ptr());
}
#[no_mangle]
pub unsafe extern "C" fn SV_ReadLevelFile() {
    let mut name: [libc::c_char; 128] = [0; 128];
    let mut f: *mut FILE = 0 as *mut FILE;
    Com_DPrintf(
        b"SV_ReadLevelFile()\n\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    Com_sprintf(
        name.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong as libc::c_int,
        b"%s/save/current/%s.sv2\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        FS_Gamedir(),
        (sv.name).as_mut_ptr(),
    );
    f = fopen(name.as_mut_ptr(), b"rb\0" as *const u8 as *const libc::c_char);
    if f.is_null() {
        Com_Printf(
            b"Failed to open %s\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            name.as_mut_ptr(),
        );
        return;
    }
    FS_Read(
        (sv.configstrings).as_mut_ptr() as *mut libc::c_void,
        ::std::mem::size_of::<[[libc::c_char; 64]; 2080]>() as libc::c_ulong
            as libc::c_int,
        f,
    );
    CM_ReadPortalState(f);
    fclose(f);
    Com_sprintf(
        name.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong as libc::c_int,
        b"%s/save/current/%s.sav\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        FS_Gamedir(),
        (sv.name).as_mut_ptr(),
    );
    ((*ge).ReadLevel).expect("non-null function pointer")(name.as_mut_ptr());
}
#[no_mangle]
pub unsafe extern "C" fn SV_WriteServerFile(mut autosave: qboolean) {
    let mut f: *mut FILE = 0 as *mut FILE;
    let mut var: *mut cvar_t = 0 as *mut cvar_t;
    let mut name: [libc::c_char; 128] = [0; 128];
    let mut string: [libc::c_char; 128] = [0; 128];
    let mut comment: [libc::c_char; 32] = [0; 32];
    let mut aclock: time_t = 0;
    let mut newtime: *mut tm = 0 as *mut tm;
    Com_DPrintf(
        b"SV_WriteServerFile(%s)\n\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        if autosave as libc::c_uint != 0 {
            b"true\0" as *const u8 as *const libc::c_char
        } else {
            b"false\0" as *const u8 as *const libc::c_char
        },
    );
    Com_sprintf(
        name.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong as libc::c_int,
        b"%s/save/current/server.ssv\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        FS_Gamedir(),
    );
    f = fopen(name.as_mut_ptr(), b"wb\0" as *const u8 as *const libc::c_char);
    if f.is_null() {
        Com_Printf(
            b"Couldn't write %s\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            name.as_mut_ptr(),
        );
        return;
    }
    memset(
        comment.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong,
    );
    if autosave as u64 == 0 {
        time(&mut aclock);
        newtime = localtime(&mut aclock);
        Com_sprintf(
            comment.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong as libc::c_int,
            b"%2i:%i%i %2i/%2i  \0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            (*newtime).tm_hour,
            (*newtime).tm_min / 10 as libc::c_int,
            (*newtime).tm_min % 10 as libc::c_int,
            (*newtime).tm_mon + 1 as libc::c_int,
            (*newtime).tm_mday,
        );
        strncat(
            comment.as_mut_ptr(),
            (sv.configstrings[0 as libc::c_int as usize]).as_mut_ptr(),
            (::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                .wrapping_sub(strlen(comment.as_mut_ptr())),
        );
    } else {
        Com_sprintf(
            comment.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong as libc::c_int,
            b"ENTERING %s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            (sv.configstrings[0 as libc::c_int as usize]).as_mut_ptr(),
        );
    }
    fwrite(
        comment.as_mut_ptr() as *const libc::c_void,
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong,
        f,
    );
    fwrite(
        (svs.mapcmd).as_mut_ptr() as *const libc::c_void,
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
        f,
    );
    var = cvar_vars;
    while !var.is_null() {
        if !((*var).flags & 16 as libc::c_int == 0) {
            if strlen((*var).name)
                >= (::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                || strlen((*var).string)
                    >= (::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            {
                Com_Printf(
                    b"Cvar too long: %s = %s\n\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    (*var).name,
                    (*var).string,
                );
            } else {
                memset(
                    name.as_mut_ptr() as *mut libc::c_void,
                    0 as libc::c_int,
                    ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
                );
                memset(
                    string.as_mut_ptr() as *mut libc::c_void,
                    0 as libc::c_int,
                    ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
                );
                strcpy(name.as_mut_ptr(), (*var).name);
                strcpy(string.as_mut_ptr(), (*var).string);
                fwrite(
                    name.as_mut_ptr() as *const libc::c_void,
                    1 as libc::c_int as libc::c_ulong,
                    ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
                    f,
                );
                fwrite(
                    string.as_mut_ptr() as *const libc::c_void,
                    1 as libc::c_int as libc::c_ulong,
                    ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
                    f,
                );
            }
        }
        var = (*var).next;
    }
    fclose(f);
    Com_sprintf(
        name.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong as libc::c_int,
        b"%s/save/current/game.ssv\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        FS_Gamedir(),
    );
    ((*ge).WriteGame).expect("non-null function pointer")(name.as_mut_ptr(), autosave);
}
#[no_mangle]
pub unsafe extern "C" fn SV_ReadServerFile() {
    let mut f: *mut FILE = 0 as *mut FILE;
    let mut name: [libc::c_char; 128] = [0; 128];
    let mut string: [libc::c_char; 128] = [0; 128];
    let mut comment: [libc::c_char; 32] = [0; 32];
    let mut mapcmd: [libc::c_char; 128] = [0; 128];
    Com_DPrintf(
        b"SV_ReadServerFile()\n\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    Com_sprintf(
        name.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong as libc::c_int,
        b"%s/save/current/server.ssv\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        FS_Gamedir(),
    );
    f = fopen(name.as_mut_ptr(), b"rb\0" as *const u8 as *const libc::c_char);
    if f.is_null() {
        Com_Printf(
            b"Couldn't read %s\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            name.as_mut_ptr(),
        );
        return;
    }
    FS_Read(
        comment.as_mut_ptr() as *mut libc::c_void,
        ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong as libc::c_int,
        f,
    );
    FS_Read(
        mapcmd.as_mut_ptr() as *mut libc::c_void,
        ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong as libc::c_int,
        f,
    );
    while !(fread(
        name.as_mut_ptr() as *mut libc::c_void,
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
        f,
    ) == 0)
    {
        FS_Read(
            string.as_mut_ptr() as *mut libc::c_void,
            ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong as libc::c_int,
            f,
        );
        Com_DPrintf(
            b"Set %s = %s\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            name.as_mut_ptr(),
            string.as_mut_ptr(),
        );
        Cvar_ForceSet(name.as_mut_ptr(), string.as_mut_ptr());
    }
    fclose(f);
    SV_InitGame();
    strcpy((svs.mapcmd).as_mut_ptr(), mapcmd.as_mut_ptr());
    Com_sprintf(
        name.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong as libc::c_int,
        b"%s/save/current/game.ssv\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        FS_Gamedir(),
    );
    ((*ge).ReadGame).expect("non-null function pointer")(name.as_mut_ptr());
}
#[no_mangle]
pub unsafe extern "C" fn SV_DemoMap_f() {
    SV_Map(true_0, Cmd_Argv(1 as libc::c_int), false_0);
}
#[no_mangle]
pub unsafe extern "C" fn SV_GameMap_f() {
    let mut map: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    let mut cl: *mut client_t = 0 as *mut client_t;
    let mut savedInuse: *mut qboolean = 0 as *mut qboolean;
    if Cmd_Argc() != 2 as libc::c_int {
        Com_Printf(
            b"USAGE: gamemap <map>\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return;
    }
    Com_DPrintf(
        b"SV_GameMap(%s)\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        Cmd_Argv(1 as libc::c_int),
    );
    FS_CreatePath(
        va(
            b"%s/save/current/\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            FS_Gamedir(),
        ),
    );
    map = Cmd_Argv(1 as libc::c_int);
    if *map.offset(0 as libc::c_int as isize) as libc::c_int == '*' as i32 {
        SV_WipeSavegame(
            b"current\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    } else if sv.state as libc::c_uint == ss_game as libc::c_int as libc::c_uint {
        savedInuse = malloc(
            ((*maxclients).value
                * ::std::mem::size_of::<qboolean>() as libc::c_ulong as libc::c_float)
                as libc::c_ulong,
        ) as *mut qboolean;
        i = 0 as libc::c_int;
        cl = svs.clients;
        while (i as libc::c_float) < (*maxclients).value {
            *savedInuse.offset(i as isize) = (*(*cl).edict).inuse;
            (*(*cl).edict).inuse = false_0;
            i += 1;
            cl = cl.offset(1);
        }
        SV_WriteLevelFile();
        i = 0 as libc::c_int;
        cl = svs.clients;
        while (i as libc::c_float) < (*maxclients).value {
            (*(*cl).edict).inuse = *savedInuse.offset(i as isize);
            i += 1;
            cl = cl.offset(1);
        }
        free(savedInuse as *mut libc::c_void);
    }
    SV_Map(false_0, Cmd_Argv(1 as libc::c_int), false_0);
    strncpy(
        (svs.mapcmd).as_mut_ptr(),
        Cmd_Argv(1 as libc::c_int),
        (::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
    );
    if (*dedicated).value == 0. {
        SV_WriteServerFile(true_0);
        SV_CopySaveGame(
            b"current\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"save0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn SV_Map_f() {
    let mut map: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut expanded: [libc::c_char; 64] = [0; 64];
    map = Cmd_Argv(1 as libc::c_int);
    if (strstr(map, b".\0" as *const u8 as *const libc::c_char)).is_null() {
        Com_sprintf(
            expanded.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
            b"maps/%s.bsp\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            map,
        );
        if FS_LoadFile(expanded.as_mut_ptr(), 0 as *mut *mut libc::c_void)
            == -(1 as libc::c_int)
        {
            Com_Printf(
                b"Can't find %s\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                expanded.as_mut_ptr(),
            );
            return;
        }
    }
    sv.state = ss_dead;
    SV_WipeSavegame(
        b"current\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    SV_GameMap_f();
}
#[no_mangle]
pub unsafe extern "C" fn SV_Loadgame_f() {
    let mut name: [libc::c_char; 128] = [0; 128];
    let mut f: *mut FILE = 0 as *mut FILE;
    let mut dir: *mut libc::c_char = 0 as *mut libc::c_char;
    if Cmd_Argc() != 2 as libc::c_int {
        Com_Printf(
            b"USAGE: loadgame <directory>\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return;
    }
    Com_Printf(
        b"Loading game...\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dir = Cmd_Argv(1 as libc::c_int);
    if !(strstr(dir, b"..\0" as *const u8 as *const libc::c_char)).is_null()
        || !(strstr(dir, b"/\0" as *const u8 as *const libc::c_char)).is_null()
        || !(strstr(dir, b"\\\0" as *const u8 as *const libc::c_char)).is_null()
    {
        Com_Printf(
            b"Bad savedir.\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
    Com_sprintf(
        name.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong as libc::c_int,
        b"%s/save/%s/server.ssv\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        FS_Gamedir(),
        Cmd_Argv(1 as libc::c_int),
    );
    f = fopen(name.as_mut_ptr(), b"rb\0" as *const u8 as *const libc::c_char);
    if f.is_null() {
        Com_Printf(
            b"No such savegame: %s\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            name.as_mut_ptr(),
        );
        return;
    }
    fclose(f);
    SV_CopySaveGame(
        Cmd_Argv(1 as libc::c_int),
        b"current\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    SV_ReadServerFile();
    sv.state = ss_dead;
    SV_Map(false_0, (svs.mapcmd).as_mut_ptr(), true_0);
}
#[no_mangle]
pub unsafe extern "C" fn SV_Savegame_f() {
    let mut dir: *mut libc::c_char = 0 as *mut libc::c_char;
    if sv.state as libc::c_uint != ss_game as libc::c_int as libc::c_uint {
        Com_Printf(
            b"You must be in a game to save.\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return;
    }
    if Cmd_Argc() != 2 as libc::c_int {
        Com_Printf(
            b"USAGE: savegame <directory>\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return;
    }
    if Cvar_VariableValue(
        b"deathmatch\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) != 0.
    {
        Com_Printf(
            b"Can't savegame in a deathmatch\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return;
    }
    if strcmp(
        Cmd_Argv(1 as libc::c_int),
        b"current\0" as *const u8 as *const libc::c_char,
    ) == 0
    {
        Com_Printf(
            b"Can't save to 'current'\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return;
    }
    if (*maxclients).value == 1 as libc::c_int as libc::c_float
        && (*(*(*(svs.clients).offset(0 as libc::c_int as isize)).edict).client)
            .ps
            .stats[1 as libc::c_int as usize] as libc::c_int <= 0 as libc::c_int
    {
        Com_Printf(
            b"\nCan't savegame while dead!\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return;
    }
    dir = Cmd_Argv(1 as libc::c_int);
    if !(strstr(dir, b"..\0" as *const u8 as *const libc::c_char)).is_null()
        || !(strstr(dir, b"/\0" as *const u8 as *const libc::c_char)).is_null()
        || !(strstr(dir, b"\\\0" as *const u8 as *const libc::c_char)).is_null()
    {
        Com_Printf(
            b"Bad savedir.\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
    Com_Printf(
        b"Saving game...\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    SV_WriteLevelFile();
    SV_WriteServerFile(false_0);
    SV_CopySaveGame(
        b"current\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        dir,
    );
    Com_Printf(b"Done.\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn SV_Kick_f() {
    if svs.initialized as u64 == 0 {
        Com_Printf(
            b"No server running.\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return;
    }
    if Cmd_Argc() != 2 as libc::c_int {
        Com_Printf(
            b"Usage: kick <userid>\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return;
    }
    if SV_SetPlayer() as u64 == 0 {
        return;
    }
    SV_BroadcastPrintf(
        2 as libc::c_int,
        b"%s was kicked\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ((*sv_client).name).as_mut_ptr(),
    );
    SV_ClientPrintf(
        sv_client,
        2 as libc::c_int,
        b"You were kicked from the game\n\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    SV_DropClient(sv_client);
    (*sv_client).lastmessage = svs.realtime;
}
#[no_mangle]
pub unsafe extern "C" fn SV_Status_f() {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut cl: *mut client_t = 0 as *mut client_t;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ping: libc::c_int = 0;
    if (svs.clients).is_null() {
        Com_Printf(
            b"No server running.\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return;
    }
    Com_Printf(
        b"map              : %s\n\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        (sv.name).as_mut_ptr(),
    );
    Com_Printf(
        b"num score ping name            lastmsg address               qport \n\0"
            as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    Com_Printf(
        b"--- ----- ---- --------------- ------- --------------------- ------\n\0"
            as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    i = 0 as libc::c_int;
    cl = svs.clients;
    while (i as libc::c_float) < (*maxclients).value {
        if !((*cl).state as u64 == 0) {
            Com_Printf(
                b"%3i \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                i,
            );
            Com_Printf(
                b"%5i \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                (*(*(*cl).edict).client).ps.stats[14 as libc::c_int as usize]
                    as libc::c_int,
            );
            if (*cl).state as libc::c_uint == cs_connected as libc::c_int as libc::c_uint
            {
                Com_Printf(
                    b"CNCT \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                );
            } else if (*cl).state as libc::c_uint
                == cs_zombie as libc::c_int as libc::c_uint
            {
                Com_Printf(
                    b"ZMBI \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                );
            } else {
                ping = if (*cl).ping < 9999 as libc::c_int {
                    (*cl).ping
                } else {
                    9999 as libc::c_int
                };
                Com_Printf(
                    b"%4i \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    ping,
                );
            }
            Com_Printf(
                b"%s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                ((*cl).name).as_mut_ptr(),
            );
            l = (16 as libc::c_int as libc::c_ulong)
                .wrapping_sub(strlen(((*cl).name).as_mut_ptr())) as libc::c_int;
            j = 0 as libc::c_int;
            while j < l {
                Com_Printf(
                    b" \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                );
                j += 1;
            }
            Com_Printf(
                b"%7i \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                svs.realtime - (*cl).lastmessage,
            );
            s = NET_AdrToString((*cl).netchan.remote_address);
            Com_Printf(
                b"%s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                s,
            );
            l = (22 as libc::c_int as libc::c_ulong).wrapping_sub(strlen(s))
                as libc::c_int;
            j = 0 as libc::c_int;
            while j < l {
                Com_Printf(
                    b" \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                );
                j += 1;
            }
            Com_Printf(
                b"%5i\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                (*cl).netchan.qport,
            );
            Com_Printf(b"\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
        }
        i += 1;
        cl = cl.offset(1);
    }
    Com_Printf(b"\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn SV_ConSay_f() {
    let mut client: *mut client_t = 0 as *mut client_t;
    let mut j: libc::c_int = 0;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut text: [libc::c_char; 1024] = [0; 1024];
    if Cmd_Argc() < 2 as libc::c_int {
        return;
    }
    strcpy(text.as_mut_ptr(), b"console: \0" as *const u8 as *const libc::c_char);
    p = Cmd_Args();
    if *p as libc::c_int == '"' as i32 {
        p = p.offset(1);
        *p
            .offset(
                (strlen(p)).wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
            ) = 0 as libc::c_int as libc::c_char;
    }
    strcat(text.as_mut_ptr(), p);
    j = 0 as libc::c_int;
    client = svs.clients;
    while (j as libc::c_float) < (*maxclients).value {
        if !((*client).state as libc::c_uint
            != cs_spawned as libc::c_int as libc::c_uint)
        {
            SV_ClientPrintf(
                client,
                3 as libc::c_int,
                b"%s\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                text.as_mut_ptr(),
            );
        }
        j += 1;
        client = client.offset(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn SV_Heartbeat_f() {
    svs.last_heartbeat = -(9999999 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn SV_Serverinfo_f() {
    Com_Printf(
        b"Server info settings:\n\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    Info_Print(Cvar_Serverinfo());
}
#[no_mangle]
pub unsafe extern "C" fn SV_DumpUser_f() {
    if Cmd_Argc() != 2 as libc::c_int {
        Com_Printf(
            b"Usage: info <userid>\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return;
    }
    if SV_SetPlayer() as u64 == 0 {
        return;
    }
    Com_Printf(b"userinfo\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    Com_Printf(b"--------\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    Info_Print(((*sv_client).userinfo).as_mut_ptr());
}
#[no_mangle]
pub unsafe extern "C" fn SV_ServerRecord_f() {
    let mut name: [libc::c_char; 128] = [0; 128];
    let mut buf_data: [libc::c_char; 32768] = [0; 32768];
    let mut buf: sizebuf_t = sizebuf_t {
        allowoverflow: false_0,
        overflowed: false_0,
        data: 0 as *const byte as *mut byte,
        maxsize: 0,
        cursize: 0,
        readcount: 0,
    };
    let mut len: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    if Cmd_Argc() != 2 as libc::c_int {
        Com_Printf(
            b"serverrecord <demoname>\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return;
    }
    if !(svs.demofile).is_null() {
        Com_Printf(
            b"Already recording.\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return;
    }
    if sv.state as libc::c_uint != ss_game as libc::c_int as libc::c_uint {
        Com_Printf(
            b"You must be in a level to record.\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return;
    }
    Com_sprintf(
        name.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong as libc::c_int,
        b"%s/demos/%s.dm2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        FS_Gamedir(),
        Cmd_Argv(1 as libc::c_int),
    );
    Com_Printf(
        b"recording to %s.\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        name.as_mut_ptr(),
    );
    FS_CreatePath(name.as_mut_ptr());
    svs.demofile = fopen(name.as_mut_ptr(), b"wb\0" as *const u8 as *const libc::c_char);
    if (svs.demofile).is_null() {
        Com_Printf(
            b"ERROR: couldn't open.\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return;
    }
    SZ_Init(
        &mut svs.demo_multicast,
        (svs.demo_multicast_buf).as_mut_ptr(),
        ::std::mem::size_of::<[byte; 1400]>() as libc::c_ulong as libc::c_int,
    );
    SZ_Init(
        &mut buf,
        buf_data.as_mut_ptr() as *mut byte,
        ::std::mem::size_of::<[libc::c_char; 32768]>() as libc::c_ulong as libc::c_int,
    );
    MSG_WriteByte(&mut buf, svc_serverdata as libc::c_int);
    MSG_WriteLong(&mut buf, 34 as libc::c_int);
    MSG_WriteLong(&mut buf, svs.spawncount);
    MSG_WriteByte(&mut buf, 2 as libc::c_int);
    MSG_WriteString(
        &mut buf,
        Cvar_VariableString(
            b"gamedir\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ),
    );
    MSG_WriteShort(&mut buf, -(1 as libc::c_int));
    MSG_WriteString(
        &mut buf,
        (sv.configstrings[0 as libc::c_int as usize]).as_mut_ptr(),
    );
    i = 0 as libc::c_int;
    while i
        < 32 as libc::c_int + 256 as libc::c_int + 256 as libc::c_int
            + 256 as libc::c_int + 256 as libc::c_int + 256 as libc::c_int
            + 256 as libc::c_int + 256 as libc::c_int * 2 as libc::c_int
    {
        if sv.configstrings[i as usize][0 as libc::c_int as usize] != 0 {
            MSG_WriteByte(&mut buf, svc_configstring as libc::c_int);
            MSG_WriteShort(&mut buf, i);
            MSG_WriteString(&mut buf, (sv.configstrings[i as usize]).as_mut_ptr());
        }
        i += 1;
    }
    Com_DPrintf(
        b"signon message length: %i\n\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        buf.cursize,
    );
    len = LittleLong(buf.cursize);
    fwrite(
        &mut len as *mut libc::c_int as *const libc::c_void,
        4 as libc::c_int as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        svs.demofile,
    );
    fwrite(
        buf.data as *const libc::c_void,
        buf.cursize as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        svs.demofile,
    );
}
#[no_mangle]
pub unsafe extern "C" fn SV_ServerStop_f() {
    if (svs.demofile).is_null() {
        Com_Printf(
            b"Not doing a serverrecord.\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return;
    }
    fclose(svs.demofile);
    svs.demofile = 0 as *mut FILE;
    Com_Printf(
        b"Recording completed.\n\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
}
#[no_mangle]
pub unsafe extern "C" fn SV_KillServer_f() {
    if svs.initialized as u64 == 0 {
        return;
    }
    SV_Shutdown(
        b"Server was killed.\n\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        false_0,
    );
    NET_Config(false_0);
}
#[no_mangle]
pub unsafe extern "C" fn SV_ServerCommand_f() {
    if ge.is_null() {
        Com_Printf(
            b"No game loaded.\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return;
    }
    ((*ge).ServerCommand).expect("non-null function pointer")();
}
#[no_mangle]
pub unsafe extern "C" fn SV_InitOperatorCommands() {
    Cmd_AddCommand(
        b"heartbeat\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        Some(SV_Heartbeat_f as unsafe extern "C" fn() -> ()),
    );
    Cmd_AddCommand(
        b"kick\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        Some(SV_Kick_f as unsafe extern "C" fn() -> ()),
    );
    Cmd_AddCommand(
        b"status\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        Some(SV_Status_f as unsafe extern "C" fn() -> ()),
    );
    Cmd_AddCommand(
        b"serverinfo\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        Some(SV_Serverinfo_f as unsafe extern "C" fn() -> ()),
    );
    Cmd_AddCommand(
        b"dumpuser\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        Some(SV_DumpUser_f as unsafe extern "C" fn() -> ()),
    );
    Cmd_AddCommand(
        b"map\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        Some(SV_Map_f as unsafe extern "C" fn() -> ()),
    );
    Cmd_AddCommand(
        b"demomap\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        Some(SV_DemoMap_f as unsafe extern "C" fn() -> ()),
    );
    Cmd_AddCommand(
        b"gamemap\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        Some(SV_GameMap_f as unsafe extern "C" fn() -> ()),
    );
    Cmd_AddCommand(
        b"setmaster\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        Some(SV_SetMaster_f as unsafe extern "C" fn() -> ()),
    );
    if (*dedicated).value != 0. {
        Cmd_AddCommand(
            b"say\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            Some(SV_ConSay_f as unsafe extern "C" fn() -> ()),
        );
    }
    Cmd_AddCommand(
        b"serverrecord\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        Some(SV_ServerRecord_f as unsafe extern "C" fn() -> ()),
    );
    Cmd_AddCommand(
        b"serverstop\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        Some(SV_ServerStop_f as unsafe extern "C" fn() -> ()),
    );
    Cmd_AddCommand(
        b"save\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        Some(SV_Savegame_f as unsafe extern "C" fn() -> ()),
    );
    Cmd_AddCommand(
        b"load\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        Some(SV_Loadgame_f as unsafe extern "C" fn() -> ()),
    );
    Cmd_AddCommand(
        b"killserver\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        Some(SV_KillServer_f as unsafe extern "C" fn() -> ()),
    );
    Cmd_AddCommand(
        b"sv\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        Some(SV_ServerCommand_f as unsafe extern "C" fn() -> ()),
    );
}
