#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn sin(_: libc::c_double) -> libc::c_double;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn rand() -> libc::c_int;
    fn abs(_: libc::c_int) -> libc::c_int;
    static mut vec3_origin: vec3_t;
    fn VectorMA(
        veca: *mut vec_t,
        scale: libc::c_float,
        vecb: *mut vec_t,
        vecc: *mut vec_t,
    );
    fn VectorNormalize(v: *mut vec_t) -> vec_t;
    fn AngleVectors(
        angles: *mut vec_t,
        forward_0: *mut vec_t,
        right_0: *mut vec_t,
        up_0: *mut vec_t,
    );
    fn va(format: *mut libc::c_char, _: ...) -> *mut libc::c_char;
    fn PMenu_Do_Update(ent: *mut edict_t);
    static mut game: game_locals_t;
    static mut level: level_locals_t;
    static mut gi: game_import_t;
    static mut snd_fry: libc::c_int;
    static mut g_edicts: *mut edict_t;
    static mut deathmatch: *mut cvar_t;
    static mut dmflags: *mut cvar_t;
    static mut gun_x: *mut cvar_t;
    static mut gun_y: *mut cvar_t;
    static mut gun_z: *mut cvar_t;
    static mut sv_rollspeed: *mut cvar_t;
    static mut sv_rollangle: *mut cvar_t;
    static mut run_pitch: *mut cvar_t;
    static mut run_roll: *mut cvar_t;
    static mut bob_up: *mut cvar_t;
    static mut bob_pitch: *mut cvar_t;
    static mut bob_roll: *mut cvar_t;
    static mut maxclients: *mut cvar_t;
    fn PowerArmorType(ent: *mut edict_t) -> libc::c_int;
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
    fn DeathmatchScoreboardMessage(client: *mut edict_t, killer: *mut edict_t);
    fn CTFEffects(player: *mut edict_t);
    fn G_SetStats(ent: *mut edict_t);
    fn PlayerNoise(who: *mut edict_t, where_0: *mut vec_t, type_0: libc::c_int);
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
pub const CTF_GRAPPLE_STATE_FLY: C2RustUnnamed_1 = 0;
pub type C2RustUnnamed_1 = libc::c_uint;
pub const CTF_GRAPPLE_STATE_HANG: C2RustUnnamed_1 = 2;
pub const CTF_GRAPPLE_STATE_PULL: C2RustUnnamed_1 = 1;
static mut current_player: *mut edict_t = 0 as *const edict_t as *mut edict_t;
static mut current_client: *mut gclient_t = 0 as *const gclient_t as *mut gclient_t;
static mut forward: vec3_t = [0.; 3];
static mut right: vec3_t = [0.; 3];
static mut up: vec3_t = [0.; 3];
#[no_mangle]
pub static mut xyspeed: libc::c_float = 0.;
#[no_mangle]
pub static mut bobmove: libc::c_float = 0.;
#[no_mangle]
pub static mut bobcycle: libc::c_int = 0;
#[no_mangle]
pub static mut bobfracsin: libc::c_float = 0.;
#[no_mangle]
pub unsafe extern "C" fn SV_CalcRoll(
    mut angles: *mut vec_t,
    mut velocity: *mut vec_t,
) -> libc::c_float {
    let mut sign: libc::c_float = 0.;
    let mut side: libc::c_float = 0.;
    let mut value: libc::c_float = 0.;
    side = *velocity.offset(0 as libc::c_int as isize) * right[0 as libc::c_int as usize]
        + *velocity.offset(1 as libc::c_int as isize) * right[1 as libc::c_int as usize]
        + *velocity.offset(2 as libc::c_int as isize) * right[2 as libc::c_int as usize];
    sign = (if side < 0 as libc::c_int as libc::c_float {
        -(1 as libc::c_int)
    } else {
        1 as libc::c_int
    }) as libc::c_float;
    side = fabs(side as libc::c_double) as libc::c_float;
    value = (*sv_rollangle).value;
    if side < (*sv_rollspeed).value {
        side = side * value / (*sv_rollspeed).value;
    } else {
        side = value;
    }
    return side * sign;
}
#[no_mangle]
pub unsafe extern "C" fn P_DamageFeedback(mut player: *mut edict_t) {
    let mut client: *mut gclient_t = 0 as *mut gclient_t;
    let mut side: libc::c_float = 0.;
    let mut realcount: libc::c_float = 0.;
    let mut count: libc::c_float = 0.;
    let mut kick: libc::c_float = 0.;
    let mut v: vec3_t = [0.; 3];
    let mut r: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    static mut power_color: vec3_t = [0.0f64 as vec_t, 1.0f64 as vec_t, 0.0f64 as vec_t];
    static mut acolor: vec3_t = [1.0f64 as vec_t, 1.0f64 as vec_t, 1.0f64 as vec_t];
    static mut bcolor: vec3_t = [1.0f64 as vec_t, 0.0f64 as vec_t, 0.0f64 as vec_t];
    client = (*player).client;
    (*client).ps.stats[15 as libc::c_int as usize] = 0 as libc::c_int as libc::c_short;
    if (*client).damage_blood != 0 {
        let ref mut fresh0 = (*client).ps.stats[15 as libc::c_int as usize];
        *fresh0 = (*fresh0 as libc::c_int | 1 as libc::c_int) as libc::c_short;
    }
    if (*client).damage_armor != 0 && (*player).flags & 0x10 as libc::c_int == 0
        && (*client).invincible_framenum <= level.framenum as libc::c_float
    {
        let ref mut fresh1 = (*client).ps.stats[15 as libc::c_int as usize];
        *fresh1 = (*fresh1 as libc::c_int | 2 as libc::c_int) as libc::c_short;
    }
    count = ((*client).damage_blood + (*client).damage_armor + (*client).damage_parmor)
        as libc::c_float;
    if count == 0 as libc::c_int as libc::c_float {
        return;
    }
    if (*client).anim_priority < 3 as libc::c_int
        && (*player).s.modelindex == 255 as libc::c_int
    {
        static mut i: libc::c_int = 0;
        (*client).anim_priority = 3 as libc::c_int;
        if (*client).ps.pmove.pm_flags as libc::c_int & 1 as libc::c_int != 0 {
            (*player).s.frame = 169 as libc::c_int - 1 as libc::c_int;
            (*client).anim_end = 172 as libc::c_int;
        } else {
            i = (i + 1 as libc::c_int) % 3 as libc::c_int;
            match i {
                0 => {
                    (*player).s.frame = 54 as libc::c_int - 1 as libc::c_int;
                    (*client).anim_end = 57 as libc::c_int;
                }
                1 => {
                    (*player).s.frame = 58 as libc::c_int - 1 as libc::c_int;
                    (*client).anim_end = 61 as libc::c_int;
                }
                2 => {
                    (*player).s.frame = 62 as libc::c_int - 1 as libc::c_int;
                    (*client).anim_end = 65 as libc::c_int;
                }
                _ => {}
            }
        }
    }
    realcount = count;
    if count < 10 as libc::c_int as libc::c_float {
        count = 10 as libc::c_int as libc::c_float;
    }
    if level.time > (*player).pain_debounce_time
        && (*player).flags & 0x10 as libc::c_int == 0
        && (*client).invincible_framenum <= level.framenum as libc::c_float
    {
        r = 1 as libc::c_int + (rand() & 1 as libc::c_int);
        (*player)
            .pain_debounce_time = (level.time as libc::c_double + 0.7f64)
            as libc::c_float;
        if (*player).health < 25 as libc::c_int {
            l = 25 as libc::c_int;
        } else if (*player).health < 50 as libc::c_int {
            l = 50 as libc::c_int;
        } else if (*player).health < 75 as libc::c_int {
            l = 75 as libc::c_int;
        } else {
            l = 100 as libc::c_int;
        }
        (gi.sound)
            .expect(
                "non-null function pointer",
            )(
            player,
            2 as libc::c_int,
            (gi.soundindex)
                .expect(
                    "non-null function pointer",
                )(
                va(
                    b"*pain%i_%i.wav\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    l,
                    r,
                ),
            ),
            1 as libc::c_int as libc::c_float,
            1 as libc::c_int as libc::c_float,
            0 as libc::c_int as libc::c_float,
        );
    }
    if (*client).damage_alpha < 0 as libc::c_int as libc::c_float {
        (*client).damage_alpha = 0 as libc::c_int as libc::c_float;
    }
    let ref mut fresh2 = (*client).damage_alpha;
    *fresh2 = (*fresh2 as libc::c_double + count as libc::c_double * 0.01f64)
        as libc::c_float;
    if ((*client).damage_alpha as libc::c_double) < 0.2f64 {
        (*client).damage_alpha = 0.2f64 as libc::c_float;
    }
    if (*client).damage_alpha as libc::c_double > 0.6f64 {
        (*client).damage_alpha = 0.6f64 as libc::c_float;
    }
    v[2 as libc::c_int as usize] = 0 as libc::c_int as vec_t;
    v[1 as libc::c_int as usize] = v[2 as libc::c_int as usize];
    v[0 as libc::c_int as usize] = v[1 as libc::c_int as usize];
    if (*client).damage_parmor != 0 {
        VectorMA(
            v.as_mut_ptr(),
            (*client).damage_parmor as libc::c_float / realcount,
            power_color.as_mut_ptr(),
            v.as_mut_ptr(),
        );
    }
    if (*client).damage_armor != 0 {
        VectorMA(
            v.as_mut_ptr(),
            (*client).damage_armor as libc::c_float / realcount,
            acolor.as_mut_ptr(),
            v.as_mut_ptr(),
        );
    }
    if (*client).damage_blood != 0 {
        VectorMA(
            v.as_mut_ptr(),
            (*client).damage_blood as libc::c_float / realcount,
            bcolor.as_mut_ptr(),
            v.as_mut_ptr(),
        );
    }
    (*client).damage_blend[0 as libc::c_int as usize] = v[0 as libc::c_int as usize];
    (*client).damage_blend[1 as libc::c_int as usize] = v[1 as libc::c_int as usize];
    (*client).damage_blend[2 as libc::c_int as usize] = v[2 as libc::c_int as usize];
    kick = abs((*client).damage_knockback) as libc::c_float;
    if kick != 0. && (*player).health > 0 as libc::c_int {
        kick = kick * 100 as libc::c_int as libc::c_float
            / (*player).health as libc::c_float;
        if (kick as libc::c_double) < count as libc::c_double * 0.5f64 {
            kick = (count as libc::c_double * 0.5f64) as libc::c_float;
        }
        if kick > 50 as libc::c_int as libc::c_float {
            kick = 50 as libc::c_int as libc::c_float;
        }
        v[0 as libc::c_int
            as usize] = (*client).damage_from[0 as libc::c_int as usize]
            - (*player).s.origin[0 as libc::c_int as usize];
        v[1 as libc::c_int
            as usize] = (*client).damage_from[1 as libc::c_int as usize]
            - (*player).s.origin[1 as libc::c_int as usize];
        v[2 as libc::c_int
            as usize] = (*client).damage_from[2 as libc::c_int as usize]
            - (*player).s.origin[2 as libc::c_int as usize];
        VectorNormalize(v.as_mut_ptr());
        side = v[0 as libc::c_int as usize] * right[0 as libc::c_int as usize]
            + v[1 as libc::c_int as usize] * right[1 as libc::c_int as usize]
            + v[2 as libc::c_int as usize] * right[2 as libc::c_int as usize];
        (*client)
            .v_dmg_roll = ((kick * side) as libc::c_double * 0.3f64) as libc::c_float;
        side = -(v[0 as libc::c_int as usize] * forward[0 as libc::c_int as usize]
            + v[1 as libc::c_int as usize] * forward[1 as libc::c_int as usize]
            + v[2 as libc::c_int as usize] * forward[2 as libc::c_int as usize]);
        (*client)
            .v_dmg_pitch = ((kick * side) as libc::c_double * 0.3f64) as libc::c_float;
        (*client).v_dmg_time = (level.time as libc::c_double + 0.5f64) as libc::c_float;
    }
    (*client).damage_blood = 0 as libc::c_int;
    (*client).damage_armor = 0 as libc::c_int;
    (*client).damage_parmor = 0 as libc::c_int;
    (*client).damage_knockback = 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn SV_CalcViewOffset(mut ent: *mut edict_t) {
    let mut angles: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut bob: libc::c_float = 0.;
    let mut ratio: libc::c_float = 0.;
    let mut delta: libc::c_float = 0.;
    let mut v: vec3_t = [0.; 3];
    angles = ((*(*ent).client).ps.kick_angles).as_mut_ptr();
    if (*ent).deadflag != 0 {
        let ref mut fresh3 = *angles.offset(2 as libc::c_int as isize);
        *fresh3 = 0 as libc::c_int as libc::c_float;
        let ref mut fresh4 = *angles.offset(1 as libc::c_int as isize);
        *fresh4 = *fresh3;
        *angles.offset(0 as libc::c_int as isize) = *fresh4;
        (*(*ent).client)
            .ps
            .viewangles[2 as libc::c_int as usize] = 40 as libc::c_int as vec_t;
        (*(*ent).client)
            .ps
            .viewangles[0 as libc::c_int as usize] = -(15 as libc::c_int) as vec_t;
        (*(*ent).client)
            .ps
            .viewangles[1 as libc::c_int as usize] = (*(*ent).client).killer_yaw;
    } else {
        *angles
            .offset(
                0 as libc::c_int as isize,
            ) = (*(*ent).client).kick_angles[0 as libc::c_int as usize];
        *angles
            .offset(
                1 as libc::c_int as isize,
            ) = (*(*ent).client).kick_angles[1 as libc::c_int as usize];
        *angles
            .offset(
                2 as libc::c_int as isize,
            ) = (*(*ent).client).kick_angles[2 as libc::c_int as usize];
        ratio = (((*(*ent).client).v_dmg_time - level.time) as libc::c_double / 0.5f64)
            as libc::c_float;
        if ratio < 0 as libc::c_int as libc::c_float {
            ratio = 0 as libc::c_int as libc::c_float;
            (*(*ent).client).v_dmg_pitch = 0 as libc::c_int as libc::c_float;
            (*(*ent).client).v_dmg_roll = 0 as libc::c_int as libc::c_float;
        }
        *angles.offset(0 as libc::c_int as isize)
            += ratio * (*(*ent).client).v_dmg_pitch;
        *angles.offset(2 as libc::c_int as isize) += ratio * (*(*ent).client).v_dmg_roll;
        ratio = (((*(*ent).client).fall_time - level.time) as libc::c_double / 0.3f64)
            as libc::c_float;
        if ratio < 0 as libc::c_int as libc::c_float {
            ratio = 0 as libc::c_int as libc::c_float;
        }
        *angles.offset(0 as libc::c_int as isize) += ratio * (*(*ent).client).fall_value;
        delta = (*ent).velocity[0 as libc::c_int as usize]
            * forward[0 as libc::c_int as usize]
            + (*ent).velocity[1 as libc::c_int as usize]
                * forward[1 as libc::c_int as usize]
            + (*ent).velocity[2 as libc::c_int as usize]
                * forward[2 as libc::c_int as usize];
        *angles.offset(0 as libc::c_int as isize) += delta * (*run_pitch).value;
        delta = (*ent).velocity[0 as libc::c_int as usize]
            * right[0 as libc::c_int as usize]
            + (*ent).velocity[1 as libc::c_int as usize]
                * right[1 as libc::c_int as usize]
            + (*ent).velocity[2 as libc::c_int as usize]
                * right[2 as libc::c_int as usize];
        *angles.offset(2 as libc::c_int as isize) += delta * (*run_roll).value;
        delta = bobfracsin * (*bob_pitch).value * xyspeed;
        if (*(*ent).client).ps.pmove.pm_flags as libc::c_int & 1 as libc::c_int != 0 {
            delta *= 6 as libc::c_int as libc::c_float;
        }
        *angles.offset(0 as libc::c_int as isize) += delta;
        delta = bobfracsin * (*bob_roll).value * xyspeed;
        if (*(*ent).client).ps.pmove.pm_flags as libc::c_int & 1 as libc::c_int != 0 {
            delta *= 6 as libc::c_int as libc::c_float;
        }
        if bobcycle & 1 as libc::c_int != 0 {
            delta = -delta;
        }
        *angles.offset(2 as libc::c_int as isize) += delta;
    }
    v[2 as libc::c_int as usize] = 0 as libc::c_int as vec_t;
    v[1 as libc::c_int as usize] = v[2 as libc::c_int as usize];
    v[0 as libc::c_int as usize] = v[1 as libc::c_int as usize];
    v[2 as libc::c_int as usize] += (*ent).viewheight as libc::c_float;
    ratio = (((*(*ent).client).fall_time - level.time) as libc::c_double / 0.3f64)
        as libc::c_float;
    if ratio < 0 as libc::c_int as libc::c_float {
        ratio = 0 as libc::c_int as libc::c_float;
    }
    v[2 as libc::c_int
        as usize] = (v[2 as libc::c_int as usize] as libc::c_double
        - (ratio * (*(*ent).client).fall_value) as libc::c_double * 0.4f64) as vec_t;
    bob = bobfracsin * xyspeed * (*bob_up).value;
    if bob > 6 as libc::c_int as libc::c_float {
        bob = 6 as libc::c_int as libc::c_float;
    }
    v[2 as libc::c_int as usize] += bob;
    v[0 as libc::c_int
        as usize] = v[0 as libc::c_int as usize]
        + (*(*ent).client).kick_origin[0 as libc::c_int as usize];
    v[1 as libc::c_int
        as usize] = v[1 as libc::c_int as usize]
        + (*(*ent).client).kick_origin[1 as libc::c_int as usize];
    v[2 as libc::c_int
        as usize] = v[2 as libc::c_int as usize]
        + (*(*ent).client).kick_origin[2 as libc::c_int as usize];
    if v[0 as libc::c_int as usize] < -(14 as libc::c_int) as libc::c_float {
        v[0 as libc::c_int as usize] = -(14 as libc::c_int) as vec_t;
    } else if v[0 as libc::c_int as usize] > 14 as libc::c_int as libc::c_float {
        v[0 as libc::c_int as usize] = 14 as libc::c_int as vec_t;
    }
    if v[1 as libc::c_int as usize] < -(14 as libc::c_int) as libc::c_float {
        v[1 as libc::c_int as usize] = -(14 as libc::c_int) as vec_t;
    } else if v[1 as libc::c_int as usize] > 14 as libc::c_int as libc::c_float {
        v[1 as libc::c_int as usize] = 14 as libc::c_int as vec_t;
    }
    if v[2 as libc::c_int as usize] < -(22 as libc::c_int) as libc::c_float {
        v[2 as libc::c_int as usize] = -(22 as libc::c_int) as vec_t;
    } else if v[2 as libc::c_int as usize] > 30 as libc::c_int as libc::c_float {
        v[2 as libc::c_int as usize] = 30 as libc::c_int as vec_t;
    }
    (*(*ent).client)
        .ps
        .viewoffset[0 as libc::c_int as usize] = v[0 as libc::c_int as usize];
    (*(*ent).client)
        .ps
        .viewoffset[1 as libc::c_int as usize] = v[1 as libc::c_int as usize];
    (*(*ent).client)
        .ps
        .viewoffset[2 as libc::c_int as usize] = v[2 as libc::c_int as usize];
}
#[no_mangle]
pub unsafe extern "C" fn SV_CalcGunOffset(mut ent: *mut edict_t) {
    let mut i: libc::c_int = 0;
    let mut delta: libc::c_float = 0.;
    (*(*ent).client)
        .ps
        .gunangles[2 as libc::c_int
        as usize] = ((xyspeed * bobfracsin) as libc::c_double * 0.005f64) as vec_t;
    (*(*ent).client)
        .ps
        .gunangles[1 as libc::c_int
        as usize] = ((xyspeed * bobfracsin) as libc::c_double * 0.01f64) as vec_t;
    if bobcycle & 1 as libc::c_int != 0 {
        (*(*ent).client)
            .ps
            .gunangles[2 as libc::c_int
            as usize] = -(*(*ent).client).ps.gunangles[2 as libc::c_int as usize];
        (*(*ent).client)
            .ps
            .gunangles[1 as libc::c_int
            as usize] = -(*(*ent).client).ps.gunangles[1 as libc::c_int as usize];
    }
    (*(*ent).client)
        .ps
        .gunangles[0 as libc::c_int
        as usize] = ((xyspeed * bobfracsin) as libc::c_double * 0.005f64) as vec_t;
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        delta = (*(*ent).client).oldviewangles[i as usize]
            - (*(*ent).client).ps.viewangles[i as usize];
        if delta > 180 as libc::c_int as libc::c_float {
            delta -= 360 as libc::c_int as libc::c_float;
        }
        if delta < -(180 as libc::c_int) as libc::c_float {
            delta += 360 as libc::c_int as libc::c_float;
        }
        if delta > 45 as libc::c_int as libc::c_float {
            delta = 45 as libc::c_int as libc::c_float;
        }
        if delta < -(45 as libc::c_int) as libc::c_float {
            delta = -(45 as libc::c_int) as libc::c_float;
        }
        if i == 1 as libc::c_int {
            let ref mut fresh5 = (*(*ent).client)
                .ps
                .gunangles[2 as libc::c_int as usize];
            *fresh5 = (*fresh5 as libc::c_double + 0.1f64 * delta as libc::c_double)
                as vec_t;
        }
        let ref mut fresh6 = (*(*ent).client).ps.gunangles[i as usize];
        *fresh6 = (*fresh6 as libc::c_double + 0.2f64 * delta as libc::c_double)
            as vec_t;
        i += 1;
    }
    let ref mut fresh7 = (*(*ent).client).ps.gunoffset[2 as libc::c_int as usize];
    *fresh7 = 0 as libc::c_int as vec_t;
    let ref mut fresh8 = (*(*ent).client).ps.gunoffset[1 as libc::c_int as usize];
    *fresh8 = *fresh7;
    (*(*ent).client).ps.gunoffset[0 as libc::c_int as usize] = *fresh8;
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        let ref mut fresh9 = (*(*ent).client).ps.gunoffset[i as usize];
        *fresh9 += forward[i as usize] * (*gun_y).value;
        let ref mut fresh10 = (*(*ent).client).ps.gunoffset[i as usize];
        *fresh10 += right[i as usize] * (*gun_x).value;
        let ref mut fresh11 = (*(*ent).client).ps.gunoffset[i as usize];
        *fresh11 += up[i as usize] * -(*gun_z).value;
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn SV_AddBlend(
    mut r: libc::c_float,
    mut g: libc::c_float,
    mut b: libc::c_float,
    mut a: libc::c_float,
    mut v_blend: *mut libc::c_float,
) {
    let mut a2: libc::c_float = 0.;
    let mut a3: libc::c_float = 0.;
    if a <= 0 as libc::c_int as libc::c_float {
        return;
    }
    a2 = *v_blend.offset(3 as libc::c_int as isize)
        + (1 as libc::c_int as libc::c_float
            - *v_blend.offset(3 as libc::c_int as isize)) * a;
    a3 = *v_blend.offset(3 as libc::c_int as isize) / a2;
    *v_blend
        .offset(
            0 as libc::c_int as isize,
        ) = *v_blend.offset(0 as libc::c_int as isize) * a3
        + r * (1 as libc::c_int as libc::c_float - a3);
    *v_blend
        .offset(
            1 as libc::c_int as isize,
        ) = *v_blend.offset(1 as libc::c_int as isize) * a3
        + g * (1 as libc::c_int as libc::c_float - a3);
    *v_blend
        .offset(
            2 as libc::c_int as isize,
        ) = *v_blend.offset(2 as libc::c_int as isize) * a3
        + b * (1 as libc::c_int as libc::c_float - a3);
    *v_blend.offset(3 as libc::c_int as isize) = a2;
}
#[no_mangle]
pub unsafe extern "C" fn SV_CalcBlend(mut ent: *mut edict_t) {
    let mut contents: libc::c_int = 0;
    let mut vieworg: vec3_t = [0.; 3];
    let mut remaining: libc::c_int = 0;
    let ref mut fresh12 = (*(*ent).client).ps.blend[3 as libc::c_int as usize];
    *fresh12 = 0 as libc::c_int as libc::c_float;
    let ref mut fresh13 = (*(*ent).client).ps.blend[2 as libc::c_int as usize];
    *fresh13 = *fresh12;
    let ref mut fresh14 = (*(*ent).client).ps.blend[1 as libc::c_int as usize];
    *fresh14 = *fresh13;
    (*(*ent).client).ps.blend[0 as libc::c_int as usize] = *fresh14;
    vieworg[0 as libc::c_int
        as usize] = (*ent).s.origin[0 as libc::c_int as usize]
        + (*(*ent).client).ps.viewoffset[0 as libc::c_int as usize];
    vieworg[1 as libc::c_int
        as usize] = (*ent).s.origin[1 as libc::c_int as usize]
        + (*(*ent).client).ps.viewoffset[1 as libc::c_int as usize];
    vieworg[2 as libc::c_int
        as usize] = (*ent).s.origin[2 as libc::c_int as usize]
        + (*(*ent).client).ps.viewoffset[2 as libc::c_int as usize];
    contents = (gi.pointcontents)
        .expect("non-null function pointer")(vieworg.as_mut_ptr());
    if contents & (8 as libc::c_int | 16 as libc::c_int | 32 as libc::c_int) != 0 {
        (*(*ent).client).ps.rdflags |= 1 as libc::c_int;
    } else {
        (*(*ent).client).ps.rdflags &= !(1 as libc::c_int);
    }
    if contents & (1 as libc::c_int | 8 as libc::c_int) != 0 {
        SV_AddBlend(
            1.0f64 as libc::c_float,
            0.3f64 as libc::c_float,
            0.0f64 as libc::c_float,
            0.6f64 as libc::c_float,
            ((*(*ent).client).ps.blend).as_mut_ptr(),
        );
    } else if contents & 16 as libc::c_int != 0 {
        SV_AddBlend(
            0.0f64 as libc::c_float,
            0.1f64 as libc::c_float,
            0.05f64 as libc::c_float,
            0.6f64 as libc::c_float,
            ((*(*ent).client).ps.blend).as_mut_ptr(),
        );
    } else if contents & 32 as libc::c_int != 0 {
        SV_AddBlend(
            0.5f64 as libc::c_float,
            0.3f64 as libc::c_float,
            0.2f64 as libc::c_float,
            0.4f64 as libc::c_float,
            ((*(*ent).client).ps.blend).as_mut_ptr(),
        );
    }
    if (*(*ent).client).quad_framenum > level.framenum as libc::c_float {
        remaining = ((*(*ent).client).quad_framenum - level.framenum as libc::c_float)
            as libc::c_int;
        if remaining == 30 as libc::c_int {
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
                    b"items/damage2.wav\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                ),
                1 as libc::c_int as libc::c_float,
                1 as libc::c_int as libc::c_float,
                0 as libc::c_int as libc::c_float,
            );
        }
        if remaining > 30 as libc::c_int || remaining & 4 as libc::c_int != 0 {
            SV_AddBlend(
                0 as libc::c_int as libc::c_float,
                0 as libc::c_int as libc::c_float,
                1 as libc::c_int as libc::c_float,
                0.08f64 as libc::c_float,
                ((*(*ent).client).ps.blend).as_mut_ptr(),
            );
        }
    } else if (*(*ent).client).invincible_framenum > level.framenum as libc::c_float {
        remaining = ((*(*ent).client).invincible_framenum
            - level.framenum as libc::c_float) as libc::c_int;
        if remaining == 30 as libc::c_int {
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
                    b"items/protect2.wav\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                ),
                1 as libc::c_int as libc::c_float,
                1 as libc::c_int as libc::c_float,
                0 as libc::c_int as libc::c_float,
            );
        }
        if remaining > 30 as libc::c_int || remaining & 4 as libc::c_int != 0 {
            SV_AddBlend(
                1 as libc::c_int as libc::c_float,
                1 as libc::c_int as libc::c_float,
                0 as libc::c_int as libc::c_float,
                0.08f64 as libc::c_float,
                ((*(*ent).client).ps.blend).as_mut_ptr(),
            );
        }
    } else if (*(*ent).client).enviro_framenum > level.framenum as libc::c_float {
        remaining = ((*(*ent).client).enviro_framenum - level.framenum as libc::c_float)
            as libc::c_int;
        if remaining == 30 as libc::c_int {
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
                    b"items/airout.wav\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                ),
                1 as libc::c_int as libc::c_float,
                1 as libc::c_int as libc::c_float,
                0 as libc::c_int as libc::c_float,
            );
        }
        if remaining > 30 as libc::c_int || remaining & 4 as libc::c_int != 0 {
            SV_AddBlend(
                0 as libc::c_int as libc::c_float,
                1 as libc::c_int as libc::c_float,
                0 as libc::c_int as libc::c_float,
                0.08f64 as libc::c_float,
                ((*(*ent).client).ps.blend).as_mut_ptr(),
            );
        }
    } else if (*(*ent).client).breather_framenum > level.framenum as libc::c_float {
        remaining = ((*(*ent).client).breather_framenum
            - level.framenum as libc::c_float) as libc::c_int;
        if remaining == 30 as libc::c_int {
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
                    b"items/airout.wav\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                ),
                1 as libc::c_int as libc::c_float,
                1 as libc::c_int as libc::c_float,
                0 as libc::c_int as libc::c_float,
            );
        }
        if remaining > 30 as libc::c_int || remaining & 4 as libc::c_int != 0 {
            SV_AddBlend(
                0.4f64 as libc::c_float,
                1 as libc::c_int as libc::c_float,
                0.4f64 as libc::c_float,
                0.04f64 as libc::c_float,
                ((*(*ent).client).ps.blend).as_mut_ptr(),
            );
        }
    }
    if (*(*ent).client).damage_alpha > 0 as libc::c_int as libc::c_float {
        SV_AddBlend(
            (*(*ent).client).damage_blend[0 as libc::c_int as usize],
            (*(*ent).client).damage_blend[1 as libc::c_int as usize],
            (*(*ent).client).damage_blend[2 as libc::c_int as usize],
            (*(*ent).client).damage_alpha,
            ((*(*ent).client).ps.blend).as_mut_ptr(),
        );
    }
    if (*(*ent).client).bonus_alpha > 0 as libc::c_int as libc::c_float {
        SV_AddBlend(
            0.85f64 as libc::c_float,
            0.7f64 as libc::c_float,
            0.3f64 as libc::c_float,
            (*(*ent).client).bonus_alpha,
            ((*(*ent).client).ps.blend).as_mut_ptr(),
        );
    }
    let ref mut fresh15 = (*(*ent).client).damage_alpha;
    *fresh15 = (*fresh15 as libc::c_double - 0.06f64) as libc::c_float;
    if (*(*ent).client).damage_alpha < 0 as libc::c_int as libc::c_float {
        (*(*ent).client).damage_alpha = 0 as libc::c_int as libc::c_float;
    }
    let ref mut fresh16 = (*(*ent).client).bonus_alpha;
    *fresh16 = (*fresh16 as libc::c_double - 0.1f64) as libc::c_float;
    if (*(*ent).client).bonus_alpha < 0 as libc::c_int as libc::c_float {
        (*(*ent).client).bonus_alpha = 0 as libc::c_int as libc::c_float;
    }
}
#[no_mangle]
pub unsafe extern "C" fn P_FallingDamage(mut ent: *mut edict_t) {
    let mut delta: libc::c_float = 0.;
    let mut damage: libc::c_int = 0;
    let mut dir: vec3_t = [0.; 3];
    if (*ent).s.modelindex != 255 as libc::c_int {
        return;
    }
    if (*ent).movetype == MOVETYPE_NOCLIP as libc::c_int {
        return;
    }
    if (*(*ent).client).oldvelocity[2 as libc::c_int as usize]
        < 0 as libc::c_int as libc::c_float
        && (*ent).velocity[2 as libc::c_int as usize]
            > (*(*ent).client).oldvelocity[2 as libc::c_int as usize]
        && ((*ent).groundentity).is_null()
    {
        delta = (*(*ent).client).oldvelocity[2 as libc::c_int as usize];
    } else {
        if ((*ent).groundentity).is_null() {
            return;
        }
        delta = (*ent).velocity[2 as libc::c_int as usize]
            - (*(*ent).client).oldvelocity[2 as libc::c_int as usize];
    }
    delta = ((delta * delta) as libc::c_double * 0.0001f64) as libc::c_float;
    if (level.time - (*(*ent).client).ctf_grapplereleasetime) as libc::c_double
        <= 0.1f64 * 2 as libc::c_int as libc::c_double
        || !((*(*ent).client).ctf_grapple).is_null()
            && (*(*ent).client).ctf_grapplestate > CTF_GRAPPLE_STATE_FLY as libc::c_int
    {
        return;
    }
    if (*ent).waterlevel == 3 as libc::c_int {
        return;
    }
    if (*ent).waterlevel == 2 as libc::c_int {
        delta = (delta as libc::c_double * 0.25f64) as libc::c_float;
    }
    if (*ent).waterlevel == 1 as libc::c_int {
        delta = (delta as libc::c_double * 0.5f64) as libc::c_float;
    }
    if delta < 1 as libc::c_int as libc::c_float {
        return;
    }
    if delta < 15 as libc::c_int as libc::c_float {
        (*ent).s.event = EV_FOOTSTEP as libc::c_int;
        return;
    }
    (*(*ent).client).fall_value = (delta as libc::c_double * 0.5f64) as libc::c_float;
    if (*(*ent).client).fall_value > 40 as libc::c_int as libc::c_float {
        (*(*ent).client).fall_value = 40 as libc::c_int as libc::c_float;
    }
    (*(*ent).client)
        .fall_time = (level.time as libc::c_double + 0.3f64) as libc::c_float;
    if delta > 30 as libc::c_int as libc::c_float {
        if (*ent).health > 0 as libc::c_int {
            if delta >= 55 as libc::c_int as libc::c_float {
                (*ent).s.event = EV_FALLFAR as libc::c_int;
            } else {
                (*ent).s.event = EV_FALL as libc::c_int;
            }
        }
        (*ent).pain_debounce_time = level.time;
        damage = ((delta - 30 as libc::c_int as libc::c_float)
            / 2 as libc::c_int as libc::c_float) as libc::c_int;
        if damage < 1 as libc::c_int {
            damage = 1 as libc::c_int;
        }
        dir[0 as libc::c_int as usize] = 0 as libc::c_int as vec_t;
        dir[1 as libc::c_int as usize] = 0 as libc::c_int as vec_t;
        dir[2 as libc::c_int as usize] = 1 as libc::c_int as vec_t;
        if (*deathmatch).value == 0.
            || (*dmflags).value as libc::c_int & 0x8 as libc::c_int == 0
        {
            T_Damage(
                ent,
                &mut *g_edicts.offset(0 as libc::c_int as isize),
                &mut *g_edicts.offset(0 as libc::c_int as isize),
                dir.as_mut_ptr(),
                ((*ent).s.origin).as_mut_ptr(),
                vec3_origin.as_mut_ptr(),
                damage,
                0 as libc::c_int,
                0 as libc::c_int,
                22 as libc::c_int,
            );
        }
    } else {
        (*ent).s.event = EV_FALLSHORT as libc::c_int;
        return;
    };
}
#[no_mangle]
pub unsafe extern "C" fn P_WorldEffects() {
    let mut breather: qboolean = false_0;
    let mut envirosuit: qboolean = false_0;
    let mut waterlevel: libc::c_int = 0;
    let mut old_waterlevel: libc::c_int = 0;
    if (*current_player).movetype == MOVETYPE_NOCLIP as libc::c_int {
        (*current_player).air_finished = level.time + 12 as libc::c_int as libc::c_float;
        return;
    }
    waterlevel = (*current_player).waterlevel;
    old_waterlevel = (*current_client).old_waterlevel;
    (*current_client).old_waterlevel = waterlevel;
    breather = ((*current_client).breather_framenum > level.framenum as libc::c_float)
        as libc::c_int as qboolean;
    envirosuit = ((*current_client).enviro_framenum > level.framenum as libc::c_float)
        as libc::c_int as qboolean;
    if old_waterlevel == 0 && waterlevel != 0 {
        PlayerNoise(
            current_player,
            ((*current_player).s.origin).as_mut_ptr(),
            0 as libc::c_int,
        );
        if (*current_player).watertype & 8 as libc::c_int != 0 {
            (gi.sound)
                .expect(
                    "non-null function pointer",
                )(
                current_player,
                4 as libc::c_int,
                (gi.soundindex)
                    .expect(
                        "non-null function pointer",
                    )(
                    b"player/lava_in.wav\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                ),
                1 as libc::c_int as libc::c_float,
                1 as libc::c_int as libc::c_float,
                0 as libc::c_int as libc::c_float,
            );
        } else if (*current_player).watertype & 16 as libc::c_int != 0 {
            (gi.sound)
                .expect(
                    "non-null function pointer",
                )(
                current_player,
                4 as libc::c_int,
                (gi.soundindex)
                    .expect(
                        "non-null function pointer",
                    )(
                    b"player/watr_in.wav\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                ),
                1 as libc::c_int as libc::c_float,
                1 as libc::c_int as libc::c_float,
                0 as libc::c_int as libc::c_float,
            );
        } else if (*current_player).watertype & 32 as libc::c_int != 0 {
            (gi.sound)
                .expect(
                    "non-null function pointer",
                )(
                current_player,
                4 as libc::c_int,
                (gi.soundindex)
                    .expect(
                        "non-null function pointer",
                    )(
                    b"player/watr_in.wav\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                ),
                1 as libc::c_int as libc::c_float,
                1 as libc::c_int as libc::c_float,
                0 as libc::c_int as libc::c_float,
            );
        }
        (*current_player).flags |= 0x8 as libc::c_int;
        (*current_player)
            .damage_debounce_time = level.time - 1 as libc::c_int as libc::c_float;
    }
    if old_waterlevel != 0 && waterlevel == 0 {
        PlayerNoise(
            current_player,
            ((*current_player).s.origin).as_mut_ptr(),
            0 as libc::c_int,
        );
        (gi.sound)
            .expect(
                "non-null function pointer",
            )(
            current_player,
            4 as libc::c_int,
            (gi.soundindex)
                .expect(
                    "non-null function pointer",
                )(
                b"player/watr_out.wav\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            ),
            1 as libc::c_int as libc::c_float,
            1 as libc::c_int as libc::c_float,
            0 as libc::c_int as libc::c_float,
        );
        (*current_player).flags &= !(0x8 as libc::c_int);
    }
    if old_waterlevel != 3 as libc::c_int && waterlevel == 3 as libc::c_int {
        (gi.sound)
            .expect(
                "non-null function pointer",
            )(
            current_player,
            4 as libc::c_int,
            (gi.soundindex)
                .expect(
                    "non-null function pointer",
                )(
                b"player/watr_un.wav\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            ),
            1 as libc::c_int as libc::c_float,
            1 as libc::c_int as libc::c_float,
            0 as libc::c_int as libc::c_float,
        );
    }
    if old_waterlevel == 3 as libc::c_int && waterlevel != 3 as libc::c_int {
        if (*current_player).air_finished < level.time {
            (gi.sound)
                .expect(
                    "non-null function pointer",
                )(
                current_player,
                2 as libc::c_int,
                (gi.soundindex)
                    .expect(
                        "non-null function pointer",
                    )(
                    b"player/gasp1.wav\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                ),
                1 as libc::c_int as libc::c_float,
                1 as libc::c_int as libc::c_float,
                0 as libc::c_int as libc::c_float,
            );
            PlayerNoise(
                current_player,
                ((*current_player).s.origin).as_mut_ptr(),
                0 as libc::c_int,
            );
        } else if (*current_player).air_finished
            < level.time + 11 as libc::c_int as libc::c_float
        {
            (gi.sound)
                .expect(
                    "non-null function pointer",
                )(
                current_player,
                2 as libc::c_int,
                (gi.soundindex)
                    .expect(
                        "non-null function pointer",
                    )(
                    b"player/gasp2.wav\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                ),
                1 as libc::c_int as libc::c_float,
                1 as libc::c_int as libc::c_float,
                0 as libc::c_int as libc::c_float,
            );
        }
    }
    if waterlevel == 3 as libc::c_int {
        if breather as libc::c_uint != 0 || envirosuit as libc::c_uint != 0 {
            (*current_player)
                .air_finished = level.time + 10 as libc::c_int as libc::c_float;
            if ((*current_client).breather_framenum - level.framenum as libc::c_float)
                as libc::c_int % 25 as libc::c_int == 0 as libc::c_int
            {
                if (*current_client).breather_sound == 0 {
                    (gi.sound)
                        .expect(
                            "non-null function pointer",
                        )(
                        current_player,
                        0 as libc::c_int,
                        (gi.soundindex)
                            .expect(
                                "non-null function pointer",
                            )(
                            b"player/u_breath1.wav\0" as *const u8 as *const libc::c_char
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
                        current_player,
                        0 as libc::c_int,
                        (gi.soundindex)
                            .expect(
                                "non-null function pointer",
                            )(
                            b"player/u_breath2.wav\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char,
                        ),
                        1 as libc::c_int as libc::c_float,
                        1 as libc::c_int as libc::c_float,
                        0 as libc::c_int as libc::c_float,
                    );
                }
                (*current_client).breather_sound ^= 1 as libc::c_int;
                PlayerNoise(
                    current_player,
                    ((*current_player).s.origin).as_mut_ptr(),
                    0 as libc::c_int,
                );
            }
        }
        if (*current_player).air_finished < level.time {
            if (*(*current_player).client).next_drown_time < level.time
                && (*current_player).health > 0 as libc::c_int
            {
                (*(*current_player).client)
                    .next_drown_time = level.time + 1 as libc::c_int as libc::c_float;
                (*current_player).dmg += 2 as libc::c_int;
                if (*current_player).dmg > 15 as libc::c_int {
                    (*current_player).dmg = 15 as libc::c_int;
                }
                if (*current_player).health <= (*current_player).dmg {
                    (gi.sound)
                        .expect(
                            "non-null function pointer",
                        )(
                        current_player,
                        2 as libc::c_int,
                        (gi.soundindex)
                            .expect(
                                "non-null function pointer",
                            )(
                            b"player/drown1.wav\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char,
                        ),
                        1 as libc::c_int as libc::c_float,
                        1 as libc::c_int as libc::c_float,
                        0 as libc::c_int as libc::c_float,
                    );
                } else if rand() & 1 as libc::c_int != 0 {
                    (gi.sound)
                        .expect(
                            "non-null function pointer",
                        )(
                        current_player,
                        2 as libc::c_int,
                        (gi.soundindex)
                            .expect(
                                "non-null function pointer",
                            )(
                            b"*gurp1.wav\0" as *const u8 as *const libc::c_char
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
                        current_player,
                        2 as libc::c_int,
                        (gi.soundindex)
                            .expect(
                                "non-null function pointer",
                            )(
                            b"*gurp2.wav\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char,
                        ),
                        1 as libc::c_int as libc::c_float,
                        1 as libc::c_int as libc::c_float,
                        0 as libc::c_int as libc::c_float,
                    );
                }
                (*current_player).pain_debounce_time = level.time;
                T_Damage(
                    current_player,
                    &mut *g_edicts.offset(0 as libc::c_int as isize),
                    &mut *g_edicts.offset(0 as libc::c_int as isize),
                    vec3_origin.as_mut_ptr(),
                    ((*current_player).s.origin).as_mut_ptr(),
                    vec3_origin.as_mut_ptr(),
                    (*current_player).dmg,
                    0 as libc::c_int,
                    0x2 as libc::c_int,
                    17 as libc::c_int,
                );
            }
        }
    } else {
        (*current_player).air_finished = level.time + 12 as libc::c_int as libc::c_float;
        (*current_player).dmg = 2 as libc::c_int;
    }
    if waterlevel != 0
        && (*current_player).watertype & (8 as libc::c_int | 16 as libc::c_int) != 0
    {
        if (*current_player).watertype & 8 as libc::c_int != 0 {
            if (*current_player).health > 0 as libc::c_int
                && (*current_player).pain_debounce_time <= level.time
                && (*current_client).invincible_framenum
                    < level.framenum as libc::c_float
            {
                if rand() & 1 as libc::c_int != 0 {
                    (gi.sound)
                        .expect(
                            "non-null function pointer",
                        )(
                        current_player,
                        2 as libc::c_int,
                        (gi.soundindex)
                            .expect(
                                "non-null function pointer",
                            )(
                            b"player/burn1.wav\0" as *const u8 as *const libc::c_char
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
                        current_player,
                        2 as libc::c_int,
                        (gi.soundindex)
                            .expect(
                                "non-null function pointer",
                            )(
                            b"player/burn2.wav\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char,
                        ),
                        1 as libc::c_int as libc::c_float,
                        1 as libc::c_int as libc::c_float,
                        0 as libc::c_int as libc::c_float,
                    );
                }
                (*current_player)
                    .pain_debounce_time = level.time + 1 as libc::c_int as libc::c_float;
            }
            if envirosuit as u64 != 0 {
                T_Damage(
                    current_player,
                    &mut *g_edicts.offset(0 as libc::c_int as isize),
                    &mut *g_edicts.offset(0 as libc::c_int as isize),
                    vec3_origin.as_mut_ptr(),
                    ((*current_player).s.origin).as_mut_ptr(),
                    vec3_origin.as_mut_ptr(),
                    1 as libc::c_int * waterlevel,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    19 as libc::c_int,
                );
            } else {
                T_Damage(
                    current_player,
                    &mut *g_edicts.offset(0 as libc::c_int as isize),
                    &mut *g_edicts.offset(0 as libc::c_int as isize),
                    vec3_origin.as_mut_ptr(),
                    ((*current_player).s.origin).as_mut_ptr(),
                    vec3_origin.as_mut_ptr(),
                    3 as libc::c_int * waterlevel,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    19 as libc::c_int,
                );
            }
        }
        if (*current_player).watertype & 16 as libc::c_int != 0 {
            if envirosuit as u64 == 0 {
                T_Damage(
                    current_player,
                    &mut *g_edicts.offset(0 as libc::c_int as isize),
                    &mut *g_edicts.offset(0 as libc::c_int as isize),
                    vec3_origin.as_mut_ptr(),
                    ((*current_player).s.origin).as_mut_ptr(),
                    vec3_origin.as_mut_ptr(),
                    1 as libc::c_int * waterlevel,
                    0 as libc::c_int,
                    0 as libc::c_int,
                    18 as libc::c_int,
                );
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn G_SetClientEffects(mut ent: *mut edict_t) {
    let mut pa_type: libc::c_int = 0;
    let mut remaining: libc::c_int = 0;
    (*ent).s.effects = 0 as libc::c_int as libc::c_uint;
    (*ent).s.renderfx = 0 as libc::c_int;
    if (*ent).health <= 0 as libc::c_int || level.intermissiontime != 0. {
        return;
    }
    if (*ent).powerarmor_time > level.time {
        pa_type = PowerArmorType(ent);
        if pa_type == 1 as libc::c_int {
            (*ent).s.effects |= 0x200 as libc::c_int as libc::c_uint;
        } else if pa_type == 2 as libc::c_int {
            (*ent).s.effects |= 0x100 as libc::c_int as libc::c_uint;
            (*ent).s.renderfx |= 2048 as libc::c_int;
        }
    }
    CTFEffects(ent);
    if (*(*ent).client).quad_framenum > level.framenum as libc::c_float
        && level.framenum & 8 as libc::c_int != 0
    {
        remaining = ((*(*ent).client).quad_framenum - level.framenum as libc::c_float)
            as libc::c_int;
        if remaining > 30 as libc::c_int || remaining & 4 as libc::c_int != 0 {
            (*ent).s.effects |= 0x8000 as libc::c_int as libc::c_uint;
        }
    }
    if (*(*ent).client).invincible_framenum > level.framenum as libc::c_float
        && level.framenum & 8 as libc::c_int != 0
    {
        remaining = ((*(*ent).client).invincible_framenum
            - level.framenum as libc::c_float) as libc::c_int;
        if remaining > 30 as libc::c_int || remaining & 4 as libc::c_int != 0 {
            (*ent).s.effects |= 0x10000 as libc::c_int as libc::c_uint;
        }
    }
    if (*ent).flags & 0x10 as libc::c_int != 0 {
        (*ent).s.effects |= 0x100 as libc::c_int as libc::c_uint;
        (*ent).s.renderfx
            |= 1024 as libc::c_int | 2048 as libc::c_int | 4096 as libc::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn G_SetClientEvent(mut ent: *mut edict_t) {
    if (*ent).s.event != 0 {
        return;
    }
    if !((*ent).groundentity).is_null() && xyspeed > 225 as libc::c_int as libc::c_float
    {
        if ((*current_client).bobtime + bobmove) as libc::c_int != bobcycle {
            (*ent).s.event = EV_FOOTSTEP as libc::c_int;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn G_SetClientSound(mut ent: *mut edict_t) {
    let mut weap: *mut libc::c_char = 0 as *mut libc::c_char;
    if (*(*ent).client).resp.game_helpchanged != game.helpchanged {
        (*(*ent).client).resp.game_helpchanged = game.helpchanged;
        (*(*ent).client).resp.helpchanged = 1 as libc::c_int;
    }
    if (*(*ent).client).resp.helpchanged != 0
        && (*(*ent).client).resp.helpchanged <= 3 as libc::c_int
        && level.framenum & 63 as libc::c_int == 0
    {
        let ref mut fresh17 = (*(*ent).client).resp.helpchanged;
        *fresh17 += 1;
        (gi.sound)
            .expect(
                "non-null function pointer",
            )(
            ent,
            2 as libc::c_int,
            (gi.soundindex)
                .expect(
                    "non-null function pointer",
                )(
                b"misc/pc_up.wav\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            ),
            1 as libc::c_int as libc::c_float,
            3 as libc::c_int as libc::c_float,
            0 as libc::c_int as libc::c_float,
        );
    }
    if !((*(*ent).client).pers.weapon).is_null() {
        weap = (*(*(*ent).client).pers.weapon).classname;
    } else {
        weap = b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    if (*ent).waterlevel != 0
        && (*ent).watertype & (8 as libc::c_int | 16 as libc::c_int) != 0
    {
        (*ent).s.sound = snd_fry;
    } else if strcmp(weap, b"weapon_railgun\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        (*ent)
            .s
            .sound = (gi.soundindex)
            .expect(
                "non-null function pointer",
            )(
            b"weapons/rg_hum.wav\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    } else if strcmp(weap, b"weapon_bfg\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        (*ent)
            .s
            .sound = (gi.soundindex)
            .expect(
                "non-null function pointer",
            )(
            b"weapons/bfg_hum.wav\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    } else if (*(*ent).client).weapon_sound != 0 {
        (*ent).s.sound = (*(*ent).client).weapon_sound;
    } else {
        (*ent).s.sound = 0 as libc::c_int;
    };
}
#[no_mangle]
pub unsafe extern "C" fn G_SetClientFrame(mut ent: *mut edict_t) {
    let mut client: *mut gclient_t = 0 as *mut gclient_t;
    let mut duck: qboolean = false_0;
    let mut run: qboolean = false_0;
    if (*ent).s.modelindex != 255 as libc::c_int {
        return;
    }
    client = (*ent).client;
    if (*client).ps.pmove.pm_flags as libc::c_int & 1 as libc::c_int != 0 {
        duck = true_0;
    } else {
        duck = false_0;
    }
    if xyspeed != 0. {
        run = true_0;
    } else {
        run = false_0;
    }
    if !(duck as libc::c_uint != (*client).anim_duck as libc::c_uint
        && (*client).anim_priority < 5 as libc::c_int)
    {
        if !(run as libc::c_uint != (*client).anim_run as libc::c_uint
            && (*client).anim_priority == 0 as libc::c_int)
        {
            if !(((*ent).groundentity).is_null()
                && (*client).anim_priority <= 1 as libc::c_int)
            {
                if (*client).anim_priority == 6 as libc::c_int {
                    if (*ent).s.frame > (*client).anim_end {
                        let ref mut fresh18 = (*ent).s.frame;
                        *fresh18 -= 1;
                        return;
                    }
                } else if (*ent).s.frame < (*client).anim_end {
                    let ref mut fresh19 = (*ent).s.frame;
                    *fresh19 += 1;
                    return;
                }
                if (*client).anim_priority == 5 as libc::c_int {
                    return;
                }
                if (*client).anim_priority == 2 as libc::c_int {
                    if ((*ent).groundentity).is_null() {
                        return;
                    }
                    (*(*ent).client).anim_priority = 1 as libc::c_int;
                    (*ent).s.frame = 68 as libc::c_int;
                    (*(*ent).client).anim_end = 71 as libc::c_int;
                    return;
                }
            }
        }
    }
    (*client).anim_priority = 0 as libc::c_int;
    (*client).anim_duck = duck;
    (*client).anim_run = run;
    if ((*ent).groundentity).is_null() {
        if !((*client).ctf_grapple).is_null() {
            (*ent).s.frame = 0 as libc::c_int;
            (*client).anim_end = 39 as libc::c_int;
        } else {
            (*client).anim_priority = 2 as libc::c_int;
            if (*ent).s.frame != 67 as libc::c_int {
                (*ent).s.frame = 66 as libc::c_int;
            }
            (*client).anim_end = 67 as libc::c_int;
        }
    } else if run as u64 != 0 {
        if duck as u64 != 0 {
            (*ent).s.frame = 154 as libc::c_int;
            (*client).anim_end = 159 as libc::c_int;
        } else {
            (*ent).s.frame = 40 as libc::c_int;
            (*client).anim_end = 45 as libc::c_int;
        }
    } else if duck as u64 != 0 {
        (*ent).s.frame = 135 as libc::c_int;
        (*client).anim_end = 153 as libc::c_int;
    } else {
        (*ent).s.frame = 0 as libc::c_int;
        (*client).anim_end = 39 as libc::c_int;
    };
}
#[no_mangle]
pub unsafe extern "C" fn ClientEndServerFrame(mut ent: *mut edict_t) {
    let mut bobtime: libc::c_float = 0.;
    let mut i: libc::c_int = 0;
    current_player = ent;
    current_client = (*ent).client;
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        (*current_client)
            .ps
            .pmove
            .origin[i
            as usize] = ((*ent).s.origin[i as usize] as libc::c_double * 8.0f64)
            as libc::c_short;
        (*current_client)
            .ps
            .pmove
            .velocity[i
            as usize] = ((*ent).velocity[i as usize] as libc::c_double * 8.0f64)
            as libc::c_short;
        i += 1;
    }
    if level.intermissiontime != 0. {
        (*current_client)
            .ps
            .blend[3 as libc::c_int as usize] = 0 as libc::c_int as libc::c_float;
        (*current_client).ps.fov = 90 as libc::c_int as libc::c_float;
        G_SetStats(ent);
        return;
    }
    AngleVectors(
        ((*(*ent).client).v_angle).as_mut_ptr(),
        forward.as_mut_ptr(),
        right.as_mut_ptr(),
        up.as_mut_ptr(),
    );
    P_WorldEffects();
    if (*(*ent).client).v_angle[0 as libc::c_int as usize]
        > 180 as libc::c_int as libc::c_float
    {
        (*ent)
            .s
            .angles[0 as libc::c_int
            as usize] = (-(360 as libc::c_int) as libc::c_float
            + (*(*ent).client).v_angle[0 as libc::c_int as usize])
            / 3 as libc::c_int as libc::c_float;
    } else {
        (*ent)
            .s
            .angles[0 as libc::c_int
            as usize] = (*(*ent).client).v_angle[0 as libc::c_int as usize]
            / 3 as libc::c_int as libc::c_float;
    }
    (*ent)
        .s
        .angles[1 as libc::c_int
        as usize] = (*(*ent).client).v_angle[1 as libc::c_int as usize];
    (*ent).s.angles[2 as libc::c_int as usize] = 0 as libc::c_int as vec_t;
    (*ent)
        .s
        .angles[2 as libc::c_int
        as usize] = SV_CalcRoll(
        ((*ent).s.angles).as_mut_ptr(),
        ((*ent).velocity).as_mut_ptr(),
    ) * 4 as libc::c_int as libc::c_float;
    xyspeed = sqrt(
        ((*ent).velocity[0 as libc::c_int as usize]
            * (*ent).velocity[0 as libc::c_int as usize]
            + (*ent).velocity[1 as libc::c_int as usize]
                * (*ent).velocity[1 as libc::c_int as usize]) as libc::c_double,
    ) as libc::c_float;
    if xyspeed < 5 as libc::c_int as libc::c_float {
        bobmove = 0 as libc::c_int as libc::c_float;
        (*current_client).bobtime = 0 as libc::c_int as libc::c_float;
    } else if !((*ent).groundentity).is_null() {
        if xyspeed > 210 as libc::c_int as libc::c_float {
            bobmove = 0.25f64 as libc::c_float;
        } else if xyspeed > 100 as libc::c_int as libc::c_float {
            bobmove = 0.125f64 as libc::c_float;
        } else {
            bobmove = 0.0625f64 as libc::c_float;
        }
    }
    let ref mut fresh20 = (*current_client).bobtime;
    *fresh20 += bobmove;
    bobtime = *fresh20;
    if (*current_client).ps.pmove.pm_flags as libc::c_int & 1 as libc::c_int != 0 {
        bobtime *= 4 as libc::c_int as libc::c_float;
    }
    bobcycle = bobtime as libc::c_int;
    bobfracsin = fabs(sin(bobtime as libc::c_double * 3.14159265358979323846f64))
        as libc::c_float;
    P_FallingDamage(ent);
    P_DamageFeedback(ent);
    SV_CalcViewOffset(ent);
    SV_CalcGunOffset(ent);
    SV_CalcBlend(ent);
    if ((*(*ent).client).chase_target).is_null() {
        G_SetStats(ent);
    }
    i = 1 as libc::c_int;
    while i as libc::c_float <= (*maxclients).value {
        let mut e: *mut edict_t = g_edicts.offset(i as isize);
        if (*e).inuse as u64 == 0 || (*(*e).client).chase_target != ent {
            i += 1;
        } else {
            memcpy(
                ((*(*e).client).ps.stats).as_mut_ptr() as *mut libc::c_void,
                ((*(*ent).client).ps.stats).as_mut_ptr() as *const libc::c_void,
                ::std::mem::size_of::<[libc::c_short; 32]>() as libc::c_ulong,
            );
            (*(*e).client)
                .ps
                .stats[13 as libc::c_int as usize] = 1 as libc::c_int as libc::c_short;
            break;
        }
    }
    G_SetClientEvent(ent);
    G_SetClientEffects(ent);
    G_SetClientSound(ent);
    G_SetClientFrame(ent);
    (*(*ent).client)
        .oldvelocity[0 as libc::c_int
        as usize] = (*ent).velocity[0 as libc::c_int as usize];
    (*(*ent).client)
        .oldvelocity[1 as libc::c_int
        as usize] = (*ent).velocity[1 as libc::c_int as usize];
    (*(*ent).client)
        .oldvelocity[2 as libc::c_int
        as usize] = (*ent).velocity[2 as libc::c_int as usize];
    (*(*ent).client)
        .oldviewangles[0 as libc::c_int
        as usize] = (*(*ent).client).ps.viewangles[0 as libc::c_int as usize];
    (*(*ent).client)
        .oldviewangles[1 as libc::c_int
        as usize] = (*(*ent).client).ps.viewangles[1 as libc::c_int as usize];
    (*(*ent).client)
        .oldviewangles[2 as libc::c_int
        as usize] = (*(*ent).client).ps.viewangles[2 as libc::c_int as usize];
    let ref mut fresh21 = (*(*ent).client).kick_origin[2 as libc::c_int as usize];
    *fresh21 = 0 as libc::c_int as vec_t;
    let ref mut fresh22 = (*(*ent).client).kick_origin[1 as libc::c_int as usize];
    *fresh22 = *fresh21;
    (*(*ent).client).kick_origin[0 as libc::c_int as usize] = *fresh22;
    let ref mut fresh23 = (*(*ent).client).kick_angles[2 as libc::c_int as usize];
    *fresh23 = 0 as libc::c_int as vec_t;
    let ref mut fresh24 = (*(*ent).client).kick_angles[1 as libc::c_int as usize];
    *fresh24 = *fresh23;
    (*(*ent).client).kick_angles[0 as libc::c_int as usize] = *fresh24;
    if (*(*ent).client).showscores as libc::c_uint != 0
        && level.framenum & 31 as libc::c_int == 0
    {
        if !((*(*ent).client).menu).is_null() {
            PMenu_Do_Update(ent);
            (*(*ent).client).menudirty = false_0;
            (*(*ent).client).menutime = level.time;
        } else {
            DeathmatchScoreboardMessage(ent, (*ent).enemy);
        }
        (gi.unicast).expect("non-null function pointer")(ent, false_0);
    }
}
