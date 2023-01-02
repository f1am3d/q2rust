#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    static mut vec3_origin: vec3_t;
    fn VectorMA(
        veca: *mut vec_t,
        scale: libc::c_float,
        vecb: *mut vec_t,
        vecc: *mut vec_t,
    );
    fn VectorCompare(v1: *mut vec_t, v2: *mut vec_t) -> libc::c_int;
    fn CrossProduct(v1: *mut vec_t, v2: *mut vec_t, cross: *mut vec_t);
    fn VectorScale(in_0: *mut vec_t, scale: vec_t, out: *mut vec_t);
    fn AngleVectors(
        angles: *mut vec_t,
        forward: *mut vec_t,
        right: *mut vec_t,
        up: *mut vec_t,
    );
    fn M_CheckBottom(ent: *mut edict_t) -> qboolean;
    static mut level: level_locals_t;
    static mut gi: game_import_t;
    static mut globals: game_export_t;
    static mut g_edicts: *mut edict_t;
    static mut sv_gravity: *mut cvar_t;
    static mut sv_maxvelocity: *mut cvar_t;
    fn G_TouchTriggers(ent: *mut edict_t);
    fn M_CheckGround(ent: *mut edict_t);
}
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
pub type C2RustUnnamed = libc::c_uint;
pub const MOVETYPE_BOUNCE: C2RustUnnamed = 9;
pub const MOVETYPE_FLYMISSILE: C2RustUnnamed = 8;
pub const MOVETYPE_TOSS: C2RustUnnamed = 7;
pub const MOVETYPE_FLY: C2RustUnnamed = 6;
pub const MOVETYPE_STEP: C2RustUnnamed = 5;
pub const MOVETYPE_WALK: C2RustUnnamed = 4;
pub const MOVETYPE_STOP: C2RustUnnamed = 3;
pub const MOVETYPE_PUSH: C2RustUnnamed = 2;
pub const MOVETYPE_NOCLIP: C2RustUnnamed = 1;
pub const MOVETYPE_NONE: C2RustUnnamed = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct level_locals_t {
    pub framenum: libc::c_int,
    pub time: libc::c_float,
    pub level_name: [libc::c_char; 64],
    pub mapname: [libc::c_char; 64],
    pub nextmap: [libc::c_char; 64],
    pub intermissiontime: libc::c_float,
    pub changemap: *mut libc::c_char,
    pub exitintermission: libc::c_int,
    pub intermission_origin: vec3_t,
    pub intermission_angle: vec3_t,
    pub sight_client: *mut edict_t,
    pub sight_entity: *mut edict_t,
    pub sight_entity_framenum: libc::c_int,
    pub sound_entity: *mut edict_t,
    pub sound_entity_framenum: libc::c_int,
    pub sound2_entity: *mut edict_t,
    pub sound2_entity_framenum: libc::c_int,
    pub pic_health: libc::c_int,
    pub total_secrets: libc::c_int,
    pub found_secrets: libc::c_int,
    pub total_goals: libc::c_int,
    pub found_goals: libc::c_int,
    pub total_monsters: libc::c_int,
    pub killed_monsters: libc::c_int,
    pub current_entity: *mut edict_t,
    pub body_que: libc::c_int,
    pub power_cubes: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pushed_t {
    pub ent: *mut edict_t,
    pub origin: vec3_t,
    pub angles: vec3_t,
    pub deltayaw: libc::c_float,
}
#[no_mangle]
pub unsafe extern "C" fn SV_TestEntityPosition(mut ent: *mut edict_t) -> *mut edict_t {
    let mut trace: trace_t = trace_t {
        allsolid: false_0,
        startsolid: false_0,
        fraction: 0.,
        endpos: [0.; 3],
        plane: cplane_t {
            normal: [0.; 3],
            dist: 0.,
            type_0: 0,
            signbits: 0,
            pad: [0; 2],
        },
        surface: 0 as *mut csurface_t,
        contents: 0,
        ent: 0 as *mut edict_s,
    };
    let mut mask: libc::c_int = 0;
    if (*ent).clipmask != 0 {
        mask = (*ent).clipmask;
    } else {
        mask = 1 as libc::c_int | 2 as libc::c_int;
    }
    trace = (gi.trace)
        .expect(
            "non-null function pointer",
        )(
        ((*ent).s.origin).as_mut_ptr(),
        ((*ent).mins).as_mut_ptr(),
        ((*ent).maxs).as_mut_ptr(),
        ((*ent).s.origin).as_mut_ptr(),
        ent,
        mask,
    );
    if trace.startsolid as u64 != 0 {
        return g_edicts;
    }
    return 0 as *mut edict_t;
}
#[no_mangle]
pub unsafe extern "C" fn SV_CheckVelocity(mut ent: *mut edict_t) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        if (*ent).velocity[i as usize] > (*sv_maxvelocity).value {
            (*ent).velocity[i as usize] = (*sv_maxvelocity).value;
        } else if (*ent).velocity[i as usize] < -(*sv_maxvelocity).value {
            (*ent).velocity[i as usize] = -(*sv_maxvelocity).value;
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn SV_RunThink(mut ent: *mut edict_t) -> qboolean {
    let mut thinktime: libc::c_float = 0.;
    thinktime = (*ent).nextthink;
    if thinktime <= 0 as libc::c_int as libc::c_float {
        return true_0;
    }
    if thinktime as libc::c_double > level.time as libc::c_double + 0.001f64 {
        return true_0;
    }
    (*ent).nextthink = 0 as libc::c_int as libc::c_float;
    if ((*ent).think).is_none() {
        (gi.error)
            .expect(
                "non-null function pointer",
            )(
            b"NULL ent->think\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
    ((*ent).think).expect("non-null function pointer")(ent);
    return false_0;
}
#[no_mangle]
pub unsafe extern "C" fn SV_Impact(mut e1: *mut edict_t, mut trace: *mut trace_t) {
    let mut e2: *mut edict_t = 0 as *mut edict_t;
    e2 = (*trace).ent;
    if ((*e1).touch).is_some()
        && (*e1).solid as libc::c_uint != SOLID_NOT as libc::c_int as libc::c_uint
    {
        ((*e1).touch)
            .expect(
                "non-null function pointer",
            )(e1, e2, &mut (*trace).plane, (*trace).surface);
    }
    if ((*e2).touch).is_some()
        && (*e2).solid as libc::c_uint != SOLID_NOT as libc::c_int as libc::c_uint
    {
        ((*e2).touch)
            .expect(
                "non-null function pointer",
            )(e2, e1, 0 as *mut cplane_t, 0 as *mut csurface_t);
    }
}
#[no_mangle]
pub unsafe extern "C" fn ClipVelocity(
    mut in_0: *mut vec_t,
    mut normal: *mut vec_t,
    mut out: *mut vec_t,
    mut overbounce: libc::c_float,
) -> libc::c_int {
    let mut backoff: libc::c_float = 0.;
    let mut change: libc::c_float = 0.;
    let mut i: libc::c_int = 0;
    let mut blocked: libc::c_int = 0;
    blocked = 0 as libc::c_int;
    if *normal.offset(2 as libc::c_int as isize) > 0 as libc::c_int as libc::c_float {
        blocked |= 1 as libc::c_int;
    }
    if *normal.offset(2 as libc::c_int as isize) == 0. {
        blocked |= 2 as libc::c_int;
    }
    backoff = (*in_0.offset(0 as libc::c_int as isize)
        * *normal.offset(0 as libc::c_int as isize)
        + *in_0.offset(1 as libc::c_int as isize)
            * *normal.offset(1 as libc::c_int as isize)
        + *in_0.offset(2 as libc::c_int as isize)
            * *normal.offset(2 as libc::c_int as isize)) * overbounce;
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        change = *normal.offset(i as isize) * backoff;
        *out.offset(i as isize) = *in_0.offset(i as isize) - change;
        if *out.offset(i as isize) as libc::c_double > -0.1f64
            && (*out.offset(i as isize) as libc::c_double) < 0.1f64
        {
            *out.offset(i as isize) = 0 as libc::c_int as vec_t;
        }
        i += 1;
    }
    return blocked;
}
#[no_mangle]
pub unsafe extern "C" fn SV_FlyMove(
    mut ent: *mut edict_t,
    mut time: libc::c_float,
    mut mask: libc::c_int,
) -> libc::c_int {
    let mut hit: *mut edict_t = 0 as *mut edict_t;
    let mut bumpcount: libc::c_int = 0;
    let mut numbumps: libc::c_int = 0;
    let mut dir: vec3_t = [0.; 3];
    let mut d: libc::c_float = 0.;
    let mut numplanes: libc::c_int = 0;
    let mut planes: [vec3_t; 5] = [[0.; 3]; 5];
    let mut primal_velocity: vec3_t = [0.; 3];
    let mut original_velocity: vec3_t = [0.; 3];
    let mut new_velocity: vec3_t = [0.; 3];
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut trace: trace_t = trace_t {
        allsolid: false_0,
        startsolid: false_0,
        fraction: 0.,
        endpos: [0.; 3],
        plane: cplane_t {
            normal: [0.; 3],
            dist: 0.,
            type_0: 0,
            signbits: 0,
            pad: [0; 2],
        },
        surface: 0 as *mut csurface_t,
        contents: 0,
        ent: 0 as *mut edict_s,
    };
    let mut end: vec3_t = [0.; 3];
    let mut time_left: libc::c_float = 0.;
    let mut blocked: libc::c_int = 0;
    numbumps = 4 as libc::c_int;
    blocked = 0 as libc::c_int;
    original_velocity[0 as libc::c_int
        as usize] = (*ent).velocity[0 as libc::c_int as usize];
    original_velocity[1 as libc::c_int
        as usize] = (*ent).velocity[1 as libc::c_int as usize];
    original_velocity[2 as libc::c_int
        as usize] = (*ent).velocity[2 as libc::c_int as usize];
    primal_velocity[0 as libc::c_int
        as usize] = (*ent).velocity[0 as libc::c_int as usize];
    primal_velocity[1 as libc::c_int
        as usize] = (*ent).velocity[1 as libc::c_int as usize];
    primal_velocity[2 as libc::c_int
        as usize] = (*ent).velocity[2 as libc::c_int as usize];
    numplanes = 0 as libc::c_int;
    time_left = time;
    let ref mut fresh0 = (*ent).groundentity;
    *fresh0 = 0 as *mut edict_t;
    bumpcount = 0 as libc::c_int;
    while bumpcount < numbumps {
        i = 0 as libc::c_int;
        while i < 3 as libc::c_int {
            end[i
                as usize] = (*ent).s.origin[i as usize]
                + time_left * (*ent).velocity[i as usize];
            i += 1;
        }
        trace = (gi.trace)
            .expect(
                "non-null function pointer",
            )(
            ((*ent).s.origin).as_mut_ptr(),
            ((*ent).mins).as_mut_ptr(),
            ((*ent).maxs).as_mut_ptr(),
            end.as_mut_ptr(),
            ent,
            mask,
        );
        if trace.allsolid as u64 != 0 {
            (*ent)
                .velocity[0 as libc::c_int
                as usize] = vec3_origin[0 as libc::c_int as usize];
            (*ent)
                .velocity[1 as libc::c_int
                as usize] = vec3_origin[1 as libc::c_int as usize];
            (*ent)
                .velocity[2 as libc::c_int
                as usize] = vec3_origin[2 as libc::c_int as usize];
            return 3 as libc::c_int;
        }
        if trace.fraction > 0 as libc::c_int as libc::c_float {
            (*ent)
                .s
                .origin[0 as libc::c_int
                as usize] = trace.endpos[0 as libc::c_int as usize];
            (*ent)
                .s
                .origin[1 as libc::c_int
                as usize] = trace.endpos[1 as libc::c_int as usize];
            (*ent)
                .s
                .origin[2 as libc::c_int
                as usize] = trace.endpos[2 as libc::c_int as usize];
            original_velocity[0 as libc::c_int
                as usize] = (*ent).velocity[0 as libc::c_int as usize];
            original_velocity[1 as libc::c_int
                as usize] = (*ent).velocity[1 as libc::c_int as usize];
            original_velocity[2 as libc::c_int
                as usize] = (*ent).velocity[2 as libc::c_int as usize];
            numplanes = 0 as libc::c_int;
        }
        if trace.fraction == 1 as libc::c_int as libc::c_float {
            break;
        }
        hit = trace.ent;
        if trace.plane.normal[2 as libc::c_int as usize] as libc::c_double > 0.7f64 {
            blocked |= 1 as libc::c_int;
            if (*hit).solid as libc::c_uint == SOLID_BSP as libc::c_int as libc::c_uint {
                let ref mut fresh1 = (*ent).groundentity;
                *fresh1 = hit;
                (*ent).groundentity_linkcount = (*hit).linkcount;
            }
        }
        if trace.plane.normal[2 as libc::c_int as usize] == 0. {
            blocked |= 2 as libc::c_int;
        }
        SV_Impact(ent, &mut trace);
        if (*ent).inuse as u64 == 0 {
            break;
        }
        time_left -= time_left * trace.fraction;
        if numplanes >= 5 as libc::c_int {
            (*ent)
                .velocity[0 as libc::c_int
                as usize] = vec3_origin[0 as libc::c_int as usize];
            (*ent)
                .velocity[1 as libc::c_int
                as usize] = vec3_origin[1 as libc::c_int as usize];
            (*ent)
                .velocity[2 as libc::c_int
                as usize] = vec3_origin[2 as libc::c_int as usize];
            return 3 as libc::c_int;
        }
        planes[numplanes
            as usize][0 as libc::c_int
            as usize] = trace.plane.normal[0 as libc::c_int as usize];
        planes[numplanes
            as usize][1 as libc::c_int
            as usize] = trace.plane.normal[1 as libc::c_int as usize];
        planes[numplanes
            as usize][2 as libc::c_int
            as usize] = trace.plane.normal[2 as libc::c_int as usize];
        numplanes += 1;
        i = 0 as libc::c_int;
        while i < numplanes {
            ClipVelocity(
                original_velocity.as_mut_ptr(),
                (planes[i as usize]).as_mut_ptr(),
                new_velocity.as_mut_ptr(),
                1 as libc::c_int as libc::c_float,
            );
            j = 0 as libc::c_int;
            while j < numplanes {
                if j != i
                    && VectorCompare(
                        (planes[i as usize]).as_mut_ptr(),
                        (planes[j as usize]).as_mut_ptr(),
                    ) == 0
                {
                    if new_velocity[0 as libc::c_int as usize]
                        * planes[j as usize][0 as libc::c_int as usize]
                        + new_velocity[1 as libc::c_int as usize]
                            * planes[j as usize][1 as libc::c_int as usize]
                        + new_velocity[2 as libc::c_int as usize]
                            * planes[j as usize][2 as libc::c_int as usize]
                        < 0 as libc::c_int as libc::c_float
                    {
                        break;
                    }
                }
                j += 1;
            }
            if j == numplanes {
                break;
            }
            i += 1;
        }
        if i != numplanes {
            (*ent)
                .velocity[0 as libc::c_int
                as usize] = new_velocity[0 as libc::c_int as usize];
            (*ent)
                .velocity[1 as libc::c_int
                as usize] = new_velocity[1 as libc::c_int as usize];
            (*ent)
                .velocity[2 as libc::c_int
                as usize] = new_velocity[2 as libc::c_int as usize];
        } else {
            if numplanes != 2 as libc::c_int {
                (*ent)
                    .velocity[0 as libc::c_int
                    as usize] = vec3_origin[0 as libc::c_int as usize];
                (*ent)
                    .velocity[1 as libc::c_int
                    as usize] = vec3_origin[1 as libc::c_int as usize];
                (*ent)
                    .velocity[2 as libc::c_int
                    as usize] = vec3_origin[2 as libc::c_int as usize];
                return 7 as libc::c_int;
            }
            CrossProduct(
                (planes[0 as libc::c_int as usize]).as_mut_ptr(),
                (planes[1 as libc::c_int as usize]).as_mut_ptr(),
                dir.as_mut_ptr(),
            );
            d = dir[0 as libc::c_int as usize]
                * (*ent).velocity[0 as libc::c_int as usize]
                + dir[1 as libc::c_int as usize]
                    * (*ent).velocity[1 as libc::c_int as usize]
                + dir[2 as libc::c_int as usize]
                    * (*ent).velocity[2 as libc::c_int as usize];
            VectorScale(dir.as_mut_ptr(), d, ((*ent).velocity).as_mut_ptr());
        }
        if (*ent).velocity[0 as libc::c_int as usize]
            * primal_velocity[0 as libc::c_int as usize]
            + (*ent).velocity[1 as libc::c_int as usize]
                * primal_velocity[1 as libc::c_int as usize]
            + (*ent).velocity[2 as libc::c_int as usize]
                * primal_velocity[2 as libc::c_int as usize]
            <= 0 as libc::c_int as libc::c_float
        {
            (*ent)
                .velocity[0 as libc::c_int
                as usize] = vec3_origin[0 as libc::c_int as usize];
            (*ent)
                .velocity[1 as libc::c_int
                as usize] = vec3_origin[1 as libc::c_int as usize];
            (*ent)
                .velocity[2 as libc::c_int
                as usize] = vec3_origin[2 as libc::c_int as usize];
            return blocked;
        }
        bumpcount += 1;
    }
    return blocked;
}
#[no_mangle]
pub unsafe extern "C" fn SV_AddGravity(mut ent: *mut edict_t) {
    let ref mut fresh2 = (*ent).velocity[2 as libc::c_int as usize];
    *fresh2 = (*fresh2 as libc::c_double
        - ((*ent).gravity * (*sv_gravity).value) as libc::c_double * 0.1f64) as vec_t;
}
#[no_mangle]
pub unsafe extern "C" fn SV_PushEntity(
    mut ent: *mut edict_t,
    mut push: *mut vec_t,
) -> trace_t {
    let mut trace: trace_t = trace_t {
        allsolid: false_0,
        startsolid: false_0,
        fraction: 0.,
        endpos: [0.; 3],
        plane: cplane_t {
            normal: [0.; 3],
            dist: 0.,
            type_0: 0,
            signbits: 0,
            pad: [0; 2],
        },
        surface: 0 as *mut csurface_t,
        contents: 0,
        ent: 0 as *mut edict_s,
    };
    let mut start: vec3_t = [0.; 3];
    let mut end: vec3_t = [0.; 3];
    let mut mask: libc::c_int = 0;
    start[0 as libc::c_int as usize] = (*ent).s.origin[0 as libc::c_int as usize];
    start[1 as libc::c_int as usize] = (*ent).s.origin[1 as libc::c_int as usize];
    start[2 as libc::c_int as usize] = (*ent).s.origin[2 as libc::c_int as usize];
    end[0 as libc::c_int
        as usize] = start[0 as libc::c_int as usize]
        + *push.offset(0 as libc::c_int as isize);
    end[1 as libc::c_int
        as usize] = start[1 as libc::c_int as usize]
        + *push.offset(1 as libc::c_int as isize);
    end[2 as libc::c_int
        as usize] = start[2 as libc::c_int as usize]
        + *push.offset(2 as libc::c_int as isize);
    loop {
        if (*ent).clipmask != 0 {
            mask = (*ent).clipmask;
        } else {
            mask = 1 as libc::c_int | 2 as libc::c_int;
        }
        trace = (gi.trace)
            .expect(
                "non-null function pointer",
            )(
            start.as_mut_ptr(),
            ((*ent).mins).as_mut_ptr(),
            ((*ent).maxs).as_mut_ptr(),
            end.as_mut_ptr(),
            ent,
            mask,
        );
        (*ent)
            .s
            .origin[0 as libc::c_int as usize] = trace.endpos[0 as libc::c_int as usize];
        (*ent)
            .s
            .origin[1 as libc::c_int as usize] = trace.endpos[1 as libc::c_int as usize];
        (*ent)
            .s
            .origin[2 as libc::c_int as usize] = trace.endpos[2 as libc::c_int as usize];
        (gi.linkentity).expect("non-null function pointer")(ent);
        if !(trace.fraction as libc::c_double != 1.0f64) {
            break;
        }
        SV_Impact(ent, &mut trace);
        if !((*trace.ent).inuse as u64 == 0 && (*ent).inuse as libc::c_uint != 0) {
            break;
        }
        (*ent).s.origin[0 as libc::c_int as usize] = start[0 as libc::c_int as usize];
        (*ent).s.origin[1 as libc::c_int as usize] = start[1 as libc::c_int as usize];
        (*ent).s.origin[2 as libc::c_int as usize] = start[2 as libc::c_int as usize];
        (gi.linkentity).expect("non-null function pointer")(ent);
    }
    if (*ent).inuse as u64 != 0 {
        G_TouchTriggers(ent);
    }
    return trace;
}
#[no_mangle]
pub static mut pushed: [pushed_t; 1024] = [pushed_t {
    ent: 0 as *const edict_t as *mut edict_t,
    origin: [0.; 3],
    angles: [0.; 3],
    deltayaw: 0.,
}; 1024];
#[no_mangle]
pub static mut pushed_p: *mut pushed_t = 0 as *const pushed_t as *mut pushed_t;
#[no_mangle]
pub static mut obstacle: *mut edict_t = 0 as *const edict_t as *mut edict_t;
#[no_mangle]
pub unsafe extern "C" fn SV_Push(
    mut pusher: *mut edict_t,
    mut move_0: *mut vec_t,
    mut amove: *mut vec_t,
) -> qboolean {
    let mut i: libc::c_int = 0;
    let mut e: libc::c_int = 0;
    let mut check: *mut edict_t = 0 as *mut edict_t;
    let mut block: *mut edict_t = 0 as *mut edict_t;
    let mut mins: vec3_t = [0.; 3];
    let mut maxs: vec3_t = [0.; 3];
    let mut p: *mut pushed_t = 0 as *mut pushed_t;
    let mut org: vec3_t = [0.; 3];
    let mut org2: vec3_t = [0.; 3];
    let mut move2: vec3_t = [0.; 3];
    let mut forward: vec3_t = [0.; 3];
    let mut right: vec3_t = [0.; 3];
    let mut up: vec3_t = [0.; 3];
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        let mut temp: libc::c_float = 0.;
        temp = (*move_0.offset(i as isize) as libc::c_double * 8.0f64) as libc::c_float;
        if temp as libc::c_double > 0.0f64 {
            temp = (temp as libc::c_double + 0.5f64) as libc::c_float;
        } else {
            temp = (temp as libc::c_double - 0.5f64) as libc::c_float;
        }
        *move_0
            .offset(
                i as isize,
            ) = (0.125f64 * temp as libc::c_int as libc::c_double) as vec_t;
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        mins[i as usize] = (*pusher).absmin[i as usize] + *move_0.offset(i as isize);
        maxs[i as usize] = (*pusher).absmax[i as usize] + *move_0.offset(i as isize);
        i += 1;
    }
    org[0 as libc::c_int
        as usize] = vec3_origin[0 as libc::c_int as usize]
        - *amove.offset(0 as libc::c_int as isize);
    org[1 as libc::c_int
        as usize] = vec3_origin[1 as libc::c_int as usize]
        - *amove.offset(1 as libc::c_int as isize);
    org[2 as libc::c_int
        as usize] = vec3_origin[2 as libc::c_int as usize]
        - *amove.offset(2 as libc::c_int as isize);
    AngleVectors(
        org.as_mut_ptr(),
        forward.as_mut_ptr(),
        right.as_mut_ptr(),
        up.as_mut_ptr(),
    );
    let ref mut fresh3 = (*pushed_p).ent;
    *fresh3 = pusher;
    (*pushed_p)
        .origin[0 as libc::c_int
        as usize] = (*pusher).s.origin[0 as libc::c_int as usize];
    (*pushed_p)
        .origin[1 as libc::c_int
        as usize] = (*pusher).s.origin[1 as libc::c_int as usize];
    (*pushed_p)
        .origin[2 as libc::c_int
        as usize] = (*pusher).s.origin[2 as libc::c_int as usize];
    (*pushed_p)
        .angles[0 as libc::c_int
        as usize] = (*pusher).s.angles[0 as libc::c_int as usize];
    (*pushed_p)
        .angles[1 as libc::c_int
        as usize] = (*pusher).s.angles[1 as libc::c_int as usize];
    (*pushed_p)
        .angles[2 as libc::c_int
        as usize] = (*pusher).s.angles[2 as libc::c_int as usize];
    if !((*pusher).client).is_null() {
        (*pushed_p)
            .deltayaw = (*(*pusher).client)
            .ps
            .pmove
            .delta_angles[1 as libc::c_int as usize] as libc::c_float;
    }
    pushed_p = pushed_p.offset(1);
    (*pusher)
        .s
        .origin[0 as libc::c_int
        as usize] = (*pusher).s.origin[0 as libc::c_int as usize]
        + *move_0.offset(0 as libc::c_int as isize);
    (*pusher)
        .s
        .origin[1 as libc::c_int
        as usize] = (*pusher).s.origin[1 as libc::c_int as usize]
        + *move_0.offset(1 as libc::c_int as isize);
    (*pusher)
        .s
        .origin[2 as libc::c_int
        as usize] = (*pusher).s.origin[2 as libc::c_int as usize]
        + *move_0.offset(2 as libc::c_int as isize);
    (*pusher)
        .s
        .angles[0 as libc::c_int
        as usize] = (*pusher).s.angles[0 as libc::c_int as usize]
        + *amove.offset(0 as libc::c_int as isize);
    (*pusher)
        .s
        .angles[1 as libc::c_int
        as usize] = (*pusher).s.angles[1 as libc::c_int as usize]
        + *amove.offset(1 as libc::c_int as isize);
    (*pusher)
        .s
        .angles[2 as libc::c_int
        as usize] = (*pusher).s.angles[2 as libc::c_int as usize]
        + *amove.offset(2 as libc::c_int as isize);
    (gi.linkentity).expect("non-null function pointer")(pusher);
    check = g_edicts.offset(1 as libc::c_int as isize);
    let mut current_block_57: u64;
    e = 1 as libc::c_int;
    while e < globals.num_edicts {
        if !((*check).inuse as u64 == 0) {
            if !((*check).movetype == MOVETYPE_PUSH as libc::c_int
                || (*check).movetype == MOVETYPE_STOP as libc::c_int
                || (*check).movetype == MOVETYPE_NONE as libc::c_int
                || (*check).movetype == MOVETYPE_NOCLIP as libc::c_int)
            {
                if !((*check).area.prev).is_null() {
                    if (*check).groundentity != pusher {
                        if (*check).absmin[0 as libc::c_int as usize]
                            >= maxs[0 as libc::c_int as usize]
                            || (*check).absmin[1 as libc::c_int as usize]
                                >= maxs[1 as libc::c_int as usize]
                            || (*check).absmin[2 as libc::c_int as usize]
                                >= maxs[2 as libc::c_int as usize]
                            || (*check).absmax[0 as libc::c_int as usize]
                                <= mins[0 as libc::c_int as usize]
                            || (*check).absmax[1 as libc::c_int as usize]
                                <= mins[1 as libc::c_int as usize]
                            || (*check).absmax[2 as libc::c_int as usize]
                                <= mins[2 as libc::c_int as usize]
                        {
                            current_block_57 = 4068382217303356765;
                        } else if (SV_TestEntityPosition(check)).is_null() {
                            current_block_57 = 4068382217303356765;
                        } else {
                            current_block_57 = 7333393191927787629;
                        }
                    } else {
                        current_block_57 = 7333393191927787629;
                    }
                    match current_block_57 {
                        4068382217303356765 => {}
                        _ => {
                            if (*pusher).movetype == MOVETYPE_PUSH as libc::c_int
                                || (*check).groundentity == pusher
                            {
                                let ref mut fresh4 = (*pushed_p).ent;
                                *fresh4 = check;
                                (*pushed_p)
                                    .origin[0 as libc::c_int
                                    as usize] = (*check).s.origin[0 as libc::c_int as usize];
                                (*pushed_p)
                                    .origin[1 as libc::c_int
                                    as usize] = (*check).s.origin[1 as libc::c_int as usize];
                                (*pushed_p)
                                    .origin[2 as libc::c_int
                                    as usize] = (*check).s.origin[2 as libc::c_int as usize];
                                (*pushed_p)
                                    .angles[0 as libc::c_int
                                    as usize] = (*check).s.angles[0 as libc::c_int as usize];
                                (*pushed_p)
                                    .angles[1 as libc::c_int
                                    as usize] = (*check).s.angles[1 as libc::c_int as usize];
                                (*pushed_p)
                                    .angles[2 as libc::c_int
                                    as usize] = (*check).s.angles[2 as libc::c_int as usize];
                                pushed_p = pushed_p.offset(1);
                                (*check)
                                    .s
                                    .origin[0 as libc::c_int
                                    as usize] = (*check).s.origin[0 as libc::c_int as usize]
                                    + *move_0.offset(0 as libc::c_int as isize);
                                (*check)
                                    .s
                                    .origin[1 as libc::c_int
                                    as usize] = (*check).s.origin[1 as libc::c_int as usize]
                                    + *move_0.offset(1 as libc::c_int as isize);
                                (*check)
                                    .s
                                    .origin[2 as libc::c_int
                                    as usize] = (*check).s.origin[2 as libc::c_int as usize]
                                    + *move_0.offset(2 as libc::c_int as isize);
                                if !((*check).client).is_null() {
                                    let ref mut fresh5 = (*(*check).client)
                                        .ps
                                        .pmove
                                        .delta_angles[1 as libc::c_int as usize];
                                    *fresh5 = (*fresh5 as libc::c_float
                                        + *amove.offset(1 as libc::c_int as isize))
                                        as libc::c_short;
                                }
                                org[0 as libc::c_int
                                    as usize] = (*check).s.origin[0 as libc::c_int as usize]
                                    - (*pusher).s.origin[0 as libc::c_int as usize];
                                org[1 as libc::c_int
                                    as usize] = (*check).s.origin[1 as libc::c_int as usize]
                                    - (*pusher).s.origin[1 as libc::c_int as usize];
                                org[2 as libc::c_int
                                    as usize] = (*check).s.origin[2 as libc::c_int as usize]
                                    - (*pusher).s.origin[2 as libc::c_int as usize];
                                org2[0 as libc::c_int
                                    as usize] = org[0 as libc::c_int as usize]
                                    * forward[0 as libc::c_int as usize]
                                    + org[1 as libc::c_int as usize]
                                        * forward[1 as libc::c_int as usize]
                                    + org[2 as libc::c_int as usize]
                                        * forward[2 as libc::c_int as usize];
                                org2[1 as libc::c_int
                                    as usize] = -(org[0 as libc::c_int as usize]
                                    * right[0 as libc::c_int as usize]
                                    + org[1 as libc::c_int as usize]
                                        * right[1 as libc::c_int as usize]
                                    + org[2 as libc::c_int as usize]
                                        * right[2 as libc::c_int as usize]);
                                org2[2 as libc::c_int
                                    as usize] = org[0 as libc::c_int as usize]
                                    * up[0 as libc::c_int as usize]
                                    + org[1 as libc::c_int as usize]
                                        * up[1 as libc::c_int as usize]
                                    + org[2 as libc::c_int as usize]
                                        * up[2 as libc::c_int as usize];
                                move2[0 as libc::c_int
                                    as usize] = org2[0 as libc::c_int as usize]
                                    - org[0 as libc::c_int as usize];
                                move2[1 as libc::c_int
                                    as usize] = org2[1 as libc::c_int as usize]
                                    - org[1 as libc::c_int as usize];
                                move2[2 as libc::c_int
                                    as usize] = org2[2 as libc::c_int as usize]
                                    - org[2 as libc::c_int as usize];
                                (*check)
                                    .s
                                    .origin[0 as libc::c_int
                                    as usize] = (*check).s.origin[0 as libc::c_int as usize]
                                    + move2[0 as libc::c_int as usize];
                                (*check)
                                    .s
                                    .origin[1 as libc::c_int
                                    as usize] = (*check).s.origin[1 as libc::c_int as usize]
                                    + move2[1 as libc::c_int as usize];
                                (*check)
                                    .s
                                    .origin[2 as libc::c_int
                                    as usize] = (*check).s.origin[2 as libc::c_int as usize]
                                    + move2[2 as libc::c_int as usize];
                                if (*check).groundentity != pusher {
                                    let ref mut fresh6 = (*check).groundentity;
                                    *fresh6 = 0 as *mut edict_t;
                                }
                                block = SV_TestEntityPosition(check);
                                if block.is_null() {
                                    (gi.linkentity).expect("non-null function pointer")(check);
                                    current_block_57 = 4068382217303356765;
                                } else {
                                    (*check)
                                        .s
                                        .origin[0 as libc::c_int
                                        as usize] = (*check).s.origin[0 as libc::c_int as usize]
                                        - *move_0.offset(0 as libc::c_int as isize);
                                    (*check)
                                        .s
                                        .origin[1 as libc::c_int
                                        as usize] = (*check).s.origin[1 as libc::c_int as usize]
                                        - *move_0.offset(1 as libc::c_int as isize);
                                    (*check)
                                        .s
                                        .origin[2 as libc::c_int
                                        as usize] = (*check).s.origin[2 as libc::c_int as usize]
                                        - *move_0.offset(2 as libc::c_int as isize);
                                    block = SV_TestEntityPosition(check);
                                    if block.is_null() {
                                        pushed_p = pushed_p.offset(-1);
                                        current_block_57 = 4068382217303356765;
                                    } else {
                                        current_block_57 = 2520131295878969859;
                                    }
                                }
                            } else {
                                current_block_57 = 2520131295878969859;
                            }
                            match current_block_57 {
                                4068382217303356765 => {}
                                _ => {
                                    obstacle = check;
                                    p = pushed_p.offset(-(1 as libc::c_int as isize));
                                    while p >= pushed.as_mut_ptr() {
                                        (*(*p).ent)
                                            .s
                                            .origin[0 as libc::c_int
                                            as usize] = (*p).origin[0 as libc::c_int as usize];
                                        (*(*p).ent)
                                            .s
                                            .origin[1 as libc::c_int
                                            as usize] = (*p).origin[1 as libc::c_int as usize];
                                        (*(*p).ent)
                                            .s
                                            .origin[2 as libc::c_int
                                            as usize] = (*p).origin[2 as libc::c_int as usize];
                                        (*(*p).ent)
                                            .s
                                            .angles[0 as libc::c_int
                                            as usize] = (*p).angles[0 as libc::c_int as usize];
                                        (*(*p).ent)
                                            .s
                                            .angles[1 as libc::c_int
                                            as usize] = (*p).angles[1 as libc::c_int as usize];
                                        (*(*p).ent)
                                            .s
                                            .angles[2 as libc::c_int
                                            as usize] = (*p).angles[2 as libc::c_int as usize];
                                        if !((*(*p).ent).client).is_null() {
                                            (*(*(*p).ent).client)
                                                .ps
                                                .pmove
                                                .delta_angles[1 as libc::c_int
                                                as usize] = (*p).deltayaw as libc::c_short;
                                        }
                                        (gi.linkentity)
                                            .expect("non-null function pointer")((*p).ent);
                                        p = p.offset(-1);
                                    }
                                    return false_0;
                                }
                            }
                        }
                    }
                }
            }
        }
        e += 1;
        check = check.offset(1);
    }
    p = pushed_p.offset(-(1 as libc::c_int as isize));
    while p >= pushed.as_mut_ptr() {
        G_TouchTriggers((*p).ent);
        p = p.offset(-1);
    }
    return true_0;
}
#[no_mangle]
pub unsafe extern "C" fn SV_Physics_Pusher(mut ent: *mut edict_t) {
    let mut move_0: vec3_t = [0.; 3];
    let mut amove: vec3_t = [0.; 3];
    let mut part: *mut edict_t = 0 as *mut edict_t;
    let mut mv: *mut edict_t = 0 as *mut edict_t;
    if (*ent).flags & 0x400 as libc::c_int != 0 {
        return;
    }
    pushed_p = pushed.as_mut_ptr();
    part = ent;
    while !part.is_null() {
        if (*part).velocity[0 as libc::c_int as usize] != 0.
            || (*part).velocity[1 as libc::c_int as usize] != 0.
            || (*part).velocity[2 as libc::c_int as usize] != 0.
            || (*part).avelocity[0 as libc::c_int as usize] != 0.
            || (*part).avelocity[1 as libc::c_int as usize] != 0.
            || (*part).avelocity[2 as libc::c_int as usize] != 0.
        {
            VectorScale(
                ((*part).velocity).as_mut_ptr(),
                0.1f64 as vec_t,
                move_0.as_mut_ptr(),
            );
            VectorScale(
                ((*part).avelocity).as_mut_ptr(),
                0.1f64 as vec_t,
                amove.as_mut_ptr(),
            );
            if SV_Push(part, move_0.as_mut_ptr(), amove.as_mut_ptr()) as u64 == 0 {
                break;
            }
        }
        part = (*part).teamchain;
    }
    if pushed_p
        > &mut *pushed.as_mut_ptr().offset(1024 as libc::c_int as isize) as *mut pushed_t
    {
        (gi.error)
            .expect(
                "non-null function pointer",
            )(
            0 as *mut libc::c_char,
            b"pushed_p > &pushed[MAX_EDICTS], memory corrupted\0" as *const u8
                as *const libc::c_char,
        );
    }
    if !part.is_null() {
        mv = ent;
        while !mv.is_null() {
            if (*mv).nextthink > 0 as libc::c_int as libc::c_float {
                let ref mut fresh7 = (*mv).nextthink;
                *fresh7 = (*fresh7 as libc::c_double + 0.1f64) as libc::c_float;
            }
            mv = (*mv).teamchain;
        }
        if ((*part).blocked).is_some() {
            ((*part).blocked).expect("non-null function pointer")(part, obstacle);
        }
    } else {
        part = ent;
        while !part.is_null() {
            SV_RunThink(part);
            part = (*part).teamchain;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn SV_Physics_None(mut ent: *mut edict_t) {
    SV_RunThink(ent);
}
#[no_mangle]
pub unsafe extern "C" fn SV_Physics_Noclip(mut ent: *mut edict_t) {
    if SV_RunThink(ent) as u64 == 0 {
        return;
    }
    VectorMA(
        ((*ent).s.angles).as_mut_ptr(),
        0.1f64 as libc::c_float,
        ((*ent).avelocity).as_mut_ptr(),
        ((*ent).s.angles).as_mut_ptr(),
    );
    VectorMA(
        ((*ent).s.origin).as_mut_ptr(),
        0.1f64 as libc::c_float,
        ((*ent).velocity).as_mut_ptr(),
        ((*ent).s.origin).as_mut_ptr(),
    );
    (gi.linkentity).expect("non-null function pointer")(ent);
}
#[no_mangle]
pub unsafe extern "C" fn SV_Physics_Toss(mut ent: *mut edict_t) {
    let mut trace: trace_t = trace_t {
        allsolid: false_0,
        startsolid: false_0,
        fraction: 0.,
        endpos: [0.; 3],
        plane: cplane_t {
            normal: [0.; 3],
            dist: 0.,
            type_0: 0,
            signbits: 0,
            pad: [0; 2],
        },
        surface: 0 as *mut csurface_t,
        contents: 0,
        ent: 0 as *mut edict_s,
    };
    let mut move_0: vec3_t = [0.; 3];
    let mut backoff: libc::c_float = 0.;
    let mut slave: *mut edict_t = 0 as *mut edict_t;
    let mut wasinwater: qboolean = false_0;
    let mut isinwater: qboolean = false_0;
    let mut old_origin: vec3_t = [0.; 3];
    SV_RunThink(ent);
    if (*ent).flags & 0x400 as libc::c_int != 0 {
        return;
    }
    if (*ent).velocity[2 as libc::c_int as usize] > 0 as libc::c_int as libc::c_float {
        let ref mut fresh8 = (*ent).groundentity;
        *fresh8 = 0 as *mut edict_t;
    }
    if !((*ent).groundentity).is_null() {
        if (*(*ent).groundentity).inuse as u64 == 0 {
            let ref mut fresh9 = (*ent).groundentity;
            *fresh9 = 0 as *mut edict_t;
        }
    }
    if !((*ent).groundentity).is_null() {
        return;
    }
    old_origin[0 as libc::c_int as usize] = (*ent).s.origin[0 as libc::c_int as usize];
    old_origin[1 as libc::c_int as usize] = (*ent).s.origin[1 as libc::c_int as usize];
    old_origin[2 as libc::c_int as usize] = (*ent).s.origin[2 as libc::c_int as usize];
    SV_CheckVelocity(ent);
    if (*ent).movetype != MOVETYPE_FLY as libc::c_int
        && (*ent).movetype != MOVETYPE_FLYMISSILE as libc::c_int
    {
        SV_AddGravity(ent);
    }
    VectorMA(
        ((*ent).s.angles).as_mut_ptr(),
        0.1f64 as libc::c_float,
        ((*ent).avelocity).as_mut_ptr(),
        ((*ent).s.angles).as_mut_ptr(),
    );
    VectorScale(((*ent).velocity).as_mut_ptr(), 0.1f64 as vec_t, move_0.as_mut_ptr());
    trace = SV_PushEntity(ent, move_0.as_mut_ptr());
    if (*ent).inuse as u64 == 0 {
        return;
    }
    if trace.fraction < 1 as libc::c_int as libc::c_float {
        if (*ent).movetype == MOVETYPE_BOUNCE as libc::c_int {
            backoff = 1.5f64 as libc::c_float;
        } else {
            backoff = 1 as libc::c_int as libc::c_float;
        }
        ClipVelocity(
            ((*ent).velocity).as_mut_ptr(),
            (trace.plane.normal).as_mut_ptr(),
            ((*ent).velocity).as_mut_ptr(),
            backoff,
        );
        if trace.plane.normal[2 as libc::c_int as usize] as libc::c_double > 0.7f64 {
            if (*ent).velocity[2 as libc::c_int as usize]
                < 60 as libc::c_int as libc::c_float
                || (*ent).movetype != MOVETYPE_BOUNCE as libc::c_int
            {
                let ref mut fresh10 = (*ent).groundentity;
                *fresh10 = trace.ent;
                (*ent).groundentity_linkcount = (*trace.ent).linkcount;
                (*ent)
                    .velocity[0 as libc::c_int
                    as usize] = vec3_origin[0 as libc::c_int as usize];
                (*ent)
                    .velocity[1 as libc::c_int
                    as usize] = vec3_origin[1 as libc::c_int as usize];
                (*ent)
                    .velocity[2 as libc::c_int
                    as usize] = vec3_origin[2 as libc::c_int as usize];
                (*ent)
                    .avelocity[0 as libc::c_int
                    as usize] = vec3_origin[0 as libc::c_int as usize];
                (*ent)
                    .avelocity[1 as libc::c_int
                    as usize] = vec3_origin[1 as libc::c_int as usize];
                (*ent)
                    .avelocity[2 as libc::c_int
                    as usize] = vec3_origin[2 as libc::c_int as usize];
            }
        }
    }
    wasinwater = ((*ent).watertype
        & (32 as libc::c_int | 8 as libc::c_int | 16 as libc::c_int)) as qboolean;
    (*ent)
        .watertype = (gi.pointcontents)
        .expect("non-null function pointer")(((*ent).s.origin).as_mut_ptr());
    isinwater = ((*ent).watertype
        & (32 as libc::c_int | 8 as libc::c_int | 16 as libc::c_int)) as qboolean;
    if isinwater as u64 != 0 {
        (*ent).waterlevel = 1 as libc::c_int;
    } else {
        (*ent).waterlevel = 0 as libc::c_int;
    }
    if wasinwater as u64 == 0 && isinwater as libc::c_uint != 0 {
        (gi.positioned_sound)
            .expect(
                "non-null function pointer",
            )(
            old_origin.as_mut_ptr(),
            g_edicts,
            0 as libc::c_int,
            (gi.soundindex)
                .expect(
                    "non-null function pointer",
                )(
                b"misc/h2ohit1.wav\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            ),
            1 as libc::c_int as libc::c_float,
            1 as libc::c_int as libc::c_float,
            0 as libc::c_int as libc::c_float,
        );
    } else if wasinwater as libc::c_uint != 0 && isinwater as u64 == 0 {
        (gi.positioned_sound)
            .expect(
                "non-null function pointer",
            )(
            ((*ent).s.origin).as_mut_ptr(),
            g_edicts,
            0 as libc::c_int,
            (gi.soundindex)
                .expect(
                    "non-null function pointer",
                )(
                b"misc/h2ohit1.wav\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            ),
            1 as libc::c_int as libc::c_float,
            1 as libc::c_int as libc::c_float,
            0 as libc::c_int as libc::c_float,
        );
    }
    slave = (*ent).teamchain;
    while !slave.is_null() {
        (*slave)
            .s
            .origin[0 as libc::c_int
            as usize] = (*ent).s.origin[0 as libc::c_int as usize];
        (*slave)
            .s
            .origin[1 as libc::c_int
            as usize] = (*ent).s.origin[1 as libc::c_int as usize];
        (*slave)
            .s
            .origin[2 as libc::c_int
            as usize] = (*ent).s.origin[2 as libc::c_int as usize];
        (gi.linkentity).expect("non-null function pointer")(slave);
        slave = (*slave).teamchain;
    }
}
#[no_mangle]
pub unsafe extern "C" fn SV_AddRotationalFriction(mut ent: *mut edict_t) {
    let mut n: libc::c_int = 0;
    let mut adjustment: libc::c_float = 0.;
    VectorMA(
        ((*ent).s.angles).as_mut_ptr(),
        0.1f64 as libc::c_float,
        ((*ent).avelocity).as_mut_ptr(),
        ((*ent).s.angles).as_mut_ptr(),
    );
    adjustment = (0.1f64 * 100 as libc::c_int as libc::c_double
        * 6 as libc::c_int as libc::c_double) as libc::c_float;
    n = 0 as libc::c_int;
    while n < 3 as libc::c_int {
        if (*ent).avelocity[n as usize] > 0 as libc::c_int as libc::c_float {
            let ref mut fresh11 = (*ent).avelocity[n as usize];
            *fresh11 -= adjustment;
            if (*ent).avelocity[n as usize] < 0 as libc::c_int as libc::c_float {
                (*ent).avelocity[n as usize] = 0 as libc::c_int as vec_t;
            }
        } else {
            let ref mut fresh12 = (*ent).avelocity[n as usize];
            *fresh12 += adjustment;
            if (*ent).avelocity[n as usize] > 0 as libc::c_int as libc::c_float {
                (*ent).avelocity[n as usize] = 0 as libc::c_int as vec_t;
            }
        }
        n += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn SV_Physics_Step(mut ent: *mut edict_t) {
    let mut wasonground: qboolean = false_0;
    let mut hitsound: qboolean = false_0;
    let mut vel: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut speed: libc::c_float = 0.;
    let mut newspeed: libc::c_float = 0.;
    let mut control: libc::c_float = 0.;
    let mut friction: libc::c_float = 0.;
    let mut groundentity: *mut edict_t = 0 as *mut edict_t;
    let mut mask: libc::c_int = 0;
    if ((*ent).groundentity).is_null() {
        M_CheckGround(ent);
    }
    groundentity = (*ent).groundentity;
    SV_CheckVelocity(ent);
    if !groundentity.is_null() {
        wasonground = true_0;
    } else {
        wasonground = false_0;
    }
    if (*ent).avelocity[0 as libc::c_int as usize] != 0.
        || (*ent).avelocity[1 as libc::c_int as usize] != 0.
        || (*ent).avelocity[2 as libc::c_int as usize] != 0.
    {
        SV_AddRotationalFriction(ent);
    }
    if wasonground as u64 == 0 {
        if (*ent).flags & 0x1 as libc::c_int == 0 {
            if !((*ent).flags & 0x2 as libc::c_int != 0
                && (*ent).waterlevel > 2 as libc::c_int)
            {
                if ((*ent).velocity[2 as libc::c_int as usize] as libc::c_double)
                    < (*sv_gravity).value as libc::c_double * -0.1f64
                {
                    hitsound = true_0;
                }
                if (*ent).waterlevel == 0 as libc::c_int {
                    SV_AddGravity(ent);
                }
            }
        }
    }
    if (*ent).flags & 0x1 as libc::c_int != 0
        && (*ent).velocity[2 as libc::c_int as usize]
            != 0 as libc::c_int as libc::c_float
    {
        speed = fabs((*ent).velocity[2 as libc::c_int as usize] as libc::c_double)
            as libc::c_float;
        control = if speed < 100 as libc::c_int as libc::c_float {
            100 as libc::c_int as libc::c_float
        } else {
            speed
        };
        friction = (6 as libc::c_int / 3 as libc::c_int) as libc::c_float;
        newspeed = (speed as libc::c_double
            - 0.1f64 * control as libc::c_double * friction as libc::c_double)
            as libc::c_float;
        if newspeed < 0 as libc::c_int as libc::c_float {
            newspeed = 0 as libc::c_int as libc::c_float;
        }
        newspeed /= speed;
        let ref mut fresh13 = (*ent).velocity[2 as libc::c_int as usize];
        *fresh13 *= newspeed;
    }
    if (*ent).flags & 0x2 as libc::c_int != 0
        && (*ent).velocity[2 as libc::c_int as usize]
            != 0 as libc::c_int as libc::c_float
    {
        speed = fabs((*ent).velocity[2 as libc::c_int as usize] as libc::c_double)
            as libc::c_float;
        control = if speed < 100 as libc::c_int as libc::c_float {
            100 as libc::c_int as libc::c_float
        } else {
            speed
        };
        newspeed = (speed as libc::c_double
            - 0.1f64 * control as libc::c_double * 1 as libc::c_int as libc::c_double
                * (*ent).waterlevel as libc::c_double) as libc::c_float;
        if newspeed < 0 as libc::c_int as libc::c_float {
            newspeed = 0 as libc::c_int as libc::c_float;
        }
        newspeed /= speed;
        let ref mut fresh14 = (*ent).velocity[2 as libc::c_int as usize];
        *fresh14 *= newspeed;
    }
    if (*ent).velocity[2 as libc::c_int as usize] != 0.
        || (*ent).velocity[1 as libc::c_int as usize] != 0.
        || (*ent).velocity[0 as libc::c_int as usize] != 0.
    {
        if wasonground as libc::c_uint != 0
            || (*ent).flags & (0x2 as libc::c_int | 0x1 as libc::c_int) != 0
        {
            if !((*ent).health as libc::c_double <= 0.0f64
                && M_CheckBottom(ent) as u64 == 0)
            {
                vel = ((*ent).velocity).as_mut_ptr();
                speed = sqrt(
                    (*vel.offset(0 as libc::c_int as isize)
                        * *vel.offset(0 as libc::c_int as isize)
                        + *vel.offset(1 as libc::c_int as isize)
                            * *vel.offset(1 as libc::c_int as isize)) as libc::c_double,
                ) as libc::c_float;
                if speed != 0. {
                    friction = 6 as libc::c_int as libc::c_float;
                    control = if speed < 100 as libc::c_int as libc::c_float {
                        100 as libc::c_int as libc::c_float
                    } else {
                        speed
                    };
                    newspeed = (speed as libc::c_double
                        - 0.1f64 * control as libc::c_double
                            * friction as libc::c_double) as libc::c_float;
                    if newspeed < 0 as libc::c_int as libc::c_float {
                        newspeed = 0 as libc::c_int as libc::c_float;
                    }
                    newspeed /= speed;
                    *vel.offset(0 as libc::c_int as isize) *= newspeed;
                    *vel.offset(1 as libc::c_int as isize) *= newspeed;
                }
            }
        }
        if (*ent).svflags & 0x4 as libc::c_int != 0 {
            mask = 1 as libc::c_int | 0x20000 as libc::c_int | 2 as libc::c_int
                | 0x2000000 as libc::c_int;
        } else {
            mask = 1 as libc::c_int | 2 as libc::c_int;
        }
        SV_FlyMove(ent, 0.1f64 as libc::c_float, mask);
        (gi.linkentity).expect("non-null function pointer")(ent);
        G_TouchTriggers(ent);
        if (*ent).inuse as u64 == 0 {
            return;
        }
        if !((*ent).groundentity).is_null() {
            if wasonground as u64 == 0 {
                if hitsound as u64 != 0 {
                    (gi.sound)
                        .expect(
                            "non-null function pointer",
                        )(
                        ent,
                        0 as libc::c_int,
                        (gi.soundindex)
                            .expect(
                                "non-null function pointer",
                            )(
                            b"world/land.wav\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char,
                        ),
                        1 as libc::c_int as libc::c_float,
                        1 as libc::c_int as libc::c_float,
                        0 as libc::c_int as libc::c_float,
                    );
                }
            }
        }
    }
    SV_RunThink(ent);
}
#[no_mangle]
pub unsafe extern "C" fn G_RunEntity(mut ent: *mut edict_t) {
    if ((*ent).prethink).is_some() {
        ((*ent).prethink).expect("non-null function pointer")(ent);
    }
    match (*ent).movetype {
        2 | 3 => {
            SV_Physics_Pusher(ent);
        }
        0 => {
            SV_Physics_None(ent);
        }
        1 => {
            SV_Physics_Noclip(ent);
        }
        5 => {
            SV_Physics_Step(ent);
        }
        7 | 9 | 6 | 8 => {
            SV_Physics_Toss(ent);
        }
        _ => {
            (gi.error)
                .expect(
                    "non-null function pointer",
                )(
                b"SV_Physics: bad movetype %i\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                (*ent).movetype,
            );
        }
    };
}
