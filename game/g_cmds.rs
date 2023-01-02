#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn atoi(__nptr: *const libc::c_char) -> libc::c_int;
    fn qsort(
        __base: *mut libc::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
    static mut vec3_origin: vec3_t;
    fn Com_sprintf(
        dest: *mut libc::c_char,
        size: libc::c_int,
        fmt: *mut libc::c_char,
        _: ...
    );
    fn Q_stricmp(s1: *mut libc::c_char, s2: *mut libc::c_char) -> libc::c_int;
    fn Info_ValueForKey(
        s: *mut libc::c_char,
        key: *mut libc::c_char,
    ) -> *mut libc::c_char;
    fn ChasePrev(ent: *mut edict_t);
    static mut game: game_locals_t;
    static mut level: level_locals_t;
    static mut gi: game_import_t;
    static mut meansOfDeath: libc::c_int;
    static mut g_edicts: *mut edict_t;
    static mut deathmatch: *mut cvar_t;
    static mut dmflags: *mut cvar_t;
    static mut dedicated: *mut cvar_t;
    static mut sv_cheats: *mut cvar_t;
    static mut maxclients: *mut cvar_t;
    static mut flood_msgs: *mut cvar_t;
    static mut flood_persecond: *mut cvar_t;
    static mut flood_waitdelay: *mut cvar_t;
    static mut itemlist: [gitem_t; 0];
    fn Cmd_Help_f(ent: *mut edict_t);
    fn Cmd_Score_f(ent: *mut edict_t);
    fn FindItem(pickup_name: *mut libc::c_char) -> *mut gitem_t;
    fn SpawnItem(ent: *mut edict_t, item: *mut gitem_t);
    fn Add_Ammo(ent: *mut edict_t, item: *mut gitem_t, count: libc::c_int) -> qboolean;
    fn Touch_Item(
        ent: *mut edict_t,
        other: *mut edict_t,
        plane: *mut cplane_t,
        surf: *mut csurface_t,
    );
    fn G_Spawn() -> *mut edict_t;
    fn G_FreeEdict(e: *mut edict_t);
    fn player_die(
        self_0: *mut edict_t,
        inflictor: *mut edict_t,
        attacker: *mut edict_t,
        damage: libc::c_int,
        point: *mut vec_t,
    );
    fn ChaseNext(ent: *mut edict_t);
}
pub type size_t = libc::c_ulong;
pub type __compar_fn_t = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
>;
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
pub unsafe extern "C" fn ClientTeam(mut ent: *mut edict_t) -> *mut libc::c_char {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    static mut value: [libc::c_char; 512] = [0; 512];
    value[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    if ((*ent).client).is_null() {
        return value.as_mut_ptr();
    }
    strcpy(
        value.as_mut_ptr(),
        Info_ValueForKey(
            ((*(*ent).client).pers.userinfo).as_mut_ptr(),
            b"skin\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ),
    );
    p = strchr(value.as_mut_ptr(), '/' as i32);
    if p.is_null() {
        return value.as_mut_ptr();
    }
    if (*dmflags).value as libc::c_int & 0x80 as libc::c_int != 0 {
        *p = 0 as libc::c_int as libc::c_char;
        return value.as_mut_ptr();
    }
    p = p.offset(1);
    return p;
}
#[no_mangle]
pub unsafe extern "C" fn OnSameTeam(
    mut ent1: *mut edict_t,
    mut ent2: *mut edict_t,
) -> qboolean {
    let mut ent1Team: [libc::c_char; 512] = [0; 512];
    let mut ent2Team: [libc::c_char; 512] = [0; 512];
    if (*dmflags).value as libc::c_int & (0x80 as libc::c_int | 0x40 as libc::c_int) == 0
    {
        return false_0;
    }
    strcpy(ent1Team.as_mut_ptr(), ClientTeam(ent1));
    strcpy(ent2Team.as_mut_ptr(), ClientTeam(ent2));
    if strcmp(ent1Team.as_mut_ptr(), ent2Team.as_mut_ptr()) == 0 as libc::c_int {
        return true_0;
    }
    return false_0;
}
#[no_mangle]
pub unsafe extern "C" fn SelectNextItem(
    mut ent: *mut edict_t,
    mut itflags: libc::c_int,
) {
    let mut cl: *mut gclient_t = 0 as *mut gclient_t;
    let mut i: libc::c_int = 0;
    let mut index: libc::c_int = 0;
    let mut it: *mut gitem_t = 0 as *mut gitem_t;
    cl = (*ent).client;
    if !((*cl).chase_target).is_null() {
        ChaseNext(ent);
        return;
    }
    i = 1 as libc::c_int;
    while i <= 256 as libc::c_int {
        index = ((*cl).pers.selected_item + i) % 256 as libc::c_int;
        if !((*cl).pers.inventory[index as usize] == 0) {
            it = &mut *itemlist.as_mut_ptr().offset(index as isize) as *mut gitem_t;
            if !((*it).use_0).is_none() {
                if !((*it).flags & itflags == 0) {
                    (*cl).pers.selected_item = index;
                    return;
                }
            }
        }
        i += 1;
    }
    (*cl).pers.selected_item = -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn SelectPrevItem(
    mut ent: *mut edict_t,
    mut itflags: libc::c_int,
) {
    let mut cl: *mut gclient_t = 0 as *mut gclient_t;
    let mut i: libc::c_int = 0;
    let mut index: libc::c_int = 0;
    let mut it: *mut gitem_t = 0 as *mut gitem_t;
    cl = (*ent).client;
    if !((*cl).chase_target).is_null() {
        ChasePrev(ent);
        return;
    }
    i = 1 as libc::c_int;
    while i <= 256 as libc::c_int {
        index = ((*cl).pers.selected_item + 256 as libc::c_int - i) % 256 as libc::c_int;
        if !((*cl).pers.inventory[index as usize] == 0) {
            it = &mut *itemlist.as_mut_ptr().offset(index as isize) as *mut gitem_t;
            if !((*it).use_0).is_none() {
                if !((*it).flags & itflags == 0) {
                    (*cl).pers.selected_item = index;
                    return;
                }
            }
        }
        i += 1;
    }
    (*cl).pers.selected_item = -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn ValidateSelectedItem(mut ent: *mut edict_t) {
    let mut cl: *mut gclient_t = 0 as *mut gclient_t;
    cl = (*ent).client;
    if (*cl).pers.inventory[(*cl).pers.selected_item as usize] != 0 {
        return;
    }
    SelectNextItem(ent, -(1 as libc::c_int));
}
#[no_mangle]
pub unsafe extern "C" fn Cmd_Give_f(mut ent: *mut edict_t) {
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut it: *mut gitem_t = 0 as *mut gitem_t;
    let mut index: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut give_all: qboolean = false_0;
    let mut it_ent: *mut edict_t = 0 as *mut edict_t;
    if (*deathmatch).value != 0. && (*sv_cheats).value == 0. {
        (gi.cprintf)
            .expect(
                "non-null function pointer",
            )(
            ent,
            2 as libc::c_int,
            b"You must run the server with '+set cheats 1' to enable this command.\n\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        return;
    }
    name = (gi.args).expect("non-null function pointer")();
    if Q_stricmp(name, b"all\0" as *const u8 as *const libc::c_char as *mut libc::c_char)
        == 0 as libc::c_int
    {
        give_all = true_0;
    } else {
        give_all = false_0;
    }
    if give_all as libc::c_uint != 0
        || Q_stricmp(
            (gi.argv).expect("non-null function pointer")(1 as libc::c_int),
            b"health\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) == 0 as libc::c_int
    {
        if (gi.argc).expect("non-null function pointer")() == 3 as libc::c_int {
            (*ent)
                .health = atoi(
                (gi.argv).expect("non-null function pointer")(2 as libc::c_int),
            );
        } else {
            (*ent).health = (*ent).max_health;
        }
        if give_all as u64 == 0 {
            return;
        }
    }
    if give_all as libc::c_uint != 0
        || Q_stricmp(
            name,
            b"weapons\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) == 0 as libc::c_int
    {
        i = 0 as libc::c_int;
        while i < game.num_items {
            it = itemlist.as_mut_ptr().offset(i as isize);
            if !((*it).pickup).is_none() {
                if !((*it).flags & 1 as libc::c_int == 0) {
                    (*(*ent).client).pers.inventory[i as usize] += 1 as libc::c_int;
                }
            }
            i += 1;
        }
        if give_all as u64 == 0 {
            return;
        }
    }
    if give_all as libc::c_uint != 0
        || Q_stricmp(
            name,
            b"ammo\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) == 0 as libc::c_int
    {
        i = 0 as libc::c_int;
        while i < game.num_items {
            it = itemlist.as_mut_ptr().offset(i as isize);
            if !((*it).pickup).is_none() {
                if !((*it).flags & 2 as libc::c_int == 0) {
                    Add_Ammo(ent, it, 1000 as libc::c_int);
                }
            }
            i += 1;
        }
        if give_all as u64 == 0 {
            return;
        }
    }
    if give_all as libc::c_uint != 0
        || Q_stricmp(
            name,
            b"armor\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) == 0 as libc::c_int
    {
        let mut info: *mut gitem_armor_t = 0 as *mut gitem_armor_t;
        it = FindItem(
            b"Jacket Armor\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        (*(*ent).client)
            .pers
            .inventory[it.offset_from(itemlist.as_mut_ptr()) as libc::c_long
            as usize] = 0 as libc::c_int;
        it = FindItem(
            b"Combat Armor\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        (*(*ent).client)
            .pers
            .inventory[it.offset_from(itemlist.as_mut_ptr()) as libc::c_long
            as usize] = 0 as libc::c_int;
        it = FindItem(
            b"Body Armor\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        info = (*it).info as *mut gitem_armor_t;
        (*(*ent).client)
            .pers
            .inventory[it.offset_from(itemlist.as_mut_ptr()) as libc::c_long
            as usize] = (*info).max_count;
        if give_all as u64 == 0 {
            return;
        }
    }
    if give_all as libc::c_uint != 0
        || Q_stricmp(
            name,
            b"Power Shield\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) == 0 as libc::c_int
    {
        it = FindItem(
            b"Power Shield\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        it_ent = G_Spawn();
        let ref mut fresh0 = (*it_ent).classname;
        *fresh0 = (*it).classname;
        SpawnItem(it_ent, it);
        Touch_Item(it_ent, ent, 0 as *mut cplane_t, 0 as *mut csurface_t);
        if (*it_ent).inuse as u64 != 0 {
            G_FreeEdict(it_ent);
        }
        if give_all as u64 == 0 {
            return;
        }
    }
    if give_all as u64 != 0 {
        i = 0 as libc::c_int;
        while i < game.num_items {
            it = itemlist.as_mut_ptr().offset(i as isize);
            if !((*it).pickup).is_none() {
                if !((*it).flags
                    & (4 as libc::c_int | 1 as libc::c_int | 2 as libc::c_int) != 0)
                {
                    (*(*ent).client).pers.inventory[i as usize] = 1 as libc::c_int;
                }
            }
            i += 1;
        }
        return;
    }
    it = FindItem(name);
    if it.is_null() {
        name = (gi.argv).expect("non-null function pointer")(1 as libc::c_int);
        it = FindItem(name);
        if it.is_null() {
            (gi.cprintf)
                .expect(
                    "non-null function pointer",
                )(
                ent,
                2 as libc::c_int,
                b"unknown item\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
            return;
        }
    }
    if ((*it).pickup).is_none() {
        (gi.cprintf)
            .expect(
                "non-null function pointer",
            )(
            ent,
            2 as libc::c_int,
            b"non-pickup item\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return;
    }
    index = it.offset_from(itemlist.as_mut_ptr()) as libc::c_long as libc::c_int;
    if (*it).flags & 2 as libc::c_int != 0 {
        if (gi.argc).expect("non-null function pointer")() == 3 as libc::c_int {
            (*(*ent).client)
                .pers
                .inventory[index
                as usize] = atoi(
                (gi.argv).expect("non-null function pointer")(2 as libc::c_int),
            );
        } else {
            (*(*ent).client).pers.inventory[index as usize] += (*it).quantity;
        }
    } else {
        it_ent = G_Spawn();
        let ref mut fresh1 = (*it_ent).classname;
        *fresh1 = (*it).classname;
        SpawnItem(it_ent, it);
        Touch_Item(it_ent, ent, 0 as *mut cplane_t, 0 as *mut csurface_t);
        if (*it_ent).inuse as u64 != 0 {
            G_FreeEdict(it_ent);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn Cmd_God_f(mut ent: *mut edict_t) {
    let mut msg: *mut libc::c_char = 0 as *mut libc::c_char;
    if (*deathmatch).value != 0. && (*sv_cheats).value == 0. {
        (gi.cprintf)
            .expect(
                "non-null function pointer",
            )(
            ent,
            2 as libc::c_int,
            b"You must run the server with '+set cheats 1' to enable this command.\n\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        return;
    }
    (*ent).flags ^= 0x10 as libc::c_int;
    if (*ent).flags & 0x10 as libc::c_int == 0 {
        msg = b"godmode OFF\n\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char;
    } else {
        msg = b"godmode ON\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    (gi.cprintf).expect("non-null function pointer")(ent, 2 as libc::c_int, msg);
}
#[no_mangle]
pub unsafe extern "C" fn Cmd_Notarget_f(mut ent: *mut edict_t) {
    let mut msg: *mut libc::c_char = 0 as *mut libc::c_char;
    if (*deathmatch).value != 0. && (*sv_cheats).value == 0. {
        (gi.cprintf)
            .expect(
                "non-null function pointer",
            )(
            ent,
            2 as libc::c_int,
            b"You must run the server with '+set cheats 1' to enable this command.\n\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        return;
    }
    (*ent).flags ^= 0x20 as libc::c_int;
    if (*ent).flags & 0x20 as libc::c_int == 0 {
        msg = b"notarget OFF\n\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char;
    } else {
        msg = b"notarget ON\n\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char;
    }
    (gi.cprintf).expect("non-null function pointer")(ent, 2 as libc::c_int, msg);
}
#[no_mangle]
pub unsafe extern "C" fn Cmd_Noclip_f(mut ent: *mut edict_t) {
    let mut msg: *mut libc::c_char = 0 as *mut libc::c_char;
    if (*deathmatch).value != 0. && (*sv_cheats).value == 0. {
        (gi.cprintf)
            .expect(
                "non-null function pointer",
            )(
            ent,
            2 as libc::c_int,
            b"You must run the server with '+set cheats 1' to enable this command.\n\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        return;
    }
    if (*ent).movetype == MOVETYPE_NOCLIP as libc::c_int {
        (*ent).movetype = MOVETYPE_WALK as libc::c_int;
        msg = b"noclip OFF\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    } else {
        (*ent).movetype = MOVETYPE_NOCLIP as libc::c_int;
        msg = b"noclip ON\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    (gi.cprintf).expect("non-null function pointer")(ent, 2 as libc::c_int, msg);
}
#[no_mangle]
pub unsafe extern "C" fn Cmd_Use_f(mut ent: *mut edict_t) {
    let mut index: libc::c_int = 0;
    let mut it: *mut gitem_t = 0 as *mut gitem_t;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    s = (gi.args).expect("non-null function pointer")();
    it = FindItem(s);
    if it.is_null() {
        (gi.cprintf)
            .expect(
                "non-null function pointer",
            )(
            ent,
            2 as libc::c_int,
            b"unknown item: %s\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            s,
        );
        return;
    }
    if ((*it).use_0).is_none() {
        (gi.cprintf)
            .expect(
                "non-null function pointer",
            )(
            ent,
            2 as libc::c_int,
            b"Item is not usable.\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return;
    }
    index = it.offset_from(itemlist.as_mut_ptr()) as libc::c_long as libc::c_int;
    if (*(*ent).client).pers.inventory[index as usize] == 0 {
        (gi.cprintf)
            .expect(
                "non-null function pointer",
            )(
            ent,
            2 as libc::c_int,
            b"Out of item: %s\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            s,
        );
        return;
    }
    ((*it).use_0).expect("non-null function pointer")(ent, it);
}
#[no_mangle]
pub unsafe extern "C" fn Cmd_Drop_f(mut ent: *mut edict_t) {
    let mut index: libc::c_int = 0;
    let mut it: *mut gitem_t = 0 as *mut gitem_t;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    s = (gi.args).expect("non-null function pointer")();
    it = FindItem(s);
    if it.is_null() {
        (gi.cprintf)
            .expect(
                "non-null function pointer",
            )(
            ent,
            2 as libc::c_int,
            b"unknown item: %s\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            s,
        );
        return;
    }
    if ((*it).drop).is_none() {
        (gi.cprintf)
            .expect(
                "non-null function pointer",
            )(
            ent,
            2 as libc::c_int,
            b"Item is not dropable.\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return;
    }
    index = it.offset_from(itemlist.as_mut_ptr()) as libc::c_long as libc::c_int;
    if (*(*ent).client).pers.inventory[index as usize] == 0 {
        (gi.cprintf)
            .expect(
                "non-null function pointer",
            )(
            ent,
            2 as libc::c_int,
            b"Out of item: %s\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            s,
        );
        return;
    }
    ((*it).drop).expect("non-null function pointer")(ent, it);
}
#[no_mangle]
pub unsafe extern "C" fn Cmd_Inven_f(mut ent: *mut edict_t) {
    let mut i: libc::c_int = 0;
    let mut cl: *mut gclient_t = 0 as *mut gclient_t;
    cl = (*ent).client;
    (*cl).showscores = false_0;
    (*cl).showhelp = false_0;
    if (*cl).showinventory as u64 != 0 {
        (*cl).showinventory = false_0;
        return;
    }
    (*cl).showinventory = true_0;
    (gi.WriteByte).expect("non-null function pointer")(5 as libc::c_int);
    i = 0 as libc::c_int;
    while i < 256 as libc::c_int {
        (gi.WriteShort)
            .expect("non-null function pointer")((*cl).pers.inventory[i as usize]);
        i += 1;
    }
    (gi.unicast).expect("non-null function pointer")(ent, true_0);
}
#[no_mangle]
pub unsafe extern "C" fn Cmd_InvUse_f(mut ent: *mut edict_t) {
    let mut it: *mut gitem_t = 0 as *mut gitem_t;
    ValidateSelectedItem(ent);
    if (*(*ent).client).pers.selected_item == -(1 as libc::c_int) {
        (gi.cprintf)
            .expect(
                "non-null function pointer",
            )(
            ent,
            2 as libc::c_int,
            b"No item to use.\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return;
    }
    it = &mut *itemlist.as_mut_ptr().offset((*(*ent).client).pers.selected_item as isize)
        as *mut gitem_t;
    if ((*it).use_0).is_none() {
        (gi.cprintf)
            .expect(
                "non-null function pointer",
            )(
            ent,
            2 as libc::c_int,
            b"Item is not usable.\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return;
    }
    ((*it).use_0).expect("non-null function pointer")(ent, it);
}
#[no_mangle]
pub unsafe extern "C" fn Cmd_WeapPrev_f(mut ent: *mut edict_t) {
    let mut cl: *mut gclient_t = 0 as *mut gclient_t;
    let mut i: libc::c_int = 0;
    let mut index: libc::c_int = 0;
    let mut it: *mut gitem_t = 0 as *mut gitem_t;
    let mut selected_weapon: libc::c_int = 0;
    cl = (*ent).client;
    if ((*cl).pers.weapon).is_null() {
        return;
    }
    selected_weapon = ((*cl).pers.weapon).offset_from(itemlist.as_mut_ptr())
        as libc::c_long as libc::c_int;
    i = 1 as libc::c_int;
    while i <= 256 as libc::c_int {
        index = (selected_weapon + i) % 256 as libc::c_int;
        if !((*cl).pers.inventory[index as usize] == 0) {
            it = &mut *itemlist.as_mut_ptr().offset(index as isize) as *mut gitem_t;
            if !((*it).use_0).is_none() {
                if !((*it).flags & 1 as libc::c_int == 0) {
                    ((*it).use_0).expect("non-null function pointer")(ent, it);
                    if (*cl).pers.weapon == it {
                        return;
                    }
                }
            }
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn Cmd_WeapNext_f(mut ent: *mut edict_t) {
    let mut cl: *mut gclient_t = 0 as *mut gclient_t;
    let mut i: libc::c_int = 0;
    let mut index: libc::c_int = 0;
    let mut it: *mut gitem_t = 0 as *mut gitem_t;
    let mut selected_weapon: libc::c_int = 0;
    cl = (*ent).client;
    if ((*cl).pers.weapon).is_null() {
        return;
    }
    selected_weapon = ((*cl).pers.weapon).offset_from(itemlist.as_mut_ptr())
        as libc::c_long as libc::c_int;
    i = 1 as libc::c_int;
    while i <= 256 as libc::c_int {
        index = (selected_weapon + 256 as libc::c_int - i) % 256 as libc::c_int;
        if !((*cl).pers.inventory[index as usize] == 0) {
            it = &mut *itemlist.as_mut_ptr().offset(index as isize) as *mut gitem_t;
            if !((*it).use_0).is_none() {
                if !((*it).flags & 1 as libc::c_int == 0) {
                    ((*it).use_0).expect("non-null function pointer")(ent, it);
                    if (*cl).pers.weapon == it {
                        return;
                    }
                }
            }
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn Cmd_WeapLast_f(mut ent: *mut edict_t) {
    let mut cl: *mut gclient_t = 0 as *mut gclient_t;
    let mut index: libc::c_int = 0;
    let mut it: *mut gitem_t = 0 as *mut gitem_t;
    cl = (*ent).client;
    if ((*cl).pers.weapon).is_null() || ((*cl).pers.lastweapon).is_null() {
        return;
    }
    index = ((*cl).pers.lastweapon).offset_from(itemlist.as_mut_ptr()) as libc::c_long
        as libc::c_int;
    if (*cl).pers.inventory[index as usize] == 0 {
        return;
    }
    it = &mut *itemlist.as_mut_ptr().offset(index as isize) as *mut gitem_t;
    if ((*it).use_0).is_none() {
        return;
    }
    if (*it).flags & 1 as libc::c_int == 0 {
        return;
    }
    ((*it).use_0).expect("non-null function pointer")(ent, it);
}
#[no_mangle]
pub unsafe extern "C" fn Cmd_InvDrop_f(mut ent: *mut edict_t) {
    let mut it: *mut gitem_t = 0 as *mut gitem_t;
    ValidateSelectedItem(ent);
    if (*(*ent).client).pers.selected_item == -(1 as libc::c_int) {
        (gi.cprintf)
            .expect(
                "non-null function pointer",
            )(
            ent,
            2 as libc::c_int,
            b"No item to drop.\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return;
    }
    it = &mut *itemlist.as_mut_ptr().offset((*(*ent).client).pers.selected_item as isize)
        as *mut gitem_t;
    if ((*it).drop).is_none() {
        (gi.cprintf)
            .expect(
                "non-null function pointer",
            )(
            ent,
            2 as libc::c_int,
            b"Item is not dropable.\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return;
    }
    ((*it).drop).expect("non-null function pointer")(ent, it);
}
#[no_mangle]
pub unsafe extern "C" fn Cmd_Kill_f(mut ent: *mut edict_t) {
    if level.time - (*(*ent).client).respawn_time < 5 as libc::c_int as libc::c_float {
        return;
    }
    (*ent).flags &= !(0x10 as libc::c_int);
    (*ent).health = 0 as libc::c_int;
    meansOfDeath = 23 as libc::c_int;
    player_die(ent, ent, ent, 100000 as libc::c_int, vec3_origin.as_mut_ptr());
}
#[no_mangle]
pub unsafe extern "C" fn Cmd_PutAway_f(mut ent: *mut edict_t) {
    (*(*ent).client).showscores = false_0;
    (*(*ent).client).showhelp = false_0;
    (*(*ent).client).showinventory = false_0;
}
#[no_mangle]
pub unsafe extern "C" fn PlayerSort(
    mut a: *const libc::c_void,
    mut b: *const libc::c_void,
) -> libc::c_int {
    let mut anum: libc::c_int = 0;
    let mut bnum: libc::c_int = 0;
    anum = *(a as *mut libc::c_int);
    bnum = *(b as *mut libc::c_int);
    anum = (*(game.clients).offset(anum as isize)).ps.stats[14 as libc::c_int as usize]
        as libc::c_int;
    bnum = (*(game.clients).offset(bnum as isize)).ps.stats[14 as libc::c_int as usize]
        as libc::c_int;
    if anum < bnum {
        return -(1 as libc::c_int);
    }
    if anum > bnum {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn Cmd_Players_f(mut ent: *mut edict_t) {
    let mut i: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    let mut small: [libc::c_char; 64] = [0; 64];
    let mut large: [libc::c_char; 1280] = [0; 1280];
    let mut index: [libc::c_int; 256] = [0; 256];
    count = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while (i as libc::c_float) < (*maxclients).value {
        if (*(game.clients).offset(i as isize)).pers.connected as u64 != 0 {
            index[count as usize] = i;
            count += 1;
        }
        i += 1;
    }
    qsort(
        index.as_mut_ptr() as *mut libc::c_void,
        count as size_t,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
        Some(
            PlayerSort
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> libc::c_int,
        ),
    );
    large[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    i = 0 as libc::c_int;
    while i < count {
        Com_sprintf(
            small.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
            b"%3i %s\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            (*(game.clients).offset(index[i as usize] as isize))
                .ps
                .stats[14 as libc::c_int as usize] as libc::c_int,
            ((*(game.clients).offset(index[i as usize] as isize)).pers.netname)
                .as_mut_ptr(),
        );
        if (strlen(small.as_mut_ptr())).wrapping_add(strlen(large.as_mut_ptr()))
            > (::std::mem::size_of::<[libc::c_char; 1280]>() as libc::c_ulong)
                .wrapping_sub(100 as libc::c_int as libc::c_ulong)
        {
            strcat(large.as_mut_ptr(), b"...\n\0" as *const u8 as *const libc::c_char);
            break;
        } else {
            strcat(large.as_mut_ptr(), small.as_mut_ptr());
            i += 1;
        }
    }
    (gi.cprintf)
        .expect(
            "non-null function pointer",
        )(
        ent,
        2 as libc::c_int,
        b"%s\n%i players\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        large.as_mut_ptr(),
        count,
    );
}
#[no_mangle]
pub unsafe extern "C" fn Cmd_Wave_f(mut ent: *mut edict_t) {
    let mut i: libc::c_int = 0;
    i = atoi((gi.argv).expect("non-null function pointer")(1 as libc::c_int));
    if (*(*ent).client).ps.pmove.pm_flags as libc::c_int & 1 as libc::c_int != 0 {
        return;
    }
    if (*(*ent).client).anim_priority > 1 as libc::c_int {
        return;
    }
    (*(*ent).client).anim_priority = 1 as libc::c_int;
    match i {
        0 => {
            (gi.cprintf)
                .expect(
                    "non-null function pointer",
                )(
                ent,
                2 as libc::c_int,
                b"flipoff\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            (*ent).s.frame = 72 as libc::c_int - 1 as libc::c_int;
            (*(*ent).client).anim_end = 83 as libc::c_int;
        }
        1 => {
            (gi.cprintf)
                .expect(
                    "non-null function pointer",
                )(
                ent,
                2 as libc::c_int,
                b"salute\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            (*ent).s.frame = 84 as libc::c_int - 1 as libc::c_int;
            (*(*ent).client).anim_end = 94 as libc::c_int;
        }
        2 => {
            (gi.cprintf)
                .expect(
                    "non-null function pointer",
                )(
                ent,
                2 as libc::c_int,
                b"taunt\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            (*ent).s.frame = 95 as libc::c_int - 1 as libc::c_int;
            (*(*ent).client).anim_end = 111 as libc::c_int;
        }
        3 => {
            (gi.cprintf)
                .expect(
                    "non-null function pointer",
                )(
                ent,
                2 as libc::c_int,
                b"wave\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            (*ent).s.frame = 112 as libc::c_int - 1 as libc::c_int;
            (*(*ent).client).anim_end = 122 as libc::c_int;
        }
        4 | _ => {
            (gi.cprintf)
                .expect(
                    "non-null function pointer",
                )(
                ent,
                2 as libc::c_int,
                b"point\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            (*ent).s.frame = 123 as libc::c_int - 1 as libc::c_int;
            (*(*ent).client).anim_end = 134 as libc::c_int;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn Cmd_Say_f(
    mut ent: *mut edict_t,
    mut team: qboolean,
    mut arg0: qboolean,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut other: *mut edict_t = 0 as *mut edict_t;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut text: [libc::c_char; 2048] = [0; 2048];
    let mut cl: *mut gclient_t = 0 as *mut gclient_t;
    if (gi.argc).expect("non-null function pointer")() < 2 as libc::c_int
        && arg0 as u64 == 0
    {
        return;
    }
    if (*dmflags).value as libc::c_int & (0x80 as libc::c_int | 0x40 as libc::c_int) == 0
    {
        team = false_0;
    }
    if team as u64 != 0 {
        Com_sprintf(
            text.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 2048]>() as libc::c_ulong
                as libc::c_int,
            b"(%s): \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ((*(*ent).client).pers.netname).as_mut_ptr(),
        );
    } else {
        Com_sprintf(
            text.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 2048]>() as libc::c_ulong
                as libc::c_int,
            b"%s: \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ((*(*ent).client).pers.netname).as_mut_ptr(),
        );
    }
    if arg0 as u64 != 0 {
        strcat(
            text.as_mut_ptr(),
            (gi.argv).expect("non-null function pointer")(0 as libc::c_int),
        );
        strcat(text.as_mut_ptr(), b" \0" as *const u8 as *const libc::c_char);
        strcat(text.as_mut_ptr(), (gi.args).expect("non-null function pointer")());
    } else {
        p = (gi.args).expect("non-null function pointer")();
        if *p as libc::c_int == '"' as i32 {
            p = p.offset(1);
            *p
                .offset(
                    (strlen(p)).wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
                ) = 0 as libc::c_int as libc::c_char;
        }
        strcat(text.as_mut_ptr(), p);
    }
    if strlen(text.as_mut_ptr()) > 150 as libc::c_int as libc::c_ulong {
        text[150 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    }
    strcat(text.as_mut_ptr(), b"\n\0" as *const u8 as *const libc::c_char);
    if (*flood_msgs).value != 0. {
        cl = (*ent).client;
        if level.time < (*cl).flood_locktill {
            (gi.cprintf)
                .expect(
                    "non-null function pointer",
                )(
                ent,
                2 as libc::c_int,
                b"You can't talk for %d more seconds\n\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ((*cl).flood_locktill - level.time) as libc::c_int,
            );
            return;
        }
        i = ((*cl).flood_whenhead as libc::c_float - (*flood_msgs).value
            + 1 as libc::c_int as libc::c_float) as libc::c_int;
        if i < 0 as libc::c_int {
            i = (::std::mem::size_of::<[libc::c_float; 10]>() as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<libc::c_float>() as libc::c_ulong)
                .wrapping_add(i as libc::c_ulong) as libc::c_int;
        }
        if (*cl).flood_when[i as usize] != 0.
            && level.time - (*cl).flood_when[i as usize] < (*flood_persecond).value
        {
            (*cl).flood_locktill = level.time + (*flood_waitdelay).value;
            (gi.cprintf)
                .expect(
                    "non-null function pointer",
                )(
                ent,
                3 as libc::c_int,
                b"Flood protection:  You can't talk for %d seconds.\n\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                (*flood_waitdelay).value as libc::c_int,
            );
            return;
        }
        (*cl)
            .flood_whenhead = (((*cl).flood_whenhead + 1 as libc::c_int)
            as libc::c_ulong)
            .wrapping_rem(
                (::std::mem::size_of::<[libc::c_float; 10]>() as libc::c_ulong)
                    .wrapping_div(
                        ::std::mem::size_of::<libc::c_float>() as libc::c_ulong,
                    ),
            ) as libc::c_int;
        (*cl).flood_when[(*cl).flood_whenhead as usize] = level.time;
    }
    if (*dedicated).value != 0. {
        (gi.cprintf)
            .expect(
                "non-null function pointer",
            )(
            0 as *mut edict_t,
            3 as libc::c_int,
            b"%s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            text.as_mut_ptr(),
        );
    }
    let mut current_block_43: u64;
    j = 1 as libc::c_int;
    while j <= game.maxclients {
        other = &mut *g_edicts.offset(j as isize) as *mut edict_t;
        if !((*other).inuse as u64 == 0) {
            if !((*other).client).is_null() {
                if team as u64 != 0 {
                    if OnSameTeam(ent, other) as u64 == 0 {
                        current_block_43 = 14072441030219150333;
                    } else {
                        current_block_43 = 12381812505308290051;
                    }
                } else {
                    current_block_43 = 12381812505308290051;
                }
                match current_block_43 {
                    14072441030219150333 => {}
                    _ => {
                        (gi.cprintf)
                            .expect(
                                "non-null function pointer",
                            )(
                            other,
                            3 as libc::c_int,
                            b"%s\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char,
                            text.as_mut_ptr(),
                        );
                    }
                }
            }
        }
        j += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn Cmd_PlayerList_f(mut ent: *mut edict_t) {
    let mut i: libc::c_int = 0;
    let mut st: [libc::c_char; 80] = [0; 80];
    let mut text: [libc::c_char; 1400] = [0; 1400];
    let mut e2: *mut edict_t = 0 as *mut edict_t;
    *text.as_mut_ptr() = 0 as libc::c_int as libc::c_char;
    i = 0 as libc::c_int;
    e2 = g_edicts.offset(1 as libc::c_int as isize);
    while (i as libc::c_float) < (*maxclients).value {
        if !((*e2).inuse as u64 == 0) {
            sprintf(
                st.as_mut_ptr(),
                b"%02d:%02d %4d %3d %s%s\n\0" as *const u8 as *const libc::c_char,
                (level.framenum - (*(*e2).client).resp.enterframe) / 600 as libc::c_int,
                (level.framenum - (*(*e2).client).resp.enterframe) % 600 as libc::c_int
                    / 10 as libc::c_int,
                (*(*e2).client).ping,
                (*(*e2).client).resp.score,
                ((*(*e2).client).pers.netname).as_mut_ptr(),
                if (*(*e2).client).resp.spectator as libc::c_uint != 0 {
                    b" (spectator)\0" as *const u8 as *const libc::c_char
                } else {
                    b"\0" as *const u8 as *const libc::c_char
                },
            );
            if (strlen(text.as_mut_ptr())).wrapping_add(strlen(st.as_mut_ptr()))
                > (::std::mem::size_of::<[libc::c_char; 1400]>() as libc::c_ulong)
                    .wrapping_sub(50 as libc::c_int as libc::c_ulong)
            {
                sprintf(
                    text.as_mut_ptr().offset(strlen(text.as_mut_ptr()) as isize),
                    b"And more...\n\0" as *const u8 as *const libc::c_char,
                );
                (gi.cprintf)
                    .expect(
                        "non-null function pointer",
                    )(
                    ent,
                    2 as libc::c_int,
                    b"%s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    text.as_mut_ptr(),
                );
                return;
            }
            strcat(text.as_mut_ptr(), st.as_mut_ptr());
        }
        i += 1;
        e2 = e2.offset(1);
    }
    (gi.cprintf)
        .expect(
            "non-null function pointer",
        )(
        ent,
        2 as libc::c_int,
        b"%s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        text.as_mut_ptr(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn ClientCommand(mut ent: *mut edict_t) {
    let mut cmd: *mut libc::c_char = 0 as *mut libc::c_char;
    if ((*ent).client).is_null() {
        return;
    }
    cmd = (gi.argv).expect("non-null function pointer")(0 as libc::c_int);
    if Q_stricmp(
        cmd,
        b"players\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) == 0 as libc::c_int
    {
        Cmd_Players_f(ent);
        return;
    }
    if Q_stricmp(cmd, b"say\0" as *const u8 as *const libc::c_char as *mut libc::c_char)
        == 0 as libc::c_int
    {
        Cmd_Say_f(ent, false_0, false_0);
        return;
    }
    if Q_stricmp(
        cmd,
        b"say_team\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) == 0 as libc::c_int
    {
        Cmd_Say_f(ent, true_0, false_0);
        return;
    }
    if Q_stricmp(
        cmd,
        b"score\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) == 0 as libc::c_int
    {
        Cmd_Score_f(ent);
        return;
    }
    if Q_stricmp(cmd, b"help\0" as *const u8 as *const libc::c_char as *mut libc::c_char)
        == 0 as libc::c_int
    {
        Cmd_Help_f(ent);
        return;
    }
    if level.intermissiontime != 0. {
        return;
    }
    if Q_stricmp(cmd, b"use\0" as *const u8 as *const libc::c_char as *mut libc::c_char)
        == 0 as libc::c_int
    {
        Cmd_Use_f(ent);
    } else if Q_stricmp(
        cmd,
        b"drop\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) == 0 as libc::c_int
    {
        Cmd_Drop_f(ent);
    } else if Q_stricmp(
        cmd,
        b"give\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) == 0 as libc::c_int
    {
        Cmd_Give_f(ent);
    } else if Q_stricmp(
        cmd,
        b"god\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) == 0 as libc::c_int
    {
        Cmd_God_f(ent);
    } else if Q_stricmp(
        cmd,
        b"notarget\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) == 0 as libc::c_int
    {
        Cmd_Notarget_f(ent);
    } else if Q_stricmp(
        cmd,
        b"noclip\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) == 0 as libc::c_int
    {
        Cmd_Noclip_f(ent);
    } else if Q_stricmp(
        cmd,
        b"inven\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) == 0 as libc::c_int
    {
        Cmd_Inven_f(ent);
    } else if Q_stricmp(
        cmd,
        b"invnext\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) == 0 as libc::c_int
    {
        SelectNextItem(ent, -(1 as libc::c_int));
    } else if Q_stricmp(
        cmd,
        b"invprev\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) == 0 as libc::c_int
    {
        SelectPrevItem(ent, -(1 as libc::c_int));
    } else if Q_stricmp(
        cmd,
        b"invnextw\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) == 0 as libc::c_int
    {
        SelectNextItem(ent, 1 as libc::c_int);
    } else if Q_stricmp(
        cmd,
        b"invprevw\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) == 0 as libc::c_int
    {
        SelectPrevItem(ent, 1 as libc::c_int);
    } else if Q_stricmp(
        cmd,
        b"invnextp\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) == 0 as libc::c_int
    {
        SelectNextItem(ent, 32 as libc::c_int);
    } else if Q_stricmp(
        cmd,
        b"invprevp\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) == 0 as libc::c_int
    {
        SelectPrevItem(ent, 32 as libc::c_int);
    } else if Q_stricmp(
        cmd,
        b"invuse\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) == 0 as libc::c_int
    {
        Cmd_InvUse_f(ent);
    } else if Q_stricmp(
        cmd,
        b"invdrop\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) == 0 as libc::c_int
    {
        Cmd_InvDrop_f(ent);
    } else if Q_stricmp(
        cmd,
        b"weapprev\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) == 0 as libc::c_int
    {
        Cmd_WeapPrev_f(ent);
    } else if Q_stricmp(
        cmd,
        b"weapnext\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) == 0 as libc::c_int
    {
        Cmd_WeapNext_f(ent);
    } else if Q_stricmp(
        cmd,
        b"weaplast\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) == 0 as libc::c_int
    {
        Cmd_WeapLast_f(ent);
    } else if Q_stricmp(
        cmd,
        b"kill\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) == 0 as libc::c_int
    {
        Cmd_Kill_f(ent);
    } else if Q_stricmp(
        cmd,
        b"putaway\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) == 0 as libc::c_int
    {
        Cmd_PutAway_f(ent);
    } else if Q_stricmp(
        cmd,
        b"wave\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) == 0 as libc::c_int
    {
        Cmd_Wave_f(ent);
    } else if Q_stricmp(
        cmd,
        b"playerlist\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) == 0 as libc::c_int
    {
        Cmd_PlayerList_f(ent);
    } else {
        Cmd_Say_f(ent, false_0, true_0);
    };
}
