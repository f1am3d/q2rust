#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(extern_types, register_tool)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn fwrite(
        _: *const libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    static mut ge: *mut game_export_t;
    static mut maxclients: *mut cvar_t;
    static mut sv: server_t;
    static mut svs: server_static_t;
    fn Com_Error(code: libc::c_int, fmt: *mut libc::c_char, _: ...);
    fn Com_DPrintf(fmt: *mut libc::c_char, _: ...);
    fn CM_HeadnodeVisible(headnode: libc::c_int, visbits: *mut byte) -> qboolean;
    fn CM_WriteAreaBits(buffer: *mut byte, area: libc::c_int) -> libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn VectorLength(v: *mut vec_t) -> vec_t;
    fn LittleLong(l: libc::c_int) -> libc::c_int;
    fn SZ_Init(buf: *mut sizebuf_t, data: *mut byte, length: libc::c_int);
    fn SZ_Clear(buf: *mut sizebuf_t);
    fn SZ_Write(buf: *mut sizebuf_t, data: *mut libc::c_void, length: libc::c_int);
    fn MSG_WriteChar(sb: *mut sizebuf_t, c: libc::c_int);
    fn MSG_WriteByte(sb: *mut sizebuf_t, c: libc::c_int);
    fn MSG_WriteShort(sb: *mut sizebuf_t, c: libc::c_int);
    fn MSG_WriteLong(sb: *mut sizebuf_t, c: libc::c_int);
    fn MSG_WriteAngle16(sb: *mut sizebuf_t, f: libc::c_float);
    fn MSG_WriteDeltaEntity(
        from: *mut entity_state_s,
        to: *mut entity_state_s,
        msg: *mut sizebuf_t,
        force: qboolean,
        newentity: qboolean,
    );
    fn CM_NumClusters() -> libc::c_int;
    fn CM_ClusterPVS(cluster: libc::c_int) -> *mut byte;
    fn CM_ClusterPHS(cluster: libc::c_int) -> *mut byte;
    fn CM_PointLeafnum(p: *mut vec_t) -> libc::c_int;
    fn CM_BoxLeafnums(
        mins: *mut vec_t,
        maxs: *mut vec_t,
        list: *mut libc::c_int,
        listsize: libc::c_int,
        topnode: *mut libc::c_int,
    ) -> libc::c_int;
    fn CM_LeafCluster(leafnum: libc::c_int) -> libc::c_int;
    fn CM_LeafArea(leafnum: libc::c_int) -> libc::c_int;
    fn CM_AreasConnected(area1: libc::c_int, area2: libc::c_int) -> qboolean;
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
pub unsafe extern "C" fn SV_EmitPacketEntities(
    mut from: *mut client_frame_t,
    mut to: *mut client_frame_t,
    mut msg: *mut sizebuf_t,
) {
    let mut oldent: *mut entity_state_t = 0 as *mut entity_state_t;
    let mut newent: *mut entity_state_t = 0 as *mut entity_state_t;
    let mut oldindex: libc::c_int = 0;
    let mut newindex: libc::c_int = 0;
    let mut oldnum: libc::c_int = 0;
    let mut newnum: libc::c_int = 0;
    let mut from_num_entities: libc::c_int = 0;
    let mut bits: libc::c_int = 0;
    MSG_WriteByte(msg, svc_packetentities as libc::c_int);
    if from.is_null() {
        from_num_entities = 0 as libc::c_int;
    } else {
        from_num_entities = (*from).num_entities;
    }
    newindex = 0 as libc::c_int;
    oldindex = 0 as libc::c_int;
    while newindex < (*to).num_entities || oldindex < from_num_entities {
        if newindex >= (*to).num_entities {
            newnum = 9999 as libc::c_int;
        } else {
            newent = &mut *(svs.client_entities)
                .offset(
                    (((*to).first_entity + newindex) % svs.num_client_entities) as isize,
                ) as *mut entity_state_t;
            newnum = (*newent).number;
        }
        if oldindex >= from_num_entities {
            oldnum = 9999 as libc::c_int;
        } else {
            oldent = &mut *(svs.client_entities)
                .offset(
                    (((*from).first_entity + oldindex) % svs.num_client_entities)
                        as isize,
                ) as *mut entity_state_t;
            oldnum = (*oldent).number;
        }
        if newnum == oldnum {
            MSG_WriteDeltaEntity(
                oldent,
                newent,
                msg,
                false_0,
                ((*newent).number as libc::c_float <= (*maxclients).value) as libc::c_int
                    as qboolean,
            );
            oldindex += 1;
            newindex += 1;
        } else if newnum < oldnum {
            MSG_WriteDeltaEntity(
                &mut *(sv.baselines).as_mut_ptr().offset(newnum as isize),
                newent,
                msg,
                true_0,
                true_0,
            );
            newindex += 1;
        } else {
            if !(newnum > oldnum) {
                continue;
            }
            bits = (1 as libc::c_int) << 6 as libc::c_int;
            if oldnum >= 256 as libc::c_int {
                bits
                    |= (1 as libc::c_int) << 8 as libc::c_int
                        | (1 as libc::c_int) << 7 as libc::c_int;
            }
            MSG_WriteByte(msg, bits & 255 as libc::c_int);
            if bits & 0xff00 as libc::c_int != 0 {
                MSG_WriteByte(msg, bits >> 8 as libc::c_int & 255 as libc::c_int);
            }
            if bits & (1 as libc::c_int) << 8 as libc::c_int != 0 {
                MSG_WriteShort(msg, oldnum);
            } else {
                MSG_WriteByte(msg, oldnum);
            }
            oldindex += 1;
        }
    }
    MSG_WriteShort(msg, 0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn SV_WritePlayerstateToClient(
    mut from: *mut client_frame_t,
    mut to: *mut client_frame_t,
    mut msg: *mut sizebuf_t,
) {
    let mut i: libc::c_int = 0;
    let mut pflags: libc::c_int = 0;
    let mut ps: *mut player_state_t = 0 as *mut player_state_t;
    let mut ops: *mut player_state_t = 0 as *mut player_state_t;
    let mut dummy: player_state_t = player_state_t {
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
    };
    let mut statbits: libc::c_int = 0;
    ps = &mut (*to).ps;
    if from.is_null() {
        memset(
            &mut dummy as *mut player_state_t as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<player_state_t>() as libc::c_ulong,
        );
        ops = &mut dummy;
    } else {
        ops = &mut (*from).ps;
    }
    pflags = 0 as libc::c_int;
    if (*ps).pmove.pm_type as libc::c_uint != (*ops).pmove.pm_type as libc::c_uint {
        pflags |= (1 as libc::c_int) << 0 as libc::c_int;
    }
    if (*ps).pmove.origin[0 as libc::c_int as usize] as libc::c_int
        != (*ops).pmove.origin[0 as libc::c_int as usize] as libc::c_int
        || (*ps).pmove.origin[1 as libc::c_int as usize] as libc::c_int
            != (*ops).pmove.origin[1 as libc::c_int as usize] as libc::c_int
        || (*ps).pmove.origin[2 as libc::c_int as usize] as libc::c_int
            != (*ops).pmove.origin[2 as libc::c_int as usize] as libc::c_int
    {
        pflags |= (1 as libc::c_int) << 1 as libc::c_int;
    }
    if (*ps).pmove.velocity[0 as libc::c_int as usize] as libc::c_int
        != (*ops).pmove.velocity[0 as libc::c_int as usize] as libc::c_int
        || (*ps).pmove.velocity[1 as libc::c_int as usize] as libc::c_int
            != (*ops).pmove.velocity[1 as libc::c_int as usize] as libc::c_int
        || (*ps).pmove.velocity[2 as libc::c_int as usize] as libc::c_int
            != (*ops).pmove.velocity[2 as libc::c_int as usize] as libc::c_int
    {
        pflags |= (1 as libc::c_int) << 2 as libc::c_int;
    }
    if (*ps).pmove.pm_time as libc::c_int != (*ops).pmove.pm_time as libc::c_int {
        pflags |= (1 as libc::c_int) << 3 as libc::c_int;
    }
    if (*ps).pmove.pm_flags as libc::c_int != (*ops).pmove.pm_flags as libc::c_int {
        pflags |= (1 as libc::c_int) << 4 as libc::c_int;
    }
    if (*ps).pmove.gravity as libc::c_int != (*ops).pmove.gravity as libc::c_int {
        pflags |= (1 as libc::c_int) << 5 as libc::c_int;
    }
    if (*ps).pmove.delta_angles[0 as libc::c_int as usize] as libc::c_int
        != (*ops).pmove.delta_angles[0 as libc::c_int as usize] as libc::c_int
        || (*ps).pmove.delta_angles[1 as libc::c_int as usize] as libc::c_int
            != (*ops).pmove.delta_angles[1 as libc::c_int as usize] as libc::c_int
        || (*ps).pmove.delta_angles[2 as libc::c_int as usize] as libc::c_int
            != (*ops).pmove.delta_angles[2 as libc::c_int as usize] as libc::c_int
    {
        pflags |= (1 as libc::c_int) << 6 as libc::c_int;
    }
    if (*ps).viewoffset[0 as libc::c_int as usize]
        != (*ops).viewoffset[0 as libc::c_int as usize]
        || (*ps).viewoffset[1 as libc::c_int as usize]
            != (*ops).viewoffset[1 as libc::c_int as usize]
        || (*ps).viewoffset[2 as libc::c_int as usize]
            != (*ops).viewoffset[2 as libc::c_int as usize]
    {
        pflags |= (1 as libc::c_int) << 7 as libc::c_int;
    }
    if (*ps).viewangles[0 as libc::c_int as usize]
        != (*ops).viewangles[0 as libc::c_int as usize]
        || (*ps).viewangles[1 as libc::c_int as usize]
            != (*ops).viewangles[1 as libc::c_int as usize]
        || (*ps).viewangles[2 as libc::c_int as usize]
            != (*ops).viewangles[2 as libc::c_int as usize]
    {
        pflags |= (1 as libc::c_int) << 8 as libc::c_int;
    }
    if (*ps).kick_angles[0 as libc::c_int as usize]
        != (*ops).kick_angles[0 as libc::c_int as usize]
        || (*ps).kick_angles[1 as libc::c_int as usize]
            != (*ops).kick_angles[1 as libc::c_int as usize]
        || (*ps).kick_angles[2 as libc::c_int as usize]
            != (*ops).kick_angles[2 as libc::c_int as usize]
    {
        pflags |= (1 as libc::c_int) << 9 as libc::c_int;
    }
    if (*ps).blend[0 as libc::c_int as usize] != (*ops).blend[0 as libc::c_int as usize]
        || (*ps).blend[1 as libc::c_int as usize]
            != (*ops).blend[1 as libc::c_int as usize]
        || (*ps).blend[2 as libc::c_int as usize]
            != (*ops).blend[2 as libc::c_int as usize]
        || (*ps).blend[3 as libc::c_int as usize]
            != (*ops).blend[3 as libc::c_int as usize]
    {
        pflags |= (1 as libc::c_int) << 10 as libc::c_int;
    }
    if (*ps).fov != (*ops).fov {
        pflags |= (1 as libc::c_int) << 11 as libc::c_int;
    }
    if (*ps).rdflags != (*ops).rdflags {
        pflags |= (1 as libc::c_int) << 14 as libc::c_int;
    }
    if (*ps).gunframe != (*ops).gunframe {
        pflags |= (1 as libc::c_int) << 13 as libc::c_int;
    }
    pflags |= (1 as libc::c_int) << 12 as libc::c_int;
    MSG_WriteByte(msg, svc_playerinfo as libc::c_int);
    MSG_WriteShort(msg, pflags);
    if pflags & (1 as libc::c_int) << 0 as libc::c_int != 0 {
        MSG_WriteByte(msg, (*ps).pmove.pm_type as libc::c_int);
    }
    if pflags & (1 as libc::c_int) << 1 as libc::c_int != 0 {
        MSG_WriteShort(
            msg,
            (*ps).pmove.origin[0 as libc::c_int as usize] as libc::c_int,
        );
        MSG_WriteShort(
            msg,
            (*ps).pmove.origin[1 as libc::c_int as usize] as libc::c_int,
        );
        MSG_WriteShort(
            msg,
            (*ps).pmove.origin[2 as libc::c_int as usize] as libc::c_int,
        );
    }
    if pflags & (1 as libc::c_int) << 2 as libc::c_int != 0 {
        MSG_WriteShort(
            msg,
            (*ps).pmove.velocity[0 as libc::c_int as usize] as libc::c_int,
        );
        MSG_WriteShort(
            msg,
            (*ps).pmove.velocity[1 as libc::c_int as usize] as libc::c_int,
        );
        MSG_WriteShort(
            msg,
            (*ps).pmove.velocity[2 as libc::c_int as usize] as libc::c_int,
        );
    }
    if pflags & (1 as libc::c_int) << 3 as libc::c_int != 0 {
        MSG_WriteByte(msg, (*ps).pmove.pm_time as libc::c_int);
    }
    if pflags & (1 as libc::c_int) << 4 as libc::c_int != 0 {
        MSG_WriteByte(msg, (*ps).pmove.pm_flags as libc::c_int);
    }
    if pflags & (1 as libc::c_int) << 5 as libc::c_int != 0 {
        MSG_WriteShort(msg, (*ps).pmove.gravity as libc::c_int);
    }
    if pflags & (1 as libc::c_int) << 6 as libc::c_int != 0 {
        MSG_WriteShort(
            msg,
            (*ps).pmove.delta_angles[0 as libc::c_int as usize] as libc::c_int,
        );
        MSG_WriteShort(
            msg,
            (*ps).pmove.delta_angles[1 as libc::c_int as usize] as libc::c_int,
        );
        MSG_WriteShort(
            msg,
            (*ps).pmove.delta_angles[2 as libc::c_int as usize] as libc::c_int,
        );
    }
    if pflags & (1 as libc::c_int) << 7 as libc::c_int != 0 {
        MSG_WriteChar(
            msg,
            ((*ps).viewoffset[0 as libc::c_int as usize]
                * 4 as libc::c_int as libc::c_float) as libc::c_int,
        );
        MSG_WriteChar(
            msg,
            ((*ps).viewoffset[1 as libc::c_int as usize]
                * 4 as libc::c_int as libc::c_float) as libc::c_int,
        );
        MSG_WriteChar(
            msg,
            ((*ps).viewoffset[2 as libc::c_int as usize]
                * 4 as libc::c_int as libc::c_float) as libc::c_int,
        );
    }
    if pflags & (1 as libc::c_int) << 8 as libc::c_int != 0 {
        MSG_WriteAngle16(msg, (*ps).viewangles[0 as libc::c_int as usize]);
        MSG_WriteAngle16(msg, (*ps).viewangles[1 as libc::c_int as usize]);
        MSG_WriteAngle16(msg, (*ps).viewangles[2 as libc::c_int as usize]);
    }
    if pflags & (1 as libc::c_int) << 9 as libc::c_int != 0 {
        MSG_WriteChar(
            msg,
            ((*ps).kick_angles[0 as libc::c_int as usize]
                * 4 as libc::c_int as libc::c_float) as libc::c_int,
        );
        MSG_WriteChar(
            msg,
            ((*ps).kick_angles[1 as libc::c_int as usize]
                * 4 as libc::c_int as libc::c_float) as libc::c_int,
        );
        MSG_WriteChar(
            msg,
            ((*ps).kick_angles[2 as libc::c_int as usize]
                * 4 as libc::c_int as libc::c_float) as libc::c_int,
        );
    }
    if pflags & (1 as libc::c_int) << 12 as libc::c_int != 0 {
        MSG_WriteByte(msg, (*ps).gunindex);
    }
    if pflags & (1 as libc::c_int) << 13 as libc::c_int != 0 {
        MSG_WriteByte(msg, (*ps).gunframe);
        MSG_WriteChar(
            msg,
            ((*ps).gunoffset[0 as libc::c_int as usize]
                * 4 as libc::c_int as libc::c_float) as libc::c_int,
        );
        MSG_WriteChar(
            msg,
            ((*ps).gunoffset[1 as libc::c_int as usize]
                * 4 as libc::c_int as libc::c_float) as libc::c_int,
        );
        MSG_WriteChar(
            msg,
            ((*ps).gunoffset[2 as libc::c_int as usize]
                * 4 as libc::c_int as libc::c_float) as libc::c_int,
        );
        MSG_WriteChar(
            msg,
            ((*ps).gunangles[0 as libc::c_int as usize]
                * 4 as libc::c_int as libc::c_float) as libc::c_int,
        );
        MSG_WriteChar(
            msg,
            ((*ps).gunangles[1 as libc::c_int as usize]
                * 4 as libc::c_int as libc::c_float) as libc::c_int,
        );
        MSG_WriteChar(
            msg,
            ((*ps).gunangles[2 as libc::c_int as usize]
                * 4 as libc::c_int as libc::c_float) as libc::c_int,
        );
    }
    if pflags & (1 as libc::c_int) << 10 as libc::c_int != 0 {
        MSG_WriteByte(
            msg,
            ((*ps).blend[0 as libc::c_int as usize]
                * 255 as libc::c_int as libc::c_float) as libc::c_int,
        );
        MSG_WriteByte(
            msg,
            ((*ps).blend[1 as libc::c_int as usize]
                * 255 as libc::c_int as libc::c_float) as libc::c_int,
        );
        MSG_WriteByte(
            msg,
            ((*ps).blend[2 as libc::c_int as usize]
                * 255 as libc::c_int as libc::c_float) as libc::c_int,
        );
        MSG_WriteByte(
            msg,
            ((*ps).blend[3 as libc::c_int as usize]
                * 255 as libc::c_int as libc::c_float) as libc::c_int,
        );
    }
    if pflags & (1 as libc::c_int) << 11 as libc::c_int != 0 {
        MSG_WriteByte(msg, (*ps).fov as libc::c_int);
    }
    if pflags & (1 as libc::c_int) << 14 as libc::c_int != 0 {
        MSG_WriteByte(msg, (*ps).rdflags);
    }
    statbits = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        if (*ps).stats[i as usize] as libc::c_int
            != (*ops).stats[i as usize] as libc::c_int
        {
            statbits |= (1 as libc::c_int) << i;
        }
        i += 1;
    }
    MSG_WriteLong(msg, statbits);
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        if statbits & (1 as libc::c_int) << i != 0 {
            MSG_WriteShort(msg, (*ps).stats[i as usize] as libc::c_int);
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn SV_WriteFrameToClient(
    mut client: *mut client_t,
    mut msg: *mut sizebuf_t,
) {
    let mut frame: *mut client_frame_t = 0 as *mut client_frame_t;
    let mut oldframe: *mut client_frame_t = 0 as *mut client_frame_t;
    let mut lastframe: libc::c_int = 0;
    frame = &mut *((*client).frames)
        .as_mut_ptr()
        .offset((sv.framenum & 16 as libc::c_int - 1 as libc::c_int) as isize)
        as *mut client_frame_t;
    if (*client).lastframe <= 0 as libc::c_int {
        oldframe = 0 as *mut client_frame_t;
        lastframe = -(1 as libc::c_int);
    } else if sv.framenum - (*client).lastframe >= 16 as libc::c_int - 3 as libc::c_int {
        oldframe = 0 as *mut client_frame_t;
        lastframe = -(1 as libc::c_int);
    } else {
        oldframe = &mut *((*client).frames)
            .as_mut_ptr()
            .offset(
                ((*client).lastframe & 16 as libc::c_int - 1 as libc::c_int) as isize,
            ) as *mut client_frame_t;
        lastframe = (*client).lastframe;
    }
    MSG_WriteByte(msg, svc_frame as libc::c_int);
    MSG_WriteLong(msg, sv.framenum);
    MSG_WriteLong(msg, lastframe);
    MSG_WriteByte(msg, (*client).surpressCount);
    (*client).surpressCount = 0 as libc::c_int;
    MSG_WriteByte(msg, (*frame).areabytes);
    SZ_Write(
        msg,
        ((*frame).areabits).as_mut_ptr() as *mut libc::c_void,
        (*frame).areabytes,
    );
    SV_WritePlayerstateToClient(oldframe, frame, msg);
    SV_EmitPacketEntities(oldframe, frame, msg);
}
#[no_mangle]
pub static mut fatpvs: [byte; 8192] = [0; 8192];
#[no_mangle]
pub unsafe extern "C" fn SV_FatPVS(mut org: *mut vec_t) {
    let mut leafs: [libc::c_int; 64] = [0; 64];
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    let mut longs: libc::c_int = 0;
    let mut src: *mut byte = 0 as *mut byte;
    let mut mins: vec3_t = [0.; 3];
    let mut maxs: vec3_t = [0.; 3];
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        mins[i as usize] = *org.offset(i as isize) - 8 as libc::c_int as libc::c_float;
        maxs[i as usize] = *org.offset(i as isize) + 8 as libc::c_int as libc::c_float;
        i += 1;
    }
    count = CM_BoxLeafnums(
        mins.as_mut_ptr(),
        maxs.as_mut_ptr(),
        leafs.as_mut_ptr(),
        64 as libc::c_int,
        0 as *mut libc::c_int,
    );
    if count < 1 as libc::c_int {
        Com_Error(
            0 as libc::c_int,
            b"SV_FatPVS: count < 1\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    longs = CM_NumClusters() + 31 as libc::c_int >> 5 as libc::c_int;
    i = 0 as libc::c_int;
    while i < count {
        leafs[i as usize] = CM_LeafCluster(leafs[i as usize]);
        i += 1;
    }
    memcpy(
        fatpvs.as_mut_ptr() as *mut libc::c_void,
        CM_ClusterPVS(leafs[0 as libc::c_int as usize]) as *const libc::c_void,
        (longs << 2 as libc::c_int) as libc::c_ulong,
    );
    i = 1 as libc::c_int;
    while i < count {
        j = 0 as libc::c_int;
        while j < i {
            if leafs[i as usize] == leafs[j as usize] {
                break;
            }
            j += 1;
        }
        if !(j != i) {
            src = CM_ClusterPVS(leafs[i as usize]);
            j = 0 as libc::c_int;
            while j < longs {
                *(fatpvs.as_mut_ptr() as *mut libc::c_long).offset(j as isize)
                    |= *(src as *mut libc::c_long).offset(j as isize);
                j += 1;
            }
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn SV_BuildClientFrame(mut client: *mut client_t) {
    let mut e: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut org: vec3_t = [0.; 3];
    let mut ent: *mut edict_t = 0 as *mut edict_t;
    let mut clent: *mut edict_t = 0 as *mut edict_t;
    let mut frame: *mut client_frame_t = 0 as *mut client_frame_t;
    let mut state: *mut entity_state_t = 0 as *mut entity_state_t;
    let mut l: libc::c_int = 0;
    let mut clientarea: libc::c_int = 0;
    let mut clientcluster: libc::c_int = 0;
    let mut leafnum: libc::c_int = 0;
    let mut c_fullsend: libc::c_int = 0;
    let mut clientphs: *mut byte = 0 as *mut byte;
    let mut bitvector: *mut byte = 0 as *mut byte;
    clent = (*client).edict;
    if ((*clent).client).is_null() {
        return;
    }
    frame = &mut *((*client).frames)
        .as_mut_ptr()
        .offset((sv.framenum & 16 as libc::c_int - 1 as libc::c_int) as isize)
        as *mut client_frame_t;
    (*frame).senttime = svs.realtime;
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        org[i
            as usize] = ((*(*clent).client).ps.pmove.origin[i as usize] as libc::c_int
            as libc::c_double * 0.125f64
            + (*(*clent).client).ps.viewoffset[i as usize] as libc::c_double) as vec_t;
        i += 1;
    }
    leafnum = CM_PointLeafnum(org.as_mut_ptr());
    clientarea = CM_LeafArea(leafnum);
    clientcluster = CM_LeafCluster(leafnum);
    (*frame).areabytes = CM_WriteAreaBits(((*frame).areabits).as_mut_ptr(), clientarea);
    (*frame).ps = (*(*clent).client).ps;
    SV_FatPVS(org.as_mut_ptr());
    clientphs = CM_ClusterPHS(clientcluster);
    (*frame).num_entities = 0 as libc::c_int;
    (*frame).first_entity = svs.next_client_entities;
    c_fullsend = 0 as libc::c_int;
    let mut current_block_40: u64;
    e = 1 as libc::c_int;
    while e < (*ge).num_edicts {
        ent = ((*ge).edicts as *mut byte).offset(((*ge).edict_size * e) as isize)
            as *mut edict_t;
        if !((*ent).svflags & 0x1 as libc::c_int != 0) {
            if !((*ent).s.modelindex == 0 && (*ent).s.effects == 0 && (*ent).s.sound == 0
                && (*ent).s.event == 0)
            {
                if ent != clent {
                    if CM_AreasConnected(clientarea, (*ent).areanum) as u64 == 0 {
                        if (*ent).areanum2 == 0
                            || CM_AreasConnected(clientarea, (*ent).areanum2) as u64 == 0
                        {
                            current_block_40 = 14401909646449704462;
                        } else {
                            current_block_40 = 11459959175219260272;
                        }
                    } else {
                        current_block_40 = 11459959175219260272;
                    }
                    match current_block_40 {
                        14401909646449704462 => {}
                        _ => {
                            if (*ent).s.renderfx & 128 as libc::c_int != 0 {
                                l = (*ent).clusternums[0 as libc::c_int as usize];
                                if *clientphs.offset((l >> 3 as libc::c_int) as isize)
                                    as libc::c_int
                                    & (1 as libc::c_int) << (l & 7 as libc::c_int) == 0
                                {
                                    current_block_40 = 14401909646449704462;
                                } else {
                                    current_block_40 = 10753070352654377903;
                                }
                            } else {
                                if (*ent).s.sound != 0 {
                                    bitvector = fatpvs.as_mut_ptr();
                                } else {
                                    bitvector = fatpvs.as_mut_ptr();
                                }
                                if (*ent).num_clusters == -(1 as libc::c_int) {
                                    if CM_HeadnodeVisible((*ent).headnode, bitvector) as u64
                                        == 0
                                    {
                                        current_block_40 = 14401909646449704462;
                                    } else {
                                        c_fullsend += 1;
                                        current_block_40 = 1924505913685386279;
                                    }
                                } else {
                                    i = 0 as libc::c_int;
                                    while i < (*ent).num_clusters {
                                        l = (*ent).clusternums[i as usize];
                                        if *bitvector.offset((l >> 3 as libc::c_int) as isize)
                                            as libc::c_int
                                            & (1 as libc::c_int) << (l & 7 as libc::c_int) != 0
                                        {
                                            break;
                                        }
                                        i += 1;
                                    }
                                    if i == (*ent).num_clusters {
                                        current_block_40 = 14401909646449704462;
                                    } else {
                                        current_block_40 = 1924505913685386279;
                                    }
                                }
                                match current_block_40 {
                                    14401909646449704462 => {}
                                    _ => {
                                        if (*ent).s.modelindex == 0 {
                                            let mut delta: vec3_t = [0.; 3];
                                            let mut len: libc::c_float = 0.;
                                            delta[0 as libc::c_int
                                                as usize] = org[0 as libc::c_int as usize]
                                                - (*ent).s.origin[0 as libc::c_int as usize];
                                            delta[1 as libc::c_int
                                                as usize] = org[1 as libc::c_int as usize]
                                                - (*ent).s.origin[1 as libc::c_int as usize];
                                            delta[2 as libc::c_int
                                                as usize] = org[2 as libc::c_int as usize]
                                                - (*ent).s.origin[2 as libc::c_int as usize];
                                            len = VectorLength(delta.as_mut_ptr());
                                            if len > 400 as libc::c_int as libc::c_float {
                                                current_block_40 = 14401909646449704462;
                                            } else {
                                                current_block_40 = 10753070352654377903;
                                            }
                                        } else {
                                            current_block_40 = 10753070352654377903;
                                        }
                                    }
                                }
                            }
                        }
                    }
                } else {
                    current_block_40 = 10753070352654377903;
                }
                match current_block_40 {
                    14401909646449704462 => {}
                    _ => {
                        state = &mut *(svs.client_entities)
                            .offset(
                                (svs.next_client_entities % svs.num_client_entities)
                                    as isize,
                            ) as *mut entity_state_t;
                        if (*ent).s.number != e {
                            Com_DPrintf(
                                b"FIXING ENT->S.NUMBER!!!\n\0" as *const u8
                                    as *const libc::c_char as *mut libc::c_char,
                            );
                            (*ent).s.number = e;
                        }
                        *state = (*ent).s;
                        if (*ent).owner == (*client).edict {
                            (*state).solid = 0 as libc::c_int;
                        }
                        svs.next_client_entities += 1;
                        let ref mut fresh0 = (*frame).num_entities;
                        *fresh0 += 1;
                    }
                }
            }
        }
        e += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn SV_RecordDemoMessage() {
    let mut e: libc::c_int = 0;
    let mut ent: *mut edict_t = 0 as *mut edict_t;
    let mut nostate: entity_state_t = entity_state_t {
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
    let mut buf: sizebuf_t = sizebuf_t {
        allowoverflow: false_0,
        overflowed: false_0,
        data: 0 as *const byte as *mut byte,
        maxsize: 0,
        cursize: 0,
        readcount: 0,
    };
    let mut buf_data: [byte; 32768] = [0; 32768];
    let mut len: libc::c_int = 0;
    if (svs.demofile).is_null() {
        return;
    }
    memset(
        &mut nostate as *mut entity_state_t as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<entity_state_t>() as libc::c_ulong,
    );
    SZ_Init(
        &mut buf,
        buf_data.as_mut_ptr(),
        ::std::mem::size_of::<[byte; 32768]>() as libc::c_ulong as libc::c_int,
    );
    MSG_WriteByte(&mut buf, svc_frame as libc::c_int);
    MSG_WriteLong(&mut buf, sv.framenum);
    MSG_WriteByte(&mut buf, svc_packetentities as libc::c_int);
    e = 1 as libc::c_int;
    ent = ((*ge).edicts as *mut byte).offset(((*ge).edict_size * e) as isize)
        as *mut edict_t;
    while e < (*ge).num_edicts {
        if (*ent).inuse as libc::c_uint != 0 && (*ent).s.number != 0
            && ((*ent).s.modelindex != 0 || (*ent).s.effects != 0 || (*ent).s.sound != 0
                || (*ent).s.event != 0) && (*ent).svflags & 0x1 as libc::c_int == 0
        {
            MSG_WriteDeltaEntity(&mut nostate, &mut (*ent).s, &mut buf, false_0, true_0);
        }
        e += 1;
        ent = ((*ge).edicts as *mut byte).offset(((*ge).edict_size * e) as isize)
            as *mut edict_t;
    }
    MSG_WriteShort(&mut buf, 0 as libc::c_int);
    SZ_Write(
        &mut buf,
        svs.demo_multicast.data as *mut libc::c_void,
        svs.demo_multicast.cursize,
    );
    SZ_Clear(&mut svs.demo_multicast);
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
