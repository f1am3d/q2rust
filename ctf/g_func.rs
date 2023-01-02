#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn floor(_: libc::c_double) -> libc::c_double;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn rand() -> libc::c_int;
    static mut vec3_origin: vec3_t;
    fn VectorMA(
        veca: *mut vec_t,
        scale: libc::c_float,
        vecb: *mut vec_t,
        vecc: *mut vec_t,
    );
    fn AddPointToBounds(v: *mut vec_t, mins: *mut vec_t, maxs: *mut vec_t);
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
    fn Q_stricmp(s1: *mut libc::c_char, s2: *mut libc::c_char) -> libc::c_int;
    static mut level: level_locals_t;
    static mut gi: game_import_t;
    static mut st: spawn_temp_t;
    static mut deathmatch: *mut cvar_t;
    fn KillBox(ent: *mut edict_t) -> qboolean;
    fn G_Find(
        from: *mut edict_t,
        fieldofs: libc::c_int,
        match_0: *mut libc::c_char,
    ) -> *mut edict_t;
    fn G_PickTarget(targetname: *mut libc::c_char) -> *mut edict_t;
    fn G_UseTargets(ent: *mut edict_t, activator: *mut edict_t);
    fn G_SetMovedir(angles: *mut vec_t, movedir: *mut vec_t);
    fn G_Spawn() -> *mut edict_t;
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
    fn BecomeExplosion1(self_0: *mut edict_t);
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
    pub forcemap: [libc::c_char; 64],
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
pub unsafe extern "C" fn Move_Done(mut ent: *mut edict_t) {
    let ref mut fresh0 = (*ent).velocity[2 as libc::c_int as usize];
    *fresh0 = 0 as libc::c_int as vec_t;
    let ref mut fresh1 = (*ent).velocity[1 as libc::c_int as usize];
    *fresh1 = *fresh0;
    (*ent).velocity[0 as libc::c_int as usize] = *fresh1;
    ((*ent).moveinfo.endfunc).expect("non-null function pointer")(ent);
}
#[no_mangle]
pub unsafe extern "C" fn Move_Final(mut ent: *mut edict_t) {
    if (*ent).moveinfo.remaining_distance == 0 as libc::c_int as libc::c_float {
        Move_Done(ent);
        return;
    }
    VectorScale(
        ((*ent).moveinfo.dir).as_mut_ptr(),
        ((*ent).moveinfo.remaining_distance as libc::c_double / 0.1f64) as vec_t,
        ((*ent).velocity).as_mut_ptr(),
    );
    let ref mut fresh2 = (*ent).think;
    *fresh2 = Some(Move_Done as unsafe extern "C" fn(*mut edict_t) -> ());
    (*ent).nextthink = (level.time as libc::c_double + 0.1f64) as libc::c_float;
}
#[no_mangle]
pub unsafe extern "C" fn Move_Begin(mut ent: *mut edict_t) {
    let mut frames: libc::c_float = 0.;
    if (*ent).moveinfo.speed as libc::c_double * 0.1f64
        >= (*ent).moveinfo.remaining_distance as libc::c_double
    {
        Move_Final(ent);
        return;
    }
    VectorScale(
        ((*ent).moveinfo.dir).as_mut_ptr(),
        (*ent).moveinfo.speed,
        ((*ent).velocity).as_mut_ptr(),
    );
    frames = floor(
        ((*ent).moveinfo.remaining_distance / (*ent).moveinfo.speed) as libc::c_double
            / 0.1f64,
    ) as libc::c_float;
    let ref mut fresh3 = (*ent).moveinfo.remaining_distance;
    *fresh3 = (*fresh3 as libc::c_double
        - (frames * (*ent).moveinfo.speed) as libc::c_double * 0.1f64) as libc::c_float;
    (*ent)
        .nextthink = (level.time as libc::c_double + frames as libc::c_double * 0.1f64)
        as libc::c_float;
    let ref mut fresh4 = (*ent).think;
    *fresh4 = Some(Move_Final as unsafe extern "C" fn(*mut edict_t) -> ());
}
#[no_mangle]
pub unsafe extern "C" fn Move_Calc(
    mut ent: *mut edict_t,
    mut dest: *mut vec_t,
    mut func: Option::<unsafe extern "C" fn(*mut edict_t) -> ()>,
) {
    let ref mut fresh5 = (*ent).velocity[2 as libc::c_int as usize];
    *fresh5 = 0 as libc::c_int as vec_t;
    let ref mut fresh6 = (*ent).velocity[1 as libc::c_int as usize];
    *fresh6 = *fresh5;
    (*ent).velocity[0 as libc::c_int as usize] = *fresh6;
    (*ent)
        .moveinfo
        .dir[0 as libc::c_int
        as usize] = *dest.offset(0 as libc::c_int as isize)
        - (*ent).s.origin[0 as libc::c_int as usize];
    (*ent)
        .moveinfo
        .dir[1 as libc::c_int
        as usize] = *dest.offset(1 as libc::c_int as isize)
        - (*ent).s.origin[1 as libc::c_int as usize];
    (*ent)
        .moveinfo
        .dir[2 as libc::c_int
        as usize] = *dest.offset(2 as libc::c_int as isize)
        - (*ent).s.origin[2 as libc::c_int as usize];
    (*ent)
        .moveinfo
        .remaining_distance = VectorNormalize(((*ent).moveinfo.dir).as_mut_ptr());
    let ref mut fresh7 = (*ent).moveinfo.endfunc;
    *fresh7 = func;
    if (*ent).moveinfo.speed == (*ent).moveinfo.accel
        && (*ent).moveinfo.speed == (*ent).moveinfo.decel
    {
        if level.current_entity
            == (if (*ent).flags & 0x400 as libc::c_int != 0 {
                (*ent).teammaster
            } else {
                ent
            })
        {
            Move_Begin(ent);
        } else {
            (*ent).nextthink = (level.time as libc::c_double + 0.1f64) as libc::c_float;
            let ref mut fresh8 = (*ent).think;
            *fresh8 = Some(Move_Begin as unsafe extern "C" fn(*mut edict_t) -> ());
        }
    } else {
        (*ent).moveinfo.current_speed = 0 as libc::c_int as libc::c_float;
        let ref mut fresh9 = (*ent).think;
        *fresh9 = Some(Think_AccelMove as unsafe extern "C" fn(*mut edict_t) -> ());
        (*ent).nextthink = (level.time as libc::c_double + 0.1f64) as libc::c_float;
    };
}
#[no_mangle]
pub unsafe extern "C" fn AngleMove_Done(mut ent: *mut edict_t) {
    let ref mut fresh10 = (*ent).avelocity[2 as libc::c_int as usize];
    *fresh10 = 0 as libc::c_int as vec_t;
    let ref mut fresh11 = (*ent).avelocity[1 as libc::c_int as usize];
    *fresh11 = *fresh10;
    (*ent).avelocity[0 as libc::c_int as usize] = *fresh11;
    ((*ent).moveinfo.endfunc).expect("non-null function pointer")(ent);
}
#[no_mangle]
pub unsafe extern "C" fn AngleMove_Final(mut ent: *mut edict_t) {
    let mut move_0: vec3_t = [0.; 3];
    if (*ent).moveinfo.state == 2 as libc::c_int {
        move_0[0 as libc::c_int
            as usize] = (*ent).moveinfo.end_angles[0 as libc::c_int as usize]
            - (*ent).s.angles[0 as libc::c_int as usize];
        move_0[1 as libc::c_int
            as usize] = (*ent).moveinfo.end_angles[1 as libc::c_int as usize]
            - (*ent).s.angles[1 as libc::c_int as usize];
        move_0[2 as libc::c_int
            as usize] = (*ent).moveinfo.end_angles[2 as libc::c_int as usize]
            - (*ent).s.angles[2 as libc::c_int as usize];
    } else {
        move_0[0 as libc::c_int
            as usize] = (*ent).moveinfo.start_angles[0 as libc::c_int as usize]
            - (*ent).s.angles[0 as libc::c_int as usize];
        move_0[1 as libc::c_int
            as usize] = (*ent).moveinfo.start_angles[1 as libc::c_int as usize]
            - (*ent).s.angles[1 as libc::c_int as usize];
        move_0[2 as libc::c_int
            as usize] = (*ent).moveinfo.start_angles[2 as libc::c_int as usize]
            - (*ent).s.angles[2 as libc::c_int as usize];
    }
    if VectorCompare(move_0.as_mut_ptr(), vec3_origin.as_mut_ptr()) != 0 {
        AngleMove_Done(ent);
        return;
    }
    VectorScale(
        move_0.as_mut_ptr(),
        (1.0f64 / 0.1f64) as vec_t,
        ((*ent).avelocity).as_mut_ptr(),
    );
    let ref mut fresh12 = (*ent).think;
    *fresh12 = Some(AngleMove_Done as unsafe extern "C" fn(*mut edict_t) -> ());
    (*ent).nextthink = (level.time as libc::c_double + 0.1f64) as libc::c_float;
}
#[no_mangle]
pub unsafe extern "C" fn AngleMove_Begin(mut ent: *mut edict_t) {
    let mut destdelta: vec3_t = [0.; 3];
    let mut len: libc::c_float = 0.;
    let mut traveltime: libc::c_float = 0.;
    let mut frames: libc::c_float = 0.;
    if (*ent).moveinfo.state == 2 as libc::c_int {
        destdelta[0 as libc::c_int
            as usize] = (*ent).moveinfo.end_angles[0 as libc::c_int as usize]
            - (*ent).s.angles[0 as libc::c_int as usize];
        destdelta[1 as libc::c_int
            as usize] = (*ent).moveinfo.end_angles[1 as libc::c_int as usize]
            - (*ent).s.angles[1 as libc::c_int as usize];
        destdelta[2 as libc::c_int
            as usize] = (*ent).moveinfo.end_angles[2 as libc::c_int as usize]
            - (*ent).s.angles[2 as libc::c_int as usize];
    } else {
        destdelta[0 as libc::c_int
            as usize] = (*ent).moveinfo.start_angles[0 as libc::c_int as usize]
            - (*ent).s.angles[0 as libc::c_int as usize];
        destdelta[1 as libc::c_int
            as usize] = (*ent).moveinfo.start_angles[1 as libc::c_int as usize]
            - (*ent).s.angles[1 as libc::c_int as usize];
        destdelta[2 as libc::c_int
            as usize] = (*ent).moveinfo.start_angles[2 as libc::c_int as usize]
            - (*ent).s.angles[2 as libc::c_int as usize];
    }
    len = VectorLength(destdelta.as_mut_ptr());
    traveltime = len / (*ent).moveinfo.speed;
    if (traveltime as libc::c_double) < 0.1f64 {
        AngleMove_Final(ent);
        return;
    }
    frames = floor(traveltime as libc::c_double / 0.1f64) as libc::c_float;
    VectorScale(
        destdelta.as_mut_ptr(),
        (1.0f64 / traveltime as libc::c_double) as vec_t,
        ((*ent).avelocity).as_mut_ptr(),
    );
    (*ent)
        .nextthink = (level.time as libc::c_double + frames as libc::c_double * 0.1f64)
        as libc::c_float;
    let ref mut fresh13 = (*ent).think;
    *fresh13 = Some(AngleMove_Final as unsafe extern "C" fn(*mut edict_t) -> ());
}
#[no_mangle]
pub unsafe extern "C" fn AngleMove_Calc(
    mut ent: *mut edict_t,
    mut func: Option::<unsafe extern "C" fn(*mut edict_t) -> ()>,
) {
    let ref mut fresh14 = (*ent).avelocity[2 as libc::c_int as usize];
    *fresh14 = 0 as libc::c_int as vec_t;
    let ref mut fresh15 = (*ent).avelocity[1 as libc::c_int as usize];
    *fresh15 = *fresh14;
    (*ent).avelocity[0 as libc::c_int as usize] = *fresh15;
    let ref mut fresh16 = (*ent).moveinfo.endfunc;
    *fresh16 = func;
    if level.current_entity
        == (if (*ent).flags & 0x400 as libc::c_int != 0 {
            (*ent).teammaster
        } else {
            ent
        })
    {
        AngleMove_Begin(ent);
    } else {
        (*ent).nextthink = (level.time as libc::c_double + 0.1f64) as libc::c_float;
        let ref mut fresh17 = (*ent).think;
        *fresh17 = Some(AngleMove_Begin as unsafe extern "C" fn(*mut edict_t) -> ());
    };
}
#[no_mangle]
pub unsafe extern "C" fn plat_CalcAcceleratedMove(mut moveinfo: *mut moveinfo_t) {
    let mut accel_dist: libc::c_float = 0.;
    let mut decel_dist: libc::c_float = 0.;
    (*moveinfo).move_speed = (*moveinfo).speed;
    if (*moveinfo).remaining_distance < (*moveinfo).accel {
        (*moveinfo).current_speed = (*moveinfo).remaining_distance;
        return;
    }
    accel_dist = (*moveinfo).speed
        * ((*moveinfo).speed / (*moveinfo).accel + 1 as libc::c_int as libc::c_float)
        / 2 as libc::c_int as libc::c_float;
    decel_dist = (*moveinfo).speed
        * ((*moveinfo).speed / (*moveinfo).decel + 1 as libc::c_int as libc::c_float)
        / 2 as libc::c_int as libc::c_float;
    if (*moveinfo).remaining_distance - accel_dist - decel_dist
        < 0 as libc::c_int as libc::c_float
    {
        let mut f: libc::c_float = 0.;
        f = ((*moveinfo).accel + (*moveinfo).decel)
            / ((*moveinfo).accel * (*moveinfo).decel);
        (*moveinfo)
            .move_speed = ((-(2 as libc::c_int) as libc::c_double
            + sqrt(
                (4 as libc::c_int as libc::c_float
                    - 4 as libc::c_int as libc::c_float * f
                        * (-(2 as libc::c_int) as libc::c_float
                            * (*moveinfo).remaining_distance)) as libc::c_double,
            )) / (2 as libc::c_int as libc::c_float * f) as libc::c_double)
            as libc::c_float;
        decel_dist = (*moveinfo).move_speed
            * ((*moveinfo).move_speed / (*moveinfo).decel
                + 1 as libc::c_int as libc::c_float) / 2 as libc::c_int as libc::c_float;
    }
    (*moveinfo).decel_distance = decel_dist;
}
#[no_mangle]
pub unsafe extern "C" fn plat_Accelerate(mut moveinfo: *mut moveinfo_t) {
    if (*moveinfo).remaining_distance <= (*moveinfo).decel_distance {
        if (*moveinfo).remaining_distance < (*moveinfo).decel_distance {
            if (*moveinfo).next_speed != 0. {
                (*moveinfo).current_speed = (*moveinfo).next_speed;
                (*moveinfo).next_speed = 0 as libc::c_int as libc::c_float;
                return;
            }
            if (*moveinfo).current_speed > (*moveinfo).decel {
                (*moveinfo).current_speed -= (*moveinfo).decel;
            }
        }
        return;
    }
    if (*moveinfo).current_speed == (*moveinfo).move_speed {
        if (*moveinfo).remaining_distance - (*moveinfo).current_speed
            < (*moveinfo).decel_distance
        {
            let mut p1_distance: libc::c_float = 0.;
            let mut p2_distance: libc::c_float = 0.;
            let mut distance: libc::c_float = 0.;
            p1_distance = (*moveinfo).remaining_distance - (*moveinfo).decel_distance;
            p2_distance = ((*moveinfo).move_speed as libc::c_double
                * (1.0f64 - (p1_distance / (*moveinfo).move_speed) as libc::c_double))
                as libc::c_float;
            distance = p1_distance + p2_distance;
            (*moveinfo).current_speed = (*moveinfo).move_speed;
            (*moveinfo)
                .next_speed = (*moveinfo).move_speed
                - (*moveinfo).decel * (p2_distance / distance);
            return;
        }
    }
    if (*moveinfo).current_speed < (*moveinfo).speed {
        let mut old_speed: libc::c_float = 0.;
        let mut p1_distance_0: libc::c_float = 0.;
        let mut p1_speed: libc::c_float = 0.;
        let mut p2_distance_0: libc::c_float = 0.;
        let mut distance_0: libc::c_float = 0.;
        old_speed = (*moveinfo).current_speed;
        (*moveinfo).current_speed += (*moveinfo).accel;
        if (*moveinfo).current_speed > (*moveinfo).speed {
            (*moveinfo).current_speed = (*moveinfo).speed;
        }
        if (*moveinfo).remaining_distance - (*moveinfo).current_speed
            >= (*moveinfo).decel_distance
        {
            return;
        }
        p1_distance_0 = (*moveinfo).remaining_distance - (*moveinfo).decel_distance;
        p1_speed = ((old_speed + (*moveinfo).move_speed) as libc::c_double / 2.0f64)
            as libc::c_float;
        p2_distance_0 = ((*moveinfo).move_speed as libc::c_double
            * (1.0f64 - (p1_distance_0 / p1_speed) as libc::c_double)) as libc::c_float;
        distance_0 = p1_distance_0 + p2_distance_0;
        (*moveinfo)
            .current_speed = p1_speed * (p1_distance_0 / distance_0)
            + (*moveinfo).move_speed * (p2_distance_0 / distance_0);
        (*moveinfo)
            .next_speed = (*moveinfo).move_speed
            - (*moveinfo).decel * (p2_distance_0 / distance_0);
        return;
    }
}
#[no_mangle]
pub unsafe extern "C" fn Think_AccelMove(mut ent: *mut edict_t) {
    (*ent).moveinfo.remaining_distance -= (*ent).moveinfo.current_speed;
    if (*ent).moveinfo.current_speed == 0 as libc::c_int as libc::c_float {
        plat_CalcAcceleratedMove(&mut (*ent).moveinfo);
    }
    plat_Accelerate(&mut (*ent).moveinfo);
    if (*ent).moveinfo.remaining_distance <= (*ent).moveinfo.current_speed {
        Move_Final(ent);
        return;
    }
    VectorScale(
        ((*ent).moveinfo.dir).as_mut_ptr(),
        (*ent).moveinfo.current_speed * 10 as libc::c_int as libc::c_float,
        ((*ent).velocity).as_mut_ptr(),
    );
    (*ent).nextthink = (level.time as libc::c_double + 0.1f64) as libc::c_float;
    let ref mut fresh18 = (*ent).think;
    *fresh18 = Some(Think_AccelMove as unsafe extern "C" fn(*mut edict_t) -> ());
}
#[no_mangle]
pub unsafe extern "C" fn plat_hit_top(mut ent: *mut edict_t) {
    if (*ent).flags & 0x400 as libc::c_int == 0 {
        if (*ent).moveinfo.sound_end != 0 {
            (gi.sound)
                .expect(
                    "non-null function pointer",
                )(
                ent,
                8 as libc::c_int + 2 as libc::c_int,
                (*ent).moveinfo.sound_end,
                1 as libc::c_int as libc::c_float,
                3 as libc::c_int as libc::c_float,
                0 as libc::c_int as libc::c_float,
            );
        }
        (*ent).s.sound = 0 as libc::c_int;
    }
    (*ent).moveinfo.state = 0 as libc::c_int;
    let ref mut fresh19 = (*ent).think;
    *fresh19 = Some(plat_go_down as unsafe extern "C" fn(*mut edict_t) -> ());
    (*ent).nextthink = level.time + 3 as libc::c_int as libc::c_float;
}
#[no_mangle]
pub unsafe extern "C" fn plat_hit_bottom(mut ent: *mut edict_t) {
    if (*ent).flags & 0x400 as libc::c_int == 0 {
        if (*ent).moveinfo.sound_end != 0 {
            (gi.sound)
                .expect(
                    "non-null function pointer",
                )(
                ent,
                8 as libc::c_int + 2 as libc::c_int,
                (*ent).moveinfo.sound_end,
                1 as libc::c_int as libc::c_float,
                3 as libc::c_int as libc::c_float,
                0 as libc::c_int as libc::c_float,
            );
        }
        (*ent).s.sound = 0 as libc::c_int;
    }
    (*ent).moveinfo.state = 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn plat_go_down(mut ent: *mut edict_t) {
    if (*ent).flags & 0x400 as libc::c_int == 0 {
        if (*ent).moveinfo.sound_start != 0 {
            (gi.sound)
                .expect(
                    "non-null function pointer",
                )(
                ent,
                8 as libc::c_int + 2 as libc::c_int,
                (*ent).moveinfo.sound_start,
                1 as libc::c_int as libc::c_float,
                3 as libc::c_int as libc::c_float,
                0 as libc::c_int as libc::c_float,
            );
        }
        (*ent).s.sound = (*ent).moveinfo.sound_middle;
    }
    (*ent).moveinfo.state = 3 as libc::c_int;
    Move_Calc(
        ent,
        ((*ent).moveinfo.end_origin).as_mut_ptr(),
        Some(plat_hit_bottom as unsafe extern "C" fn(*mut edict_t) -> ()),
    );
}
#[no_mangle]
pub unsafe extern "C" fn plat_go_up(mut ent: *mut edict_t) {
    if (*ent).flags & 0x400 as libc::c_int == 0 {
        if (*ent).moveinfo.sound_start != 0 {
            (gi.sound)
                .expect(
                    "non-null function pointer",
                )(
                ent,
                8 as libc::c_int + 2 as libc::c_int,
                (*ent).moveinfo.sound_start,
                1 as libc::c_int as libc::c_float,
                3 as libc::c_int as libc::c_float,
                0 as libc::c_int as libc::c_float,
            );
        }
        (*ent).s.sound = (*ent).moveinfo.sound_middle;
    }
    (*ent).moveinfo.state = 2 as libc::c_int;
    Move_Calc(
        ent,
        ((*ent).moveinfo.start_origin).as_mut_ptr(),
        Some(plat_hit_top as unsafe extern "C" fn(*mut edict_t) -> ()),
    );
}
#[no_mangle]
pub unsafe extern "C" fn plat_blocked(
    mut self_0: *mut edict_t,
    mut other: *mut edict_t,
) {
    if (*other).svflags & 0x4 as libc::c_int == 0 && ((*other).client).is_null() {
        T_Damage(
            other,
            self_0,
            self_0,
            vec3_origin.as_mut_ptr(),
            ((*other).s.origin).as_mut_ptr(),
            vec3_origin.as_mut_ptr(),
            100000 as libc::c_int,
            1 as libc::c_int,
            0 as libc::c_int,
            20 as libc::c_int,
        );
        if !other.is_null() {
            BecomeExplosion1(other);
        }
        return;
    }
    T_Damage(
        other,
        self_0,
        self_0,
        vec3_origin.as_mut_ptr(),
        ((*other).s.origin).as_mut_ptr(),
        vec3_origin.as_mut_ptr(),
        (*self_0).dmg,
        1 as libc::c_int,
        0 as libc::c_int,
        20 as libc::c_int,
    );
    if (*self_0).moveinfo.state == 2 as libc::c_int {
        plat_go_down(self_0);
    } else if (*self_0).moveinfo.state == 3 as libc::c_int {
        plat_go_up(self_0);
    }
}
#[no_mangle]
pub unsafe extern "C" fn Use_Plat(
    mut ent: *mut edict_t,
    mut other: *mut edict_t,
    mut activator: *mut edict_t,
) {
    if ((*ent).think).is_some() {
        return;
    }
    plat_go_down(ent);
}
#[no_mangle]
pub unsafe extern "C" fn Touch_Plat_Center(
    mut ent: *mut edict_t,
    mut other: *mut edict_t,
    mut plane: *mut cplane_t,
    mut surf: *mut csurface_t,
) {
    if ((*other).client).is_null() {
        return;
    }
    if (*other).health <= 0 as libc::c_int {
        return;
    }
    ent = (*ent).enemy;
    if (*ent).moveinfo.state == 1 as libc::c_int {
        plat_go_up(ent);
    } else if (*ent).moveinfo.state == 0 as libc::c_int {
        (*ent).nextthink = level.time + 1 as libc::c_int as libc::c_float;
    }
}
#[no_mangle]
pub unsafe extern "C" fn plat_spawn_inside_trigger(mut ent: *mut edict_t) {
    let mut trigger: *mut edict_t = 0 as *mut edict_t;
    let mut tmin: vec3_t = [0.; 3];
    let mut tmax: vec3_t = [0.; 3];
    trigger = G_Spawn();
    let ref mut fresh20 = (*trigger).touch;
    *fresh20 = Some(
        Touch_Plat_Center
            as unsafe extern "C" fn(
                *mut edict_t,
                *mut edict_t,
                *mut cplane_t,
                *mut csurface_t,
            ) -> (),
    );
    (*trigger).movetype = MOVETYPE_NONE as libc::c_int;
    (*trigger).solid = SOLID_TRIGGER;
    let ref mut fresh21 = (*trigger).enemy;
    *fresh21 = ent;
    tmin[0 as libc::c_int
        as usize] = (*ent).mins[0 as libc::c_int as usize]
        + 25 as libc::c_int as libc::c_float;
    tmin[1 as libc::c_int
        as usize] = (*ent).mins[1 as libc::c_int as usize]
        + 25 as libc::c_int as libc::c_float;
    tmin[2 as libc::c_int as usize] = (*ent).mins[2 as libc::c_int as usize];
    tmax[0 as libc::c_int
        as usize] = (*ent).maxs[0 as libc::c_int as usize]
        - 25 as libc::c_int as libc::c_float;
    tmax[1 as libc::c_int
        as usize] = (*ent).maxs[1 as libc::c_int as usize]
        - 25 as libc::c_int as libc::c_float;
    tmax[2 as libc::c_int
        as usize] = (*ent).maxs[2 as libc::c_int as usize]
        + 8 as libc::c_int as libc::c_float;
    tmin[2 as libc::c_int
        as usize] = tmax[2 as libc::c_int as usize]
        - ((*ent).pos1[2 as libc::c_int as usize]
            - (*ent).pos2[2 as libc::c_int as usize] + st.lip as libc::c_float);
    if (*ent).spawnflags & 1 as libc::c_int != 0 {
        tmax[2 as libc::c_int
            as usize] = tmin[2 as libc::c_int as usize]
            + 8 as libc::c_int as libc::c_float;
    }
    if tmax[0 as libc::c_int as usize] - tmin[0 as libc::c_int as usize]
        <= 0 as libc::c_int as libc::c_float
    {
        tmin[0 as libc::c_int
            as usize] = (((*ent).mins[0 as libc::c_int as usize]
            + (*ent).maxs[0 as libc::c_int as usize]) as libc::c_double * 0.5f64)
            as vec_t;
        tmax[0 as libc::c_int
            as usize] = tmin[0 as libc::c_int as usize]
            + 1 as libc::c_int as libc::c_float;
    }
    if tmax[1 as libc::c_int as usize] - tmin[1 as libc::c_int as usize]
        <= 0 as libc::c_int as libc::c_float
    {
        tmin[1 as libc::c_int
            as usize] = (((*ent).mins[1 as libc::c_int as usize]
            + (*ent).maxs[1 as libc::c_int as usize]) as libc::c_double * 0.5f64)
            as vec_t;
        tmax[1 as libc::c_int
            as usize] = tmin[1 as libc::c_int as usize]
            + 1 as libc::c_int as libc::c_float;
    }
    (*trigger).mins[0 as libc::c_int as usize] = tmin[0 as libc::c_int as usize];
    (*trigger).mins[1 as libc::c_int as usize] = tmin[1 as libc::c_int as usize];
    (*trigger).mins[2 as libc::c_int as usize] = tmin[2 as libc::c_int as usize];
    (*trigger).maxs[0 as libc::c_int as usize] = tmax[0 as libc::c_int as usize];
    (*trigger).maxs[1 as libc::c_int as usize] = tmax[1 as libc::c_int as usize];
    (*trigger).maxs[2 as libc::c_int as usize] = tmax[2 as libc::c_int as usize];
    (gi.linkentity).expect("non-null function pointer")(trigger);
}
#[no_mangle]
pub unsafe extern "C" fn SP_func_plat(mut ent: *mut edict_t) {
    let ref mut fresh22 = (*ent).s.angles[2 as libc::c_int as usize];
    *fresh22 = 0 as libc::c_int as vec_t;
    let ref mut fresh23 = (*ent).s.angles[1 as libc::c_int as usize];
    *fresh23 = *fresh22;
    (*ent).s.angles[0 as libc::c_int as usize] = *fresh23;
    (*ent).solid = SOLID_BSP;
    (*ent).movetype = MOVETYPE_PUSH as libc::c_int;
    (gi.setmodel).expect("non-null function pointer")(ent, (*ent).model);
    let ref mut fresh24 = (*ent).blocked;
    *fresh24 = Some(
        plat_blocked as unsafe extern "C" fn(*mut edict_t, *mut edict_t) -> (),
    );
    if (*ent).speed == 0. {
        (*ent).speed = 20 as libc::c_int as libc::c_float;
    } else {
        let ref mut fresh25 = (*ent).speed;
        *fresh25 = (*fresh25 as libc::c_double * 0.1f64) as libc::c_float;
    }
    if (*ent).accel == 0. {
        (*ent).accel = 5 as libc::c_int as libc::c_float;
    } else {
        let ref mut fresh26 = (*ent).accel;
        *fresh26 = (*fresh26 as libc::c_double * 0.1f64) as libc::c_float;
    }
    if (*ent).decel == 0. {
        (*ent).decel = 5 as libc::c_int as libc::c_float;
    } else {
        let ref mut fresh27 = (*ent).decel;
        *fresh27 = (*fresh27 as libc::c_double * 0.1f64) as libc::c_float;
    }
    if (*ent).dmg == 0 {
        (*ent).dmg = 2 as libc::c_int;
    }
    if st.lip == 0 {
        st.lip = 8 as libc::c_int;
    }
    (*ent).pos1[0 as libc::c_int as usize] = (*ent).s.origin[0 as libc::c_int as usize];
    (*ent).pos1[1 as libc::c_int as usize] = (*ent).s.origin[1 as libc::c_int as usize];
    (*ent).pos1[2 as libc::c_int as usize] = (*ent).s.origin[2 as libc::c_int as usize];
    (*ent).pos2[0 as libc::c_int as usize] = (*ent).s.origin[0 as libc::c_int as usize];
    (*ent).pos2[1 as libc::c_int as usize] = (*ent).s.origin[1 as libc::c_int as usize];
    (*ent).pos2[2 as libc::c_int as usize] = (*ent).s.origin[2 as libc::c_int as usize];
    if st.height != 0 {
        let ref mut fresh28 = (*ent).pos2[2 as libc::c_int as usize];
        *fresh28 -= st.height as libc::c_float;
    } else {
        let ref mut fresh29 = (*ent).pos2[2 as libc::c_int as usize];
        *fresh29
            -= (*ent).maxs[2 as libc::c_int as usize]
                - (*ent).mins[2 as libc::c_int as usize] - st.lip as libc::c_float;
    }
    let ref mut fresh30 = (*ent).use_0;
    *fresh30 = Some(
        Use_Plat as unsafe extern "C" fn(*mut edict_t, *mut edict_t, *mut edict_t) -> (),
    );
    plat_spawn_inside_trigger(ent);
    if !((*ent).targetname).is_null() {
        (*ent).moveinfo.state = 2 as libc::c_int;
    } else {
        (*ent)
            .s
            .origin[0 as libc::c_int as usize] = (*ent).pos2[0 as libc::c_int as usize];
        (*ent)
            .s
            .origin[1 as libc::c_int as usize] = (*ent).pos2[1 as libc::c_int as usize];
        (*ent)
            .s
            .origin[2 as libc::c_int as usize] = (*ent).pos2[2 as libc::c_int as usize];
        (gi.linkentity).expect("non-null function pointer")(ent);
        (*ent).moveinfo.state = 1 as libc::c_int;
    }
    (*ent).moveinfo.speed = (*ent).speed;
    (*ent).moveinfo.accel = (*ent).accel;
    (*ent).moveinfo.decel = (*ent).decel;
    (*ent).moveinfo.wait = (*ent).wait;
    (*ent)
        .moveinfo
        .start_origin[0 as libc::c_int
        as usize] = (*ent).pos1[0 as libc::c_int as usize];
    (*ent)
        .moveinfo
        .start_origin[1 as libc::c_int
        as usize] = (*ent).pos1[1 as libc::c_int as usize];
    (*ent)
        .moveinfo
        .start_origin[2 as libc::c_int
        as usize] = (*ent).pos1[2 as libc::c_int as usize];
    (*ent)
        .moveinfo
        .start_angles[0 as libc::c_int
        as usize] = (*ent).s.angles[0 as libc::c_int as usize];
    (*ent)
        .moveinfo
        .start_angles[1 as libc::c_int
        as usize] = (*ent).s.angles[1 as libc::c_int as usize];
    (*ent)
        .moveinfo
        .start_angles[2 as libc::c_int
        as usize] = (*ent).s.angles[2 as libc::c_int as usize];
    (*ent)
        .moveinfo
        .end_origin[0 as libc::c_int as usize] = (*ent).pos2[0 as libc::c_int as usize];
    (*ent)
        .moveinfo
        .end_origin[1 as libc::c_int as usize] = (*ent).pos2[1 as libc::c_int as usize];
    (*ent)
        .moveinfo
        .end_origin[2 as libc::c_int as usize] = (*ent).pos2[2 as libc::c_int as usize];
    (*ent)
        .moveinfo
        .end_angles[0 as libc::c_int
        as usize] = (*ent).s.angles[0 as libc::c_int as usize];
    (*ent)
        .moveinfo
        .end_angles[1 as libc::c_int
        as usize] = (*ent).s.angles[1 as libc::c_int as usize];
    (*ent)
        .moveinfo
        .end_angles[2 as libc::c_int
        as usize] = (*ent).s.angles[2 as libc::c_int as usize];
    (*ent)
        .moveinfo
        .sound_start = (gi.soundindex)
        .expect(
            "non-null function pointer",
        )(
        b"plats/pt1_strt.wav\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    (*ent)
        .moveinfo
        .sound_middle = (gi.soundindex)
        .expect(
            "non-null function pointer",
        )(
        b"plats/pt1_mid.wav\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    (*ent)
        .moveinfo
        .sound_end = (gi.soundindex)
        .expect(
            "non-null function pointer",
        )(
        b"plats/pt1_end.wav\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
}
#[no_mangle]
pub unsafe extern "C" fn rotating_blocked(
    mut self_0: *mut edict_t,
    mut other: *mut edict_t,
) {
    T_Damage(
        other,
        self_0,
        self_0,
        vec3_origin.as_mut_ptr(),
        ((*other).s.origin).as_mut_ptr(),
        vec3_origin.as_mut_ptr(),
        (*self_0).dmg,
        1 as libc::c_int,
        0 as libc::c_int,
        20 as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn rotating_touch(
    mut self_0: *mut edict_t,
    mut other: *mut edict_t,
    mut plane: *mut cplane_t,
    mut surf: *mut csurface_t,
) {
    if (*self_0).avelocity[0 as libc::c_int as usize] != 0.
        || (*self_0).avelocity[1 as libc::c_int as usize] != 0.
        || (*self_0).avelocity[2 as libc::c_int as usize] != 0.
    {
        T_Damage(
            other,
            self_0,
            self_0,
            vec3_origin.as_mut_ptr(),
            ((*other).s.origin).as_mut_ptr(),
            vec3_origin.as_mut_ptr(),
            (*self_0).dmg,
            1 as libc::c_int,
            0 as libc::c_int,
            20 as libc::c_int,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn rotating_use(
    mut self_0: *mut edict_t,
    mut other: *mut edict_t,
    mut activator: *mut edict_t,
) {
    if VectorCompare(((*self_0).avelocity).as_mut_ptr(), vec3_origin.as_mut_ptr()) == 0 {
        (*self_0).s.sound = 0 as libc::c_int;
        let ref mut fresh31 = (*self_0).avelocity[2 as libc::c_int as usize];
        *fresh31 = 0 as libc::c_int as vec_t;
        let ref mut fresh32 = (*self_0).avelocity[1 as libc::c_int as usize];
        *fresh32 = *fresh31;
        (*self_0).avelocity[0 as libc::c_int as usize] = *fresh32;
        let ref mut fresh33 = (*self_0).touch;
        *fresh33 = None;
    } else {
        (*self_0).s.sound = (*self_0).moveinfo.sound_middle;
        VectorScale(
            ((*self_0).movedir).as_mut_ptr(),
            (*self_0).speed,
            ((*self_0).avelocity).as_mut_ptr(),
        );
        if (*self_0).spawnflags & 16 as libc::c_int != 0 {
            let ref mut fresh34 = (*self_0).touch;
            *fresh34 = Some(
                rotating_touch
                    as unsafe extern "C" fn(
                        *mut edict_t,
                        *mut edict_t,
                        *mut cplane_t,
                        *mut csurface_t,
                    ) -> (),
            );
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn SP_func_rotating(mut ent: *mut edict_t) {
    (*ent).solid = SOLID_BSP;
    if (*ent).spawnflags & 32 as libc::c_int != 0 {
        (*ent).movetype = MOVETYPE_STOP as libc::c_int;
    } else {
        (*ent).movetype = MOVETYPE_PUSH as libc::c_int;
    }
    let ref mut fresh35 = (*ent).movedir[2 as libc::c_int as usize];
    *fresh35 = 0 as libc::c_int as vec_t;
    let ref mut fresh36 = (*ent).movedir[1 as libc::c_int as usize];
    *fresh36 = *fresh35;
    (*ent).movedir[0 as libc::c_int as usize] = *fresh36;
    if (*ent).spawnflags & 4 as libc::c_int != 0 {
        (*ent).movedir[2 as libc::c_int as usize] = 1.0f64 as vec_t;
    } else if (*ent).spawnflags & 8 as libc::c_int != 0 {
        (*ent).movedir[0 as libc::c_int as usize] = 1.0f64 as vec_t;
    } else {
        (*ent).movedir[1 as libc::c_int as usize] = 1.0f64 as vec_t;
    }
    if (*ent).spawnflags & 2 as libc::c_int != 0 {
        (*ent)
            .movedir[0 as libc::c_int
            as usize] = -(*ent).movedir[0 as libc::c_int as usize];
        (*ent)
            .movedir[1 as libc::c_int
            as usize] = -(*ent).movedir[1 as libc::c_int as usize];
        (*ent)
            .movedir[2 as libc::c_int
            as usize] = -(*ent).movedir[2 as libc::c_int as usize];
    }
    if (*ent).speed == 0. {
        (*ent).speed = 100 as libc::c_int as libc::c_float;
    }
    if (*ent).dmg == 0 {
        (*ent).dmg = 2 as libc::c_int;
    }
    let ref mut fresh37 = (*ent).use_0;
    *fresh37 = Some(
        rotating_use
            as unsafe extern "C" fn(*mut edict_t, *mut edict_t, *mut edict_t) -> (),
    );
    if (*ent).dmg != 0 {
        let ref mut fresh38 = (*ent).blocked;
        *fresh38 = Some(
            rotating_blocked as unsafe extern "C" fn(*mut edict_t, *mut edict_t) -> (),
        );
    }
    if (*ent).spawnflags & 1 as libc::c_int != 0 {
        ((*ent).use_0)
            .expect(
                "non-null function pointer",
            )(ent, 0 as *mut edict_t, 0 as *mut edict_t);
    }
    if (*ent).spawnflags & 64 as libc::c_int != 0 {
        (*ent).s.effects |= 0x1000 as libc::c_int as libc::c_uint;
    }
    if (*ent).spawnflags & 128 as libc::c_int != 0 {
        (*ent).s.effects |= 0x2000 as libc::c_int as libc::c_uint;
    }
    (gi.setmodel).expect("non-null function pointer")(ent, (*ent).model);
    (gi.linkentity).expect("non-null function pointer")(ent);
}
#[no_mangle]
pub unsafe extern "C" fn button_done(mut self_0: *mut edict_t) {
    (*self_0).moveinfo.state = 1 as libc::c_int;
    (*self_0).s.effects &= !(0x800 as libc::c_int) as libc::c_uint;
    (*self_0).s.effects |= 0x400 as libc::c_int as libc::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn button_return(mut self_0: *mut edict_t) {
    (*self_0).moveinfo.state = 3 as libc::c_int;
    Move_Calc(
        self_0,
        ((*self_0).moveinfo.start_origin).as_mut_ptr(),
        Some(button_done as unsafe extern "C" fn(*mut edict_t) -> ()),
    );
    (*self_0).s.frame = 0 as libc::c_int;
    if (*self_0).health != 0 {
        (*self_0).takedamage = DAMAGE_YES as libc::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn button_wait(mut self_0: *mut edict_t) {
    (*self_0).moveinfo.state = 0 as libc::c_int;
    (*self_0).s.effects &= !(0x400 as libc::c_int) as libc::c_uint;
    (*self_0).s.effects |= 0x800 as libc::c_int as libc::c_uint;
    G_UseTargets(self_0, (*self_0).activator);
    (*self_0).s.frame = 1 as libc::c_int;
    if (*self_0).moveinfo.wait >= 0 as libc::c_int as libc::c_float {
        (*self_0).nextthink = level.time + (*self_0).moveinfo.wait;
        let ref mut fresh39 = (*self_0).think;
        *fresh39 = Some(button_return as unsafe extern "C" fn(*mut edict_t) -> ());
    }
}
#[no_mangle]
pub unsafe extern "C" fn button_fire(mut self_0: *mut edict_t) {
    if (*self_0).moveinfo.state == 2 as libc::c_int
        || (*self_0).moveinfo.state == 0 as libc::c_int
    {
        return;
    }
    (*self_0).moveinfo.state = 2 as libc::c_int;
    if (*self_0).moveinfo.sound_start != 0 && (*self_0).flags & 0x400 as libc::c_int == 0
    {
        (gi.sound)
            .expect(
                "non-null function pointer",
            )(
            self_0,
            8 as libc::c_int + 2 as libc::c_int,
            (*self_0).moveinfo.sound_start,
            1 as libc::c_int as libc::c_float,
            3 as libc::c_int as libc::c_float,
            0 as libc::c_int as libc::c_float,
        );
    }
    Move_Calc(
        self_0,
        ((*self_0).moveinfo.end_origin).as_mut_ptr(),
        Some(button_wait as unsafe extern "C" fn(*mut edict_t) -> ()),
    );
}
#[no_mangle]
pub unsafe extern "C" fn button_use(
    mut self_0: *mut edict_t,
    mut other: *mut edict_t,
    mut activator: *mut edict_t,
) {
    let ref mut fresh40 = (*self_0).activator;
    *fresh40 = activator;
    button_fire(self_0);
}
#[no_mangle]
pub unsafe extern "C" fn button_touch(
    mut self_0: *mut edict_t,
    mut other: *mut edict_t,
    mut plane: *mut cplane_t,
    mut surf: *mut csurface_t,
) {
    if ((*other).client).is_null() {
        return;
    }
    if (*other).health <= 0 as libc::c_int {
        return;
    }
    let ref mut fresh41 = (*self_0).activator;
    *fresh41 = other;
    button_fire(self_0);
}
#[no_mangle]
pub unsafe extern "C" fn button_killed(
    mut self_0: *mut edict_t,
    mut inflictor: *mut edict_t,
    mut attacker: *mut edict_t,
    mut damage: libc::c_int,
    mut point: *mut vec_t,
) {
    let ref mut fresh42 = (*self_0).activator;
    *fresh42 = attacker;
    (*self_0).health = (*self_0).max_health;
    (*self_0).takedamage = DAMAGE_NO as libc::c_int;
    button_fire(self_0);
}
#[no_mangle]
pub unsafe extern "C" fn SP_func_button(mut ent: *mut edict_t) {
    let mut abs_movedir: vec3_t = [0.; 3];
    let mut dist: libc::c_float = 0.;
    G_SetMovedir(((*ent).s.angles).as_mut_ptr(), ((*ent).movedir).as_mut_ptr());
    (*ent).movetype = MOVETYPE_STOP as libc::c_int;
    (*ent).solid = SOLID_BSP;
    (gi.setmodel).expect("non-null function pointer")(ent, (*ent).model);
    if (*ent).sounds != 1 as libc::c_int {
        (*ent)
            .moveinfo
            .sound_start = (gi.soundindex)
            .expect(
                "non-null function pointer",
            )(
            b"switches/butn2.wav\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    if (*ent).speed == 0. {
        (*ent).speed = 40 as libc::c_int as libc::c_float;
    }
    if (*ent).accel == 0. {
        (*ent).accel = (*ent).speed;
    }
    if (*ent).decel == 0. {
        (*ent).decel = (*ent).speed;
    }
    if (*ent).wait == 0. {
        (*ent).wait = 3 as libc::c_int as libc::c_float;
    }
    if st.lip == 0 {
        st.lip = 4 as libc::c_int;
    }
    (*ent).pos1[0 as libc::c_int as usize] = (*ent).s.origin[0 as libc::c_int as usize];
    (*ent).pos1[1 as libc::c_int as usize] = (*ent).s.origin[1 as libc::c_int as usize];
    (*ent).pos1[2 as libc::c_int as usize] = (*ent).s.origin[2 as libc::c_int as usize];
    abs_movedir[0 as libc::c_int
        as usize] = fabs((*ent).movedir[0 as libc::c_int as usize] as libc::c_double)
        as vec_t;
    abs_movedir[1 as libc::c_int
        as usize] = fabs((*ent).movedir[1 as libc::c_int as usize] as libc::c_double)
        as vec_t;
    abs_movedir[2 as libc::c_int
        as usize] = fabs((*ent).movedir[2 as libc::c_int as usize] as libc::c_double)
        as vec_t;
    dist = abs_movedir[0 as libc::c_int as usize]
        * (*ent).size[0 as libc::c_int as usize]
        + abs_movedir[1 as libc::c_int as usize] * (*ent).size[1 as libc::c_int as usize]
        + abs_movedir[2 as libc::c_int as usize] * (*ent).size[2 as libc::c_int as usize]
        - st.lip as libc::c_float;
    VectorMA(
        ((*ent).pos1).as_mut_ptr(),
        dist,
        ((*ent).movedir).as_mut_ptr(),
        ((*ent).pos2).as_mut_ptr(),
    );
    let ref mut fresh43 = (*ent).use_0;
    *fresh43 = Some(
        button_use
            as unsafe extern "C" fn(*mut edict_t, *mut edict_t, *mut edict_t) -> (),
    );
    (*ent).s.effects |= 0x400 as libc::c_int as libc::c_uint;
    if (*ent).health != 0 {
        (*ent).max_health = (*ent).health;
        let ref mut fresh44 = (*ent).die;
        *fresh44 = Some(
            button_killed
                as unsafe extern "C" fn(
                    *mut edict_t,
                    *mut edict_t,
                    *mut edict_t,
                    libc::c_int,
                    *mut vec_t,
                ) -> (),
        );
        (*ent).takedamage = DAMAGE_YES as libc::c_int;
    } else if ((*ent).targetname).is_null() {
        let ref mut fresh45 = (*ent).touch;
        *fresh45 = Some(
            button_touch
                as unsafe extern "C" fn(
                    *mut edict_t,
                    *mut edict_t,
                    *mut cplane_t,
                    *mut csurface_t,
                ) -> (),
        );
    }
    (*ent).moveinfo.state = 1 as libc::c_int;
    (*ent).moveinfo.speed = (*ent).speed;
    (*ent).moveinfo.accel = (*ent).accel;
    (*ent).moveinfo.decel = (*ent).decel;
    (*ent).moveinfo.wait = (*ent).wait;
    (*ent)
        .moveinfo
        .start_origin[0 as libc::c_int
        as usize] = (*ent).pos1[0 as libc::c_int as usize];
    (*ent)
        .moveinfo
        .start_origin[1 as libc::c_int
        as usize] = (*ent).pos1[1 as libc::c_int as usize];
    (*ent)
        .moveinfo
        .start_origin[2 as libc::c_int
        as usize] = (*ent).pos1[2 as libc::c_int as usize];
    (*ent)
        .moveinfo
        .start_angles[0 as libc::c_int
        as usize] = (*ent).s.angles[0 as libc::c_int as usize];
    (*ent)
        .moveinfo
        .start_angles[1 as libc::c_int
        as usize] = (*ent).s.angles[1 as libc::c_int as usize];
    (*ent)
        .moveinfo
        .start_angles[2 as libc::c_int
        as usize] = (*ent).s.angles[2 as libc::c_int as usize];
    (*ent)
        .moveinfo
        .end_origin[0 as libc::c_int as usize] = (*ent).pos2[0 as libc::c_int as usize];
    (*ent)
        .moveinfo
        .end_origin[1 as libc::c_int as usize] = (*ent).pos2[1 as libc::c_int as usize];
    (*ent)
        .moveinfo
        .end_origin[2 as libc::c_int as usize] = (*ent).pos2[2 as libc::c_int as usize];
    (*ent)
        .moveinfo
        .end_angles[0 as libc::c_int
        as usize] = (*ent).s.angles[0 as libc::c_int as usize];
    (*ent)
        .moveinfo
        .end_angles[1 as libc::c_int
        as usize] = (*ent).s.angles[1 as libc::c_int as usize];
    (*ent)
        .moveinfo
        .end_angles[2 as libc::c_int
        as usize] = (*ent).s.angles[2 as libc::c_int as usize];
    (gi.linkentity).expect("non-null function pointer")(ent);
}
#[no_mangle]
pub unsafe extern "C" fn door_use_areaportals(
    mut self_0: *mut edict_t,
    mut open: qboolean,
) {
    let mut t: *mut edict_t = 0 as *mut edict_t;
    if ((*self_0).target).is_null() {
        return;
    }
    loop {
        t = G_Find(
            t,
            &mut (*(0 as *mut edict_t)).targetname as *mut *mut libc::c_char
                as libc::c_int,
            (*self_0).target,
        );
        if t.is_null() {
            break;
        }
        if Q_stricmp(
            (*t).classname,
            b"func_areaportal\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) == 0 as libc::c_int
        {
            (gi.SetAreaPortalState)
                .expect("non-null function pointer")((*t).style, open);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn door_hit_top(mut self_0: *mut edict_t) {
    if (*self_0).flags & 0x400 as libc::c_int == 0 {
        if (*self_0).moveinfo.sound_end != 0 {
            (gi.sound)
                .expect(
                    "non-null function pointer",
                )(
                self_0,
                8 as libc::c_int + 2 as libc::c_int,
                (*self_0).moveinfo.sound_end,
                1 as libc::c_int as libc::c_float,
                3 as libc::c_int as libc::c_float,
                0 as libc::c_int as libc::c_float,
            );
        }
        (*self_0).s.sound = 0 as libc::c_int;
    }
    (*self_0).moveinfo.state = 0 as libc::c_int;
    if (*self_0).spawnflags & 32 as libc::c_int != 0 {
        return;
    }
    if (*self_0).moveinfo.wait >= 0 as libc::c_int as libc::c_float {
        let ref mut fresh46 = (*self_0).think;
        *fresh46 = Some(door_go_down as unsafe extern "C" fn(*mut edict_t) -> ());
        (*self_0).nextthink = level.time + (*self_0).moveinfo.wait;
    }
}
#[no_mangle]
pub unsafe extern "C" fn door_hit_bottom(mut self_0: *mut edict_t) {
    if (*self_0).flags & 0x400 as libc::c_int == 0 {
        if (*self_0).moveinfo.sound_end != 0 {
            (gi.sound)
                .expect(
                    "non-null function pointer",
                )(
                self_0,
                8 as libc::c_int + 2 as libc::c_int,
                (*self_0).moveinfo.sound_end,
                1 as libc::c_int as libc::c_float,
                3 as libc::c_int as libc::c_float,
                0 as libc::c_int as libc::c_float,
            );
        }
        (*self_0).s.sound = 0 as libc::c_int;
    }
    (*self_0).moveinfo.state = 1 as libc::c_int;
    door_use_areaportals(self_0, false_0);
}
#[no_mangle]
pub unsafe extern "C" fn door_go_down(mut self_0: *mut edict_t) {
    if (*self_0).flags & 0x400 as libc::c_int == 0 {
        if (*self_0).moveinfo.sound_start != 0 {
            (gi.sound)
                .expect(
                    "non-null function pointer",
                )(
                self_0,
                8 as libc::c_int + 2 as libc::c_int,
                (*self_0).moveinfo.sound_start,
                1 as libc::c_int as libc::c_float,
                3 as libc::c_int as libc::c_float,
                0 as libc::c_int as libc::c_float,
            );
        }
        (*self_0).s.sound = (*self_0).moveinfo.sound_middle;
    }
    if (*self_0).max_health != 0 {
        (*self_0).takedamage = DAMAGE_YES as libc::c_int;
        (*self_0).health = (*self_0).max_health;
    }
    (*self_0).moveinfo.state = 3 as libc::c_int;
    if strcmp((*self_0).classname, b"func_door\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        Move_Calc(
            self_0,
            ((*self_0).moveinfo.start_origin).as_mut_ptr(),
            Some(door_hit_bottom as unsafe extern "C" fn(*mut edict_t) -> ()),
        );
    } else if strcmp(
        (*self_0).classname,
        b"func_door_rotating\0" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int
    {
        AngleMove_Calc(
            self_0,
            Some(door_hit_bottom as unsafe extern "C" fn(*mut edict_t) -> ()),
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn door_go_up(
    mut self_0: *mut edict_t,
    mut activator: *mut edict_t,
) {
    if (*self_0).moveinfo.state == 2 as libc::c_int {
        return;
    }
    if (*self_0).moveinfo.state == 0 as libc::c_int {
        if (*self_0).moveinfo.wait >= 0 as libc::c_int as libc::c_float {
            (*self_0).nextthink = level.time + (*self_0).moveinfo.wait;
        }
        return;
    }
    if (*self_0).flags & 0x400 as libc::c_int == 0 {
        if (*self_0).moveinfo.sound_start != 0 {
            (gi.sound)
                .expect(
                    "non-null function pointer",
                )(
                self_0,
                8 as libc::c_int + 2 as libc::c_int,
                (*self_0).moveinfo.sound_start,
                1 as libc::c_int as libc::c_float,
                3 as libc::c_int as libc::c_float,
                0 as libc::c_int as libc::c_float,
            );
        }
        (*self_0).s.sound = (*self_0).moveinfo.sound_middle;
    }
    (*self_0).moveinfo.state = 2 as libc::c_int;
    if strcmp((*self_0).classname, b"func_door\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        Move_Calc(
            self_0,
            ((*self_0).moveinfo.end_origin).as_mut_ptr(),
            Some(door_hit_top as unsafe extern "C" fn(*mut edict_t) -> ()),
        );
    } else if strcmp(
        (*self_0).classname,
        b"func_door_rotating\0" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int
    {
        AngleMove_Calc(
            self_0,
            Some(door_hit_top as unsafe extern "C" fn(*mut edict_t) -> ()),
        );
    }
    G_UseTargets(self_0, activator);
    door_use_areaportals(self_0, true_0);
}
#[no_mangle]
pub unsafe extern "C" fn door_use(
    mut self_0: *mut edict_t,
    mut other: *mut edict_t,
    mut activator: *mut edict_t,
) {
    let mut ent: *mut edict_t = 0 as *mut edict_t;
    if (*self_0).flags & 0x400 as libc::c_int != 0 {
        return;
    }
    if (*self_0).spawnflags & 32 as libc::c_int != 0 {
        if (*self_0).moveinfo.state == 2 as libc::c_int
            || (*self_0).moveinfo.state == 0 as libc::c_int
        {
            ent = self_0;
            while !ent.is_null() {
                let ref mut fresh47 = (*ent).message;
                *fresh47 = 0 as *mut libc::c_char;
                let ref mut fresh48 = (*ent).touch;
                *fresh48 = None;
                door_go_down(ent);
                ent = (*ent).teamchain;
            }
            return;
        }
    }
    ent = self_0;
    while !ent.is_null() {
        let ref mut fresh49 = (*ent).message;
        *fresh49 = 0 as *mut libc::c_char;
        let ref mut fresh50 = (*ent).touch;
        *fresh50 = None;
        door_go_up(ent, activator);
        ent = (*ent).teamchain;
    }
}
#[no_mangle]
pub unsafe extern "C" fn Touch_DoorTrigger(
    mut self_0: *mut edict_t,
    mut other: *mut edict_t,
    mut plane: *mut cplane_t,
    mut surf: *mut csurface_t,
) {
    if (*other).health <= 0 as libc::c_int {
        return;
    }
    if (*other).svflags & 0x4 as libc::c_int == 0 && ((*other).client).is_null() {
        return;
    }
    if (*(*self_0).owner).spawnflags & 8 as libc::c_int != 0
        && (*other).svflags & 0x4 as libc::c_int != 0
    {
        return;
    }
    if level.time < (*self_0).touch_debounce_time {
        return;
    }
    (*self_0)
        .touch_debounce_time = (level.time as libc::c_double + 1.0f64) as libc::c_float;
    door_use((*self_0).owner, other, other);
}
#[no_mangle]
pub unsafe extern "C" fn Think_CalcMoveSpeed(mut self_0: *mut edict_t) {
    let mut ent: *mut edict_t = 0 as *mut edict_t;
    let mut min: libc::c_float = 0.;
    let mut time: libc::c_float = 0.;
    let mut newspeed: libc::c_float = 0.;
    let mut ratio: libc::c_float = 0.;
    let mut dist: libc::c_float = 0.;
    if (*self_0).flags & 0x400 as libc::c_int != 0 {
        return;
    }
    min = fabs((*self_0).moveinfo.distance as libc::c_double) as libc::c_float;
    ent = (*self_0).teamchain;
    while !ent.is_null() {
        dist = fabs((*ent).moveinfo.distance as libc::c_double) as libc::c_float;
        if dist < min {
            min = dist;
        }
        ent = (*ent).teamchain;
    }
    time = min / (*self_0).moveinfo.speed;
    ent = self_0;
    while !ent.is_null() {
        newspeed = (fabs((*ent).moveinfo.distance as libc::c_double)
            / time as libc::c_double) as libc::c_float;
        ratio = newspeed / (*ent).moveinfo.speed;
        if (*ent).moveinfo.accel == (*ent).moveinfo.speed {
            (*ent).moveinfo.accel = newspeed;
        } else {
            (*ent).moveinfo.accel *= ratio;
        }
        if (*ent).moveinfo.decel == (*ent).moveinfo.speed {
            (*ent).moveinfo.decel = newspeed;
        } else {
            (*ent).moveinfo.decel *= ratio;
        }
        (*ent).moveinfo.speed = newspeed;
        ent = (*ent).teamchain;
    }
}
#[no_mangle]
pub unsafe extern "C" fn Think_SpawnDoorTrigger(mut ent: *mut edict_t) {
    let mut other: *mut edict_t = 0 as *mut edict_t;
    let mut mins: vec3_t = [0.; 3];
    let mut maxs: vec3_t = [0.; 3];
    if (*ent).flags & 0x400 as libc::c_int != 0 {
        return;
    }
    mins[0 as libc::c_int as usize] = (*ent).absmin[0 as libc::c_int as usize];
    mins[1 as libc::c_int as usize] = (*ent).absmin[1 as libc::c_int as usize];
    mins[2 as libc::c_int as usize] = (*ent).absmin[2 as libc::c_int as usize];
    maxs[0 as libc::c_int as usize] = (*ent).absmax[0 as libc::c_int as usize];
    maxs[1 as libc::c_int as usize] = (*ent).absmax[1 as libc::c_int as usize];
    maxs[2 as libc::c_int as usize] = (*ent).absmax[2 as libc::c_int as usize];
    other = (*ent).teamchain;
    while !other.is_null() {
        AddPointToBounds(
            ((*other).absmin).as_mut_ptr(),
            mins.as_mut_ptr(),
            maxs.as_mut_ptr(),
        );
        AddPointToBounds(
            ((*other).absmax).as_mut_ptr(),
            mins.as_mut_ptr(),
            maxs.as_mut_ptr(),
        );
        other = (*other).teamchain;
    }
    mins[0 as libc::c_int as usize] -= 60 as libc::c_int as libc::c_float;
    mins[1 as libc::c_int as usize] -= 60 as libc::c_int as libc::c_float;
    maxs[0 as libc::c_int as usize] += 60 as libc::c_int as libc::c_float;
    maxs[1 as libc::c_int as usize] += 60 as libc::c_int as libc::c_float;
    other = G_Spawn();
    (*other).mins[0 as libc::c_int as usize] = mins[0 as libc::c_int as usize];
    (*other).mins[1 as libc::c_int as usize] = mins[1 as libc::c_int as usize];
    (*other).mins[2 as libc::c_int as usize] = mins[2 as libc::c_int as usize];
    (*other).maxs[0 as libc::c_int as usize] = maxs[0 as libc::c_int as usize];
    (*other).maxs[1 as libc::c_int as usize] = maxs[1 as libc::c_int as usize];
    (*other).maxs[2 as libc::c_int as usize] = maxs[2 as libc::c_int as usize];
    let ref mut fresh51 = (*other).owner;
    *fresh51 = ent;
    (*other).solid = SOLID_TRIGGER;
    (*other).movetype = MOVETYPE_NONE as libc::c_int;
    let ref mut fresh52 = (*other).touch;
    *fresh52 = Some(
        Touch_DoorTrigger
            as unsafe extern "C" fn(
                *mut edict_t,
                *mut edict_t,
                *mut cplane_t,
                *mut csurface_t,
            ) -> (),
    );
    (gi.linkentity).expect("non-null function pointer")(other);
    if (*ent).spawnflags & 1 as libc::c_int != 0 {
        door_use_areaportals(ent, true_0);
    }
    Think_CalcMoveSpeed(ent);
}
#[no_mangle]
pub unsafe extern "C" fn door_blocked(
    mut self_0: *mut edict_t,
    mut other: *mut edict_t,
) {
    let mut ent: *mut edict_t = 0 as *mut edict_t;
    if (*other).svflags & 0x4 as libc::c_int == 0 && ((*other).client).is_null() {
        T_Damage(
            other,
            self_0,
            self_0,
            vec3_origin.as_mut_ptr(),
            ((*other).s.origin).as_mut_ptr(),
            vec3_origin.as_mut_ptr(),
            100000 as libc::c_int,
            1 as libc::c_int,
            0 as libc::c_int,
            20 as libc::c_int,
        );
        if !other.is_null() {
            BecomeExplosion1(other);
        }
        return;
    }
    T_Damage(
        other,
        self_0,
        self_0,
        vec3_origin.as_mut_ptr(),
        ((*other).s.origin).as_mut_ptr(),
        vec3_origin.as_mut_ptr(),
        (*self_0).dmg,
        1 as libc::c_int,
        0 as libc::c_int,
        20 as libc::c_int,
    );
    if (*self_0).spawnflags & 4 as libc::c_int != 0 {
        return;
    }
    if (*self_0).moveinfo.wait >= 0 as libc::c_int as libc::c_float {
        if (*self_0).moveinfo.state == 3 as libc::c_int {
            ent = (*self_0).teammaster;
            while !ent.is_null() {
                door_go_up(ent, (*ent).activator);
                ent = (*ent).teamchain;
            }
        } else {
            ent = (*self_0).teammaster;
            while !ent.is_null() {
                door_go_down(ent);
                ent = (*ent).teamchain;
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn door_killed(
    mut self_0: *mut edict_t,
    mut inflictor: *mut edict_t,
    mut attacker: *mut edict_t,
    mut damage: libc::c_int,
    mut point: *mut vec_t,
) {
    let mut ent: *mut edict_t = 0 as *mut edict_t;
    ent = (*self_0).teammaster;
    while !ent.is_null() {
        (*ent).health = (*ent).max_health;
        (*ent).takedamage = DAMAGE_NO as libc::c_int;
        ent = (*ent).teamchain;
    }
    door_use((*self_0).teammaster, attacker, attacker);
}
#[no_mangle]
pub unsafe extern "C" fn door_touch(
    mut self_0: *mut edict_t,
    mut other: *mut edict_t,
    mut plane: *mut cplane_t,
    mut surf: *mut csurface_t,
) {
    if ((*other).client).is_null() {
        return;
    }
    if level.time < (*self_0).touch_debounce_time {
        return;
    }
    (*self_0)
        .touch_debounce_time = (level.time as libc::c_double + 5.0f64) as libc::c_float;
    (gi.centerprintf)
        .expect(
            "non-null function pointer",
        )(
        other,
        b"%s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        (*self_0).message,
    );
    (gi.sound)
        .expect(
            "non-null function pointer",
        )(
        other,
        0 as libc::c_int,
        (gi.soundindex)
            .expect(
                "non-null function pointer",
            )(
            b"misc/talk1.wav\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ),
        1 as libc::c_int as libc::c_float,
        1 as libc::c_int as libc::c_float,
        0 as libc::c_int as libc::c_float,
    );
}
#[no_mangle]
pub unsafe extern "C" fn SP_func_door(mut ent: *mut edict_t) {
    let mut abs_movedir: vec3_t = [0.; 3];
    if (*ent).sounds != 1 as libc::c_int {
        (*ent)
            .moveinfo
            .sound_start = (gi.soundindex)
            .expect(
                "non-null function pointer",
            )(
            b"doors/dr1_strt.wav\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        (*ent)
            .moveinfo
            .sound_middle = (gi.soundindex)
            .expect(
                "non-null function pointer",
            )(
            b"doors/dr1_mid.wav\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        (*ent)
            .moveinfo
            .sound_end = (gi.soundindex)
            .expect(
                "non-null function pointer",
            )(
            b"doors/dr1_end.wav\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    G_SetMovedir(((*ent).s.angles).as_mut_ptr(), ((*ent).movedir).as_mut_ptr());
    (*ent).movetype = MOVETYPE_PUSH as libc::c_int;
    (*ent).solid = SOLID_BSP;
    (gi.setmodel).expect("non-null function pointer")(ent, (*ent).model);
    let ref mut fresh53 = (*ent).blocked;
    *fresh53 = Some(
        door_blocked as unsafe extern "C" fn(*mut edict_t, *mut edict_t) -> (),
    );
    let ref mut fresh54 = (*ent).use_0;
    *fresh54 = Some(
        door_use as unsafe extern "C" fn(*mut edict_t, *mut edict_t, *mut edict_t) -> (),
    );
    if (*ent).speed == 0. {
        (*ent).speed = 100 as libc::c_int as libc::c_float;
    }
    if (*deathmatch).value != 0. {
        (*ent).speed *= 2 as libc::c_int as libc::c_float;
    }
    if (*ent).accel == 0. {
        (*ent).accel = (*ent).speed;
    }
    if (*ent).decel == 0. {
        (*ent).decel = (*ent).speed;
    }
    if (*ent).wait == 0. {
        (*ent).wait = 3 as libc::c_int as libc::c_float;
    }
    if st.lip == 0 {
        st.lip = 8 as libc::c_int;
    }
    if (*ent).dmg == 0 {
        (*ent).dmg = 2 as libc::c_int;
    }
    (*ent).pos1[0 as libc::c_int as usize] = (*ent).s.origin[0 as libc::c_int as usize];
    (*ent).pos1[1 as libc::c_int as usize] = (*ent).s.origin[1 as libc::c_int as usize];
    (*ent).pos1[2 as libc::c_int as usize] = (*ent).s.origin[2 as libc::c_int as usize];
    abs_movedir[0 as libc::c_int
        as usize] = fabs((*ent).movedir[0 as libc::c_int as usize] as libc::c_double)
        as vec_t;
    abs_movedir[1 as libc::c_int
        as usize] = fabs((*ent).movedir[1 as libc::c_int as usize] as libc::c_double)
        as vec_t;
    abs_movedir[2 as libc::c_int
        as usize] = fabs((*ent).movedir[2 as libc::c_int as usize] as libc::c_double)
        as vec_t;
    (*ent)
        .moveinfo
        .distance = abs_movedir[0 as libc::c_int as usize]
        * (*ent).size[0 as libc::c_int as usize]
        + abs_movedir[1 as libc::c_int as usize] * (*ent).size[1 as libc::c_int as usize]
        + abs_movedir[2 as libc::c_int as usize] * (*ent).size[2 as libc::c_int as usize]
        - st.lip as libc::c_float;
    VectorMA(
        ((*ent).pos1).as_mut_ptr(),
        (*ent).moveinfo.distance,
        ((*ent).movedir).as_mut_ptr(),
        ((*ent).pos2).as_mut_ptr(),
    );
    if (*ent).spawnflags & 1 as libc::c_int != 0 {
        (*ent)
            .s
            .origin[0 as libc::c_int as usize] = (*ent).pos2[0 as libc::c_int as usize];
        (*ent)
            .s
            .origin[1 as libc::c_int as usize] = (*ent).pos2[1 as libc::c_int as usize];
        (*ent)
            .s
            .origin[2 as libc::c_int as usize] = (*ent).pos2[2 as libc::c_int as usize];
        (*ent).pos2[0 as libc::c_int as usize] = (*ent).pos1[0 as libc::c_int as usize];
        (*ent).pos2[1 as libc::c_int as usize] = (*ent).pos1[1 as libc::c_int as usize];
        (*ent).pos2[2 as libc::c_int as usize] = (*ent).pos1[2 as libc::c_int as usize];
        (*ent)
            .pos1[0 as libc::c_int
            as usize] = (*ent).s.origin[0 as libc::c_int as usize];
        (*ent)
            .pos1[1 as libc::c_int
            as usize] = (*ent).s.origin[1 as libc::c_int as usize];
        (*ent)
            .pos1[2 as libc::c_int
            as usize] = (*ent).s.origin[2 as libc::c_int as usize];
    }
    (*ent).moveinfo.state = 1 as libc::c_int;
    if (*ent).health != 0 {
        (*ent).takedamage = DAMAGE_YES as libc::c_int;
        let ref mut fresh55 = (*ent).die;
        *fresh55 = Some(
            door_killed
                as unsafe extern "C" fn(
                    *mut edict_t,
                    *mut edict_t,
                    *mut edict_t,
                    libc::c_int,
                    *mut vec_t,
                ) -> (),
        );
        (*ent).max_health = (*ent).health;
    } else if !((*ent).targetname).is_null() && !((*ent).message).is_null() {
        (gi.soundindex)
            .expect(
                "non-null function pointer",
            )(
            b"misc/talk.wav\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        let ref mut fresh56 = (*ent).touch;
        *fresh56 = Some(
            door_touch
                as unsafe extern "C" fn(
                    *mut edict_t,
                    *mut edict_t,
                    *mut cplane_t,
                    *mut csurface_t,
                ) -> (),
        );
    }
    (*ent).moveinfo.speed = (*ent).speed;
    (*ent).moveinfo.accel = (*ent).accel;
    (*ent).moveinfo.decel = (*ent).decel;
    (*ent).moveinfo.wait = (*ent).wait;
    (*ent)
        .moveinfo
        .start_origin[0 as libc::c_int
        as usize] = (*ent).pos1[0 as libc::c_int as usize];
    (*ent)
        .moveinfo
        .start_origin[1 as libc::c_int
        as usize] = (*ent).pos1[1 as libc::c_int as usize];
    (*ent)
        .moveinfo
        .start_origin[2 as libc::c_int
        as usize] = (*ent).pos1[2 as libc::c_int as usize];
    (*ent)
        .moveinfo
        .start_angles[0 as libc::c_int
        as usize] = (*ent).s.angles[0 as libc::c_int as usize];
    (*ent)
        .moveinfo
        .start_angles[1 as libc::c_int
        as usize] = (*ent).s.angles[1 as libc::c_int as usize];
    (*ent)
        .moveinfo
        .start_angles[2 as libc::c_int
        as usize] = (*ent).s.angles[2 as libc::c_int as usize];
    (*ent)
        .moveinfo
        .end_origin[0 as libc::c_int as usize] = (*ent).pos2[0 as libc::c_int as usize];
    (*ent)
        .moveinfo
        .end_origin[1 as libc::c_int as usize] = (*ent).pos2[1 as libc::c_int as usize];
    (*ent)
        .moveinfo
        .end_origin[2 as libc::c_int as usize] = (*ent).pos2[2 as libc::c_int as usize];
    (*ent)
        .moveinfo
        .end_angles[0 as libc::c_int
        as usize] = (*ent).s.angles[0 as libc::c_int as usize];
    (*ent)
        .moveinfo
        .end_angles[1 as libc::c_int
        as usize] = (*ent).s.angles[1 as libc::c_int as usize];
    (*ent)
        .moveinfo
        .end_angles[2 as libc::c_int
        as usize] = (*ent).s.angles[2 as libc::c_int as usize];
    if (*ent).spawnflags & 16 as libc::c_int != 0 {
        (*ent).s.effects |= 0x1000 as libc::c_int as libc::c_uint;
    }
    if (*ent).spawnflags & 64 as libc::c_int != 0 {
        (*ent).s.effects |= 0x2000 as libc::c_int as libc::c_uint;
    }
    if ((*ent).team).is_null() {
        let ref mut fresh57 = (*ent).teammaster;
        *fresh57 = ent;
    }
    (gi.linkentity).expect("non-null function pointer")(ent);
    (*ent).nextthink = (level.time as libc::c_double + 0.1f64) as libc::c_float;
    if (*ent).health != 0 || !((*ent).targetname).is_null() {
        let ref mut fresh58 = (*ent).think;
        *fresh58 = Some(Think_CalcMoveSpeed as unsafe extern "C" fn(*mut edict_t) -> ());
    } else {
        let ref mut fresh59 = (*ent).think;
        *fresh59 = Some(
            Think_SpawnDoorTrigger as unsafe extern "C" fn(*mut edict_t) -> (),
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn SP_func_door_rotating(mut ent: *mut edict_t) {
    let ref mut fresh60 = (*ent).s.angles[2 as libc::c_int as usize];
    *fresh60 = 0 as libc::c_int as vec_t;
    let ref mut fresh61 = (*ent).s.angles[1 as libc::c_int as usize];
    *fresh61 = *fresh60;
    (*ent).s.angles[0 as libc::c_int as usize] = *fresh61;
    let ref mut fresh62 = (*ent).movedir[2 as libc::c_int as usize];
    *fresh62 = 0 as libc::c_int as vec_t;
    let ref mut fresh63 = (*ent).movedir[1 as libc::c_int as usize];
    *fresh63 = *fresh62;
    (*ent).movedir[0 as libc::c_int as usize] = *fresh63;
    if (*ent).spawnflags & 64 as libc::c_int != 0 {
        (*ent).movedir[2 as libc::c_int as usize] = 1.0f64 as vec_t;
    } else if (*ent).spawnflags & 128 as libc::c_int != 0 {
        (*ent).movedir[0 as libc::c_int as usize] = 1.0f64 as vec_t;
    } else {
        (*ent).movedir[1 as libc::c_int as usize] = 1.0f64 as vec_t;
    }
    if (*ent).spawnflags & 2 as libc::c_int != 0 {
        (*ent)
            .movedir[0 as libc::c_int
            as usize] = -(*ent).movedir[0 as libc::c_int as usize];
        (*ent)
            .movedir[1 as libc::c_int
            as usize] = -(*ent).movedir[1 as libc::c_int as usize];
        (*ent)
            .movedir[2 as libc::c_int
            as usize] = -(*ent).movedir[2 as libc::c_int as usize];
    }
    if st.distance == 0 {
        (gi.dprintf)
            .expect(
                "non-null function pointer",
            )(
            b"%s at %s with no distance set\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            (*ent).classname,
            vtos(((*ent).s.origin).as_mut_ptr()),
        );
        st.distance = 90 as libc::c_int;
    }
    (*ent).pos1[0 as libc::c_int as usize] = (*ent).s.angles[0 as libc::c_int as usize];
    (*ent).pos1[1 as libc::c_int as usize] = (*ent).s.angles[1 as libc::c_int as usize];
    (*ent).pos1[2 as libc::c_int as usize] = (*ent).s.angles[2 as libc::c_int as usize];
    VectorMA(
        ((*ent).s.angles).as_mut_ptr(),
        st.distance as libc::c_float,
        ((*ent).movedir).as_mut_ptr(),
        ((*ent).pos2).as_mut_ptr(),
    );
    (*ent).moveinfo.distance = st.distance as libc::c_float;
    (*ent).movetype = MOVETYPE_PUSH as libc::c_int;
    (*ent).solid = SOLID_BSP;
    (gi.setmodel).expect("non-null function pointer")(ent, (*ent).model);
    let ref mut fresh64 = (*ent).blocked;
    *fresh64 = Some(
        door_blocked as unsafe extern "C" fn(*mut edict_t, *mut edict_t) -> (),
    );
    let ref mut fresh65 = (*ent).use_0;
    *fresh65 = Some(
        door_use as unsafe extern "C" fn(*mut edict_t, *mut edict_t, *mut edict_t) -> (),
    );
    if (*ent).speed == 0. {
        (*ent).speed = 100 as libc::c_int as libc::c_float;
    }
    if (*ent).accel == 0. {
        (*ent).accel = (*ent).speed;
    }
    if (*ent).decel == 0. {
        (*ent).decel = (*ent).speed;
    }
    if (*ent).wait == 0. {
        (*ent).wait = 3 as libc::c_int as libc::c_float;
    }
    if (*ent).dmg == 0 {
        (*ent).dmg = 2 as libc::c_int;
    }
    if (*ent).sounds != 1 as libc::c_int {
        (*ent)
            .moveinfo
            .sound_start = (gi.soundindex)
            .expect(
                "non-null function pointer",
            )(
            b"doors/dr1_strt.wav\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        (*ent)
            .moveinfo
            .sound_middle = (gi.soundindex)
            .expect(
                "non-null function pointer",
            )(
            b"doors/dr1_mid.wav\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        (*ent)
            .moveinfo
            .sound_end = (gi.soundindex)
            .expect(
                "non-null function pointer",
            )(
            b"doors/dr1_end.wav\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    if (*ent).spawnflags & 1 as libc::c_int != 0 {
        (*ent)
            .s
            .angles[0 as libc::c_int as usize] = (*ent).pos2[0 as libc::c_int as usize];
        (*ent)
            .s
            .angles[1 as libc::c_int as usize] = (*ent).pos2[1 as libc::c_int as usize];
        (*ent)
            .s
            .angles[2 as libc::c_int as usize] = (*ent).pos2[2 as libc::c_int as usize];
        (*ent).pos2[0 as libc::c_int as usize] = (*ent).pos1[0 as libc::c_int as usize];
        (*ent).pos2[1 as libc::c_int as usize] = (*ent).pos1[1 as libc::c_int as usize];
        (*ent).pos2[2 as libc::c_int as usize] = (*ent).pos1[2 as libc::c_int as usize];
        (*ent)
            .pos1[0 as libc::c_int
            as usize] = (*ent).s.angles[0 as libc::c_int as usize];
        (*ent)
            .pos1[1 as libc::c_int
            as usize] = (*ent).s.angles[1 as libc::c_int as usize];
        (*ent)
            .pos1[2 as libc::c_int
            as usize] = (*ent).s.angles[2 as libc::c_int as usize];
        (*ent)
            .movedir[0 as libc::c_int
            as usize] = -(*ent).movedir[0 as libc::c_int as usize];
        (*ent)
            .movedir[1 as libc::c_int
            as usize] = -(*ent).movedir[1 as libc::c_int as usize];
        (*ent)
            .movedir[2 as libc::c_int
            as usize] = -(*ent).movedir[2 as libc::c_int as usize];
    }
    if (*ent).health != 0 {
        (*ent).takedamage = DAMAGE_YES as libc::c_int;
        let ref mut fresh66 = (*ent).die;
        *fresh66 = Some(
            door_killed
                as unsafe extern "C" fn(
                    *mut edict_t,
                    *mut edict_t,
                    *mut edict_t,
                    libc::c_int,
                    *mut vec_t,
                ) -> (),
        );
        (*ent).max_health = (*ent).health;
    }
    if !((*ent).targetname).is_null() && !((*ent).message).is_null() {
        (gi.soundindex)
            .expect(
                "non-null function pointer",
            )(
            b"misc/talk.wav\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        let ref mut fresh67 = (*ent).touch;
        *fresh67 = Some(
            door_touch
                as unsafe extern "C" fn(
                    *mut edict_t,
                    *mut edict_t,
                    *mut cplane_t,
                    *mut csurface_t,
                ) -> (),
        );
    }
    (*ent).moveinfo.state = 1 as libc::c_int;
    (*ent).moveinfo.speed = (*ent).speed;
    (*ent).moveinfo.accel = (*ent).accel;
    (*ent).moveinfo.decel = (*ent).decel;
    (*ent).moveinfo.wait = (*ent).wait;
    (*ent)
        .moveinfo
        .start_origin[0 as libc::c_int
        as usize] = (*ent).s.origin[0 as libc::c_int as usize];
    (*ent)
        .moveinfo
        .start_origin[1 as libc::c_int
        as usize] = (*ent).s.origin[1 as libc::c_int as usize];
    (*ent)
        .moveinfo
        .start_origin[2 as libc::c_int
        as usize] = (*ent).s.origin[2 as libc::c_int as usize];
    (*ent)
        .moveinfo
        .start_angles[0 as libc::c_int
        as usize] = (*ent).pos1[0 as libc::c_int as usize];
    (*ent)
        .moveinfo
        .start_angles[1 as libc::c_int
        as usize] = (*ent).pos1[1 as libc::c_int as usize];
    (*ent)
        .moveinfo
        .start_angles[2 as libc::c_int
        as usize] = (*ent).pos1[2 as libc::c_int as usize];
    (*ent)
        .moveinfo
        .end_origin[0 as libc::c_int
        as usize] = (*ent).s.origin[0 as libc::c_int as usize];
    (*ent)
        .moveinfo
        .end_origin[1 as libc::c_int
        as usize] = (*ent).s.origin[1 as libc::c_int as usize];
    (*ent)
        .moveinfo
        .end_origin[2 as libc::c_int
        as usize] = (*ent).s.origin[2 as libc::c_int as usize];
    (*ent)
        .moveinfo
        .end_angles[0 as libc::c_int as usize] = (*ent).pos2[0 as libc::c_int as usize];
    (*ent)
        .moveinfo
        .end_angles[1 as libc::c_int as usize] = (*ent).pos2[1 as libc::c_int as usize];
    (*ent)
        .moveinfo
        .end_angles[2 as libc::c_int as usize] = (*ent).pos2[2 as libc::c_int as usize];
    if (*ent).spawnflags & 16 as libc::c_int != 0 {
        (*ent).s.effects |= 0x1000 as libc::c_int as libc::c_uint;
    }
    if ((*ent).team).is_null() {
        let ref mut fresh68 = (*ent).teammaster;
        *fresh68 = ent;
    }
    (gi.linkentity).expect("non-null function pointer")(ent);
    (*ent).nextthink = (level.time as libc::c_double + 0.1f64) as libc::c_float;
    if (*ent).health != 0 || !((*ent).targetname).is_null() {
        let ref mut fresh69 = (*ent).think;
        *fresh69 = Some(Think_CalcMoveSpeed as unsafe extern "C" fn(*mut edict_t) -> ());
    } else {
        let ref mut fresh70 = (*ent).think;
        *fresh70 = Some(
            Think_SpawnDoorTrigger as unsafe extern "C" fn(*mut edict_t) -> (),
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn SP_func_water(mut self_0: *mut edict_t) {
    let mut abs_movedir: vec3_t = [0.; 3];
    G_SetMovedir(((*self_0).s.angles).as_mut_ptr(), ((*self_0).movedir).as_mut_ptr());
    (*self_0).movetype = MOVETYPE_PUSH as libc::c_int;
    (*self_0).solid = SOLID_BSP;
    (gi.setmodel).expect("non-null function pointer")(self_0, (*self_0).model);
    match (*self_0).sounds {
        1 => {
            (*self_0)
                .moveinfo
                .sound_start = (gi.soundindex)
                .expect(
                    "non-null function pointer",
                )(
                b"world/mov_watr.wav\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
            (*self_0)
                .moveinfo
                .sound_end = (gi.soundindex)
                .expect(
                    "non-null function pointer",
                )(
                b"world/stp_watr.wav\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
        }
        2 => {
            (*self_0)
                .moveinfo
                .sound_start = (gi.soundindex)
                .expect(
                    "non-null function pointer",
                )(
                b"world/mov_watr.wav\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
            (*self_0)
                .moveinfo
                .sound_end = (gi.soundindex)
                .expect(
                    "non-null function pointer",
                )(
                b"world/stp_watr.wav\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
        }
        _ => {}
    }
    (*self_0)
        .pos1[0 as libc::c_int as usize] = (*self_0).s.origin[0 as libc::c_int as usize];
    (*self_0)
        .pos1[1 as libc::c_int as usize] = (*self_0).s.origin[1 as libc::c_int as usize];
    (*self_0)
        .pos1[2 as libc::c_int as usize] = (*self_0).s.origin[2 as libc::c_int as usize];
    abs_movedir[0 as libc::c_int
        as usize] = fabs((*self_0).movedir[0 as libc::c_int as usize] as libc::c_double)
        as vec_t;
    abs_movedir[1 as libc::c_int
        as usize] = fabs((*self_0).movedir[1 as libc::c_int as usize] as libc::c_double)
        as vec_t;
    abs_movedir[2 as libc::c_int
        as usize] = fabs((*self_0).movedir[2 as libc::c_int as usize] as libc::c_double)
        as vec_t;
    (*self_0)
        .moveinfo
        .distance = abs_movedir[0 as libc::c_int as usize]
        * (*self_0).size[0 as libc::c_int as usize]
        + abs_movedir[1 as libc::c_int as usize]
            * (*self_0).size[1 as libc::c_int as usize]
        + abs_movedir[2 as libc::c_int as usize]
            * (*self_0).size[2 as libc::c_int as usize] - st.lip as libc::c_float;
    VectorMA(
        ((*self_0).pos1).as_mut_ptr(),
        (*self_0).moveinfo.distance,
        ((*self_0).movedir).as_mut_ptr(),
        ((*self_0).pos2).as_mut_ptr(),
    );
    if (*self_0).spawnflags & 1 as libc::c_int != 0 {
        (*self_0)
            .s
            .origin[0 as libc::c_int
            as usize] = (*self_0).pos2[0 as libc::c_int as usize];
        (*self_0)
            .s
            .origin[1 as libc::c_int
            as usize] = (*self_0).pos2[1 as libc::c_int as usize];
        (*self_0)
            .s
            .origin[2 as libc::c_int
            as usize] = (*self_0).pos2[2 as libc::c_int as usize];
        (*self_0)
            .pos2[0 as libc::c_int as usize] = (*self_0).pos1[0 as libc::c_int as usize];
        (*self_0)
            .pos2[1 as libc::c_int as usize] = (*self_0).pos1[1 as libc::c_int as usize];
        (*self_0)
            .pos2[2 as libc::c_int as usize] = (*self_0).pos1[2 as libc::c_int as usize];
        (*self_0)
            .pos1[0 as libc::c_int
            as usize] = (*self_0).s.origin[0 as libc::c_int as usize];
        (*self_0)
            .pos1[1 as libc::c_int
            as usize] = (*self_0).s.origin[1 as libc::c_int as usize];
        (*self_0)
            .pos1[2 as libc::c_int
            as usize] = (*self_0).s.origin[2 as libc::c_int as usize];
    }
    (*self_0)
        .moveinfo
        .start_origin[0 as libc::c_int
        as usize] = (*self_0).pos1[0 as libc::c_int as usize];
    (*self_0)
        .moveinfo
        .start_origin[1 as libc::c_int
        as usize] = (*self_0).pos1[1 as libc::c_int as usize];
    (*self_0)
        .moveinfo
        .start_origin[2 as libc::c_int
        as usize] = (*self_0).pos1[2 as libc::c_int as usize];
    (*self_0)
        .moveinfo
        .start_angles[0 as libc::c_int
        as usize] = (*self_0).s.angles[0 as libc::c_int as usize];
    (*self_0)
        .moveinfo
        .start_angles[1 as libc::c_int
        as usize] = (*self_0).s.angles[1 as libc::c_int as usize];
    (*self_0)
        .moveinfo
        .start_angles[2 as libc::c_int
        as usize] = (*self_0).s.angles[2 as libc::c_int as usize];
    (*self_0)
        .moveinfo
        .end_origin[0 as libc::c_int
        as usize] = (*self_0).pos2[0 as libc::c_int as usize];
    (*self_0)
        .moveinfo
        .end_origin[1 as libc::c_int
        as usize] = (*self_0).pos2[1 as libc::c_int as usize];
    (*self_0)
        .moveinfo
        .end_origin[2 as libc::c_int
        as usize] = (*self_0).pos2[2 as libc::c_int as usize];
    (*self_0)
        .moveinfo
        .end_angles[0 as libc::c_int
        as usize] = (*self_0).s.angles[0 as libc::c_int as usize];
    (*self_0)
        .moveinfo
        .end_angles[1 as libc::c_int
        as usize] = (*self_0).s.angles[1 as libc::c_int as usize];
    (*self_0)
        .moveinfo
        .end_angles[2 as libc::c_int
        as usize] = (*self_0).s.angles[2 as libc::c_int as usize];
    (*self_0).moveinfo.state = 1 as libc::c_int;
    if (*self_0).speed == 0. {
        (*self_0).speed = 25 as libc::c_int as libc::c_float;
    }
    let ref mut fresh71 = (*self_0).moveinfo.speed;
    *fresh71 = (*self_0).speed;
    let ref mut fresh72 = (*self_0).moveinfo.decel;
    *fresh72 = *fresh71;
    (*self_0).moveinfo.accel = *fresh72;
    if (*self_0).wait == 0. {
        (*self_0).wait = -(1 as libc::c_int) as libc::c_float;
    }
    (*self_0).moveinfo.wait = (*self_0).wait;
    let ref mut fresh73 = (*self_0).use_0;
    *fresh73 = Some(
        door_use as unsafe extern "C" fn(*mut edict_t, *mut edict_t, *mut edict_t) -> (),
    );
    if (*self_0).wait == -(1 as libc::c_int) as libc::c_float {
        (*self_0).spawnflags |= 32 as libc::c_int;
    }
    let ref mut fresh74 = (*self_0).classname;
    *fresh74 = b"func_door\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    (gi.linkentity).expect("non-null function pointer")(self_0);
}
#[no_mangle]
pub unsafe extern "C" fn train_blocked(
    mut self_0: *mut edict_t,
    mut other: *mut edict_t,
) {
    if (*other).svflags & 0x4 as libc::c_int == 0 && ((*other).client).is_null() {
        T_Damage(
            other,
            self_0,
            self_0,
            vec3_origin.as_mut_ptr(),
            ((*other).s.origin).as_mut_ptr(),
            vec3_origin.as_mut_ptr(),
            100000 as libc::c_int,
            1 as libc::c_int,
            0 as libc::c_int,
            20 as libc::c_int,
        );
        if !other.is_null() {
            BecomeExplosion1(other);
        }
        return;
    }
    if level.time < (*self_0).touch_debounce_time {
        return;
    }
    if (*self_0).dmg == 0 {
        return;
    }
    (*self_0)
        .touch_debounce_time = (level.time as libc::c_double + 0.5f64) as libc::c_float;
    T_Damage(
        other,
        self_0,
        self_0,
        vec3_origin.as_mut_ptr(),
        ((*other).s.origin).as_mut_ptr(),
        vec3_origin.as_mut_ptr(),
        (*self_0).dmg,
        1 as libc::c_int,
        0 as libc::c_int,
        20 as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn train_wait(mut self_0: *mut edict_t) {
    if !((*(*self_0).target_ent).pathtarget).is_null() {
        let mut savetarget: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut ent: *mut edict_t = 0 as *mut edict_t;
        ent = (*self_0).target_ent;
        savetarget = (*ent).target;
        let ref mut fresh75 = (*ent).target;
        *fresh75 = (*ent).pathtarget;
        G_UseTargets(ent, (*self_0).activator);
        let ref mut fresh76 = (*ent).target;
        *fresh76 = savetarget;
        if (*self_0).inuse as u64 == 0 {
            return;
        }
    }
    if (*self_0).moveinfo.wait != 0. {
        if (*self_0).moveinfo.wait > 0 as libc::c_int as libc::c_float {
            (*self_0).nextthink = level.time + (*self_0).moveinfo.wait;
            let ref mut fresh77 = (*self_0).think;
            *fresh77 = Some(train_next as unsafe extern "C" fn(*mut edict_t) -> ());
        } else if (*self_0).spawnflags & 2 as libc::c_int != 0 {
            train_next(self_0);
            (*self_0).spawnflags &= !(1 as libc::c_int);
            let ref mut fresh78 = (*self_0).velocity[2 as libc::c_int as usize];
            *fresh78 = 0 as libc::c_int as vec_t;
            let ref mut fresh79 = (*self_0).velocity[1 as libc::c_int as usize];
            *fresh79 = *fresh78;
            (*self_0).velocity[0 as libc::c_int as usize] = *fresh79;
            (*self_0).nextthink = 0 as libc::c_int as libc::c_float;
        }
        if (*self_0).flags & 0x400 as libc::c_int == 0 {
            if (*self_0).moveinfo.sound_end != 0 {
                (gi.sound)
                    .expect(
                        "non-null function pointer",
                    )(
                    self_0,
                    8 as libc::c_int + 2 as libc::c_int,
                    (*self_0).moveinfo.sound_end,
                    1 as libc::c_int as libc::c_float,
                    3 as libc::c_int as libc::c_float,
                    0 as libc::c_int as libc::c_float,
                );
            }
            (*self_0).s.sound = 0 as libc::c_int;
        }
    } else {
        train_next(self_0);
    };
}
#[no_mangle]
pub unsafe extern "C" fn train_next(mut self_0: *mut edict_t) {
    let mut ent: *mut edict_t = 0 as *mut edict_t;
    let mut dest: vec3_t = [0.; 3];
    let mut first: qboolean = false_0;
    first = true_0;
    loop {
        if ((*self_0).target).is_null() {
            return;
        }
        ent = G_PickTarget((*self_0).target);
        if ent.is_null() {
            (gi.dprintf)
                .expect(
                    "non-null function pointer",
                )(
                b"train_next: bad target %s\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                (*self_0).target,
            );
            return;
        }
        let ref mut fresh80 = (*self_0).target;
        *fresh80 = (*ent).target;
        if !((*ent).spawnflags & 1 as libc::c_int != 0) {
            break;
        }
        if first as u64 == 0 {
            (gi.dprintf)
                .expect(
                    "non-null function pointer",
                )(
                b"connected teleport path_corners, see %s at %s\n\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                (*ent).classname,
                vtos(((*ent).s.origin).as_mut_ptr()),
            );
            return;
        }
        first = false_0;
        (*self_0)
            .s
            .origin[0 as libc::c_int
            as usize] = (*ent).s.origin[0 as libc::c_int as usize]
            - (*self_0).mins[0 as libc::c_int as usize];
        (*self_0)
            .s
            .origin[1 as libc::c_int
            as usize] = (*ent).s.origin[1 as libc::c_int as usize]
            - (*self_0).mins[1 as libc::c_int as usize];
        (*self_0)
            .s
            .origin[2 as libc::c_int
            as usize] = (*ent).s.origin[2 as libc::c_int as usize]
            - (*self_0).mins[2 as libc::c_int as usize];
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
        (gi.linkentity).expect("non-null function pointer")(self_0);
    }
    (*self_0).moveinfo.wait = (*ent).wait;
    let ref mut fresh81 = (*self_0).target_ent;
    *fresh81 = ent;
    if (*self_0).flags & 0x400 as libc::c_int == 0 {
        if (*self_0).moveinfo.sound_start != 0 {
            (gi.sound)
                .expect(
                    "non-null function pointer",
                )(
                self_0,
                8 as libc::c_int + 2 as libc::c_int,
                (*self_0).moveinfo.sound_start,
                1 as libc::c_int as libc::c_float,
                3 as libc::c_int as libc::c_float,
                0 as libc::c_int as libc::c_float,
            );
        }
        (*self_0).s.sound = (*self_0).moveinfo.sound_middle;
    }
    dest[0 as libc::c_int
        as usize] = (*ent).s.origin[0 as libc::c_int as usize]
        - (*self_0).mins[0 as libc::c_int as usize];
    dest[1 as libc::c_int
        as usize] = (*ent).s.origin[1 as libc::c_int as usize]
        - (*self_0).mins[1 as libc::c_int as usize];
    dest[2 as libc::c_int
        as usize] = (*ent).s.origin[2 as libc::c_int as usize]
        - (*self_0).mins[2 as libc::c_int as usize];
    (*self_0).moveinfo.state = 0 as libc::c_int;
    (*self_0)
        .moveinfo
        .start_origin[0 as libc::c_int
        as usize] = (*self_0).s.origin[0 as libc::c_int as usize];
    (*self_0)
        .moveinfo
        .start_origin[1 as libc::c_int
        as usize] = (*self_0).s.origin[1 as libc::c_int as usize];
    (*self_0)
        .moveinfo
        .start_origin[2 as libc::c_int
        as usize] = (*self_0).s.origin[2 as libc::c_int as usize];
    (*self_0)
        .moveinfo
        .end_origin[0 as libc::c_int as usize] = dest[0 as libc::c_int as usize];
    (*self_0)
        .moveinfo
        .end_origin[1 as libc::c_int as usize] = dest[1 as libc::c_int as usize];
    (*self_0)
        .moveinfo
        .end_origin[2 as libc::c_int as usize] = dest[2 as libc::c_int as usize];
    Move_Calc(
        self_0,
        dest.as_mut_ptr(),
        Some(train_wait as unsafe extern "C" fn(*mut edict_t) -> ()),
    );
    (*self_0).spawnflags |= 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn train_resume(mut self_0: *mut edict_t) {
    let mut ent: *mut edict_t = 0 as *mut edict_t;
    let mut dest: vec3_t = [0.; 3];
    ent = (*self_0).target_ent;
    dest[0 as libc::c_int
        as usize] = (*ent).s.origin[0 as libc::c_int as usize]
        - (*self_0).mins[0 as libc::c_int as usize];
    dest[1 as libc::c_int
        as usize] = (*ent).s.origin[1 as libc::c_int as usize]
        - (*self_0).mins[1 as libc::c_int as usize];
    dest[2 as libc::c_int
        as usize] = (*ent).s.origin[2 as libc::c_int as usize]
        - (*self_0).mins[2 as libc::c_int as usize];
    (*self_0).moveinfo.state = 0 as libc::c_int;
    (*self_0)
        .moveinfo
        .start_origin[0 as libc::c_int
        as usize] = (*self_0).s.origin[0 as libc::c_int as usize];
    (*self_0)
        .moveinfo
        .start_origin[1 as libc::c_int
        as usize] = (*self_0).s.origin[1 as libc::c_int as usize];
    (*self_0)
        .moveinfo
        .start_origin[2 as libc::c_int
        as usize] = (*self_0).s.origin[2 as libc::c_int as usize];
    (*self_0)
        .moveinfo
        .end_origin[0 as libc::c_int as usize] = dest[0 as libc::c_int as usize];
    (*self_0)
        .moveinfo
        .end_origin[1 as libc::c_int as usize] = dest[1 as libc::c_int as usize];
    (*self_0)
        .moveinfo
        .end_origin[2 as libc::c_int as usize] = dest[2 as libc::c_int as usize];
    Move_Calc(
        self_0,
        dest.as_mut_ptr(),
        Some(train_wait as unsafe extern "C" fn(*mut edict_t) -> ()),
    );
    (*self_0).spawnflags |= 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn func_train_find(mut self_0: *mut edict_t) {
    let mut ent: *mut edict_t = 0 as *mut edict_t;
    if ((*self_0).target).is_null() {
        (gi.dprintf)
            .expect(
                "non-null function pointer",
            )(
            b"train_find: no target\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return;
    }
    ent = G_PickTarget((*self_0).target);
    if ent.is_null() {
        (gi.dprintf)
            .expect(
                "non-null function pointer",
            )(
            b"train_find: target %s not found\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            (*self_0).target,
        );
        return;
    }
    let ref mut fresh82 = (*self_0).target;
    *fresh82 = (*ent).target;
    (*self_0)
        .s
        .origin[0 as libc::c_int
        as usize] = (*ent).s.origin[0 as libc::c_int as usize]
        - (*self_0).mins[0 as libc::c_int as usize];
    (*self_0)
        .s
        .origin[1 as libc::c_int
        as usize] = (*ent).s.origin[1 as libc::c_int as usize]
        - (*self_0).mins[1 as libc::c_int as usize];
    (*self_0)
        .s
        .origin[2 as libc::c_int
        as usize] = (*ent).s.origin[2 as libc::c_int as usize]
        - (*self_0).mins[2 as libc::c_int as usize];
    (gi.linkentity).expect("non-null function pointer")(self_0);
    if ((*self_0).targetname).is_null() {
        (*self_0).spawnflags |= 1 as libc::c_int;
    }
    if (*self_0).spawnflags & 1 as libc::c_int != 0 {
        (*self_0).nextthink = (level.time as libc::c_double + 0.1f64) as libc::c_float;
        let ref mut fresh83 = (*self_0).think;
        *fresh83 = Some(train_next as unsafe extern "C" fn(*mut edict_t) -> ());
        let ref mut fresh84 = (*self_0).activator;
        *fresh84 = self_0;
    }
}
#[no_mangle]
pub unsafe extern "C" fn train_use(
    mut self_0: *mut edict_t,
    mut other: *mut edict_t,
    mut activator: *mut edict_t,
) {
    let ref mut fresh85 = (*self_0).activator;
    *fresh85 = activator;
    if (*self_0).spawnflags & 1 as libc::c_int != 0 {
        if (*self_0).spawnflags & 2 as libc::c_int == 0 {
            return;
        }
        (*self_0).spawnflags &= !(1 as libc::c_int);
        let ref mut fresh86 = (*self_0).velocity[2 as libc::c_int as usize];
        *fresh86 = 0 as libc::c_int as vec_t;
        let ref mut fresh87 = (*self_0).velocity[1 as libc::c_int as usize];
        *fresh87 = *fresh86;
        (*self_0).velocity[0 as libc::c_int as usize] = *fresh87;
        (*self_0).nextthink = 0 as libc::c_int as libc::c_float;
    } else if !((*self_0).target_ent).is_null() {
        train_resume(self_0);
    } else {
        train_next(self_0);
    };
}
#[no_mangle]
pub unsafe extern "C" fn SP_func_train(mut self_0: *mut edict_t) {
    (*self_0).movetype = MOVETYPE_PUSH as libc::c_int;
    let ref mut fresh88 = (*self_0).s.angles[2 as libc::c_int as usize];
    *fresh88 = 0 as libc::c_int as vec_t;
    let ref mut fresh89 = (*self_0).s.angles[1 as libc::c_int as usize];
    *fresh89 = *fresh88;
    (*self_0).s.angles[0 as libc::c_int as usize] = *fresh89;
    let ref mut fresh90 = (*self_0).blocked;
    *fresh90 = Some(
        train_blocked as unsafe extern "C" fn(*mut edict_t, *mut edict_t) -> (),
    );
    if (*self_0).spawnflags & 4 as libc::c_int != 0 {
        (*self_0).dmg = 0 as libc::c_int;
    } else if (*self_0).dmg == 0 {
        (*self_0).dmg = 100 as libc::c_int;
    }
    (*self_0).solid = SOLID_BSP;
    (gi.setmodel).expect("non-null function pointer")(self_0, (*self_0).model);
    if !(st.noise).is_null() {
        (*self_0)
            .moveinfo
            .sound_middle = (gi.soundindex)
            .expect("non-null function pointer")(st.noise);
    }
    if (*self_0).speed == 0. {
        (*self_0).speed = 100 as libc::c_int as libc::c_float;
    }
    (*self_0).moveinfo.speed = (*self_0).speed;
    let ref mut fresh91 = (*self_0).moveinfo.decel;
    *fresh91 = (*self_0).moveinfo.speed;
    (*self_0).moveinfo.accel = *fresh91;
    let ref mut fresh92 = (*self_0).use_0;
    *fresh92 = Some(
        train_use as unsafe extern "C" fn(*mut edict_t, *mut edict_t, *mut edict_t) -> (),
    );
    (gi.linkentity).expect("non-null function pointer")(self_0);
    if !((*self_0).target).is_null() {
        (*self_0).nextthink = (level.time as libc::c_double + 0.1f64) as libc::c_float;
        let ref mut fresh93 = (*self_0).think;
        *fresh93 = Some(func_train_find as unsafe extern "C" fn(*mut edict_t) -> ());
    } else {
        (gi.dprintf)
            .expect(
                "non-null function pointer",
            )(
            b"func_train without a target at %s\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            vtos(((*self_0).absmin).as_mut_ptr()),
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn trigger_elevator_use(
    mut self_0: *mut edict_t,
    mut other: *mut edict_t,
    mut activator: *mut edict_t,
) {
    let mut target: *mut edict_t = 0 as *mut edict_t;
    if (*(*self_0).movetarget).nextthink != 0. {
        return;
    }
    if ((*other).pathtarget).is_null() {
        (gi.dprintf)
            .expect(
                "non-null function pointer",
            )(
            b"elevator used with no pathtarget\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return;
    }
    target = G_PickTarget((*other).pathtarget);
    if target.is_null() {
        (gi.dprintf)
            .expect(
                "non-null function pointer",
            )(
            b"elevator used with bad pathtarget: %s\n\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            (*other).pathtarget,
        );
        return;
    }
    let ref mut fresh94 = (*(*self_0).movetarget).target_ent;
    *fresh94 = target;
    train_resume((*self_0).movetarget);
}
#[no_mangle]
pub unsafe extern "C" fn trigger_elevator_init(mut self_0: *mut edict_t) {
    if ((*self_0).target).is_null() {
        (gi.dprintf)
            .expect(
                "non-null function pointer",
            )(
            b"trigger_elevator has no target\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return;
    }
    let ref mut fresh95 = (*self_0).movetarget;
    *fresh95 = G_PickTarget((*self_0).target);
    if ((*self_0).movetarget).is_null() {
        (gi.dprintf)
            .expect(
                "non-null function pointer",
            )(
            b"trigger_elevator unable to find target %s\n\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            (*self_0).target,
        );
        return;
    }
    if strcmp(
        (*(*self_0).movetarget).classname,
        b"func_train\0" as *const u8 as *const libc::c_char,
    ) != 0 as libc::c_int
    {
        (gi.dprintf)
            .expect(
                "non-null function pointer",
            )(
            b"trigger_elevator target %s is not a train\n\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            (*self_0).target,
        );
        return;
    }
    let ref mut fresh96 = (*self_0).use_0;
    *fresh96 = Some(
        trigger_elevator_use
            as unsafe extern "C" fn(*mut edict_t, *mut edict_t, *mut edict_t) -> (),
    );
    (*self_0).svflags = 0x1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn SP_trigger_elevator(mut self_0: *mut edict_t) {
    let ref mut fresh97 = (*self_0).think;
    *fresh97 = Some(trigger_elevator_init as unsafe extern "C" fn(*mut edict_t) -> ());
    (*self_0).nextthink = (level.time as libc::c_double + 0.1f64) as libc::c_float;
}
#[no_mangle]
pub unsafe extern "C" fn func_timer_think(mut self_0: *mut edict_t) {
    G_UseTargets(self_0, (*self_0).activator);
    (*self_0)
        .nextthink = ((level.time + (*self_0).wait) as libc::c_double
        + 2.0f64
            * (((rand() & 0x7fff as libc::c_int) as libc::c_float
                / 0x7fff as libc::c_int as libc::c_float) as libc::c_double - 0.5f64)
            * (*self_0).random as libc::c_double) as libc::c_float;
}
#[no_mangle]
pub unsafe extern "C" fn func_timer_use(
    mut self_0: *mut edict_t,
    mut other: *mut edict_t,
    mut activator: *mut edict_t,
) {
    let ref mut fresh98 = (*self_0).activator;
    *fresh98 = activator;
    if (*self_0).nextthink != 0. {
        (*self_0).nextthink = 0 as libc::c_int as libc::c_float;
        return;
    }
    if (*self_0).delay != 0. {
        (*self_0).nextthink = level.time + (*self_0).delay;
    } else {
        func_timer_think(self_0);
    };
}
#[no_mangle]
pub unsafe extern "C" fn SP_func_timer(mut self_0: *mut edict_t) {
    if (*self_0).wait == 0. {
        (*self_0).wait = 1.0f64 as libc::c_float;
    }
    let ref mut fresh99 = (*self_0).use_0;
    *fresh99 = Some(
        func_timer_use
            as unsafe extern "C" fn(*mut edict_t, *mut edict_t, *mut edict_t) -> (),
    );
    let ref mut fresh100 = (*self_0).think;
    *fresh100 = Some(func_timer_think as unsafe extern "C" fn(*mut edict_t) -> ());
    if (*self_0).random >= (*self_0).wait {
        (*self_0).random = ((*self_0).wait as libc::c_double - 0.1f64) as libc::c_float;
        (gi.dprintf)
            .expect(
                "non-null function pointer",
            )(
            b"func_timer at %s has random >= wait\n\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            vtos(((*self_0).s.origin).as_mut_ptr()),
        );
    }
    if (*self_0).spawnflags & 1 as libc::c_int != 0 {
        (*self_0)
            .nextthink = (level.time as libc::c_double + 1.0f64
            + st.pausetime as libc::c_double + (*self_0).delay as libc::c_double
            + (*self_0).wait as libc::c_double
            + 2.0f64
                * (((rand() & 0x7fff as libc::c_int) as libc::c_float
                    / 0x7fff as libc::c_int as libc::c_float) as libc::c_double - 0.5f64)
                * (*self_0).random as libc::c_double) as libc::c_float;
        let ref mut fresh101 = (*self_0).activator;
        *fresh101 = self_0;
    }
    (*self_0).svflags = 0x1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn func_conveyor_use(
    mut self_0: *mut edict_t,
    mut other: *mut edict_t,
    mut activator: *mut edict_t,
) {
    if (*self_0).spawnflags & 1 as libc::c_int != 0 {
        (*self_0).speed = 0 as libc::c_int as libc::c_float;
        (*self_0).spawnflags &= !(1 as libc::c_int);
    } else {
        (*self_0).speed = (*self_0).count as libc::c_float;
        (*self_0).spawnflags |= 1 as libc::c_int;
    }
    if (*self_0).spawnflags & 2 as libc::c_int == 0 {
        (*self_0).count = 0 as libc::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn SP_func_conveyor(mut self_0: *mut edict_t) {
    if (*self_0).speed == 0. {
        (*self_0).speed = 100 as libc::c_int as libc::c_float;
    }
    if (*self_0).spawnflags & 1 as libc::c_int == 0 {
        (*self_0).count = (*self_0).speed as libc::c_int;
        (*self_0).speed = 0 as libc::c_int as libc::c_float;
    }
    let ref mut fresh102 = (*self_0).use_0;
    *fresh102 = Some(
        func_conveyor_use
            as unsafe extern "C" fn(*mut edict_t, *mut edict_t, *mut edict_t) -> (),
    );
    (gi.setmodel).expect("non-null function pointer")(self_0, (*self_0).model);
    (*self_0).solid = SOLID_BSP;
    (gi.linkentity).expect("non-null function pointer")(self_0);
}
#[no_mangle]
pub unsafe extern "C" fn door_secret_use(
    mut self_0: *mut edict_t,
    mut other: *mut edict_t,
    mut activator: *mut edict_t,
) {
    if VectorCompare(((*self_0).s.origin).as_mut_ptr(), vec3_origin.as_mut_ptr()) == 0 {
        return;
    }
    Move_Calc(
        self_0,
        ((*self_0).pos1).as_mut_ptr(),
        Some(door_secret_move1 as unsafe extern "C" fn(*mut edict_t) -> ()),
    );
    door_use_areaportals(self_0, true_0);
}
#[no_mangle]
pub unsafe extern "C" fn door_secret_move1(mut self_0: *mut edict_t) {
    (*self_0).nextthink = (level.time as libc::c_double + 1.0f64) as libc::c_float;
    let ref mut fresh103 = (*self_0).think;
    *fresh103 = Some(door_secret_move2 as unsafe extern "C" fn(*mut edict_t) -> ());
}
#[no_mangle]
pub unsafe extern "C" fn door_secret_move2(mut self_0: *mut edict_t) {
    Move_Calc(
        self_0,
        ((*self_0).pos2).as_mut_ptr(),
        Some(door_secret_move3 as unsafe extern "C" fn(*mut edict_t) -> ()),
    );
}
#[no_mangle]
pub unsafe extern "C" fn door_secret_move3(mut self_0: *mut edict_t) {
    if (*self_0).wait == -(1 as libc::c_int) as libc::c_float {
        return;
    }
    (*self_0).nextthink = level.time + (*self_0).wait;
    let ref mut fresh104 = (*self_0).think;
    *fresh104 = Some(door_secret_move4 as unsafe extern "C" fn(*mut edict_t) -> ());
}
#[no_mangle]
pub unsafe extern "C" fn door_secret_move4(mut self_0: *mut edict_t) {
    Move_Calc(
        self_0,
        ((*self_0).pos1).as_mut_ptr(),
        Some(door_secret_move5 as unsafe extern "C" fn(*mut edict_t) -> ()),
    );
}
#[no_mangle]
pub unsafe extern "C" fn door_secret_move5(mut self_0: *mut edict_t) {
    (*self_0).nextthink = (level.time as libc::c_double + 1.0f64) as libc::c_float;
    let ref mut fresh105 = (*self_0).think;
    *fresh105 = Some(door_secret_move6 as unsafe extern "C" fn(*mut edict_t) -> ());
}
#[no_mangle]
pub unsafe extern "C" fn door_secret_move6(mut self_0: *mut edict_t) {
    Move_Calc(
        self_0,
        vec3_origin.as_mut_ptr(),
        Some(door_secret_done as unsafe extern "C" fn(*mut edict_t) -> ()),
    );
}
#[no_mangle]
pub unsafe extern "C" fn door_secret_done(mut self_0: *mut edict_t) {
    if ((*self_0).targetname).is_null() || (*self_0).spawnflags & 1 as libc::c_int != 0 {
        (*self_0).health = 0 as libc::c_int;
        (*self_0).takedamage = DAMAGE_YES as libc::c_int;
    }
    door_use_areaportals(self_0, false_0);
}
#[no_mangle]
pub unsafe extern "C" fn door_secret_blocked(
    mut self_0: *mut edict_t,
    mut other: *mut edict_t,
) {
    if (*other).svflags & 0x4 as libc::c_int == 0 && ((*other).client).is_null() {
        T_Damage(
            other,
            self_0,
            self_0,
            vec3_origin.as_mut_ptr(),
            ((*other).s.origin).as_mut_ptr(),
            vec3_origin.as_mut_ptr(),
            100000 as libc::c_int,
            1 as libc::c_int,
            0 as libc::c_int,
            20 as libc::c_int,
        );
        if !other.is_null() {
            BecomeExplosion1(other);
        }
        return;
    }
    if level.time < (*self_0).touch_debounce_time {
        return;
    }
    (*self_0)
        .touch_debounce_time = (level.time as libc::c_double + 0.5f64) as libc::c_float;
    T_Damage(
        other,
        self_0,
        self_0,
        vec3_origin.as_mut_ptr(),
        ((*other).s.origin).as_mut_ptr(),
        vec3_origin.as_mut_ptr(),
        (*self_0).dmg,
        1 as libc::c_int,
        0 as libc::c_int,
        20 as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn door_secret_die(
    mut self_0: *mut edict_t,
    mut inflictor: *mut edict_t,
    mut attacker: *mut edict_t,
    mut damage: libc::c_int,
    mut point: *mut vec_t,
) {
    (*self_0).takedamage = DAMAGE_NO as libc::c_int;
    door_secret_use(self_0, attacker, attacker);
}
#[no_mangle]
pub unsafe extern "C" fn SP_func_door_secret(mut ent: *mut edict_t) {
    let mut forward: vec3_t = [0.; 3];
    let mut right: vec3_t = [0.; 3];
    let mut up: vec3_t = [0.; 3];
    let mut side: libc::c_float = 0.;
    let mut width: libc::c_float = 0.;
    let mut length: libc::c_float = 0.;
    (*ent)
        .moveinfo
        .sound_start = (gi.soundindex)
        .expect(
            "non-null function pointer",
        )(
        b"doors/dr1_strt.wav\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    (*ent)
        .moveinfo
        .sound_middle = (gi.soundindex)
        .expect(
            "non-null function pointer",
        )(
        b"doors/dr1_mid.wav\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    (*ent)
        .moveinfo
        .sound_end = (gi.soundindex)
        .expect(
            "non-null function pointer",
        )(
        b"doors/dr1_end.wav\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    (*ent).movetype = MOVETYPE_PUSH as libc::c_int;
    (*ent).solid = SOLID_BSP;
    (gi.setmodel).expect("non-null function pointer")(ent, (*ent).model);
    let ref mut fresh106 = (*ent).blocked;
    *fresh106 = Some(
        door_secret_blocked as unsafe extern "C" fn(*mut edict_t, *mut edict_t) -> (),
    );
    let ref mut fresh107 = (*ent).use_0;
    *fresh107 = Some(
        door_secret_use
            as unsafe extern "C" fn(*mut edict_t, *mut edict_t, *mut edict_t) -> (),
    );
    if ((*ent).targetname).is_null() || (*ent).spawnflags & 1 as libc::c_int != 0 {
        (*ent).health = 0 as libc::c_int;
        (*ent).takedamage = DAMAGE_YES as libc::c_int;
        let ref mut fresh108 = (*ent).die;
        *fresh108 = Some(
            door_secret_die
                as unsafe extern "C" fn(
                    *mut edict_t,
                    *mut edict_t,
                    *mut edict_t,
                    libc::c_int,
                    *mut vec_t,
                ) -> (),
        );
    }
    if (*ent).dmg == 0 {
        (*ent).dmg = 2 as libc::c_int;
    }
    if (*ent).wait == 0. {
        (*ent).wait = 5 as libc::c_int as libc::c_float;
    }
    let ref mut fresh109 = (*ent).moveinfo.speed;
    *fresh109 = 50 as libc::c_int as libc::c_float;
    let ref mut fresh110 = (*ent).moveinfo.decel;
    *fresh110 = *fresh109;
    (*ent).moveinfo.accel = *fresh110;
    AngleVectors(
        ((*ent).s.angles).as_mut_ptr(),
        forward.as_mut_ptr(),
        right.as_mut_ptr(),
        up.as_mut_ptr(),
    );
    let ref mut fresh111 = (*ent).s.angles[2 as libc::c_int as usize];
    *fresh111 = 0 as libc::c_int as vec_t;
    let ref mut fresh112 = (*ent).s.angles[1 as libc::c_int as usize];
    *fresh112 = *fresh111;
    (*ent).s.angles[0 as libc::c_int as usize] = *fresh112;
    side = (1.0f64 - ((*ent).spawnflags & 2 as libc::c_int) as libc::c_double)
        as libc::c_float;
    if (*ent).spawnflags & 4 as libc::c_int != 0 {
        width = fabs(
            (up[0 as libc::c_int as usize] * (*ent).size[0 as libc::c_int as usize]
                + up[1 as libc::c_int as usize] * (*ent).size[1 as libc::c_int as usize]
                + up[2 as libc::c_int as usize] * (*ent).size[2 as libc::c_int as usize])
                as libc::c_double,
        ) as libc::c_float;
    } else {
        width = fabs(
            (right[0 as libc::c_int as usize] * (*ent).size[0 as libc::c_int as usize]
                + right[1 as libc::c_int as usize]
                    * (*ent).size[1 as libc::c_int as usize]
                + right[2 as libc::c_int as usize]
                    * (*ent).size[2 as libc::c_int as usize]) as libc::c_double,
        ) as libc::c_float;
    }
    length = fabs(
        (forward[0 as libc::c_int as usize] * (*ent).size[0 as libc::c_int as usize]
            + forward[1 as libc::c_int as usize] * (*ent).size[1 as libc::c_int as usize]
            + forward[2 as libc::c_int as usize]
                * (*ent).size[2 as libc::c_int as usize]) as libc::c_double,
    ) as libc::c_float;
    if (*ent).spawnflags & 4 as libc::c_int != 0 {
        VectorMA(
            ((*ent).s.origin).as_mut_ptr(),
            -(1 as libc::c_int) as libc::c_float * width,
            up.as_mut_ptr(),
            ((*ent).pos1).as_mut_ptr(),
        );
    } else {
        VectorMA(
            ((*ent).s.origin).as_mut_ptr(),
            side * width,
            right.as_mut_ptr(),
            ((*ent).pos1).as_mut_ptr(),
        );
    }
    VectorMA(
        ((*ent).pos1).as_mut_ptr(),
        length,
        forward.as_mut_ptr(),
        ((*ent).pos2).as_mut_ptr(),
    );
    if (*ent).health != 0 {
        (*ent).takedamage = DAMAGE_YES as libc::c_int;
        let ref mut fresh113 = (*ent).die;
        *fresh113 = Some(
            door_killed
                as unsafe extern "C" fn(
                    *mut edict_t,
                    *mut edict_t,
                    *mut edict_t,
                    libc::c_int,
                    *mut vec_t,
                ) -> (),
        );
        (*ent).max_health = (*ent).health;
    } else if !((*ent).targetname).is_null() && !((*ent).message).is_null() {
        (gi.soundindex)
            .expect(
                "non-null function pointer",
            )(
            b"misc/talk.wav\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        let ref mut fresh114 = (*ent).touch;
        *fresh114 = Some(
            door_touch
                as unsafe extern "C" fn(
                    *mut edict_t,
                    *mut edict_t,
                    *mut cplane_t,
                    *mut csurface_t,
                ) -> (),
        );
    }
    let ref mut fresh115 = (*ent).classname;
    *fresh115 = b"func_door\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    (gi.linkentity).expect("non-null function pointer")(ent);
}
#[no_mangle]
pub unsafe extern "C" fn use_killbox(
    mut self_0: *mut edict_t,
    mut other: *mut edict_t,
    mut activator: *mut edict_t,
) {
    KillBox(self_0);
}
#[no_mangle]
pub unsafe extern "C" fn SP_func_killbox(mut ent: *mut edict_t) {
    (gi.setmodel).expect("non-null function pointer")(ent, (*ent).model);
    let ref mut fresh116 = (*ent).use_0;
    *fresh116 = Some(
        use_killbox
            as unsafe extern "C" fn(*mut edict_t, *mut edict_t, *mut edict_t) -> (),
    );
    (*ent).svflags = 0x1 as libc::c_int;
}
