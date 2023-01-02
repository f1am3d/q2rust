#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(extern_types, register_tool)]

extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type image_s;
    pub type model_s;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn VectorNormalize(v: *mut vec_t) -> vec_t;
    fn Com_sprintf(
        dest: *mut libc::c_char,
        size: libc::c_int,
        fmt: *mut libc::c_char,
        _: ...
    );
    fn Com_PageInMemory(buffer: *mut byte, size: libc::c_int);
    fn LittleShort(l: libc::c_short) -> libc::c_short;
    fn Com_Printf(msg: *mut libc::c_char, _: ...);
    fn Cmd_AddCommand(cmd_name: *mut libc::c_char, function: xcommand_t);
    fn Cmd_RemoveCommand(cmd_name: *mut libc::c_char);
    fn Cmd_Argc() -> libc::c_int;
    fn Cmd_Argv(arg: libc::c_int) -> *mut libc::c_char;
    fn Cvar_Get(
        var_name: *mut libc::c_char,
        value: *mut libc::c_char,
        flags: libc::c_int,
    ) -> *mut cvar_t;
    fn FS_FOpenFile(filename: *mut libc::c_char, file: *mut *mut FILE) -> libc::c_int;
    fn FS_FCloseFile(f: *mut FILE);
    fn Com_DPrintf(fmt: *mut libc::c_char, _: ...);
    fn Com_Error(code: libc::c_int, fmt: *mut libc::c_char, _: ...);
    fn Z_Free(ptr: *mut libc::c_void);
    fn Z_Malloc(size: libc::c_int) -> *mut libc::c_void;
    static mut cl: client_state_t;
    static mut cl_entities: [centity_t; 1024];
    static mut cls: client_static_t;
    static mut cl_parse_entities: [entity_state_t; 1024];
    static mut cl_paused: *mut cvar_t;
    fn CL_GetEntitySoundOrigin(ent: libc::c_int, org: *mut vec_t);
    fn SNDDMA_Init() -> qboolean;
    fn SNDDMA_GetDMAPos() -> libc::c_int;
    fn SNDDMA_Shutdown();
    fn SNDDMA_BeginPainting();
    fn SNDDMA_Submit();
    fn S_InitScaletable();
    fn S_LoadSound(s: *mut sfx_t) -> *mut sfxcache_t;
    fn S_PaintChannels(endtime: libc::c_int);
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
pub struct sfx_s {
    pub name: [libc::c_char; 64],
    pub registration_sequence: libc::c_int,
    pub cache: *mut sfxcache_t,
    pub truename: *mut libc::c_char,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct sfxcache_t {
    pub length: libc::c_int,
    pub loopstart: libc::c_int,
    pub speed: libc::c_int,
    pub width: libc::c_int,
    pub stereo: libc::c_int,
    pub data: [byte; 1],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct dma_t {
    pub channels: libc::c_int,
    pub samples: libc::c_int,
    pub submission_chunk: libc::c_int,
    pub samplepos: libc::c_int,
    pub samplebits: libc::c_int,
    pub speed: libc::c_int,
    pub buffer: *mut byte,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct channel_t {
    pub sfx: *mut sfx_t,
    pub leftvol: libc::c_int,
    pub rightvol: libc::c_int,
    pub end: libc::c_int,
    pub pos: libc::c_int,
    pub looping: libc::c_int,
    pub entnum: libc::c_int,
    pub entchannel: libc::c_int,
    pub origin: vec3_t,
    pub dist_mult: vec_t,
    pub master_vol: libc::c_int,
    pub fixed_origin: qboolean,
    pub autosound: qboolean,
}

pub type sfx_t = sfx_s;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct playsound_s {
    pub prev: *mut playsound_s,
    pub next: *mut playsound_s,
    pub sfx: *mut sfx_t,
    pub volume: libc::c_float,
    pub attenuation: libc::c_float,
    pub entnum: libc::c_int,
    pub entchannel: libc::c_int,
    pub fixed_origin: qboolean,
    pub origin: vec3_t,
    pub begin: libc::c_uint,
}

pub type playsound_t = playsound_s;

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
pub struct portable_samplepair_t {
    pub left: libc::c_int,
    pub right: libc::c_int,
}

pub const ca_active: connstate_t = 4;

pub type connstate_t = libc::c_uint;

pub const ca_connected: connstate_t = 3;
pub const ca_connecting: connstate_t = 2;
pub const ca_disconnected: connstate_t = 1;
pub const ca_uninitialized: connstate_t = 0;

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
#[no_mangle]
pub static mut s_registration_sequence: libc::c_int = 0;
#[no_mangle]
pub static mut channels: [channel_t; 32] = [channel_t {
    sfx: 0 as *const sfx_t as *mut sfx_t,
    leftvol: 0,
    rightvol: 0,
    end: 0,
    pos: 0,
    looping: 0,
    entnum: 0,
    entchannel: 0,
    origin: [0.; 3],
    dist_mult: 0.,
    master_vol: 0,
    fixed_origin: false_0,
    autosound: false_0,
}; 32];
#[no_mangle]
pub static mut snd_initialized: qboolean = false_0;
#[no_mangle]
pub static mut sound_started: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut dma: dma_t = dma_t {
    channels: 0,
    samples: 0,
    submission_chunk: 0,
    samplepos: 0,
    samplebits: 0,
    speed: 0,
    buffer: 0 as *const byte as *mut byte,
};
#[no_mangle]
pub static mut listener_origin: vec3_t = [0.; 3];
#[no_mangle]
pub static mut listener_forward: vec3_t = [0.; 3];
#[no_mangle]
pub static mut listener_right: vec3_t = [0.; 3];
#[no_mangle]
pub static mut listener_up: vec3_t = [0.; 3];
#[no_mangle]
pub static mut s_registering: qboolean = false_0;
#[no_mangle]
pub static mut soundtime: libc::c_int = 0;
#[no_mangle]
pub static mut paintedtime: libc::c_int = 0;
#[no_mangle]
pub static mut known_sfx: [sfx_t; 512] = [sfx_t {
    name: [0; 64],
    registration_sequence: 0,
    cache: 0 as *const sfxcache_t as *mut sfxcache_t,
    truename: 0 as *const libc::c_char as *mut libc::c_char,
}; 512];
#[no_mangle]
pub static mut num_sfx: libc::c_int = 0;
#[no_mangle]
pub static mut s_playsounds: [playsound_t; 128] = [playsound_t {
    prev: 0 as *const playsound_s as *mut playsound_s,
    next: 0 as *const playsound_s as *mut playsound_s,
    sfx: 0 as *const sfx_t as *mut sfx_t,
    volume: 0.,
    attenuation: 0.,
    entnum: 0,
    entchannel: 0,
    fixed_origin: false_0,
    origin: [0.; 3],
    begin: 0,
}; 128];
#[no_mangle]
pub static mut s_freeplays: playsound_t = playsound_t {
    prev: 0 as *const playsound_s as *mut playsound_s,
    next: 0 as *const playsound_s as *mut playsound_s,
    sfx: 0 as *const sfx_t as *mut sfx_t,
    volume: 0.,
    attenuation: 0.,
    entnum: 0,
    entchannel: 0,
    fixed_origin: false_0,
    origin: [0.; 3],
    begin: 0,
};
#[no_mangle]
pub static mut s_pendingplays: playsound_t = playsound_t {
    prev: 0 as *const playsound_s as *mut playsound_s,
    next: 0 as *const playsound_s as *mut playsound_s,
    sfx: 0 as *const sfx_t as *mut sfx_t,
    volume: 0.,
    attenuation: 0.,
    entnum: 0,
    entchannel: 0,
    fixed_origin: false_0,
    origin: [0.; 3],
    begin: 0,
};
#[no_mangle]
pub static mut s_beginofs: libc::c_int = 0;
#[no_mangle]
pub static mut s_volume: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut s_testsound: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut s_loadas8bit: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut s_khz: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut s_show: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut s_mixahead: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut s_primary: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut s_rawend: libc::c_int = 0;
#[no_mangle]
pub static mut s_rawsamples: [portable_samplepair_t; 8192] = [portable_samplepair_t {
    left: 0,
    right: 0,
}; 8192];

#[no_mangle]
pub unsafe extern "C" fn S_SoundInfo_f() {
    if sound_started == 0 {
        Com_Printf(
            b"sound system not started\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return;
    }
    Com_Printf(
        b"%5d stereo\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        dma.channels - 1 as libc::c_int,
    );
    Com_Printf(
        b"%5d samples\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        dma.samples,
    );
    Com_Printf(
        b"%5d samplepos\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        dma.samplepos,
    );
    Com_Printf(
        b"%5d samplebits\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        dma.samplebits,
    );
    Com_Printf(
        b"%5d submission_chunk\n\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        dma.submission_chunk,
    );
    Com_Printf(
        b"%5d speed\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        dma.speed,
    );
    Com_Printf(
        b"0x%x dma buffer\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        dma.buffer,
    );
}

#[no_mangle]
pub unsafe extern "C" fn S_Init() {
    let mut cv: *mut cvar_t = 0 as *mut cvar_t;
    Com_Printf(
        b"\n------- sound initialization -------\n\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    cv = Cvar_Get(
        b"s_initsound\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    if (*cv).value == 0. {
        Com_Printf(
            b"not initializing.\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    } else {
        s_volume = Cvar_Get(
            b"s_volume\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"0.7\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            1 as libc::c_int,
        );
        s_khz = Cvar_Get(
            b"s_khz\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"11\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            1 as libc::c_int,
        );
        s_loadas8bit = Cvar_Get(
            b"s_loadas8bit\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            1 as libc::c_int,
        );
        s_mixahead = Cvar_Get(
            b"s_mixahead\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"0.2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            1 as libc::c_int,
        );
        s_show = Cvar_Get(
            b"s_show\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            0 as libc::c_int,
        );
        s_testsound = Cvar_Get(
            b"s_testsound\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            0 as libc::c_int,
        );
        s_primary = Cvar_Get(
            b"s_primary\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            1 as libc::c_int,
        );
        Cmd_AddCommand(
            b"play\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            Some(S_Play as unsafe extern "C" fn() -> ()),
        );
        Cmd_AddCommand(
            b"stopsound\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            Some(S_StopAllSounds as unsafe extern "C" fn() -> ()),
        );
        Cmd_AddCommand(
            b"soundlist\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            Some(S_SoundList as unsafe extern "C" fn() -> ()),
        );
        Cmd_AddCommand(
            b"soundinfo\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            Some(S_SoundInfo_f as unsafe extern "C" fn() -> ()),
        );
        if SNDDMA_Init() as u64 == 0 {
            return;
        }
        S_InitScaletable();
        sound_started = 1 as libc::c_int;
        num_sfx = 0 as libc::c_int;
        soundtime = 0 as libc::c_int;
        paintedtime = 0 as libc::c_int;
        Com_Printf(
            b"sound sampling rate: %i\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            dma.speed,
        );
        S_StopAllSounds();
    }
    Com_Printf(
        b"------------------------------------\n\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
}

#[no_mangle]
pub unsafe extern "C" fn S_Shutdown() {
    let mut i: libc::c_int = 0;
    let mut sfx: *mut sfx_t = 0 as *mut sfx_t;
    if sound_started == 0 {
        return;
    }
    SNDDMA_Shutdown();
    sound_started = 0 as libc::c_int;
    Cmd_RemoveCommand(
        b"play\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    Cmd_RemoveCommand(
        b"stopsound\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    Cmd_RemoveCommand(
        b"soundlist\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    Cmd_RemoveCommand(
        b"soundinfo\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    i = 0 as libc::c_int;
    sfx = known_sfx.as_mut_ptr();
    while i < num_sfx {
        if !((*sfx).name[0 as libc::c_int as usize] == 0) {
            if !((*sfx).cache).is_null() {
                Z_Free((*sfx).cache as *mut libc::c_void);
            }
            memset(
                sfx as *mut libc::c_void,
                0 as libc::c_int,
                ::std::mem::size_of::<sfx_t>() as libc::c_ulong,
            );
        }
        i += 1;
        sfx = sfx.offset(1);
    }
    num_sfx = 0 as libc::c_int;
}

#[no_mangle]
pub unsafe extern "C" fn S_FindName(
    mut name: *mut libc::c_char,
    mut create: qboolean,
) -> *mut sfx_s {
    let mut i: libc::c_int = 0;
    let mut sfx: *mut sfx_t = 0 as *mut sfx_t;
    if name.is_null() {
        Com_Error(
            0 as libc::c_int,
            b"S_FindName: NULL\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    if *name.offset(0 as libc::c_int as isize) == 0 {
        Com_Error(
            0 as libc::c_int,
            b"S_FindName: empty name\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    if strlen(name) >= 64 as libc::c_int as libc::c_ulong {
        Com_Error(
            0 as libc::c_int,
            b"Sound name too long: %s\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            name,
        );
    }
    i = 0 as libc::c_int;
    while i < num_sfx {
        if strcmp((known_sfx[i as usize].name).as_mut_ptr(), name) == 0 {
            return &mut *known_sfx.as_mut_ptr().offset(i as isize) as *mut sfx_t;
        }
        i += 1;
    }
    if create as u64 == 0 {
        return 0 as *mut sfx_s;
    }
    i = 0 as libc::c_int;
    while i < num_sfx {
        if known_sfx[i as usize].name[0 as libc::c_int as usize] == 0 {
            break;
        }
        i += 1;
    }
    if i == num_sfx {
        if num_sfx == 256 as libc::c_int * 2 as libc::c_int {
            Com_Error(
                0 as libc::c_int,
                b"S_FindName: out of sfx_t\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
        }
        num_sfx += 1;
    }
    sfx = &mut *known_sfx.as_mut_ptr().offset(i as isize) as *mut sfx_t;
    memset(
        sfx as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<sfx_t>() as libc::c_ulong,
    );
    strcpy(((*sfx).name).as_mut_ptr(), name);
    (*sfx).registration_sequence = s_registration_sequence;
    return sfx;
}

#[no_mangle]
pub unsafe extern "C" fn S_AliasName(
    mut aliasname: *mut libc::c_char,
    mut truename: *mut libc::c_char,
) -> *mut sfx_t {
    let mut sfx: *mut sfx_t = 0 as *mut sfx_t;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    s = Z_Malloc(64 as libc::c_int) as *mut libc::c_char;
    strcpy(s, truename);
    i = 0 as libc::c_int;
    while i < num_sfx {
        if known_sfx[i as usize].name[0 as libc::c_int as usize] == 0 {
            break;
        }
        i += 1;
    }
    if i == num_sfx {
        if num_sfx == 256 as libc::c_int * 2 as libc::c_int {
            Com_Error(
                0 as libc::c_int,
                b"S_FindName: out of sfx_t\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
        }
        num_sfx += 1;
    }
    sfx = &mut *known_sfx.as_mut_ptr().offset(i as isize) as *mut sfx_t;
    memset(
        sfx as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<sfx_t>() as libc::c_ulong,
    );
    strcpy(((*sfx).name).as_mut_ptr(), aliasname);
    (*sfx).registration_sequence = s_registration_sequence;
    let ref mut fresh0 = (*sfx).truename;
    *fresh0 = s;
    return sfx;
}

#[no_mangle]
pub unsafe extern "C" fn S_BeginRegistration() {
    s_registration_sequence += 1;
    s_registering = true_0;
}

#[no_mangle]
pub unsafe extern "C" fn S_RegisterSound(mut name: *mut libc::c_char) -> *mut sfx_s {
    let mut sfx: *mut sfx_t = 0 as *mut sfx_t;
    if sound_started == 0 {
        return 0 as *mut sfx_s;
    }
    sfx = S_FindName(name, true_0);
    (*sfx).registration_sequence = s_registration_sequence;
    if s_registering as u64 == 0 {
        S_LoadSound(sfx);
    }
    return sfx;
}

#[no_mangle]
pub unsafe extern "C" fn S_EndRegistration() {
    let mut i: libc::c_int = 0;
    let mut sfx: *mut sfx_t = 0 as *mut sfx_t;
    let mut size: libc::c_int = 0;
    i = 0 as libc::c_int;
    sfx = known_sfx.as_mut_ptr();
    while i < num_sfx {
        if !((*sfx).name[0 as libc::c_int as usize] == 0) {
            if (*sfx).registration_sequence != s_registration_sequence {
                if !((*sfx).cache).is_null() {
                    Z_Free((*sfx).cache as *mut libc::c_void);
                }
                memset(
                    sfx as *mut libc::c_void,
                    0 as libc::c_int,
                    ::std::mem::size_of::<sfx_t>() as libc::c_ulong,
                );
            } else if !((*sfx).cache).is_null() {
                size = (*(*sfx).cache).length * (*(*sfx).cache).width;
                Com_PageInMemory((*sfx).cache as *mut byte, size);
            }
        }
        i += 1;
        sfx = sfx.offset(1);
    }
    i = 0 as libc::c_int;
    sfx = known_sfx.as_mut_ptr();
    while i < num_sfx {
        if !((*sfx).name[0 as libc::c_int as usize] == 0) {
            S_LoadSound(sfx);
        }
        i += 1;
        sfx = sfx.offset(1);
    }
    s_registering = false_0;
}

#[no_mangle]
pub unsafe extern "C" fn S_PickChannel(
    mut entnum: libc::c_int,
    mut entchannel: libc::c_int,
) -> *mut channel_t {
    let mut ch_idx: libc::c_int = 0;
    let mut first_to_die: libc::c_int = 0;
    let mut life_left: libc::c_int = 0;
    let mut ch: *mut channel_t = 0 as *mut channel_t;
    if entchannel < 0 as libc::c_int {
        Com_Error(
            1 as libc::c_int,
            b"S_PickChannel: entchannel<0\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    first_to_die = -(1 as libc::c_int);
    life_left = 0x7fffffff as libc::c_int;
    ch_idx = 0 as libc::c_int;
    while ch_idx < 32 as libc::c_int {
        if entchannel != 0 as libc::c_int && channels[ch_idx as usize].entnum == entnum
            && channels[ch_idx as usize].entchannel == entchannel
        {
            first_to_die = ch_idx;
            break;
        } else {
            if !(channels[ch_idx as usize].entnum == cl.playernum + 1 as libc::c_int
                && entnum != cl.playernum + 1 as libc::c_int
                && !(channels[ch_idx as usize].sfx).is_null())
            {
                if channels[ch_idx as usize].end - paintedtime < life_left {
                    life_left = channels[ch_idx as usize].end - paintedtime;
                    first_to_die = ch_idx;
                }
            }
            ch_idx += 1;
        }
    }
    if first_to_die == -(1 as libc::c_int) {
        return 0 as *mut channel_t;
    }
    ch = &mut *channels.as_mut_ptr().offset(first_to_die as isize) as *mut channel_t;
    memset(
        ch as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<channel_t>() as libc::c_ulong,
    );
    return ch;
}

#[no_mangle]
pub unsafe extern "C" fn S_SpatializeOrigin(
    mut origin: *mut vec_t,
    mut master_vol: libc::c_float,
    mut dist_mult: libc::c_float,
    mut left_vol: *mut libc::c_int,
    mut right_vol: *mut libc::c_int,
) {
    let mut dot: vec_t = 0.;
    let mut dist: vec_t = 0.;
    let mut lscale: vec_t = 0.;
    let mut rscale: vec_t = 0.;
    let mut scale: vec_t = 0.;
    let mut source_vec: vec3_t = [0.; 3];
    if cls.state as libc::c_uint != ca_active as libc::c_int as libc::c_uint {
        *right_vol = 255 as libc::c_int;
        *left_vol = *right_vol;
        return;
    }
    source_vec[0 as libc::c_int
        as usize] = *origin.offset(0 as libc::c_int as isize)
        - listener_origin[0 as libc::c_int as usize];
    source_vec[1 as libc::c_int
        as usize] = *origin.offset(1 as libc::c_int as isize)
        - listener_origin[1 as libc::c_int as usize];
    source_vec[2 as libc::c_int
        as usize] = *origin.offset(2 as libc::c_int as isize)
        - listener_origin[2 as libc::c_int as usize];
    dist = VectorNormalize(source_vec.as_mut_ptr());
    dist -= 80 as libc::c_int as libc::c_float;
    if dist < 0 as libc::c_int as libc::c_float {
        dist = 0 as libc::c_int as vec_t;
    }
    dist *= dist_mult;
    dot = listener_right[0 as libc::c_int as usize]
        * source_vec[0 as libc::c_int as usize]
        + listener_right[1 as libc::c_int as usize]
        * source_vec[1 as libc::c_int as usize]
        + listener_right[2 as libc::c_int as usize]
        * source_vec[2 as libc::c_int as usize];
    if dma.channels == 1 as libc::c_int || dist_mult == 0. {
        rscale = 1.0f64 as vec_t;
        lscale = 1.0f64 as vec_t;
    } else {
        rscale = (0.5f64 * (1.0f64 + dot as libc::c_double)) as vec_t;
        lscale = (0.5f64 * (1.0f64 - dot as libc::c_double)) as vec_t;
    }
    scale = ((1.0f64 - dist as libc::c_double) * rscale as libc::c_double) as vec_t;
    *right_vol = (master_vol * scale) as libc::c_int;
    if *right_vol < 0 as libc::c_int {
        *right_vol = 0 as libc::c_int;
    }
    scale = ((1.0f64 - dist as libc::c_double) * lscale as libc::c_double) as vec_t;
    *left_vol = (master_vol * scale) as libc::c_int;
    if *left_vol < 0 as libc::c_int {
        *left_vol = 0 as libc::c_int;
    }
}

#[no_mangle]
pub unsafe extern "C" fn S_Spatialize(mut ch: *mut channel_t) {
    let mut origin: vec3_t = [0.; 3];
    if (*ch).entnum == cl.playernum + 1 as libc::c_int {
        (*ch).leftvol = (*ch).master_vol;
        (*ch).rightvol = (*ch).master_vol;
        return;
    }
    if (*ch).fixed_origin as u64 != 0 {
        origin[0 as libc::c_int as usize] = (*ch).origin[0 as libc::c_int as usize];
        origin[1 as libc::c_int as usize] = (*ch).origin[1 as libc::c_int as usize];
        origin[2 as libc::c_int as usize] = (*ch).origin[2 as libc::c_int as usize];
    } else {
        CL_GetEntitySoundOrigin((*ch).entnum, origin.as_mut_ptr());
    }
    S_SpatializeOrigin(
        origin.as_mut_ptr(),
        (*ch).master_vol as libc::c_float,
        (*ch).dist_mult,
        &mut (*ch).leftvol,
        &mut (*ch).rightvol,
    );
}

#[no_mangle]
pub unsafe extern "C" fn S_AllocPlaysound() -> *mut playsound_t {
    let mut ps: *mut playsound_t = 0 as *mut playsound_t;
    ps = s_freeplays.next;
    if ps == &mut s_freeplays as *mut playsound_t {
        return 0 as *mut playsound_t;
    }
    let ref mut fresh1 = (*(*ps).prev).next;
    *fresh1 = (*ps).next;
    let ref mut fresh2 = (*(*ps).next).prev;
    *fresh2 = (*ps).prev;
    return ps;
}

#[no_mangle]
pub unsafe extern "C" fn S_FreePlaysound(mut ps: *mut playsound_t) {
    let ref mut fresh3 = (*(*ps).prev).next;
    *fresh3 = (*ps).next;
    let ref mut fresh4 = (*(*ps).next).prev;
    *fresh4 = (*ps).prev;
    let ref mut fresh5 = (*ps).next;
    *fresh5 = s_freeplays.next;
    let ref mut fresh6 = (*s_freeplays.next).prev;
    *fresh6 = ps;
    let ref mut fresh7 = (*ps).prev;
    *fresh7 = &mut s_freeplays;
    s_freeplays.next = ps;
}

#[no_mangle]
pub unsafe extern "C" fn S_IssuePlaysound(mut ps: *mut playsound_t) {
    let mut ch: *mut channel_t = 0 as *mut channel_t;
    let mut sc: *mut sfxcache_t = 0 as *mut sfxcache_t;
    if (*s_show).value != 0. {
        Com_Printf(
            b"Issue %i\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            (*ps).begin,
        );
    }
    ch = S_PickChannel((*ps).entnum, (*ps).entchannel);
    if ch.is_null() {
        S_FreePlaysound(ps);
        return;
    }
    if (*ps).attenuation == 3 as libc::c_int as libc::c_float {
        (*ch).dist_mult = ((*ps).attenuation as libc::c_double * 0.001f64) as vec_t;
    } else {
        (*ch).dist_mult = ((*ps).attenuation as libc::c_double * 0.0005f64) as vec_t;
    }
    (*ch).master_vol = (*ps).volume as libc::c_int;
    (*ch).entnum = (*ps).entnum;
    (*ch).entchannel = (*ps).entchannel;
    let ref mut fresh8 = (*ch).sfx;
    *fresh8 = (*ps).sfx;
    (*ch).origin[0 as libc::c_int as usize] = (*ps).origin[0 as libc::c_int as usize];
    (*ch).origin[1 as libc::c_int as usize] = (*ps).origin[1 as libc::c_int as usize];
    (*ch).origin[2 as libc::c_int as usize] = (*ps).origin[2 as libc::c_int as usize];
    (*ch).fixed_origin = (*ps).fixed_origin;
    S_Spatialize(ch);
    (*ch).pos = 0 as libc::c_int;
    sc = S_LoadSound((*ch).sfx);
    (*ch).end = paintedtime + (*sc).length;
    S_FreePlaysound(ps);
}

#[no_mangle]
pub unsafe extern "C" fn S_RegisterSexedSound(
    mut ent: *mut entity_state_t,
    mut base: *mut libc::c_char,
) -> *mut sfx_s {
    let mut n: libc::c_int = 0;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut sfx: *mut sfx_s = 0 as *mut sfx_s;
    let mut f: *mut FILE = 0 as *mut FILE;
    let mut model: [libc::c_char; 64] = [0; 64];
    let mut sexedFilename: [libc::c_char; 64] = [0; 64];
    let mut maleFilename: [libc::c_char; 64] = [0; 64];
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
        sexedFilename.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
        b"#players/%s/%s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        model.as_mut_ptr(),
        base.offset(1 as libc::c_int as isize),
    );
    sfx = S_FindName(sexedFilename.as_mut_ptr(), false_0);
    if sfx.is_null() {
        FS_FOpenFile(
            &mut *sexedFilename.as_mut_ptr().offset(1 as libc::c_int as isize),
            &mut f,
        );
        if !f.is_null() {
            FS_FCloseFile(f);
            sfx = S_RegisterSound(sexedFilename.as_mut_ptr());
        } else {
            Com_sprintf(
                maleFilename.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong
                    as libc::c_int,
                b"player/%s/%s\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                b"male\0" as *const u8 as *const libc::c_char,
                base.offset(1 as libc::c_int as isize),
            );
            sfx = S_AliasName(sexedFilename.as_mut_ptr(), maleFilename.as_mut_ptr());
        }
    }
    return sfx;
}

#[no_mangle]
pub unsafe extern "C" fn S_StartSound(
    mut origin: *mut vec_t,
    mut entnum: libc::c_int,
    mut entchannel: libc::c_int,
    mut sfx: *mut sfx_t,
    mut fvol: libc::c_float,
    mut attenuation: libc::c_float,
    mut timeofs: libc::c_float,
) {
    let mut sc: *mut sfxcache_t = 0 as *mut sfxcache_t;
    let mut vol: libc::c_int = 0;
    let mut ps: *mut playsound_t = 0 as *mut playsound_t;
    let mut sort: *mut playsound_t = 0 as *mut playsound_t;
    let mut start: libc::c_int = 0;
    if sound_started == 0 {
        return;
    }
    if sfx.is_null() {
        return;
    }
    if (*sfx).name[0 as libc::c_int as usize] as libc::c_int == '*' as i32 {
        sfx = S_RegisterSexedSound(
            &mut (*cl_entities.as_mut_ptr().offset(entnum as isize)).current,
            ((*sfx).name).as_mut_ptr(),
        );
    }
    sc = S_LoadSound(sfx);
    if sc.is_null() {
        return;
    }
    vol = (fvol * 255 as libc::c_int as libc::c_float) as libc::c_int;
    ps = S_AllocPlaysound();
    if ps.is_null() {
        return;
    }
    if !origin.is_null() {
        (*ps)
            .origin[0 as libc::c_int
            as usize] = *origin.offset(0 as libc::c_int as isize);
        (*ps)
            .origin[1 as libc::c_int
            as usize] = *origin.offset(1 as libc::c_int as isize);
        (*ps)
            .origin[2 as libc::c_int
            as usize] = *origin.offset(2 as libc::c_int as isize);
        (*ps).fixed_origin = true_0;
    } else {
        (*ps).fixed_origin = false_0;
    }
    (*ps).entnum = entnum;
    (*ps).entchannel = entchannel;
    (*ps).attenuation = attenuation;
    (*ps).volume = vol as libc::c_float;
    let ref mut fresh9 = (*ps).sfx;
    *fresh9 = sfx;
    start = (cl.frame.servertime as libc::c_double * 0.001f64
        * dma.speed as libc::c_double + s_beginofs as libc::c_double) as libc::c_int;
    if start < paintedtime {
        start = paintedtime;
        s_beginofs = (start as libc::c_double
            - cl.frame.servertime as libc::c_double * 0.001f64
            * dma.speed as libc::c_double) as libc::c_int;
    } else if start as libc::c_double
        > paintedtime as libc::c_double + 0.3f64 * dma.speed as libc::c_double
    {
        start = (paintedtime as libc::c_double + 0.1f64 * dma.speed as libc::c_double)
            as libc::c_int;
        s_beginofs = (start as libc::c_double
            - cl.frame.servertime as libc::c_double * 0.001f64
            * dma.speed as libc::c_double) as libc::c_int;
    } else {
        s_beginofs -= 10 as libc::c_int;
    }
    if timeofs == 0. {
        (*ps).begin = paintedtime as libc::c_uint;
    } else {
        (*ps)
            .begin = (start as libc::c_float + timeofs * dma.speed as libc::c_float)
            as libc::c_uint;
    }
    sort = s_pendingplays.next;
    while sort != &mut s_pendingplays as *mut playsound_t && (*sort).begin < (*ps).begin
    {
        sort = (*sort).next;
    }
    let ref mut fresh10 = (*ps).next;
    *fresh10 = sort;
    let ref mut fresh11 = (*ps).prev;
    *fresh11 = (*sort).prev;
    let ref mut fresh12 = (*(*ps).next).prev;
    *fresh12 = ps;
    let ref mut fresh13 = (*(*ps).prev).next;
    *fresh13 = ps;
}

#[no_mangle]
pub unsafe extern "C" fn S_StartLocalSound(mut sound: *mut libc::c_char) {
    let mut sfx: *mut sfx_t = 0 as *mut sfx_t;
    if sound_started == 0 {
        return;
    }
    sfx = S_RegisterSound(sound);
    if sfx.is_null() {
        Com_Printf(
            b"S_StartLocalSound: can't cache %s\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            sound,
        );
        return;
    }
    S_StartSound(
        0 as *mut vec_t,
        cl.playernum + 1 as libc::c_int,
        0 as libc::c_int,
        sfx,
        1 as libc::c_int as libc::c_float,
        1 as libc::c_int as libc::c_float,
        0 as libc::c_int as libc::c_float,
    );
}

#[no_mangle]
pub unsafe extern "C" fn S_ClearBuffer() {
    let mut clear: libc::c_int = 0;
    if sound_started == 0 {
        return;
    }
    s_rawend = 0 as libc::c_int;
    if dma.samplebits == 8 as libc::c_int {
        clear = 0x80 as libc::c_int;
    } else {
        clear = 0 as libc::c_int;
    }
    SNDDMA_BeginPainting();
    if !(dma.buffer).is_null() {
        memset(
            dma.buffer as *mut libc::c_void,
            clear,
            (dma.samples * dma.samplebits / 8 as libc::c_int) as libc::c_ulong,
        );
    }
    SNDDMA_Submit();
}

#[no_mangle]
pub unsafe extern "C" fn S_StopAllSounds() {
    let mut i: libc::c_int = 0;
    if sound_started == 0 {
        return;
    }
    memset(
        s_playsounds.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<[playsound_t; 128]>() as libc::c_ulong,
    );
    s_freeplays.prev = &mut s_freeplays;
    s_freeplays.next = s_freeplays.prev;
    s_pendingplays.prev = &mut s_pendingplays;
    s_pendingplays.next = s_pendingplays.prev;
    i = 0 as libc::c_int;
    while i < 128 as libc::c_int {
        s_playsounds[i as usize].prev = &mut s_freeplays;
        s_playsounds[i as usize].next = s_freeplays.next;
        let ref mut fresh14 = (*s_playsounds[i as usize].prev).next;
        *fresh14 = &mut *s_playsounds.as_mut_ptr().offset(i as isize)
            as *mut playsound_t;
        let ref mut fresh15 = (*s_playsounds[i as usize].next).prev;
        *fresh15 = &mut *s_playsounds.as_mut_ptr().offset(i as isize)
            as *mut playsound_t;
        i += 1;
    }
    memset(
        channels.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<[channel_t; 32]>() as libc::c_ulong,
    );
    S_ClearBuffer();
}

#[no_mangle]
pub unsafe extern "C" fn S_AddLoopSounds() {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut sounds: [libc::c_int; 1024] = [0; 1024];
    let mut left: libc::c_int = 0;
    let mut right: libc::c_int = 0;
    let mut left_total: libc::c_int = 0;
    let mut right_total: libc::c_int = 0;
    let mut ch: *mut channel_t = 0 as *mut channel_t;
    let mut sfx: *mut sfx_t = 0 as *mut sfx_t;
    let mut sc: *mut sfxcache_t = 0 as *mut sfxcache_t;
    let mut num: libc::c_int = 0;
    let mut ent: *mut entity_state_t = 0 as *mut entity_state_t;
    if (*cl_paused).value != 0. {
        return;
    }
    if cls.state as libc::c_uint != ca_active as libc::c_int as libc::c_uint {
        return;
    }
    if cl.sound_prepped as u64 == 0 {
        return;
    }
    i = 0 as libc::c_int;
    while i < cl.frame.num_entities {
        num = cl.frame.parse_entities + i & 1024 as libc::c_int - 1 as libc::c_int;
        ent = &mut *cl_parse_entities.as_mut_ptr().offset(num as isize)
            as *mut entity_state_t;
        sounds[i as usize] = (*ent).sound;
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < cl.frame.num_entities {
        if !(sounds[i as usize] == 0) {
            sfx = cl.sound_precache[sounds[i as usize] as usize];
            if !sfx.is_null() {
                sc = (*sfx).cache;
                if !sc.is_null() {
                    num = cl.frame.parse_entities + i
                        & 1024 as libc::c_int - 1 as libc::c_int;
                    ent = &mut *cl_parse_entities.as_mut_ptr().offset(num as isize)
                        as *mut entity_state_t;
                    S_SpatializeOrigin(
                        ((*ent).origin).as_mut_ptr(),
                        255.0f64 as libc::c_float,
                        0.003f64 as libc::c_float,
                        &mut left_total,
                        &mut right_total,
                    );
                    j = i + 1 as libc::c_int;
                    while j < cl.frame.num_entities {
                        if !(sounds[j as usize] != sounds[i as usize]) {
                            sounds[j as usize] = 0 as libc::c_int;
                            num = cl.frame.parse_entities + j
                                & 1024 as libc::c_int - 1 as libc::c_int;
                            ent = &mut *cl_parse_entities
                                .as_mut_ptr()
                                .offset(num as isize) as *mut entity_state_t;
                            S_SpatializeOrigin(
                                ((*ent).origin).as_mut_ptr(),
                                255.0f64 as libc::c_float,
                                0.003f64 as libc::c_float,
                                &mut left,
                                &mut right,
                            );
                            left_total += left;
                            right_total += right;
                        }
                        j += 1;
                    }
                    if !(left_total == 0 as libc::c_int
                        && right_total == 0 as libc::c_int)
                    {
                        ch = S_PickChannel(0 as libc::c_int, 0 as libc::c_int);
                        if ch.is_null() {
                            return;
                        }
                        if left_total > 255 as libc::c_int {
                            left_total = 255 as libc::c_int;
                        }
                        if right_total > 255 as libc::c_int {
                            right_total = 255 as libc::c_int;
                        }
                        (*ch).leftvol = left_total;
                        (*ch).rightvol = right_total;
                        (*ch).autosound = true_0;
                        let ref mut fresh16 = (*ch).sfx;
                        *fresh16 = sfx;
                        (*ch).pos = paintedtime % (*sc).length;
                        (*ch).end = paintedtime + (*sc).length - (*ch).pos;
                    }
                }
            }
        }
        i += 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn S_RawSamples(
    mut samples: libc::c_int,
    mut rate: libc::c_int,
    mut width: libc::c_int,
    mut channels_0: libc::c_int,
    mut data: *mut byte,
) {
    let mut i: libc::c_int = 0;
    let mut src: libc::c_int = 0;
    let mut dst: libc::c_int = 0;
    let mut scale: libc::c_float = 0.;
    if sound_started == 0 {
        return;
    }
    if s_rawend < paintedtime {
        s_rawend = paintedtime;
    }
    scale = rate as libc::c_float / dma.speed as libc::c_float;
    if channels_0 == 2 as libc::c_int && width == 2 as libc::c_int {
        if scale as libc::c_double == 1.0f64 {
            i = 0 as libc::c_int;
            while i < samples {
                dst = s_rawend & 8192 as libc::c_int - 1 as libc::c_int;
                s_rawend += 1;
                s_rawsamples[dst as usize]
                    .left = (LittleShort(
                    *(data as *mut libc::c_short).offset((i * 2 as libc::c_int) as isize),
                ) as libc::c_int) << 8 as libc::c_int;
                s_rawsamples[dst as usize]
                    .right = (LittleShort(
                    *(data as *mut libc::c_short)
                        .offset((i * 2 as libc::c_int + 1 as libc::c_int) as isize),
                ) as libc::c_int) << 8 as libc::c_int;
                i += 1;
            }
        } else {
            i = 0 as libc::c_int;
            loop {
                src = (i as libc::c_float * scale) as libc::c_int;
                if src >= samples {
                    break;
                }
                dst = s_rawend & 8192 as libc::c_int - 1 as libc::c_int;
                s_rawend += 1;
                s_rawsamples[dst as usize]
                    .left = (LittleShort(
                    *(data as *mut libc::c_short)
                        .offset((src * 2 as libc::c_int) as isize),
                ) as libc::c_int) << 8 as libc::c_int;
                s_rawsamples[dst as usize]
                    .right = (LittleShort(
                    *(data as *mut libc::c_short)
                        .offset((src * 2 as libc::c_int + 1 as libc::c_int) as isize),
                ) as libc::c_int) << 8 as libc::c_int;
                i += 1;
            }
        }
    } else if channels_0 == 1 as libc::c_int && width == 2 as libc::c_int {
        i = 0 as libc::c_int;
        loop {
            src = (i as libc::c_float * scale) as libc::c_int;
            if src >= samples {
                break;
            }
            dst = s_rawend & 8192 as libc::c_int - 1 as libc::c_int;
            s_rawend += 1;
            s_rawsamples[dst as usize]
                .left = (LittleShort(*(data as *mut libc::c_short).offset(src as isize))
                as libc::c_int) << 8 as libc::c_int;
            s_rawsamples[dst as usize]
                .right = (LittleShort(*(data as *mut libc::c_short).offset(src as isize))
                as libc::c_int) << 8 as libc::c_int;
            i += 1;
        }
    } else if channels_0 == 2 as libc::c_int && width == 1 as libc::c_int {
        i = 0 as libc::c_int;
        loop {
            src = (i as libc::c_float * scale) as libc::c_int;
            if src >= samples {
                break;
            }
            dst = s_rawend & 8192 as libc::c_int - 1 as libc::c_int;
            s_rawend += 1;
            s_rawsamples[dst as usize]
                .left = (*(data as *mut libc::c_char)
                .offset((src * 2 as libc::c_int) as isize) as libc::c_int)
                << 16 as libc::c_int;
            s_rawsamples[dst as usize]
                .right = (*(data as *mut libc::c_char)
                .offset((src * 2 as libc::c_int + 1 as libc::c_int) as isize)
                as libc::c_int) << 16 as libc::c_int;
            i += 1;
        }
    } else if channels_0 == 1 as libc::c_int && width == 1 as libc::c_int {
        i = 0 as libc::c_int;
        loop {
            src = (i as libc::c_float * scale) as libc::c_int;
            if src >= samples {
                break;
            }
            dst = s_rawend & 8192 as libc::c_int - 1 as libc::c_int;
            s_rawend += 1;
            s_rawsamples[dst as usize]
                .left = (*data.offset(src as isize) as libc::c_int - 128 as libc::c_int)
                << 16 as libc::c_int;
            s_rawsamples[dst as usize]
                .right = (*data.offset(src as isize) as libc::c_int - 128 as libc::c_int)
                << 16 as libc::c_int;
            i += 1;
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn S_Update(
    mut origin: *mut vec_t,
    mut forward: *mut vec_t,
    mut right: *mut vec_t,
    mut up: *mut vec_t,
) {
    let mut i: libc::c_int = 0;
    let mut total: libc::c_int = 0;
    let mut ch: *mut channel_t = 0 as *mut channel_t;
    let mut combine: *mut channel_t = 0 as *mut channel_t;
    if sound_started == 0 {
        return;
    }
    if cls.disable_screen != 0. {
        S_ClearBuffer();
        return;
    }
    if (*s_volume).modified as u64 != 0 {
        S_InitScaletable();
    }
    listener_origin[0 as libc::c_int
        as usize] = *origin.offset(0 as libc::c_int as isize);
    listener_origin[1 as libc::c_int
        as usize] = *origin.offset(1 as libc::c_int as isize);
    listener_origin[2 as libc::c_int
        as usize] = *origin.offset(2 as libc::c_int as isize);
    listener_forward[0 as libc::c_int
        as usize] = *forward.offset(0 as libc::c_int as isize);
    listener_forward[1 as libc::c_int
        as usize] = *forward.offset(1 as libc::c_int as isize);
    listener_forward[2 as libc::c_int
        as usize] = *forward.offset(2 as libc::c_int as isize);
    listener_right[0 as libc::c_int as usize] = *right.offset(0 as libc::c_int as isize);
    listener_right[1 as libc::c_int as usize] = *right.offset(1 as libc::c_int as isize);
    listener_right[2 as libc::c_int as usize] = *right.offset(2 as libc::c_int as isize);
    listener_up[0 as libc::c_int as usize] = *up.offset(0 as libc::c_int as isize);
    listener_up[1 as libc::c_int as usize] = *up.offset(1 as libc::c_int as isize);
    listener_up[2 as libc::c_int as usize] = *up.offset(2 as libc::c_int as isize);
    combine = 0 as *mut channel_t;
    ch = channels.as_mut_ptr();
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        if !((*ch).sfx).is_null() {
            if (*ch).autosound as u64 != 0 {
                memset(
                    ch as *mut libc::c_void,
                    0 as libc::c_int,
                    ::std::mem::size_of::<channel_t>() as libc::c_ulong,
                );
            } else {
                S_Spatialize(ch);
                if (*ch).leftvol == 0 && (*ch).rightvol == 0 {
                    memset(
                        ch as *mut libc::c_void,
                        0 as libc::c_int,
                        ::std::mem::size_of::<channel_t>() as libc::c_ulong,
                    );
                }
            }
        }
        i += 1;
        ch = ch.offset(1);
    }
    S_AddLoopSounds();
    if (*s_show).value != 0. {
        total = 0 as libc::c_int;
        ch = channels.as_mut_ptr();
        i = 0 as libc::c_int;
        while i < 32 as libc::c_int {
            if !((*ch).sfx).is_null() && ((*ch).leftvol != 0 || (*ch).rightvol != 0) {
                Com_Printf(
                    b"%3i %3i %s\n\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    (*ch).leftvol,
                    (*ch).rightvol,
                    ((*(*ch).sfx).name).as_mut_ptr(),
                );
                total += 1;
            }
            i += 1;
            ch = ch.offset(1);
        }
        Com_Printf(
            b"----(%i)---- painted: %i\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            total,
            paintedtime,
        );
    }
    S_Update_();
}

#[no_mangle]
pub unsafe extern "C" fn GetSoundtime() {
    let mut samplepos: libc::c_int = 0;
    static mut buffers: libc::c_int = 0;
    static mut oldsamplepos: libc::c_int = 0;
    let mut fullsamples: libc::c_int = 0;
    fullsamples = dma.samples / dma.channels;
    samplepos = SNDDMA_GetDMAPos();
    if samplepos < oldsamplepos {
        buffers += 1;
        if paintedtime > 0x40000000 as libc::c_int {
            buffers = 0 as libc::c_int;
            paintedtime = fullsamples;
            S_StopAllSounds();
        }
    }
    oldsamplepos = samplepos;
    soundtime = buffers * fullsamples + samplepos / dma.channels;
}

#[no_mangle]
pub unsafe extern "C" fn S_Update_() {
    let mut endtime: libc::c_uint = 0;
    let mut samps: libc::c_int = 0;
    if sound_started == 0 {
        return;
    }
    SNDDMA_BeginPainting();
    if (dma.buffer).is_null() {
        return;
    }
    GetSoundtime();
    if paintedtime < soundtime {
        Com_DPrintf(
            b"S_Update_ : overflow\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        paintedtime = soundtime;
    }
    endtime = (soundtime as libc::c_float
        + (*s_mixahead).value * dma.speed as libc::c_float) as libc::c_uint;
    endtime = endtime
        .wrapping_add(dma.submission_chunk as libc::c_uint)
        .wrapping_sub(1 as libc::c_int as libc::c_uint)
        & !(dma.submission_chunk - 1 as libc::c_int) as libc::c_uint;
    samps = dma.samples >> dma.channels - 1 as libc::c_int;
    if endtime.wrapping_sub(soundtime as libc::c_uint) > samps as libc::c_uint {
        endtime = (soundtime + samps) as libc::c_uint;
    }
    S_PaintChannels(endtime as libc::c_int);
    SNDDMA_Submit();
}

#[no_mangle]
pub unsafe extern "C" fn S_Play() {
    let mut i: libc::c_int = 0;
    let mut name: [libc::c_char; 256] = [0; 256];
    let mut sfx: *mut sfx_t = 0 as *mut sfx_t;
    i = 1 as libc::c_int;
    while i < Cmd_Argc() {
        if (strrchr(Cmd_Argv(i), '.' as i32)).is_null() {
            strcpy(name.as_mut_ptr(), Cmd_Argv(i));
            strcat(name.as_mut_ptr(), b".wav\0" as *const u8 as *const libc::c_char);
        } else {
            strcpy(name.as_mut_ptr(), Cmd_Argv(i));
        }
        sfx = S_RegisterSound(name.as_mut_ptr());
        S_StartSound(
            0 as *mut vec_t,
            cl.playernum + 1 as libc::c_int,
            0 as libc::c_int,
            sfx,
            1.0f64 as libc::c_float,
            1.0f64 as libc::c_float,
            0 as libc::c_int as libc::c_float,
        );
        i += 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn S_SoundList() {
    let mut i: libc::c_int = 0;
    let mut sfx: *mut sfx_t = 0 as *mut sfx_t;
    let mut sc: *mut sfxcache_t = 0 as *mut sfxcache_t;
    let mut size: libc::c_int = 0;
    let mut total: libc::c_int = 0;
    total = 0 as libc::c_int;
    sfx = known_sfx.as_mut_ptr();
    i = 0 as libc::c_int;
    while i < num_sfx {
        if !((*sfx).registration_sequence == 0) {
            sc = (*sfx).cache;
            if !sc.is_null() {
                size = (*sc).length * (*sc).width * ((*sc).stereo + 1 as libc::c_int);
                total += size;
                if (*sc).loopstart >= 0 as libc::c_int {
                    Com_Printf(
                        b"L\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    );
                } else {
                    Com_Printf(
                        b" \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    );
                }
                Com_Printf(
                    b"(%2db) %6i : %s\n\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    (*sc).width * 8 as libc::c_int,
                    size,
                    ((*sfx).name).as_mut_ptr(),
                );
            } else if (*sfx).name[0 as libc::c_int as usize] as libc::c_int == '*' as i32
            {
                Com_Printf(
                    b"  placeholder : %s\n\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    ((*sfx).name).as_mut_ptr(),
                );
            } else {
                Com_Printf(
                    b"  not loaded  : %s\n\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    ((*sfx).name).as_mut_ptr(),
                );
            }
        }
        i += 1;
        sfx = sfx.offset(1);
    }
    Com_Printf(
        b"Total resident: %i\n\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        total,
    );
}
