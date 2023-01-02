#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    fn floor(_: libc::c_double) -> libc::c_double;
    fn M_walkmove(
        ent: *mut edict_t,
        yaw: libc::c_float,
        dist: libc::c_float,
    ) -> qboolean;
    fn FoundTarget(self_0: *mut edict_t);
    fn M_CheckAttack(self_0: *mut edict_t) -> qboolean;
    fn fire_bfg(
        self_0: *mut edict_t,
        start: *mut vec_t,
        dir: *mut vec_t,
        damage: libc::c_int,
        speed: libc::c_int,
        damage_radius: libc::c_float,
    );
    fn fire_rail(
        self_0: *mut edict_t,
        start: *mut vec_t,
        aimdir: *mut vec_t,
        damage: libc::c_int,
        kick: libc::c_int,
    );
    fn fire_rocket(
        self_0: *mut edict_t,
        start: *mut vec_t,
        dir: *mut vec_t,
        damage: libc::c_int,
        speed: libc::c_int,
        damage_radius: libc::c_float,
        radius_damage: libc::c_int,
    );
    fn fire_grenade(
        self_0: *mut edict_t,
        start: *mut vec_t,
        aimdir: *mut vec_t,
        damage: libc::c_int,
        speed: libc::c_int,
        timer: libc::c_float,
        damage_radius: libc::c_float,
    );
    fn fire_blaster(
        self_0: *mut edict_t,
        start: *mut vec_t,
        aimdir: *mut vec_t,
        damage: libc::c_int,
        speed: libc::c_int,
        effect: libc::c_int,
        hyper: qboolean,
    );
    fn fire_shotgun(
        self_0: *mut edict_t,
        start: *mut vec_t,
        aimdir: *mut vec_t,
        damage: libc::c_int,
        kick: libc::c_int,
        hspread: libc::c_int,
        vspread: libc::c_int,
        count: libc::c_int,
        mod_0: libc::c_int,
    );
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn rand() -> libc::c_int;
    static mut vec3_origin: vec3_t;
    static mut level: level_locals_t;
    static mut gi: game_import_t;
    static mut st: spawn_temp_t;
    static mut g_edicts: *mut edict_t;
    static mut deathmatch: *mut cvar_t;
    fn FindItemByClassname(classname: *mut libc::c_char) -> *mut gitem_t;
    fn Drop_Item(ent: *mut edict_t, item: *mut gitem_t) -> *mut edict_t;
    fn KillBox(ent: *mut edict_t) -> qboolean;
    fn G_Find(
        from: *mut edict_t,
        fieldofs: libc::c_int,
        match_0: *mut libc::c_char,
    ) -> *mut edict_t;
    fn G_PickTarget(targetname: *mut libc::c_char) -> *mut edict_t;
    fn G_UseTargets(ent: *mut edict_t, activator: *mut edict_t);
    fn G_FreeEdict(e: *mut edict_t);
    fn vtos(v: *mut vec_t) -> *mut libc::c_char;
    fn vectoyaw(vec: *mut vec_t) -> libc::c_float;
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
    fn fire_bullet(
        self_0: *mut edict_t,
        start: *mut vec_t,
        aimdir: *mut vec_t,
        damage: libc::c_int,
        kick: libc::c_int,
        hspread: libc::c_int,
        vspread: libc::c_int,
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
pub unsafe extern "C" fn monster_fire_bullet(
    mut self_0: *mut edict_t,
    mut start: *mut vec_t,
    mut dir: *mut vec_t,
    mut damage: libc::c_int,
    mut kick: libc::c_int,
    mut hspread: libc::c_int,
    mut vspread: libc::c_int,
    mut flashtype: libc::c_int,
) {
    fire_bullet(self_0, start, dir, damage, kick, hspread, vspread, 0 as libc::c_int);
    (gi.WriteByte).expect("non-null function pointer")(2 as libc::c_int);
    (gi.WriteShort)
        .expect(
            "non-null function pointer",
        )(self_0.offset_from(g_edicts) as libc::c_long as libc::c_int);
    (gi.WriteByte).expect("non-null function pointer")(flashtype);
    (gi.multicast).expect("non-null function pointer")(start, MULTICAST_PVS);
}
#[no_mangle]
pub unsafe extern "C" fn monster_fire_shotgun(
    mut self_0: *mut edict_t,
    mut start: *mut vec_t,
    mut aimdir: *mut vec_t,
    mut damage: libc::c_int,
    mut kick: libc::c_int,
    mut hspread: libc::c_int,
    mut vspread: libc::c_int,
    mut count: libc::c_int,
    mut flashtype: libc::c_int,
) {
    fire_shotgun(
        self_0,
        start,
        aimdir,
        damage,
        kick,
        hspread,
        vspread,
        count,
        0 as libc::c_int,
    );
    (gi.WriteByte).expect("non-null function pointer")(2 as libc::c_int);
    (gi.WriteShort)
        .expect(
            "non-null function pointer",
        )(self_0.offset_from(g_edicts) as libc::c_long as libc::c_int);
    (gi.WriteByte).expect("non-null function pointer")(flashtype);
    (gi.multicast).expect("non-null function pointer")(start, MULTICAST_PVS);
}
#[no_mangle]
pub unsafe extern "C" fn monster_fire_blaster(
    mut self_0: *mut edict_t,
    mut start: *mut vec_t,
    mut dir: *mut vec_t,
    mut damage: libc::c_int,
    mut speed: libc::c_int,
    mut flashtype: libc::c_int,
    mut effect: libc::c_int,
) {
    fire_blaster(self_0, start, dir, damage, speed, effect, false_0);
    (gi.WriteByte).expect("non-null function pointer")(2 as libc::c_int);
    (gi.WriteShort)
        .expect(
            "non-null function pointer",
        )(self_0.offset_from(g_edicts) as libc::c_long as libc::c_int);
    (gi.WriteByte).expect("non-null function pointer")(flashtype);
    (gi.multicast).expect("non-null function pointer")(start, MULTICAST_PVS);
}
#[no_mangle]
pub unsafe extern "C" fn monster_fire_grenade(
    mut self_0: *mut edict_t,
    mut start: *mut vec_t,
    mut aimdir: *mut vec_t,
    mut damage: libc::c_int,
    mut speed: libc::c_int,
    mut flashtype: libc::c_int,
) {
    fire_grenade(
        self_0,
        start,
        aimdir,
        damage,
        speed,
        2.5f64 as libc::c_float,
        (damage + 40 as libc::c_int) as libc::c_float,
    );
    (gi.WriteByte).expect("non-null function pointer")(2 as libc::c_int);
    (gi.WriteShort)
        .expect(
            "non-null function pointer",
        )(self_0.offset_from(g_edicts) as libc::c_long as libc::c_int);
    (gi.WriteByte).expect("non-null function pointer")(flashtype);
    (gi.multicast).expect("non-null function pointer")(start, MULTICAST_PVS);
}
#[no_mangle]
pub unsafe extern "C" fn monster_fire_rocket(
    mut self_0: *mut edict_t,
    mut start: *mut vec_t,
    mut dir: *mut vec_t,
    mut damage: libc::c_int,
    mut speed: libc::c_int,
    mut flashtype: libc::c_int,
) {
    fire_rocket(
        self_0,
        start,
        dir,
        damage,
        speed,
        (damage + 20 as libc::c_int) as libc::c_float,
        damage,
    );
    (gi.WriteByte).expect("non-null function pointer")(2 as libc::c_int);
    (gi.WriteShort)
        .expect(
            "non-null function pointer",
        )(self_0.offset_from(g_edicts) as libc::c_long as libc::c_int);
    (gi.WriteByte).expect("non-null function pointer")(flashtype);
    (gi.multicast).expect("non-null function pointer")(start, MULTICAST_PVS);
}
#[no_mangle]
pub unsafe extern "C" fn monster_fire_railgun(
    mut self_0: *mut edict_t,
    mut start: *mut vec_t,
    mut aimdir: *mut vec_t,
    mut damage: libc::c_int,
    mut kick: libc::c_int,
    mut flashtype: libc::c_int,
) {
    fire_rail(self_0, start, aimdir, damage, kick);
    (gi.WriteByte).expect("non-null function pointer")(2 as libc::c_int);
    (gi.WriteShort)
        .expect(
            "non-null function pointer",
        )(self_0.offset_from(g_edicts) as libc::c_long as libc::c_int);
    (gi.WriteByte).expect("non-null function pointer")(flashtype);
    (gi.multicast).expect("non-null function pointer")(start, MULTICAST_PVS);
}
#[no_mangle]
pub unsafe extern "C" fn monster_fire_bfg(
    mut self_0: *mut edict_t,
    mut start: *mut vec_t,
    mut aimdir: *mut vec_t,
    mut damage: libc::c_int,
    mut speed: libc::c_int,
    mut kick: libc::c_int,
    mut damage_radius: libc::c_float,
    mut flashtype: libc::c_int,
) {
    fire_bfg(self_0, start, aimdir, damage, speed, damage_radius);
    (gi.WriteByte).expect("non-null function pointer")(2 as libc::c_int);
    (gi.WriteShort)
        .expect(
            "non-null function pointer",
        )(self_0.offset_from(g_edicts) as libc::c_long as libc::c_int);
    (gi.WriteByte).expect("non-null function pointer")(flashtype);
    (gi.multicast).expect("non-null function pointer")(start, MULTICAST_PVS);
}
unsafe extern "C" fn M_FliesOff(mut self_0: *mut edict_t) {
    (*self_0).s.effects &= !(0x4000 as libc::c_int) as libc::c_uint;
    (*self_0).s.sound = 0 as libc::c_int;
}
unsafe extern "C" fn M_FliesOn(mut self_0: *mut edict_t) {
    if (*self_0).waterlevel != 0 {
        return;
    }
    (*self_0).s.effects |= 0x4000 as libc::c_int as libc::c_uint;
    (*self_0)
        .s
        .sound = (gi.soundindex)
        .expect(
            "non-null function pointer",
        )(
        b"infantry/inflies1.wav\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    let ref mut fresh0 = (*self_0).think;
    *fresh0 = Some(M_FliesOff as unsafe extern "C" fn(*mut edict_t) -> ());
    (*self_0).nextthink = level.time + 60 as libc::c_int as libc::c_float;
}
#[no_mangle]
pub unsafe extern "C" fn M_FlyCheck(mut self_0: *mut edict_t) {
    if (*self_0).waterlevel != 0 {
        return;
    }
    if ((rand() & 0x7fff as libc::c_int) as libc::c_float
        / 0x7fff as libc::c_int as libc::c_float) as libc::c_double > 0.5f64
    {
        return;
    }
    let ref mut fresh1 = (*self_0).think;
    *fresh1 = Some(M_FliesOn as unsafe extern "C" fn(*mut edict_t) -> ());
    (*self_0)
        .nextthink = level.time + 5 as libc::c_int as libc::c_float
        + 10 as libc::c_int as libc::c_float
            * ((rand() & 0x7fff as libc::c_int) as libc::c_float
                / 0x7fff as libc::c_int as libc::c_float);
}
#[no_mangle]
pub unsafe extern "C" fn AttackFinished(
    mut self_0: *mut edict_t,
    mut time: libc::c_float,
) {
    (*self_0).monsterinfo.attack_finished = level.time + time;
}
#[no_mangle]
pub unsafe extern "C" fn M_CheckGround(mut ent: *mut edict_t) {
    let mut point: vec3_t = [0.; 3];
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
    if (*ent).flags & (0x2 as libc::c_int | 0x1 as libc::c_int) != 0 {
        return;
    }
    if (*ent).velocity[2 as libc::c_int as usize] > 100 as libc::c_int as libc::c_float {
        let ref mut fresh2 = (*ent).groundentity;
        *fresh2 = 0 as *mut edict_t;
        return;
    }
    point[0 as libc::c_int as usize] = (*ent).s.origin[0 as libc::c_int as usize];
    point[1 as libc::c_int as usize] = (*ent).s.origin[1 as libc::c_int as usize];
    point[2 as libc::c_int
        as usize] = ((*ent).s.origin[2 as libc::c_int as usize] as libc::c_double
        - 0.25f64) as vec_t;
    trace = (gi.trace)
        .expect(
            "non-null function pointer",
        )(
        ((*ent).s.origin).as_mut_ptr(),
        ((*ent).mins).as_mut_ptr(),
        ((*ent).maxs).as_mut_ptr(),
        point.as_mut_ptr(),
        ent,
        1 as libc::c_int | 0x20000 as libc::c_int | 2 as libc::c_int
            | 0x2000000 as libc::c_int,
    );
    if (trace.plane.normal[2 as libc::c_int as usize] as libc::c_double) < 0.7f64
        && trace.startsolid as u64 == 0
    {
        let ref mut fresh3 = (*ent).groundentity;
        *fresh3 = 0 as *mut edict_t;
        return;
    }
    if trace.startsolid as u64 == 0 && trace.allsolid as u64 == 0 {
        (*ent)
            .s
            .origin[0 as libc::c_int as usize] = trace.endpos[0 as libc::c_int as usize];
        (*ent)
            .s
            .origin[1 as libc::c_int as usize] = trace.endpos[1 as libc::c_int as usize];
        (*ent)
            .s
            .origin[2 as libc::c_int as usize] = trace.endpos[2 as libc::c_int as usize];
        let ref mut fresh4 = (*ent).groundentity;
        *fresh4 = trace.ent;
        (*ent).groundentity_linkcount = (*trace.ent).linkcount;
        (*ent).velocity[2 as libc::c_int as usize] = 0 as libc::c_int as vec_t;
    }
}
#[no_mangle]
pub unsafe extern "C" fn M_CatagorizePosition(mut ent: *mut edict_t) {
    let mut point: vec3_t = [0.; 3];
    let mut cont: libc::c_int = 0;
    point[0 as libc::c_int as usize] = (*ent).s.origin[0 as libc::c_int as usize];
    point[1 as libc::c_int as usize] = (*ent).s.origin[1 as libc::c_int as usize];
    point[2 as libc::c_int
        as usize] = (*ent).s.origin[2 as libc::c_int as usize]
        + (*ent).mins[2 as libc::c_int as usize] + 1 as libc::c_int as libc::c_float;
    cont = (gi.pointcontents).expect("non-null function pointer")(point.as_mut_ptr());
    if cont & (32 as libc::c_int | 8 as libc::c_int | 16 as libc::c_int) == 0 {
        (*ent).waterlevel = 0 as libc::c_int;
        (*ent).watertype = 0 as libc::c_int;
        return;
    }
    (*ent).watertype = cont;
    (*ent).waterlevel = 1 as libc::c_int;
    point[2 as libc::c_int as usize] += 26 as libc::c_int as libc::c_float;
    cont = (gi.pointcontents).expect("non-null function pointer")(point.as_mut_ptr());
    if cont & (32 as libc::c_int | 8 as libc::c_int | 16 as libc::c_int) == 0 {
        return;
    }
    (*ent).waterlevel = 2 as libc::c_int;
    point[2 as libc::c_int as usize] += 22 as libc::c_int as libc::c_float;
    cont = (gi.pointcontents).expect("non-null function pointer")(point.as_mut_ptr());
    if cont & (32 as libc::c_int | 8 as libc::c_int | 16 as libc::c_int) != 0 {
        (*ent).waterlevel = 3 as libc::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn M_WorldEffects(mut ent: *mut edict_t) {
    let mut dmg: libc::c_int = 0;
    if (*ent).health > 0 as libc::c_int {
        if (*ent).flags & 0x2 as libc::c_int == 0 {
            if (*ent).waterlevel < 3 as libc::c_int {
                (*ent).air_finished = level.time + 12 as libc::c_int as libc::c_float;
            } else if (*ent).air_finished < level.time {
                if (*ent).pain_debounce_time < level.time {
                    dmg = (2 as libc::c_int as libc::c_double
                        + 2 as libc::c_int as libc::c_double
                            * floor(
                                (level.time - (*ent).air_finished) as libc::c_double,
                            )) as libc::c_int;
                    if dmg > 15 as libc::c_int {
                        dmg = 15 as libc::c_int;
                    }
                    T_Damage(
                        ent,
                        &mut *g_edicts.offset(0 as libc::c_int as isize),
                        &mut *g_edicts.offset(0 as libc::c_int as isize),
                        vec3_origin.as_mut_ptr(),
                        ((*ent).s.origin).as_mut_ptr(),
                        vec3_origin.as_mut_ptr(),
                        dmg,
                        0 as libc::c_int,
                        0x2 as libc::c_int,
                        17 as libc::c_int,
                    );
                    (*ent)
                        .pain_debounce_time = level.time
                        + 1 as libc::c_int as libc::c_float;
                }
            }
        } else if (*ent).waterlevel > 0 as libc::c_int {
            (*ent).air_finished = level.time + 9 as libc::c_int as libc::c_float;
        } else if (*ent).air_finished < level.time {
            if (*ent).pain_debounce_time < level.time {
                dmg = (2 as libc::c_int as libc::c_double
                    + 2 as libc::c_int as libc::c_double
                        * floor((level.time - (*ent).air_finished) as libc::c_double))
                    as libc::c_int;
                if dmg > 15 as libc::c_int {
                    dmg = 15 as libc::c_int;
                }
                T_Damage(
                    ent,
                    &mut *g_edicts.offset(0 as libc::c_int as isize),
                    &mut *g_edicts.offset(0 as libc::c_int as isize),
                    vec3_origin.as_mut_ptr(),
                    ((*ent).s.origin).as_mut_ptr(),
                    vec3_origin.as_mut_ptr(),
                    dmg,
                    0 as libc::c_int,
                    0x2 as libc::c_int,
                    17 as libc::c_int,
                );
                (*ent)
                    .pain_debounce_time = level.time + 1 as libc::c_int as libc::c_float;
            }
        }
    }
    if (*ent).waterlevel == 0 as libc::c_int {
        if (*ent).flags & 0x8 as libc::c_int != 0 {
            (gi.sound)
                .expect(
                    "non-null function pointer",
                )(
                ent,
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
            (*ent).flags &= !(0x8 as libc::c_int);
        }
        return;
    }
    if (*ent).watertype & 8 as libc::c_int != 0
        && (*ent).flags & 0x80 as libc::c_int == 0
    {
        if (*ent).damage_debounce_time < level.time {
            (*ent)
                .damage_debounce_time = (level.time as libc::c_double + 0.2f64)
                as libc::c_float;
            T_Damage(
                ent,
                &mut *g_edicts.offset(0 as libc::c_int as isize),
                &mut *g_edicts.offset(0 as libc::c_int as isize),
                vec3_origin.as_mut_ptr(),
                ((*ent).s.origin).as_mut_ptr(),
                vec3_origin.as_mut_ptr(),
                10 as libc::c_int * (*ent).waterlevel,
                0 as libc::c_int,
                0 as libc::c_int,
                19 as libc::c_int,
            );
        }
    }
    if (*ent).watertype & 16 as libc::c_int != 0
        && (*ent).flags & 0x40 as libc::c_int == 0
    {
        if (*ent).damage_debounce_time < level.time {
            (*ent).damage_debounce_time = level.time + 1 as libc::c_int as libc::c_float;
            T_Damage(
                ent,
                &mut *g_edicts.offset(0 as libc::c_int as isize),
                &mut *g_edicts.offset(0 as libc::c_int as isize),
                vec3_origin.as_mut_ptr(),
                ((*ent).s.origin).as_mut_ptr(),
                vec3_origin.as_mut_ptr(),
                4 as libc::c_int * (*ent).waterlevel,
                0 as libc::c_int,
                0 as libc::c_int,
                18 as libc::c_int,
            );
        }
    }
    if (*ent).flags & 0x8 as libc::c_int == 0 {
        if (*ent).svflags & 0x2 as libc::c_int == 0 {
            if (*ent).watertype & 8 as libc::c_int != 0 {
                if ((rand() & 0x7fff as libc::c_int) as libc::c_float
                    / 0x7fff as libc::c_int as libc::c_float) as libc::c_double <= 0.5f64
                {
                    (gi.sound)
                        .expect(
                            "non-null function pointer",
                        )(
                        ent,
                        4 as libc::c_int,
                        (gi.soundindex)
                            .expect(
                                "non-null function pointer",
                            )(
                            b"player/lava1.wav\0" as *const u8 as *const libc::c_char
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
                        ent,
                        4 as libc::c_int,
                        (gi.soundindex)
                            .expect(
                                "non-null function pointer",
                            )(
                            b"player/lava2.wav\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char,
                        ),
                        1 as libc::c_int as libc::c_float,
                        1 as libc::c_int as libc::c_float,
                        0 as libc::c_int as libc::c_float,
                    );
                }
            } else if (*ent).watertype & 16 as libc::c_int != 0 {
                (gi.sound)
                    .expect(
                        "non-null function pointer",
                    )(
                    ent,
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
            } else if (*ent).watertype & 32 as libc::c_int != 0 {
                (gi.sound)
                    .expect(
                        "non-null function pointer",
                    )(
                    ent,
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
        }
        (*ent).flags |= 0x8 as libc::c_int;
        (*ent).damage_debounce_time = 0 as libc::c_int as libc::c_float;
    }
}
#[no_mangle]
pub unsafe extern "C" fn M_droptofloor(mut ent: *mut edict_t) {
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
    let ref mut fresh5 = (*ent).s.origin[2 as libc::c_int as usize];
    *fresh5 += 1 as libc::c_int as libc::c_float;
    end[0 as libc::c_int as usize] = (*ent).s.origin[0 as libc::c_int as usize];
    end[1 as libc::c_int as usize] = (*ent).s.origin[1 as libc::c_int as usize];
    end[2 as libc::c_int as usize] = (*ent).s.origin[2 as libc::c_int as usize];
    end[2 as libc::c_int as usize] -= 256 as libc::c_int as libc::c_float;
    trace = (gi.trace)
        .expect(
            "non-null function pointer",
        )(
        ((*ent).s.origin).as_mut_ptr(),
        ((*ent).mins).as_mut_ptr(),
        ((*ent).maxs).as_mut_ptr(),
        end.as_mut_ptr(),
        ent,
        1 as libc::c_int | 0x20000 as libc::c_int | 2 as libc::c_int
            | 0x2000000 as libc::c_int,
    );
    if trace.fraction == 1 as libc::c_int as libc::c_float
        || trace.allsolid as libc::c_uint != 0
    {
        return;
    }
    (*ent).s.origin[0 as libc::c_int as usize] = trace.endpos[0 as libc::c_int as usize];
    (*ent).s.origin[1 as libc::c_int as usize] = trace.endpos[1 as libc::c_int as usize];
    (*ent).s.origin[2 as libc::c_int as usize] = trace.endpos[2 as libc::c_int as usize];
    (gi.linkentity).expect("non-null function pointer")(ent);
    M_CheckGround(ent);
    M_CatagorizePosition(ent);
}
#[no_mangle]
pub unsafe extern "C" fn M_SetEffects(mut ent: *mut edict_t) {
    (*ent).s.effects &= !(0x100 as libc::c_int | 0x200 as libc::c_int) as libc::c_uint;
    (*ent).s.renderfx
        &= !(1024 as libc::c_int | 2048 as libc::c_int | 4096 as libc::c_int);
    if (*ent).monsterinfo.aiflags & 0x4000 as libc::c_int != 0 {
        (*ent).s.effects |= 0x100 as libc::c_int as libc::c_uint;
        (*ent).s.renderfx |= 1024 as libc::c_int;
    }
    if (*ent).health <= 0 as libc::c_int {
        return;
    }
    if (*ent).powerarmor_time > level.time {
        if (*ent).monsterinfo.power_armor_type == 1 as libc::c_int {
            (*ent).s.effects |= 0x200 as libc::c_int as libc::c_uint;
        } else if (*ent).monsterinfo.power_armor_type == 2 as libc::c_int {
            (*ent).s.effects |= 0x100 as libc::c_int as libc::c_uint;
            (*ent).s.renderfx |= 2048 as libc::c_int;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn M_MoveFrame(mut self_0: *mut edict_t) {
    let mut move_0: *mut mmove_t = 0 as *mut mmove_t;
    let mut index: libc::c_int = 0;
    move_0 = (*self_0).monsterinfo.currentmove;
    (*self_0).nextthink = (level.time as libc::c_double + 0.1f64) as libc::c_float;
    if (*self_0).monsterinfo.nextframe != 0
        && (*self_0).monsterinfo.nextframe >= (*move_0).firstframe
        && (*self_0).monsterinfo.nextframe <= (*move_0).lastframe
    {
        (*self_0).s.frame = (*self_0).monsterinfo.nextframe;
        (*self_0).monsterinfo.nextframe = 0 as libc::c_int;
    } else {
        if (*self_0).s.frame == (*move_0).lastframe {
            if ((*move_0).endfunc).is_some() {
                ((*move_0).endfunc).expect("non-null function pointer")(self_0);
                move_0 = (*self_0).monsterinfo.currentmove;
                if (*self_0).svflags & 0x2 as libc::c_int != 0 {
                    return;
                }
            }
        }
        if (*self_0).s.frame < (*move_0).firstframe
            || (*self_0).s.frame > (*move_0).lastframe
        {
            (*self_0).monsterinfo.aiflags &= !(0x80 as libc::c_int);
            (*self_0).s.frame = (*move_0).firstframe;
        } else if (*self_0).monsterinfo.aiflags & 0x80 as libc::c_int == 0 {
            let ref mut fresh6 = (*self_0).s.frame;
            *fresh6 += 1;
            if (*self_0).s.frame > (*move_0).lastframe {
                (*self_0).s.frame = (*move_0).firstframe;
            }
        }
    }
    index = (*self_0).s.frame - (*move_0).firstframe;
    if ((*((*move_0).frame).offset(index as isize)).aifunc).is_some() {
        if (*self_0).monsterinfo.aiflags & 0x80 as libc::c_int == 0 {
            ((*((*move_0).frame).offset(index as isize)).aifunc)
                .expect(
                    "non-null function pointer",
                )(
                self_0,
                (*((*move_0).frame).offset(index as isize)).dist
                    * (*self_0).monsterinfo.scale,
            );
        } else {
            ((*((*move_0).frame).offset(index as isize)).aifunc)
                .expect(
                    "non-null function pointer",
                )(self_0, 0 as libc::c_int as libc::c_float);
        }
    }
    if ((*((*move_0).frame).offset(index as isize)).thinkfunc).is_some() {
        ((*((*move_0).frame).offset(index as isize)).thinkfunc)
            .expect("non-null function pointer")(self_0);
    }
}
#[no_mangle]
pub unsafe extern "C" fn monster_think(mut self_0: *mut edict_t) {
    M_MoveFrame(self_0);
    if (*self_0).linkcount != (*self_0).monsterinfo.linkcount {
        (*self_0).monsterinfo.linkcount = (*self_0).linkcount;
        M_CheckGround(self_0);
    }
    M_CatagorizePosition(self_0);
    M_WorldEffects(self_0);
    M_SetEffects(self_0);
}
#[no_mangle]
pub unsafe extern "C" fn monster_use(
    mut self_0: *mut edict_t,
    mut other: *mut edict_t,
    mut activator: *mut edict_t,
) {
    if !((*self_0).enemy).is_null() {
        return;
    }
    if (*self_0).health <= 0 as libc::c_int {
        return;
    }
    if (*activator).flags & 0x20 as libc::c_int != 0 {
        return;
    }
    if ((*activator).client).is_null()
        && (*activator).monsterinfo.aiflags & 0x100 as libc::c_int == 0
    {
        return;
    }
    let ref mut fresh7 = (*self_0).enemy;
    *fresh7 = activator;
    FoundTarget(self_0);
}
#[no_mangle]
pub unsafe extern "C" fn monster_triggered_spawn(mut self_0: *mut edict_t) {
    let ref mut fresh8 = (*self_0).s.origin[2 as libc::c_int as usize];
    *fresh8 += 1 as libc::c_int as libc::c_float;
    KillBox(self_0);
    (*self_0).solid = SOLID_BBOX;
    (*self_0).movetype = MOVETYPE_STEP as libc::c_int;
    (*self_0).svflags &= !(0x1 as libc::c_int);
    (*self_0).air_finished = level.time + 12 as libc::c_int as libc::c_float;
    (gi.linkentity).expect("non-null function pointer")(self_0);
    monster_start_go(self_0);
    if !((*self_0).enemy).is_null() && (*self_0).spawnflags & 1 as libc::c_int == 0
        && (*(*self_0).enemy).flags & 0x20 as libc::c_int == 0
    {
        FoundTarget(self_0);
    } else {
        let ref mut fresh9 = (*self_0).enemy;
        *fresh9 = 0 as *mut edict_t;
    };
}
#[no_mangle]
pub unsafe extern "C" fn monster_triggered_spawn_use(
    mut self_0: *mut edict_t,
    mut other: *mut edict_t,
    mut activator: *mut edict_t,
) {
    let ref mut fresh10 = (*self_0).think;
    *fresh10 = Some(monster_triggered_spawn as unsafe extern "C" fn(*mut edict_t) -> ());
    (*self_0).nextthink = (level.time as libc::c_double + 0.1f64) as libc::c_float;
    if !((*activator).client).is_null() {
        let ref mut fresh11 = (*self_0).enemy;
        *fresh11 = activator;
    }
    let ref mut fresh12 = (*self_0).use_0;
    *fresh12 = Some(
        monster_use
            as unsafe extern "C" fn(*mut edict_t, *mut edict_t, *mut edict_t) -> (),
    );
}
#[no_mangle]
pub unsafe extern "C" fn monster_triggered_start(mut self_0: *mut edict_t) {
    (*self_0).solid = SOLID_NOT;
    (*self_0).movetype = MOVETYPE_NONE as libc::c_int;
    (*self_0).svflags |= 0x1 as libc::c_int;
    (*self_0).nextthink = 0 as libc::c_int as libc::c_float;
    let ref mut fresh13 = (*self_0).use_0;
    *fresh13 = Some(
        monster_triggered_spawn_use
            as unsafe extern "C" fn(*mut edict_t, *mut edict_t, *mut edict_t) -> (),
    );
}
#[no_mangle]
pub unsafe extern "C" fn monster_death_use(mut self_0: *mut edict_t) {
    (*self_0).flags &= !(0x1 as libc::c_int | 0x2 as libc::c_int);
    (*self_0).monsterinfo.aiflags &= 0x100 as libc::c_int;
    if !((*self_0).item).is_null() {
        Drop_Item(self_0, (*self_0).item);
        let ref mut fresh14 = (*self_0).item;
        *fresh14 = 0 as *mut gitem_t;
    }
    if !((*self_0).deathtarget).is_null() {
        let ref mut fresh15 = (*self_0).target;
        *fresh15 = (*self_0).deathtarget;
    }
    if ((*self_0).target).is_null() {
        return;
    }
    G_UseTargets(self_0, (*self_0).enemy);
}
#[no_mangle]
pub unsafe extern "C" fn monster_start(mut self_0: *mut edict_t) -> qboolean {
    if (*deathmatch).value != 0. {
        G_FreeEdict(self_0);
        return false_0;
    }
    if (*self_0).spawnflags & 4 as libc::c_int != 0
        && (*self_0).monsterinfo.aiflags & 0x100 as libc::c_int == 0
    {
        (*self_0).spawnflags &= !(4 as libc::c_int);
        (*self_0).spawnflags |= 1 as libc::c_int;
    }
    if (*self_0).monsterinfo.aiflags & 0x100 as libc::c_int == 0 {
        level.total_monsters += 1;
    }
    (*self_0).nextthink = (level.time as libc::c_double + 0.1f64) as libc::c_float;
    (*self_0).svflags |= 0x4 as libc::c_int;
    (*self_0).s.renderfx |= 64 as libc::c_int;
    (*self_0).takedamage = DAMAGE_AIM as libc::c_int;
    (*self_0).air_finished = level.time + 12 as libc::c_int as libc::c_float;
    let ref mut fresh16 = (*self_0).use_0;
    *fresh16 = Some(
        monster_use
            as unsafe extern "C" fn(*mut edict_t, *mut edict_t, *mut edict_t) -> (),
    );
    (*self_0).max_health = (*self_0).health;
    (*self_0)
        .clipmask = 1 as libc::c_int | 0x20000 as libc::c_int | 2 as libc::c_int
        | 0x2000000 as libc::c_int;
    (*self_0).s.skinnum = 0 as libc::c_int;
    (*self_0).deadflag = 0 as libc::c_int;
    (*self_0).svflags &= !(0x2 as libc::c_int);
    if ((*self_0).monsterinfo.checkattack).is_none() {
        let ref mut fresh17 = (*self_0).monsterinfo.checkattack;
        *fresh17 = Some(M_CheckAttack as unsafe extern "C" fn(*mut edict_t) -> qboolean);
    }
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
    if !(st.item).is_null() {
        let ref mut fresh18 = (*self_0).item;
        *fresh18 = FindItemByClassname(st.item);
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
    if !((*self_0).monsterinfo.currentmove).is_null() {
        (*self_0)
            .s
            .frame = (*(*self_0).monsterinfo.currentmove).firstframe
            + rand()
                % ((*(*self_0).monsterinfo.currentmove).lastframe
                    - (*(*self_0).monsterinfo.currentmove).firstframe
                    + 1 as libc::c_int);
    }
    return true_0;
}
#[no_mangle]
pub unsafe extern "C" fn monster_start_go(mut self_0: *mut edict_t) {
    let mut v: vec3_t = [0.; 3];
    if (*self_0).health <= 0 as libc::c_int {
        return;
    }
    if !((*self_0).target).is_null() {
        let mut notcombat: qboolean = false_0;
        let mut fixup: qboolean = false_0;
        let mut target: *mut edict_t = 0 as *mut edict_t;
        target = 0 as *mut edict_t;
        notcombat = false_0;
        fixup = false_0;
        loop {
            target = G_Find(
                target,
                &mut (*(0 as *mut edict_t)).targetname as *mut *mut libc::c_char
                    as libc::c_int,
                (*self_0).target,
            );
            if target.is_null() {
                break;
            }
            if strcmp(
                (*target).classname,
                b"point_combat\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
            {
                let ref mut fresh19 = (*self_0).combattarget;
                *fresh19 = (*self_0).target;
                fixup = true_0;
            } else {
                notcombat = true_0;
            }
        }
        if notcombat as libc::c_uint != 0 && !((*self_0).combattarget).is_null() {
            (gi.dprintf)
                .expect(
                    "non-null function pointer",
                )(
                b"%s at %s has target with mixed types\n\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                (*self_0).classname,
                vtos(((*self_0).s.origin).as_mut_ptr()),
            );
        }
        if fixup as u64 != 0 {
            let ref mut fresh20 = (*self_0).target;
            *fresh20 = 0 as *mut libc::c_char;
        }
    }
    if !((*self_0).combattarget).is_null() {
        let mut target_0: *mut edict_t = 0 as *mut edict_t;
        target_0 = 0 as *mut edict_t;
        loop {
            target_0 = G_Find(
                target_0,
                &mut (*(0 as *mut edict_t)).targetname as *mut *mut libc::c_char
                    as libc::c_int,
                (*self_0).combattarget,
            );
            if target_0.is_null() {
                break;
            }
            if strcmp(
                (*target_0).classname,
                b"point_combat\0" as *const u8 as *const libc::c_char,
            ) != 0 as libc::c_int
            {
                (gi.dprintf)
                    .expect(
                        "non-null function pointer",
                    )(
                    b"%s at (%i %i %i) has a bad combattarget %s : %s at (%i %i %i)\n\0"
                        as *const u8 as *const libc::c_char as *mut libc::c_char,
                    (*self_0).classname,
                    (*self_0).s.origin[0 as libc::c_int as usize] as libc::c_int,
                    (*self_0).s.origin[1 as libc::c_int as usize] as libc::c_int,
                    (*self_0).s.origin[2 as libc::c_int as usize] as libc::c_int,
                    (*self_0).combattarget,
                    (*target_0).classname,
                    (*target_0).s.origin[0 as libc::c_int as usize] as libc::c_int,
                    (*target_0).s.origin[1 as libc::c_int as usize] as libc::c_int,
                    (*target_0).s.origin[2 as libc::c_int as usize] as libc::c_int,
                );
            }
        }
    }
    if !((*self_0).target).is_null() {
        let ref mut fresh21 = (*self_0).movetarget;
        *fresh21 = G_PickTarget((*self_0).target);
        let ref mut fresh22 = (*self_0).goalentity;
        *fresh22 = *fresh21;
        if ((*self_0).movetarget).is_null() {
            (gi.dprintf)
                .expect(
                    "non-null function pointer",
                )(
                b"%s can't find target %s at %s\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                (*self_0).classname,
                (*self_0).target,
                vtos(((*self_0).s.origin).as_mut_ptr()),
            );
            let ref mut fresh23 = (*self_0).target;
            *fresh23 = 0 as *mut libc::c_char;
            (*self_0).monsterinfo.pausetime = 100000000 as libc::c_int as libc::c_float;
            ((*self_0).monsterinfo.stand).expect("non-null function pointer")(self_0);
        } else if strcmp(
            (*(*self_0).movetarget).classname,
            b"path_corner\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            v[0 as libc::c_int
                as usize] = (*(*self_0).goalentity).s.origin[0 as libc::c_int as usize]
                - (*self_0).s.origin[0 as libc::c_int as usize];
            v[1 as libc::c_int
                as usize] = (*(*self_0).goalentity).s.origin[1 as libc::c_int as usize]
                - (*self_0).s.origin[1 as libc::c_int as usize];
            v[2 as libc::c_int
                as usize] = (*(*self_0).goalentity).s.origin[2 as libc::c_int as usize]
                - (*self_0).s.origin[2 as libc::c_int as usize];
            let ref mut fresh24 = (*self_0).s.angles[1 as libc::c_int as usize];
            *fresh24 = vectoyaw(v.as_mut_ptr());
            (*self_0).ideal_yaw = *fresh24;
            ((*self_0).monsterinfo.walk).expect("non-null function pointer")(self_0);
            let ref mut fresh25 = (*self_0).target;
            *fresh25 = 0 as *mut libc::c_char;
        } else {
            let ref mut fresh26 = (*self_0).movetarget;
            *fresh26 = 0 as *mut edict_t;
            let ref mut fresh27 = (*self_0).goalentity;
            *fresh27 = *fresh26;
            (*self_0).monsterinfo.pausetime = 100000000 as libc::c_int as libc::c_float;
            ((*self_0).monsterinfo.stand).expect("non-null function pointer")(self_0);
        }
    } else {
        (*self_0).monsterinfo.pausetime = 100000000 as libc::c_int as libc::c_float;
        ((*self_0).monsterinfo.stand).expect("non-null function pointer")(self_0);
    }
    let ref mut fresh28 = (*self_0).think;
    *fresh28 = Some(monster_think as unsafe extern "C" fn(*mut edict_t) -> ());
    (*self_0).nextthink = (level.time as libc::c_double + 0.1f64) as libc::c_float;
}
#[no_mangle]
pub unsafe extern "C" fn walkmonster_start_go(mut self_0: *mut edict_t) {
    if (*self_0).spawnflags & 2 as libc::c_int == 0
        && level.time < 1 as libc::c_int as libc::c_float
    {
        M_droptofloor(self_0);
        if !((*self_0).groundentity).is_null() {
            if M_walkmove(
                self_0,
                0 as libc::c_int as libc::c_float,
                0 as libc::c_int as libc::c_float,
            ) as u64 == 0
            {
                (gi.dprintf)
                    .expect(
                        "non-null function pointer",
                    )(
                    b"%s in solid at %s\n\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    (*self_0).classname,
                    vtos(((*self_0).s.origin).as_mut_ptr()),
                );
            }
        }
    }
    if (*self_0).yaw_speed == 0. {
        (*self_0).yaw_speed = 20 as libc::c_int as libc::c_float;
    }
    (*self_0).viewheight = 25 as libc::c_int;
    monster_start_go(self_0);
    if (*self_0).spawnflags & 2 as libc::c_int != 0 {
        monster_triggered_start(self_0);
    }
}
#[no_mangle]
pub unsafe extern "C" fn walkmonster_start(mut self_0: *mut edict_t) {
    let ref mut fresh29 = (*self_0).think;
    *fresh29 = Some(walkmonster_start_go as unsafe extern "C" fn(*mut edict_t) -> ());
    monster_start(self_0);
}
#[no_mangle]
pub unsafe extern "C" fn flymonster_start_go(mut self_0: *mut edict_t) {
    if M_walkmove(
        self_0,
        0 as libc::c_int as libc::c_float,
        0 as libc::c_int as libc::c_float,
    ) as u64 == 0
    {
        (gi.dprintf)
            .expect(
                "non-null function pointer",
            )(
            b"%s in solid at %s\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            (*self_0).classname,
            vtos(((*self_0).s.origin).as_mut_ptr()),
        );
    }
    if (*self_0).yaw_speed == 0. {
        (*self_0).yaw_speed = 10 as libc::c_int as libc::c_float;
    }
    (*self_0).viewheight = 25 as libc::c_int;
    monster_start_go(self_0);
    if (*self_0).spawnflags & 2 as libc::c_int != 0 {
        monster_triggered_start(self_0);
    }
}
#[no_mangle]
pub unsafe extern "C" fn flymonster_start(mut self_0: *mut edict_t) {
    (*self_0).flags |= 0x1 as libc::c_int;
    let ref mut fresh30 = (*self_0).think;
    *fresh30 = Some(flymonster_start_go as unsafe extern "C" fn(*mut edict_t) -> ());
    monster_start(self_0);
}
#[no_mangle]
pub unsafe extern "C" fn swimmonster_start_go(mut self_0: *mut edict_t) {
    if (*self_0).yaw_speed == 0. {
        (*self_0).yaw_speed = 10 as libc::c_int as libc::c_float;
    }
    (*self_0).viewheight = 10 as libc::c_int;
    monster_start_go(self_0);
    if (*self_0).spawnflags & 2 as libc::c_int != 0 {
        monster_triggered_start(self_0);
    }
}
#[no_mangle]
pub unsafe extern "C" fn swimmonster_start(mut self_0: *mut edict_t) {
    (*self_0).flags |= 0x2 as libc::c_int;
    let ref mut fresh31 = (*self_0).think;
    *fresh31 = Some(swimmonster_start_go as unsafe extern "C" fn(*mut edict_t) -> ());
    monster_start(self_0);
}
