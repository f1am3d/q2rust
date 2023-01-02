#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    fn cos(_: libc::c_double) -> libc::c_double;
    fn sin(_: libc::c_double) -> libc::c_double;
    fn tan(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn fire_rocket(
        self_0: *mut edict_t,
        start: *mut vec_t,
        dir: *mut vec_t,
        damage: libc::c_int,
        speed: libc::c_int,
        damage_radius: libc::c_float,
        radius_damage: libc::c_int,
    );
    fn visible(self_0: *mut edict_t, other: *mut edict_t) -> qboolean;
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
    fn vectoangles(vec: *mut vec_t, angles: *mut vec_t);
    fn vtos(v: *mut vec_t) -> *mut libc::c_char;
    fn G_FreeEdict(e: *mut edict_t);
    fn FindItemByClassname(classname: *mut libc::c_char) -> *mut gitem_t;
    static mut skill: *mut cvar_t;
    static mut deathmatch: *mut cvar_t;
    static mut st: spawn_temp_t;
    static mut gi: game_import_t;
    static mut level: level_locals_t;
    fn G_PickTarget(targetname: *mut libc::c_char) -> *mut edict_t;
    fn rand() -> libc::c_int;
    static mut vec3_origin: vec3_t;
    fn VectorMA(
        veca: *mut vec_t,
        scale: libc::c_float,
        vecb: *mut vec_t,
        vecc: *mut vec_t,
    );
    fn VectorLength(v: *mut vec_t) -> vec_t;
    fn VectorScale(in_0: *mut vec_t, scale: vec_t, out: *mut vec_t);
    fn AngleVectors(
        angles: *mut vec_t,
        forward: *mut vec_t,
        right: *mut vec_t,
        up: *mut vec_t,
    );
    fn infantry_die(
        self_0: *mut edict_t,
        inflictor: *mut edict_t,
        attacker: *mut edict_t,
        damage: libc::c_int,
    );
    fn infantry_stand(self_0: *mut edict_t);
    fn monster_use(self_0: *mut edict_t, other: *mut edict_t, activator: *mut edict_t);
    fn FindTarget(self_0: *mut edict_t) -> qboolean;
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
pub type C2RustUnnamed = libc::c_uint;
pub const DAMAGE_AIM: C2RustUnnamed = 2;
pub const DAMAGE_YES: C2RustUnnamed = 1;
pub const DAMAGE_NO: C2RustUnnamed = 0;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const MOVETYPE_BOUNCE: C2RustUnnamed_0 = 9;
pub const MOVETYPE_FLYMISSILE: C2RustUnnamed_0 = 8;
pub const MOVETYPE_TOSS: C2RustUnnamed_0 = 7;
pub const MOVETYPE_FLY: C2RustUnnamed_0 = 6;
pub const MOVETYPE_STEP: C2RustUnnamed_0 = 5;
pub const MOVETYPE_WALK: C2RustUnnamed_0 = 4;
pub const MOVETYPE_STOP: C2RustUnnamed_0 = 3;
pub const MOVETYPE_PUSH: C2RustUnnamed_0 = 2;
pub const MOVETYPE_NOCLIP: C2RustUnnamed_0 = 1;
pub const MOVETYPE_NONE: C2RustUnnamed_0 = 0;
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
pub unsafe extern "C" fn AnglesNormalize(mut vec: *mut vec_t) {
    while *vec.offset(0 as libc::c_int as isize) > 360 as libc::c_int as libc::c_float {
        let ref mut fresh0 = *vec.offset(0 as libc::c_int as isize);
        *fresh0 -= 360 as libc::c_int as libc::c_float;
    }
    while *vec.offset(0 as libc::c_int as isize) < 0 as libc::c_int as libc::c_float {
        let ref mut fresh1 = *vec.offset(0 as libc::c_int as isize);
        *fresh1 += 360 as libc::c_int as libc::c_float;
    }
    while *vec.offset(1 as libc::c_int as isize) > 360 as libc::c_int as libc::c_float {
        let ref mut fresh2 = *vec.offset(1 as libc::c_int as isize);
        *fresh2 -= 360 as libc::c_int as libc::c_float;
    }
    while *vec.offset(1 as libc::c_int as isize) < 0 as libc::c_int as libc::c_float {
        let ref mut fresh3 = *vec.offset(1 as libc::c_int as isize);
        *fresh3 += 360 as libc::c_int as libc::c_float;
    }
}
#[no_mangle]
pub unsafe extern "C" fn SnapToEights(mut x: libc::c_float) -> libc::c_float {
    x = (x as libc::c_double * 8.0f64) as libc::c_float;
    if x as libc::c_double > 0.0f64 {
        x = (x as libc::c_double + 0.5f64) as libc::c_float;
    } else {
        x = (x as libc::c_double - 0.5f64) as libc::c_float;
    }
    return (0.125f64 * x as libc::c_int as libc::c_double) as libc::c_float;
}
#[no_mangle]
pub unsafe extern "C" fn turret_blocked(
    mut self_0: *mut edict_t,
    mut other: *mut edict_t,
) {
    let mut attacker: *mut edict_t = 0 as *mut edict_t;
    if (*other).takedamage != 0 {
        if !((*(*self_0).teammaster).owner).is_null() {
            attacker = (*(*self_0).teammaster).owner;
        } else {
            attacker = (*self_0).teammaster;
        }
        T_Damage(
            other,
            self_0,
            attacker,
            vec3_origin.as_mut_ptr(),
            ((*other).s.origin).as_mut_ptr(),
            vec3_origin.as_mut_ptr(),
            (*(*self_0).teammaster).dmg,
            10 as libc::c_int,
            0 as libc::c_int,
            20 as libc::c_int,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn turret_breach_fire(mut self_0: *mut edict_t) {
    let mut f: vec3_t = [0.; 3];
    let mut r: vec3_t = [0.; 3];
    let mut u: vec3_t = [0.; 3];
    let mut start: vec3_t = [0.; 3];
    let mut damage: libc::c_int = 0;
    let mut speed: libc::c_int = 0;
    AngleVectors(
        ((*self_0).s.angles).as_mut_ptr(),
        f.as_mut_ptr(),
        r.as_mut_ptr(),
        u.as_mut_ptr(),
    );
    VectorMA(
        ((*self_0).s.origin).as_mut_ptr(),
        (*self_0).move_origin[0 as libc::c_int as usize],
        f.as_mut_ptr(),
        start.as_mut_ptr(),
    );
    VectorMA(
        start.as_mut_ptr(),
        (*self_0).move_origin[1 as libc::c_int as usize],
        r.as_mut_ptr(),
        start.as_mut_ptr(),
    );
    VectorMA(
        start.as_mut_ptr(),
        (*self_0).move_origin[2 as libc::c_int as usize],
        u.as_mut_ptr(),
        start.as_mut_ptr(),
    );
    damage = (100 as libc::c_int as libc::c_float
        + (rand() & 0x7fff as libc::c_int) as libc::c_float
            / 0x7fff as libc::c_int as libc::c_float
            * 50 as libc::c_int as libc::c_float) as libc::c_int;
    speed = (550 as libc::c_int as libc::c_float
        + 50 as libc::c_int as libc::c_float * (*skill).value) as libc::c_int;
    fire_rocket(
        (*(*self_0).teammaster).owner,
        start.as_mut_ptr(),
        f.as_mut_ptr(),
        damage,
        speed,
        150 as libc::c_int as libc::c_float,
        damage,
    );
    (gi.positioned_sound)
        .expect(
            "non-null function pointer",
        )(
        start.as_mut_ptr(),
        self_0,
        1 as libc::c_int,
        (gi.soundindex)
            .expect(
                "non-null function pointer",
            )(
            b"weapons/rocklf1a.wav\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        ),
        1 as libc::c_int as libc::c_float,
        1 as libc::c_int as libc::c_float,
        0 as libc::c_int as libc::c_float,
    );
}
#[no_mangle]
pub unsafe extern "C" fn turret_breach_think(mut self_0: *mut edict_t) {
    let mut ent: *mut edict_t = 0 as *mut edict_t;
    let mut current_angles: vec3_t = [0.; 3];
    let mut delta: vec3_t = [0.; 3];
    current_angles[0 as libc::c_int
        as usize] = (*self_0).s.angles[0 as libc::c_int as usize];
    current_angles[1 as libc::c_int
        as usize] = (*self_0).s.angles[1 as libc::c_int as usize];
    current_angles[2 as libc::c_int
        as usize] = (*self_0).s.angles[2 as libc::c_int as usize];
    AnglesNormalize(current_angles.as_mut_ptr());
    AnglesNormalize(((*self_0).move_angles).as_mut_ptr());
    if (*self_0).move_angles[0 as libc::c_int as usize]
        > 180 as libc::c_int as libc::c_float
    {
        let ref mut fresh4 = (*self_0).move_angles[0 as libc::c_int as usize];
        *fresh4 -= 360 as libc::c_int as libc::c_float;
    }
    if (*self_0).move_angles[0 as libc::c_int as usize]
        > (*self_0).pos1[0 as libc::c_int as usize]
    {
        (*self_0)
            .move_angles[0 as libc::c_int
            as usize] = (*self_0).pos1[0 as libc::c_int as usize];
    } else if (*self_0).move_angles[0 as libc::c_int as usize]
        < (*self_0).pos2[0 as libc::c_int as usize]
    {
        (*self_0)
            .move_angles[0 as libc::c_int
            as usize] = (*self_0).pos2[0 as libc::c_int as usize];
    }
    if (*self_0).move_angles[1 as libc::c_int as usize]
        < (*self_0).pos1[1 as libc::c_int as usize]
        || (*self_0).move_angles[1 as libc::c_int as usize]
            > (*self_0).pos2[1 as libc::c_int as usize]
    {
        let mut dmin: libc::c_float = 0.;
        let mut dmax: libc::c_float = 0.;
        dmin = fabs(
            ((*self_0).pos1[1 as libc::c_int as usize]
                - (*self_0).move_angles[1 as libc::c_int as usize]) as libc::c_double,
        ) as libc::c_float;
        if dmin < -(180 as libc::c_int) as libc::c_float {
            dmin += 360 as libc::c_int as libc::c_float;
        } else if dmin > 180 as libc::c_int as libc::c_float {
            dmin -= 360 as libc::c_int as libc::c_float;
        }
        dmax = fabs(
            ((*self_0).pos2[1 as libc::c_int as usize]
                - (*self_0).move_angles[1 as libc::c_int as usize]) as libc::c_double,
        ) as libc::c_float;
        if dmax < -(180 as libc::c_int) as libc::c_float {
            dmax += 360 as libc::c_int as libc::c_float;
        } else if dmax > 180 as libc::c_int as libc::c_float {
            dmax -= 360 as libc::c_int as libc::c_float;
        }
        if fabs(dmin as libc::c_double) < fabs(dmax as libc::c_double) {
            (*self_0)
                .move_angles[1 as libc::c_int
                as usize] = (*self_0).pos1[1 as libc::c_int as usize];
        } else {
            (*self_0)
                .move_angles[1 as libc::c_int
                as usize] = (*self_0).pos2[1 as libc::c_int as usize];
        }
    }
    delta[0 as libc::c_int
        as usize] = (*self_0).move_angles[0 as libc::c_int as usize]
        - current_angles[0 as libc::c_int as usize];
    delta[1 as libc::c_int
        as usize] = (*self_0).move_angles[1 as libc::c_int as usize]
        - current_angles[1 as libc::c_int as usize];
    delta[2 as libc::c_int
        as usize] = (*self_0).move_angles[2 as libc::c_int as usize]
        - current_angles[2 as libc::c_int as usize];
    if delta[0 as libc::c_int as usize] < -(180 as libc::c_int) as libc::c_float {
        delta[0 as libc::c_int as usize] += 360 as libc::c_int as libc::c_float;
    } else if delta[0 as libc::c_int as usize] > 180 as libc::c_int as libc::c_float {
        delta[0 as libc::c_int as usize] -= 360 as libc::c_int as libc::c_float;
    }
    if delta[1 as libc::c_int as usize] < -(180 as libc::c_int) as libc::c_float {
        delta[1 as libc::c_int as usize] += 360 as libc::c_int as libc::c_float;
    } else if delta[1 as libc::c_int as usize] > 180 as libc::c_int as libc::c_float {
        delta[1 as libc::c_int as usize] -= 360 as libc::c_int as libc::c_float;
    }
    delta[2 as libc::c_int as usize] = 0 as libc::c_int as vec_t;
    if delta[0 as libc::c_int as usize] as libc::c_double
        > (*self_0).speed as libc::c_double * 0.1f64
    {
        delta[0 as libc::c_int
            as usize] = ((*self_0).speed as libc::c_double * 0.1f64) as vec_t;
    }
    if (delta[0 as libc::c_int as usize] as libc::c_double)
        < (-(1 as libc::c_int) as libc::c_float * (*self_0).speed) as libc::c_double
            * 0.1f64
    {
        delta[0 as libc::c_int
            as usize] = ((-(1 as libc::c_int) as libc::c_float * (*self_0).speed)
            as libc::c_double * 0.1f64) as vec_t;
    }
    if delta[1 as libc::c_int as usize] as libc::c_double
        > (*self_0).speed as libc::c_double * 0.1f64
    {
        delta[1 as libc::c_int
            as usize] = ((*self_0).speed as libc::c_double * 0.1f64) as vec_t;
    }
    if (delta[1 as libc::c_int as usize] as libc::c_double)
        < (-(1 as libc::c_int) as libc::c_float * (*self_0).speed) as libc::c_double
            * 0.1f64
    {
        delta[1 as libc::c_int
            as usize] = ((-(1 as libc::c_int) as libc::c_float * (*self_0).speed)
            as libc::c_double * 0.1f64) as vec_t;
    }
    VectorScale(
        delta.as_mut_ptr(),
        (1.0f64 / 0.1f64) as vec_t,
        ((*self_0).avelocity).as_mut_ptr(),
    );
    (*self_0).nextthink = (level.time as libc::c_double + 0.1f64) as libc::c_float;
    ent = (*self_0).teammaster;
    while !ent.is_null() {
        (*ent)
            .avelocity[1 as libc::c_int
            as usize] = (*self_0).avelocity[1 as libc::c_int as usize];
        ent = (*ent).teamchain;
    }
    if !((*self_0).owner).is_null() {
        let mut angle: libc::c_float = 0.;
        let mut target_z: libc::c_float = 0.;
        let mut diff: libc::c_float = 0.;
        let mut target: vec3_t = [0.; 3];
        let mut dir: vec3_t = [0.; 3];
        (*(*self_0).owner)
            .avelocity[0 as libc::c_int
            as usize] = (*self_0).avelocity[0 as libc::c_int as usize];
        (*(*self_0).owner)
            .avelocity[1 as libc::c_int
            as usize] = (*self_0).avelocity[1 as libc::c_int as usize];
        angle = (*self_0).s.angles[1 as libc::c_int as usize]
            + (*(*self_0).owner).move_origin[1 as libc::c_int as usize];
        angle = (angle as libc::c_double
            * (3.14159265358979323846f64 * 2 as libc::c_int as libc::c_double
                / 360 as libc::c_int as libc::c_double)) as libc::c_float;
        target[0 as libc::c_int
            as usize] = SnapToEights(
            ((*self_0).s.origin[0 as libc::c_int as usize] as libc::c_double
                + cos(angle as libc::c_double)
                    * (*(*self_0).owner).move_origin[0 as libc::c_int as usize]
                        as libc::c_double) as libc::c_float,
        );
        target[1 as libc::c_int
            as usize] = SnapToEights(
            ((*self_0).s.origin[1 as libc::c_int as usize] as libc::c_double
                + sin(angle as libc::c_double)
                    * (*(*self_0).owner).move_origin[0 as libc::c_int as usize]
                        as libc::c_double) as libc::c_float,
        );
        target[2 as libc::c_int
            as usize] = (*(*self_0).owner).s.origin[2 as libc::c_int as usize];
        dir[0 as libc::c_int
            as usize] = target[0 as libc::c_int as usize]
            - (*(*self_0).owner).s.origin[0 as libc::c_int as usize];
        dir[1 as libc::c_int
            as usize] = target[1 as libc::c_int as usize]
            - (*(*self_0).owner).s.origin[1 as libc::c_int as usize];
        dir[2 as libc::c_int
            as usize] = target[2 as libc::c_int as usize]
            - (*(*self_0).owner).s.origin[2 as libc::c_int as usize];
        (*(*self_0).owner)
            .velocity[0 as libc::c_int
            as usize] = (dir[0 as libc::c_int as usize] as libc::c_double * 1.0f64
            / 0.1f64) as vec_t;
        (*(*self_0).owner)
            .velocity[1 as libc::c_int
            as usize] = (dir[1 as libc::c_int as usize] as libc::c_double * 1.0f64
            / 0.1f64) as vec_t;
        angle = ((*self_0).s.angles[0 as libc::c_int as usize] as libc::c_double
            * (3.14159265358979323846f64 * 2 as libc::c_int as libc::c_double
                / 360 as libc::c_int as libc::c_double)) as libc::c_float;
        target_z = SnapToEights(
            ((*self_0).s.origin[2 as libc::c_int as usize] as libc::c_double
                + (*(*self_0).owner).move_origin[0 as libc::c_int as usize]
                    as libc::c_double * tan(angle as libc::c_double)
                + (*(*self_0).owner).move_origin[2 as libc::c_int as usize]
                    as libc::c_double) as libc::c_float,
        );
        diff = target_z - (*(*self_0).owner).s.origin[2 as libc::c_int as usize];
        (*(*self_0).owner)
            .velocity[2 as libc::c_int
            as usize] = (diff as libc::c_double * 1.0f64 / 0.1f64) as vec_t;
        if (*self_0).spawnflags & 65536 as libc::c_int != 0 {
            turret_breach_fire(self_0);
            (*self_0).spawnflags &= !(65536 as libc::c_int);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn turret_breach_finish_init(mut self_0: *mut edict_t) {
    if ((*self_0).target).is_null() {
        (gi.dprintf)
            .expect(
                "non-null function pointer",
            )(
            b"%s at %s needs a target\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            (*self_0).classname,
            vtos(((*self_0).s.origin).as_mut_ptr()),
        );
    } else {
        let ref mut fresh5 = (*self_0).target_ent;
        *fresh5 = G_PickTarget((*self_0).target);
        (*self_0)
            .move_origin[0 as libc::c_int
            as usize] = (*(*self_0).target_ent).s.origin[0 as libc::c_int as usize]
            - (*self_0).s.origin[0 as libc::c_int as usize];
        (*self_0)
            .move_origin[1 as libc::c_int
            as usize] = (*(*self_0).target_ent).s.origin[1 as libc::c_int as usize]
            - (*self_0).s.origin[1 as libc::c_int as usize];
        (*self_0)
            .move_origin[2 as libc::c_int
            as usize] = (*(*self_0).target_ent).s.origin[2 as libc::c_int as usize]
            - (*self_0).s.origin[2 as libc::c_int as usize];
        G_FreeEdict((*self_0).target_ent);
    }
    (*(*self_0).teammaster).dmg = (*self_0).dmg;
    let ref mut fresh6 = (*self_0).think;
    *fresh6 = Some(turret_breach_think as unsafe extern "C" fn(*mut edict_t) -> ());
    ((*self_0).think).expect("non-null function pointer")(self_0);
}
#[no_mangle]
pub unsafe extern "C" fn SP_turret_breach(mut self_0: *mut edict_t) {
    (*self_0).solid = SOLID_BSP;
    (*self_0).movetype = MOVETYPE_PUSH as libc::c_int;
    (gi.setmodel).expect("non-null function pointer")(self_0, (*self_0).model);
    if (*self_0).speed == 0. {
        (*self_0).speed = 50 as libc::c_int as libc::c_float;
    }
    if (*self_0).dmg == 0 {
        (*self_0).dmg = 10 as libc::c_int;
    }
    if st.minpitch == 0. {
        st.minpitch = -(30 as libc::c_int) as libc::c_float;
    }
    if st.maxpitch == 0. {
        st.maxpitch = 30 as libc::c_int as libc::c_float;
    }
    if st.maxyaw == 0. {
        st.maxyaw = 360 as libc::c_int as libc::c_float;
    }
    (*self_0)
        .pos1[0 as libc::c_int
        as usize] = -(1 as libc::c_int) as libc::c_float * st.minpitch;
    (*self_0).pos1[1 as libc::c_int as usize] = st.minyaw;
    (*self_0)
        .pos2[0 as libc::c_int
        as usize] = -(1 as libc::c_int) as libc::c_float * st.maxpitch;
    (*self_0).pos2[1 as libc::c_int as usize] = st.maxyaw;
    (*self_0).ideal_yaw = (*self_0).s.angles[1 as libc::c_int as usize];
    (*self_0).move_angles[1 as libc::c_int as usize] = (*self_0).ideal_yaw;
    let ref mut fresh7 = (*self_0).blocked;
    *fresh7 = Some(
        turret_blocked as unsafe extern "C" fn(*mut edict_t, *mut edict_t) -> (),
    );
    let ref mut fresh8 = (*self_0).think;
    *fresh8 = Some(
        turret_breach_finish_init as unsafe extern "C" fn(*mut edict_t) -> (),
    );
    (*self_0).nextthink = (level.time as libc::c_double + 0.1f64) as libc::c_float;
    (gi.linkentity).expect("non-null function pointer")(self_0);
}
#[no_mangle]
pub unsafe extern "C" fn SP_turret_base(mut self_0: *mut edict_t) {
    (*self_0).solid = SOLID_BSP;
    (*self_0).movetype = MOVETYPE_PUSH as libc::c_int;
    (gi.setmodel).expect("non-null function pointer")(self_0, (*self_0).model);
    let ref mut fresh9 = (*self_0).blocked;
    *fresh9 = Some(
        turret_blocked as unsafe extern "C" fn(*mut edict_t, *mut edict_t) -> (),
    );
    (gi.linkentity).expect("non-null function pointer")(self_0);
}
#[no_mangle]
pub unsafe extern "C" fn turret_driver_die(
    mut self_0: *mut edict_t,
    mut inflictor: *mut edict_t,
    mut attacker: *mut edict_t,
    mut damage: libc::c_int,
    mut point: *mut vec_t,
) {
    let mut ent: *mut edict_t = 0 as *mut edict_t;
    (*(*self_0).target_ent)
        .move_angles[0 as libc::c_int as usize] = 0 as libc::c_int as vec_t;
    ent = (*(*self_0).target_ent).teammaster;
    while (*ent).teamchain != self_0 {
        ent = (*ent).teamchain;
    }
    let ref mut fresh10 = (*ent).teamchain;
    *fresh10 = 0 as *mut edict_t;
    let ref mut fresh11 = (*self_0).teammaster;
    *fresh11 = 0 as *mut edict_t;
    (*self_0).flags &= !(0x400 as libc::c_int);
    let ref mut fresh12 = (*(*self_0).target_ent).owner;
    *fresh12 = 0 as *mut edict_t;
    let ref mut fresh13 = (*(*(*self_0).target_ent).teammaster).owner;
    *fresh13 = 0 as *mut edict_t;
    infantry_die(self_0, inflictor, attacker, damage);
}
#[no_mangle]
pub unsafe extern "C" fn turret_driver_think(mut self_0: *mut edict_t) {
    let mut target: vec3_t = [0.; 3];
    let mut dir: vec3_t = [0.; 3];
    let mut reaction_time: libc::c_float = 0.;
    (*self_0).nextthink = (level.time as libc::c_double + 0.1f64) as libc::c_float;
    if !((*self_0).enemy).is_null()
        && ((*(*self_0).enemy).inuse as u64 == 0
            || (*(*self_0).enemy).health <= 0 as libc::c_int)
    {
        let ref mut fresh14 = (*self_0).enemy;
        *fresh14 = 0 as *mut edict_t;
    }
    if ((*self_0).enemy).is_null() {
        if FindTarget(self_0) as u64 == 0 {
            return;
        }
        (*self_0).monsterinfo.trail_time = level.time;
        (*self_0).monsterinfo.aiflags &= !(0x8 as libc::c_int);
    } else if visible(self_0, (*self_0).enemy) as u64 != 0 {
        if (*self_0).monsterinfo.aiflags & 0x8 as libc::c_int != 0 {
            (*self_0).monsterinfo.trail_time = level.time;
            (*self_0).monsterinfo.aiflags &= !(0x8 as libc::c_int);
        }
    } else {
        (*self_0).monsterinfo.aiflags |= 0x8 as libc::c_int;
        return;
    }
    target[0 as libc::c_int
        as usize] = (*(*self_0).enemy).s.origin[0 as libc::c_int as usize];
    target[1 as libc::c_int
        as usize] = (*(*self_0).enemy).s.origin[1 as libc::c_int as usize];
    target[2 as libc::c_int
        as usize] = (*(*self_0).enemy).s.origin[2 as libc::c_int as usize];
    target[2 as libc::c_int as usize] += (*(*self_0).enemy).viewheight as libc::c_float;
    dir[0 as libc::c_int
        as usize] = target[0 as libc::c_int as usize]
        - (*(*self_0).target_ent).s.origin[0 as libc::c_int as usize];
    dir[1 as libc::c_int
        as usize] = target[1 as libc::c_int as usize]
        - (*(*self_0).target_ent).s.origin[1 as libc::c_int as usize];
    dir[2 as libc::c_int
        as usize] = target[2 as libc::c_int as usize]
        - (*(*self_0).target_ent).s.origin[2 as libc::c_int as usize];
    vectoangles(dir.as_mut_ptr(), ((*(*self_0).target_ent).move_angles).as_mut_ptr());
    if level.time < (*self_0).monsterinfo.attack_finished {
        return;
    }
    reaction_time = ((3 as libc::c_int as libc::c_float - (*skill).value)
        as libc::c_double * 1.0f64) as libc::c_float;
    if level.time - (*self_0).monsterinfo.trail_time < reaction_time {
        return;
    }
    (*self_0)
        .monsterinfo
        .attack_finished = ((level.time + reaction_time) as libc::c_double + 1.0f64)
        as libc::c_float;
    (*(*self_0).target_ent).spawnflags |= 65536 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn turret_driver_link(mut self_0: *mut edict_t) {
    let mut vec: vec3_t = [0.; 3];
    let mut ent: *mut edict_t = 0 as *mut edict_t;
    let ref mut fresh15 = (*self_0).think;
    *fresh15 = Some(turret_driver_think as unsafe extern "C" fn(*mut edict_t) -> ());
    (*self_0).nextthink = (level.time as libc::c_double + 0.1f64) as libc::c_float;
    let ref mut fresh16 = (*self_0).target_ent;
    *fresh16 = G_PickTarget((*self_0).target);
    let ref mut fresh17 = (*(*self_0).target_ent).owner;
    *fresh17 = self_0;
    let ref mut fresh18 = (*(*(*self_0).target_ent).teammaster).owner;
    *fresh18 = self_0;
    (*self_0)
        .s
        .angles[0 as libc::c_int
        as usize] = (*(*self_0).target_ent).s.angles[0 as libc::c_int as usize];
    (*self_0)
        .s
        .angles[1 as libc::c_int
        as usize] = (*(*self_0).target_ent).s.angles[1 as libc::c_int as usize];
    (*self_0)
        .s
        .angles[2 as libc::c_int
        as usize] = (*(*self_0).target_ent).s.angles[2 as libc::c_int as usize];
    vec[0 as libc::c_int
        as usize] = (*(*self_0).target_ent).s.origin[0 as libc::c_int as usize]
        - (*self_0).s.origin[0 as libc::c_int as usize];
    vec[1 as libc::c_int
        as usize] = (*(*self_0).target_ent).s.origin[1 as libc::c_int as usize]
        - (*self_0).s.origin[1 as libc::c_int as usize];
    vec[2 as libc::c_int as usize] = 0 as libc::c_int as vec_t;
    (*self_0).move_origin[0 as libc::c_int as usize] = VectorLength(vec.as_mut_ptr());
    vec[0 as libc::c_int
        as usize] = (*self_0).s.origin[0 as libc::c_int as usize]
        - (*(*self_0).target_ent).s.origin[0 as libc::c_int as usize];
    vec[1 as libc::c_int
        as usize] = (*self_0).s.origin[1 as libc::c_int as usize]
        - (*(*self_0).target_ent).s.origin[1 as libc::c_int as usize];
    vec[2 as libc::c_int
        as usize] = (*self_0).s.origin[2 as libc::c_int as usize]
        - (*(*self_0).target_ent).s.origin[2 as libc::c_int as usize];
    vectoangles(vec.as_mut_ptr(), vec.as_mut_ptr());
    AnglesNormalize(vec.as_mut_ptr());
    (*self_0).move_origin[1 as libc::c_int as usize] = vec[1 as libc::c_int as usize];
    (*self_0)
        .move_origin[2 as libc::c_int
        as usize] = (*self_0).s.origin[2 as libc::c_int as usize]
        - (*(*self_0).target_ent).s.origin[2 as libc::c_int as usize];
    ent = (*(*self_0).target_ent).teammaster;
    while !((*ent).teamchain).is_null() {
        ent = (*ent).teamchain;
    }
    let ref mut fresh19 = (*ent).teamchain;
    *fresh19 = self_0;
    let ref mut fresh20 = (*self_0).teammaster;
    *fresh20 = (*(*self_0).target_ent).teammaster;
    (*self_0).flags |= 0x400 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn SP_turret_driver(mut self_0: *mut edict_t) {
    if (*deathmatch).value != 0. {
        G_FreeEdict(self_0);
        return;
    }
    (*self_0).movetype = MOVETYPE_PUSH as libc::c_int;
    (*self_0).solid = SOLID_BBOX;
    (*self_0)
        .s
        .modelindex = (gi.modelindex)
        .expect(
            "non-null function pointer",
        )(
        b"models/monsters/infantry/tris.md2\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    (*self_0).mins[0 as libc::c_int as usize] = -(16 as libc::c_int) as vec_t;
    (*self_0).mins[1 as libc::c_int as usize] = -(16 as libc::c_int) as vec_t;
    (*self_0).mins[2 as libc::c_int as usize] = -(24 as libc::c_int) as vec_t;
    (*self_0).maxs[0 as libc::c_int as usize] = 16 as libc::c_int as vec_t;
    (*self_0).maxs[1 as libc::c_int as usize] = 16 as libc::c_int as vec_t;
    (*self_0).maxs[2 as libc::c_int as usize] = 32 as libc::c_int as vec_t;
    (*self_0).health = 100 as libc::c_int;
    (*self_0).gib_health = 0 as libc::c_int;
    (*self_0).mass = 200 as libc::c_int;
    (*self_0).viewheight = 24 as libc::c_int;
    let ref mut fresh21 = (*self_0).die;
    *fresh21 = Some(
        turret_driver_die
            as unsafe extern "C" fn(
                *mut edict_t,
                *mut edict_t,
                *mut edict_t,
                libc::c_int,
                *mut vec_t,
            ) -> (),
    );
    let ref mut fresh22 = (*self_0).monsterinfo.stand;
    *fresh22 = Some(infantry_stand as unsafe extern "C" fn(*mut edict_t) -> ());
    (*self_0).flags |= 0x800 as libc::c_int;
    level.total_monsters += 1;
    (*self_0).svflags |= 0x4 as libc::c_int;
    (*self_0).s.renderfx |= 64 as libc::c_int;
    (*self_0).takedamage = DAMAGE_AIM as libc::c_int;
    let ref mut fresh23 = (*self_0).use_0;
    *fresh23 = Some(
        monster_use
            as unsafe extern "C" fn(*mut edict_t, *mut edict_t, *mut edict_t) -> (),
    );
    (*self_0)
        .clipmask = 1 as libc::c_int | 0x20000 as libc::c_int | 2 as libc::c_int
        | 0x2000000 as libc::c_int;
    (*self_0)
        .s
        .old_origin[0 as libc::c_int
        as usize] = (*self_0).s.origin[0 as libc::c_int as usize];
    (*self_0)
        .s
        .old_origin[1 as libc::c_int
        as usize] = (*self_0).s.origin[1 as libc::c_int as usize];
    (*self_0)
        .s
        .old_origin[2 as libc::c_int
        as usize] = (*self_0).s.origin[2 as libc::c_int as usize];
    (*self_0).monsterinfo.aiflags |= 0x1 as libc::c_int | 0x800 as libc::c_int;
    if !(st.item).is_null() {
        let ref mut fresh24 = (*self_0).item;
        *fresh24 = FindItemByClassname(st.item);
        if ((*self_0).item).is_null() {
            (gi.dprintf)
                .expect(
                    "non-null function pointer",
                )(
                b"%s at %s has bad item: %s\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                (*self_0).classname,
                vtos(((*self_0).s.origin).as_mut_ptr()),
                st.item,
            );
        }
    }
    let ref mut fresh25 = (*self_0).think;
    *fresh25 = Some(turret_driver_link as unsafe extern "C" fn(*mut edict_t) -> ());
    (*self_0).nextthink = (level.time as libc::c_double + 0.1f64) as libc::c_float;
    (gi.linkentity).expect("non-null function pointer")(self_0);
}
