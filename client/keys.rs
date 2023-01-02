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
    fn Con_ToggleConsole_f();
    static mut cl: client_state_t;
    fn M_Menu_Main_f();
    fn M_Keydown(key: libc::c_int);
    static mut cls: client_static_t;
    static mut con: console_t;
    fn SCR_UpdateScreen();
    fn Sys_GetClipboardData() -> *mut libc::c_char;
    fn Sys_SendKeyEvents();
    fn Z_Malloc(size: libc::c_int) -> *mut libc::c_void;
    fn Z_Free(ptr: *mut libc::c_void);
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strtok(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn free(__ptr: *mut libc::c_void);
    fn Com_sprintf(
        dest: *mut libc::c_char,
        size: libc::c_int,
        fmt: *mut libc::c_char,
        _: ...
    );
    fn Q_strcasecmp(s1: *mut libc::c_char, s2: *mut libc::c_char) -> libc::c_int;
    fn Com_Printf(msg: *mut libc::c_char, _: ...);
    fn Cbuf_AddText(text: *mut libc::c_char);
    fn Cmd_AddCommand(cmd_name: *mut libc::c_char, function: xcommand_t);
    fn Cmd_CompleteCommand(partial: *mut libc::c_char) -> *mut libc::c_char;
    fn Cmd_Argc() -> libc::c_int;
    fn Cmd_Argv(arg: libc::c_int) -> *mut libc::c_char;
    fn Cvar_CompleteVariable(partial: *mut libc::c_char) -> *mut libc::c_char;
    fn Com_Error(code: libc::c_int, fmt: *mut libc::c_char, _: ...);
    fn toupper(_: libc::c_int) -> libc::c_int;
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

pub const ca_disconnected: connstate_t = 1;

pub type connstate_t = libc::c_uint;

pub const ca_active: connstate_t = 4;
pub const ca_connected: connstate_t = 3;
pub const ca_connecting: connstate_t = 2;
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
pub struct keyname_t {
    pub name: *mut libc::c_char,
    pub keynum: libc::c_int,
}

#[no_mangle]
pub static mut key_lines: [[libc::c_char; 256]; 32] = [[0; 256]; 32];
#[no_mangle]
pub static mut key_linepos: libc::c_int = 0;
#[no_mangle]
pub static mut shift_down: libc::c_int = false_0 as libc::c_int;
#[no_mangle]
pub static mut anykeydown: libc::c_int = 0;
#[no_mangle]
pub static mut edit_line: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut history_line: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut key_waiting: libc::c_int = 0;
#[no_mangle]
pub static mut keybindings: [*mut libc::c_char; 256] = [0 as *const libc::c_char
    as *mut libc::c_char; 256];
#[no_mangle]
pub static mut consolekeys: [qboolean; 256] = [false_0; 256];
#[no_mangle]
pub static mut menubound: [qboolean; 256] = [false_0; 256];
#[no_mangle]
pub static mut keyshift: [libc::c_int; 256] = [0; 256];
#[no_mangle]
pub static mut key_repeats: [libc::c_int; 256] = [0; 256];
#[no_mangle]
pub static mut keydown: [qboolean; 256] = [false_0; 256];
#[no_mangle]
pub static mut keynames: [keyname_t; 89] = [
    {
        let mut init = keyname_t {
            name: b"TAB\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: 9 as libc::c_int,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"ENTER\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: 13 as libc::c_int,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"ESCAPE\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: 27 as libc::c_int,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"SPACE\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: 32 as libc::c_int,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"BACKSPACE\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            keynum: 127 as libc::c_int,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"UPARROW\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: 128 as libc::c_int,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"DOWNARROW\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            keynum: 129 as libc::c_int,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"LEFTARROW\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            keynum: 130 as libc::c_int,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"RIGHTARROW\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            keynum: 131 as libc::c_int,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"ALT\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: 132 as libc::c_int,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"CTRL\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: 133 as libc::c_int,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"SHIFT\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: 134 as libc::c_int,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"F1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: 135 as libc::c_int,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"F2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: 136 as libc::c_int,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"F3\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: 137 as libc::c_int,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"F4\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: 138 as libc::c_int,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"F5\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: 139 as libc::c_int,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"F6\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: 140 as libc::c_int,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"F7\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: 141 as libc::c_int,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"F8\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: 142 as libc::c_int,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"F9\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: 143 as libc::c_int,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"F10\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: 144 as libc::c_int,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"F11\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: 145 as libc::c_int,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"F12\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: 146 as libc::c_int,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"INS\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: 147 as libc::c_int,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"DEL\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: 148 as libc::c_int,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"PGDN\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: 149 as libc::c_int,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"PGUP\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: 150 as libc::c_int,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"HOME\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: 151 as libc::c_int,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"END\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: 152 as libc::c_int,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"MOUSE1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: 200 as libc::c_int,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"MOUSE2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: 201 as libc::c_int,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"MOUSE3\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: 202 as libc::c_int,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"JOY1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: 203 as libc::c_int,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"JOY2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: 204 as libc::c_int,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"JOY3\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: 205 as libc::c_int,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"JOY4\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: 206 as libc::c_int,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"AUX1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: 207 as libc::c_int,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"AUX2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: 208 as libc::c_int,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"AUX3\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: 209 as libc::c_int,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"AUX4\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: 210 as libc::c_int,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"AUX5\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: 211 as libc::c_int,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"AUX6\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: 212 as libc::c_int,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"AUX7\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: 213 as libc::c_int,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"AUX8\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: 214 as libc::c_int,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"AUX9\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: 215 as libc::c_int,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"AUX10\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: 216 as libc::c_int,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"AUX11\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: 217 as libc::c_int,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"AUX12\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: 218 as libc::c_int,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"AUX13\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: 219 as libc::c_int,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"AUX14\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: 220 as libc::c_int,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"AUX15\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: 221 as libc::c_int,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"AUX16\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: 222 as libc::c_int,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"AUX17\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: 223 as libc::c_int,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"AUX18\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: 224 as libc::c_int,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"AUX19\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: 225 as libc::c_int,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"AUX20\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: 226 as libc::c_int,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"AUX21\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: 227 as libc::c_int,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"AUX22\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: 228 as libc::c_int,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"AUX23\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: 229 as libc::c_int,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"AUX24\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: 230 as libc::c_int,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"AUX25\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: 231 as libc::c_int,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"AUX26\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: 232 as libc::c_int,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"AUX27\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: 233 as libc::c_int,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"AUX28\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: 234 as libc::c_int,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"AUX29\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: 235 as libc::c_int,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"AUX30\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: 236 as libc::c_int,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"AUX31\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: 237 as libc::c_int,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"AUX32\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: 238 as libc::c_int,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"KP_HOME\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: 160 as libc::c_int,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"KP_UPARROW\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            keynum: 161 as libc::c_int,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"KP_PGUP\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: 162 as libc::c_int,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"KP_LEFTARROW\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            keynum: 163 as libc::c_int,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"KP_5\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: 164 as libc::c_int,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"KP_RIGHTARROW\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            keynum: 165 as libc::c_int,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"KP_END\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: 166 as libc::c_int,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"KP_DOWNARROW\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            keynum: 167 as libc::c_int,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"KP_PGDN\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: 168 as libc::c_int,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"KP_ENTER\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: 169 as libc::c_int,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"KP_INS\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: 170 as libc::c_int,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"KP_DEL\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: 171 as libc::c_int,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"KP_SLASH\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: 172 as libc::c_int,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"KP_MINUS\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: 173 as libc::c_int,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"KP_PLUS\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: 174 as libc::c_int,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"MWHEELUP\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: 240 as libc::c_int,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"MWHEELDOWN\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            keynum: 239 as libc::c_int,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"PAUSE\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            keynum: 255 as libc::c_int,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: b"SEMICOLON\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            keynum: ';' as i32,
        };
        init
    },
    {
        let mut init = keyname_t {
            name: 0 as *const libc::c_char as *mut libc::c_char,
            keynum: 0 as libc::c_int,
        };
        init
    },
];

#[no_mangle]
pub unsafe extern "C" fn CompleteCommand() {
    let mut cmd: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    s = (key_lines[edit_line as usize]).as_mut_ptr().offset(1 as libc::c_int as isize);
    if *s as libc::c_int == '\\' as i32 || *s as libc::c_int == '/' as i32 {
        s = s.offset(1);
    }
    cmd = Cmd_CompleteCommand(s);
    if cmd.is_null() {
        cmd = Cvar_CompleteVariable(s);
    }
    if !cmd.is_null() {
        key_lines[edit_line
            as usize][1 as libc::c_int as usize] = '/' as i32 as libc::c_char;
        strcpy(
            (key_lines[edit_line as usize])
                .as_mut_ptr()
                .offset(2 as libc::c_int as isize),
            cmd,
        );
        key_linepos = (strlen(cmd)).wrapping_add(2 as libc::c_int as libc::c_ulong)
            as libc::c_int;
        key_lines[edit_line as usize][key_linepos as usize] = ' ' as i32 as libc::c_char;
        key_linepos += 1;
        key_lines[edit_line
            as usize][key_linepos as usize] = 0 as libc::c_int as libc::c_char;
        return;
    }
}

#[no_mangle]
pub unsafe extern "C" fn Key_Console(mut key: libc::c_int) {
    match key {
        172 => {
            key = '/' as i32;
        }
        173 => {
            key = '-' as i32;
        }
        174 => {
            key = '+' as i32;
        }
        160 => {
            key = '7' as i32;
        }
        161 => {
            key = '8' as i32;
        }
        162 => {
            key = '9' as i32;
        }
        163 => {
            key = '4' as i32;
        }
        164 => {
            key = '5' as i32;
        }
        165 => {
            key = '6' as i32;
        }
        166 => {
            key = '1' as i32;
        }
        167 => {
            key = '2' as i32;
        }
        168 => {
            key = '3' as i32;
        }
        170 => {
            key = '0' as i32;
        }
        171 => {
            key = '.' as i32;
        }
        _ => {}
    }
    if toupper(key) == 'V' as i32
        && keydown[133 as libc::c_int as usize] as libc::c_uint != 0
        || (key == 147 as libc::c_int || key == 170 as libc::c_int)
        && keydown[134 as libc::c_int as usize] as libc::c_uint != 0
    {
        let mut cbd: *mut libc::c_char = 0 as *mut libc::c_char;
        cbd = Sys_GetClipboardData();
        if !cbd.is_null() {
            let mut i: libc::c_int = 0;
            strtok(cbd, b"\n\r\x08\0" as *const u8 as *const libc::c_char);
            i = strlen(cbd) as libc::c_int;
            if i + key_linepos >= 256 as libc::c_int {
                i = 256 as libc::c_int - key_linepos;
            }
            if i > 0 as libc::c_int {
                *cbd.offset(i as isize) = 0 as libc::c_int as libc::c_char;
                strcat((key_lines[edit_line as usize]).as_mut_ptr(), cbd);
                key_linepos += i;
            }
            free(cbd as *mut libc::c_void);
        }
        return;
    }
    if key == 'l' as i32 {
        if keydown[133 as libc::c_int as usize] as u64 != 0 {
            Cbuf_AddText(
                b"clear\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            return;
        }
    }
    if key == 13 as libc::c_int || key == 169 as libc::c_int {
        if key_lines[edit_line as usize][1 as libc::c_int as usize] as libc::c_int
            == '\\' as i32
            || key_lines[edit_line as usize][1 as libc::c_int as usize] as libc::c_int
            == '/' as i32
        {
            Cbuf_AddText(
                (key_lines[edit_line as usize])
                    .as_mut_ptr()
                    .offset(2 as libc::c_int as isize),
            );
        } else {
            Cbuf_AddText(
                (key_lines[edit_line as usize])
                    .as_mut_ptr()
                    .offset(1 as libc::c_int as isize),
            );
        }
        Cbuf_AddText(b"\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
        Com_Printf(
            b"%s\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            (key_lines[edit_line as usize]).as_mut_ptr(),
        );
        edit_line = edit_line + 1 as libc::c_int & 31 as libc::c_int;
        history_line = edit_line;
        key_lines[edit_line
            as usize][0 as libc::c_int as usize] = ']' as i32 as libc::c_char;
        key_linepos = 1 as libc::c_int;
        if cls.state as libc::c_uint == ca_disconnected as libc::c_int as libc::c_uint {
            SCR_UpdateScreen();
        }
        return;
    }
    if key == 9 as libc::c_int {
        CompleteCommand();
        return;
    }
    if key == 127 as libc::c_int || key == 130 as libc::c_int
        || key == 163 as libc::c_int
        || key == 'h' as i32 && keydown[133 as libc::c_int as usize] as libc::c_uint != 0
    {
        if key_linepos > 1 as libc::c_int {
            key_linepos -= 1;
        }
        return;
    }
    if key == 128 as libc::c_int || key == 161 as libc::c_int
        || key == 'p' as i32 && keydown[133 as libc::c_int as usize] as libc::c_uint != 0
    {
        loop {
            history_line = history_line - 1 as libc::c_int & 31 as libc::c_int;
            if !(history_line != edit_line
                && key_lines[history_line as usize][1 as libc::c_int as usize] == 0)
            {
                break;
            }
        }
        if history_line == edit_line {
            history_line = edit_line + 1 as libc::c_int & 31 as libc::c_int;
        }
        strcpy(
            (key_lines[edit_line as usize]).as_mut_ptr(),
            (key_lines[history_line as usize]).as_mut_ptr(),
        );
        key_linepos = strlen((key_lines[edit_line as usize]).as_mut_ptr())
            as libc::c_int;
        return;
    }
    if key == 129 as libc::c_int || key == 167 as libc::c_int
        || key == 'n' as i32 && keydown[133 as libc::c_int as usize] as libc::c_uint != 0
    {
        if history_line == edit_line {
            return;
        }
        loop {
            history_line = history_line + 1 as libc::c_int & 31 as libc::c_int;
            if !(history_line != edit_line
                && key_lines[history_line as usize][1 as libc::c_int as usize] == 0)
            {
                break;
            }
        }
        if history_line == edit_line {
            key_lines[edit_line
                as usize][0 as libc::c_int as usize] = ']' as i32 as libc::c_char;
            key_linepos = 1 as libc::c_int;
        } else {
            strcpy(
                (key_lines[edit_line as usize]).as_mut_ptr(),
                (key_lines[history_line as usize]).as_mut_ptr(),
            );
            key_linepos = strlen((key_lines[edit_line as usize]).as_mut_ptr())
                as libc::c_int;
        }
        return;
    }
    if key == 150 as libc::c_int || key == 162 as libc::c_int {
        con.display -= 2 as libc::c_int;
        return;
    }
    if key == 149 as libc::c_int || key == 168 as libc::c_int {
        con.display += 2 as libc::c_int;
        if con.display > con.current {
            con.display = con.current;
        }
        return;
    }
    if key == 151 as libc::c_int || key == 160 as libc::c_int {
        con.display = con.current - con.totallines + 10 as libc::c_int;
        return;
    }
    if key == 152 as libc::c_int || key == 166 as libc::c_int {
        con.display = con.current;
        return;
    }
    if key < 32 as libc::c_int || key > 127 as libc::c_int {
        return;
    }
    if key_linepos < 256 as libc::c_int - 1 as libc::c_int {
        key_lines[edit_line as usize][key_linepos as usize] = key as libc::c_char;
        key_linepos += 1;
        key_lines[edit_line
            as usize][key_linepos as usize] = 0 as libc::c_int as libc::c_char;
    }
}

#[no_mangle]
pub static mut chat_team: qboolean = false_0;
#[no_mangle]
pub static mut chat_buffer: [libc::c_char; 256] = [0; 256];
#[no_mangle]
pub static mut chat_bufferlen: libc::c_int = 0 as libc::c_int;

#[no_mangle]
pub unsafe extern "C" fn Key_Message(mut key: libc::c_int) {
    if key == 13 as libc::c_int || key == 169 as libc::c_int {
        if chat_team as u64 != 0 {
            Cbuf_AddText(
                b"say_team \"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
        } else {
            Cbuf_AddText(
                b"say \"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
        }
        Cbuf_AddText(chat_buffer.as_mut_ptr());
        Cbuf_AddText(b"\"\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
        cls.key_dest = key_game;
        chat_bufferlen = 0 as libc::c_int;
        chat_buffer[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        return;
    }
    if key == 27 as libc::c_int {
        cls.key_dest = key_game;
        chat_bufferlen = 0 as libc::c_int;
        chat_buffer[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        return;
    }
    if key < 32 as libc::c_int || key > 127 as libc::c_int {
        return;
    }
    if key == 127 as libc::c_int {
        if chat_bufferlen != 0 {
            chat_bufferlen -= 1;
            chat_buffer[chat_bufferlen as usize] = 0 as libc::c_int as libc::c_char;
        }
        return;
    }
    if chat_bufferlen as libc::c_ulong
        == (::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
    {
        return;
    }
    let fresh0 = chat_bufferlen;
    chat_bufferlen = chat_bufferlen + 1;
    chat_buffer[fresh0 as usize] = key as libc::c_char;
    chat_buffer[chat_bufferlen as usize] = 0 as libc::c_int as libc::c_char;
}

#[no_mangle]
pub unsafe extern "C" fn Key_StringToKeynum(mut str: *mut libc::c_char) -> libc::c_int {
    let mut kn: *mut keyname_t = 0 as *mut keyname_t;
    if str.is_null() || *str.offset(0 as libc::c_int as isize) == 0 {
        return -(1 as libc::c_int);
    }
    if *str.offset(1 as libc::c_int as isize) == 0 {
        return *str.offset(0 as libc::c_int as isize) as libc::c_int;
    }
    kn = keynames.as_mut_ptr();
    while !((*kn).name).is_null() {
        if Q_strcasecmp(str, (*kn).name) == 0 {
            return (*kn).keynum;
        }
        kn = kn.offset(1);
    }
    return -(1 as libc::c_int);
}

#[no_mangle]
pub unsafe extern "C" fn Key_KeynumToString(
    mut keynum: libc::c_int,
) -> *mut libc::c_char {
    let mut kn: *mut keyname_t = 0 as *mut keyname_t;
    static mut tinystr: [libc::c_char; 2] = [0; 2];
    if keynum == -(1 as libc::c_int) {
        return b"<KEY NOT FOUND>\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char;
    }
    if keynum > 32 as libc::c_int && keynum < 127 as libc::c_int {
        tinystr[0 as libc::c_int as usize] = keynum as libc::c_char;
        tinystr[1 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        return tinystr.as_mut_ptr();
    }
    kn = keynames.as_mut_ptr();
    while !((*kn).name).is_null() {
        if keynum == (*kn).keynum {
            return (*kn).name;
        }
        kn = kn.offset(1);
    }
    return b"<UNKNOWN KEYNUM>\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char;
}

#[no_mangle]
pub unsafe extern "C" fn Key_SetBinding(
    mut keynum: libc::c_int,
    mut binding: *mut libc::c_char,
) {
    let mut new: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut l: libc::c_int = 0;
    if keynum == -(1 as libc::c_int) {
        return;
    }
    if !(keybindings[keynum as usize]).is_null() {
        Z_Free(keybindings[keynum as usize] as *mut libc::c_void);
        keybindings[keynum as usize] = 0 as *mut libc::c_char;
    }
    l = strlen(binding) as libc::c_int;
    new = Z_Malloc(l + 1 as libc::c_int) as *mut libc::c_char;
    strcpy(new, binding);
    *new.offset(l as isize) = 0 as libc::c_int as libc::c_char;
    keybindings[keynum as usize] = new;
}

#[no_mangle]
pub unsafe extern "C" fn Key_Unbind_f() {
    let mut b: libc::c_int = 0;
    if Cmd_Argc() != 2 as libc::c_int {
        Com_Printf(
            b"unbind <key> : remove commands from a key\n\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        );
        return;
    }
    b = Key_StringToKeynum(Cmd_Argv(1 as libc::c_int));
    if b == -(1 as libc::c_int) {
        Com_Printf(
            b"\"%s\" isn't a valid key\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            Cmd_Argv(1 as libc::c_int),
        );
        return;
    }
    Key_SetBinding(b, b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
}

#[no_mangle]
pub unsafe extern "C" fn Key_Unbindall_f() {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 256 as libc::c_int {
        if !(keybindings[i as usize]).is_null() {
            Key_SetBinding(
                i,
                b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
        }
        i += 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn Key_Bind_f() {
    let mut i: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut b: libc::c_int = 0;
    let mut cmd: [libc::c_char; 1024] = [0; 1024];
    c = Cmd_Argc();
    if c < 2 as libc::c_int {
        Com_Printf(
            b"bind <key> [command] : attach a command to a key\n\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        );
        return;
    }
    b = Key_StringToKeynum(Cmd_Argv(1 as libc::c_int));
    if b == -(1 as libc::c_int) {
        Com_Printf(
            b"\"%s\" isn't a valid key\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            Cmd_Argv(1 as libc::c_int),
        );
        return;
    }
    if c == 2 as libc::c_int {
        if !(keybindings[b as usize]).is_null() {
            Com_Printf(
                b"\"%s\" = \"%s\"\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                Cmd_Argv(1 as libc::c_int),
                keybindings[b as usize],
            );
        } else {
            Com_Printf(
                b"\"%s\" is not bound\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                Cmd_Argv(1 as libc::c_int),
            );
        }
        return;
    }
    cmd[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    i = 2 as libc::c_int;
    while i < c {
        strcat(cmd.as_mut_ptr(), Cmd_Argv(i));
        if i != c - 1 as libc::c_int {
            strcat(cmd.as_mut_ptr(), b" \0" as *const u8 as *const libc::c_char);
        }
        i += 1;
    }
    Key_SetBinding(b, cmd.as_mut_ptr());
}

#[no_mangle]
pub unsafe extern "C" fn Key_WriteBindings(mut f: *mut FILE) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 256 as libc::c_int {
        if !(keybindings[i as usize]).is_null()
            && *(keybindings[i as usize]).offset(0 as libc::c_int as isize)
            as libc::c_int != 0
        {
            fprintf(
                f,
                b"bind %s \"%s\"\n\0" as *const u8 as *const libc::c_char,
                Key_KeynumToString(i),
                keybindings[i as usize],
            );
        }
        i += 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn Key_Bindlist_f() {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 256 as libc::c_int {
        if !(keybindings[i as usize]).is_null()
            && *(keybindings[i as usize]).offset(0 as libc::c_int as isize)
            as libc::c_int != 0
        {
            Com_Printf(
                b"%s \"%s\"\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                Key_KeynumToString(i),
                keybindings[i as usize],
            );
        }
        i += 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn Key_Init() {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        key_lines[i as usize][0 as libc::c_int as usize] = ']' as i32 as libc::c_char;
        key_lines[i
            as usize][1 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        i += 1;
    }
    key_linepos = 1 as libc::c_int;
    i = 32 as libc::c_int;
    while i < 128 as libc::c_int {
        consolekeys[i as usize] = true_0;
        i += 1;
    }
    consolekeys[13 as libc::c_int as usize] = true_0;
    consolekeys[169 as libc::c_int as usize] = true_0;
    consolekeys[9 as libc::c_int as usize] = true_0;
    consolekeys[130 as libc::c_int as usize] = true_0;
    consolekeys[163 as libc::c_int as usize] = true_0;
    consolekeys[131 as libc::c_int as usize] = true_0;
    consolekeys[165 as libc::c_int as usize] = true_0;
    consolekeys[128 as libc::c_int as usize] = true_0;
    consolekeys[161 as libc::c_int as usize] = true_0;
    consolekeys[129 as libc::c_int as usize] = true_0;
    consolekeys[167 as libc::c_int as usize] = true_0;
    consolekeys[127 as libc::c_int as usize] = true_0;
    consolekeys[151 as libc::c_int as usize] = true_0;
    consolekeys[160 as libc::c_int as usize] = true_0;
    consolekeys[152 as libc::c_int as usize] = true_0;
    consolekeys[166 as libc::c_int as usize] = true_0;
    consolekeys[150 as libc::c_int as usize] = true_0;
    consolekeys[162 as libc::c_int as usize] = true_0;
    consolekeys[149 as libc::c_int as usize] = true_0;
    consolekeys[168 as libc::c_int as usize] = true_0;
    consolekeys[134 as libc::c_int as usize] = true_0;
    consolekeys[147 as libc::c_int as usize] = true_0;
    consolekeys[170 as libc::c_int as usize] = true_0;
    consolekeys[171 as libc::c_int as usize] = true_0;
    consolekeys[172 as libc::c_int as usize] = true_0;
    consolekeys[174 as libc::c_int as usize] = true_0;
    consolekeys[173 as libc::c_int as usize] = true_0;
    consolekeys[164 as libc::c_int as usize] = true_0;
    consolekeys['`' as i32 as usize] = false_0;
    consolekeys['~' as i32 as usize] = false_0;
    i = 0 as libc::c_int;
    while i < 256 as libc::c_int {
        keyshift[i as usize] = i;
        i += 1;
    }
    i = 'a' as i32;
    while i <= 'z' as i32 {
        keyshift[i as usize] = i - 'a' as i32 + 'A' as i32;
        i += 1;
    }
    keyshift['1' as i32 as usize] = '!' as i32;
    keyshift['2' as i32 as usize] = '@' as i32;
    keyshift['3' as i32 as usize] = '#' as i32;
    keyshift['4' as i32 as usize] = '$' as i32;
    keyshift['5' as i32 as usize] = '%' as i32;
    keyshift['6' as i32 as usize] = '^' as i32;
    keyshift['7' as i32 as usize] = '&' as i32;
    keyshift['8' as i32 as usize] = '*' as i32;
    keyshift['9' as i32 as usize] = '(' as i32;
    keyshift['0' as i32 as usize] = ')' as i32;
    keyshift['-' as i32 as usize] = '_' as i32;
    keyshift['=' as i32 as usize] = '+' as i32;
    keyshift[',' as i32 as usize] = '<' as i32;
    keyshift['.' as i32 as usize] = '>' as i32;
    keyshift['/' as i32 as usize] = '?' as i32;
    keyshift[';' as i32 as usize] = ':' as i32;
    keyshift['\'' as i32 as usize] = '"' as i32;
    keyshift['[' as i32 as usize] = '{' as i32;
    keyshift[']' as i32 as usize] = '}' as i32;
    keyshift['`' as i32 as usize] = '~' as i32;
    keyshift['\\' as i32 as usize] = '|' as i32;
    menubound[27 as libc::c_int as usize] = true_0;
    i = 0 as libc::c_int;
    while i < 12 as libc::c_int {
        menubound[(135 as libc::c_int + i) as usize] = true_0;
        i += 1;
    }
    Cmd_AddCommand(
        b"bind\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        Some(Key_Bind_f as unsafe extern "C" fn() -> ()),
    );
    Cmd_AddCommand(
        b"unbind\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        Some(Key_Unbind_f as unsafe extern "C" fn() -> ()),
    );
    Cmd_AddCommand(
        b"unbindall\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        Some(Key_Unbindall_f as unsafe extern "C" fn() -> ()),
    );
    Cmd_AddCommand(
        b"bindlist\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        Some(Key_Bindlist_f as unsafe extern "C" fn() -> ()),
    );
}

#[no_mangle]
pub unsafe extern "C" fn Key_Event(
    mut key: libc::c_int,
    mut down: qboolean,
    mut time: libc::c_uint,
) {
    let mut kb: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cmd: [libc::c_char; 1024] = [0; 1024];
    if key_waiting == -(1 as libc::c_int) {
        if down as u64 != 0 {
            key_waiting = key;
        }
        return;
    }
    if down as u64 != 0 {
        key_repeats[key as usize] += 1;
        if key != 127 as libc::c_int && key != 255 as libc::c_int
            && key != 150 as libc::c_int && key != 162 as libc::c_int
            && key != 149 as libc::c_int && key != 168 as libc::c_int
            && key_repeats[key as usize] > 1 as libc::c_int
        {
            return;
        }
        if key >= 200 as libc::c_int && (keybindings[key as usize]).is_null() {
            Com_Printf(
                b"%s is unbound, hit F4 to set.\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                Key_KeynumToString(key),
            );
        }
    } else {
        key_repeats[key as usize] = 0 as libc::c_int;
    }
    if key == 134 as libc::c_int {
        shift_down = down as libc::c_int;
    }
    if key == '`' as i32 || key == '~' as i32 {
        if down as u64 == 0 {
            return;
        }
        Con_ToggleConsole_f();
        return;
    }
    if cl.attractloop as libc::c_uint != 0
        && cls.key_dest as libc::c_uint != key_menu as libc::c_int as libc::c_uint
    {
        key = 27 as libc::c_int;
    }
    if key == 27 as libc::c_int {
        if down as u64 == 0 {
            return;
        }
        if cl.frame.playerstate.stats[13 as libc::c_int as usize] as libc::c_int != 0
            && cls.key_dest as libc::c_uint == key_game as libc::c_int as libc::c_uint
        {
            Cbuf_AddText(
                b"cmd putaway\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
            return;
        }
        match cls.key_dest as libc::c_uint {
            2 => {
                Key_Message(key);
            }
            3 => {
                M_Keydown(key);
            }
            0 | 1 => {
                M_Menu_Main_f();
            }
            _ => {
                Com_Error(
                    0 as libc::c_int,
                    b"Bad cls.key_dest\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                );
            }
        }
        return;
    }
    keydown[key as usize] = down;
    if down as u64 != 0 {
        if key_repeats[key as usize] == 1 as libc::c_int {
            anykeydown += 1;
        }
    } else {
        anykeydown -= 1;
        if anykeydown < 0 as libc::c_int {
            anykeydown = 0 as libc::c_int;
        }
    }
    if down as u64 == 0 {
        kb = keybindings[key as usize];
        if !kb.is_null()
            && *kb.offset(0 as libc::c_int as isize) as libc::c_int == '+' as i32
        {
            Com_sprintf(
                cmd.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong
                    as libc::c_int,
                b"-%s %i %i\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                kb.offset(1 as libc::c_int as isize),
                key,
                time,
            );
            Cbuf_AddText(cmd.as_mut_ptr());
        }
        if keyshift[key as usize] != key {
            kb = keybindings[keyshift[key as usize] as usize];
            if !kb.is_null()
                && *kb.offset(0 as libc::c_int as isize) as libc::c_int == '+' as i32
            {
                Com_sprintf(
                    cmd.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong
                        as libc::c_int,
                    b"-%s %i %i\n\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    kb.offset(1 as libc::c_int as isize),
                    key,
                    time,
                );
                Cbuf_AddText(cmd.as_mut_ptr());
            }
        }
        return;
    }
    if cls.key_dest as libc::c_uint == key_menu as libc::c_int as libc::c_uint
        && menubound[key as usize] as libc::c_uint != 0
        || cls.key_dest as libc::c_uint == key_console as libc::c_int as libc::c_uint
        && consolekeys[key as usize] as u64 == 0
        || cls.key_dest as libc::c_uint == key_game as libc::c_int as libc::c_uint
        && (cls.state as libc::c_uint == ca_active as libc::c_int as libc::c_uint
        || consolekeys[key as usize] as u64 == 0)
    {
        kb = keybindings[key as usize];
        if !kb.is_null() {
            if *kb.offset(0 as libc::c_int as isize) as libc::c_int == '+' as i32 {
                Com_sprintf(
                    cmd.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong
                        as libc::c_int,
                    b"%s %i %i\n\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    kb,
                    key,
                    time,
                );
                Cbuf_AddText(cmd.as_mut_ptr());
            } else {
                Cbuf_AddText(kb);
                Cbuf_AddText(
                    b"\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                );
            }
        }
        return;
    }
    if down as u64 == 0 {
        return;
    }
    if shift_down != 0 {
        key = keyshift[key as usize];
    }
    match cls.key_dest as libc::c_uint {
        2 => {
            Key_Message(key);
        }
        3 => {
            M_Keydown(key);
        }
        0 | 1 => {
            Key_Console(key);
        }
        _ => {
            Com_Error(
                0 as libc::c_int,
                b"Bad cls.key_dest\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
        }
    };
}

#[no_mangle]
pub unsafe extern "C" fn Key_ClearStates() {
    let mut i: libc::c_int = 0;
    anykeydown = false_0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < 256 as libc::c_int {
        if keydown[i as usize] as libc::c_uint != 0 || key_repeats[i as usize] != 0 {
            Key_Event(i, false_0, 0 as libc::c_int as libc::c_uint);
        }
        keydown[i as usize] = false_0;
        key_repeats[i as usize] = 0 as libc::c_int;
        i += 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn Key_GetKey() -> libc::c_int {
    key_waiting = -(1 as libc::c_int);
    while key_waiting == -(1 as libc::c_int) {
        Sys_SendKeyEvents();
    }
    return key_waiting;
}
