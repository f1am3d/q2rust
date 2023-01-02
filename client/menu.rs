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
    fn toupper(_: libc::c_int) -> libc::c_int;
    fn atoi(__nptr: *const libc::c_char) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn qsort(
        __base: *mut libc::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
    fn fabs(_: libc::c_double) -> libc::c_double;
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
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fread(
        _: *mut libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn fseek(
        __stream: *mut FILE,
        __off: libc::c_long,
        __whence: libc::c_int,
    ) -> libc::c_int;
    fn ftell(__stream: *mut FILE) -> libc::c_long;
    fn COM_Parse(data_p: *mut *mut libc::c_char) -> *mut libc::c_char;
    fn Com_sprintf(
        dest: *mut libc::c_char,
        size: libc::c_int,
        fmt: *mut libc::c_char,
        _: ...
    );
    fn Q_stricmp(s1: *mut libc::c_char, s2: *mut libc::c_char) -> libc::c_int;
    fn va(format: *mut libc::c_char, _: ...) -> *mut libc::c_char;
    fn Sys_Milliseconds() -> libc::c_int;
    fn Sys_FindFirst(
        path: *mut libc::c_char,
        musthave: libc::c_uint,
        canthave: libc::c_uint,
    ) -> *mut libc::c_char;
    fn Sys_FindClose();
    fn Cbuf_AddText(text: *mut libc::c_char);
    fn Cbuf_InsertText(text: *mut libc::c_char);
    fn Cbuf_Execute();
    fn Cmd_AddCommand(cmd_name: *mut libc::c_char, function: xcommand_t);
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
    fn Cvar_SetValue(var_name: *mut libc::c_char, value: libc::c_float);
    fn Cvar_VariableValue(var_name: *mut libc::c_char) -> libc::c_float;
    fn Cvar_VariableString(var_name: *mut libc::c_char) -> *mut libc::c_char;
    fn NET_AdrToString(a: netadr_t) -> *mut libc::c_char;
    fn FS_Gamedir() -> *mut libc::c_char;
    fn FS_NextPath(prevpath: *mut libc::c_char) -> *mut libc::c_char;
    fn FS_LoadFile(
        path: *mut libc::c_char,
        buffer: *mut *mut libc::c_void,
    ) -> libc::c_int;
    fn FS_Read(buffer: *mut libc::c_void, len: libc::c_int, f: *mut FILE);
    fn FS_FreeFile(buffer: *mut libc::c_void);
    fn Com_Error(code: libc::c_int, fmt: *mut libc::c_char, _: ...);
    fn Com_ServerState() -> libc::c_int;
    static mut viddef: viddef_t;
    fn VID_MenuInit();
    fn VID_MenuDraw();
    fn VID_MenuKey(_: libc::c_int) -> *const libc::c_char;
    static mut crosshair: *mut cvar_t;
    fn SCR_DirtyScreen();
    fn S_StartLocalSound(s: *mut libc::c_char);
    static mut keybindings: [*mut libc::c_char; 256];
    fn Key_SetBinding(keynum: libc::c_int, binding: *mut libc::c_char);
    fn Key_ClearStates();
    fn Con_ClearNotify();
    static mut cl: client_state_t;
    static mut cls: client_static_t;
    static mut cl_run: *mut cvar_t;
    static mut lookspring: *mut cvar_t;
    static mut lookstrafe: *mut cvar_t;
    static mut sensitivity: *mut cvar_t;
    static mut m_pitch: *mut cvar_t;
    static mut freelook: *mut cvar_t;
    fn CL_Quit_f();
    static mut re: refexport_t;
    fn CL_PingServers_f();
    fn CL_Snd_Restart_f();
    fn Key_KeynumToString(keynum: libc::c_int) -> *mut libc::c_char;
    fn Field_Key(field: *mut menufield_s, key: libc::c_int) -> qboolean;
    fn Menu_AddItem(menu: *mut menuframework_s, item: *mut libc::c_void);
    fn Menu_AdjustCursor(menu: *mut menuframework_s, dir: libc::c_int);
    fn Menu_Center(menu: *mut menuframework_s);
    fn Menu_Draw(menu: *mut menuframework_s);
    fn Menu_ItemAtCursor(m: *mut menuframework_s) -> *mut libc::c_void;
    fn Menu_SelectItem(s: *mut menuframework_s) -> qboolean;
    fn Menu_SetStatusBar(s: *mut menuframework_s, string: *const libc::c_char);
    fn Menu_SlideItem(s: *mut menuframework_s, dir: libc::c_int);
    fn Menu_DrawString(_: libc::c_int, _: libc::c_int, _: *const libc::c_char);
    static mut in_joystick: *mut cvar_t;
    fn Developer_searchpath(who: libc::c_int) -> libc::c_int;
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

#[derive(Copy, Clone)]
#[repr(C)]
pub struct menulayer_t {
    pub draw: Option::<unsafe extern "C" fn() -> ()>,
    pub key: Option::<unsafe extern "C" fn(libc::c_int) -> *const libc::c_char>,
}

pub type menuframework_s = _tag_menuframework;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _tag_menuframework {
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub cursor: libc::c_int,
    pub nitems: libc::c_int,
    pub nslots: libc::c_int,
    pub items: [*mut libc::c_void; 64],
    pub statusbar: *const libc::c_char,
    pub cursordraw: Option::<unsafe extern "C" fn(*mut _tag_menuframework) -> ()>,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct menufield_s {
    pub generic: menucommon_s,
    pub buffer: [libc::c_char; 80],
    pub cursor: libc::c_int,
    pub length: libc::c_int,
    pub visible_length: libc::c_int,
    pub visible_offset: libc::c_int,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct menucommon_s {
    pub type_0: libc::c_int,
    pub name: *const libc::c_char,
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub parent: *mut menuframework_s,
    pub cursor_offset: libc::c_int,
    pub localdata: [libc::c_int; 4],
    pub flags: libc::c_uint,
    pub statusbar: *const libc::c_char,
    pub callback: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub statusbarfunc: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub ownerdraw: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub cursordraw: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct menuaction_s {
    pub generic: menucommon_s,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct menulist_s {
    pub generic: menucommon_s,
    pub curvalue: libc::c_int,
    pub itemnames: *mut *const libc::c_char,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct menuslider_s {
    pub generic: menucommon_s,
    pub minvalue: libc::c_float,
    pub maxvalue: libc::c_float,
    pub curvalue: libc::c_float,
    pub range: libc::c_float,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct playermodelinfo_s {
    pub nskins: libc::c_int,
    pub skindisplaynames: *mut *mut libc::c_char,
    pub displayname: [libc::c_char; 16],
    pub directory: [libc::c_char; 64],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct menuseparator_s {
    pub generic: menucommon_s,
}

static mut m_main_cursor: libc::c_int = 0;
static mut menu_in_sound: *mut libc::c_char = b"misc/menu1.wav\0" as *const u8
    as *const libc::c_char as *mut libc::c_char;
static mut menu_move_sound: *mut libc::c_char = b"misc/menu2.wav\0" as *const u8
    as *const libc::c_char as *mut libc::c_char;
static mut menu_out_sound: *mut libc::c_char = b"misc/menu3.wav\0" as *const u8
    as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub static mut m_entersound: qboolean = false_0;
#[no_mangle]
pub static mut m_drawfunc: Option::<unsafe extern "C" fn() -> ()> = None;
#[no_mangle]
pub static mut m_keyfunc: Option::<
    unsafe extern "C" fn(libc::c_int) -> *const libc::c_char,
> = None;
#[no_mangle]
pub static mut m_layers: [menulayer_t; 8] = [menulayer_t {
    draw: None,
    key: None,
}; 8];
#[no_mangle]
pub static mut m_menudepth: libc::c_int = 0;

unsafe extern "C" fn M_Banner(mut name: *mut libc::c_char) {
    let mut w: libc::c_int = 0;
    let mut h: libc::c_int = 0;
    (re.DrawGetPicSize).expect("non-null function pointer")(&mut w, &mut h, name);
    (re.DrawPic)
        .expect(
            "non-null function pointer",
        )(
        viddef.width / 2 as libc::c_int - w / 2 as libc::c_int,
        viddef.height / 2 as libc::c_int - 110 as libc::c_int,
        name,
    );
}

#[no_mangle]
pub unsafe extern "C" fn M_PushMenu(
    mut draw: Option::<unsafe extern "C" fn() -> ()>,
    mut key: Option::<unsafe extern "C" fn(libc::c_int) -> *const libc::c_char>,
) {
    let mut i: libc::c_int = 0;
    if Cvar_VariableValue(
        b"maxclients\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) == 1 as libc::c_int as libc::c_float && Com_ServerState() != 0
    {
        Cvar_Set(
            b"paused\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
    i = 0 as libc::c_int;
    while i < m_menudepth {
        if m_layers[i as usize].draw == draw && m_layers[i as usize].key == key {
            m_menudepth = i;
        }
        i += 1;
    }
    if i == m_menudepth {
        if m_menudepth >= 8 as libc::c_int {
            Com_Error(
                0 as libc::c_int,
                b"M_PushMenu: MAX_MENU_DEPTH\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
        }
        m_layers[m_menudepth as usize].draw = m_drawfunc;
        m_layers[m_menudepth as usize].key = m_keyfunc;
        m_menudepth += 1;
    }
    m_drawfunc = draw;
    m_keyfunc = key;
    m_entersound = true_0;
    cls.key_dest = key_menu;
}

#[no_mangle]
pub unsafe extern "C" fn M_ForceMenuOff() {
    m_drawfunc = None;
    m_keyfunc = None;
    cls.key_dest = key_game;
    m_menudepth = 0 as libc::c_int;
    Key_ClearStates();
    Cvar_Set(
        b"paused\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
}

#[no_mangle]
pub unsafe extern "C" fn M_PopMenu() {
    S_StartLocalSound(menu_out_sound);
    if m_menudepth < 1 as libc::c_int {
        Com_Error(
            0 as libc::c_int,
            b"M_PopMenu: depth < 1\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    m_menudepth -= 1;
    m_drawfunc = m_layers[m_menudepth as usize].draw;
    m_keyfunc = m_layers[m_menudepth as usize].key;
    if m_menudepth == 0 {
        M_ForceMenuOff();
    }
}

#[no_mangle]
pub unsafe extern "C" fn Default_MenuKey(
    mut m: *mut menuframework_s,
    mut key: libc::c_int,
) -> *const libc::c_char {
    let mut sound: *const libc::c_char = 0 as *const libc::c_char;
    let mut item: *mut menucommon_s = 0 as *mut menucommon_s;
    if !m.is_null() {
        item = Menu_ItemAtCursor(m) as *mut menucommon_s;
        if !item.is_null() {
            if (*item).type_0 == 5 as libc::c_int {
                if Field_Key(item as *mut menufield_s, key) as u64 != 0 {
                    return 0 as *const libc::c_char;
                }
            }
        }
    }
    match key {
        27 => {
            M_PopMenu();
            return menu_out_sound;
        }
        161 | 128 => {
            if !m.is_null() {
                let ref mut fresh0 = (*m).cursor;
                *fresh0 -= 1;
                Menu_AdjustCursor(m, -(1 as libc::c_int));
                sound = menu_move_sound;
            }
        }
        9 => {
            if !m.is_null() {
                let ref mut fresh1 = (*m).cursor;
                *fresh1 += 1;
                Menu_AdjustCursor(m, 1 as libc::c_int);
                sound = menu_move_sound;
            }
        }
        167 | 129 => {
            if !m.is_null() {
                let ref mut fresh2 = (*m).cursor;
                *fresh2 += 1;
                Menu_AdjustCursor(m, 1 as libc::c_int);
                sound = menu_move_sound;
            }
        }
        163 | 130 => {
            if !m.is_null() {
                Menu_SlideItem(m, -(1 as libc::c_int));
                sound = menu_move_sound;
            }
        }
        165 | 131 => {
            if !m.is_null() {
                Menu_SlideItem(m, 1 as libc::c_int);
                sound = menu_move_sound;
            }
        }
        200 | 201 | 202 | 203 | 204 | 205 | 206 | 207 | 208 | 209 | 210 | 211 | 212 | 213
        | 214 | 215 | 216 | 217 | 218 | 219 | 220 | 221 | 222 | 223 | 224 | 225 | 226
        | 227 | 228 | 229 | 230 | 231 | 232 | 233 | 234 | 235 | 236 | 237 | 238 | 169
        | 13 => {
            if !m.is_null() {
                Menu_SelectItem(m);
            }
            sound = menu_move_sound;
        }
        _ => {}
    }
    return sound;
}

#[no_mangle]
pub unsafe extern "C" fn M_DrawCharacter(
    mut cx: libc::c_int,
    mut cy: libc::c_int,
    mut num: libc::c_int,
) {
    (re.DrawChar)
        .expect(
            "non-null function pointer",
        )(
        cx + (viddef.width - 320 as libc::c_int >> 1 as libc::c_int),
        cy + (viddef.height - 240 as libc::c_int >> 1 as libc::c_int),
        num,
    );
}

#[no_mangle]
pub unsafe extern "C" fn M_Print(
    mut cx: libc::c_int,
    mut cy: libc::c_int,
    mut str: *mut libc::c_char,
) {
    while *str != 0 {
        M_DrawCharacter(cx, cy, *str as libc::c_int + 128 as libc::c_int);
        str = str.offset(1);
        cx += 8 as libc::c_int;
    }
}

#[no_mangle]
pub unsafe extern "C" fn M_PrintWhite(
    mut cx: libc::c_int,
    mut cy: libc::c_int,
    mut str: *mut libc::c_char,
) {
    while *str != 0 {
        M_DrawCharacter(cx, cy, *str as libc::c_int);
        str = str.offset(1);
        cx += 8 as libc::c_int;
    }
}

#[no_mangle]
pub unsafe extern "C" fn M_DrawPic(
    mut x: libc::c_int,
    mut y: libc::c_int,
    mut pic: *mut libc::c_char,
) {
    (re.DrawPic)
        .expect(
            "non-null function pointer",
        )(
        x + (viddef.width - 320 as libc::c_int >> 1 as libc::c_int),
        y + (viddef.height - 240 as libc::c_int >> 1 as libc::c_int),
        pic,
    );
}

#[no_mangle]
pub unsafe extern "C" fn M_DrawCursor(
    mut x: libc::c_int,
    mut y: libc::c_int,
    mut f: libc::c_int,
) {
    let mut cursorname: [libc::c_char; 80] = [0; 80];
    static mut cached: qboolean = false_0;
    if cached as u64 == 0 {
        let mut i: libc::c_int = 0;
        i = 0 as libc::c_int;
        while i < 15 as libc::c_int {
            Com_sprintf(
                cursorname.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 80]>() as libc::c_ulong
                    as libc::c_int,
                b"m_cursor%d\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                i,
            );
            (re.RegisterPic)
                .expect("non-null function pointer")(cursorname.as_mut_ptr());
            i += 1;
        }
        cached = true_0;
    }
    Com_sprintf(
        cursorname.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 80]>() as libc::c_ulong as libc::c_int,
        b"m_cursor%d\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        f,
    );
    (re.DrawPic).expect("non-null function pointer")(x, y, cursorname.as_mut_ptr());
}

#[no_mangle]
pub unsafe extern "C" fn M_DrawTextBox(
    mut x: libc::c_int,
    mut y: libc::c_int,
    mut width: libc::c_int,
    mut lines: libc::c_int,
) {
    let mut cx: libc::c_int = 0;
    let mut cy: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    cx = x;
    cy = y;
    M_DrawCharacter(cx, cy, 1 as libc::c_int);
    n = 0 as libc::c_int;
    while n < lines {
        cy += 8 as libc::c_int;
        M_DrawCharacter(cx, cy, 4 as libc::c_int);
        n += 1;
    }
    M_DrawCharacter(cx, cy + 8 as libc::c_int, 7 as libc::c_int);
    cx += 8 as libc::c_int;
    while width > 0 as libc::c_int {
        cy = y;
        M_DrawCharacter(cx, cy, 2 as libc::c_int);
        n = 0 as libc::c_int;
        while n < lines {
            cy += 8 as libc::c_int;
            M_DrawCharacter(cx, cy, 5 as libc::c_int);
            n += 1;
        }
        M_DrawCharacter(cx, cy + 8 as libc::c_int, 8 as libc::c_int);
        width -= 1 as libc::c_int;
        cx += 8 as libc::c_int;
    }
    cy = y;
    M_DrawCharacter(cx, cy, 3 as libc::c_int);
    n = 0 as libc::c_int;
    while n < lines {
        cy += 8 as libc::c_int;
        M_DrawCharacter(cx, cy, 6 as libc::c_int);
        n += 1;
    }
    M_DrawCharacter(cx, cy + 8 as libc::c_int, 9 as libc::c_int);
}

#[no_mangle]
pub unsafe extern "C" fn M_Main_Draw() {
    let mut i: libc::c_int = 0;
    let mut w: libc::c_int = 0;
    let mut h: libc::c_int = 0;
    let mut ystart: libc::c_int = 0;
    let mut xoffset: libc::c_int = 0;
    let mut widest: libc::c_int = -(1 as libc::c_int);
    let mut totalheight: libc::c_int = 0 as libc::c_int;
    let mut litname: [libc::c_char; 80] = [0; 80];
    let mut names: [*mut libc::c_char; 6] = [
        b"m_main_game\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"m_main_multiplayer\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"m_main_options\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"m_main_video\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"m_main_quit\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as *mut libc::c_char,
    ];
    i = 0 as libc::c_int;
    while !(names[i as usize]).is_null() {
        (re.DrawGetPicSize)
            .expect("non-null function pointer")(&mut w, &mut h, names[i as usize]);
        if w > widest {
            widest = w;
        }
        totalheight += h + 12 as libc::c_int;
        i += 1;
    }
    ystart = viddef.height / 2 as libc::c_int - 110 as libc::c_int;
    xoffset = (viddef.width - widest + 70 as libc::c_int) / 2 as libc::c_int;
    i = 0 as libc::c_int;
    while !(names[i as usize]).is_null() {
        if i != m_main_cursor {
            (re.DrawPic)
                .expect(
                    "non-null function pointer",
                )(
                xoffset,
                ystart + i * 40 as libc::c_int + 13 as libc::c_int,
                names[i as usize],
            );
        }
        i += 1;
    }
    strcpy(litname.as_mut_ptr(), names[m_main_cursor as usize]);
    strcat(litname.as_mut_ptr(), b"_sel\0" as *const u8 as *const libc::c_char);
    (re.DrawPic)
        .expect(
            "non-null function pointer",
        )(
        xoffset,
        ystart + m_main_cursor * 40 as libc::c_int + 13 as libc::c_int,
        litname.as_mut_ptr(),
    );
    M_DrawCursor(
        xoffset - 25 as libc::c_int,
        ystart + m_main_cursor * 40 as libc::c_int + 11 as libc::c_int,
        cls.realtime / 100 as libc::c_int % 15 as libc::c_int,
    );
    (re.DrawGetPicSize)
        .expect(
            "non-null function pointer",
        )(
        &mut w,
        &mut h,
        b"m_main_plaque\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    (re.DrawPic)
        .expect(
            "non-null function pointer",
        )(
        xoffset - 30 as libc::c_int - w,
        ystart,
        b"m_main_plaque\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    (re.DrawPic)
        .expect(
            "non-null function pointer",
        )(
        xoffset - 30 as libc::c_int - w,
        ystart + h + 5 as libc::c_int,
        b"m_main_logo\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
}

#[no_mangle]
pub unsafe extern "C" fn M_Main_Key(mut key: libc::c_int) -> *const libc::c_char {
    let mut sound: *const libc::c_char = menu_move_sound;
    match key {
        27 => {
            M_PopMenu();
        }
        167 | 129 => {
            m_main_cursor += 1;
            if m_main_cursor >= 5 as libc::c_int {
                m_main_cursor = 0 as libc::c_int;
            }
            return sound;
        }
        161 | 128 => {
            m_main_cursor -= 1;
            if m_main_cursor < 0 as libc::c_int {
                m_main_cursor = 5 as libc::c_int - 1 as libc::c_int;
            }
            return sound;
        }
        169 | 13 => {
            m_entersound = true_0;
            match m_main_cursor {
                0 => {
                    M_Menu_Game_f();
                }
                1 => {
                    M_Menu_Multiplayer_f();
                }
                2 => {
                    M_Menu_Options_f();
                }
                3 => {
                    M_Menu_Video_f();
                }
                4 => {
                    M_Menu_Quit_f();
                }
                _ => {}
            }
        }
        _ => {}
    }
    return 0 as *const libc::c_char;
}

#[no_mangle]
pub unsafe extern "C" fn M_Menu_Main_f() {
    M_PushMenu(
        Some(M_Main_Draw as unsafe extern "C" fn() -> ()),
        Some(M_Main_Key as unsafe extern "C" fn(libc::c_int) -> *const libc::c_char),
    );
}

static mut s_multiplayer_menu: menuframework_s = menuframework_s {
    x: 0,
    y: 0,
    cursor: 0,
    nitems: 0,
    nslots: 0,
    items: [0 as *const libc::c_void as *mut libc::c_void; 64],
    statusbar: 0 as *const libc::c_char,
    cursordraw: None,
};
static mut s_join_network_server_action: menuaction_s = menuaction_s {
    generic: menucommon_s {
        type_0: 0,
        name: 0 as *const libc::c_char,
        x: 0,
        y: 0,
        parent: 0 as *const menuframework_s as *mut menuframework_s,
        cursor_offset: 0,
        localdata: [0; 4],
        flags: 0,
        statusbar: 0 as *const libc::c_char,
        callback: None,
        statusbarfunc: None,
        ownerdraw: None,
        cursordraw: None,
    },
};
static mut s_start_network_server_action: menuaction_s = menuaction_s {
    generic: menucommon_s {
        type_0: 0,
        name: 0 as *const libc::c_char,
        x: 0,
        y: 0,
        parent: 0 as *const menuframework_s as *mut menuframework_s,
        cursor_offset: 0,
        localdata: [0; 4],
        flags: 0,
        statusbar: 0 as *const libc::c_char,
        callback: None,
        statusbarfunc: None,
        ownerdraw: None,
        cursordraw: None,
    },
};
static mut s_player_setup_action: menuaction_s = menuaction_s {
    generic: menucommon_s {
        type_0: 0,
        name: 0 as *const libc::c_char,
        x: 0,
        y: 0,
        parent: 0 as *const menuframework_s as *mut menuframework_s,
        cursor_offset: 0,
        localdata: [0; 4],
        flags: 0,
        statusbar: 0 as *const libc::c_char,
        callback: None,
        statusbarfunc: None,
        ownerdraw: None,
        cursordraw: None,
    },
};

unsafe extern "C" fn Multiplayer_MenuDraw() {
    M_Banner(
        b"m_banner_multiplayer\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    Menu_AdjustCursor(&mut s_multiplayer_menu, 1 as libc::c_int);
    Menu_Draw(&mut s_multiplayer_menu);
}

unsafe extern "C" fn PlayerSetupFunc(mut unused: *mut libc::c_void) {
    M_Menu_PlayerConfig_f();
}

unsafe extern "C" fn JoinNetworkServerFunc(mut unused: *mut libc::c_void) {
    M_Menu_JoinServer_f();
}

unsafe extern "C" fn StartNetworkServerFunc(mut unused: *mut libc::c_void) {
    M_Menu_StartServer_f();
}

#[no_mangle]
pub unsafe extern "C" fn Multiplayer_MenuInit() {
    s_multiplayer_menu
        .x = (viddef.width as libc::c_double * 0.50f64
        - 64 as libc::c_int as libc::c_double) as libc::c_int;
    s_multiplayer_menu.nitems = 0 as libc::c_int;
    s_join_network_server_action.generic.type_0 = 2 as libc::c_int;
    s_join_network_server_action.generic.flags = 0x1 as libc::c_int as libc::c_uint;
    s_join_network_server_action.generic.x = 0 as libc::c_int;
    s_join_network_server_action.generic.y = 0 as libc::c_int;
    s_join_network_server_action
        .generic
        .name = b" join network server\0" as *const u8 as *const libc::c_char;
    s_join_network_server_action
        .generic
        .callback = Some(
        JoinNetworkServerFunc as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    s_start_network_server_action.generic.type_0 = 2 as libc::c_int;
    s_start_network_server_action.generic.flags = 0x1 as libc::c_int as libc::c_uint;
    s_start_network_server_action.generic.x = 0 as libc::c_int;
    s_start_network_server_action.generic.y = 10 as libc::c_int;
    s_start_network_server_action
        .generic
        .name = b" start network server\0" as *const u8 as *const libc::c_char;
    s_start_network_server_action
        .generic
        .callback = Some(
        StartNetworkServerFunc as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    s_player_setup_action.generic.type_0 = 2 as libc::c_int;
    s_player_setup_action.generic.flags = 0x1 as libc::c_int as libc::c_uint;
    s_player_setup_action.generic.x = 0 as libc::c_int;
    s_player_setup_action.generic.y = 20 as libc::c_int;
    s_player_setup_action
        .generic
        .name = b" player setup\0" as *const u8 as *const libc::c_char;
    s_player_setup_action
        .generic
        .callback = Some(
        PlayerSetupFunc as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    Menu_AddItem(
        &mut s_multiplayer_menu,
        &mut s_join_network_server_action as *mut menuaction_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_multiplayer_menu,
        &mut s_start_network_server_action as *mut menuaction_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_multiplayer_menu,
        &mut s_player_setup_action as *mut menuaction_s as *mut libc::c_void,
    );
    Menu_SetStatusBar(&mut s_multiplayer_menu, 0 as *const libc::c_char);
    Menu_Center(&mut s_multiplayer_menu);
}

#[no_mangle]
pub unsafe extern "C" fn Multiplayer_MenuKey(
    mut key: libc::c_int,
) -> *const libc::c_char {
    return Default_MenuKey(&mut s_multiplayer_menu, key);
}

#[no_mangle]
pub unsafe extern "C" fn M_Menu_Multiplayer_f() {
    Multiplayer_MenuInit();
    M_PushMenu(
        Some(Multiplayer_MenuDraw as unsafe extern "C" fn() -> ()),
        Some(
            Multiplayer_MenuKey
                as unsafe extern "C" fn(libc::c_int) -> *const libc::c_char,
        ),
    );
}

#[no_mangle]
pub static mut bindnames: [[*mut libc::c_char; 2]; 24] = [
    [
        b"+attack\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"attack\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ],
    [
        b"weapnext\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"next weapon\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ],
    [
        b"+forward\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"walk forward\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ],
    [
        b"+back\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"backpedal\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ],
    [
        b"+left\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"turn left\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ],
    [
        b"+right\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"turn right\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ],
    [
        b"+speed\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"run\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ],
    [
        b"+moveleft\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"step left\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ],
    [
        b"+moveright\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"step right\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ],
    [
        b"+strafe\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"sidestep\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ],
    [
        b"+lookup\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"look up\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ],
    [
        b"+lookdown\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"look down\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ],
    [
        b"centerview\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"center view\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ],
    [
        b"+mlook\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"mouse look\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ],
    [
        b"+klook\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"keyboard look\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ],
    [
        b"+moveup\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"up / jump\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ],
    [
        b"+movedown\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"down / crouch\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ],
    [
        b"inven\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"inventory\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ],
    [
        b"invuse\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"use item\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ],
    [
        b"invdrop\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"drop item\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ],
    [
        b"invprev\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"prev item\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ],
    [
        b"invnext\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"next item\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ],
    [
        b"cmd help\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"help computer\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ],
    [
        0 as *const libc::c_char as *mut libc::c_char,
        0 as *const libc::c_char as *mut libc::c_char,
    ],
];
#[no_mangle]
pub static mut keys_cursor: libc::c_int = 0;
static mut bind_grab: libc::c_int = 0;
static mut s_keys_menu: menuframework_s = menuframework_s {
    x: 0,
    y: 0,
    cursor: 0,
    nitems: 0,
    nslots: 0,
    items: [0 as *const libc::c_void as *mut libc::c_void; 64],
    statusbar: 0 as *const libc::c_char,
    cursordraw: None,
};
static mut s_keys_attack_action: menuaction_s = menuaction_s {
    generic: menucommon_s {
        type_0: 0,
        name: 0 as *const libc::c_char,
        x: 0,
        y: 0,
        parent: 0 as *const menuframework_s as *mut menuframework_s,
        cursor_offset: 0,
        localdata: [0; 4],
        flags: 0,
        statusbar: 0 as *const libc::c_char,
        callback: None,
        statusbarfunc: None,
        ownerdraw: None,
        cursordraw: None,
    },
};
static mut s_keys_change_weapon_action: menuaction_s = menuaction_s {
    generic: menucommon_s {
        type_0: 0,
        name: 0 as *const libc::c_char,
        x: 0,
        y: 0,
        parent: 0 as *const menuframework_s as *mut menuframework_s,
        cursor_offset: 0,
        localdata: [0; 4],
        flags: 0,
        statusbar: 0 as *const libc::c_char,
        callback: None,
        statusbarfunc: None,
        ownerdraw: None,
        cursordraw: None,
    },
};
static mut s_keys_walk_forward_action: menuaction_s = menuaction_s {
    generic: menucommon_s {
        type_0: 0,
        name: 0 as *const libc::c_char,
        x: 0,
        y: 0,
        parent: 0 as *const menuframework_s as *mut menuframework_s,
        cursor_offset: 0,
        localdata: [0; 4],
        flags: 0,
        statusbar: 0 as *const libc::c_char,
        callback: None,
        statusbarfunc: None,
        ownerdraw: None,
        cursordraw: None,
    },
};
static mut s_keys_backpedal_action: menuaction_s = menuaction_s {
    generic: menucommon_s {
        type_0: 0,
        name: 0 as *const libc::c_char,
        x: 0,
        y: 0,
        parent: 0 as *const menuframework_s as *mut menuframework_s,
        cursor_offset: 0,
        localdata: [0; 4],
        flags: 0,
        statusbar: 0 as *const libc::c_char,
        callback: None,
        statusbarfunc: None,
        ownerdraw: None,
        cursordraw: None,
    },
};
static mut s_keys_turn_left_action: menuaction_s = menuaction_s {
    generic: menucommon_s {
        type_0: 0,
        name: 0 as *const libc::c_char,
        x: 0,
        y: 0,
        parent: 0 as *const menuframework_s as *mut menuframework_s,
        cursor_offset: 0,
        localdata: [0; 4],
        flags: 0,
        statusbar: 0 as *const libc::c_char,
        callback: None,
        statusbarfunc: None,
        ownerdraw: None,
        cursordraw: None,
    },
};
static mut s_keys_turn_right_action: menuaction_s = menuaction_s {
    generic: menucommon_s {
        type_0: 0,
        name: 0 as *const libc::c_char,
        x: 0,
        y: 0,
        parent: 0 as *const menuframework_s as *mut menuframework_s,
        cursor_offset: 0,
        localdata: [0; 4],
        flags: 0,
        statusbar: 0 as *const libc::c_char,
        callback: None,
        statusbarfunc: None,
        ownerdraw: None,
        cursordraw: None,
    },
};
static mut s_keys_run_action: menuaction_s = menuaction_s {
    generic: menucommon_s {
        type_0: 0,
        name: 0 as *const libc::c_char,
        x: 0,
        y: 0,
        parent: 0 as *const menuframework_s as *mut menuframework_s,
        cursor_offset: 0,
        localdata: [0; 4],
        flags: 0,
        statusbar: 0 as *const libc::c_char,
        callback: None,
        statusbarfunc: None,
        ownerdraw: None,
        cursordraw: None,
    },
};
static mut s_keys_step_left_action: menuaction_s = menuaction_s {
    generic: menucommon_s {
        type_0: 0,
        name: 0 as *const libc::c_char,
        x: 0,
        y: 0,
        parent: 0 as *const menuframework_s as *mut menuframework_s,
        cursor_offset: 0,
        localdata: [0; 4],
        flags: 0,
        statusbar: 0 as *const libc::c_char,
        callback: None,
        statusbarfunc: None,
        ownerdraw: None,
        cursordraw: None,
    },
};
static mut s_keys_step_right_action: menuaction_s = menuaction_s {
    generic: menucommon_s {
        type_0: 0,
        name: 0 as *const libc::c_char,
        x: 0,
        y: 0,
        parent: 0 as *const menuframework_s as *mut menuframework_s,
        cursor_offset: 0,
        localdata: [0; 4],
        flags: 0,
        statusbar: 0 as *const libc::c_char,
        callback: None,
        statusbarfunc: None,
        ownerdraw: None,
        cursordraw: None,
    },
};
static mut s_keys_sidestep_action: menuaction_s = menuaction_s {
    generic: menucommon_s {
        type_0: 0,
        name: 0 as *const libc::c_char,
        x: 0,
        y: 0,
        parent: 0 as *const menuframework_s as *mut menuframework_s,
        cursor_offset: 0,
        localdata: [0; 4],
        flags: 0,
        statusbar: 0 as *const libc::c_char,
        callback: None,
        statusbarfunc: None,
        ownerdraw: None,
        cursordraw: None,
    },
};
static mut s_keys_look_up_action: menuaction_s = menuaction_s {
    generic: menucommon_s {
        type_0: 0,
        name: 0 as *const libc::c_char,
        x: 0,
        y: 0,
        parent: 0 as *const menuframework_s as *mut menuframework_s,
        cursor_offset: 0,
        localdata: [0; 4],
        flags: 0,
        statusbar: 0 as *const libc::c_char,
        callback: None,
        statusbarfunc: None,
        ownerdraw: None,
        cursordraw: None,
    },
};
static mut s_keys_look_down_action: menuaction_s = menuaction_s {
    generic: menucommon_s {
        type_0: 0,
        name: 0 as *const libc::c_char,
        x: 0,
        y: 0,
        parent: 0 as *const menuframework_s as *mut menuframework_s,
        cursor_offset: 0,
        localdata: [0; 4],
        flags: 0,
        statusbar: 0 as *const libc::c_char,
        callback: None,
        statusbarfunc: None,
        ownerdraw: None,
        cursordraw: None,
    },
};
static mut s_keys_center_view_action: menuaction_s = menuaction_s {
    generic: menucommon_s {
        type_0: 0,
        name: 0 as *const libc::c_char,
        x: 0,
        y: 0,
        parent: 0 as *const menuframework_s as *mut menuframework_s,
        cursor_offset: 0,
        localdata: [0; 4],
        flags: 0,
        statusbar: 0 as *const libc::c_char,
        callback: None,
        statusbarfunc: None,
        ownerdraw: None,
        cursordraw: None,
    },
};
static mut s_keys_mouse_look_action: menuaction_s = menuaction_s {
    generic: menucommon_s {
        type_0: 0,
        name: 0 as *const libc::c_char,
        x: 0,
        y: 0,
        parent: 0 as *const menuframework_s as *mut menuframework_s,
        cursor_offset: 0,
        localdata: [0; 4],
        flags: 0,
        statusbar: 0 as *const libc::c_char,
        callback: None,
        statusbarfunc: None,
        ownerdraw: None,
        cursordraw: None,
    },
};
static mut s_keys_keyboard_look_action: menuaction_s = menuaction_s {
    generic: menucommon_s {
        type_0: 0,
        name: 0 as *const libc::c_char,
        x: 0,
        y: 0,
        parent: 0 as *const menuframework_s as *mut menuframework_s,
        cursor_offset: 0,
        localdata: [0; 4],
        flags: 0,
        statusbar: 0 as *const libc::c_char,
        callback: None,
        statusbarfunc: None,
        ownerdraw: None,
        cursordraw: None,
    },
};
static mut s_keys_move_up_action: menuaction_s = menuaction_s {
    generic: menucommon_s {
        type_0: 0,
        name: 0 as *const libc::c_char,
        x: 0,
        y: 0,
        parent: 0 as *const menuframework_s as *mut menuframework_s,
        cursor_offset: 0,
        localdata: [0; 4],
        flags: 0,
        statusbar: 0 as *const libc::c_char,
        callback: None,
        statusbarfunc: None,
        ownerdraw: None,
        cursordraw: None,
    },
};
static mut s_keys_move_down_action: menuaction_s = menuaction_s {
    generic: menucommon_s {
        type_0: 0,
        name: 0 as *const libc::c_char,
        x: 0,
        y: 0,
        parent: 0 as *const menuframework_s as *mut menuframework_s,
        cursor_offset: 0,
        localdata: [0; 4],
        flags: 0,
        statusbar: 0 as *const libc::c_char,
        callback: None,
        statusbarfunc: None,
        ownerdraw: None,
        cursordraw: None,
    },
};
static mut s_keys_inventory_action: menuaction_s = menuaction_s {
    generic: menucommon_s {
        type_0: 0,
        name: 0 as *const libc::c_char,
        x: 0,
        y: 0,
        parent: 0 as *const menuframework_s as *mut menuframework_s,
        cursor_offset: 0,
        localdata: [0; 4],
        flags: 0,
        statusbar: 0 as *const libc::c_char,
        callback: None,
        statusbarfunc: None,
        ownerdraw: None,
        cursordraw: None,
    },
};
static mut s_keys_inv_use_action: menuaction_s = menuaction_s {
    generic: menucommon_s {
        type_0: 0,
        name: 0 as *const libc::c_char,
        x: 0,
        y: 0,
        parent: 0 as *const menuframework_s as *mut menuframework_s,
        cursor_offset: 0,
        localdata: [0; 4],
        flags: 0,
        statusbar: 0 as *const libc::c_char,
        callback: None,
        statusbarfunc: None,
        ownerdraw: None,
        cursordraw: None,
    },
};
static mut s_keys_inv_drop_action: menuaction_s = menuaction_s {
    generic: menucommon_s {
        type_0: 0,
        name: 0 as *const libc::c_char,
        x: 0,
        y: 0,
        parent: 0 as *const menuframework_s as *mut menuframework_s,
        cursor_offset: 0,
        localdata: [0; 4],
        flags: 0,
        statusbar: 0 as *const libc::c_char,
        callback: None,
        statusbarfunc: None,
        ownerdraw: None,
        cursordraw: None,
    },
};
static mut s_keys_inv_prev_action: menuaction_s = menuaction_s {
    generic: menucommon_s {
        type_0: 0,
        name: 0 as *const libc::c_char,
        x: 0,
        y: 0,
        parent: 0 as *const menuframework_s as *mut menuframework_s,
        cursor_offset: 0,
        localdata: [0; 4],
        flags: 0,
        statusbar: 0 as *const libc::c_char,
        callback: None,
        statusbarfunc: None,
        ownerdraw: None,
        cursordraw: None,
    },
};
static mut s_keys_inv_next_action: menuaction_s = menuaction_s {
    generic: menucommon_s {
        type_0: 0,
        name: 0 as *const libc::c_char,
        x: 0,
        y: 0,
        parent: 0 as *const menuframework_s as *mut menuframework_s,
        cursor_offset: 0,
        localdata: [0; 4],
        flags: 0,
        statusbar: 0 as *const libc::c_char,
        callback: None,
        statusbarfunc: None,
        ownerdraw: None,
        cursordraw: None,
    },
};
static mut s_keys_help_computer_action: menuaction_s = menuaction_s {
    generic: menucommon_s {
        type_0: 0,
        name: 0 as *const libc::c_char,
        x: 0,
        y: 0,
        parent: 0 as *const menuframework_s as *mut menuframework_s,
        cursor_offset: 0,
        localdata: [0; 4],
        flags: 0,
        statusbar: 0 as *const libc::c_char,
        callback: None,
        statusbarfunc: None,
        ownerdraw: None,
        cursordraw: None,
    },
};

unsafe extern "C" fn M_UnbindCommand(mut command: *mut libc::c_char) {
    let mut j: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut b: *mut libc::c_char = 0 as *mut libc::c_char;
    l = strlen(command) as libc::c_int;
    j = 0 as libc::c_int;
    while j < 256 as libc::c_int {
        b = keybindings[j as usize];
        if !b.is_null() {
            if strncmp(b, command, l as libc::c_ulong) == 0 {
                Key_SetBinding(
                    j,
                    b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                );
            }
        }
        j += 1;
    }
}

unsafe extern "C" fn M_FindKeysForCommand(
    mut command: *mut libc::c_char,
    mut twokeys: *mut libc::c_int,
) {
    let mut count: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut b: *mut libc::c_char = 0 as *mut libc::c_char;
    let ref mut fresh3 = *twokeys.offset(1 as libc::c_int as isize);
    *fresh3 = -(1 as libc::c_int);
    *twokeys.offset(0 as libc::c_int as isize) = *fresh3;
    l = strlen(command) as libc::c_int;
    count = 0 as libc::c_int;
    j = 0 as libc::c_int;
    while j < 256 as libc::c_int {
        b = keybindings[j as usize];
        if !b.is_null() {
            if strncmp(b, command, l as libc::c_ulong) == 0 {
                *twokeys.offset(count as isize) = j;
                count += 1;
                if count == 2 as libc::c_int {
                    break;
                }
            }
        }
        j += 1;
    }
}

unsafe extern "C" fn KeyCursorDrawFunc(mut menu: *mut menuframework_s) {
    if bind_grab != 0 {
        (re.DrawChar)
            .expect(
                "non-null function pointer",
            )((*menu).x, (*menu).y + (*menu).cursor * 9 as libc::c_int, '=' as i32);
    } else {
        (re.DrawChar)
            .expect(
                "non-null function pointer",
            )(
            (*menu).x,
            (*menu).y + (*menu).cursor * 9 as libc::c_int,
            12 as libc::c_int
                + (Sys_Milliseconds() / 250 as libc::c_int & 1 as libc::c_int),
        );
    };
}

unsafe extern "C" fn DrawKeyBindingFunc(mut self_0: *mut libc::c_void) {
    let mut keys: [libc::c_int; 2] = [0; 2];
    let mut a: *mut menuaction_s = self_0 as *mut menuaction_s;
    M_FindKeysForCommand(
        bindnames[(*a).generic.localdata[0 as libc::c_int as usize]
            as usize][0 as libc::c_int as usize],
        keys.as_mut_ptr(),
    );
    if keys[0 as libc::c_int as usize] == -(1 as libc::c_int) {
        Menu_DrawString(
            (*a).generic.x + (*(*a).generic.parent).x + 16 as libc::c_int,
            (*a).generic.y + (*(*a).generic.parent).y,
            b"???\0" as *const u8 as *const libc::c_char,
        );
    } else {
        let mut x: libc::c_int = 0;
        let mut name: *const libc::c_char = 0 as *const libc::c_char;
        name = Key_KeynumToString(keys[0 as libc::c_int as usize]);
        Menu_DrawString(
            (*a).generic.x + (*(*a).generic.parent).x + 16 as libc::c_int,
            (*a).generic.y + (*(*a).generic.parent).y,
            name,
        );
        x = (strlen(name)).wrapping_mul(8 as libc::c_int as libc::c_ulong)
            as libc::c_int;
        if keys[1 as libc::c_int as usize] != -(1 as libc::c_int) {
            Menu_DrawString(
                (*a).generic.x + (*(*a).generic.parent).x + 24 as libc::c_int + x,
                (*a).generic.y + (*(*a).generic.parent).y,
                b"or\0" as *const u8 as *const libc::c_char,
            );
            Menu_DrawString(
                (*a).generic.x + (*(*a).generic.parent).x + 48 as libc::c_int + x,
                (*a).generic.y + (*(*a).generic.parent).y,
                Key_KeynumToString(keys[1 as libc::c_int as usize]),
            );
        }
    };
}

unsafe extern "C" fn KeyBindingFunc(mut self_0: *mut libc::c_void) {
    let mut a: *mut menuaction_s = self_0 as *mut menuaction_s;
    let mut keys: [libc::c_int; 2] = [0; 2];
    M_FindKeysForCommand(
        bindnames[(*a).generic.localdata[0 as libc::c_int as usize]
            as usize][0 as libc::c_int as usize],
        keys.as_mut_ptr(),
    );
    if keys[1 as libc::c_int as usize] != -(1 as libc::c_int) {
        M_UnbindCommand(
            bindnames[(*a).generic.localdata[0 as libc::c_int as usize]
                as usize][0 as libc::c_int as usize],
        );
    }
    bind_grab = true_0 as libc::c_int;
    Menu_SetStatusBar(
        &mut s_keys_menu,
        b"press a key or button for this action\0" as *const u8 as *const libc::c_char,
    );
}

unsafe extern "C" fn Keys_MenuInit() {
    let mut y: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    s_keys_menu.x = (viddef.width as libc::c_double * 0.50f64) as libc::c_int;
    s_keys_menu.nitems = 0 as libc::c_int;
    s_keys_menu
        .cursordraw = Some(
        KeyCursorDrawFunc as unsafe extern "C" fn(*mut menuframework_s) -> (),
    );
    s_keys_attack_action.generic.type_0 = 2 as libc::c_int;
    s_keys_attack_action.generic.flags = 0x2 as libc::c_int as libc::c_uint;
    s_keys_attack_action.generic.x = 0 as libc::c_int;
    s_keys_attack_action.generic.y = y;
    s_keys_attack_action
        .generic
        .ownerdraw = Some(
        DrawKeyBindingFunc as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    s_keys_attack_action.generic.localdata[0 as libc::c_int as usize] = i;
    s_keys_attack_action
        .generic
        .name = bindnames[s_keys_attack_action
        .generic
        .localdata[0 as libc::c_int as usize] as usize][1 as libc::c_int as usize];
    s_keys_change_weapon_action.generic.type_0 = 2 as libc::c_int;
    s_keys_change_weapon_action.generic.flags = 0x2 as libc::c_int as libc::c_uint;
    s_keys_change_weapon_action.generic.x = 0 as libc::c_int;
    y += 9 as libc::c_int;
    s_keys_change_weapon_action.generic.y = y;
    s_keys_change_weapon_action
        .generic
        .ownerdraw = Some(
        DrawKeyBindingFunc as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    i += 1;
    s_keys_change_weapon_action.generic.localdata[0 as libc::c_int as usize] = i;
    s_keys_change_weapon_action
        .generic
        .name = bindnames[s_keys_change_weapon_action
        .generic
        .localdata[0 as libc::c_int as usize] as usize][1 as libc::c_int as usize];
    s_keys_walk_forward_action.generic.type_0 = 2 as libc::c_int;
    s_keys_walk_forward_action.generic.flags = 0x2 as libc::c_int as libc::c_uint;
    s_keys_walk_forward_action.generic.x = 0 as libc::c_int;
    y += 9 as libc::c_int;
    s_keys_walk_forward_action.generic.y = y;
    s_keys_walk_forward_action
        .generic
        .ownerdraw = Some(
        DrawKeyBindingFunc as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    i += 1;
    s_keys_walk_forward_action.generic.localdata[0 as libc::c_int as usize] = i;
    s_keys_walk_forward_action
        .generic
        .name = bindnames[s_keys_walk_forward_action
        .generic
        .localdata[0 as libc::c_int as usize] as usize][1 as libc::c_int as usize];
    s_keys_backpedal_action.generic.type_0 = 2 as libc::c_int;
    s_keys_backpedal_action.generic.flags = 0x2 as libc::c_int as libc::c_uint;
    s_keys_backpedal_action.generic.x = 0 as libc::c_int;
    y += 9 as libc::c_int;
    s_keys_backpedal_action.generic.y = y;
    s_keys_backpedal_action
        .generic
        .ownerdraw = Some(
        DrawKeyBindingFunc as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    i += 1;
    s_keys_backpedal_action.generic.localdata[0 as libc::c_int as usize] = i;
    s_keys_backpedal_action
        .generic
        .name = bindnames[s_keys_backpedal_action
        .generic
        .localdata[0 as libc::c_int as usize] as usize][1 as libc::c_int as usize];
    s_keys_turn_left_action.generic.type_0 = 2 as libc::c_int;
    s_keys_turn_left_action.generic.flags = 0x2 as libc::c_int as libc::c_uint;
    s_keys_turn_left_action.generic.x = 0 as libc::c_int;
    y += 9 as libc::c_int;
    s_keys_turn_left_action.generic.y = y;
    s_keys_turn_left_action
        .generic
        .ownerdraw = Some(
        DrawKeyBindingFunc as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    i += 1;
    s_keys_turn_left_action.generic.localdata[0 as libc::c_int as usize] = i;
    s_keys_turn_left_action
        .generic
        .name = bindnames[s_keys_turn_left_action
        .generic
        .localdata[0 as libc::c_int as usize] as usize][1 as libc::c_int as usize];
    s_keys_turn_right_action.generic.type_0 = 2 as libc::c_int;
    s_keys_turn_right_action.generic.flags = 0x2 as libc::c_int as libc::c_uint;
    s_keys_turn_right_action.generic.x = 0 as libc::c_int;
    y += 9 as libc::c_int;
    s_keys_turn_right_action.generic.y = y;
    s_keys_turn_right_action
        .generic
        .ownerdraw = Some(
        DrawKeyBindingFunc as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    i += 1;
    s_keys_turn_right_action.generic.localdata[0 as libc::c_int as usize] = i;
    s_keys_turn_right_action
        .generic
        .name = bindnames[s_keys_turn_right_action
        .generic
        .localdata[0 as libc::c_int as usize] as usize][1 as libc::c_int as usize];
    s_keys_run_action.generic.type_0 = 2 as libc::c_int;
    s_keys_run_action.generic.flags = 0x2 as libc::c_int as libc::c_uint;
    s_keys_run_action.generic.x = 0 as libc::c_int;
    y += 9 as libc::c_int;
    s_keys_run_action.generic.y = y;
    s_keys_run_action
        .generic
        .ownerdraw = Some(
        DrawKeyBindingFunc as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    i += 1;
    s_keys_run_action.generic.localdata[0 as libc::c_int as usize] = i;
    s_keys_run_action
        .generic
        .name = bindnames[s_keys_run_action.generic.localdata[0 as libc::c_int as usize]
        as usize][1 as libc::c_int as usize];
    s_keys_step_left_action.generic.type_0 = 2 as libc::c_int;
    s_keys_step_left_action.generic.flags = 0x2 as libc::c_int as libc::c_uint;
    s_keys_step_left_action.generic.x = 0 as libc::c_int;
    y += 9 as libc::c_int;
    s_keys_step_left_action.generic.y = y;
    s_keys_step_left_action
        .generic
        .ownerdraw = Some(
        DrawKeyBindingFunc as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    i += 1;
    s_keys_step_left_action.generic.localdata[0 as libc::c_int as usize] = i;
    s_keys_step_left_action
        .generic
        .name = bindnames[s_keys_step_left_action
        .generic
        .localdata[0 as libc::c_int as usize] as usize][1 as libc::c_int as usize];
    s_keys_step_right_action.generic.type_0 = 2 as libc::c_int;
    s_keys_step_right_action.generic.flags = 0x2 as libc::c_int as libc::c_uint;
    s_keys_step_right_action.generic.x = 0 as libc::c_int;
    y += 9 as libc::c_int;
    s_keys_step_right_action.generic.y = y;
    s_keys_step_right_action
        .generic
        .ownerdraw = Some(
        DrawKeyBindingFunc as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    i += 1;
    s_keys_step_right_action.generic.localdata[0 as libc::c_int as usize] = i;
    s_keys_step_right_action
        .generic
        .name = bindnames[s_keys_step_right_action
        .generic
        .localdata[0 as libc::c_int as usize] as usize][1 as libc::c_int as usize];
    s_keys_sidestep_action.generic.type_0 = 2 as libc::c_int;
    s_keys_sidestep_action.generic.flags = 0x2 as libc::c_int as libc::c_uint;
    s_keys_sidestep_action.generic.x = 0 as libc::c_int;
    y += 9 as libc::c_int;
    s_keys_sidestep_action.generic.y = y;
    s_keys_sidestep_action
        .generic
        .ownerdraw = Some(
        DrawKeyBindingFunc as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    i += 1;
    s_keys_sidestep_action.generic.localdata[0 as libc::c_int as usize] = i;
    s_keys_sidestep_action
        .generic
        .name = bindnames[s_keys_sidestep_action
        .generic
        .localdata[0 as libc::c_int as usize] as usize][1 as libc::c_int as usize];
    s_keys_look_up_action.generic.type_0 = 2 as libc::c_int;
    s_keys_look_up_action.generic.flags = 0x2 as libc::c_int as libc::c_uint;
    s_keys_look_up_action.generic.x = 0 as libc::c_int;
    y += 9 as libc::c_int;
    s_keys_look_up_action.generic.y = y;
    s_keys_look_up_action
        .generic
        .ownerdraw = Some(
        DrawKeyBindingFunc as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    i += 1;
    s_keys_look_up_action.generic.localdata[0 as libc::c_int as usize] = i;
    s_keys_look_up_action
        .generic
        .name = bindnames[s_keys_look_up_action
        .generic
        .localdata[0 as libc::c_int as usize] as usize][1 as libc::c_int as usize];
    s_keys_look_down_action.generic.type_0 = 2 as libc::c_int;
    s_keys_look_down_action.generic.flags = 0x2 as libc::c_int as libc::c_uint;
    s_keys_look_down_action.generic.x = 0 as libc::c_int;
    y += 9 as libc::c_int;
    s_keys_look_down_action.generic.y = y;
    s_keys_look_down_action
        .generic
        .ownerdraw = Some(
        DrawKeyBindingFunc as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    i += 1;
    s_keys_look_down_action.generic.localdata[0 as libc::c_int as usize] = i;
    s_keys_look_down_action
        .generic
        .name = bindnames[s_keys_look_down_action
        .generic
        .localdata[0 as libc::c_int as usize] as usize][1 as libc::c_int as usize];
    s_keys_center_view_action.generic.type_0 = 2 as libc::c_int;
    s_keys_center_view_action.generic.flags = 0x2 as libc::c_int as libc::c_uint;
    s_keys_center_view_action.generic.x = 0 as libc::c_int;
    y += 9 as libc::c_int;
    s_keys_center_view_action.generic.y = y;
    s_keys_center_view_action
        .generic
        .ownerdraw = Some(
        DrawKeyBindingFunc as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    i += 1;
    s_keys_center_view_action.generic.localdata[0 as libc::c_int as usize] = i;
    s_keys_center_view_action
        .generic
        .name = bindnames[s_keys_center_view_action
        .generic
        .localdata[0 as libc::c_int as usize] as usize][1 as libc::c_int as usize];
    s_keys_mouse_look_action.generic.type_0 = 2 as libc::c_int;
    s_keys_mouse_look_action.generic.flags = 0x2 as libc::c_int as libc::c_uint;
    s_keys_mouse_look_action.generic.x = 0 as libc::c_int;
    y += 9 as libc::c_int;
    s_keys_mouse_look_action.generic.y = y;
    s_keys_mouse_look_action
        .generic
        .ownerdraw = Some(
        DrawKeyBindingFunc as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    i += 1;
    s_keys_mouse_look_action.generic.localdata[0 as libc::c_int as usize] = i;
    s_keys_mouse_look_action
        .generic
        .name = bindnames[s_keys_mouse_look_action
        .generic
        .localdata[0 as libc::c_int as usize] as usize][1 as libc::c_int as usize];
    s_keys_keyboard_look_action.generic.type_0 = 2 as libc::c_int;
    s_keys_keyboard_look_action.generic.flags = 0x2 as libc::c_int as libc::c_uint;
    s_keys_keyboard_look_action.generic.x = 0 as libc::c_int;
    y += 9 as libc::c_int;
    s_keys_keyboard_look_action.generic.y = y;
    s_keys_keyboard_look_action
        .generic
        .ownerdraw = Some(
        DrawKeyBindingFunc as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    i += 1;
    s_keys_keyboard_look_action.generic.localdata[0 as libc::c_int as usize] = i;
    s_keys_keyboard_look_action
        .generic
        .name = bindnames[s_keys_keyboard_look_action
        .generic
        .localdata[0 as libc::c_int as usize] as usize][1 as libc::c_int as usize];
    s_keys_move_up_action.generic.type_0 = 2 as libc::c_int;
    s_keys_move_up_action.generic.flags = 0x2 as libc::c_int as libc::c_uint;
    s_keys_move_up_action.generic.x = 0 as libc::c_int;
    y += 9 as libc::c_int;
    s_keys_move_up_action.generic.y = y;
    s_keys_move_up_action
        .generic
        .ownerdraw = Some(
        DrawKeyBindingFunc as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    i += 1;
    s_keys_move_up_action.generic.localdata[0 as libc::c_int as usize] = i;
    s_keys_move_up_action
        .generic
        .name = bindnames[s_keys_move_up_action
        .generic
        .localdata[0 as libc::c_int as usize] as usize][1 as libc::c_int as usize];
    s_keys_move_down_action.generic.type_0 = 2 as libc::c_int;
    s_keys_move_down_action.generic.flags = 0x2 as libc::c_int as libc::c_uint;
    s_keys_move_down_action.generic.x = 0 as libc::c_int;
    y += 9 as libc::c_int;
    s_keys_move_down_action.generic.y = y;
    s_keys_move_down_action
        .generic
        .ownerdraw = Some(
        DrawKeyBindingFunc as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    i += 1;
    s_keys_move_down_action.generic.localdata[0 as libc::c_int as usize] = i;
    s_keys_move_down_action
        .generic
        .name = bindnames[s_keys_move_down_action
        .generic
        .localdata[0 as libc::c_int as usize] as usize][1 as libc::c_int as usize];
    s_keys_inventory_action.generic.type_0 = 2 as libc::c_int;
    s_keys_inventory_action.generic.flags = 0x2 as libc::c_int as libc::c_uint;
    s_keys_inventory_action.generic.x = 0 as libc::c_int;
    y += 9 as libc::c_int;
    s_keys_inventory_action.generic.y = y;
    s_keys_inventory_action
        .generic
        .ownerdraw = Some(
        DrawKeyBindingFunc as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    i += 1;
    s_keys_inventory_action.generic.localdata[0 as libc::c_int as usize] = i;
    s_keys_inventory_action
        .generic
        .name = bindnames[s_keys_inventory_action
        .generic
        .localdata[0 as libc::c_int as usize] as usize][1 as libc::c_int as usize];
    s_keys_inv_use_action.generic.type_0 = 2 as libc::c_int;
    s_keys_inv_use_action.generic.flags = 0x2 as libc::c_int as libc::c_uint;
    s_keys_inv_use_action.generic.x = 0 as libc::c_int;
    y += 9 as libc::c_int;
    s_keys_inv_use_action.generic.y = y;
    s_keys_inv_use_action
        .generic
        .ownerdraw = Some(
        DrawKeyBindingFunc as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    i += 1;
    s_keys_inv_use_action.generic.localdata[0 as libc::c_int as usize] = i;
    s_keys_inv_use_action
        .generic
        .name = bindnames[s_keys_inv_use_action
        .generic
        .localdata[0 as libc::c_int as usize] as usize][1 as libc::c_int as usize];
    s_keys_inv_drop_action.generic.type_0 = 2 as libc::c_int;
    s_keys_inv_drop_action.generic.flags = 0x2 as libc::c_int as libc::c_uint;
    s_keys_inv_drop_action.generic.x = 0 as libc::c_int;
    y += 9 as libc::c_int;
    s_keys_inv_drop_action.generic.y = y;
    s_keys_inv_drop_action
        .generic
        .ownerdraw = Some(
        DrawKeyBindingFunc as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    i += 1;
    s_keys_inv_drop_action.generic.localdata[0 as libc::c_int as usize] = i;
    s_keys_inv_drop_action
        .generic
        .name = bindnames[s_keys_inv_drop_action
        .generic
        .localdata[0 as libc::c_int as usize] as usize][1 as libc::c_int as usize];
    s_keys_inv_prev_action.generic.type_0 = 2 as libc::c_int;
    s_keys_inv_prev_action.generic.flags = 0x2 as libc::c_int as libc::c_uint;
    s_keys_inv_prev_action.generic.x = 0 as libc::c_int;
    y += 9 as libc::c_int;
    s_keys_inv_prev_action.generic.y = y;
    s_keys_inv_prev_action
        .generic
        .ownerdraw = Some(
        DrawKeyBindingFunc as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    i += 1;
    s_keys_inv_prev_action.generic.localdata[0 as libc::c_int as usize] = i;
    s_keys_inv_prev_action
        .generic
        .name = bindnames[s_keys_inv_prev_action
        .generic
        .localdata[0 as libc::c_int as usize] as usize][1 as libc::c_int as usize];
    s_keys_inv_next_action.generic.type_0 = 2 as libc::c_int;
    s_keys_inv_next_action.generic.flags = 0x2 as libc::c_int as libc::c_uint;
    s_keys_inv_next_action.generic.x = 0 as libc::c_int;
    y += 9 as libc::c_int;
    s_keys_inv_next_action.generic.y = y;
    s_keys_inv_next_action
        .generic
        .ownerdraw = Some(
        DrawKeyBindingFunc as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    i += 1;
    s_keys_inv_next_action.generic.localdata[0 as libc::c_int as usize] = i;
    s_keys_inv_next_action
        .generic
        .name = bindnames[s_keys_inv_next_action
        .generic
        .localdata[0 as libc::c_int as usize] as usize][1 as libc::c_int as usize];
    s_keys_help_computer_action.generic.type_0 = 2 as libc::c_int;
    s_keys_help_computer_action.generic.flags = 0x2 as libc::c_int as libc::c_uint;
    s_keys_help_computer_action.generic.x = 0 as libc::c_int;
    y += 9 as libc::c_int;
    s_keys_help_computer_action.generic.y = y;
    s_keys_help_computer_action
        .generic
        .ownerdraw = Some(
        DrawKeyBindingFunc as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    i += 1;
    s_keys_help_computer_action.generic.localdata[0 as libc::c_int as usize] = i;
    s_keys_help_computer_action
        .generic
        .name = bindnames[s_keys_help_computer_action
        .generic
        .localdata[0 as libc::c_int as usize] as usize][1 as libc::c_int as usize];
    Menu_AddItem(
        &mut s_keys_menu,
        &mut s_keys_attack_action as *mut menuaction_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_keys_menu,
        &mut s_keys_change_weapon_action as *mut menuaction_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_keys_menu,
        &mut s_keys_walk_forward_action as *mut menuaction_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_keys_menu,
        &mut s_keys_backpedal_action as *mut menuaction_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_keys_menu,
        &mut s_keys_turn_left_action as *mut menuaction_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_keys_menu,
        &mut s_keys_turn_right_action as *mut menuaction_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_keys_menu,
        &mut s_keys_run_action as *mut menuaction_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_keys_menu,
        &mut s_keys_step_left_action as *mut menuaction_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_keys_menu,
        &mut s_keys_step_right_action as *mut menuaction_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_keys_menu,
        &mut s_keys_sidestep_action as *mut menuaction_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_keys_menu,
        &mut s_keys_look_up_action as *mut menuaction_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_keys_menu,
        &mut s_keys_look_down_action as *mut menuaction_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_keys_menu,
        &mut s_keys_center_view_action as *mut menuaction_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_keys_menu,
        &mut s_keys_mouse_look_action as *mut menuaction_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_keys_menu,
        &mut s_keys_keyboard_look_action as *mut menuaction_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_keys_menu,
        &mut s_keys_move_up_action as *mut menuaction_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_keys_menu,
        &mut s_keys_move_down_action as *mut menuaction_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_keys_menu,
        &mut s_keys_inventory_action as *mut menuaction_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_keys_menu,
        &mut s_keys_inv_use_action as *mut menuaction_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_keys_menu,
        &mut s_keys_inv_drop_action as *mut menuaction_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_keys_menu,
        &mut s_keys_inv_prev_action as *mut menuaction_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_keys_menu,
        &mut s_keys_inv_next_action as *mut menuaction_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_keys_menu,
        &mut s_keys_help_computer_action as *mut menuaction_s as *mut libc::c_void,
    );
    Menu_SetStatusBar(
        &mut s_keys_menu,
        b"enter to change, backspace to clear\0" as *const u8 as *const libc::c_char,
    );
    Menu_Center(&mut s_keys_menu);
}

unsafe extern "C" fn Keys_MenuDraw() {
    Menu_AdjustCursor(&mut s_keys_menu, 1 as libc::c_int);
    Menu_Draw(&mut s_keys_menu);
}

unsafe extern "C" fn Keys_MenuKey(mut key: libc::c_int) -> *const libc::c_char {
    let mut item: *mut menuaction_s = Menu_ItemAtCursor(&mut s_keys_menu)
        as *mut menuaction_s;
    if bind_grab != 0 {
        if key != 27 as libc::c_int && key != '`' as i32 {
            let mut cmd: [libc::c_char; 1024] = [0; 1024];
            Com_sprintf(
                cmd.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong
                    as libc::c_int,
                b"bind \"%s\" \"%s\"\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                Key_KeynumToString(key),
                bindnames[(*item).generic.localdata[0 as libc::c_int as usize]
                    as usize][0 as libc::c_int as usize],
            );
            Cbuf_InsertText(cmd.as_mut_ptr());
        }
        Menu_SetStatusBar(
            &mut s_keys_menu,
            b"enter to change, backspace to clear\0" as *const u8 as *const libc::c_char,
        );
        bind_grab = false_0 as libc::c_int;
        return menu_out_sound;
    }
    match key {
        169 | 13 => {
            KeyBindingFunc(item as *mut libc::c_void);
            return menu_in_sound;
        }
        148 => {}
        127 | 171 => {}
        _ => return Default_MenuKey(&mut s_keys_menu, key),
    }
    M_UnbindCommand(
        bindnames[(*item).generic.localdata[0 as libc::c_int as usize]
            as usize][0 as libc::c_int as usize],
    );
    return menu_out_sound;
}

#[no_mangle]
pub unsafe extern "C" fn M_Menu_Keys_f() {
    Keys_MenuInit();
    M_PushMenu(
        Some(Keys_MenuDraw as unsafe extern "C" fn() -> ()),
        Some(Keys_MenuKey as unsafe extern "C" fn(libc::c_int) -> *const libc::c_char),
    );
}

static mut win_noalttab: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
static mut s_options_menu: menuframework_s = menuframework_s {
    x: 0,
    y: 0,
    cursor: 0,
    nitems: 0,
    nslots: 0,
    items: [0 as *const libc::c_void as *mut libc::c_void; 64],
    statusbar: 0 as *const libc::c_char,
    cursordraw: None,
};
static mut s_options_defaults_action: menuaction_s = menuaction_s {
    generic: menucommon_s {
        type_0: 0,
        name: 0 as *const libc::c_char,
        x: 0,
        y: 0,
        parent: 0 as *const menuframework_s as *mut menuframework_s,
        cursor_offset: 0,
        localdata: [0; 4],
        flags: 0,
        statusbar: 0 as *const libc::c_char,
        callback: None,
        statusbarfunc: None,
        ownerdraw: None,
        cursordraw: None,
    },
};
static mut s_options_customize_options_action: menuaction_s = menuaction_s {
    generic: menucommon_s {
        type_0: 0,
        name: 0 as *const libc::c_char,
        x: 0,
        y: 0,
        parent: 0 as *const menuframework_s as *mut menuframework_s,
        cursor_offset: 0,
        localdata: [0; 4],
        flags: 0,
        statusbar: 0 as *const libc::c_char,
        callback: None,
        statusbarfunc: None,
        ownerdraw: None,
        cursordraw: None,
    },
};
static mut s_options_sensitivity_slider: menuslider_s = menuslider_s {
    generic: menucommon_s {
        type_0: 0,
        name: 0 as *const libc::c_char,
        x: 0,
        y: 0,
        parent: 0 as *const menuframework_s as *mut menuframework_s,
        cursor_offset: 0,
        localdata: [0; 4],
        flags: 0,
        statusbar: 0 as *const libc::c_char,
        callback: None,
        statusbarfunc: None,
        ownerdraw: None,
        cursordraw: None,
    },
    minvalue: 0.,
    maxvalue: 0.,
    curvalue: 0.,
    range: 0.,
};
static mut s_options_freelook_box: menulist_s = menulist_s {
    generic: menucommon_s {
        type_0: 0,
        name: 0 as *const libc::c_char,
        x: 0,
        y: 0,
        parent: 0 as *const menuframework_s as *mut menuframework_s,
        cursor_offset: 0,
        localdata: [0; 4],
        flags: 0,
        statusbar: 0 as *const libc::c_char,
        callback: None,
        statusbarfunc: None,
        ownerdraw: None,
        cursordraw: None,
    },
    curvalue: 0,
    itemnames: 0 as *const *const libc::c_char as *mut *const libc::c_char,
};
static mut s_options_noalttab_box: menulist_s = menulist_s {
    generic: menucommon_s {
        type_0: 0,
        name: 0 as *const libc::c_char,
        x: 0,
        y: 0,
        parent: 0 as *const menuframework_s as *mut menuframework_s,
        cursor_offset: 0,
        localdata: [0; 4],
        flags: 0,
        statusbar: 0 as *const libc::c_char,
        callback: None,
        statusbarfunc: None,
        ownerdraw: None,
        cursordraw: None,
    },
    curvalue: 0,
    itemnames: 0 as *const *const libc::c_char as *mut *const libc::c_char,
};
static mut s_options_alwaysrun_box: menulist_s = menulist_s {
    generic: menucommon_s {
        type_0: 0,
        name: 0 as *const libc::c_char,
        x: 0,
        y: 0,
        parent: 0 as *const menuframework_s as *mut menuframework_s,
        cursor_offset: 0,
        localdata: [0; 4],
        flags: 0,
        statusbar: 0 as *const libc::c_char,
        callback: None,
        statusbarfunc: None,
        ownerdraw: None,
        cursordraw: None,
    },
    curvalue: 0,
    itemnames: 0 as *const *const libc::c_char as *mut *const libc::c_char,
};
static mut s_options_invertmouse_box: menulist_s = menulist_s {
    generic: menucommon_s {
        type_0: 0,
        name: 0 as *const libc::c_char,
        x: 0,
        y: 0,
        parent: 0 as *const menuframework_s as *mut menuframework_s,
        cursor_offset: 0,
        localdata: [0; 4],
        flags: 0,
        statusbar: 0 as *const libc::c_char,
        callback: None,
        statusbarfunc: None,
        ownerdraw: None,
        cursordraw: None,
    },
    curvalue: 0,
    itemnames: 0 as *const *const libc::c_char as *mut *const libc::c_char,
};
static mut s_options_lookspring_box: menulist_s = menulist_s {
    generic: menucommon_s {
        type_0: 0,
        name: 0 as *const libc::c_char,
        x: 0,
        y: 0,
        parent: 0 as *const menuframework_s as *mut menuframework_s,
        cursor_offset: 0,
        localdata: [0; 4],
        flags: 0,
        statusbar: 0 as *const libc::c_char,
        callback: None,
        statusbarfunc: None,
        ownerdraw: None,
        cursordraw: None,
    },
    curvalue: 0,
    itemnames: 0 as *const *const libc::c_char as *mut *const libc::c_char,
};
static mut s_options_lookstrafe_box: menulist_s = menulist_s {
    generic: menucommon_s {
        type_0: 0,
        name: 0 as *const libc::c_char,
        x: 0,
        y: 0,
        parent: 0 as *const menuframework_s as *mut menuframework_s,
        cursor_offset: 0,
        localdata: [0; 4],
        flags: 0,
        statusbar: 0 as *const libc::c_char,
        callback: None,
        statusbarfunc: None,
        ownerdraw: None,
        cursordraw: None,
    },
    curvalue: 0,
    itemnames: 0 as *const *const libc::c_char as *mut *const libc::c_char,
};
static mut s_options_crosshair_box: menulist_s = menulist_s {
    generic: menucommon_s {
        type_0: 0,
        name: 0 as *const libc::c_char,
        x: 0,
        y: 0,
        parent: 0 as *const menuframework_s as *mut menuframework_s,
        cursor_offset: 0,
        localdata: [0; 4],
        flags: 0,
        statusbar: 0 as *const libc::c_char,
        callback: None,
        statusbarfunc: None,
        ownerdraw: None,
        cursordraw: None,
    },
    curvalue: 0,
    itemnames: 0 as *const *const libc::c_char as *mut *const libc::c_char,
};
static mut s_options_sfxvolume_slider: menuslider_s = menuslider_s {
    generic: menucommon_s {
        type_0: 0,
        name: 0 as *const libc::c_char,
        x: 0,
        y: 0,
        parent: 0 as *const menuframework_s as *mut menuframework_s,
        cursor_offset: 0,
        localdata: [0; 4],
        flags: 0,
        statusbar: 0 as *const libc::c_char,
        callback: None,
        statusbarfunc: None,
        ownerdraw: None,
        cursordraw: None,
    },
    minvalue: 0.,
    maxvalue: 0.,
    curvalue: 0.,
    range: 0.,
};
static mut s_options_joystick_box: menulist_s = menulist_s {
    generic: menucommon_s {
        type_0: 0,
        name: 0 as *const libc::c_char,
        x: 0,
        y: 0,
        parent: 0 as *const menuframework_s as *mut menuframework_s,
        cursor_offset: 0,
        localdata: [0; 4],
        flags: 0,
        statusbar: 0 as *const libc::c_char,
        callback: None,
        statusbarfunc: None,
        ownerdraw: None,
        cursordraw: None,
    },
    curvalue: 0,
    itemnames: 0 as *const *const libc::c_char as *mut *const libc::c_char,
};
static mut s_options_cdvolume_box: menulist_s = menulist_s {
    generic: menucommon_s {
        type_0: 0,
        name: 0 as *const libc::c_char,
        x: 0,
        y: 0,
        parent: 0 as *const menuframework_s as *mut menuframework_s,
        cursor_offset: 0,
        localdata: [0; 4],
        flags: 0,
        statusbar: 0 as *const libc::c_char,
        callback: None,
        statusbarfunc: None,
        ownerdraw: None,
        cursordraw: None,
    },
    curvalue: 0,
    itemnames: 0 as *const *const libc::c_char as *mut *const libc::c_char,
};
static mut s_options_quality_list: menulist_s = menulist_s {
    generic: menucommon_s {
        type_0: 0,
        name: 0 as *const libc::c_char,
        x: 0,
        y: 0,
        parent: 0 as *const menuframework_s as *mut menuframework_s,
        cursor_offset: 0,
        localdata: [0; 4],
        flags: 0,
        statusbar: 0 as *const libc::c_char,
        callback: None,
        statusbarfunc: None,
        ownerdraw: None,
        cursordraw: None,
    },
    curvalue: 0,
    itemnames: 0 as *const *const libc::c_char as *mut *const libc::c_char,
};
static mut s_options_compatibility_list: menulist_s = menulist_s {
    generic: menucommon_s {
        type_0: 0,
        name: 0 as *const libc::c_char,
        x: 0,
        y: 0,
        parent: 0 as *const menuframework_s as *mut menuframework_s,
        cursor_offset: 0,
        localdata: [0; 4],
        flags: 0,
        statusbar: 0 as *const libc::c_char,
        callback: None,
        statusbarfunc: None,
        ownerdraw: None,
        cursordraw: None,
    },
    curvalue: 0,
    itemnames: 0 as *const *const libc::c_char as *mut *const libc::c_char,
};
static mut s_options_console_action: menulist_s = menulist_s {
    generic: menucommon_s {
        type_0: 0,
        name: 0 as *const libc::c_char,
        x: 0,
        y: 0,
        parent: 0 as *const menuframework_s as *mut menuframework_s,
        cursor_offset: 0,
        localdata: [0; 4],
        flags: 0,
        statusbar: 0 as *const libc::c_char,
        callback: None,
        statusbarfunc: None,
        ownerdraw: None,
        cursordraw: None,
    },
    curvalue: 0,
    itemnames: 0 as *const *const libc::c_char as *mut *const libc::c_char,
};

unsafe extern "C" fn CrosshairFunc(mut unused: *mut libc::c_void) {
    Cvar_SetValue(
        b"crosshair\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        s_options_crosshair_box.curvalue as libc::c_float,
    );
}

unsafe extern "C" fn JoystickFunc(mut unused: *mut libc::c_void) {
    Cvar_SetValue(
        b"in_joystick\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        s_options_joystick_box.curvalue as libc::c_float,
    );
}

unsafe extern "C" fn CustomizeControlsFunc(mut unused: *mut libc::c_void) {
    M_Menu_Keys_f();
}

unsafe extern "C" fn AlwaysRunFunc(mut unused: *mut libc::c_void) {
    Cvar_SetValue(
        b"cl_run\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        s_options_alwaysrun_box.curvalue as libc::c_float,
    );
}

unsafe extern "C" fn FreeLookFunc(mut unused: *mut libc::c_void) {
    Cvar_SetValue(
        b"freelook\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        s_options_freelook_box.curvalue as libc::c_float,
    );
}

unsafe extern "C" fn MouseSpeedFunc(mut unused: *mut libc::c_void) {
    Cvar_SetValue(
        b"sensitivity\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        s_options_sensitivity_slider.curvalue / 2.0f32,
    );
}

unsafe extern "C" fn ClampCvar(
    mut min: libc::c_float,
    mut max: libc::c_float,
    mut value: libc::c_float,
) -> libc::c_float {
    if value < min {
        return min;
    }
    if value > max {
        return max;
    }
    return value;
}

unsafe extern "C" fn ControlsSetMenuItemValues() {
    s_options_sfxvolume_slider
        .curvalue = Cvar_VariableValue(
        b"s_volume\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) * 10 as libc::c_int as libc::c_float;
    s_options_cdvolume_box
        .curvalue = (Cvar_VariableValue(
        b"cd_nocd\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) == 0.) as libc::c_int;
    s_options_quality_list
        .curvalue = (Cvar_VariableValue(
        b"s_loadas8bit\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) == 0.) as libc::c_int;
    s_options_sensitivity_slider
        .curvalue = (*sensitivity).value * 2 as libc::c_int as libc::c_float;
    Cvar_SetValue(
        b"cl_run\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ClampCvar(
            0 as libc::c_int as libc::c_float,
            1 as libc::c_int as libc::c_float,
            (*cl_run).value,
        ),
    );
    s_options_alwaysrun_box.curvalue = (*cl_run).value as libc::c_int;
    s_options_invertmouse_box
        .curvalue = ((*m_pitch).value < 0 as libc::c_int as libc::c_float)
        as libc::c_int;
    Cvar_SetValue(
        b"lookspring\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ClampCvar(
            0 as libc::c_int as libc::c_float,
            1 as libc::c_int as libc::c_float,
            (*lookspring).value,
        ),
    );
    s_options_lookspring_box.curvalue = (*lookspring).value as libc::c_int;
    Cvar_SetValue(
        b"lookstrafe\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ClampCvar(
            0 as libc::c_int as libc::c_float,
            1 as libc::c_int as libc::c_float,
            (*lookstrafe).value,
        ),
    );
    s_options_lookstrafe_box.curvalue = (*lookstrafe).value as libc::c_int;
    Cvar_SetValue(
        b"freelook\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ClampCvar(
            0 as libc::c_int as libc::c_float,
            1 as libc::c_int as libc::c_float,
            (*freelook).value,
        ),
    );
    s_options_freelook_box.curvalue = (*freelook).value as libc::c_int;
    Cvar_SetValue(
        b"crosshair\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ClampCvar(
            0 as libc::c_int as libc::c_float,
            3 as libc::c_int as libc::c_float,
            (*crosshair).value,
        ),
    );
    s_options_crosshair_box.curvalue = (*crosshair).value as libc::c_int;
    Cvar_SetValue(
        b"in_joystick\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ClampCvar(
            0 as libc::c_int as libc::c_float,
            1 as libc::c_int as libc::c_float,
            (*in_joystick).value,
        ),
    );
    s_options_joystick_box.curvalue = (*in_joystick).value as libc::c_int;
    s_options_noalttab_box.curvalue = (*win_noalttab).value as libc::c_int;
}

unsafe extern "C" fn ControlsResetDefaultsFunc(mut unused: *mut libc::c_void) {
    Cbuf_AddText(
        b"exec default.cfg\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    Cbuf_Execute();
    ControlsSetMenuItemValues();
}

unsafe extern "C" fn InvertMouseFunc(mut unused: *mut libc::c_void) {
    if s_options_invertmouse_box.curvalue == 0 as libc::c_int {
        Cvar_SetValue(
            b"m_pitch\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            fabs((*m_pitch).value as libc::c_double) as libc::c_float,
        );
    } else {
        Cvar_SetValue(
            b"m_pitch\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            -fabs((*m_pitch).value as libc::c_double) as libc::c_float,
        );
    };
}

unsafe extern "C" fn LookspringFunc(mut unused: *mut libc::c_void) {
    Cvar_SetValue(
        b"lookspring\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        s_options_lookspring_box.curvalue as libc::c_float,
    );
}

unsafe extern "C" fn LookstrafeFunc(mut unused: *mut libc::c_void) {
    Cvar_SetValue(
        b"lookstrafe\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        s_options_lookstrafe_box.curvalue as libc::c_float,
    );
}

unsafe extern "C" fn UpdateVolumeFunc(mut unused: *mut libc::c_void) {
    Cvar_SetValue(
        b"s_volume\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        s_options_sfxvolume_slider.curvalue / 10 as libc::c_int as libc::c_float,
    );
}

unsafe extern "C" fn UpdateCDVolumeFunc(mut unused: *mut libc::c_void) {
    Cvar_SetValue(
        b"cd_nocd\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        (s_options_cdvolume_box.curvalue == 0) as libc::c_int as libc::c_float,
    );
}

unsafe extern "C" fn ConsoleFunc(mut unused: *mut libc::c_void) {
    extern "C" {
        #[link_name = "Key_ClearTyping"]
        fn Key_ClearTyping_0();
    }
    if cl.attractloop as u64 != 0 {
        Cbuf_AddText(
            b"killserver\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        return;
    }
    Key_ClearTyping_0();
    Con_ClearNotify();
    M_ForceMenuOff();
    cls.key_dest = key_console;
}

unsafe extern "C" fn UpdateSoundQualityFunc(mut unused: *mut libc::c_void) {
    if s_options_quality_list.curvalue != 0 {
        Cvar_SetValue(
            b"s_khz\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            22 as libc::c_int as libc::c_float,
        );
        Cvar_SetValue(
            b"s_loadas8bit\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            false_0 as libc::c_int as libc::c_float,
        );
    } else {
        Cvar_SetValue(
            b"s_khz\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            11 as libc::c_int as libc::c_float,
        );
        Cvar_SetValue(
            b"s_loadas8bit\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            true_0 as libc::c_int as libc::c_float,
        );
    }
    Cvar_SetValue(
        b"s_primary\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        s_options_compatibility_list.curvalue as libc::c_float,
    );
    M_DrawTextBox(
        8 as libc::c_int,
        120 as libc::c_int - 48 as libc::c_int,
        36 as libc::c_int,
        3 as libc::c_int,
    );
    M_Print(
        16 as libc::c_int + 16 as libc::c_int,
        120 as libc::c_int - 48 as libc::c_int + 8 as libc::c_int,
        b"Restarting the sound system. This\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    M_Print(
        16 as libc::c_int + 16 as libc::c_int,
        120 as libc::c_int - 48 as libc::c_int + 16 as libc::c_int,
        b"could take up to a minute, so\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    M_Print(
        16 as libc::c_int + 16 as libc::c_int,
        120 as libc::c_int - 48 as libc::c_int + 24 as libc::c_int,
        b"please be patient.\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    (re.EndFrame).expect("non-null function pointer")();
    CL_Snd_Restart_f();
}

#[no_mangle]
pub unsafe extern "C" fn Options_MenuInit() {
    static mut cd_music_items: [*const libc::c_char; 3] = [
        b"disabled\0" as *const u8 as *const libc::c_char,
        b"enabled\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    static mut quality_items: [*const libc::c_char; 3] = [
        b"low\0" as *const u8 as *const libc::c_char,
        b"high\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    static mut compatibility_items: [*const libc::c_char; 3] = [
        b"max compatibility\0" as *const u8 as *const libc::c_char,
        b"max performance\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    static mut yesno_names: [*const libc::c_char; 3] = [
        b"no\0" as *const u8 as *const libc::c_char,
        b"yes\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    static mut crosshair_names: [*const libc::c_char; 5] = [
        b"none\0" as *const u8 as *const libc::c_char,
        b"cross\0" as *const u8 as *const libc::c_char,
        b"dot\0" as *const u8 as *const libc::c_char,
        b"angle\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    win_noalttab = Cvar_Get(
        b"win_noalttab\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        1 as libc::c_int,
    );
    s_options_menu.x = viddef.width / 2 as libc::c_int;
    s_options_menu.y = viddef.height / 2 as libc::c_int - 58 as libc::c_int;
    s_options_menu.nitems = 0 as libc::c_int;
    s_options_sfxvolume_slider.generic.type_0 = 0 as libc::c_int;
    s_options_sfxvolume_slider.generic.x = 0 as libc::c_int;
    s_options_sfxvolume_slider.generic.y = 0 as libc::c_int;
    s_options_sfxvolume_slider
        .generic
        .name = b"effects volume\0" as *const u8 as *const libc::c_char;
    s_options_sfxvolume_slider
        .generic
        .callback = Some(
        UpdateVolumeFunc as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    s_options_sfxvolume_slider.minvalue = 0 as libc::c_int as libc::c_float;
    s_options_sfxvolume_slider.maxvalue = 10 as libc::c_int as libc::c_float;
    s_options_sfxvolume_slider
        .curvalue = Cvar_VariableValue(
        b"s_volume\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) * 10 as libc::c_int as libc::c_float;
    s_options_cdvolume_box.generic.type_0 = 3 as libc::c_int;
    s_options_cdvolume_box.generic.x = 0 as libc::c_int;
    s_options_cdvolume_box.generic.y = 10 as libc::c_int;
    s_options_cdvolume_box
        .generic
        .name = b"CD music\0" as *const u8 as *const libc::c_char;
    s_options_cdvolume_box
        .generic
        .callback = Some(
        UpdateCDVolumeFunc as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    s_options_cdvolume_box.itemnames = cd_music_items.as_mut_ptr();
    s_options_cdvolume_box
        .curvalue = (Cvar_VariableValue(
        b"cd_nocd\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) == 0.) as libc::c_int;
    s_options_quality_list.generic.type_0 = 3 as libc::c_int;
    s_options_quality_list.generic.x = 0 as libc::c_int;
    s_options_quality_list.generic.y = 20 as libc::c_int;
    s_options_quality_list
        .generic
        .name = b"sound quality\0" as *const u8 as *const libc::c_char;
    s_options_quality_list
        .generic
        .callback = Some(
        UpdateSoundQualityFunc as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    s_options_quality_list.itemnames = quality_items.as_mut_ptr();
    s_options_quality_list
        .curvalue = (Cvar_VariableValue(
        b"s_loadas8bit\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) == 0.) as libc::c_int;
    s_options_compatibility_list.generic.type_0 = 3 as libc::c_int;
    s_options_compatibility_list.generic.x = 0 as libc::c_int;
    s_options_compatibility_list.generic.y = 30 as libc::c_int;
    s_options_compatibility_list
        .generic
        .name = b"sound compatibility\0" as *const u8 as *const libc::c_char;
    s_options_compatibility_list
        .generic
        .callback = Some(
        UpdateSoundQualityFunc as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    s_options_compatibility_list.itemnames = compatibility_items.as_mut_ptr();
    s_options_compatibility_list
        .curvalue = Cvar_VariableValue(
        b"s_primary\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as libc::c_int;
    s_options_sensitivity_slider.generic.type_0 = 0 as libc::c_int;
    s_options_sensitivity_slider.generic.x = 0 as libc::c_int;
    s_options_sensitivity_slider.generic.y = 50 as libc::c_int;
    s_options_sensitivity_slider
        .generic
        .name = b"mouse speed\0" as *const u8 as *const libc::c_char;
    s_options_sensitivity_slider
        .generic
        .callback = Some(
        MouseSpeedFunc as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    s_options_sensitivity_slider.minvalue = 2 as libc::c_int as libc::c_float;
    s_options_sensitivity_slider.maxvalue = 22 as libc::c_int as libc::c_float;
    s_options_alwaysrun_box.generic.type_0 = 3 as libc::c_int;
    s_options_alwaysrun_box.generic.x = 0 as libc::c_int;
    s_options_alwaysrun_box.generic.y = 60 as libc::c_int;
    s_options_alwaysrun_box
        .generic
        .name = b"always run\0" as *const u8 as *const libc::c_char;
    s_options_alwaysrun_box
        .generic
        .callback = Some(AlwaysRunFunc as unsafe extern "C" fn(*mut libc::c_void) -> ());
    s_options_alwaysrun_box.itemnames = yesno_names.as_mut_ptr();
    s_options_invertmouse_box.generic.type_0 = 3 as libc::c_int;
    s_options_invertmouse_box.generic.x = 0 as libc::c_int;
    s_options_invertmouse_box.generic.y = 70 as libc::c_int;
    s_options_invertmouse_box
        .generic
        .name = b"invert mouse\0" as *const u8 as *const libc::c_char;
    s_options_invertmouse_box
        .generic
        .callback = Some(
        InvertMouseFunc as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    s_options_invertmouse_box.itemnames = yesno_names.as_mut_ptr();
    s_options_lookspring_box.generic.type_0 = 3 as libc::c_int;
    s_options_lookspring_box.generic.x = 0 as libc::c_int;
    s_options_lookspring_box.generic.y = 80 as libc::c_int;
    s_options_lookspring_box
        .generic
        .name = b"lookspring\0" as *const u8 as *const libc::c_char;
    s_options_lookspring_box
        .generic
        .callback = Some(
        LookspringFunc as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    s_options_lookspring_box.itemnames = yesno_names.as_mut_ptr();
    s_options_lookstrafe_box.generic.type_0 = 3 as libc::c_int;
    s_options_lookstrafe_box.generic.x = 0 as libc::c_int;
    s_options_lookstrafe_box.generic.y = 90 as libc::c_int;
    s_options_lookstrafe_box
        .generic
        .name = b"lookstrafe\0" as *const u8 as *const libc::c_char;
    s_options_lookstrafe_box
        .generic
        .callback = Some(
        LookstrafeFunc as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    s_options_lookstrafe_box.itemnames = yesno_names.as_mut_ptr();
    s_options_freelook_box.generic.type_0 = 3 as libc::c_int;
    s_options_freelook_box.generic.x = 0 as libc::c_int;
    s_options_freelook_box.generic.y = 100 as libc::c_int;
    s_options_freelook_box
        .generic
        .name = b"free look\0" as *const u8 as *const libc::c_char;
    s_options_freelook_box
        .generic
        .callback = Some(FreeLookFunc as unsafe extern "C" fn(*mut libc::c_void) -> ());
    s_options_freelook_box.itemnames = yesno_names.as_mut_ptr();
    s_options_crosshair_box.generic.type_0 = 3 as libc::c_int;
    s_options_crosshair_box.generic.x = 0 as libc::c_int;
    s_options_crosshair_box.generic.y = 110 as libc::c_int;
    s_options_crosshair_box
        .generic
        .name = b"crosshair\0" as *const u8 as *const libc::c_char;
    s_options_crosshair_box
        .generic
        .callback = Some(CrosshairFunc as unsafe extern "C" fn(*mut libc::c_void) -> ());
    s_options_crosshair_box.itemnames = crosshair_names.as_mut_ptr();
    s_options_joystick_box.generic.type_0 = 3 as libc::c_int;
    s_options_joystick_box.generic.x = 0 as libc::c_int;
    s_options_joystick_box.generic.y = 120 as libc::c_int;
    s_options_joystick_box
        .generic
        .name = b"use joystick\0" as *const u8 as *const libc::c_char;
    s_options_joystick_box
        .generic
        .callback = Some(JoystickFunc as unsafe extern "C" fn(*mut libc::c_void) -> ());
    s_options_joystick_box.itemnames = yesno_names.as_mut_ptr();
    s_options_customize_options_action.generic.type_0 = 2 as libc::c_int;
    s_options_customize_options_action.generic.x = 0 as libc::c_int;
    s_options_customize_options_action.generic.y = 140 as libc::c_int;
    s_options_customize_options_action
        .generic
        .name = b"customize controls\0" as *const u8 as *const libc::c_char;
    s_options_customize_options_action
        .generic
        .callback = Some(
        CustomizeControlsFunc as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    s_options_defaults_action.generic.type_0 = 2 as libc::c_int;
    s_options_defaults_action.generic.x = 0 as libc::c_int;
    s_options_defaults_action.generic.y = 150 as libc::c_int;
    s_options_defaults_action
        .generic
        .name = b"reset defaults\0" as *const u8 as *const libc::c_char;
    s_options_defaults_action
        .generic
        .callback = Some(
        ControlsResetDefaultsFunc as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    s_options_console_action.generic.type_0 = 2 as libc::c_int;
    s_options_console_action.generic.x = 0 as libc::c_int;
    s_options_console_action.generic.y = 160 as libc::c_int;
    s_options_console_action
        .generic
        .name = b"go to console\0" as *const u8 as *const libc::c_char;
    s_options_console_action
        .generic
        .callback = Some(ConsoleFunc as unsafe extern "C" fn(*mut libc::c_void) -> ());
    ControlsSetMenuItemValues();
    Menu_AddItem(
        &mut s_options_menu,
        &mut s_options_sfxvolume_slider as *mut menuslider_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_options_menu,
        &mut s_options_cdvolume_box as *mut menulist_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_options_menu,
        &mut s_options_quality_list as *mut menulist_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_options_menu,
        &mut s_options_compatibility_list as *mut menulist_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_options_menu,
        &mut s_options_sensitivity_slider as *mut menuslider_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_options_menu,
        &mut s_options_alwaysrun_box as *mut menulist_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_options_menu,
        &mut s_options_invertmouse_box as *mut menulist_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_options_menu,
        &mut s_options_lookspring_box as *mut menulist_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_options_menu,
        &mut s_options_lookstrafe_box as *mut menulist_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_options_menu,
        &mut s_options_freelook_box as *mut menulist_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_options_menu,
        &mut s_options_crosshair_box as *mut menulist_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_options_menu,
        &mut s_options_joystick_box as *mut menulist_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_options_menu,
        &mut s_options_customize_options_action as *mut menuaction_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_options_menu,
        &mut s_options_defaults_action as *mut menuaction_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_options_menu,
        &mut s_options_console_action as *mut menulist_s as *mut libc::c_void,
    );
}

#[no_mangle]
pub unsafe extern "C" fn Options_MenuDraw() {
    M_Banner(
        b"m_banner_options\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    Menu_AdjustCursor(&mut s_options_menu, 1 as libc::c_int);
    Menu_Draw(&mut s_options_menu);
}

#[no_mangle]
pub unsafe extern "C" fn Options_MenuKey(mut key: libc::c_int) -> *const libc::c_char {
    return Default_MenuKey(&mut s_options_menu, key);
}

#[no_mangle]
pub unsafe extern "C" fn M_Menu_Options_f() {
    Options_MenuInit();
    M_PushMenu(
        Some(Options_MenuDraw as unsafe extern "C" fn() -> ()),
        Some(Options_MenuKey as unsafe extern "C" fn(libc::c_int) -> *const libc::c_char),
    );
}

#[no_mangle]
pub unsafe extern "C" fn M_Menu_Video_f() {
    VID_MenuInit();
    M_PushMenu(
        Some(VID_MenuDraw as unsafe extern "C" fn() -> ()),
        Some(VID_MenuKey as unsafe extern "C" fn(libc::c_int) -> *const libc::c_char),
    );
}

static mut credits_start_time: libc::c_int = 0;
static mut credits: *mut *const libc::c_char = 0 as *const *const libc::c_char
    as *mut *const libc::c_char;
static mut creditsIndex: [*mut libc::c_char; 256] = [0 as *const libc::c_char
    as *mut libc::c_char; 256];
static mut creditsBuffer: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
static mut idcredits: [*const libc::c_char; 87] = [
    b"+QUAKE II BY ID SOFTWARE\0" as *const u8 as *const libc::c_char,
    b"\0" as *const u8 as *const libc::c_char,
    b"+PROGRAMMING\0" as *const u8 as *const libc::c_char,
    b"John Carmack\0" as *const u8 as *const libc::c_char,
    b"John Cash\0" as *const u8 as *const libc::c_char,
    b"Brian Hook\0" as *const u8 as *const libc::c_char,
    b"\0" as *const u8 as *const libc::c_char,
    b"+ART\0" as *const u8 as *const libc::c_char,
    b"Adrian Carmack\0" as *const u8 as *const libc::c_char,
    b"Kevin Cloud\0" as *const u8 as *const libc::c_char,
    b"Paul Steed\0" as *const u8 as *const libc::c_char,
    b"\0" as *const u8 as *const libc::c_char,
    b"+LEVEL DESIGN\0" as *const u8 as *const libc::c_char,
    b"Tim Willits\0" as *const u8 as *const libc::c_char,
    b"American McGee\0" as *const u8 as *const libc::c_char,
    b"Christian Antkow\0" as *const u8 as *const libc::c_char,
    b"Paul Jaquays\0" as *const u8 as *const libc::c_char,
    b"Brandon James\0" as *const u8 as *const libc::c_char,
    b"\0" as *const u8 as *const libc::c_char,
    b"+BIZ\0" as *const u8 as *const libc::c_char,
    b"Todd Hollenshead\0" as *const u8 as *const libc::c_char,
    b"Barrett (Bear) Alexander\0" as *const u8 as *const libc::c_char,
    b"Donna Jackson\0" as *const u8 as *const libc::c_char,
    b"\0" as *const u8 as *const libc::c_char,
    b"\0" as *const u8 as *const libc::c_char,
    b"+SPECIAL THANKS\0" as *const u8 as *const libc::c_char,
    b"Ben Donges for beta testing\0" as *const u8 as *const libc::c_char,
    b"\0" as *const u8 as *const libc::c_char,
    b"\0" as *const u8 as *const libc::c_char,
    b"\0" as *const u8 as *const libc::c_char,
    b"\0" as *const u8 as *const libc::c_char,
    b"\0" as *const u8 as *const libc::c_char,
    b"\0" as *const u8 as *const libc::c_char,
    b"+ADDITIONAL SUPPORT\0" as *const u8 as *const libc::c_char,
    b"\0" as *const u8 as *const libc::c_char,
    b"+LINUX PORT AND CTF\0" as *const u8 as *const libc::c_char,
    b"Dave \"Zoid\" Kirsch\0" as *const u8 as *const libc::c_char,
    b"\0" as *const u8 as *const libc::c_char,
    b"+CINEMATIC SEQUENCES\0" as *const u8 as *const libc::c_char,
    b"Ending Cinematic by Blur Studio - \0" as *const u8 as *const libc::c_char,
    b"Venice, CA\0" as *const u8 as *const libc::c_char,
    b"\0" as *const u8 as *const libc::c_char,
    b"Environment models for Introduction\0" as *const u8 as *const libc::c_char,
    b"Cinematic by Karl Dolgener\0" as *const u8 as *const libc::c_char,
    b"\0" as *const u8 as *const libc::c_char,
    b"Assistance with environment design\0" as *const u8 as *const libc::c_char,
    b"by Cliff Iwai\0" as *const u8 as *const libc::c_char,
    b"\0" as *const u8 as *const libc::c_char,
    b"+SOUND EFFECTS AND MUSIC\0" as *const u8 as *const libc::c_char,
    b"Sound Design by Soundelux Media Labs.\0" as *const u8 as *const libc::c_char,
    b"Music Composed and Produced by\0" as *const u8 as *const libc::c_char,
    b"Soundelux Media Labs.  Special thanks\0" as *const u8 as *const libc::c_char,
    b"to Bill Brown, Tom Ozanich, Brian\0" as *const u8 as *const libc::c_char,
    b"Celano, Jeff Eisner, and The Soundelux\0" as *const u8 as *const libc::c_char,
    b"Players.\0" as *const u8 as *const libc::c_char,
    b"\0" as *const u8 as *const libc::c_char,
    b"\"Level Music\" by Sonic Mayhem\0" as *const u8 as *const libc::c_char,
    b"www.sonicmayhem.com\0" as *const u8 as *const libc::c_char,
    b"\0" as *const u8 as *const libc::c_char,
    b"\"Quake II Theme Song\"\0" as *const u8 as *const libc::c_char,
    b"(C) 1997 Rob Zombie. All Rights\0" as *const u8 as *const libc::c_char,
    b"Reserved.\0" as *const u8 as *const libc::c_char,
    b"\0" as *const u8 as *const libc::c_char,
    b"Track 10 (\"Climb\") by Jer Sypult\0" as *const u8 as *const libc::c_char,
    b"\0" as *const u8 as *const libc::c_char,
    b"Voice of computers by\0" as *const u8 as *const libc::c_char,
    b"Carly Staehlin-Taylor\0" as *const u8 as *const libc::c_char,
    b"\0" as *const u8 as *const libc::c_char,
    b"+THANKS TO ACTIVISION\0" as *const u8 as *const libc::c_char,
    b"+IN PARTICULAR:\0" as *const u8 as *const libc::c_char,
    b"\0" as *const u8 as *const libc::c_char,
    b"John Tam\0" as *const u8 as *const libc::c_char,
    b"Steve Rosenthal\0" as *const u8 as *const libc::c_char,
    b"Marty Stratton\0" as *const u8 as *const libc::c_char,
    b"Henk Hartong\0" as *const u8 as *const libc::c_char,
    b"\0" as *const u8 as *const libc::c_char,
    b"Quake II(tm) (C)1997 Id Software, Inc.\0" as *const u8 as *const libc::c_char,
    b"All Rights Reserved.  Distributed by\0" as *const u8 as *const libc::c_char,
    b"Activision, Inc. under license.\0" as *const u8 as *const libc::c_char,
    b"Quake II(tm), the Id Software name,\0" as *const u8 as *const libc::c_char,
    b"the \"Q II\"(tm) logo and id(tm)\0" as *const u8 as *const libc::c_char,
    b"logo are trademarks of Id Software,\0" as *const u8 as *const libc::c_char,
    b"Inc. Activision(R) is a registered\0" as *const u8 as *const libc::c_char,
    b"trademark of Activision, Inc. All\0" as *const u8 as *const libc::c_char,
    b"other trademarks and trade names are\0" as *const u8 as *const libc::c_char,
    b"properties of their respective owners.\0" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
static mut xatcredits: [*const libc::c_char; 137] = [
    b"+QUAKE II MISSION PACK: THE RECKONING\0" as *const u8 as *const libc::c_char,
    b"+BY\0" as *const u8 as *const libc::c_char,
    b"+XATRIX ENTERTAINMENT, INC.\0" as *const u8 as *const libc::c_char,
    b"\0" as *const u8 as *const libc::c_char,
    b"+DESIGN AND DIRECTION\0" as *const u8 as *const libc::c_char,
    b"Drew Markham\0" as *const u8 as *const libc::c_char,
    b"\0" as *const u8 as *const libc::c_char,
    b"+PRODUCED BY\0" as *const u8 as *const libc::c_char,
    b"Greg Goodrich\0" as *const u8 as *const libc::c_char,
    b"\0" as *const u8 as *const libc::c_char,
    b"+PROGRAMMING\0" as *const u8 as *const libc::c_char,
    b"Rafael Paiz\0" as *const u8 as *const libc::c_char,
    b"\0" as *const u8 as *const libc::c_char,
    b"+LEVEL DESIGN / ADDITIONAL GAME DESIGN\0" as *const u8 as *const libc::c_char,
    b"Alex Mayberry\0" as *const u8 as *const libc::c_char,
    b"\0" as *const u8 as *const libc::c_char,
    b"+LEVEL DESIGN\0" as *const u8 as *const libc::c_char,
    b"Mal Blackwell\0" as *const u8 as *const libc::c_char,
    b"Dan Koppel\0" as *const u8 as *const libc::c_char,
    b"\0" as *const u8 as *const libc::c_char,
    b"+ART DIRECTION\0" as *const u8 as *const libc::c_char,
    b"Michael \"Maxx\" Kaufman\0" as *const u8 as *const libc::c_char,
    b"\0" as *const u8 as *const libc::c_char,
    b"+COMPUTER GRAPHICS SUPERVISOR AND\0" as *const u8 as *const libc::c_char,
    b"+CHARACTER ANIMATION DIRECTION\0" as *const u8 as *const libc::c_char,
    b"Barry Dempsey\0" as *const u8 as *const libc::c_char,
    b"\0" as *const u8 as *const libc::c_char,
    b"+SENIOR ANIMATOR AND MODELER\0" as *const u8 as *const libc::c_char,
    b"Jason Hoover\0" as *const u8 as *const libc::c_char,
    b"\0" as *const u8 as *const libc::c_char,
    b"+CHARACTER ANIMATION AND\0" as *const u8 as *const libc::c_char,
    b"+MOTION CAPTURE SPECIALIST\0" as *const u8 as *const libc::c_char,
    b"Amit Doron\0" as *const u8 as *const libc::c_char,
    b"\0" as *const u8 as *const libc::c_char,
    b"+ART\0" as *const u8 as *const libc::c_char,
    b"Claire Praderie-Markham\0" as *const u8 as *const libc::c_char,
    b"Viktor Antonov\0" as *const u8 as *const libc::c_char,
    b"Corky Lehmkuhl\0" as *const u8 as *const libc::c_char,
    b"\0" as *const u8 as *const libc::c_char,
    b"+INTRODUCTION ANIMATION\0" as *const u8 as *const libc::c_char,
    b"Dominique Drozdz\0" as *const u8 as *const libc::c_char,
    b"\0" as *const u8 as *const libc::c_char,
    b"+ADDITIONAL LEVEL DESIGN\0" as *const u8 as *const libc::c_char,
    b"Aaron Barber\0" as *const u8 as *const libc::c_char,
    b"Rhett Baldwin\0" as *const u8 as *const libc::c_char,
    b"\0" as *const u8 as *const libc::c_char,
    b"+3D CHARACTER ANIMATION TOOLS\0" as *const u8 as *const libc::c_char,
    b"Gerry Tyra, SA Technology\0" as *const u8 as *const libc::c_char,
    b"\0" as *const u8 as *const libc::c_char,
    b"+ADDITIONAL EDITOR TOOL PROGRAMMING\0" as *const u8 as *const libc::c_char,
    b"Robert Duffy\0" as *const u8 as *const libc::c_char,
    b"\0" as *const u8 as *const libc::c_char,
    b"+ADDITIONAL PROGRAMMING\0" as *const u8 as *const libc::c_char,
    b"Ryan Feltrin\0" as *const u8 as *const libc::c_char,
    b"\0" as *const u8 as *const libc::c_char,
    b"+PRODUCTION COORDINATOR\0" as *const u8 as *const libc::c_char,
    b"Victoria Sylvester\0" as *const u8 as *const libc::c_char,
    b"\0" as *const u8 as *const libc::c_char,
    b"+SOUND DESIGN\0" as *const u8 as *const libc::c_char,
    b"Gary Bradfield\0" as *const u8 as *const libc::c_char,
    b"\0" as *const u8 as *const libc::c_char,
    b"+MUSIC BY\0" as *const u8 as *const libc::c_char,
    b"Sonic Mayhem\0" as *const u8 as *const libc::c_char,
    b"\0" as *const u8 as *const libc::c_char,
    b"\0" as *const u8 as *const libc::c_char,
    b"\0" as *const u8 as *const libc::c_char,
    b"+SPECIAL THANKS\0" as *const u8 as *const libc::c_char,
    b"+TO\0" as *const u8 as *const libc::c_char,
    b"+OUR FRIENDS AT ID SOFTWARE\0" as *const u8 as *const libc::c_char,
    b"\0" as *const u8 as *const libc::c_char,
    b"John Carmack\0" as *const u8 as *const libc::c_char,
    b"John Cash\0" as *const u8 as *const libc::c_char,
    b"Brian Hook\0" as *const u8 as *const libc::c_char,
    b"Adrian Carmack\0" as *const u8 as *const libc::c_char,
    b"Kevin Cloud\0" as *const u8 as *const libc::c_char,
    b"Paul Steed\0" as *const u8 as *const libc::c_char,
    b"Tim Willits\0" as *const u8 as *const libc::c_char,
    b"Christian Antkow\0" as *const u8 as *const libc::c_char,
    b"Paul Jaquays\0" as *const u8 as *const libc::c_char,
    b"Brandon James\0" as *const u8 as *const libc::c_char,
    b"Todd Hollenshead\0" as *const u8 as *const libc::c_char,
    b"Barrett (Bear) Alexander\0" as *const u8 as *const libc::c_char,
    b"Dave \"Zoid\" Kirsch\0" as *const u8 as *const libc::c_char,
    b"Donna Jackson\0" as *const u8 as *const libc::c_char,
    b"\0" as *const u8 as *const libc::c_char,
    b"\0" as *const u8 as *const libc::c_char,
    b"\0" as *const u8 as *const libc::c_char,
    b"+THANKS TO ACTIVISION\0" as *const u8 as *const libc::c_char,
    b"+IN PARTICULAR:\0" as *const u8 as *const libc::c_char,
    b"\0" as *const u8 as *const libc::c_char,
    b"Marty Stratton\0" as *const u8 as *const libc::c_char,
    b"Henk \"The Original Ripper\" Hartong\0" as *const u8 as *const libc::c_char,
    b"Kevin Kraff\0" as *const u8 as *const libc::c_char,
    b"Jamey Gottlieb\0" as *const u8 as *const libc::c_char,
    b"Chris Hepburn\0" as *const u8 as *const libc::c_char,
    b"\0" as *const u8 as *const libc::c_char,
    b"+AND THE GAME TESTERS\0" as *const u8 as *const libc::c_char,
    b"\0" as *const u8 as *const libc::c_char,
    b"Tim Vanlaw\0" as *const u8 as *const libc::c_char,
    b"Doug Jacobs\0" as *const u8 as *const libc::c_char,
    b"Steven Rosenthal\0" as *const u8 as *const libc::c_char,
    b"David Baker\0" as *const u8 as *const libc::c_char,
    b"Chris Campbell\0" as *const u8 as *const libc::c_char,
    b"Aaron Casillas\0" as *const u8 as *const libc::c_char,
    b"Steve Elwell\0" as *const u8 as *const libc::c_char,
    b"Derek Johnstone\0" as *const u8 as *const libc::c_char,
    b"Igor Krinitskiy\0" as *const u8 as *const libc::c_char,
    b"Samantha Lee\0" as *const u8 as *const libc::c_char,
    b"Michael Spann\0" as *const u8 as *const libc::c_char,
    b"Chris Toft\0" as *const u8 as *const libc::c_char,
    b"Juan Valdes\0" as *const u8 as *const libc::c_char,
    b"\0" as *const u8 as *const libc::c_char,
    b"+THANKS TO INTERGRAPH COMPUTER SYTEMS\0" as *const u8 as *const libc::c_char,
    b"+IN PARTICULAR:\0" as *const u8 as *const libc::c_char,
    b"\0" as *const u8 as *const libc::c_char,
    b"Michael T. Nicolaou\0" as *const u8 as *const libc::c_char,
    b"\0" as *const u8 as *const libc::c_char,
    b"\0" as *const u8 as *const libc::c_char,
    b"Quake II Mission Pack: The Reckoning\0" as *const u8 as *const libc::c_char,
    b"(tm) (C)1998 Id Software, Inc. All\0" as *const u8 as *const libc::c_char,
    b"Rights Reserved. Developed by Xatrix\0" as *const u8 as *const libc::c_char,
    b"Entertainment, Inc. for Id Software,\0" as *const u8 as *const libc::c_char,
    b"Inc. Distributed by Activision Inc.\0" as *const u8 as *const libc::c_char,
    b"under license. Quake(R) is a\0" as *const u8 as *const libc::c_char,
    b"registered trademark of Id Software,\0" as *const u8 as *const libc::c_char,
    b"Inc. Quake II Mission Pack: The\0" as *const u8 as *const libc::c_char,
    b"Reckoning(tm), Quake II(tm), the Id\0" as *const u8 as *const libc::c_char,
    b"Software name, the \"Q II\"(tm) logo\0" as *const u8 as *const libc::c_char,
    b"and id(tm) logo are trademarks of Id\0" as *const u8 as *const libc::c_char,
    b"Software, Inc. Activision(R) is a\0" as *const u8 as *const libc::c_char,
    b"registered trademark of Activision,\0" as *const u8 as *const libc::c_char,
    b"Inc. Xatrix(R) is a registered\0" as *const u8 as *const libc::c_char,
    b"trademark of Xatrix Entertainment,\0" as *const u8 as *const libc::c_char,
    b"Inc. All other trademarks and trade\0" as *const u8 as *const libc::c_char,
    b"names are properties of their\0" as *const u8 as *const libc::c_char,
    b"respective owners.\0" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
static mut roguecredits: [*const libc::c_char; 110] = [
    b"+QUAKE II MISSION PACK 2: GROUND ZERO\0" as *const u8 as *const libc::c_char,
    b"+BY\0" as *const u8 as *const libc::c_char,
    b"+ROGUE ENTERTAINMENT, INC.\0" as *const u8 as *const libc::c_char,
    b"\0" as *const u8 as *const libc::c_char,
    b"+PRODUCED BY\0" as *const u8 as *const libc::c_char,
    b"Jim Molinets\0" as *const u8 as *const libc::c_char,
    b"\0" as *const u8 as *const libc::c_char,
    b"+PROGRAMMING\0" as *const u8 as *const libc::c_char,
    b"Peter Mack\0" as *const u8 as *const libc::c_char,
    b"Patrick Magruder\0" as *const u8 as *const libc::c_char,
    b"\0" as *const u8 as *const libc::c_char,
    b"+LEVEL DESIGN\0" as *const u8 as *const libc::c_char,
    b"Jim Molinets\0" as *const u8 as *const libc::c_char,
    b"Cameron Lamprecht\0" as *const u8 as *const libc::c_char,
    b"Berenger Fish\0" as *const u8 as *const libc::c_char,
    b"Robert Selitto\0" as *const u8 as *const libc::c_char,
    b"Steve Tietze\0" as *const u8 as *const libc::c_char,
    b"Steve Thoms\0" as *const u8 as *const libc::c_char,
    b"\0" as *const u8 as *const libc::c_char,
    b"+ART DIRECTION\0" as *const u8 as *const libc::c_char,
    b"Rich Fleider\0" as *const u8 as *const libc::c_char,
    b"\0" as *const u8 as *const libc::c_char,
    b"+ART\0" as *const u8 as *const libc::c_char,
    b"Rich Fleider\0" as *const u8 as *const libc::c_char,
    b"Steve Maines\0" as *const u8 as *const libc::c_char,
    b"Won Choi\0" as *const u8 as *const libc::c_char,
    b"\0" as *const u8 as *const libc::c_char,
    b"+ANIMATION SEQUENCES\0" as *const u8 as *const libc::c_char,
    b"Creat Studios\0" as *const u8 as *const libc::c_char,
    b"Steve Maines\0" as *const u8 as *const libc::c_char,
    b"\0" as *const u8 as *const libc::c_char,
    b"+ADDITIONAL LEVEL DESIGN\0" as *const u8 as *const libc::c_char,
    b"Rich Fleider\0" as *const u8 as *const libc::c_char,
    b"Steve Maines\0" as *const u8 as *const libc::c_char,
    b"Peter Mack\0" as *const u8 as *const libc::c_char,
    b"\0" as *const u8 as *const libc::c_char,
    b"+SOUND\0" as *const u8 as *const libc::c_char,
    b"James Grunke\0" as *const u8 as *const libc::c_char,
    b"\0" as *const u8 as *const libc::c_char,
    b"+GROUND ZERO THEME\0" as *const u8 as *const libc::c_char,
    b"+AND\0" as *const u8 as *const libc::c_char,
    b"+MUSIC BY\0" as *const u8 as *const libc::c_char,
    b"Sonic Mayhem\0" as *const u8 as *const libc::c_char,
    b"\0" as *const u8 as *const libc::c_char,
    b"+VWEP MODELS\0" as *const u8 as *const libc::c_char,
    b"Brent \"Hentai\" Dill\0" as *const u8 as *const libc::c_char,
    b"\0" as *const u8 as *const libc::c_char,
    b"\0" as *const u8 as *const libc::c_char,
    b"\0" as *const u8 as *const libc::c_char,
    b"+SPECIAL THANKS\0" as *const u8 as *const libc::c_char,
    b"+TO\0" as *const u8 as *const libc::c_char,
    b"+OUR FRIENDS AT ID SOFTWARE\0" as *const u8 as *const libc::c_char,
    b"\0" as *const u8 as *const libc::c_char,
    b"John Carmack\0" as *const u8 as *const libc::c_char,
    b"John Cash\0" as *const u8 as *const libc::c_char,
    b"Brian Hook\0" as *const u8 as *const libc::c_char,
    b"Adrian Carmack\0" as *const u8 as *const libc::c_char,
    b"Kevin Cloud\0" as *const u8 as *const libc::c_char,
    b"Paul Steed\0" as *const u8 as *const libc::c_char,
    b"Tim Willits\0" as *const u8 as *const libc::c_char,
    b"Christian Antkow\0" as *const u8 as *const libc::c_char,
    b"Paul Jaquays\0" as *const u8 as *const libc::c_char,
    b"Brandon James\0" as *const u8 as *const libc::c_char,
    b"Todd Hollenshead\0" as *const u8 as *const libc::c_char,
    b"Barrett (Bear) Alexander\0" as *const u8 as *const libc::c_char,
    b"Katherine Anna Kang\0" as *const u8 as *const libc::c_char,
    b"Donna Jackson\0" as *const u8 as *const libc::c_char,
    b"Dave \"Zoid\" Kirsch\0" as *const u8 as *const libc::c_char,
    b"\0" as *const u8 as *const libc::c_char,
    b"\0" as *const u8 as *const libc::c_char,
    b"\0" as *const u8 as *const libc::c_char,
    b"+THANKS TO ACTIVISION\0" as *const u8 as *const libc::c_char,
    b"+IN PARTICULAR:\0" as *const u8 as *const libc::c_char,
    b"\0" as *const u8 as *const libc::c_char,
    b"Marty Stratton\0" as *const u8 as *const libc::c_char,
    b"Henk Hartong\0" as *const u8 as *const libc::c_char,
    b"Mitch Lasky\0" as *const u8 as *const libc::c_char,
    b"Steve Rosenthal\0" as *const u8 as *const libc::c_char,
    b"Steve Elwell\0" as *const u8 as *const libc::c_char,
    b"\0" as *const u8 as *const libc::c_char,
    b"+AND THE GAME TESTERS\0" as *const u8 as *const libc::c_char,
    b"\0" as *const u8 as *const libc::c_char,
    b"The Ranger Clan\0" as *const u8 as *const libc::c_char,
    b"Dave \"Zoid\" Kirsch\0" as *const u8 as *const libc::c_char,
    b"Nihilistic Software\0" as *const u8 as *const libc::c_char,
    b"Robert Duffy\0" as *const u8 as *const libc::c_char,
    b"\0" as *const u8 as *const libc::c_char,
    b"And Countless Others\0" as *const u8 as *const libc::c_char,
    b"\0" as *const u8 as *const libc::c_char,
    b"\0" as *const u8 as *const libc::c_char,
    b"\0" as *const u8 as *const libc::c_char,
    b"Quake II Mission Pack 2: Ground Zero\0" as *const u8 as *const libc::c_char,
    b"(tm) (C)1998 Id Software, Inc. All\0" as *const u8 as *const libc::c_char,
    b"Rights Reserved. Developed by Rogue\0" as *const u8 as *const libc::c_char,
    b"Entertainment, Inc. for Id Software,\0" as *const u8 as *const libc::c_char,
    b"Inc. Distributed by Activision Inc.\0" as *const u8 as *const libc::c_char,
    b"under license. Quake(R) is a\0" as *const u8 as *const libc::c_char,
    b"registered trademark of Id Software,\0" as *const u8 as *const libc::c_char,
    b"Inc. Quake II Mission Pack 2: Ground\0" as *const u8 as *const libc::c_char,
    b"Zero(tm), Quake II(tm), the Id\0" as *const u8 as *const libc::c_char,
    b"Software name, the \"Q II\"(tm) logo\0" as *const u8 as *const libc::c_char,
    b"and id(tm) logo are trademarks of Id\0" as *const u8 as *const libc::c_char,
    b"Software, Inc. Activision(R) is a\0" as *const u8 as *const libc::c_char,
    b"registered trademark of Activision,\0" as *const u8 as *const libc::c_char,
    b"Inc. Rogue(R) is a registered\0" as *const u8 as *const libc::c_char,
    b"trademark of Rogue Entertainment,\0" as *const u8 as *const libc::c_char,
    b"Inc. All other trademarks and trade\0" as *const u8 as *const libc::c_char,
    b"names are properties of their\0" as *const u8 as *const libc::c_char,
    b"respective owners.\0" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];

#[no_mangle]
pub unsafe extern "C" fn M_Credits_MenuDraw() {
    let mut i: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    i = 0 as libc::c_int;
    y = (viddef.height as libc::c_float
        - (cls.realtime - credits_start_time) as libc::c_float / 40.0f32) as libc::c_int;
    while !(*credits.offset(i as isize)).is_null() && y < viddef.height {
        let mut j: libc::c_int = 0;
        let mut stringoffset: libc::c_int = 0 as libc::c_int;
        let mut bold: libc::c_int = false_0 as libc::c_int;
        if !(y <= -(8 as libc::c_int)) {
            if *(*credits.offset(i as isize)).offset(0 as libc::c_int as isize)
                as libc::c_int == '+' as i32
            {
                bold = true_0 as libc::c_int;
                stringoffset = 1 as libc::c_int;
            } else {
                bold = false_0 as libc::c_int;
                stringoffset = 0 as libc::c_int;
            }
            j = 0 as libc::c_int;
            while *(*credits.offset(i as isize)).offset((j + stringoffset) as isize) != 0
            {
                let mut x: libc::c_int = 0;
                x = (viddef.width as libc::c_ulong)
                    .wrapping_sub(
                        (strlen(*credits.offset(i as isize)))
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong),
                    )
                    .wrapping_sub((stringoffset * 8 as libc::c_int) as libc::c_ulong)
                    .wrapping_div(2 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        ((j + stringoffset) * 8 as libc::c_int) as libc::c_ulong,
                    ) as libc::c_int;
                if bold != 0 {
                    (re.DrawChar)
                        .expect(
                            "non-null function pointer",
                        )(
                        x,
                        y,
                        *(*credits.offset(i as isize))
                            .offset((j + stringoffset) as isize) as libc::c_int
                            + 128 as libc::c_int,
                    );
                } else {
                    (re.DrawChar)
                        .expect(
                            "non-null function pointer",
                        )(
                        x,
                        y,
                        *(*credits.offset(i as isize))
                            .offset((j + stringoffset) as isize) as libc::c_int,
                    );
                }
                j += 1;
            }
        }
        y += 10 as libc::c_int;
        i += 1;
    }
    if y < 0 as libc::c_int {
        credits_start_time = cls.realtime;
    }
}

#[no_mangle]
pub unsafe extern "C" fn M_Credits_Key(mut key: libc::c_int) -> *const libc::c_char {
    match key {
        27 => {
            if !creditsBuffer.is_null() {
                FS_FreeFile(creditsBuffer as *mut libc::c_void);
            }
            M_PopMenu();
        }
        _ => {}
    }
    return menu_out_sound;
}

#[no_mangle]
pub unsafe extern "C" fn M_Menu_Credits_f() {
    let mut n: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut isdeveloper: libc::c_int = 0 as libc::c_int;
    creditsBuffer = 0 as *mut libc::c_char;
    count = FS_LoadFile(
        b"credits\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        &mut creditsBuffer as *mut *mut libc::c_char as *mut *mut libc::c_void,
    );
    if count != -(1 as libc::c_int) {
        p = creditsBuffer;
        n = 0 as libc::c_int;
        while n < 255 as libc::c_int {
            creditsIndex[n as usize] = p;
            while *p as libc::c_int != '\r' as i32 && *p as libc::c_int != '\n' as i32 {
                p = p.offset(1);
                count -= 1;
                if count == 0 as libc::c_int {
                    break;
                }
            }
            if *p as libc::c_int == '\r' as i32 {
                let fresh4 = p;
                p = p.offset(1);
                *fresh4 = 0 as libc::c_int as libc::c_char;
                count -= 1;
                if count == 0 as libc::c_int {
                    break;
                }
            }
            let fresh5 = p;
            p = p.offset(1);
            *fresh5 = 0 as libc::c_int as libc::c_char;
            count -= 1;
            if count == 0 as libc::c_int {
                break;
            }
            n += 1;
        }
        n += 1;
        creditsIndex[n as usize] = 0 as *mut libc::c_char;
        credits = creditsIndex.as_mut_ptr() as *mut *const libc::c_char;
    } else {
        isdeveloper = Developer_searchpath(1 as libc::c_int);
        if isdeveloper == 1 as libc::c_int {
            credits = xatcredits.as_mut_ptr();
        } else if isdeveloper == 2 as libc::c_int {
            credits = roguecredits.as_mut_ptr();
        } else {
            credits = idcredits.as_mut_ptr();
        }
    }
    credits_start_time = cls.realtime;
    M_PushMenu(
        Some(M_Credits_MenuDraw as unsafe extern "C" fn() -> ()),
        Some(M_Credits_Key as unsafe extern "C" fn(libc::c_int) -> *const libc::c_char),
    );
}

static mut m_game_cursor: libc::c_int = 0;
static mut s_game_menu: menuframework_s = menuframework_s {
    x: 0,
    y: 0,
    cursor: 0,
    nitems: 0,
    nslots: 0,
    items: [0 as *const libc::c_void as *mut libc::c_void; 64],
    statusbar: 0 as *const libc::c_char,
    cursordraw: None,
};
static mut s_easy_game_action: menuaction_s = menuaction_s {
    generic: menucommon_s {
        type_0: 0,
        name: 0 as *const libc::c_char,
        x: 0,
        y: 0,
        parent: 0 as *const menuframework_s as *mut menuframework_s,
        cursor_offset: 0,
        localdata: [0; 4],
        flags: 0,
        statusbar: 0 as *const libc::c_char,
        callback: None,
        statusbarfunc: None,
        ownerdraw: None,
        cursordraw: None,
    },
};
static mut s_medium_game_action: menuaction_s = menuaction_s {
    generic: menucommon_s {
        type_0: 0,
        name: 0 as *const libc::c_char,
        x: 0,
        y: 0,
        parent: 0 as *const menuframework_s as *mut menuframework_s,
        cursor_offset: 0,
        localdata: [0; 4],
        flags: 0,
        statusbar: 0 as *const libc::c_char,
        callback: None,
        statusbarfunc: None,
        ownerdraw: None,
        cursordraw: None,
    },
};
static mut s_hard_game_action: menuaction_s = menuaction_s {
    generic: menucommon_s {
        type_0: 0,
        name: 0 as *const libc::c_char,
        x: 0,
        y: 0,
        parent: 0 as *const menuframework_s as *mut menuframework_s,
        cursor_offset: 0,
        localdata: [0; 4],
        flags: 0,
        statusbar: 0 as *const libc::c_char,
        callback: None,
        statusbarfunc: None,
        ownerdraw: None,
        cursordraw: None,
    },
};
static mut s_load_game_action: menuaction_s = menuaction_s {
    generic: menucommon_s {
        type_0: 0,
        name: 0 as *const libc::c_char,
        x: 0,
        y: 0,
        parent: 0 as *const menuframework_s as *mut menuframework_s,
        cursor_offset: 0,
        localdata: [0; 4],
        flags: 0,
        statusbar: 0 as *const libc::c_char,
        callback: None,
        statusbarfunc: None,
        ownerdraw: None,
        cursordraw: None,
    },
};
static mut s_save_game_action: menuaction_s = menuaction_s {
    generic: menucommon_s {
        type_0: 0,
        name: 0 as *const libc::c_char,
        x: 0,
        y: 0,
        parent: 0 as *const menuframework_s as *mut menuframework_s,
        cursor_offset: 0,
        localdata: [0; 4],
        flags: 0,
        statusbar: 0 as *const libc::c_char,
        callback: None,
        statusbarfunc: None,
        ownerdraw: None,
        cursordraw: None,
    },
};
static mut s_credits_action: menuaction_s = menuaction_s {
    generic: menucommon_s {
        type_0: 0,
        name: 0 as *const libc::c_char,
        x: 0,
        y: 0,
        parent: 0 as *const menuframework_s as *mut menuframework_s,
        cursor_offset: 0,
        localdata: [0; 4],
        flags: 0,
        statusbar: 0 as *const libc::c_char,
        callback: None,
        statusbarfunc: None,
        ownerdraw: None,
        cursordraw: None,
    },
};
static mut s_blankline: menuseparator_s = menuseparator_s {
    generic: menucommon_s {
        type_0: 0,
        name: 0 as *const libc::c_char,
        x: 0,
        y: 0,
        parent: 0 as *const menuframework_s as *mut menuframework_s,
        cursor_offset: 0,
        localdata: [0; 4],
        flags: 0,
        statusbar: 0 as *const libc::c_char,
        callback: None,
        statusbarfunc: None,
        ownerdraw: None,
        cursordraw: None,
    },
};

unsafe extern "C" fn StartGame() {
    cl.servercount = -(1 as libc::c_int);
    M_ForceMenuOff();
    Cvar_SetValue(
        b"deathmatch\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int as libc::c_float,
    );
    Cvar_SetValue(
        b"coop\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int as libc::c_float,
    );
    Cvar_SetValue(
        b"gamerules\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int as libc::c_float,
    );
    Cbuf_AddText(
        b"loading ; killserver ; wait ; newgame\n\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    cls.key_dest = key_game;
}

unsafe extern "C" fn EasyGameFunc(mut data: *mut libc::c_void) {
    Cvar_ForceSet(
        b"skill\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    StartGame();
}

unsafe extern "C" fn MediumGameFunc(mut data: *mut libc::c_void) {
    Cvar_ForceSet(
        b"skill\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    StartGame();
}

unsafe extern "C" fn HardGameFunc(mut data: *mut libc::c_void) {
    Cvar_ForceSet(
        b"skill\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    StartGame();
}

unsafe extern "C" fn LoadGameFunc(mut unused: *mut libc::c_void) {
    M_Menu_LoadGame_f();
}

unsafe extern "C" fn SaveGameFunc(mut unused: *mut libc::c_void) {
    M_Menu_SaveGame_f();
}

unsafe extern "C" fn CreditsFunc(mut unused: *mut libc::c_void) {
    M_Menu_Credits_f();
}

#[no_mangle]
pub unsafe extern "C" fn Game_MenuInit() {
    static mut difficulty_names: [*const libc::c_char; 4] = [
        b"easy\0" as *const u8 as *const libc::c_char,
        b"medium\0" as *const u8 as *const libc::c_char,
        b"hard\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    s_game_menu.x = (viddef.width as libc::c_double * 0.50f64) as libc::c_int;
    s_game_menu.nitems = 0 as libc::c_int;
    s_easy_game_action.generic.type_0 = 2 as libc::c_int;
    s_easy_game_action.generic.flags = 0x1 as libc::c_int as libc::c_uint;
    s_easy_game_action.generic.x = 0 as libc::c_int;
    s_easy_game_action.generic.y = 0 as libc::c_int;
    s_easy_game_action.generic.name = b"easy\0" as *const u8 as *const libc::c_char;
    s_easy_game_action
        .generic
        .callback = Some(EasyGameFunc as unsafe extern "C" fn(*mut libc::c_void) -> ());
    s_medium_game_action.generic.type_0 = 2 as libc::c_int;
    s_medium_game_action.generic.flags = 0x1 as libc::c_int as libc::c_uint;
    s_medium_game_action.generic.x = 0 as libc::c_int;
    s_medium_game_action.generic.y = 10 as libc::c_int;
    s_medium_game_action.generic.name = b"medium\0" as *const u8 as *const libc::c_char;
    s_medium_game_action
        .generic
        .callback = Some(
        MediumGameFunc as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    s_hard_game_action.generic.type_0 = 2 as libc::c_int;
    s_hard_game_action.generic.flags = 0x1 as libc::c_int as libc::c_uint;
    s_hard_game_action.generic.x = 0 as libc::c_int;
    s_hard_game_action.generic.y = 20 as libc::c_int;
    s_hard_game_action.generic.name = b"hard\0" as *const u8 as *const libc::c_char;
    s_hard_game_action
        .generic
        .callback = Some(HardGameFunc as unsafe extern "C" fn(*mut libc::c_void) -> ());
    s_blankline.generic.type_0 = 4 as libc::c_int;
    s_load_game_action.generic.type_0 = 2 as libc::c_int;
    s_load_game_action.generic.flags = 0x1 as libc::c_int as libc::c_uint;
    s_load_game_action.generic.x = 0 as libc::c_int;
    s_load_game_action.generic.y = 40 as libc::c_int;
    s_load_game_action.generic.name = b"load game\0" as *const u8 as *const libc::c_char;
    s_load_game_action
        .generic
        .callback = Some(LoadGameFunc as unsafe extern "C" fn(*mut libc::c_void) -> ());
    s_save_game_action.generic.type_0 = 2 as libc::c_int;
    s_save_game_action.generic.flags = 0x1 as libc::c_int as libc::c_uint;
    s_save_game_action.generic.x = 0 as libc::c_int;
    s_save_game_action.generic.y = 50 as libc::c_int;
    s_save_game_action.generic.name = b"save game\0" as *const u8 as *const libc::c_char;
    s_save_game_action
        .generic
        .callback = Some(SaveGameFunc as unsafe extern "C" fn(*mut libc::c_void) -> ());
    s_credits_action.generic.type_0 = 2 as libc::c_int;
    s_credits_action.generic.flags = 0x1 as libc::c_int as libc::c_uint;
    s_credits_action.generic.x = 0 as libc::c_int;
    s_credits_action.generic.y = 60 as libc::c_int;
    s_credits_action.generic.name = b"credits\0" as *const u8 as *const libc::c_char;
    s_credits_action
        .generic
        .callback = Some(CreditsFunc as unsafe extern "C" fn(*mut libc::c_void) -> ());
    Menu_AddItem(
        &mut s_game_menu,
        &mut s_easy_game_action as *mut menuaction_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_game_menu,
        &mut s_medium_game_action as *mut menuaction_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_game_menu,
        &mut s_hard_game_action as *mut menuaction_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_game_menu,
        &mut s_blankline as *mut menuseparator_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_game_menu,
        &mut s_load_game_action as *mut menuaction_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_game_menu,
        &mut s_save_game_action as *mut menuaction_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_game_menu,
        &mut s_blankline as *mut menuseparator_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_game_menu,
        &mut s_credits_action as *mut menuaction_s as *mut libc::c_void,
    );
    Menu_Center(&mut s_game_menu);
}

#[no_mangle]
pub unsafe extern "C" fn Game_MenuDraw() {
    M_Banner(
        b"m_banner_game\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    Menu_AdjustCursor(&mut s_game_menu, 1 as libc::c_int);
    Menu_Draw(&mut s_game_menu);
}

#[no_mangle]
pub unsafe extern "C" fn Game_MenuKey(mut key: libc::c_int) -> *const libc::c_char {
    return Default_MenuKey(&mut s_game_menu, key);
}

#[no_mangle]
pub unsafe extern "C" fn M_Menu_Game_f() {
    Game_MenuInit();
    M_PushMenu(
        Some(Game_MenuDraw as unsafe extern "C" fn() -> ()),
        Some(Game_MenuKey as unsafe extern "C" fn(libc::c_int) -> *const libc::c_char),
    );
    m_game_cursor = 1 as libc::c_int;
}

static mut s_loadgame_menu: menuframework_s = menuframework_s {
    x: 0,
    y: 0,
    cursor: 0,
    nitems: 0,
    nslots: 0,
    items: [0 as *const libc::c_void as *mut libc::c_void; 64],
    statusbar: 0 as *const libc::c_char,
    cursordraw: None,
};
static mut s_loadgame_actions: [menuaction_s; 15] = [menuaction_s {
    generic: menucommon_s {
        type_0: 0,
        name: 0 as *const libc::c_char,
        x: 0,
        y: 0,
        parent: 0 as *const menuframework_s as *mut menuframework_s,
        cursor_offset: 0,
        localdata: [0; 4],
        flags: 0,
        statusbar: 0 as *const libc::c_char,
        callback: None,
        statusbarfunc: None,
        ownerdraw: None,
        cursordraw: None,
    },
}; 15];
#[no_mangle]
pub static mut m_savestrings: [[libc::c_char; 32]; 15] = [[0; 32]; 15];
#[no_mangle]
pub static mut m_savevalid: [qboolean; 15] = [false_0; 15];

#[no_mangle]
pub unsafe extern "C" fn Create_Savestrings() {
    let mut i: libc::c_int = 0;
    let mut f: *mut FILE = 0 as *mut FILE;
    let mut name: [libc::c_char; 128] = [0; 128];
    i = 0 as libc::c_int;
    while i < 15 as libc::c_int {
        Com_sprintf(
            name.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong as libc::c_int,
            b"%s/save/save%i/server.ssv\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            FS_Gamedir(),
            i,
        );
        f = fopen(name.as_mut_ptr(), b"rb\0" as *const u8 as *const libc::c_char);
        if f.is_null() {
            strcpy(
                (m_savestrings[i as usize]).as_mut_ptr(),
                b"<EMPTY>\0" as *const u8 as *const libc::c_char,
            );
            m_savevalid[i as usize] = false_0;
        } else {
            FS_Read(
                (m_savestrings[i as usize]).as_mut_ptr() as *mut libc::c_void,
                ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong
                    as libc::c_int,
                f,
            );
            fclose(f);
            m_savevalid[i as usize] = true_0;
        }
        i += 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn LoadGameCallback(mut self_0: *mut libc::c_void) {
    let mut a: *mut menuaction_s = self_0 as *mut menuaction_s;
    if m_savevalid[(*a).generic.localdata[0 as libc::c_int as usize] as usize] as u64
        != 0
    {
        Cbuf_AddText(
            va(
                b"load save%i\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                (*a).generic.localdata[0 as libc::c_int as usize],
            ),
        );
    }
    M_ForceMenuOff();
}

#[no_mangle]
pub unsafe extern "C" fn LoadGame_MenuInit() {
    let mut i: libc::c_int = 0;
    s_loadgame_menu.x = viddef.width / 2 as libc::c_int - 120 as libc::c_int;
    s_loadgame_menu.y = viddef.height / 2 as libc::c_int - 58 as libc::c_int;
    s_loadgame_menu.nitems = 0 as libc::c_int;
    Create_Savestrings();
    i = 0 as libc::c_int;
    while i < 15 as libc::c_int {
        s_loadgame_actions[i as usize]
            .generic
            .name = (m_savestrings[i as usize]).as_mut_ptr();
        s_loadgame_actions[i as usize]
            .generic
            .flags = 0x1 as libc::c_int as libc::c_uint;
        s_loadgame_actions[i as usize].generic.localdata[0 as libc::c_int as usize] = i;
        s_loadgame_actions[i as usize]
            .generic
            .callback = Some(
            LoadGameCallback as unsafe extern "C" fn(*mut libc::c_void) -> (),
        );
        s_loadgame_actions[i as usize].generic.x = 0 as libc::c_int;
        s_loadgame_actions[i as usize].generic.y = i * 10 as libc::c_int;
        if i > 0 as libc::c_int {
            s_loadgame_actions[i as usize].generic.y += 10 as libc::c_int;
        }
        s_loadgame_actions[i as usize].generic.type_0 = 2 as libc::c_int;
        Menu_AddItem(
            &mut s_loadgame_menu,
            &mut *s_loadgame_actions.as_mut_ptr().offset(i as isize) as *mut menuaction_s
                as *mut libc::c_void,
        );
        i += 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn LoadGame_MenuDraw() {
    M_Banner(
        b"m_banner_load_game\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    Menu_Draw(&mut s_loadgame_menu);
}

#[no_mangle]
pub unsafe extern "C" fn LoadGame_MenuKey(mut key: libc::c_int) -> *const libc::c_char {
    if key == 27 as libc::c_int || key == 13 as libc::c_int {
        s_savegame_menu.cursor = s_loadgame_menu.cursor - 1 as libc::c_int;
        if s_savegame_menu.cursor < 0 as libc::c_int {
            s_savegame_menu.cursor = 0 as libc::c_int;
        }
    }
    return Default_MenuKey(&mut s_loadgame_menu, key);
}

#[no_mangle]
pub unsafe extern "C" fn M_Menu_LoadGame_f() {
    LoadGame_MenuInit();
    M_PushMenu(
        Some(LoadGame_MenuDraw as unsafe extern "C" fn() -> ()),
        Some(
            LoadGame_MenuKey as unsafe extern "C" fn(libc::c_int) -> *const libc::c_char,
        ),
    );
}

static mut s_savegame_menu: menuframework_s = menuframework_s {
    x: 0,
    y: 0,
    cursor: 0,
    nitems: 0,
    nslots: 0,
    items: [0 as *const libc::c_void as *mut libc::c_void; 64],
    statusbar: 0 as *const libc::c_char,
    cursordraw: None,
};
static mut s_savegame_actions: [menuaction_s; 15] = [menuaction_s {
    generic: menucommon_s {
        type_0: 0,
        name: 0 as *const libc::c_char,
        x: 0,
        y: 0,
        parent: 0 as *const menuframework_s as *mut menuframework_s,
        cursor_offset: 0,
        localdata: [0; 4],
        flags: 0,
        statusbar: 0 as *const libc::c_char,
        callback: None,
        statusbarfunc: None,
        ownerdraw: None,
        cursordraw: None,
    },
}; 15];

#[no_mangle]
pub unsafe extern "C" fn SaveGameCallback(mut self_0: *mut libc::c_void) {
    let mut a: *mut menuaction_s = self_0 as *mut menuaction_s;
    Cbuf_AddText(
        va(
            b"save save%i\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            (*a).generic.localdata[0 as libc::c_int as usize],
        ),
    );
    M_ForceMenuOff();
}

#[no_mangle]
pub unsafe extern "C" fn SaveGame_MenuDraw() {
    M_Banner(
        b"m_banner_save_game\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    Menu_AdjustCursor(&mut s_savegame_menu, 1 as libc::c_int);
    Menu_Draw(&mut s_savegame_menu);
}

#[no_mangle]
pub unsafe extern "C" fn SaveGame_MenuInit() {
    let mut i: libc::c_int = 0;
    s_savegame_menu.x = viddef.width / 2 as libc::c_int - 120 as libc::c_int;
    s_savegame_menu.y = viddef.height / 2 as libc::c_int - 58 as libc::c_int;
    s_savegame_menu.nitems = 0 as libc::c_int;
    Create_Savestrings();
    i = 0 as libc::c_int;
    while i < 15 as libc::c_int - 1 as libc::c_int {
        s_savegame_actions[i as usize]
            .generic
            .name = (m_savestrings[(i + 1 as libc::c_int) as usize]).as_mut_ptr();
        s_savegame_actions[i as usize]
            .generic
            .localdata[0 as libc::c_int as usize] = i + 1 as libc::c_int;
        s_savegame_actions[i as usize]
            .generic
            .flags = 0x1 as libc::c_int as libc::c_uint;
        s_savegame_actions[i as usize]
            .generic
            .callback = Some(
            SaveGameCallback as unsafe extern "C" fn(*mut libc::c_void) -> (),
        );
        s_savegame_actions[i as usize].generic.x = 0 as libc::c_int;
        s_savegame_actions[i as usize].generic.y = i * 10 as libc::c_int;
        s_savegame_actions[i as usize].generic.type_0 = 2 as libc::c_int;
        Menu_AddItem(
            &mut s_savegame_menu,
            &mut *s_savegame_actions.as_mut_ptr().offset(i as isize) as *mut menuaction_s
                as *mut libc::c_void,
        );
        i += 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn SaveGame_MenuKey(mut key: libc::c_int) -> *const libc::c_char {
    if key == 13 as libc::c_int || key == 27 as libc::c_int {
        s_loadgame_menu.cursor = s_savegame_menu.cursor - 1 as libc::c_int;
        if s_loadgame_menu.cursor < 0 as libc::c_int {
            s_loadgame_menu.cursor = 0 as libc::c_int;
        }
    }
    return Default_MenuKey(&mut s_savegame_menu, key);
}

#[no_mangle]
pub unsafe extern "C" fn M_Menu_SaveGame_f() {
    if Com_ServerState() == 0 {
        return;
    }
    SaveGame_MenuInit();
    M_PushMenu(
        Some(SaveGame_MenuDraw as unsafe extern "C" fn() -> ()),
        Some(
            SaveGame_MenuKey as unsafe extern "C" fn(libc::c_int) -> *const libc::c_char,
        ),
    );
    Create_Savestrings();
}

static mut s_joinserver_menu: menuframework_s = menuframework_s {
    x: 0,
    y: 0,
    cursor: 0,
    nitems: 0,
    nslots: 0,
    items: [0 as *const libc::c_void as *mut libc::c_void; 64],
    statusbar: 0 as *const libc::c_char,
    cursordraw: None,
};
static mut s_joinserver_server_title: menuseparator_s = menuseparator_s {
    generic: menucommon_s {
        type_0: 0,
        name: 0 as *const libc::c_char,
        x: 0,
        y: 0,
        parent: 0 as *const menuframework_s as *mut menuframework_s,
        cursor_offset: 0,
        localdata: [0; 4],
        flags: 0,
        statusbar: 0 as *const libc::c_char,
        callback: None,
        statusbarfunc: None,
        ownerdraw: None,
        cursordraw: None,
    },
};
static mut s_joinserver_search_action: menuaction_s = menuaction_s {
    generic: menucommon_s {
        type_0: 0,
        name: 0 as *const libc::c_char,
        x: 0,
        y: 0,
        parent: 0 as *const menuframework_s as *mut menuframework_s,
        cursor_offset: 0,
        localdata: [0; 4],
        flags: 0,
        statusbar: 0 as *const libc::c_char,
        callback: None,
        statusbarfunc: None,
        ownerdraw: None,
        cursordraw: None,
    },
};
static mut s_joinserver_address_book_action: menuaction_s = menuaction_s {
    generic: menucommon_s {
        type_0: 0,
        name: 0 as *const libc::c_char,
        x: 0,
        y: 0,
        parent: 0 as *const menuframework_s as *mut menuframework_s,
        cursor_offset: 0,
        localdata: [0; 4],
        flags: 0,
        statusbar: 0 as *const libc::c_char,
        callback: None,
        statusbarfunc: None,
        ownerdraw: None,
        cursordraw: None,
    },
};
static mut s_joinserver_server_actions: [menuaction_s; 8] = [menuaction_s {
    generic: menucommon_s {
        type_0: 0,
        name: 0 as *const libc::c_char,
        x: 0,
        y: 0,
        parent: 0 as *const menuframework_s as *mut menuframework_s,
        cursor_offset: 0,
        localdata: [0; 4],
        flags: 0,
        statusbar: 0 as *const libc::c_char,
        callback: None,
        statusbarfunc: None,
        ownerdraw: None,
        cursordraw: None,
    },
}; 8];
#[no_mangle]
pub static mut m_num_servers: libc::c_int = 0;
static mut local_server_names: [[libc::c_char; 80]; 8] = [[0; 80]; 8];
static mut local_server_netadr: [netadr_t; 8] = [netadr_t {
    type_0: NA_LOOPBACK,
    ip: [0; 4],
    ipx: [0; 10],
    port: 0,
}; 8];

#[no_mangle]
pub unsafe extern "C" fn M_AddToServerList(
    mut adr: netadr_t,
    mut info: *mut libc::c_char,
) {
    let mut i: libc::c_int = 0;
    if m_num_servers == 8 as libc::c_int {
        return;
    }
    while *info as libc::c_int == ' ' as i32 {
        info = info.offset(1);
    }
    i = 0 as libc::c_int;
    while i < m_num_servers {
        if strcmp(info, (local_server_names[i as usize]).as_mut_ptr()) == 0 {
            return;
        }
        i += 1;
    }
    local_server_netadr[m_num_servers as usize] = adr;
    strncpy(
        (local_server_names[m_num_servers as usize]).as_mut_ptr(),
        info,
        (::std::mem::size_of::<[libc::c_char; 80]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
    );
    m_num_servers += 1;
}

#[no_mangle]
pub unsafe extern "C" fn JoinServerFunc(mut self_0: *mut libc::c_void) {
    let mut buffer: [libc::c_char; 128] = [0; 128];
    let mut index: libc::c_int = 0;
    index = (self_0 as *mut menuaction_s)
        .offset_from(s_joinserver_server_actions.as_mut_ptr()) as libc::c_long
        as libc::c_int;
    if Q_stricmp(
        (local_server_names[index as usize]).as_mut_ptr(),
        b"<no server>\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) == 0 as libc::c_int
    {
        return;
    }
    if index >= m_num_servers {
        return;
    }
    Com_sprintf(
        buffer.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong as libc::c_int,
        b"connect %s\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        NET_AdrToString(local_server_netadr[index as usize]),
    );
    Cbuf_AddText(buffer.as_mut_ptr());
    M_ForceMenuOff();
}

#[no_mangle]
pub unsafe extern "C" fn AddressBookFunc(mut self_0: *mut libc::c_void) {
    M_Menu_AddressBook_f();
}

#[no_mangle]
pub unsafe extern "C" fn NullCursorDraw(mut self_0: *mut libc::c_void) {}

#[no_mangle]
pub unsafe extern "C" fn SearchLocalGames() {
    let mut i: libc::c_int = 0;
    m_num_servers = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        strcpy(
            (local_server_names[i as usize]).as_mut_ptr(),
            b"<no server>\0" as *const u8 as *const libc::c_char,
        );
        i += 1;
    }
    M_DrawTextBox(
        8 as libc::c_int,
        120 as libc::c_int - 48 as libc::c_int,
        36 as libc::c_int,
        3 as libc::c_int,
    );
    M_Print(
        16 as libc::c_int + 16 as libc::c_int,
        120 as libc::c_int - 48 as libc::c_int + 8 as libc::c_int,
        b"Searching for local servers, this\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    M_Print(
        16 as libc::c_int + 16 as libc::c_int,
        120 as libc::c_int - 48 as libc::c_int + 16 as libc::c_int,
        b"could take up to a minute, so\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    M_Print(
        16 as libc::c_int + 16 as libc::c_int,
        120 as libc::c_int - 48 as libc::c_int + 24 as libc::c_int,
        b"please be patient.\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    (re.EndFrame).expect("non-null function pointer")();
    CL_PingServers_f();
}

#[no_mangle]
pub unsafe extern "C" fn SearchLocalGamesFunc(mut self_0: *mut libc::c_void) {
    SearchLocalGames();
}

#[no_mangle]
pub unsafe extern "C" fn JoinServer_MenuInit() {
    let mut i: libc::c_int = 0;
    s_joinserver_menu
        .x = (viddef.width as libc::c_double * 0.50f64
        - 120 as libc::c_int as libc::c_double) as libc::c_int;
    s_joinserver_menu.nitems = 0 as libc::c_int;
    s_joinserver_address_book_action.generic.type_0 = 2 as libc::c_int;
    s_joinserver_address_book_action
        .generic
        .name = b"address book\0" as *const u8 as *const libc::c_char;
    s_joinserver_address_book_action.generic.flags = 0x1 as libc::c_int as libc::c_uint;
    s_joinserver_address_book_action.generic.x = 0 as libc::c_int;
    s_joinserver_address_book_action.generic.y = 0 as libc::c_int;
    s_joinserver_address_book_action
        .generic
        .callback = Some(
        AddressBookFunc as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    s_joinserver_search_action.generic.type_0 = 2 as libc::c_int;
    s_joinserver_search_action
        .generic
        .name = b"refresh server list\0" as *const u8 as *const libc::c_char;
    s_joinserver_search_action.generic.flags = 0x1 as libc::c_int as libc::c_uint;
    s_joinserver_search_action.generic.x = 0 as libc::c_int;
    s_joinserver_search_action.generic.y = 10 as libc::c_int;
    s_joinserver_search_action
        .generic
        .callback = Some(
        SearchLocalGamesFunc as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    s_joinserver_search_action
        .generic
        .statusbar = b"search for servers\0" as *const u8 as *const libc::c_char;
    s_joinserver_server_title.generic.type_0 = 4 as libc::c_int;
    s_joinserver_server_title
        .generic
        .name = b"connect to...\0" as *const u8 as *const libc::c_char;
    s_joinserver_server_title.generic.x = 80 as libc::c_int;
    s_joinserver_server_title.generic.y = 30 as libc::c_int;
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        s_joinserver_server_actions[i as usize].generic.type_0 = 2 as libc::c_int;
        strcpy(
            (local_server_names[i as usize]).as_mut_ptr(),
            b"<no server>\0" as *const u8 as *const libc::c_char,
        );
        s_joinserver_server_actions[i as usize]
            .generic
            .name = (local_server_names[i as usize]).as_mut_ptr();
        s_joinserver_server_actions[i as usize]
            .generic
            .flags = 0x1 as libc::c_int as libc::c_uint;
        s_joinserver_server_actions[i as usize].generic.x = 0 as libc::c_int;
        s_joinserver_server_actions[i as usize]
            .generic
            .y = 40 as libc::c_int + i * 10 as libc::c_int;
        s_joinserver_server_actions[i as usize]
            .generic
            .callback = Some(
            JoinServerFunc as unsafe extern "C" fn(*mut libc::c_void) -> (),
        );
        s_joinserver_server_actions[i as usize]
            .generic
            .statusbar = b"press ENTER to connect\0" as *const u8 as *const libc::c_char;
        i += 1;
    }
    Menu_AddItem(
        &mut s_joinserver_menu,
        &mut s_joinserver_address_book_action as *mut menuaction_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_joinserver_menu,
        &mut s_joinserver_server_title as *mut menuseparator_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_joinserver_menu,
        &mut s_joinserver_search_action as *mut menuaction_s as *mut libc::c_void,
    );
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        Menu_AddItem(
            &mut s_joinserver_menu,
            &mut *s_joinserver_server_actions.as_mut_ptr().offset(i as isize)
                as *mut menuaction_s as *mut libc::c_void,
        );
        i += 1;
    }
    Menu_Center(&mut s_joinserver_menu);
    SearchLocalGames();
}

#[no_mangle]
pub unsafe extern "C" fn JoinServer_MenuDraw() {
    M_Banner(
        b"m_banner_join_server\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    Menu_Draw(&mut s_joinserver_menu);
}

#[no_mangle]
pub unsafe extern "C" fn JoinServer_MenuKey(
    mut key: libc::c_int,
) -> *const libc::c_char {
    return Default_MenuKey(&mut s_joinserver_menu, key);
}

#[no_mangle]
pub unsafe extern "C" fn M_Menu_JoinServer_f() {
    JoinServer_MenuInit();
    M_PushMenu(
        Some(JoinServer_MenuDraw as unsafe extern "C" fn() -> ()),
        Some(
            JoinServer_MenuKey
                as unsafe extern "C" fn(libc::c_int) -> *const libc::c_char,
        ),
    );
}

static mut s_startserver_menu: menuframework_s = menuframework_s {
    x: 0,
    y: 0,
    cursor: 0,
    nitems: 0,
    nslots: 0,
    items: [0 as *const libc::c_void as *mut libc::c_void; 64],
    statusbar: 0 as *const libc::c_char,
    cursordraw: None,
};
static mut mapnames: *mut *mut libc::c_char = 0 as *const *mut libc::c_char
    as *mut *mut libc::c_char;
static mut nummaps: libc::c_int = 0;
static mut s_startserver_start_action: menuaction_s = menuaction_s {
    generic: menucommon_s {
        type_0: 0,
        name: 0 as *const libc::c_char,
        x: 0,
        y: 0,
        parent: 0 as *const menuframework_s as *mut menuframework_s,
        cursor_offset: 0,
        localdata: [0; 4],
        flags: 0,
        statusbar: 0 as *const libc::c_char,
        callback: None,
        statusbarfunc: None,
        ownerdraw: None,
        cursordraw: None,
    },
};
static mut s_startserver_dmoptions_action: menuaction_s = menuaction_s {
    generic: menucommon_s {
        type_0: 0,
        name: 0 as *const libc::c_char,
        x: 0,
        y: 0,
        parent: 0 as *const menuframework_s as *mut menuframework_s,
        cursor_offset: 0,
        localdata: [0; 4],
        flags: 0,
        statusbar: 0 as *const libc::c_char,
        callback: None,
        statusbarfunc: None,
        ownerdraw: None,
        cursordraw: None,
    },
};
static mut s_timelimit_field: menufield_s = menufield_s {
    generic: menucommon_s {
        type_0: 0,
        name: 0 as *const libc::c_char,
        x: 0,
        y: 0,
        parent: 0 as *const menuframework_s as *mut menuframework_s,
        cursor_offset: 0,
        localdata: [0; 4],
        flags: 0,
        statusbar: 0 as *const libc::c_char,
        callback: None,
        statusbarfunc: None,
        ownerdraw: None,
        cursordraw: None,
    },
    buffer: [0; 80],
    cursor: 0,
    length: 0,
    visible_length: 0,
    visible_offset: 0,
};
static mut s_fraglimit_field: menufield_s = menufield_s {
    generic: menucommon_s {
        type_0: 0,
        name: 0 as *const libc::c_char,
        x: 0,
        y: 0,
        parent: 0 as *const menuframework_s as *mut menuframework_s,
        cursor_offset: 0,
        localdata: [0; 4],
        flags: 0,
        statusbar: 0 as *const libc::c_char,
        callback: None,
        statusbarfunc: None,
        ownerdraw: None,
        cursordraw: None,
    },
    buffer: [0; 80],
    cursor: 0,
    length: 0,
    visible_length: 0,
    visible_offset: 0,
};
static mut s_maxclients_field: menufield_s = menufield_s {
    generic: menucommon_s {
        type_0: 0,
        name: 0 as *const libc::c_char,
        x: 0,
        y: 0,
        parent: 0 as *const menuframework_s as *mut menuframework_s,
        cursor_offset: 0,
        localdata: [0; 4],
        flags: 0,
        statusbar: 0 as *const libc::c_char,
        callback: None,
        statusbarfunc: None,
        ownerdraw: None,
        cursordraw: None,
    },
    buffer: [0; 80],
    cursor: 0,
    length: 0,
    visible_length: 0,
    visible_offset: 0,
};
static mut s_hostname_field: menufield_s = menufield_s {
    generic: menucommon_s {
        type_0: 0,
        name: 0 as *const libc::c_char,
        x: 0,
        y: 0,
        parent: 0 as *const menuframework_s as *mut menuframework_s,
        cursor_offset: 0,
        localdata: [0; 4],
        flags: 0,
        statusbar: 0 as *const libc::c_char,
        callback: None,
        statusbarfunc: None,
        ownerdraw: None,
        cursordraw: None,
    },
    buffer: [0; 80],
    cursor: 0,
    length: 0,
    visible_length: 0,
    visible_offset: 0,
};
static mut s_startmap_list: menulist_s = menulist_s {
    generic: menucommon_s {
        type_0: 0,
        name: 0 as *const libc::c_char,
        x: 0,
        y: 0,
        parent: 0 as *const menuframework_s as *mut menuframework_s,
        cursor_offset: 0,
        localdata: [0; 4],
        flags: 0,
        statusbar: 0 as *const libc::c_char,
        callback: None,
        statusbarfunc: None,
        ownerdraw: None,
        cursordraw: None,
    },
    curvalue: 0,
    itemnames: 0 as *const *const libc::c_char as *mut *const libc::c_char,
};
static mut s_rules_box: menulist_s = menulist_s {
    generic: menucommon_s {
        type_0: 0,
        name: 0 as *const libc::c_char,
        x: 0,
        y: 0,
        parent: 0 as *const menuframework_s as *mut menuframework_s,
        cursor_offset: 0,
        localdata: [0; 4],
        flags: 0,
        statusbar: 0 as *const libc::c_char,
        callback: None,
        statusbarfunc: None,
        ownerdraw: None,
        cursordraw: None,
    },
    curvalue: 0,
    itemnames: 0 as *const *const libc::c_char as *mut *const libc::c_char,
};

#[no_mangle]
pub unsafe extern "C" fn DMOptionsFunc(mut self_0: *mut libc::c_void) {
    if s_rules_box.curvalue == 1 as libc::c_int {
        return;
    }
    M_Menu_DMOptions_f();
}

#[no_mangle]
pub unsafe extern "C" fn RulesChangeFunc(mut self_0: *mut libc::c_void) {
    if s_rules_box.curvalue == 0 as libc::c_int {
        s_maxclients_field.generic.statusbar = 0 as *const libc::c_char;
        s_startserver_dmoptions_action.generic.statusbar = 0 as *const libc::c_char;
    } else if s_rules_box.curvalue == 1 as libc::c_int {
        s_maxclients_field
            .generic
            .statusbar = b"4 maximum for cooperative\0" as *const u8
            as *const libc::c_char;
        if atoi((s_maxclients_field.buffer).as_mut_ptr()) > 4 as libc::c_int {
            strcpy(
                (s_maxclients_field.buffer).as_mut_ptr(),
                b"4\0" as *const u8 as *const libc::c_char,
            );
        }
        s_startserver_dmoptions_action
            .generic
            .statusbar = b"N/A for cooperative\0" as *const u8 as *const libc::c_char;
    } else if Developer_searchpath(2 as libc::c_int) == 2 as libc::c_int {
        if s_rules_box.curvalue == 2 as libc::c_int {
            s_maxclients_field.generic.statusbar = 0 as *const libc::c_char;
            s_startserver_dmoptions_action.generic.statusbar = 0 as *const libc::c_char;
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn StartServerActionFunc(mut self_0: *mut libc::c_void) {
    let mut startmap: [libc::c_char; 1024] = [0; 1024];
    let mut timelimit: libc::c_int = 0;
    let mut fraglimit: libc::c_int = 0;
    let mut maxclients: libc::c_int = 0;
    let mut spot: *mut libc::c_char = 0 as *mut libc::c_char;
    strcpy(
        startmap.as_mut_ptr(),
        (strchr(*mapnames.offset(s_startmap_list.curvalue as isize), '\n' as i32))
            .offset(1 as libc::c_int as isize),
    );
    maxclients = atoi((s_maxclients_field.buffer).as_mut_ptr());
    timelimit = atoi((s_timelimit_field.buffer).as_mut_ptr());
    fraglimit = atoi((s_fraglimit_field.buffer).as_mut_ptr());
    Cvar_SetValue(
        b"maxclients\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ClampCvar(
            0 as libc::c_int as libc::c_float,
            maxclients as libc::c_float,
            maxclients as libc::c_float,
        ),
    );
    Cvar_SetValue(
        b"timelimit\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ClampCvar(
            0 as libc::c_int as libc::c_float,
            timelimit as libc::c_float,
            timelimit as libc::c_float,
        ),
    );
    Cvar_SetValue(
        b"fraglimit\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ClampCvar(
            0 as libc::c_int as libc::c_float,
            fraglimit as libc::c_float,
            fraglimit as libc::c_float,
        ),
    );
    Cvar_Set(
        b"hostname\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        (s_hostname_field.buffer).as_mut_ptr(),
    );
    if s_rules_box.curvalue < 2 as libc::c_int
        || Developer_searchpath(2 as libc::c_int) != 2 as libc::c_int
    {
        Cvar_SetValue(
            b"deathmatch\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            (s_rules_box.curvalue == 0) as libc::c_int as libc::c_float,
        );
        Cvar_SetValue(
            b"coop\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            s_rules_box.curvalue as libc::c_float,
        );
        Cvar_SetValue(
            b"gamerules\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            0 as libc::c_int as libc::c_float,
        );
    } else {
        Cvar_SetValue(
            b"deathmatch\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            1 as libc::c_int as libc::c_float,
        );
        Cvar_SetValue(
            b"coop\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            0 as libc::c_int as libc::c_float,
        );
        Cvar_SetValue(
            b"gamerules\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            s_rules_box.curvalue as libc::c_float,
        );
    }
    spot = 0 as *mut libc::c_char;
    if s_rules_box.curvalue == 1 as libc::c_int {
        if Q_stricmp(
            startmap.as_mut_ptr(),
            b"bunk1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) == 0 as libc::c_int
        {
            spot = b"start\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        } else if Q_stricmp(
            startmap.as_mut_ptr(),
            b"mintro\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) == 0 as libc::c_int
        {
            spot = b"start\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        } else if Q_stricmp(
            startmap.as_mut_ptr(),
            b"fact1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) == 0 as libc::c_int
        {
            spot = b"start\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        } else if Q_stricmp(
            startmap.as_mut_ptr(),
            b"power1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) == 0 as libc::c_int
        {
            spot = b"pstart\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        } else if Q_stricmp(
            startmap.as_mut_ptr(),
            b"biggun\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) == 0 as libc::c_int
        {
            spot = b"bstart\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        } else if Q_stricmp(
            startmap.as_mut_ptr(),
            b"hangar1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) == 0 as libc::c_int
        {
            spot = b"unitstart\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char;
        } else if Q_stricmp(
            startmap.as_mut_ptr(),
            b"city1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) == 0 as libc::c_int
        {
            spot = b"unitstart\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char;
        } else if Q_stricmp(
            startmap.as_mut_ptr(),
            b"boss1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) == 0 as libc::c_int
        {
            spot = b"bosstart\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char;
        }
    }
    if !spot.is_null() {
        if Com_ServerState() != 0 {
            Cbuf_AddText(
                b"disconnect\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
        }
        Cbuf_AddText(
            va(
                b"gamemap \"*%s$%s\"\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                startmap.as_mut_ptr(),
                spot,
            ),
        );
    } else {
        Cbuf_AddText(
            va(
                b"map %s\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                startmap.as_mut_ptr(),
            ),
        );
    }
    M_ForceMenuOff();
}

#[no_mangle]
pub unsafe extern "C" fn StartServer_MenuInit() {
    static mut dm_coop_names: [*const libc::c_char; 3] = [
        b"deathmatch\0" as *const u8 as *const libc::c_char,
        b"cooperative\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    static mut dm_coop_names_rogue: [*const libc::c_char; 4] = [
        b"deathmatch\0" as *const u8 as *const libc::c_char,
        b"cooperative\0" as *const u8 as *const libc::c_char,
        b"tag\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut buffer: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut mapsname: [libc::c_char; 1024] = [0; 1024];
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut length: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut fp: *mut FILE = 0 as *mut FILE;
    Com_sprintf(
        mapsname.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
        b"%s/maps.lst\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        FS_Gamedir(),
    );
    fp = fopen(mapsname.as_mut_ptr(), b"rb\0" as *const u8 as *const libc::c_char);
    if fp.is_null() {
        length = FS_LoadFile(
            b"maps.lst\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            &mut buffer as *mut *mut libc::c_char as *mut *mut libc::c_void,
        );
        if length == -(1 as libc::c_int) {
            Com_Error(
                1 as libc::c_int,
                b"couldn't find maps.lst\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
        }
    } else {
        fseek(fp, 0 as libc::c_int as libc::c_long, 2 as libc::c_int);
        length = ftell(fp) as libc::c_int;
        fseek(fp, 0 as libc::c_int as libc::c_long, 0 as libc::c_int);
        buffer = malloc(length as libc::c_ulong) as *mut libc::c_char;
        fread(
            buffer as *mut libc::c_void,
            length as libc::c_ulong,
            1 as libc::c_int as libc::c_ulong,
            fp,
        );
    }
    s = buffer;
    i = 0 as libc::c_int;
    while i < length {
        if *s.offset(i as isize) as libc::c_int == '\r' as i32 {
            nummaps += 1;
        }
        i += 1;
    }
    if nummaps == 0 as libc::c_int {
        Com_Error(
            1 as libc::c_int,
            b"no maps in maps.lst\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    mapnames = malloc(
        (::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
            .wrapping_mul((nummaps + 1 as libc::c_int) as libc::c_ulong),
    ) as *mut *mut libc::c_char;
    memset(
        mapnames as *mut libc::c_void,
        0 as libc::c_int,
        (::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
            .wrapping_mul((nummaps + 1 as libc::c_int) as libc::c_ulong),
    );
    s = buffer;
    i = 0 as libc::c_int;
    while i < nummaps {
        let mut shortname: [libc::c_char; 128] = [0; 128];
        let mut longname: [libc::c_char; 128] = [0; 128];
        let mut scratch: [libc::c_char; 200] = [0; 200];
        let mut j: libc::c_int = 0;
        let mut l: libc::c_int = 0;
        strcpy(shortname.as_mut_ptr(), COM_Parse(&mut s));
        l = strlen(shortname.as_mut_ptr()) as libc::c_int;
        j = 0 as libc::c_int;
        while j < l {
            shortname[j
                as usize] = toupper(shortname[j as usize] as libc::c_int)
                as libc::c_char;
            j += 1;
        }
        strcpy(longname.as_mut_ptr(), COM_Parse(&mut s));
        Com_sprintf(
            scratch.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 200]>() as libc::c_ulong as libc::c_int,
            b"%s\n%s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            longname.as_mut_ptr(),
            shortname.as_mut_ptr(),
        );
        let ref mut fresh6 = *mapnames.offset(i as isize);
        *fresh6 = malloc(
            (strlen(scratch.as_mut_ptr()))
                .wrapping_add(1 as libc::c_int as libc::c_ulong),
        ) as *mut libc::c_char;
        strcpy(*mapnames.offset(i as isize), scratch.as_mut_ptr());
        i += 1;
    }
    let ref mut fresh7 = *mapnames.offset(nummaps as isize);
    *fresh7 = 0 as *mut libc::c_char;
    if !fp.is_null() {
        fp = 0 as *mut FILE;
        free(buffer as *mut libc::c_void);
    } else {
        FS_FreeFile(buffer as *mut libc::c_void);
    }
    s_startserver_menu.x = (viddef.width as libc::c_double * 0.50f64) as libc::c_int;
    s_startserver_menu.nitems = 0 as libc::c_int;
    s_startmap_list.generic.type_0 = 3 as libc::c_int;
    s_startmap_list.generic.x = 0 as libc::c_int;
    s_startmap_list.generic.y = 0 as libc::c_int;
    s_startmap_list.generic.name = b"initial map\0" as *const u8 as *const libc::c_char;
    s_startmap_list.itemnames = mapnames as *mut *const libc::c_char;
    s_rules_box.generic.type_0 = 3 as libc::c_int;
    s_rules_box.generic.x = 0 as libc::c_int;
    s_rules_box.generic.y = 20 as libc::c_int;
    s_rules_box.generic.name = b"rules\0" as *const u8 as *const libc::c_char;
    if Developer_searchpath(2 as libc::c_int) == 2 as libc::c_int {
        s_rules_box.itemnames = dm_coop_names_rogue.as_mut_ptr();
    } else {
        s_rules_box.itemnames = dm_coop_names.as_mut_ptr();
    }
    if Cvar_VariableValue(
        b"coop\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) != 0.
    {
        s_rules_box.curvalue = 1 as libc::c_int;
    } else {
        s_rules_box.curvalue = 0 as libc::c_int;
    }
    s_rules_box
        .generic
        .callback = Some(
        RulesChangeFunc as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    s_timelimit_field.generic.type_0 = 5 as libc::c_int;
    s_timelimit_field.generic.name = b"time limit\0" as *const u8 as *const libc::c_char;
    s_timelimit_field.generic.flags = 0x4 as libc::c_int as libc::c_uint;
    s_timelimit_field.generic.x = 0 as libc::c_int;
    s_timelimit_field.generic.y = 36 as libc::c_int;
    s_timelimit_field
        .generic
        .statusbar = b"0 = no limit\0" as *const u8 as *const libc::c_char;
    s_timelimit_field.length = 3 as libc::c_int;
    s_timelimit_field.visible_length = 3 as libc::c_int;
    strcpy(
        (s_timelimit_field.buffer).as_mut_ptr(),
        Cvar_VariableString(
            b"timelimit\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ),
    );
    s_fraglimit_field.generic.type_0 = 5 as libc::c_int;
    s_fraglimit_field.generic.name = b"frag limit\0" as *const u8 as *const libc::c_char;
    s_fraglimit_field.generic.flags = 0x4 as libc::c_int as libc::c_uint;
    s_fraglimit_field.generic.x = 0 as libc::c_int;
    s_fraglimit_field.generic.y = 54 as libc::c_int;
    s_fraglimit_field
        .generic
        .statusbar = b"0 = no limit\0" as *const u8 as *const libc::c_char;
    s_fraglimit_field.length = 3 as libc::c_int;
    s_fraglimit_field.visible_length = 3 as libc::c_int;
    strcpy(
        (s_fraglimit_field.buffer).as_mut_ptr(),
        Cvar_VariableString(
            b"fraglimit\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ),
    );
    s_maxclients_field.generic.type_0 = 5 as libc::c_int;
    s_maxclients_field
        .generic
        .name = b"max players\0" as *const u8 as *const libc::c_char;
    s_maxclients_field.generic.flags = 0x4 as libc::c_int as libc::c_uint;
    s_maxclients_field.generic.x = 0 as libc::c_int;
    s_maxclients_field.generic.y = 72 as libc::c_int;
    s_maxclients_field.generic.statusbar = 0 as *const libc::c_char;
    s_maxclients_field.length = 3 as libc::c_int;
    s_maxclients_field.visible_length = 3 as libc::c_int;
    if Cvar_VariableValue(
        b"maxclients\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) == 1 as libc::c_int as libc::c_float
    {
        strcpy(
            (s_maxclients_field.buffer).as_mut_ptr(),
            b"8\0" as *const u8 as *const libc::c_char,
        );
    } else {
        strcpy(
            (s_maxclients_field.buffer).as_mut_ptr(),
            Cvar_VariableString(
                b"maxclients\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ),
        );
    }
    s_hostname_field.generic.type_0 = 5 as libc::c_int;
    s_hostname_field.generic.name = b"hostname\0" as *const u8 as *const libc::c_char;
    s_hostname_field.generic.flags = 0 as libc::c_int as libc::c_uint;
    s_hostname_field.generic.x = 0 as libc::c_int;
    s_hostname_field.generic.y = 90 as libc::c_int;
    s_hostname_field.generic.statusbar = 0 as *const libc::c_char;
    s_hostname_field.length = 12 as libc::c_int;
    s_hostname_field.visible_length = 12 as libc::c_int;
    strcpy(
        (s_hostname_field.buffer).as_mut_ptr(),
        Cvar_VariableString(
            b"hostname\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ),
    );
    s_startserver_dmoptions_action.generic.type_0 = 2 as libc::c_int;
    s_startserver_dmoptions_action
        .generic
        .name = b" deathmatch flags\0" as *const u8 as *const libc::c_char;
    s_startserver_dmoptions_action.generic.flags = 0x1 as libc::c_int as libc::c_uint;
    s_startserver_dmoptions_action.generic.x = 24 as libc::c_int;
    s_startserver_dmoptions_action.generic.y = 108 as libc::c_int;
    s_startserver_dmoptions_action.generic.statusbar = 0 as *const libc::c_char;
    s_startserver_dmoptions_action
        .generic
        .callback = Some(DMOptionsFunc as unsafe extern "C" fn(*mut libc::c_void) -> ());
    s_startserver_start_action.generic.type_0 = 2 as libc::c_int;
    s_startserver_start_action
        .generic
        .name = b" begin\0" as *const u8 as *const libc::c_char;
    s_startserver_start_action.generic.flags = 0x1 as libc::c_int as libc::c_uint;
    s_startserver_start_action.generic.x = 24 as libc::c_int;
    s_startserver_start_action.generic.y = 128 as libc::c_int;
    s_startserver_start_action
        .generic
        .callback = Some(
        StartServerActionFunc as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    Menu_AddItem(
        &mut s_startserver_menu,
        &mut s_startmap_list as *mut menulist_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_startserver_menu,
        &mut s_rules_box as *mut menulist_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_startserver_menu,
        &mut s_timelimit_field as *mut menufield_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_startserver_menu,
        &mut s_fraglimit_field as *mut menufield_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_startserver_menu,
        &mut s_maxclients_field as *mut menufield_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_startserver_menu,
        &mut s_hostname_field as *mut menufield_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_startserver_menu,
        &mut s_startserver_dmoptions_action as *mut menuaction_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_startserver_menu,
        &mut s_startserver_start_action as *mut menuaction_s as *mut libc::c_void,
    );
    Menu_Center(&mut s_startserver_menu);
    RulesChangeFunc(0 as *mut libc::c_void);
}

#[no_mangle]
pub unsafe extern "C" fn StartServer_MenuDraw() {
    Menu_Draw(&mut s_startserver_menu);
}

#[no_mangle]
pub unsafe extern "C" fn StartServer_MenuKey(
    mut key: libc::c_int,
) -> *const libc::c_char {
    if key == 27 as libc::c_int {
        if !mapnames.is_null() {
            let mut i: libc::c_int = 0;
            i = 0 as libc::c_int;
            while i < nummaps {
                free(*mapnames.offset(i as isize) as *mut libc::c_void);
                i += 1;
            }
            free(mapnames as *mut libc::c_void);
        }
        mapnames = 0 as *mut *mut libc::c_char;
        nummaps = 0 as libc::c_int;
    }
    return Default_MenuKey(&mut s_startserver_menu, key);
}

#[no_mangle]
pub unsafe extern "C" fn M_Menu_StartServer_f() {
    StartServer_MenuInit();
    M_PushMenu(
        Some(StartServer_MenuDraw as unsafe extern "C" fn() -> ()),
        Some(
            StartServer_MenuKey
                as unsafe extern "C" fn(libc::c_int) -> *const libc::c_char,
        ),
    );
}

static mut dmoptions_statusbar: [libc::c_char; 128] = [0; 128];
static mut s_dmoptions_menu: menuframework_s = menuframework_s {
    x: 0,
    y: 0,
    cursor: 0,
    nitems: 0,
    nslots: 0,
    items: [0 as *const libc::c_void as *mut libc::c_void; 64],
    statusbar: 0 as *const libc::c_char,
    cursordraw: None,
};
static mut s_friendlyfire_box: menulist_s = menulist_s {
    generic: menucommon_s {
        type_0: 0,
        name: 0 as *const libc::c_char,
        x: 0,
        y: 0,
        parent: 0 as *const menuframework_s as *mut menuframework_s,
        cursor_offset: 0,
        localdata: [0; 4],
        flags: 0,
        statusbar: 0 as *const libc::c_char,
        callback: None,
        statusbarfunc: None,
        ownerdraw: None,
        cursordraw: None,
    },
    curvalue: 0,
    itemnames: 0 as *const *const libc::c_char as *mut *const libc::c_char,
};
static mut s_falls_box: menulist_s = menulist_s {
    generic: menucommon_s {
        type_0: 0,
        name: 0 as *const libc::c_char,
        x: 0,
        y: 0,
        parent: 0 as *const menuframework_s as *mut menuframework_s,
        cursor_offset: 0,
        localdata: [0; 4],
        flags: 0,
        statusbar: 0 as *const libc::c_char,
        callback: None,
        statusbarfunc: None,
        ownerdraw: None,
        cursordraw: None,
    },
    curvalue: 0,
    itemnames: 0 as *const *const libc::c_char as *mut *const libc::c_char,
};
static mut s_weapons_stay_box: menulist_s = menulist_s {
    generic: menucommon_s {
        type_0: 0,
        name: 0 as *const libc::c_char,
        x: 0,
        y: 0,
        parent: 0 as *const menuframework_s as *mut menuframework_s,
        cursor_offset: 0,
        localdata: [0; 4],
        flags: 0,
        statusbar: 0 as *const libc::c_char,
        callback: None,
        statusbarfunc: None,
        ownerdraw: None,
        cursordraw: None,
    },
    curvalue: 0,
    itemnames: 0 as *const *const libc::c_char as *mut *const libc::c_char,
};
static mut s_instant_powerups_box: menulist_s = menulist_s {
    generic: menucommon_s {
        type_0: 0,
        name: 0 as *const libc::c_char,
        x: 0,
        y: 0,
        parent: 0 as *const menuframework_s as *mut menuframework_s,
        cursor_offset: 0,
        localdata: [0; 4],
        flags: 0,
        statusbar: 0 as *const libc::c_char,
        callback: None,
        statusbarfunc: None,
        ownerdraw: None,
        cursordraw: None,
    },
    curvalue: 0,
    itemnames: 0 as *const *const libc::c_char as *mut *const libc::c_char,
};
static mut s_powerups_box: menulist_s = menulist_s {
    generic: menucommon_s {
        type_0: 0,
        name: 0 as *const libc::c_char,
        x: 0,
        y: 0,
        parent: 0 as *const menuframework_s as *mut menuframework_s,
        cursor_offset: 0,
        localdata: [0; 4],
        flags: 0,
        statusbar: 0 as *const libc::c_char,
        callback: None,
        statusbarfunc: None,
        ownerdraw: None,
        cursordraw: None,
    },
    curvalue: 0,
    itemnames: 0 as *const *const libc::c_char as *mut *const libc::c_char,
};
static mut s_health_box: menulist_s = menulist_s {
    generic: menucommon_s {
        type_0: 0,
        name: 0 as *const libc::c_char,
        x: 0,
        y: 0,
        parent: 0 as *const menuframework_s as *mut menuframework_s,
        cursor_offset: 0,
        localdata: [0; 4],
        flags: 0,
        statusbar: 0 as *const libc::c_char,
        callback: None,
        statusbarfunc: None,
        ownerdraw: None,
        cursordraw: None,
    },
    curvalue: 0,
    itemnames: 0 as *const *const libc::c_char as *mut *const libc::c_char,
};
static mut s_spawn_farthest_box: menulist_s = menulist_s {
    generic: menucommon_s {
        type_0: 0,
        name: 0 as *const libc::c_char,
        x: 0,
        y: 0,
        parent: 0 as *const menuframework_s as *mut menuframework_s,
        cursor_offset: 0,
        localdata: [0; 4],
        flags: 0,
        statusbar: 0 as *const libc::c_char,
        callback: None,
        statusbarfunc: None,
        ownerdraw: None,
        cursordraw: None,
    },
    curvalue: 0,
    itemnames: 0 as *const *const libc::c_char as *mut *const libc::c_char,
};
static mut s_teamplay_box: menulist_s = menulist_s {
    generic: menucommon_s {
        type_0: 0,
        name: 0 as *const libc::c_char,
        x: 0,
        y: 0,
        parent: 0 as *const menuframework_s as *mut menuframework_s,
        cursor_offset: 0,
        localdata: [0; 4],
        flags: 0,
        statusbar: 0 as *const libc::c_char,
        callback: None,
        statusbarfunc: None,
        ownerdraw: None,
        cursordraw: None,
    },
    curvalue: 0,
    itemnames: 0 as *const *const libc::c_char as *mut *const libc::c_char,
};
static mut s_samelevel_box: menulist_s = menulist_s {
    generic: menucommon_s {
        type_0: 0,
        name: 0 as *const libc::c_char,
        x: 0,
        y: 0,
        parent: 0 as *const menuframework_s as *mut menuframework_s,
        cursor_offset: 0,
        localdata: [0; 4],
        flags: 0,
        statusbar: 0 as *const libc::c_char,
        callback: None,
        statusbarfunc: None,
        ownerdraw: None,
        cursordraw: None,
    },
    curvalue: 0,
    itemnames: 0 as *const *const libc::c_char as *mut *const libc::c_char,
};
static mut s_force_respawn_box: menulist_s = menulist_s {
    generic: menucommon_s {
        type_0: 0,
        name: 0 as *const libc::c_char,
        x: 0,
        y: 0,
        parent: 0 as *const menuframework_s as *mut menuframework_s,
        cursor_offset: 0,
        localdata: [0; 4],
        flags: 0,
        statusbar: 0 as *const libc::c_char,
        callback: None,
        statusbarfunc: None,
        ownerdraw: None,
        cursordraw: None,
    },
    curvalue: 0,
    itemnames: 0 as *const *const libc::c_char as *mut *const libc::c_char,
};
static mut s_armor_box: menulist_s = menulist_s {
    generic: menucommon_s {
        type_0: 0,
        name: 0 as *const libc::c_char,
        x: 0,
        y: 0,
        parent: 0 as *const menuframework_s as *mut menuframework_s,
        cursor_offset: 0,
        localdata: [0; 4],
        flags: 0,
        statusbar: 0 as *const libc::c_char,
        callback: None,
        statusbarfunc: None,
        ownerdraw: None,
        cursordraw: None,
    },
    curvalue: 0,
    itemnames: 0 as *const *const libc::c_char as *mut *const libc::c_char,
};
static mut s_allow_exit_box: menulist_s = menulist_s {
    generic: menucommon_s {
        type_0: 0,
        name: 0 as *const libc::c_char,
        x: 0,
        y: 0,
        parent: 0 as *const menuframework_s as *mut menuframework_s,
        cursor_offset: 0,
        localdata: [0; 4],
        flags: 0,
        statusbar: 0 as *const libc::c_char,
        callback: None,
        statusbarfunc: None,
        ownerdraw: None,
        cursordraw: None,
    },
    curvalue: 0,
    itemnames: 0 as *const *const libc::c_char as *mut *const libc::c_char,
};
static mut s_infinite_ammo_box: menulist_s = menulist_s {
    generic: menucommon_s {
        type_0: 0,
        name: 0 as *const libc::c_char,
        x: 0,
        y: 0,
        parent: 0 as *const menuframework_s as *mut menuframework_s,
        cursor_offset: 0,
        localdata: [0; 4],
        flags: 0,
        statusbar: 0 as *const libc::c_char,
        callback: None,
        statusbarfunc: None,
        ownerdraw: None,
        cursordraw: None,
    },
    curvalue: 0,
    itemnames: 0 as *const *const libc::c_char as *mut *const libc::c_char,
};
static mut s_fixed_fov_box: menulist_s = menulist_s {
    generic: menucommon_s {
        type_0: 0,
        name: 0 as *const libc::c_char,
        x: 0,
        y: 0,
        parent: 0 as *const menuframework_s as *mut menuframework_s,
        cursor_offset: 0,
        localdata: [0; 4],
        flags: 0,
        statusbar: 0 as *const libc::c_char,
        callback: None,
        statusbarfunc: None,
        ownerdraw: None,
        cursordraw: None,
    },
    curvalue: 0,
    itemnames: 0 as *const *const libc::c_char as *mut *const libc::c_char,
};
static mut s_quad_drop_box: menulist_s = menulist_s {
    generic: menucommon_s {
        type_0: 0,
        name: 0 as *const libc::c_char,
        x: 0,
        y: 0,
        parent: 0 as *const menuframework_s as *mut menuframework_s,
        cursor_offset: 0,
        localdata: [0; 4],
        flags: 0,
        statusbar: 0 as *const libc::c_char,
        callback: None,
        statusbarfunc: None,
        ownerdraw: None,
        cursordraw: None,
    },
    curvalue: 0,
    itemnames: 0 as *const *const libc::c_char as *mut *const libc::c_char,
};
static mut s_no_mines_box: menulist_s = menulist_s {
    generic: menucommon_s {
        type_0: 0,
        name: 0 as *const libc::c_char,
        x: 0,
        y: 0,
        parent: 0 as *const menuframework_s as *mut menuframework_s,
        cursor_offset: 0,
        localdata: [0; 4],
        flags: 0,
        statusbar: 0 as *const libc::c_char,
        callback: None,
        statusbarfunc: None,
        ownerdraw: None,
        cursordraw: None,
    },
    curvalue: 0,
    itemnames: 0 as *const *const libc::c_char as *mut *const libc::c_char,
};
static mut s_no_nukes_box: menulist_s = menulist_s {
    generic: menucommon_s {
        type_0: 0,
        name: 0 as *const libc::c_char,
        x: 0,
        y: 0,
        parent: 0 as *const menuframework_s as *mut menuframework_s,
        cursor_offset: 0,
        localdata: [0; 4],
        flags: 0,
        statusbar: 0 as *const libc::c_char,
        callback: None,
        statusbarfunc: None,
        ownerdraw: None,
        cursordraw: None,
    },
    curvalue: 0,
    itemnames: 0 as *const *const libc::c_char as *mut *const libc::c_char,
};
static mut s_stack_double_box: menulist_s = menulist_s {
    generic: menucommon_s {
        type_0: 0,
        name: 0 as *const libc::c_char,
        x: 0,
        y: 0,
        parent: 0 as *const menuframework_s as *mut menuframework_s,
        cursor_offset: 0,
        localdata: [0; 4],
        flags: 0,
        statusbar: 0 as *const libc::c_char,
        callback: None,
        statusbarfunc: None,
        ownerdraw: None,
        cursordraw: None,
    },
    curvalue: 0,
    itemnames: 0 as *const *const libc::c_char as *mut *const libc::c_char,
};
static mut s_no_spheres_box: menulist_s = menulist_s {
    generic: menucommon_s {
        type_0: 0,
        name: 0 as *const libc::c_char,
        x: 0,
        y: 0,
        parent: 0 as *const menuframework_s as *mut menuframework_s,
        cursor_offset: 0,
        localdata: [0; 4],
        flags: 0,
        statusbar: 0 as *const libc::c_char,
        callback: None,
        statusbarfunc: None,
        ownerdraw: None,
        cursordraw: None,
    },
    curvalue: 0,
    itemnames: 0 as *const *const libc::c_char as *mut *const libc::c_char,
};

unsafe extern "C" fn DMFlagCallback(mut self_0: *mut libc::c_void) {
    let mut current_block: u64;
    let mut f: *mut menulist_s = self_0 as *mut menulist_s;
    let mut flags: libc::c_int = 0;
    let mut bit: libc::c_int = 0 as libc::c_int;
    flags = Cvar_VariableValue(
        b"dmflags\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as libc::c_int;
    if f == &mut s_friendlyfire_box as *mut menulist_s {
        if (*f).curvalue != 0 {
            flags &= !(0x100 as libc::c_int);
        } else {
            flags |= 0x100 as libc::c_int;
        }
    } else if f == &mut s_falls_box as *mut menulist_s {
        if (*f).curvalue != 0 {
            flags &= !(0x8 as libc::c_int);
        } else {
            flags |= 0x8 as libc::c_int;
        }
    } else {
        if f == &mut s_weapons_stay_box as *mut menulist_s {
            bit = 0x4 as libc::c_int;
            current_block = 12961834331865314435;
        } else if f == &mut s_instant_powerups_box as *mut menulist_s {
            bit = 0x10 as libc::c_int;
            current_block = 12961834331865314435;
        } else if f == &mut s_allow_exit_box as *mut menulist_s {
            bit = 0x1000 as libc::c_int;
            current_block = 12961834331865314435;
        } else if f == &mut s_powerups_box as *mut menulist_s {
            if (*f).curvalue != 0 {
                flags &= !(0x2 as libc::c_int);
            } else {
                flags |= 0x2 as libc::c_int;
            }
            current_block = 7654488161721462904;
        } else if f == &mut s_health_box as *mut menulist_s {
            if (*f).curvalue != 0 {
                flags &= !(0x1 as libc::c_int);
            } else {
                flags |= 0x1 as libc::c_int;
            }
            current_block = 7654488161721462904;
        } else if f == &mut s_spawn_farthest_box as *mut menulist_s {
            bit = 0x200 as libc::c_int;
            current_block = 12961834331865314435;
        } else if f == &mut s_teamplay_box as *mut menulist_s {
            if (*f).curvalue == 1 as libc::c_int {
                flags |= 0x40 as libc::c_int;
                flags &= !(0x80 as libc::c_int);
            } else if (*f).curvalue == 2 as libc::c_int {
                flags |= 0x80 as libc::c_int;
                flags &= !(0x40 as libc::c_int);
            } else {
                flags &= !(0x80 as libc::c_int | 0x40 as libc::c_int);
            }
            current_block = 7654488161721462904;
        } else if f == &mut s_samelevel_box as *mut menulist_s {
            bit = 0x20 as libc::c_int;
            current_block = 12961834331865314435;
        } else if f == &mut s_force_respawn_box as *mut menulist_s {
            bit = 0x400 as libc::c_int;
            current_block = 12961834331865314435;
        } else if f == &mut s_armor_box as *mut menulist_s {
            if (*f).curvalue != 0 {
                flags &= !(0x800 as libc::c_int);
            } else {
                flags |= 0x800 as libc::c_int;
            }
            current_block = 7654488161721462904;
        } else {
            if f == &mut s_infinite_ammo_box as *mut menulist_s {
                bit = 0x2000 as libc::c_int;
            } else if f == &mut s_fixed_fov_box as *mut menulist_s {
                bit = 0x8000 as libc::c_int;
            } else if f == &mut s_quad_drop_box as *mut menulist_s {
                bit = 0x4000 as libc::c_int;
            } else if Developer_searchpath(2 as libc::c_int) == 2 as libc::c_int {
                if f == &mut s_no_mines_box as *mut menulist_s {
                    bit = 0x20000 as libc::c_int;
                } else if f == &mut s_no_nukes_box as *mut menulist_s {
                    bit = 0x80000 as libc::c_int;
                } else if f == &mut s_stack_double_box as *mut menulist_s {
                    bit = 0x40000 as libc::c_int;
                } else if f == &mut s_no_spheres_box as *mut menulist_s {
                    bit = 0x100000 as libc::c_int;
                }
            }
            current_block = 12961834331865314435;
        }
        match current_block {
            7654488161721462904 => {}
            _ => {
                if !f.is_null() {
                    if (*f).curvalue == 0 as libc::c_int {
                        flags &= !bit;
                    } else {
                        flags |= bit;
                    }
                }
            }
        }
    }
    Cvar_SetValue(
        b"dmflags\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        flags as libc::c_float,
    );
    Com_sprintf(
        dmoptions_statusbar.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong as libc::c_int,
        b"dmflags = %d\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        flags,
    );
}

#[no_mangle]
pub unsafe extern "C" fn DMOptions_MenuInit() {
    static mut yes_no_names: [*const libc::c_char; 3] = [
        b"no\0" as *const u8 as *const libc::c_char,
        b"yes\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    static mut teamplay_names: [*const libc::c_char; 4] = [
        b"disabled\0" as *const u8 as *const libc::c_char,
        b"by skin\0" as *const u8 as *const libc::c_char,
        b"by model\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut dmflags: libc::c_int = Cvar_VariableValue(
        b"dmflags\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as libc::c_int;
    let mut y: libc::c_int = 0 as libc::c_int;
    s_dmoptions_menu.x = (viddef.width as libc::c_double * 0.50f64) as libc::c_int;
    s_dmoptions_menu.nitems = 0 as libc::c_int;
    s_falls_box.generic.type_0 = 3 as libc::c_int;
    s_falls_box.generic.x = 0 as libc::c_int;
    s_falls_box.generic.y = y;
    s_falls_box.generic.name = b"falling damage\0" as *const u8 as *const libc::c_char;
    s_falls_box
        .generic
        .callback = Some(
        DMFlagCallback as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    s_falls_box.itemnames = yes_no_names.as_mut_ptr();
    s_falls_box
        .curvalue = (dmflags & 0x8 as libc::c_int == 0 as libc::c_int) as libc::c_int;
    s_weapons_stay_box.generic.type_0 = 3 as libc::c_int;
    s_weapons_stay_box.generic.x = 0 as libc::c_int;
    y += 10 as libc::c_int;
    s_weapons_stay_box.generic.y = y;
    s_weapons_stay_box
        .generic
        .name = b"weapons stay\0" as *const u8 as *const libc::c_char;
    s_weapons_stay_box
        .generic
        .callback = Some(
        DMFlagCallback as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    s_weapons_stay_box.itemnames = yes_no_names.as_mut_ptr();
    s_weapons_stay_box
        .curvalue = (dmflags & 0x4 as libc::c_int != 0 as libc::c_int) as libc::c_int;
    s_instant_powerups_box.generic.type_0 = 3 as libc::c_int;
    s_instant_powerups_box.generic.x = 0 as libc::c_int;
    y += 10 as libc::c_int;
    s_instant_powerups_box.generic.y = y;
    s_instant_powerups_box
        .generic
        .name = b"instant powerups\0" as *const u8 as *const libc::c_char;
    s_instant_powerups_box
        .generic
        .callback = Some(
        DMFlagCallback as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    s_instant_powerups_box.itemnames = yes_no_names.as_mut_ptr();
    s_instant_powerups_box
        .curvalue = (dmflags & 0x10 as libc::c_int != 0 as libc::c_int) as libc::c_int;
    s_powerups_box.generic.type_0 = 3 as libc::c_int;
    s_powerups_box.generic.x = 0 as libc::c_int;
    y += 10 as libc::c_int;
    s_powerups_box.generic.y = y;
    s_powerups_box
        .generic
        .name = b"allow powerups\0" as *const u8 as *const libc::c_char;
    s_powerups_box
        .generic
        .callback = Some(
        DMFlagCallback as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    s_powerups_box.itemnames = yes_no_names.as_mut_ptr();
    s_powerups_box
        .curvalue = (dmflags & 0x2 as libc::c_int == 0 as libc::c_int) as libc::c_int;
    s_health_box.generic.type_0 = 3 as libc::c_int;
    s_health_box.generic.x = 0 as libc::c_int;
    y += 10 as libc::c_int;
    s_health_box.generic.y = y;
    s_health_box
        .generic
        .callback = Some(
        DMFlagCallback as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    s_health_box.generic.name = b"allow health\0" as *const u8 as *const libc::c_char;
    s_health_box.itemnames = yes_no_names.as_mut_ptr();
    s_health_box
        .curvalue = (dmflags & 0x1 as libc::c_int == 0 as libc::c_int) as libc::c_int;
    s_armor_box.generic.type_0 = 3 as libc::c_int;
    s_armor_box.generic.x = 0 as libc::c_int;
    y += 10 as libc::c_int;
    s_armor_box.generic.y = y;
    s_armor_box.generic.name = b"allow armor\0" as *const u8 as *const libc::c_char;
    s_armor_box
        .generic
        .callback = Some(
        DMFlagCallback as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    s_armor_box.itemnames = yes_no_names.as_mut_ptr();
    s_armor_box
        .curvalue = (dmflags & 0x800 as libc::c_int == 0 as libc::c_int) as libc::c_int;
    s_spawn_farthest_box.generic.type_0 = 3 as libc::c_int;
    s_spawn_farthest_box.generic.x = 0 as libc::c_int;
    y += 10 as libc::c_int;
    s_spawn_farthest_box.generic.y = y;
    s_spawn_farthest_box
        .generic
        .name = b"spawn farthest\0" as *const u8 as *const libc::c_char;
    s_spawn_farthest_box
        .generic
        .callback = Some(
        DMFlagCallback as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    s_spawn_farthest_box.itemnames = yes_no_names.as_mut_ptr();
    s_spawn_farthest_box
        .curvalue = (dmflags & 0x200 as libc::c_int != 0 as libc::c_int) as libc::c_int;
    s_samelevel_box.generic.type_0 = 3 as libc::c_int;
    s_samelevel_box.generic.x = 0 as libc::c_int;
    y += 10 as libc::c_int;
    s_samelevel_box.generic.y = y;
    s_samelevel_box.generic.name = b"same map\0" as *const u8 as *const libc::c_char;
    s_samelevel_box
        .generic
        .callback = Some(
        DMFlagCallback as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    s_samelevel_box.itemnames = yes_no_names.as_mut_ptr();
    s_samelevel_box
        .curvalue = (dmflags & 0x20 as libc::c_int != 0 as libc::c_int) as libc::c_int;
    s_force_respawn_box.generic.type_0 = 3 as libc::c_int;
    s_force_respawn_box.generic.x = 0 as libc::c_int;
    y += 10 as libc::c_int;
    s_force_respawn_box.generic.y = y;
    s_force_respawn_box
        .generic
        .name = b"force respawn\0" as *const u8 as *const libc::c_char;
    s_force_respawn_box
        .generic
        .callback = Some(
        DMFlagCallback as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    s_force_respawn_box.itemnames = yes_no_names.as_mut_ptr();
    s_force_respawn_box
        .curvalue = (dmflags & 0x400 as libc::c_int != 0 as libc::c_int) as libc::c_int;
    s_teamplay_box.generic.type_0 = 3 as libc::c_int;
    s_teamplay_box.generic.x = 0 as libc::c_int;
    y += 10 as libc::c_int;
    s_teamplay_box.generic.y = y;
    s_teamplay_box.generic.name = b"teamplay\0" as *const u8 as *const libc::c_char;
    s_teamplay_box
        .generic
        .callback = Some(
        DMFlagCallback as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    s_teamplay_box.itemnames = teamplay_names.as_mut_ptr();
    s_allow_exit_box.generic.type_0 = 3 as libc::c_int;
    s_allow_exit_box.generic.x = 0 as libc::c_int;
    y += 10 as libc::c_int;
    s_allow_exit_box.generic.y = y;
    s_allow_exit_box.generic.name = b"allow exit\0" as *const u8 as *const libc::c_char;
    s_allow_exit_box
        .generic
        .callback = Some(
        DMFlagCallback as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    s_allow_exit_box.itemnames = yes_no_names.as_mut_ptr();
    s_allow_exit_box
        .curvalue = (dmflags & 0x1000 as libc::c_int != 0 as libc::c_int) as libc::c_int;
    s_infinite_ammo_box.generic.type_0 = 3 as libc::c_int;
    s_infinite_ammo_box.generic.x = 0 as libc::c_int;
    y += 10 as libc::c_int;
    s_infinite_ammo_box.generic.y = y;
    s_infinite_ammo_box
        .generic
        .name = b"infinite ammo\0" as *const u8 as *const libc::c_char;
    s_infinite_ammo_box
        .generic
        .callback = Some(
        DMFlagCallback as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    s_infinite_ammo_box.itemnames = yes_no_names.as_mut_ptr();
    s_infinite_ammo_box
        .curvalue = (dmflags & 0x2000 as libc::c_int != 0 as libc::c_int) as libc::c_int;
    s_fixed_fov_box.generic.type_0 = 3 as libc::c_int;
    s_fixed_fov_box.generic.x = 0 as libc::c_int;
    y += 10 as libc::c_int;
    s_fixed_fov_box.generic.y = y;
    s_fixed_fov_box.generic.name = b"fixed FOV\0" as *const u8 as *const libc::c_char;
    s_fixed_fov_box
        .generic
        .callback = Some(
        DMFlagCallback as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    s_fixed_fov_box.itemnames = yes_no_names.as_mut_ptr();
    s_fixed_fov_box
        .curvalue = (dmflags & 0x8000 as libc::c_int != 0 as libc::c_int) as libc::c_int;
    s_quad_drop_box.generic.type_0 = 3 as libc::c_int;
    s_quad_drop_box.generic.x = 0 as libc::c_int;
    y += 10 as libc::c_int;
    s_quad_drop_box.generic.y = y;
    s_quad_drop_box.generic.name = b"quad drop\0" as *const u8 as *const libc::c_char;
    s_quad_drop_box
        .generic
        .callback = Some(
        DMFlagCallback as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    s_quad_drop_box.itemnames = yes_no_names.as_mut_ptr();
    s_quad_drop_box
        .curvalue = (dmflags & 0x4000 as libc::c_int != 0 as libc::c_int) as libc::c_int;
    s_friendlyfire_box.generic.type_0 = 3 as libc::c_int;
    s_friendlyfire_box.generic.x = 0 as libc::c_int;
    y += 10 as libc::c_int;
    s_friendlyfire_box.generic.y = y;
    s_friendlyfire_box
        .generic
        .name = b"friendly fire\0" as *const u8 as *const libc::c_char;
    s_friendlyfire_box
        .generic
        .callback = Some(
        DMFlagCallback as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    s_friendlyfire_box.itemnames = yes_no_names.as_mut_ptr();
    s_friendlyfire_box
        .curvalue = (dmflags & 0x100 as libc::c_int == 0 as libc::c_int) as libc::c_int;
    if Developer_searchpath(2 as libc::c_int) == 2 as libc::c_int {
        s_no_mines_box.generic.type_0 = 3 as libc::c_int;
        s_no_mines_box.generic.x = 0 as libc::c_int;
        y += 10 as libc::c_int;
        s_no_mines_box.generic.y = y;
        s_no_mines_box
            .generic
            .name = b"remove mines\0" as *const u8 as *const libc::c_char;
        s_no_mines_box
            .generic
            .callback = Some(
            DMFlagCallback as unsafe extern "C" fn(*mut libc::c_void) -> (),
        );
        s_no_mines_box.itemnames = yes_no_names.as_mut_ptr();
        s_no_mines_box
            .curvalue = (dmflags & 0x20000 as libc::c_int != 0 as libc::c_int)
            as libc::c_int;
        s_no_nukes_box.generic.type_0 = 3 as libc::c_int;
        s_no_nukes_box.generic.x = 0 as libc::c_int;
        y += 10 as libc::c_int;
        s_no_nukes_box.generic.y = y;
        s_no_nukes_box
            .generic
            .name = b"remove nukes\0" as *const u8 as *const libc::c_char;
        s_no_nukes_box
            .generic
            .callback = Some(
            DMFlagCallback as unsafe extern "C" fn(*mut libc::c_void) -> (),
        );
        s_no_nukes_box.itemnames = yes_no_names.as_mut_ptr();
        s_no_nukes_box
            .curvalue = (dmflags & 0x80000 as libc::c_int != 0 as libc::c_int)
            as libc::c_int;
        s_stack_double_box.generic.type_0 = 3 as libc::c_int;
        s_stack_double_box.generic.x = 0 as libc::c_int;
        y += 10 as libc::c_int;
        s_stack_double_box.generic.y = y;
        s_stack_double_box
            .generic
            .name = b"2x/4x stacking off\0" as *const u8 as *const libc::c_char;
        s_stack_double_box
            .generic
            .callback = Some(
            DMFlagCallback as unsafe extern "C" fn(*mut libc::c_void) -> (),
        );
        s_stack_double_box.itemnames = yes_no_names.as_mut_ptr();
        s_stack_double_box
            .curvalue = (dmflags & 0x40000 as libc::c_int != 0 as libc::c_int)
            as libc::c_int;
        s_no_spheres_box.generic.type_0 = 3 as libc::c_int;
        s_no_spheres_box.generic.x = 0 as libc::c_int;
        y += 10 as libc::c_int;
        s_no_spheres_box.generic.y = y;
        s_no_spheres_box
            .generic
            .name = b"remove spheres\0" as *const u8 as *const libc::c_char;
        s_no_spheres_box
            .generic
            .callback = Some(
            DMFlagCallback as unsafe extern "C" fn(*mut libc::c_void) -> (),
        );
        s_no_spheres_box.itemnames = yes_no_names.as_mut_ptr();
        s_no_spheres_box
            .curvalue = (dmflags & 0x100000 as libc::c_int != 0 as libc::c_int)
            as libc::c_int;
    }
    Menu_AddItem(
        &mut s_dmoptions_menu,
        &mut s_falls_box as *mut menulist_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_dmoptions_menu,
        &mut s_weapons_stay_box as *mut menulist_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_dmoptions_menu,
        &mut s_instant_powerups_box as *mut menulist_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_dmoptions_menu,
        &mut s_powerups_box as *mut menulist_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_dmoptions_menu,
        &mut s_health_box as *mut menulist_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_dmoptions_menu,
        &mut s_armor_box as *mut menulist_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_dmoptions_menu,
        &mut s_spawn_farthest_box as *mut menulist_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_dmoptions_menu,
        &mut s_samelevel_box as *mut menulist_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_dmoptions_menu,
        &mut s_force_respawn_box as *mut menulist_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_dmoptions_menu,
        &mut s_teamplay_box as *mut menulist_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_dmoptions_menu,
        &mut s_allow_exit_box as *mut menulist_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_dmoptions_menu,
        &mut s_infinite_ammo_box as *mut menulist_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_dmoptions_menu,
        &mut s_fixed_fov_box as *mut menulist_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_dmoptions_menu,
        &mut s_quad_drop_box as *mut menulist_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_dmoptions_menu,
        &mut s_friendlyfire_box as *mut menulist_s as *mut libc::c_void,
    );
    if Developer_searchpath(2 as libc::c_int) == 2 as libc::c_int {
        Menu_AddItem(
            &mut s_dmoptions_menu,
            &mut s_no_mines_box as *mut menulist_s as *mut libc::c_void,
        );
        Menu_AddItem(
            &mut s_dmoptions_menu,
            &mut s_no_nukes_box as *mut menulist_s as *mut libc::c_void,
        );
        Menu_AddItem(
            &mut s_dmoptions_menu,
            &mut s_stack_double_box as *mut menulist_s as *mut libc::c_void,
        );
        Menu_AddItem(
            &mut s_dmoptions_menu,
            &mut s_no_spheres_box as *mut menulist_s as *mut libc::c_void,
        );
    }
    Menu_Center(&mut s_dmoptions_menu);
    DMFlagCallback(0 as *mut libc::c_void);
    Menu_SetStatusBar(&mut s_dmoptions_menu, dmoptions_statusbar.as_mut_ptr());
}

#[no_mangle]
pub unsafe extern "C" fn DMOptions_MenuDraw() {
    Menu_Draw(&mut s_dmoptions_menu);
}

#[no_mangle]
pub unsafe extern "C" fn DMOptions_MenuKey(mut key: libc::c_int) -> *const libc::c_char {
    return Default_MenuKey(&mut s_dmoptions_menu, key);
}

#[no_mangle]
pub unsafe extern "C" fn M_Menu_DMOptions_f() {
    DMOptions_MenuInit();
    M_PushMenu(
        Some(DMOptions_MenuDraw as unsafe extern "C" fn() -> ()),
        Some(
            DMOptions_MenuKey as unsafe extern "C" fn(libc::c_int) -> *const libc::c_char,
        ),
    );
}

static mut s_downloadoptions_menu: menuframework_s = menuframework_s {
    x: 0,
    y: 0,
    cursor: 0,
    nitems: 0,
    nslots: 0,
    items: [0 as *const libc::c_void as *mut libc::c_void; 64],
    statusbar: 0 as *const libc::c_char,
    cursordraw: None,
};
static mut s_download_title: menuseparator_s = menuseparator_s {
    generic: menucommon_s {
        type_0: 0,
        name: 0 as *const libc::c_char,
        x: 0,
        y: 0,
        parent: 0 as *const menuframework_s as *mut menuframework_s,
        cursor_offset: 0,
        localdata: [0; 4],
        flags: 0,
        statusbar: 0 as *const libc::c_char,
        callback: None,
        statusbarfunc: None,
        ownerdraw: None,
        cursordraw: None,
    },
};
static mut s_allow_download_box: menulist_s = menulist_s {
    generic: menucommon_s {
        type_0: 0,
        name: 0 as *const libc::c_char,
        x: 0,
        y: 0,
        parent: 0 as *const menuframework_s as *mut menuframework_s,
        cursor_offset: 0,
        localdata: [0; 4],
        flags: 0,
        statusbar: 0 as *const libc::c_char,
        callback: None,
        statusbarfunc: None,
        ownerdraw: None,
        cursordraw: None,
    },
    curvalue: 0,
    itemnames: 0 as *const *const libc::c_char as *mut *const libc::c_char,
};
static mut s_allow_download_maps_box: menulist_s = menulist_s {
    generic: menucommon_s {
        type_0: 0,
        name: 0 as *const libc::c_char,
        x: 0,
        y: 0,
        parent: 0 as *const menuframework_s as *mut menuframework_s,
        cursor_offset: 0,
        localdata: [0; 4],
        flags: 0,
        statusbar: 0 as *const libc::c_char,
        callback: None,
        statusbarfunc: None,
        ownerdraw: None,
        cursordraw: None,
    },
    curvalue: 0,
    itemnames: 0 as *const *const libc::c_char as *mut *const libc::c_char,
};
static mut s_allow_download_models_box: menulist_s = menulist_s {
    generic: menucommon_s {
        type_0: 0,
        name: 0 as *const libc::c_char,
        x: 0,
        y: 0,
        parent: 0 as *const menuframework_s as *mut menuframework_s,
        cursor_offset: 0,
        localdata: [0; 4],
        flags: 0,
        statusbar: 0 as *const libc::c_char,
        callback: None,
        statusbarfunc: None,
        ownerdraw: None,
        cursordraw: None,
    },
    curvalue: 0,
    itemnames: 0 as *const *const libc::c_char as *mut *const libc::c_char,
};
static mut s_allow_download_players_box: menulist_s = menulist_s {
    generic: menucommon_s {
        type_0: 0,
        name: 0 as *const libc::c_char,
        x: 0,
        y: 0,
        parent: 0 as *const menuframework_s as *mut menuframework_s,
        cursor_offset: 0,
        localdata: [0; 4],
        flags: 0,
        statusbar: 0 as *const libc::c_char,
        callback: None,
        statusbarfunc: None,
        ownerdraw: None,
        cursordraw: None,
    },
    curvalue: 0,
    itemnames: 0 as *const *const libc::c_char as *mut *const libc::c_char,
};
static mut s_allow_download_sounds_box: menulist_s = menulist_s {
    generic: menucommon_s {
        type_0: 0,
        name: 0 as *const libc::c_char,
        x: 0,
        y: 0,
        parent: 0 as *const menuframework_s as *mut menuframework_s,
        cursor_offset: 0,
        localdata: [0; 4],
        flags: 0,
        statusbar: 0 as *const libc::c_char,
        callback: None,
        statusbarfunc: None,
        ownerdraw: None,
        cursordraw: None,
    },
    curvalue: 0,
    itemnames: 0 as *const *const libc::c_char as *mut *const libc::c_char,
};

unsafe extern "C" fn DownloadCallback(mut self_0: *mut libc::c_void) {
    let mut f: *mut menulist_s = self_0 as *mut menulist_s;
    if f == &mut s_allow_download_box as *mut menulist_s {
        Cvar_SetValue(
            b"allow_download\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            (*f).curvalue as libc::c_float,
        );
    } else if f == &mut s_allow_download_maps_box as *mut menulist_s {
        Cvar_SetValue(
            b"allow_download_maps\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            (*f).curvalue as libc::c_float,
        );
    } else if f == &mut s_allow_download_models_box as *mut menulist_s {
        Cvar_SetValue(
            b"allow_download_models\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            (*f).curvalue as libc::c_float,
        );
    } else if f == &mut s_allow_download_players_box as *mut menulist_s {
        Cvar_SetValue(
            b"allow_download_players\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            (*f).curvalue as libc::c_float,
        );
    } else if f == &mut s_allow_download_sounds_box as *mut menulist_s {
        Cvar_SetValue(
            b"allow_download_sounds\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            (*f).curvalue as libc::c_float,
        );
    }
}

#[no_mangle]
pub unsafe extern "C" fn DownloadOptions_MenuInit() {
    static mut yes_no_names: [*const libc::c_char; 3] = [
        b"no\0" as *const u8 as *const libc::c_char,
        b"yes\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut y: libc::c_int = 0 as libc::c_int;
    s_downloadoptions_menu.x = (viddef.width as libc::c_double * 0.50f64) as libc::c_int;
    s_downloadoptions_menu.nitems = 0 as libc::c_int;
    s_download_title.generic.type_0 = 4 as libc::c_int;
    s_download_title
        .generic
        .name = b"Download Options\0" as *const u8 as *const libc::c_char;
    s_download_title.generic.x = 48 as libc::c_int;
    s_download_title.generic.y = y;
    s_allow_download_box.generic.type_0 = 3 as libc::c_int;
    s_allow_download_box.generic.x = 0 as libc::c_int;
    y += 20 as libc::c_int;
    s_allow_download_box.generic.y = y;
    s_allow_download_box
        .generic
        .name = b"allow downloading\0" as *const u8 as *const libc::c_char;
    s_allow_download_box
        .generic
        .callback = Some(
        DownloadCallback as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    s_allow_download_box.itemnames = yes_no_names.as_mut_ptr();
    s_allow_download_box
        .curvalue = (Cvar_VariableValue(
        b"allow_download\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) != 0 as libc::c_int as libc::c_float) as libc::c_int;
    s_allow_download_maps_box.generic.type_0 = 3 as libc::c_int;
    s_allow_download_maps_box.generic.x = 0 as libc::c_int;
    y += 20 as libc::c_int;
    s_allow_download_maps_box.generic.y = y;
    s_allow_download_maps_box
        .generic
        .name = b"maps\0" as *const u8 as *const libc::c_char;
    s_allow_download_maps_box
        .generic
        .callback = Some(
        DownloadCallback as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    s_allow_download_maps_box.itemnames = yes_no_names.as_mut_ptr();
    s_allow_download_maps_box
        .curvalue = (Cvar_VariableValue(
        b"allow_download_maps\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) != 0 as libc::c_int as libc::c_float) as libc::c_int;
    s_allow_download_players_box.generic.type_0 = 3 as libc::c_int;
    s_allow_download_players_box.generic.x = 0 as libc::c_int;
    y += 10 as libc::c_int;
    s_allow_download_players_box.generic.y = y;
    s_allow_download_players_box
        .generic
        .name = b"player models/skins\0" as *const u8 as *const libc::c_char;
    s_allow_download_players_box
        .generic
        .callback = Some(
        DownloadCallback as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    s_allow_download_players_box.itemnames = yes_no_names.as_mut_ptr();
    s_allow_download_players_box
        .curvalue = (Cvar_VariableValue(
        b"allow_download_players\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    ) != 0 as libc::c_int as libc::c_float) as libc::c_int;
    s_allow_download_models_box.generic.type_0 = 3 as libc::c_int;
    s_allow_download_models_box.generic.x = 0 as libc::c_int;
    y += 10 as libc::c_int;
    s_allow_download_models_box.generic.y = y;
    s_allow_download_models_box
        .generic
        .name = b"models\0" as *const u8 as *const libc::c_char;
    s_allow_download_models_box
        .generic
        .callback = Some(
        DownloadCallback as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    s_allow_download_models_box.itemnames = yes_no_names.as_mut_ptr();
    s_allow_download_models_box
        .curvalue = (Cvar_VariableValue(
        b"allow_download_models\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    ) != 0 as libc::c_int as libc::c_float) as libc::c_int;
    s_allow_download_sounds_box.generic.type_0 = 3 as libc::c_int;
    s_allow_download_sounds_box.generic.x = 0 as libc::c_int;
    y += 10 as libc::c_int;
    s_allow_download_sounds_box.generic.y = y;
    s_allow_download_sounds_box
        .generic
        .name = b"sounds\0" as *const u8 as *const libc::c_char;
    s_allow_download_sounds_box
        .generic
        .callback = Some(
        DownloadCallback as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    s_allow_download_sounds_box.itemnames = yes_no_names.as_mut_ptr();
    s_allow_download_sounds_box
        .curvalue = (Cvar_VariableValue(
        b"allow_download_sounds\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    ) != 0 as libc::c_int as libc::c_float) as libc::c_int;
    Menu_AddItem(
        &mut s_downloadoptions_menu,
        &mut s_download_title as *mut menuseparator_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_downloadoptions_menu,
        &mut s_allow_download_box as *mut menulist_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_downloadoptions_menu,
        &mut s_allow_download_maps_box as *mut menulist_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_downloadoptions_menu,
        &mut s_allow_download_players_box as *mut menulist_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_downloadoptions_menu,
        &mut s_allow_download_models_box as *mut menulist_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_downloadoptions_menu,
        &mut s_allow_download_sounds_box as *mut menulist_s as *mut libc::c_void,
    );
    Menu_Center(&mut s_downloadoptions_menu);
    if s_downloadoptions_menu.cursor == 0 as libc::c_int {
        s_downloadoptions_menu.cursor = 1 as libc::c_int;
    }
}

#[no_mangle]
pub unsafe extern "C" fn DownloadOptions_MenuDraw() {
    Menu_Draw(&mut s_downloadoptions_menu);
}

#[no_mangle]
pub unsafe extern "C" fn DownloadOptions_MenuKey(
    mut key: libc::c_int,
) -> *const libc::c_char {
    return Default_MenuKey(&mut s_downloadoptions_menu, key);
}

#[no_mangle]
pub unsafe extern "C" fn M_Menu_DownloadOptions_f() {
    DownloadOptions_MenuInit();
    M_PushMenu(
        Some(DownloadOptions_MenuDraw as unsafe extern "C" fn() -> ()),
        Some(
            DownloadOptions_MenuKey
                as unsafe extern "C" fn(libc::c_int) -> *const libc::c_char,
        ),
    );
}

static mut s_addressbook_menu: menuframework_s = menuframework_s {
    x: 0,
    y: 0,
    cursor: 0,
    nitems: 0,
    nslots: 0,
    items: [0 as *const libc::c_void as *mut libc::c_void; 64],
    statusbar: 0 as *const libc::c_char,
    cursordraw: None,
};
static mut s_addressbook_fields: [menufield_s; 9] = [menufield_s {
    generic: menucommon_s {
        type_0: 0,
        name: 0 as *const libc::c_char,
        x: 0,
        y: 0,
        parent: 0 as *const menuframework_s as *mut menuframework_s,
        cursor_offset: 0,
        localdata: [0; 4],
        flags: 0,
        statusbar: 0 as *const libc::c_char,
        callback: None,
        statusbarfunc: None,
        ownerdraw: None,
        cursordraw: None,
    },
    buffer: [0; 80],
    cursor: 0,
    length: 0,
    visible_length: 0,
    visible_offset: 0,
}; 9];

#[no_mangle]
pub unsafe extern "C" fn AddressBook_MenuInit() {
    let mut i: libc::c_int = 0;
    s_addressbook_menu.x = viddef.width / 2 as libc::c_int - 142 as libc::c_int;
    s_addressbook_menu.y = viddef.height / 2 as libc::c_int - 58 as libc::c_int;
    s_addressbook_menu.nitems = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < 9 as libc::c_int {
        let mut adr: *mut cvar_t = 0 as *mut cvar_t;
        let mut buffer: [libc::c_char; 20] = [0; 20];
        Com_sprintf(
            buffer.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 20]>() as libc::c_ulong as libc::c_int,
            b"adr%d\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            i,
        );
        adr = Cvar_Get(
            buffer.as_mut_ptr(),
            b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            1 as libc::c_int,
        );
        s_addressbook_fields[i as usize].generic.type_0 = 5 as libc::c_int;
        s_addressbook_fields[i as usize].generic.name = 0 as *const libc::c_char;
        s_addressbook_fields[i as usize].generic.callback = None;
        s_addressbook_fields[i as usize].generic.x = 0 as libc::c_int;
        s_addressbook_fields[i as usize]
            .generic
            .y = i * 18 as libc::c_int + 0 as libc::c_int;
        s_addressbook_fields[i as usize]
            .generic
            .localdata[0 as libc::c_int as usize] = i;
        s_addressbook_fields[i as usize].cursor = 0 as libc::c_int;
        s_addressbook_fields[i as usize].length = 60 as libc::c_int;
        s_addressbook_fields[i as usize].visible_length = 30 as libc::c_int;
        strcpy((s_addressbook_fields[i as usize].buffer).as_mut_ptr(), (*adr).string);
        Menu_AddItem(
            &mut s_addressbook_menu,
            &mut *s_addressbook_fields.as_mut_ptr().offset(i as isize)
                as *mut menufield_s as *mut libc::c_void,
        );
        i += 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn AddressBook_MenuKey(
    mut key: libc::c_int,
) -> *const libc::c_char {
    if key == 27 as libc::c_int {
        let mut index: libc::c_int = 0;
        let mut buffer: [libc::c_char; 20] = [0; 20];
        index = 0 as libc::c_int;
        while index < 9 as libc::c_int {
            Com_sprintf(
                buffer.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 20]>() as libc::c_ulong
                    as libc::c_int,
                b"adr%d\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                index,
            );
            Cvar_Set(
                buffer.as_mut_ptr(),
                (s_addressbook_fields[index as usize].buffer).as_mut_ptr(),
            );
            index += 1;
        }
    }
    return Default_MenuKey(&mut s_addressbook_menu, key);
}

#[no_mangle]
pub unsafe extern "C" fn AddressBook_MenuDraw() {
    M_Banner(
        b"m_banner_addressbook\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    Menu_Draw(&mut s_addressbook_menu);
}

#[no_mangle]
pub unsafe extern "C" fn M_Menu_AddressBook_f() {
    AddressBook_MenuInit();
    M_PushMenu(
        Some(AddressBook_MenuDraw as unsafe extern "C" fn() -> ()),
        Some(
            AddressBook_MenuKey
                as unsafe extern "C" fn(libc::c_int) -> *const libc::c_char,
        ),
    );
}

static mut s_player_config_menu: menuframework_s = menuframework_s {
    x: 0,
    y: 0,
    cursor: 0,
    nitems: 0,
    nslots: 0,
    items: [0 as *const libc::c_void as *mut libc::c_void; 64],
    statusbar: 0 as *const libc::c_char,
    cursordraw: None,
};
static mut s_player_name_field: menufield_s = menufield_s {
    generic: menucommon_s {
        type_0: 0,
        name: 0 as *const libc::c_char,
        x: 0,
        y: 0,
        parent: 0 as *const menuframework_s as *mut menuframework_s,
        cursor_offset: 0,
        localdata: [0; 4],
        flags: 0,
        statusbar: 0 as *const libc::c_char,
        callback: None,
        statusbarfunc: None,
        ownerdraw: None,
        cursordraw: None,
    },
    buffer: [0; 80],
    cursor: 0,
    length: 0,
    visible_length: 0,
    visible_offset: 0,
};
static mut s_player_model_box: menulist_s = menulist_s {
    generic: menucommon_s {
        type_0: 0,
        name: 0 as *const libc::c_char,
        x: 0,
        y: 0,
        parent: 0 as *const menuframework_s as *mut menuframework_s,
        cursor_offset: 0,
        localdata: [0; 4],
        flags: 0,
        statusbar: 0 as *const libc::c_char,
        callback: None,
        statusbarfunc: None,
        ownerdraw: None,
        cursordraw: None,
    },
    curvalue: 0,
    itemnames: 0 as *const *const libc::c_char as *mut *const libc::c_char,
};
static mut s_player_skin_box: menulist_s = menulist_s {
    generic: menucommon_s {
        type_0: 0,
        name: 0 as *const libc::c_char,
        x: 0,
        y: 0,
        parent: 0 as *const menuframework_s as *mut menuframework_s,
        cursor_offset: 0,
        localdata: [0; 4],
        flags: 0,
        statusbar: 0 as *const libc::c_char,
        callback: None,
        statusbarfunc: None,
        ownerdraw: None,
        cursordraw: None,
    },
    curvalue: 0,
    itemnames: 0 as *const *const libc::c_char as *mut *const libc::c_char,
};
static mut s_player_handedness_box: menulist_s = menulist_s {
    generic: menucommon_s {
        type_0: 0,
        name: 0 as *const libc::c_char,
        x: 0,
        y: 0,
        parent: 0 as *const menuframework_s as *mut menuframework_s,
        cursor_offset: 0,
        localdata: [0; 4],
        flags: 0,
        statusbar: 0 as *const libc::c_char,
        callback: None,
        statusbarfunc: None,
        ownerdraw: None,
        cursordraw: None,
    },
    curvalue: 0,
    itemnames: 0 as *const *const libc::c_char as *mut *const libc::c_char,
};
static mut s_player_rate_box: menulist_s = menulist_s {
    generic: menucommon_s {
        type_0: 0,
        name: 0 as *const libc::c_char,
        x: 0,
        y: 0,
        parent: 0 as *const menuframework_s as *mut menuframework_s,
        cursor_offset: 0,
        localdata: [0; 4],
        flags: 0,
        statusbar: 0 as *const libc::c_char,
        callback: None,
        statusbarfunc: None,
        ownerdraw: None,
        cursordraw: None,
    },
    curvalue: 0,
    itemnames: 0 as *const *const libc::c_char as *mut *const libc::c_char,
};
static mut s_player_skin_title: menuseparator_s = menuseparator_s {
    generic: menucommon_s {
        type_0: 0,
        name: 0 as *const libc::c_char,
        x: 0,
        y: 0,
        parent: 0 as *const menuframework_s as *mut menuframework_s,
        cursor_offset: 0,
        localdata: [0; 4],
        flags: 0,
        statusbar: 0 as *const libc::c_char,
        callback: None,
        statusbarfunc: None,
        ownerdraw: None,
        cursordraw: None,
    },
};
static mut s_player_model_title: menuseparator_s = menuseparator_s {
    generic: menucommon_s {
        type_0: 0,
        name: 0 as *const libc::c_char,
        x: 0,
        y: 0,
        parent: 0 as *const menuframework_s as *mut menuframework_s,
        cursor_offset: 0,
        localdata: [0; 4],
        flags: 0,
        statusbar: 0 as *const libc::c_char,
        callback: None,
        statusbarfunc: None,
        ownerdraw: None,
        cursordraw: None,
    },
};
static mut s_player_hand_title: menuseparator_s = menuseparator_s {
    generic: menucommon_s {
        type_0: 0,
        name: 0 as *const libc::c_char,
        x: 0,
        y: 0,
        parent: 0 as *const menuframework_s as *mut menuframework_s,
        cursor_offset: 0,
        localdata: [0; 4],
        flags: 0,
        statusbar: 0 as *const libc::c_char,
        callback: None,
        statusbarfunc: None,
        ownerdraw: None,
        cursordraw: None,
    },
};
static mut s_player_rate_title: menuseparator_s = menuseparator_s {
    generic: menucommon_s {
        type_0: 0,
        name: 0 as *const libc::c_char,
        x: 0,
        y: 0,
        parent: 0 as *const menuframework_s as *mut menuframework_s,
        cursor_offset: 0,
        localdata: [0; 4],
        flags: 0,
        statusbar: 0 as *const libc::c_char,
        callback: None,
        statusbarfunc: None,
        ownerdraw: None,
        cursordraw: None,
    },
};
static mut s_player_download_action: menuaction_s = menuaction_s {
    generic: menucommon_s {
        type_0: 0,
        name: 0 as *const libc::c_char,
        x: 0,
        y: 0,
        parent: 0 as *const menuframework_s as *mut menuframework_s,
        cursor_offset: 0,
        localdata: [0; 4],
        flags: 0,
        statusbar: 0 as *const libc::c_char,
        callback: None,
        statusbarfunc: None,
        ownerdraw: None,
        cursordraw: None,
    },
};
static mut s_pmi: [playermodelinfo_s; 1024] = [playermodelinfo_s {
    nskins: 0,
    skindisplaynames: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
    displayname: [0; 16],
    directory: [0; 64],
}; 1024];
static mut s_pmnames: [*mut libc::c_char; 1024] = [0 as *const libc::c_char
    as *mut libc::c_char; 1024];
static mut s_numplayermodels: libc::c_int = 0;
static mut rate_tbl: [libc::c_int; 6] = [
    2500 as libc::c_int,
    3200 as libc::c_int,
    5000 as libc::c_int,
    10000 as libc::c_int,
    25000 as libc::c_int,
    0 as libc::c_int,
];
static mut rate_names: [*const libc::c_char; 7] = [
    b"28.8 Modem\0" as *const u8 as *const libc::c_char,
    b"33.6 Modem\0" as *const u8 as *const libc::c_char,
    b"Single ISDN\0" as *const u8 as *const libc::c_char,
    b"Dual ISDN/Cable\0" as *const u8 as *const libc::c_char,
    b"T1/LAN\0" as *const u8 as *const libc::c_char,
    b"User defined\0" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];

#[no_mangle]
pub unsafe extern "C" fn DownloadOptionsFunc(mut self_0: *mut libc::c_void) {
    M_Menu_DownloadOptions_f();
}

unsafe extern "C" fn HandednessCallback(mut unused: *mut libc::c_void) {
    Cvar_SetValue(
        b"hand\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        s_player_handedness_box.curvalue as libc::c_float,
    );
}

unsafe extern "C" fn RateCallback(mut unused: *mut libc::c_void) {
    if s_player_rate_box.curvalue as libc::c_ulong
        != (::std::mem::size_of::<[libc::c_int; 6]>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
    {
        Cvar_SetValue(
            b"rate\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            rate_tbl[s_player_rate_box.curvalue as usize] as libc::c_float,
        );
    }
}

unsafe extern "C" fn ModelCallback(mut unused: *mut libc::c_void) {
    s_player_skin_box
        .itemnames = s_pmi[s_player_model_box.curvalue as usize].skindisplaynames
        as *mut *const libc::c_char;
    s_player_skin_box.curvalue = 0 as libc::c_int;
}

unsafe extern "C" fn FreeFileList(mut list: *mut *mut libc::c_char, mut n: libc::c_int) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < n {
        if !(*list.offset(i as isize)).is_null() {
            free(*list.offset(i as isize) as *mut libc::c_void);
            let ref mut fresh8 = *list.offset(i as isize);
            *fresh8 = 0 as *mut libc::c_char;
        }
        i += 1;
    }
    free(list as *mut libc::c_void);
}

unsafe extern "C" fn IconOfSkinExists(
    mut skin: *mut libc::c_char,
    mut pcxfiles: *mut *mut libc::c_char,
    mut npcxfiles: libc::c_int,
) -> qboolean {
    let mut i: libc::c_int = 0;
    let mut scratch: [libc::c_char; 1024] = [0; 1024];
    strcpy(scratch.as_mut_ptr(), skin);
    *strrchr(scratch.as_mut_ptr(), '.' as i32) = 0 as libc::c_int as libc::c_char;
    strcat(scratch.as_mut_ptr(), b"_i.pcx\0" as *const u8 as *const libc::c_char);
    i = 0 as libc::c_int;
    while i < npcxfiles {
        if strcmp(*pcxfiles.offset(i as isize), scratch.as_mut_ptr()) == 0 as libc::c_int
        {
            return true_0;
        }
        i += 1;
    }
    return false_0;
}

unsafe extern "C" fn PlayerConfig_ScanDirectories() -> qboolean {
    let mut findname: [libc::c_char; 1024] = [0; 1024];
    let mut scratch: [libc::c_char; 1024] = [0; 1024];
    let mut ndirs: libc::c_int = 0 as libc::c_int;
    let mut npms: libc::c_int = 0 as libc::c_int;
    let mut dirnames: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut path: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    extern "C" {
        #[link_name = "FS_ListFiles"]
        fn FS_ListFiles_0(
            _: *mut libc::c_char,
            _: *mut libc::c_int,
            _: libc::c_uint,
            _: libc::c_uint,
        ) -> *mut *mut libc::c_char;
    }
    s_numplayermodels = 0 as libc::c_int;
    loop {
        path = FS_NextPath(path);
        Com_sprintf(
            findname.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong
                as libc::c_int,
            b"%s/players/*.*\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            path,
        );
        dirnames = FS_ListFiles_0(
            findname.as_mut_ptr(),
            &mut ndirs,
            0x8 as libc::c_int as libc::c_uint,
            0 as libc::c_int as libc::c_uint,
        );
        if !dirnames.is_null() {
            break;
        }
        if path.is_null() {
            break;
        }
    }
    if dirnames.is_null() {
        return false_0;
    }
    npms = ndirs;
    if npms > 1024 as libc::c_int {
        npms = 1024 as libc::c_int;
    }
    i = 0 as libc::c_int;
    while i < npms {
        let mut k: libc::c_int = 0;
        let mut s: libc::c_int = 0;
        let mut a: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut b: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut c: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut pcxnames: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
        let mut skinnames: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
        let mut npcxfiles: libc::c_int = 0;
        let mut nskins: libc::c_int = 0 as libc::c_int;
        if !(*dirnames.offset(i as isize)).is_null() {
            strcpy(scratch.as_mut_ptr(), *dirnames.offset(i as isize));
            strcat(
                scratch.as_mut_ptr(),
                b"/tris.md2\0" as *const u8 as *const libc::c_char,
            );
            if (Sys_FindFirst(
                scratch.as_mut_ptr(),
                0 as libc::c_int as libc::c_uint,
                (0x8 as libc::c_int | 0x2 as libc::c_int | 0x10 as libc::c_int)
                    as libc::c_uint,
            ))
                .is_null()
            {
                free(*dirnames.offset(i as isize) as *mut libc::c_void);
                let ref mut fresh9 = *dirnames.offset(i as isize);
                *fresh9 = 0 as *mut libc::c_char;
                Sys_FindClose();
            } else {
                Sys_FindClose();
                strcpy(scratch.as_mut_ptr(), *dirnames.offset(i as isize));
                strcat(
                    scratch.as_mut_ptr(),
                    b"/*.pcx\0" as *const u8 as *const libc::c_char,
                );
                pcxnames = FS_ListFiles_0(
                    scratch.as_mut_ptr(),
                    &mut npcxfiles,
                    0 as libc::c_int as libc::c_uint,
                    (0x8 as libc::c_int | 0x2 as libc::c_int | 0x10 as libc::c_int)
                        as libc::c_uint,
                );
                if pcxnames.is_null() {
                    free(*dirnames.offset(i as isize) as *mut libc::c_void);
                    let ref mut fresh10 = *dirnames.offset(i as isize);
                    *fresh10 = 0 as *mut libc::c_char;
                } else {
                    k = 0 as libc::c_int;
                    while k < npcxfiles - 1 as libc::c_int {
                        if (strstr(
                            *pcxnames.offset(k as isize),
                            b"_i.pcx\0" as *const u8 as *const libc::c_char,
                        ))
                            .is_null()
                        {
                            if IconOfSkinExists(
                                *pcxnames.offset(k as isize),
                                pcxnames,
                                npcxfiles - 1 as libc::c_int,
                            ) as u64 != 0
                            {
                                nskins += 1;
                            }
                        }
                        k += 1;
                    }
                    if !(nskins == 0) {
                        skinnames = malloc(
                            (::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
                                .wrapping_mul((nskins + 1 as libc::c_int) as libc::c_ulong),
                        ) as *mut *mut libc::c_char;
                        memset(
                            skinnames as *mut libc::c_void,
                            0 as libc::c_int,
                            (::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
                                .wrapping_mul((nskins + 1 as libc::c_int) as libc::c_ulong),
                        );
                        s = 0 as libc::c_int;
                        k = 0 as libc::c_int;
                        while k < npcxfiles - 1 as libc::c_int {
                            let mut a_0: *mut libc::c_char = 0 as *mut libc::c_char;
                            let mut b_0: *mut libc::c_char = 0 as *mut libc::c_char;
                            let mut c_0: *mut libc::c_char = 0 as *mut libc::c_char;
                            if (strstr(
                                *pcxnames.offset(k as isize),
                                b"_i.pcx\0" as *const u8 as *const libc::c_char,
                            ))
                                .is_null()
                            {
                                if IconOfSkinExists(
                                    *pcxnames.offset(k as isize),
                                    pcxnames,
                                    npcxfiles - 1 as libc::c_int,
                                ) as u64 != 0
                                {
                                    a_0 = strrchr(*pcxnames.offset(k as isize), '/' as i32);
                                    b_0 = strrchr(*pcxnames.offset(k as isize), '\\' as i32);
                                    if a_0 > b_0 {
                                        c_0 = a_0;
                                    } else {
                                        c_0 = b_0;
                                    }
                                    strcpy(
                                        scratch.as_mut_ptr(),
                                        c_0.offset(1 as libc::c_int as isize),
                                    );
                                    if !(strrchr(scratch.as_mut_ptr(), '.' as i32)).is_null() {
                                        *strrchr(
                                            scratch.as_mut_ptr(),
                                            '.' as i32,
                                        ) = 0 as libc::c_int as libc::c_char;
                                    }
                                    let ref mut fresh11 = *skinnames.offset(s as isize);
                                    *fresh11 = strdup(scratch.as_mut_ptr());
                                    s += 1;
                                }
                            }
                            k += 1;
                        }
                        s_pmi[s_numplayermodels as usize].nskins = nskins;
                        s_pmi[s_numplayermodels as usize].skindisplaynames = skinnames;
                        a = strrchr(*dirnames.offset(i as isize), '/' as i32);
                        b = strrchr(*dirnames.offset(i as isize), '\\' as i32);
                        if a > b {
                            c = a;
                        } else {
                            c = b;
                        }
                        strncpy(
                            (s_pmi[s_numplayermodels as usize].displayname).as_mut_ptr(),
                            c.offset(1 as libc::c_int as isize),
                            (16 as libc::c_int - 1 as libc::c_int) as libc::c_ulong,
                        );
                        strcpy(
                            (s_pmi[s_numplayermodels as usize].directory).as_mut_ptr(),
                            c.offset(1 as libc::c_int as isize),
                        );
                        FreeFileList(pcxnames, npcxfiles);
                        s_numplayermodels += 1;
                    }
                }
            }
        }
        i += 1;
    }
    if !dirnames.is_null() {
        FreeFileList(dirnames, ndirs);
    }
    panic!("Reached end of non-void function without returning");
}

unsafe extern "C" fn pmicmpfnc(
    mut _a: *const libc::c_void,
    mut _b: *const libc::c_void,
) -> libc::c_int {
    let mut a: *const playermodelinfo_s = _a as *const playermodelinfo_s;
    let mut b: *const playermodelinfo_s = _b as *const playermodelinfo_s;
    if strcmp(((*a).directory).as_ptr(), b"male\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        return -(1 as libc::c_int);
    } else {
        if strcmp(
            ((*b).directory).as_ptr(),
            b"male\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            return 1 as libc::c_int;
        }
    }
    if strcmp(((*a).directory).as_ptr(), b"female\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        return -(1 as libc::c_int);
    } else {
        if strcmp(
            ((*b).directory).as_ptr(),
            b"female\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            return 1 as libc::c_int;
        }
    }
    return strcmp(((*a).directory).as_ptr(), ((*b).directory).as_ptr());
}

#[no_mangle]
pub unsafe extern "C" fn PlayerConfig_MenuInit() -> qboolean {
    extern "C" {
        static mut name: *mut cvar_t;
    }
    extern "C" {
        static mut team: *mut cvar_t;
    }
    extern "C" {
        static mut skin: *mut cvar_t;
    }
    let mut currentdirectory: [libc::c_char; 1024] = [0; 1024];
    let mut currentskin: [libc::c_char; 1024] = [0; 1024];
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut currentdirectoryindex: libc::c_int = 0 as libc::c_int;
    let mut currentskinindex: libc::c_int = 0 as libc::c_int;
    let mut hand: *mut cvar_t = Cvar_Get(
        b"hand\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        2 as libc::c_int | 1 as libc::c_int,
    );
    static mut handedness: [*const libc::c_char; 4] = [
        b"right\0" as *const u8 as *const libc::c_char,
        b"left\0" as *const u8 as *const libc::c_char,
        b"center\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    PlayerConfig_ScanDirectories();
    if s_numplayermodels == 0 as libc::c_int {
        return false_0;
    }
    if (*hand).value < 0 as libc::c_int as libc::c_float
        || (*hand).value > 2 as libc::c_int as libc::c_float
    {
        Cvar_SetValue(
            b"hand\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            0 as libc::c_int as libc::c_float,
        );
    }
    strcpy(currentdirectory.as_mut_ptr(), (*skin).string);
    if !(strchr(currentdirectory.as_mut_ptr(), '/' as i32)).is_null() {
        strcpy(
            currentskin.as_mut_ptr(),
            (strchr(currentdirectory.as_mut_ptr(), '/' as i32))
                .offset(1 as libc::c_int as isize),
        );
        *strchr(
            currentdirectory.as_mut_ptr(),
            '/' as i32,
        ) = 0 as libc::c_int as libc::c_char;
    } else if !(strchr(currentdirectory.as_mut_ptr(), '\\' as i32)).is_null() {
        strcpy(
            currentskin.as_mut_ptr(),
            (strchr(currentdirectory.as_mut_ptr(), '\\' as i32))
                .offset(1 as libc::c_int as isize),
        );
        *strchr(
            currentdirectory.as_mut_ptr(),
            '\\' as i32,
        ) = 0 as libc::c_int as libc::c_char;
    } else {
        strcpy(
            currentdirectory.as_mut_ptr(),
            b"male\0" as *const u8 as *const libc::c_char,
        );
        strcpy(currentskin.as_mut_ptr(), b"grunt\0" as *const u8 as *const libc::c_char);
    }
    qsort(
        s_pmi.as_mut_ptr() as *mut libc::c_void,
        s_numplayermodels as size_t,
        ::std::mem::size_of::<playermodelinfo_s>() as libc::c_ulong,
        Some(
            pmicmpfnc
                as unsafe extern "C" fn(
                *const libc::c_void,
                *const libc::c_void,
            ) -> libc::c_int,
        ),
    );
    memset(
        s_pmnames.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<[*mut libc::c_char; 1024]>() as libc::c_ulong,
    );
    i = 0 as libc::c_int;
    while i < s_numplayermodels {
        s_pmnames[i as usize] = (s_pmi[i as usize].displayname).as_mut_ptr();
        if Q_stricmp(
            (s_pmi[i as usize].directory).as_mut_ptr(),
            currentdirectory.as_mut_ptr(),
        ) == 0 as libc::c_int
        {
            let mut j: libc::c_int = 0;
            currentdirectoryindex = i;
            j = 0 as libc::c_int;
            while j < s_pmi[i as usize].nskins {
                if Q_stricmp(
                    *(s_pmi[i as usize].skindisplaynames).offset(j as isize),
                    currentskin.as_mut_ptr(),
                ) == 0 as libc::c_int
                {
                    currentskinindex = j;
                    break;
                } else {
                    j += 1;
                }
            }
        }
        i += 1;
    }
    s_player_config_menu.x = viddef.width / 2 as libc::c_int - 95 as libc::c_int;
    s_player_config_menu.y = viddef.height / 2 as libc::c_int - 97 as libc::c_int;
    s_player_config_menu.nitems = 0 as libc::c_int;
    s_player_name_field.generic.type_0 = 5 as libc::c_int;
    s_player_name_field.generic.name = b"name\0" as *const u8 as *const libc::c_char;
    s_player_name_field.generic.callback = None;
    s_player_name_field.generic.x = 0 as libc::c_int;
    s_player_name_field.generic.y = 0 as libc::c_int;
    s_player_name_field.length = 20 as libc::c_int;
    s_player_name_field.visible_length = 20 as libc::c_int;
    strcpy((s_player_name_field.buffer).as_mut_ptr(), (*name).string);
    s_player_name_field.cursor = strlen((*name).string) as libc::c_int;
    s_player_model_title.generic.type_0 = 4 as libc::c_int;
    s_player_model_title.generic.name = b"model\0" as *const u8 as *const libc::c_char;
    s_player_model_title.generic.x = -(8 as libc::c_int);
    s_player_model_title.generic.y = 60 as libc::c_int;
    s_player_model_box.generic.type_0 = 3 as libc::c_int;
    s_player_model_box.generic.x = -(56 as libc::c_int);
    s_player_model_box.generic.y = 70 as libc::c_int;
    s_player_model_box
        .generic
        .callback = Some(ModelCallback as unsafe extern "C" fn(*mut libc::c_void) -> ());
    s_player_model_box.generic.cursor_offset = -(48 as libc::c_int);
    s_player_model_box.curvalue = currentdirectoryindex;
    s_player_model_box.itemnames = s_pmnames.as_mut_ptr() as *mut *const libc::c_char;
    s_player_skin_title.generic.type_0 = 4 as libc::c_int;
    s_player_skin_title.generic.name = b"skin\0" as *const u8 as *const libc::c_char;
    s_player_skin_title.generic.x = -(16 as libc::c_int);
    s_player_skin_title.generic.y = 84 as libc::c_int;
    s_player_skin_box.generic.type_0 = 3 as libc::c_int;
    s_player_skin_box.generic.x = -(56 as libc::c_int);
    s_player_skin_box.generic.y = 94 as libc::c_int;
    s_player_skin_box.generic.name = 0 as *const libc::c_char;
    s_player_skin_box.generic.callback = None;
    s_player_skin_box.generic.cursor_offset = -(48 as libc::c_int);
    s_player_skin_box.curvalue = currentskinindex;
    s_player_skin_box
        .itemnames = s_pmi[currentdirectoryindex as usize].skindisplaynames
        as *mut *const libc::c_char;
    s_player_hand_title.generic.type_0 = 4 as libc::c_int;
    s_player_hand_title
        .generic
        .name = b"handedness\0" as *const u8 as *const libc::c_char;
    s_player_hand_title.generic.x = 32 as libc::c_int;
    s_player_hand_title.generic.y = 108 as libc::c_int;
    s_player_handedness_box.generic.type_0 = 3 as libc::c_int;
    s_player_handedness_box.generic.x = -(56 as libc::c_int);
    s_player_handedness_box.generic.y = 118 as libc::c_int;
    s_player_handedness_box.generic.name = 0 as *const libc::c_char;
    s_player_handedness_box.generic.cursor_offset = -(48 as libc::c_int);
    s_player_handedness_box
        .generic
        .callback = Some(
        HandednessCallback as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    s_player_handedness_box
        .curvalue = Cvar_VariableValue(
        b"hand\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as libc::c_int;
    s_player_handedness_box.itemnames = handedness.as_mut_ptr();
    i = 0 as libc::c_int;
    while (i as libc::c_ulong)
        < (::std::mem::size_of::<[libc::c_int; 6]>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
    {
        if Cvar_VariableValue(
            b"rate\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) == rate_tbl[i as usize] as libc::c_float
        {
            break;
        }
        i += 1;
    }
    s_player_rate_title.generic.type_0 = 4 as libc::c_int;
    s_player_rate_title
        .generic
        .name = b"connect speed\0" as *const u8 as *const libc::c_char;
    s_player_rate_title.generic.x = 56 as libc::c_int;
    s_player_rate_title.generic.y = 156 as libc::c_int;
    s_player_rate_box.generic.type_0 = 3 as libc::c_int;
    s_player_rate_box.generic.x = -(56 as libc::c_int);
    s_player_rate_box.generic.y = 166 as libc::c_int;
    s_player_rate_box.generic.name = 0 as *const libc::c_char;
    s_player_rate_box.generic.cursor_offset = -(48 as libc::c_int);
    s_player_rate_box
        .generic
        .callback = Some(RateCallback as unsafe extern "C" fn(*mut libc::c_void) -> ());
    s_player_rate_box.curvalue = i;
    s_player_rate_box.itemnames = rate_names.as_mut_ptr();
    s_player_download_action.generic.type_0 = 2 as libc::c_int;
    s_player_download_action
        .generic
        .name = b"download options\0" as *const u8 as *const libc::c_char;
    s_player_download_action.generic.flags = 0x1 as libc::c_int as libc::c_uint;
    s_player_download_action.generic.x = -(24 as libc::c_int);
    s_player_download_action.generic.y = 186 as libc::c_int;
    s_player_download_action.generic.statusbar = 0 as *const libc::c_char;
    s_player_download_action
        .generic
        .callback = Some(
        DownloadOptionsFunc as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    Menu_AddItem(
        &mut s_player_config_menu,
        &mut s_player_name_field as *mut menufield_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_player_config_menu,
        &mut s_player_model_title as *mut menuseparator_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_player_config_menu,
        &mut s_player_model_box as *mut menulist_s as *mut libc::c_void,
    );
    if !(s_player_skin_box.itemnames).is_null() {
        Menu_AddItem(
            &mut s_player_config_menu,
            &mut s_player_skin_title as *mut menuseparator_s as *mut libc::c_void,
        );
        Menu_AddItem(
            &mut s_player_config_menu,
            &mut s_player_skin_box as *mut menulist_s as *mut libc::c_void,
        );
    }
    Menu_AddItem(
        &mut s_player_config_menu,
        &mut s_player_hand_title as *mut menuseparator_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_player_config_menu,
        &mut s_player_handedness_box as *mut menulist_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_player_config_menu,
        &mut s_player_rate_title as *mut menuseparator_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_player_config_menu,
        &mut s_player_rate_box as *mut menulist_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_player_config_menu,
        &mut s_player_download_action as *mut menuaction_s as *mut libc::c_void,
    );
    return true_0;
}

#[no_mangle]
pub unsafe extern "C" fn PlayerConfig_MenuDraw() {
    extern "C" {
        #[link_name = "CalcFov"]
        fn CalcFov_0(
            fov_x: libc::c_float,
            w: libc::c_float,
            h: libc::c_float,
        ) -> libc::c_float;
    }
    let mut refdef: refdef_t = refdef_t {
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
    };
    let mut scratch: [libc::c_char; 64] = [0; 64];
    memset(
        &mut refdef as *mut refdef_t as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<refdef_t>() as libc::c_ulong,
    );
    refdef.x = viddef.width / 2 as libc::c_int;
    refdef.y = viddef.height / 2 as libc::c_int - 72 as libc::c_int;
    refdef.width = 144 as libc::c_int;
    refdef.height = 168 as libc::c_int;
    refdef.fov_x = 40 as libc::c_int as libc::c_float;
    refdef
        .fov_y = CalcFov_0(
        refdef.fov_x,
        refdef.width as libc::c_float,
        refdef.height as libc::c_float,
    );
    refdef.time = (cls.realtime as libc::c_double * 0.001f64) as libc::c_float;
    if !(s_pmi[s_player_model_box.curvalue as usize].skindisplaynames).is_null() {
        static mut yaw: libc::c_int = 0;
        let mut maxframe: libc::c_int = 29 as libc::c_int;
        let mut entity: entity_t = entity_t {
            model: 0 as *mut model_s,
            angles: [0.; 3],
            origin: [0.; 3],
            frame: 0,
            oldorigin: [0.; 3],
            oldframe: 0,
            backlerp: 0.,
            skinnum: 0,
            lightstyle: 0,
            alpha: 0.,
            skin: 0 as *mut image_s,
            flags: 0,
        };
        memset(
            &mut entity as *mut entity_t as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<entity_t>() as libc::c_ulong,
        );
        Com_sprintf(
            scratch.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
            b"players/%s/tris.md2\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            (s_pmi[s_player_model_box.curvalue as usize].directory).as_mut_ptr(),
        );
        entity
            .model = (re.RegisterModel)
            .expect("non-null function pointer")(scratch.as_mut_ptr());
        Com_sprintf(
            scratch.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
            b"players/%s/%s.pcx\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            (s_pmi[s_player_model_box.curvalue as usize].directory).as_mut_ptr(),
            *(s_pmi[s_player_model_box.curvalue as usize].skindisplaynames)
                .offset(s_player_skin_box.curvalue as isize),
        );
        entity
            .skin = (re.RegisterSkin)
            .expect("non-null function pointer")(scratch.as_mut_ptr());
        entity.flags = 8 as libc::c_int;
        entity.origin[0 as libc::c_int as usize] = 80 as libc::c_int as libc::c_float;
        entity.origin[1 as libc::c_int as usize] = 0 as libc::c_int as libc::c_float;
        entity.origin[2 as libc::c_int as usize] = 0 as libc::c_int as libc::c_float;
        entity
            .oldorigin[0 as libc::c_int
            as usize] = entity.origin[0 as libc::c_int as usize];
        entity
            .oldorigin[1 as libc::c_int
            as usize] = entity.origin[1 as libc::c_int as usize];
        entity
            .oldorigin[2 as libc::c_int
            as usize] = entity.origin[2 as libc::c_int as usize];
        entity.frame = 0 as libc::c_int;
        entity.oldframe = 0 as libc::c_int;
        entity.backlerp = 0.0f64 as libc::c_float;
        let fresh12 = yaw;
        yaw = yaw + 1;
        entity.angles[1 as libc::c_int as usize] = fresh12 as libc::c_float;
        yaw += 1;
        if yaw > 360 as libc::c_int {
            yaw -= 360 as libc::c_int;
        }
        refdef.areabits = 0 as *mut byte;
        refdef.num_entities = 1 as libc::c_int;
        refdef.entities = &mut entity;
        refdef.lightstyles = 0 as *mut lightstyle_t;
        refdef.rdflags = 2 as libc::c_int;
        Menu_Draw(&mut s_player_config_menu);
        M_DrawTextBox(
            (refdef.x as libc::c_float * (320.0f32 / viddef.width as libc::c_float)
                - 8 as libc::c_int as libc::c_float) as libc::c_int,
            ((viddef.height / 2 as libc::c_int) as libc::c_float
                * (240.0f32 / viddef.height as libc::c_float)
                - 77 as libc::c_int as libc::c_float) as libc::c_int,
            refdef.width / 8 as libc::c_int,
            refdef.height / 8 as libc::c_int,
        );
        refdef.height += 4 as libc::c_int;
        (re.RenderFrame).expect("non-null function pointer")(&mut refdef);
        Com_sprintf(
            scratch.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
            b"/players/%s/%s_i.pcx\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            (s_pmi[s_player_model_box.curvalue as usize].directory).as_mut_ptr(),
            *(s_pmi[s_player_model_box.curvalue as usize].skindisplaynames)
                .offset(s_player_skin_box.curvalue as isize),
        );
        (re.DrawPic)
            .expect(
                "non-null function pointer",
            )(
            s_player_config_menu.x - 40 as libc::c_int,
            refdef.y,
            scratch.as_mut_ptr(),
        );
    }
}

#[no_mangle]
pub unsafe extern "C" fn PlayerConfig_MenuKey(
    mut key: libc::c_int,
) -> *const libc::c_char {
    let mut i: libc::c_int = 0;
    if key == 27 as libc::c_int {
        let mut scratch: [libc::c_char; 1024] = [0; 1024];
        Cvar_Set(
            b"name\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            (s_player_name_field.buffer).as_mut_ptr(),
        );
        Com_sprintf(
            scratch.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong
                as libc::c_int,
            b"%s/%s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            (s_pmi[s_player_model_box.curvalue as usize].directory).as_mut_ptr(),
            *(s_pmi[s_player_model_box.curvalue as usize].skindisplaynames)
                .offset(s_player_skin_box.curvalue as isize),
        );
        Cvar_Set(
            b"skin\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            scratch.as_mut_ptr(),
        );
        i = 0 as libc::c_int;
        while i < s_numplayermodels {
            let mut j: libc::c_int = 0;
            j = 0 as libc::c_int;
            while j < s_pmi[i as usize].nskins {
                if !(*(s_pmi[i as usize].skindisplaynames).offset(j as isize)).is_null()
                {
                    free(
                        *(s_pmi[i as usize].skindisplaynames).offset(j as isize)
                            as *mut libc::c_void,
                    );
                }
                let ref mut fresh13 = *(s_pmi[i as usize].skindisplaynames)
                    .offset(j as isize);
                *fresh13 = 0 as *mut libc::c_char;
                j += 1;
            }
            free(s_pmi[i as usize].skindisplaynames as *mut libc::c_void);
            s_pmi[i as usize].skindisplaynames = 0 as *mut *mut libc::c_char;
            s_pmi[i as usize].nskins = 0 as libc::c_int;
            i += 1;
        }
    }
    return Default_MenuKey(&mut s_player_config_menu, key);
}

#[no_mangle]
pub unsafe extern "C" fn M_Menu_PlayerConfig_f() {
    if PlayerConfig_MenuInit() as u64 == 0 {
        Menu_SetStatusBar(
            &mut s_multiplayer_menu,
            b"No valid player models found\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    Menu_SetStatusBar(&mut s_multiplayer_menu, 0 as *const libc::c_char);
    M_PushMenu(
        Some(PlayerConfig_MenuDraw as unsafe extern "C" fn() -> ()),
        Some(
            PlayerConfig_MenuKey
                as unsafe extern "C" fn(libc::c_int) -> *const libc::c_char,
        ),
    );
}

#[no_mangle]
pub unsafe extern "C" fn M_Quit_Key(mut key: libc::c_int) -> *const libc::c_char {
    match key {
        27 | 110 | 78 => {
            M_PopMenu();
        }
        89 | 121 => {
            cls.key_dest = key_console;
            CL_Quit_f();
        }
        _ => {}
    }
    return 0 as *const libc::c_char;
}

#[no_mangle]
pub unsafe extern "C" fn M_Quit_Draw() {
    let mut w: libc::c_int = 0;
    let mut h: libc::c_int = 0;
    (re.DrawGetPicSize)
        .expect(
            "non-null function pointer",
        )(
        &mut w,
        &mut h,
        b"quit\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    (re.DrawPic)
        .expect(
            "non-null function pointer",
        )(
        (viddef.width - w) / 2 as libc::c_int,
        (viddef.height - h) / 2 as libc::c_int,
        b"quit\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
}

#[no_mangle]
pub unsafe extern "C" fn M_Menu_Quit_f() {
    M_PushMenu(
        Some(M_Quit_Draw as unsafe extern "C" fn() -> ()),
        Some(M_Quit_Key as unsafe extern "C" fn(libc::c_int) -> *const libc::c_char),
    );
}

#[no_mangle]
pub unsafe extern "C" fn M_Init() {
    Cmd_AddCommand(
        b"menu_main\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        Some(M_Menu_Main_f as unsafe extern "C" fn() -> ()),
    );
    Cmd_AddCommand(
        b"menu_game\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        Some(M_Menu_Game_f as unsafe extern "C" fn() -> ()),
    );
    Cmd_AddCommand(
        b"menu_loadgame\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        Some(M_Menu_LoadGame_f as unsafe extern "C" fn() -> ()),
    );
    Cmd_AddCommand(
        b"menu_savegame\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        Some(M_Menu_SaveGame_f as unsafe extern "C" fn() -> ()),
    );
    Cmd_AddCommand(
        b"menu_joinserver\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        Some(M_Menu_JoinServer_f as unsafe extern "C" fn() -> ()),
    );
    Cmd_AddCommand(
        b"menu_addressbook\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        Some(M_Menu_AddressBook_f as unsafe extern "C" fn() -> ()),
    );
    Cmd_AddCommand(
        b"menu_startserver\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        Some(M_Menu_StartServer_f as unsafe extern "C" fn() -> ()),
    );
    Cmd_AddCommand(
        b"menu_dmoptions\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        Some(M_Menu_DMOptions_f as unsafe extern "C" fn() -> ()),
    );
    Cmd_AddCommand(
        b"menu_playerconfig\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        Some(M_Menu_PlayerConfig_f as unsafe extern "C" fn() -> ()),
    );
    Cmd_AddCommand(
        b"menu_downloadoptions\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        Some(M_Menu_DownloadOptions_f as unsafe extern "C" fn() -> ()),
    );
    Cmd_AddCommand(
        b"menu_credits\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        Some(M_Menu_Credits_f as unsafe extern "C" fn() -> ()),
    );
    Cmd_AddCommand(
        b"menu_multiplayer\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        Some(M_Menu_Multiplayer_f as unsafe extern "C" fn() -> ()),
    );
    Cmd_AddCommand(
        b"menu_video\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        Some(M_Menu_Video_f as unsafe extern "C" fn() -> ()),
    );
    Cmd_AddCommand(
        b"menu_options\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        Some(M_Menu_Options_f as unsafe extern "C" fn() -> ()),
    );
    Cmd_AddCommand(
        b"menu_keys\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        Some(M_Menu_Keys_f as unsafe extern "C" fn() -> ()),
    );
    Cmd_AddCommand(
        b"menu_quit\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        Some(M_Menu_Quit_f as unsafe extern "C" fn() -> ()),
    );
}

#[no_mangle]
pub unsafe extern "C" fn M_Draw() {
    if cls.key_dest as libc::c_uint != key_menu as libc::c_int as libc::c_uint {
        return;
    }
    SCR_DirtyScreen();
    if cl.cinematictime > 0 as libc::c_int {
        (re.DrawFill)
            .expect(
                "non-null function pointer",
            )(
            0 as libc::c_int,
            0 as libc::c_int,
            viddef.width,
            viddef.height,
            0 as libc::c_int,
        );
    } else {
        (re.DrawFadeScreen).expect("non-null function pointer")();
    }
    m_drawfunc.expect("non-null function pointer")();
    if m_entersound as u64 != 0 {
        S_StartLocalSound(menu_in_sound);
        m_entersound = false_0;
    }
}

#[no_mangle]
pub unsafe extern "C" fn M_Keydown(mut key: libc::c_int) {
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    if m_keyfunc.is_some() {
        s = m_keyfunc.expect("non-null function pointer")(key);
        if !s.is_null() {
            S_StartLocalSound(s as *mut libc::c_char);
        }
    }
}
