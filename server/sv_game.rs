#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(c_variadic, extern_types, register_tool)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn SV_UnlinkEdict(ent: *mut edict_t);
    fn SV_AreaEdicts(
        mins: *mut vec_t,
        maxs: *mut vec_t,
        list: *mut *mut edict_t,
        maxcount: libc::c_int,
        areatype: libc::c_int,
    ) -> libc::c_int;
    fn SV_Trace(
        start: *mut vec_t,
        mins: *mut vec_t,
        maxs: *mut vec_t,
        end: *mut vec_t,
        passedict: *mut edict_t,
        contentmask: libc::c_int,
    ) -> trace_t;
    fn SV_PointContents(p: *mut vec_t) -> libc::c_int;
    fn SV_LinkEdict(ent: *mut edict_t);
    fn SV_BroadcastPrintf(level: libc::c_int, fmt: *mut libc::c_char, _: ...);
    fn SV_ClientPrintf(
        cl: *mut client_t,
        level: libc::c_int,
        fmt: *mut libc::c_char,
        _: ...
    );
    fn SV_StartSound(
        origin: *mut vec_t,
        entity: *mut edict_t,
        channel: libc::c_int,
        soundindex: libc::c_int,
        volume: libc::c_float,
        attenuation: libc::c_float,
        timeofs: libc::c_float,
    );
    fn SV_Multicast(origin: *mut vec_t, to: multicast_t);
    fn SV_ImageIndex(name: *mut libc::c_char) -> libc::c_int;
    fn SV_SoundIndex(name: *mut libc::c_char) -> libc::c_int;
    fn SV_ModelIndex(name: *mut libc::c_char) -> libc::c_int;
    static mut maxclients: *mut cvar_t;
    static mut sv: server_t;
    static mut svs: server_static_t;
    fn Sys_GetGameAPI(parms: *mut libc::c_void) -> *mut libc::c_void;
    fn Sys_UnloadGame();
    fn SCR_DebugGraph(value: libc::c_float, color: libc::c_int);
    fn Z_FreeTags(tag: libc::c_int);
    fn Z_TagMalloc(size: libc::c_int, tag: libc::c_int) -> *mut libc::c_void;
    fn Z_Free(ptr: *mut libc::c_void);
    fn Com_Error(code: libc::c_int, fmt: *mut libc::c_char, _: ...);
    fn Pmove(pmove: *mut pmove_t);
    fn CM_AreasConnected(area1: libc::c_int, area2: libc::c_int) -> qboolean;
    fn CM_SetAreaPortalState(portalnum: libc::c_int, open: qboolean);
    fn CM_LeafArea(leafnum: libc::c_int) -> libc::c_int;
    fn CM_LeafCluster(leafnum: libc::c_int) -> libc::c_int;
    fn CM_PointLeafnum(p: *mut vec_t) -> libc::c_int;
    fn CM_ClusterPHS(cluster: libc::c_int) -> *mut byte;
    fn CM_ClusterPVS(cluster: libc::c_int) -> *mut byte;
    fn CM_InlineModel(name: *mut libc::c_char) -> *mut cmodel_t;
    fn vsprintf(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: ::std::ffi::VaList,
    ) -> libc::c_int;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    static mut vec3_origin: vec3_t;
    fn Com_Printf(msg: *mut libc::c_char, _: ...);
    fn SZ_Clear(buf: *mut sizebuf_t);
    fn SZ_Write(buf: *mut sizebuf_t, data: *mut libc::c_void, length: libc::c_int);
    fn MSG_WriteChar(sb: *mut sizebuf_t, c: libc::c_int);
    fn MSG_WriteByte(sb: *mut sizebuf_t, c: libc::c_int);
    fn MSG_WriteShort(sb: *mut sizebuf_t, c: libc::c_int);
    fn MSG_WriteLong(sb: *mut sizebuf_t, c: libc::c_int);
    fn MSG_WriteFloat(sb: *mut sizebuf_t, f: libc::c_float);
    fn MSG_WriteString(sb: *mut sizebuf_t, s: *mut libc::c_char);
    fn MSG_WritePos(sb: *mut sizebuf_t, pos: *mut vec_t);
    fn MSG_WriteAngle(sb: *mut sizebuf_t, f: libc::c_float);
    fn MSG_WriteDir(sb: *mut sizebuf_t, vector: *mut vec_t);
    fn Cbuf_AddText(text: *mut libc::c_char);
    fn Cmd_Argc() -> libc::c_int;
    fn Cmd_Argv(arg: libc::c_int) -> *mut libc::c_char;
    fn Cmd_Args() -> *mut libc::c_char;
    fn Cvar_Get(
        var_name: *mut libc::c_char,
        value: *mut libc::c_char,
        flags: libc::c_int,
    ) -> *mut cvar_t;
    fn Cvar_Set(var_name: *mut libc::c_char, value: *mut libc::c_char) -> *mut cvar_t;
    fn Cvar_ForceSet(
        var_name: *mut libc::c_char,
        value: *mut libc::c_char,
    ) -> *mut cvar_t;
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
pub struct cplane_s {
    pub normal: vec3_t,
    pub dist: libc::c_float,
    pub type_0: byte,
    pub signbits: byte,
    pub pad: [byte; 2],
}
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
pub type cplane_t = cplane_s;
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
pub struct trace_t {
    pub allsolid: qboolean,
    pub startsolid: qboolean,
    pub fraction: libc::c_float,
    pub endpos: vec3_t,
    pub plane: cplane_t,
    pub surface: *mut csurface_t,
    pub contents: libc::c_int,
    pub ent: *mut edict_s,
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
pub struct pmove_t {
    pub s: pmove_state_t,
    pub cmd: usercmd_t,
    pub snapinitial: qboolean,
    pub numtouch: libc::c_int,
    pub touchents: [*mut edict_s; 32],
    pub viewangles: vec3_t,
    pub viewheight: libc::c_float,
    pub mins: vec3_t,
    pub maxs: vec3_t,
    pub groundentity: *mut edict_s,
    pub watertype: libc::c_int,
    pub waterlevel: libc::c_int,
    pub trace: Option::<
        unsafe extern "C" fn(*mut vec_t, *mut vec_t, *mut vec_t, *mut vec_t) -> trace_t,
    >,
    pub pointcontents: Option::<unsafe extern "C" fn(*mut vec_t) -> libc::c_int>,
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
pub struct game_import_t {
    pub bprintf: Option::<
        unsafe extern "C" fn(libc::c_int, *mut libc::c_char, ...) -> (),
    >,
    pub dprintf: Option::<unsafe extern "C" fn(*mut libc::c_char, ...) -> ()>,
    pub cprintf: Option::<
        unsafe extern "C" fn(*mut edict_t, libc::c_int, *mut libc::c_char, ...) -> (),
    >,
    pub centerprintf: Option::<
        unsafe extern "C" fn(*mut edict_t, *mut libc::c_char, ...) -> (),
    >,
    pub sound: Option::<
        unsafe extern "C" fn(
            *mut edict_t,
            libc::c_int,
            libc::c_int,
            libc::c_float,
            libc::c_float,
            libc::c_float,
        ) -> (),
    >,
    pub positioned_sound: Option::<
        unsafe extern "C" fn(
            *mut vec_t,
            *mut edict_t,
            libc::c_int,
            libc::c_int,
            libc::c_float,
            libc::c_float,
            libc::c_float,
        ) -> (),
    >,
    pub configstring: Option::<
        unsafe extern "C" fn(libc::c_int, *mut libc::c_char) -> (),
    >,
    pub error: Option::<unsafe extern "C" fn(*mut libc::c_char, ...) -> ()>,
    pub modelindex: Option::<unsafe extern "C" fn(*mut libc::c_char) -> libc::c_int>,
    pub soundindex: Option::<unsafe extern "C" fn(*mut libc::c_char) -> libc::c_int>,
    pub imageindex: Option::<unsafe extern "C" fn(*mut libc::c_char) -> libc::c_int>,
    pub setmodel: Option::<unsafe extern "C" fn(*mut edict_t, *mut libc::c_char) -> ()>,
    pub trace: Option::<
        unsafe extern "C" fn(
            *mut vec_t,
            *mut vec_t,
            *mut vec_t,
            *mut vec_t,
            *mut edict_t,
            libc::c_int,
        ) -> trace_t,
    >,
    pub pointcontents: Option::<unsafe extern "C" fn(*mut vec_t) -> libc::c_int>,
    pub inPVS: Option::<unsafe extern "C" fn(*mut vec_t, *mut vec_t) -> qboolean>,
    pub inPHS: Option::<unsafe extern "C" fn(*mut vec_t, *mut vec_t) -> qboolean>,
    pub SetAreaPortalState: Option::<unsafe extern "C" fn(libc::c_int, qboolean) -> ()>,
    pub AreasConnected: Option::<
        unsafe extern "C" fn(libc::c_int, libc::c_int) -> qboolean,
    >,
    pub linkentity: Option::<unsafe extern "C" fn(*mut edict_t) -> ()>,
    pub unlinkentity: Option::<unsafe extern "C" fn(*mut edict_t) -> ()>,
    pub BoxEdicts: Option::<
        unsafe extern "C" fn(
            *mut vec_t,
            *mut vec_t,
            *mut *mut edict_t,
            libc::c_int,
            libc::c_int,
        ) -> libc::c_int,
    >,
    pub Pmove: Option::<unsafe extern "C" fn(*mut pmove_t) -> ()>,
    pub multicast: Option::<unsafe extern "C" fn(*mut vec_t, multicast_t) -> ()>,
    pub unicast: Option::<unsafe extern "C" fn(*mut edict_t, qboolean) -> ()>,
    pub WriteChar: Option::<unsafe extern "C" fn(libc::c_int) -> ()>,
    pub WriteByte: Option::<unsafe extern "C" fn(libc::c_int) -> ()>,
    pub WriteShort: Option::<unsafe extern "C" fn(libc::c_int) -> ()>,
    pub WriteLong: Option::<unsafe extern "C" fn(libc::c_int) -> ()>,
    pub WriteFloat: Option::<unsafe extern "C" fn(libc::c_float) -> ()>,
    pub WriteString: Option::<unsafe extern "C" fn(*mut libc::c_char) -> ()>,
    pub WritePosition: Option::<unsafe extern "C" fn(*mut vec_t) -> ()>,
    pub WriteDir: Option::<unsafe extern "C" fn(*mut vec_t) -> ()>,
    pub WriteAngle: Option::<unsafe extern "C" fn(libc::c_float) -> ()>,
    pub TagMalloc: Option::<
        unsafe extern "C" fn(libc::c_int, libc::c_int) -> *mut libc::c_void,
    >,
    pub TagFree: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub FreeTags: Option::<unsafe extern "C" fn(libc::c_int) -> ()>,
    pub cvar: Option::<
        unsafe extern "C" fn(
            *mut libc::c_char,
            *mut libc::c_char,
            libc::c_int,
        ) -> *mut cvar_t,
    >,
    pub cvar_set: Option::<
        unsafe extern "C" fn(*mut libc::c_char, *mut libc::c_char) -> *mut cvar_t,
    >,
    pub cvar_forceset: Option::<
        unsafe extern "C" fn(*mut libc::c_char, *mut libc::c_char) -> *mut cvar_t,
    >,
    pub argc: Option::<unsafe extern "C" fn() -> libc::c_int>,
    pub argv: Option::<unsafe extern "C" fn(libc::c_int) -> *mut libc::c_char>,
    pub args: Option::<unsafe extern "C" fn() -> *mut libc::c_char>,
    pub AddCommandString: Option::<unsafe extern "C" fn(*mut libc::c_char) -> ()>,
    pub DebugGraph: Option::<unsafe extern "C" fn(libc::c_float, libc::c_int) -> ()>,
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
pub static mut ge: *mut game_export_t = 0 as *const game_export_t as *mut game_export_t;
#[no_mangle]
pub unsafe extern "C" fn PF_Unicast(mut ent: *mut edict_t, mut reliable: qboolean) {
    let mut p: libc::c_int = 0;
    let mut client: *mut client_t = 0 as *mut client_t;
    if ent.is_null() {
        return;
    }
    p = ((ent as *mut byte).offset_from((*ge).edicts as *mut byte) as libc::c_long
        / (*ge).edict_size as libc::c_long) as libc::c_int;
    if p < 1 as libc::c_int || p as libc::c_float > (*maxclients).value {
        return;
    }
    client = (svs.clients).offset((p - 1 as libc::c_int) as isize);
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
    SZ_Clear(&mut sv.multicast);
}
#[no_mangle]
pub unsafe extern "C" fn PF_dprintf(mut fmt: *mut libc::c_char, mut args: ...) {
    let mut msg: [libc::c_char; 1024] = [0; 1024];
    let mut argptr: ::std::ffi::VaListImpl;
    argptr = args.clone();
    vsprintf(msg.as_mut_ptr(), fmt, argptr.as_va_list());
    Com_Printf(
        b"%s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        msg.as_mut_ptr(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn PF_cprintf(
    mut ent: *mut edict_t,
    mut level: libc::c_int,
    mut fmt: *mut libc::c_char,
    mut args: ...
) {
    let mut msg: [libc::c_char; 1024] = [0; 1024];
    let mut argptr: ::std::ffi::VaListImpl;
    let mut n: libc::c_int = 0;
    if !ent.is_null() {
        n = ((ent as *mut byte).offset_from((*ge).edicts as *mut byte) as libc::c_long
            / (*ge).edict_size as libc::c_long) as libc::c_int;
        if n < 1 as libc::c_int || n as libc::c_float > (*maxclients).value {
            Com_Error(
                1 as libc::c_int,
                b"cprintf to a non-client\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
        }
    }
    argptr = args.clone();
    vsprintf(msg.as_mut_ptr(), fmt, argptr.as_va_list());
    if !ent.is_null() {
        SV_ClientPrintf(
            (svs.clients).offset((n - 1 as libc::c_int) as isize),
            level,
            b"%s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            msg.as_mut_ptr(),
        );
    } else {
        Com_Printf(
            b"%s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            msg.as_mut_ptr(),
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn PF_centerprintf(
    mut ent: *mut edict_t,
    mut fmt: *mut libc::c_char,
    mut args: ...
) {
    let mut msg: [libc::c_char; 1024] = [0; 1024];
    let mut argptr: ::std::ffi::VaListImpl;
    let mut n: libc::c_int = 0;
    n = ((ent as *mut byte).offset_from((*ge).edicts as *mut byte) as libc::c_long
        / (*ge).edict_size as libc::c_long) as libc::c_int;
    if n < 1 as libc::c_int || n as libc::c_float > (*maxclients).value {
        return;
    }
    argptr = args.clone();
    vsprintf(msg.as_mut_ptr(), fmt, argptr.as_va_list());
    MSG_WriteByte(&mut sv.multicast, svc_centerprint as libc::c_int);
    MSG_WriteString(&mut sv.multicast, msg.as_mut_ptr());
    PF_Unicast(ent, true_0);
}
#[no_mangle]
pub unsafe extern "C" fn PF_error(mut fmt: *mut libc::c_char, mut args: ...) {
    let mut msg: [libc::c_char; 1024] = [0; 1024];
    let mut argptr: ::std::ffi::VaListImpl;
    argptr = args.clone();
    vsprintf(msg.as_mut_ptr(), fmt, argptr.as_va_list());
    Com_Error(
        1 as libc::c_int,
        b"Game Error: %s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        msg.as_mut_ptr(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn PF_setmodel(
    mut ent: *mut edict_t,
    mut name: *mut libc::c_char,
) {
    let mut i: libc::c_int = 0;
    let mut mod_0: *mut cmodel_t = 0 as *mut cmodel_t;
    if name.is_null() {
        Com_Error(
            1 as libc::c_int,
            b"PF_setmodel: NULL\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    i = SV_ModelIndex(name);
    (*ent).s.modelindex = i;
    if *name.offset(0 as libc::c_int as isize) as libc::c_int == '*' as i32 {
        mod_0 = CM_InlineModel(name);
        (*ent)
            .mins[0 as libc::c_int as usize] = (*mod_0).mins[0 as libc::c_int as usize];
        (*ent)
            .mins[1 as libc::c_int as usize] = (*mod_0).mins[1 as libc::c_int as usize];
        (*ent)
            .mins[2 as libc::c_int as usize] = (*mod_0).mins[2 as libc::c_int as usize];
        (*ent)
            .maxs[0 as libc::c_int as usize] = (*mod_0).maxs[0 as libc::c_int as usize];
        (*ent)
            .maxs[1 as libc::c_int as usize] = (*mod_0).maxs[1 as libc::c_int as usize];
        (*ent)
            .maxs[2 as libc::c_int as usize] = (*mod_0).maxs[2 as libc::c_int as usize];
        SV_LinkEdict(ent);
    }
}
#[no_mangle]
pub unsafe extern "C" fn PF_Configstring(
    mut index: libc::c_int,
    mut val: *mut libc::c_char,
) {
    if index < 0 as libc::c_int
        || index
            >= 32 as libc::c_int + 256 as libc::c_int + 256 as libc::c_int
                + 256 as libc::c_int + 256 as libc::c_int + 256 as libc::c_int
                + 256 as libc::c_int + 256 as libc::c_int * 2 as libc::c_int
    {
        Com_Error(
            1 as libc::c_int,
            b"configstring: bad index %i\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            index,
        );
    }
    if val.is_null() {
        val = b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    strcpy((sv.configstrings[index as usize]).as_mut_ptr(), val);
    if sv.state as libc::c_uint != ss_loading as libc::c_int as libc::c_uint {
        SZ_Clear(&mut sv.multicast);
        MSG_WriteChar(&mut sv.multicast, svc_configstring as libc::c_int);
        MSG_WriteShort(&mut sv.multicast, index);
        MSG_WriteString(&mut sv.multicast, val);
        SV_Multicast(vec3_origin.as_mut_ptr(), MULTICAST_ALL_R);
    }
}
#[no_mangle]
pub unsafe extern "C" fn PF_WriteChar(mut c: libc::c_int) {
    MSG_WriteChar(&mut sv.multicast, c);
}
#[no_mangle]
pub unsafe extern "C" fn PF_WriteByte(mut c: libc::c_int) {
    MSG_WriteByte(&mut sv.multicast, c);
}
#[no_mangle]
pub unsafe extern "C" fn PF_WriteShort(mut c: libc::c_int) {
    MSG_WriteShort(&mut sv.multicast, c);
}
#[no_mangle]
pub unsafe extern "C" fn PF_WriteLong(mut c: libc::c_int) {
    MSG_WriteLong(&mut sv.multicast, c);
}
#[no_mangle]
pub unsafe extern "C" fn PF_WriteFloat(mut f: libc::c_float) {
    MSG_WriteFloat(&mut sv.multicast, f);
}
#[no_mangle]
pub unsafe extern "C" fn PF_WriteString(mut s: *mut libc::c_char) {
    MSG_WriteString(&mut sv.multicast, s);
}
#[no_mangle]
pub unsafe extern "C" fn PF_WritePos(mut pos: *mut vec_t) {
    MSG_WritePos(&mut sv.multicast, pos);
}
#[no_mangle]
pub unsafe extern "C" fn PF_WriteDir(mut dir: *mut vec_t) {
    MSG_WriteDir(&mut sv.multicast, dir);
}
#[no_mangle]
pub unsafe extern "C" fn PF_WriteAngle(mut f: libc::c_float) {
    MSG_WriteAngle(&mut sv.multicast, f);
}
#[no_mangle]
pub unsafe extern "C" fn PF_inPVS(mut p1: *mut vec_t, mut p2: *mut vec_t) -> qboolean {
    let mut leafnum: libc::c_int = 0;
    let mut cluster: libc::c_int = 0;
    let mut area1: libc::c_int = 0;
    let mut area2: libc::c_int = 0;
    let mut mask: *mut byte = 0 as *mut byte;
    leafnum = CM_PointLeafnum(p1);
    cluster = CM_LeafCluster(leafnum);
    area1 = CM_LeafArea(leafnum);
    mask = CM_ClusterPVS(cluster);
    leafnum = CM_PointLeafnum(p2);
    cluster = CM_LeafCluster(leafnum);
    area2 = CM_LeafArea(leafnum);
    if !mask.is_null()
        && *mask.offset((cluster >> 3 as libc::c_int) as isize) as libc::c_int
            & (1 as libc::c_int) << (cluster & 7 as libc::c_int) == 0
    {
        return false_0;
    }
    if CM_AreasConnected(area1, area2) as u64 == 0 {
        return false_0;
    }
    return true_0;
}
#[no_mangle]
pub unsafe extern "C" fn PF_inPHS(mut p1: *mut vec_t, mut p2: *mut vec_t) -> qboolean {
    let mut leafnum: libc::c_int = 0;
    let mut cluster: libc::c_int = 0;
    let mut area1: libc::c_int = 0;
    let mut area2: libc::c_int = 0;
    let mut mask: *mut byte = 0 as *mut byte;
    leafnum = CM_PointLeafnum(p1);
    cluster = CM_LeafCluster(leafnum);
    area1 = CM_LeafArea(leafnum);
    mask = CM_ClusterPHS(cluster);
    leafnum = CM_PointLeafnum(p2);
    cluster = CM_LeafCluster(leafnum);
    area2 = CM_LeafArea(leafnum);
    if !mask.is_null()
        && *mask.offset((cluster >> 3 as libc::c_int) as isize) as libc::c_int
            & (1 as libc::c_int) << (cluster & 7 as libc::c_int) == 0
    {
        return false_0;
    }
    if CM_AreasConnected(area1, area2) as u64 == 0 {
        return false_0;
    }
    return true_0;
}
#[no_mangle]
pub unsafe extern "C" fn PF_StartSound(
    mut entity: *mut edict_t,
    mut channel: libc::c_int,
    mut sound_num: libc::c_int,
    mut volume: libc::c_float,
    mut attenuation: libc::c_float,
    mut timeofs: libc::c_float,
) {
    if entity.is_null() {
        return;
    }
    SV_StartSound(
        0 as *mut vec_t,
        entity,
        channel,
        sound_num,
        volume,
        attenuation,
        timeofs,
    );
}
#[no_mangle]
pub unsafe extern "C" fn SV_ShutdownGameProgs() {
    if ge.is_null() {
        return;
    }
    ((*ge).Shutdown).expect("non-null function pointer")();
    Sys_UnloadGame();
    ge = 0 as *mut game_export_t;
}
#[no_mangle]
pub unsafe extern "C" fn SV_InitGameProgs() {
    let mut import: game_import_t = game_import_t {
        bprintf: None,
        dprintf: None,
        cprintf: None,
        centerprintf: None,
        sound: None,
        positioned_sound: None,
        configstring: None,
        error: None,
        modelindex: None,
        soundindex: None,
        imageindex: None,
        setmodel: None,
        trace: None,
        pointcontents: None,
        inPVS: None,
        inPHS: None,
        SetAreaPortalState: None,
        AreasConnected: None,
        linkentity: None,
        unlinkentity: None,
        BoxEdicts: None,
        Pmove: None,
        multicast: None,
        unicast: None,
        WriteChar: None,
        WriteByte: None,
        WriteShort: None,
        WriteLong: None,
        WriteFloat: None,
        WriteString: None,
        WritePosition: None,
        WriteDir: None,
        WriteAngle: None,
        TagMalloc: None,
        TagFree: None,
        FreeTags: None,
        cvar: None,
        cvar_set: None,
        cvar_forceset: None,
        argc: None,
        argv: None,
        args: None,
        AddCommandString: None,
        DebugGraph: None,
    };
    if !ge.is_null() {
        SV_ShutdownGameProgs();
    }
    import
        .multicast = Some(
        SV_Multicast as unsafe extern "C" fn(*mut vec_t, multicast_t) -> (),
    );
    import
        .unicast = Some(
        PF_Unicast as unsafe extern "C" fn(*mut edict_t, qboolean) -> (),
    );
    import
        .bprintf = Some(
        SV_BroadcastPrintf
            as unsafe extern "C" fn(libc::c_int, *mut libc::c_char, ...) -> (),
    );
    import
        .dprintf = Some(
        PF_dprintf as unsafe extern "C" fn(*mut libc::c_char, ...) -> (),
    );
    import
        .cprintf = Some(
        PF_cprintf
            as unsafe extern "C" fn(
                *mut edict_t,
                libc::c_int,
                *mut libc::c_char,
                ...
            ) -> (),
    );
    import
        .centerprintf = Some(
        PF_centerprintf
            as unsafe extern "C" fn(*mut edict_t, *mut libc::c_char, ...) -> (),
    );
    import.error = Some(PF_error as unsafe extern "C" fn(*mut libc::c_char, ...) -> ());
    import.linkentity = Some(SV_LinkEdict as unsafe extern "C" fn(*mut edict_t) -> ());
    import
        .unlinkentity = Some(SV_UnlinkEdict as unsafe extern "C" fn(*mut edict_t) -> ());
    import
        .BoxEdicts = Some(
        SV_AreaEdicts
            as unsafe extern "C" fn(
                *mut vec_t,
                *mut vec_t,
                *mut *mut edict_t,
                libc::c_int,
                libc::c_int,
            ) -> libc::c_int,
    );
    import
        .trace = Some(
        SV_Trace
            as unsafe extern "C" fn(
                *mut vec_t,
                *mut vec_t,
                *mut vec_t,
                *mut vec_t,
                *mut edict_t,
                libc::c_int,
            ) -> trace_t,
    );
    import
        .pointcontents = Some(
        SV_PointContents as unsafe extern "C" fn(*mut vec_t) -> libc::c_int,
    );
    import
        .setmodel = Some(
        PF_setmodel as unsafe extern "C" fn(*mut edict_t, *mut libc::c_char) -> (),
    );
    import
        .inPVS = Some(
        PF_inPVS as unsafe extern "C" fn(*mut vec_t, *mut vec_t) -> qboolean,
    );
    import
        .inPHS = Some(
        PF_inPHS as unsafe extern "C" fn(*mut vec_t, *mut vec_t) -> qboolean,
    );
    import.Pmove = Some(Pmove as unsafe extern "C" fn(*mut pmove_t) -> ());
    import
        .modelindex = Some(
        SV_ModelIndex as unsafe extern "C" fn(*mut libc::c_char) -> libc::c_int,
    );
    import
        .soundindex = Some(
        SV_SoundIndex as unsafe extern "C" fn(*mut libc::c_char) -> libc::c_int,
    );
    import
        .imageindex = Some(
        SV_ImageIndex as unsafe extern "C" fn(*mut libc::c_char) -> libc::c_int,
    );
    import
        .configstring = Some(
        PF_Configstring as unsafe extern "C" fn(libc::c_int, *mut libc::c_char) -> (),
    );
    import
        .sound = Some(
        PF_StartSound
            as unsafe extern "C" fn(
                *mut edict_t,
                libc::c_int,
                libc::c_int,
                libc::c_float,
                libc::c_float,
                libc::c_float,
            ) -> (),
    );
    import
        .positioned_sound = Some(
        SV_StartSound
            as unsafe extern "C" fn(
                *mut vec_t,
                *mut edict_t,
                libc::c_int,
                libc::c_int,
                libc::c_float,
                libc::c_float,
                libc::c_float,
            ) -> (),
    );
    import.WriteChar = Some(PF_WriteChar as unsafe extern "C" fn(libc::c_int) -> ());
    import.WriteByte = Some(PF_WriteByte as unsafe extern "C" fn(libc::c_int) -> ());
    import.WriteShort = Some(PF_WriteShort as unsafe extern "C" fn(libc::c_int) -> ());
    import.WriteLong = Some(PF_WriteLong as unsafe extern "C" fn(libc::c_int) -> ());
    import.WriteFloat = Some(PF_WriteFloat as unsafe extern "C" fn(libc::c_float) -> ());
    import
        .WriteString = Some(
        PF_WriteString as unsafe extern "C" fn(*mut libc::c_char) -> (),
    );
    import.WritePosition = Some(PF_WritePos as unsafe extern "C" fn(*mut vec_t) -> ());
    import.WriteDir = Some(PF_WriteDir as unsafe extern "C" fn(*mut vec_t) -> ());
    import.WriteAngle = Some(PF_WriteAngle as unsafe extern "C" fn(libc::c_float) -> ());
    import
        .TagMalloc = Some(
        Z_TagMalloc
            as unsafe extern "C" fn(libc::c_int, libc::c_int) -> *mut libc::c_void,
    );
    import.TagFree = Some(Z_Free as unsafe extern "C" fn(*mut libc::c_void) -> ());
    import.FreeTags = Some(Z_FreeTags as unsafe extern "C" fn(libc::c_int) -> ());
    import
        .cvar = Some(
        Cvar_Get
            as unsafe extern "C" fn(
                *mut libc::c_char,
                *mut libc::c_char,
                libc::c_int,
            ) -> *mut cvar_t,
    );
    import
        .cvar_set = Some(
        Cvar_Set
            as unsafe extern "C" fn(*mut libc::c_char, *mut libc::c_char) -> *mut cvar_t,
    );
    import
        .cvar_forceset = Some(
        Cvar_ForceSet
            as unsafe extern "C" fn(*mut libc::c_char, *mut libc::c_char) -> *mut cvar_t,
    );
    import.argc = Some(Cmd_Argc as unsafe extern "C" fn() -> libc::c_int);
    import
        .argv = Some(Cmd_Argv as unsafe extern "C" fn(libc::c_int) -> *mut libc::c_char);
    import.args = Some(Cmd_Args as unsafe extern "C" fn() -> *mut libc::c_char);
    import
        .AddCommandString = Some(
        Cbuf_AddText as unsafe extern "C" fn(*mut libc::c_char) -> (),
    );
    import
        .DebugGraph = Some(
        SCR_DebugGraph as unsafe extern "C" fn(libc::c_float, libc::c_int) -> (),
    );
    import
        .SetAreaPortalState = Some(
        CM_SetAreaPortalState as unsafe extern "C" fn(libc::c_int, qboolean) -> (),
    );
    import
        .AreasConnected = Some(
        CM_AreasConnected as unsafe extern "C" fn(libc::c_int, libc::c_int) -> qboolean,
    );
    ge = Sys_GetGameAPI(&mut import as *mut game_import_t as *mut libc::c_void)
        as *mut game_export_t;
    if ge.is_null() {
        Com_Error(
            1 as libc::c_int,
            b"failed to load game DLL\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    if (*ge).apiversion != 3 as libc::c_int {
        Com_Error(
            1 as libc::c_int,
            b"game is version %i, not %i\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            (*ge).apiversion,
            3 as libc::c_int,
        );
    }
    ((*ge).Init).expect("non-null function pointer")();
}
