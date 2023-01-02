#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn rand() -> libc::c_int;
    fn PlayerTrail_PickFirst(self_0: *mut edict_t) -> *mut edict_t;
    fn PlayerTrail_PickNext(self_0: *mut edict_t) -> *mut edict_t;
    fn M_MoveToGoal(ent: *mut edict_t, dist: libc::c_float);
    fn M_walkmove(
        ent: *mut edict_t,
        yaw: libc::c_float,
        dist: libc::c_float,
    ) -> qboolean;
    fn M_ChangeYaw(ent: *mut edict_t);
    fn AttackFinished(self_0: *mut edict_t, time: libc::c_float);
    static mut game: game_locals_t;
    static mut level: level_locals_t;
    static mut gi: game_import_t;
    static mut g_edicts: *mut edict_t;
    static mut coop: *mut cvar_t;
    static mut skill: *mut cvar_t;
    fn G_ProjectSource(
        point: *mut vec_t,
        distance: *mut vec_t,
        forward: *mut vec_t,
        right: *mut vec_t,
        result: *mut vec_t,
    );
    fn G_PickTarget(targetname: *mut libc::c_char) -> *mut edict_t;
    fn G_Spawn() -> *mut edict_t;
    fn G_FreeEdict(e: *mut edict_t);
    fn vtos(v: *mut vec_t) -> *mut libc::c_char;
    fn vectoyaw(vec: *mut vec_t) -> libc::c_float;
    static mut vec3_origin: vec3_t;
    fn VectorLength(v: *mut vec_t) -> vec_t;
    fn VectorNormalize(v: *mut vec_t) -> vec_t;
    fn AngleVectors(
        angles: *mut vec_t,
        forward: *mut vec_t,
        right: *mut vec_t,
        up: *mut vec_t,
    );
    fn anglemod(a: libc::c_float) -> libc::c_float;
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
pub static mut enemy_vis: qboolean = false_0;
#[no_mangle]
pub static mut enemy_infront: qboolean = false_0;
#[no_mangle]
pub static mut enemy_range: libc::c_int = 0;
#[no_mangle]
pub static mut enemy_yaw: libc::c_float = 0.;
#[no_mangle]
pub unsafe extern "C" fn AI_SetSightClient() {
    let mut ent: *mut edict_t = 0 as *mut edict_t;
    let mut start: libc::c_int = 0;
    let mut check: libc::c_int = 0;
    if (level.sight_client).is_null() {
        start = 1 as libc::c_int;
    } else {
        start = (level.sight_client).offset_from(g_edicts) as libc::c_long
            as libc::c_int;
    }
    check = start;
    loop {
        check += 1;
        if check > game.maxclients {
            check = 1 as libc::c_int;
        }
        ent = &mut *g_edicts.offset(check as isize) as *mut edict_t;
        if (*ent).inuse as libc::c_uint != 0 && (*ent).health > 0 as libc::c_int
            && (*ent).flags & 0x20 as libc::c_int == 0
        {
            level.sight_client = ent;
            return;
        }
        if check == start {
            level.sight_client = 0 as *mut edict_t;
            return;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn ai_move(mut self_0: *mut edict_t, mut dist: libc::c_float) {
    M_walkmove(self_0, (*self_0).s.angles[1 as libc::c_int as usize], dist);
}
#[no_mangle]
pub unsafe extern "C" fn ai_stand(mut self_0: *mut edict_t, mut dist: libc::c_float) {
    let mut v: vec3_t = [0.; 3];
    if dist != 0. {
        M_walkmove(self_0, (*self_0).s.angles[1 as libc::c_int as usize], dist);
    }
    if (*self_0).monsterinfo.aiflags & 0x1 as libc::c_int != 0 {
        if !((*self_0).enemy).is_null() {
            v[0 as libc::c_int
                as usize] = (*(*self_0).enemy).s.origin[0 as libc::c_int as usize]
                - (*self_0).s.origin[0 as libc::c_int as usize];
            v[1 as libc::c_int
                as usize] = (*(*self_0).enemy).s.origin[1 as libc::c_int as usize]
                - (*self_0).s.origin[1 as libc::c_int as usize];
            v[2 as libc::c_int
                as usize] = (*(*self_0).enemy).s.origin[2 as libc::c_int as usize]
                - (*self_0).s.origin[2 as libc::c_int as usize];
            (*self_0).ideal_yaw = vectoyaw(v.as_mut_ptr());
            if (*self_0).s.angles[1 as libc::c_int as usize] != (*self_0).ideal_yaw
                && (*self_0).monsterinfo.aiflags & 0x2 as libc::c_int != 0
            {
                (*self_0).monsterinfo.aiflags
                    &= !(0x1 as libc::c_int | 0x2 as libc::c_int);
                ((*self_0).monsterinfo.run).expect("non-null function pointer")(self_0);
            }
            M_ChangeYaw(self_0);
            ai_checkattack(self_0, 0 as libc::c_int as libc::c_float);
        } else {
            FindTarget(self_0);
        }
        return;
    }
    if FindTarget(self_0) as u64 != 0 {
        return;
    }
    if level.time > (*self_0).monsterinfo.pausetime {
        ((*self_0).monsterinfo.walk).expect("non-null function pointer")(self_0);
        return;
    }
    if (*self_0).spawnflags & 1 as libc::c_int == 0
        && ((*self_0).monsterinfo.idle).is_some()
        && level.time > (*self_0).monsterinfo.idle_time
    {
        if (*self_0).monsterinfo.idle_time != 0. {
            ((*self_0).monsterinfo.idle).expect("non-null function pointer")(self_0);
            (*self_0)
                .monsterinfo
                .idle_time = level.time + 15 as libc::c_int as libc::c_float
                + (rand() & 0x7fff as libc::c_int) as libc::c_float
                    / 0x7fff as libc::c_int as libc::c_float
                    * 15 as libc::c_int as libc::c_float;
        } else {
            (*self_0)
                .monsterinfo
                .idle_time = level.time
                + (rand() & 0x7fff as libc::c_int) as libc::c_float
                    / 0x7fff as libc::c_int as libc::c_float
                    * 15 as libc::c_int as libc::c_float;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn ai_walk(mut self_0: *mut edict_t, mut dist: libc::c_float) {
    M_MoveToGoal(self_0, dist);
    if FindTarget(self_0) as u64 != 0 {
        return;
    }
    if ((*self_0).monsterinfo.search).is_some()
        && level.time > (*self_0).monsterinfo.idle_time
    {
        if (*self_0).monsterinfo.idle_time != 0. {
            ((*self_0).monsterinfo.search).expect("non-null function pointer")(self_0);
            (*self_0)
                .monsterinfo
                .idle_time = level.time + 15 as libc::c_int as libc::c_float
                + (rand() & 0x7fff as libc::c_int) as libc::c_float
                    / 0x7fff as libc::c_int as libc::c_float
                    * 15 as libc::c_int as libc::c_float;
        } else {
            (*self_0)
                .monsterinfo
                .idle_time = level.time
                + (rand() & 0x7fff as libc::c_int) as libc::c_float
                    / 0x7fff as libc::c_int as libc::c_float
                    * 15 as libc::c_int as libc::c_float;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn ai_charge(mut self_0: *mut edict_t, mut dist: libc::c_float) {
    let mut v: vec3_t = [0.; 3];
    v[0 as libc::c_int
        as usize] = (*(*self_0).enemy).s.origin[0 as libc::c_int as usize]
        - (*self_0).s.origin[0 as libc::c_int as usize];
    v[1 as libc::c_int
        as usize] = (*(*self_0).enemy).s.origin[1 as libc::c_int as usize]
        - (*self_0).s.origin[1 as libc::c_int as usize];
    v[2 as libc::c_int
        as usize] = (*(*self_0).enemy).s.origin[2 as libc::c_int as usize]
        - (*self_0).s.origin[2 as libc::c_int as usize];
    (*self_0).ideal_yaw = vectoyaw(v.as_mut_ptr());
    M_ChangeYaw(self_0);
    if dist != 0. {
        M_walkmove(self_0, (*self_0).s.angles[1 as libc::c_int as usize], dist);
    }
}
#[no_mangle]
pub unsafe extern "C" fn ai_turn(mut self_0: *mut edict_t, mut dist: libc::c_float) {
    if dist != 0. {
        M_walkmove(self_0, (*self_0).s.angles[1 as libc::c_int as usize], dist);
    }
    if FindTarget(self_0) as u64 != 0 {
        return;
    }
    M_ChangeYaw(self_0);
}
#[no_mangle]
pub unsafe extern "C" fn range(
    mut self_0: *mut edict_t,
    mut other: *mut edict_t,
) -> libc::c_int {
    let mut v: vec3_t = [0.; 3];
    let mut len: libc::c_float = 0.;
    v[0 as libc::c_int
        as usize] = (*self_0).s.origin[0 as libc::c_int as usize]
        - (*other).s.origin[0 as libc::c_int as usize];
    v[1 as libc::c_int
        as usize] = (*self_0).s.origin[1 as libc::c_int as usize]
        - (*other).s.origin[1 as libc::c_int as usize];
    v[2 as libc::c_int
        as usize] = (*self_0).s.origin[2 as libc::c_int as usize]
        - (*other).s.origin[2 as libc::c_int as usize];
    len = VectorLength(v.as_mut_ptr());
    if len < 80 as libc::c_int as libc::c_float {
        return 0 as libc::c_int;
    }
    if len < 500 as libc::c_int as libc::c_float {
        return 1 as libc::c_int;
    }
    if len < 1000 as libc::c_int as libc::c_float {
        return 2 as libc::c_int;
    }
    return 3 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn visible(
    mut self_0: *mut edict_t,
    mut other: *mut edict_t,
) -> qboolean {
    let mut spot1: vec3_t = [0.; 3];
    let mut spot2: vec3_t = [0.; 3];
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
    spot1[0 as libc::c_int as usize] = (*self_0).s.origin[0 as libc::c_int as usize];
    spot1[1 as libc::c_int as usize] = (*self_0).s.origin[1 as libc::c_int as usize];
    spot1[2 as libc::c_int as usize] = (*self_0).s.origin[2 as libc::c_int as usize];
    spot1[2 as libc::c_int as usize] += (*self_0).viewheight as libc::c_float;
    spot2[0 as libc::c_int as usize] = (*other).s.origin[0 as libc::c_int as usize];
    spot2[1 as libc::c_int as usize] = (*other).s.origin[1 as libc::c_int as usize];
    spot2[2 as libc::c_int as usize] = (*other).s.origin[2 as libc::c_int as usize];
    spot2[2 as libc::c_int as usize] += (*other).viewheight as libc::c_float;
    trace = (gi.trace)
        .expect(
            "non-null function pointer",
        )(
        spot1.as_mut_ptr(),
        vec3_origin.as_mut_ptr(),
        vec3_origin.as_mut_ptr(),
        spot2.as_mut_ptr(),
        self_0,
        1 as libc::c_int | 16 as libc::c_int | 8 as libc::c_int,
    );
    if trace.fraction as libc::c_double == 1.0f64 {
        return true_0;
    }
    return false_0;
}
#[no_mangle]
pub unsafe extern "C" fn infront(
    mut self_0: *mut edict_t,
    mut other: *mut edict_t,
) -> qboolean {
    let mut vec: vec3_t = [0.; 3];
    let mut dot: libc::c_float = 0.;
    let mut forward: vec3_t = [0.; 3];
    AngleVectors(
        ((*self_0).s.angles).as_mut_ptr(),
        forward.as_mut_ptr(),
        0 as *mut vec_t,
        0 as *mut vec_t,
    );
    vec[0 as libc::c_int
        as usize] = (*other).s.origin[0 as libc::c_int as usize]
        - (*self_0).s.origin[0 as libc::c_int as usize];
    vec[1 as libc::c_int
        as usize] = (*other).s.origin[1 as libc::c_int as usize]
        - (*self_0).s.origin[1 as libc::c_int as usize];
    vec[2 as libc::c_int
        as usize] = (*other).s.origin[2 as libc::c_int as usize]
        - (*self_0).s.origin[2 as libc::c_int as usize];
    VectorNormalize(vec.as_mut_ptr());
    dot = vec[0 as libc::c_int as usize] * forward[0 as libc::c_int as usize]
        + vec[1 as libc::c_int as usize] * forward[1 as libc::c_int as usize]
        + vec[2 as libc::c_int as usize] * forward[2 as libc::c_int as usize];
    if dot as libc::c_double > 0.3f64 {
        return true_0;
    }
    return false_0;
}
#[no_mangle]
pub unsafe extern "C" fn HuntTarget(mut self_0: *mut edict_t) {
    let mut vec: vec3_t = [0.; 3];
    let ref mut fresh0 = (*self_0).goalentity;
    *fresh0 = (*self_0).enemy;
    if (*self_0).monsterinfo.aiflags & 0x1 as libc::c_int != 0 {
        ((*self_0).monsterinfo.stand).expect("non-null function pointer")(self_0);
    } else {
        ((*self_0).monsterinfo.run).expect("non-null function pointer")(self_0);
    }
    vec[0 as libc::c_int
        as usize] = (*(*self_0).enemy).s.origin[0 as libc::c_int as usize]
        - (*self_0).s.origin[0 as libc::c_int as usize];
    vec[1 as libc::c_int
        as usize] = (*(*self_0).enemy).s.origin[1 as libc::c_int as usize]
        - (*self_0).s.origin[1 as libc::c_int as usize];
    vec[2 as libc::c_int
        as usize] = (*(*self_0).enemy).s.origin[2 as libc::c_int as usize]
        - (*self_0).s.origin[2 as libc::c_int as usize];
    (*self_0).ideal_yaw = vectoyaw(vec.as_mut_ptr());
    if (*self_0).monsterinfo.aiflags & 0x1 as libc::c_int == 0 {
        AttackFinished(self_0, 1 as libc::c_int as libc::c_float);
    }
}
#[no_mangle]
pub unsafe extern "C" fn FoundTarget(mut self_0: *mut edict_t) {
    if !((*(*self_0).enemy).client).is_null() {
        level.sight_entity = self_0;
        level.sight_entity_framenum = level.framenum;
        (*level.sight_entity).light_level = 128 as libc::c_int;
    }
    (*self_0)
        .show_hostile = (level.time + 1 as libc::c_int as libc::c_float) as qboolean;
    (*self_0)
        .monsterinfo
        .last_sighting[0 as libc::c_int
        as usize] = (*(*self_0).enemy).s.origin[0 as libc::c_int as usize];
    (*self_0)
        .monsterinfo
        .last_sighting[1 as libc::c_int
        as usize] = (*(*self_0).enemy).s.origin[1 as libc::c_int as usize];
    (*self_0)
        .monsterinfo
        .last_sighting[2 as libc::c_int
        as usize] = (*(*self_0).enemy).s.origin[2 as libc::c_int as usize];
    (*self_0).monsterinfo.trail_time = level.time;
    if ((*self_0).combattarget).is_null() {
        HuntTarget(self_0);
        return;
    }
    let ref mut fresh1 = (*self_0).movetarget;
    *fresh1 = G_PickTarget((*self_0).combattarget);
    let ref mut fresh2 = (*self_0).goalentity;
    *fresh2 = *fresh1;
    if ((*self_0).movetarget).is_null() {
        let ref mut fresh3 = (*self_0).movetarget;
        *fresh3 = (*self_0).enemy;
        let ref mut fresh4 = (*self_0).goalentity;
        *fresh4 = *fresh3;
        HuntTarget(self_0);
        (gi.dprintf)
            .expect(
                "non-null function pointer",
            )(
            b"%s at %s, combattarget %s not found\n\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            (*self_0).classname,
            vtos(((*self_0).s.origin).as_mut_ptr()),
            (*self_0).combattarget,
        );
        return;
    }
    let ref mut fresh5 = (*self_0).combattarget;
    *fresh5 = 0 as *mut libc::c_char;
    (*self_0).monsterinfo.aiflags |= 0x1000 as libc::c_int;
    let ref mut fresh6 = (*(*self_0).movetarget).targetname;
    *fresh6 = 0 as *mut libc::c_char;
    (*self_0).monsterinfo.pausetime = 0 as libc::c_int as libc::c_float;
    ((*self_0).monsterinfo.run).expect("non-null function pointer")(self_0);
}
#[no_mangle]
pub unsafe extern "C" fn FindTarget(mut self_0: *mut edict_t) -> qboolean {
    let mut client: *mut edict_t = 0 as *mut edict_t;
    let mut heardit: qboolean = false_0;
    let mut r: libc::c_int = 0;
    if (*self_0).monsterinfo.aiflags & 0x100 as libc::c_int != 0 {
        if !((*self_0).goalentity).is_null()
            && (*(*self_0).goalentity).inuse as libc::c_uint != 0
            && !((*(*self_0).goalentity).classname).is_null()
        {
            if strcmp(
                (*(*self_0).goalentity).classname,
                b"target_actor\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
            {
                return false_0;
            }
        }
        return false_0;
    }
    if (*self_0).monsterinfo.aiflags & 0x1000 as libc::c_int != 0 {
        return false_0;
    }
    heardit = false_0;
    if level.sight_entity_framenum >= level.framenum - 1 as libc::c_int
        && (*self_0).spawnflags & 1 as libc::c_int == 0
    {
        client = level.sight_entity;
        if (*client).enemy == (*self_0).enemy {
            return false_0;
        }
    } else if level.sound_entity_framenum >= level.framenum - 1 as libc::c_int {
        client = level.sound_entity;
        heardit = true_0;
    } else if ((*self_0).enemy).is_null()
        && level.sound2_entity_framenum >= level.framenum - 1 as libc::c_int
        && (*self_0).spawnflags & 1 as libc::c_int == 0
    {
        client = level.sound2_entity;
        heardit = true_0;
    } else {
        client = level.sight_client;
        if client.is_null() {
            return false_0;
        }
    }
    if (*client).inuse as u64 == 0 {
        return false_0;
    }
    if client == (*self_0).enemy {
        return true_0;
    }
    if !((*client).client).is_null() {
        if (*client).flags & 0x20 as libc::c_int != 0 {
            return false_0;
        }
    } else if (*client).svflags & 0x4 as libc::c_int != 0 {
        if ((*client).enemy).is_null() {
            return false_0;
        }
        if (*(*client).enemy).flags & 0x20 as libc::c_int != 0 {
            return false_0;
        }
    } else if heardit as u64 != 0 {
        if (*(*client).owner).flags & 0x20 as libc::c_int != 0 {
            return false_0;
        }
    } else {
        return false_0
    }
    if heardit as u64 == 0 {
        r = range(self_0, client);
        if r == 3 as libc::c_int {
            return false_0;
        }
        if (*client).light_level <= 5 as libc::c_int {
            return false_0;
        }
        if visible(self_0, client) as u64 == 0 {
            return false_0;
        }
        if r == 1 as libc::c_int {
            if ((*client).show_hostile as libc::c_uint as libc::c_float) < level.time
                && infront(self_0, client) as u64 == 0
            {
                return false_0;
            }
        } else if r == 2 as libc::c_int {
            if infront(self_0, client) as u64 == 0 {
                return false_0;
            }
        }
        let ref mut fresh7 = (*self_0).enemy;
        *fresh7 = client;
        if strcmp(
            (*(*self_0).enemy).classname,
            b"player_noise\0" as *const u8 as *const libc::c_char,
        ) != 0 as libc::c_int
        {
            (*self_0).monsterinfo.aiflags &= !(0x4 as libc::c_int);
            if ((*(*self_0).enemy).client).is_null() {
                let ref mut fresh8 = (*self_0).enemy;
                *fresh8 = (*(*self_0).enemy).enemy;
                if ((*(*self_0).enemy).client).is_null() {
                    let ref mut fresh9 = (*self_0).enemy;
                    *fresh9 = 0 as *mut edict_t;
                    return false_0;
                }
            }
        }
    } else {
        let mut temp: vec3_t = [0.; 3];
        if (*self_0).spawnflags & 1 as libc::c_int != 0 {
            if visible(self_0, client) as u64 == 0 {
                return false_0;
            }
        } else if (gi.inPHS)
            .expect(
                "non-null function pointer",
            )(((*self_0).s.origin).as_mut_ptr(), ((*client).s.origin).as_mut_ptr())
            as u64 == 0
        {
            return false_0
        }
        temp[0 as libc::c_int
            as usize] = (*client).s.origin[0 as libc::c_int as usize]
            - (*self_0).s.origin[0 as libc::c_int as usize];
        temp[1 as libc::c_int
            as usize] = (*client).s.origin[1 as libc::c_int as usize]
            - (*self_0).s.origin[1 as libc::c_int as usize];
        temp[2 as libc::c_int
            as usize] = (*client).s.origin[2 as libc::c_int as usize]
            - (*self_0).s.origin[2 as libc::c_int as usize];
        if VectorLength(temp.as_mut_ptr()) > 1000 as libc::c_int as libc::c_float {
            return false_0;
        }
        if (*client).areanum != (*self_0).areanum {
            if (gi.AreasConnected)
                .expect(
                    "non-null function pointer",
                )((*self_0).areanum, (*client).areanum) as u64 == 0
            {
                return false_0;
            }
        }
        (*self_0).ideal_yaw = vectoyaw(temp.as_mut_ptr());
        M_ChangeYaw(self_0);
        (*self_0).monsterinfo.aiflags |= 0x4 as libc::c_int;
        let ref mut fresh10 = (*self_0).enemy;
        *fresh10 = client;
    }
    FoundTarget(self_0);
    if (*self_0).monsterinfo.aiflags & 0x4 as libc::c_int == 0
        && ((*self_0).monsterinfo.sight).is_some()
    {
        ((*self_0).monsterinfo.sight)
            .expect("non-null function pointer")(self_0, (*self_0).enemy);
    }
    return true_0;
}
#[no_mangle]
pub unsafe extern "C" fn FacingIdeal(mut self_0: *mut edict_t) -> qboolean {
    let mut delta: libc::c_float = 0.;
    delta = anglemod(
        (*self_0).s.angles[1 as libc::c_int as usize] - (*self_0).ideal_yaw,
    );
    if delta > 45 as libc::c_int as libc::c_float
        && delta < 315 as libc::c_int as libc::c_float
    {
        return false_0;
    }
    return true_0;
}
#[no_mangle]
pub unsafe extern "C" fn M_CheckAttack(mut self_0: *mut edict_t) -> qboolean {
    let mut spot1: vec3_t = [0.; 3];
    let mut spot2: vec3_t = [0.; 3];
    let mut chance: libc::c_float = 0.;
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
    if (*(*self_0).enemy).health > 0 as libc::c_int {
        spot1[0 as libc::c_int as usize] = (*self_0).s.origin[0 as libc::c_int as usize];
        spot1[1 as libc::c_int as usize] = (*self_0).s.origin[1 as libc::c_int as usize];
        spot1[2 as libc::c_int as usize] = (*self_0).s.origin[2 as libc::c_int as usize];
        spot1[2 as libc::c_int as usize] += (*self_0).viewheight as libc::c_float;
        spot2[0 as libc::c_int
            as usize] = (*(*self_0).enemy).s.origin[0 as libc::c_int as usize];
        spot2[1 as libc::c_int
            as usize] = (*(*self_0).enemy).s.origin[1 as libc::c_int as usize];
        spot2[2 as libc::c_int
            as usize] = (*(*self_0).enemy).s.origin[2 as libc::c_int as usize];
        spot2[2 as libc::c_int as usize]
            += (*(*self_0).enemy).viewheight as libc::c_float;
        tr = (gi.trace)
            .expect(
                "non-null function pointer",
            )(
            spot1.as_mut_ptr(),
            0 as *mut vec_t,
            0 as *mut vec_t,
            spot2.as_mut_ptr(),
            self_0,
            1 as libc::c_int | 0x2000000 as libc::c_int | 16 as libc::c_int
                | 8 as libc::c_int | 2 as libc::c_int,
        );
        if tr.ent != (*self_0).enemy {
            return false_0;
        }
    }
    if enemy_range == 0 as libc::c_int {
        if (*skill).value == 0 as libc::c_int as libc::c_float
            && rand() & 3 as libc::c_int != 0
        {
            return false_0;
        }
        if ((*self_0).monsterinfo.melee).is_some() {
            (*self_0).monsterinfo.attack_state = 3 as libc::c_int;
        } else {
            (*self_0).monsterinfo.attack_state = 4 as libc::c_int;
        }
        return true_0;
    }
    if ((*self_0).monsterinfo.attack).is_none() {
        return false_0;
    }
    if level.time < (*self_0).monsterinfo.attack_finished {
        return false_0;
    }
    if enemy_range == 3 as libc::c_int {
        return false_0;
    }
    if (*self_0).monsterinfo.aiflags & 0x1 as libc::c_int != 0 {
        chance = 0.4f64 as libc::c_float;
    } else if enemy_range == 0 as libc::c_int {
        chance = 0.2f64 as libc::c_float;
    } else if enemy_range == 1 as libc::c_int {
        chance = 0.1f64 as libc::c_float;
    } else if enemy_range == 2 as libc::c_int {
        chance = 0.02f64 as libc::c_float;
    } else {
        return false_0
    }
    if (*skill).value == 0 as libc::c_int as libc::c_float {
        chance = (chance as libc::c_double * 0.5f64) as libc::c_float;
    } else if (*skill).value >= 2 as libc::c_int as libc::c_float {
        chance *= 2 as libc::c_int as libc::c_float;
    }
    if ((rand() & 0x7fff as libc::c_int) as libc::c_float
        / 0x7fff as libc::c_int as libc::c_float) < chance
    {
        (*self_0).monsterinfo.attack_state = 4 as libc::c_int;
        (*self_0)
            .monsterinfo
            .attack_finished = level.time
            + 2 as libc::c_int as libc::c_float
                * ((rand() & 0x7fff as libc::c_int) as libc::c_float
                    / 0x7fff as libc::c_int as libc::c_float);
        return true_0;
    }
    if (*self_0).flags & 0x1 as libc::c_int != 0 {
        if (((rand() & 0x7fff as libc::c_int) as libc::c_float
            / 0x7fff as libc::c_int as libc::c_float) as libc::c_double) < 0.3f64
        {
            (*self_0).monsterinfo.attack_state = 2 as libc::c_int;
        } else {
            (*self_0).monsterinfo.attack_state = 1 as libc::c_int;
        }
    }
    return false_0;
}
#[no_mangle]
pub unsafe extern "C" fn ai_run_melee(mut self_0: *mut edict_t) {
    (*self_0).ideal_yaw = enemy_yaw;
    M_ChangeYaw(self_0);
    if FacingIdeal(self_0) as u64 != 0 {
        ((*self_0).monsterinfo.melee).expect("non-null function pointer")(self_0);
        (*self_0).monsterinfo.attack_state = 1 as libc::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ai_run_missile(mut self_0: *mut edict_t) {
    (*self_0).ideal_yaw = enemy_yaw;
    M_ChangeYaw(self_0);
    if FacingIdeal(self_0) as u64 != 0 {
        ((*self_0).monsterinfo.attack).expect("non-null function pointer")(self_0);
        (*self_0).monsterinfo.attack_state = 1 as libc::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ai_run_slide(
    mut self_0: *mut edict_t,
    mut distance: libc::c_float,
) {
    let mut ofs: libc::c_float = 0.;
    (*self_0).ideal_yaw = enemy_yaw;
    M_ChangeYaw(self_0);
    if (*self_0).monsterinfo.lefty != 0 {
        ofs = 90 as libc::c_int as libc::c_float;
    } else {
        ofs = -(90 as libc::c_int) as libc::c_float;
    }
    if M_walkmove(self_0, (*self_0).ideal_yaw + ofs, distance) as u64 != 0 {
        return;
    }
    (*self_0).monsterinfo.lefty = 1 as libc::c_int - (*self_0).monsterinfo.lefty;
    M_walkmove(self_0, (*self_0).ideal_yaw - ofs, distance);
}
#[no_mangle]
pub unsafe extern "C" fn ai_checkattack(
    mut self_0: *mut edict_t,
    mut dist: libc::c_float,
) -> qboolean {
    let mut temp: vec3_t = [0.; 3];
    let mut hesDeadJim: qboolean = false_0;
    if !((*self_0).goalentity).is_null() {
        if (*self_0).monsterinfo.aiflags & 0x1000 as libc::c_int != 0 {
            return false_0;
        }
        if (*self_0).monsterinfo.aiflags & 0x4 as libc::c_int != 0 {
            if (level.time - (*(*self_0).enemy).teleport_time) as libc::c_double > 5.0f64
            {
                if (*self_0).goalentity == (*self_0).enemy {
                    if !((*self_0).movetarget).is_null() {
                        let ref mut fresh11 = (*self_0).goalentity;
                        *fresh11 = (*self_0).movetarget;
                    } else {
                        let ref mut fresh12 = (*self_0).goalentity;
                        *fresh12 = 0 as *mut edict_t;
                    }
                }
                (*self_0).monsterinfo.aiflags &= !(0x4 as libc::c_int);
                if (*self_0).monsterinfo.aiflags & 0x2 as libc::c_int != 0 {
                    (*self_0).monsterinfo.aiflags
                        &= !(0x1 as libc::c_int | 0x2 as libc::c_int);
                }
            } else {
                (*self_0)
                    .show_hostile = (level.time + 1 as libc::c_int as libc::c_float)
                    as qboolean;
                return false_0;
            }
        }
    }
    enemy_vis = false_0;
    hesDeadJim = false_0;
    if ((*self_0).enemy).is_null() || (*(*self_0).enemy).inuse as u64 == 0 {
        hesDeadJim = true_0;
    } else if (*self_0).monsterinfo.aiflags & 0x2000 as libc::c_int != 0 {
        if (*(*self_0).enemy).health > 0 as libc::c_int {
            hesDeadJim = true_0;
            (*self_0).monsterinfo.aiflags &= !(0x2000 as libc::c_int);
        }
    } else if (*self_0).monsterinfo.aiflags & 0x200 as libc::c_int != 0 {
        if (*(*self_0).enemy).health <= -(80 as libc::c_int) {
            hesDeadJim = true_0;
        }
    } else if (*(*self_0).enemy).health <= 0 as libc::c_int {
        hesDeadJim = true_0;
    }
    if hesDeadJim as u64 != 0 {
        let ref mut fresh13 = (*self_0).enemy;
        *fresh13 = 0 as *mut edict_t;
        if !((*self_0).oldenemy).is_null()
            && (*(*self_0).oldenemy).health > 0 as libc::c_int
        {
            let ref mut fresh14 = (*self_0).enemy;
            *fresh14 = (*self_0).oldenemy;
            let ref mut fresh15 = (*self_0).oldenemy;
            *fresh15 = 0 as *mut edict_t;
            HuntTarget(self_0);
        } else {
            if !((*self_0).movetarget).is_null() {
                let ref mut fresh16 = (*self_0).goalentity;
                *fresh16 = (*self_0).movetarget;
                ((*self_0).monsterinfo.walk).expect("non-null function pointer")(self_0);
            } else {
                (*self_0)
                    .monsterinfo
                    .pausetime = level.time + 100000000 as libc::c_int as libc::c_float;
                ((*self_0).monsterinfo.stand)
                    .expect("non-null function pointer")(self_0);
            }
            return true_0;
        }
    }
    (*self_0)
        .show_hostile = (level.time + 1 as libc::c_int as libc::c_float) as qboolean;
    enemy_vis = visible(self_0, (*self_0).enemy);
    if enemy_vis as u64 != 0 {
        (*self_0)
            .monsterinfo
            .search_time = level.time + 5 as libc::c_int as libc::c_float;
        (*self_0)
            .monsterinfo
            .last_sighting[0 as libc::c_int
            as usize] = (*(*self_0).enemy).s.origin[0 as libc::c_int as usize];
        (*self_0)
            .monsterinfo
            .last_sighting[1 as libc::c_int
            as usize] = (*(*self_0).enemy).s.origin[1 as libc::c_int as usize];
        (*self_0)
            .monsterinfo
            .last_sighting[2 as libc::c_int
            as usize] = (*(*self_0).enemy).s.origin[2 as libc::c_int as usize];
    }
    enemy_infront = infront(self_0, (*self_0).enemy);
    enemy_range = range(self_0, (*self_0).enemy);
    temp[0 as libc::c_int
        as usize] = (*(*self_0).enemy).s.origin[0 as libc::c_int as usize]
        - (*self_0).s.origin[0 as libc::c_int as usize];
    temp[1 as libc::c_int
        as usize] = (*(*self_0).enemy).s.origin[1 as libc::c_int as usize]
        - (*self_0).s.origin[1 as libc::c_int as usize];
    temp[2 as libc::c_int
        as usize] = (*(*self_0).enemy).s.origin[2 as libc::c_int as usize]
        - (*self_0).s.origin[2 as libc::c_int as usize];
    enemy_yaw = vectoyaw(temp.as_mut_ptr());
    if (*self_0).monsterinfo.attack_state == 4 as libc::c_int {
        ai_run_missile(self_0);
        return true_0;
    }
    if (*self_0).monsterinfo.attack_state == 3 as libc::c_int {
        ai_run_melee(self_0);
        return true_0;
    }
    if enemy_vis as u64 == 0 {
        return false_0;
    }
    return ((*self_0).monsterinfo.checkattack)
        .expect("non-null function pointer")(self_0);
}
#[no_mangle]
pub unsafe extern "C" fn ai_run(mut self_0: *mut edict_t, mut dist: libc::c_float) {
    let mut v: vec3_t = [0.; 3];
    let mut tempgoal: *mut edict_t = 0 as *mut edict_t;
    let mut save: *mut edict_t = 0 as *mut edict_t;
    let mut new: qboolean = false_0;
    let mut marker: *mut edict_t = 0 as *mut edict_t;
    let mut d1: libc::c_float = 0.;
    let mut d2: libc::c_float = 0.;
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
    let mut v_forward: vec3_t = [0.; 3];
    let mut v_right: vec3_t = [0.; 3];
    let mut left: libc::c_float = 0.;
    let mut center: libc::c_float = 0.;
    let mut right: libc::c_float = 0.;
    let mut left_target: vec3_t = [0.; 3];
    let mut right_target: vec3_t = [0.; 3];
    if (*self_0).monsterinfo.aiflags & 0x1000 as libc::c_int != 0 {
        M_MoveToGoal(self_0, dist);
        return;
    }
    if (*self_0).monsterinfo.aiflags & 0x4 as libc::c_int != 0 {
        v[0 as libc::c_int
            as usize] = (*self_0).s.origin[0 as libc::c_int as usize]
            - (*(*self_0).enemy).s.origin[0 as libc::c_int as usize];
        v[1 as libc::c_int
            as usize] = (*self_0).s.origin[1 as libc::c_int as usize]
            - (*(*self_0).enemy).s.origin[1 as libc::c_int as usize];
        v[2 as libc::c_int
            as usize] = (*self_0).s.origin[2 as libc::c_int as usize]
            - (*(*self_0).enemy).s.origin[2 as libc::c_int as usize];
        if VectorLength(v.as_mut_ptr()) < 64 as libc::c_int as libc::c_float {
            (*self_0).monsterinfo.aiflags |= 0x1 as libc::c_int | 0x2 as libc::c_int;
            ((*self_0).monsterinfo.stand).expect("non-null function pointer")(self_0);
            return;
        }
        M_MoveToGoal(self_0, dist);
        if FindTarget(self_0) as u64 == 0 {
            return;
        }
    }
    if ai_checkattack(self_0, dist) as u64 != 0 {
        return;
    }
    if (*self_0).monsterinfo.attack_state == 2 as libc::c_int {
        ai_run_slide(self_0, dist);
        return;
    }
    if enemy_vis as u64 != 0 {
        M_MoveToGoal(self_0, dist);
        (*self_0).monsterinfo.aiflags &= !(0x8 as libc::c_int);
        (*self_0)
            .monsterinfo
            .last_sighting[0 as libc::c_int
            as usize] = (*(*self_0).enemy).s.origin[0 as libc::c_int as usize];
        (*self_0)
            .monsterinfo
            .last_sighting[1 as libc::c_int
            as usize] = (*(*self_0).enemy).s.origin[1 as libc::c_int as usize];
        (*self_0)
            .monsterinfo
            .last_sighting[2 as libc::c_int
            as usize] = (*(*self_0).enemy).s.origin[2 as libc::c_int as usize];
        (*self_0).monsterinfo.trail_time = level.time;
        return;
    }
    if (*coop).value != 0. {
        if FindTarget(self_0) as u64 != 0 {
            return;
        }
    }
    if (*self_0).monsterinfo.search_time != 0.
        && level.time
            > (*self_0).monsterinfo.search_time + 20 as libc::c_int as libc::c_float
    {
        M_MoveToGoal(self_0, dist);
        (*self_0).monsterinfo.search_time = 0 as libc::c_int as libc::c_float;
        return;
    }
    save = (*self_0).goalentity;
    tempgoal = G_Spawn();
    let ref mut fresh17 = (*self_0).goalentity;
    *fresh17 = tempgoal;
    new = false_0;
    if (*self_0).monsterinfo.aiflags & 0x8 as libc::c_int == 0 {
        (*self_0).monsterinfo.aiflags |= 0x8 as libc::c_int | 0x10 as libc::c_int;
        (*self_0).monsterinfo.aiflags &= !(0x20 as libc::c_int | 0x40 as libc::c_int);
        new = true_0;
    }
    if (*self_0).monsterinfo.aiflags & 0x20 as libc::c_int != 0 {
        (*self_0).monsterinfo.aiflags &= !(0x20 as libc::c_int);
        (*self_0)
            .monsterinfo
            .search_time = level.time + 5 as libc::c_int as libc::c_float;
        if (*self_0).monsterinfo.aiflags & 0x40 as libc::c_int != 0 {
            (*self_0).monsterinfo.aiflags &= !(0x40 as libc::c_int);
            marker = 0 as *mut edict_t;
            (*self_0)
                .monsterinfo
                .last_sighting[0 as libc::c_int
                as usize] = (*self_0).monsterinfo.saved_goal[0 as libc::c_int as usize];
            (*self_0)
                .monsterinfo
                .last_sighting[1 as libc::c_int
                as usize] = (*self_0).monsterinfo.saved_goal[1 as libc::c_int as usize];
            (*self_0)
                .monsterinfo
                .last_sighting[2 as libc::c_int
                as usize] = (*self_0).monsterinfo.saved_goal[2 as libc::c_int as usize];
            new = true_0;
        } else if (*self_0).monsterinfo.aiflags & 0x10 as libc::c_int != 0 {
            (*self_0).monsterinfo.aiflags &= !(0x10 as libc::c_int);
            marker = PlayerTrail_PickFirst(self_0);
        } else {
            marker = PlayerTrail_PickNext(self_0);
        }
        if !marker.is_null() {
            (*self_0)
                .monsterinfo
                .last_sighting[0 as libc::c_int
                as usize] = (*marker).s.origin[0 as libc::c_int as usize];
            (*self_0)
                .monsterinfo
                .last_sighting[1 as libc::c_int
                as usize] = (*marker).s.origin[1 as libc::c_int as usize];
            (*self_0)
                .monsterinfo
                .last_sighting[2 as libc::c_int
                as usize] = (*marker).s.origin[2 as libc::c_int as usize];
            (*self_0).monsterinfo.trail_time = (*marker).timestamp;
            let ref mut fresh18 = (*self_0).ideal_yaw;
            *fresh18 = (*marker).s.angles[1 as libc::c_int as usize];
            (*self_0).s.angles[1 as libc::c_int as usize] = *fresh18;
            new = true_0;
        }
    }
    v[0 as libc::c_int
        as usize] = (*self_0).s.origin[0 as libc::c_int as usize]
        - (*self_0).monsterinfo.last_sighting[0 as libc::c_int as usize];
    v[1 as libc::c_int
        as usize] = (*self_0).s.origin[1 as libc::c_int as usize]
        - (*self_0).monsterinfo.last_sighting[1 as libc::c_int as usize];
    v[2 as libc::c_int
        as usize] = (*self_0).s.origin[2 as libc::c_int as usize]
        - (*self_0).monsterinfo.last_sighting[2 as libc::c_int as usize];
    d1 = VectorLength(v.as_mut_ptr());
    if d1 <= dist {
        (*self_0).monsterinfo.aiflags |= 0x20 as libc::c_int;
        dist = d1;
    }
    (*(*self_0).goalentity)
        .s
        .origin[0 as libc::c_int
        as usize] = (*self_0).monsterinfo.last_sighting[0 as libc::c_int as usize];
    (*(*self_0).goalentity)
        .s
        .origin[1 as libc::c_int
        as usize] = (*self_0).monsterinfo.last_sighting[1 as libc::c_int as usize];
    (*(*self_0).goalentity)
        .s
        .origin[2 as libc::c_int
        as usize] = (*self_0).monsterinfo.last_sighting[2 as libc::c_int as usize];
    if new as u64 != 0 {
        tr = (gi.trace)
            .expect(
                "non-null function pointer",
            )(
            ((*self_0).s.origin).as_mut_ptr(),
            ((*self_0).mins).as_mut_ptr(),
            ((*self_0).maxs).as_mut_ptr(),
            ((*self_0).monsterinfo.last_sighting).as_mut_ptr(),
            self_0,
            1 as libc::c_int | 0x10000 as libc::c_int | 2 as libc::c_int
                | 0x2000000 as libc::c_int,
        );
        if tr.fraction < 1 as libc::c_int as libc::c_float {
            v[0 as libc::c_int
                as usize] = (*(*self_0).goalentity).s.origin[0 as libc::c_int as usize]
                - (*self_0).s.origin[0 as libc::c_int as usize];
            v[1 as libc::c_int
                as usize] = (*(*self_0).goalentity).s.origin[1 as libc::c_int as usize]
                - (*self_0).s.origin[1 as libc::c_int as usize];
            v[2 as libc::c_int
                as usize] = (*(*self_0).goalentity).s.origin[2 as libc::c_int as usize]
                - (*self_0).s.origin[2 as libc::c_int as usize];
            d1 = VectorLength(v.as_mut_ptr());
            center = tr.fraction;
            d2 = d1
                * ((center + 1 as libc::c_int as libc::c_float)
                    / 2 as libc::c_int as libc::c_float);
            let ref mut fresh19 = (*self_0).ideal_yaw;
            *fresh19 = vectoyaw(v.as_mut_ptr());
            (*self_0).s.angles[1 as libc::c_int as usize] = *fresh19;
            AngleVectors(
                ((*self_0).s.angles).as_mut_ptr(),
                v_forward.as_mut_ptr(),
                v_right.as_mut_ptr(),
                0 as *mut vec_t,
            );
            v[0 as libc::c_int as usize] = d2;
            v[1 as libc::c_int as usize] = -(16 as libc::c_int) as vec_t;
            v[2 as libc::c_int as usize] = 0 as libc::c_int as vec_t;
            G_ProjectSource(
                ((*self_0).s.origin).as_mut_ptr(),
                v.as_mut_ptr(),
                v_forward.as_mut_ptr(),
                v_right.as_mut_ptr(),
                left_target.as_mut_ptr(),
            );
            tr = (gi.trace)
                .expect(
                    "non-null function pointer",
                )(
                ((*self_0).s.origin).as_mut_ptr(),
                ((*self_0).mins).as_mut_ptr(),
                ((*self_0).maxs).as_mut_ptr(),
                left_target.as_mut_ptr(),
                self_0,
                1 as libc::c_int | 0x10000 as libc::c_int | 2 as libc::c_int
                    | 0x2000000 as libc::c_int,
            );
            left = tr.fraction;
            v[0 as libc::c_int as usize] = d2;
            v[1 as libc::c_int as usize] = 16 as libc::c_int as vec_t;
            v[2 as libc::c_int as usize] = 0 as libc::c_int as vec_t;
            G_ProjectSource(
                ((*self_0).s.origin).as_mut_ptr(),
                v.as_mut_ptr(),
                v_forward.as_mut_ptr(),
                v_right.as_mut_ptr(),
                right_target.as_mut_ptr(),
            );
            tr = (gi.trace)
                .expect(
                    "non-null function pointer",
                )(
                ((*self_0).s.origin).as_mut_ptr(),
                ((*self_0).mins).as_mut_ptr(),
                ((*self_0).maxs).as_mut_ptr(),
                right_target.as_mut_ptr(),
                self_0,
                1 as libc::c_int | 0x10000 as libc::c_int | 2 as libc::c_int
                    | 0x2000000 as libc::c_int,
            );
            right = tr.fraction;
            center = d1 * center / d2;
            if left >= center && left > right {
                if left < 1 as libc::c_int as libc::c_float {
                    v[0 as libc::c_int
                        as usize] = ((d2 * left) as libc::c_double * 0.5f64) as vec_t;
                    v[1 as libc::c_int as usize] = -(16 as libc::c_int) as vec_t;
                    v[2 as libc::c_int as usize] = 0 as libc::c_int as vec_t;
                    G_ProjectSource(
                        ((*self_0).s.origin).as_mut_ptr(),
                        v.as_mut_ptr(),
                        v_forward.as_mut_ptr(),
                        v_right.as_mut_ptr(),
                        left_target.as_mut_ptr(),
                    );
                }
                (*self_0)
                    .monsterinfo
                    .saved_goal[0 as libc::c_int
                    as usize] = (*self_0)
                    .monsterinfo
                    .last_sighting[0 as libc::c_int as usize];
                (*self_0)
                    .monsterinfo
                    .saved_goal[1 as libc::c_int
                    as usize] = (*self_0)
                    .monsterinfo
                    .last_sighting[1 as libc::c_int as usize];
                (*self_0)
                    .monsterinfo
                    .saved_goal[2 as libc::c_int
                    as usize] = (*self_0)
                    .monsterinfo
                    .last_sighting[2 as libc::c_int as usize];
                (*self_0).monsterinfo.aiflags |= 0x40 as libc::c_int;
                (*(*self_0).goalentity)
                    .s
                    .origin[0 as libc::c_int
                    as usize] = left_target[0 as libc::c_int as usize];
                (*(*self_0).goalentity)
                    .s
                    .origin[1 as libc::c_int
                    as usize] = left_target[1 as libc::c_int as usize];
                (*(*self_0).goalentity)
                    .s
                    .origin[2 as libc::c_int
                    as usize] = left_target[2 as libc::c_int as usize];
                (*self_0)
                    .monsterinfo
                    .last_sighting[0 as libc::c_int
                    as usize] = left_target[0 as libc::c_int as usize];
                (*self_0)
                    .monsterinfo
                    .last_sighting[1 as libc::c_int
                    as usize] = left_target[1 as libc::c_int as usize];
                (*self_0)
                    .monsterinfo
                    .last_sighting[2 as libc::c_int
                    as usize] = left_target[2 as libc::c_int as usize];
                v[0 as libc::c_int
                    as usize] = (*(*self_0).goalentity)
                    .s
                    .origin[0 as libc::c_int as usize]
                    - (*self_0).s.origin[0 as libc::c_int as usize];
                v[1 as libc::c_int
                    as usize] = (*(*self_0).goalentity)
                    .s
                    .origin[1 as libc::c_int as usize]
                    - (*self_0).s.origin[1 as libc::c_int as usize];
                v[2 as libc::c_int
                    as usize] = (*(*self_0).goalentity)
                    .s
                    .origin[2 as libc::c_int as usize]
                    - (*self_0).s.origin[2 as libc::c_int as usize];
                let ref mut fresh20 = (*self_0).ideal_yaw;
                *fresh20 = vectoyaw(v.as_mut_ptr());
                (*self_0).s.angles[1 as libc::c_int as usize] = *fresh20;
            } else if right >= center && right > left {
                if right < 1 as libc::c_int as libc::c_float {
                    v[0 as libc::c_int
                        as usize] = ((d2 * right) as libc::c_double * 0.5f64) as vec_t;
                    v[1 as libc::c_int as usize] = 16 as libc::c_int as vec_t;
                    v[2 as libc::c_int as usize] = 0 as libc::c_int as vec_t;
                    G_ProjectSource(
                        ((*self_0).s.origin).as_mut_ptr(),
                        v.as_mut_ptr(),
                        v_forward.as_mut_ptr(),
                        v_right.as_mut_ptr(),
                        right_target.as_mut_ptr(),
                    );
                }
                (*self_0)
                    .monsterinfo
                    .saved_goal[0 as libc::c_int
                    as usize] = (*self_0)
                    .monsterinfo
                    .last_sighting[0 as libc::c_int as usize];
                (*self_0)
                    .monsterinfo
                    .saved_goal[1 as libc::c_int
                    as usize] = (*self_0)
                    .monsterinfo
                    .last_sighting[1 as libc::c_int as usize];
                (*self_0)
                    .monsterinfo
                    .saved_goal[2 as libc::c_int
                    as usize] = (*self_0)
                    .monsterinfo
                    .last_sighting[2 as libc::c_int as usize];
                (*self_0).monsterinfo.aiflags |= 0x40 as libc::c_int;
                (*(*self_0).goalentity)
                    .s
                    .origin[0 as libc::c_int
                    as usize] = right_target[0 as libc::c_int as usize];
                (*(*self_0).goalentity)
                    .s
                    .origin[1 as libc::c_int
                    as usize] = right_target[1 as libc::c_int as usize];
                (*(*self_0).goalentity)
                    .s
                    .origin[2 as libc::c_int
                    as usize] = right_target[2 as libc::c_int as usize];
                (*self_0)
                    .monsterinfo
                    .last_sighting[0 as libc::c_int
                    as usize] = right_target[0 as libc::c_int as usize];
                (*self_0)
                    .monsterinfo
                    .last_sighting[1 as libc::c_int
                    as usize] = right_target[1 as libc::c_int as usize];
                (*self_0)
                    .monsterinfo
                    .last_sighting[2 as libc::c_int
                    as usize] = right_target[2 as libc::c_int as usize];
                v[0 as libc::c_int
                    as usize] = (*(*self_0).goalentity)
                    .s
                    .origin[0 as libc::c_int as usize]
                    - (*self_0).s.origin[0 as libc::c_int as usize];
                v[1 as libc::c_int
                    as usize] = (*(*self_0).goalentity)
                    .s
                    .origin[1 as libc::c_int as usize]
                    - (*self_0).s.origin[1 as libc::c_int as usize];
                v[2 as libc::c_int
                    as usize] = (*(*self_0).goalentity)
                    .s
                    .origin[2 as libc::c_int as usize]
                    - (*self_0).s.origin[2 as libc::c_int as usize];
                let ref mut fresh21 = (*self_0).ideal_yaw;
                *fresh21 = vectoyaw(v.as_mut_ptr());
                (*self_0).s.angles[1 as libc::c_int as usize] = *fresh21;
            }
        }
    }
    M_MoveToGoal(self_0, dist);
    G_FreeEdict(tempgoal);
    if !self_0.is_null() {
        let ref mut fresh22 = (*self_0).goalentity;
        *fresh22 = save;
    }
}
