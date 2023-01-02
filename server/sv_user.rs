#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(extern_types, register_tool)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn SV_UserinfoChanged(cl: *mut client_t);
    static mut ge: *mut game_export_t;
    fn SV_DropClient(drop_0: *mut client_t);
    static mut sv_client: *mut client_t;
    static mut sv_enforcetime: *mut cvar_t;
    static mut sv_paused: *mut cvar_t;
    static mut sv: server_t;
    static mut svs: server_static_t;
    fn COM_BlockSequenceCRCByte(
        base: *mut byte,
        length: libc::c_int,
        sequence: libc::c_int,
    ) -> byte;
    fn Com_Error(code: libc::c_int, fmt: *mut libc::c_char, _: ...);
    fn Com_DPrintf(fmt: *mut libc::c_char, _: ...);
    fn FS_FreeFile(buffer: *mut libc::c_void);
    fn FS_LoadFile(
        path: *mut libc::c_char,
        buffer: *mut *mut libc::c_void,
    ) -> libc::c_int;
    fn FS_FOpenFile(filename: *mut libc::c_char, file: *mut *mut FILE) -> libc::c_int;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn atoi(__nptr: *const libc::c_char) -> libc::c_int;
    fn Com_sprintf(
        dest: *mut libc::c_char,
        size: libc::c_int,
        fmt: *mut libc::c_char,
        _: ...
    );
    fn va(format: *mut libc::c_char, _: ...) -> *mut libc::c_char;
    fn Com_Printf(msg: *mut libc::c_char, _: ...);
    fn SZ_Write(buf: *mut sizebuf_t, data: *mut libc::c_void, length: libc::c_int);
    fn MSG_WriteByte(sb: *mut sizebuf_t, c: libc::c_int);
    fn MSG_WriteShort(sb: *mut sizebuf_t, c: libc::c_int);
    fn MSG_WriteLong(sb: *mut sizebuf_t, c: libc::c_int);
    fn MSG_WriteString(sb: *mut sizebuf_t, s: *mut libc::c_char);
    fn MSG_WriteDeltaEntity(
        from: *mut entity_state_s,
        to: *mut entity_state_s,
        msg: *mut sizebuf_t,
        force: qboolean,
        newentity: qboolean,
    );
    fn MSG_ReadByte(sb: *mut sizebuf_t) -> libc::c_int;
    fn MSG_ReadLong(sb: *mut sizebuf_t) -> libc::c_int;
    fn MSG_ReadString(sb: *mut sizebuf_t) -> *mut libc::c_char;
    fn MSG_ReadDeltaUsercmd(
        sb: *mut sizebuf_t,
        from: *mut usercmd_s,
        cmd: *mut usercmd_s,
    );
    fn Info_Print(s: *mut libc::c_char);
    fn Cbuf_AddText(text: *mut libc::c_char);
    fn Cbuf_InsertFromDefer();
    fn Cmd_Argc() -> libc::c_int;
    fn Cmd_Argv(arg: libc::c_int) -> *mut libc::c_char;
    fn Cmd_TokenizeString(text: *mut libc::c_char, macroExpand: qboolean);
    fn Cvar_Set(var_name: *mut libc::c_char, value: *mut libc::c_char) -> *mut cvar_t;
    fn Cvar_VariableValue(var_name: *mut libc::c_char) -> libc::c_float;
    fn Cvar_VariableString(var_name: *mut libc::c_char) -> *mut libc::c_char;
    fn Cvar_Serverinfo() -> *mut libc::c_char;
    static mut net_message: sizebuf_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ucmd_t {
    pub name: *mut libc::c_char,
    pub func: Option::<unsafe extern "C" fn() -> ()>,
}
#[no_mangle]
pub static mut sv_player: *mut edict_t = 0 as *const edict_t as *mut edict_t;
#[no_mangle]
pub unsafe extern "C" fn SV_BeginDemoserver() {
    let mut name: [libc::c_char; 128] = [0; 128];
    Com_sprintf(
        name.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong as libc::c_int,
        b"demos/%s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        (sv.name).as_mut_ptr(),
    );
    FS_FOpenFile(name.as_mut_ptr(), &mut sv.demofile);
    if (sv.demofile).is_null() {
        Com_Error(
            1 as libc::c_int,
            b"Couldn't open %s\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            name.as_mut_ptr(),
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn SV_New_f() {
    let mut gamedir: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut playernum: libc::c_int = 0;
    let mut ent: *mut edict_t = 0 as *mut edict_t;
    Com_DPrintf(
        b"New() from %s\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ((*sv_client).name).as_mut_ptr(),
    );
    if (*sv_client).state as libc::c_uint != cs_connected as libc::c_int as libc::c_uint
    {
        Com_Printf(
            b"New not valid -- already spawned\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return;
    }
    if sv.state as libc::c_uint == ss_demo as libc::c_int as libc::c_uint {
        SV_BeginDemoserver();
        return;
    }
    gamedir = Cvar_VariableString(
        b"gamedir\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    MSG_WriteByte(&mut (*sv_client).netchan.message, svc_serverdata as libc::c_int);
    MSG_WriteLong(&mut (*sv_client).netchan.message, 34 as libc::c_int);
    MSG_WriteLong(&mut (*sv_client).netchan.message, svs.spawncount);
    MSG_WriteByte(&mut (*sv_client).netchan.message, sv.attractloop as libc::c_int);
    MSG_WriteString(&mut (*sv_client).netchan.message, gamedir);
    if sv.state as libc::c_uint == ss_cinematic as libc::c_int as libc::c_uint
        || sv.state as libc::c_uint == ss_pic as libc::c_int as libc::c_uint
    {
        playernum = -(1 as libc::c_int);
    } else {
        playernum = sv_client.offset_from(svs.clients) as libc::c_long as libc::c_int;
    }
    MSG_WriteShort(&mut (*sv_client).netchan.message, playernum);
    MSG_WriteString(
        &mut (*sv_client).netchan.message,
        (sv.configstrings[0 as libc::c_int as usize]).as_mut_ptr(),
    );
    if sv.state as libc::c_uint == ss_game as libc::c_int as libc::c_uint {
        ent = ((*ge).edicts as *mut byte)
            .offset(((*ge).edict_size * (playernum + 1 as libc::c_int)) as isize)
            as *mut edict_t;
        (*ent).s.number = playernum + 1 as libc::c_int;
        let ref mut fresh0 = (*sv_client).edict;
        *fresh0 = ent;
        memset(
            &mut (*sv_client).lastcmd as *mut usercmd_t as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<usercmd_t>() as libc::c_ulong,
        );
        MSG_WriteByte(&mut (*sv_client).netchan.message, svc_stufftext as libc::c_int);
        MSG_WriteString(
            &mut (*sv_client).netchan.message,
            va(
                b"cmd configstrings %i 0\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                svs.spawncount,
            ),
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn SV_Configstrings_f() {
    let mut start: libc::c_int = 0;
    Com_DPrintf(
        b"Configstrings() from %s\n\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        ((*sv_client).name).as_mut_ptr(),
    );
    if (*sv_client).state as libc::c_uint != cs_connected as libc::c_int as libc::c_uint
    {
        Com_Printf(
            b"configstrings not valid -- already spawned\n\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        );
        return;
    }
    if atoi(Cmd_Argv(1 as libc::c_int)) != svs.spawncount {
        Com_Printf(
            b"SV_Configstrings_f from different level\n\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        );
        SV_New_f();
        return;
    }
    start = atoi(Cmd_Argv(2 as libc::c_int));
    while (*sv_client).netchan.message.cursize < 1400 as libc::c_int / 2 as libc::c_int
        && start
            < 32 as libc::c_int + 256 as libc::c_int + 256 as libc::c_int
                + 256 as libc::c_int + 256 as libc::c_int + 256 as libc::c_int
                + 256 as libc::c_int + 256 as libc::c_int * 2 as libc::c_int
    {
        if sv.configstrings[start as usize][0 as libc::c_int as usize] != 0 {
            MSG_WriteByte(
                &mut (*sv_client).netchan.message,
                svc_configstring as libc::c_int,
            );
            MSG_WriteShort(&mut (*sv_client).netchan.message, start);
            MSG_WriteString(
                &mut (*sv_client).netchan.message,
                (sv.configstrings[start as usize]).as_mut_ptr(),
            );
        }
        start += 1;
    }
    if start
        == 32 as libc::c_int + 256 as libc::c_int + 256 as libc::c_int
            + 256 as libc::c_int + 256 as libc::c_int + 256 as libc::c_int
            + 256 as libc::c_int + 256 as libc::c_int * 2 as libc::c_int
    {
        MSG_WriteByte(&mut (*sv_client).netchan.message, svc_stufftext as libc::c_int);
        MSG_WriteString(
            &mut (*sv_client).netchan.message,
            va(
                b"cmd baselines %i 0\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                svs.spawncount,
            ),
        );
    } else {
        MSG_WriteByte(&mut (*sv_client).netchan.message, svc_stufftext as libc::c_int);
        MSG_WriteString(
            &mut (*sv_client).netchan.message,
            va(
                b"cmd configstrings %i %i\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                svs.spawncount,
                start,
            ),
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn SV_Baselines_f() {
    let mut start: libc::c_int = 0;
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
    let mut base: *mut entity_state_t = 0 as *mut entity_state_t;
    Com_DPrintf(
        b"Baselines() from %s\n\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        ((*sv_client).name).as_mut_ptr(),
    );
    if (*sv_client).state as libc::c_uint != cs_connected as libc::c_int as libc::c_uint
    {
        Com_Printf(
            b"baselines not valid -- already spawned\n\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        );
        return;
    }
    if atoi(Cmd_Argv(1 as libc::c_int)) != svs.spawncount {
        Com_Printf(
            b"SV_Baselines_f from different level\n\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        );
        SV_New_f();
        return;
    }
    start = atoi(Cmd_Argv(2 as libc::c_int));
    memset(
        &mut nullstate as *mut entity_state_t as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<entity_state_t>() as libc::c_ulong,
    );
    while (*sv_client).netchan.message.cursize < 1400 as libc::c_int / 2 as libc::c_int
        && start < 1024 as libc::c_int
    {
        base = &mut *(sv.baselines).as_mut_ptr().offset(start as isize)
            as *mut entity_state_t;
        if (*base).modelindex != 0 || (*base).sound != 0 || (*base).effects != 0 {
            MSG_WriteByte(
                &mut (*sv_client).netchan.message,
                svc_spawnbaseline as libc::c_int,
            );
            MSG_WriteDeltaEntity(
                &mut nullstate,
                base,
                &mut (*sv_client).netchan.message,
                true_0,
                true_0,
            );
        }
        start += 1;
    }
    if start == 1024 as libc::c_int {
        MSG_WriteByte(&mut (*sv_client).netchan.message, svc_stufftext as libc::c_int);
        MSG_WriteString(
            &mut (*sv_client).netchan.message,
            va(
                b"precache %i\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                svs.spawncount,
            ),
        );
    } else {
        MSG_WriteByte(&mut (*sv_client).netchan.message, svc_stufftext as libc::c_int);
        MSG_WriteString(
            &mut (*sv_client).netchan.message,
            va(
                b"cmd baselines %i %i\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                svs.spawncount,
                start,
            ),
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn SV_Begin_f() {
    Com_DPrintf(
        b"Begin() from %s\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ((*sv_client).name).as_mut_ptr(),
    );
    if atoi(Cmd_Argv(1 as libc::c_int)) != svs.spawncount {
        Com_Printf(
            b"SV_Begin_f from different level\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        SV_New_f();
        return;
    }
    (*sv_client).state = cs_spawned;
    ((*ge).ClientBegin).expect("non-null function pointer")(sv_player);
    Cbuf_InsertFromDefer();
}
#[no_mangle]
pub unsafe extern "C" fn SV_NextDownload_f() {
    let mut r: libc::c_int = 0;
    let mut percent: libc::c_int = 0;
    let mut size: libc::c_int = 0;
    if ((*sv_client).download).is_null() {
        return;
    }
    r = (*sv_client).downloadsize - (*sv_client).downloadcount;
    if r > 1024 as libc::c_int {
        r = 1024 as libc::c_int;
    }
    MSG_WriteByte(&mut (*sv_client).netchan.message, svc_download as libc::c_int);
    MSG_WriteShort(&mut (*sv_client).netchan.message, r);
    (*sv_client).downloadcount += r;
    size = (*sv_client).downloadsize;
    if size == 0 {
        size = 1 as libc::c_int;
    }
    percent = (*sv_client).downloadcount * 100 as libc::c_int / size;
    MSG_WriteByte(&mut (*sv_client).netchan.message, percent);
    SZ_Write(
        &mut (*sv_client).netchan.message,
        ((*sv_client).download)
            .offset((*sv_client).downloadcount as isize)
            .offset(-(r as isize)) as *mut libc::c_void,
        r,
    );
    if (*sv_client).downloadcount != (*sv_client).downloadsize {
        return;
    }
    FS_FreeFile((*sv_client).download as *mut libc::c_void);
    let ref mut fresh1 = (*sv_client).download;
    *fresh1 = 0 as *mut byte;
}
#[no_mangle]
pub unsafe extern "C" fn SV_BeginDownload_f() {
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    extern "C" {
        static mut allow_download: *mut cvar_t;
    }
    extern "C" {
        static mut allow_download_players: *mut cvar_t;
    }
    extern "C" {
        static mut allow_download_models: *mut cvar_t;
    }
    extern "C" {
        static mut allow_download_sounds: *mut cvar_t;
    }
    extern "C" {
        static mut allow_download_maps: *mut cvar_t;
    }
    extern "C" {
        static mut file_from_pak: libc::c_int;
    }
    let mut offset: libc::c_int = 0 as libc::c_int;
    name = Cmd_Argv(1 as libc::c_int);
    if Cmd_Argc() > 2 as libc::c_int {
        offset = atoi(Cmd_Argv(2 as libc::c_int));
    }
    if !(strstr(name, b"..\0" as *const u8 as *const libc::c_char)).is_null()
        || (*allow_download).value == 0. || *name as libc::c_int == '.' as i32
        || *name as libc::c_int == '/' as i32
        || strncmp(
            name,
            b"players/\0" as *const u8 as *const libc::c_char,
            6 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int && (*allow_download_players).value == 0.
        || strncmp(
            name,
            b"models/\0" as *const u8 as *const libc::c_char,
            6 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int && (*allow_download_models).value == 0.
        || strncmp(
            name,
            b"sound/\0" as *const u8 as *const libc::c_char,
            6 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int && (*allow_download_sounds).value == 0.
        || strncmp(
            name,
            b"maps/\0" as *const u8 as *const libc::c_char,
            6 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int && (*allow_download_maps).value == 0.
        || (strstr(name, b"/\0" as *const u8 as *const libc::c_char)).is_null()
    {
        MSG_WriteByte(&mut (*sv_client).netchan.message, svc_download as libc::c_int);
        MSG_WriteShort(&mut (*sv_client).netchan.message, -(1 as libc::c_int));
        MSG_WriteByte(&mut (*sv_client).netchan.message, 0 as libc::c_int);
        return;
    }
    if !((*sv_client).download).is_null() {
        FS_FreeFile((*sv_client).download as *mut libc::c_void);
    }
    (*sv_client)
        .downloadsize = FS_LoadFile(
        name,
        &mut (*sv_client).download as *mut *mut byte as *mut *mut libc::c_void,
    );
    (*sv_client).downloadcount = offset;
    if offset > (*sv_client).downloadsize {
        (*sv_client).downloadcount = (*sv_client).downloadsize;
    }
    if ((*sv_client).download).is_null()
        || strncmp(
            name,
            b"maps/\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int && file_from_pak != 0
    {
        Com_DPrintf(
            b"Couldn't download %s to %s\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            name,
            ((*sv_client).name).as_mut_ptr(),
        );
        if !((*sv_client).download).is_null() {
            FS_FreeFile((*sv_client).download as *mut libc::c_void);
            let ref mut fresh2 = (*sv_client).download;
            *fresh2 = 0 as *mut byte;
        }
        MSG_WriteByte(&mut (*sv_client).netchan.message, svc_download as libc::c_int);
        MSG_WriteShort(&mut (*sv_client).netchan.message, -(1 as libc::c_int));
        MSG_WriteByte(&mut (*sv_client).netchan.message, 0 as libc::c_int);
        return;
    }
    SV_NextDownload_f();
    Com_DPrintf(
        b"Downloading %s to %s\n\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        name,
        ((*sv_client).name).as_mut_ptr(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn SV_Disconnect_f() {
    SV_DropClient(sv_client);
}
#[no_mangle]
pub unsafe extern "C" fn SV_ShowServerinfo_f() {
    Info_Print(Cvar_Serverinfo());
}
#[no_mangle]
pub unsafe extern "C" fn SV_Nextserver() {
    let mut v: *mut libc::c_char = 0 as *mut libc::c_char;
    if sv.state as libc::c_uint == ss_game as libc::c_int as libc::c_uint
        || sv.state as libc::c_uint == ss_pic as libc::c_int as libc::c_uint
            && Cvar_VariableValue(
                b"coop\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ) == 0.
    {
        return;
    }
    svs.spawncount += 1;
    v = Cvar_VariableString(
        b"nextserver\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if *v.offset(0 as libc::c_int as isize) == 0 {
        Cbuf_AddText(
            b"killserver\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    } else {
        Cbuf_AddText(v);
        Cbuf_AddText(b"\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    }
    Cvar_Set(
        b"nextserver\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
}
#[no_mangle]
pub unsafe extern "C" fn SV_Nextserver_f() {
    if atoi(Cmd_Argv(1 as libc::c_int)) != svs.spawncount {
        Com_DPrintf(
            b"Nextserver() from wrong level, from %s\n\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            ((*sv_client).name).as_mut_ptr(),
        );
        return;
    }
    Com_DPrintf(
        b"Nextserver() from %s\n\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        ((*sv_client).name).as_mut_ptr(),
    );
    SV_Nextserver();
}
#[no_mangle]
pub static mut ucmds: [ucmd_t; 10] = unsafe {
    [
        {
            let mut init = ucmd_t {
                name: b"new\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                func: Some(SV_New_f as unsafe extern "C" fn() -> ()),
            };
            init
        },
        {
            let mut init = ucmd_t {
                name: b"configstrings\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                func: Some(SV_Configstrings_f as unsafe extern "C" fn() -> ()),
            };
            init
        },
        {
            let mut init = ucmd_t {
                name: b"baselines\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                func: Some(SV_Baselines_f as unsafe extern "C" fn() -> ()),
            };
            init
        },
        {
            let mut init = ucmd_t {
                name: b"begin\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                func: Some(SV_Begin_f as unsafe extern "C" fn() -> ()),
            };
            init
        },
        {
            let mut init = ucmd_t {
                name: b"nextserver\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                func: Some(SV_Nextserver_f as unsafe extern "C" fn() -> ()),
            };
            init
        },
        {
            let mut init = ucmd_t {
                name: b"disconnect\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                func: Some(SV_Disconnect_f as unsafe extern "C" fn() -> ()),
            };
            init
        },
        {
            let mut init = ucmd_t {
                name: b"info\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                func: Some(SV_ShowServerinfo_f as unsafe extern "C" fn() -> ()),
            };
            init
        },
        {
            let mut init = ucmd_t {
                name: b"download\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                func: Some(SV_BeginDownload_f as unsafe extern "C" fn() -> ()),
            };
            init
        },
        {
            let mut init = ucmd_t {
                name: b"nextdl\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                func: Some(SV_NextDownload_f as unsafe extern "C" fn() -> ()),
            };
            init
        },
        {
            let mut init = ucmd_t {
                name: 0 as *const libc::c_char as *mut libc::c_char,
                func: None,
            };
            init
        },
    ]
};
#[no_mangle]
pub unsafe extern "C" fn SV_ExecuteUserCommand(mut s: *mut libc::c_char) {
    let mut u: *mut ucmd_t = 0 as *mut ucmd_t;
    Cmd_TokenizeString(s, true_0);
    sv_player = (*sv_client).edict;
    u = ucmds.as_mut_ptr();
    while !((*u).name).is_null() {
        if strcmp(Cmd_Argv(0 as libc::c_int), (*u).name) == 0 {
            ((*u).func).expect("non-null function pointer")();
            break;
        } else {
            u = u.offset(1);
        }
    }
    if ((*u).name).is_null()
        && sv.state as libc::c_uint == ss_game as libc::c_int as libc::c_uint
    {
        ((*ge).ClientCommand).expect("non-null function pointer")(sv_player);
    }
}
#[no_mangle]
pub unsafe extern "C" fn SV_ClientThink(mut cl: *mut client_t, mut cmd: *mut usercmd_t) {
    (*cl).commandMsec -= (*cmd).msec as libc::c_int;
    if (*cl).commandMsec < 0 as libc::c_int && (*sv_enforcetime).value != 0. {
        Com_DPrintf(
            b"commandMsec underflow from %s\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            ((*cl).name).as_mut_ptr(),
        );
        return;
    }
    ((*ge).ClientThink).expect("non-null function pointer")((*cl).edict, cmd);
}
#[no_mangle]
pub unsafe extern "C" fn SV_ExecuteClientMessage(mut cl: *mut client_t) {
    let mut c: libc::c_int = 0;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
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
    let mut oldest: usercmd_t = usercmd_t {
        msec: 0,
        buttons: 0,
        angles: [0; 3],
        forwardmove: 0,
        sidemove: 0,
        upmove: 0,
        impulse: 0,
        lightlevel: 0,
    };
    let mut oldcmd: usercmd_t = usercmd_t {
        msec: 0,
        buttons: 0,
        angles: [0; 3],
        forwardmove: 0,
        sidemove: 0,
        upmove: 0,
        impulse: 0,
        lightlevel: 0,
    };
    let mut newcmd: usercmd_t = usercmd_t {
        msec: 0,
        buttons: 0,
        angles: [0; 3],
        forwardmove: 0,
        sidemove: 0,
        upmove: 0,
        impulse: 0,
        lightlevel: 0,
    };
    let mut net_drop: libc::c_int = 0;
    let mut stringCmdCount: libc::c_int = 0;
    let mut checksum: libc::c_int = 0;
    let mut calculatedChecksum: libc::c_int = 0;
    let mut checksumIndex: libc::c_int = 0;
    let mut move_issued: qboolean = false_0;
    let mut lastframe: libc::c_int = 0;
    sv_client = cl;
    sv_player = (*sv_client).edict;
    move_issued = false_0;
    stringCmdCount = 0 as libc::c_int;
    loop {
        if net_message.readcount > net_message.cursize {
            Com_Printf(
                b"SV_ReadClientMessage: badread\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
            SV_DropClient(cl);
            return;
        }
        c = MSG_ReadByte(&mut net_message);
        if c == -(1 as libc::c_int) {
            break;
        }
        match c {
            1 => {}
            3 => {
                strncpy(
                    ((*cl).userinfo).as_mut_ptr(),
                    MSG_ReadString(&mut net_message),
                    (::std::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                );
                SV_UserinfoChanged(cl);
            }
            2 => {
                if move_issued as u64 != 0 {
                    return;
                }
                move_issued = true_0;
                checksumIndex = net_message.readcount;
                checksum = MSG_ReadByte(&mut net_message);
                lastframe = MSG_ReadLong(&mut net_message);
                if lastframe != (*cl).lastframe {
                    (*cl).lastframe = lastframe;
                    if (*cl).lastframe > 0 as libc::c_int {
                        (*cl)
                            .frame_latency[((*cl).lastframe
                            & 16 as libc::c_int - 1 as libc::c_int)
                            as usize] = svs.realtime
                            - (*cl)
                                .frames[((*cl).lastframe
                                    & 16 as libc::c_int - 1 as libc::c_int) as usize]
                                .senttime;
                    }
                }
                memset(
                    &mut nullcmd as *mut usercmd_t as *mut libc::c_void,
                    0 as libc::c_int,
                    ::std::mem::size_of::<usercmd_t>() as libc::c_ulong,
                );
                MSG_ReadDeltaUsercmd(&mut net_message, &mut nullcmd, &mut oldest);
                MSG_ReadDeltaUsercmd(&mut net_message, &mut oldest, &mut oldcmd);
                MSG_ReadDeltaUsercmd(&mut net_message, &mut oldcmd, &mut newcmd);
                if (*cl).state as libc::c_uint
                    != cs_spawned as libc::c_int as libc::c_uint
                {
                    (*cl).lastframe = -(1 as libc::c_int);
                } else {
                    calculatedChecksum = COM_BlockSequenceCRCByte(
                        (net_message.data)
                            .offset(checksumIndex as isize)
                            .offset(1 as libc::c_int as isize),
                        net_message.readcount - checksumIndex - 1 as libc::c_int,
                        (*cl).netchan.incoming_sequence,
                    ) as libc::c_int;
                    if calculatedChecksum != checksum {
                        Com_DPrintf(
                            b"Failed command checksum for %s (%d != %d)/%d\n\0"
                                as *const u8 as *const libc::c_char as *mut libc::c_char,
                            ((*cl).name).as_mut_ptr(),
                            calculatedChecksum,
                            checksum,
                            (*cl).netchan.incoming_sequence,
                        );
                        return;
                    }
                    if (*sv_paused).value == 0. {
                        net_drop = (*cl).netchan.dropped;
                        if net_drop < 20 as libc::c_int {
                            while net_drop > 2 as libc::c_int {
                                SV_ClientThink(cl, &mut (*cl).lastcmd);
                                net_drop -= 1;
                            }
                            if net_drop > 1 as libc::c_int {
                                SV_ClientThink(cl, &mut oldest);
                            }
                            if net_drop > 0 as libc::c_int {
                                SV_ClientThink(cl, &mut oldcmd);
                            }
                        }
                        SV_ClientThink(cl, &mut newcmd);
                    }
                    (*cl).lastcmd = newcmd;
                }
            }
            4 => {
                s = MSG_ReadString(&mut net_message);
                stringCmdCount += 1;
                if stringCmdCount < 8 as libc::c_int {
                    SV_ExecuteUserCommand(s);
                }
                if (*cl).state as libc::c_uint
                    == cs_zombie as libc::c_int as libc::c_uint
                {
                    return;
                }
            }
            _ => {
                Com_Printf(
                    b"SV_ReadClientMessage: unknown command char\n\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                );
                SV_DropClient(cl);
                return;
            }
        }
    };
}
