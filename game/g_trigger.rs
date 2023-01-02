#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn atoi(__nptr: *const libc::c_char) -> libc::c_int;
    static mut vec3_origin: vec3_t;
    fn VectorMA(
        veca: *mut vec_t,
        scale: libc::c_float,
        vecb: *mut vec_t,
        vecc: *mut vec_t,
    );
    fn _DotProduct(v1: *mut vec_t, v2: *mut vec_t) -> vec_t;
    fn VectorCompare(v1: *mut vec_t, v2: *mut vec_t) -> libc::c_int;
    fn VectorScale(in_0: *mut vec_t, scale: vec_t, out: *mut vec_t);
    fn AngleVectors(
        angles: *mut vec_t,
        forward: *mut vec_t,
        right: *mut vec_t,
        up: *mut vec_t,
    );
    static mut game: game_locals_t;
    static mut level: level_locals_t;
    static mut gi: game_import_t;
    static mut st: spawn_temp_t;
    static mut g_edicts: *mut edict_t;
    static mut coop: *mut cvar_t;
    static mut itemlist: [gitem_t; 0];
    fn FindItemByClassname(classname: *mut libc::c_char) -> *mut gitem_t;
    fn G_UseTargets(ent: *mut edict_t, activator: *mut edict_t);
    fn G_SetMovedir(angles: *mut vec_t, movedir: *mut vec_t);
    fn G_FreeEdict(e: *mut edict_t);
    fn vtos(v: *mut vec_t) -> *mut libc::c_char;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spawn_temp_t {
    pub sky: *mut libc::c_char,
    pub skyrotate: libc::c_float,
    pub skyaxis: vec3_t,
    pub nextmap: *mut libc::c_char,
    pub lip: libc::c_int,
    pub distance: libc::c_int,
    pub height: libc::c_int,
    pub noise: *mut libc::c_char,
    pub pausetime: libc::c_float,
    pub item: *mut libc::c_char,
    pub gravity: *mut libc::c_char,
    pub minyaw: libc::c_float,
    pub maxyaw: libc::c_float,
    pub minpitch: libc::c_float,
    pub maxpitch: libc::c_float,
}
#[no_mangle]
pub unsafe extern "C" fn InitTrigger(mut self_0: *mut edict_t) {
    if VectorCompare(((*self_0).s.angles).as_mut_ptr(), vec3_origin.as_mut_ptr()) == 0 {
        G_SetMovedir(
            ((*self_0).s.angles).as_mut_ptr(),
            ((*self_0).movedir).as_mut_ptr(),
        );
    }
    (*self_0).solid = SOLID_TRIGGER;
    (*self_0).movetype = MOVETYPE_NONE as libc::c_int;
    (gi.setmodel).expect("non-null function pointer")(self_0, (*self_0).model);
    (*self_0).svflags = 0x1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn multi_wait(mut ent: *mut edict_t) {
    (*ent).nextthink = 0 as libc::c_int as libc::c_float;
}
#[no_mangle]
pub unsafe extern "C" fn multi_trigger(mut ent: *mut edict_t) {
    if (*ent).nextthink != 0. {
        return;
    }
    G_UseTargets(ent, (*ent).activator);
    if (*ent).wait > 0 as libc::c_int as libc::c_float {
        let ref mut fresh0 = (*ent).think;
        *fresh0 = Some(multi_wait as unsafe extern "C" fn(*mut edict_t) -> ());
        (*ent).nextthink = level.time + (*ent).wait;
    } else {
        let ref mut fresh1 = (*ent).touch;
        *fresh1 = None;
        (*ent).nextthink = (level.time as libc::c_double + 0.1f64) as libc::c_float;
        let ref mut fresh2 = (*ent).think;
        *fresh2 = Some(G_FreeEdict as unsafe extern "C" fn(*mut edict_t) -> ());
    };
}
#[no_mangle]
pub unsafe extern "C" fn Use_Multi(
    mut ent: *mut edict_t,
    mut other: *mut edict_t,
    mut activator: *mut edict_t,
) {
    let ref mut fresh3 = (*ent).activator;
    *fresh3 = activator;
    multi_trigger(ent);
}
#[no_mangle]
pub unsafe extern "C" fn Touch_Multi(
    mut self_0: *mut edict_t,
    mut other: *mut edict_t,
    mut plane: *mut cplane_t,
    mut surf: *mut csurface_t,
) {
    if !((*other).client).is_null() {
        if (*self_0).spawnflags & 2 as libc::c_int != 0 {
            return;
        }
    } else if (*other).svflags & 0x4 as libc::c_int != 0 {
        if (*self_0).spawnflags & 1 as libc::c_int == 0 {
            return;
        }
    } else {
        return
    }
    if VectorCompare(((*self_0).movedir).as_mut_ptr(), vec3_origin.as_mut_ptr()) == 0 {
        let mut forward: vec3_t = [0.; 3];
        AngleVectors(
            ((*other).s.angles).as_mut_ptr(),
            forward.as_mut_ptr(),
            0 as *mut vec_t,
            0 as *mut vec_t,
        );
        if _DotProduct(forward.as_mut_ptr(), ((*self_0).movedir).as_mut_ptr())
            < 0 as libc::c_int as libc::c_float
        {
            return;
        }
    }
    let ref mut fresh4 = (*self_0).activator;
    *fresh4 = other;
    multi_trigger(self_0);
}
#[no_mangle]
pub unsafe extern "C" fn trigger_enable(
    mut self_0: *mut edict_t,
    mut other: *mut edict_t,
    mut activator: *mut edict_t,
) {
    (*self_0).solid = SOLID_TRIGGER;
    let ref mut fresh5 = (*self_0).use_0;
    *fresh5 = Some(
        Use_Multi as unsafe extern "C" fn(*mut edict_t, *mut edict_t, *mut edict_t) -> (),
    );
    (gi.linkentity).expect("non-null function pointer")(self_0);
}
#[no_mangle]
pub unsafe extern "C" fn SP_trigger_multiple(mut ent: *mut edict_t) {
    if (*ent).sounds == 1 as libc::c_int {
        (*ent)
            .noise_index = (gi.soundindex)
            .expect(
                "non-null function pointer",
            )(
            b"misc/secret.wav\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    } else if (*ent).sounds == 2 as libc::c_int {
        (*ent)
            .noise_index = (gi.soundindex)
            .expect(
                "non-null function pointer",
            )(
            b"misc/talk.wav\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    } else if (*ent).sounds == 3 as libc::c_int {
        (*ent)
            .noise_index = (gi.soundindex)
            .expect(
                "non-null function pointer",
            )(
            b"misc/trigger1.wav\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    if (*ent).wait == 0. {
        (*ent).wait = 0.2f64 as libc::c_float;
    }
    let ref mut fresh6 = (*ent).touch;
    *fresh6 = Some(
        Touch_Multi
            as unsafe extern "C" fn(
                *mut edict_t,
                *mut edict_t,
                *mut cplane_t,
                *mut csurface_t,
            ) -> (),
    );
    (*ent).movetype = MOVETYPE_NONE as libc::c_int;
    (*ent).svflags |= 0x1 as libc::c_int;
    if (*ent).spawnflags & 4 as libc::c_int != 0 {
        (*ent).solid = SOLID_NOT;
        let ref mut fresh7 = (*ent).use_0;
        *fresh7 = Some(
            trigger_enable
                as unsafe extern "C" fn(*mut edict_t, *mut edict_t, *mut edict_t) -> (),
        );
    } else {
        (*ent).solid = SOLID_TRIGGER;
        let ref mut fresh8 = (*ent).use_0;
        *fresh8 = Some(
            Use_Multi
                as unsafe extern "C" fn(*mut edict_t, *mut edict_t, *mut edict_t) -> (),
        );
    }
    if VectorCompare(((*ent).s.angles).as_mut_ptr(), vec3_origin.as_mut_ptr()) == 0 {
        G_SetMovedir(((*ent).s.angles).as_mut_ptr(), ((*ent).movedir).as_mut_ptr());
    }
    (gi.setmodel).expect("non-null function pointer")(ent, (*ent).model);
    (gi.linkentity).expect("non-null function pointer")(ent);
}
#[no_mangle]
pub unsafe extern "C" fn SP_trigger_once(mut ent: *mut edict_t) {
    if (*ent).spawnflags & 1 as libc::c_int != 0 {
        let mut v: vec3_t = [0.; 3];
        VectorMA(
            ((*ent).mins).as_mut_ptr(),
            0.5f64 as libc::c_float,
            ((*ent).size).as_mut_ptr(),
            v.as_mut_ptr(),
        );
        (*ent).spawnflags &= !(1 as libc::c_int);
        (*ent).spawnflags |= 4 as libc::c_int;
        (gi.dprintf)
            .expect(
                "non-null function pointer",
            )(
            b"fixed TRIGGERED flag on %s at %s\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            (*ent).classname,
            vtos(v.as_mut_ptr()),
        );
    }
    (*ent).wait = -(1 as libc::c_int) as libc::c_float;
    SP_trigger_multiple(ent);
}
#[no_mangle]
pub unsafe extern "C" fn trigger_relay_use(
    mut self_0: *mut edict_t,
    mut other: *mut edict_t,
    mut activator: *mut edict_t,
) {
    G_UseTargets(self_0, activator);
}
#[no_mangle]
pub unsafe extern "C" fn SP_trigger_relay(mut self_0: *mut edict_t) {
    let ref mut fresh9 = (*self_0).use_0;
    *fresh9 = Some(
        trigger_relay_use
            as unsafe extern "C" fn(*mut edict_t, *mut edict_t, *mut edict_t) -> (),
    );
}
#[no_mangle]
pub unsafe extern "C" fn trigger_key_use(
    mut self_0: *mut edict_t,
    mut other: *mut edict_t,
    mut activator: *mut edict_t,
) {
    let mut index: libc::c_int = 0;
    if ((*self_0).item).is_null() {
        return;
    }
    if ((*activator).client).is_null() {
        return;
    }
    index = ((*self_0).item).offset_from(itemlist.as_mut_ptr()) as libc::c_long
        as libc::c_int;
    if (*(*activator).client).pers.inventory[index as usize] == 0 {
        if level.time < (*self_0).touch_debounce_time {
            return;
        }
        (*self_0)
            .touch_debounce_time = (level.time as libc::c_double + 5.0f64)
            as libc::c_float;
        (gi.centerprintf)
            .expect(
                "non-null function pointer",
            )(
            activator,
            b"You need the %s\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            (*(*self_0).item).pickup_name,
        );
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
                b"misc/keytry.wav\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            ),
            1 as libc::c_int as libc::c_float,
            1 as libc::c_int as libc::c_float,
            0 as libc::c_int as libc::c_float,
        );
        return;
    }
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
            b"misc/keyuse.wav\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ),
        1 as libc::c_int as libc::c_float,
        1 as libc::c_int as libc::c_float,
        0 as libc::c_int as libc::c_float,
    );
    if (*coop).value != 0. {
        let mut player: libc::c_int = 0;
        let mut ent: *mut edict_t = 0 as *mut edict_t;
        if strcmp(
            (*(*self_0).item).classname,
            b"key_power_cube\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            let mut cube: libc::c_int = 0;
            cube = 0 as libc::c_int;
            while cube < 8 as libc::c_int {
                if (*(*activator).client).pers.power_cubes & (1 as libc::c_int) << cube
                    != 0
                {
                    break;
                }
                cube += 1;
            }
            player = 1 as libc::c_int;
            while player <= game.maxclients {
                ent = &mut *g_edicts.offset(player as isize) as *mut edict_t;
                if !((*ent).inuse as u64 == 0) {
                    if !((*ent).client).is_null() {
                        if (*(*ent).client).pers.power_cubes & (1 as libc::c_int) << cube
                            != 0
                        {
                            let ref mut fresh10 = (*(*ent).client)
                                .pers
                                .inventory[index as usize];
                            *fresh10 -= 1;
                            (*(*ent).client).pers.power_cubes
                                &= !((1 as libc::c_int) << cube);
                        }
                    }
                }
                player += 1;
            }
        } else {
            player = 1 as libc::c_int;
            while player <= game.maxclients {
                ent = &mut *g_edicts.offset(player as isize) as *mut edict_t;
                if !((*ent).inuse as u64 == 0) {
                    if !((*ent).client).is_null() {
                        (*(*ent).client)
                            .pers
                            .inventory[index as usize] = 0 as libc::c_int;
                    }
                }
                player += 1;
            }
        }
    } else {
        let ref mut fresh11 = (*(*activator).client).pers.inventory[index as usize];
        *fresh11 -= 1;
    }
    G_UseTargets(self_0, activator);
    let ref mut fresh12 = (*self_0).use_0;
    *fresh12 = None;
}
#[no_mangle]
pub unsafe extern "C" fn SP_trigger_key(mut self_0: *mut edict_t) {
    if (st.item).is_null() {
        (gi.dprintf)
            .expect(
                "non-null function pointer",
            )(
            b"no key item for trigger_key at %s\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            vtos(((*self_0).s.origin).as_mut_ptr()),
        );
        return;
    }
    let ref mut fresh13 = (*self_0).item;
    *fresh13 = FindItemByClassname(st.item);
    if ((*self_0).item).is_null() {
        (gi.dprintf)
            .expect(
                "non-null function pointer",
            )(
            b"item %s not found for trigger_key at %s\n\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            st.item,
            vtos(((*self_0).s.origin).as_mut_ptr()),
        );
        return;
    }
    if ((*self_0).target).is_null() {
        (gi.dprintf)
            .expect(
                "non-null function pointer",
            )(
            b"%s at %s has no target\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            (*self_0).classname,
            vtos(((*self_0).s.origin).as_mut_ptr()),
        );
        return;
    }
    (gi.soundindex)
        .expect(
            "non-null function pointer",
        )(b"misc/keytry.wav\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    (gi.soundindex)
        .expect(
            "non-null function pointer",
        )(b"misc/keyuse.wav\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    let ref mut fresh14 = (*self_0).use_0;
    *fresh14 = Some(
        trigger_key_use
            as unsafe extern "C" fn(*mut edict_t, *mut edict_t, *mut edict_t) -> (),
    );
}
#[no_mangle]
pub unsafe extern "C" fn trigger_counter_use(
    mut self_0: *mut edict_t,
    mut other: *mut edict_t,
    mut activator: *mut edict_t,
) {
    if (*self_0).count == 0 as libc::c_int {
        return;
    }
    let ref mut fresh15 = (*self_0).count;
    *fresh15 -= 1;
    if (*self_0).count != 0 {
        if (*self_0).spawnflags & 1 as libc::c_int == 0 {
            (gi.centerprintf)
                .expect(
                    "non-null function pointer",
                )(
                activator,
                b"%i more to go...\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                (*self_0).count,
            );
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
        return;
    }
    if (*self_0).spawnflags & 1 as libc::c_int == 0 {
        (gi.centerprintf)
            .expect(
                "non-null function pointer",
            )(
            activator,
            b"Sequence completed!\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
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
    let ref mut fresh16 = (*self_0).activator;
    *fresh16 = activator;
    multi_trigger(self_0);
}
#[no_mangle]
pub unsafe extern "C" fn SP_trigger_counter(mut self_0: *mut edict_t) {
    (*self_0).wait = -(1 as libc::c_int) as libc::c_float;
    if (*self_0).count == 0 {
        (*self_0).count = 2 as libc::c_int;
    }
    let ref mut fresh17 = (*self_0).use_0;
    *fresh17 = Some(
        trigger_counter_use
            as unsafe extern "C" fn(*mut edict_t, *mut edict_t, *mut edict_t) -> (),
    );
}
#[no_mangle]
pub unsafe extern "C" fn SP_trigger_always(mut ent: *mut edict_t) {
    if ((*ent).delay as libc::c_double) < 0.2f64 {
        (*ent).delay = 0.2f64 as libc::c_float;
    }
    G_UseTargets(ent, ent);
}
static mut windsound: libc::c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn trigger_push_touch(
    mut self_0: *mut edict_t,
    mut other: *mut edict_t,
    mut plane: *mut cplane_t,
    mut surf: *mut csurface_t,
) {
    if strcmp((*other).classname, b"grenade\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        VectorScale(
            ((*self_0).movedir).as_mut_ptr(),
            (*self_0).speed * 10 as libc::c_int as libc::c_float,
            ((*other).velocity).as_mut_ptr(),
        );
    } else if (*other).health > 0 as libc::c_int {
        VectorScale(
            ((*self_0).movedir).as_mut_ptr(),
            (*self_0).speed * 10 as libc::c_int as libc::c_float,
            ((*other).velocity).as_mut_ptr(),
        );
        if !((*other).client).is_null() {
            (*(*other).client)
                .oldvelocity[0 as libc::c_int
                as usize] = (*other).velocity[0 as libc::c_int as usize];
            (*(*other).client)
                .oldvelocity[1 as libc::c_int
                as usize] = (*other).velocity[1 as libc::c_int as usize];
            (*(*other).client)
                .oldvelocity[2 as libc::c_int
                as usize] = (*other).velocity[2 as libc::c_int as usize];
            if (*other).fly_sound_debounce_time < level.time {
                (*other)
                    .fly_sound_debounce_time = (level.time as libc::c_double + 1.5f64)
                    as libc::c_float;
                (gi.sound)
                    .expect(
                        "non-null function pointer",
                    )(
                    other,
                    0 as libc::c_int,
                    windsound,
                    1 as libc::c_int as libc::c_float,
                    1 as libc::c_int as libc::c_float,
                    0 as libc::c_int as libc::c_float,
                );
            }
        }
    }
    if (*self_0).spawnflags & 1 as libc::c_int != 0 {
        G_FreeEdict(self_0);
    }
}
#[no_mangle]
pub unsafe extern "C" fn SP_trigger_push(mut self_0: *mut edict_t) {
    InitTrigger(self_0);
    windsound = (gi.soundindex)
        .expect(
            "non-null function pointer",
        )(
        b"misc/windfly.wav\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    let ref mut fresh18 = (*self_0).touch;
    *fresh18 = Some(
        trigger_push_touch
            as unsafe extern "C" fn(
                *mut edict_t,
                *mut edict_t,
                *mut cplane_t,
                *mut csurface_t,
            ) -> (),
    );
    if (*self_0).speed == 0. {
        (*self_0).speed = 1000 as libc::c_int as libc::c_float;
    }
    (gi.linkentity).expect("non-null function pointer")(self_0);
}
#[no_mangle]
pub unsafe extern "C" fn hurt_use(
    mut self_0: *mut edict_t,
    mut other: *mut edict_t,
    mut activator: *mut edict_t,
) {
    if (*self_0).solid as libc::c_uint == SOLID_NOT as libc::c_int as libc::c_uint {
        (*self_0).solid = SOLID_TRIGGER;
    } else {
        (*self_0).solid = SOLID_NOT;
    }
    (gi.linkentity).expect("non-null function pointer")(self_0);
    if (*self_0).spawnflags & 2 as libc::c_int == 0 {
        let ref mut fresh19 = (*self_0).use_0;
        *fresh19 = None;
    }
}
#[no_mangle]
pub unsafe extern "C" fn hurt_touch(
    mut self_0: *mut edict_t,
    mut other: *mut edict_t,
    mut plane: *mut cplane_t,
    mut surf: *mut csurface_t,
) {
    let mut dflags: libc::c_int = 0;
    if (*other).takedamage == 0 {
        return;
    }
    if (*self_0).timestamp > level.time {
        return;
    }
    if (*self_0).spawnflags & 16 as libc::c_int != 0 {
        (*self_0).timestamp = level.time + 1 as libc::c_int as libc::c_float;
    } else {
        (*self_0).timestamp = (level.time as libc::c_double + 0.1f64) as libc::c_float;
    }
    if (*self_0).spawnflags & 4 as libc::c_int == 0 {
        if level.framenum % 10 as libc::c_int == 0 as libc::c_int {
            (gi.sound)
                .expect(
                    "non-null function pointer",
                )(
                other,
                0 as libc::c_int,
                (*self_0).noise_index,
                1 as libc::c_int as libc::c_float,
                1 as libc::c_int as libc::c_float,
                0 as libc::c_int as libc::c_float,
            );
        }
    }
    if (*self_0).spawnflags & 8 as libc::c_int != 0 {
        dflags = 0x20 as libc::c_int;
    } else {
        dflags = 0 as libc::c_int;
    }
    T_Damage(
        other,
        self_0,
        self_0,
        vec3_origin.as_mut_ptr(),
        ((*other).s.origin).as_mut_ptr(),
        vec3_origin.as_mut_ptr(),
        (*self_0).dmg,
        (*self_0).dmg,
        dflags,
        31 as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn SP_trigger_hurt(mut self_0: *mut edict_t) {
    InitTrigger(self_0);
    (*self_0)
        .noise_index = (gi.soundindex)
        .expect(
            "non-null function pointer",
        )(
        b"world/electro.wav\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    let ref mut fresh20 = (*self_0).touch;
    *fresh20 = Some(
        hurt_touch
            as unsafe extern "C" fn(
                *mut edict_t,
                *mut edict_t,
                *mut cplane_t,
                *mut csurface_t,
            ) -> (),
    );
    if (*self_0).dmg == 0 {
        (*self_0).dmg = 5 as libc::c_int;
    }
    if (*self_0).spawnflags & 1 as libc::c_int != 0 {
        (*self_0).solid = SOLID_NOT;
    } else {
        (*self_0).solid = SOLID_TRIGGER;
    }
    if (*self_0).spawnflags & 2 as libc::c_int != 0 {
        let ref mut fresh21 = (*self_0).use_0;
        *fresh21 = Some(
            hurt_use
                as unsafe extern "C" fn(*mut edict_t, *mut edict_t, *mut edict_t) -> (),
        );
    }
    (gi.linkentity).expect("non-null function pointer")(self_0);
}
#[no_mangle]
pub unsafe extern "C" fn trigger_gravity_touch(
    mut self_0: *mut edict_t,
    mut other: *mut edict_t,
    mut plane: *mut cplane_t,
    mut surf: *mut csurface_t,
) {
    (*other).gravity = (*self_0).gravity;
}
#[no_mangle]
pub unsafe extern "C" fn SP_trigger_gravity(mut self_0: *mut edict_t) {
    if (st.gravity).is_null() {
        (gi.dprintf)
            .expect(
                "non-null function pointer",
            )(
            b"trigger_gravity without gravity set at %s\n\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            vtos(((*self_0).s.origin).as_mut_ptr()),
        );
        G_FreeEdict(self_0);
        return;
    }
    InitTrigger(self_0);
    (*self_0).gravity = atoi(st.gravity) as libc::c_float;
    let ref mut fresh22 = (*self_0).touch;
    *fresh22 = Some(
        trigger_gravity_touch
            as unsafe extern "C" fn(
                *mut edict_t,
                *mut edict_t,
                *mut cplane_t,
                *mut csurface_t,
            ) -> (),
    );
}
#[no_mangle]
pub unsafe extern "C" fn trigger_monsterjump_touch(
    mut self_0: *mut edict_t,
    mut other: *mut edict_t,
    mut plane: *mut cplane_t,
    mut surf: *mut csurface_t,
) {
    if (*other).flags & (0x1 as libc::c_int | 0x2 as libc::c_int) != 0 {
        return;
    }
    if (*other).svflags & 0x2 as libc::c_int != 0 {
        return;
    }
    if (*other).svflags & 0x4 as libc::c_int == 0 {
        return;
    }
    (*other)
        .velocity[0 as libc::c_int
        as usize] = (*self_0).movedir[0 as libc::c_int as usize] * (*self_0).speed;
    (*other)
        .velocity[1 as libc::c_int
        as usize] = (*self_0).movedir[1 as libc::c_int as usize] * (*self_0).speed;
    if ((*other).groundentity).is_null() {
        return;
    }
    let ref mut fresh23 = (*other).groundentity;
    *fresh23 = 0 as *mut edict_t;
    (*other)
        .velocity[2 as libc::c_int
        as usize] = (*self_0).movedir[2 as libc::c_int as usize];
}
#[no_mangle]
pub unsafe extern "C" fn SP_trigger_monsterjump(mut self_0: *mut edict_t) {
    if (*self_0).speed == 0. {
        (*self_0).speed = 200 as libc::c_int as libc::c_float;
    }
    if st.height == 0 {
        st.height = 200 as libc::c_int;
    }
    if (*self_0).s.angles[1 as libc::c_int as usize] == 0 as libc::c_int as libc::c_float
    {
        (*self_0).s.angles[1 as libc::c_int as usize] = 360 as libc::c_int as vec_t;
    }
    InitTrigger(self_0);
    let ref mut fresh24 = (*self_0).touch;
    *fresh24 = Some(
        trigger_monsterjump_touch
            as unsafe extern "C" fn(
                *mut edict_t,
                *mut edict_t,
                *mut cplane_t,
                *mut csurface_t,
            ) -> (),
    );
    (*self_0).movedir[2 as libc::c_int as usize] = st.height as vec_t;
}
