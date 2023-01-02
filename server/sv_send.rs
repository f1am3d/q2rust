#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(c_variadic, extern_types, register_tool)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn vsprintf(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: ::std::ffi::VaList,
    ) -> libc::c_int;
    fn fread(
        _: *mut libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn LittleLong(l: libc::c_int) -> libc::c_int;
    static mut curtime: libc::c_int;
    fn Com_Printf(msg: *mut libc::c_char, _: ...);
    fn SZ_Init(buf: *mut sizebuf_t, data: *mut byte, length: libc::c_int);
    fn SZ_Clear(buf: *mut sizebuf_t);
    fn SZ_Write(buf: *mut sizebuf_t, data: *mut libc::c_void, length: libc::c_int);
    fn MSG_WriteByte(sb: *mut sizebuf_t, c: libc::c_int);
    fn MSG_WriteShort(sb: *mut sizebuf_t, c: libc::c_int);
    fn MSG_WriteString(sb: *mut sizebuf_t, s: *mut libc::c_char);
    fn MSG_WritePos(sb: *mut sizebuf_t, pos: *mut vec_t);
    static mut ge: *mut game_export_t;
    fn SV_BuildClientFrame(client: *mut client_t);
    fn SV_WriteFrameToClient(client: *mut client_t, msg: *mut sizebuf_t);
    fn SV_Nextserver();
    fn SV_DropClient(drop_0: *mut client_t);
    static mut sv_client: *mut client_t;
    static mut maxclients: *mut cvar_t;
    static mut sv_paused: *mut cvar_t;
    static mut sv: server_t;
    static mut svs: server_static_t;
    static mut net_from: netadr_t;
    fn Netchan_Transmit(chan: *mut netchan_t, length: libc::c_int, data: *mut byte);
    fn Netchan_OutOfBandPrint(
        net_socket: libc::c_int,
        adr: netadr_t,
        format: *mut libc::c_char,
        _: ...
    );
    static mut dedicated: *mut cvar_t;
    fn CM_ClusterPVS(cluster: libc::c_int) -> *mut byte;
    fn CM_ClusterPHS(cluster: libc::c_int) -> *mut byte;
    fn CM_PointLeafnum(p: *mut vec_t) -> libc::c_int;
    fn CM_LeafCluster(leafnum: libc::c_int) -> libc::c_int;
    fn CM_LeafArea(leafnum: libc::c_int) -> libc::c_int;
    fn CM_AreasConnected(area1: libc::c_int, area2: libc::c_int) -> qboolean;
    fn Com_Error(code: libc::c_int, fmt: *mut libc::c_char, _: ...);
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type size_t = libc::c_ulong;
pub type va_list = __builtin_va_list;
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
pub type C2RustUnnamed = libc::c_uint;
pub const RD_PACKET: C2RustUnnamed = 2;
pub const RD_CLIENT: C2RustUnnamed = 1;
pub const RD_NONE: C2RustUnnamed = 0;
#[no_mangle]
pub static mut sv_outputbuf: [libc::c_char; 1384] = [0; 1384];
#[no_mangle]
pub unsafe extern "C" fn SV_FlushRedirect(
    mut sv_redirected: libc::c_int,
    mut outputbuf: *mut libc::c_char,
) {
    if sv_redirected == RD_PACKET as libc::c_int {
        Netchan_OutOfBandPrint(
            NS_SERVER as libc::c_int,
            net_from,
            b"print\n%s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            outputbuf,
        );
    } else if sv_redirected == RD_CLIENT as libc::c_int {
        MSG_WriteByte(&mut (*sv_client).netchan.message, svc_print as libc::c_int);
        MSG_WriteByte(&mut (*sv_client).netchan.message, 2 as libc::c_int);
        MSG_WriteString(&mut (*sv_client).netchan.message, outputbuf);
    }
}
#[no_mangle]
pub unsafe extern "C" fn SV_ClientPrintf(
    mut cl: *mut client_t,
    mut level: libc::c_int,
    mut fmt: *mut libc::c_char,
    mut args: ...
) {
    let mut argptr: ::std::ffi::VaListImpl;
    let mut string: [libc::c_char; 1024] = [0; 1024];
    if level < (*cl).messagelevel {
        return;
    }
    argptr = args.clone();
    vsprintf(string.as_mut_ptr(), fmt, argptr.as_va_list());
    MSG_WriteByte(&mut (*cl).netchan.message, svc_print as libc::c_int);
    MSG_WriteByte(&mut (*cl).netchan.message, level);
    MSG_WriteString(&mut (*cl).netchan.message, string.as_mut_ptr());
}
#[no_mangle]
pub unsafe extern "C" fn SV_BroadcastPrintf(
    mut level: libc::c_int,
    mut fmt: *mut libc::c_char,
    mut args: ...
) {
    let mut argptr: ::std::ffi::VaListImpl;
    let mut string: [libc::c_char; 2048] = [0; 2048];
    let mut cl: *mut client_t = 0 as *mut client_t;
    let mut i: libc::c_int = 0;
    argptr = args.clone();
    vsprintf(string.as_mut_ptr(), fmt, argptr.as_va_list());
    if (*dedicated).value != 0. {
        let mut copy: [libc::c_char; 1024] = [0; 1024];
        let mut i_0: libc::c_int = 0;
        i_0 = 0 as libc::c_int;
        while i_0 < 1023 as libc::c_int && string[i_0 as usize] as libc::c_int != 0 {
            copy[i_0
                as usize] = (string[i_0 as usize] as libc::c_int & 127 as libc::c_int)
                as libc::c_char;
            i_0 += 1;
        }
        copy[i_0 as usize] = 0 as libc::c_int as libc::c_char;
        Com_Printf(
            b"%s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            copy.as_mut_ptr(),
        );
    }
    i = 0 as libc::c_int;
    cl = svs.clients;
    while (i as libc::c_float) < (*maxclients).value {
        if !(level < (*cl).messagelevel) {
            if !((*cl).state as libc::c_uint
                != cs_spawned as libc::c_int as libc::c_uint)
            {
                MSG_WriteByte(&mut (*cl).netchan.message, svc_print as libc::c_int);
                MSG_WriteByte(&mut (*cl).netchan.message, level);
                MSG_WriteString(&mut (*cl).netchan.message, string.as_mut_ptr());
            }
        }
        i += 1;
        cl = cl.offset(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn SV_BroadcastCommand(mut fmt: *mut libc::c_char, mut args: ...) {
    let mut argptr: ::std::ffi::VaListImpl;
    let mut string: [libc::c_char; 1024] = [0; 1024];
    if sv.state as u64 == 0 {
        return;
    }
    argptr = args.clone();
    vsprintf(string.as_mut_ptr(), fmt, argptr.as_va_list());
    MSG_WriteByte(&mut sv.multicast, svc_stufftext as libc::c_int);
    MSG_WriteString(&mut sv.multicast, string.as_mut_ptr());
    SV_Multicast(0 as *mut vec_t, MULTICAST_ALL_R);
}
#[no_mangle]
pub unsafe extern "C" fn SV_Multicast(mut origin: *mut vec_t, mut to: multicast_t) {
    let mut client: *mut client_t = 0 as *mut client_t;
    let mut mask: *mut byte = 0 as *mut byte;
    let mut leafnum: libc::c_int = 0;
    let mut cluster: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut reliable: qboolean = false_0;
    let mut area1: libc::c_int = 0;
    let mut area2: libc::c_int = 0;
    reliable = false_0;
    if to as libc::c_uint != MULTICAST_ALL_R as libc::c_int as libc::c_uint
        && to as libc::c_uint != MULTICAST_ALL as libc::c_int as libc::c_uint
    {
        leafnum = CM_PointLeafnum(origin);
        area1 = CM_LeafArea(leafnum);
    } else {
        leafnum = 0 as libc::c_int;
        area1 = 0 as libc::c_int;
    }
    if !(svs.demofile).is_null() {
        SZ_Write(
            &mut svs.demo_multicast,
            sv.multicast.data as *mut libc::c_void,
            sv.multicast.cursize,
        );
    }
    let mut current_block_22: u64;
    match to as libc::c_uint {
        3 => {
            reliable = true_0;
            current_block_22 = 34601305967147713;
        }
        0 => {
            current_block_22 = 34601305967147713;
        }
        4 => {
            reliable = true_0;
            current_block_22 = 1427587873524194456;
        }
        1 => {
            current_block_22 = 1427587873524194456;
        }
        5 => {
            reliable = true_0;
            current_block_22 = 2390421670684857061;
        }
        2 => {
            current_block_22 = 2390421670684857061;
        }
        _ => {
            mask = 0 as *mut byte;
            Com_Error(
                0 as libc::c_int,
                b"SV_Multicast: bad to:%i\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                to as libc::c_uint,
            );
            current_block_22 = 15345278821338558188;
        }
    }
    match current_block_22 {
        2390421670684857061 => {
            leafnum = CM_PointLeafnum(origin);
            cluster = CM_LeafCluster(leafnum);
            mask = CM_ClusterPVS(cluster);
        }
        1427587873524194456 => {
            leafnum = CM_PointLeafnum(origin);
            cluster = CM_LeafCluster(leafnum);
            mask = CM_ClusterPHS(cluster);
        }
        34601305967147713 => {
            leafnum = 0 as libc::c_int;
            mask = 0 as *mut byte;
        }
        _ => {}
    }
    let mut current_block_30: u64;
    j = 0 as libc::c_int;
    client = svs.clients;
    while (j as libc::c_float) < (*maxclients).value {
        if !((*client).state as libc::c_uint == cs_free as libc::c_int as libc::c_uint
            || (*client).state as libc::c_uint
                == cs_zombie as libc::c_int as libc::c_uint)
        {
            if !((*client).state as libc::c_uint
                != cs_spawned as libc::c_int as libc::c_uint && reliable as u64 == 0)
            {
                if !mask.is_null() {
                    leafnum = CM_PointLeafnum(
                        ((*(*client).edict).s.origin).as_mut_ptr(),
                    );
                    cluster = CM_LeafCluster(leafnum);
                    area2 = CM_LeafArea(leafnum);
                    if CM_AreasConnected(area1, area2) as u64 == 0 {
                        current_block_30 = 4761528863920922185;
                    } else if !mask.is_null()
                        && *mask.offset((cluster >> 3 as libc::c_int) as isize)
                            as libc::c_int
                            & (1 as libc::c_int) << (cluster & 7 as libc::c_int) == 0
                    {
                        current_block_30 = 4761528863920922185;
                    } else {
                        current_block_30 = 13131896068329595644;
                    }
                } else {
                    current_block_30 = 13131896068329595644;
                }
                match current_block_30 {
                    4761528863920922185 => {}
                    _ => {
                        if reliable as u64 != 0 {
                            SZ_Write(
                                &mut (*client).netchan.message,
                                sv.multicast.data as *mut libc::c_void,
                                sv.multicast.cursize,
                            );
                        } else {
                            SZ_Write(
                                &mut (*client).datagram,
                                sv.multicast.data as *mut libc::c_void,
                                sv.multicast.cursize,
                            );
                        }
                    }
                }
            }
        }
        j += 1;
        client = client.offset(1);
    }
    SZ_Clear(&mut sv.multicast);
}
#[no_mangle]
pub unsafe extern "C" fn SV_StartSound(
    mut origin: *mut vec_t,
    mut entity: *mut edict_t,
    mut channel: libc::c_int,
    mut soundindex: libc::c_int,
    mut volume: libc::c_float,
    mut attenuation: libc::c_float,
    mut timeofs: libc::c_float,
) {
    let mut sendchan: libc::c_int = 0;
    let mut flags: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut ent: libc::c_int = 0;
    let mut origin_v: vec3_t = [0.; 3];
    let mut use_phs: qboolean = false_0;
    if volume < 0 as libc::c_int as libc::c_float || volume as libc::c_double > 1.0f64 {
        Com_Error(
            0 as libc::c_int,
            b"SV_StartSound: volume = %f\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            volume as libc::c_double,
        );
    }
    if attenuation < 0 as libc::c_int as libc::c_float
        || attenuation > 4 as libc::c_int as libc::c_float
    {
        Com_Error(
            0 as libc::c_int,
            b"SV_StartSound: attenuation = %f\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            attenuation as libc::c_double,
        );
    }
    if timeofs < 0 as libc::c_int as libc::c_float
        || timeofs as libc::c_double > 0.255f64
    {
        Com_Error(
            0 as libc::c_int,
            b"SV_StartSound: timeofs = %f\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            timeofs as libc::c_double,
        );
    }
    ent = ((entity as *mut byte).offset_from((*ge).edicts as *mut byte) as libc::c_long
        / (*ge).edict_size as libc::c_long) as libc::c_int;
    if channel & 8 as libc::c_int != 0 {
        use_phs = false_0;
        channel &= 7 as libc::c_int;
    } else {
        use_phs = true_0;
    }
    sendchan = ent << 3 as libc::c_int | channel & 7 as libc::c_int;
    flags = 0 as libc::c_int;
    if volume as libc::c_double != 1.0f64 {
        flags |= (1 as libc::c_int) << 0 as libc::c_int;
    }
    if attenuation as libc::c_double != 1.0f64 {
        flags |= (1 as libc::c_int) << 1 as libc::c_int;
    }
    if (*entity).svflags & 0x1 as libc::c_int != 0
        || (*entity).solid as libc::c_uint == SOLID_BSP as libc::c_int as libc::c_uint
        || !origin.is_null()
    {
        flags |= (1 as libc::c_int) << 2 as libc::c_int;
    }
    flags |= (1 as libc::c_int) << 3 as libc::c_int;
    if timeofs != 0. {
        flags |= (1 as libc::c_int) << 4 as libc::c_int;
    }
    if origin.is_null() {
        origin = origin_v.as_mut_ptr();
        if (*entity).solid as libc::c_uint == SOLID_BSP as libc::c_int as libc::c_uint {
            i = 0 as libc::c_int;
            while i < 3 as libc::c_int {
                origin_v[i
                    as usize] = ((*entity).s.origin[i as usize] as libc::c_double
                    + 0.5f64
                        * ((*entity).mins[i as usize] + (*entity).maxs[i as usize])
                            as libc::c_double) as vec_t;
                i += 1;
            }
        } else {
            origin_v[0 as libc::c_int
                as usize] = (*entity).s.origin[0 as libc::c_int as usize];
            origin_v[1 as libc::c_int
                as usize] = (*entity).s.origin[1 as libc::c_int as usize];
            origin_v[2 as libc::c_int
                as usize] = (*entity).s.origin[2 as libc::c_int as usize];
        }
    }
    MSG_WriteByte(&mut sv.multicast, svc_sound as libc::c_int);
    MSG_WriteByte(&mut sv.multicast, flags);
    MSG_WriteByte(&mut sv.multicast, soundindex);
    if flags & (1 as libc::c_int) << 0 as libc::c_int != 0 {
        MSG_WriteByte(
            &mut sv.multicast,
            (volume * 255 as libc::c_int as libc::c_float) as libc::c_int,
        );
    }
    if flags & (1 as libc::c_int) << 1 as libc::c_int != 0 {
        MSG_WriteByte(
            &mut sv.multicast,
            (attenuation * 64 as libc::c_int as libc::c_float) as libc::c_int,
        );
    }
    if flags & (1 as libc::c_int) << 4 as libc::c_int != 0 {
        MSG_WriteByte(
            &mut sv.multicast,
            (timeofs * 1000 as libc::c_int as libc::c_float) as libc::c_int,
        );
    }
    if flags & (1 as libc::c_int) << 3 as libc::c_int != 0 {
        MSG_WriteShort(&mut sv.multicast, sendchan);
    }
    if flags & (1 as libc::c_int) << 2 as libc::c_int != 0 {
        MSG_WritePos(&mut sv.multicast, origin);
    }
    if attenuation == 0 as libc::c_int as libc::c_float {
        use_phs = false_0;
    }
    if channel & 16 as libc::c_int != 0 {
        if use_phs as u64 != 0 {
            SV_Multicast(origin, MULTICAST_PHS_R);
        } else {
            SV_Multicast(origin, MULTICAST_ALL_R);
        }
    } else if use_phs as u64 != 0 {
        SV_Multicast(origin, MULTICAST_PHS);
    } else {
        SV_Multicast(origin, MULTICAST_ALL);
    };
}
#[no_mangle]
pub unsafe extern "C" fn SV_SendClientDatagram(mut client: *mut client_t) -> qboolean {
    let mut msg_buf: [byte; 1400] = [0; 1400];
    let mut msg: sizebuf_t = sizebuf_t {
        allowoverflow: false_0,
        overflowed: false_0,
        data: 0 as *const byte as *mut byte,
        maxsize: 0,
        cursize: 0,
        readcount: 0,
    };
    SV_BuildClientFrame(client);
    SZ_Init(
        &mut msg,
        msg_buf.as_mut_ptr(),
        ::std::mem::size_of::<[byte; 1400]>() as libc::c_ulong as libc::c_int,
    );
    msg.allowoverflow = true_0;
    SV_WriteFrameToClient(client, &mut msg);
    if (*client).datagram.overflowed as u64 != 0 {
        Com_Printf(
            b"WARNING: datagram overflowed for %s\n\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            ((*client).name).as_mut_ptr(),
        );
    } else {
        SZ_Write(
            &mut msg,
            (*client).datagram.data as *mut libc::c_void,
            (*client).datagram.cursize,
        );
    }
    SZ_Clear(&mut (*client).datagram);
    if msg.overflowed as u64 != 0 {
        Com_Printf(
            b"WARNING: msg overflowed for %s\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            ((*client).name).as_mut_ptr(),
        );
        SZ_Clear(&mut msg);
    }
    Netchan_Transmit(&mut (*client).netchan, msg.cursize, msg.data);
    (*client).message_size[(sv.framenum % 10 as libc::c_int) as usize] = msg.cursize;
    return true_0;
}
#[no_mangle]
pub unsafe extern "C" fn SV_DemoCompleted() {
    if !(sv.demofile).is_null() {
        fclose(sv.demofile);
        sv.demofile = 0 as *mut FILE;
    }
    SV_Nextserver();
}
#[no_mangle]
pub unsafe extern "C" fn SV_RateDrop(mut c: *mut client_t) -> qboolean {
    let mut total: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    if (*c).netchan.remote_address.type_0 as libc::c_uint
        == NA_LOOPBACK as libc::c_int as libc::c_uint
    {
        return false_0;
    }
    total = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < 10 as libc::c_int {
        total += (*c).message_size[i as usize];
        i += 1;
    }
    if total > (*c).rate {
        let ref mut fresh0 = (*c).surpressCount;
        *fresh0 += 1;
        (*c).message_size[(sv.framenum % 10 as libc::c_int) as usize] = 0 as libc::c_int;
        return true_0;
    }
    return false_0;
}
#[no_mangle]
pub unsafe extern "C" fn SV_SendClientMessages() {
    let mut i: libc::c_int = 0;
    let mut c: *mut client_t = 0 as *mut client_t;
    let mut msglen: libc::c_int = 0;
    let mut msgbuf: [byte; 1400] = [0; 1400];
    let mut r: libc::c_int = 0;
    msglen = 0 as libc::c_int;
    if sv.state as libc::c_uint == ss_demo as libc::c_int as libc::c_uint
        && !(sv.demofile).is_null()
    {
        if (*sv_paused).value != 0. {
            msglen = 0 as libc::c_int;
        } else {
            r = fread(
                &mut msglen as *mut libc::c_int as *mut libc::c_void,
                4 as libc::c_int as libc::c_ulong,
                1 as libc::c_int as libc::c_ulong,
                sv.demofile,
            ) as libc::c_int;
            if r != 1 as libc::c_int {
                SV_DemoCompleted();
                return;
            }
            msglen = LittleLong(msglen);
            if msglen == -(1 as libc::c_int) {
                SV_DemoCompleted();
                return;
            }
            if msglen > 1400 as libc::c_int {
                Com_Error(
                    1 as libc::c_int,
                    b"SV_SendClientMessages: msglen > MAX_MSGLEN\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                );
            }
            r = fread(
                msgbuf.as_mut_ptr() as *mut libc::c_void,
                msglen as libc::c_ulong,
                1 as libc::c_int as libc::c_ulong,
                sv.demofile,
            ) as libc::c_int;
            if r != 1 as libc::c_int {
                SV_DemoCompleted();
                return;
            }
        }
    }
    i = 0 as libc::c_int;
    c = svs.clients;
    while (i as libc::c_float) < (*maxclients).value {
        if !((*c).state as u64 == 0) {
            if (*c).netchan.message.overflowed as u64 != 0 {
                SZ_Clear(&mut (*c).netchan.message);
                SZ_Clear(&mut (*c).datagram);
                SV_BroadcastPrintf(
                    2 as libc::c_int,
                    b"%s overflowed\n\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    ((*c).name).as_mut_ptr(),
                );
                SV_DropClient(c);
            }
            if sv.state as libc::c_uint == ss_cinematic as libc::c_int as libc::c_uint
                || sv.state as libc::c_uint == ss_demo as libc::c_int as libc::c_uint
                || sv.state as libc::c_uint == ss_pic as libc::c_int as libc::c_uint
            {
                Netchan_Transmit(&mut (*c).netchan, msglen, msgbuf.as_mut_ptr());
            } else if (*c).state as libc::c_uint
                == cs_spawned as libc::c_int as libc::c_uint
            {
                if !(SV_RateDrop(c) as u64 != 0) {
                    SV_SendClientDatagram(c);
                }
            } else if (*c).netchan.message.cursize != 0
                || curtime - (*c).netchan.last_sent > 1000 as libc::c_int
            {
                Netchan_Transmit(&mut (*c).netchan, 0 as libc::c_int, 0 as *mut byte);
            }
        }
        i += 1;
        c = c.offset(1);
    }
}
