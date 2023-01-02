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
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fread(
        _: *mut libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn Com_sprintf(
        dest: *mut libc::c_char,
        size: libc::c_int,
        fmt: *mut libc::c_char,
        _: ...
    );
    fn LittleLong(l: libc::c_int) -> libc::c_int;
    fn va(format: *mut libc::c_char, _: ...) -> *mut libc::c_char;
    fn Sys_Milliseconds() -> libc::c_int;
    fn Com_Printf(msg: *mut libc::c_char, _: ...);
    fn SZ_Print(buf: *mut sizebuf_t, data: *mut libc::c_char);
    fn MSG_WriteByte(sb: *mut sizebuf_t, c: libc::c_int);
    fn Cvar_SetValue(var_name: *mut libc::c_char, value: libc::c_float);
    fn Cvar_VariableValue(var_name: *mut libc::c_char) -> libc::c_float;
    static mut viddef: viddef_t;
    fn SCR_EndLoadingPlaque();
    static mut cl: client_state_t;
    fn S_RawSamples(
        samples: libc::c_int,
        rate: libc::c_int,
        width: libc::c_int,
        channels: libc::c_int,
        data: *mut byte,
    );
    fn CL_Snd_Restart_f();
    static mut cls: client_static_t;
    fn CDAudio_Stop();
    static mut re: refexport_t;
    fn FS_FOpenFile(filename: *mut libc::c_char, file: *mut *mut FILE) -> libc::c_int;
    fn FS_LoadFile(
        path: *mut libc::c_char,
        buffer: *mut *mut libc::c_void,
    ) -> libc::c_int;
    fn FS_Read(buffer: *mut libc::c_void, len: libc::c_int, f: *mut FILE);
    fn FS_FreeFile(buffer: *mut libc::c_void);
    fn Com_Error(code: libc::c_int, fmt: *mut libc::c_char, _: ...);
    fn Z_Malloc(size: libc::c_int) -> *mut libc::c_void;
    fn Z_Free(ptr: *mut libc::c_void);
    fn SCR_BeginLoadingPlaque();
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
pub struct pcx_t {
    pub manufacturer: libc::c_char,
    pub version: libc::c_char,
    pub encoding: libc::c_char,
    pub bits_per_pixel: libc::c_char,
    pub xmin: libc::c_ushort,
    pub ymin: libc::c_ushort,
    pub xmax: libc::c_ushort,
    pub ymax: libc::c_ushort,
    pub hres: libc::c_ushort,
    pub vres: libc::c_ushort,
    pub palette: [libc::c_uchar; 48],
    pub reserved: libc::c_char,
    pub color_planes: libc::c_char,
    pub bytes_per_line: libc::c_ushort,
    pub palette_type: libc::c_ushort,
    pub filler: [libc::c_char; 58],
    pub data: libc::c_uchar,
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
pub struct cblock_t {
    pub data: *mut byte,
    pub count: libc::c_int,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct cinematics_t {
    pub restart_sound: qboolean,
    pub s_rate: libc::c_int,
    pub s_width: libc::c_int,
    pub s_channels: libc::c_int,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub pic: *mut byte,
    pub pic_pending: *mut byte,
    pub hnodes1: *mut libc::c_int,
    pub numhnodes1: [libc::c_int; 256],
    pub h_used: [libc::c_int; 512],
    pub h_count: [libc::c_int; 512],
}

pub type connstate_t = libc::c_uint;

pub const ca_active: connstate_t = 4;
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
pub static mut cin: cinematics_t = cinematics_t {
    restart_sound: false_0,
    s_rate: 0,
    s_width: 0,
    s_channels: 0,
    width: 0,
    height: 0,
    pic: 0 as *const byte as *mut byte,
    pic_pending: 0 as *const byte as *mut byte,
    hnodes1: 0 as *const libc::c_int as *mut libc::c_int,
    numhnodes1: [0; 256],
    h_used: [0; 512],
    h_count: [0; 512],
};

#[no_mangle]
pub unsafe extern "C" fn SCR_LoadPCX(
    mut filename: *mut libc::c_char,
    mut pic: *mut *mut byte,
    mut palette: *mut *mut byte,
    mut width: *mut libc::c_int,
    mut height: *mut libc::c_int,
) {
    let mut raw: *mut byte = 0 as *mut byte;
    let mut pcx: *mut pcx_t = 0 as *mut pcx_t;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut dataByte: libc::c_int = 0;
    let mut runLength: libc::c_int = 0;
    let mut out: *mut byte = 0 as *mut byte;
    let mut pix: *mut byte = 0 as *mut byte;
    *pic = 0 as *mut byte;
    len = FS_LoadFile(filename, &mut raw as *mut *mut byte as *mut *mut libc::c_void);
    if raw.is_null() {
        return;
    }
    pcx = raw as *mut pcx_t;
    raw = &mut (*pcx).data;
    if (*pcx).manufacturer as libc::c_int != 0xa as libc::c_int
        || (*pcx).version as libc::c_int != 5 as libc::c_int
        || (*pcx).encoding as libc::c_int != 1 as libc::c_int
        || (*pcx).bits_per_pixel as libc::c_int != 8 as libc::c_int
        || (*pcx).xmax as libc::c_int >= 640 as libc::c_int
        || (*pcx).ymax as libc::c_int >= 480 as libc::c_int
    {
        Com_Printf(
            b"Bad pcx file %s\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            filename,
        );
        return;
    }
    out = Z_Malloc(
        ((*pcx).ymax as libc::c_int + 1 as libc::c_int)
            * ((*pcx).xmax as libc::c_int + 1 as libc::c_int),
    ) as *mut byte;
    *pic = out;
    pix = out;
    if !palette.is_null() {
        *palette = Z_Malloc(768 as libc::c_int) as *mut byte;
        memcpy(
            *palette as *mut libc::c_void,
            (pcx as *mut byte)
                .offset(len as isize)
                .offset(-(768 as libc::c_int as isize)) as *const libc::c_void,
            768 as libc::c_int as libc::c_ulong,
        );
    }
    if !width.is_null() {
        *width = (*pcx).xmax as libc::c_int + 1 as libc::c_int;
    }
    if !height.is_null() {
        *height = (*pcx).ymax as libc::c_int + 1 as libc::c_int;
    }
    y = 0 as libc::c_int;
    while y <= (*pcx).ymax as libc::c_int {
        x = 0 as libc::c_int;
        while x <= (*pcx).xmax as libc::c_int {
            let fresh0 = raw;
            raw = raw.offset(1);
            dataByte = *fresh0 as libc::c_int;
            if dataByte & 0xc0 as libc::c_int == 0xc0 as libc::c_int {
                runLength = dataByte & 0x3f as libc::c_int;
                let fresh1 = raw;
                raw = raw.offset(1);
                dataByte = *fresh1 as libc::c_int;
            } else {
                runLength = 1 as libc::c_int;
            }
            loop {
                let fresh2 = runLength;
                runLength = runLength - 1;
                if !(fresh2 > 0 as libc::c_int) {
                    break;
                }
                let fresh3 = x;
                x = x + 1;
                *pix.offset(fresh3 as isize) = dataByte as byte;
            }
        }
        y += 1;
        pix = pix.offset(((*pcx).xmax as libc::c_int + 1 as libc::c_int) as isize);
    }
    if raw.offset_from(pcx as *mut byte) as libc::c_long > len as libc::c_long {
        Com_Printf(
            b"PCX file %s was malformed\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            filename,
        );
        Z_Free(*pic as *mut libc::c_void);
        *pic = 0 as *mut byte;
    }
    FS_FreeFile(pcx as *mut libc::c_void);
}

#[no_mangle]
pub unsafe extern "C" fn SCR_StopCinematic() {
    cl.cinematictime = 0 as libc::c_int;
    if !(cin.pic).is_null() {
        Z_Free(cin.pic as *mut libc::c_void);
        cin.pic = 0 as *mut byte;
    }
    if !(cin.pic_pending).is_null() {
        Z_Free(cin.pic_pending as *mut libc::c_void);
        cin.pic_pending = 0 as *mut byte;
    }
    if cl.cinematicpalette_active as u64 != 0 {
        (re.CinematicSetPalette)
            .expect("non-null function pointer")(0 as *const libc::c_uchar);
        cl.cinematicpalette_active = false_0;
    }
    if !(cl.cinematic_file).is_null() {
        fclose(cl.cinematic_file);
        cl.cinematic_file = 0 as *mut FILE;
    }
    if !(cin.hnodes1).is_null() {
        Z_Free(cin.hnodes1 as *mut libc::c_void);
        cin.hnodes1 = 0 as *mut libc::c_int;
    }
    if cin.restart_sound as u64 != 0 {
        cin.restart_sound = false_0;
        CL_Snd_Restart_f();
    }
}

#[no_mangle]
pub unsafe extern "C" fn SCR_FinishCinematic() {
    MSG_WriteByte(&mut cls.netchan.message, clc_stringcmd as libc::c_int);
    SZ_Print(
        &mut cls.netchan.message,
        va(
            b"nextserver %i\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            cl.servercount,
        ),
    );
}

#[no_mangle]
pub unsafe extern "C" fn SmallestNode1(mut numhnodes: libc::c_int) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut best: libc::c_int = 0;
    let mut bestnode: libc::c_int = 0;
    best = 99999999 as libc::c_int;
    bestnode = -(1 as libc::c_int);
    i = 0 as libc::c_int;
    while i < numhnodes {
        if !(cin.h_used[i as usize] != 0) {
            if !(cin.h_count[i as usize] == 0) {
                if cin.h_count[i as usize] < best {
                    best = cin.h_count[i as usize];
                    bestnode = i;
                }
            }
        }
        i += 1;
    }
    if bestnode == -(1 as libc::c_int) {
        return -(1 as libc::c_int);
    }
    cin.h_used[bestnode as usize] = true_0 as libc::c_int;
    return bestnode;
}

#[no_mangle]
pub unsafe extern "C" fn Huff1TableInit() {
    let mut prev: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut node: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut nodebase: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut counts: [byte; 256] = [0; 256];
    let mut numhnodes: libc::c_int = 0;
    cin
        .hnodes1 = Z_Malloc(
        256 as libc::c_int * 256 as libc::c_int * 2 as libc::c_int * 4 as libc::c_int,
    ) as *mut libc::c_int;
    memset(
        cin.hnodes1 as *mut libc::c_void,
        0 as libc::c_int,
        (256 as libc::c_int * 256 as libc::c_int * 2 as libc::c_int * 4 as libc::c_int)
            as libc::c_ulong,
    );
    prev = 0 as libc::c_int;
    while prev < 256 as libc::c_int {
        memset(
            (cin.h_count).as_mut_ptr() as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<[libc::c_int; 512]>() as libc::c_ulong,
        );
        memset(
            (cin.h_used).as_mut_ptr() as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<[libc::c_int; 512]>() as libc::c_ulong,
        );
        FS_Read(
            counts.as_mut_ptr() as *mut libc::c_void,
            ::std::mem::size_of::<[byte; 256]>() as libc::c_ulong as libc::c_int,
            cl.cinematic_file,
        );
        j = 0 as libc::c_int;
        while j < 256 as libc::c_int {
            cin.h_count[j as usize] = counts[j as usize] as libc::c_int;
            j += 1;
        }
        numhnodes = 256 as libc::c_int;
        nodebase = (cin.hnodes1)
            .offset((prev * 256 as libc::c_int * 2 as libc::c_int) as isize);
        while numhnodes != 511 as libc::c_int {
            node = nodebase
                .offset(((numhnodes - 256 as libc::c_int) * 2 as libc::c_int) as isize);
            *node.offset(0 as libc::c_int as isize) = SmallestNode1(numhnodes);
            if *node.offset(0 as libc::c_int as isize) == -(1 as libc::c_int) {
                break;
            }
            *node.offset(1 as libc::c_int as isize) = SmallestNode1(numhnodes);
            if *node.offset(1 as libc::c_int as isize) == -(1 as libc::c_int) {
                break;
            }
            cin
                .h_count[numhnodes
                as usize] = cin.h_count[*node.offset(0 as libc::c_int as isize) as usize]
                + cin.h_count[*node.offset(1 as libc::c_int as isize) as usize];
            numhnodes += 1;
        }
        cin.numhnodes1[prev as usize] = numhnodes - 1 as libc::c_int;
        prev += 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn Huff1Decompress(mut in_0: cblock_t) -> cblock_t {
    let mut input: *mut byte = 0 as *mut byte;
    let mut out_p: *mut byte = 0 as *mut byte;
    let mut nodenum: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    let mut out: cblock_t = cblock_t {
        data: 0 as *mut byte,
        count: 0,
    };
    let mut inbyte: libc::c_int = 0;
    let mut hnodes: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut hnodesbase: *mut libc::c_int = 0 as *mut libc::c_int;
    count = *(in_0.data).offset(0 as libc::c_int as isize) as libc::c_int
        + ((*(in_0.data).offset(1 as libc::c_int as isize) as libc::c_int)
        << 8 as libc::c_int)
        + ((*(in_0.data).offset(2 as libc::c_int as isize) as libc::c_int)
        << 16 as libc::c_int)
        + ((*(in_0.data).offset(3 as libc::c_int as isize) as libc::c_int)
        << 24 as libc::c_int);
    input = (in_0.data).offset(4 as libc::c_int as isize);
    out.data = Z_Malloc(count) as *mut byte;
    out_p = out.data;
    hnodesbase = (cin.hnodes1)
        .offset(-((256 as libc::c_int * 2 as libc::c_int) as isize));
    hnodes = hnodesbase;
    nodenum = cin.numhnodes1[0 as libc::c_int as usize];
    while count != 0 {
        let fresh4 = input;
        input = input.offset(1);
        inbyte = *fresh4 as libc::c_int;
        if nodenum < 256 as libc::c_int {
            hnodes = hnodesbase.offset((nodenum << 9 as libc::c_int) as isize);
            let fresh5 = out_p;
            out_p = out_p.offset(1);
            *fresh5 = nodenum as byte;
            count -= 1;
            if count == 0 {
                break;
            }
            nodenum = cin.numhnodes1[nodenum as usize];
        }
        nodenum = *hnodes
            .offset((nodenum * 2 as libc::c_int + (inbyte & 1 as libc::c_int)) as isize);
        inbyte >>= 1 as libc::c_int;
        if nodenum < 256 as libc::c_int {
            hnodes = hnodesbase.offset((nodenum << 9 as libc::c_int) as isize);
            let fresh6 = out_p;
            out_p = out_p.offset(1);
            *fresh6 = nodenum as byte;
            count -= 1;
            if count == 0 {
                break;
            }
            nodenum = cin.numhnodes1[nodenum as usize];
        }
        nodenum = *hnodes
            .offset((nodenum * 2 as libc::c_int + (inbyte & 1 as libc::c_int)) as isize);
        inbyte >>= 1 as libc::c_int;
        if nodenum < 256 as libc::c_int {
            hnodes = hnodesbase.offset((nodenum << 9 as libc::c_int) as isize);
            let fresh7 = out_p;
            out_p = out_p.offset(1);
            *fresh7 = nodenum as byte;
            count -= 1;
            if count == 0 {
                break;
            }
            nodenum = cin.numhnodes1[nodenum as usize];
        }
        nodenum = *hnodes
            .offset((nodenum * 2 as libc::c_int + (inbyte & 1 as libc::c_int)) as isize);
        inbyte >>= 1 as libc::c_int;
        if nodenum < 256 as libc::c_int {
            hnodes = hnodesbase.offset((nodenum << 9 as libc::c_int) as isize);
            let fresh8 = out_p;
            out_p = out_p.offset(1);
            *fresh8 = nodenum as byte;
            count -= 1;
            if count == 0 {
                break;
            }
            nodenum = cin.numhnodes1[nodenum as usize];
        }
        nodenum = *hnodes
            .offset((nodenum * 2 as libc::c_int + (inbyte & 1 as libc::c_int)) as isize);
        inbyte >>= 1 as libc::c_int;
        if nodenum < 256 as libc::c_int {
            hnodes = hnodesbase.offset((nodenum << 9 as libc::c_int) as isize);
            let fresh9 = out_p;
            out_p = out_p.offset(1);
            *fresh9 = nodenum as byte;
            count -= 1;
            if count == 0 {
                break;
            }
            nodenum = cin.numhnodes1[nodenum as usize];
        }
        nodenum = *hnodes
            .offset((nodenum * 2 as libc::c_int + (inbyte & 1 as libc::c_int)) as isize);
        inbyte >>= 1 as libc::c_int;
        if nodenum < 256 as libc::c_int {
            hnodes = hnodesbase.offset((nodenum << 9 as libc::c_int) as isize);
            let fresh10 = out_p;
            out_p = out_p.offset(1);
            *fresh10 = nodenum as byte;
            count -= 1;
            if count == 0 {
                break;
            }
            nodenum = cin.numhnodes1[nodenum as usize];
        }
        nodenum = *hnodes
            .offset((nodenum * 2 as libc::c_int + (inbyte & 1 as libc::c_int)) as isize);
        inbyte >>= 1 as libc::c_int;
        if nodenum < 256 as libc::c_int {
            hnodes = hnodesbase.offset((nodenum << 9 as libc::c_int) as isize);
            let fresh11 = out_p;
            out_p = out_p.offset(1);
            *fresh11 = nodenum as byte;
            count -= 1;
            if count == 0 {
                break;
            }
            nodenum = cin.numhnodes1[nodenum as usize];
        }
        nodenum = *hnodes
            .offset((nodenum * 2 as libc::c_int + (inbyte & 1 as libc::c_int)) as isize);
        inbyte >>= 1 as libc::c_int;
        if nodenum < 256 as libc::c_int {
            hnodes = hnodesbase.offset((nodenum << 9 as libc::c_int) as isize);
            let fresh12 = out_p;
            out_p = out_p.offset(1);
            *fresh12 = nodenum as byte;
            count -= 1;
            if count == 0 {
                break;
            }
            nodenum = cin.numhnodes1[nodenum as usize];
        }
        nodenum = *hnodes
            .offset((nodenum * 2 as libc::c_int + (inbyte & 1 as libc::c_int)) as isize);
        inbyte >>= 1 as libc::c_int;
    }
    if input.offset_from(in_0.data) as libc::c_long != in_0.count as libc::c_long
        && input.offset_from(in_0.data) as libc::c_long
        != (in_0.count + 1 as libc::c_int) as libc::c_long
    {
        Com_Printf(
            b"Decompression overread by %i\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            input.offset_from(in_0.data) as libc::c_long - in_0.count as libc::c_long,
        );
    }
    out.count = out_p.offset_from(out.data) as libc::c_long as libc::c_int;
    return out;
}

#[no_mangle]
pub unsafe extern "C" fn SCR_ReadNextFrame() -> *mut byte {
    let mut r: libc::c_int = 0;
    let mut command: libc::c_int = 0;
    let mut samples: [byte; 6300] = [0; 6300];
    let mut compressed: [byte; 131072] = [0; 131072];
    let mut size: libc::c_int = 0;
    let mut pic: *mut byte = 0 as *mut byte;
    let mut in_0: cblock_t = cblock_t {
        data: 0 as *mut byte,
        count: 0,
    };
    let mut huf1: cblock_t = cblock_t {
        data: 0 as *mut byte,
        count: 0,
    };
    let mut start: libc::c_int = 0;
    let mut end: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    r = fread(
        &mut command as *mut libc::c_int as *mut libc::c_void,
        4 as libc::c_int as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        cl.cinematic_file,
    ) as libc::c_int;
    if r == 0 as libc::c_int {
        r = fread(
            &mut command as *mut libc::c_int as *mut libc::c_void,
            4 as libc::c_int as libc::c_ulong,
            1 as libc::c_int as libc::c_ulong,
            cl.cinematic_file,
        ) as libc::c_int;
    }
    if r != 1 as libc::c_int {
        return 0 as *mut byte;
    }
    command = LittleLong(command);
    if command == 2 as libc::c_int {
        return 0 as *mut byte;
    }
    if command == 1 as libc::c_int {
        FS_Read(
            (cl.cinematicpalette).as_mut_ptr() as *mut libc::c_void,
            ::std::mem::size_of::<[libc::c_char; 768]>() as libc::c_ulong as libc::c_int,
            cl.cinematic_file,
        );
        cl.cinematicpalette_active = false_0;
    }
    FS_Read(
        &mut size as *mut libc::c_int as *mut libc::c_void,
        4 as libc::c_int,
        cl.cinematic_file,
    );
    size = LittleLong(size);
    if size as libc::c_ulong > ::std::mem::size_of::<[byte; 131072]>() as libc::c_ulong
        || size < 1 as libc::c_int
    {
        Com_Error(
            1 as libc::c_int,
            b"Bad compressed frame size\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    FS_Read(compressed.as_mut_ptr() as *mut libc::c_void, size, cl.cinematic_file);
    start = cl.cinematicframe * cin.s_rate / 14 as libc::c_int;
    end = (cl.cinematicframe + 1 as libc::c_int) * cin.s_rate / 14 as libc::c_int;
    count = end - start;
    FS_Read(
        samples.as_mut_ptr() as *mut libc::c_void,
        count * cin.s_width * cin.s_channels,
        cl.cinematic_file,
    );
    S_RawSamples(count, cin.s_rate, cin.s_width, cin.s_channels, samples.as_mut_ptr());
    in_0.data = compressed.as_mut_ptr();
    in_0.count = size;
    huf1 = Huff1Decompress(in_0);
    pic = huf1.data;
    cl.cinematicframe += 1;
    return pic;
}

#[no_mangle]
pub unsafe extern "C" fn SCR_RunCinematic() {
    let mut frame: libc::c_int = 0;
    if cl.cinematictime <= 0 as libc::c_int {
        SCR_StopCinematic();
        return;
    }
    if cl.cinematicframe == -(1 as libc::c_int) {
        return;
    }
    if cls.key_dest as libc::c_uint != key_game as libc::c_int as libc::c_uint {
        cl
            .cinematictime = cls.realtime
            - cl.cinematicframe * 1000 as libc::c_int / 14 as libc::c_int;
        return;
    }
    frame = ((cls.realtime - cl.cinematictime) as libc::c_double * 14.0f64
        / 1000 as libc::c_int as libc::c_double) as libc::c_int;
    if frame <= cl.cinematicframe {
        return;
    }
    if frame > cl.cinematicframe + 1 as libc::c_int {
        Com_Printf(
            b"Dropped frame: %i > %i\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            frame,
            cl.cinematicframe + 1 as libc::c_int,
        );
        cl
            .cinematictime = cls.realtime
            - cl.cinematicframe * 1000 as libc::c_int / 14 as libc::c_int;
    }
    if !(cin.pic).is_null() {
        Z_Free(cin.pic as *mut libc::c_void);
    }
    cin.pic = cin.pic_pending;
    cin.pic_pending = 0 as *mut byte;
    cin.pic_pending = SCR_ReadNextFrame();
    if (cin.pic_pending).is_null() {
        SCR_StopCinematic();
        SCR_FinishCinematic();
        cl.cinematictime = 1 as libc::c_int;
        SCR_BeginLoadingPlaque();
        cl.cinematictime = 0 as libc::c_int;
        return;
    }
}

#[no_mangle]
pub unsafe extern "C" fn SCR_DrawCinematic() -> qboolean {
    if cl.cinematictime <= 0 as libc::c_int {
        return false_0;
    }
    if cls.key_dest as libc::c_uint == key_menu as libc::c_int as libc::c_uint {
        (re.CinematicSetPalette)
            .expect("non-null function pointer")(0 as *const libc::c_uchar);
        cl.cinematicpalette_active = false_0;
        return true_0;
    }
    if cl.cinematicpalette_active as u64 == 0 {
        (re.CinematicSetPalette)
            .expect(
                "non-null function pointer",
            )((cl.cinematicpalette).as_mut_ptr() as *const libc::c_uchar);
        cl.cinematicpalette_active = true_0;
    }
    if (cin.pic).is_null() {
        return true_0;
    }
    (re.DrawStretchRaw)
        .expect(
            "non-null function pointer",
        )(
        0 as libc::c_int,
        0 as libc::c_int,
        viddef.width,
        viddef.height,
        cin.width,
        cin.height,
        cin.pic,
    );
    return true_0;
}

#[no_mangle]
pub unsafe extern "C" fn SCR_PlayCinematic(mut arg: *mut libc::c_char) {
    let mut width: libc::c_int = 0;
    let mut height: libc::c_int = 0;
    let mut palette: *mut byte = 0 as *mut byte;
    let mut name: [libc::c_char; 128] = [0; 128];
    let mut dot: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut old_khz: libc::c_int = 0;
    CDAudio_Stop();
    cl.cinematicframe = 0 as libc::c_int;
    dot = strstr(arg, b".\0" as *const u8 as *const libc::c_char);
    if !dot.is_null() && strcmp(dot, b".pcx\0" as *const u8 as *const libc::c_char) == 0
    {
        Com_sprintf(
            name.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong as libc::c_int,
            b"pics/%s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            arg,
        );
        SCR_LoadPCX(
            name.as_mut_ptr(),
            &mut cin.pic,
            &mut palette,
            &mut cin.width,
            &mut cin.height,
        );
        cl.cinematicframe = -(1 as libc::c_int);
        cl.cinematictime = 1 as libc::c_int;
        SCR_EndLoadingPlaque();
        cls.state = ca_active;
        if (cin.pic).is_null() {
            Com_Printf(
                b"%s not found.\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                name.as_mut_ptr(),
            );
            cl.cinematictime = 0 as libc::c_int;
        } else {
            memcpy(
                (cl.cinematicpalette).as_mut_ptr() as *mut libc::c_void,
                palette as *const libc::c_void,
                ::std::mem::size_of::<[libc::c_char; 768]>() as libc::c_ulong,
            );
            Z_Free(palette as *mut libc::c_void);
        }
        return;
    }
    Com_sprintf(
        name.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong as libc::c_int,
        b"video/%s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        arg,
    );
    FS_FOpenFile(name.as_mut_ptr(), &mut cl.cinematic_file);
    if (cl.cinematic_file).is_null() {
        SCR_FinishCinematic();
        cl.cinematictime = 0 as libc::c_int;
        return;
    }
    SCR_EndLoadingPlaque();
    cls.state = ca_active;
    FS_Read(
        &mut width as *mut libc::c_int as *mut libc::c_void,
        4 as libc::c_int,
        cl.cinematic_file,
    );
    FS_Read(
        &mut height as *mut libc::c_int as *mut libc::c_void,
        4 as libc::c_int,
        cl.cinematic_file,
    );
    cin.width = LittleLong(width);
    cin.height = LittleLong(height);
    FS_Read(
        &mut cin.s_rate as *mut libc::c_int as *mut libc::c_void,
        4 as libc::c_int,
        cl.cinematic_file,
    );
    cin.s_rate = LittleLong(cin.s_rate);
    FS_Read(
        &mut cin.s_width as *mut libc::c_int as *mut libc::c_void,
        4 as libc::c_int,
        cl.cinematic_file,
    );
    cin.s_width = LittleLong(cin.s_width);
    FS_Read(
        &mut cin.s_channels as *mut libc::c_int as *mut libc::c_void,
        4 as libc::c_int,
        cl.cinematic_file,
    );
    cin.s_channels = LittleLong(cin.s_channels);
    Huff1TableInit();
    old_khz = Cvar_VariableValue(
        b"s_khz\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as libc::c_int;
    if old_khz != cin.s_rate / 1000 as libc::c_int {
        cin.restart_sound = true_0;
        Cvar_SetValue(
            b"s_khz\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            (cin.s_rate / 1000 as libc::c_int) as libc::c_float,
        );
        CL_Snd_Restart_f();
        Cvar_SetValue(
            b"s_khz\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            old_khz as libc::c_float,
        );
    }
    cl.cinematicframe = 0 as libc::c_int;
    cin.pic = SCR_ReadNextFrame();
    cl.cinematictime = Sys_Milliseconds();
}
