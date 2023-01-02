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
    fn acos(_: libc::c_double) -> libc::c_double;
    fn atan2(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn ceil(_: libc::c_double) -> libc::c_double;
    fn floor(_: libc::c_double) -> libc::c_double;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn rand() -> libc::c_int;
    static mut vec3_origin: vec3_t;
    fn VectorMA(
        veca: *mut vec_t,
        scale: libc::c_float,
        vecb: *mut vec_t,
        vecc: *mut vec_t,
    );
    fn VectorCompare(v1: *mut vec_t, v2: *mut vec_t) -> libc::c_int;
    fn VectorLength(v: *mut vec_t) -> vec_t;
    fn VectorNormalize(v: *mut vec_t) -> vec_t;
    fn VectorScale(in_0: *mut vec_t, scale: vec_t, out: *mut vec_t);
    fn AngleVectors(
        angles: *mut vec_t,
        forward: *mut vec_t,
        right: *mut vec_t,
        up: *mut vec_t,
    );
    fn Com_sprintf(
        dest: *mut libc::c_char,
        size: libc::c_int,
        fmt: *mut libc::c_char,
        _: ...
    );
    fn Com_Printf(msg: *mut libc::c_char, _: ...);
    fn MSG_ReadByte(sb: *mut sizebuf_t) -> libc::c_int;
    fn MSG_ReadShort(sb: *mut sizebuf_t) -> libc::c_int;
    fn MSG_ReadLong(sb: *mut sizebuf_t) -> libc::c_int;
    fn MSG_ReadPos(sb: *mut sizebuf_t, pos: *mut vec_t);
    fn MSG_ReadDir(sb: *mut sizebuf_t, vector: *mut vec_t);
    static mut net_message: sizebuf_t;
    fn Com_Error(code: libc::c_int, fmt: *mut libc::c_char, _: ...);
    fn frand() -> libc::c_float;
    fn S_StartSound(
        origin: *mut vec_t,
        entnum: libc::c_int,
        entchannel: libc::c_int,
        sfx: *mut sfx_s,
        fvol: libc::c_float,
        attenuation: libc::c_float,
        timeofs: libc::c_float,
    );
    fn S_RegisterSound(sample: *mut libc::c_char) -> *mut sfx_s;
    static mut cl: client_state_t;
    fn CL_ParticleSteamEffect2(self_0: *mut cl_sustain_t);
    fn CL_ParticleEffect(
        org: *mut vec_t,
        dir: *mut vec_t,
        color: libc::c_int,
        count: libc::c_int,
    );
    fn CL_ParticleEffect2(
        org: *mut vec_t,
        dir: *mut vec_t,
        color: libc::c_int,
        count: libc::c_int,
    );
    fn CL_ParticleEffect3(
        org: *mut vec_t,
        dir: *mut vec_t,
        color: libc::c_int,
        count: libc::c_int,
    );
    fn CL_RailTrail(start: *mut vec_t, end: *mut vec_t);
    fn CL_BubbleTrail(start: *mut vec_t, end: *mut vec_t);
    fn CL_BlasterParticles2(org: *mut vec_t, dir: *mut vec_t, color: libc::c_uint);
    fn CL_DebugTrail(start: *mut vec_t, end: *mut vec_t);
    fn CL_Flashlight(ent: libc::c_int, pos: *mut vec_t);
    fn CL_ForceWall(start: *mut vec_t, end: *mut vec_t, color: libc::c_int);
    fn CL_BubbleTrail2(start: *mut vec_t, end: *mut vec_t, dist: libc::c_int);
    fn CL_Heatbeam(start: *mut vec_t, end: *mut vec_t);
    fn CL_ParticleSteamEffect(
        org: *mut vec_t,
        dir: *mut vec_t,
        color: libc::c_int,
        count: libc::c_int,
        magnitude: libc::c_int,
    );
    fn CL_ColorFlash(
        pos: *mut vec_t,
        ent: libc::c_int,
        intensity: libc::c_int,
        r: libc::c_float,
        g: libc::c_float,
        b: libc::c_float,
    );
    fn CL_MonsterPlasma_Shell(origin: *mut vec_t);
    fn CL_ColorExplosionParticles(org: *mut vec_t, color: libc::c_int, run: libc::c_int);
    fn CL_ParticleSmokeEffect(
        org: *mut vec_t,
        dir: *mut vec_t,
        color: libc::c_int,
        count: libc::c_int,
        magnitude: libc::c_int,
    );
    fn CL_Widowbeamout(self_0: *mut cl_sustain_t);
    fn CL_Nukeblast(self_0: *mut cl_sustain_t);
    fn CL_WidowSplash(org: *mut vec_t);
    fn CL_BigTeleportParticles(org: *mut vec_t);
    fn V_AddEntity(ent: *mut entity_t);
    fn V_AddLight(
        org: *mut vec_t,
        intensity: libc::c_float,
        r: libc::c_float,
        g: libc::c_float,
        b: libc::c_float,
    );
    static mut re: refexport_t;
    fn CL_TeleportParticles(org: *mut vec_t);
    fn CL_BlasterParticles(org: *mut vec_t, dir: *mut vec_t);
    fn CL_ExplosionParticles(org: *mut vec_t);
    fn CL_BFGExplosionParticles(org: *mut vec_t);
    static mut hand: *mut cvar_t;
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
pub type C2RustUnnamed = libc::c_uint;

pub const TE_FLECHETTE: C2RustUnnamed = 55;
pub const TE_EXPLOSION1_NP: C2RustUnnamed = 54;
pub const TE_EXPLOSION1_BIG: C2RustUnnamed = 53;
pub const TE_WIDOWSPLASH: C2RustUnnamed = 52;
pub const TE_NUKEBLAST: C2RustUnnamed = 51;
pub const TE_WIDOWBEAMOUT: C2RustUnnamed = 50;
pub const TE_DBALL_GOAL: C2RustUnnamed = 49;
pub const TE_TELEPORT_EFFECT: C2RustUnnamed = 48;
pub const TE_TRACKER_EXPLOSION: C2RustUnnamed = 47;
pub const TE_ELECTRIC_SPARKS: C2RustUnnamed = 46;
pub const TE_CHAINFIST_SMOKE: C2RustUnnamed = 45;
pub const TE_HEATBEAM_STEAM: C2RustUnnamed = 44;
pub const TE_HEATBEAM_SPARKS: C2RustUnnamed = 43;
pub const TE_MOREBLOOD: C2RustUnnamed = 42;
pub const TE_BUBBLETRAIL2: C2RustUnnamed = 41;
pub const TE_STEAM: C2RustUnnamed = 40;
pub const TE_MONSTER_HEATBEAM: C2RustUnnamed = 39;
pub const TE_HEATBEAM: C2RustUnnamed = 38;
pub const TE_FORCEWALL: C2RustUnnamed = 37;
pub const TE_FLASHLIGHT: C2RustUnnamed = 36;
pub const TE_PLAIN_EXPLOSION: C2RustUnnamed = 35;
pub const TE_DEBUGTRAIL: C2RustUnnamed = 34;
pub const TE_LIGHTNING: C2RustUnnamed = 33;
pub const TE_FLAME: C2RustUnnamed = 32;
pub const TE_RAILTRAIL2: C2RustUnnamed = 31;
pub const TE_BLASTER2: C2RustUnnamed = 30;
pub const TE_TUNNEL_SPARKS: C2RustUnnamed = 29;
pub const TE_PLASMA_EXPLOSION: C2RustUnnamed = 28;
pub const TE_BLUEHYPERBLASTER: C2RustUnnamed = 27;
pub const TE_GREENBLOOD: C2RustUnnamed = 26;
pub const TE_WELDING_SPARKS: C2RustUnnamed = 25;
pub const TE_GRAPPLE_CABLE: C2RustUnnamed = 24;
pub const TE_BFG_LASER: C2RustUnnamed = 23;
pub const TE_BOSSTPORT: C2RustUnnamed = 22;
pub const TE_BFG_BIGEXPLOSION: C2RustUnnamed = 21;
pub const TE_BFG_EXPLOSION: C2RustUnnamed = 20;
pub const TE_MEDIC_CABLE_ATTACK: C2RustUnnamed = 19;
pub const TE_GRENADE_EXPLOSION_WATER: C2RustUnnamed = 18;
pub const TE_ROCKET_EXPLOSION_WATER: C2RustUnnamed = 17;
pub const TE_PARASITE_ATTACK: C2RustUnnamed = 16;
pub const TE_LASER_SPARKS: C2RustUnnamed = 15;
pub const TE_BULLET_SPARKS: C2RustUnnamed = 14;
pub const TE_SHIELD_SPARKS: C2RustUnnamed = 13;
pub const TE_SCREEN_SPARKS: C2RustUnnamed = 12;
pub const TE_BUBBLETRAIL: C2RustUnnamed = 11;
pub const TE_SPLASH: C2RustUnnamed = 10;
pub const TE_SPARKS: C2RustUnnamed = 9;
pub const TE_GRENADE_EXPLOSION: C2RustUnnamed = 8;
pub const TE_ROCKET_EXPLOSION: C2RustUnnamed = 7;
pub const TE_EXPLOSION2: C2RustUnnamed = 6;
pub const TE_EXPLOSION1: C2RustUnnamed = 5;
pub const TE_SHOTGUN: C2RustUnnamed = 4;
pub const TE_RAILTRAIL: C2RustUnnamed = 3;
pub const TE_BLASTER: C2RustUnnamed = 2;
pub const TE_BLOOD: C2RustUnnamed = 1;
pub const TE_GUNSHOT: C2RustUnnamed = 0;

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

#[derive(Copy, Clone)]
#[repr(C)]
pub struct cl_sustain {
    pub id: libc::c_int,
    pub type_0: libc::c_int,
    pub endtime: libc::c_int,
    pub nextthink: libc::c_int,
    pub thinkinterval: libc::c_int,
    pub org: vec3_t,
    pub dir: vec3_t,
    pub color: libc::c_int,
    pub count: libc::c_int,
    pub magnitude: libc::c_int,
    pub think: Option::<unsafe extern "C" fn(*mut cl_sustain) -> ()>,
}

pub type cl_sustain_t = cl_sustain;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct beam_t {
    pub entity: libc::c_int,
    pub dest_entity: libc::c_int,
    pub model: *mut model_s,
    pub endtime: libc::c_int,
    pub offset: vec3_t,
    pub start: vec3_t,
    pub end: vec3_t,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct laser_t {
    pub ent: entity_t,
    pub endtime: libc::c_int,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct explosion_t {
    pub type_0: exptype_t,
    pub ent: entity_t,
    pub frames: libc::c_int,
    pub light: libc::c_float,
    pub lightcolor: vec3_t,
    pub start: libc::c_float,
    pub baseframe: libc::c_int,
}

pub type exptype_t = libc::c_uint;

pub const ex_poly2: exptype_t = 6;
pub const ex_poly: exptype_t = 5;
pub const ex_mflash: exptype_t = 4;
pub const ex_flash: exptype_t = 3;
pub const ex_misc: exptype_t = 2;
pub const ex_explosion: exptype_t = 1;
pub const ex_free: exptype_t = 0;
#[no_mangle]
pub static mut cl_explosions: [explosion_t; 32] = [explosion_t {
    type_0: ex_free,
    ent: entity_t {
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
    },
    frames: 0,
    light: 0.,
    lightcolor: [0.; 3],
    start: 0.,
    baseframe: 0,
}; 32];
#[no_mangle]
pub static mut cl_beams: [beam_t; 32] = [beam_t {
    entity: 0,
    dest_entity: 0,
    model: 0 as *const model_s as *mut model_s,
    endtime: 0,
    offset: [0.; 3],
    start: [0.; 3],
    end: [0.; 3],
}; 32];
#[no_mangle]
pub static mut cl_playerbeams: [beam_t; 32] = [beam_t {
    entity: 0,
    dest_entity: 0,
    model: 0 as *const model_s as *mut model_s,
    endtime: 0,
    offset: [0.; 3],
    start: [0.; 3],
    end: [0.; 3],
}; 32];
#[no_mangle]
pub static mut cl_lasers: [laser_t; 32] = [laser_t {
    ent: entity_t {
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
    },
    endtime: 0,
}; 32];
#[no_mangle]
pub static mut cl_sustains: [cl_sustain_t; 32] = [cl_sustain_t {
    id: 0,
    type_0: 0,
    endtime: 0,
    nextthink: 0,
    thinkinterval: 0,
    org: [0.; 3],
    dir: [0.; 3],
    color: 0,
    count: 0,
    magnitude: 0,
    think: None,
}; 32];
#[no_mangle]
pub static mut cl_sfx_ric1: *mut sfx_s = 0 as *const sfx_s as *mut sfx_s;
#[no_mangle]
pub static mut cl_sfx_ric2: *mut sfx_s = 0 as *const sfx_s as *mut sfx_s;
#[no_mangle]
pub static mut cl_sfx_ric3: *mut sfx_s = 0 as *const sfx_s as *mut sfx_s;
#[no_mangle]
pub static mut cl_sfx_lashit: *mut sfx_s = 0 as *const sfx_s as *mut sfx_s;
#[no_mangle]
pub static mut cl_sfx_spark5: *mut sfx_s = 0 as *const sfx_s as *mut sfx_s;
#[no_mangle]
pub static mut cl_sfx_spark6: *mut sfx_s = 0 as *const sfx_s as *mut sfx_s;
#[no_mangle]
pub static mut cl_sfx_spark7: *mut sfx_s = 0 as *const sfx_s as *mut sfx_s;
#[no_mangle]
pub static mut cl_sfx_railg: *mut sfx_s = 0 as *const sfx_s as *mut sfx_s;
#[no_mangle]
pub static mut cl_sfx_rockexp: *mut sfx_s = 0 as *const sfx_s as *mut sfx_s;
#[no_mangle]
pub static mut cl_sfx_grenexp: *mut sfx_s = 0 as *const sfx_s as *mut sfx_s;
#[no_mangle]
pub static mut cl_sfx_watrexp: *mut sfx_s = 0 as *const sfx_s as *mut sfx_s;
#[no_mangle]
pub static mut cl_sfx_plasexp: *mut sfx_s = 0 as *const sfx_s as *mut sfx_s;
#[no_mangle]
pub static mut cl_sfx_footsteps: [*mut sfx_s; 4] = [0 as *const sfx_s as *mut sfx_s; 4];
#[no_mangle]
pub static mut cl_mod_explode: *mut model_s = 0 as *const model_s as *mut model_s;
#[no_mangle]
pub static mut cl_mod_smoke: *mut model_s = 0 as *const model_s as *mut model_s;
#[no_mangle]
pub static mut cl_mod_flash: *mut model_s = 0 as *const model_s as *mut model_s;
#[no_mangle]
pub static mut cl_mod_parasite_segment: *mut model_s = 0 as *const model_s
    as *mut model_s;
#[no_mangle]
pub static mut cl_mod_grapple_cable: *mut model_s = 0 as *const model_s as *mut model_s;
#[no_mangle]
pub static mut cl_mod_parasite_tip: *mut model_s = 0 as *const model_s as *mut model_s;
#[no_mangle]
pub static mut cl_mod_explo4: *mut model_s = 0 as *const model_s as *mut model_s;
#[no_mangle]
pub static mut cl_mod_bfg_explo: *mut model_s = 0 as *const model_s as *mut model_s;
#[no_mangle]
pub static mut cl_mod_powerscreen: *mut model_s = 0 as *const model_s as *mut model_s;
#[no_mangle]
pub static mut cl_mod_plasmaexplo: *mut model_s = 0 as *const model_s as *mut model_s;
#[no_mangle]
pub static mut cl_sfx_lightning: *mut sfx_s = 0 as *const sfx_s as *mut sfx_s;
#[no_mangle]
pub static mut cl_sfx_disrexp: *mut sfx_s = 0 as *const sfx_s as *mut sfx_s;
#[no_mangle]
pub static mut cl_mod_lightning: *mut model_s = 0 as *const model_s as *mut model_s;
#[no_mangle]
pub static mut cl_mod_heatbeam: *mut model_s = 0 as *const model_s as *mut model_s;
#[no_mangle]
pub static mut cl_mod_monster_heatbeam: *mut model_s = 0 as *const model_s
    as *mut model_s;
#[no_mangle]
pub static mut cl_mod_explo4_big: *mut model_s = 0 as *const model_s as *mut model_s;

#[no_mangle]
pub unsafe extern "C" fn CL_RegisterTEntSounds() {
    let mut i: libc::c_int = 0;
    let mut name: [libc::c_char; 64] = [0; 64];
    cl_sfx_ric1 = S_RegisterSound(
        b"world/ric1.wav\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    cl_sfx_ric2 = S_RegisterSound(
        b"world/ric2.wav\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    cl_sfx_ric3 = S_RegisterSound(
        b"world/ric3.wav\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    cl_sfx_lashit = S_RegisterSound(
        b"weapons/lashit.wav\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    cl_sfx_spark5 = S_RegisterSound(
        b"world/spark5.wav\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    cl_sfx_spark6 = S_RegisterSound(
        b"world/spark6.wav\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    cl_sfx_spark7 = S_RegisterSound(
        b"world/spark7.wav\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    cl_sfx_railg = S_RegisterSound(
        b"weapons/railgf1a.wav\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    cl_sfx_rockexp = S_RegisterSound(
        b"weapons/rocklx1a.wav\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    cl_sfx_grenexp = S_RegisterSound(
        b"weapons/grenlx1a.wav\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    cl_sfx_watrexp = S_RegisterSound(
        b"weapons/xpld_wat.wav\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    S_RegisterSound(
        b"player/land1.wav\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    S_RegisterSound(
        b"player/fall2.wav\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    S_RegisterSound(
        b"player/fall1.wav\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        Com_sprintf(
            name.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
            b"player/step%i.wav\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            i + 1 as libc::c_int,
        );
        cl_sfx_footsteps[i as usize] = S_RegisterSound(name.as_mut_ptr());
        i += 1;
    }
    cl_sfx_lightning = S_RegisterSound(
        b"weapons/tesla.wav\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    cl_sfx_disrexp = S_RegisterSound(
        b"weapons/disrupthit.wav\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    sprintf(
        name.as_mut_ptr(),
        b"weapons/sound%d.wav\0" as *const u8 as *const libc::c_char,
        1278 as libc::c_int,
    );
    if name[0 as libc::c_int as usize] as libc::c_int == 'w' as i32 {
        name[0 as libc::c_int as usize] = 'W' as i32 as libc::c_char;
    }
}

#[no_mangle]
pub unsafe extern "C" fn CL_RegisterTEntModels() {
    cl_mod_explode = (re.RegisterModel)
        .expect(
            "non-null function pointer",
        )(
        b"models/objects/explode/tris.md2\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    cl_mod_smoke = (re.RegisterModel)
        .expect(
            "non-null function pointer",
        )(
        b"models/objects/smoke/tris.md2\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    cl_mod_flash = (re.RegisterModel)
        .expect(
            "non-null function pointer",
        )(
        b"models/objects/flash/tris.md2\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    cl_mod_parasite_segment = (re.RegisterModel)
        .expect(
            "non-null function pointer",
        )(
        b"models/monsters/parasite/segment/tris.md2\0" as *const u8
            as *const libc::c_char as *mut libc::c_char,
    );
    cl_mod_grapple_cable = (re.RegisterModel)
        .expect(
            "non-null function pointer",
        )(
        b"models/ctf/segment/tris.md2\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    cl_mod_parasite_tip = (re.RegisterModel)
        .expect(
            "non-null function pointer",
        )(
        b"models/monsters/parasite/tip/tris.md2\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    cl_mod_explo4 = (re.RegisterModel)
        .expect(
            "non-null function pointer",
        )(
        b"models/objects/r_explode/tris.md2\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    cl_mod_bfg_explo = (re.RegisterModel)
        .expect(
            "non-null function pointer",
        )(
        b"sprites/s_bfg2.sp2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    cl_mod_powerscreen = (re.RegisterModel)
        .expect(
            "non-null function pointer",
        )(
        b"models/items/armor/effect/tris.md2\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    (re.RegisterModel)
        .expect(
            "non-null function pointer",
        )(
        b"models/objects/laser/tris.md2\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    (re.RegisterModel)
        .expect(
            "non-null function pointer",
        )(
        b"models/objects/grenade2/tris.md2\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    (re.RegisterModel)
        .expect(
            "non-null function pointer",
        )(
        b"models/weapons/v_machn/tris.md2\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    (re.RegisterModel)
        .expect(
            "non-null function pointer",
        )(
        b"models/weapons/v_handgr/tris.md2\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    (re.RegisterModel)
        .expect(
            "non-null function pointer",
        )(
        b"models/weapons/v_shotg2/tris.md2\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    (re.RegisterModel)
        .expect(
            "non-null function pointer",
        )(
        b"models/objects/gibs/bone/tris.md2\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    (re.RegisterModel)
        .expect(
            "non-null function pointer",
        )(
        b"models/objects/gibs/sm_meat/tris.md2\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    (re.RegisterModel)
        .expect(
            "non-null function pointer",
        )(
        b"models/objects/gibs/bone2/tris.md2\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    (re.RegisterPic)
        .expect(
            "non-null function pointer",
        )(b"w_machinegun\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    (re.RegisterPic)
        .expect(
            "non-null function pointer",
        )(b"a_bullets\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    (re.RegisterPic)
        .expect(
            "non-null function pointer",
        )(b"i_health\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    (re.RegisterPic)
        .expect(
            "non-null function pointer",
        )(b"a_grenades\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    cl_mod_explo4_big = (re.RegisterModel)
        .expect(
            "non-null function pointer",
        )(
        b"models/objects/r_explode2/tris.md2\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    cl_mod_lightning = (re.RegisterModel)
        .expect(
            "non-null function pointer",
        )(
        b"models/proj/lightning/tris.md2\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    cl_mod_heatbeam = (re.RegisterModel)
        .expect(
            "non-null function pointer",
        )(
        b"models/proj/beam/tris.md2\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    cl_mod_monster_heatbeam = (re.RegisterModel)
        .expect(
            "non-null function pointer",
        )(
        b"models/proj/widowbeam/tris.md2\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
}

#[no_mangle]
pub unsafe extern "C" fn CL_ClearTEnts() {
    memset(
        cl_beams.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<[beam_t; 32]>() as libc::c_ulong,
    );
    memset(
        cl_explosions.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<[explosion_t; 32]>() as libc::c_ulong,
    );
    memset(
        cl_lasers.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<[laser_t; 32]>() as libc::c_ulong,
    );
    memset(
        cl_playerbeams.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<[beam_t; 32]>() as libc::c_ulong,
    );
    memset(
        cl_sustains.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<[cl_sustain_t; 32]>() as libc::c_ulong,
    );
}

#[no_mangle]
pub unsafe extern "C" fn CL_AllocExplosion() -> *mut explosion_t {
    let mut i: libc::c_int = 0;
    let mut time: libc::c_int = 0;
    let mut index: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        if cl_explosions[i as usize].type_0 as libc::c_uint
            == ex_free as libc::c_int as libc::c_uint
        {
            memset(
                &mut *cl_explosions.as_mut_ptr().offset(i as isize) as *mut explosion_t
                    as *mut libc::c_void,
                0 as libc::c_int,
                ::std::mem::size_of::<explosion_t>() as libc::c_ulong,
            );
            return &mut *cl_explosions.as_mut_ptr().offset(i as isize)
                as *mut explosion_t;
        }
        i += 1;
    }
    time = cl.time;
    index = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        if cl_explosions[i as usize].start < time as libc::c_float {
            time = cl_explosions[i as usize].start as libc::c_int;
            index = i;
        }
        i += 1;
    }
    memset(
        &mut *cl_explosions.as_mut_ptr().offset(index as isize) as *mut explosion_t
            as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<explosion_t>() as libc::c_ulong,
    );
    return &mut *cl_explosions.as_mut_ptr().offset(index as isize) as *mut explosion_t;
}

#[no_mangle]
pub unsafe extern "C" fn CL_SmokeAndFlash(mut origin: *mut vec_t) {
    let mut ex: *mut explosion_t = 0 as *mut explosion_t;
    ex = CL_AllocExplosion();
    (*ex)
        .ent
        .origin[0 as libc::c_int as usize] = *origin.offset(0 as libc::c_int as isize);
    (*ex)
        .ent
        .origin[1 as libc::c_int as usize] = *origin.offset(1 as libc::c_int as isize);
    (*ex)
        .ent
        .origin[2 as libc::c_int as usize] = *origin.offset(2 as libc::c_int as isize);
    (*ex).type_0 = ex_misc;
    (*ex).frames = 4 as libc::c_int;
    (*ex).ent.flags = 32 as libc::c_int;
    (*ex).start = (cl.frame.servertime - 100 as libc::c_int) as libc::c_float;
    let ref mut fresh0 = (*ex).ent.model;
    *fresh0 = cl_mod_smoke;
    ex = CL_AllocExplosion();
    (*ex)
        .ent
        .origin[0 as libc::c_int as usize] = *origin.offset(0 as libc::c_int as isize);
    (*ex)
        .ent
        .origin[1 as libc::c_int as usize] = *origin.offset(1 as libc::c_int as isize);
    (*ex)
        .ent
        .origin[2 as libc::c_int as usize] = *origin.offset(2 as libc::c_int as isize);
    (*ex).type_0 = ex_flash;
    (*ex).ent.flags = 8 as libc::c_int;
    (*ex).frames = 2 as libc::c_int;
    (*ex).start = (cl.frame.servertime - 100 as libc::c_int) as libc::c_float;
    let ref mut fresh1 = (*ex).ent.model;
    *fresh1 = cl_mod_flash;
}

#[no_mangle]
pub unsafe extern "C" fn CL_ParseParticles() {
    let mut color: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    let mut pos: vec3_t = [0.; 3];
    let mut dir: vec3_t = [0.; 3];
    MSG_ReadPos(&mut net_message, pos.as_mut_ptr());
    MSG_ReadDir(&mut net_message, dir.as_mut_ptr());
    color = MSG_ReadByte(&mut net_message);
    count = MSG_ReadByte(&mut net_message);
    CL_ParticleEffect(pos.as_mut_ptr(), dir.as_mut_ptr(), color, count);
}

#[no_mangle]
pub unsafe extern "C" fn CL_ParseBeam(mut model: *mut model_s) -> libc::c_int {
    let mut ent: libc::c_int = 0;
    let mut start: vec3_t = [0.; 3];
    let mut end: vec3_t = [0.; 3];
    let mut b: *mut beam_t = 0 as *mut beam_t;
    let mut i: libc::c_int = 0;
    ent = MSG_ReadShort(&mut net_message);
    MSG_ReadPos(&mut net_message, start.as_mut_ptr());
    MSG_ReadPos(&mut net_message, end.as_mut_ptr());
    i = 0 as libc::c_int;
    b = cl_beams.as_mut_ptr();
    while i < 32 as libc::c_int {
        if (*b).entity == ent {
            (*b).entity = ent;
            let ref mut fresh2 = (*b).model;
            *fresh2 = model;
            (*b).endtime = cl.time + 200 as libc::c_int;
            (*b).start[0 as libc::c_int as usize] = start[0 as libc::c_int as usize];
            (*b).start[1 as libc::c_int as usize] = start[1 as libc::c_int as usize];
            (*b).start[2 as libc::c_int as usize] = start[2 as libc::c_int as usize];
            (*b).end[0 as libc::c_int as usize] = end[0 as libc::c_int as usize];
            (*b).end[1 as libc::c_int as usize] = end[1 as libc::c_int as usize];
            (*b).end[2 as libc::c_int as usize] = end[2 as libc::c_int as usize];
            let ref mut fresh3 = (*b).offset[2 as libc::c_int as usize];
            *fresh3 = 0 as libc::c_int as vec_t;
            let ref mut fresh4 = (*b).offset[1 as libc::c_int as usize];
            *fresh4 = *fresh3;
            (*b).offset[0 as libc::c_int as usize] = *fresh4;
            return ent;
        }
        i += 1;
        b = b.offset(1);
    }
    i = 0 as libc::c_int;
    b = cl_beams.as_mut_ptr();
    while i < 32 as libc::c_int {
        if ((*b).model).is_null() || (*b).endtime < cl.time {
            (*b).entity = ent;
            let ref mut fresh5 = (*b).model;
            *fresh5 = model;
            (*b).endtime = cl.time + 200 as libc::c_int;
            (*b).start[0 as libc::c_int as usize] = start[0 as libc::c_int as usize];
            (*b).start[1 as libc::c_int as usize] = start[1 as libc::c_int as usize];
            (*b).start[2 as libc::c_int as usize] = start[2 as libc::c_int as usize];
            (*b).end[0 as libc::c_int as usize] = end[0 as libc::c_int as usize];
            (*b).end[1 as libc::c_int as usize] = end[1 as libc::c_int as usize];
            (*b).end[2 as libc::c_int as usize] = end[2 as libc::c_int as usize];
            let ref mut fresh6 = (*b).offset[2 as libc::c_int as usize];
            *fresh6 = 0 as libc::c_int as vec_t;
            let ref mut fresh7 = (*b).offset[1 as libc::c_int as usize];
            *fresh7 = *fresh6;
            (*b).offset[0 as libc::c_int as usize] = *fresh7;
            return ent;
        }
        i += 1;
        b = b.offset(1);
    }
    Com_Printf(
        b"beam list overflow!\n\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    return ent;
}

#[no_mangle]
pub unsafe extern "C" fn CL_ParseBeam2(mut model: *mut model_s) -> libc::c_int {
    let mut ent: libc::c_int = 0;
    let mut start: vec3_t = [0.; 3];
    let mut end: vec3_t = [0.; 3];
    let mut offset: vec3_t = [0.; 3];
    let mut b: *mut beam_t = 0 as *mut beam_t;
    let mut i: libc::c_int = 0;
    ent = MSG_ReadShort(&mut net_message);
    MSG_ReadPos(&mut net_message, start.as_mut_ptr());
    MSG_ReadPos(&mut net_message, end.as_mut_ptr());
    MSG_ReadPos(&mut net_message, offset.as_mut_ptr());
    i = 0 as libc::c_int;
    b = cl_beams.as_mut_ptr();
    while i < 32 as libc::c_int {
        if (*b).entity == ent {
            (*b).entity = ent;
            let ref mut fresh8 = (*b).model;
            *fresh8 = model;
            (*b).endtime = cl.time + 200 as libc::c_int;
            (*b).start[0 as libc::c_int as usize] = start[0 as libc::c_int as usize];
            (*b).start[1 as libc::c_int as usize] = start[1 as libc::c_int as usize];
            (*b).start[2 as libc::c_int as usize] = start[2 as libc::c_int as usize];
            (*b).end[0 as libc::c_int as usize] = end[0 as libc::c_int as usize];
            (*b).end[1 as libc::c_int as usize] = end[1 as libc::c_int as usize];
            (*b).end[2 as libc::c_int as usize] = end[2 as libc::c_int as usize];
            (*b).offset[0 as libc::c_int as usize] = offset[0 as libc::c_int as usize];
            (*b).offset[1 as libc::c_int as usize] = offset[1 as libc::c_int as usize];
            (*b).offset[2 as libc::c_int as usize] = offset[2 as libc::c_int as usize];
            return ent;
        }
        i += 1;
        b = b.offset(1);
    }
    i = 0 as libc::c_int;
    b = cl_beams.as_mut_ptr();
    while i < 32 as libc::c_int {
        if ((*b).model).is_null() || (*b).endtime < cl.time {
            (*b).entity = ent;
            let ref mut fresh9 = (*b).model;
            *fresh9 = model;
            (*b).endtime = cl.time + 200 as libc::c_int;
            (*b).start[0 as libc::c_int as usize] = start[0 as libc::c_int as usize];
            (*b).start[1 as libc::c_int as usize] = start[1 as libc::c_int as usize];
            (*b).start[2 as libc::c_int as usize] = start[2 as libc::c_int as usize];
            (*b).end[0 as libc::c_int as usize] = end[0 as libc::c_int as usize];
            (*b).end[1 as libc::c_int as usize] = end[1 as libc::c_int as usize];
            (*b).end[2 as libc::c_int as usize] = end[2 as libc::c_int as usize];
            (*b).offset[0 as libc::c_int as usize] = offset[0 as libc::c_int as usize];
            (*b).offset[1 as libc::c_int as usize] = offset[1 as libc::c_int as usize];
            (*b).offset[2 as libc::c_int as usize] = offset[2 as libc::c_int as usize];
            return ent;
        }
        i += 1;
        b = b.offset(1);
    }
    Com_Printf(
        b"beam list overflow!\n\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    return ent;
}

#[no_mangle]
pub unsafe extern "C" fn CL_ParsePlayerBeam(mut model: *mut model_s) -> libc::c_int {
    let mut ent: libc::c_int = 0;
    let mut start: vec3_t = [0.; 3];
    let mut end: vec3_t = [0.; 3];
    let mut offset: vec3_t = [0.; 3];
    let mut b: *mut beam_t = 0 as *mut beam_t;
    let mut i: libc::c_int = 0;
    ent = MSG_ReadShort(&mut net_message);
    MSG_ReadPos(&mut net_message, start.as_mut_ptr());
    MSG_ReadPos(&mut net_message, end.as_mut_ptr());
    if model == cl_mod_heatbeam {
        offset[0 as libc::c_int as usize] = 2 as libc::c_int as vec_t;
        offset[1 as libc::c_int as usize] = 7 as libc::c_int as vec_t;
        offset[2 as libc::c_int as usize] = -(3 as libc::c_int) as vec_t;
    } else if model == cl_mod_monster_heatbeam {
        model = cl_mod_heatbeam;
        offset[0 as libc::c_int as usize] = 0 as libc::c_int as vec_t;
        offset[1 as libc::c_int as usize] = 0 as libc::c_int as vec_t;
        offset[2 as libc::c_int as usize] = 0 as libc::c_int as vec_t;
    } else {
        MSG_ReadPos(&mut net_message, offset.as_mut_ptr());
    }
    i = 0 as libc::c_int;
    b = cl_playerbeams.as_mut_ptr();
    while i < 32 as libc::c_int {
        if (*b).entity == ent {
            (*b).entity = ent;
            let ref mut fresh10 = (*b).model;
            *fresh10 = model;
            (*b).endtime = cl.time + 200 as libc::c_int;
            (*b).start[0 as libc::c_int as usize] = start[0 as libc::c_int as usize];
            (*b).start[1 as libc::c_int as usize] = start[1 as libc::c_int as usize];
            (*b).start[2 as libc::c_int as usize] = start[2 as libc::c_int as usize];
            (*b).end[0 as libc::c_int as usize] = end[0 as libc::c_int as usize];
            (*b).end[1 as libc::c_int as usize] = end[1 as libc::c_int as usize];
            (*b).end[2 as libc::c_int as usize] = end[2 as libc::c_int as usize];
            (*b).offset[0 as libc::c_int as usize] = offset[0 as libc::c_int as usize];
            (*b).offset[1 as libc::c_int as usize] = offset[1 as libc::c_int as usize];
            (*b).offset[2 as libc::c_int as usize] = offset[2 as libc::c_int as usize];
            return ent;
        }
        i += 1;
        b = b.offset(1);
    }
    i = 0 as libc::c_int;
    b = cl_playerbeams.as_mut_ptr();
    while i < 32 as libc::c_int {
        if ((*b).model).is_null() || (*b).endtime < cl.time {
            (*b).entity = ent;
            let ref mut fresh11 = (*b).model;
            *fresh11 = model;
            (*b).endtime = cl.time + 100 as libc::c_int;
            (*b).start[0 as libc::c_int as usize] = start[0 as libc::c_int as usize];
            (*b).start[1 as libc::c_int as usize] = start[1 as libc::c_int as usize];
            (*b).start[2 as libc::c_int as usize] = start[2 as libc::c_int as usize];
            (*b).end[0 as libc::c_int as usize] = end[0 as libc::c_int as usize];
            (*b).end[1 as libc::c_int as usize] = end[1 as libc::c_int as usize];
            (*b).end[2 as libc::c_int as usize] = end[2 as libc::c_int as usize];
            (*b).offset[0 as libc::c_int as usize] = offset[0 as libc::c_int as usize];
            (*b).offset[1 as libc::c_int as usize] = offset[1 as libc::c_int as usize];
            (*b).offset[2 as libc::c_int as usize] = offset[2 as libc::c_int as usize];
            return ent;
        }
        i += 1;
        b = b.offset(1);
    }
    Com_Printf(
        b"beam list overflow!\n\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    return ent;
}

#[no_mangle]
pub unsafe extern "C" fn CL_ParseLightning(mut model: *mut model_s) -> libc::c_int {
    let mut srcEnt: libc::c_int = 0;
    let mut destEnt: libc::c_int = 0;
    let mut start: vec3_t = [0.; 3];
    let mut end: vec3_t = [0.; 3];
    let mut b: *mut beam_t = 0 as *mut beam_t;
    let mut i: libc::c_int = 0;
    srcEnt = MSG_ReadShort(&mut net_message);
    destEnt = MSG_ReadShort(&mut net_message);
    MSG_ReadPos(&mut net_message, start.as_mut_ptr());
    MSG_ReadPos(&mut net_message, end.as_mut_ptr());
    i = 0 as libc::c_int;
    b = cl_beams.as_mut_ptr();
    while i < 32 as libc::c_int {
        if (*b).entity == srcEnt && (*b).dest_entity == destEnt {
            (*b).entity = srcEnt;
            (*b).dest_entity = destEnt;
            let ref mut fresh12 = (*b).model;
            *fresh12 = model;
            (*b).endtime = cl.time + 200 as libc::c_int;
            (*b).start[0 as libc::c_int as usize] = start[0 as libc::c_int as usize];
            (*b).start[1 as libc::c_int as usize] = start[1 as libc::c_int as usize];
            (*b).start[2 as libc::c_int as usize] = start[2 as libc::c_int as usize];
            (*b).end[0 as libc::c_int as usize] = end[0 as libc::c_int as usize];
            (*b).end[1 as libc::c_int as usize] = end[1 as libc::c_int as usize];
            (*b).end[2 as libc::c_int as usize] = end[2 as libc::c_int as usize];
            let ref mut fresh13 = (*b).offset[2 as libc::c_int as usize];
            *fresh13 = 0 as libc::c_int as vec_t;
            let ref mut fresh14 = (*b).offset[1 as libc::c_int as usize];
            *fresh14 = *fresh13;
            (*b).offset[0 as libc::c_int as usize] = *fresh14;
            return srcEnt;
        }
        i += 1;
        b = b.offset(1);
    }
    i = 0 as libc::c_int;
    b = cl_beams.as_mut_ptr();
    while i < 32 as libc::c_int {
        if ((*b).model).is_null() || (*b).endtime < cl.time {
            (*b).entity = srcEnt;
            (*b).dest_entity = destEnt;
            let ref mut fresh15 = (*b).model;
            *fresh15 = model;
            (*b).endtime = cl.time + 200 as libc::c_int;
            (*b).start[0 as libc::c_int as usize] = start[0 as libc::c_int as usize];
            (*b).start[1 as libc::c_int as usize] = start[1 as libc::c_int as usize];
            (*b).start[2 as libc::c_int as usize] = start[2 as libc::c_int as usize];
            (*b).end[0 as libc::c_int as usize] = end[0 as libc::c_int as usize];
            (*b).end[1 as libc::c_int as usize] = end[1 as libc::c_int as usize];
            (*b).end[2 as libc::c_int as usize] = end[2 as libc::c_int as usize];
            let ref mut fresh16 = (*b).offset[2 as libc::c_int as usize];
            *fresh16 = 0 as libc::c_int as vec_t;
            let ref mut fresh17 = (*b).offset[1 as libc::c_int as usize];
            *fresh17 = *fresh16;
            (*b).offset[0 as libc::c_int as usize] = *fresh17;
            return srcEnt;
        }
        i += 1;
        b = b.offset(1);
    }
    Com_Printf(
        b"beam list overflow!\n\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    return srcEnt;
}

#[no_mangle]
pub unsafe extern "C" fn CL_ParseLaser(mut colors: libc::c_int) {
    let mut start: vec3_t = [0.; 3];
    let mut end: vec3_t = [0.; 3];
    let mut l: *mut laser_t = 0 as *mut laser_t;
    let mut i: libc::c_int = 0;
    MSG_ReadPos(&mut net_message, start.as_mut_ptr());
    MSG_ReadPos(&mut net_message, end.as_mut_ptr());
    i = 0 as libc::c_int;
    l = cl_lasers.as_mut_ptr();
    while i < 32 as libc::c_int {
        if (*l).endtime < cl.time {
            (*l).ent.flags = 32 as libc::c_int | 128 as libc::c_int;
            (*l)
                .ent
                .origin[0 as libc::c_int as usize] = start[0 as libc::c_int as usize];
            (*l)
                .ent
                .origin[1 as libc::c_int as usize] = start[1 as libc::c_int as usize];
            (*l)
                .ent
                .origin[2 as libc::c_int as usize] = start[2 as libc::c_int as usize];
            (*l)
                .ent
                .oldorigin[0 as libc::c_int as usize] = end[0 as libc::c_int as usize];
            (*l)
                .ent
                .oldorigin[1 as libc::c_int as usize] = end[1 as libc::c_int as usize];
            (*l)
                .ent
                .oldorigin[2 as libc::c_int as usize] = end[2 as libc::c_int as usize];
            (*l).ent.alpha = 0.30f64 as libc::c_float;
            (*l)
                .ent
                .skinnum = colors >> rand() % 4 as libc::c_int * 8 as libc::c_int
                & 0xff as libc::c_int;
            let ref mut fresh18 = (*l).ent.model;
            *fresh18 = 0 as *mut model_s;
            (*l).ent.frame = 4 as libc::c_int;
            (*l).endtime = cl.time + 100 as libc::c_int;
            return;
        }
        i += 1;
        l = l.offset(1);
    }
}

#[no_mangle]
pub unsafe extern "C" fn CL_ParseSteam() {
    let mut pos: vec3_t = [0.; 3];
    let mut dir: vec3_t = [0.; 3];
    let mut id: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    let mut cnt: libc::c_int = 0;
    let mut color: libc::c_int = 0;
    let mut magnitude: libc::c_int = 0;
    let mut s: *mut cl_sustain_t = 0 as *mut cl_sustain_t;
    let mut free_sustain: *mut cl_sustain_t = 0 as *mut cl_sustain_t;
    id = MSG_ReadShort(&mut net_message);
    if id != -(1 as libc::c_int) {
        free_sustain = 0 as *mut cl_sustain_t;
        i = 0 as libc::c_int;
        s = cl_sustains.as_mut_ptr();
        while i < 32 as libc::c_int {
            if (*s).id == 0 as libc::c_int {
                free_sustain = s;
                break;
            } else {
                i += 1;
                s = s.offset(1);
            }
        }
        if !free_sustain.is_null() {
            (*s).id = id;
            (*s).count = MSG_ReadByte(&mut net_message);
            MSG_ReadPos(&mut net_message, ((*s).org).as_mut_ptr());
            MSG_ReadDir(&mut net_message, ((*s).dir).as_mut_ptr());
            r = MSG_ReadByte(&mut net_message);
            (*s).color = r & 0xff as libc::c_int;
            (*s).magnitude = MSG_ReadShort(&mut net_message);
            (*s).endtime = cl.time + MSG_ReadLong(&mut net_message);
            let ref mut fresh19 = (*s).think;
            *fresh19 = Some(
                CL_ParticleSteamEffect2 as unsafe extern "C" fn(*mut cl_sustain_t) -> (),
            );
            (*s).thinkinterval = 100 as libc::c_int;
            (*s).nextthink = cl.time;
        } else {
            cnt = MSG_ReadByte(&mut net_message);
            MSG_ReadPos(&mut net_message, pos.as_mut_ptr());
            MSG_ReadDir(&mut net_message, dir.as_mut_ptr());
            r = MSG_ReadByte(&mut net_message);
            magnitude = MSG_ReadShort(&mut net_message);
            magnitude = MSG_ReadLong(&mut net_message);
        }
    } else {
        cnt = MSG_ReadByte(&mut net_message);
        MSG_ReadPos(&mut net_message, pos.as_mut_ptr());
        MSG_ReadDir(&mut net_message, dir.as_mut_ptr());
        r = MSG_ReadByte(&mut net_message);
        magnitude = MSG_ReadShort(&mut net_message);
        color = r & 0xff as libc::c_int;
        CL_ParticleSteamEffect(
            pos.as_mut_ptr(),
            dir.as_mut_ptr(),
            color,
            cnt,
            magnitude,
        );
    };
}

#[no_mangle]
pub unsafe extern "C" fn CL_ParseWidow() {
    let mut pos: vec3_t = [0.; 3];
    let mut id: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut s: *mut cl_sustain_t = 0 as *mut cl_sustain_t;
    let mut free_sustain: *mut cl_sustain_t = 0 as *mut cl_sustain_t;
    id = MSG_ReadShort(&mut net_message);
    free_sustain = 0 as *mut cl_sustain_t;
    i = 0 as libc::c_int;
    s = cl_sustains.as_mut_ptr();
    while i < 32 as libc::c_int {
        if (*s).id == 0 as libc::c_int {
            free_sustain = s;
            break;
        } else {
            i += 1;
            s = s.offset(1);
        }
    }
    if !free_sustain.is_null() {
        (*s).id = id;
        MSG_ReadPos(&mut net_message, ((*s).org).as_mut_ptr());
        (*s).endtime = cl.time + 2100 as libc::c_int;
        let ref mut fresh20 = (*s).think;
        *fresh20 = Some(
            CL_Widowbeamout as unsafe extern "C" fn(*mut cl_sustain_t) -> (),
        );
        (*s).thinkinterval = 1 as libc::c_int;
        (*s).nextthink = cl.time;
    } else {
        MSG_ReadPos(&mut net_message, pos.as_mut_ptr());
    };
}

#[no_mangle]
pub unsafe extern "C" fn CL_ParseNuke() {
    let mut pos: vec3_t = [0.; 3];
    let mut i: libc::c_int = 0;
    let mut s: *mut cl_sustain_t = 0 as *mut cl_sustain_t;
    let mut free_sustain: *mut cl_sustain_t = 0 as *mut cl_sustain_t;
    free_sustain = 0 as *mut cl_sustain_t;
    i = 0 as libc::c_int;
    s = cl_sustains.as_mut_ptr();
    while i < 32 as libc::c_int {
        if (*s).id == 0 as libc::c_int {
            free_sustain = s;
            break;
        } else {
            i += 1;
            s = s.offset(1);
        }
    }
    if !free_sustain.is_null() {
        (*s).id = 21000 as libc::c_int;
        MSG_ReadPos(&mut net_message, ((*s).org).as_mut_ptr());
        (*s).endtime = cl.time + 1000 as libc::c_int;
        let ref mut fresh21 = (*s).think;
        *fresh21 = Some(CL_Nukeblast as unsafe extern "C" fn(*mut cl_sustain_t) -> ());
        (*s).thinkinterval = 1 as libc::c_int;
        (*s).nextthink = cl.time;
    } else {
        MSG_ReadPos(&mut net_message, pos.as_mut_ptr());
    };
}

static mut splash_color: [byte; 7] = [
    0 as libc::c_int as byte,
    0xe0 as libc::c_int as byte,
    0xb0 as libc::c_int as byte,
    0x50 as libc::c_int as byte,
    0xd0 as libc::c_int as byte,
    0xe0 as libc::c_int as byte,
    0xe8 as libc::c_int as byte,
];

#[no_mangle]
pub unsafe extern "C" fn CL_ParseTEnt() {
    let mut type_0: libc::c_int = 0;
    let mut pos: vec3_t = [0.; 3];
    let mut pos2: vec3_t = [0.; 3];
    let mut dir: vec3_t = [0.; 3];
    let mut ex: *mut explosion_t = 0 as *mut explosion_t;
    let mut cnt: libc::c_int = 0;
    let mut color: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    let mut ent: libc::c_int = 0;
    let mut magnitude: libc::c_int = 0;
    type_0 = MSG_ReadByte(&mut net_message);
    match type_0 {
        1 => {
            MSG_ReadPos(&mut net_message, pos.as_mut_ptr());
            MSG_ReadDir(&mut net_message, dir.as_mut_ptr());
            CL_ParticleEffect(
                pos.as_mut_ptr(),
                dir.as_mut_ptr(),
                0xe8 as libc::c_int,
                60 as libc::c_int,
            );
        }
        0 | 9 | 14 => {
            MSG_ReadPos(&mut net_message, pos.as_mut_ptr());
            MSG_ReadDir(&mut net_message, dir.as_mut_ptr());
            if type_0 == TE_GUNSHOT as libc::c_int {
                CL_ParticleEffect(
                    pos.as_mut_ptr(),
                    dir.as_mut_ptr(),
                    0 as libc::c_int,
                    40 as libc::c_int,
                );
            } else {
                CL_ParticleEffect(
                    pos.as_mut_ptr(),
                    dir.as_mut_ptr(),
                    0xe0 as libc::c_int,
                    6 as libc::c_int,
                );
            }
            if type_0 != TE_SPARKS as libc::c_int {
                CL_SmokeAndFlash(pos.as_mut_ptr());
                cnt = rand() & 15 as libc::c_int;
                if cnt == 1 as libc::c_int {
                    S_StartSound(
                        pos.as_mut_ptr(),
                        0 as libc::c_int,
                        0 as libc::c_int,
                        cl_sfx_ric1,
                        1 as libc::c_int as libc::c_float,
                        1 as libc::c_int as libc::c_float,
                        0 as libc::c_int as libc::c_float,
                    );
                } else if cnt == 2 as libc::c_int {
                    S_StartSound(
                        pos.as_mut_ptr(),
                        0 as libc::c_int,
                        0 as libc::c_int,
                        cl_sfx_ric2,
                        1 as libc::c_int as libc::c_float,
                        1 as libc::c_int as libc::c_float,
                        0 as libc::c_int as libc::c_float,
                    );
                } else if cnt == 3 as libc::c_int {
                    S_StartSound(
                        pos.as_mut_ptr(),
                        0 as libc::c_int,
                        0 as libc::c_int,
                        cl_sfx_ric3,
                        1 as libc::c_int as libc::c_float,
                        1 as libc::c_int as libc::c_float,
                        0 as libc::c_int as libc::c_float,
                    );
                }
            }
        }
        12 | 13 => {
            MSG_ReadPos(&mut net_message, pos.as_mut_ptr());
            MSG_ReadDir(&mut net_message, dir.as_mut_ptr());
            if type_0 == TE_SCREEN_SPARKS as libc::c_int {
                CL_ParticleEffect(
                    pos.as_mut_ptr(),
                    dir.as_mut_ptr(),
                    0xd0 as libc::c_int,
                    40 as libc::c_int,
                );
            } else {
                CL_ParticleEffect(
                    pos.as_mut_ptr(),
                    dir.as_mut_ptr(),
                    0xb0 as libc::c_int,
                    40 as libc::c_int,
                );
            }
            S_StartSound(
                pos.as_mut_ptr(),
                0 as libc::c_int,
                0 as libc::c_int,
                cl_sfx_lashit,
                1 as libc::c_int as libc::c_float,
                1 as libc::c_int as libc::c_float,
                0 as libc::c_int as libc::c_float,
            );
        }
        4 => {
            MSG_ReadPos(&mut net_message, pos.as_mut_ptr());
            MSG_ReadDir(&mut net_message, dir.as_mut_ptr());
            CL_ParticleEffect(
                pos.as_mut_ptr(),
                dir.as_mut_ptr(),
                0 as libc::c_int,
                20 as libc::c_int,
            );
            CL_SmokeAndFlash(pos.as_mut_ptr());
        }
        10 => {
            cnt = MSG_ReadByte(&mut net_message);
            MSG_ReadPos(&mut net_message, pos.as_mut_ptr());
            MSG_ReadDir(&mut net_message, dir.as_mut_ptr());
            r = MSG_ReadByte(&mut net_message);
            if r > 6 as libc::c_int {
                color = 0 as libc::c_int;
            } else {
                color = splash_color[r as usize] as libc::c_int;
            }
            CL_ParticleEffect(pos.as_mut_ptr(), dir.as_mut_ptr(), color, cnt);
            if r == 1 as libc::c_int {
                r = rand() & 3 as libc::c_int;
                if r == 0 as libc::c_int {
                    S_StartSound(
                        pos.as_mut_ptr(),
                        0 as libc::c_int,
                        0 as libc::c_int,
                        cl_sfx_spark5,
                        1 as libc::c_int as libc::c_float,
                        3 as libc::c_int as libc::c_float,
                        0 as libc::c_int as libc::c_float,
                    );
                } else if r == 1 as libc::c_int {
                    S_StartSound(
                        pos.as_mut_ptr(),
                        0 as libc::c_int,
                        0 as libc::c_int,
                        cl_sfx_spark6,
                        1 as libc::c_int as libc::c_float,
                        3 as libc::c_int as libc::c_float,
                        0 as libc::c_int as libc::c_float,
                    );
                } else {
                    S_StartSound(
                        pos.as_mut_ptr(),
                        0 as libc::c_int,
                        0 as libc::c_int,
                        cl_sfx_spark7,
                        1 as libc::c_int as libc::c_float,
                        3 as libc::c_int as libc::c_float,
                        0 as libc::c_int as libc::c_float,
                    );
                }
            }
        }
        15 => {
            cnt = MSG_ReadByte(&mut net_message);
            MSG_ReadPos(&mut net_message, pos.as_mut_ptr());
            MSG_ReadDir(&mut net_message, dir.as_mut_ptr());
            color = MSG_ReadByte(&mut net_message);
            CL_ParticleEffect2(pos.as_mut_ptr(), dir.as_mut_ptr(), color, cnt);
        }
        27 => {
            MSG_ReadPos(&mut net_message, pos.as_mut_ptr());
            MSG_ReadPos(&mut net_message, dir.as_mut_ptr());
            CL_BlasterParticles(pos.as_mut_ptr(), dir.as_mut_ptr());
        }
        2 => {
            MSG_ReadPos(&mut net_message, pos.as_mut_ptr());
            MSG_ReadDir(&mut net_message, dir.as_mut_ptr());
            CL_BlasterParticles(pos.as_mut_ptr(), dir.as_mut_ptr());
            ex = CL_AllocExplosion();
            (*ex).ent.origin[0 as libc::c_int as usize] = pos[0 as libc::c_int as usize];
            (*ex).ent.origin[1 as libc::c_int as usize] = pos[1 as libc::c_int as usize];
            (*ex).ent.origin[2 as libc::c_int as usize] = pos[2 as libc::c_int as usize];
            (*ex)
                .ent
                .angles[0 as libc::c_int
                as usize] = (acos(dir[2 as libc::c_int as usize] as libc::c_double)
                / 3.14159265358979323846f64 * 180 as libc::c_int as libc::c_double)
                as libc::c_float;
            if dir[0 as libc::c_int as usize] != 0. {
                (*ex)
                    .ent
                    .angles[1 as libc::c_int
                    as usize] = (atan2(
                    dir[1 as libc::c_int as usize] as libc::c_double,
                    dir[0 as libc::c_int as usize] as libc::c_double,
                ) / 3.14159265358979323846f64 * 180 as libc::c_int as libc::c_double)
                    as libc::c_float;
            } else if dir[1 as libc::c_int as usize] > 0 as libc::c_int as libc::c_float
            {
                (*ex)
                    .ent
                    .angles[1 as libc::c_int
                    as usize] = 90 as libc::c_int as libc::c_float;
            } else if dir[1 as libc::c_int as usize] < 0 as libc::c_int as libc::c_float
            {
                (*ex)
                    .ent
                    .angles[1 as libc::c_int
                    as usize] = 270 as libc::c_int as libc::c_float;
            } else {
                (*ex)
                    .ent
                    .angles[1 as libc::c_int
                    as usize] = 0 as libc::c_int as libc::c_float;
            }
            (*ex).type_0 = ex_misc;
            (*ex).ent.flags = 8 as libc::c_int | 32 as libc::c_int;
            (*ex).start = (cl.frame.servertime - 100 as libc::c_int) as libc::c_float;
            (*ex).light = 150 as libc::c_int as libc::c_float;
            (*ex).lightcolor[0 as libc::c_int as usize] = 1 as libc::c_int as vec_t;
            (*ex).lightcolor[1 as libc::c_int as usize] = 1 as libc::c_int as vec_t;
            let ref mut fresh22 = (*ex).ent.model;
            *fresh22 = cl_mod_explode;
            (*ex).frames = 4 as libc::c_int;
            S_StartSound(
                pos.as_mut_ptr(),
                0 as libc::c_int,
                0 as libc::c_int,
                cl_sfx_lashit,
                1 as libc::c_int as libc::c_float,
                1 as libc::c_int as libc::c_float,
                0 as libc::c_int as libc::c_float,
            );
        }
        3 => {
            MSG_ReadPos(&mut net_message, pos.as_mut_ptr());
            MSG_ReadPos(&mut net_message, pos2.as_mut_ptr());
            CL_RailTrail(pos.as_mut_ptr(), pos2.as_mut_ptr());
            S_StartSound(
                pos2.as_mut_ptr(),
                0 as libc::c_int,
                0 as libc::c_int,
                cl_sfx_railg,
                1 as libc::c_int as libc::c_float,
                1 as libc::c_int as libc::c_float,
                0 as libc::c_int as libc::c_float,
            );
        }
        6 | 8 | 18 => {
            MSG_ReadPos(&mut net_message, pos.as_mut_ptr());
            ex = CL_AllocExplosion();
            (*ex).ent.origin[0 as libc::c_int as usize] = pos[0 as libc::c_int as usize];
            (*ex).ent.origin[1 as libc::c_int as usize] = pos[1 as libc::c_int as usize];
            (*ex).ent.origin[2 as libc::c_int as usize] = pos[2 as libc::c_int as usize];
            (*ex).type_0 = ex_poly;
            (*ex).ent.flags = 8 as libc::c_int;
            (*ex).start = (cl.frame.servertime - 100 as libc::c_int) as libc::c_float;
            (*ex).light = 350 as libc::c_int as libc::c_float;
            (*ex).lightcolor[0 as libc::c_int as usize] = 1.0f64 as vec_t;
            (*ex).lightcolor[1 as libc::c_int as usize] = 0.5f64 as vec_t;
            (*ex).lightcolor[2 as libc::c_int as usize] = 0.5f64 as vec_t;
            let ref mut fresh23 = (*ex).ent.model;
            *fresh23 = cl_mod_explo4;
            (*ex).frames = 19 as libc::c_int;
            (*ex).baseframe = 30 as libc::c_int;
            (*ex)
                .ent
                .angles[1 as libc::c_int
                as usize] = (rand() % 360 as libc::c_int) as libc::c_float;
            CL_ExplosionParticles(pos.as_mut_ptr());
            if type_0 == TE_GRENADE_EXPLOSION_WATER as libc::c_int {
                S_StartSound(
                    pos.as_mut_ptr(),
                    0 as libc::c_int,
                    0 as libc::c_int,
                    cl_sfx_watrexp,
                    1 as libc::c_int as libc::c_float,
                    1 as libc::c_int as libc::c_float,
                    0 as libc::c_int as libc::c_float,
                );
            } else {
                S_StartSound(
                    pos.as_mut_ptr(),
                    0 as libc::c_int,
                    0 as libc::c_int,
                    cl_sfx_grenexp,
                    1 as libc::c_int as libc::c_float,
                    1 as libc::c_int as libc::c_float,
                    0 as libc::c_int as libc::c_float,
                );
            }
        }
        28 => {
            MSG_ReadPos(&mut net_message, pos.as_mut_ptr());
            ex = CL_AllocExplosion();
            (*ex).ent.origin[0 as libc::c_int as usize] = pos[0 as libc::c_int as usize];
            (*ex).ent.origin[1 as libc::c_int as usize] = pos[1 as libc::c_int as usize];
            (*ex).ent.origin[2 as libc::c_int as usize] = pos[2 as libc::c_int as usize];
            (*ex).type_0 = ex_poly;
            (*ex).ent.flags = 8 as libc::c_int;
            (*ex).start = (cl.frame.servertime - 100 as libc::c_int) as libc::c_float;
            (*ex).light = 350 as libc::c_int as libc::c_float;
            (*ex).lightcolor[0 as libc::c_int as usize] = 1.0f64 as vec_t;
            (*ex).lightcolor[1 as libc::c_int as usize] = 0.5f64 as vec_t;
            (*ex).lightcolor[2 as libc::c_int as usize] = 0.5f64 as vec_t;
            (*ex)
                .ent
                .angles[1 as libc::c_int
                as usize] = (rand() % 360 as libc::c_int) as libc::c_float;
            let ref mut fresh24 = (*ex).ent.model;
            *fresh24 = cl_mod_explo4;
            if (frand() as libc::c_double) < 0.5f64 {
                (*ex).baseframe = 15 as libc::c_int;
            }
            (*ex).frames = 15 as libc::c_int;
            CL_ExplosionParticles(pos.as_mut_ptr());
            S_StartSound(
                pos.as_mut_ptr(),
                0 as libc::c_int,
                0 as libc::c_int,
                cl_sfx_rockexp,
                1 as libc::c_int as libc::c_float,
                1 as libc::c_int as libc::c_float,
                0 as libc::c_int as libc::c_float,
            );
        }
        5 | 53 | 7 | 17 | 54 => {
            MSG_ReadPos(&mut net_message, pos.as_mut_ptr());
            ex = CL_AllocExplosion();
            (*ex).ent.origin[0 as libc::c_int as usize] = pos[0 as libc::c_int as usize];
            (*ex).ent.origin[1 as libc::c_int as usize] = pos[1 as libc::c_int as usize];
            (*ex).ent.origin[2 as libc::c_int as usize] = pos[2 as libc::c_int as usize];
            (*ex).type_0 = ex_poly;
            (*ex).ent.flags = 8 as libc::c_int;
            (*ex).start = (cl.frame.servertime - 100 as libc::c_int) as libc::c_float;
            (*ex).light = 350 as libc::c_int as libc::c_float;
            (*ex).lightcolor[0 as libc::c_int as usize] = 1.0f64 as vec_t;
            (*ex).lightcolor[1 as libc::c_int as usize] = 0.5f64 as vec_t;
            (*ex).lightcolor[2 as libc::c_int as usize] = 0.5f64 as vec_t;
            (*ex)
                .ent
                .angles[1 as libc::c_int
                as usize] = (rand() % 360 as libc::c_int) as libc::c_float;
            if type_0 != TE_EXPLOSION1_BIG as libc::c_int {
                let ref mut fresh25 = (*ex).ent.model;
                *fresh25 = cl_mod_explo4;
            } else {
                let ref mut fresh26 = (*ex).ent.model;
                *fresh26 = cl_mod_explo4_big;
            }
            if (frand() as libc::c_double) < 0.5f64 {
                (*ex).baseframe = 15 as libc::c_int;
            }
            (*ex).frames = 15 as libc::c_int;
            if type_0 != TE_EXPLOSION1_BIG as libc::c_int
                && type_0 != TE_EXPLOSION1_NP as libc::c_int
            {
                CL_ExplosionParticles(pos.as_mut_ptr());
            }
            if type_0 == TE_ROCKET_EXPLOSION_WATER as libc::c_int {
                S_StartSound(
                    pos.as_mut_ptr(),
                    0 as libc::c_int,
                    0 as libc::c_int,
                    cl_sfx_watrexp,
                    1 as libc::c_int as libc::c_float,
                    1 as libc::c_int as libc::c_float,
                    0 as libc::c_int as libc::c_float,
                );
            } else {
                S_StartSound(
                    pos.as_mut_ptr(),
                    0 as libc::c_int,
                    0 as libc::c_int,
                    cl_sfx_rockexp,
                    1 as libc::c_int as libc::c_float,
                    1 as libc::c_int as libc::c_float,
                    0 as libc::c_int as libc::c_float,
                );
            }
        }
        20 => {
            MSG_ReadPos(&mut net_message, pos.as_mut_ptr());
            ex = CL_AllocExplosion();
            (*ex).ent.origin[0 as libc::c_int as usize] = pos[0 as libc::c_int as usize];
            (*ex).ent.origin[1 as libc::c_int as usize] = pos[1 as libc::c_int as usize];
            (*ex).ent.origin[2 as libc::c_int as usize] = pos[2 as libc::c_int as usize];
            (*ex).type_0 = ex_poly;
            (*ex).ent.flags = 8 as libc::c_int;
            (*ex).start = (cl.frame.servertime - 100 as libc::c_int) as libc::c_float;
            (*ex).light = 350 as libc::c_int as libc::c_float;
            (*ex).lightcolor[0 as libc::c_int as usize] = 0.0f64 as vec_t;
            (*ex).lightcolor[1 as libc::c_int as usize] = 1.0f64 as vec_t;
            (*ex).lightcolor[2 as libc::c_int as usize] = 0.0f64 as vec_t;
            let ref mut fresh27 = (*ex).ent.model;
            *fresh27 = cl_mod_bfg_explo;
            (*ex).ent.flags |= 32 as libc::c_int;
            (*ex).ent.alpha = 0.30f64 as libc::c_float;
            (*ex).frames = 4 as libc::c_int;
        }
        21 => {
            MSG_ReadPos(&mut net_message, pos.as_mut_ptr());
            CL_BFGExplosionParticles(pos.as_mut_ptr());
        }
        23 => {
            CL_ParseLaser(0xd0d1d2d3 as libc::c_uint as libc::c_int);
        }
        11 => {
            MSG_ReadPos(&mut net_message, pos.as_mut_ptr());
            MSG_ReadPos(&mut net_message, pos2.as_mut_ptr());
            CL_BubbleTrail(pos.as_mut_ptr(), pos2.as_mut_ptr());
        }
        16 | 19 => {
            ent = CL_ParseBeam(cl_mod_parasite_segment);
        }
        22 => {
            MSG_ReadPos(&mut net_message, pos.as_mut_ptr());
            CL_BigTeleportParticles(pos.as_mut_ptr());
            S_StartSound(
                pos.as_mut_ptr(),
                0 as libc::c_int,
                0 as libc::c_int,
                S_RegisterSound(
                    b"misc/bigtele.wav\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                ),
                1 as libc::c_int as libc::c_float,
                0 as libc::c_int as libc::c_float,
                0 as libc::c_int as libc::c_float,
            );
        }
        24 => {
            ent = CL_ParseBeam2(cl_mod_grapple_cable);
        }
        25 => {
            cnt = MSG_ReadByte(&mut net_message);
            MSG_ReadPos(&mut net_message, pos.as_mut_ptr());
            MSG_ReadDir(&mut net_message, dir.as_mut_ptr());
            color = MSG_ReadByte(&mut net_message);
            CL_ParticleEffect2(pos.as_mut_ptr(), dir.as_mut_ptr(), color, cnt);
            ex = CL_AllocExplosion();
            (*ex).ent.origin[0 as libc::c_int as usize] = pos[0 as libc::c_int as usize];
            (*ex).ent.origin[1 as libc::c_int as usize] = pos[1 as libc::c_int as usize];
            (*ex).ent.origin[2 as libc::c_int as usize] = pos[2 as libc::c_int as usize];
            (*ex).type_0 = ex_flash;
            (*ex).ent.flags = 128 as libc::c_int;
            (*ex)
                .start = (cl.frame.servertime as libc::c_double - 0.1f64)
                as libc::c_float;
            (*ex)
                .light = (100 as libc::c_int + rand() % 75 as libc::c_int)
                as libc::c_float;
            (*ex).lightcolor[0 as libc::c_int as usize] = 1.0f64 as vec_t;
            (*ex).lightcolor[1 as libc::c_int as usize] = 1.0f64 as vec_t;
            (*ex).lightcolor[2 as libc::c_int as usize] = 0.3f64 as vec_t;
            let ref mut fresh28 = (*ex).ent.model;
            *fresh28 = cl_mod_flash;
            (*ex).frames = 2 as libc::c_int;
        }
        26 => {
            MSG_ReadPos(&mut net_message, pos.as_mut_ptr());
            MSG_ReadDir(&mut net_message, dir.as_mut_ptr());
            CL_ParticleEffect2(
                pos.as_mut_ptr(),
                dir.as_mut_ptr(),
                0xdf as libc::c_int,
                30 as libc::c_int,
            );
        }
        29 => {
            cnt = MSG_ReadByte(&mut net_message);
            MSG_ReadPos(&mut net_message, pos.as_mut_ptr());
            MSG_ReadDir(&mut net_message, dir.as_mut_ptr());
            color = MSG_ReadByte(&mut net_message);
            CL_ParticleEffect3(pos.as_mut_ptr(), dir.as_mut_ptr(), color, cnt);
        }
        30 | 55 => {
            MSG_ReadPos(&mut net_message, pos.as_mut_ptr());
            MSG_ReadDir(&mut net_message, dir.as_mut_ptr());
            if type_0 == TE_BLASTER2 as libc::c_int {
                CL_BlasterParticles2(
                    pos.as_mut_ptr(),
                    dir.as_mut_ptr(),
                    0xd0 as libc::c_int as libc::c_uint,
                );
            } else {
                CL_BlasterParticles2(
                    pos.as_mut_ptr(),
                    dir.as_mut_ptr(),
                    0x6f as libc::c_int as libc::c_uint,
                );
            }
            ex = CL_AllocExplosion();
            (*ex).ent.origin[0 as libc::c_int as usize] = pos[0 as libc::c_int as usize];
            (*ex).ent.origin[1 as libc::c_int as usize] = pos[1 as libc::c_int as usize];
            (*ex).ent.origin[2 as libc::c_int as usize] = pos[2 as libc::c_int as usize];
            (*ex)
                .ent
                .angles[0 as libc::c_int
                as usize] = (acos(dir[2 as libc::c_int as usize] as libc::c_double)
                / 3.14159265358979323846f64 * 180 as libc::c_int as libc::c_double)
                as libc::c_float;
            if dir[0 as libc::c_int as usize] != 0. {
                (*ex)
                    .ent
                    .angles[1 as libc::c_int
                    as usize] = (atan2(
                    dir[1 as libc::c_int as usize] as libc::c_double,
                    dir[0 as libc::c_int as usize] as libc::c_double,
                ) / 3.14159265358979323846f64 * 180 as libc::c_int as libc::c_double)
                    as libc::c_float;
            } else if dir[1 as libc::c_int as usize] > 0 as libc::c_int as libc::c_float
            {
                (*ex)
                    .ent
                    .angles[1 as libc::c_int
                    as usize] = 90 as libc::c_int as libc::c_float;
            } else if dir[1 as libc::c_int as usize] < 0 as libc::c_int as libc::c_float
            {
                (*ex)
                    .ent
                    .angles[1 as libc::c_int
                    as usize] = 270 as libc::c_int as libc::c_float;
            } else {
                (*ex)
                    .ent
                    .angles[1 as libc::c_int
                    as usize] = 0 as libc::c_int as libc::c_float;
            }
            (*ex).type_0 = ex_misc;
            (*ex).ent.flags = 8 as libc::c_int | 32 as libc::c_int;
            if type_0 == TE_BLASTER2 as libc::c_int {
                (*ex).ent.skinnum = 1 as libc::c_int;
            } else {
                (*ex).ent.skinnum = 2 as libc::c_int;
            }
            (*ex).start = (cl.frame.servertime - 100 as libc::c_int) as libc::c_float;
            (*ex).light = 150 as libc::c_int as libc::c_float;
            if type_0 == TE_BLASTER2 as libc::c_int {
                (*ex).lightcolor[1 as libc::c_int as usize] = 1 as libc::c_int as vec_t;
            } else {
                (*ex).lightcolor[0 as libc::c_int as usize] = 0.19f64 as vec_t;
                (*ex).lightcolor[1 as libc::c_int as usize] = 0.41f64 as vec_t;
                (*ex).lightcolor[2 as libc::c_int as usize] = 0.75f64 as vec_t;
            }
            let ref mut fresh29 = (*ex).ent.model;
            *fresh29 = cl_mod_explode;
            (*ex).frames = 4 as libc::c_int;
            S_StartSound(
                pos.as_mut_ptr(),
                0 as libc::c_int,
                0 as libc::c_int,
                cl_sfx_lashit,
                1 as libc::c_int as libc::c_float,
                1 as libc::c_int as libc::c_float,
                0 as libc::c_int as libc::c_float,
            );
        }
        33 => {
            ent = CL_ParseLightning(cl_mod_lightning);
            S_StartSound(
                0 as *mut vec_t,
                ent,
                1 as libc::c_int,
                cl_sfx_lightning,
                1 as libc::c_int as libc::c_float,
                1 as libc::c_int as libc::c_float,
                0 as libc::c_int as libc::c_float,
            );
        }
        34 => {
            MSG_ReadPos(&mut net_message, pos.as_mut_ptr());
            MSG_ReadPos(&mut net_message, pos2.as_mut_ptr());
            CL_DebugTrail(pos.as_mut_ptr(), pos2.as_mut_ptr());
        }
        35 => {
            MSG_ReadPos(&mut net_message, pos.as_mut_ptr());
            ex = CL_AllocExplosion();
            (*ex).ent.origin[0 as libc::c_int as usize] = pos[0 as libc::c_int as usize];
            (*ex).ent.origin[1 as libc::c_int as usize] = pos[1 as libc::c_int as usize];
            (*ex).ent.origin[2 as libc::c_int as usize] = pos[2 as libc::c_int as usize];
            (*ex).type_0 = ex_poly;
            (*ex).ent.flags = 8 as libc::c_int;
            (*ex).start = (cl.frame.servertime - 100 as libc::c_int) as libc::c_float;
            (*ex).light = 350 as libc::c_int as libc::c_float;
            (*ex).lightcolor[0 as libc::c_int as usize] = 1.0f64 as vec_t;
            (*ex).lightcolor[1 as libc::c_int as usize] = 0.5f64 as vec_t;
            (*ex).lightcolor[2 as libc::c_int as usize] = 0.5f64 as vec_t;
            (*ex)
                .ent
                .angles[1 as libc::c_int
                as usize] = (rand() % 360 as libc::c_int) as libc::c_float;
            let ref mut fresh30 = (*ex).ent.model;
            *fresh30 = cl_mod_explo4;
            if (frand() as libc::c_double) < 0.5f64 {
                (*ex).baseframe = 15 as libc::c_int;
            }
            (*ex).frames = 15 as libc::c_int;
            if type_0 == TE_ROCKET_EXPLOSION_WATER as libc::c_int {
                S_StartSound(
                    pos.as_mut_ptr(),
                    0 as libc::c_int,
                    0 as libc::c_int,
                    cl_sfx_watrexp,
                    1 as libc::c_int as libc::c_float,
                    1 as libc::c_int as libc::c_float,
                    0 as libc::c_int as libc::c_float,
                );
            } else {
                S_StartSound(
                    pos.as_mut_ptr(),
                    0 as libc::c_int,
                    0 as libc::c_int,
                    cl_sfx_rockexp,
                    1 as libc::c_int as libc::c_float,
                    1 as libc::c_int as libc::c_float,
                    0 as libc::c_int as libc::c_float,
                );
            }
        }
        36 => {
            MSG_ReadPos(&mut net_message, pos.as_mut_ptr());
            ent = MSG_ReadShort(&mut net_message);
            CL_Flashlight(ent, pos.as_mut_ptr());
        }
        37 => {
            MSG_ReadPos(&mut net_message, pos.as_mut_ptr());
            MSG_ReadPos(&mut net_message, pos2.as_mut_ptr());
            color = MSG_ReadByte(&mut net_message);
            CL_ForceWall(pos.as_mut_ptr(), pos2.as_mut_ptr(), color);
        }
        38 => {
            ent = CL_ParsePlayerBeam(cl_mod_heatbeam);
        }
        39 => {
            ent = CL_ParsePlayerBeam(cl_mod_monster_heatbeam);
        }
        43 => {
            cnt = 50 as libc::c_int;
            MSG_ReadPos(&mut net_message, pos.as_mut_ptr());
            MSG_ReadDir(&mut net_message, dir.as_mut_ptr());
            r = 8 as libc::c_int;
            magnitude = 60 as libc::c_int;
            color = r & 0xff as libc::c_int;
            CL_ParticleSteamEffect(
                pos.as_mut_ptr(),
                dir.as_mut_ptr(),
                color,
                cnt,
                magnitude,
            );
            S_StartSound(
                pos.as_mut_ptr(),
                0 as libc::c_int,
                0 as libc::c_int,
                cl_sfx_lashit,
                1 as libc::c_int as libc::c_float,
                1 as libc::c_int as libc::c_float,
                0 as libc::c_int as libc::c_float,
            );
        }
        44 => {
            cnt = 20 as libc::c_int;
            MSG_ReadPos(&mut net_message, pos.as_mut_ptr());
            MSG_ReadDir(&mut net_message, dir.as_mut_ptr());
            color = 0xe0 as libc::c_int;
            magnitude = 60 as libc::c_int;
            CL_ParticleSteamEffect(
                pos.as_mut_ptr(),
                dir.as_mut_ptr(),
                color,
                cnt,
                magnitude,
            );
            S_StartSound(
                pos.as_mut_ptr(),
                0 as libc::c_int,
                0 as libc::c_int,
                cl_sfx_lashit,
                1 as libc::c_int as libc::c_float,
                1 as libc::c_int as libc::c_float,
                0 as libc::c_int as libc::c_float,
            );
        }
        40 => {
            CL_ParseSteam();
        }
        41 => {
            cnt = 8 as libc::c_int;
            MSG_ReadPos(&mut net_message, pos.as_mut_ptr());
            MSG_ReadPos(&mut net_message, pos2.as_mut_ptr());
            CL_BubbleTrail2(pos.as_mut_ptr(), pos2.as_mut_ptr(), cnt);
            S_StartSound(
                pos.as_mut_ptr(),
                0 as libc::c_int,
                0 as libc::c_int,
                cl_sfx_lashit,
                1 as libc::c_int as libc::c_float,
                1 as libc::c_int as libc::c_float,
                0 as libc::c_int as libc::c_float,
            );
        }
        42 => {
            MSG_ReadPos(&mut net_message, pos.as_mut_ptr());
            MSG_ReadDir(&mut net_message, dir.as_mut_ptr());
            CL_ParticleEffect(
                pos.as_mut_ptr(),
                dir.as_mut_ptr(),
                0xe8 as libc::c_int,
                250 as libc::c_int,
            );
        }
        45 => {
            dir[0 as libc::c_int as usize] = 0 as libc::c_int as vec_t;
            dir[1 as libc::c_int as usize] = 0 as libc::c_int as vec_t;
            dir[2 as libc::c_int as usize] = 1 as libc::c_int as vec_t;
            MSG_ReadPos(&mut net_message, pos.as_mut_ptr());
            CL_ParticleSmokeEffect(
                pos.as_mut_ptr(),
                dir.as_mut_ptr(),
                0 as libc::c_int,
                20 as libc::c_int,
                20 as libc::c_int,
            );
        }
        46 => {
            MSG_ReadPos(&mut net_message, pos.as_mut_ptr());
            MSG_ReadDir(&mut net_message, dir.as_mut_ptr());
            CL_ParticleEffect(
                pos.as_mut_ptr(),
                dir.as_mut_ptr(),
                0x75 as libc::c_int,
                40 as libc::c_int,
            );
            S_StartSound(
                pos.as_mut_ptr(),
                0 as libc::c_int,
                0 as libc::c_int,
                cl_sfx_lashit,
                1 as libc::c_int as libc::c_float,
                1 as libc::c_int as libc::c_float,
                0 as libc::c_int as libc::c_float,
            );
        }
        47 => {
            MSG_ReadPos(&mut net_message, pos.as_mut_ptr());
            CL_ColorFlash(
                pos.as_mut_ptr(),
                0 as libc::c_int,
                150 as libc::c_int,
                -(1 as libc::c_int) as libc::c_float,
                -(1 as libc::c_int) as libc::c_float,
                -(1 as libc::c_int) as libc::c_float,
            );
            CL_ColorExplosionParticles(
                pos.as_mut_ptr(),
                0 as libc::c_int,
                1 as libc::c_int,
            );
            S_StartSound(
                pos.as_mut_ptr(),
                0 as libc::c_int,
                0 as libc::c_int,
                cl_sfx_disrexp,
                1 as libc::c_int as libc::c_float,
                1 as libc::c_int as libc::c_float,
                0 as libc::c_int as libc::c_float,
            );
        }
        48 | 49 => {
            MSG_ReadPos(&mut net_message, pos.as_mut_ptr());
            CL_TeleportParticles(pos.as_mut_ptr());
        }
        50 => {
            CL_ParseWidow();
        }
        51 => {
            CL_ParseNuke();
        }
        52 => {
            MSG_ReadPos(&mut net_message, pos.as_mut_ptr());
            CL_WidowSplash(pos.as_mut_ptr());
        }
        _ => {
            Com_Error(
                1 as libc::c_int,
                b"CL_ParseTEnt: bad type\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
        }
    };
}

#[no_mangle]
pub unsafe extern "C" fn CL_AddBeams() {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut b: *mut beam_t = 0 as *mut beam_t;
    let mut dist: vec3_t = [0.; 3];
    let mut org: vec3_t = [0.; 3];
    let mut d: libc::c_float = 0.;
    let mut ent: entity_t = entity_t {
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
    };
    let mut yaw: libc::c_float = 0.;
    let mut pitch: libc::c_float = 0.;
    let mut forward: libc::c_float = 0.;
    let mut len: libc::c_float = 0.;
    let mut steps: libc::c_float = 0.;
    let mut model_length: libc::c_float = 0.;
    i = 0 as libc::c_int;
    b = cl_beams.as_mut_ptr();
    while i < 32 as libc::c_int {
        if !(((*b).model).is_null() || (*b).endtime < cl.time) {
            if (*b).entity == cl.playernum + 1 as libc::c_int {
                (*b)
                    .start[0 as libc::c_int
                    as usize] = cl.refdef.vieworg[0 as libc::c_int as usize];
                (*b)
                    .start[1 as libc::c_int
                    as usize] = cl.refdef.vieworg[1 as libc::c_int as usize];
                (*b)
                    .start[2 as libc::c_int
                    as usize] = cl.refdef.vieworg[2 as libc::c_int as usize];
                let ref mut fresh31 = (*b).start[2 as libc::c_int as usize];
                *fresh31 -= 22 as libc::c_int as libc::c_float;
            }
            org[0 as libc::c_int
                as usize] = (*b).start[0 as libc::c_int as usize]
                + (*b).offset[0 as libc::c_int as usize];
            org[1 as libc::c_int
                as usize] = (*b).start[1 as libc::c_int as usize]
                + (*b).offset[1 as libc::c_int as usize];
            org[2 as libc::c_int
                as usize] = (*b).start[2 as libc::c_int as usize]
                + (*b).offset[2 as libc::c_int as usize];
            dist[0 as libc::c_int
                as usize] = (*b).end[0 as libc::c_int as usize]
                - org[0 as libc::c_int as usize];
            dist[1 as libc::c_int
                as usize] = (*b).end[1 as libc::c_int as usize]
                - org[1 as libc::c_int as usize];
            dist[2 as libc::c_int
                as usize] = (*b).end[2 as libc::c_int as usize]
                - org[2 as libc::c_int as usize];
            if dist[1 as libc::c_int as usize] == 0 as libc::c_int as libc::c_float
                && dist[0 as libc::c_int as usize] == 0 as libc::c_int as libc::c_float
            {
                yaw = 0 as libc::c_int as libc::c_float;
                if dist[2 as libc::c_int as usize] > 0 as libc::c_int as libc::c_float {
                    pitch = 90 as libc::c_int as libc::c_float;
                } else {
                    pitch = 270 as libc::c_int as libc::c_float;
                }
            } else {
                if dist[0 as libc::c_int as usize] != 0. {
                    yaw = (atan2(
                        dist[1 as libc::c_int as usize] as libc::c_double,
                        dist[0 as libc::c_int as usize] as libc::c_double,
                    ) * 180 as libc::c_int as libc::c_double / 3.14159265358979323846f64)
                        as libc::c_float;
                } else if dist[1 as libc::c_int as usize]
                    > 0 as libc::c_int as libc::c_float
                {
                    yaw = 90 as libc::c_int as libc::c_float;
                } else {
                    yaw = 270 as libc::c_int as libc::c_float;
                }
                if yaw < 0 as libc::c_int as libc::c_float {
                    yaw += 360 as libc::c_int as libc::c_float;
                }
                forward = sqrt(
                    (dist[0 as libc::c_int as usize] * dist[0 as libc::c_int as usize]
                        + dist[1 as libc::c_int as usize]
                        * dist[1 as libc::c_int as usize]) as libc::c_double,
                ) as libc::c_float;
                pitch = (atan2(
                    dist[2 as libc::c_int as usize] as libc::c_double,
                    forward as libc::c_double,
                ) * -180.0f64 / 3.14159265358979323846f64) as libc::c_float;
                if pitch < 0 as libc::c_int as libc::c_float {
                    pitch = (pitch as libc::c_double + 360.0f64) as libc::c_float;
                }
            }
            d = VectorNormalize(dist.as_mut_ptr());
            memset(
                &mut ent as *mut entity_t as *mut libc::c_void,
                0 as libc::c_int,
                ::std::mem::size_of::<entity_t>() as libc::c_ulong,
            );
            if (*b).model == cl_mod_lightning {
                model_length = 35.0f64 as libc::c_float;
                d = (d as libc::c_double - 20.0f64) as libc::c_float;
            } else {
                model_length = 30.0f64 as libc::c_float;
            }
            steps = ceil((d / model_length) as libc::c_double) as libc::c_float;
            len = (d - model_length) / (steps - 1 as libc::c_int as libc::c_float);
            if (*b).model == cl_mod_lightning && d <= model_length {
                ent
                    .origin[0 as libc::c_int
                    as usize] = (*b).end[0 as libc::c_int as usize];
                ent
                    .origin[1 as libc::c_int
                    as usize] = (*b).end[1 as libc::c_int as usize];
                ent
                    .origin[2 as libc::c_int
                    as usize] = (*b).end[2 as libc::c_int as usize];
                ent.model = (*b).model;
                ent.flags = 8 as libc::c_int;
                ent.angles[0 as libc::c_int as usize] = pitch;
                ent.angles[1 as libc::c_int as usize] = yaw;
                ent
                    .angles[2 as libc::c_int
                    as usize] = (rand() % 360 as libc::c_int) as libc::c_float;
                V_AddEntity(&mut ent);
                return;
            }
            while d > 0 as libc::c_int as libc::c_float {
                ent.origin[0 as libc::c_int as usize] = org[0 as libc::c_int as usize];
                ent.origin[1 as libc::c_int as usize] = org[1 as libc::c_int as usize];
                ent.origin[2 as libc::c_int as usize] = org[2 as libc::c_int as usize];
                ent.model = (*b).model;
                if (*b).model == cl_mod_lightning {
                    ent.flags = 8 as libc::c_int;
                    ent.angles[0 as libc::c_int as usize] = -pitch;
                    ent
                        .angles[1 as libc::c_int
                        as usize] = (yaw as libc::c_double + 180.0f64) as libc::c_float;
                    ent
                        .angles[2 as libc::c_int
                        as usize] = (rand() % 360 as libc::c_int) as libc::c_float;
                } else {
                    ent.angles[0 as libc::c_int as usize] = pitch;
                    ent.angles[1 as libc::c_int as usize] = yaw;
                    ent
                        .angles[2 as libc::c_int
                        as usize] = (rand() % 360 as libc::c_int) as libc::c_float;
                }
                V_AddEntity(&mut ent);
                j = 0 as libc::c_int;
                while j < 3 as libc::c_int {
                    org[j as usize] += dist[j as usize] * len;
                    j += 1;
                }
                d -= model_length;
            }
        }
        i += 1;
        b = b.offset(1);
    }
}

#[no_mangle]
pub unsafe extern "C" fn CL_AddPlayerBeams() {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut b: *mut beam_t = 0 as *mut beam_t;
    let mut dist: vec3_t = [0.; 3];
    let mut org: vec3_t = [0.; 3];
    let mut d: libc::c_float = 0.;
    let mut ent: entity_t = entity_t {
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
    };
    let mut yaw: libc::c_float = 0.;
    let mut pitch: libc::c_float = 0.;
    let mut forward: libc::c_float = 0.;
    let mut len: libc::c_float = 0.;
    let mut steps: libc::c_float = 0.;
    let mut framenum: libc::c_int = 0;
    let mut model_length: libc::c_float = 0.;
    let mut hand_multiplier: libc::c_float = 0.;
    let mut oldframe: *mut frame_t = 0 as *mut frame_t;
    let mut ps: *mut player_state_t = 0 as *mut player_state_t;
    let mut ops: *mut player_state_t = 0 as *mut player_state_t;
    if !hand.is_null() {
        if (*hand).value == 2 as libc::c_int as libc::c_float {
            hand_multiplier = 0 as libc::c_int as libc::c_float;
        } else if (*hand).value == 1 as libc::c_int as libc::c_float {
            hand_multiplier = -(1 as libc::c_int) as libc::c_float;
        } else {
            hand_multiplier = 1 as libc::c_int as libc::c_float;
        }
    } else {
        hand_multiplier = 1 as libc::c_int as libc::c_float;
    }
    i = 0 as libc::c_int;
    b = cl_playerbeams.as_mut_ptr();
    while i < 32 as libc::c_int {
        let mut f: vec3_t = [0.; 3];
        let mut r: vec3_t = [0.; 3];
        let mut u: vec3_t = [0.; 3];
        if !(((*b).model).is_null() || (*b).endtime < cl.time) {
            if !cl_mod_heatbeam.is_null() && (*b).model == cl_mod_heatbeam {
                if (*b).entity == cl.playernum + 1 as libc::c_int {
                    ps = &mut cl.frame.playerstate;
                    j = cl.frame.serverframe - 1 as libc::c_int
                        & 16 as libc::c_int - 1 as libc::c_int;
                    oldframe = &mut *(cl.frames).as_mut_ptr().offset(j as isize)
                        as *mut frame_t;
                    if (*oldframe).serverframe != cl.frame.serverframe - 1 as libc::c_int
                        || (*oldframe).valid as u64 == 0
                    {
                        oldframe = &mut cl.frame;
                    }
                    ops = &mut (*oldframe).playerstate;
                    j = 0 as libc::c_int;
                    while j < 3 as libc::c_int {
                        (*b)
                            .start[j
                            as usize] = cl.refdef.vieworg[j as usize]
                            + (*ops).gunoffset[j as usize]
                            + cl.lerpfrac
                            * ((*ps).gunoffset[j as usize]
                            - (*ops).gunoffset[j as usize]);
                        j += 1;
                    }
                    VectorMA(
                        ((*b).start).as_mut_ptr(),
                        hand_multiplier * (*b).offset[0 as libc::c_int as usize],
                        (cl.v_right).as_mut_ptr(),
                        org.as_mut_ptr(),
                    );
                    VectorMA(
                        org.as_mut_ptr(),
                        (*b).offset[1 as libc::c_int as usize],
                        (cl.v_forward).as_mut_ptr(),
                        org.as_mut_ptr(),
                    );
                    VectorMA(
                        org.as_mut_ptr(),
                        (*b).offset[2 as libc::c_int as usize],
                        (cl.v_up).as_mut_ptr(),
                        org.as_mut_ptr(),
                    );
                    if !hand.is_null()
                        && (*hand).value == 2 as libc::c_int as libc::c_float
                    {
                        VectorMA(
                            org.as_mut_ptr(),
                            -(1 as libc::c_int) as libc::c_float,
                            (cl.v_up).as_mut_ptr(),
                            org.as_mut_ptr(),
                        );
                    }
                    r[0 as libc::c_int as usize] = cl.v_right[0 as libc::c_int as usize];
                    r[1 as libc::c_int as usize] = cl.v_right[1 as libc::c_int as usize];
                    r[2 as libc::c_int as usize] = cl.v_right[2 as libc::c_int as usize];
                    f[0 as libc::c_int
                        as usize] = cl.v_forward[0 as libc::c_int as usize];
                    f[1 as libc::c_int
                        as usize] = cl.v_forward[1 as libc::c_int as usize];
                    f[2 as libc::c_int
                        as usize] = cl.v_forward[2 as libc::c_int as usize];
                    u[0 as libc::c_int as usize] = cl.v_up[0 as libc::c_int as usize];
                    u[1 as libc::c_int as usize] = cl.v_up[1 as libc::c_int as usize];
                    u[2 as libc::c_int as usize] = cl.v_up[2 as libc::c_int as usize];
                } else {
                    org[0 as libc::c_int
                        as usize] = (*b).start[0 as libc::c_int as usize];
                    org[1 as libc::c_int
                        as usize] = (*b).start[1 as libc::c_int as usize];
                    org[2 as libc::c_int
                        as usize] = (*b).start[2 as libc::c_int as usize];
                }
            } else {
                if (*b).entity == cl.playernum + 1 as libc::c_int {
                    (*b)
                        .start[0 as libc::c_int
                        as usize] = cl.refdef.vieworg[0 as libc::c_int as usize];
                    (*b)
                        .start[1 as libc::c_int
                        as usize] = cl.refdef.vieworg[1 as libc::c_int as usize];
                    (*b)
                        .start[2 as libc::c_int
                        as usize] = cl.refdef.vieworg[2 as libc::c_int as usize];
                    let ref mut fresh32 = (*b).start[2 as libc::c_int as usize];
                    *fresh32 -= 22 as libc::c_int as libc::c_float;
                }
                org[0 as libc::c_int
                    as usize] = (*b).start[0 as libc::c_int as usize]
                    + (*b).offset[0 as libc::c_int as usize];
                org[1 as libc::c_int
                    as usize] = (*b).start[1 as libc::c_int as usize]
                    + (*b).offset[1 as libc::c_int as usize];
                org[2 as libc::c_int
                    as usize] = (*b).start[2 as libc::c_int as usize]
                    + (*b).offset[2 as libc::c_int as usize];
            }
            dist[0 as libc::c_int
                as usize] = (*b).end[0 as libc::c_int as usize]
                - org[0 as libc::c_int as usize];
            dist[1 as libc::c_int
                as usize] = (*b).end[1 as libc::c_int as usize]
                - org[1 as libc::c_int as usize];
            dist[2 as libc::c_int
                as usize] = (*b).end[2 as libc::c_int as usize]
                - org[2 as libc::c_int as usize];
            if !cl_mod_heatbeam.is_null() && (*b).model == cl_mod_heatbeam
                && (*b).entity == cl.playernum + 1 as libc::c_int
            {
                let mut len_0: vec_t = 0.;
                len_0 = VectorLength(dist.as_mut_ptr());
                VectorScale(f.as_mut_ptr(), len_0, dist.as_mut_ptr());
                VectorMA(
                    dist.as_mut_ptr(),
                    hand_multiplier * (*b).offset[0 as libc::c_int as usize],
                    r.as_mut_ptr(),
                    dist.as_mut_ptr(),
                );
                VectorMA(
                    dist.as_mut_ptr(),
                    (*b).offset[1 as libc::c_int as usize],
                    f.as_mut_ptr(),
                    dist.as_mut_ptr(),
                );
                VectorMA(
                    dist.as_mut_ptr(),
                    (*b).offset[2 as libc::c_int as usize],
                    u.as_mut_ptr(),
                    dist.as_mut_ptr(),
                );
                if !hand.is_null() && (*hand).value == 2 as libc::c_int as libc::c_float
                {
                    VectorMA(
                        org.as_mut_ptr(),
                        -(1 as libc::c_int) as libc::c_float,
                        (cl.v_up).as_mut_ptr(),
                        org.as_mut_ptr(),
                    );
                }
            }
            if dist[1 as libc::c_int as usize] == 0 as libc::c_int as libc::c_float
                && dist[0 as libc::c_int as usize] == 0 as libc::c_int as libc::c_float
            {
                yaw = 0 as libc::c_int as libc::c_float;
                if dist[2 as libc::c_int as usize] > 0 as libc::c_int as libc::c_float {
                    pitch = 90 as libc::c_int as libc::c_float;
                } else {
                    pitch = 270 as libc::c_int as libc::c_float;
                }
            } else {
                if dist[0 as libc::c_int as usize] != 0. {
                    yaw = (atan2(
                        dist[1 as libc::c_int as usize] as libc::c_double,
                        dist[0 as libc::c_int as usize] as libc::c_double,
                    ) * 180 as libc::c_int as libc::c_double / 3.14159265358979323846f64)
                        as libc::c_float;
                } else if dist[1 as libc::c_int as usize]
                    > 0 as libc::c_int as libc::c_float
                {
                    yaw = 90 as libc::c_int as libc::c_float;
                } else {
                    yaw = 270 as libc::c_int as libc::c_float;
                }
                if yaw < 0 as libc::c_int as libc::c_float {
                    yaw += 360 as libc::c_int as libc::c_float;
                }
                forward = sqrt(
                    (dist[0 as libc::c_int as usize] * dist[0 as libc::c_int as usize]
                        + dist[1 as libc::c_int as usize]
                        * dist[1 as libc::c_int as usize]) as libc::c_double,
                ) as libc::c_float;
                pitch = (atan2(
                    dist[2 as libc::c_int as usize] as libc::c_double,
                    forward as libc::c_double,
                ) * -180.0f64 / 3.14159265358979323846f64) as libc::c_float;
                if pitch < 0 as libc::c_int as libc::c_float {
                    pitch = (pitch as libc::c_double + 360.0f64) as libc::c_float;
                }
            }
            if !cl_mod_heatbeam.is_null() && (*b).model == cl_mod_heatbeam {
                if (*b).entity != cl.playernum + 1 as libc::c_int {
                    framenum = 2 as libc::c_int;
                    ent.angles[0 as libc::c_int as usize] = -pitch;
                    ent
                        .angles[1 as libc::c_int
                        as usize] = (yaw as libc::c_double + 180.0f64) as libc::c_float;
                    ent
                        .angles[2 as libc::c_int
                        as usize] = 0 as libc::c_int as libc::c_float;
                    AngleVectors(
                        (ent.angles).as_mut_ptr(),
                        f.as_mut_ptr(),
                        r.as_mut_ptr(),
                        u.as_mut_ptr(),
                    );
                    if VectorCompare(
                        ((*b).offset).as_mut_ptr(),
                        vec3_origin.as_mut_ptr(),
                    ) == 0
                    {
                        VectorMA(
                            org.as_mut_ptr(),
                            -(*b).offset[0 as libc::c_int as usize]
                                + 1 as libc::c_int as libc::c_float,
                            r.as_mut_ptr(),
                            org.as_mut_ptr(),
                        );
                        VectorMA(
                            org.as_mut_ptr(),
                            -(*b).offset[1 as libc::c_int as usize],
                            f.as_mut_ptr(),
                            org.as_mut_ptr(),
                        );
                        VectorMA(
                            org.as_mut_ptr(),
                            -(*b).offset[2 as libc::c_int as usize]
                                - 10 as libc::c_int as libc::c_float,
                            u.as_mut_ptr(),
                            org.as_mut_ptr(),
                        );
                    } else {
                        CL_MonsterPlasma_Shell(((*b).start).as_mut_ptr());
                    }
                } else {
                    framenum = 1 as libc::c_int;
                }
            }
            if !cl_mod_heatbeam.is_null() && (*b).model == cl_mod_heatbeam
                && (*b).entity == cl.playernum + 1 as libc::c_int
            {
                CL_Heatbeam(org.as_mut_ptr(), dist.as_mut_ptr());
            }
            d = VectorNormalize(dist.as_mut_ptr());
            memset(
                &mut ent as *mut entity_t as *mut libc::c_void,
                0 as libc::c_int,
                ::std::mem::size_of::<entity_t>() as libc::c_ulong,
            );
            if (*b).model == cl_mod_heatbeam {
                model_length = 32.0f64 as libc::c_float;
            } else if (*b).model == cl_mod_lightning {
                model_length = 35.0f64 as libc::c_float;
                d = (d as libc::c_double - 20.0f64) as libc::c_float;
            } else {
                model_length = 30.0f64 as libc::c_float;
            }
            steps = ceil((d / model_length) as libc::c_double) as libc::c_float;
            len = (d - model_length) / (steps - 1 as libc::c_int as libc::c_float);
            if (*b).model == cl_mod_lightning && d <= model_length {
                ent
                    .origin[0 as libc::c_int
                    as usize] = (*b).end[0 as libc::c_int as usize];
                ent
                    .origin[1 as libc::c_int
                    as usize] = (*b).end[1 as libc::c_int as usize];
                ent
                    .origin[2 as libc::c_int
                    as usize] = (*b).end[2 as libc::c_int as usize];
                ent.model = (*b).model;
                ent.flags = 8 as libc::c_int;
                ent.angles[0 as libc::c_int as usize] = pitch;
                ent.angles[1 as libc::c_int as usize] = yaw;
                ent
                    .angles[2 as libc::c_int
                    as usize] = (rand() % 360 as libc::c_int) as libc::c_float;
                V_AddEntity(&mut ent);
                return;
            }
            while d > 0 as libc::c_int as libc::c_float {
                ent.origin[0 as libc::c_int as usize] = org[0 as libc::c_int as usize];
                ent.origin[1 as libc::c_int as usize] = org[1 as libc::c_int as usize];
                ent.origin[2 as libc::c_int as usize] = org[2 as libc::c_int as usize];
                ent.model = (*b).model;
                if !cl_mod_heatbeam.is_null() && (*b).model == cl_mod_heatbeam {
                    ent.flags = 8 as libc::c_int;
                    ent.angles[0 as libc::c_int as usize] = -pitch;
                    ent
                        .angles[1 as libc::c_int
                        as usize] = (yaw as libc::c_double + 180.0f64) as libc::c_float;
                    ent
                        .angles[2 as libc::c_int
                        as usize] = (cl.time % 360 as libc::c_int) as libc::c_float;
                    ent.frame = framenum;
                } else if (*b).model == cl_mod_lightning {
                    ent.flags = 8 as libc::c_int;
                    ent.angles[0 as libc::c_int as usize] = -pitch;
                    ent
                        .angles[1 as libc::c_int
                        as usize] = (yaw as libc::c_double + 180.0f64) as libc::c_float;
                    ent
                        .angles[2 as libc::c_int
                        as usize] = (rand() % 360 as libc::c_int) as libc::c_float;
                } else {
                    ent.angles[0 as libc::c_int as usize] = pitch;
                    ent.angles[1 as libc::c_int as usize] = yaw;
                    ent
                        .angles[2 as libc::c_int
                        as usize] = (rand() % 360 as libc::c_int) as libc::c_float;
                }
                V_AddEntity(&mut ent);
                j = 0 as libc::c_int;
                while j < 3 as libc::c_int {
                    org[j as usize] += dist[j as usize] * len;
                    j += 1;
                }
                d -= model_length;
            }
        }
        i += 1;
        b = b.offset(1);
    }
}

#[no_mangle]
pub unsafe extern "C" fn CL_AddExplosions() {
    let mut ent: *mut entity_t = 0 as *mut entity_t;
    let mut i: libc::c_int = 0;
    let mut ex: *mut explosion_t = 0 as *mut explosion_t;
    let mut frac: libc::c_float = 0.;
    let mut f: libc::c_int = 0;
    memset(
        &mut ent as *mut *mut entity_t as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<*mut entity_t>() as libc::c_ulong,
    );
    i = 0 as libc::c_int;
    ex = cl_explosions.as_mut_ptr();
    while i < 32 as libc::c_int {
        if !((*ex).type_0 as libc::c_uint == ex_free as libc::c_int as libc::c_uint) {
            frac = ((cl.time as libc::c_float - (*ex).start) as libc::c_double
                / 100.0f64) as libc::c_float;
            f = floor(frac as libc::c_double) as libc::c_int;
            ent = &mut (*ex).ent;
            match (*ex).type_0 as libc::c_uint {
                4 => {
                    if f >= (*ex).frames - 1 as libc::c_int {
                        (*ex).type_0 = ex_free;
                    }
                }
                2 => {
                    if f >= (*ex).frames - 1 as libc::c_int {
                        (*ex).type_0 = ex_free;
                    } else {
                        (*ent)
                            .alpha = (1.0f64
                            - (frac / ((*ex).frames - 1 as libc::c_int) as libc::c_float)
                            as libc::c_double) as libc::c_float;
                    }
                }
                3 => {
                    if f >= 1 as libc::c_int {
                        (*ex).type_0 = ex_free;
                    } else {
                        (*ent).alpha = 1.0f64 as libc::c_float;
                    }
                }
                5 => {
                    if f >= (*ex).frames - 1 as libc::c_int {
                        (*ex).type_0 = ex_free;
                    } else {
                        (*ent)
                            .alpha = ((16.0f64 - f as libc::c_float as libc::c_double)
                            / 16.0f64) as libc::c_float;
                        if f < 10 as libc::c_int {
                            (*ent).skinnum = f >> 1 as libc::c_int;
                            if (*ent).skinnum < 0 as libc::c_int {
                                (*ent).skinnum = 0 as libc::c_int;
                            }
                        } else {
                            (*ent).flags |= 32 as libc::c_int;
                            if f < 13 as libc::c_int {
                                (*ent).skinnum = 5 as libc::c_int;
                            } else {
                                (*ent).skinnum = 6 as libc::c_int;
                            }
                        }
                    }
                }
                6 => {
                    if f >= (*ex).frames - 1 as libc::c_int {
                        (*ex).type_0 = ex_free;
                    } else {
                        (*ent)
                            .alpha = ((5.0f64 - f as libc::c_float as libc::c_double)
                            / 5.0f64) as libc::c_float;
                        (*ent).skinnum = 0 as libc::c_int;
                        (*ent).flags |= 32 as libc::c_int;
                    }
                }
                _ => {}
            }
            if !((*ex).type_0 as libc::c_uint == ex_free as libc::c_int as libc::c_uint)
            {
                if (*ex).light != 0. {
                    V_AddLight(
                        ((*ent).origin).as_mut_ptr(),
                        (*ex).light * (*ent).alpha,
                        (*ex).lightcolor[0 as libc::c_int as usize],
                        (*ex).lightcolor[1 as libc::c_int as usize],
                        (*ex).lightcolor[2 as libc::c_int as usize],
                    );
                }
                (*ent)
                    .oldorigin[0 as libc::c_int
                    as usize] = (*ent).origin[0 as libc::c_int as usize];
                (*ent)
                    .oldorigin[1 as libc::c_int
                    as usize] = (*ent).origin[1 as libc::c_int as usize];
                (*ent)
                    .oldorigin[2 as libc::c_int
                    as usize] = (*ent).origin[2 as libc::c_int as usize];
                if f < 0 as libc::c_int {
                    f = 0 as libc::c_int;
                }
                (*ent).frame = (*ex).baseframe + f + 1 as libc::c_int;
                (*ent).oldframe = (*ex).baseframe + f;
                (*ent)
                    .backlerp = (1.0f64 - cl.lerpfrac as libc::c_double)
                    as libc::c_float;
                V_AddEntity(ent);
            }
        }
        i += 1;
        ex = ex.offset(1);
    }
}

#[no_mangle]
pub unsafe extern "C" fn CL_AddLasers() {
    let mut l: *mut laser_t = 0 as *mut laser_t;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    l = cl_lasers.as_mut_ptr();
    while i < 32 as libc::c_int {
        if (*l).endtime >= cl.time {
            V_AddEntity(&mut (*l).ent);
        }
        i += 1;
        l = l.offset(1);
    }
}

#[no_mangle]
pub unsafe extern "C" fn CL_ProcessSustain() {
    let mut s: *mut cl_sustain_t = 0 as *mut cl_sustain_t;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    s = cl_sustains.as_mut_ptr();
    while i < 32 as libc::c_int {
        if (*s).id != 0 {
            if (*s).endtime >= cl.time && cl.time >= (*s).nextthink {
                ((*s).think).expect("non-null function pointer")(s);
            } else if (*s).endtime < cl.time {
                (*s).id = 0 as libc::c_int;
            }
        }
        i += 1;
        s = s.offset(1);
    }
}

#[no_mangle]
pub unsafe extern "C" fn CL_AddTEnts() {
    CL_AddBeams();
    CL_AddPlayerBeams();
    CL_AddExplosions();
    CL_AddLasers();
    CL_ProcessSustain();
}
