#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    fn sqrt(_: libc::c_double) -> libc::c_double;
    static mut ctf: *mut cvar_t;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn rand() -> libc::c_int;
    static mut vec3_origin: vec3_t;
    fn VectorMA(
        veca: *mut vec_t,
        scale: libc::c_float,
        vecb: *mut vec_t,
        vecc: *mut vec_t,
    );
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
    static mut level: level_locals_t;
    static mut gi: game_import_t;
    static mut deathmatch: *mut cvar_t;
    static mut coop: *mut cvar_t;
    static mut skill: *mut cvar_t;
    fn findradius(
        from: *mut edict_t,
        org: *mut vec_t,
        rad: libc::c_float,
    ) -> *mut edict_t;
    fn G_Spawn() -> *mut edict_t;
    fn G_FreeEdict(e: *mut edict_t);
    fn vectoangles(vec: *mut vec_t, angles: *mut vec_t);
    fn CanDamage(targ: *mut edict_t, inflictor: *mut edict_t) -> qboolean;
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
    fn T_RadiusDamage(
        inflictor: *mut edict_t,
        attacker: *mut edict_t,
        damage: libc::c_float,
        ignore: *mut edict_t,
        radius: libc::c_float,
        mod_0: libc::c_int,
    );
    fn infront(self_0: *mut edict_t, other: *mut edict_t) -> qboolean;
    fn ThrowDebris(
        self_0: *mut edict_t,
        modelname: *mut libc::c_char,
        speed: libc::c_float,
        origin: *mut vec_t,
    );
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
pub const TE_FLECHETTE: C2RustUnnamed = 55;
pub const TE_EXPLOSION1_NP: C2RustUnnamed = 54;
pub const TE_EXPLOSION1_BIG: C2RustUnnamed = 53;
pub const TE_WIDOWSPLASH: C2RustUnnamed = 52;
pub const TE_NUKEBLAST: C2RustUnnamed = 51;
pub const TE_WIDOWBEAMOUT: C2RustUnnamed = 50;
pub const TE_DBALL_GOAL: C2RustUnnamed = 49;
pub const TE_TELEPORT_EFFECT: C2RustUnnamed = 48;
pub const TE_TRACKER_EXPLOSION: C2RustUnnamed = 47;
pub const TE_ELECTRIC_SPARKS: C2RustUnnamed = 46;
pub const TE_CHAINFIST_SMOKE: C2RustUnnamed = 45;
pub const TE_HEATBEAM_STEAM: C2RustUnnamed = 44;
pub const TE_HEATBEAM_SPARKS: C2RustUnnamed = 43;
pub const TE_MOREBLOOD: C2RustUnnamed = 42;
pub const TE_BUBBLETRAIL2: C2RustUnnamed = 41;
pub const TE_STEAM: C2RustUnnamed = 40;
pub const TE_MONSTER_HEATBEAM: C2RustUnnamed = 39;
pub const TE_HEATBEAM: C2RustUnnamed = 38;
pub const TE_FORCEWALL: C2RustUnnamed = 37;
pub const TE_FLASHLIGHT: C2RustUnnamed = 36;
pub const TE_PLAIN_EXPLOSION: C2RustUnnamed = 35;
pub const TE_DEBUGTRAIL: C2RustUnnamed = 34;
pub const TE_LIGHTNING: C2RustUnnamed = 33;
pub const TE_FLAME: C2RustUnnamed = 32;
pub const TE_RAILTRAIL2: C2RustUnnamed = 31;
pub const TE_BLASTER2: C2RustUnnamed = 30;
pub const TE_TUNNEL_SPARKS: C2RustUnnamed = 29;
pub const TE_PLASMA_EXPLOSION: C2RustUnnamed = 28;
pub const TE_BLUEHYPERBLASTER: C2RustUnnamed = 27;
pub const TE_GREENBLOOD: C2RustUnnamed = 26;
pub const TE_WELDING_SPARKS: C2RustUnnamed = 25;
pub const TE_GRAPPLE_CABLE: C2RustUnnamed = 24;
pub const TE_BFG_LASER: C2RustUnnamed = 23;
pub const TE_BOSSTPORT: C2RustUnnamed = 22;
pub const TE_BFG_BIGEXPLOSION: C2RustUnnamed = 21;
pub const TE_BFG_EXPLOSION: C2RustUnnamed = 20;
pub const TE_MEDIC_CABLE_ATTACK: C2RustUnnamed = 19;
pub const TE_GRENADE_EXPLOSION_WATER: C2RustUnnamed = 18;
pub const TE_ROCKET_EXPLOSION_WATER: C2RustUnnamed = 17;
pub const TE_PARASITE_ATTACK: C2RustUnnamed = 16;
pub const TE_LASER_SPARKS: C2RustUnnamed = 15;
pub const TE_BULLET_SPARKS: C2RustUnnamed = 14;
pub const TE_SHIELD_SPARKS: C2RustUnnamed = 13;
pub const TE_SCREEN_SPARKS: C2RustUnnamed = 12;
pub const TE_BUBBLETRAIL: C2RustUnnamed = 11;
pub const TE_SPLASH: C2RustUnnamed = 10;
pub const TE_SPARKS: C2RustUnnamed = 9;
pub const TE_GRENADE_EXPLOSION: C2RustUnnamed = 8;
pub const TE_ROCKET_EXPLOSION: C2RustUnnamed = 7;
pub const TE_EXPLOSION2: C2RustUnnamed = 6;
pub const TE_EXPLOSION1: C2RustUnnamed = 5;
pub const TE_SHOTGUN: C2RustUnnamed = 4;
pub const TE_RAILTRAIL: C2RustUnnamed = 3;
pub const TE_BLASTER: C2RustUnnamed = 2;
pub const TE_BLOOD: C2RustUnnamed = 1;
pub const TE_GUNSHOT: C2RustUnnamed = 0;
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
unsafe extern "C" fn check_dodge(
    mut self_0: *mut edict_t,
    mut start: *mut vec_t,
    mut dir: *mut vec_t,
    mut speed: libc::c_int,
) {
    let mut end: vec3_t = [0.; 3];
    let mut v: vec3_t = [0.; 3];
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
    let mut eta: libc::c_float = 0.;
    if (*skill).value == 0 as libc::c_int as libc::c_float {
        if ((rand() & 0x7fff as libc::c_int) as libc::c_float
            / 0x7fff as libc::c_int as libc::c_float) as libc::c_double > 0.25f64
        {
            return;
        }
    }
    VectorMA(start, 8192 as libc::c_int as libc::c_float, dir, end.as_mut_ptr());
    tr = (gi.trace)
        .expect(
            "non-null function pointer",
        )(
        start,
        0 as *mut vec_t,
        0 as *mut vec_t,
        end.as_mut_ptr(),
        self_0,
        1 as libc::c_int | 0x2000000 as libc::c_int | 2 as libc::c_int
            | 0x4000000 as libc::c_int,
    );
    if !(tr.ent).is_null() && (*tr.ent).svflags & 0x4 as libc::c_int != 0
        && (*tr.ent).health > 0 as libc::c_int && ((*tr.ent).monsterinfo.dodge).is_some()
        && infront(tr.ent, self_0) as libc::c_uint != 0
    {
        v[0 as libc::c_int
            as usize] = tr.endpos[0 as libc::c_int as usize]
            - *start.offset(0 as libc::c_int as isize);
        v[1 as libc::c_int
            as usize] = tr.endpos[1 as libc::c_int as usize]
            - *start.offset(1 as libc::c_int as isize);
        v[2 as libc::c_int
            as usize] = tr.endpos[2 as libc::c_int as usize]
            - *start.offset(2 as libc::c_int as isize);
        eta = (VectorLength(v.as_mut_ptr()) - (*tr.ent).maxs[0 as libc::c_int as usize])
            / speed as libc::c_float;
        ((*tr.ent).monsterinfo.dodge)
            .expect("non-null function pointer")(tr.ent, self_0, eta);
    }
}
#[no_mangle]
pub unsafe extern "C" fn fire_hit(
    mut self_0: *mut edict_t,
    mut aim: *mut vec_t,
    mut damage: libc::c_int,
    mut kick: libc::c_int,
) -> qboolean {
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
    let mut forward: vec3_t = [0.; 3];
    let mut right: vec3_t = [0.; 3];
    let mut up: vec3_t = [0.; 3];
    let mut v: vec3_t = [0.; 3];
    let mut point: vec3_t = [0.; 3];
    let mut range: libc::c_float = 0.;
    let mut dir: vec3_t = [0.; 3];
    dir[0 as libc::c_int
        as usize] = (*(*self_0).enemy).s.origin[0 as libc::c_int as usize]
        - (*self_0).s.origin[0 as libc::c_int as usize];
    dir[1 as libc::c_int
        as usize] = (*(*self_0).enemy).s.origin[1 as libc::c_int as usize]
        - (*self_0).s.origin[1 as libc::c_int as usize];
    dir[2 as libc::c_int
        as usize] = (*(*self_0).enemy).s.origin[2 as libc::c_int as usize]
        - (*self_0).s.origin[2 as libc::c_int as usize];
    range = VectorLength(dir.as_mut_ptr());
    if range > *aim.offset(0 as libc::c_int as isize) {
        return false_0;
    }
    if *aim.offset(1 as libc::c_int as isize) > (*self_0).mins[0 as libc::c_int as usize]
        && *aim.offset(1 as libc::c_int as isize)
            < (*self_0).maxs[0 as libc::c_int as usize]
    {
        range -= (*(*self_0).enemy).maxs[0 as libc::c_int as usize];
    } else if *aim.offset(1 as libc::c_int as isize) < 0 as libc::c_int as libc::c_float
    {
        *aim
            .offset(
                1 as libc::c_int as isize,
            ) = (*(*self_0).enemy).mins[0 as libc::c_int as usize];
    } else {
        *aim
            .offset(
                1 as libc::c_int as isize,
            ) = (*(*self_0).enemy).maxs[0 as libc::c_int as usize];
    }
    VectorMA(
        ((*self_0).s.origin).as_mut_ptr(),
        range,
        dir.as_mut_ptr(),
        point.as_mut_ptr(),
    );
    tr = (gi.trace)
        .expect(
            "non-null function pointer",
        )(
        ((*self_0).s.origin).as_mut_ptr(),
        0 as *mut vec_t,
        0 as *mut vec_t,
        point.as_mut_ptr(),
        self_0,
        1 as libc::c_int | 0x2000000 as libc::c_int | 2 as libc::c_int
            | 0x4000000 as libc::c_int,
    );
    if tr.fraction < 1 as libc::c_int as libc::c_float {
        if (*tr.ent).takedamage == 0 {
            return false_0;
        }
        if (*tr.ent).svflags & 0x4 as libc::c_int != 0 || !((*tr.ent).client).is_null() {
            tr.ent = (*self_0).enemy;
        }
    }
    AngleVectors(
        ((*self_0).s.angles).as_mut_ptr(),
        forward.as_mut_ptr(),
        right.as_mut_ptr(),
        up.as_mut_ptr(),
    );
    VectorMA(
        ((*self_0).s.origin).as_mut_ptr(),
        range,
        forward.as_mut_ptr(),
        point.as_mut_ptr(),
    );
    VectorMA(
        point.as_mut_ptr(),
        *aim.offset(1 as libc::c_int as isize),
        right.as_mut_ptr(),
        point.as_mut_ptr(),
    );
    VectorMA(
        point.as_mut_ptr(),
        *aim.offset(2 as libc::c_int as isize),
        up.as_mut_ptr(),
        point.as_mut_ptr(),
    );
    dir[0 as libc::c_int
        as usize] = point[0 as libc::c_int as usize]
        - (*(*self_0).enemy).s.origin[0 as libc::c_int as usize];
    dir[1 as libc::c_int
        as usize] = point[1 as libc::c_int as usize]
        - (*(*self_0).enemy).s.origin[1 as libc::c_int as usize];
    dir[2 as libc::c_int
        as usize] = point[2 as libc::c_int as usize]
        - (*(*self_0).enemy).s.origin[2 as libc::c_int as usize];
    T_Damage(
        tr.ent,
        self_0,
        self_0,
        dir.as_mut_ptr(),
        point.as_mut_ptr(),
        vec3_origin.as_mut_ptr(),
        damage,
        kick / 2 as libc::c_int,
        0x8 as libc::c_int,
        32 as libc::c_int,
    );
    if (*tr.ent).svflags & 0x4 as libc::c_int == 0 && ((*tr.ent).client).is_null() {
        return false_0;
    }
    VectorMA(
        ((*(*self_0).enemy).absmin).as_mut_ptr(),
        0.5f64 as libc::c_float,
        ((*(*self_0).enemy).size).as_mut_ptr(),
        v.as_mut_ptr(),
    );
    v[0 as libc::c_int
        as usize] = v[0 as libc::c_int as usize] - point[0 as libc::c_int as usize];
    v[1 as libc::c_int
        as usize] = v[1 as libc::c_int as usize] - point[1 as libc::c_int as usize];
    v[2 as libc::c_int
        as usize] = v[2 as libc::c_int as usize] - point[2 as libc::c_int as usize];
    VectorNormalize(v.as_mut_ptr());
    VectorMA(
        ((*(*self_0).enemy).velocity).as_mut_ptr(),
        kick as libc::c_float,
        v.as_mut_ptr(),
        ((*(*self_0).enemy).velocity).as_mut_ptr(),
    );
    if (*(*self_0).enemy).velocity[2 as libc::c_int as usize]
        > 0 as libc::c_int as libc::c_float
    {
        let ref mut fresh0 = (*(*self_0).enemy).groundentity;
        *fresh0 = 0 as *mut edict_t;
    }
    return true_0;
}
unsafe extern "C" fn fire_lead(
    mut self_0: *mut edict_t,
    mut start: *mut vec_t,
    mut aimdir: *mut vec_t,
    mut damage: libc::c_int,
    mut kick: libc::c_int,
    mut te_impact: libc::c_int,
    mut hspread: libc::c_int,
    mut vspread: libc::c_int,
    mut mod_0: libc::c_int,
) {
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
    let mut dir: vec3_t = [0.; 3];
    let mut forward: vec3_t = [0.; 3];
    let mut right: vec3_t = [0.; 3];
    let mut up: vec3_t = [0.; 3];
    let mut end: vec3_t = [0.; 3];
    let mut r: libc::c_float = 0.;
    let mut u: libc::c_float = 0.;
    let mut water_start: vec3_t = [0.; 3];
    let mut water: qboolean = false_0;
    let mut content_mask: libc::c_int = 1 as libc::c_int | 0x2000000 as libc::c_int
        | 2 as libc::c_int | 0x4000000 as libc::c_int
        | (32 as libc::c_int | 8 as libc::c_int | 16 as libc::c_int);
    tr = (gi.trace)
        .expect(
            "non-null function pointer",
        )(
        ((*self_0).s.origin).as_mut_ptr(),
        0 as *mut vec_t,
        0 as *mut vec_t,
        start,
        self_0,
        1 as libc::c_int | 0x2000000 as libc::c_int | 2 as libc::c_int
            | 0x4000000 as libc::c_int,
    );
    if !((tr.fraction as libc::c_double) < 1.0f64) {
        vectoangles(aimdir, dir.as_mut_ptr());
        AngleVectors(
            dir.as_mut_ptr(),
            forward.as_mut_ptr(),
            right.as_mut_ptr(),
            up.as_mut_ptr(),
        );
        r = (2.0f64
            * (((rand() & 0x7fff as libc::c_int) as libc::c_float
                / 0x7fff as libc::c_int as libc::c_float) as libc::c_double - 0.5f64)
            * hspread as libc::c_double) as libc::c_float;
        u = (2.0f64
            * (((rand() & 0x7fff as libc::c_int) as libc::c_float
                / 0x7fff as libc::c_int as libc::c_float) as libc::c_double - 0.5f64)
            * vspread as libc::c_double) as libc::c_float;
        VectorMA(
            start,
            8192 as libc::c_int as libc::c_float,
            forward.as_mut_ptr(),
            end.as_mut_ptr(),
        );
        VectorMA(end.as_mut_ptr(), r, right.as_mut_ptr(), end.as_mut_ptr());
        VectorMA(end.as_mut_ptr(), u, up.as_mut_ptr(), end.as_mut_ptr());
        if (gi.pointcontents).expect("non-null function pointer")(start)
            & (32 as libc::c_int | 8 as libc::c_int | 16 as libc::c_int) != 0
        {
            water = true_0;
            water_start[0 as libc::c_int
                as usize] = *start.offset(0 as libc::c_int as isize);
            water_start[1 as libc::c_int
                as usize] = *start.offset(1 as libc::c_int as isize);
            water_start[2 as libc::c_int
                as usize] = *start.offset(2 as libc::c_int as isize);
            content_mask &= !(32 as libc::c_int | 8 as libc::c_int | 16 as libc::c_int);
        }
        tr = (gi.trace)
            .expect(
                "non-null function pointer",
            )(
            start,
            0 as *mut vec_t,
            0 as *mut vec_t,
            end.as_mut_ptr(),
            self_0,
            content_mask,
        );
        if tr.contents & (32 as libc::c_int | 8 as libc::c_int | 16 as libc::c_int) != 0
        {
            let mut color: libc::c_int = 0;
            water = true_0;
            water_start[0 as libc::c_int
                as usize] = tr.endpos[0 as libc::c_int as usize];
            water_start[1 as libc::c_int
                as usize] = tr.endpos[1 as libc::c_int as usize];
            water_start[2 as libc::c_int
                as usize] = tr.endpos[2 as libc::c_int as usize];
            if VectorCompare(start, (tr.endpos).as_mut_ptr()) == 0 {
                if tr.contents & 32 as libc::c_int != 0 {
                    if strcmp(
                        ((*tr.surface).name).as_mut_ptr(),
                        b"*brwater\0" as *const u8 as *const libc::c_char,
                    ) == 0 as libc::c_int
                    {
                        color = 3 as libc::c_int;
                    } else {
                        color = 2 as libc::c_int;
                    }
                } else if tr.contents & 16 as libc::c_int != 0 {
                    color = 4 as libc::c_int;
                } else if tr.contents & 8 as libc::c_int != 0 {
                    color = 5 as libc::c_int;
                } else {
                    color = 0 as libc::c_int;
                }
                if color != 0 as libc::c_int {
                    (gi.WriteByte).expect("non-null function pointer")(3 as libc::c_int);
                    (gi.WriteByte)
                        .expect("non-null function pointer")(TE_SPLASH as libc::c_int);
                    (gi.WriteByte).expect("non-null function pointer")(8 as libc::c_int);
                    (gi.WritePosition)
                        .expect("non-null function pointer")((tr.endpos).as_mut_ptr());
                    (gi.WriteDir)
                        .expect(
                            "non-null function pointer",
                        )((tr.plane.normal).as_mut_ptr());
                    (gi.WriteByte).expect("non-null function pointer")(color);
                    (gi.multicast)
                        .expect(
                            "non-null function pointer",
                        )((tr.endpos).as_mut_ptr(), MULTICAST_PVS);
                }
                dir[0 as libc::c_int
                    as usize] = end[0 as libc::c_int as usize]
                    - *start.offset(0 as libc::c_int as isize);
                dir[1 as libc::c_int
                    as usize] = end[1 as libc::c_int as usize]
                    - *start.offset(1 as libc::c_int as isize);
                dir[2 as libc::c_int
                    as usize] = end[2 as libc::c_int as usize]
                    - *start.offset(2 as libc::c_int as isize);
                vectoangles(dir.as_mut_ptr(), dir.as_mut_ptr());
                AngleVectors(
                    dir.as_mut_ptr(),
                    forward.as_mut_ptr(),
                    right.as_mut_ptr(),
                    up.as_mut_ptr(),
                );
                r = (2.0f64
                    * (((rand() & 0x7fff as libc::c_int) as libc::c_float
                        / 0x7fff as libc::c_int as libc::c_float) as libc::c_double
                        - 0.5f64) * hspread as libc::c_double
                    * 2 as libc::c_int as libc::c_double) as libc::c_float;
                u = (2.0f64
                    * (((rand() & 0x7fff as libc::c_int) as libc::c_float
                        / 0x7fff as libc::c_int as libc::c_float) as libc::c_double
                        - 0.5f64) * vspread as libc::c_double
                    * 2 as libc::c_int as libc::c_double) as libc::c_float;
                VectorMA(
                    water_start.as_mut_ptr(),
                    8192 as libc::c_int as libc::c_float,
                    forward.as_mut_ptr(),
                    end.as_mut_ptr(),
                );
                VectorMA(end.as_mut_ptr(), r, right.as_mut_ptr(), end.as_mut_ptr());
                VectorMA(end.as_mut_ptr(), u, up.as_mut_ptr(), end.as_mut_ptr());
            }
            tr = (gi.trace)
                .expect(
                    "non-null function pointer",
                )(
                water_start.as_mut_ptr(),
                0 as *mut vec_t,
                0 as *mut vec_t,
                end.as_mut_ptr(),
                self_0,
                1 as libc::c_int | 0x2000000 as libc::c_int | 2 as libc::c_int
                    | 0x4000000 as libc::c_int,
            );
        }
    }
    if !(!(tr.surface).is_null() && (*tr.surface).flags & 0x4 as libc::c_int != 0) {
        if (tr.fraction as libc::c_double) < 1.0f64 {
            if (*tr.ent).takedamage != 0 {
                T_Damage(
                    tr.ent,
                    self_0,
                    self_0,
                    aimdir,
                    (tr.endpos).as_mut_ptr(),
                    (tr.plane.normal).as_mut_ptr(),
                    damage,
                    kick,
                    0x10 as libc::c_int,
                    mod_0,
                );
            } else if strncmp(
                ((*tr.surface).name).as_mut_ptr(),
                b"sky\0" as *const u8 as *const libc::c_char,
                3 as libc::c_int as libc::c_ulong,
            ) != 0 as libc::c_int
            {
                (gi.WriteByte).expect("non-null function pointer")(3 as libc::c_int);
                (gi.WriteByte).expect("non-null function pointer")(te_impact);
                (gi.WritePosition)
                    .expect("non-null function pointer")((tr.endpos).as_mut_ptr());
                (gi.WriteDir)
                    .expect("non-null function pointer")((tr.plane.normal).as_mut_ptr());
                (gi.multicast)
                    .expect(
                        "non-null function pointer",
                    )((tr.endpos).as_mut_ptr(), MULTICAST_PVS);
                if !((*self_0).client).is_null() {
                    PlayerNoise(self_0, (tr.endpos).as_mut_ptr(), 2 as libc::c_int);
                }
            }
        }
    }
    if water as u64 != 0 {
        let mut pos: vec3_t = [0.; 3];
        dir[0 as libc::c_int
            as usize] = tr.endpos[0 as libc::c_int as usize]
            - water_start[0 as libc::c_int as usize];
        dir[1 as libc::c_int
            as usize] = tr.endpos[1 as libc::c_int as usize]
            - water_start[1 as libc::c_int as usize];
        dir[2 as libc::c_int
            as usize] = tr.endpos[2 as libc::c_int as usize]
            - water_start[2 as libc::c_int as usize];
        VectorNormalize(dir.as_mut_ptr());
        VectorMA(
            (tr.endpos).as_mut_ptr(),
            -(2 as libc::c_int) as libc::c_float,
            dir.as_mut_ptr(),
            pos.as_mut_ptr(),
        );
        if (gi.pointcontents).expect("non-null function pointer")(pos.as_mut_ptr())
            & (32 as libc::c_int | 8 as libc::c_int | 16 as libc::c_int) != 0
        {
            tr.endpos[0 as libc::c_int as usize] = pos[0 as libc::c_int as usize];
            tr.endpos[1 as libc::c_int as usize] = pos[1 as libc::c_int as usize];
            tr.endpos[2 as libc::c_int as usize] = pos[2 as libc::c_int as usize];
        } else {
            tr = (gi.trace)
                .expect(
                    "non-null function pointer",
                )(
                pos.as_mut_ptr(),
                0 as *mut vec_t,
                0 as *mut vec_t,
                water_start.as_mut_ptr(),
                tr.ent,
                32 as libc::c_int | 8 as libc::c_int | 16 as libc::c_int,
            );
        }
        pos[0 as libc::c_int
            as usize] = water_start[0 as libc::c_int as usize]
            + tr.endpos[0 as libc::c_int as usize];
        pos[1 as libc::c_int
            as usize] = water_start[1 as libc::c_int as usize]
            + tr.endpos[1 as libc::c_int as usize];
        pos[2 as libc::c_int
            as usize] = water_start[2 as libc::c_int as usize]
            + tr.endpos[2 as libc::c_int as usize];
        VectorScale(pos.as_mut_ptr(), 0.5f64 as vec_t, pos.as_mut_ptr());
        (gi.WriteByte).expect("non-null function pointer")(3 as libc::c_int);
        (gi.WriteByte)
            .expect("non-null function pointer")(TE_BUBBLETRAIL as libc::c_int);
        (gi.WritePosition).expect("non-null function pointer")(water_start.as_mut_ptr());
        (gi.WritePosition).expect("non-null function pointer")((tr.endpos).as_mut_ptr());
        (gi.multicast)
            .expect("non-null function pointer")(pos.as_mut_ptr(), MULTICAST_PVS);
    }
}
#[no_mangle]
pub unsafe extern "C" fn fire_bullet(
    mut self_0: *mut edict_t,
    mut start: *mut vec_t,
    mut aimdir: *mut vec_t,
    mut damage: libc::c_int,
    mut kick: libc::c_int,
    mut hspread: libc::c_int,
    mut vspread: libc::c_int,
    mut mod_0: libc::c_int,
) {
    fire_lead(
        self_0,
        start,
        aimdir,
        damage,
        kick,
        TE_GUNSHOT as libc::c_int,
        hspread,
        vspread,
        mod_0,
    );
}
#[no_mangle]
pub unsafe extern "C" fn fire_shotgun(
    mut self_0: *mut edict_t,
    mut start: *mut vec_t,
    mut aimdir: *mut vec_t,
    mut damage: libc::c_int,
    mut kick: libc::c_int,
    mut hspread: libc::c_int,
    mut vspread: libc::c_int,
    mut count: libc::c_int,
    mut mod_0: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < count {
        fire_lead(
            self_0,
            start,
            aimdir,
            damage,
            kick,
            TE_SHOTGUN as libc::c_int,
            hspread,
            vspread,
            mod_0,
        );
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn blaster_touch(
    mut self_0: *mut edict_t,
    mut other: *mut edict_t,
    mut plane: *mut cplane_t,
    mut surf: *mut csurface_t,
) {
    let mut mod_0: libc::c_int = 0;
    if other == (*self_0).owner {
        return;
    }
    if !surf.is_null() && (*surf).flags & 0x4 as libc::c_int != 0 {
        G_FreeEdict(self_0);
        return;
    }
    if !((*(*self_0).owner).client).is_null() {
        PlayerNoise(
            (*self_0).owner,
            ((*self_0).s.origin).as_mut_ptr(),
            2 as libc::c_int,
        );
    }
    if (*other).takedamage != 0 {
        if (*self_0).spawnflags & 1 as libc::c_int != 0 {
            mod_0 = 10 as libc::c_int;
        } else {
            mod_0 = 1 as libc::c_int;
        }
        T_Damage(
            other,
            self_0,
            (*self_0).owner,
            ((*self_0).velocity).as_mut_ptr(),
            ((*self_0).s.origin).as_mut_ptr(),
            ((*plane).normal).as_mut_ptr(),
            (*self_0).dmg,
            1 as libc::c_int,
            0x4 as libc::c_int,
            mod_0,
        );
    } else {
        (gi.WriteByte).expect("non-null function pointer")(3 as libc::c_int);
        (gi.WriteByte).expect("non-null function pointer")(TE_BLASTER as libc::c_int);
        (gi.WritePosition)
            .expect("non-null function pointer")(((*self_0).s.origin).as_mut_ptr());
        if plane.is_null() {
            (gi.WriteDir).expect("non-null function pointer")(vec3_origin.as_mut_ptr());
        } else {
            (gi.WriteDir)
                .expect("non-null function pointer")(((*plane).normal).as_mut_ptr());
        }
        (gi.multicast)
            .expect(
                "non-null function pointer",
            )(((*self_0).s.origin).as_mut_ptr(), MULTICAST_PVS);
    }
    G_FreeEdict(self_0);
}
#[no_mangle]
pub unsafe extern "C" fn fire_blaster(
    mut self_0: *mut edict_t,
    mut start: *mut vec_t,
    mut dir: *mut vec_t,
    mut damage: libc::c_int,
    mut speed: libc::c_int,
    mut effect: libc::c_int,
    mut hyper: qboolean,
) {
    let mut bolt: *mut edict_t = 0 as *mut edict_t;
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
    VectorNormalize(dir);
    bolt = G_Spawn();
    (*bolt).svflags = 0x8 as libc::c_int;
    (*bolt)
        .s
        .origin[0 as libc::c_int as usize] = *start.offset(0 as libc::c_int as isize);
    (*bolt)
        .s
        .origin[1 as libc::c_int as usize] = *start.offset(1 as libc::c_int as isize);
    (*bolt)
        .s
        .origin[2 as libc::c_int as usize] = *start.offset(2 as libc::c_int as isize);
    (*bolt)
        .s
        .old_origin[0 as libc::c_int
        as usize] = *start.offset(0 as libc::c_int as isize);
    (*bolt)
        .s
        .old_origin[1 as libc::c_int
        as usize] = *start.offset(1 as libc::c_int as isize);
    (*bolt)
        .s
        .old_origin[2 as libc::c_int
        as usize] = *start.offset(2 as libc::c_int as isize);
    vectoangles(dir, ((*bolt).s.angles).as_mut_ptr());
    VectorScale(dir, speed as vec_t, ((*bolt).velocity).as_mut_ptr());
    (*bolt).movetype = MOVETYPE_FLYMISSILE as libc::c_int;
    (*bolt)
        .clipmask = 1 as libc::c_int | 0x2000000 as libc::c_int | 2 as libc::c_int
        | 0x4000000 as libc::c_int;
    (*bolt).solid = SOLID_BBOX;
    (*bolt).s.effects |= effect as libc::c_uint;
    let ref mut fresh1 = (*bolt).mins[2 as libc::c_int as usize];
    *fresh1 = 0 as libc::c_int as vec_t;
    let ref mut fresh2 = (*bolt).mins[1 as libc::c_int as usize];
    *fresh2 = *fresh1;
    (*bolt).mins[0 as libc::c_int as usize] = *fresh2;
    let ref mut fresh3 = (*bolt).maxs[2 as libc::c_int as usize];
    *fresh3 = 0 as libc::c_int as vec_t;
    let ref mut fresh4 = (*bolt).maxs[1 as libc::c_int as usize];
    *fresh4 = *fresh3;
    (*bolt).maxs[0 as libc::c_int as usize] = *fresh4;
    (*bolt)
        .s
        .modelindex = (gi.modelindex)
        .expect(
            "non-null function pointer",
        )(
        b"models/objects/laser/tris.md2\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    (*bolt)
        .s
        .sound = (gi.soundindex)
        .expect(
            "non-null function pointer",
        )(b"misc/lasfly.wav\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    let ref mut fresh5 = (*bolt).owner;
    *fresh5 = self_0;
    let ref mut fresh6 = (*bolt).touch;
    *fresh6 = Some(
        blaster_touch
            as unsafe extern "C" fn(
                *mut edict_t,
                *mut edict_t,
                *mut cplane_t,
                *mut csurface_t,
            ) -> (),
    );
    (*bolt).nextthink = level.time + 2 as libc::c_int as libc::c_float;
    let ref mut fresh7 = (*bolt).think;
    *fresh7 = Some(G_FreeEdict as unsafe extern "C" fn(*mut edict_t) -> ());
    (*bolt).dmg = damage;
    let ref mut fresh8 = (*bolt).classname;
    *fresh8 = b"bolt\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    if hyper as u64 != 0 {
        (*bolt).spawnflags = 1 as libc::c_int;
    }
    (gi.linkentity).expect("non-null function pointer")(bolt);
    if !((*self_0).client).is_null() {
        check_dodge(self_0, ((*bolt).s.origin).as_mut_ptr(), dir, speed);
    }
    tr = (gi.trace)
        .expect(
            "non-null function pointer",
        )(
        ((*self_0).s.origin).as_mut_ptr(),
        0 as *mut vec_t,
        0 as *mut vec_t,
        ((*bolt).s.origin).as_mut_ptr(),
        bolt,
        1 as libc::c_int | 0x2000000 as libc::c_int | 2 as libc::c_int
            | 0x4000000 as libc::c_int,
    );
    if (tr.fraction as libc::c_double) < 1.0f64 {
        VectorMA(
            ((*bolt).s.origin).as_mut_ptr(),
            -(10 as libc::c_int) as libc::c_float,
            dir,
            ((*bolt).s.origin).as_mut_ptr(),
        );
        ((*bolt).touch)
            .expect(
                "non-null function pointer",
            )(bolt, tr.ent, 0 as *mut cplane_t, 0 as *mut csurface_t);
    }
}
unsafe extern "C" fn Grenade_Explode(mut ent: *mut edict_t) {
    let mut origin: vec3_t = [0.; 3];
    let mut mod_0: libc::c_int = 0;
    if !((*(*ent).owner).client).is_null() {
        PlayerNoise((*ent).owner, ((*ent).s.origin).as_mut_ptr(), 2 as libc::c_int);
    }
    if !((*ent).enemy).is_null() {
        let mut points: libc::c_float = 0.;
        let mut v: vec3_t = [0.; 3];
        let mut dir: vec3_t = [0.; 3];
        v[0 as libc::c_int
            as usize] = (*(*ent).enemy).mins[0 as libc::c_int as usize]
            + (*(*ent).enemy).maxs[0 as libc::c_int as usize];
        v[1 as libc::c_int
            as usize] = (*(*ent).enemy).mins[1 as libc::c_int as usize]
            + (*(*ent).enemy).maxs[1 as libc::c_int as usize];
        v[2 as libc::c_int
            as usize] = (*(*ent).enemy).mins[2 as libc::c_int as usize]
            + (*(*ent).enemy).maxs[2 as libc::c_int as usize];
        VectorMA(
            ((*(*ent).enemy).s.origin).as_mut_ptr(),
            0.5f64 as libc::c_float,
            v.as_mut_ptr(),
            v.as_mut_ptr(),
        );
        v[0 as libc::c_int
            as usize] = (*ent).s.origin[0 as libc::c_int as usize]
            - v[0 as libc::c_int as usize];
        v[1 as libc::c_int
            as usize] = (*ent).s.origin[1 as libc::c_int as usize]
            - v[1 as libc::c_int as usize];
        v[2 as libc::c_int
            as usize] = (*ent).s.origin[2 as libc::c_int as usize]
            - v[2 as libc::c_int as usize];
        points = ((*ent).dmg as libc::c_double
            - 0.5f64 * VectorLength(v.as_mut_ptr()) as libc::c_double) as libc::c_float;
        dir[0 as libc::c_int
            as usize] = (*(*ent).enemy).s.origin[0 as libc::c_int as usize]
            - (*ent).s.origin[0 as libc::c_int as usize];
        dir[1 as libc::c_int
            as usize] = (*(*ent).enemy).s.origin[1 as libc::c_int as usize]
            - (*ent).s.origin[1 as libc::c_int as usize];
        dir[2 as libc::c_int
            as usize] = (*(*ent).enemy).s.origin[2 as libc::c_int as usize]
            - (*ent).s.origin[2 as libc::c_int as usize];
        if (*ent).spawnflags & 1 as libc::c_int != 0 {
            mod_0 = 15 as libc::c_int;
        } else {
            mod_0 = 6 as libc::c_int;
        }
        T_Damage(
            (*ent).enemy,
            ent,
            (*ent).owner,
            dir.as_mut_ptr(),
            ((*ent).s.origin).as_mut_ptr(),
            vec3_origin.as_mut_ptr(),
            points as libc::c_int,
            points as libc::c_int,
            0x1 as libc::c_int,
            mod_0,
        );
    }
    if (*ent).spawnflags & 2 as libc::c_int != 0 {
        mod_0 = 24 as libc::c_int;
    } else if (*ent).spawnflags & 1 as libc::c_int != 0 {
        mod_0 = 16 as libc::c_int;
    } else {
        mod_0 = 7 as libc::c_int;
    }
    T_RadiusDamage(
        ent,
        (*ent).owner,
        (*ent).dmg as libc::c_float,
        (*ent).enemy,
        (*ent).dmg_radius,
        mod_0,
    );
    VectorMA(
        ((*ent).s.origin).as_mut_ptr(),
        -0.02f64 as libc::c_float,
        ((*ent).velocity).as_mut_ptr(),
        origin.as_mut_ptr(),
    );
    (gi.WriteByte).expect("non-null function pointer")(3 as libc::c_int);
    if (*ent).waterlevel != 0 {
        if !((*ent).groundentity).is_null() {
            (gi.WriteByte)
                .expect(
                    "non-null function pointer",
                )(TE_GRENADE_EXPLOSION_WATER as libc::c_int);
        } else {
            (gi.WriteByte)
                .expect(
                    "non-null function pointer",
                )(TE_ROCKET_EXPLOSION_WATER as libc::c_int);
        }
    } else if !((*ent).groundentity).is_null() {
        (gi.WriteByte)
            .expect("non-null function pointer")(TE_GRENADE_EXPLOSION as libc::c_int);
    } else {
        (gi.WriteByte)
            .expect("non-null function pointer")(TE_ROCKET_EXPLOSION as libc::c_int);
    }
    (gi.WritePosition).expect("non-null function pointer")(origin.as_mut_ptr());
    (gi.multicast)
        .expect(
            "non-null function pointer",
        )(((*ent).s.origin).as_mut_ptr(), MULTICAST_PHS);
    G_FreeEdict(ent);
}
unsafe extern "C" fn Grenade_Touch(
    mut ent: *mut edict_t,
    mut other: *mut edict_t,
    mut plane: *mut cplane_t,
    mut surf: *mut csurface_t,
) {
    if other == (*ent).owner {
        return;
    }
    if !surf.is_null() && (*surf).flags & 0x4 as libc::c_int != 0 {
        G_FreeEdict(ent);
        return;
    }
    if (*other).takedamage == 0 {
        if (*ent).spawnflags & 1 as libc::c_int != 0 {
            if ((rand() & 0x7fff as libc::c_int) as libc::c_float
                / 0x7fff as libc::c_int as libc::c_float) as libc::c_double > 0.5f64
            {
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
                        b"weapons/hgrenb1a.wav\0" as *const u8 as *const libc::c_char
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
                    2 as libc::c_int,
                    (gi.soundindex)
                        .expect(
                            "non-null function pointer",
                        )(
                        b"weapons/hgrenb2a.wav\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    ),
                    1 as libc::c_int as libc::c_float,
                    1 as libc::c_int as libc::c_float,
                    0 as libc::c_int as libc::c_float,
                );
            }
        } else {
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
                    b"weapons/grenlb1b.wav\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                ),
                1 as libc::c_int as libc::c_float,
                1 as libc::c_int as libc::c_float,
                0 as libc::c_int as libc::c_float,
            );
        }
        return;
    }
    let ref mut fresh9 = (*ent).enemy;
    *fresh9 = other;
    Grenade_Explode(ent);
}
#[no_mangle]
pub unsafe extern "C" fn fire_grenade(
    mut self_0: *mut edict_t,
    mut start: *mut vec_t,
    mut aimdir: *mut vec_t,
    mut damage: libc::c_int,
    mut speed: libc::c_int,
    mut timer: libc::c_float,
    mut damage_radius: libc::c_float,
) {
    let mut grenade: *mut edict_t = 0 as *mut edict_t;
    let mut dir: vec3_t = [0.; 3];
    let mut forward: vec3_t = [0.; 3];
    let mut right: vec3_t = [0.; 3];
    let mut up: vec3_t = [0.; 3];
    vectoangles(aimdir, dir.as_mut_ptr());
    AngleVectors(
        dir.as_mut_ptr(),
        forward.as_mut_ptr(),
        right.as_mut_ptr(),
        up.as_mut_ptr(),
    );
    grenade = G_Spawn();
    (*grenade)
        .s
        .origin[0 as libc::c_int as usize] = *start.offset(0 as libc::c_int as isize);
    (*grenade)
        .s
        .origin[1 as libc::c_int as usize] = *start.offset(1 as libc::c_int as isize);
    (*grenade)
        .s
        .origin[2 as libc::c_int as usize] = *start.offset(2 as libc::c_int as isize);
    VectorScale(aimdir, speed as vec_t, ((*grenade).velocity).as_mut_ptr());
    VectorMA(
        ((*grenade).velocity).as_mut_ptr(),
        (200 as libc::c_int as libc::c_double
            + 2.0f64
                * (((rand() & 0x7fff as libc::c_int) as libc::c_float
                    / 0x7fff as libc::c_int as libc::c_float) as libc::c_double - 0.5f64)
                * 10.0f64) as libc::c_float,
        up.as_mut_ptr(),
        ((*grenade).velocity).as_mut_ptr(),
    );
    VectorMA(
        ((*grenade).velocity).as_mut_ptr(),
        (2.0f64
            * (((rand() & 0x7fff as libc::c_int) as libc::c_float
                / 0x7fff as libc::c_int as libc::c_float) as libc::c_double - 0.5f64)
            * 10.0f64) as libc::c_float,
        right.as_mut_ptr(),
        ((*grenade).velocity).as_mut_ptr(),
    );
    (*grenade).avelocity[0 as libc::c_int as usize] = 300 as libc::c_int as vec_t;
    (*grenade).avelocity[1 as libc::c_int as usize] = 300 as libc::c_int as vec_t;
    (*grenade).avelocity[2 as libc::c_int as usize] = 300 as libc::c_int as vec_t;
    (*grenade).movetype = MOVETYPE_BOUNCE as libc::c_int;
    (*grenade)
        .clipmask = 1 as libc::c_int | 0x2000000 as libc::c_int | 2 as libc::c_int
        | 0x4000000 as libc::c_int;
    (*grenade).solid = SOLID_BBOX;
    (*grenade).s.effects |= 0x20 as libc::c_int as libc::c_uint;
    let ref mut fresh10 = (*grenade).mins[2 as libc::c_int as usize];
    *fresh10 = 0 as libc::c_int as vec_t;
    let ref mut fresh11 = (*grenade).mins[1 as libc::c_int as usize];
    *fresh11 = *fresh10;
    (*grenade).mins[0 as libc::c_int as usize] = *fresh11;
    let ref mut fresh12 = (*grenade).maxs[2 as libc::c_int as usize];
    *fresh12 = 0 as libc::c_int as vec_t;
    let ref mut fresh13 = (*grenade).maxs[1 as libc::c_int as usize];
    *fresh13 = *fresh12;
    (*grenade).maxs[0 as libc::c_int as usize] = *fresh13;
    (*grenade)
        .s
        .modelindex = (gi.modelindex)
        .expect(
            "non-null function pointer",
        )(
        b"models/objects/grenade/tris.md2\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    let ref mut fresh14 = (*grenade).owner;
    *fresh14 = self_0;
    let ref mut fresh15 = (*grenade).touch;
    *fresh15 = Some(
        Grenade_Touch
            as unsafe extern "C" fn(
                *mut edict_t,
                *mut edict_t,
                *mut cplane_t,
                *mut csurface_t,
            ) -> (),
    );
    (*grenade).nextthink = level.time + timer;
    let ref mut fresh16 = (*grenade).think;
    *fresh16 = Some(Grenade_Explode as unsafe extern "C" fn(*mut edict_t) -> ());
    (*grenade).dmg = damage;
    (*grenade).dmg_radius = damage_radius;
    let ref mut fresh17 = (*grenade).classname;
    *fresh17 = b"grenade\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    (gi.linkentity).expect("non-null function pointer")(grenade);
}
#[no_mangle]
pub unsafe extern "C" fn fire_grenade2(
    mut self_0: *mut edict_t,
    mut start: *mut vec_t,
    mut aimdir: *mut vec_t,
    mut damage: libc::c_int,
    mut speed: libc::c_int,
    mut timer: libc::c_float,
    mut damage_radius: libc::c_float,
    mut held: qboolean,
) {
    let mut grenade: *mut edict_t = 0 as *mut edict_t;
    let mut dir: vec3_t = [0.; 3];
    let mut forward: vec3_t = [0.; 3];
    let mut right: vec3_t = [0.; 3];
    let mut up: vec3_t = [0.; 3];
    vectoangles(aimdir, dir.as_mut_ptr());
    AngleVectors(
        dir.as_mut_ptr(),
        forward.as_mut_ptr(),
        right.as_mut_ptr(),
        up.as_mut_ptr(),
    );
    grenade = G_Spawn();
    (*grenade)
        .s
        .origin[0 as libc::c_int as usize] = *start.offset(0 as libc::c_int as isize);
    (*grenade)
        .s
        .origin[1 as libc::c_int as usize] = *start.offset(1 as libc::c_int as isize);
    (*grenade)
        .s
        .origin[2 as libc::c_int as usize] = *start.offset(2 as libc::c_int as isize);
    VectorScale(aimdir, speed as vec_t, ((*grenade).velocity).as_mut_ptr());
    VectorMA(
        ((*grenade).velocity).as_mut_ptr(),
        (200 as libc::c_int as libc::c_double
            + 2.0f64
                * (((rand() & 0x7fff as libc::c_int) as libc::c_float
                    / 0x7fff as libc::c_int as libc::c_float) as libc::c_double - 0.5f64)
                * 10.0f64) as libc::c_float,
        up.as_mut_ptr(),
        ((*grenade).velocity).as_mut_ptr(),
    );
    VectorMA(
        ((*grenade).velocity).as_mut_ptr(),
        (2.0f64
            * (((rand() & 0x7fff as libc::c_int) as libc::c_float
                / 0x7fff as libc::c_int as libc::c_float) as libc::c_double - 0.5f64)
            * 10.0f64) as libc::c_float,
        right.as_mut_ptr(),
        ((*grenade).velocity).as_mut_ptr(),
    );
    (*grenade).avelocity[0 as libc::c_int as usize] = 300 as libc::c_int as vec_t;
    (*grenade).avelocity[1 as libc::c_int as usize] = 300 as libc::c_int as vec_t;
    (*grenade).avelocity[2 as libc::c_int as usize] = 300 as libc::c_int as vec_t;
    (*grenade).movetype = MOVETYPE_BOUNCE as libc::c_int;
    (*grenade)
        .clipmask = 1 as libc::c_int | 0x2000000 as libc::c_int | 2 as libc::c_int
        | 0x4000000 as libc::c_int;
    (*grenade).solid = SOLID_BBOX;
    (*grenade).s.effects |= 0x20 as libc::c_int as libc::c_uint;
    let ref mut fresh18 = (*grenade).mins[2 as libc::c_int as usize];
    *fresh18 = 0 as libc::c_int as vec_t;
    let ref mut fresh19 = (*grenade).mins[1 as libc::c_int as usize];
    *fresh19 = *fresh18;
    (*grenade).mins[0 as libc::c_int as usize] = *fresh19;
    let ref mut fresh20 = (*grenade).maxs[2 as libc::c_int as usize];
    *fresh20 = 0 as libc::c_int as vec_t;
    let ref mut fresh21 = (*grenade).maxs[1 as libc::c_int as usize];
    *fresh21 = *fresh20;
    (*grenade).maxs[0 as libc::c_int as usize] = *fresh21;
    (*grenade)
        .s
        .modelindex = (gi.modelindex)
        .expect(
            "non-null function pointer",
        )(
        b"models/objects/grenade2/tris.md2\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    let ref mut fresh22 = (*grenade).owner;
    *fresh22 = self_0;
    let ref mut fresh23 = (*grenade).touch;
    *fresh23 = Some(
        Grenade_Touch
            as unsafe extern "C" fn(
                *mut edict_t,
                *mut edict_t,
                *mut cplane_t,
                *mut csurface_t,
            ) -> (),
    );
    (*grenade).nextthink = level.time + timer;
    let ref mut fresh24 = (*grenade).think;
    *fresh24 = Some(Grenade_Explode as unsafe extern "C" fn(*mut edict_t) -> ());
    (*grenade).dmg = damage;
    (*grenade).dmg_radius = damage_radius;
    let ref mut fresh25 = (*grenade).classname;
    *fresh25 = b"hgrenade\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    if held as u64 != 0 {
        (*grenade).spawnflags = 3 as libc::c_int;
    } else {
        (*grenade).spawnflags = 1 as libc::c_int;
    }
    (*grenade)
        .s
        .sound = (gi.soundindex)
        .expect(
            "non-null function pointer",
        )(
        b"weapons/hgrenc1b.wav\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    if timer as libc::c_double <= 0.0f64 {
        Grenade_Explode(grenade);
    } else {
        (gi.sound)
            .expect(
                "non-null function pointer",
            )(
            self_0,
            1 as libc::c_int,
            (gi.soundindex)
                .expect(
                    "non-null function pointer",
                )(
                b"weapons/hgrent1a.wav\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            ),
            1 as libc::c_int as libc::c_float,
            1 as libc::c_int as libc::c_float,
            0 as libc::c_int as libc::c_float,
        );
        (gi.linkentity).expect("non-null function pointer")(grenade);
    };
}
#[no_mangle]
pub unsafe extern "C" fn rocket_touch(
    mut ent: *mut edict_t,
    mut other: *mut edict_t,
    mut plane: *mut cplane_t,
    mut surf: *mut csurface_t,
) {
    let mut origin: vec3_t = [0.; 3];
    let mut n: libc::c_int = 0;
    if other == (*ent).owner {
        return;
    }
    if !surf.is_null() && (*surf).flags & 0x4 as libc::c_int != 0 {
        G_FreeEdict(ent);
        return;
    }
    if !((*(*ent).owner).client).is_null() {
        PlayerNoise((*ent).owner, ((*ent).s.origin).as_mut_ptr(), 2 as libc::c_int);
    }
    VectorMA(
        ((*ent).s.origin).as_mut_ptr(),
        -0.02f64 as libc::c_float,
        ((*ent).velocity).as_mut_ptr(),
        origin.as_mut_ptr(),
    );
    if (*other).takedamage != 0 {
        T_Damage(
            other,
            ent,
            (*ent).owner,
            ((*ent).velocity).as_mut_ptr(),
            ((*ent).s.origin).as_mut_ptr(),
            ((*plane).normal).as_mut_ptr(),
            (*ent).dmg,
            0 as libc::c_int,
            0 as libc::c_int,
            8 as libc::c_int,
        );
    } else if (*deathmatch).value == 0. && (*coop).value == 0. {
        if !surf.is_null()
            && (*surf).flags
                & (0x8 as libc::c_int | 0x10 as libc::c_int | 0x20 as libc::c_int
                    | 0x40 as libc::c_int) == 0
        {
            n = rand() % 5 as libc::c_int;
            loop {
                let fresh26 = n;
                n = n - 1;
                if !(fresh26 != 0) {
                    break;
                }
                ThrowDebris(
                    ent,
                    b"models/objects/debris2/tris.md2\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    2 as libc::c_int as libc::c_float,
                    ((*ent).s.origin).as_mut_ptr(),
                );
            }
        }
    }
    T_RadiusDamage(
        ent,
        (*ent).owner,
        (*ent).radius_dmg as libc::c_float,
        other,
        (*ent).dmg_radius,
        9 as libc::c_int,
    );
    (gi.WriteByte).expect("non-null function pointer")(3 as libc::c_int);
    if (*ent).waterlevel != 0 {
        (gi.WriteByte)
            .expect(
                "non-null function pointer",
            )(TE_ROCKET_EXPLOSION_WATER as libc::c_int);
    } else {
        (gi.WriteByte)
            .expect("non-null function pointer")(TE_ROCKET_EXPLOSION as libc::c_int);
    }
    (gi.WritePosition).expect("non-null function pointer")(origin.as_mut_ptr());
    (gi.multicast)
        .expect(
            "non-null function pointer",
        )(((*ent).s.origin).as_mut_ptr(), MULTICAST_PHS);
    G_FreeEdict(ent);
}
#[no_mangle]
pub unsafe extern "C" fn fire_rocket(
    mut self_0: *mut edict_t,
    mut start: *mut vec_t,
    mut dir: *mut vec_t,
    mut damage: libc::c_int,
    mut speed: libc::c_int,
    mut damage_radius: libc::c_float,
    mut radius_damage: libc::c_int,
) {
    let mut rocket: *mut edict_t = 0 as *mut edict_t;
    rocket = G_Spawn();
    (*rocket)
        .s
        .origin[0 as libc::c_int as usize] = *start.offset(0 as libc::c_int as isize);
    (*rocket)
        .s
        .origin[1 as libc::c_int as usize] = *start.offset(1 as libc::c_int as isize);
    (*rocket)
        .s
        .origin[2 as libc::c_int as usize] = *start.offset(2 as libc::c_int as isize);
    (*rocket)
        .movedir[0 as libc::c_int as usize] = *dir.offset(0 as libc::c_int as isize);
    (*rocket)
        .movedir[1 as libc::c_int as usize] = *dir.offset(1 as libc::c_int as isize);
    (*rocket)
        .movedir[2 as libc::c_int as usize] = *dir.offset(2 as libc::c_int as isize);
    vectoangles(dir, ((*rocket).s.angles).as_mut_ptr());
    VectorScale(dir, speed as vec_t, ((*rocket).velocity).as_mut_ptr());
    (*rocket).movetype = MOVETYPE_FLYMISSILE as libc::c_int;
    (*rocket)
        .clipmask = 1 as libc::c_int | 0x2000000 as libc::c_int | 2 as libc::c_int
        | 0x4000000 as libc::c_int;
    (*rocket).solid = SOLID_BBOX;
    (*rocket).s.effects |= 0x10 as libc::c_int as libc::c_uint;
    let ref mut fresh27 = (*rocket).mins[2 as libc::c_int as usize];
    *fresh27 = 0 as libc::c_int as vec_t;
    let ref mut fresh28 = (*rocket).mins[1 as libc::c_int as usize];
    *fresh28 = *fresh27;
    (*rocket).mins[0 as libc::c_int as usize] = *fresh28;
    let ref mut fresh29 = (*rocket).maxs[2 as libc::c_int as usize];
    *fresh29 = 0 as libc::c_int as vec_t;
    let ref mut fresh30 = (*rocket).maxs[1 as libc::c_int as usize];
    *fresh30 = *fresh29;
    (*rocket).maxs[0 as libc::c_int as usize] = *fresh30;
    (*rocket)
        .s
        .modelindex = (gi.modelindex)
        .expect(
            "non-null function pointer",
        )(
        b"models/objects/rocket/tris.md2\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    let ref mut fresh31 = (*rocket).owner;
    *fresh31 = self_0;
    let ref mut fresh32 = (*rocket).touch;
    *fresh32 = Some(
        rocket_touch
            as unsafe extern "C" fn(
                *mut edict_t,
                *mut edict_t,
                *mut cplane_t,
                *mut csurface_t,
            ) -> (),
    );
    (*rocket).nextthink = level.time + (8000 as libc::c_int / speed) as libc::c_float;
    let ref mut fresh33 = (*rocket).think;
    *fresh33 = Some(G_FreeEdict as unsafe extern "C" fn(*mut edict_t) -> ());
    (*rocket).dmg = damage;
    (*rocket).radius_dmg = radius_damage;
    (*rocket).dmg_radius = damage_radius;
    (*rocket)
        .s
        .sound = (gi.soundindex)
        .expect(
            "non-null function pointer",
        )(
        b"weapons/rockfly.wav\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    let ref mut fresh34 = (*rocket).classname;
    *fresh34 = b"rocket\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    if !((*self_0).client).is_null() {
        check_dodge(self_0, ((*rocket).s.origin).as_mut_ptr(), dir, speed);
    }
    (gi.linkentity).expect("non-null function pointer")(rocket);
}
#[no_mangle]
pub unsafe extern "C" fn fire_rail(
    mut self_0: *mut edict_t,
    mut start: *mut vec_t,
    mut aimdir: *mut vec_t,
    mut damage: libc::c_int,
    mut kick: libc::c_int,
) {
    let mut from: vec3_t = [0.; 3];
    let mut end: vec3_t = [0.; 3];
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
    let mut ignore: *mut edict_t = 0 as *mut edict_t;
    let mut mask: libc::c_int = 0;
    let mut water: qboolean = false_0;
    VectorMA(start, 8192 as libc::c_int as libc::c_float, aimdir, end.as_mut_ptr());
    from[0 as libc::c_int as usize] = *start.offset(0 as libc::c_int as isize);
    from[1 as libc::c_int as usize] = *start.offset(1 as libc::c_int as isize);
    from[2 as libc::c_int as usize] = *start.offset(2 as libc::c_int as isize);
    ignore = self_0;
    water = false_0;
    mask = 1 as libc::c_int | 0x2000000 as libc::c_int | 2 as libc::c_int
        | 0x4000000 as libc::c_int | 16 as libc::c_int | 8 as libc::c_int;
    while !ignore.is_null() {
        tr = (gi.trace)
            .expect(
                "non-null function pointer",
            )(
            from.as_mut_ptr(),
            0 as *mut vec_t,
            0 as *mut vec_t,
            end.as_mut_ptr(),
            ignore,
            mask,
        );
        if tr.contents & (16 as libc::c_int | 8 as libc::c_int) != 0 {
            mask &= !(16 as libc::c_int | 8 as libc::c_int);
            water = true_0;
        } else {
            if (*tr.ent).svflags & 0x4 as libc::c_int != 0
                || !((*tr.ent).client).is_null()
            {
                ignore = tr.ent;
            } else {
                ignore = 0 as *mut edict_t;
            }
            if tr.ent != self_0 && (*tr.ent).takedamage != 0 {
                T_Damage(
                    tr.ent,
                    self_0,
                    self_0,
                    aimdir,
                    (tr.endpos).as_mut_ptr(),
                    (tr.plane.normal).as_mut_ptr(),
                    damage,
                    kick,
                    0 as libc::c_int,
                    11 as libc::c_int,
                );
            }
        }
        from[0 as libc::c_int as usize] = tr.endpos[0 as libc::c_int as usize];
        from[1 as libc::c_int as usize] = tr.endpos[1 as libc::c_int as usize];
        from[2 as libc::c_int as usize] = tr.endpos[2 as libc::c_int as usize];
    }
    (gi.WriteByte).expect("non-null function pointer")(3 as libc::c_int);
    (gi.WriteByte).expect("non-null function pointer")(TE_RAILTRAIL as libc::c_int);
    (gi.WritePosition).expect("non-null function pointer")(start);
    (gi.WritePosition).expect("non-null function pointer")((tr.endpos).as_mut_ptr());
    (gi.multicast)
        .expect(
            "non-null function pointer",
        )(((*self_0).s.origin).as_mut_ptr(), MULTICAST_PHS);
    if water as u64 != 0 {
        (gi.WriteByte).expect("non-null function pointer")(3 as libc::c_int);
        (gi.WriteByte).expect("non-null function pointer")(TE_RAILTRAIL as libc::c_int);
        (gi.WritePosition).expect("non-null function pointer")(start);
        (gi.WritePosition).expect("non-null function pointer")((tr.endpos).as_mut_ptr());
        (gi.multicast)
            .expect(
                "non-null function pointer",
            )((tr.endpos).as_mut_ptr(), MULTICAST_PHS);
    }
    if !((*self_0).client).is_null() {
        PlayerNoise(self_0, (tr.endpos).as_mut_ptr(), 2 as libc::c_int);
    }
}
#[no_mangle]
pub unsafe extern "C" fn bfg_explode(mut self_0: *mut edict_t) {
    let mut ent: *mut edict_t = 0 as *mut edict_t;
    let mut points: libc::c_float = 0.;
    let mut v: vec3_t = [0.; 3];
    let mut dist: libc::c_float = 0.;
    if (*self_0).s.frame == 0 as libc::c_int {
        ent = 0 as *mut edict_t;
        loop {
            ent = findradius(
                ent,
                ((*self_0).s.origin).as_mut_ptr(),
                (*self_0).dmg_radius,
            );
            if ent.is_null() {
                break;
            }
            if (*ent).takedamage == 0 {
                continue;
            }
            if ent == (*self_0).owner {
                continue;
            }
            if CanDamage(ent, self_0) as u64 == 0 {
                continue;
            }
            if CanDamage(ent, (*self_0).owner) as u64 == 0 {
                continue;
            }
            v[0 as libc::c_int
                as usize] = (*ent).mins[0 as libc::c_int as usize]
                + (*ent).maxs[0 as libc::c_int as usize];
            v[1 as libc::c_int
                as usize] = (*ent).mins[1 as libc::c_int as usize]
                + (*ent).maxs[1 as libc::c_int as usize];
            v[2 as libc::c_int
                as usize] = (*ent).mins[2 as libc::c_int as usize]
                + (*ent).maxs[2 as libc::c_int as usize];
            VectorMA(
                ((*ent).s.origin).as_mut_ptr(),
                0.5f64 as libc::c_float,
                v.as_mut_ptr(),
                v.as_mut_ptr(),
            );
            v[0 as libc::c_int
                as usize] = (*self_0).s.origin[0 as libc::c_int as usize]
                - v[0 as libc::c_int as usize];
            v[1 as libc::c_int
                as usize] = (*self_0).s.origin[1 as libc::c_int as usize]
                - v[1 as libc::c_int as usize];
            v[2 as libc::c_int
                as usize] = (*self_0).s.origin[2 as libc::c_int as usize]
                - v[2 as libc::c_int as usize];
            dist = VectorLength(v.as_mut_ptr());
            points = ((*self_0).radius_dmg as libc::c_double
                * (1.0f64 - sqrt((dist / (*self_0).dmg_radius) as libc::c_double)))
                as libc::c_float;
            if ent == (*self_0).owner {
                points = (points as libc::c_double * 0.5f64) as libc::c_float;
            }
            (gi.WriteByte).expect("non-null function pointer")(3 as libc::c_int);
            (gi.WriteByte)
                .expect("non-null function pointer")(TE_BFG_EXPLOSION as libc::c_int);
            (gi.WritePosition)
                .expect("non-null function pointer")(((*ent).s.origin).as_mut_ptr());
            (gi.multicast)
                .expect(
                    "non-null function pointer",
                )(((*ent).s.origin).as_mut_ptr(), MULTICAST_PHS);
            T_Damage(
                ent,
                self_0,
                (*self_0).owner,
                ((*self_0).velocity).as_mut_ptr(),
                ((*ent).s.origin).as_mut_ptr(),
                vec3_origin.as_mut_ptr(),
                points as libc::c_int,
                0 as libc::c_int,
                0x4 as libc::c_int,
                14 as libc::c_int,
            );
        }
    }
    (*self_0).nextthink = (level.time as libc::c_double + 0.1f64) as libc::c_float;
    let ref mut fresh35 = (*self_0).s.frame;
    *fresh35 += 1;
    if (*self_0).s.frame == 5 as libc::c_int {
        let ref mut fresh36 = (*self_0).think;
        *fresh36 = Some(G_FreeEdict as unsafe extern "C" fn(*mut edict_t) -> ());
    }
}
#[no_mangle]
pub unsafe extern "C" fn bfg_touch(
    mut self_0: *mut edict_t,
    mut other: *mut edict_t,
    mut plane: *mut cplane_t,
    mut surf: *mut csurface_t,
) {
    if other == (*self_0).owner {
        return;
    }
    if !surf.is_null() && (*surf).flags & 0x4 as libc::c_int != 0 {
        G_FreeEdict(self_0);
        return;
    }
    if !((*(*self_0).owner).client).is_null() {
        PlayerNoise(
            (*self_0).owner,
            ((*self_0).s.origin).as_mut_ptr(),
            2 as libc::c_int,
        );
    }
    if (*other).takedamage != 0 {
        T_Damage(
            other,
            self_0,
            (*self_0).owner,
            ((*self_0).velocity).as_mut_ptr(),
            ((*self_0).s.origin).as_mut_ptr(),
            ((*plane).normal).as_mut_ptr(),
            200 as libc::c_int,
            0 as libc::c_int,
            0 as libc::c_int,
            13 as libc::c_int,
        );
    }
    T_RadiusDamage(
        self_0,
        (*self_0).owner,
        200 as libc::c_int as libc::c_float,
        other,
        100 as libc::c_int as libc::c_float,
        13 as libc::c_int,
    );
    (gi.sound)
        .expect(
            "non-null function pointer",
        )(
        self_0,
        2 as libc::c_int,
        (gi.soundindex)
            .expect(
                "non-null function pointer",
            )(
            b"weapons/bfg__x1b.wav\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        ),
        1 as libc::c_int as libc::c_float,
        1 as libc::c_int as libc::c_float,
        0 as libc::c_int as libc::c_float,
    );
    (*self_0).solid = SOLID_NOT;
    let ref mut fresh37 = (*self_0).touch;
    *fresh37 = None;
    VectorMA(
        ((*self_0).s.origin).as_mut_ptr(),
        (-(1 as libc::c_int) as libc::c_double * 0.1f64) as libc::c_float,
        ((*self_0).velocity).as_mut_ptr(),
        ((*self_0).s.origin).as_mut_ptr(),
    );
    let ref mut fresh38 = (*self_0).velocity[2 as libc::c_int as usize];
    *fresh38 = 0 as libc::c_int as vec_t;
    let ref mut fresh39 = (*self_0).velocity[1 as libc::c_int as usize];
    *fresh39 = *fresh38;
    (*self_0).velocity[0 as libc::c_int as usize] = *fresh39;
    (*self_0)
        .s
        .modelindex = (gi.modelindex)
        .expect(
            "non-null function pointer",
        )(
        b"sprites/s_bfg3.sp2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    (*self_0).s.frame = 0 as libc::c_int;
    (*self_0).s.sound = 0 as libc::c_int;
    (*self_0).s.effects &= !(0x2000 as libc::c_int) as libc::c_uint;
    let ref mut fresh40 = (*self_0).think;
    *fresh40 = Some(bfg_explode as unsafe extern "C" fn(*mut edict_t) -> ());
    (*self_0).nextthink = (level.time as libc::c_double + 0.1f64) as libc::c_float;
    let ref mut fresh41 = (*self_0).enemy;
    *fresh41 = other;
    (gi.WriteByte).expect("non-null function pointer")(3 as libc::c_int);
    (gi.WriteByte)
        .expect("non-null function pointer")(TE_BFG_BIGEXPLOSION as libc::c_int);
    (gi.WritePosition)
        .expect("non-null function pointer")(((*self_0).s.origin).as_mut_ptr());
    (gi.multicast)
        .expect(
            "non-null function pointer",
        )(((*self_0).s.origin).as_mut_ptr(), MULTICAST_PVS);
}
#[no_mangle]
pub unsafe extern "C" fn bfg_think(mut self_0: *mut edict_t) {
    let mut ent: *mut edict_t = 0 as *mut edict_t;
    let mut ignore: *mut edict_t = 0 as *mut edict_t;
    let mut point: vec3_t = [0.; 3];
    let mut dir: vec3_t = [0.; 3];
    let mut start: vec3_t = [0.; 3];
    let mut end: vec3_t = [0.; 3];
    let mut dmg: libc::c_int = 0;
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
    if (*deathmatch).value != 0. {
        dmg = 5 as libc::c_int;
    } else {
        dmg = 10 as libc::c_int;
    }
    ent = 0 as *mut edict_t;
    loop {
        ent = findradius(
            ent,
            ((*self_0).s.origin).as_mut_ptr(),
            256 as libc::c_int as libc::c_float,
        );
        if ent.is_null() {
            break;
        }
        if ent == self_0 {
            continue;
        }
        if ent == (*self_0).owner {
            continue;
        }
        if (*ent).takedamage == 0 {
            continue;
        }
        if (*ent).svflags & 0x4 as libc::c_int == 0 && ((*ent).client).is_null()
            && strcmp(
                (*ent).classname,
                b"misc_explobox\0" as *const u8 as *const libc::c_char,
            ) != 0 as libc::c_int
        {
            continue;
        }
        if (*ctf).value != 0. && !((*ent).client).is_null()
            && !((*(*self_0).owner).client).is_null()
            && (*(*ent).client).resp.ctf_team
                == (*(*(*self_0).owner).client).resp.ctf_team
        {
            continue;
        }
        VectorMA(
            ((*ent).absmin).as_mut_ptr(),
            0.5f64 as libc::c_float,
            ((*ent).size).as_mut_ptr(),
            point.as_mut_ptr(),
        );
        dir[0 as libc::c_int
            as usize] = point[0 as libc::c_int as usize]
            - (*self_0).s.origin[0 as libc::c_int as usize];
        dir[1 as libc::c_int
            as usize] = point[1 as libc::c_int as usize]
            - (*self_0).s.origin[1 as libc::c_int as usize];
        dir[2 as libc::c_int
            as usize] = point[2 as libc::c_int as usize]
            - (*self_0).s.origin[2 as libc::c_int as usize];
        VectorNormalize(dir.as_mut_ptr());
        ignore = self_0;
        start[0 as libc::c_int as usize] = (*self_0).s.origin[0 as libc::c_int as usize];
        start[1 as libc::c_int as usize] = (*self_0).s.origin[1 as libc::c_int as usize];
        start[2 as libc::c_int as usize] = (*self_0).s.origin[2 as libc::c_int as usize];
        VectorMA(
            start.as_mut_ptr(),
            2048 as libc::c_int as libc::c_float,
            dir.as_mut_ptr(),
            end.as_mut_ptr(),
        );
        loop {
            tr = (gi.trace)
                .expect(
                    "non-null function pointer",
                )(
                start.as_mut_ptr(),
                0 as *mut vec_t,
                0 as *mut vec_t,
                end.as_mut_ptr(),
                ignore,
                1 as libc::c_int | 0x2000000 as libc::c_int | 0x4000000 as libc::c_int,
            );
            if (tr.ent).is_null() {
                break;
            }
            if (*tr.ent).takedamage != 0 && (*tr.ent).flags & 0x4 as libc::c_int == 0
                && tr.ent != (*self_0).owner
            {
                T_Damage(
                    tr.ent,
                    self_0,
                    (*self_0).owner,
                    dir.as_mut_ptr(),
                    (tr.endpos).as_mut_ptr(),
                    vec3_origin.as_mut_ptr(),
                    dmg,
                    1 as libc::c_int,
                    0x4 as libc::c_int,
                    12 as libc::c_int,
                );
            }
            if (*tr.ent).svflags & 0x4 as libc::c_int == 0
                && ((*tr.ent).client).is_null()
            {
                (gi.WriteByte).expect("non-null function pointer")(3 as libc::c_int);
                (gi.WriteByte)
                    .expect("non-null function pointer")(TE_LASER_SPARKS as libc::c_int);
                (gi.WriteByte).expect("non-null function pointer")(4 as libc::c_int);
                (gi.WritePosition)
                    .expect("non-null function pointer")((tr.endpos).as_mut_ptr());
                (gi.WriteDir)
                    .expect("non-null function pointer")((tr.plane.normal).as_mut_ptr());
                (gi.WriteByte).expect("non-null function pointer")((*self_0).s.skinnum);
                (gi.multicast)
                    .expect(
                        "non-null function pointer",
                    )((tr.endpos).as_mut_ptr(), MULTICAST_PVS);
                break;
            } else {
                ignore = tr.ent;
                start[0 as libc::c_int as usize] = tr.endpos[0 as libc::c_int as usize];
                start[1 as libc::c_int as usize] = tr.endpos[1 as libc::c_int as usize];
                start[2 as libc::c_int as usize] = tr.endpos[2 as libc::c_int as usize];
            }
        }
        (gi.WriteByte).expect("non-null function pointer")(3 as libc::c_int);
        (gi.WriteByte).expect("non-null function pointer")(TE_BFG_LASER as libc::c_int);
        (gi.WritePosition)
            .expect("non-null function pointer")(((*self_0).s.origin).as_mut_ptr());
        (gi.WritePosition).expect("non-null function pointer")((tr.endpos).as_mut_ptr());
        (gi.multicast)
            .expect(
                "non-null function pointer",
            )(((*self_0).s.origin).as_mut_ptr(), MULTICAST_PHS);
    }
    (*self_0).nextthink = (level.time as libc::c_double + 0.1f64) as libc::c_float;
}
#[no_mangle]
pub unsafe extern "C" fn fire_bfg(
    mut self_0: *mut edict_t,
    mut start: *mut vec_t,
    mut dir: *mut vec_t,
    mut damage: libc::c_int,
    mut speed: libc::c_int,
    mut damage_radius: libc::c_float,
) {
    let mut bfg: *mut edict_t = 0 as *mut edict_t;
    bfg = G_Spawn();
    (*bfg)
        .s
        .origin[0 as libc::c_int as usize] = *start.offset(0 as libc::c_int as isize);
    (*bfg)
        .s
        .origin[1 as libc::c_int as usize] = *start.offset(1 as libc::c_int as isize);
    (*bfg)
        .s
        .origin[2 as libc::c_int as usize] = *start.offset(2 as libc::c_int as isize);
    (*bfg).movedir[0 as libc::c_int as usize] = *dir.offset(0 as libc::c_int as isize);
    (*bfg).movedir[1 as libc::c_int as usize] = *dir.offset(1 as libc::c_int as isize);
    (*bfg).movedir[2 as libc::c_int as usize] = *dir.offset(2 as libc::c_int as isize);
    vectoangles(dir, ((*bfg).s.angles).as_mut_ptr());
    VectorScale(dir, speed as vec_t, ((*bfg).velocity).as_mut_ptr());
    (*bfg).movetype = MOVETYPE_FLYMISSILE as libc::c_int;
    (*bfg)
        .clipmask = 1 as libc::c_int | 0x2000000 as libc::c_int | 2 as libc::c_int
        | 0x4000000 as libc::c_int;
    (*bfg).solid = SOLID_BBOX;
    (*bfg).s.effects |= (0x80 as libc::c_int | 0x2000 as libc::c_int) as libc::c_uint;
    let ref mut fresh42 = (*bfg).mins[2 as libc::c_int as usize];
    *fresh42 = 0 as libc::c_int as vec_t;
    let ref mut fresh43 = (*bfg).mins[1 as libc::c_int as usize];
    *fresh43 = *fresh42;
    (*bfg).mins[0 as libc::c_int as usize] = *fresh43;
    let ref mut fresh44 = (*bfg).maxs[2 as libc::c_int as usize];
    *fresh44 = 0 as libc::c_int as vec_t;
    let ref mut fresh45 = (*bfg).maxs[1 as libc::c_int as usize];
    *fresh45 = *fresh44;
    (*bfg).maxs[0 as libc::c_int as usize] = *fresh45;
    (*bfg)
        .s
        .modelindex = (gi.modelindex)
        .expect(
            "non-null function pointer",
        )(
        b"sprites/s_bfg1.sp2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    let ref mut fresh46 = (*bfg).owner;
    *fresh46 = self_0;
    let ref mut fresh47 = (*bfg).touch;
    *fresh47 = Some(
        bfg_touch
            as unsafe extern "C" fn(
                *mut edict_t,
                *mut edict_t,
                *mut cplane_t,
                *mut csurface_t,
            ) -> (),
    );
    (*bfg).nextthink = level.time + (8000 as libc::c_int / speed) as libc::c_float;
    let ref mut fresh48 = (*bfg).think;
    *fresh48 = Some(G_FreeEdict as unsafe extern "C" fn(*mut edict_t) -> ());
    (*bfg).radius_dmg = damage;
    (*bfg).dmg_radius = damage_radius;
    let ref mut fresh49 = (*bfg).classname;
    *fresh49 = b"bfg blast\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    (*bfg)
        .s
        .sound = (gi.soundindex)
        .expect(
            "non-null function pointer",
        )(
        b"weapons/bfg__l1a.wav\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    let ref mut fresh50 = (*bfg).think;
    *fresh50 = Some(bfg_think as unsafe extern "C" fn(*mut edict_t) -> ());
    (*bfg).nextthink = (level.time as libc::c_double + 0.1f64) as libc::c_float;
    let ref mut fresh51 = (*bfg).teammaster;
    *fresh51 = bfg;
    let ref mut fresh52 = (*bfg).teamchain;
    *fresh52 = 0 as *mut edict_t;
    if !((*self_0).client).is_null() {
        check_dodge(self_0, ((*bfg).s.origin).as_mut_ptr(), dir, speed);
    }
    (gi.linkentity).expect("non-null function pointer")(bfg);
}
