#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(extern_types, register_tool)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn SV_SendClientMessages();
    fn SV_ClearWorld();
    fn SV_ReadLevelFile();
    fn SV_BroadcastCommand(fmt: *mut libc::c_char, _: ...);
    fn SV_InitGameProgs();
    static mut ge: *mut game_export_t;
    fn SV_Multicast(origin: *mut vec_t, to: multicast_t);
    static mut sv_airaccelerate: *mut cvar_t;
    static mut sv_noreload: *mut cvar_t;
    static mut maxclients: *mut cvar_t;
    static mut master_adr: [netadr_t; 8];
    fn SV_Shutdown(finalmsg: *mut libc::c_char, reconnect: qboolean);
    fn SCR_BeginLoadingPlaque();
    fn CL_Drop();
    fn Z_Malloc(size: libc::c_int) -> *mut libc::c_void;
    static mut dedicated: *mut cvar_t;
    fn Com_SetServerState(state: libc::c_int);
    fn Com_Error(code: libc::c_int, fmt: *mut libc::c_char, _: ...);
    fn Com_DPrintf(fmt: *mut libc::c_char, _: ...);
    fn FS_Gamedir() -> *mut libc::c_char;
    static mut pm_airaccelerate: libc::c_float;
    fn CM_EntityString() -> *mut libc::c_char;
    fn CM_NumInlineModels() -> libc::c_int;
    fn CM_InlineModel(name: *mut libc::c_char) -> *mut cmodel_t;
    fn CM_LoadMap(
        name: *mut libc::c_char,
        clientload: qboolean,
        checksum: *mut libc::c_uint,
    ) -> *mut cmodel_t;
    fn NET_StringToAdr(s: *mut libc::c_char, a: *mut netadr_t) -> qboolean;
    fn NET_Config(multiplayer: qboolean);
    fn Cvar_GetLatchedVars();
    fn Cvar_VariableValue(var_name: *mut libc::c_char) -> libc::c_float;
    fn Cvar_FullSet(
        var_name: *mut libc::c_char,
        value: *mut libc::c_char,
        flags: libc::c_int,
    ) -> *mut cvar_t;
    fn Cvar_Set(var_name: *mut libc::c_char, value: *mut libc::c_char) -> *mut cvar_t;
    fn Cbuf_CopyToDefer();
    fn MSG_WriteString(sb: *mut sizebuf_t, s: *mut libc::c_char);
    fn MSG_WriteShort(sb: *mut sizebuf_t, c: libc::c_int);
    fn MSG_WriteChar(sb: *mut sizebuf_t, c: libc::c_int);
    fn SZ_Clear(buf: *mut sizebuf_t);
    fn SZ_Init(buf: *mut sizebuf_t, data: *mut byte, length: libc::c_int);
    fn Com_Printf(msg: *mut libc::c_char, _: ...);
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
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn rand() -> libc::c_int;
    static mut vec3_origin: vec3_t;
    fn Com_sprintf(
        dest: *mut libc::c_char,
        size: libc::c_int,
        fmt: *mut libc::c_char,
        _: ...
    );
    fn Q_stricmp(s1: *mut libc::c_char, s2: *mut libc::c_char) -> libc::c_int;
    fn va(format: *mut libc::c_char, _: ...) -> *mut libc::c_char;
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
pub type multicast_t = libc::c_uint;
pub const MULTICAST_PVS_R: multicast_t = 5;
pub const MULTICAST_PHS_R: multicast_t = 4;
pub const MULTICAST_ALL_R: multicast_t = 3;
pub const MULTICAST_PVS: multicast_t = 2;
pub const MULTICAST_PHS: multicast_t = 1;
pub const MULTICAST_ALL: multicast_t = 0;
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
pub static mut svs: server_static_t = server_static_t {
    initialized: false_0,
    realtime: 0,
    mapcmd: [0; 128],
    spawncount: 0,
    clients: 0 as *const client_t as *mut client_t,
    num_client_entities: 0,
    next_client_entities: 0,
    client_entities: 0 as *const entity_state_t as *mut entity_state_t,
    last_heartbeat: 0,
    challenges: [challenge_t {
        adr: netadr_t {
            type_0: NA_LOOPBACK,
            ip: [0; 4],
            ipx: [0; 10],
            port: 0,
        },
        challenge: 0,
        time: 0,
    }; 1024],
    demofile: 0 as *const FILE as *mut FILE,
    demo_multicast: sizebuf_t {
        allowoverflow: false_0,
        overflowed: false_0,
        data: 0 as *const byte as *mut byte,
        maxsize: 0,
        cursize: 0,
        readcount: 0,
    },
    demo_multicast_buf: [0; 1400],
};
#[no_mangle]
pub static mut sv: server_t = server_t {
    state: ss_dead,
    attractloop: false_0,
    loadgame: false_0,
    time: 0,
    framenum: 0,
    name: [0; 64],
    models: [0 as *const cmodel_s as *mut cmodel_s; 256],
    configstrings: [[0; 64]; 2080],
    baselines: [entity_state_t {
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
    }; 1024],
    multicast: sizebuf_t {
        allowoverflow: false_0,
        overflowed: false_0,
        data: 0 as *const byte as *mut byte,
        maxsize: 0,
        cursize: 0,
        readcount: 0,
    },
    multicast_buf: [0; 1400],
    demofile: 0 as *const FILE as *mut FILE,
    timedemo: false_0,
};
#[no_mangle]
pub unsafe extern "C" fn SV_FindIndex(
    mut name: *mut libc::c_char,
    mut start: libc::c_int,
    mut max: libc::c_int,
    mut create: qboolean,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    if name.is_null() || *name.offset(0 as libc::c_int as isize) == 0 {
        return 0 as libc::c_int;
    }
    i = 1 as libc::c_int;
    while i < max
        && sv.configstrings[(start + i) as usize][0 as libc::c_int as usize]
            as libc::c_int != 0
    {
        if strcmp((sv.configstrings[(start + i) as usize]).as_mut_ptr(), name) == 0 {
            return i;
        }
        i += 1;
    }
    if create as u64 == 0 {
        return 0 as libc::c_int;
    }
    if i == max {
        Com_Error(
            1 as libc::c_int,
            b"*Index: overflow\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    strncpy(
        (sv.configstrings[(start + i) as usize]).as_mut_ptr(),
        name,
        ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong,
    );
    if sv.state as libc::c_uint != ss_loading as libc::c_int as libc::c_uint {
        SZ_Clear(&mut sv.multicast);
        MSG_WriteChar(&mut sv.multicast, svc_configstring as libc::c_int);
        MSG_WriteShort(&mut sv.multicast, start + i);
        MSG_WriteString(&mut sv.multicast, name);
        SV_Multicast(vec3_origin.as_mut_ptr(), MULTICAST_ALL_R);
    }
    return i;
}
#[no_mangle]
pub unsafe extern "C" fn SV_ModelIndex(mut name: *mut libc::c_char) -> libc::c_int {
    return SV_FindIndex(name, 32 as libc::c_int, 256 as libc::c_int, true_0);
}
#[no_mangle]
pub unsafe extern "C" fn SV_SoundIndex(mut name: *mut libc::c_char) -> libc::c_int {
    return SV_FindIndex(
        name,
        32 as libc::c_int + 256 as libc::c_int,
        256 as libc::c_int,
        true_0,
    );
}
#[no_mangle]
pub unsafe extern "C" fn SV_ImageIndex(mut name: *mut libc::c_char) -> libc::c_int {
    return SV_FindIndex(
        name,
        32 as libc::c_int + 256 as libc::c_int + 256 as libc::c_int,
        256 as libc::c_int,
        true_0,
    );
}
#[no_mangle]
pub unsafe extern "C" fn SV_CreateBaseline() {
    let mut svent: *mut edict_t = 0 as *mut edict_t;
    let mut entnum: libc::c_int = 0;
    entnum = 1 as libc::c_int;
    while entnum < (*ge).num_edicts {
        svent = ((*ge).edicts as *mut byte).offset(((*ge).edict_size * entnum) as isize)
            as *mut edict_t;
        if !((*svent).inuse as u64 == 0) {
            if !((*svent).s.modelindex == 0 && (*svent).s.sound == 0
                && (*svent).s.effects == 0)
            {
                (*svent).s.number = entnum;
                (*svent)
                    .s
                    .old_origin[0 as libc::c_int
                    as usize] = (*svent).s.origin[0 as libc::c_int as usize];
                (*svent)
                    .s
                    .old_origin[1 as libc::c_int
                    as usize] = (*svent).s.origin[1 as libc::c_int as usize];
                (*svent)
                    .s
                    .old_origin[2 as libc::c_int
                    as usize] = (*svent).s.origin[2 as libc::c_int as usize];
                sv.baselines[entnum as usize] = (*svent).s;
            }
        }
        entnum += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn SV_CheckForSavegame() {
    let mut name: [libc::c_char; 128] = [0; 128];
    let mut f: *mut FILE = 0 as *mut FILE;
    let mut i: libc::c_int = 0;
    if (*sv_noreload).value != 0. {
        return;
    }
    if Cvar_VariableValue(
        b"deathmatch\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) != 0.
    {
        return;
    }
    Com_sprintf(
        name.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong as libc::c_int,
        b"%s/save/current/%s.sav\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        FS_Gamedir(),
        (sv.name).as_mut_ptr(),
    );
    f = fopen(name.as_mut_ptr(), b"rb\0" as *const u8 as *const libc::c_char);
    if f.is_null() {
        return;
    }
    fclose(f);
    SV_ClearWorld();
    SV_ReadLevelFile();
    if sv.loadgame as u64 == 0 {
        let mut previousState: server_state_t = ss_dead;
        previousState = sv.state;
        sv.state = ss_loading;
        i = 0 as libc::c_int;
        while i < 100 as libc::c_int {
            ((*ge).RunFrame).expect("non-null function pointer")();
            i += 1;
        }
        sv.state = previousState;
    }
}
#[no_mangle]
pub unsafe extern "C" fn SV_SpawnServer(
    mut server: *mut libc::c_char,
    mut spawnpoint: *mut libc::c_char,
    mut serverstate: server_state_t,
    mut attractloop: qboolean,
    mut loadgame: qboolean,
) {
    let mut i: libc::c_int = 0;
    let mut checksum: libc::c_uint = 0;
    if attractloop as u64 != 0 {
        Cvar_Set(
            b"paused\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
    Com_Printf(
        b"------- Server Initialization -------\n\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    Com_DPrintf(
        b"SpawnServer: %s\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        server,
    );
    if !(sv.demofile).is_null() {
        fclose(sv.demofile);
    }
    svs.spawncount += 1;
    sv.state = ss_dead;
    Com_SetServerState(sv.state as libc::c_int);
    memset(
        &mut sv as *mut server_t as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<server_t>() as libc::c_ulong,
    );
    svs.realtime = 0 as libc::c_int;
    sv.loadgame = loadgame;
    sv.attractloop = attractloop;
    strcpy((sv.configstrings[0 as libc::c_int as usize]).as_mut_ptr(), server);
    if Cvar_VariableValue(
        b"deathmatch\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) != 0.
    {
        sprintf(
            (sv.configstrings[29 as libc::c_int as usize]).as_mut_ptr(),
            b"%g\0" as *const u8 as *const libc::c_char,
            (*sv_airaccelerate).value as libc::c_double,
        );
        pm_airaccelerate = (*sv_airaccelerate).value;
    } else {
        strcpy(
            (sv.configstrings[29 as libc::c_int as usize]).as_mut_ptr(),
            b"0\0" as *const u8 as *const libc::c_char,
        );
        pm_airaccelerate = 0 as libc::c_int as libc::c_float;
    }
    SZ_Init(
        &mut sv.multicast,
        (sv.multicast_buf).as_mut_ptr(),
        ::std::mem::size_of::<[byte; 1400]>() as libc::c_ulong as libc::c_int,
    );
    strcpy((sv.name).as_mut_ptr(), server);
    i = 0 as libc::c_int;
    while (i as libc::c_float) < (*maxclients).value {
        if (*(svs.clients).offset(i as isize)).state as libc::c_uint
            > cs_connected as libc::c_int as libc::c_uint
        {
            (*(svs.clients).offset(i as isize)).state = cs_connected;
        }
        (*(svs.clients).offset(i as isize)).lastframe = -(1 as libc::c_int);
        i += 1;
    }
    sv.time = 1000 as libc::c_int as libc::c_uint;
    strcpy((sv.name).as_mut_ptr(), server);
    strcpy((sv.configstrings[0 as libc::c_int as usize]).as_mut_ptr(), server);
    if serverstate as libc::c_uint != ss_game as libc::c_int as libc::c_uint {
        sv
            .models[1 as libc::c_int
            as usize] = CM_LoadMap(
            b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            false_0,
            &mut checksum,
        );
    } else {
        Com_sprintf(
            (sv.configstrings[(32 as libc::c_int + 1 as libc::c_int) as usize])
                .as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
            b"maps/%s.bsp\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            server,
        );
        sv
            .models[1 as libc::c_int
            as usize] = CM_LoadMap(
            (sv.configstrings[(32 as libc::c_int + 1 as libc::c_int) as usize])
                .as_mut_ptr(),
            false_0,
            &mut checksum,
        );
    }
    Com_sprintf(
        (sv.configstrings[31 as libc::c_int as usize]).as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
        b"%i\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        checksum,
    );
    SV_ClearWorld();
    i = 1 as libc::c_int;
    while i < CM_NumInlineModels() {
        Com_sprintf(
            (sv.configstrings[(32 as libc::c_int + 1 as libc::c_int + i) as usize])
                .as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
            b"*%i\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            i,
        );
        sv
            .models[(i + 1 as libc::c_int)
            as usize] = CM_InlineModel(
            (sv.configstrings[(32 as libc::c_int + 1 as libc::c_int + i) as usize])
                .as_mut_ptr(),
        );
        i += 1;
    }
    sv.state = ss_loading;
    Com_SetServerState(sv.state as libc::c_int);
    ((*ge).SpawnEntities)
        .expect(
            "non-null function pointer",
        )((sv.name).as_mut_ptr(), CM_EntityString(), spawnpoint);
    ((*ge).RunFrame).expect("non-null function pointer")();
    ((*ge).RunFrame).expect("non-null function pointer")();
    sv.state = serverstate;
    Com_SetServerState(sv.state as libc::c_int);
    SV_CreateBaseline();
    SV_CheckForSavegame();
    Cvar_FullSet(
        b"mapname\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        (sv.name).as_mut_ptr(),
        4 as libc::c_int | 8 as libc::c_int,
    );
    Com_Printf(
        b"-------------------------------------\n\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
}
#[no_mangle]
pub unsafe extern "C" fn SV_InitGame() {
    let mut i: libc::c_int = 0;
    let mut ent: *mut edict_t = 0 as *mut edict_t;
    let mut idmaster: [libc::c_char; 32] = [0; 32];
    if svs.initialized as u64 != 0 {
        SV_Shutdown(
            b"Server restarted\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            true_0,
        );
    } else {
        CL_Drop();
        SCR_BeginLoadingPlaque();
    }
    Cvar_GetLatchedVars();
    svs.initialized = true_0;
    if Cvar_VariableValue(
        b"coop\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) != 0.
        && Cvar_VariableValue(
            b"deathmatch\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) != 0.
    {
        Com_Printf(
            b"Deathmatch and Coop both set, disabling Coop\n\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        );
        Cvar_FullSet(
            b"coop\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            4 as libc::c_int | 16 as libc::c_int,
        );
    }
    if (*dedicated).value != 0. {
        if Cvar_VariableValue(
            b"coop\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) == 0.
        {
            Cvar_FullSet(
                b"deathmatch\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                b"1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                4 as libc::c_int | 16 as libc::c_int,
            );
        }
    }
    if Cvar_VariableValue(
        b"deathmatch\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) != 0.
    {
        if (*maxclients).value <= 1 as libc::c_int as libc::c_float {
            Cvar_FullSet(
                b"maxclients\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                b"8\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                4 as libc::c_int | 16 as libc::c_int,
            );
        } else if (*maxclients).value > 256 as libc::c_int as libc::c_float {
            Cvar_FullSet(
                b"maxclients\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                va(
                    b"%i\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    256 as libc::c_int,
                ),
                4 as libc::c_int | 16 as libc::c_int,
            );
        }
    } else if Cvar_VariableValue(
        b"coop\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) != 0.
    {
        if (*maxclients).value <= 1 as libc::c_int as libc::c_float
            || (*maxclients).value > 4 as libc::c_int as libc::c_float
        {
            Cvar_FullSet(
                b"maxclients\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                b"4\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                4 as libc::c_int | 16 as libc::c_int,
            );
        }
    } else {
        Cvar_FullSet(
            b"maxclients\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            4 as libc::c_int | 16 as libc::c_int,
        );
    }
    svs.spawncount = rand();
    svs
        .clients = Z_Malloc(
        (::std::mem::size_of::<client_t>() as libc::c_ulong as libc::c_float
            * (*maxclients).value) as libc::c_int,
    ) as *mut client_t;
    svs
        .num_client_entities = ((*maxclients).value * 16 as libc::c_int as libc::c_float
        * 64 as libc::c_int as libc::c_float) as libc::c_int;
    svs
        .client_entities = Z_Malloc(
        (::std::mem::size_of::<entity_state_t>() as libc::c_ulong)
            .wrapping_mul(svs.num_client_entities as libc::c_ulong) as libc::c_int,
    ) as *mut entity_state_t;
    NET_Config(
        ((*maxclients).value > 1 as libc::c_int as libc::c_float) as libc::c_int
            as qboolean,
    );
    svs.last_heartbeat = -(99999 as libc::c_int);
    Com_sprintf(
        idmaster.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong as libc::c_int,
        b"192.246.40.37:%i\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        27900 as libc::c_int,
    );
    NET_StringToAdr(
        idmaster.as_mut_ptr(),
        &mut *master_adr.as_mut_ptr().offset(0 as libc::c_int as isize),
    );
    SV_InitGameProgs();
    i = 0 as libc::c_int;
    while (i as libc::c_float) < (*maxclients).value {
        ent = ((*ge).edicts as *mut byte)
            .offset(((*ge).edict_size * (i + 1 as libc::c_int)) as isize)
            as *mut edict_t;
        (*ent).s.number = i + 1 as libc::c_int;
        let ref mut fresh0 = (*(svs.clients).offset(i as isize)).edict;
        *fresh0 = ent;
        memset(
            &mut (*(svs.clients).offset(i as isize)).lastcmd as *mut usercmd_t
                as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<usercmd_t>() as libc::c_ulong,
        );
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn SV_Map(
    mut attractloop: qboolean,
    mut levelstring: *mut libc::c_char,
    mut loadgame: qboolean,
) {
    let mut level: [libc::c_char; 64] = [0; 64];
    let mut ch: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut l: libc::c_int = 0;
    let mut spawnpoint: [libc::c_char; 64] = [0; 64];
    sv.loadgame = loadgame;
    sv.attractloop = attractloop;
    if sv.state as libc::c_uint == ss_dead as libc::c_int as libc::c_uint
        && sv.loadgame as u64 == 0
    {
        SV_InitGame();
    }
    strcpy(level.as_mut_ptr(), levelstring);
    ch = strstr(level.as_mut_ptr(), b"+\0" as *const u8 as *const libc::c_char);
    if !ch.is_null() {
        *ch = 0 as libc::c_int as libc::c_char;
        Cvar_Set(
            b"nextserver\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            va(
                b"gamemap \"%s\"\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ch.offset(1 as libc::c_int as isize),
            ),
        );
    } else {
        Cvar_Set(
            b"nextserver\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
    if Cvar_VariableValue(
        b"coop\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) != 0.
        && Q_stricmp(
            level.as_mut_ptr(),
            b"victory.pcx\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) == 0
    {
        Cvar_Set(
            b"nextserver\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"gamemap \"*base1\"\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    ch = strstr(level.as_mut_ptr(), b"$\0" as *const u8 as *const libc::c_char);
    if !ch.is_null() {
        *ch = 0 as libc::c_int as libc::c_char;
        strcpy(spawnpoint.as_mut_ptr(), ch.offset(1 as libc::c_int as isize));
    } else {
        spawnpoint[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    }
    if level[0 as libc::c_int as usize] as libc::c_int == '*' as i32 {
        strcpy(level.as_mut_ptr(), level.as_mut_ptr().offset(1 as libc::c_int as isize));
    }
    l = strlen(level.as_mut_ptr()) as libc::c_int;
    if l > 4 as libc::c_int
        && strcmp(
            level.as_mut_ptr().offset(l as isize).offset(-(4 as libc::c_int as isize)),
            b".cin\0" as *const u8 as *const libc::c_char,
        ) == 0
    {
        SCR_BeginLoadingPlaque();
        SV_BroadcastCommand(
            b"changing\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        SV_SpawnServer(
            level.as_mut_ptr(),
            spawnpoint.as_mut_ptr(),
            ss_cinematic,
            attractloop,
            loadgame,
        );
    } else if l > 4 as libc::c_int
        && strcmp(
            level.as_mut_ptr().offset(l as isize).offset(-(4 as libc::c_int as isize)),
            b".dm2\0" as *const u8 as *const libc::c_char,
        ) == 0
    {
        SCR_BeginLoadingPlaque();
        SV_BroadcastCommand(
            b"changing\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        SV_SpawnServer(
            level.as_mut_ptr(),
            spawnpoint.as_mut_ptr(),
            ss_demo,
            attractloop,
            loadgame,
        );
    } else if l > 4 as libc::c_int
        && strcmp(
            level.as_mut_ptr().offset(l as isize).offset(-(4 as libc::c_int as isize)),
            b".pcx\0" as *const u8 as *const libc::c_char,
        ) == 0
    {
        SCR_BeginLoadingPlaque();
        SV_BroadcastCommand(
            b"changing\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        SV_SpawnServer(
            level.as_mut_ptr(),
            spawnpoint.as_mut_ptr(),
            ss_pic,
            attractloop,
            loadgame,
        );
    } else {
        SCR_BeginLoadingPlaque();
        SV_BroadcastCommand(
            b"changing\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        SV_SendClientMessages();
        SV_SpawnServer(
            level.as_mut_ptr(),
            spawnpoint.as_mut_ptr(),
            ss_game,
            attractloop,
            loadgame,
        );
        Cbuf_CopyToDefer();
    }
    SV_BroadcastCommand(
        b"reconnect\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
}
