#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn rand() -> libc::c_int;
    fn VectorScale(in_0: *mut vec_t, scale: vec_t, out: *mut vec_t);
    fn AngleVectors(
        angles: *mut vec_t,
        forward: *mut vec_t,
        right: *mut vec_t,
        up: *mut vec_t,
    );
    fn Q_stricmp(s1: *mut libc::c_char, s2: *mut libc::c_char) -> libc::c_int;
    static mut game: game_locals_t;
    static mut level: level_locals_t;
    static mut gi: game_import_t;
    static mut jacket_armor_index: libc::c_int;
    static mut combat_armor_index: libc::c_int;
    static mut body_armor_index: libc::c_int;
    static mut deathmatch: *mut cvar_t;
    static mut coop: *mut cvar_t;
    static mut dmflags: *mut cvar_t;
    static mut skill: *mut cvar_t;
    fn G_FreeEdict(e: *mut edict_t);
    fn ValidateSelectedItem(ent: *mut edict_t);
    fn G_UseTargets(ent: *mut edict_t, activator: *mut edict_t);
    fn G_ProjectSource(
        point: *mut vec_t,
        distance: *mut vec_t,
        forward: *mut vec_t,
        right: *mut vec_t,
        result: *mut vec_t,
    );
    fn G_Spawn() -> *mut edict_t;
    fn vtos(v: *mut vec_t) -> *mut libc::c_char;
    fn tv(x: libc::c_float, y: libc::c_float, z: libc::c_float) -> *mut libc::c_float;
    fn Pickup_Weapon(ent: *mut edict_t, other: *mut edict_t) -> qboolean;
    fn Use_Weapon(ent: *mut edict_t, inv: *mut gitem_t);
    fn Drop_Weapon(ent: *mut edict_t, inv: *mut gitem_t);
    fn Weapon_Blaster(ent: *mut edict_t);
    fn Weapon_Shotgun(ent: *mut edict_t);
    fn Weapon_SuperShotgun(ent: *mut edict_t);
    fn Weapon_Machinegun(ent: *mut edict_t);
    fn Weapon_Chaingun(ent: *mut edict_t);
    fn Weapon_HyperBlaster(ent: *mut edict_t);
    fn Weapon_RocketLauncher(ent: *mut edict_t);
    fn Weapon_Grenade(ent: *mut edict_t);
    fn Weapon_GrenadeLauncher(ent: *mut edict_t);
    fn Weapon_Railgun(ent: *mut edict_t);
    fn Weapon_BFG(ent: *mut edict_t);
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
pub type C2RustUnnamed = libc::c_uint;
pub const EV_OTHER_TELEPORT: C2RustUnnamed = 7;
pub const EV_PLAYER_TELEPORT: C2RustUnnamed = 6;
pub const EV_FALLFAR: C2RustUnnamed = 5;
pub const EV_FALL: C2RustUnnamed = 4;
pub const EV_FALLSHORT: C2RustUnnamed = 3;
pub const EV_FOOTSTEP: C2RustUnnamed = 2;
pub const EV_ITEM_RESPAWN: C2RustUnnamed = 1;
pub const EV_NONE: C2RustUnnamed = 0;
pub type gclient_t = gclient_s;
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
pub type C2RustUnnamed_0 = libc::c_uint;
pub const AMMO_SLUGS: C2RustUnnamed_0 = 5;
pub const AMMO_CELLS: C2RustUnnamed_0 = 4;
pub const AMMO_GRENADES: C2RustUnnamed_0 = 3;
pub const AMMO_ROCKETS: C2RustUnnamed_0 = 2;
pub const AMMO_SHELLS: C2RustUnnamed_0 = 1;
pub const AMMO_BULLETS: C2RustUnnamed_0 = 0;
pub type C2RustUnnamed_1 = libc::c_uint;
pub const MOVETYPE_BOUNCE: C2RustUnnamed_1 = 9;
pub const MOVETYPE_FLYMISSILE: C2RustUnnamed_1 = 8;
pub const MOVETYPE_TOSS: C2RustUnnamed_1 = 7;
pub const MOVETYPE_FLY: C2RustUnnamed_1 = 6;
pub const MOVETYPE_STEP: C2RustUnnamed_1 = 5;
pub const MOVETYPE_WALK: C2RustUnnamed_1 = 4;
pub const MOVETYPE_STOP: C2RustUnnamed_1 = 3;
pub const MOVETYPE_PUSH: C2RustUnnamed_1 = 2;
pub const MOVETYPE_NOCLIP: C2RustUnnamed_1 = 1;
pub const MOVETYPE_NONE: C2RustUnnamed_1 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gitem_armor_t {
    pub base_count: libc::c_int,
    pub max_count: libc::c_int,
    pub normal_protection: libc::c_float,
    pub energy_protection: libc::c_float,
    pub armor: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct game_locals_t {
    pub helpmessage1: [libc::c_char; 512],
    pub helpmessage2: [libc::c_char; 512],
    pub helpchanged: libc::c_int,
    pub clients: *mut gclient_t,
    pub spawnpoint: [libc::c_char; 512],
    pub maxclients: libc::c_int,
    pub maxentities: libc::c_int,
    pub serverflags: libc::c_int,
    pub num_items: libc::c_int,
    pub autosaved: qboolean,
}
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
#[no_mangle]
pub static mut jacketarmor_info: gitem_armor_t = {
    let mut init = gitem_armor_t {
        base_count: 25 as libc::c_int,
        max_count: 50 as libc::c_int,
        normal_protection: 0.30f64 as libc::c_float,
        energy_protection: 0.00f64 as libc::c_float,
        armor: 1 as libc::c_int,
    };
    init
};
#[no_mangle]
pub static mut combatarmor_info: gitem_armor_t = {
    let mut init = gitem_armor_t {
        base_count: 50 as libc::c_int,
        max_count: 100 as libc::c_int,
        normal_protection: 0.60f64 as libc::c_float,
        energy_protection: 0.30f64 as libc::c_float,
        armor: 2 as libc::c_int,
    };
    init
};
#[no_mangle]
pub static mut bodyarmor_info: gitem_armor_t = {
    let mut init = gitem_armor_t {
        base_count: 100 as libc::c_int,
        max_count: 200 as libc::c_int,
        normal_protection: 0.80f64 as libc::c_float,
        energy_protection: 0.60f64 as libc::c_float,
        armor: 3 as libc::c_int,
    };
    init
};
static mut power_screen_index: libc::c_int = 0;
static mut power_shield_index: libc::c_int = 0;
static mut quad_drop_timeout_hack: libc::c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn GetItemByIndex(mut index: libc::c_int) -> *mut gitem_t {
    if index == 0 as libc::c_int || index >= game.num_items {
        return 0 as *mut gitem_t;
    }
    return &mut *itemlist.as_mut_ptr().offset(index as isize) as *mut gitem_t;
}
#[no_mangle]
pub unsafe extern "C" fn FindItemByClassname(
    mut classname: *mut libc::c_char,
) -> *mut gitem_t {
    let mut i: libc::c_int = 0;
    let mut it: *mut gitem_t = 0 as *mut gitem_t;
    it = itemlist.as_mut_ptr();
    i = 0 as libc::c_int;
    while i < game.num_items {
        if !((*it).classname).is_null() {
            if Q_stricmp((*it).classname, classname) == 0 {
                return it;
            }
        }
        i += 1;
        it = it.offset(1);
    }
    return 0 as *mut gitem_t;
}
#[no_mangle]
pub unsafe extern "C" fn FindItem(mut pickup_name: *mut libc::c_char) -> *mut gitem_t {
    let mut i: libc::c_int = 0;
    let mut it: *mut gitem_t = 0 as *mut gitem_t;
    it = itemlist.as_mut_ptr();
    i = 0 as libc::c_int;
    while i < game.num_items {
        if !((*it).pickup_name).is_null() {
            if Q_stricmp((*it).pickup_name, pickup_name) == 0 {
                return it;
            }
        }
        i += 1;
        it = it.offset(1);
    }
    return 0 as *mut gitem_t;
}
#[no_mangle]
pub unsafe extern "C" fn DoRespawn(mut ent: *mut edict_t) {
    if !((*ent).team).is_null() {
        let mut master: *mut edict_t = 0 as *mut edict_t;
        let mut count: libc::c_int = 0;
        let mut choice: libc::c_int = 0;
        master = (*ent).teammaster;
        count = 0 as libc::c_int;
        ent = master;
        while !ent.is_null() {
            ent = (*ent).chain;
            count += 1;
        }
        choice = rand() % count;
        count = 0 as libc::c_int;
        ent = master;
        while count < choice {
            ent = (*ent).chain;
            count += 1;
        }
    }
    (*ent).svflags &= !(0x1 as libc::c_int);
    (*ent).solid = SOLID_TRIGGER;
    (gi.linkentity).expect("non-null function pointer")(ent);
    (*ent).s.event = EV_ITEM_RESPAWN as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn SetRespawn(mut ent: *mut edict_t, mut delay: libc::c_float) {
    let ref mut fresh0 = (*ent).flags;
    *fresh0 = (*fresh0 as libc::c_uint | 0x80000000 as libc::c_uint) as libc::c_int;
    (*ent).svflags |= 0x1 as libc::c_int;
    (*ent).solid = SOLID_NOT;
    (*ent).nextthink = level.time + delay;
    let ref mut fresh1 = (*ent).think;
    *fresh1 = Some(DoRespawn as unsafe extern "C" fn(*mut edict_t) -> ());
    (gi.linkentity).expect("non-null function pointer")(ent);
}
#[no_mangle]
pub unsafe extern "C" fn Pickup_Powerup(
    mut ent: *mut edict_t,
    mut other: *mut edict_t,
) -> qboolean {
    let mut quantity: libc::c_int = 0;
    quantity = (*(*other).client)
        .pers
        .inventory[((*ent).item).offset_from(itemlist.as_mut_ptr()) as libc::c_long
        as usize];
    if (*skill).value == 1 as libc::c_int as libc::c_float
        && quantity >= 2 as libc::c_int
        || (*skill).value >= 2 as libc::c_int as libc::c_float
            && quantity >= 1 as libc::c_int
    {
        return false_0;
    }
    if (*coop).value != 0. && (*(*ent).item).flags & 8 as libc::c_int != 0
        && quantity > 0 as libc::c_int
    {
        return false_0;
    }
    let ref mut fresh2 = (*(*other).client)
        .pers
        .inventory[((*ent).item).offset_from(itemlist.as_mut_ptr()) as libc::c_long
        as usize];
    *fresh2 += 1;
    if (*deathmatch).value != 0. {
        if (*ent).spawnflags & 0x10000 as libc::c_int == 0 {
            SetRespawn(ent, (*(*ent).item).quantity as libc::c_float);
        }
        if (*dmflags).value as libc::c_int & 0x10 as libc::c_int != 0
            || (*(*ent).item).use_0
                == Some(
                    Use_Quad as unsafe extern "C" fn(*mut edict_t, *mut gitem_t) -> (),
                ) && (*ent).spawnflags & 0x20000 as libc::c_int != 0
        {
            if (*(*ent).item).use_0
                == Some(
                    Use_Quad as unsafe extern "C" fn(*mut edict_t, *mut gitem_t) -> (),
                ) && (*ent).spawnflags & 0x20000 as libc::c_int != 0
            {
                quad_drop_timeout_hack = (((*ent).nextthink - level.time)
                    as libc::c_double / 0.1f64) as libc::c_int;
            }
            ((*(*ent).item).use_0)
                .expect("non-null function pointer")(other, (*ent).item);
        }
    }
    return true_0;
}
#[no_mangle]
pub unsafe extern "C" fn Drop_General(mut ent: *mut edict_t, mut item: *mut gitem_t) {
    Drop_Item(ent, item);
    let ref mut fresh3 = (*(*ent).client)
        .pers
        .inventory[item.offset_from(itemlist.as_mut_ptr()) as libc::c_long as usize];
    *fresh3 -= 1;
    ValidateSelectedItem(ent);
}
#[no_mangle]
pub unsafe extern "C" fn Pickup_Adrenaline(
    mut ent: *mut edict_t,
    mut other: *mut edict_t,
) -> qboolean {
    if (*deathmatch).value == 0. {
        (*other).max_health += 1 as libc::c_int;
    }
    if (*other).health < (*other).max_health {
        (*other).health = (*other).max_health;
    }
    if (*ent).spawnflags & 0x10000 as libc::c_int == 0 && (*deathmatch).value != 0. {
        SetRespawn(ent, (*(*ent).item).quantity as libc::c_float);
    }
    return true_0;
}
#[no_mangle]
pub unsafe extern "C" fn Pickup_AncientHead(
    mut ent: *mut edict_t,
    mut other: *mut edict_t,
) -> qboolean {
    (*other).max_health += 2 as libc::c_int;
    if (*ent).spawnflags & 0x10000 as libc::c_int == 0 && (*deathmatch).value != 0. {
        SetRespawn(ent, (*(*ent).item).quantity as libc::c_float);
    }
    return true_0;
}
#[no_mangle]
pub unsafe extern "C" fn Pickup_Bandolier(
    mut ent: *mut edict_t,
    mut other: *mut edict_t,
) -> qboolean {
    let mut item: *mut gitem_t = 0 as *mut gitem_t;
    let mut index: libc::c_int = 0;
    if (*(*other).client).pers.max_bullets < 250 as libc::c_int {
        (*(*other).client).pers.max_bullets = 250 as libc::c_int;
    }
    if (*(*other).client).pers.max_shells < 150 as libc::c_int {
        (*(*other).client).pers.max_shells = 150 as libc::c_int;
    }
    if (*(*other).client).pers.max_cells < 250 as libc::c_int {
        (*(*other).client).pers.max_cells = 250 as libc::c_int;
    }
    if (*(*other).client).pers.max_slugs < 75 as libc::c_int {
        (*(*other).client).pers.max_slugs = 75 as libc::c_int;
    }
    item = FindItem(
        b"Bullets\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if !item.is_null() {
        index = item.offset_from(itemlist.as_mut_ptr()) as libc::c_long as libc::c_int;
        (*(*other).client).pers.inventory[index as usize] += (*item).quantity;
        if (*(*other).client).pers.inventory[index as usize]
            > (*(*other).client).pers.max_bullets
        {
            (*(*other).client)
                .pers
                .inventory[index as usize] = (*(*other).client).pers.max_bullets;
        }
    }
    item = FindItem(
        b"Shells\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if !item.is_null() {
        index = item.offset_from(itemlist.as_mut_ptr()) as libc::c_long as libc::c_int;
        (*(*other).client).pers.inventory[index as usize] += (*item).quantity;
        if (*(*other).client).pers.inventory[index as usize]
            > (*(*other).client).pers.max_shells
        {
            (*(*other).client)
                .pers
                .inventory[index as usize] = (*(*other).client).pers.max_shells;
        }
    }
    if (*ent).spawnflags & 0x10000 as libc::c_int == 0 && (*deathmatch).value != 0. {
        SetRespawn(ent, (*(*ent).item).quantity as libc::c_float);
    }
    return true_0;
}
#[no_mangle]
pub unsafe extern "C" fn Pickup_Pack(
    mut ent: *mut edict_t,
    mut other: *mut edict_t,
) -> qboolean {
    let mut item: *mut gitem_t = 0 as *mut gitem_t;
    let mut index: libc::c_int = 0;
    if (*(*other).client).pers.max_bullets < 300 as libc::c_int {
        (*(*other).client).pers.max_bullets = 300 as libc::c_int;
    }
    if (*(*other).client).pers.max_shells < 200 as libc::c_int {
        (*(*other).client).pers.max_shells = 200 as libc::c_int;
    }
    if (*(*other).client).pers.max_rockets < 100 as libc::c_int {
        (*(*other).client).pers.max_rockets = 100 as libc::c_int;
    }
    if (*(*other).client).pers.max_grenades < 100 as libc::c_int {
        (*(*other).client).pers.max_grenades = 100 as libc::c_int;
    }
    if (*(*other).client).pers.max_cells < 300 as libc::c_int {
        (*(*other).client).pers.max_cells = 300 as libc::c_int;
    }
    if (*(*other).client).pers.max_slugs < 100 as libc::c_int {
        (*(*other).client).pers.max_slugs = 100 as libc::c_int;
    }
    item = FindItem(
        b"Bullets\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if !item.is_null() {
        index = item.offset_from(itemlist.as_mut_ptr()) as libc::c_long as libc::c_int;
        (*(*other).client).pers.inventory[index as usize] += (*item).quantity;
        if (*(*other).client).pers.inventory[index as usize]
            > (*(*other).client).pers.max_bullets
        {
            (*(*other).client)
                .pers
                .inventory[index as usize] = (*(*other).client).pers.max_bullets;
        }
    }
    item = FindItem(
        b"Shells\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if !item.is_null() {
        index = item.offset_from(itemlist.as_mut_ptr()) as libc::c_long as libc::c_int;
        (*(*other).client).pers.inventory[index as usize] += (*item).quantity;
        if (*(*other).client).pers.inventory[index as usize]
            > (*(*other).client).pers.max_shells
        {
            (*(*other).client)
                .pers
                .inventory[index as usize] = (*(*other).client).pers.max_shells;
        }
    }
    item = FindItem(b"Cells\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    if !item.is_null() {
        index = item.offset_from(itemlist.as_mut_ptr()) as libc::c_long as libc::c_int;
        (*(*other).client).pers.inventory[index as usize] += (*item).quantity;
        if (*(*other).client).pers.inventory[index as usize]
            > (*(*other).client).pers.max_cells
        {
            (*(*other).client)
                .pers
                .inventory[index as usize] = (*(*other).client).pers.max_cells;
        }
    }
    item = FindItem(
        b"Grenades\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if !item.is_null() {
        index = item.offset_from(itemlist.as_mut_ptr()) as libc::c_long as libc::c_int;
        (*(*other).client).pers.inventory[index as usize] += (*item).quantity;
        if (*(*other).client).pers.inventory[index as usize]
            > (*(*other).client).pers.max_grenades
        {
            (*(*other).client)
                .pers
                .inventory[index as usize] = (*(*other).client).pers.max_grenades;
        }
    }
    item = FindItem(
        b"Rockets\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if !item.is_null() {
        index = item.offset_from(itemlist.as_mut_ptr()) as libc::c_long as libc::c_int;
        (*(*other).client).pers.inventory[index as usize] += (*item).quantity;
        if (*(*other).client).pers.inventory[index as usize]
            > (*(*other).client).pers.max_rockets
        {
            (*(*other).client)
                .pers
                .inventory[index as usize] = (*(*other).client).pers.max_rockets;
        }
    }
    item = FindItem(b"Slugs\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    if !item.is_null() {
        index = item.offset_from(itemlist.as_mut_ptr()) as libc::c_long as libc::c_int;
        (*(*other).client).pers.inventory[index as usize] += (*item).quantity;
        if (*(*other).client).pers.inventory[index as usize]
            > (*(*other).client).pers.max_slugs
        {
            (*(*other).client)
                .pers
                .inventory[index as usize] = (*(*other).client).pers.max_slugs;
        }
    }
    if (*ent).spawnflags & 0x10000 as libc::c_int == 0 && (*deathmatch).value != 0. {
        SetRespawn(ent, (*(*ent).item).quantity as libc::c_float);
    }
    return true_0;
}
#[no_mangle]
pub unsafe extern "C" fn Use_Quad(mut ent: *mut edict_t, mut item: *mut gitem_t) {
    let mut timeout: libc::c_int = 0;
    let ref mut fresh4 = (*(*ent).client)
        .pers
        .inventory[item.offset_from(itemlist.as_mut_ptr()) as libc::c_long as usize];
    *fresh4 -= 1;
    ValidateSelectedItem(ent);
    if quad_drop_timeout_hack != 0 {
        timeout = quad_drop_timeout_hack;
        quad_drop_timeout_hack = 0 as libc::c_int;
    } else {
        timeout = 300 as libc::c_int;
    }
    if (*(*ent).client).quad_framenum > level.framenum as libc::c_float {
        (*(*ent).client).quad_framenum += timeout as libc::c_float;
    } else {
        (*(*ent).client).quad_framenum = (level.framenum + timeout) as libc::c_float;
    }
    (gi.sound)
        .expect(
            "non-null function pointer",
        )(
        ent,
        3 as libc::c_int,
        (gi.soundindex)
            .expect(
                "non-null function pointer",
            )(
            b"items/damage.wav\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        ),
        1 as libc::c_int as libc::c_float,
        1 as libc::c_int as libc::c_float,
        0 as libc::c_int as libc::c_float,
    );
}
#[no_mangle]
pub unsafe extern "C" fn Use_Breather(mut ent: *mut edict_t, mut item: *mut gitem_t) {
    let ref mut fresh5 = (*(*ent).client)
        .pers
        .inventory[item.offset_from(itemlist.as_mut_ptr()) as libc::c_long as usize];
    *fresh5 -= 1;
    ValidateSelectedItem(ent);
    if (*(*ent).client).breather_framenum > level.framenum as libc::c_float {
        (*(*ent).client).breather_framenum += 300 as libc::c_int as libc::c_float;
    } else {
        (*(*ent).client)
            .breather_framenum = (level.framenum + 300 as libc::c_int) as libc::c_float;
    };
}
#[no_mangle]
pub unsafe extern "C" fn Use_Envirosuit(mut ent: *mut edict_t, mut item: *mut gitem_t) {
    let ref mut fresh6 = (*(*ent).client)
        .pers
        .inventory[item.offset_from(itemlist.as_mut_ptr()) as libc::c_long as usize];
    *fresh6 -= 1;
    ValidateSelectedItem(ent);
    if (*(*ent).client).enviro_framenum > level.framenum as libc::c_float {
        (*(*ent).client).enviro_framenum += 300 as libc::c_int as libc::c_float;
    } else {
        (*(*ent).client)
            .enviro_framenum = (level.framenum + 300 as libc::c_int) as libc::c_float;
    };
}
#[no_mangle]
pub unsafe extern "C" fn Use_Invulnerability(
    mut ent: *mut edict_t,
    mut item: *mut gitem_t,
) {
    let ref mut fresh7 = (*(*ent).client)
        .pers
        .inventory[item.offset_from(itemlist.as_mut_ptr()) as libc::c_long as usize];
    *fresh7 -= 1;
    ValidateSelectedItem(ent);
    if (*(*ent).client).invincible_framenum > level.framenum as libc::c_float {
        (*(*ent).client).invincible_framenum += 300 as libc::c_int as libc::c_float;
    } else {
        (*(*ent).client)
            .invincible_framenum = (level.framenum + 300 as libc::c_int)
            as libc::c_float;
    }
    (gi.sound)
        .expect(
            "non-null function pointer",
        )(
        ent,
        3 as libc::c_int,
        (gi.soundindex)
            .expect(
                "non-null function pointer",
            )(
            b"items/protect.wav\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        ),
        1 as libc::c_int as libc::c_float,
        1 as libc::c_int as libc::c_float,
        0 as libc::c_int as libc::c_float,
    );
}
#[no_mangle]
pub unsafe extern "C" fn Use_Silencer(mut ent: *mut edict_t, mut item: *mut gitem_t) {
    let ref mut fresh8 = (*(*ent).client)
        .pers
        .inventory[item.offset_from(itemlist.as_mut_ptr()) as libc::c_long as usize];
    *fresh8 -= 1;
    ValidateSelectedItem(ent);
    (*(*ent).client).silencer_shots += 30 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn Pickup_Key(
    mut ent: *mut edict_t,
    mut other: *mut edict_t,
) -> qboolean {
    if (*coop).value != 0. {
        if strcmp(
            (*ent).classname,
            b"key_power_cube\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            if (*(*other).client).pers.power_cubes
                & ((*ent).spawnflags & 0xff00 as libc::c_int) >> 8 as libc::c_int != 0
            {
                return false_0;
            }
            let ref mut fresh9 = (*(*other).client)
                .pers
                .inventory[((*ent).item).offset_from(itemlist.as_mut_ptr())
                as libc::c_long as usize];
            *fresh9 += 1;
            (*(*other).client).pers.power_cubes
                |= ((*ent).spawnflags & 0xff00 as libc::c_int) >> 8 as libc::c_int;
        } else {
            if (*(*other).client)
                .pers
                .inventory[((*ent).item).offset_from(itemlist.as_mut_ptr())
                as libc::c_long as usize] != 0
            {
                return false_0;
            }
            (*(*other).client)
                .pers
                .inventory[((*ent).item).offset_from(itemlist.as_mut_ptr())
                as libc::c_long as usize] = 1 as libc::c_int;
        }
        return true_0;
    }
    let ref mut fresh10 = (*(*other).client)
        .pers
        .inventory[((*ent).item).offset_from(itemlist.as_mut_ptr()) as libc::c_long
        as usize];
    *fresh10 += 1;
    return true_0;
}
#[no_mangle]
pub unsafe extern "C" fn Add_Ammo(
    mut ent: *mut edict_t,
    mut item: *mut gitem_t,
    mut count: libc::c_int,
) -> qboolean {
    let mut index: libc::c_int = 0;
    let mut max: libc::c_int = 0;
    if ((*ent).client).is_null() {
        return false_0;
    }
    if (*item).tag == AMMO_BULLETS as libc::c_int {
        max = (*(*ent).client).pers.max_bullets;
    } else if (*item).tag == AMMO_SHELLS as libc::c_int {
        max = (*(*ent).client).pers.max_shells;
    } else if (*item).tag == AMMO_ROCKETS as libc::c_int {
        max = (*(*ent).client).pers.max_rockets;
    } else if (*item).tag == AMMO_GRENADES as libc::c_int {
        max = (*(*ent).client).pers.max_grenades;
    } else if (*item).tag == AMMO_CELLS as libc::c_int {
        max = (*(*ent).client).pers.max_cells;
    } else if (*item).tag == AMMO_SLUGS as libc::c_int {
        max = (*(*ent).client).pers.max_slugs;
    } else {
        return false_0
    }
    index = item.offset_from(itemlist.as_mut_ptr()) as libc::c_long as libc::c_int;
    if (*(*ent).client).pers.inventory[index as usize] == max {
        return false_0;
    }
    (*(*ent).client).pers.inventory[index as usize] += count;
    if (*(*ent).client).pers.inventory[index as usize] > max {
        (*(*ent).client).pers.inventory[index as usize] = max;
    }
    return true_0;
}
#[no_mangle]
pub unsafe extern "C" fn Pickup_Ammo(
    mut ent: *mut edict_t,
    mut other: *mut edict_t,
) -> qboolean {
    let mut oldcount: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    let mut weapon: qboolean = false_0;
    weapon = ((*(*ent).item).flags & 1 as libc::c_int) as qboolean;
    if weapon as libc::c_uint != 0
        && (*dmflags).value as libc::c_int & 0x2000 as libc::c_int != 0
    {
        count = 1000 as libc::c_int;
    } else if (*ent).count != 0 {
        count = (*ent).count;
    } else {
        count = (*(*ent).item).quantity;
    }
    oldcount = (*(*other).client)
        .pers
        .inventory[((*ent).item).offset_from(itemlist.as_mut_ptr()) as libc::c_long
        as usize];
    if Add_Ammo(other, (*ent).item, count) as u64 == 0 {
        return false_0;
    }
    if weapon as libc::c_uint != 0 && oldcount == 0 {
        if (*(*other).client).pers.weapon != (*ent).item
            && ((*deathmatch).value == 0.
                || (*(*other).client).pers.weapon
                    == FindItem(
                        b"blaster\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    ))
        {
            let ref mut fresh11 = (*(*other).client).newweapon;
            *fresh11 = (*ent).item;
        }
    }
    if (*ent).spawnflags & (0x10000 as libc::c_int | 0x20000 as libc::c_int) == 0
        && (*deathmatch).value != 0.
    {
        SetRespawn(ent, 30 as libc::c_int as libc::c_float);
    }
    return true_0;
}
#[no_mangle]
pub unsafe extern "C" fn Drop_Ammo(mut ent: *mut edict_t, mut item: *mut gitem_t) {
    let mut dropped: *mut edict_t = 0 as *mut edict_t;
    let mut index: libc::c_int = 0;
    index = item.offset_from(itemlist.as_mut_ptr()) as libc::c_long as libc::c_int;
    dropped = Drop_Item(ent, item);
    if (*(*ent).client).pers.inventory[index as usize] >= (*item).quantity {
        (*dropped).count = (*item).quantity;
    } else {
        (*dropped).count = (*(*ent).client).pers.inventory[index as usize];
    }
    if !((*(*ent).client).pers.weapon).is_null()
        && (*(*(*ent).client).pers.weapon).tag == AMMO_GRENADES as libc::c_int
        && (*item).tag == AMMO_GRENADES as libc::c_int
        && (*(*ent).client).pers.inventory[index as usize] - (*dropped).count
            <= 0 as libc::c_int
    {
        (gi.cprintf)
            .expect(
                "non-null function pointer",
            )(
            ent,
            2 as libc::c_int,
            b"Can't drop current weapon\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        G_FreeEdict(dropped);
        return;
    }
    (*(*ent).client).pers.inventory[index as usize] -= (*dropped).count;
    ValidateSelectedItem(ent);
}
#[no_mangle]
pub unsafe extern "C" fn MegaHealth_think(mut self_0: *mut edict_t) {
    if (*(*self_0).owner).health > (*(*self_0).owner).max_health {
        (*self_0).nextthink = level.time + 1 as libc::c_int as libc::c_float;
        (*(*self_0).owner).health -= 1 as libc::c_int;
        return;
    }
    if (*self_0).spawnflags & 0x10000 as libc::c_int == 0 && (*deathmatch).value != 0. {
        SetRespawn(self_0, 20 as libc::c_int as libc::c_float);
    } else {
        G_FreeEdict(self_0);
    };
}
#[no_mangle]
pub unsafe extern "C" fn Pickup_Health(
    mut ent: *mut edict_t,
    mut other: *mut edict_t,
) -> qboolean {
    if (*ent).style & 1 as libc::c_int == 0 {
        if (*other).health >= (*other).max_health {
            return false_0;
        }
    }
    (*other).health += (*ent).count;
    if (*ent).style & 1 as libc::c_int == 0 {
        if (*other).health > (*other).max_health {
            (*other).health = (*other).max_health;
        }
    }
    if (*ent).style & 2 as libc::c_int != 0 {
        let ref mut fresh12 = (*ent).think;
        *fresh12 = Some(MegaHealth_think as unsafe extern "C" fn(*mut edict_t) -> ());
        (*ent).nextthink = level.time + 5 as libc::c_int as libc::c_float;
        let ref mut fresh13 = (*ent).owner;
        *fresh13 = other;
        let ref mut fresh14 = (*ent).flags;
        *fresh14 = (*fresh14 as libc::c_uint | 0x80000000 as libc::c_uint)
            as libc::c_int;
        (*ent).svflags |= 0x1 as libc::c_int;
        (*ent).solid = SOLID_NOT;
    } else if (*ent).spawnflags & 0x10000 as libc::c_int == 0
        && (*deathmatch).value != 0.
    {
        SetRespawn(ent, 30 as libc::c_int as libc::c_float);
    }
    return true_0;
}
#[no_mangle]
pub unsafe extern "C" fn ArmorIndex(mut ent: *mut edict_t) -> libc::c_int {
    if ((*ent).client).is_null() {
        return 0 as libc::c_int;
    }
    if (*(*ent).client).pers.inventory[jacket_armor_index as usize] > 0 as libc::c_int {
        return jacket_armor_index;
    }
    if (*(*ent).client).pers.inventory[combat_armor_index as usize] > 0 as libc::c_int {
        return combat_armor_index;
    }
    if (*(*ent).client).pers.inventory[body_armor_index as usize] > 0 as libc::c_int {
        return body_armor_index;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn Pickup_Armor(
    mut ent: *mut edict_t,
    mut other: *mut edict_t,
) -> qboolean {
    let mut old_armor_index: libc::c_int = 0;
    let mut oldinfo: *mut gitem_armor_t = 0 as *mut gitem_armor_t;
    let mut newinfo: *mut gitem_armor_t = 0 as *mut gitem_armor_t;
    let mut newcount: libc::c_int = 0;
    let mut salvage: libc::c_float = 0.;
    let mut salvagecount: libc::c_int = 0;
    newinfo = (*(*ent).item).info as *mut gitem_armor_t;
    old_armor_index = ArmorIndex(other);
    if (*(*ent).item).tag == 4 as libc::c_int {
        if old_armor_index == 0 {
            (*(*other).client)
                .pers
                .inventory[jacket_armor_index as usize] = 2 as libc::c_int;
        } else {
            (*(*other).client).pers.inventory[old_armor_index as usize]
                += 2 as libc::c_int;
        }
    } else if old_armor_index == 0 {
        (*(*other).client)
            .pers
            .inventory[((*ent).item).offset_from(itemlist.as_mut_ptr()) as libc::c_long
            as usize] = (*newinfo).base_count;
    } else {
        if old_armor_index == jacket_armor_index {
            oldinfo = &mut jacketarmor_info;
        } else if old_armor_index == combat_armor_index {
            oldinfo = &mut combatarmor_info;
        } else {
            oldinfo = &mut bodyarmor_info;
        }
        if (*newinfo).normal_protection > (*oldinfo).normal_protection {
            salvage = (*oldinfo).normal_protection / (*newinfo).normal_protection;
            salvagecount = (salvage
                * (*(*other).client).pers.inventory[old_armor_index as usize]
                    as libc::c_float) as libc::c_int;
            newcount = (*newinfo).base_count + salvagecount;
            if newcount > (*newinfo).max_count {
                newcount = (*newinfo).max_count;
            }
            (*(*other).client)
                .pers
                .inventory[old_armor_index as usize] = 0 as libc::c_int;
            (*(*other).client)
                .pers
                .inventory[((*ent).item).offset_from(itemlist.as_mut_ptr())
                as libc::c_long as usize] = newcount;
        } else {
            salvage = (*newinfo).normal_protection / (*oldinfo).normal_protection;
            salvagecount = (salvage * (*newinfo).base_count as libc::c_float)
                as libc::c_int;
            newcount = (*(*other).client).pers.inventory[old_armor_index as usize]
                + salvagecount;
            if newcount > (*oldinfo).max_count {
                newcount = (*oldinfo).max_count;
            }
            if (*(*other).client).pers.inventory[old_armor_index as usize] >= newcount {
                return false_0;
            }
            (*(*other).client).pers.inventory[old_armor_index as usize] = newcount;
        }
    }
    if (*ent).spawnflags & 0x10000 as libc::c_int == 0 && (*deathmatch).value != 0. {
        SetRespawn(ent, 20 as libc::c_int as libc::c_float);
    }
    return true_0;
}
#[no_mangle]
pub unsafe extern "C" fn PowerArmorType(mut ent: *mut edict_t) -> libc::c_int {
    if ((*ent).client).is_null() {
        return 0 as libc::c_int;
    }
    if (*ent).flags & 0x1000 as libc::c_int == 0 {
        return 0 as libc::c_int;
    }
    if (*(*ent).client).pers.inventory[power_shield_index as usize] > 0 as libc::c_int {
        return 2 as libc::c_int;
    }
    if (*(*ent).client).pers.inventory[power_screen_index as usize] > 0 as libc::c_int {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn Use_PowerArmor(mut ent: *mut edict_t, mut item: *mut gitem_t) {
    let mut index: libc::c_int = 0;
    if (*ent).flags & 0x1000 as libc::c_int != 0 {
        (*ent).flags &= !(0x1000 as libc::c_int);
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
                b"misc/power2.wav\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            ),
            1 as libc::c_int as libc::c_float,
            1 as libc::c_int as libc::c_float,
            0 as libc::c_int as libc::c_float,
        );
    } else {
        index = (FindItem(
            b"cells\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ))
            .offset_from(itemlist.as_mut_ptr()) as libc::c_long as libc::c_int;
        if (*(*ent).client).pers.inventory[index as usize] == 0 {
            (gi.cprintf)
                .expect(
                    "non-null function pointer",
                )(
                ent,
                2 as libc::c_int,
                b"No cells for power armor.\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
            return;
        }
        (*ent).flags |= 0x1000 as libc::c_int;
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
                b"misc/power1.wav\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            ),
            1 as libc::c_int as libc::c_float,
            1 as libc::c_int as libc::c_float,
            0 as libc::c_int as libc::c_float,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn Pickup_PowerArmor(
    mut ent: *mut edict_t,
    mut other: *mut edict_t,
) -> qboolean {
    let mut quantity: libc::c_int = 0;
    quantity = (*(*other).client)
        .pers
        .inventory[((*ent).item).offset_from(itemlist.as_mut_ptr()) as libc::c_long
        as usize];
    let ref mut fresh15 = (*(*other).client)
        .pers
        .inventory[((*ent).item).offset_from(itemlist.as_mut_ptr()) as libc::c_long
        as usize];
    *fresh15 += 1;
    if (*deathmatch).value != 0. {
        if (*ent).spawnflags & 0x10000 as libc::c_int == 0 {
            SetRespawn(ent, (*(*ent).item).quantity as libc::c_float);
        }
        if quantity == 0 {
            ((*(*ent).item).use_0)
                .expect("non-null function pointer")(other, (*ent).item);
        }
    }
    return true_0;
}
#[no_mangle]
pub unsafe extern "C" fn Drop_PowerArmor(mut ent: *mut edict_t, mut item: *mut gitem_t) {
    if (*ent).flags & 0x1000 as libc::c_int != 0
        && (*(*ent).client)
            .pers
            .inventory[item.offset_from(itemlist.as_mut_ptr()) as libc::c_long as usize]
            == 1 as libc::c_int
    {
        Use_PowerArmor(ent, item);
    }
    Drop_General(ent, item);
}
#[no_mangle]
pub unsafe extern "C" fn Touch_Item(
    mut ent: *mut edict_t,
    mut other: *mut edict_t,
    mut plane: *mut cplane_t,
    mut surf: *mut csurface_t,
) {
    let mut taken: qboolean = false_0;
    if ((*other).client).is_null() {
        return;
    }
    if (*other).health < 1 as libc::c_int {
        return;
    }
    if ((*(*ent).item).pickup).is_none() {
        return;
    }
    taken = ((*(*ent).item).pickup).expect("non-null function pointer")(ent, other);
    if taken as u64 != 0 {
        (*(*other).client).bonus_alpha = 0.25f64 as libc::c_float;
        (*(*other).client)
            .ps
            .stats[7 as libc::c_int
            as usize] = (gi.imageindex)
            .expect("non-null function pointer")((*(*ent).item).icon) as libc::c_short;
        (*(*other).client)
            .ps
            .stats[8 as libc::c_int
            as usize] = ((32 as libc::c_int + 256 as libc::c_int + 256 as libc::c_int
            + 256 as libc::c_int + 256 as libc::c_int) as libc::c_long
            + ((*ent).item).offset_from(itemlist.as_mut_ptr()) as libc::c_long)
            as libc::c_short;
        (*(*other).client)
            .pickup_msg_time = (level.time as libc::c_double + 3.0f64) as libc::c_float;
        if ((*(*ent).item).use_0).is_some() {
            let ref mut fresh16 = (*(*other).client)
                .ps
                .stats[12 as libc::c_int as usize];
            *fresh16 = ((*ent).item).offset_from(itemlist.as_mut_ptr()) as libc::c_long
                as libc::c_short;
            (*(*other).client).pers.selected_item = *fresh16 as libc::c_int;
        }
        if (*(*ent).item).pickup
            == Some(
                Pickup_Health
                    as unsafe extern "C" fn(*mut edict_t, *mut edict_t) -> qboolean,
            )
        {
            if (*ent).count == 2 as libc::c_int {
                (gi.sound)
                    .expect(
                        "non-null function pointer",
                    )(
                    other,
                    3 as libc::c_int,
                    (gi.soundindex)
                        .expect(
                            "non-null function pointer",
                        )(
                        b"items/s_health.wav\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    ),
                    1 as libc::c_int as libc::c_float,
                    1 as libc::c_int as libc::c_float,
                    0 as libc::c_int as libc::c_float,
                );
            } else if (*ent).count == 10 as libc::c_int {
                (gi.sound)
                    .expect(
                        "non-null function pointer",
                    )(
                    other,
                    3 as libc::c_int,
                    (gi.soundindex)
                        .expect(
                            "non-null function pointer",
                        )(
                        b"items/n_health.wav\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    ),
                    1 as libc::c_int as libc::c_float,
                    1 as libc::c_int as libc::c_float,
                    0 as libc::c_int as libc::c_float,
                );
            } else if (*ent).count == 25 as libc::c_int {
                (gi.sound)
                    .expect(
                        "non-null function pointer",
                    )(
                    other,
                    3 as libc::c_int,
                    (gi.soundindex)
                        .expect(
                            "non-null function pointer",
                        )(
                        b"items/l_health.wav\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    ),
                    1 as libc::c_int as libc::c_float,
                    1 as libc::c_int as libc::c_float,
                    0 as libc::c_int as libc::c_float,
                );
            } else {
                (gi.sound)
                    .expect(
                        "non-null function pointer",
                    )(
                    other,
                    3 as libc::c_int,
                    (gi.soundindex)
                        .expect(
                            "non-null function pointer",
                        )(
                        b"items/m_health.wav\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    ),
                    1 as libc::c_int as libc::c_float,
                    1 as libc::c_int as libc::c_float,
                    0 as libc::c_int as libc::c_float,
                );
            }
        } else if !((*(*ent).item).pickup_sound).is_null() {
            (gi.sound)
                .expect(
                    "non-null function pointer",
                )(
                other,
                3 as libc::c_int,
                (gi.soundindex)
                    .expect("non-null function pointer")((*(*ent).item).pickup_sound),
                1 as libc::c_int as libc::c_float,
                1 as libc::c_int as libc::c_float,
                0 as libc::c_int as libc::c_float,
            );
        }
    }
    if (*ent).spawnflags & 0x40000 as libc::c_int == 0 {
        G_UseTargets(ent, other);
        (*ent).spawnflags |= 0x40000 as libc::c_int;
    }
    if taken as u64 == 0 {
        return;
    }
    if !((*coop).value != 0. && (*(*ent).item).flags & 8 as libc::c_int != 0)
        || (*ent).spawnflags & (0x10000 as libc::c_int | 0x20000 as libc::c_int) != 0
    {
        if (*ent).flags as libc::c_uint & 0x80000000 as libc::c_uint != 0 {
            let ref mut fresh17 = (*ent).flags;
            *fresh17 = (*fresh17 as libc::c_uint & !(0x80000000 as libc::c_uint))
                as libc::c_int;
        } else {
            G_FreeEdict(ent);
        }
    }
}
unsafe extern "C" fn drop_temp_touch(
    mut ent: *mut edict_t,
    mut other: *mut edict_t,
    mut plane: *mut cplane_t,
    mut surf: *mut csurface_t,
) {
    if other == (*ent).owner {
        return;
    }
    Touch_Item(ent, other, plane, surf);
}
unsafe extern "C" fn drop_make_touchable(mut ent: *mut edict_t) {
    let ref mut fresh18 = (*ent).touch;
    *fresh18 = Some(
        Touch_Item
            as unsafe extern "C" fn(
                *mut edict_t,
                *mut edict_t,
                *mut cplane_t,
                *mut csurface_t,
            ) -> (),
    );
    if (*deathmatch).value != 0. {
        (*ent).nextthink = level.time + 29 as libc::c_int as libc::c_float;
        let ref mut fresh19 = (*ent).think;
        *fresh19 = Some(G_FreeEdict as unsafe extern "C" fn(*mut edict_t) -> ());
    }
}
#[no_mangle]
pub unsafe extern "C" fn Drop_Item(
    mut ent: *mut edict_t,
    mut item: *mut gitem_t,
) -> *mut edict_t {
    let mut dropped: *mut edict_t = 0 as *mut edict_t;
    let mut forward: vec3_t = [0.; 3];
    let mut right: vec3_t = [0.; 3];
    let mut offset: vec3_t = [0.; 3];
    dropped = G_Spawn();
    let ref mut fresh20 = (*dropped).classname;
    *fresh20 = (*item).classname;
    let ref mut fresh21 = (*dropped).item;
    *fresh21 = item;
    (*dropped).spawnflags = 0x10000 as libc::c_int;
    (*dropped).s.effects = (*item).world_model_flags as libc::c_uint;
    (*dropped).s.renderfx = 512 as libc::c_int;
    (*dropped).mins[0 as libc::c_int as usize] = -(15 as libc::c_int) as vec_t;
    (*dropped).mins[1 as libc::c_int as usize] = -(15 as libc::c_int) as vec_t;
    (*dropped).mins[2 as libc::c_int as usize] = -(15 as libc::c_int) as vec_t;
    (*dropped).maxs[0 as libc::c_int as usize] = 15 as libc::c_int as vec_t;
    (*dropped).maxs[1 as libc::c_int as usize] = 15 as libc::c_int as vec_t;
    (*dropped).maxs[2 as libc::c_int as usize] = 15 as libc::c_int as vec_t;
    (gi.setmodel)
        .expect("non-null function pointer")(dropped, (*(*dropped).item).world_model);
    (*dropped).solid = SOLID_TRIGGER;
    (*dropped).movetype = MOVETYPE_TOSS as libc::c_int;
    let ref mut fresh22 = (*dropped).touch;
    *fresh22 = Some(
        drop_temp_touch
            as unsafe extern "C" fn(
                *mut edict_t,
                *mut edict_t,
                *mut cplane_t,
                *mut csurface_t,
            ) -> (),
    );
    let ref mut fresh23 = (*dropped).owner;
    *fresh23 = ent;
    if !((*ent).client).is_null() {
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
        AngleVectors(
            ((*(*ent).client).v_angle).as_mut_ptr(),
            forward.as_mut_ptr(),
            right.as_mut_ptr(),
            0 as *mut vec_t,
        );
        offset[0 as libc::c_int as usize] = 24 as libc::c_int as vec_t;
        offset[1 as libc::c_int as usize] = 0 as libc::c_int as vec_t;
        offset[2 as libc::c_int as usize] = -(16 as libc::c_int) as vec_t;
        G_ProjectSource(
            ((*ent).s.origin).as_mut_ptr(),
            offset.as_mut_ptr(),
            forward.as_mut_ptr(),
            right.as_mut_ptr(),
            ((*dropped).s.origin).as_mut_ptr(),
        );
        trace = (gi.trace)
            .expect(
                "non-null function pointer",
            )(
            ((*ent).s.origin).as_mut_ptr(),
            ((*dropped).mins).as_mut_ptr(),
            ((*dropped).maxs).as_mut_ptr(),
            ((*dropped).s.origin).as_mut_ptr(),
            ent,
            1 as libc::c_int,
        );
        (*dropped)
            .s
            .origin[0 as libc::c_int as usize] = trace.endpos[0 as libc::c_int as usize];
        (*dropped)
            .s
            .origin[1 as libc::c_int as usize] = trace.endpos[1 as libc::c_int as usize];
        (*dropped)
            .s
            .origin[2 as libc::c_int as usize] = trace.endpos[2 as libc::c_int as usize];
    } else {
        AngleVectors(
            ((*ent).s.angles).as_mut_ptr(),
            forward.as_mut_ptr(),
            right.as_mut_ptr(),
            0 as *mut vec_t,
        );
        (*dropped)
            .s
            .origin[0 as libc::c_int
            as usize] = (*ent).s.origin[0 as libc::c_int as usize];
        (*dropped)
            .s
            .origin[1 as libc::c_int
            as usize] = (*ent).s.origin[1 as libc::c_int as usize];
        (*dropped)
            .s
            .origin[2 as libc::c_int
            as usize] = (*ent).s.origin[2 as libc::c_int as usize];
    }
    VectorScale(
        forward.as_mut_ptr(),
        100 as libc::c_int as vec_t,
        ((*dropped).velocity).as_mut_ptr(),
    );
    (*dropped).velocity[2 as libc::c_int as usize] = 300 as libc::c_int as vec_t;
    let ref mut fresh24 = (*dropped).think;
    *fresh24 = Some(drop_make_touchable as unsafe extern "C" fn(*mut edict_t) -> ());
    (*dropped).nextthink = level.time + 1 as libc::c_int as libc::c_float;
    (gi.linkentity).expect("non-null function pointer")(dropped);
    return dropped;
}
#[no_mangle]
pub unsafe extern "C" fn Use_Item(
    mut ent: *mut edict_t,
    mut other: *mut edict_t,
    mut activator: *mut edict_t,
) {
    (*ent).svflags &= !(0x1 as libc::c_int);
    let ref mut fresh25 = (*ent).use_0;
    *fresh25 = None;
    if (*ent).spawnflags & 0x2 as libc::c_int != 0 {
        (*ent).solid = SOLID_BBOX;
        let ref mut fresh26 = (*ent).touch;
        *fresh26 = None;
    } else {
        (*ent).solid = SOLID_TRIGGER;
        let ref mut fresh27 = (*ent).touch;
        *fresh27 = Some(
            Touch_Item
                as unsafe extern "C" fn(
                    *mut edict_t,
                    *mut edict_t,
                    *mut cplane_t,
                    *mut csurface_t,
                ) -> (),
        );
    }
    (gi.linkentity).expect("non-null function pointer")(ent);
}
#[no_mangle]
pub unsafe extern "C" fn droptofloor(mut ent: *mut edict_t) {
    let mut tr: trace_t = trace_t {
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
    let mut dest: vec3_t = [0.; 3];
    let mut v: *mut libc::c_float = 0 as *mut libc::c_float;
    v = tv(
        -(15 as libc::c_int) as libc::c_float,
        -(15 as libc::c_int) as libc::c_float,
        -(15 as libc::c_int) as libc::c_float,
    );
    (*ent).mins[0 as libc::c_int as usize] = *v.offset(0 as libc::c_int as isize);
    (*ent).mins[1 as libc::c_int as usize] = *v.offset(1 as libc::c_int as isize);
    (*ent).mins[2 as libc::c_int as usize] = *v.offset(2 as libc::c_int as isize);
    v = tv(
        15 as libc::c_int as libc::c_float,
        15 as libc::c_int as libc::c_float,
        15 as libc::c_int as libc::c_float,
    );
    (*ent).maxs[0 as libc::c_int as usize] = *v.offset(0 as libc::c_int as isize);
    (*ent).maxs[1 as libc::c_int as usize] = *v.offset(1 as libc::c_int as isize);
    (*ent).maxs[2 as libc::c_int as usize] = *v.offset(2 as libc::c_int as isize);
    if !((*ent).model).is_null() {
        (gi.setmodel).expect("non-null function pointer")(ent, (*ent).model);
    } else {
        (gi.setmodel)
            .expect("non-null function pointer")(ent, (*(*ent).item).world_model);
    }
    (*ent).solid = SOLID_TRIGGER;
    (*ent).movetype = MOVETYPE_TOSS as libc::c_int;
    let ref mut fresh28 = (*ent).touch;
    *fresh28 = Some(
        Touch_Item
            as unsafe extern "C" fn(
                *mut edict_t,
                *mut edict_t,
                *mut cplane_t,
                *mut csurface_t,
            ) -> (),
    );
    v = tv(
        0 as libc::c_int as libc::c_float,
        0 as libc::c_int as libc::c_float,
        -(128 as libc::c_int) as libc::c_float,
    );
    dest[0 as libc::c_int
        as usize] = (*ent).s.origin[0 as libc::c_int as usize]
        + *v.offset(0 as libc::c_int as isize);
    dest[1 as libc::c_int
        as usize] = (*ent).s.origin[1 as libc::c_int as usize]
        + *v.offset(1 as libc::c_int as isize);
    dest[2 as libc::c_int
        as usize] = (*ent).s.origin[2 as libc::c_int as usize]
        + *v.offset(2 as libc::c_int as isize);
    tr = (gi.trace)
        .expect(
            "non-null function pointer",
        )(
        ((*ent).s.origin).as_mut_ptr(),
        ((*ent).mins).as_mut_ptr(),
        ((*ent).maxs).as_mut_ptr(),
        dest.as_mut_ptr(),
        ent,
        1 as libc::c_int | 2 as libc::c_int,
    );
    if tr.startsolid as u64 != 0 {
        (gi.dprintf)
            .expect(
                "non-null function pointer",
            )(
            b"droptofloor: %s startsolid at %s\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            (*ent).classname,
            vtos(((*ent).s.origin).as_mut_ptr()),
        );
        G_FreeEdict(ent);
        return;
    }
    (*ent).s.origin[0 as libc::c_int as usize] = tr.endpos[0 as libc::c_int as usize];
    (*ent).s.origin[1 as libc::c_int as usize] = tr.endpos[1 as libc::c_int as usize];
    (*ent).s.origin[2 as libc::c_int as usize] = tr.endpos[2 as libc::c_int as usize];
    if !((*ent).team).is_null() {
        (*ent).flags &= !(0x400 as libc::c_int);
        let ref mut fresh29 = (*ent).chain;
        *fresh29 = (*ent).teamchain;
        let ref mut fresh30 = (*ent).teamchain;
        *fresh30 = 0 as *mut edict_t;
        (*ent).svflags |= 0x1 as libc::c_int;
        (*ent).solid = SOLID_NOT;
        if ent == (*ent).teammaster {
            (*ent).nextthink = (level.time as libc::c_double + 0.1f64) as libc::c_float;
            let ref mut fresh31 = (*ent).think;
            *fresh31 = Some(DoRespawn as unsafe extern "C" fn(*mut edict_t) -> ());
        }
    }
    if (*ent).spawnflags & 0x2 as libc::c_int != 0 {
        (*ent).solid = SOLID_BBOX;
        let ref mut fresh32 = (*ent).touch;
        *fresh32 = None;
        (*ent).s.effects &= !(0x1 as libc::c_int) as libc::c_uint;
        (*ent).s.renderfx &= !(512 as libc::c_int);
    }
    if (*ent).spawnflags & 0x1 as libc::c_int != 0 {
        (*ent).svflags |= 0x1 as libc::c_int;
        (*ent).solid = SOLID_NOT;
        let ref mut fresh33 = (*ent).use_0;
        *fresh33 = Some(
            Use_Item
                as unsafe extern "C" fn(*mut edict_t, *mut edict_t, *mut edict_t) -> (),
        );
    }
    (gi.linkentity).expect("non-null function pointer")(ent);
}
#[no_mangle]
pub unsafe extern "C" fn PrecacheItem(mut it: *mut gitem_t) {
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut start: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut data: [libc::c_char; 64] = [0; 64];
    let mut len: libc::c_int = 0;
    let mut ammo: *mut gitem_t = 0 as *mut gitem_t;
    if it.is_null() {
        return;
    }
    if !((*it).pickup_sound).is_null() {
        (gi.soundindex).expect("non-null function pointer")((*it).pickup_sound);
    }
    if !((*it).world_model).is_null() {
        (gi.modelindex).expect("non-null function pointer")((*it).world_model);
    }
    if !((*it).view_model).is_null() {
        (gi.modelindex).expect("non-null function pointer")((*it).view_model);
    }
    if !((*it).icon).is_null() {
        (gi.imageindex).expect("non-null function pointer")((*it).icon);
    }
    if !((*it).ammo).is_null()
        && *((*it).ammo).offset(0 as libc::c_int as isize) as libc::c_int != 0
    {
        ammo = FindItem((*it).ammo);
        if ammo != it {
            PrecacheItem(ammo);
        }
    }
    s = (*it).precaches;
    if s.is_null() || *s.offset(0 as libc::c_int as isize) == 0 {
        return;
    }
    while *s != 0 {
        start = s;
        while *s as libc::c_int != 0 && *s as libc::c_int != ' ' as i32 {
            s = s.offset(1);
        }
        len = s.offset_from(start) as libc::c_long as libc::c_int;
        if len >= 64 as libc::c_int || len < 5 as libc::c_int {
            (gi.error)
                .expect(
                    "non-null function pointer",
                )(
                b"PrecacheItem: %s has bad precache string\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                (*it).classname,
            );
        }
        memcpy(
            data.as_mut_ptr() as *mut libc::c_void,
            start as *const libc::c_void,
            len as libc::c_ulong,
        );
        data[len as usize] = 0 as libc::c_int as libc::c_char;
        if *s != 0 {
            s = s.offset(1);
        }
        if strcmp(
            data.as_mut_ptr().offset(len as isize).offset(-(3 as libc::c_int as isize)),
            b"md2\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            (gi.modelindex).expect("non-null function pointer")(data.as_mut_ptr());
        } else if strcmp(
            data.as_mut_ptr().offset(len as isize).offset(-(3 as libc::c_int as isize)),
            b"sp2\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            (gi.modelindex).expect("non-null function pointer")(data.as_mut_ptr());
        } else if strcmp(
            data.as_mut_ptr().offset(len as isize).offset(-(3 as libc::c_int as isize)),
            b"wav\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            (gi.soundindex).expect("non-null function pointer")(data.as_mut_ptr());
        }
        if strcmp(
            data.as_mut_ptr().offset(len as isize).offset(-(3 as libc::c_int as isize)),
            b"pcx\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            (gi.imageindex).expect("non-null function pointer")(data.as_mut_ptr());
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn SpawnItem(mut ent: *mut edict_t, mut item: *mut gitem_t) {
    PrecacheItem(item);
    if (*ent).spawnflags != 0 {
        if strcmp(
            (*ent).classname,
            b"key_power_cube\0" as *const u8 as *const libc::c_char,
        ) != 0 as libc::c_int
        {
            (*ent).spawnflags = 0 as libc::c_int;
            (gi.dprintf)
                .expect(
                    "non-null function pointer",
                )(
                b"%s at %s has invalid spawnflags set\n\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                (*ent).classname,
                vtos(((*ent).s.origin).as_mut_ptr()),
            );
        }
    }
    if (*deathmatch).value != 0. {
        if (*dmflags).value as libc::c_int & 0x800 as libc::c_int != 0 {
            if (*item).pickup
                == Some(
                    Pickup_Armor
                        as unsafe extern "C" fn(*mut edict_t, *mut edict_t) -> qboolean,
                )
                || (*item).pickup
                    == Some(
                        Pickup_PowerArmor
                            as unsafe extern "C" fn(
                                *mut edict_t,
                                *mut edict_t,
                            ) -> qboolean,
                    )
            {
                G_FreeEdict(ent);
                return;
            }
        }
        if (*dmflags).value as libc::c_int & 0x2 as libc::c_int != 0 {
            if (*item).pickup
                == Some(
                    Pickup_Powerup
                        as unsafe extern "C" fn(*mut edict_t, *mut edict_t) -> qboolean,
                )
            {
                G_FreeEdict(ent);
                return;
            }
        }
        if (*dmflags).value as libc::c_int & 0x1 as libc::c_int != 0 {
            if (*item).pickup
                == Some(
                    Pickup_Health
                        as unsafe extern "C" fn(*mut edict_t, *mut edict_t) -> qboolean,
                )
                || (*item).pickup
                    == Some(
                        Pickup_Adrenaline
                            as unsafe extern "C" fn(
                                *mut edict_t,
                                *mut edict_t,
                            ) -> qboolean,
                    )
                || (*item).pickup
                    == Some(
                        Pickup_AncientHead
                            as unsafe extern "C" fn(
                                *mut edict_t,
                                *mut edict_t,
                            ) -> qboolean,
                    )
            {
                G_FreeEdict(ent);
                return;
            }
        }
        if (*dmflags).value as libc::c_int & 0x2000 as libc::c_int != 0 {
            if (*item).flags == 2 as libc::c_int
                || strcmp(
                    (*ent).classname,
                    b"weapon_bfg\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
            {
                G_FreeEdict(ent);
                return;
            }
        }
    }
    if (*coop).value != 0.
        && strcmp(
            (*ent).classname,
            b"key_power_cube\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
    {
        (*ent).spawnflags |= (1 as libc::c_int) << 8 as libc::c_int + level.power_cubes;
        level.power_cubes += 1;
    }
    if (*coop).value != 0. && (*item).flags & 8 as libc::c_int != 0 {
        let ref mut fresh34 = (*item).drop;
        *fresh34 = None;
    }
    let ref mut fresh35 = (*ent).item;
    *fresh35 = item;
    (*ent)
        .nextthink = (level.time as libc::c_double
        + 2 as libc::c_int as libc::c_double * 0.1f64) as libc::c_float;
    let ref mut fresh36 = (*ent).think;
    *fresh36 = Some(droptofloor as unsafe extern "C" fn(*mut edict_t) -> ());
    (*ent).s.effects = (*item).world_model_flags as libc::c_uint;
    (*ent).s.renderfx = 512 as libc::c_int;
    if !((*ent).model).is_null() {
        (gi.modelindex).expect("non-null function pointer")((*ent).model);
    }
}
#[no_mangle]
pub static mut itemlist: [gitem_t; 43] = unsafe {
    [
        {
            let mut init = gitem_s {
                classname: 0 as *const libc::c_char as *mut libc::c_char,
                pickup: None,
                use_0: None,
                drop: None,
                weaponthink: None,
                pickup_sound: 0 as *const libc::c_char as *mut libc::c_char,
                world_model: 0 as *const libc::c_char as *mut libc::c_char,
                world_model_flags: 0,
                view_model: 0 as *const libc::c_char as *mut libc::c_char,
                icon: 0 as *const libc::c_char as *mut libc::c_char,
                pickup_name: 0 as *const libc::c_char as *mut libc::c_char,
                count_width: 0,
                quantity: 0,
                ammo: 0 as *const libc::c_char as *mut libc::c_char,
                flags: 0,
                weapmodel: 0,
                info: 0 as *const libc::c_void as *mut libc::c_void,
                tag: 0,
                precaches: 0 as *const libc::c_char as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = gitem_s {
                classname: b"item_armor_body\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                pickup: Some(
                    Pickup_Armor
                        as unsafe extern "C" fn(*mut edict_t, *mut edict_t) -> qboolean,
                ),
                use_0: None,
                drop: None,
                weaponthink: None,
                pickup_sound: b"misc/ar1_pkup.wav\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                world_model: b"models/items/armor/body/tris.md2\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                world_model_flags: 0x1 as libc::c_int,
                view_model: 0 as *const libc::c_char as *mut libc::c_char,
                icon: b"i_bodyarmor\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                pickup_name: b"Body Armor\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                count_width: 3 as libc::c_int,
                quantity: 0 as libc::c_int,
                ammo: 0 as *const libc::c_char as *mut libc::c_char,
                flags: 4 as libc::c_int,
                weapmodel: 0 as libc::c_int,
                info: &bodyarmor_info as *const gitem_armor_t as *mut gitem_armor_t
                    as *mut libc::c_void,
                tag: 3 as libc::c_int,
                precaches: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = gitem_s {
                classname: b"item_armor_combat\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                pickup: Some(
                    Pickup_Armor
                        as unsafe extern "C" fn(*mut edict_t, *mut edict_t) -> qboolean,
                ),
                use_0: None,
                drop: None,
                weaponthink: None,
                pickup_sound: b"misc/ar1_pkup.wav\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                world_model: b"models/items/armor/combat/tris.md2\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                world_model_flags: 0x1 as libc::c_int,
                view_model: 0 as *const libc::c_char as *mut libc::c_char,
                icon: b"i_combatarmor\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                pickup_name: b"Combat Armor\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                count_width: 3 as libc::c_int,
                quantity: 0 as libc::c_int,
                ammo: 0 as *const libc::c_char as *mut libc::c_char,
                flags: 4 as libc::c_int,
                weapmodel: 0 as libc::c_int,
                info: &combatarmor_info as *const gitem_armor_t as *mut gitem_armor_t
                    as *mut libc::c_void,
                tag: 2 as libc::c_int,
                precaches: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = gitem_s {
                classname: b"item_armor_jacket\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                pickup: Some(
                    Pickup_Armor
                        as unsafe extern "C" fn(*mut edict_t, *mut edict_t) -> qboolean,
                ),
                use_0: None,
                drop: None,
                weaponthink: None,
                pickup_sound: b"misc/ar1_pkup.wav\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                world_model: b"models/items/armor/jacket/tris.md2\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                world_model_flags: 0x1 as libc::c_int,
                view_model: 0 as *const libc::c_char as *mut libc::c_char,
                icon: b"i_jacketarmor\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                pickup_name: b"Jacket Armor\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                count_width: 3 as libc::c_int,
                quantity: 0 as libc::c_int,
                ammo: 0 as *const libc::c_char as *mut libc::c_char,
                flags: 4 as libc::c_int,
                weapmodel: 0 as libc::c_int,
                info: &jacketarmor_info as *const gitem_armor_t as *mut gitem_armor_t
                    as *mut libc::c_void,
                tag: 1 as libc::c_int,
                precaches: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = gitem_s {
                classname: b"item_armor_shard\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                pickup: Some(
                    Pickup_Armor
                        as unsafe extern "C" fn(*mut edict_t, *mut edict_t) -> qboolean,
                ),
                use_0: None,
                drop: None,
                weaponthink: None,
                pickup_sound: b"misc/ar2_pkup.wav\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                world_model: b"models/items/armor/shard/tris.md2\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                world_model_flags: 0x1 as libc::c_int,
                view_model: 0 as *const libc::c_char as *mut libc::c_char,
                icon: b"i_jacketarmor\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                pickup_name: b"Armor Shard\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                count_width: 3 as libc::c_int,
                quantity: 0 as libc::c_int,
                ammo: 0 as *const libc::c_char as *mut libc::c_char,
                flags: 4 as libc::c_int,
                weapmodel: 0 as libc::c_int,
                info: 0 as *const libc::c_void as *mut libc::c_void,
                tag: 4 as libc::c_int,
                precaches: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = gitem_s {
                classname: b"item_power_screen\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                pickup: Some(
                    Pickup_PowerArmor
                        as unsafe extern "C" fn(*mut edict_t, *mut edict_t) -> qboolean,
                ),
                use_0: Some(
                    Use_PowerArmor
                        as unsafe extern "C" fn(*mut edict_t, *mut gitem_t) -> (),
                ),
                drop: Some(
                    Drop_PowerArmor
                        as unsafe extern "C" fn(*mut edict_t, *mut gitem_t) -> (),
                ),
                weaponthink: None,
                pickup_sound: b"misc/ar3_pkup.wav\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                world_model: b"models/items/armor/screen/tris.md2\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                world_model_flags: 0x1 as libc::c_int,
                view_model: 0 as *const libc::c_char as *mut libc::c_char,
                icon: b"i_powerscreen\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                pickup_name: b"Power Screen\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                count_width: 0 as libc::c_int,
                quantity: 60 as libc::c_int,
                ammo: 0 as *const libc::c_char as *mut libc::c_char,
                flags: 4 as libc::c_int,
                weapmodel: 0 as libc::c_int,
                info: 0 as *const libc::c_void as *mut libc::c_void,
                tag: 0 as libc::c_int,
                precaches: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = gitem_s {
                classname: b"item_power_shield\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                pickup: Some(
                    Pickup_PowerArmor
                        as unsafe extern "C" fn(*mut edict_t, *mut edict_t) -> qboolean,
                ),
                use_0: Some(
                    Use_PowerArmor
                        as unsafe extern "C" fn(*mut edict_t, *mut gitem_t) -> (),
                ),
                drop: Some(
                    Drop_PowerArmor
                        as unsafe extern "C" fn(*mut edict_t, *mut gitem_t) -> (),
                ),
                weaponthink: None,
                pickup_sound: b"misc/ar3_pkup.wav\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                world_model: b"models/items/armor/shield/tris.md2\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                world_model_flags: 0x1 as libc::c_int,
                view_model: 0 as *const libc::c_char as *mut libc::c_char,
                icon: b"i_powershield\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                pickup_name: b"Power Shield\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                count_width: 0 as libc::c_int,
                quantity: 60 as libc::c_int,
                ammo: 0 as *const libc::c_char as *mut libc::c_char,
                flags: 4 as libc::c_int,
                weapmodel: 0 as libc::c_int,
                info: 0 as *const libc::c_void as *mut libc::c_void,
                tag: 0 as libc::c_int,
                precaches: b"misc/power2.wav misc/power1.wav\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = gitem_s {
                classname: b"weapon_blaster\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                pickup: None,
                use_0: Some(
                    Use_Weapon as unsafe extern "C" fn(*mut edict_t, *mut gitem_t) -> (),
                ),
                drop: None,
                weaponthink: Some(
                    Weapon_Blaster as unsafe extern "C" fn(*mut edict_t) -> (),
                ),
                pickup_sound: b"misc/w_pkup.wav\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                world_model: 0 as *const libc::c_char as *mut libc::c_char,
                world_model_flags: 0 as libc::c_int,
                view_model: b"models/weapons/v_blast/tris.md2\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                icon: b"w_blaster\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                pickup_name: b"Blaster\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                count_width: 0 as libc::c_int,
                quantity: 0 as libc::c_int,
                ammo: 0 as *const libc::c_char as *mut libc::c_char,
                flags: 1 as libc::c_int | 8 as libc::c_int,
                weapmodel: 1 as libc::c_int,
                info: 0 as *const libc::c_void as *mut libc::c_void,
                tag: 0 as libc::c_int,
                precaches: b"weapons/blastf1a.wav misc/lasfly.wav\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = gitem_s {
                classname: b"weapon_shotgun\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                pickup: Some(
                    Pickup_Weapon
                        as unsafe extern "C" fn(*mut edict_t, *mut edict_t) -> qboolean,
                ),
                use_0: Some(
                    Use_Weapon as unsafe extern "C" fn(*mut edict_t, *mut gitem_t) -> (),
                ),
                drop: Some(
                    Drop_Weapon as unsafe extern "C" fn(*mut edict_t, *mut gitem_t) -> (),
                ),
                weaponthink: Some(
                    Weapon_Shotgun as unsafe extern "C" fn(*mut edict_t) -> (),
                ),
                pickup_sound: b"misc/w_pkup.wav\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                world_model: b"models/weapons/g_shotg/tris.md2\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                world_model_flags: 0x1 as libc::c_int,
                view_model: b"models/weapons/v_shotg/tris.md2\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                icon: b"w_shotgun\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                pickup_name: b"Shotgun\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                count_width: 0 as libc::c_int,
                quantity: 1 as libc::c_int,
                ammo: b"Shells\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                flags: 1 as libc::c_int | 8 as libc::c_int,
                weapmodel: 2 as libc::c_int,
                info: 0 as *const libc::c_void as *mut libc::c_void,
                tag: 0 as libc::c_int,
                precaches: b"weapons/shotgf1b.wav weapons/shotgr1b.wav\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = gitem_s {
                classname: b"weapon_supershotgun\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                pickup: Some(
                    Pickup_Weapon
                        as unsafe extern "C" fn(*mut edict_t, *mut edict_t) -> qboolean,
                ),
                use_0: Some(
                    Use_Weapon as unsafe extern "C" fn(*mut edict_t, *mut gitem_t) -> (),
                ),
                drop: Some(
                    Drop_Weapon as unsafe extern "C" fn(*mut edict_t, *mut gitem_t) -> (),
                ),
                weaponthink: Some(
                    Weapon_SuperShotgun as unsafe extern "C" fn(*mut edict_t) -> (),
                ),
                pickup_sound: b"misc/w_pkup.wav\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                world_model: b"models/weapons/g_shotg2/tris.md2\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                world_model_flags: 0x1 as libc::c_int,
                view_model: b"models/weapons/v_shotg2/tris.md2\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                icon: b"w_sshotgun\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                pickup_name: b"Super Shotgun\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                count_width: 0 as libc::c_int,
                quantity: 2 as libc::c_int,
                ammo: b"Shells\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                flags: 1 as libc::c_int | 8 as libc::c_int,
                weapmodel: 3 as libc::c_int,
                info: 0 as *const libc::c_void as *mut libc::c_void,
                tag: 0 as libc::c_int,
                precaches: b"weapons/sshotf1b.wav\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = gitem_s {
                classname: b"weapon_machinegun\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                pickup: Some(
                    Pickup_Weapon
                        as unsafe extern "C" fn(*mut edict_t, *mut edict_t) -> qboolean,
                ),
                use_0: Some(
                    Use_Weapon as unsafe extern "C" fn(*mut edict_t, *mut gitem_t) -> (),
                ),
                drop: Some(
                    Drop_Weapon as unsafe extern "C" fn(*mut edict_t, *mut gitem_t) -> (),
                ),
                weaponthink: Some(
                    Weapon_Machinegun as unsafe extern "C" fn(*mut edict_t) -> (),
                ),
                pickup_sound: b"misc/w_pkup.wav\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                world_model: b"models/weapons/g_machn/tris.md2\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                world_model_flags: 0x1 as libc::c_int,
                view_model: b"models/weapons/v_machn/tris.md2\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                icon: b"w_machinegun\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                pickup_name: b"Machinegun\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                count_width: 0 as libc::c_int,
                quantity: 1 as libc::c_int,
                ammo: b"Bullets\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                flags: 1 as libc::c_int | 8 as libc::c_int,
                weapmodel: 4 as libc::c_int,
                info: 0 as *const libc::c_void as *mut libc::c_void,
                tag: 0 as libc::c_int,
                precaches: b"weapons/machgf1b.wav weapons/machgf2b.wav weapons/machgf3b.wav weapons/machgf4b.wav weapons/machgf5b.wav\0"
                    as *const u8 as *const libc::c_char as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = gitem_s {
                classname: b"weapon_chaingun\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                pickup: Some(
                    Pickup_Weapon
                        as unsafe extern "C" fn(*mut edict_t, *mut edict_t) -> qboolean,
                ),
                use_0: Some(
                    Use_Weapon as unsafe extern "C" fn(*mut edict_t, *mut gitem_t) -> (),
                ),
                drop: Some(
                    Drop_Weapon as unsafe extern "C" fn(*mut edict_t, *mut gitem_t) -> (),
                ),
                weaponthink: Some(
                    Weapon_Chaingun as unsafe extern "C" fn(*mut edict_t) -> (),
                ),
                pickup_sound: b"misc/w_pkup.wav\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                world_model: b"models/weapons/g_chain/tris.md2\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                world_model_flags: 0x1 as libc::c_int,
                view_model: b"models/weapons/v_chain/tris.md2\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                icon: b"w_chaingun\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                pickup_name: b"Chaingun\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                count_width: 0 as libc::c_int,
                quantity: 1 as libc::c_int,
                ammo: b"Bullets\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                flags: 1 as libc::c_int | 8 as libc::c_int,
                weapmodel: 5 as libc::c_int,
                info: 0 as *const libc::c_void as *mut libc::c_void,
                tag: 0 as libc::c_int,
                precaches: b"weapons/chngnu1a.wav weapons/chngnl1a.wav weapons/machgf3b.wav` weapons/chngnd1a.wav\0"
                    as *const u8 as *const libc::c_char as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = gitem_s {
                classname: b"ammo_grenades\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                pickup: Some(
                    Pickup_Ammo
                        as unsafe extern "C" fn(*mut edict_t, *mut edict_t) -> qboolean,
                ),
                use_0: Some(
                    Use_Weapon as unsafe extern "C" fn(*mut edict_t, *mut gitem_t) -> (),
                ),
                drop: Some(
                    Drop_Ammo as unsafe extern "C" fn(*mut edict_t, *mut gitem_t) -> (),
                ),
                weaponthink: Some(
                    Weapon_Grenade as unsafe extern "C" fn(*mut edict_t) -> (),
                ),
                pickup_sound: b"misc/am_pkup.wav\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                world_model: b"models/items/ammo/grenades/medium/tris.md2\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                world_model_flags: 0 as libc::c_int,
                view_model: b"models/weapons/v_handgr/tris.md2\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                icon: b"a_grenades\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                pickup_name: b"Grenades\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                count_width: 3 as libc::c_int,
                quantity: 5 as libc::c_int,
                ammo: b"grenades\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                flags: 2 as libc::c_int | 1 as libc::c_int,
                weapmodel: 6 as libc::c_int,
                info: 0 as *const libc::c_void as *mut libc::c_void,
                tag: AMMO_GRENADES as libc::c_int,
                precaches: b"weapons/hgrent1a.wav weapons/hgrena1b.wav weapons/hgrenc1b.wav weapons/hgrenb1a.wav weapons/hgrenb2a.wav \0"
                    as *const u8 as *const libc::c_char as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = gitem_s {
                classname: b"weapon_grenadelauncher\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                pickup: Some(
                    Pickup_Weapon
                        as unsafe extern "C" fn(*mut edict_t, *mut edict_t) -> qboolean,
                ),
                use_0: Some(
                    Use_Weapon as unsafe extern "C" fn(*mut edict_t, *mut gitem_t) -> (),
                ),
                drop: Some(
                    Drop_Weapon as unsafe extern "C" fn(*mut edict_t, *mut gitem_t) -> (),
                ),
                weaponthink: Some(
                    Weapon_GrenadeLauncher as unsafe extern "C" fn(*mut edict_t) -> (),
                ),
                pickup_sound: b"misc/w_pkup.wav\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                world_model: b"models/weapons/g_launch/tris.md2\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                world_model_flags: 0x1 as libc::c_int,
                view_model: b"models/weapons/v_launch/tris.md2\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                icon: b"w_glauncher\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                pickup_name: b"Grenade Launcher\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                count_width: 0 as libc::c_int,
                quantity: 1 as libc::c_int,
                ammo: b"Grenades\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                flags: 1 as libc::c_int | 8 as libc::c_int,
                weapmodel: 7 as libc::c_int,
                info: 0 as *const libc::c_void as *mut libc::c_void,
                tag: 0 as libc::c_int,
                precaches: b"models/objects/grenade/tris.md2 weapons/grenlf1a.wav weapons/grenlr1b.wav weapons/grenlb1b.wav\0"
                    as *const u8 as *const libc::c_char as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = gitem_s {
                classname: b"weapon_rocketlauncher\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                pickup: Some(
                    Pickup_Weapon
                        as unsafe extern "C" fn(*mut edict_t, *mut edict_t) -> qboolean,
                ),
                use_0: Some(
                    Use_Weapon as unsafe extern "C" fn(*mut edict_t, *mut gitem_t) -> (),
                ),
                drop: Some(
                    Drop_Weapon as unsafe extern "C" fn(*mut edict_t, *mut gitem_t) -> (),
                ),
                weaponthink: Some(
                    Weapon_RocketLauncher as unsafe extern "C" fn(*mut edict_t) -> (),
                ),
                pickup_sound: b"misc/w_pkup.wav\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                world_model: b"models/weapons/g_rocket/tris.md2\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                world_model_flags: 0x1 as libc::c_int,
                view_model: b"models/weapons/v_rocket/tris.md2\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                icon: b"w_rlauncher\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                pickup_name: b"Rocket Launcher\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                count_width: 0 as libc::c_int,
                quantity: 1 as libc::c_int,
                ammo: b"Rockets\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                flags: 1 as libc::c_int | 8 as libc::c_int,
                weapmodel: 8 as libc::c_int,
                info: 0 as *const libc::c_void as *mut libc::c_void,
                tag: 0 as libc::c_int,
                precaches: b"models/objects/rocket/tris.md2 weapons/rockfly.wav weapons/rocklf1a.wav weapons/rocklr1b.wav models/objects/debris2/tris.md2\0"
                    as *const u8 as *const libc::c_char as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = gitem_s {
                classname: b"weapon_hyperblaster\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                pickup: Some(
                    Pickup_Weapon
                        as unsafe extern "C" fn(*mut edict_t, *mut edict_t) -> qboolean,
                ),
                use_0: Some(
                    Use_Weapon as unsafe extern "C" fn(*mut edict_t, *mut gitem_t) -> (),
                ),
                drop: Some(
                    Drop_Weapon as unsafe extern "C" fn(*mut edict_t, *mut gitem_t) -> (),
                ),
                weaponthink: Some(
                    Weapon_HyperBlaster as unsafe extern "C" fn(*mut edict_t) -> (),
                ),
                pickup_sound: b"misc/w_pkup.wav\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                world_model: b"models/weapons/g_hyperb/tris.md2\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                world_model_flags: 0x1 as libc::c_int,
                view_model: b"models/weapons/v_hyperb/tris.md2\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                icon: b"w_hyperblaster\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                pickup_name: b"HyperBlaster\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                count_width: 0 as libc::c_int,
                quantity: 1 as libc::c_int,
                ammo: b"Cells\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                flags: 1 as libc::c_int | 8 as libc::c_int,
                weapmodel: 9 as libc::c_int,
                info: 0 as *const libc::c_void as *mut libc::c_void,
                tag: 0 as libc::c_int,
                precaches: b"weapons/hyprbu1a.wav weapons/hyprbl1a.wav weapons/hyprbf1a.wav weapons/hyprbd1a.wav misc/lasfly.wav\0"
                    as *const u8 as *const libc::c_char as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = gitem_s {
                classname: b"weapon_railgun\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                pickup: Some(
                    Pickup_Weapon
                        as unsafe extern "C" fn(*mut edict_t, *mut edict_t) -> qboolean,
                ),
                use_0: Some(
                    Use_Weapon as unsafe extern "C" fn(*mut edict_t, *mut gitem_t) -> (),
                ),
                drop: Some(
                    Drop_Weapon as unsafe extern "C" fn(*mut edict_t, *mut gitem_t) -> (),
                ),
                weaponthink: Some(
                    Weapon_Railgun as unsafe extern "C" fn(*mut edict_t) -> (),
                ),
                pickup_sound: b"misc/w_pkup.wav\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                world_model: b"models/weapons/g_rail/tris.md2\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                world_model_flags: 0x1 as libc::c_int,
                view_model: b"models/weapons/v_rail/tris.md2\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                icon: b"w_railgun\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                pickup_name: b"Railgun\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                count_width: 0 as libc::c_int,
                quantity: 1 as libc::c_int,
                ammo: b"Slugs\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                flags: 1 as libc::c_int | 8 as libc::c_int,
                weapmodel: 10 as libc::c_int,
                info: 0 as *const libc::c_void as *mut libc::c_void,
                tag: 0 as libc::c_int,
                precaches: b"weapons/rg_hum.wav\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = gitem_s {
                classname: b"weapon_bfg\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                pickup: Some(
                    Pickup_Weapon
                        as unsafe extern "C" fn(*mut edict_t, *mut edict_t) -> qboolean,
                ),
                use_0: Some(
                    Use_Weapon as unsafe extern "C" fn(*mut edict_t, *mut gitem_t) -> (),
                ),
                drop: Some(
                    Drop_Weapon as unsafe extern "C" fn(*mut edict_t, *mut gitem_t) -> (),
                ),
                weaponthink: Some(
                    Weapon_BFG as unsafe extern "C" fn(*mut edict_t) -> (),
                ),
                pickup_sound: b"misc/w_pkup.wav\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                world_model: b"models/weapons/g_bfg/tris.md2\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                world_model_flags: 0x1 as libc::c_int,
                view_model: b"models/weapons/v_bfg/tris.md2\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                icon: b"w_bfg\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                pickup_name: b"BFG10K\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                count_width: 0 as libc::c_int,
                quantity: 50 as libc::c_int,
                ammo: b"Cells\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                flags: 1 as libc::c_int | 8 as libc::c_int,
                weapmodel: 11 as libc::c_int,
                info: 0 as *const libc::c_void as *mut libc::c_void,
                tag: 0 as libc::c_int,
                precaches: b"sprites/s_bfg1.sp2 sprites/s_bfg2.sp2 sprites/s_bfg3.sp2 weapons/bfg__f1y.wav weapons/bfg__l1a.wav weapons/bfg__x1b.wav weapons/bfg_hum.wav\0"
                    as *const u8 as *const libc::c_char as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = gitem_s {
                classname: b"ammo_shells\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                pickup: Some(
                    Pickup_Ammo
                        as unsafe extern "C" fn(*mut edict_t, *mut edict_t) -> qboolean,
                ),
                use_0: None,
                drop: Some(
                    Drop_Ammo as unsafe extern "C" fn(*mut edict_t, *mut gitem_t) -> (),
                ),
                weaponthink: None,
                pickup_sound: b"misc/am_pkup.wav\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                world_model: b"models/items/ammo/shells/medium/tris.md2\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                world_model_flags: 0 as libc::c_int,
                view_model: 0 as *const libc::c_char as *mut libc::c_char,
                icon: b"a_shells\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                pickup_name: b"Shells\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                count_width: 3 as libc::c_int,
                quantity: 10 as libc::c_int,
                ammo: 0 as *const libc::c_char as *mut libc::c_char,
                flags: 2 as libc::c_int,
                weapmodel: 0 as libc::c_int,
                info: 0 as *const libc::c_void as *mut libc::c_void,
                tag: AMMO_SHELLS as libc::c_int,
                precaches: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = gitem_s {
                classname: b"ammo_bullets\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                pickup: Some(
                    Pickup_Ammo
                        as unsafe extern "C" fn(*mut edict_t, *mut edict_t) -> qboolean,
                ),
                use_0: None,
                drop: Some(
                    Drop_Ammo as unsafe extern "C" fn(*mut edict_t, *mut gitem_t) -> (),
                ),
                weaponthink: None,
                pickup_sound: b"misc/am_pkup.wav\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                world_model: b"models/items/ammo/bullets/medium/tris.md2\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                world_model_flags: 0 as libc::c_int,
                view_model: 0 as *const libc::c_char as *mut libc::c_char,
                icon: b"a_bullets\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                pickup_name: b"Bullets\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                count_width: 3 as libc::c_int,
                quantity: 50 as libc::c_int,
                ammo: 0 as *const libc::c_char as *mut libc::c_char,
                flags: 2 as libc::c_int,
                weapmodel: 0 as libc::c_int,
                info: 0 as *const libc::c_void as *mut libc::c_void,
                tag: AMMO_BULLETS as libc::c_int,
                precaches: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = gitem_s {
                classname: b"ammo_cells\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                pickup: Some(
                    Pickup_Ammo
                        as unsafe extern "C" fn(*mut edict_t, *mut edict_t) -> qboolean,
                ),
                use_0: None,
                drop: Some(
                    Drop_Ammo as unsafe extern "C" fn(*mut edict_t, *mut gitem_t) -> (),
                ),
                weaponthink: None,
                pickup_sound: b"misc/am_pkup.wav\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                world_model: b"models/items/ammo/cells/medium/tris.md2\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                world_model_flags: 0 as libc::c_int,
                view_model: 0 as *const libc::c_char as *mut libc::c_char,
                icon: b"a_cells\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                pickup_name: b"Cells\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                count_width: 3 as libc::c_int,
                quantity: 50 as libc::c_int,
                ammo: 0 as *const libc::c_char as *mut libc::c_char,
                flags: 2 as libc::c_int,
                weapmodel: 0 as libc::c_int,
                info: 0 as *const libc::c_void as *mut libc::c_void,
                tag: AMMO_CELLS as libc::c_int,
                precaches: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = gitem_s {
                classname: b"ammo_rockets\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                pickup: Some(
                    Pickup_Ammo
                        as unsafe extern "C" fn(*mut edict_t, *mut edict_t) -> qboolean,
                ),
                use_0: None,
                drop: Some(
                    Drop_Ammo as unsafe extern "C" fn(*mut edict_t, *mut gitem_t) -> (),
                ),
                weaponthink: None,
                pickup_sound: b"misc/am_pkup.wav\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                world_model: b"models/items/ammo/rockets/medium/tris.md2\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                world_model_flags: 0 as libc::c_int,
                view_model: 0 as *const libc::c_char as *mut libc::c_char,
                icon: b"a_rockets\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                pickup_name: b"Rockets\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                count_width: 3 as libc::c_int,
                quantity: 5 as libc::c_int,
                ammo: 0 as *const libc::c_char as *mut libc::c_char,
                flags: 2 as libc::c_int,
                weapmodel: 0 as libc::c_int,
                info: 0 as *const libc::c_void as *mut libc::c_void,
                tag: AMMO_ROCKETS as libc::c_int,
                precaches: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = gitem_s {
                classname: b"ammo_slugs\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                pickup: Some(
                    Pickup_Ammo
                        as unsafe extern "C" fn(*mut edict_t, *mut edict_t) -> qboolean,
                ),
                use_0: None,
                drop: Some(
                    Drop_Ammo as unsafe extern "C" fn(*mut edict_t, *mut gitem_t) -> (),
                ),
                weaponthink: None,
                pickup_sound: b"misc/am_pkup.wav\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                world_model: b"models/items/ammo/slugs/medium/tris.md2\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                world_model_flags: 0 as libc::c_int,
                view_model: 0 as *const libc::c_char as *mut libc::c_char,
                icon: b"a_slugs\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                pickup_name: b"Slugs\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                count_width: 3 as libc::c_int,
                quantity: 10 as libc::c_int,
                ammo: 0 as *const libc::c_char as *mut libc::c_char,
                flags: 2 as libc::c_int,
                weapmodel: 0 as libc::c_int,
                info: 0 as *const libc::c_void as *mut libc::c_void,
                tag: AMMO_SLUGS as libc::c_int,
                precaches: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = gitem_s {
                classname: b"item_quad\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                pickup: Some(
                    Pickup_Powerup
                        as unsafe extern "C" fn(*mut edict_t, *mut edict_t) -> qboolean,
                ),
                use_0: Some(
                    Use_Quad as unsafe extern "C" fn(*mut edict_t, *mut gitem_t) -> (),
                ),
                drop: Some(
                    Drop_General
                        as unsafe extern "C" fn(*mut edict_t, *mut gitem_t) -> (),
                ),
                weaponthink: None,
                pickup_sound: b"items/pkup.wav\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                world_model: b"models/items/quaddama/tris.md2\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                world_model_flags: 0x1 as libc::c_int,
                view_model: 0 as *const libc::c_char as *mut libc::c_char,
                icon: b"p_quad\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                pickup_name: b"Quad Damage\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                count_width: 2 as libc::c_int,
                quantity: 60 as libc::c_int,
                ammo: 0 as *const libc::c_char as *mut libc::c_char,
                flags: 32 as libc::c_int,
                weapmodel: 0 as libc::c_int,
                info: 0 as *const libc::c_void as *mut libc::c_void,
                tag: 0 as libc::c_int,
                precaches: b"items/damage.wav items/damage2.wav items/damage3.wav\0"
                    as *const u8 as *const libc::c_char as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = gitem_s {
                classname: b"item_invulnerability\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                pickup: Some(
                    Pickup_Powerup
                        as unsafe extern "C" fn(*mut edict_t, *mut edict_t) -> qboolean,
                ),
                use_0: Some(
                    Use_Invulnerability
                        as unsafe extern "C" fn(*mut edict_t, *mut gitem_t) -> (),
                ),
                drop: Some(
                    Drop_General
                        as unsafe extern "C" fn(*mut edict_t, *mut gitem_t) -> (),
                ),
                weaponthink: None,
                pickup_sound: b"items/pkup.wav\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                world_model: b"models/items/invulner/tris.md2\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                world_model_flags: 0x1 as libc::c_int,
                view_model: 0 as *const libc::c_char as *mut libc::c_char,
                icon: b"p_invulnerability\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                pickup_name: b"Invulnerability\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                count_width: 2 as libc::c_int,
                quantity: 300 as libc::c_int,
                ammo: 0 as *const libc::c_char as *mut libc::c_char,
                flags: 32 as libc::c_int,
                weapmodel: 0 as libc::c_int,
                info: 0 as *const libc::c_void as *mut libc::c_void,
                tag: 0 as libc::c_int,
                precaches: b"items/protect.wav items/protect2.wav items/protect4.wav\0"
                    as *const u8 as *const libc::c_char as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = gitem_s {
                classname: b"item_silencer\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                pickup: Some(
                    Pickup_Powerup
                        as unsafe extern "C" fn(*mut edict_t, *mut edict_t) -> qboolean,
                ),
                use_0: Some(
                    Use_Silencer
                        as unsafe extern "C" fn(*mut edict_t, *mut gitem_t) -> (),
                ),
                drop: Some(
                    Drop_General
                        as unsafe extern "C" fn(*mut edict_t, *mut gitem_t) -> (),
                ),
                weaponthink: None,
                pickup_sound: b"items/pkup.wav\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                world_model: b"models/items/silencer/tris.md2\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                world_model_flags: 0x1 as libc::c_int,
                view_model: 0 as *const libc::c_char as *mut libc::c_char,
                icon: b"p_silencer\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                pickup_name: b"Silencer\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                count_width: 2 as libc::c_int,
                quantity: 60 as libc::c_int,
                ammo: 0 as *const libc::c_char as *mut libc::c_char,
                flags: 32 as libc::c_int,
                weapmodel: 0 as libc::c_int,
                info: 0 as *const libc::c_void as *mut libc::c_void,
                tag: 0 as libc::c_int,
                precaches: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = gitem_s {
                classname: b"item_breather\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                pickup: Some(
                    Pickup_Powerup
                        as unsafe extern "C" fn(*mut edict_t, *mut edict_t) -> qboolean,
                ),
                use_0: Some(
                    Use_Breather
                        as unsafe extern "C" fn(*mut edict_t, *mut gitem_t) -> (),
                ),
                drop: Some(
                    Drop_General
                        as unsafe extern "C" fn(*mut edict_t, *mut gitem_t) -> (),
                ),
                weaponthink: None,
                pickup_sound: b"items/pkup.wav\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                world_model: b"models/items/breather/tris.md2\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                world_model_flags: 0x1 as libc::c_int,
                view_model: 0 as *const libc::c_char as *mut libc::c_char,
                icon: b"p_rebreather\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                pickup_name: b"Rebreather\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                count_width: 2 as libc::c_int,
                quantity: 60 as libc::c_int,
                ammo: 0 as *const libc::c_char as *mut libc::c_char,
                flags: 8 as libc::c_int | 32 as libc::c_int,
                weapmodel: 0 as libc::c_int,
                info: 0 as *const libc::c_void as *mut libc::c_void,
                tag: 0 as libc::c_int,
                precaches: b"items/airout.wav\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = gitem_s {
                classname: b"item_enviro\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                pickup: Some(
                    Pickup_Powerup
                        as unsafe extern "C" fn(*mut edict_t, *mut edict_t) -> qboolean,
                ),
                use_0: Some(
                    Use_Envirosuit
                        as unsafe extern "C" fn(*mut edict_t, *mut gitem_t) -> (),
                ),
                drop: Some(
                    Drop_General
                        as unsafe extern "C" fn(*mut edict_t, *mut gitem_t) -> (),
                ),
                weaponthink: None,
                pickup_sound: b"items/pkup.wav\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                world_model: b"models/items/enviro/tris.md2\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                world_model_flags: 0x1 as libc::c_int,
                view_model: 0 as *const libc::c_char as *mut libc::c_char,
                icon: b"p_envirosuit\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                pickup_name: b"Environment Suit\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                count_width: 2 as libc::c_int,
                quantity: 60 as libc::c_int,
                ammo: 0 as *const libc::c_char as *mut libc::c_char,
                flags: 8 as libc::c_int | 32 as libc::c_int,
                weapmodel: 0 as libc::c_int,
                info: 0 as *const libc::c_void as *mut libc::c_void,
                tag: 0 as libc::c_int,
                precaches: b"items/airout.wav\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = gitem_s {
                classname: b"item_ancient_head\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                pickup: Some(
                    Pickup_AncientHead
                        as unsafe extern "C" fn(*mut edict_t, *mut edict_t) -> qboolean,
                ),
                use_0: None,
                drop: None,
                weaponthink: None,
                pickup_sound: b"items/pkup.wav\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                world_model: b"models/items/c_head/tris.md2\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                world_model_flags: 0x1 as libc::c_int,
                view_model: 0 as *const libc::c_char as *mut libc::c_char,
                icon: b"i_fixme\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                pickup_name: b"Ancient Head\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                count_width: 2 as libc::c_int,
                quantity: 60 as libc::c_int,
                ammo: 0 as *const libc::c_char as *mut libc::c_char,
                flags: 0 as libc::c_int,
                weapmodel: 0 as libc::c_int,
                info: 0 as *const libc::c_void as *mut libc::c_void,
                tag: 0 as libc::c_int,
                precaches: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = gitem_s {
                classname: b"item_adrenaline\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                pickup: Some(
                    Pickup_Adrenaline
                        as unsafe extern "C" fn(*mut edict_t, *mut edict_t) -> qboolean,
                ),
                use_0: None,
                drop: None,
                weaponthink: None,
                pickup_sound: b"items/pkup.wav\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                world_model: b"models/items/adrenal/tris.md2\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                world_model_flags: 0x1 as libc::c_int,
                view_model: 0 as *const libc::c_char as *mut libc::c_char,
                icon: b"p_adrenaline\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                pickup_name: b"Adrenaline\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                count_width: 2 as libc::c_int,
                quantity: 60 as libc::c_int,
                ammo: 0 as *const libc::c_char as *mut libc::c_char,
                flags: 0 as libc::c_int,
                weapmodel: 0 as libc::c_int,
                info: 0 as *const libc::c_void as *mut libc::c_void,
                tag: 0 as libc::c_int,
                precaches: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = gitem_s {
                classname: b"item_bandolier\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                pickup: Some(
                    Pickup_Bandolier
                        as unsafe extern "C" fn(*mut edict_t, *mut edict_t) -> qboolean,
                ),
                use_0: None,
                drop: None,
                weaponthink: None,
                pickup_sound: b"items/pkup.wav\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                world_model: b"models/items/band/tris.md2\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                world_model_flags: 0x1 as libc::c_int,
                view_model: 0 as *const libc::c_char as *mut libc::c_char,
                icon: b"p_bandolier\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                pickup_name: b"Bandolier\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                count_width: 2 as libc::c_int,
                quantity: 60 as libc::c_int,
                ammo: 0 as *const libc::c_char as *mut libc::c_char,
                flags: 0 as libc::c_int,
                weapmodel: 0 as libc::c_int,
                info: 0 as *const libc::c_void as *mut libc::c_void,
                tag: 0 as libc::c_int,
                precaches: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = gitem_s {
                classname: b"item_pack\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                pickup: Some(
                    Pickup_Pack
                        as unsafe extern "C" fn(*mut edict_t, *mut edict_t) -> qboolean,
                ),
                use_0: None,
                drop: None,
                weaponthink: None,
                pickup_sound: b"items/pkup.wav\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                world_model: b"models/items/pack/tris.md2\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                world_model_flags: 0x1 as libc::c_int,
                view_model: 0 as *const libc::c_char as *mut libc::c_char,
                icon: b"i_pack\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                pickup_name: b"Ammo Pack\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                count_width: 2 as libc::c_int,
                quantity: 180 as libc::c_int,
                ammo: 0 as *const libc::c_char as *mut libc::c_char,
                flags: 0 as libc::c_int,
                weapmodel: 0 as libc::c_int,
                info: 0 as *const libc::c_void as *mut libc::c_void,
                tag: 0 as libc::c_int,
                precaches: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = gitem_s {
                classname: b"key_data_cd\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                pickup: Some(
                    Pickup_Key
                        as unsafe extern "C" fn(*mut edict_t, *mut edict_t) -> qboolean,
                ),
                use_0: None,
                drop: Some(
                    Drop_General
                        as unsafe extern "C" fn(*mut edict_t, *mut gitem_t) -> (),
                ),
                weaponthink: None,
                pickup_sound: b"items/pkup.wav\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                world_model: b"models/items/keys/data_cd/tris.md2\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                world_model_flags: 0x1 as libc::c_int,
                view_model: 0 as *const libc::c_char as *mut libc::c_char,
                icon: b"k_datacd\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                pickup_name: b"Data CD\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                count_width: 2 as libc::c_int,
                quantity: 0 as libc::c_int,
                ammo: 0 as *const libc::c_char as *mut libc::c_char,
                flags: 8 as libc::c_int | 16 as libc::c_int,
                weapmodel: 0 as libc::c_int,
                info: 0 as *const libc::c_void as *mut libc::c_void,
                tag: 0 as libc::c_int,
                precaches: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = gitem_s {
                classname: b"key_power_cube\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                pickup: Some(
                    Pickup_Key
                        as unsafe extern "C" fn(*mut edict_t, *mut edict_t) -> qboolean,
                ),
                use_0: None,
                drop: Some(
                    Drop_General
                        as unsafe extern "C" fn(*mut edict_t, *mut gitem_t) -> (),
                ),
                weaponthink: None,
                pickup_sound: b"items/pkup.wav\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                world_model: b"models/items/keys/power/tris.md2\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                world_model_flags: 0x1 as libc::c_int,
                view_model: 0 as *const libc::c_char as *mut libc::c_char,
                icon: b"k_powercube\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                pickup_name: b"Power Cube\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                count_width: 2 as libc::c_int,
                quantity: 0 as libc::c_int,
                ammo: 0 as *const libc::c_char as *mut libc::c_char,
                flags: 8 as libc::c_int | 16 as libc::c_int,
                weapmodel: 0 as libc::c_int,
                info: 0 as *const libc::c_void as *mut libc::c_void,
                tag: 0 as libc::c_int,
                precaches: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = gitem_s {
                classname: b"key_pyramid\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                pickup: Some(
                    Pickup_Key
                        as unsafe extern "C" fn(*mut edict_t, *mut edict_t) -> qboolean,
                ),
                use_0: None,
                drop: Some(
                    Drop_General
                        as unsafe extern "C" fn(*mut edict_t, *mut gitem_t) -> (),
                ),
                weaponthink: None,
                pickup_sound: b"items/pkup.wav\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                world_model: b"models/items/keys/pyramid/tris.md2\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                world_model_flags: 0x1 as libc::c_int,
                view_model: 0 as *const libc::c_char as *mut libc::c_char,
                icon: b"k_pyramid\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                pickup_name: b"Pyramid Key\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                count_width: 2 as libc::c_int,
                quantity: 0 as libc::c_int,
                ammo: 0 as *const libc::c_char as *mut libc::c_char,
                flags: 8 as libc::c_int | 16 as libc::c_int,
                weapmodel: 0 as libc::c_int,
                info: 0 as *const libc::c_void as *mut libc::c_void,
                tag: 0 as libc::c_int,
                precaches: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = gitem_s {
                classname: b"key_data_spinner\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                pickup: Some(
                    Pickup_Key
                        as unsafe extern "C" fn(*mut edict_t, *mut edict_t) -> qboolean,
                ),
                use_0: None,
                drop: Some(
                    Drop_General
                        as unsafe extern "C" fn(*mut edict_t, *mut gitem_t) -> (),
                ),
                weaponthink: None,
                pickup_sound: b"items/pkup.wav\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                world_model: b"models/items/keys/spinner/tris.md2\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                world_model_flags: 0x1 as libc::c_int,
                view_model: 0 as *const libc::c_char as *mut libc::c_char,
                icon: b"k_dataspin\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                pickup_name: b"Data Spinner\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                count_width: 2 as libc::c_int,
                quantity: 0 as libc::c_int,
                ammo: 0 as *const libc::c_char as *mut libc::c_char,
                flags: 8 as libc::c_int | 16 as libc::c_int,
                weapmodel: 0 as libc::c_int,
                info: 0 as *const libc::c_void as *mut libc::c_void,
                tag: 0 as libc::c_int,
                precaches: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = gitem_s {
                classname: b"key_pass\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                pickup: Some(
                    Pickup_Key
                        as unsafe extern "C" fn(*mut edict_t, *mut edict_t) -> qboolean,
                ),
                use_0: None,
                drop: Some(
                    Drop_General
                        as unsafe extern "C" fn(*mut edict_t, *mut gitem_t) -> (),
                ),
                weaponthink: None,
                pickup_sound: b"items/pkup.wav\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                world_model: b"models/items/keys/pass/tris.md2\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                world_model_flags: 0x1 as libc::c_int,
                view_model: 0 as *const libc::c_char as *mut libc::c_char,
                icon: b"k_security\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                pickup_name: b"Security Pass\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                count_width: 2 as libc::c_int,
                quantity: 0 as libc::c_int,
                ammo: 0 as *const libc::c_char as *mut libc::c_char,
                flags: 8 as libc::c_int | 16 as libc::c_int,
                weapmodel: 0 as libc::c_int,
                info: 0 as *const libc::c_void as *mut libc::c_void,
                tag: 0 as libc::c_int,
                precaches: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = gitem_s {
                classname: b"key_blue_key\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                pickup: Some(
                    Pickup_Key
                        as unsafe extern "C" fn(*mut edict_t, *mut edict_t) -> qboolean,
                ),
                use_0: None,
                drop: Some(
                    Drop_General
                        as unsafe extern "C" fn(*mut edict_t, *mut gitem_t) -> (),
                ),
                weaponthink: None,
                pickup_sound: b"items/pkup.wav\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                world_model: b"models/items/keys/key/tris.md2\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                world_model_flags: 0x1 as libc::c_int,
                view_model: 0 as *const libc::c_char as *mut libc::c_char,
                icon: b"k_bluekey\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                pickup_name: b"Blue Key\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                count_width: 2 as libc::c_int,
                quantity: 0 as libc::c_int,
                ammo: 0 as *const libc::c_char as *mut libc::c_char,
                flags: 8 as libc::c_int | 16 as libc::c_int,
                weapmodel: 0 as libc::c_int,
                info: 0 as *const libc::c_void as *mut libc::c_void,
                tag: 0 as libc::c_int,
                precaches: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = gitem_s {
                classname: b"key_red_key\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                pickup: Some(
                    Pickup_Key
                        as unsafe extern "C" fn(*mut edict_t, *mut edict_t) -> qboolean,
                ),
                use_0: None,
                drop: Some(
                    Drop_General
                        as unsafe extern "C" fn(*mut edict_t, *mut gitem_t) -> (),
                ),
                weaponthink: None,
                pickup_sound: b"items/pkup.wav\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                world_model: b"models/items/keys/red_key/tris.md2\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                world_model_flags: 0x1 as libc::c_int,
                view_model: 0 as *const libc::c_char as *mut libc::c_char,
                icon: b"k_redkey\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                pickup_name: b"Red Key\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                count_width: 2 as libc::c_int,
                quantity: 0 as libc::c_int,
                ammo: 0 as *const libc::c_char as *mut libc::c_char,
                flags: 8 as libc::c_int | 16 as libc::c_int,
                weapmodel: 0 as libc::c_int,
                info: 0 as *const libc::c_void as *mut libc::c_void,
                tag: 0 as libc::c_int,
                precaches: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = gitem_s {
                classname: b"key_commander_head\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                pickup: Some(
                    Pickup_Key
                        as unsafe extern "C" fn(*mut edict_t, *mut edict_t) -> qboolean,
                ),
                use_0: None,
                drop: Some(
                    Drop_General
                        as unsafe extern "C" fn(*mut edict_t, *mut gitem_t) -> (),
                ),
                weaponthink: None,
                pickup_sound: b"items/pkup.wav\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                world_model: b"models/monsters/commandr/head/tris.md2\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                world_model_flags: 0x2 as libc::c_int,
                view_model: 0 as *const libc::c_char as *mut libc::c_char,
                icon: b"k_comhead\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                pickup_name: b"Commander's Head\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                count_width: 2 as libc::c_int,
                quantity: 0 as libc::c_int,
                ammo: 0 as *const libc::c_char as *mut libc::c_char,
                flags: 8 as libc::c_int | 16 as libc::c_int,
                weapmodel: 0 as libc::c_int,
                info: 0 as *const libc::c_void as *mut libc::c_void,
                tag: 0 as libc::c_int,
                precaches: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = gitem_s {
                classname: b"key_airstrike_target\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                pickup: Some(
                    Pickup_Key
                        as unsafe extern "C" fn(*mut edict_t, *mut edict_t) -> qboolean,
                ),
                use_0: None,
                drop: Some(
                    Drop_General
                        as unsafe extern "C" fn(*mut edict_t, *mut gitem_t) -> (),
                ),
                weaponthink: None,
                pickup_sound: b"items/pkup.wav\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                world_model: b"models/items/keys/target/tris.md2\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                world_model_flags: 0x1 as libc::c_int,
                view_model: 0 as *const libc::c_char as *mut libc::c_char,
                icon: b"i_airstrike\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                pickup_name: b"Airstrike Marker\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                count_width: 2 as libc::c_int,
                quantity: 0 as libc::c_int,
                ammo: 0 as *const libc::c_char as *mut libc::c_char,
                flags: 8 as libc::c_int | 16 as libc::c_int,
                weapmodel: 0 as libc::c_int,
                info: 0 as *const libc::c_void as *mut libc::c_void,
                tag: 0 as libc::c_int,
                precaches: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = gitem_s {
                classname: 0 as *const libc::c_char as *mut libc::c_char,
                pickup: Some(
                    Pickup_Health
                        as unsafe extern "C" fn(*mut edict_t, *mut edict_t) -> qboolean,
                ),
                use_0: None,
                drop: None,
                weaponthink: None,
                pickup_sound: b"items/pkup.wav\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                world_model: 0 as *const libc::c_char as *mut libc::c_char,
                world_model_flags: 0 as libc::c_int,
                view_model: 0 as *const libc::c_char as *mut libc::c_char,
                icon: b"i_health\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                pickup_name: b"Health\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                count_width: 3 as libc::c_int,
                quantity: 0 as libc::c_int,
                ammo: 0 as *const libc::c_char as *mut libc::c_char,
                flags: 0 as libc::c_int,
                weapmodel: 0 as libc::c_int,
                info: 0 as *const libc::c_void as *mut libc::c_void,
                tag: 0 as libc::c_int,
                precaches: b"items/s_health.wav items/n_health.wav items/l_health.wav items/m_health.wav\0"
                    as *const u8 as *const libc::c_char as *mut libc::c_char,
            };
            init
        },
        {
            let mut init = gitem_s {
                classname: 0 as *const libc::c_char as *mut libc::c_char,
                pickup: None,
                use_0: None,
                drop: None,
                weaponthink: None,
                pickup_sound: 0 as *const libc::c_char as *mut libc::c_char,
                world_model: 0 as *const libc::c_char as *mut libc::c_char,
                world_model_flags: 0,
                view_model: 0 as *const libc::c_char as *mut libc::c_char,
                icon: 0 as *const libc::c_char as *mut libc::c_char,
                pickup_name: 0 as *const libc::c_char as *mut libc::c_char,
                count_width: 0,
                quantity: 0,
                ammo: 0 as *const libc::c_char as *mut libc::c_char,
                flags: 0,
                weapmodel: 0,
                info: 0 as *const libc::c_void as *mut libc::c_void,
                tag: 0,
                precaches: 0 as *const libc::c_char as *mut libc::c_char,
            };
            init
        },
    ]
};
#[no_mangle]
pub unsafe extern "C" fn SP_item_health(mut self_0: *mut edict_t) {
    if (*deathmatch).value != 0.
        && (*dmflags).value as libc::c_int & 0x1 as libc::c_int != 0
    {
        G_FreeEdict(self_0);
        return;
    }
    let ref mut fresh37 = (*self_0).model;
    *fresh37 = b"models/items/healing/medium/tris.md2\0" as *const u8
        as *const libc::c_char as *mut libc::c_char;
    (*self_0).count = 10 as libc::c_int;
    SpawnItem(
        self_0,
        FindItem(b"Health\0" as *const u8 as *const libc::c_char as *mut libc::c_char),
    );
    (gi.soundindex)
        .expect(
            "non-null function pointer",
        )(
        b"items/n_health.wav\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
}
#[no_mangle]
pub unsafe extern "C" fn SP_item_health_small(mut self_0: *mut edict_t) {
    if (*deathmatch).value != 0.
        && (*dmflags).value as libc::c_int & 0x1 as libc::c_int != 0
    {
        G_FreeEdict(self_0);
        return;
    }
    let ref mut fresh38 = (*self_0).model;
    *fresh38 = b"models/items/healing/stimpack/tris.md2\0" as *const u8
        as *const libc::c_char as *mut libc::c_char;
    (*self_0).count = 2 as libc::c_int;
    SpawnItem(
        self_0,
        FindItem(b"Health\0" as *const u8 as *const libc::c_char as *mut libc::c_char),
    );
    (*self_0).style = 1 as libc::c_int;
    (gi.soundindex)
        .expect(
            "non-null function pointer",
        )(
        b"items/s_health.wav\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
}
#[no_mangle]
pub unsafe extern "C" fn SP_item_health_large(mut self_0: *mut edict_t) {
    if (*deathmatch).value != 0.
        && (*dmflags).value as libc::c_int & 0x1 as libc::c_int != 0
    {
        G_FreeEdict(self_0);
        return;
    }
    let ref mut fresh39 = (*self_0).model;
    *fresh39 = b"models/items/healing/large/tris.md2\0" as *const u8
        as *const libc::c_char as *mut libc::c_char;
    (*self_0).count = 25 as libc::c_int;
    SpawnItem(
        self_0,
        FindItem(b"Health\0" as *const u8 as *const libc::c_char as *mut libc::c_char),
    );
    (gi.soundindex)
        .expect(
            "non-null function pointer",
        )(
        b"items/l_health.wav\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
}
#[no_mangle]
pub unsafe extern "C" fn SP_item_health_mega(mut self_0: *mut edict_t) {
    if (*deathmatch).value != 0.
        && (*dmflags).value as libc::c_int & 0x1 as libc::c_int != 0
    {
        G_FreeEdict(self_0);
        return;
    }
    let ref mut fresh40 = (*self_0).model;
    *fresh40 = b"models/items/mega_h/tris.md2\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char;
    (*self_0).count = 100 as libc::c_int;
    SpawnItem(
        self_0,
        FindItem(b"Health\0" as *const u8 as *const libc::c_char as *mut libc::c_char),
    );
    (gi.soundindex)
        .expect(
            "non-null function pointer",
        )(
        b"items/m_health.wav\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    (*self_0).style = 1 as libc::c_int | 2 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn InitItems() {
    game
        .num_items = (::std::mem::size_of::<[gitem_t; 43]>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<gitem_t>() as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn SetItemNames() {
    let mut i: libc::c_int = 0;
    let mut it: *mut gitem_t = 0 as *mut gitem_t;
    i = 0 as libc::c_int;
    while i < game.num_items {
        it = &mut *itemlist.as_mut_ptr().offset(i as isize) as *mut gitem_t;
        (gi.configstring)
            .expect(
                "non-null function pointer",
            )(
            32 as libc::c_int + 256 as libc::c_int + 256 as libc::c_int
                + 256 as libc::c_int + 256 as libc::c_int + i,
            (*it).pickup_name,
        );
        i += 1;
    }
    jacket_armor_index = (FindItem(
        b"Jacket Armor\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ))
        .offset_from(itemlist.as_mut_ptr()) as libc::c_long as libc::c_int;
    combat_armor_index = (FindItem(
        b"Combat Armor\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ))
        .offset_from(itemlist.as_mut_ptr()) as libc::c_long as libc::c_int;
    body_armor_index = (FindItem(
        b"Body Armor\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ))
        .offset_from(itemlist.as_mut_ptr()) as libc::c_long as libc::c_int;
    power_screen_index = (FindItem(
        b"Power Screen\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ))
        .offset_from(itemlist.as_mut_ptr()) as libc::c_long as libc::c_int;
    power_shield_index = (FindItem(
        b"Power Shield\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ))
        .offset_from(itemlist.as_mut_ptr()) as libc::c_long as libc::c_int;
}
