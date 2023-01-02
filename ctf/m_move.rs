#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    fn cos(_: libc::c_double) -> libc::c_double;
    fn sin(_: libc::c_double) -> libc::c_double;
    fn rand() -> libc::c_int;
    fn abs(_: libc::c_int) -> libc::c_int;
    static mut vec3_origin: vec3_t;
    fn anglemod(a: libc::c_float) -> libc::c_float;
    static mut gi: game_import_t;
    fn G_TouchTriggers(ent: *mut edict_t);
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
    pub inmenu: qboolean,
    pub menu: *mut pmenuhnd_t,
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
    pub ctf_grapple: *mut libc::c_void,
    pub ctf_grapplestate: libc::c_int,
    pub ctf_grapplereleasetime: libc::c_float,
    pub ctf_regentime: libc::c_float,
    pub ctf_techsndtime: libc::c_float,
    pub ctf_lasttechmsg: libc::c_float,
    pub chase_target: *mut edict_t,
    pub update_chase: qboolean,
    pub menutime: libc::c_float,
    pub menudirty: qboolean,
}
pub type weaponstate_t = libc::c_uint;
pub const WEAPON_FIRING: weaponstate_t = 3;
pub const WEAPON_DROPPING: weaponstate_t = 2;
pub const WEAPON_ACTIVATING: weaponstate_t = 1;
pub const WEAPON_READY: weaponstate_t = 0;
pub type pmenuhnd_t = pmenuhnd_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pmenuhnd_s {
    pub entries: *mut pmenu_s,
    pub cur: libc::c_int,
    pub num: libc::c_int,
    pub arg: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pmenu_s {
    pub text: *mut libc::c_char,
    pub align: libc::c_int,
    pub SelectFunc: SelectFunc_t,
}
pub type SelectFunc_t = Option::<
    unsafe extern "C" fn(*mut edict_t, *mut pmenuhnd_t) -> (),
>;
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
    pub ctf_team: libc::c_int,
    pub ctf_state: libc::c_int,
    pub ctf_lasthurtcarrier: libc::c_float,
    pub ctf_lastreturnedflag: libc::c_float,
    pub ctf_flagsince: libc::c_float,
    pub ctf_lastfraggedcarrier: libc::c_float,
    pub id_state: qboolean,
    pub voted: qboolean,
    pub ready: qboolean,
    pub admin: qboolean,
    pub ghost: *mut ghost_s,
    pub cmd_angles: vec3_t,
    pub game_helpchanged: libc::c_int,
    pub helpchanged: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ghost_s {
    pub netname: [libc::c_char; 16],
    pub number: libc::c_int,
    pub deaths: libc::c_int,
    pub kills: libc::c_int,
    pub caps: libc::c_int,
    pub basedef: libc::c_int,
    pub carrierdef: libc::c_int,
    pub code: libc::c_int,
    pub team: libc::c_int,
    pub score: libc::c_int,
    pub ent: *mut edict_t,
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
    pub powerArmorActive: qboolean,
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
#[no_mangle]
pub static mut c_yes: libc::c_int = 0;
#[no_mangle]
pub static mut c_no: libc::c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn M_CheckBottom(mut ent: *mut edict_t) -> qboolean {
    let mut current_block: u64;
    let mut mins: vec3_t = [0.; 3];
    let mut maxs: vec3_t = [0.; 3];
    let mut start: vec3_t = [0.; 3];
    let mut stop: vec3_t = [0.; 3];
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
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut mid: libc::c_float = 0.;
    let mut bottom: libc::c_float = 0.;
    mins[0 as libc::c_int
        as usize] = (*ent).s.origin[0 as libc::c_int as usize]
        + (*ent).mins[0 as libc::c_int as usize];
    mins[1 as libc::c_int
        as usize] = (*ent).s.origin[1 as libc::c_int as usize]
        + (*ent).mins[1 as libc::c_int as usize];
    mins[2 as libc::c_int
        as usize] = (*ent).s.origin[2 as libc::c_int as usize]
        + (*ent).mins[2 as libc::c_int as usize];
    maxs[0 as libc::c_int
        as usize] = (*ent).s.origin[0 as libc::c_int as usize]
        + (*ent).maxs[0 as libc::c_int as usize];
    maxs[1 as libc::c_int
        as usize] = (*ent).s.origin[1 as libc::c_int as usize]
        + (*ent).maxs[1 as libc::c_int as usize];
    maxs[2 as libc::c_int
        as usize] = (*ent).s.origin[2 as libc::c_int as usize]
        + (*ent).maxs[2 as libc::c_int as usize];
    start[2 as libc::c_int
        as usize] = mins[2 as libc::c_int as usize] - 1 as libc::c_int as libc::c_float;
    x = 0 as libc::c_int;
    's_27: loop {
        if !(x <= 1 as libc::c_int) {
            current_block = 5399440093318478209;
            break;
        }
        y = 0 as libc::c_int;
        while y <= 1 as libc::c_int {
            start[0 as libc::c_int
                as usize] = if x != 0 {
                maxs[0 as libc::c_int as usize]
            } else {
                mins[0 as libc::c_int as usize]
            };
            start[1 as libc::c_int
                as usize] = if y != 0 {
                maxs[1 as libc::c_int as usize]
            } else {
                mins[1 as libc::c_int as usize]
            };
            if (gi.pointcontents).expect("non-null function pointer")(start.as_mut_ptr())
                != 1 as libc::c_int
            {
                current_block = 17361171665073872999;
                break 's_27;
            }
            y += 1;
        }
        x += 1;
    }
    match current_block {
        5399440093318478209 => {
            c_yes += 1;
            return true_0;
        }
        _ => {
            c_no += 1;
            start[2 as libc::c_int as usize] = mins[2 as libc::c_int as usize];
            stop[0 as libc::c_int
                as usize] = ((mins[0 as libc::c_int as usize]
                + maxs[0 as libc::c_int as usize]) as libc::c_double * 0.5f64) as vec_t;
            start[0 as libc::c_int as usize] = stop[0 as libc::c_int as usize];
            stop[1 as libc::c_int
                as usize] = ((mins[1 as libc::c_int as usize]
                + maxs[1 as libc::c_int as usize]) as libc::c_double * 0.5f64) as vec_t;
            start[1 as libc::c_int as usize] = stop[1 as libc::c_int as usize];
            stop[2 as libc::c_int
                as usize] = start[2 as libc::c_int as usize]
                - (2 as libc::c_int * 18 as libc::c_int) as libc::c_float;
            trace = (gi.trace)
                .expect(
                    "non-null function pointer",
                )(
                start.as_mut_ptr(),
                vec3_origin.as_mut_ptr(),
                vec3_origin.as_mut_ptr(),
                stop.as_mut_ptr(),
                ent,
                1 as libc::c_int | 0x20000 as libc::c_int | 2 as libc::c_int
                    | 0x2000000 as libc::c_int,
            );
            if trace.fraction as libc::c_double == 1.0f64 {
                return false_0;
            }
            bottom = trace.endpos[2 as libc::c_int as usize];
            mid = bottom;
            x = 0 as libc::c_int;
            while x <= 1 as libc::c_int {
                y = 0 as libc::c_int;
                while y <= 1 as libc::c_int {
                    stop[0 as libc::c_int
                        as usize] = if x != 0 {
                        maxs[0 as libc::c_int as usize]
                    } else {
                        mins[0 as libc::c_int as usize]
                    };
                    start[0 as libc::c_int as usize] = stop[0 as libc::c_int as usize];
                    stop[1 as libc::c_int
                        as usize] = if y != 0 {
                        maxs[1 as libc::c_int as usize]
                    } else {
                        mins[1 as libc::c_int as usize]
                    };
                    start[1 as libc::c_int as usize] = stop[1 as libc::c_int as usize];
                    trace = (gi.trace)
                        .expect(
                            "non-null function pointer",
                        )(
                        start.as_mut_ptr(),
                        vec3_origin.as_mut_ptr(),
                        vec3_origin.as_mut_ptr(),
                        stop.as_mut_ptr(),
                        ent,
                        1 as libc::c_int | 0x20000 as libc::c_int | 2 as libc::c_int
                            | 0x2000000 as libc::c_int,
                    );
                    if trace.fraction as libc::c_double != 1.0f64
                        && trace.endpos[2 as libc::c_int as usize] > bottom
                    {
                        bottom = trace.endpos[2 as libc::c_int as usize];
                    }
                    if trace.fraction as libc::c_double == 1.0f64
                        || mid - trace.endpos[2 as libc::c_int as usize]
                            > 18 as libc::c_int as libc::c_float
                    {
                        return false_0;
                    }
                    y += 1;
                }
                x += 1;
            }
            c_yes += 1;
            return true_0;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn SV_movestep(
    mut ent: *mut edict_t,
    mut move_0: *mut vec_t,
    mut relink: qboolean,
) -> qboolean {
    let mut dz: libc::c_float = 0.;
    let mut oldorg: vec3_t = [0.; 3];
    let mut neworg: vec3_t = [0.; 3];
    let mut end: vec3_t = [0.; 3];
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
    let mut i: libc::c_int = 0;
    let mut stepsize: libc::c_float = 0.;
    let mut test: vec3_t = [0.; 3];
    let mut contents: libc::c_int = 0;
    oldorg[0 as libc::c_int as usize] = (*ent).s.origin[0 as libc::c_int as usize];
    oldorg[1 as libc::c_int as usize] = (*ent).s.origin[1 as libc::c_int as usize];
    oldorg[2 as libc::c_int as usize] = (*ent).s.origin[2 as libc::c_int as usize];
    neworg[0 as libc::c_int
        as usize] = (*ent).s.origin[0 as libc::c_int as usize]
        + *move_0.offset(0 as libc::c_int as isize);
    neworg[1 as libc::c_int
        as usize] = (*ent).s.origin[1 as libc::c_int as usize]
        + *move_0.offset(1 as libc::c_int as isize);
    neworg[2 as libc::c_int
        as usize] = (*ent).s.origin[2 as libc::c_int as usize]
        + *move_0.offset(2 as libc::c_int as isize);
    if (*ent).flags & (0x2 as libc::c_int | 0x1 as libc::c_int) != 0 {
        i = 0 as libc::c_int;
        while i < 2 as libc::c_int {
            neworg[0 as libc::c_int
                as usize] = (*ent).s.origin[0 as libc::c_int as usize]
                + *move_0.offset(0 as libc::c_int as isize);
            neworg[1 as libc::c_int
                as usize] = (*ent).s.origin[1 as libc::c_int as usize]
                + *move_0.offset(1 as libc::c_int as isize);
            neworg[2 as libc::c_int
                as usize] = (*ent).s.origin[2 as libc::c_int as usize]
                + *move_0.offset(2 as libc::c_int as isize);
            if i == 0 as libc::c_int && !((*ent).enemy).is_null() {
                if ((*ent).goalentity).is_null() {
                    let ref mut fresh0 = (*ent).goalentity;
                    *fresh0 = (*ent).enemy;
                }
                dz = (*ent).s.origin[2 as libc::c_int as usize]
                    - (*(*ent).goalentity).s.origin[2 as libc::c_int as usize];
                if !((*(*ent).goalentity).client).is_null() {
                    if dz > 40 as libc::c_int as libc::c_float {
                        neworg[2 as libc::c_int as usize]
                            -= 8 as libc::c_int as libc::c_float;
                    }
                    if !((*ent).flags & 0x2 as libc::c_int != 0
                        && (*ent).waterlevel < 2 as libc::c_int)
                    {
                        if dz < 30 as libc::c_int as libc::c_float {
                            neworg[2 as libc::c_int as usize]
                                += 8 as libc::c_int as libc::c_float;
                        }
                    }
                } else if dz > 8 as libc::c_int as libc::c_float {
                    neworg[2 as libc::c_int as usize]
                        -= 8 as libc::c_int as libc::c_float;
                } else if dz > 0 as libc::c_int as libc::c_float {
                    neworg[2 as libc::c_int as usize] -= dz;
                } else if dz < -(8 as libc::c_int) as libc::c_float {
                    neworg[2 as libc::c_int as usize]
                        += 8 as libc::c_int as libc::c_float;
                } else {
                    neworg[2 as libc::c_int as usize] += dz;
                }
            }
            trace = (gi.trace)
                .expect(
                    "non-null function pointer",
                )(
                ((*ent).s.origin).as_mut_ptr(),
                ((*ent).mins).as_mut_ptr(),
                ((*ent).maxs).as_mut_ptr(),
                neworg.as_mut_ptr(),
                ent,
                1 as libc::c_int | 0x20000 as libc::c_int | 2 as libc::c_int
                    | 0x2000000 as libc::c_int,
            );
            if (*ent).flags & 0x1 as libc::c_int != 0 {
                if (*ent).waterlevel == 0 {
                    test[0 as libc::c_int
                        as usize] = trace.endpos[0 as libc::c_int as usize];
                    test[1 as libc::c_int
                        as usize] = trace.endpos[1 as libc::c_int as usize];
                    test[2 as libc::c_int
                        as usize] = trace.endpos[2 as libc::c_int as usize]
                        + (*ent).mins[2 as libc::c_int as usize]
                        + 1 as libc::c_int as libc::c_float;
                    contents = (gi.pointcontents)
                        .expect("non-null function pointer")(test.as_mut_ptr());
                    if contents
                        & (32 as libc::c_int | 8 as libc::c_int | 16 as libc::c_int) != 0
                    {
                        return false_0;
                    }
                }
            }
            if (*ent).flags & 0x2 as libc::c_int != 0 {
                if (*ent).waterlevel < 2 as libc::c_int {
                    test[0 as libc::c_int
                        as usize] = trace.endpos[0 as libc::c_int as usize];
                    test[1 as libc::c_int
                        as usize] = trace.endpos[1 as libc::c_int as usize];
                    test[2 as libc::c_int
                        as usize] = trace.endpos[2 as libc::c_int as usize]
                        + (*ent).mins[2 as libc::c_int as usize]
                        + 1 as libc::c_int as libc::c_float;
                    contents = (gi.pointcontents)
                        .expect("non-null function pointer")(test.as_mut_ptr());
                    if contents
                        & (32 as libc::c_int | 8 as libc::c_int | 16 as libc::c_int) == 0
                    {
                        return false_0;
                    }
                }
            }
            if trace.fraction == 1 as libc::c_int as libc::c_float {
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
                if relink as u64 != 0 {
                    (gi.linkentity).expect("non-null function pointer")(ent);
                    G_TouchTriggers(ent);
                }
                return true_0;
            }
            if ((*ent).enemy).is_null() {
                break;
            }
            i += 1;
        }
        return false_0;
    }
    if (*ent).monsterinfo.aiflags & 0x400 as libc::c_int == 0 {
        stepsize = 18 as libc::c_int as libc::c_float;
    } else {
        stepsize = 1 as libc::c_int as libc::c_float;
    }
    neworg[2 as libc::c_int as usize] += stepsize;
    end[0 as libc::c_int as usize] = neworg[0 as libc::c_int as usize];
    end[1 as libc::c_int as usize] = neworg[1 as libc::c_int as usize];
    end[2 as libc::c_int as usize] = neworg[2 as libc::c_int as usize];
    end[2 as libc::c_int as usize] -= stepsize * 2 as libc::c_int as libc::c_float;
    trace = (gi.trace)
        .expect(
            "non-null function pointer",
        )(
        neworg.as_mut_ptr(),
        ((*ent).mins).as_mut_ptr(),
        ((*ent).maxs).as_mut_ptr(),
        end.as_mut_ptr(),
        ent,
        1 as libc::c_int | 0x20000 as libc::c_int | 2 as libc::c_int
            | 0x2000000 as libc::c_int,
    );
    if trace.allsolid as u64 != 0 {
        return false_0;
    }
    if trace.startsolid as u64 != 0 {
        neworg[2 as libc::c_int as usize] -= stepsize;
        trace = (gi.trace)
            .expect(
                "non-null function pointer",
            )(
            neworg.as_mut_ptr(),
            ((*ent).mins).as_mut_ptr(),
            ((*ent).maxs).as_mut_ptr(),
            end.as_mut_ptr(),
            ent,
            1 as libc::c_int | 0x20000 as libc::c_int | 2 as libc::c_int
                | 0x2000000 as libc::c_int,
        );
        if trace.allsolid as libc::c_uint != 0 || trace.startsolid as libc::c_uint != 0 {
            return false_0;
        }
    }
    if (*ent).waterlevel == 0 as libc::c_int {
        test[0 as libc::c_int as usize] = trace.endpos[0 as libc::c_int as usize];
        test[1 as libc::c_int as usize] = trace.endpos[1 as libc::c_int as usize];
        test[2 as libc::c_int
            as usize] = trace.endpos[2 as libc::c_int as usize]
            + (*ent).mins[2 as libc::c_int as usize] + 1 as libc::c_int as libc::c_float;
        contents = (gi.pointcontents)
            .expect("non-null function pointer")(test.as_mut_ptr());
        if contents & (32 as libc::c_int | 8 as libc::c_int | 16 as libc::c_int) != 0 {
            return false_0;
        }
    }
    if trace.fraction == 1 as libc::c_int as libc::c_float {
        if (*ent).flags & 0x100 as libc::c_int != 0 {
            (*ent)
                .s
                .origin[0 as libc::c_int
                as usize] = (*ent).s.origin[0 as libc::c_int as usize]
                + *move_0.offset(0 as libc::c_int as isize);
            (*ent)
                .s
                .origin[1 as libc::c_int
                as usize] = (*ent).s.origin[1 as libc::c_int as usize]
                + *move_0.offset(1 as libc::c_int as isize);
            (*ent)
                .s
                .origin[2 as libc::c_int
                as usize] = (*ent).s.origin[2 as libc::c_int as usize]
                + *move_0.offset(2 as libc::c_int as isize);
            if relink as u64 != 0 {
                (gi.linkentity).expect("non-null function pointer")(ent);
                G_TouchTriggers(ent);
            }
            let ref mut fresh1 = (*ent).groundentity;
            *fresh1 = 0 as *mut edict_t;
            return true_0;
        }
        return false_0;
    }
    (*ent).s.origin[0 as libc::c_int as usize] = trace.endpos[0 as libc::c_int as usize];
    (*ent).s.origin[1 as libc::c_int as usize] = trace.endpos[1 as libc::c_int as usize];
    (*ent).s.origin[2 as libc::c_int as usize] = trace.endpos[2 as libc::c_int as usize];
    if M_CheckBottom(ent) as u64 == 0 {
        if (*ent).flags & 0x100 as libc::c_int != 0 {
            if relink as u64 != 0 {
                (gi.linkentity).expect("non-null function pointer")(ent);
                G_TouchTriggers(ent);
            }
            return true_0;
        }
        (*ent).s.origin[0 as libc::c_int as usize] = oldorg[0 as libc::c_int as usize];
        (*ent).s.origin[1 as libc::c_int as usize] = oldorg[1 as libc::c_int as usize];
        (*ent).s.origin[2 as libc::c_int as usize] = oldorg[2 as libc::c_int as usize];
        return false_0;
    }
    if (*ent).flags & 0x100 as libc::c_int != 0 {
        (*ent).flags &= !(0x100 as libc::c_int);
    }
    let ref mut fresh2 = (*ent).groundentity;
    *fresh2 = trace.ent;
    (*ent).groundentity_linkcount = (*trace.ent).linkcount;
    if relink as u64 != 0 {
        (gi.linkentity).expect("non-null function pointer")(ent);
        G_TouchTriggers(ent);
    }
    return true_0;
}
#[no_mangle]
pub unsafe extern "C" fn M_ChangeYaw(mut ent: *mut edict_t) {
    let mut ideal: libc::c_float = 0.;
    let mut current: libc::c_float = 0.;
    let mut move_0: libc::c_float = 0.;
    let mut speed: libc::c_float = 0.;
    current = anglemod((*ent).s.angles[1 as libc::c_int as usize]);
    ideal = (*ent).ideal_yaw;
    if current == ideal {
        return;
    }
    move_0 = ideal - current;
    speed = (*ent).yaw_speed;
    if ideal > current {
        if move_0 >= 180 as libc::c_int as libc::c_float {
            move_0 = move_0 - 360 as libc::c_int as libc::c_float;
        }
    } else if move_0 <= -(180 as libc::c_int) as libc::c_float {
        move_0 = move_0 + 360 as libc::c_int as libc::c_float;
    }
    if move_0 > 0 as libc::c_int as libc::c_float {
        if move_0 > speed {
            move_0 = speed;
        }
    } else if move_0 < -speed {
        move_0 = -speed;
    }
    (*ent).s.angles[1 as libc::c_int as usize] = anglemod(current + move_0);
}
#[no_mangle]
pub unsafe extern "C" fn SV_StepDirection(
    mut ent: *mut edict_t,
    mut yaw: libc::c_float,
    mut dist: libc::c_float,
) -> qboolean {
    let mut move_0: vec3_t = [0.; 3];
    let mut oldorigin: vec3_t = [0.; 3];
    let mut delta: libc::c_float = 0.;
    (*ent).ideal_yaw = yaw;
    M_ChangeYaw(ent);
    yaw = (yaw as libc::c_double * 3.14159265358979323846f64
        * 2 as libc::c_int as libc::c_double / 360 as libc::c_int as libc::c_double)
        as libc::c_float;
    move_0[0 as libc::c_int
        as usize] = (cos(yaw as libc::c_double) * dist as libc::c_double) as vec_t;
    move_0[1 as libc::c_int
        as usize] = (sin(yaw as libc::c_double) * dist as libc::c_double) as vec_t;
    move_0[2 as libc::c_int as usize] = 0 as libc::c_int as vec_t;
    oldorigin[0 as libc::c_int as usize] = (*ent).s.origin[0 as libc::c_int as usize];
    oldorigin[1 as libc::c_int as usize] = (*ent).s.origin[1 as libc::c_int as usize];
    oldorigin[2 as libc::c_int as usize] = (*ent).s.origin[2 as libc::c_int as usize];
    if SV_movestep(ent, move_0.as_mut_ptr(), false_0) as u64 != 0 {
        delta = (*ent).s.angles[1 as libc::c_int as usize] - (*ent).ideal_yaw;
        if delta > 45 as libc::c_int as libc::c_float
            && delta < 315 as libc::c_int as libc::c_float
        {
            (*ent)
                .s
                .origin[0 as libc::c_int
                as usize] = oldorigin[0 as libc::c_int as usize];
            (*ent)
                .s
                .origin[1 as libc::c_int
                as usize] = oldorigin[1 as libc::c_int as usize];
            (*ent)
                .s
                .origin[2 as libc::c_int
                as usize] = oldorigin[2 as libc::c_int as usize];
        }
        (gi.linkentity).expect("non-null function pointer")(ent);
        G_TouchTriggers(ent);
        return true_0;
    }
    (gi.linkentity).expect("non-null function pointer")(ent);
    G_TouchTriggers(ent);
    return false_0;
}
#[no_mangle]
pub unsafe extern "C" fn SV_FixCheckBottom(mut ent: *mut edict_t) {
    (*ent).flags |= 0x100 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn SV_NewChaseDir(
    mut actor: *mut edict_t,
    mut enemy: *mut edict_t,
    mut dist: libc::c_float,
) {
    let mut deltax: libc::c_float = 0.;
    let mut deltay: libc::c_float = 0.;
    let mut d: [libc::c_float; 3] = [0.; 3];
    let mut tdir: libc::c_float = 0.;
    let mut olddir: libc::c_float = 0.;
    let mut turnaround: libc::c_float = 0.;
    if enemy.is_null() {
        return;
    }
    olddir = anglemod(
        (((*actor).ideal_yaw / 45 as libc::c_int as libc::c_float) as libc::c_int
            * 45 as libc::c_int) as libc::c_float,
    );
    turnaround = anglemod(olddir - 180 as libc::c_int as libc::c_float);
    deltax = (*enemy).s.origin[0 as libc::c_int as usize]
        - (*actor).s.origin[0 as libc::c_int as usize];
    deltay = (*enemy).s.origin[1 as libc::c_int as usize]
        - (*actor).s.origin[1 as libc::c_int as usize];
    if deltax > 10 as libc::c_int as libc::c_float {
        d[1 as libc::c_int as usize] = 0 as libc::c_int as libc::c_float;
    } else if deltax < -(10 as libc::c_int) as libc::c_float {
        d[1 as libc::c_int as usize] = 180 as libc::c_int as libc::c_float;
    } else {
        d[1 as libc::c_int as usize] = -(1 as libc::c_int) as libc::c_float;
    }
    if deltay < -(10 as libc::c_int) as libc::c_float {
        d[2 as libc::c_int as usize] = 270 as libc::c_int as libc::c_float;
    } else if deltay > 10 as libc::c_int as libc::c_float {
        d[2 as libc::c_int as usize] = 90 as libc::c_int as libc::c_float;
    } else {
        d[2 as libc::c_int as usize] = -(1 as libc::c_int) as libc::c_float;
    }
    if d[1 as libc::c_int as usize] != -(1 as libc::c_int) as libc::c_float
        && d[2 as libc::c_int as usize] != -(1 as libc::c_int) as libc::c_float
    {
        if d[1 as libc::c_int as usize] == 0 as libc::c_int as libc::c_float {
            tdir = (if d[2 as libc::c_int as usize] == 90 as libc::c_int as libc::c_float
            {
                45 as libc::c_int
            } else {
                315 as libc::c_int
            }) as libc::c_float;
        } else {
            tdir = (if d[2 as libc::c_int as usize] == 90 as libc::c_int as libc::c_float
            {
                135 as libc::c_int
            } else {
                215 as libc::c_int
            }) as libc::c_float;
        }
        if tdir != turnaround && SV_StepDirection(actor, tdir, dist) as libc::c_uint != 0
        {
            return;
        }
    }
    if rand() & 3 as libc::c_int & 1 as libc::c_int != 0
        || abs(deltay as libc::c_int) > abs(deltax as libc::c_int)
    {
        tdir = d[1 as libc::c_int as usize];
        d[1 as libc::c_int as usize] = d[2 as libc::c_int as usize];
        d[2 as libc::c_int as usize] = tdir;
    }
    if d[1 as libc::c_int as usize] != -(1 as libc::c_int) as libc::c_float
        && d[1 as libc::c_int as usize] != turnaround
        && SV_StepDirection(actor, d[1 as libc::c_int as usize], dist) as libc::c_uint
            != 0
    {
        return;
    }
    if d[2 as libc::c_int as usize] != -(1 as libc::c_int) as libc::c_float
        && d[2 as libc::c_int as usize] != turnaround
        && SV_StepDirection(actor, d[2 as libc::c_int as usize], dist) as libc::c_uint
            != 0
    {
        return;
    }
    if olddir != -(1 as libc::c_int) as libc::c_float
        && SV_StepDirection(actor, olddir, dist) as libc::c_uint != 0
    {
        return;
    }
    if rand() & 1 as libc::c_int != 0 {
        tdir = 0 as libc::c_int as libc::c_float;
        while tdir <= 315 as libc::c_int as libc::c_float {
            if tdir != turnaround
                && SV_StepDirection(actor, tdir, dist) as libc::c_uint != 0
            {
                return;
            }
            tdir += 45 as libc::c_int as libc::c_float;
        }
    } else {
        tdir = 315 as libc::c_int as libc::c_float;
        while tdir >= 0 as libc::c_int as libc::c_float {
            if tdir != turnaround
                && SV_StepDirection(actor, tdir, dist) as libc::c_uint != 0
            {
                return;
            }
            tdir -= 45 as libc::c_int as libc::c_float;
        }
    }
    if turnaround != -(1 as libc::c_int) as libc::c_float
        && SV_StepDirection(actor, turnaround, dist) as libc::c_uint != 0
    {
        return;
    }
    (*actor).ideal_yaw = olddir;
    if M_CheckBottom(actor) as u64 == 0 {
        SV_FixCheckBottom(actor);
    }
}
#[no_mangle]
pub unsafe extern "C" fn SV_CloseEnough(
    mut ent: *mut edict_t,
    mut goal: *mut edict_t,
    mut dist: libc::c_float,
) -> qboolean {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        if (*goal).absmin[i as usize] > (*ent).absmax[i as usize] + dist {
            return false_0;
        }
        if (*goal).absmax[i as usize] < (*ent).absmin[i as usize] - dist {
            return false_0;
        }
        i += 1;
    }
    return true_0;
}
#[no_mangle]
pub unsafe extern "C" fn M_MoveToGoal(mut ent: *mut edict_t, mut dist: libc::c_float) {
    let mut goal: *mut edict_t = 0 as *mut edict_t;
    goal = (*ent).goalentity;
    if ((*ent).groundentity).is_null()
        && (*ent).flags & (0x1 as libc::c_int | 0x2 as libc::c_int) == 0
    {
        return;
    }
    if !((*ent).enemy).is_null()
        && SV_CloseEnough(ent, (*ent).enemy, dist) as libc::c_uint != 0
    {
        return;
    }
    if rand() & 3 as libc::c_int == 1 as libc::c_int
        || SV_StepDirection(ent, (*ent).ideal_yaw, dist) as u64 == 0
    {
        if (*ent).inuse as u64 != 0 {
            SV_NewChaseDir(ent, goal, dist);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn M_walkmove(
    mut ent: *mut edict_t,
    mut yaw: libc::c_float,
    mut dist: libc::c_float,
) -> qboolean {
    let mut move_0: vec3_t = [0.; 3];
    if ((*ent).groundentity).is_null()
        && (*ent).flags & (0x1 as libc::c_int | 0x2 as libc::c_int) == 0
    {
        return false_0;
    }
    yaw = (yaw as libc::c_double * 3.14159265358979323846f64
        * 2 as libc::c_int as libc::c_double / 360 as libc::c_int as libc::c_double)
        as libc::c_float;
    move_0[0 as libc::c_int
        as usize] = (cos(yaw as libc::c_double) * dist as libc::c_double) as vec_t;
    move_0[1 as libc::c_int
        as usize] = (sin(yaw as libc::c_double) * dist as libc::c_double) as vec_t;
    move_0[2 as libc::c_int as usize] = 0 as libc::c_int as vec_t;
    return SV_movestep(ent, move_0.as_mut_ptr(), true_0);
}
