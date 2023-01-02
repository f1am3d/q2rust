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
    fn atof(__nptr: *const libc::c_char) -> libc::c_double;
    fn atoi(__nptr: *const libc::c_char) -> libc::c_int;
    fn atan(_: libc::c_double) -> libc::c_double;
    fn tan(_: libc::c_double) -> libc::c_double;
    fn CL_RegisterTEntModels();
    fn CL_ParseClientinfo(player: libc::c_int);
    fn CL_LoadClientinfo(ci: *mut clientinfo_t, s: *mut libc::c_char);
    static mut re: refexport_t;
    fn CL_AddEntities();
    static mut cl_timedemo: *mut cvar_t;
    static mut cl_paused: *mut cvar_t;
    static mut cl_add_entities: *mut cvar_t;
    static mut cl_add_particles: *mut cvar_t;
    static mut cl_add_lights: *mut cvar_t;
    static mut cl_add_blend: *mut cvar_t;
    static mut cls: client_static_t;
    static mut cl: client_state_t;
    fn CDAudio_Play(track: libc::c_int, looping: qboolean);
    fn Con_ClearNotify();
    fn SCR_AddDirtyPoint(x: libc::c_int, y: libc::c_int);
    static mut crosshair_height: libc::c_int;
    static mut crosshair_width: libc::c_int;
    static mut crosshair_pic: [libc::c_char; 64];
    static mut scr_vrect: vrect_t;
    fn SCR_TouchPics();
    fn SCR_UpdateScreen();
    static mut viddef: viddef_t;
    fn Sys_SendKeyEvents();
    static mut log_stats_file: *mut FILE;
    static mut log_stats: *mut cvar_t;
    fn Com_Error(code: libc::c_int, fmt: *mut libc::c_char, _: ...);
    fn CM_InlineModel(name: *mut libc::c_char) -> *mut cmodel_t;
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
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn qsort(
        __base: *mut libc::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
    fn VectorScale(in_0: *mut vec_t, scale: vec_t, out: *mut vec_t);
    fn Com_sprintf(
        dest: *mut libc::c_char,
        size: libc::c_int,
        fmt: *mut libc::c_char,
        _: ...
    );
    fn Sys_Milliseconds() -> libc::c_int;
    fn Com_Printf(msg: *mut libc::c_char, _: ...);
    fn Cmd_AddCommand(cmd_name: *mut libc::c_char, function: xcommand_t);
    fn Cmd_Argc() -> libc::c_int;
    fn Cmd_Argv(arg: libc::c_int) -> *mut libc::c_char;
    fn Cvar_Get(
        var_name: *mut libc::c_char,
        value: *mut libc::c_char,
        flags: libc::c_int,
    ) -> *mut cvar_t;
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
pub type __compar_fn_t = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
>;
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
pub struct vrect_s {
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub width: libc::c_int,
    pub height: libc::c_int,
}

pub type vrect_t = vrect_s;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct viddef_t {
    pub width: libc::c_int,
    pub height: libc::c_int,
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
pub static mut gun_frame: libc::c_int = 0;
#[no_mangle]
pub static mut gun_model: *mut model_s = 0 as *const model_s as *mut model_s;
#[no_mangle]
pub static mut crosshair: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut cl_testparticles: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut cl_testentities: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut cl_testlights: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut cl_testblend: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut cl_stats: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut r_numdlights: libc::c_int = 0;
#[no_mangle]
pub static mut r_dlights: [dlight_t; 32] = [dlight_t {
    origin: [0.; 3],
    color: [0.; 3],
    intensity: 0.,
}; 32];
#[no_mangle]
pub static mut r_numentities: libc::c_int = 0;
#[no_mangle]
pub static mut r_entities: [entity_t; 128] = [entity_t {
    model: 0 as *const model_s as *mut model_s,
    angles: [0.; 3],
    origin: [0.; 3],
    frame: 0,
    oldorigin: [0.; 3],
    oldframe: 0,
    backlerp: 0.,
    skinnum: 0,
    lightstyle: 0,
    alpha: 0.,
    skin: 0 as *const image_s as *mut image_s,
    flags: 0,
}; 128];
#[no_mangle]
pub static mut r_numparticles: libc::c_int = 0;
#[no_mangle]
pub static mut r_particles: [particle_t; 4096] = [particle_t {
    origin: [0.; 3],
    color: 0,
    alpha: 0.,
}; 4096];
#[no_mangle]
pub static mut r_lightstyles: [lightstyle_t; 256] = [lightstyle_t {
    rgb: [0.; 3],
    white: 0.,
}; 256];
#[no_mangle]
pub static mut cl_weaponmodels: [[libc::c_char; 64]; 20] = [[0; 64]; 20];
#[no_mangle]
pub static mut num_cl_weaponmodels: libc::c_int = 0;

#[no_mangle]
pub unsafe extern "C" fn V_ClearScene() {
    r_numdlights = 0 as libc::c_int;
    r_numentities = 0 as libc::c_int;
    r_numparticles = 0 as libc::c_int;
}

#[no_mangle]
pub unsafe extern "C" fn V_AddEntity(mut ent: *mut entity_t) {
    if r_numentities >= 128 as libc::c_int {
        return;
    }
    let fresh0 = r_numentities;
    r_numentities = r_numentities + 1;
    r_entities[fresh0 as usize] = *ent;
}

#[no_mangle]
pub unsafe extern "C" fn V_AddParticle(
    mut org: *mut vec_t,
    mut color: libc::c_int,
    mut alpha: libc::c_float,
) {
    let mut p: *mut particle_t = 0 as *mut particle_t;
    if r_numparticles >= 4096 as libc::c_int {
        return;
    }
    let fresh1 = r_numparticles;
    r_numparticles = r_numparticles + 1;
    p = &mut *r_particles.as_mut_ptr().offset(fresh1 as isize) as *mut particle_t;
    (*p).origin[0 as libc::c_int as usize] = *org.offset(0 as libc::c_int as isize);
    (*p).origin[1 as libc::c_int as usize] = *org.offset(1 as libc::c_int as isize);
    (*p).origin[2 as libc::c_int as usize] = *org.offset(2 as libc::c_int as isize);
    (*p).color = color;
    (*p).alpha = alpha;
}

#[no_mangle]
pub unsafe extern "C" fn V_AddLight(
    mut org: *mut vec_t,
    mut intensity: libc::c_float,
    mut r: libc::c_float,
    mut g: libc::c_float,
    mut b: libc::c_float,
) {
    let mut dl: *mut dlight_t = 0 as *mut dlight_t;
    if r_numdlights >= 32 as libc::c_int {
        return;
    }
    let fresh2 = r_numdlights;
    r_numdlights = r_numdlights + 1;
    dl = &mut *r_dlights.as_mut_ptr().offset(fresh2 as isize) as *mut dlight_t;
    (*dl).origin[0 as libc::c_int as usize] = *org.offset(0 as libc::c_int as isize);
    (*dl).origin[1 as libc::c_int as usize] = *org.offset(1 as libc::c_int as isize);
    (*dl).origin[2 as libc::c_int as usize] = *org.offset(2 as libc::c_int as isize);
    (*dl).intensity = intensity;
    (*dl).color[0 as libc::c_int as usize] = r;
    (*dl).color[1 as libc::c_int as usize] = g;
    (*dl).color[2 as libc::c_int as usize] = b;
}

#[no_mangle]
pub unsafe extern "C" fn V_AddLightStyle(
    mut style: libc::c_int,
    mut r: libc::c_float,
    mut g: libc::c_float,
    mut b: libc::c_float,
) {
    let mut ls: *mut lightstyle_t = 0 as *mut lightstyle_t;
    if style < 0 as libc::c_int || style > 256 as libc::c_int {
        Com_Error(
            1 as libc::c_int,
            b"Bad light style %i\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            style,
        );
    }
    ls = &mut *r_lightstyles.as_mut_ptr().offset(style as isize) as *mut lightstyle_t;
    (*ls).white = r + g + b;
    (*ls).rgb[0 as libc::c_int as usize] = r;
    (*ls).rgb[1 as libc::c_int as usize] = g;
    (*ls).rgb[2 as libc::c_int as usize] = b;
}

#[no_mangle]
pub unsafe extern "C" fn V_TestParticles() {
    let mut p: *mut particle_t = 0 as *mut particle_t;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut d: libc::c_float = 0.;
    let mut r: libc::c_float = 0.;
    let mut u: libc::c_float = 0.;
    r_numparticles = 4096 as libc::c_int;
    i = 0 as libc::c_int;
    while i < r_numparticles {
        d = (i as libc::c_double * 0.25f64) as libc::c_float;
        r = (4 as libc::c_int as libc::c_double
            * ((i & 7 as libc::c_int) as libc::c_double - 3.5f64)) as libc::c_float;
        u = (4 as libc::c_int as libc::c_double
            * ((i >> 3 as libc::c_int & 7 as libc::c_int) as libc::c_double - 3.5f64))
            as libc::c_float;
        p = &mut *r_particles.as_mut_ptr().offset(i as isize) as *mut particle_t;
        j = 0 as libc::c_int;
        while j < 3 as libc::c_int {
            (*p)
                .origin[j
                as usize] = cl.refdef.vieworg[j as usize] + cl.v_forward[j as usize] * d
                + cl.v_right[j as usize] * r + cl.v_up[j as usize] * u;
            j += 1;
        }
        (*p).color = 8 as libc::c_int;
        (*p).alpha = (*cl_testparticles).value;
        i += 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn V_TestEntities() {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut f: libc::c_float = 0.;
    let mut r: libc::c_float = 0.;
    let mut ent: *mut entity_t = 0 as *mut entity_t;
    r_numentities = 32 as libc::c_int;
    memset(
        r_entities.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<[entity_t; 128]>() as libc::c_ulong,
    );
    i = 0 as libc::c_int;
    while i < r_numentities {
        ent = &mut *r_entities.as_mut_ptr().offset(i as isize) as *mut entity_t;
        r = (64 as libc::c_int as libc::c_double
            * ((i % 4 as libc::c_int) as libc::c_double - 1.5f64)) as libc::c_float;
        f = (64 as libc::c_int * (i / 4 as libc::c_int) + 128 as libc::c_int)
            as libc::c_float;
        j = 0 as libc::c_int;
        while j < 3 as libc::c_int {
            (*ent)
                .origin[j
                as usize] = cl.refdef.vieworg[j as usize] + cl.v_forward[j as usize] * f
                + cl.v_right[j as usize] * r;
            j += 1;
        }
        let ref mut fresh3 = (*ent).model;
        *fresh3 = cl.baseclientinfo.model;
        let ref mut fresh4 = (*ent).skin;
        *fresh4 = cl.baseclientinfo.skin;
        i += 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn V_TestLights() {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut f: libc::c_float = 0.;
    let mut r: libc::c_float = 0.;
    let mut dl: *mut dlight_t = 0 as *mut dlight_t;
    r_numdlights = 32 as libc::c_int;
    memset(
        r_dlights.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<[dlight_t; 32]>() as libc::c_ulong,
    );
    i = 0 as libc::c_int;
    while i < r_numdlights {
        dl = &mut *r_dlights.as_mut_ptr().offset(i as isize) as *mut dlight_t;
        r = (64 as libc::c_int as libc::c_double
            * ((i % 4 as libc::c_int) as libc::c_double - 1.5f64)) as libc::c_float;
        f = (64 as libc::c_int * (i / 4 as libc::c_int) + 128 as libc::c_int)
            as libc::c_float;
        j = 0 as libc::c_int;
        while j < 3 as libc::c_int {
            (*dl)
                .origin[j
                as usize] = cl.refdef.vieworg[j as usize] + cl.v_forward[j as usize] * f
                + cl.v_right[j as usize] * r;
            j += 1;
        }
        (*dl)
            .color[0 as libc::c_int
            as usize] = (i % 6 as libc::c_int + 1 as libc::c_int & 1 as libc::c_int)
            as vec_t;
        (*dl)
            .color[1 as libc::c_int
            as usize] = ((i % 6 as libc::c_int + 1 as libc::c_int & 2 as libc::c_int)
            >> 1 as libc::c_int) as vec_t;
        (*dl)
            .color[2 as libc::c_int
            as usize] = ((i % 6 as libc::c_int + 1 as libc::c_int & 4 as libc::c_int)
            >> 2 as libc::c_int) as vec_t;
        (*dl).intensity = 200 as libc::c_int as libc::c_float;
        i += 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn CL_PrepRefresh() {
    let mut mapname: [libc::c_char; 32] = [0; 32];
    let mut i: libc::c_int = 0;
    let mut name: [libc::c_char; 64] = [0; 64];
    let mut rotate: libc::c_float = 0.;
    let mut axis: vec3_t = [0.; 3];
    if cl
        .configstrings[(32 as libc::c_int + 1 as libc::c_int)
        as usize][0 as libc::c_int as usize] == 0
    {
        return;
    }
    SCR_AddDirtyPoint(0 as libc::c_int, 0 as libc::c_int);
    SCR_AddDirtyPoint(viddef.width - 1 as libc::c_int, viddef.height - 1 as libc::c_int);
    strcpy(
        mapname.as_mut_ptr(),
        (cl.configstrings[(32 as libc::c_int + 1 as libc::c_int) as usize])
            .as_mut_ptr()
            .offset(5 as libc::c_int as isize),
    );
    mapname[(strlen(mapname.as_mut_ptr()))
        .wrapping_sub(4 as libc::c_int as libc::c_ulong)
        as usize] = 0 as libc::c_int as libc::c_char;
    Com_Printf(
        b"Map: %s\r\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        mapname.as_mut_ptr(),
    );
    SCR_UpdateScreen();
    (re.BeginRegistration).expect("non-null function pointer")(mapname.as_mut_ptr());
    Com_Printf(
        b"                                     \r\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    Com_Printf(b"pics\r\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    SCR_UpdateScreen();
    SCR_TouchPics();
    Com_Printf(
        b"                                     \r\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    CL_RegisterTEntModels();
    num_cl_weaponmodels = 1 as libc::c_int;
    strcpy(
        (cl_weaponmodels[0 as libc::c_int as usize]).as_mut_ptr(),
        b"weapon.md2\0" as *const u8 as *const libc::c_char,
    );
    i = 1 as libc::c_int;
    while i < 256 as libc::c_int
        && cl.configstrings[(32 as libc::c_int + i) as usize][0 as libc::c_int as usize]
        as libc::c_int != 0
    {
        strcpy(
            name.as_mut_ptr(),
            (cl.configstrings[(32 as libc::c_int + i) as usize]).as_mut_ptr(),
        );
        name[37 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        if name[0 as libc::c_int as usize] as libc::c_int != '*' as i32 {
            Com_Printf(
                b"%s\r\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                name.as_mut_ptr(),
            );
        }
        SCR_UpdateScreen();
        Sys_SendKeyEvents();
        if name[0 as libc::c_int as usize] as libc::c_int == '#' as i32 {
            if num_cl_weaponmodels < 20 as libc::c_int {
                strncpy(
                    (cl_weaponmodels[num_cl_weaponmodels as usize]).as_mut_ptr(),
                    (cl.configstrings[(32 as libc::c_int + i) as usize])
                        .as_mut_ptr()
                        .offset(1 as libc::c_int as isize),
                    (::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                );
                num_cl_weaponmodels += 1;
            }
        } else {
            cl
                .model_draw[i
                as usize] = (re.RegisterModel)
                .expect(
                    "non-null function pointer",
                )((cl.configstrings[(32 as libc::c_int + i) as usize]).as_mut_ptr());
            if name[0 as libc::c_int as usize] as libc::c_int == '*' as i32 {
                cl
                    .model_clip[i
                    as usize] = CM_InlineModel(
                    (cl.configstrings[(32 as libc::c_int + i) as usize]).as_mut_ptr(),
                );
            } else {
                cl.model_clip[i as usize] = 0 as *mut cmodel_s;
            }
        }
        if name[0 as libc::c_int as usize] as libc::c_int != '*' as i32 {
            Com_Printf(
                b"                                     \r\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
            );
        }
        i += 1;
    }
    Com_Printf(
        b"images\r\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        i,
    );
    SCR_UpdateScreen();
    i = 1 as libc::c_int;
    while i < 256 as libc::c_int
        && cl
        .configstrings[(32 as libc::c_int + 256 as libc::c_int + 256 as libc::c_int
        + i) as usize][0 as libc::c_int as usize] as libc::c_int != 0
    {
        cl
            .image_precache[i
            as usize] = (re.RegisterPic)
            .expect(
                "non-null function pointer",
            )(
            (cl
                .configstrings[(32 as libc::c_int + 256 as libc::c_int
                + 256 as libc::c_int + i) as usize])
                .as_mut_ptr(),
        );
        Sys_SendKeyEvents();
        i += 1;
    }
    Com_Printf(
        b"                                     \r\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    i = 0 as libc::c_int;
    while i < 256 as libc::c_int {
        if !(cl
            .configstrings[(32 as libc::c_int + 256 as libc::c_int + 256 as libc::c_int
            + 256 as libc::c_int + 256 as libc::c_int + 256 as libc::c_int + i)
            as usize][0 as libc::c_int as usize] == 0)
        {
            Com_Printf(
                b"client %i\r\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                i,
            );
            SCR_UpdateScreen();
            Sys_SendKeyEvents();
            CL_ParseClientinfo(i);
            Com_Printf(
                b"                                     \r\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
            );
        }
        i += 1;
    }
    CL_LoadClientinfo(
        &mut cl.baseclientinfo,
        b"unnamed\\male/grunt\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    Com_Printf(b"sky\r\0" as *const u8 as *const libc::c_char as *mut libc::c_char, i);
    SCR_UpdateScreen();
    rotate = atof((cl.configstrings[4 as libc::c_int as usize]).as_mut_ptr())
        as libc::c_float;
    sscanf(
        (cl.configstrings[3 as libc::c_int as usize]).as_mut_ptr(),
        b"%f %f %f\0" as *const u8 as *const libc::c_char,
        &mut *axis.as_mut_ptr().offset(0 as libc::c_int as isize) as *mut vec_t,
        &mut *axis.as_mut_ptr().offset(1 as libc::c_int as isize) as *mut vec_t,
        &mut *axis.as_mut_ptr().offset(2 as libc::c_int as isize) as *mut vec_t,
    );
    (re.SetSky)
        .expect(
            "non-null function pointer",
        )(
        (cl.configstrings[2 as libc::c_int as usize]).as_mut_ptr(),
        rotate,
        axis.as_mut_ptr(),
    );
    Com_Printf(
        b"                                     \r\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    (re.EndRegistration).expect("non-null function pointer")();
    Con_ClearNotify();
    SCR_UpdateScreen();
    cl.refresh_prepped = true_0;
    cl.force_refdef = true_0;
    CDAudio_Play(
        atoi((cl.configstrings[1 as libc::c_int as usize]).as_mut_ptr()),
        true_0,
    );
}

#[no_mangle]
pub unsafe extern "C" fn CalcFov(
    mut fov_x: libc::c_float,
    mut width: libc::c_float,
    mut height: libc::c_float,
) -> libc::c_float {
    let mut a: libc::c_float = 0.;
    let mut x: libc::c_float = 0.;
    if fov_x < 1 as libc::c_int as libc::c_float
        || fov_x > 179 as libc::c_int as libc::c_float
    {
        Com_Error(
            1 as libc::c_int,
            b"Bad fov: %f\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            fov_x as libc::c_double,
        );
    }
    x = (width as libc::c_double
        / tan(
        (fov_x / 360 as libc::c_int as libc::c_float) as libc::c_double
            * 3.14159265358979323846f64,
    )) as libc::c_float;
    a = atan((height / x) as libc::c_double) as libc::c_float;
    a = ((a * 360 as libc::c_int as libc::c_float) as libc::c_double
        / 3.14159265358979323846f64) as libc::c_float;
    return a;
}

#[no_mangle]
pub unsafe extern "C" fn V_Gun_Next_f() {
    gun_frame += 1;
    Com_Printf(
        b"frame %i\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        gun_frame,
    );
}

#[no_mangle]
pub unsafe extern "C" fn V_Gun_Prev_f() {
    gun_frame -= 1;
    if gun_frame < 0 as libc::c_int {
        gun_frame = 0 as libc::c_int;
    }
    Com_Printf(
        b"frame %i\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        gun_frame,
    );
}

#[no_mangle]
pub unsafe extern "C" fn V_Gun_Model_f() {
    let mut name: [libc::c_char; 64] = [0; 64];
    if Cmd_Argc() != 2 as libc::c_int {
        gun_model = 0 as *mut model_s;
        return;
    }
    Com_sprintf(
        name.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
        b"models/%s/tris.md2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        Cmd_Argv(1 as libc::c_int),
    );
    gun_model = (re.RegisterModel)
        .expect("non-null function pointer")(name.as_mut_ptr());
}

#[no_mangle]
pub unsafe extern "C" fn SCR_DrawCrosshair() {
    if (*crosshair).value == 0. {
        return;
    }
    if (*crosshair).modified as u64 != 0 {
        (*crosshair).modified = false_0;
        SCR_TouchPics();
    }
    if crosshair_pic[0 as libc::c_int as usize] == 0 {
        return;
    }
    (re.DrawPic)
        .expect(
            "non-null function pointer",
        )(
        scr_vrect.x + (scr_vrect.width - crosshair_width >> 1 as libc::c_int),
        scr_vrect.y + (scr_vrect.height - crosshair_height >> 1 as libc::c_int),
        crosshair_pic.as_mut_ptr(),
    );
}

#[no_mangle]
pub unsafe extern "C" fn V_RenderView(mut stereo_separation: libc::c_float) {
    extern "C" {
        #[link_name = "entitycmpfnc"]
        fn entitycmpfnc_0(_: *const entity_t, _: *const entity_t) -> libc::c_int;
    }
    if cls.state as libc::c_uint != ca_active as libc::c_int as libc::c_uint {
        return;
    }
    if cl.refresh_prepped as u64 == 0 {
        return;
    }
    if (*cl_timedemo).value != 0. {
        if cl.timedemo_start == 0 {
            cl.timedemo_start = Sys_Milliseconds();
        }
        cl.timedemo_frames += 1;
    }
    if cl.frame.valid as libc::c_uint != 0
        && (cl.force_refdef as libc::c_uint != 0 || (*cl_paused).value == 0.)
    {
        cl.force_refdef = false_0;
        V_ClearScene();
        CL_AddEntities();
        if (*cl_testparticles).value != 0. {
            V_TestParticles();
        }
        if (*cl_testentities).value != 0. {
            V_TestEntities();
        }
        if (*cl_testlights).value != 0. {
            V_TestLights();
        }
        if (*cl_testblend).value != 0. {
            cl
                .refdef
                .blend[0 as libc::c_int as usize] = 1 as libc::c_int as libc::c_float;
            cl.refdef.blend[1 as libc::c_int as usize] = 0.5f64 as libc::c_float;
            cl.refdef.blend[2 as libc::c_int as usize] = 0.25f64 as libc::c_float;
            cl.refdef.blend[3 as libc::c_int as usize] = 0.5f64 as libc::c_float;
        }
        if stereo_separation != 0 as libc::c_int as libc::c_float {
            let mut tmp: vec3_t = [0.; 3];
            VectorScale((cl.v_right).as_mut_ptr(), stereo_separation, tmp.as_mut_ptr());
            cl
                .refdef
                .vieworg[0 as libc::c_int
                as usize] = cl.refdef.vieworg[0 as libc::c_int as usize]
                + tmp[0 as libc::c_int as usize];
            cl
                .refdef
                .vieworg[1 as libc::c_int
                as usize] = cl.refdef.vieworg[1 as libc::c_int as usize]
                + tmp[1 as libc::c_int as usize];
            cl
                .refdef
                .vieworg[2 as libc::c_int
                as usize] = cl.refdef.vieworg[2 as libc::c_int as usize]
                + tmp[2 as libc::c_int as usize];
        }
        cl
            .refdef
            .vieworg[0 as libc::c_int
            as usize] = (cl.refdef.vieworg[0 as libc::c_int as usize] as libc::c_double
            + 1.0f64 / 16 as libc::c_int as libc::c_double) as libc::c_float;
        cl
            .refdef
            .vieworg[1 as libc::c_int
            as usize] = (cl.refdef.vieworg[1 as libc::c_int as usize] as libc::c_double
            + 1.0f64 / 16 as libc::c_int as libc::c_double) as libc::c_float;
        cl
            .refdef
            .vieworg[2 as libc::c_int
            as usize] = (cl.refdef.vieworg[2 as libc::c_int as usize] as libc::c_double
            + 1.0f64 / 16 as libc::c_int as libc::c_double) as libc::c_float;
        cl.refdef.x = scr_vrect.x;
        cl.refdef.y = scr_vrect.y;
        cl.refdef.width = scr_vrect.width;
        cl.refdef.height = scr_vrect.height;
        cl
            .refdef
            .fov_y = CalcFov(
            cl.refdef.fov_x,
            cl.refdef.width as libc::c_float,
            cl.refdef.height as libc::c_float,
        );
        cl.refdef.time = (cl.time as libc::c_double * 0.001f64) as libc::c_float;
        cl.refdef.areabits = (cl.frame.areabits).as_mut_ptr();
        if (*cl_add_entities).value == 0. {
            r_numentities = 0 as libc::c_int;
        }
        if (*cl_add_particles).value == 0. {
            r_numparticles = 0 as libc::c_int;
        }
        if (*cl_add_lights).value == 0. {
            r_numdlights = 0 as libc::c_int;
        }
        if (*cl_add_blend).value == 0. {
            cl
                .refdef
                .blend[2 as libc::c_int as usize] = 0 as libc::c_int as libc::c_float;
            cl
                .refdef
                .blend[1 as libc::c_int
                as usize] = cl.refdef.blend[2 as libc::c_int as usize];
            cl
                .refdef
                .blend[0 as libc::c_int
                as usize] = cl.refdef.blend[1 as libc::c_int as usize];
        }
        cl.refdef.num_entities = r_numentities;
        cl.refdef.entities = r_entities.as_mut_ptr();
        cl.refdef.num_particles = r_numparticles;
        cl.refdef.particles = r_particles.as_mut_ptr();
        cl.refdef.num_dlights = r_numdlights;
        cl.refdef.dlights = r_dlights.as_mut_ptr();
        cl.refdef.lightstyles = r_lightstyles.as_mut_ptr();
        cl.refdef.rdflags = cl.frame.playerstate.rdflags;
        qsort(
            cl.refdef.entities as *mut libc::c_void,
            cl.refdef.num_entities as size_t,
            ::std::mem::size_of::<entity_t>() as libc::c_ulong,
            ::std::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(*const entity_t, *const entity_t) -> libc::c_int,
                >,
                Option::<
                    unsafe extern "C" fn(
                        *const libc::c_void,
                        *const libc::c_void,
                    ) -> libc::c_int,
                >,
            >(
                Some(
                    entitycmpfnc_0
                        as unsafe extern "C" fn(
                        *const entity_t,
                        *const entity_t,
                    ) -> libc::c_int,
                ),
            ),
        );
    }
    (re.RenderFrame).expect("non-null function pointer")(&mut cl.refdef);
    if (*cl_stats).value != 0. {
        Com_Printf(
            b"ent:%i  lt:%i  part:%i\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            r_numentities,
            r_numdlights,
            r_numparticles,
        );
    }
    if (*log_stats).value != 0. && !log_stats_file.is_null() {
        fprintf(
            log_stats_file,
            b"%i,%i,%i,\0" as *const u8 as *const libc::c_char,
            r_numentities,
            r_numdlights,
            r_numparticles,
        );
    }
    SCR_AddDirtyPoint(scr_vrect.x, scr_vrect.y);
    SCR_AddDirtyPoint(
        scr_vrect.x + scr_vrect.width - 1 as libc::c_int,
        scr_vrect.y + scr_vrect.height - 1 as libc::c_int,
    );
    SCR_DrawCrosshair();
}

#[no_mangle]
pub unsafe extern "C" fn V_Viewpos_f() {
    Com_Printf(
        b"(%i %i %i) : %i\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        cl.refdef.vieworg[0 as libc::c_int as usize] as libc::c_int,
        cl.refdef.vieworg[1 as libc::c_int as usize] as libc::c_int,
        cl.refdef.vieworg[2 as libc::c_int as usize] as libc::c_int,
        cl.refdef.viewangles[1 as libc::c_int as usize] as libc::c_int,
    );
}

#[no_mangle]
pub unsafe extern "C" fn V_Init() {
    Cmd_AddCommand(
        b"gun_next\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        Some(V_Gun_Next_f as unsafe extern "C" fn() -> ()),
    );
    Cmd_AddCommand(
        b"gun_prev\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        Some(V_Gun_Prev_f as unsafe extern "C" fn() -> ()),
    );
    Cmd_AddCommand(
        b"gun_model\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        Some(V_Gun_Model_f as unsafe extern "C" fn() -> ()),
    );
    Cmd_AddCommand(
        b"viewpos\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        Some(V_Viewpos_f as unsafe extern "C" fn() -> ()),
    );
    crosshair = Cvar_Get(
        b"crosshair\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        1 as libc::c_int,
    );
    cl_testblend = Cvar_Get(
        b"cl_testblend\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    cl_testparticles = Cvar_Get(
        b"cl_testparticles\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    cl_testentities = Cvar_Get(
        b"cl_testentities\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    cl_testlights = Cvar_Get(
        b"cl_testlights\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    cl_stats = Cvar_Get(
        b"cl_stats\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
}
