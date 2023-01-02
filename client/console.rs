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
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn Com_sprintf(
        dest: *mut libc::c_char,
        size: libc::c_int,
        fmt: *mut libc::c_char,
        _: ...
    );
    fn Com_Printf(msg: *mut libc::c_char, _: ...);
    fn Cbuf_AddText(text: *mut libc::c_char);
    fn Cmd_AddCommand(cmd_name: *mut libc::c_char, function: xcommand_t);
    fn Cmd_Argc() -> libc::c_int;
    fn Cmd_Argv(arg: libc::c_int) -> *mut libc::c_char;
    fn Cvar_Get(
        var_name: *mut libc::c_char,
        value: *mut libc::c_char,
        flags: libc::c_int,
    ) -> *mut cvar_t;
    fn Cvar_Set(var_name: *mut libc::c_char, value: *mut libc::c_char) -> *mut cvar_t;
    fn Cvar_VariableValue(var_name: *mut libc::c_char) -> libc::c_float;
    fn FS_Gamedir() -> *mut libc::c_char;
    fn FS_CreatePath(path: *mut libc::c_char);
    fn Com_ServerState() -> libc::c_int;
    static mut cls: client_static_t;
    static mut viddef: viddef_t;
    fn SCR_EndLoadingPlaque();
    fn SCR_AddDirtyPoint(x: libc::c_int, y: libc::c_int);
    static mut chat_buffer: [libc::c_char; 0];
    static mut chat_bufferlen: libc::c_int;
    static mut chat_team: qboolean;
    fn M_ForceMenuOff();
    static mut cl: client_state_t;
    static mut re: refexport_t;
    static mut key_lines: [[libc::c_char; 256]; 32];
    static mut edit_line: libc::c_int;
    static mut key_linepos: libc::c_int;
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
pub struct console_t {
    pub initialized: qboolean,
    pub text: [libc::c_char; 32768],
    pub current: libc::c_int,
    pub x: libc::c_int,
    pub display: libc::c_int,
    pub ormask: libc::c_int,
    pub linewidth: libc::c_int,
    pub totallines: libc::c_int,
    pub cursorspeed: libc::c_float,
    pub vislines: libc::c_int,
    pub times: [libc::c_float; 4],
}

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

pub type connstate_t = libc::c_uint;

pub const ca_active: connstate_t = 4;
pub const ca_connected: connstate_t = 3;
pub const ca_connecting: connstate_t = 2;
pub const ca_disconnected: connstate_t = 1;
pub const ca_uninitialized: connstate_t = 0;

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

#[no_mangle]
pub static mut con: console_t = console_t {
    initialized: false_0,
    text: [0; 32768],
    current: 0,
    x: 0,
    display: 0,
    ormask: 0,
    linewidth: 0,
    totallines: 0,
    cursorspeed: 0.,
    vislines: 0,
    times: [0.; 4],
};
#[no_mangle]
pub static mut con_notifytime: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;

#[no_mangle]
pub unsafe extern "C" fn DrawString(
    mut x: libc::c_int,
    mut y: libc::c_int,
    mut s: *mut libc::c_char,
) {
    while *s != 0 {
        (re.DrawChar).expect("non-null function pointer")(x, y, *s as libc::c_int);
        x += 8 as libc::c_int;
        s = s.offset(1);
    }
}

#[no_mangle]
pub unsafe extern "C" fn DrawAltString(
    mut x: libc::c_int,
    mut y: libc::c_int,
    mut s: *mut libc::c_char,
) {
    while *s != 0 {
        (re.DrawChar)
            .expect(
                "non-null function pointer",
            )(x, y, *s as libc::c_int ^ 0x80 as libc::c_int);
        x += 8 as libc::c_int;
        s = s.offset(1);
    }
}

#[no_mangle]
pub unsafe extern "C" fn Key_ClearTyping() {
    key_lines[edit_line
        as usize][1 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    key_linepos = 1 as libc::c_int;
}

#[no_mangle]
pub unsafe extern "C" fn Con_ToggleConsole_f() {
    SCR_EndLoadingPlaque();
    if cl.attractloop as u64 != 0 {
        Cbuf_AddText(
            b"killserver\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        return;
    }
    if cls.state as libc::c_uint == ca_disconnected as libc::c_int as libc::c_uint {
        Cbuf_AddText(b"d1\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
        return;
    }
    Key_ClearTyping();
    Con_ClearNotify();
    if cls.key_dest as libc::c_uint == key_console as libc::c_int as libc::c_uint {
        M_ForceMenuOff();
        Cvar_Set(
            b"paused\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    } else {
        M_ForceMenuOff();
        cls.key_dest = key_console;
        if Cvar_VariableValue(
            b"maxclients\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) == 1 as libc::c_int as libc::c_float && Com_ServerState() != 0
        {
            Cvar_Set(
                b"paused\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                b"1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
        }
    };
}

#[no_mangle]
pub unsafe extern "C" fn Con_ToggleChat_f() {
    Key_ClearTyping();
    if cls.key_dest as libc::c_uint == key_console as libc::c_int as libc::c_uint {
        if cls.state as libc::c_uint == ca_active as libc::c_int as libc::c_uint {
            M_ForceMenuOff();
            cls.key_dest = key_game;
        }
    } else {
        cls.key_dest = key_console;
    }
    Con_ClearNotify();
}

#[no_mangle]
pub unsafe extern "C" fn Con_Clear_f() {
    memset(
        (con.text).as_mut_ptr() as *mut libc::c_void,
        ' ' as i32,
        32768 as libc::c_int as libc::c_ulong,
    );
}

#[no_mangle]
pub unsafe extern "C" fn Con_Dump_f() {
    let mut l: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut line: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut f: *mut FILE = 0 as *mut FILE;
    let mut buffer: [libc::c_char; 1024] = [0; 1024];
    let mut name: [libc::c_char; 128] = [0; 128];
    if Cmd_Argc() != 2 as libc::c_int {
        Com_Printf(
            b"usage: condump <filename>\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return;
    }
    Com_sprintf(
        name.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong as libc::c_int,
        b"%s/%s.txt\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        FS_Gamedir(),
        Cmd_Argv(1 as libc::c_int),
    );
    Com_Printf(
        b"Dumped console text to %s.\n\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        name.as_mut_ptr(),
    );
    FS_CreatePath(name.as_mut_ptr());
    f = fopen(name.as_mut_ptr(), b"w\0" as *const u8 as *const libc::c_char);
    if f.is_null() {
        Com_Printf(
            b"ERROR: couldn't open.\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return;
    }
    l = con.current - con.totallines + 1 as libc::c_int;
    while l <= con.current {
        line = (con.text)
            .as_mut_ptr()
            .offset((l % con.totallines * con.linewidth) as isize);
        x = 0 as libc::c_int;
        while x < con.linewidth {
            if *line.offset(x as isize) as libc::c_int != ' ' as i32 {
                break;
            }
            x += 1;
        }
        if x != con.linewidth {
            break;
        }
        l += 1;
    }
    buffer[con.linewidth as usize] = 0 as libc::c_int as libc::c_char;
    while l <= con.current {
        line = (con.text)
            .as_mut_ptr()
            .offset((l % con.totallines * con.linewidth) as isize);
        strncpy(buffer.as_mut_ptr(), line, con.linewidth as libc::c_ulong);
        x = con.linewidth - 1 as libc::c_int;
        while x >= 0 as libc::c_int {
            if !(buffer[x as usize] as libc::c_int == ' ' as i32) {
                break;
            }
            buffer[x as usize] = 0 as libc::c_int as libc::c_char;
            x -= 1;
        }
        x = 0 as libc::c_int;
        while buffer[x as usize] != 0 {
            buffer[x
                as usize] = (buffer[x as usize] as libc::c_int & 0x7f as libc::c_int)
                as libc::c_char;
            x += 1;
        }
        fprintf(f, b"%s\n\0" as *const u8 as *const libc::c_char, buffer.as_mut_ptr());
        l += 1;
    }
    fclose(f);
}

#[no_mangle]
pub unsafe extern "C" fn Con_ClearNotify() {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        con.times[i as usize] = 0 as libc::c_int as libc::c_float;
        i += 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn Con_MessageMode_f() {
    chat_team = false_0;
    cls.key_dest = key_message;
}

#[no_mangle]
pub unsafe extern "C" fn Con_MessageMode2_f() {
    chat_team = true_0;
    cls.key_dest = key_message;
}

#[no_mangle]
pub unsafe extern "C" fn Con_CheckResize() {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut width: libc::c_int = 0;
    let mut oldwidth: libc::c_int = 0;
    let mut oldtotallines: libc::c_int = 0;
    let mut numlines: libc::c_int = 0;
    let mut numchars: libc::c_int = 0;
    let mut tbuf: [libc::c_char; 32768] = [0; 32768];
    width = (viddef.width >> 3 as libc::c_int) - 2 as libc::c_int;
    if width == con.linewidth {
        return;
    }
    if width < 1 as libc::c_int {
        width = 38 as libc::c_int;
        con.linewidth = width;
        con.totallines = 32768 as libc::c_int / con.linewidth;
        memset(
            (con.text).as_mut_ptr() as *mut libc::c_void,
            ' ' as i32,
            32768 as libc::c_int as libc::c_ulong,
        );
    } else {
        oldwidth = con.linewidth;
        con.linewidth = width;
        oldtotallines = con.totallines;
        con.totallines = 32768 as libc::c_int / con.linewidth;
        numlines = oldtotallines;
        if con.totallines < numlines {
            numlines = con.totallines;
        }
        numchars = oldwidth;
        if con.linewidth < numchars {
            numchars = con.linewidth;
        }
        memcpy(
            tbuf.as_mut_ptr() as *mut libc::c_void,
            (con.text).as_mut_ptr() as *const libc::c_void,
            32768 as libc::c_int as libc::c_ulong,
        );
        memset(
            (con.text).as_mut_ptr() as *mut libc::c_void,
            ' ' as i32,
            32768 as libc::c_int as libc::c_ulong,
        );
        i = 0 as libc::c_int;
        while i < numlines {
            j = 0 as libc::c_int;
            while j < numchars {
                con
                    .text[((con.totallines - 1 as libc::c_int - i) * con.linewidth + j)
                    as usize] = tbuf[((con.current - i + oldtotallines) % oldtotallines
                    * oldwidth + j) as usize];
                j += 1;
            }
            i += 1;
        }
        Con_ClearNotify();
    }
    con.current = con.totallines - 1 as libc::c_int;
    con.display = con.current;
}

#[no_mangle]
pub unsafe extern "C" fn Con_Init() {
    con.linewidth = -(1 as libc::c_int);
    Con_CheckResize();
    Com_Printf(
        b"Console initialized.\n\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    con_notifytime = Cvar_Get(
        b"con_notifytime\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"3\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    Cmd_AddCommand(
        b"toggleconsole\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        Some(Con_ToggleConsole_f as unsafe extern "C" fn() -> ()),
    );
    Cmd_AddCommand(
        b"togglechat\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        Some(Con_ToggleChat_f as unsafe extern "C" fn() -> ()),
    );
    Cmd_AddCommand(
        b"messagemode\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        Some(Con_MessageMode_f as unsafe extern "C" fn() -> ()),
    );
    Cmd_AddCommand(
        b"messagemode2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        Some(Con_MessageMode2_f as unsafe extern "C" fn() -> ()),
    );
    Cmd_AddCommand(
        b"clear\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        Some(Con_Clear_f as unsafe extern "C" fn() -> ()),
    );
    Cmd_AddCommand(
        b"condump\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        Some(Con_Dump_f as unsafe extern "C" fn() -> ()),
    );
    con.initialized = true_0;
}

#[no_mangle]
pub unsafe extern "C" fn Con_Linefeed() {
    con.x = 0 as libc::c_int;
    if con.display == con.current {
        con.display += 1;
    }
    con.current += 1;
    memset(
        &mut *(con.text)
            .as_mut_ptr()
            .offset((con.current % con.totallines * con.linewidth) as isize)
            as *mut libc::c_char as *mut libc::c_void,
        ' ' as i32,
        con.linewidth as libc::c_ulong,
    );
}

#[no_mangle]
pub unsafe extern "C" fn Con_Print(mut txt: *mut libc::c_char) {
    let mut y: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    static mut cr: libc::c_int = 0;
    let mut mask: libc::c_int = 0;
    if con.initialized as u64 == 0 {
        return;
    }
    if *txt.offset(0 as libc::c_int as isize) as libc::c_int == 1 as libc::c_int
        || *txt.offset(0 as libc::c_int as isize) as libc::c_int == 2 as libc::c_int
    {
        mask = 128 as libc::c_int;
        txt = txt.offset(1);
    } else {
        mask = 0 as libc::c_int;
    }
    loop {
        c = *txt as libc::c_int;
        if !(c != 0) {
            break;
        }
        l = 0 as libc::c_int;
        while l < con.linewidth {
            if *txt.offset(l as isize) as libc::c_int <= ' ' as i32 {
                break;
            }
            l += 1;
        }
        if l != con.linewidth && con.x + l > con.linewidth {
            con.x = 0 as libc::c_int;
        }
        txt = txt.offset(1);
        if cr != 0 {
            con.current -= 1;
            cr = false_0 as libc::c_int;
        }
        if con.x == 0 {
            Con_Linefeed();
            if con.current >= 0 as libc::c_int {
                con
                    .times[(con.current % 4 as libc::c_int)
                    as usize] = cls.realtime as libc::c_float;
            }
        }
        match c {
            10 => {
                con.x = 0 as libc::c_int;
            }
            13 => {
                con.x = 0 as libc::c_int;
                cr = 1 as libc::c_int;
            }
            _ => {
                y = con.current % con.totallines;
                con
                    .text[(y * con.linewidth + con.x)
                    as usize] = (c | mask | con.ormask) as libc::c_char;
                con.x += 1;
                if con.x >= con.linewidth {
                    con.x = 0 as libc::c_int;
                }
            }
        }
    };
}

#[no_mangle]
pub unsafe extern "C" fn Con_CenteredPrint(mut text: *mut libc::c_char) {
    let mut l: libc::c_int = 0;
    let mut buffer: [libc::c_char; 1024] = [0; 1024];
    l = strlen(text) as libc::c_int;
    l = (con.linewidth - l) / 2 as libc::c_int;
    if l < 0 as libc::c_int {
        l = 0 as libc::c_int;
    }
    memset(buffer.as_mut_ptr() as *mut libc::c_void, ' ' as i32, l as libc::c_ulong);
    strcpy(buffer.as_mut_ptr().offset(l as isize), text);
    strcat(buffer.as_mut_ptr(), b"\n\0" as *const u8 as *const libc::c_char);
    Con_Print(buffer.as_mut_ptr());
}

#[no_mangle]
pub unsafe extern "C" fn Con_DrawInput() {
    let mut y: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut text: *mut libc::c_char = 0 as *mut libc::c_char;
    if cls.key_dest as libc::c_uint == key_menu as libc::c_int as libc::c_uint {
        return;
    }
    if cls.key_dest as libc::c_uint != key_console as libc::c_int as libc::c_uint
        && cls.state as libc::c_uint == ca_active as libc::c_int as libc::c_uint
    {
        return;
    }
    text = (key_lines[edit_line as usize]).as_mut_ptr();
    *text
        .offset(
            key_linepos as isize,
        ) = (10 as libc::c_int + (cls.realtime >> 8 as libc::c_int & 1 as libc::c_int))
        as libc::c_char;
    i = key_linepos + 1 as libc::c_int;
    while i < con.linewidth {
        *text.offset(i as isize) = ' ' as i32 as libc::c_char;
        i += 1;
    }
    if key_linepos >= con.linewidth {
        text = text.offset((1 as libc::c_int + key_linepos - con.linewidth) as isize);
    }
    y = con.vislines - 16 as libc::c_int;
    i = 0 as libc::c_int;
    while i < con.linewidth {
        (re.DrawChar)
            .expect(
                "non-null function pointer",
            )(
            (i + 1 as libc::c_int) << 3 as libc::c_int,
            con.vislines - 22 as libc::c_int,
            *text.offset(i as isize) as libc::c_int,
        );
        i += 1;
    }
    key_lines[edit_line
        as usize][key_linepos as usize] = 0 as libc::c_int as libc::c_char;
}

#[no_mangle]
pub unsafe extern "C" fn Con_DrawNotify() {
    let mut x: libc::c_int = 0;
    let mut v: libc::c_int = 0;
    let mut text: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    let mut time: libc::c_int = 0;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut skip: libc::c_int = 0;
    v = 0 as libc::c_int;
    i = con.current - 4 as libc::c_int + 1 as libc::c_int;
    while i <= con.current {
        if !(i < 0 as libc::c_int) {
            time = con.times[(i % 4 as libc::c_int) as usize] as libc::c_int;
            if !(time == 0 as libc::c_int) {
                time = cls.realtime - time;
                if !(time as libc::c_float
                    > (*con_notifytime).value * 1000 as libc::c_int as libc::c_float)
                {
                    text = (con.text)
                        .as_mut_ptr()
                        .offset((i % con.totallines * con.linewidth) as isize);
                    x = 0 as libc::c_int;
                    while x < con.linewidth {
                        (re.DrawChar)
                            .expect(
                                "non-null function pointer",
                            )(
                            (x + 1 as libc::c_int) << 3 as libc::c_int,
                            v,
                            *text.offset(x as isize) as libc::c_int,
                        );
                        x += 1;
                    }
                    v += 8 as libc::c_int;
                }
            }
        }
        i += 1;
    }
    if cls.key_dest as libc::c_uint == key_message as libc::c_int as libc::c_uint {
        if chat_team as u64 != 0 {
            DrawString(
                8 as libc::c_int,
                v,
                b"say_team:\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            skip = 11 as libc::c_int;
        } else {
            DrawString(
                8 as libc::c_int,
                v,
                b"say:\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            skip = 5 as libc::c_int;
        }
        s = chat_buffer.as_mut_ptr();
        if chat_bufferlen
            > (viddef.width >> 3 as libc::c_int) - (skip + 1 as libc::c_int)
        {
            s = s
                .offset(
                    (chat_bufferlen
                        - ((viddef.width >> 3 as libc::c_int)
                        - (skip + 1 as libc::c_int))) as isize,
                );
        }
        x = 0 as libc::c_int;
        while *s.offset(x as isize) != 0 {
            (re.DrawChar)
                .expect(
                    "non-null function pointer",
                )(x + skip << 3 as libc::c_int, v, *s.offset(x as isize) as libc::c_int);
            x += 1;
        }
        (re.DrawChar)
            .expect(
                "non-null function pointer",
            )(
            x + skip << 3 as libc::c_int,
            v,
            10 as libc::c_int + (cls.realtime >> 8 as libc::c_int & 1 as libc::c_int),
        );
        v += 8 as libc::c_int;
    }
    if v != 0 {
        SCR_AddDirtyPoint(0 as libc::c_int, 0 as libc::c_int);
        SCR_AddDirtyPoint(viddef.width - 1 as libc::c_int, v);
    }
}

#[no_mangle]
pub unsafe extern "C" fn Con_DrawConsole(mut frac: libc::c_float) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut rows: libc::c_int = 0;
    let mut text: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut row: libc::c_int = 0;
    let mut lines: libc::c_int = 0;
    let mut version: [libc::c_char; 64] = [0; 64];
    let mut dlbar: [libc::c_char; 1024] = [0; 1024];
    lines = (viddef.height as libc::c_float * frac) as libc::c_int;
    if lines <= 0 as libc::c_int {
        return;
    }
    if lines > viddef.height {
        lines = viddef.height;
    }
    (re.DrawStretchPic)
        .expect(
            "non-null function pointer",
        )(
        0 as libc::c_int,
        -viddef.height + lines,
        viddef.width,
        viddef.height,
        b"conback\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    SCR_AddDirtyPoint(0 as libc::c_int, 0 as libc::c_int);
    SCR_AddDirtyPoint(viddef.width - 1 as libc::c_int, lines - 1 as libc::c_int);
    Com_sprintf(
        version.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
        b"v%4.2f\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        3.19f64,
    );
    x = 0 as libc::c_int;
    while x < 5 as libc::c_int {
        (re.DrawChar)
            .expect(
                "non-null function pointer",
            )(
            viddef.width - 44 as libc::c_int + x * 8 as libc::c_int,
            lines - 12 as libc::c_int,
            128 as libc::c_int + version[x as usize] as libc::c_int,
        );
        x += 1;
    }
    con.vislines = lines;
    rows = lines - 22 as libc::c_int >> 3 as libc::c_int;
    y = lines - 30 as libc::c_int;
    if con.display != con.current {
        x = 0 as libc::c_int;
        while x < con.linewidth {
            (re.DrawChar)
                .expect(
                    "non-null function pointer",
                )((x + 1 as libc::c_int) << 3 as libc::c_int, y, '^' as i32);
            x += 4 as libc::c_int;
        }
        y -= 8 as libc::c_int;
        rows -= 1;
    }
    row = con.display;
    i = 0 as libc::c_int;
    while i < rows {
        if row < 0 as libc::c_int {
            break;
        }
        if con.current - row >= con.totallines {
            break;
        }
        text = (con.text)
            .as_mut_ptr()
            .offset((row % con.totallines * con.linewidth) as isize);
        x = 0 as libc::c_int;
        while x < con.linewidth {
            (re.DrawChar)
                .expect(
                    "non-null function pointer",
                )(
                (x + 1 as libc::c_int) << 3 as libc::c_int,
                y,
                *text.offset(x as isize) as libc::c_int,
            );
            x += 1;
        }
        i += 1;
        y -= 8 as libc::c_int;
        row -= 1;
    }
    if !(cls.download).is_null() {
        text = strrchr((cls.downloadname).as_mut_ptr(), '/' as i32);
        if !text.is_null() {
            text = text.offset(1);
        } else {
            text = (cls.downloadname).as_mut_ptr();
        }
        x = con.linewidth - con.linewidth * 7 as libc::c_int / 40 as libc::c_int;
        y = (x as libc::c_ulong)
            .wrapping_sub(strlen(text))
            .wrapping_sub(8 as libc::c_int as libc::c_ulong) as libc::c_int;
        i = con.linewidth / 3 as libc::c_int;
        if strlen(text) > i as libc::c_ulong {
            y = x - i - 11 as libc::c_int;
            strncpy(dlbar.as_mut_ptr(), text, i as libc::c_ulong);
            dlbar[i as usize] = 0 as libc::c_int as libc::c_char;
            strcat(dlbar.as_mut_ptr(), b"...\0" as *const u8 as *const libc::c_char);
        } else {
            strcpy(dlbar.as_mut_ptr(), text);
        }
        strcat(dlbar.as_mut_ptr(), b": \0" as *const u8 as *const libc::c_char);
        i = strlen(dlbar.as_mut_ptr()) as libc::c_int;
        let fresh0 = i;
        i = i + 1;
        dlbar[fresh0 as usize] = -128i32 as libc::c_char;
        if cls.downloadpercent == 0 as libc::c_int {
            n = 0 as libc::c_int;
        } else {
            n = y * cls.downloadpercent / 100 as libc::c_int;
        }
        j = 0 as libc::c_int;
        while j < y {
            if j == n {
                let fresh1 = i;
                i = i + 1;
                dlbar[fresh1 as usize] = -125i32 as libc::c_char;
            } else {
                let fresh2 = i;
                i = i + 1;
                dlbar[fresh2 as usize] = -127i32 as libc::c_char;
            }
            j += 1;
        }
        let fresh3 = i;
        i = i + 1;
        dlbar[fresh3 as usize] = -126i32 as libc::c_char;
        dlbar[i as usize] = 0 as libc::c_int as libc::c_char;
        sprintf(
            dlbar.as_mut_ptr().offset(strlen(dlbar.as_mut_ptr()) as isize),
            b" %02d%%\0" as *const u8 as *const libc::c_char,
            cls.downloadpercent,
        );
        y = con.vislines - 12 as libc::c_int;
        i = 0 as libc::c_int;
        while (i as libc::c_ulong) < strlen(dlbar.as_mut_ptr()) {
            (re.DrawChar)
                .expect(
                    "non-null function pointer",
                )(
                (i + 1 as libc::c_int) << 3 as libc::c_int,
                y,
                dlbar[i as usize] as libc::c_int,
            );
            i += 1;
        }
    }
    Con_DrawInput();
}
