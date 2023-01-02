#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(extern_types, register_tool)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    static mut filterban: *mut cvar_t;
    static mut gi: game_import_t;
    fn atoi(__nptr: *const libc::c_char) -> libc::c_int;
    fn Q_stricmp(s1: *mut libc::c_char, s2: *mut libc::c_char) -> libc::c_int;
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
    pub movetype: libc::c_int,
    pub flags: libc::c_int,
    pub model: *mut libc::c_char,
    pub freetime: libc::c_float,
    pub message: *mut libc::c_char,
    pub classname: *mut libc::c_char,
    pub spawnflags: libc::c_int,
    pub timestamp: libc::c_float,
    pub angle: libc::c_float,
    pub target: *mut libc::c_char,
    pub targetname: *mut libc::c_char,
    pub killtarget: *mut libc::c_char,
    pub team: *mut libc::c_char,
    pub pathtarget: *mut libc::c_char,
    pub deathtarget: *mut libc::c_char,
    pub combattarget: *mut libc::c_char,
    pub target_ent: *mut edict_t,
    pub speed: libc::c_float,
    pub accel: libc::c_float,
    pub decel: libc::c_float,
    pub movedir: vec3_t,
    pub pos1: vec3_t,
    pub pos2: vec3_t,
    pub velocity: vec3_t,
    pub avelocity: vec3_t,
    pub mass: libc::c_int,
    pub air_finished: libc::c_float,
    pub gravity: libc::c_float,
    pub goalentity: *mut edict_t,
    pub movetarget: *mut edict_t,
    pub yaw_speed: libc::c_float,
    pub ideal_yaw: libc::c_float,
    pub nextthink: libc::c_float,
    pub prethink: Option::<unsafe extern "C" fn(*mut edict_t) -> ()>,
    pub think: Option::<unsafe extern "C" fn(*mut edict_t) -> ()>,
    pub blocked: Option::<unsafe extern "C" fn(*mut edict_t, *mut edict_t) -> ()>,
    pub touch: Option::<
        unsafe extern "C" fn(
            *mut edict_t,
            *mut edict_t,
            *mut cplane_t,
            *mut csurface_t,
        ) -> (),
    >,
    pub use_0: Option::<
        unsafe extern "C" fn(*mut edict_t, *mut edict_t, *mut edict_t) -> (),
    >,
    pub pain: Option::<
        unsafe extern "C" fn(
            *mut edict_t,
            *mut edict_t,
            libc::c_float,
            libc::c_int,
        ) -> (),
    >,
    pub die: Option::<
        unsafe extern "C" fn(
            *mut edict_t,
            *mut edict_t,
            *mut edict_t,
            libc::c_int,
            *mut vec_t,
        ) -> (),
    >,
    pub touch_debounce_time: libc::c_float,
    pub pain_debounce_time: libc::c_float,
    pub damage_debounce_time: libc::c_float,
    pub fly_sound_debounce_time: libc::c_float,
    pub last_move_time: libc::c_float,
    pub health: libc::c_int,
    pub max_health: libc::c_int,
    pub gib_health: libc::c_int,
    pub deadflag: libc::c_int,
    pub show_hostile: qboolean,
    pub powerarmor_time: libc::c_float,
    pub map: *mut libc::c_char,
    pub viewheight: libc::c_int,
    pub takedamage: libc::c_int,
    pub dmg: libc::c_int,
    pub radius_dmg: libc::c_int,
    pub dmg_radius: libc::c_float,
    pub sounds: libc::c_int,
    pub count: libc::c_int,
    pub chain: *mut edict_t,
    pub enemy: *mut edict_t,
    pub oldenemy: *mut edict_t,
    pub activator: *mut edict_t,
    pub groundentity: *mut edict_t,
    pub groundentity_linkcount: libc::c_int,
    pub teamchain: *mut edict_t,
    pub teammaster: *mut edict_t,
    pub mynoise: *mut edict_t,
    pub mynoise2: *mut edict_t,
    pub noise_index: libc::c_int,
    pub noise_index2: libc::c_int,
    pub volume: libc::c_float,
    pub attenuation: libc::c_float,
    pub wait: libc::c_float,
    pub delay: libc::c_float,
    pub random: libc::c_float,
    pub teleport_time: libc::c_float,
    pub watertype: libc::c_int,
    pub waterlevel: libc::c_int,
    pub move_origin: vec3_t,
    pub move_angles: vec3_t,
    pub light_level: libc::c_int,
    pub style: libc::c_int,
    pub item: *mut gitem_t,
    pub moveinfo: moveinfo_t,
    pub monsterinfo: monsterinfo_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct monsterinfo_t {
    pub currentmove: *mut mmove_t,
    pub aiflags: libc::c_int,
    pub nextframe: libc::c_int,
    pub scale: libc::c_float,
    pub stand: Option::<unsafe extern "C" fn(*mut edict_t) -> ()>,
    pub idle: Option::<unsafe extern "C" fn(*mut edict_t) -> ()>,
    pub search: Option::<unsafe extern "C" fn(*mut edict_t) -> ()>,
    pub walk: Option::<unsafe extern "C" fn(*mut edict_t) -> ()>,
    pub run: Option::<unsafe extern "C" fn(*mut edict_t) -> ()>,
    pub dodge: Option::<
        unsafe extern "C" fn(*mut edict_t, *mut edict_t, libc::c_float) -> (),
    >,
    pub attack: Option::<unsafe extern "C" fn(*mut edict_t) -> ()>,
    pub melee: Option::<unsafe extern "C" fn(*mut edict_t) -> ()>,
    pub sight: Option::<unsafe extern "C" fn(*mut edict_t, *mut edict_t) -> ()>,
    pub checkattack: Option::<unsafe extern "C" fn(*mut edict_t) -> qboolean>,
    pub pausetime: libc::c_float,
    pub attack_finished: libc::c_float,
    pub saved_goal: vec3_t,
    pub search_time: libc::c_float,
    pub trail_time: libc::c_float,
    pub last_sighting: vec3_t,
    pub attack_state: libc::c_int,
    pub lefty: libc::c_int,
    pub idle_time: libc::c_float,
    pub linkcount: libc::c_int,
    pub power_armor_type: libc::c_int,
    pub power_armor_power: libc::c_int,
}
pub type edict_t = edict_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mmove_t {
    pub firstframe: libc::c_int,
    pub lastframe: libc::c_int,
    pub frame: *mut mframe_t,
    pub endfunc: Option::<unsafe extern "C" fn(*mut edict_t) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mframe_t {
    pub aifunc: Option::<unsafe extern "C" fn(*mut edict_t, libc::c_float) -> ()>,
    pub dist: libc::c_float,
    pub thinkfunc: Option::<unsafe extern "C" fn(*mut edict_t) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct moveinfo_t {
    pub start_origin: vec3_t,
    pub start_angles: vec3_t,
    pub end_origin: vec3_t,
    pub end_angles: vec3_t,
    pub sound_start: libc::c_int,
    pub sound_middle: libc::c_int,
    pub sound_end: libc::c_int,
    pub accel: libc::c_float,
    pub speed: libc::c_float,
    pub decel: libc::c_float,
    pub distance: libc::c_float,
    pub wait: libc::c_float,
    pub state: libc::c_int,
    pub dir: vec3_t,
    pub current_speed: libc::c_float,
    pub move_speed: libc::c_float,
    pub next_speed: libc::c_float,
    pub remaining_distance: libc::c_float,
    pub decel_distance: libc::c_float,
    pub endfunc: Option::<unsafe extern "C" fn(*mut edict_t) -> ()>,
}
pub type gitem_t = gitem_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gitem_s {
    pub classname: *mut libc::c_char,
    pub pickup: Option::<unsafe extern "C" fn(*mut edict_s, *mut edict_s) -> qboolean>,
    pub use_0: Option::<unsafe extern "C" fn(*mut edict_s, *mut gitem_s) -> ()>,
    pub drop: Option::<unsafe extern "C" fn(*mut edict_s, *mut gitem_s) -> ()>,
    pub weaponthink: Option::<unsafe extern "C" fn(*mut edict_s) -> ()>,
    pub pickup_sound: *mut libc::c_char,
    pub world_model: *mut libc::c_char,
    pub world_model_flags: libc::c_int,
    pub view_model: *mut libc::c_char,
    pub icon: *mut libc::c_char,
    pub pickup_name: *mut libc::c_char,
    pub count_width: libc::c_int,
    pub quantity: libc::c_int,
    pub ammo: *mut libc::c_char,
    pub flags: libc::c_int,
    pub weapmodel: libc::c_int,
    pub info: *mut libc::c_void,
    pub tag: libc::c_int,
    pub precaches: *mut libc::c_char,
}
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
    pub pers: client_persistant_t,
    pub resp: client_respawn_t,
    pub old_pmove: pmove_state_t,
    pub showscores: qboolean,
    pub showinventory: qboolean,
    pub showhelp: qboolean,
    pub showhelpicon: qboolean,
    pub ammo_index: libc::c_int,
    pub buttons: libc::c_int,
    pub oldbuttons: libc::c_int,
    pub latched_buttons: libc::c_int,
    pub weapon_thunk: qboolean,
    pub newweapon: *mut gitem_t,
    pub damage_armor: libc::c_int,
    pub damage_parmor: libc::c_int,
    pub damage_blood: libc::c_int,
    pub damage_knockback: libc::c_int,
    pub damage_from: vec3_t,
    pub killer_yaw: libc::c_float,
    pub weaponstate: weaponstate_t,
    pub kick_angles: vec3_t,
    pub kick_origin: vec3_t,
    pub v_dmg_roll: libc::c_float,
    pub v_dmg_pitch: libc::c_float,
    pub v_dmg_time: libc::c_float,
    pub fall_time: libc::c_float,
    pub fall_value: libc::c_float,
    pub damage_alpha: libc::c_float,
    pub bonus_alpha: libc::c_float,
    pub damage_blend: vec3_t,
    pub v_angle: vec3_t,
    pub bobtime: libc::c_float,
    pub oldviewangles: vec3_t,
    pub oldvelocity: vec3_t,
    pub next_drown_time: libc::c_float,
    pub old_waterlevel: libc::c_int,
    pub breather_sound: libc::c_int,
    pub machinegun_shots: libc::c_int,
    pub anim_end: libc::c_int,
    pub anim_priority: libc::c_int,
    pub anim_duck: qboolean,
    pub anim_run: qboolean,
    pub quad_framenum: libc::c_float,
    pub invincible_framenum: libc::c_float,
    pub breather_framenum: libc::c_float,
    pub enviro_framenum: libc::c_float,
    pub grenade_blew_up: qboolean,
    pub grenade_time: libc::c_float,
    pub silencer_shots: libc::c_int,
    pub weapon_sound: libc::c_int,
    pub pickup_msg_time: libc::c_float,
    pub flood_locktill: libc::c_float,
    pub flood_when: [libc::c_float; 10],
    pub flood_whenhead: libc::c_int,
    pub respawn_time: libc::c_float,
    pub chase_target: *mut edict_t,
    pub update_chase: qboolean,
}
pub type weaponstate_t = libc::c_uint;
pub const WEAPON_FIRING: weaponstate_t = 3;
pub const WEAPON_DROPPING: weaponstate_t = 2;
pub const WEAPON_ACTIVATING: weaponstate_t = 1;
pub const WEAPON_READY: weaponstate_t = 0;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct client_respawn_t {
    pub coop_respawn: client_persistant_t,
    pub enterframe: libc::c_int,
    pub score: libc::c_int,
    pub cmd_angles: vec3_t,
    pub spectator: qboolean,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct client_persistant_t {
    pub userinfo: [libc::c_char; 512],
    pub netname: [libc::c_char; 16],
    pub hand: libc::c_int,
    pub connected: qboolean,
    pub health: libc::c_int,
    pub max_health: libc::c_int,
    pub savedFlags: libc::c_int,
    pub selected_item: libc::c_int,
    pub inventory: [libc::c_int; 256],
    pub max_bullets: libc::c_int,
    pub max_shells: libc::c_int,
    pub max_rockets: libc::c_int,
    pub max_grenades: libc::c_int,
    pub max_cells: libc::c_int,
    pub max_slugs: libc::c_int,
    pub weapon: *mut gitem_t,
    pub lastweapon: *mut gitem_t,
    pub power_cubes: libc::c_int,
    pub score: libc::c_int,
    pub game_helpchanged: libc::c_int,
    pub helpchanged: libc::c_int,
    pub spectator: qboolean,
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
pub struct ipfilter_t {
    pub mask: libc::c_uint,
    pub compare: libc::c_uint,
}
#[no_mangle]
pub unsafe extern "C" fn Svcmd_Test_f() {
    (gi.cprintf)
        .expect(
            "non-null function pointer",
        )(
        0 as *mut edict_t,
        2 as libc::c_int,
        b"Svcmd_Test_f()\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
}
#[no_mangle]
pub static mut ipfilters: [ipfilter_t; 1024] = [ipfilter_t {
    mask: 0,
    compare: 0,
}; 1024];
#[no_mangle]
pub static mut numipfilters: libc::c_int = 0;
unsafe extern "C" fn StringToFilter(
    mut s: *mut libc::c_char,
    mut f: *mut ipfilter_t,
) -> qboolean {
    let mut num: [libc::c_char; 128] = [0; 128];
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut b: [byte; 4] = [0; 4];
    let mut m: [byte; 4] = [0; 4];
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        b[i as usize] = 0 as libc::c_int as byte;
        m[i as usize] = 0 as libc::c_int as byte;
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        if (*s as libc::c_int) < '0' as i32 || *s as libc::c_int > '9' as i32 {
            (gi.cprintf)
                .expect(
                    "non-null function pointer",
                )(
                0 as *mut edict_t,
                2 as libc::c_int,
                b"Bad filter address: %s\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                s,
            );
            return false_0;
        }
        j = 0 as libc::c_int;
        while *s as libc::c_int >= '0' as i32 && *s as libc::c_int <= '9' as i32 {
            let fresh0 = s;
            s = s.offset(1);
            let fresh1 = j;
            j = j + 1;
            num[fresh1 as usize] = *fresh0;
        }
        num[j as usize] = 0 as libc::c_int as libc::c_char;
        b[i as usize] = atoi(num.as_mut_ptr()) as byte;
        if b[i as usize] as libc::c_int != 0 as libc::c_int {
            m[i as usize] = 255 as libc::c_int as byte;
        }
        if *s == 0 {
            break;
        }
        s = s.offset(1);
        i += 1;
    }
    (*f).mask = *(m.as_mut_ptr() as *mut libc::c_uint);
    (*f).compare = *(b.as_mut_ptr() as *mut libc::c_uint);
    return true_0;
}
#[no_mangle]
pub unsafe extern "C" fn SV_FilterPacket(mut from: *mut libc::c_char) -> qboolean {
    let mut i: libc::c_int = 0;
    let mut in_0: libc::c_uint = 0;
    let mut m: [byte; 4] = [0; 4];
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    i = 0 as libc::c_int;
    p = from;
    while *p as libc::c_int != 0 && i < 4 as libc::c_int {
        m[i as usize] = 0 as libc::c_int as byte;
        while *p as libc::c_int >= '0' as i32 && *p as libc::c_int <= '9' as i32 {
            m[i
                as usize] = (m[i as usize] as libc::c_int * 10 as libc::c_int
                + (*p as libc::c_int - '0' as i32)) as byte;
            p = p.offset(1);
        }
        if *p == 0 || *p as libc::c_int == ':' as i32 {
            break;
        }
        i += 1;
        p = p.offset(1);
    }
    in_0 = *(m.as_mut_ptr() as *mut libc::c_uint);
    i = 0 as libc::c_int;
    while i < numipfilters {
        if in_0 & ipfilters[i as usize].mask == ipfilters[i as usize].compare {
            return (*filterban).value as libc::c_int as qboolean;
        }
        i += 1;
    }
    return ((*filterban).value == 0.) as libc::c_int as qboolean;
}
#[no_mangle]
pub unsafe extern "C" fn SVCmd_AddIP_f() {
    let mut i: libc::c_int = 0;
    if (gi.argc).expect("non-null function pointer")() < 3 as libc::c_int {
        (gi.cprintf)
            .expect(
                "non-null function pointer",
            )(
            0 as *mut edict_t,
            2 as libc::c_int,
            b"Usage:  addip <ip-mask>\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return;
    }
    i = 0 as libc::c_int;
    while i < numipfilters {
        if ipfilters[i as usize].compare == 0xffffffff as libc::c_uint {
            break;
        }
        i += 1;
    }
    if i == numipfilters {
        if numipfilters == 1024 as libc::c_int {
            (gi.cprintf)
                .expect(
                    "non-null function pointer",
                )(
                0 as *mut edict_t,
                2 as libc::c_int,
                b"IP filter list is full\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
            return;
        }
        numipfilters += 1;
    }
    if StringToFilter(
        (gi.argv).expect("non-null function pointer")(2 as libc::c_int),
        &mut *ipfilters.as_mut_ptr().offset(i as isize),
    ) as u64 == 0
    {
        ipfilters[i as usize].compare = 0xffffffff as libc::c_uint;
    }
}
#[no_mangle]
pub unsafe extern "C" fn SVCmd_RemoveIP_f() {
    let mut f: ipfilter_t = ipfilter_t { mask: 0, compare: 0 };
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    if (gi.argc).expect("non-null function pointer")() < 3 as libc::c_int {
        (gi.cprintf)
            .expect(
                "non-null function pointer",
            )(
            0 as *mut edict_t,
            2 as libc::c_int,
            b"Usage:  sv removeip <ip-mask>\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return;
    }
    if StringToFilter(
        (gi.argv).expect("non-null function pointer")(2 as libc::c_int),
        &mut f,
    ) as u64 == 0
    {
        return;
    }
    i = 0 as libc::c_int;
    while i < numipfilters {
        if ipfilters[i as usize].mask == f.mask
            && ipfilters[i as usize].compare == f.compare
        {
            j = i + 1 as libc::c_int;
            while j < numipfilters {
                ipfilters[(j - 1 as libc::c_int) as usize] = ipfilters[j as usize];
                j += 1;
            }
            numipfilters -= 1;
            (gi.cprintf)
                .expect(
                    "non-null function pointer",
                )(
                0 as *mut edict_t,
                2 as libc::c_int,
                b"Removed.\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            return;
        }
        i += 1;
    }
    (gi.cprintf)
        .expect(
            "non-null function pointer",
        )(
        0 as *mut edict_t,
        2 as libc::c_int,
        b"Didn't find %s.\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        (gi.argv).expect("non-null function pointer")(2 as libc::c_int),
    );
}
#[no_mangle]
pub unsafe extern "C" fn SVCmd_ListIP_f() {
    let mut i: libc::c_int = 0;
    let mut b: [byte; 4] = [0; 4];
    (gi.cprintf)
        .expect(
            "non-null function pointer",
        )(
        0 as *mut edict_t,
        2 as libc::c_int,
        b"Filter list:\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    i = 0 as libc::c_int;
    while i < numipfilters {
        *(b.as_mut_ptr() as *mut libc::c_uint) = ipfilters[i as usize].compare;
        (gi.cprintf)
            .expect(
                "non-null function pointer",
            )(
            0 as *mut edict_t,
            2 as libc::c_int,
            b"%3i.%3i.%3i.%3i\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            b[0 as libc::c_int as usize] as libc::c_int,
            b[1 as libc::c_int as usize] as libc::c_int,
            b[2 as libc::c_int as usize] as libc::c_int,
            b[3 as libc::c_int as usize] as libc::c_int,
        );
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn SVCmd_WriteIP_f() {
    let mut f: *mut FILE = 0 as *mut FILE;
    let mut name: [libc::c_char; 128] = [0; 128];
    let mut b: [byte; 4] = [0; 4];
    let mut i: libc::c_int = 0;
    let mut game: *mut cvar_t = 0 as *mut cvar_t;
    game = (gi.cvar)
        .expect(
            "non-null function pointer",
        )(
        b"game\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    if *(*game).string == 0 {
        sprintf(
            name.as_mut_ptr(),
            b"%s/listip.cfg\0" as *const u8 as *const libc::c_char,
            b"baseq2\0" as *const u8 as *const libc::c_char,
        );
    } else {
        sprintf(
            name.as_mut_ptr(),
            b"%s/listip.cfg\0" as *const u8 as *const libc::c_char,
            (*game).string,
        );
    }
    (gi.cprintf)
        .expect(
            "non-null function pointer",
        )(
        0 as *mut edict_t,
        2 as libc::c_int,
        b"Writing %s.\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        name.as_mut_ptr(),
    );
    f = fopen(name.as_mut_ptr(), b"wb\0" as *const u8 as *const libc::c_char);
    if f.is_null() {
        (gi.cprintf)
            .expect(
                "non-null function pointer",
            )(
            0 as *mut edict_t,
            2 as libc::c_int,
            b"Couldn't open %s\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            name.as_mut_ptr(),
        );
        return;
    }
    fprintf(
        f,
        b"set filterban %d\n\0" as *const u8 as *const libc::c_char,
        (*filterban).value as libc::c_int,
    );
    i = 0 as libc::c_int;
    while i < numipfilters {
        *(b.as_mut_ptr() as *mut libc::c_uint) = ipfilters[i as usize].compare;
        fprintf(
            f,
            b"sv addip %i.%i.%i.%i\n\0" as *const u8 as *const libc::c_char,
            b[0 as libc::c_int as usize] as libc::c_int,
            b[1 as libc::c_int as usize] as libc::c_int,
            b[2 as libc::c_int as usize] as libc::c_int,
            b[3 as libc::c_int as usize] as libc::c_int,
        );
        i += 1;
    }
    fclose(f);
}
#[no_mangle]
pub unsafe extern "C" fn ServerCommand() {
    let mut cmd: *mut libc::c_char = 0 as *mut libc::c_char;
    cmd = (gi.argv).expect("non-null function pointer")(1 as libc::c_int);
    if Q_stricmp(cmd, b"test\0" as *const u8 as *const libc::c_char as *mut libc::c_char)
        == 0 as libc::c_int
    {
        Svcmd_Test_f();
    } else if Q_stricmp(
        cmd,
        b"addip\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) == 0 as libc::c_int
    {
        SVCmd_AddIP_f();
    } else if Q_stricmp(
        cmd,
        b"removeip\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) == 0 as libc::c_int
    {
        SVCmd_RemoveIP_f();
    } else if Q_stricmp(
        cmd,
        b"listip\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) == 0 as libc::c_int
    {
        SVCmd_ListIP_f();
    } else if Q_stricmp(
        cmd,
        b"writeip\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) == 0 as libc::c_int
    {
        SVCmd_WriteIP_f();
    } else {
        (gi.cprintf)
            .expect(
                "non-null function pointer",
            )(
            0 as *mut edict_t,
            2 as libc::c_int,
            b"Unknown server command \"%s\"\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            cmd,
        );
    };
}
