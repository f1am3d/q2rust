#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    static mut gi: game_import_t;
    fn atan2(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn rand() -> libc::c_int;
    static mut vec3_origin: vec3_t;
    fn VectorCompare(v1: *mut vec_t, v2: *mut vec_t) -> libc::c_int;
    fn VectorLength(v: *mut vec_t) -> vec_t;
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
    fn Q_stricmp(s1: *mut libc::c_char, s2: *mut libc::c_char) -> libc::c_int;
    static mut game: game_locals_t;
    static mut level: level_locals_t;
    static mut maxclients: *mut cvar_t;
    static mut globals: game_export_t;
    static mut g_edicts: *mut edict_t;
    fn T_Damage(
        targ: *mut edict_t,
        inflictor: *mut edict_t,
        attacker: *mut edict_t,
        dir: *mut vec_t,
        point: *mut vec_t,
        normal: *mut vec_t,
        damage: libc::c_int,
        knockback: libc::c_int,
        dflags: libc::c_int,
        mod_0: libc::c_int,
    );
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
pub unsafe extern "C" fn G_ProjectSource(
    mut point: *mut vec_t,
    mut distance: *mut vec_t,
    mut forward: *mut vec_t,
    mut right: *mut vec_t,
    mut result: *mut vec_t,
) {
    *result
        .offset(
            0 as libc::c_int as isize,
        ) = *point.offset(0 as libc::c_int as isize)
        + *forward.offset(0 as libc::c_int as isize)
            * *distance.offset(0 as libc::c_int as isize)
        + *right.offset(0 as libc::c_int as isize)
            * *distance.offset(1 as libc::c_int as isize);
    *result
        .offset(
            1 as libc::c_int as isize,
        ) = *point.offset(1 as libc::c_int as isize)
        + *forward.offset(1 as libc::c_int as isize)
            * *distance.offset(0 as libc::c_int as isize)
        + *right.offset(1 as libc::c_int as isize)
            * *distance.offset(1 as libc::c_int as isize);
    *result
        .offset(
            2 as libc::c_int as isize,
        ) = *point.offset(2 as libc::c_int as isize)
        + *forward.offset(2 as libc::c_int as isize)
            * *distance.offset(0 as libc::c_int as isize)
        + *right.offset(2 as libc::c_int as isize)
            * *distance.offset(1 as libc::c_int as isize)
        + *distance.offset(2 as libc::c_int as isize);
}
#[no_mangle]
pub unsafe extern "C" fn G_Find(
    mut from: *mut edict_t,
    mut fieldofs: libc::c_int,
    mut match_0: *mut libc::c_char,
) -> *mut edict_t {
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    if from.is_null() {
        from = g_edicts;
    } else {
        from = from.offset(1);
    }
    while from < &mut *g_edicts.offset(globals.num_edicts as isize) as *mut edict_t {
        if !((*from).inuse as u64 == 0) {
            s = *((from as *mut byte).offset(fieldofs as isize)
                as *mut *mut libc::c_char);
            if !s.is_null() {
                if Q_stricmp(s, match_0) == 0 {
                    return from;
                }
            }
        }
        from = from.offset(1);
    }
    return 0 as *mut edict_t;
}
#[no_mangle]
pub unsafe extern "C" fn findradius(
    mut from: *mut edict_t,
    mut org: *mut vec_t,
    mut rad: libc::c_float,
) -> *mut edict_t {
    let mut eorg: vec3_t = [0.; 3];
    let mut j: libc::c_int = 0;
    if from.is_null() {
        from = g_edicts;
    } else {
        from = from.offset(1);
    }
    while from < &mut *g_edicts.offset(globals.num_edicts as isize) as *mut edict_t {
        if !((*from).inuse as u64 == 0) {
            if !((*from).solid as libc::c_uint
                == SOLID_NOT as libc::c_int as libc::c_uint)
            {
                j = 0 as libc::c_int;
                while j < 3 as libc::c_int {
                    eorg[j
                        as usize] = (*org.offset(j as isize) as libc::c_double
                        - ((*from).s.origin[j as usize] as libc::c_double
                            + ((*from).mins[j as usize] + (*from).maxs[j as usize])
                                as libc::c_double * 0.5f64)) as vec_t;
                    j += 1;
                }
                if !(VectorLength(eorg.as_mut_ptr()) > rad) {
                    return from;
                }
            }
        }
        from = from.offset(1);
    }
    return 0 as *mut edict_t;
}
#[no_mangle]
pub unsafe extern "C" fn G_PickTarget(
    mut targetname: *mut libc::c_char,
) -> *mut edict_t {
    let mut ent: *mut edict_t = 0 as *mut edict_t;
    let mut num_choices: libc::c_int = 0 as libc::c_int;
    let mut choice: [*mut edict_t; 8] = [0 as *mut edict_t; 8];
    if targetname.is_null() {
        (gi.dprintf)
            .expect(
                "non-null function pointer",
            )(
            b"G_PickTarget called with NULL targetname\n\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        );
        return 0 as *mut edict_t;
    }
    loop {
        ent = G_Find(
            ent,
            &mut (*(0 as *mut edict_t)).targetname as *mut *mut libc::c_char
                as libc::c_int,
            targetname,
        );
        if ent.is_null() {
            break;
        }
        let fresh0 = num_choices;
        num_choices = num_choices + 1;
        choice[fresh0 as usize] = ent;
        if num_choices == 8 as libc::c_int {
            break;
        }
    }
    if num_choices == 0 {
        (gi.dprintf)
            .expect(
                "non-null function pointer",
            )(
            b"G_PickTarget: target %s not found\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            targetname,
        );
        return 0 as *mut edict_t;
    }
    return choice[(rand() % num_choices) as usize];
}
#[no_mangle]
pub unsafe extern "C" fn Think_Delay(mut ent: *mut edict_t) {
    G_UseTargets(ent, (*ent).activator);
    G_FreeEdict(ent);
}
#[no_mangle]
pub unsafe extern "C" fn G_UseTargets(
    mut ent: *mut edict_t,
    mut activator: *mut edict_t,
) {
    let mut t: *mut edict_t = 0 as *mut edict_t;
    if (*ent).delay != 0. {
        t = G_Spawn();
        let ref mut fresh1 = (*t).classname;
        *fresh1 = b"DelayedUse\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char;
        (*t).nextthink = level.time + (*ent).delay;
        let ref mut fresh2 = (*t).think;
        *fresh2 = Some(Think_Delay as unsafe extern "C" fn(*mut edict_t) -> ());
        let ref mut fresh3 = (*t).activator;
        *fresh3 = activator;
        if activator.is_null() {
            (gi.dprintf)
                .expect(
                    "non-null function pointer",
                )(
                b"Think_Delay with no activator\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
        }
        let ref mut fresh4 = (*t).message;
        *fresh4 = (*ent).message;
        let ref mut fresh5 = (*t).target;
        *fresh5 = (*ent).target;
        let ref mut fresh6 = (*t).killtarget;
        *fresh6 = (*ent).killtarget;
        return;
    }
    if !((*ent).message).is_null() && (*activator).svflags & 0x4 as libc::c_int == 0 {
        (gi.centerprintf)
            .expect(
                "non-null function pointer",
            )(
            activator,
            b"%s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            (*ent).message,
        );
        if (*ent).noise_index != 0 {
            (gi.sound)
                .expect(
                    "non-null function pointer",
                )(
                activator,
                0 as libc::c_int,
                (*ent).noise_index,
                1 as libc::c_int as libc::c_float,
                1 as libc::c_int as libc::c_float,
                0 as libc::c_int as libc::c_float,
            );
        } else {
            (gi.sound)
                .expect(
                    "non-null function pointer",
                )(
                activator,
                0 as libc::c_int,
                (gi.soundindex)
                    .expect(
                        "non-null function pointer",
                    )(
                    b"misc/talk1.wav\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                ),
                1 as libc::c_int as libc::c_float,
                1 as libc::c_int as libc::c_float,
                0 as libc::c_int as libc::c_float,
            );
        }
    }
    if !((*ent).killtarget).is_null() {
        t = 0 as *mut edict_t;
        loop {
            t = G_Find(
                t,
                &mut (*(0 as *mut edict_t)).targetname as *mut *mut libc::c_char
                    as libc::c_int,
                (*ent).killtarget,
            );
            if t.is_null() {
                break;
            }
            G_FreeEdict(t);
            if (*ent).inuse as u64 == 0 {
                (gi.dprintf)
                    .expect(
                        "non-null function pointer",
                    )(
                    b"entity was removed while using killtargets\n\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                );
                return;
            }
        }
    }
    if !((*ent).target).is_null() {
        t = 0 as *mut edict_t;
        loop {
            t = G_Find(
                t,
                &mut (*(0 as *mut edict_t)).targetname as *mut *mut libc::c_char
                    as libc::c_int,
                (*ent).target,
            );
            if t.is_null() {
                break;
            }
            if Q_stricmp(
                (*t).classname,
                b"func_areaportal\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            ) == 0
                && (Q_stricmp(
                    (*ent).classname,
                    b"func_door\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                ) == 0
                    || Q_stricmp(
                        (*ent).classname,
                        b"func_door_rotating\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    ) == 0)
            {
                continue;
            }
            if t == ent {
                (gi.dprintf)
                    .expect(
                        "non-null function pointer",
                    )(
                    b"WARNING: Entity used itself.\n\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                );
            } else if ((*t).use_0).is_some() {
                ((*t).use_0).expect("non-null function pointer")(t, ent, activator);
            }
            if (*ent).inuse as u64 == 0 {
                (gi.dprintf)
                    .expect(
                        "non-null function pointer",
                    )(
                    b"entity was removed while using targets\n\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                );
                return;
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn tv(
    mut x: libc::c_float,
    mut y: libc::c_float,
    mut z: libc::c_float,
) -> *mut libc::c_float {
    static mut index: libc::c_int = 0;
    static mut vecs: [vec3_t; 8] = [[0.; 3]; 8];
    let mut v: *mut libc::c_float = 0 as *mut libc::c_float;
    v = (vecs[index as usize]).as_mut_ptr();
    index = index + 1 as libc::c_int & 7 as libc::c_int;
    *v.offset(0 as libc::c_int as isize) = x;
    *v.offset(1 as libc::c_int as isize) = y;
    *v.offset(2 as libc::c_int as isize) = z;
    return v;
}
#[no_mangle]
pub unsafe extern "C" fn vtos(mut v: *mut vec_t) -> *mut libc::c_char {
    static mut index: libc::c_int = 0;
    static mut str: [[libc::c_char; 32]; 8] = [[0; 32]; 8];
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    s = (str[index as usize]).as_mut_ptr();
    index = index + 1 as libc::c_int & 7 as libc::c_int;
    Com_sprintf(
        s,
        32 as libc::c_int,
        b"(%i %i %i)\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        *v.offset(0 as libc::c_int as isize) as libc::c_int,
        *v.offset(1 as libc::c_int as isize) as libc::c_int,
        *v.offset(2 as libc::c_int as isize) as libc::c_int,
    );
    return s;
}
#[no_mangle]
pub static mut VEC_UP: vec3_t = [
    0 as libc::c_int as vec_t,
    -(1 as libc::c_int) as vec_t,
    0 as libc::c_int as vec_t,
];
#[no_mangle]
pub static mut MOVEDIR_UP: vec3_t = [
    0 as libc::c_int as vec_t,
    0 as libc::c_int as vec_t,
    1 as libc::c_int as vec_t,
];
#[no_mangle]
pub static mut VEC_DOWN: vec3_t = [
    0 as libc::c_int as vec_t,
    -(2 as libc::c_int) as vec_t,
    0 as libc::c_int as vec_t,
];
#[no_mangle]
pub static mut MOVEDIR_DOWN: vec3_t = [
    0 as libc::c_int as vec_t,
    0 as libc::c_int as vec_t,
    -(1 as libc::c_int) as vec_t,
];
#[no_mangle]
pub unsafe extern "C" fn G_SetMovedir(mut angles: *mut vec_t, mut movedir: *mut vec_t) {
    if VectorCompare(angles, VEC_UP.as_mut_ptr()) != 0 {
        *movedir
            .offset(0 as libc::c_int as isize) = MOVEDIR_UP[0 as libc::c_int as usize];
        *movedir
            .offset(1 as libc::c_int as isize) = MOVEDIR_UP[1 as libc::c_int as usize];
        *movedir
            .offset(2 as libc::c_int as isize) = MOVEDIR_UP[2 as libc::c_int as usize];
    } else if VectorCompare(angles, VEC_DOWN.as_mut_ptr()) != 0 {
        *movedir
            .offset(0 as libc::c_int as isize) = MOVEDIR_DOWN[0 as libc::c_int as usize];
        *movedir
            .offset(1 as libc::c_int as isize) = MOVEDIR_DOWN[1 as libc::c_int as usize];
        *movedir
            .offset(2 as libc::c_int as isize) = MOVEDIR_DOWN[2 as libc::c_int as usize];
    } else {
        AngleVectors(angles, movedir, 0 as *mut vec_t, 0 as *mut vec_t);
    }
    let ref mut fresh7 = *angles.offset(2 as libc::c_int as isize);
    *fresh7 = 0 as libc::c_int as vec_t;
    let ref mut fresh8 = *angles.offset(1 as libc::c_int as isize);
    *fresh8 = *fresh7;
    *angles.offset(0 as libc::c_int as isize) = *fresh8;
}
#[no_mangle]
pub unsafe extern "C" fn vectoyaw(mut vec: *mut vec_t) -> libc::c_float {
    let mut yaw: libc::c_float = 0.;
    if *vec.offset(0 as libc::c_int as isize) == 0 as libc::c_int as libc::c_float {
        yaw = 0 as libc::c_int as libc::c_float;
        if *vec.offset(1 as libc::c_int as isize) > 0 as libc::c_int as libc::c_float {
            yaw = 90 as libc::c_int as libc::c_float;
        } else if *vec.offset(1 as libc::c_int as isize)
            < 0 as libc::c_int as libc::c_float
        {
            yaw = -(90 as libc::c_int) as libc::c_float;
        }
    } else {
        yaw = (atan2(
            *vec.offset(1 as libc::c_int as isize) as libc::c_double,
            *vec.offset(0 as libc::c_int as isize) as libc::c_double,
        ) * 180 as libc::c_int as libc::c_double / 3.14159265358979323846f64)
            as libc::c_int as libc::c_float;
        if yaw < 0 as libc::c_int as libc::c_float {
            yaw += 360 as libc::c_int as libc::c_float;
        }
    }
    return yaw;
}
#[no_mangle]
pub unsafe extern "C" fn vectoangles(mut value1: *mut vec_t, mut angles: *mut vec_t) {
    let mut forward: libc::c_float = 0.;
    let mut yaw: libc::c_float = 0.;
    let mut pitch: libc::c_float = 0.;
    if *value1.offset(1 as libc::c_int as isize) == 0 as libc::c_int as libc::c_float
        && *value1.offset(0 as libc::c_int as isize) == 0 as libc::c_int as libc::c_float
    {
        yaw = 0 as libc::c_int as libc::c_float;
        if *value1.offset(2 as libc::c_int as isize) > 0 as libc::c_int as libc::c_float
        {
            pitch = 90 as libc::c_int as libc::c_float;
        } else {
            pitch = 270 as libc::c_int as libc::c_float;
        }
    } else {
        if *value1.offset(0 as libc::c_int as isize) != 0. {
            yaw = (atan2(
                *value1.offset(1 as libc::c_int as isize) as libc::c_double,
                *value1.offset(0 as libc::c_int as isize) as libc::c_double,
            ) * 180 as libc::c_int as libc::c_double / 3.14159265358979323846f64)
                as libc::c_int as libc::c_float;
        } else if *value1.offset(1 as libc::c_int as isize)
            > 0 as libc::c_int as libc::c_float
        {
            yaw = 90 as libc::c_int as libc::c_float;
        } else {
            yaw = -(90 as libc::c_int) as libc::c_float;
        }
        if yaw < 0 as libc::c_int as libc::c_float {
            yaw += 360 as libc::c_int as libc::c_float;
        }
        forward = sqrt(
            (*value1.offset(0 as libc::c_int as isize)
                * *value1.offset(0 as libc::c_int as isize)
                + *value1.offset(1 as libc::c_int as isize)
                    * *value1.offset(1 as libc::c_int as isize)) as libc::c_double,
        ) as libc::c_float;
        pitch = (atan2(
            *value1.offset(2 as libc::c_int as isize) as libc::c_double,
            forward as libc::c_double,
        ) * 180 as libc::c_int as libc::c_double / 3.14159265358979323846f64)
            as libc::c_int as libc::c_float;
        if pitch < 0 as libc::c_int as libc::c_float {
            pitch += 360 as libc::c_int as libc::c_float;
        }
    }
    *angles.offset(0 as libc::c_int as isize) = -pitch;
    *angles.offset(1 as libc::c_int as isize) = yaw;
    *angles.offset(2 as libc::c_int as isize) = 0 as libc::c_int as vec_t;
}
#[no_mangle]
pub unsafe extern "C" fn G_CopyString(mut in_0: *mut libc::c_char) -> *mut libc::c_char {
    let mut out: *mut libc::c_char = 0 as *mut libc::c_char;
    out = (gi.TagMalloc)
        .expect(
            "non-null function pointer",
        )(
        (strlen(in_0)).wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int,
        766 as libc::c_int,
    ) as *mut libc::c_char;
    strcpy(out, in_0);
    return out;
}
#[no_mangle]
pub unsafe extern "C" fn G_InitEdict(mut e: *mut edict_t) {
    (*e).inuse = true_0;
    let ref mut fresh9 = (*e).classname;
    *fresh9 = b"noclass\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    (*e).gravity = 1.0f64 as libc::c_float;
    (*e).s.number = e.offset_from(g_edicts) as libc::c_long as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn G_Spawn() -> *mut edict_t {
    let mut i: libc::c_int = 0;
    let mut e: *mut edict_t = 0 as *mut edict_t;
    e = &mut *g_edicts
        .offset(((*maxclients).value as libc::c_int + 1 as libc::c_int) as isize)
        as *mut edict_t;
    i = ((*maxclients).value + 1 as libc::c_int as libc::c_float) as libc::c_int;
    while i < globals.num_edicts {
        if (*e).inuse as u64 == 0
            && ((*e).freetime < 2 as libc::c_int as libc::c_float
                || (level.time - (*e).freetime) as libc::c_double > 0.5f64)
        {
            G_InitEdict(e);
            return e;
        }
        i += 1;
        e = e.offset(1);
    }
    if i == game.maxentities {
        (gi.error)
            .expect(
                "non-null function pointer",
            )(
            b"ED_Alloc: no free edicts\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    globals.num_edicts += 1;
    G_InitEdict(e);
    return e;
}
#[no_mangle]
pub unsafe extern "C" fn G_FreeEdict(mut ed: *mut edict_t) {
    (gi.unlinkentity).expect("non-null function pointer")(ed);
    if ed.offset_from(g_edicts) as libc::c_long as libc::c_float
        <= (*maxclients).value + 8 as libc::c_int as libc::c_float
    {
        return;
    }
    memset(
        ed as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<edict_t>() as libc::c_ulong,
    );
    let ref mut fresh10 = (*ed).classname;
    *fresh10 = b"freed\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    (*ed).freetime = level.time;
    (*ed).inuse = false_0;
}
#[no_mangle]
pub unsafe extern "C" fn G_TouchTriggers(mut ent: *mut edict_t) {
    let mut i: libc::c_int = 0;
    let mut num: libc::c_int = 0;
    let mut touch: [*mut edict_t; 1024] = [0 as *mut edict_t; 1024];
    let mut hit: *mut edict_t = 0 as *mut edict_t;
    if (!((*ent).client).is_null() || (*ent).svflags & 0x4 as libc::c_int != 0)
        && (*ent).health <= 0 as libc::c_int
    {
        return;
    }
    num = (gi.BoxEdicts)
        .expect(
            "non-null function pointer",
        )(
        ((*ent).absmin).as_mut_ptr(),
        ((*ent).absmax).as_mut_ptr(),
        touch.as_mut_ptr(),
        1024 as libc::c_int,
        2 as libc::c_int,
    );
    i = 0 as libc::c_int;
    while i < num {
        hit = touch[i as usize];
        if !((*hit).inuse as u64 == 0) {
            if !((*hit).touch).is_none() {
                ((*hit).touch)
                    .expect(
                        "non-null function pointer",
                    )(hit, ent, 0 as *mut cplane_t, 0 as *mut csurface_t);
            }
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn G_TouchSolids(mut ent: *mut edict_t) {
    let mut i: libc::c_int = 0;
    let mut num: libc::c_int = 0;
    let mut touch: [*mut edict_t; 1024] = [0 as *mut edict_t; 1024];
    let mut hit: *mut edict_t = 0 as *mut edict_t;
    num = (gi.BoxEdicts)
        .expect(
            "non-null function pointer",
        )(
        ((*ent).absmin).as_mut_ptr(),
        ((*ent).absmax).as_mut_ptr(),
        touch.as_mut_ptr(),
        1024 as libc::c_int,
        1 as libc::c_int,
    );
    i = 0 as libc::c_int;
    while i < num {
        hit = touch[i as usize];
        if !((*hit).inuse as u64 == 0) {
            if ((*ent).touch).is_some() {
                ((*ent).touch)
                    .expect(
                        "non-null function pointer",
                    )(hit, ent, 0 as *mut cplane_t, 0 as *mut csurface_t);
            }
            if (*ent).inuse as u64 == 0 {
                break;
            }
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn KillBox(mut ent: *mut edict_t) -> qboolean {
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
    loop {
        tr = (gi.trace)
            .expect(
                "non-null function pointer",
            )(
            ((*ent).s.origin).as_mut_ptr(),
            ((*ent).mins).as_mut_ptr(),
            ((*ent).maxs).as_mut_ptr(),
            ((*ent).s.origin).as_mut_ptr(),
            0 as *mut edict_t,
            1 as libc::c_int | 0x10000 as libc::c_int | 2 as libc::c_int
                | 0x2000000 as libc::c_int,
        );
        if (tr.ent).is_null() {
            break;
        }
        T_Damage(
            tr.ent,
            ent,
            ent,
            vec3_origin.as_mut_ptr(),
            ((*ent).s.origin).as_mut_ptr(),
            vec3_origin.as_mut_ptr(),
            100000 as libc::c_int,
            0 as libc::c_int,
            0x20 as libc::c_int,
            21 as libc::c_int,
        );
        if (*tr.ent).solid as u64 != 0 {
            return false_0;
        }
    }
    return true_0;
}
