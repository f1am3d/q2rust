#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    fn respawn(ent: *mut edict_t);
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn rand() -> libc::c_int;
    fn Com_sprintf(
        dest: *mut libc::c_char,
        size: libc::c_int,
        fmt: *mut libc::c_char,
        _: ...
    );
    static mut game: game_locals_t;
    static mut level: level_locals_t;
    static mut gi: game_import_t;
    static mut g_edicts: *mut edict_t;
    static mut deathmatch: *mut cvar_t;
    static mut coop: *mut cvar_t;
    static mut skill: *mut cvar_t;
    static mut maxclients: *mut cvar_t;
    static mut itemlist: [gitem_t; 0];
    fn FindItem(pickup_name: *mut libc::c_char) -> *mut gitem_t;
    fn ArmorIndex(ent: *mut edict_t) -> libc::c_int;
    fn PowerArmorType(ent: *mut edict_t) -> libc::c_int;
    fn GetItemByIndex(index: libc::c_int) -> *mut gitem_t;
    fn G_Find(
        from: *mut edict_t,
        fieldofs: libc::c_int,
        match_0: *mut libc::c_char,
    ) -> *mut edict_t;
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
pub unsafe extern "C" fn MoveClientToIntermission(mut ent: *mut edict_t) {
    if (*deathmatch).value != 0. || (*coop).value != 0. {
        (*(*ent).client).showscores = true_0;
    }
    (*ent)
        .s
        .origin[0 as libc::c_int
        as usize] = level.intermission_origin[0 as libc::c_int as usize];
    (*ent)
        .s
        .origin[1 as libc::c_int
        as usize] = level.intermission_origin[1 as libc::c_int as usize];
    (*ent)
        .s
        .origin[2 as libc::c_int
        as usize] = level.intermission_origin[2 as libc::c_int as usize];
    (*(*ent).client)
        .ps
        .pmove
        .origin[0 as libc::c_int
        as usize] = (level.intermission_origin[0 as libc::c_int as usize]
        * 8 as libc::c_int as libc::c_float) as libc::c_short;
    (*(*ent).client)
        .ps
        .pmove
        .origin[1 as libc::c_int
        as usize] = (level.intermission_origin[1 as libc::c_int as usize]
        * 8 as libc::c_int as libc::c_float) as libc::c_short;
    (*(*ent).client)
        .ps
        .pmove
        .origin[2 as libc::c_int
        as usize] = (level.intermission_origin[2 as libc::c_int as usize]
        * 8 as libc::c_int as libc::c_float) as libc::c_short;
    (*(*ent).client)
        .ps
        .viewangles[0 as libc::c_int
        as usize] = level.intermission_angle[0 as libc::c_int as usize];
    (*(*ent).client)
        .ps
        .viewangles[1 as libc::c_int
        as usize] = level.intermission_angle[1 as libc::c_int as usize];
    (*(*ent).client)
        .ps
        .viewangles[2 as libc::c_int
        as usize] = level.intermission_angle[2 as libc::c_int as usize];
    (*(*ent).client).ps.pmove.pm_type = PM_FREEZE;
    (*(*ent).client).ps.gunindex = 0 as libc::c_int;
    (*(*ent).client)
        .ps
        .blend[3 as libc::c_int as usize] = 0 as libc::c_int as libc::c_float;
    (*(*ent).client).ps.rdflags &= !(1 as libc::c_int);
    (*(*ent).client).quad_framenum = 0 as libc::c_int as libc::c_float;
    (*(*ent).client).invincible_framenum = 0 as libc::c_int as libc::c_float;
    (*(*ent).client).breather_framenum = 0 as libc::c_int as libc::c_float;
    (*(*ent).client).enviro_framenum = 0 as libc::c_int as libc::c_float;
    (*(*ent).client).grenade_blew_up = false_0;
    (*(*ent).client).grenade_time = 0 as libc::c_int as libc::c_float;
    (*ent).viewheight = 0 as libc::c_int;
    (*ent).s.modelindex = 0 as libc::c_int;
    (*ent).s.modelindex2 = 0 as libc::c_int;
    (*ent).s.modelindex3 = 0 as libc::c_int;
    (*ent).s.modelindex = 0 as libc::c_int;
    (*ent).s.effects = 0 as libc::c_int as libc::c_uint;
    (*ent).s.sound = 0 as libc::c_int;
    (*ent).solid = SOLID_NOT;
    if (*deathmatch).value != 0. || (*coop).value != 0. {
        DeathmatchScoreboardMessage(ent, 0 as *mut edict_t);
        (gi.unicast).expect("non-null function pointer")(ent, true_0);
    }
}
#[no_mangle]
pub unsafe extern "C" fn BeginIntermission(mut targ: *mut edict_t) {
    let mut i: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut ent: *mut edict_t = 0 as *mut edict_t;
    let mut client: *mut edict_t = 0 as *mut edict_t;
    if level.intermissiontime != 0. {
        return;
    }
    game.autosaved = false_0;
    i = 0 as libc::c_int;
    while (i as libc::c_float) < (*maxclients).value {
        client = g_edicts.offset(1 as libc::c_int as isize).offset(i as isize);
        if !((*client).inuse as u64 == 0) {
            if (*client).health <= 0 as libc::c_int {
                respawn(client);
            }
        }
        i += 1;
    }
    level.intermissiontime = level.time;
    level.changemap = (*targ).map;
    if !(strstr(level.changemap, b"*\0" as *const u8 as *const libc::c_char)).is_null() {
        if (*coop).value != 0. {
            i = 0 as libc::c_int;
            while (i as libc::c_float) < (*maxclients).value {
                client = g_edicts.offset(1 as libc::c_int as isize).offset(i as isize);
                if !((*client).inuse as u64 == 0) {
                    n = 0 as libc::c_int;
                    while n < 256 as libc::c_int {
                        if (*itemlist.as_mut_ptr().offset(n as isize)).flags
                            & 16 as libc::c_int != 0
                        {
                            (*(*client).client)
                                .pers
                                .inventory[n as usize] = 0 as libc::c_int;
                        }
                        n += 1;
                    }
                }
                i += 1;
            }
        }
    } else if (*deathmatch).value == 0. {
        level.exitintermission = 1 as libc::c_int;
        return;
    }
    level.exitintermission = 0 as libc::c_int;
    ent = G_Find(
        0 as *mut edict_t,
        &mut (*(0 as *mut edict_t)).classname as *mut *mut libc::c_char as libc::c_int,
        b"info_player_intermission\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    if ent.is_null() {
        ent = G_Find(
            0 as *mut edict_t,
            &mut (*(0 as *mut edict_t)).classname as *mut *mut libc::c_char
                as libc::c_int,
            b"info_player_start\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        if ent.is_null() {
            ent = G_Find(
                0 as *mut edict_t,
                &mut (*(0 as *mut edict_t)).classname as *mut *mut libc::c_char
                    as libc::c_int,
                b"info_player_deathmatch\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
        }
    } else {
        i = rand() & 3 as libc::c_int;
        loop {
            let fresh0 = i;
            i = i - 1;
            if !(fresh0 != 0) {
                break;
            }
            ent = G_Find(
                ent,
                &mut (*(0 as *mut edict_t)).classname as *mut *mut libc::c_char
                    as libc::c_int,
                b"info_player_intermission\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
            if ent.is_null() {
                ent = G_Find(
                    ent,
                    &mut (*(0 as *mut edict_t)).classname as *mut *mut libc::c_char
                        as libc::c_int,
                    b"info_player_intermission\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                );
            }
        }
    }
    level
        .intermission_origin[0 as libc::c_int
        as usize] = (*ent).s.origin[0 as libc::c_int as usize];
    level
        .intermission_origin[1 as libc::c_int
        as usize] = (*ent).s.origin[1 as libc::c_int as usize];
    level
        .intermission_origin[2 as libc::c_int
        as usize] = (*ent).s.origin[2 as libc::c_int as usize];
    level
        .intermission_angle[0 as libc::c_int
        as usize] = (*ent).s.angles[0 as libc::c_int as usize];
    level
        .intermission_angle[1 as libc::c_int
        as usize] = (*ent).s.angles[1 as libc::c_int as usize];
    level
        .intermission_angle[2 as libc::c_int
        as usize] = (*ent).s.angles[2 as libc::c_int as usize];
    i = 0 as libc::c_int;
    while (i as libc::c_float) < (*maxclients).value {
        client = g_edicts.offset(1 as libc::c_int as isize).offset(i as isize);
        if !((*client).inuse as u64 == 0) {
            MoveClientToIntermission(client);
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn DeathmatchScoreboardMessage(
    mut ent: *mut edict_t,
    mut killer: *mut edict_t,
) {
    let mut entry: [libc::c_char; 1024] = [0; 1024];
    let mut string: [libc::c_char; 1400] = [0; 1400];
    let mut stringlength: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut sorted: [libc::c_int; 256] = [0; 256];
    let mut sortedscores: [libc::c_int; 256] = [0; 256];
    let mut score: libc::c_int = 0;
    let mut total: libc::c_int = 0;
    let mut picnum: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut cl: *mut gclient_t = 0 as *mut gclient_t;
    let mut cl_ent: *mut edict_t = 0 as *mut edict_t;
    let mut tag: *mut libc::c_char = 0 as *mut libc::c_char;
    total = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < game.maxclients {
        cl_ent = g_edicts.offset(1 as libc::c_int as isize).offset(i as isize);
        if !((*cl_ent).inuse as u64 == 0
            || (*(game.clients).offset(i as isize)).resp.spectator as libc::c_uint != 0)
        {
            score = (*(game.clients).offset(i as isize)).resp.score;
            j = 0 as libc::c_int;
            while j < total {
                if score > sortedscores[j as usize] {
                    break;
                }
                j += 1;
            }
            k = total;
            while k > j {
                sorted[k as usize] = sorted[(k - 1 as libc::c_int) as usize];
                sortedscores[k as usize] = sortedscores[(k - 1 as libc::c_int) as usize];
                k -= 1;
            }
            sorted[j as usize] = i;
            sortedscores[j as usize] = score;
            total += 1;
        }
        i += 1;
    }
    string[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    stringlength = strlen(string.as_mut_ptr()) as libc::c_int;
    if total > 12 as libc::c_int {
        total = 12 as libc::c_int;
    }
    i = 0 as libc::c_int;
    while i < total {
        cl = &mut *(game.clients)
            .offset(*sorted.as_mut_ptr().offset(i as isize) as isize) as *mut gclient_t;
        cl_ent = g_edicts
            .offset(1 as libc::c_int as isize)
            .offset(sorted[i as usize] as isize);
        picnum = (gi.imageindex)
            .expect(
                "non-null function pointer",
            )(b"i_fixme\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
        x = if i >= 6 as libc::c_int { 160 as libc::c_int } else { 0 as libc::c_int };
        y = 32 as libc::c_int + 32 as libc::c_int * (i % 6 as libc::c_int);
        if cl_ent == ent {
            tag = b"tag1\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        } else if cl_ent == killer {
            tag = b"tag2\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        } else {
            tag = 0 as *mut libc::c_char;
        }
        if !tag.is_null() {
            Com_sprintf(
                entry.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong
                    as libc::c_int,
                b"xv %i yv %i picn %s \0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                x + 32 as libc::c_int,
                y,
                tag,
            );
            j = strlen(entry.as_mut_ptr()) as libc::c_int;
            if stringlength + j > 1024 as libc::c_int {
                break;
            }
            strcpy(
                string.as_mut_ptr().offset(stringlength as isize),
                entry.as_mut_ptr(),
            );
            stringlength += j;
        }
        Com_sprintf(
            entry.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong
                as libc::c_int,
            b"client %i %i %i %i %i %i \0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            x,
            y,
            sorted[i as usize],
            (*cl).resp.score,
            (*cl).ping,
            (level.framenum - (*cl).resp.enterframe) / 600 as libc::c_int,
        );
        j = strlen(entry.as_mut_ptr()) as libc::c_int;
        if stringlength + j > 1024 as libc::c_int {
            break;
        }
        strcpy(string.as_mut_ptr().offset(stringlength as isize), entry.as_mut_ptr());
        stringlength += j;
        i += 1;
    }
    (gi.WriteByte).expect("non-null function pointer")(4 as libc::c_int);
    (gi.WriteString).expect("non-null function pointer")(string.as_mut_ptr());
}
#[no_mangle]
pub unsafe extern "C" fn DeathmatchScoreboard(mut ent: *mut edict_t) {
    DeathmatchScoreboardMessage(ent, (*ent).enemy);
    (gi.unicast).expect("non-null function pointer")(ent, true_0);
}
#[no_mangle]
pub unsafe extern "C" fn Cmd_Score_f(mut ent: *mut edict_t) {
    (*(*ent).client).showinventory = false_0;
    (*(*ent).client).showhelp = false_0;
    if (*deathmatch).value == 0. && (*coop).value == 0. {
        return;
    }
    if (*(*ent).client).showscores as u64 != 0 {
        (*(*ent).client).showscores = false_0;
        return;
    }
    (*(*ent).client).showscores = true_0;
    DeathmatchScoreboard(ent);
}
#[no_mangle]
pub unsafe extern "C" fn HelpComputer(mut ent: *mut edict_t) {
    let mut string: [libc::c_char; 1024] = [0; 1024];
    let mut sk: *mut libc::c_char = 0 as *mut libc::c_char;
    if (*skill).value == 0 as libc::c_int as libc::c_float {
        sk = b"easy\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    } else if (*skill).value == 1 as libc::c_int as libc::c_float {
        sk = b"medium\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    } else if (*skill).value == 2 as libc::c_int as libc::c_float {
        sk = b"hard\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    } else {
        sk = b"hard+\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    Com_sprintf(
        string.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
        b"xv 32 yv 8 picn help xv 202 yv 12 string2 \"%s\" xv 0 yv 24 cstring2 \"%s\" xv 0 yv 54 cstring2 \"%s\" xv 0 yv 110 cstring2 \"%s\" xv 50 yv 164 string2 \" kills     goals    secrets\" xv 50 yv 172 string2 \"%3i/%3i     %i/%i       %i/%i\" \0"
            as *const u8 as *const libc::c_char as *mut libc::c_char,
        sk,
        (level.level_name).as_mut_ptr(),
        (game.helpmessage1).as_mut_ptr(),
        (game.helpmessage2).as_mut_ptr(),
        level.killed_monsters,
        level.total_monsters,
        level.found_goals,
        level.total_goals,
        level.found_secrets,
        level.total_secrets,
    );
    (gi.WriteByte).expect("non-null function pointer")(4 as libc::c_int);
    (gi.WriteString).expect("non-null function pointer")(string.as_mut_ptr());
    (gi.unicast).expect("non-null function pointer")(ent, true_0);
}
#[no_mangle]
pub unsafe extern "C" fn Cmd_Help_f(mut ent: *mut edict_t) {
    if (*deathmatch).value != 0. {
        Cmd_Score_f(ent);
        return;
    }
    (*(*ent).client).showinventory = false_0;
    (*(*ent).client).showscores = false_0;
    if (*(*ent).client).showhelp as libc::c_uint != 0
        && (*(*ent).client).pers.game_helpchanged == game.helpchanged
    {
        (*(*ent).client).showhelp = false_0;
        return;
    }
    (*(*ent).client).showhelp = true_0;
    (*(*ent).client).pers.helpchanged = 0 as libc::c_int;
    HelpComputer(ent);
}
#[no_mangle]
pub unsafe extern "C" fn G_SetStats(mut ent: *mut edict_t) {
    let mut item: *mut gitem_t = 0 as *mut gitem_t;
    let mut index: libc::c_int = 0;
    let mut cells: libc::c_int = 0;
    let mut power_armor_type: libc::c_int = 0;
    (*(*ent).client)
        .ps
        .stats[0 as libc::c_int as usize] = level.pic_health as libc::c_short;
    (*(*ent).client)
        .ps
        .stats[1 as libc::c_int as usize] = (*ent).health as libc::c_short;
    if (*(*ent).client).ammo_index == 0 {
        (*(*ent).client)
            .ps
            .stats[2 as libc::c_int as usize] = 0 as libc::c_int as libc::c_short;
        (*(*ent).client)
            .ps
            .stats[3 as libc::c_int as usize] = 0 as libc::c_int as libc::c_short;
    } else {
        item = &mut *itemlist.as_mut_ptr().offset((*(*ent).client).ammo_index as isize)
            as *mut gitem_t;
        (*(*ent).client)
            .ps
            .stats[2 as libc::c_int
            as usize] = (gi.imageindex).expect("non-null function pointer")((*item).icon)
            as libc::c_short;
        (*(*ent).client)
            .ps
            .stats[3 as libc::c_int
            as usize] = (*(*ent).client)
            .pers
            .inventory[(*(*ent).client).ammo_index as usize] as libc::c_short;
    }
    power_armor_type = PowerArmorType(ent);
    if power_armor_type != 0 {
        cells = (*(*ent).client)
            .pers
            .inventory[(FindItem(
            b"cells\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ))
            .offset_from(itemlist.as_mut_ptr()) as libc::c_long as usize];
        if cells == 0 as libc::c_int {
            (*ent).flags &= !(0x1000 as libc::c_int);
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
                    b"misc/power2.wav\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                ),
                1 as libc::c_int as libc::c_float,
                1 as libc::c_int as libc::c_float,
                0 as libc::c_int as libc::c_float,
            );
            power_armor_type = 0 as libc::c_int;
        }
    }
    index = ArmorIndex(ent);
    if power_armor_type != 0 && (index == 0 || level.framenum & 8 as libc::c_int != 0) {
        (*(*ent).client)
            .ps
            .stats[4 as libc::c_int
            as usize] = (gi.imageindex)
            .expect(
                "non-null function pointer",
            )(
            b"i_powershield\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) as libc::c_short;
        (*(*ent).client).ps.stats[5 as libc::c_int as usize] = cells as libc::c_short;
    } else if index != 0 {
        item = GetItemByIndex(index);
        (*(*ent).client)
            .ps
            .stats[4 as libc::c_int
            as usize] = (gi.imageindex).expect("non-null function pointer")((*item).icon)
            as libc::c_short;
        (*(*ent).client)
            .ps
            .stats[5 as libc::c_int
            as usize] = (*(*ent).client).pers.inventory[index as usize] as libc::c_short;
    } else {
        (*(*ent).client)
            .ps
            .stats[4 as libc::c_int as usize] = 0 as libc::c_int as libc::c_short;
        (*(*ent).client)
            .ps
            .stats[5 as libc::c_int as usize] = 0 as libc::c_int as libc::c_short;
    }
    if level.time > (*(*ent).client).pickup_msg_time {
        (*(*ent).client)
            .ps
            .stats[7 as libc::c_int as usize] = 0 as libc::c_int as libc::c_short;
        (*(*ent).client)
            .ps
            .stats[8 as libc::c_int as usize] = 0 as libc::c_int as libc::c_short;
    }
    if (*(*ent).client).quad_framenum > level.framenum as libc::c_float {
        (*(*ent).client)
            .ps
            .stats[9 as libc::c_int
            as usize] = (gi.imageindex)
            .expect(
                "non-null function pointer",
            )(b"p_quad\0" as *const u8 as *const libc::c_char as *mut libc::c_char)
            as libc::c_short;
        (*(*ent).client)
            .ps
            .stats[10 as libc::c_int
            as usize] = (((*(*ent).client).quad_framenum
            - level.framenum as libc::c_float) / 10 as libc::c_int as libc::c_float)
            as libc::c_short;
    } else if (*(*ent).client).invincible_framenum > level.framenum as libc::c_float {
        (*(*ent).client)
            .ps
            .stats[9 as libc::c_int
            as usize] = (gi.imageindex)
            .expect(
                "non-null function pointer",
            )(
            b"p_invulnerability\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        ) as libc::c_short;
        (*(*ent).client)
            .ps
            .stats[10 as libc::c_int
            as usize] = (((*(*ent).client).invincible_framenum
            - level.framenum as libc::c_float) / 10 as libc::c_int as libc::c_float)
            as libc::c_short;
    } else if (*(*ent).client).enviro_framenum > level.framenum as libc::c_float {
        (*(*ent).client)
            .ps
            .stats[9 as libc::c_int
            as usize] = (gi.imageindex)
            .expect(
                "non-null function pointer",
            )(b"p_envirosuit\0" as *const u8 as *const libc::c_char as *mut libc::c_char)
            as libc::c_short;
        (*(*ent).client)
            .ps
            .stats[10 as libc::c_int
            as usize] = (((*(*ent).client).enviro_framenum
            - level.framenum as libc::c_float) / 10 as libc::c_int as libc::c_float)
            as libc::c_short;
    } else if (*(*ent).client).breather_framenum > level.framenum as libc::c_float {
        (*(*ent).client)
            .ps
            .stats[9 as libc::c_int
            as usize] = (gi.imageindex)
            .expect(
                "non-null function pointer",
            )(b"p_rebreather\0" as *const u8 as *const libc::c_char as *mut libc::c_char)
            as libc::c_short;
        (*(*ent).client)
            .ps
            .stats[10 as libc::c_int
            as usize] = (((*(*ent).client).breather_framenum
            - level.framenum as libc::c_float) / 10 as libc::c_int as libc::c_float)
            as libc::c_short;
    } else {
        (*(*ent).client)
            .ps
            .stats[9 as libc::c_int as usize] = 0 as libc::c_int as libc::c_short;
        (*(*ent).client)
            .ps
            .stats[10 as libc::c_int as usize] = 0 as libc::c_int as libc::c_short;
    }
    if (*(*ent).client).pers.selected_item == -(1 as libc::c_int) {
        (*(*ent).client)
            .ps
            .stats[6 as libc::c_int as usize] = 0 as libc::c_int as libc::c_short;
    } else {
        (*(*ent).client)
            .ps
            .stats[6 as libc::c_int
            as usize] = (gi.imageindex)
            .expect(
                "non-null function pointer",
            )(
            (*itemlist.as_mut_ptr().offset((*(*ent).client).pers.selected_item as isize))
                .icon,
        ) as libc::c_short;
    }
    (*(*ent).client)
        .ps
        .stats[12 as libc::c_int
        as usize] = (*(*ent).client).pers.selected_item as libc::c_short;
    (*(*ent).client)
        .ps
        .stats[13 as libc::c_int as usize] = 0 as libc::c_int as libc::c_short;
    if (*deathmatch).value != 0. {
        if (*(*ent).client).pers.health <= 0 as libc::c_int
            || level.intermissiontime != 0.
            || (*(*ent).client).showscores as libc::c_uint != 0
        {
            let ref mut fresh1 = (*(*ent).client).ps.stats[13 as libc::c_int as usize];
            *fresh1 = (*fresh1 as libc::c_int | 1 as libc::c_int) as libc::c_short;
        }
        if (*(*ent).client).showinventory as libc::c_uint != 0
            && (*(*ent).client).pers.health > 0 as libc::c_int
        {
            let ref mut fresh2 = (*(*ent).client).ps.stats[13 as libc::c_int as usize];
            *fresh2 = (*fresh2 as libc::c_int | 2 as libc::c_int) as libc::c_short;
        }
    } else {
        if (*(*ent).client).showscores as libc::c_uint != 0
            || (*(*ent).client).showhelp as libc::c_uint != 0
        {
            let ref mut fresh3 = (*(*ent).client).ps.stats[13 as libc::c_int as usize];
            *fresh3 = (*fresh3 as libc::c_int | 1 as libc::c_int) as libc::c_short;
        }
        if (*(*ent).client).showinventory as libc::c_uint != 0
            && (*(*ent).client).pers.health > 0 as libc::c_int
        {
            let ref mut fresh4 = (*(*ent).client).ps.stats[13 as libc::c_int as usize];
            *fresh4 = (*fresh4 as libc::c_int | 2 as libc::c_int) as libc::c_short;
        }
    }
    (*(*ent).client)
        .ps
        .stats[14 as libc::c_int
        as usize] = (*(*ent).client).resp.score as libc::c_short;
    if (*(*ent).client).pers.helpchanged != 0 && level.framenum & 8 as libc::c_int != 0 {
        (*(*ent).client)
            .ps
            .stats[11 as libc::c_int
            as usize] = (gi.imageindex)
            .expect(
                "non-null function pointer",
            )(b"i_help\0" as *const u8 as *const libc::c_char as *mut libc::c_char)
            as libc::c_short;
    } else if ((*(*ent).client).pers.hand == 2 as libc::c_int
        || (*(*ent).client).ps.fov > 91 as libc::c_int as libc::c_float)
        && !((*(*ent).client).pers.weapon).is_null()
    {
        (*(*ent).client)
            .ps
            .stats[11 as libc::c_int
            as usize] = (gi.imageindex)
            .expect("non-null function pointer")((*(*(*ent).client).pers.weapon).icon)
            as libc::c_short;
    } else {
        (*(*ent).client)
            .ps
            .stats[11 as libc::c_int as usize] = 0 as libc::c_int as libc::c_short;
    }
    (*(*ent).client)
        .ps
        .stats[17 as libc::c_int as usize] = 0 as libc::c_int as libc::c_short;
}
#[no_mangle]
pub unsafe extern "C" fn G_CheckChaseStats(mut ent: *mut edict_t) {
    let mut i: libc::c_int = 0;
    let mut cl: *mut gclient_t = 0 as *mut gclient_t;
    i = 1 as libc::c_int;
    while i as libc::c_float <= (*maxclients).value {
        cl = (*g_edicts.offset(i as isize)).client;
        if !((*g_edicts.offset(i as isize)).inuse as u64 == 0
            || (*cl).chase_target != ent)
        {
            memcpy(
                ((*cl).ps.stats).as_mut_ptr() as *mut libc::c_void,
                ((*(*ent).client).ps.stats).as_mut_ptr() as *const libc::c_void,
                ::std::mem::size_of::<[libc::c_short; 32]>() as libc::c_ulong,
            );
            G_SetSpectatorStats(g_edicts.offset(i as isize));
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn G_SetSpectatorStats(mut ent: *mut edict_t) {
    let mut cl: *mut gclient_t = (*ent).client;
    if ((*cl).chase_target).is_null() {
        G_SetStats(ent);
    }
    (*cl).ps.stats[17 as libc::c_int as usize] = 1 as libc::c_int as libc::c_short;
    (*cl).ps.stats[13 as libc::c_int as usize] = 0 as libc::c_int as libc::c_short;
    if (*cl).pers.health <= 0 as libc::c_int || level.intermissiontime != 0.
        || (*cl).showscores as libc::c_uint != 0
    {
        let ref mut fresh5 = (*cl).ps.stats[13 as libc::c_int as usize];
        *fresh5 = (*fresh5 as libc::c_int | 1 as libc::c_int) as libc::c_short;
    }
    if (*cl).showinventory as libc::c_uint != 0 && (*cl).pers.health > 0 as libc::c_int {
        let ref mut fresh6 = (*cl).ps.stats[13 as libc::c_int as usize];
        *fresh6 = (*fresh6 as libc::c_int | 2 as libc::c_int) as libc::c_short;
    }
    if !((*cl).chase_target).is_null()
        && (*(*cl).chase_target).inuse as libc::c_uint != 0
    {
        (*cl)
            .ps
            .stats[16 as libc::c_int
            as usize] = ((32 as libc::c_int + 256 as libc::c_int + 256 as libc::c_int
            + 256 as libc::c_int + 256 as libc::c_int + 256 as libc::c_int)
            as libc::c_long + ((*cl).chase_target).offset_from(g_edicts) as libc::c_long
            - 1 as libc::c_int as libc::c_long) as libc::c_short;
    } else {
        (*cl).ps.stats[16 as libc::c_int as usize] = 0 as libc::c_int as libc::c_short;
    };
}
