#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn rand() -> libc::c_int;
    fn time(__timer: *mut time_t) -> time_t;
    fn localtime(__timer: *const time_t) -> *mut tm;
    static mut vec3_origin: vec3_t;
    fn VectorMA(
        veca: *mut vec_t,
        scale: libc::c_float,
        vecb: *mut vec_t,
        vecc: *mut vec_t,
    );
    fn VectorNormalize(v: *mut vec_t) -> vec_t;
    fn VectorScale(in_0: *mut vec_t, scale: vec_t, out: *mut vec_t);
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
    static mut level: level_locals_t;
    static mut gi: game_import_t;
    static mut sm_meat_index: libc::c_int;
    static mut deathmatch: *mut cvar_t;
    fn KillBox(ent: *mut edict_t) -> qboolean;
    fn G_Find(
        from: *mut edict_t,
        fieldofs: libc::c_int,
        match_0: *mut libc::c_char,
    ) -> *mut edict_t;
    fn G_PickTarget(targetname: *mut libc::c_char) -> *mut edict_t;
    fn G_UseTargets(ent: *mut edict_t, activator: *mut edict_t);
    fn G_Spawn() -> *mut edict_t;
    fn G_FreeEdict(e: *mut edict_t);
    fn vtos(v: *mut vec_t) -> *mut libc::c_char;
    fn vectoyaw(vec: *mut vec_t) -> libc::c_float;
    fn vectoangles(vec: *mut vec_t, angles: *mut vec_t);
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
    fn M_droptofloor(ent: *mut edict_t);
    fn CTFRespawnTech(ent: *mut edict_t);
    fn CTFTeamName(team: libc::c_int) -> *mut libc::c_char;
    fn CTFResetFlag(ctf_team: libc::c_int);
    fn M_walkmove(
        ent: *mut edict_t,
        yaw: libc::c_float,
        dist: libc::c_float,
    ) -> qboolean;
    fn CTFPlayerResetGrapple(ent: *mut edict_t);
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn train_use(self_0: *mut edict_t, other: *mut edict_t, activator: *mut edict_t);
    fn func_train_find(self_0: *mut edict_t);
}
pub type __time_t = libc::c_long;
pub type time_t = __time_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tm {
    pub tm_sec: libc::c_int,
    pub tm_min: libc::c_int,
    pub tm_hour: libc::c_int,
    pub tm_mday: libc::c_int,
    pub tm_mon: libc::c_int,
    pub tm_year: libc::c_int,
    pub tm_wday: libc::c_int,
    pub tm_yday: libc::c_int,
    pub tm_isdst: libc::c_int,
    pub tm_gmtoff: libc::c_long,
    pub tm_zone: *const libc::c_char,
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
pub type C2RustUnnamed_0 = libc::c_uint;
pub const EV_OTHER_TELEPORT: C2RustUnnamed_0 = 7;
pub const EV_PLAYER_TELEPORT: C2RustUnnamed_0 = 6;
pub const EV_FALLFAR: C2RustUnnamed_0 = 5;
pub const EV_FALL: C2RustUnnamed_0 = 4;
pub const EV_FALLSHORT: C2RustUnnamed_0 = 3;
pub const EV_FOOTSTEP: C2RustUnnamed_0 = 2;
pub const EV_ITEM_RESPAWN: C2RustUnnamed_0 = 1;
pub const EV_NONE: C2RustUnnamed_0 = 0;
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
pub type C2RustUnnamed_1 = libc::c_uint;
pub const DAMAGE_AIM: C2RustUnnamed_1 = 2;
pub const DAMAGE_YES: C2RustUnnamed_1 = 1;
pub const DAMAGE_NO: C2RustUnnamed_1 = 0;
pub type C2RustUnnamed_2 = libc::c_uint;
pub const MOVETYPE_BOUNCE: C2RustUnnamed_2 = 9;
pub const MOVETYPE_FLYMISSILE: C2RustUnnamed_2 = 8;
pub const MOVETYPE_TOSS: C2RustUnnamed_2 = 7;
pub const MOVETYPE_FLY: C2RustUnnamed_2 = 6;
pub const MOVETYPE_STEP: C2RustUnnamed_2 = 5;
pub const MOVETYPE_WALK: C2RustUnnamed_2 = 4;
pub const MOVETYPE_STOP: C2RustUnnamed_2 = 3;
pub const MOVETYPE_PUSH: C2RustUnnamed_2 = 2;
pub const MOVETYPE_NOCLIP: C2RustUnnamed_2 = 1;
pub const MOVETYPE_NONE: C2RustUnnamed_2 = 0;
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
pub const CTF_TEAM1: C2RustUnnamed_3 = 1;
pub const CTF_TEAM2: C2RustUnnamed_3 = 2;
pub type C2RustUnnamed_3 = libc::c_uint;
pub const CTF_NOTEAM: C2RustUnnamed_3 = 0;
#[no_mangle]
pub unsafe extern "C" fn Use_Areaportal(
    mut ent: *mut edict_t,
    mut other: *mut edict_t,
    mut activator: *mut edict_t,
) {
    (*ent).count ^= 1 as libc::c_int;
    (gi.SetAreaPortalState)
        .expect("non-null function pointer")((*ent).style, (*ent).count as qboolean);
}
#[no_mangle]
pub unsafe extern "C" fn SP_func_areaportal(mut ent: *mut edict_t) {
    let ref mut fresh0 = (*ent).use_0;
    *fresh0 = Some(
        Use_Areaportal
            as unsafe extern "C" fn(*mut edict_t, *mut edict_t, *mut edict_t) -> (),
    );
    (*ent).count = 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn VelocityForDamage(mut damage: libc::c_int, mut v: *mut vec_t) {
    *v
        .offset(
            0 as libc::c_int as isize,
        ) = (100.0f64
        * (2.0f64
            * (((rand() & 0x7fff as libc::c_int) as libc::c_float
                / 0x7fff as libc::c_int as libc::c_float) as libc::c_double - 0.5f64)))
        as vec_t;
    *v
        .offset(
            1 as libc::c_int as isize,
        ) = (100.0f64
        * (2.0f64
            * (((rand() & 0x7fff as libc::c_int) as libc::c_float
                / 0x7fff as libc::c_int as libc::c_float) as libc::c_double - 0.5f64)))
        as vec_t;
    *v
        .offset(
            2 as libc::c_int as isize,
        ) = (200.0f64
        + 100.0f64
            * ((rand() & 0x7fff as libc::c_int) as libc::c_float
                / 0x7fff as libc::c_int as libc::c_float) as libc::c_double) as vec_t;
    if damage < 50 as libc::c_int {
        VectorScale(v, 0.7f64 as vec_t, v);
    } else {
        VectorScale(v, 1.2f64 as vec_t, v);
    };
}
#[no_mangle]
pub unsafe extern "C" fn ClipGibVelocity(mut ent: *mut edict_t) {
    if (*ent).velocity[0 as libc::c_int as usize]
        < -(300 as libc::c_int) as libc::c_float
    {
        (*ent).velocity[0 as libc::c_int as usize] = -(300 as libc::c_int) as vec_t;
    } else if (*ent).velocity[0 as libc::c_int as usize]
        > 300 as libc::c_int as libc::c_float
    {
        (*ent).velocity[0 as libc::c_int as usize] = 300 as libc::c_int as vec_t;
    }
    if (*ent).velocity[1 as libc::c_int as usize]
        < -(300 as libc::c_int) as libc::c_float
    {
        (*ent).velocity[1 as libc::c_int as usize] = -(300 as libc::c_int) as vec_t;
    } else if (*ent).velocity[1 as libc::c_int as usize]
        > 300 as libc::c_int as libc::c_float
    {
        (*ent).velocity[1 as libc::c_int as usize] = 300 as libc::c_int as vec_t;
    }
    if (*ent).velocity[2 as libc::c_int as usize] < 200 as libc::c_int as libc::c_float {
        (*ent).velocity[2 as libc::c_int as usize] = 200 as libc::c_int as vec_t;
    } else if (*ent).velocity[2 as libc::c_int as usize]
        > 500 as libc::c_int as libc::c_float
    {
        (*ent).velocity[2 as libc::c_int as usize] = 500 as libc::c_int as vec_t;
    }
}
#[no_mangle]
pub unsafe extern "C" fn gib_think(mut self_0: *mut edict_t) {
    let ref mut fresh1 = (*self_0).s.frame;
    *fresh1 += 1;
    (*self_0).nextthink = (level.time as libc::c_double + 0.1f64) as libc::c_float;
    if (*self_0).s.frame == 10 as libc::c_int {
        let ref mut fresh2 = (*self_0).think;
        *fresh2 = Some(G_FreeEdict as unsafe extern "C" fn(*mut edict_t) -> ());
        (*self_0)
            .nextthink = level.time + 8 as libc::c_int as libc::c_float
            + (rand() & 0x7fff as libc::c_int) as libc::c_float
                / 0x7fff as libc::c_int as libc::c_float
                * 10 as libc::c_int as libc::c_float;
    }
}
#[no_mangle]
pub unsafe extern "C" fn gib_touch(
    mut self_0: *mut edict_t,
    mut other: *mut edict_t,
    mut plane: *mut cplane_t,
    mut surf: *mut csurface_t,
) {
    let mut normal_angles: vec3_t = [0.; 3];
    let mut right: vec3_t = [0.; 3];
    if ((*self_0).groundentity).is_null() {
        return;
    }
    let ref mut fresh3 = (*self_0).touch;
    *fresh3 = None;
    if !plane.is_null() {
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
                b"misc/fhit3.wav\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            ),
            1 as libc::c_int as libc::c_float,
            1 as libc::c_int as libc::c_float,
            0 as libc::c_int as libc::c_float,
        );
        vectoangles(((*plane).normal).as_mut_ptr(), normal_angles.as_mut_ptr());
        AngleVectors(
            normal_angles.as_mut_ptr(),
            0 as *mut vec_t,
            right.as_mut_ptr(),
            0 as *mut vec_t,
        );
        vectoangles(right.as_mut_ptr(), ((*self_0).s.angles).as_mut_ptr());
        if (*self_0).s.modelindex == sm_meat_index {
            let ref mut fresh4 = (*self_0).s.frame;
            *fresh4 += 1;
            let ref mut fresh5 = (*self_0).think;
            *fresh5 = Some(gib_think as unsafe extern "C" fn(*mut edict_t) -> ());
            (*self_0)
                .nextthink = (level.time as libc::c_double + 0.1f64) as libc::c_float;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn gib_die(
    mut self_0: *mut edict_t,
    mut inflictor: *mut edict_t,
    mut attacker: *mut edict_t,
    mut damage: libc::c_int,
    mut point: *mut vec_t,
) {
    G_FreeEdict(self_0);
}
#[no_mangle]
pub unsafe extern "C" fn ThrowGib(
    mut self_0: *mut edict_t,
    mut gibname: *mut libc::c_char,
    mut damage: libc::c_int,
    mut type_0: libc::c_int,
) {
    let mut gib: *mut edict_t = 0 as *mut edict_t;
    let mut vd: vec3_t = [0.; 3];
    let mut origin: vec3_t = [0.; 3];
    let mut size: vec3_t = [0.; 3];
    let mut vscale: libc::c_float = 0.;
    gib = G_Spawn();
    VectorScale(((*self_0).size).as_mut_ptr(), 0.5f64 as vec_t, size.as_mut_ptr());
    origin[0 as libc::c_int
        as usize] = (*self_0).absmin[0 as libc::c_int as usize]
        + size[0 as libc::c_int as usize];
    origin[1 as libc::c_int
        as usize] = (*self_0).absmin[1 as libc::c_int as usize]
        + size[1 as libc::c_int as usize];
    origin[2 as libc::c_int
        as usize] = (*self_0).absmin[2 as libc::c_int as usize]
        + size[2 as libc::c_int as usize];
    (*gib)
        .s
        .origin[0 as libc::c_int
        as usize] = (origin[0 as libc::c_int as usize] as libc::c_double
        + 2.0f64
            * (((rand() & 0x7fff as libc::c_int) as libc::c_float
                / 0x7fff as libc::c_int as libc::c_float) as libc::c_double - 0.5f64)
            * size[0 as libc::c_int as usize] as libc::c_double) as vec_t;
    (*gib)
        .s
        .origin[1 as libc::c_int
        as usize] = (origin[1 as libc::c_int as usize] as libc::c_double
        + 2.0f64
            * (((rand() & 0x7fff as libc::c_int) as libc::c_float
                / 0x7fff as libc::c_int as libc::c_float) as libc::c_double - 0.5f64)
            * size[1 as libc::c_int as usize] as libc::c_double) as vec_t;
    (*gib)
        .s
        .origin[2 as libc::c_int
        as usize] = (origin[2 as libc::c_int as usize] as libc::c_double
        + 2.0f64
            * (((rand() & 0x7fff as libc::c_int) as libc::c_float
                / 0x7fff as libc::c_int as libc::c_float) as libc::c_double - 0.5f64)
            * size[2 as libc::c_int as usize] as libc::c_double) as vec_t;
    (gi.setmodel).expect("non-null function pointer")(gib, gibname);
    (*gib).solid = SOLID_NOT;
    (*gib).s.effects |= 0x2 as libc::c_int as libc::c_uint;
    (*gib).flags |= 0x800 as libc::c_int;
    (*gib).takedamage = DAMAGE_YES as libc::c_int;
    let ref mut fresh6 = (*gib).die;
    *fresh6 = Some(
        gib_die
            as unsafe extern "C" fn(
                *mut edict_t,
                *mut edict_t,
                *mut edict_t,
                libc::c_int,
                *mut vec_t,
            ) -> (),
    );
    if type_0 == 0 as libc::c_int {
        (*gib).movetype = MOVETYPE_TOSS as libc::c_int;
        let ref mut fresh7 = (*gib).touch;
        *fresh7 = Some(
            gib_touch
                as unsafe extern "C" fn(
                    *mut edict_t,
                    *mut edict_t,
                    *mut cplane_t,
                    *mut csurface_t,
                ) -> (),
        );
        vscale = 0.5f64 as libc::c_float;
    } else {
        (*gib).movetype = MOVETYPE_BOUNCE as libc::c_int;
        vscale = 1.0f64 as libc::c_float;
    }
    VelocityForDamage(damage, vd.as_mut_ptr());
    VectorMA(
        ((*self_0).velocity).as_mut_ptr(),
        vscale,
        vd.as_mut_ptr(),
        ((*gib).velocity).as_mut_ptr(),
    );
    ClipGibVelocity(gib);
    (*gib)
        .avelocity[0 as libc::c_int
        as usize] = (rand() & 0x7fff as libc::c_int) as libc::c_float
        / 0x7fff as libc::c_int as libc::c_float * 600 as libc::c_int as libc::c_float;
    (*gib)
        .avelocity[1 as libc::c_int
        as usize] = (rand() & 0x7fff as libc::c_int) as libc::c_float
        / 0x7fff as libc::c_int as libc::c_float * 600 as libc::c_int as libc::c_float;
    (*gib)
        .avelocity[2 as libc::c_int
        as usize] = (rand() & 0x7fff as libc::c_int) as libc::c_float
        / 0x7fff as libc::c_int as libc::c_float * 600 as libc::c_int as libc::c_float;
    let ref mut fresh8 = (*gib).think;
    *fresh8 = Some(G_FreeEdict as unsafe extern "C" fn(*mut edict_t) -> ());
    (*gib)
        .nextthink = level.time + 10 as libc::c_int as libc::c_float
        + (rand() & 0x7fff as libc::c_int) as libc::c_float
            / 0x7fff as libc::c_int as libc::c_float
            * 10 as libc::c_int as libc::c_float;
    (gi.linkentity).expect("non-null function pointer")(gib);
}
#[no_mangle]
pub unsafe extern "C" fn ThrowHead(
    mut self_0: *mut edict_t,
    mut gibname: *mut libc::c_char,
    mut damage: libc::c_int,
    mut type_0: libc::c_int,
) {
    let mut vd: vec3_t = [0.; 3];
    let mut vscale: libc::c_float = 0.;
    (*self_0).s.skinnum = 0 as libc::c_int;
    (*self_0).s.frame = 0 as libc::c_int;
    let ref mut fresh9 = (*self_0).mins[2 as libc::c_int as usize];
    *fresh9 = 0 as libc::c_int as vec_t;
    let ref mut fresh10 = (*self_0).mins[1 as libc::c_int as usize];
    *fresh10 = *fresh9;
    (*self_0).mins[0 as libc::c_int as usize] = *fresh10;
    let ref mut fresh11 = (*self_0).maxs[2 as libc::c_int as usize];
    *fresh11 = 0 as libc::c_int as vec_t;
    let ref mut fresh12 = (*self_0).maxs[1 as libc::c_int as usize];
    *fresh12 = *fresh11;
    (*self_0).maxs[0 as libc::c_int as usize] = *fresh12;
    (*self_0).s.modelindex2 = 0 as libc::c_int;
    (gi.setmodel).expect("non-null function pointer")(self_0, gibname);
    (*self_0).solid = SOLID_NOT;
    (*self_0).s.effects |= 0x2 as libc::c_int as libc::c_uint;
    (*self_0).s.effects &= !(0x4000 as libc::c_int) as libc::c_uint;
    (*self_0).s.sound = 0 as libc::c_int;
    (*self_0).flags |= 0x800 as libc::c_int;
    (*self_0).svflags &= !(0x4 as libc::c_int);
    (*self_0).takedamage = DAMAGE_YES as libc::c_int;
    let ref mut fresh13 = (*self_0).die;
    *fresh13 = Some(
        gib_die
            as unsafe extern "C" fn(
                *mut edict_t,
                *mut edict_t,
                *mut edict_t,
                libc::c_int,
                *mut vec_t,
            ) -> (),
    );
    if type_0 == 0 as libc::c_int {
        (*self_0).movetype = MOVETYPE_TOSS as libc::c_int;
        let ref mut fresh14 = (*self_0).touch;
        *fresh14 = Some(
            gib_touch
                as unsafe extern "C" fn(
                    *mut edict_t,
                    *mut edict_t,
                    *mut cplane_t,
                    *mut csurface_t,
                ) -> (),
        );
        vscale = 0.5f64 as libc::c_float;
    } else {
        (*self_0).movetype = MOVETYPE_BOUNCE as libc::c_int;
        vscale = 1.0f64 as libc::c_float;
    }
    VelocityForDamage(damage, vd.as_mut_ptr());
    VectorMA(
        ((*self_0).velocity).as_mut_ptr(),
        vscale,
        vd.as_mut_ptr(),
        ((*self_0).velocity).as_mut_ptr(),
    );
    ClipGibVelocity(self_0);
    (*self_0)
        .avelocity[1 as libc::c_int
        as usize] = (2.0f64
        * (((rand() & 0x7fff as libc::c_int) as libc::c_float
            / 0x7fff as libc::c_int as libc::c_float) as libc::c_double - 0.5f64)
        * 600 as libc::c_int as libc::c_double) as vec_t;
    let ref mut fresh15 = (*self_0).think;
    *fresh15 = Some(G_FreeEdict as unsafe extern "C" fn(*mut edict_t) -> ());
    (*self_0)
        .nextthink = level.time + 10 as libc::c_int as libc::c_float
        + (rand() & 0x7fff as libc::c_int) as libc::c_float
            / 0x7fff as libc::c_int as libc::c_float
            * 10 as libc::c_int as libc::c_float;
    (gi.linkentity).expect("non-null function pointer")(self_0);
}
#[no_mangle]
pub unsafe extern "C" fn ThrowClientHead(
    mut self_0: *mut edict_t,
    mut damage: libc::c_int,
) {
    let mut vd: vec3_t = [0.; 3];
    let mut gibname: *mut libc::c_char = 0 as *mut libc::c_char;
    if rand() & 1 as libc::c_int != 0 {
        gibname = b"models/objects/gibs/head2/tris.md2\0" as *const u8
            as *const libc::c_char as *mut libc::c_char;
        (*self_0).s.skinnum = 1 as libc::c_int;
    } else {
        gibname = b"models/objects/gibs/skull/tris.md2\0" as *const u8
            as *const libc::c_char as *mut libc::c_char;
        (*self_0).s.skinnum = 0 as libc::c_int;
    }
    let ref mut fresh16 = (*self_0).s.origin[2 as libc::c_int as usize];
    *fresh16 += 32 as libc::c_int as libc::c_float;
    (*self_0).s.frame = 0 as libc::c_int;
    (gi.setmodel).expect("non-null function pointer")(self_0, gibname);
    (*self_0).mins[0 as libc::c_int as usize] = -(16 as libc::c_int) as vec_t;
    (*self_0).mins[1 as libc::c_int as usize] = -(16 as libc::c_int) as vec_t;
    (*self_0).mins[2 as libc::c_int as usize] = 0 as libc::c_int as vec_t;
    (*self_0).maxs[0 as libc::c_int as usize] = 16 as libc::c_int as vec_t;
    (*self_0).maxs[1 as libc::c_int as usize] = 16 as libc::c_int as vec_t;
    (*self_0).maxs[2 as libc::c_int as usize] = 16 as libc::c_int as vec_t;
    (*self_0).takedamage = DAMAGE_NO as libc::c_int;
    (*self_0).solid = SOLID_NOT;
    (*self_0).s.effects = 0x2 as libc::c_int as libc::c_uint;
    (*self_0).s.sound = 0 as libc::c_int;
    (*self_0).flags |= 0x800 as libc::c_int;
    (*self_0).movetype = MOVETYPE_BOUNCE as libc::c_int;
    VelocityForDamage(damage, vd.as_mut_ptr());
    (*self_0)
        .velocity[0 as libc::c_int
        as usize] = (*self_0).velocity[0 as libc::c_int as usize]
        + vd[0 as libc::c_int as usize];
    (*self_0)
        .velocity[1 as libc::c_int
        as usize] = (*self_0).velocity[1 as libc::c_int as usize]
        + vd[1 as libc::c_int as usize];
    (*self_0)
        .velocity[2 as libc::c_int
        as usize] = (*self_0).velocity[2 as libc::c_int as usize]
        + vd[2 as libc::c_int as usize];
    if !((*self_0).client).is_null() {
        (*(*self_0).client).anim_priority = 5 as libc::c_int;
        (*(*self_0).client).anim_end = (*self_0).s.frame;
    }
    (gi.linkentity).expect("non-null function pointer")(self_0);
}
#[no_mangle]
pub unsafe extern "C" fn debris_die(
    mut self_0: *mut edict_t,
    mut inflictor: *mut edict_t,
    mut attacker: *mut edict_t,
    mut damage: libc::c_int,
    mut point: *mut vec_t,
) {
    G_FreeEdict(self_0);
}
#[no_mangle]
pub unsafe extern "C" fn ThrowDebris(
    mut self_0: *mut edict_t,
    mut modelname: *mut libc::c_char,
    mut speed: libc::c_float,
    mut origin: *mut vec_t,
) {
    let mut chunk: *mut edict_t = 0 as *mut edict_t;
    let mut v: vec3_t = [0.; 3];
    chunk = G_Spawn();
    (*chunk)
        .s
        .origin[0 as libc::c_int as usize] = *origin.offset(0 as libc::c_int as isize);
    (*chunk)
        .s
        .origin[1 as libc::c_int as usize] = *origin.offset(1 as libc::c_int as isize);
    (*chunk)
        .s
        .origin[2 as libc::c_int as usize] = *origin.offset(2 as libc::c_int as isize);
    (gi.setmodel).expect("non-null function pointer")(chunk, modelname);
    v[0 as libc::c_int
        as usize] = (100 as libc::c_int as libc::c_double
        * (2.0f64
            * (((rand() & 0x7fff as libc::c_int) as libc::c_float
                / 0x7fff as libc::c_int as libc::c_float) as libc::c_double - 0.5f64)))
        as vec_t;
    v[1 as libc::c_int
        as usize] = (100 as libc::c_int as libc::c_double
        * (2.0f64
            * (((rand() & 0x7fff as libc::c_int) as libc::c_float
                / 0x7fff as libc::c_int as libc::c_float) as libc::c_double - 0.5f64)))
        as vec_t;
    v[2 as libc::c_int
        as usize] = (100 as libc::c_int as libc::c_double
        + 100 as libc::c_int as libc::c_double
            * (2.0f64
                * (((rand() & 0x7fff as libc::c_int) as libc::c_float
                    / 0x7fff as libc::c_int as libc::c_float) as libc::c_double
                    - 0.5f64))) as vec_t;
    VectorMA(
        ((*self_0).velocity).as_mut_ptr(),
        speed,
        v.as_mut_ptr(),
        ((*chunk).velocity).as_mut_ptr(),
    );
    (*chunk).movetype = MOVETYPE_BOUNCE as libc::c_int;
    (*chunk).solid = SOLID_NOT;
    (*chunk)
        .avelocity[0 as libc::c_int
        as usize] = (rand() & 0x7fff as libc::c_int) as libc::c_float
        / 0x7fff as libc::c_int as libc::c_float * 600 as libc::c_int as libc::c_float;
    (*chunk)
        .avelocity[1 as libc::c_int
        as usize] = (rand() & 0x7fff as libc::c_int) as libc::c_float
        / 0x7fff as libc::c_int as libc::c_float * 600 as libc::c_int as libc::c_float;
    (*chunk)
        .avelocity[2 as libc::c_int
        as usize] = (rand() & 0x7fff as libc::c_int) as libc::c_float
        / 0x7fff as libc::c_int as libc::c_float * 600 as libc::c_int as libc::c_float;
    let ref mut fresh17 = (*chunk).think;
    *fresh17 = Some(G_FreeEdict as unsafe extern "C" fn(*mut edict_t) -> ());
    (*chunk)
        .nextthink = level.time + 5 as libc::c_int as libc::c_float
        + (rand() & 0x7fff as libc::c_int) as libc::c_float
            / 0x7fff as libc::c_int as libc::c_float * 5 as libc::c_int as libc::c_float;
    (*chunk).s.frame = 0 as libc::c_int;
    (*chunk).flags = 0 as libc::c_int;
    let ref mut fresh18 = (*chunk).classname;
    *fresh18 = b"debris\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    (*chunk).takedamage = DAMAGE_YES as libc::c_int;
    let ref mut fresh19 = (*chunk).die;
    *fresh19 = Some(
        debris_die
            as unsafe extern "C" fn(
                *mut edict_t,
                *mut edict_t,
                *mut edict_t,
                libc::c_int,
                *mut vec_t,
            ) -> (),
    );
    (gi.linkentity).expect("non-null function pointer")(chunk);
}
#[no_mangle]
pub unsafe extern "C" fn BecomeExplosion1(mut self_0: *mut edict_t) {
    if strcmp(
        (*self_0).classname,
        b"item_flag_team1\0" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int
    {
        CTFResetFlag(CTF_TEAM1 as libc::c_int);
        (gi.bprintf)
            .expect(
                "non-null function pointer",
            )(
            2 as libc::c_int,
            b"The %s flag has returned!\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            CTFTeamName(CTF_TEAM1 as libc::c_int),
        );
        return;
    }
    if strcmp(
        (*self_0).classname,
        b"item_flag_team2\0" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int
    {
        CTFResetFlag(CTF_TEAM2 as libc::c_int);
        (gi.bprintf)
            .expect(
                "non-null function pointer",
            )(
            2 as libc::c_int,
            b"The %s flag has returned!\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            CTFTeamName(CTF_TEAM1 as libc::c_int),
        );
        return;
    }
    if !((*self_0).item).is_null() && (*(*self_0).item).flags & 64 as libc::c_int != 0 {
        CTFRespawnTech(self_0);
        return;
    }
    (gi.WriteByte).expect("non-null function pointer")(3 as libc::c_int);
    (gi.WriteByte).expect("non-null function pointer")(TE_EXPLOSION1 as libc::c_int);
    (gi.WritePosition)
        .expect("non-null function pointer")(((*self_0).s.origin).as_mut_ptr());
    (gi.multicast)
        .expect(
            "non-null function pointer",
        )(((*self_0).s.origin).as_mut_ptr(), MULTICAST_PVS);
    G_FreeEdict(self_0);
}
#[no_mangle]
pub unsafe extern "C" fn BecomeExplosion2(mut self_0: *mut edict_t) {
    (gi.WriteByte).expect("non-null function pointer")(3 as libc::c_int);
    (gi.WriteByte).expect("non-null function pointer")(TE_EXPLOSION2 as libc::c_int);
    (gi.WritePosition)
        .expect("non-null function pointer")(((*self_0).s.origin).as_mut_ptr());
    (gi.multicast)
        .expect(
            "non-null function pointer",
        )(((*self_0).s.origin).as_mut_ptr(), MULTICAST_PVS);
    G_FreeEdict(self_0);
}
#[no_mangle]
pub unsafe extern "C" fn path_corner_touch(
    mut self_0: *mut edict_t,
    mut other: *mut edict_t,
    mut plane: *mut cplane_t,
    mut surf: *mut csurface_t,
) {
    let mut v: vec3_t = [0.; 3];
    let mut next: *mut edict_t = 0 as *mut edict_t;
    if (*other).movetarget != self_0 {
        return;
    }
    if !((*other).enemy).is_null() {
        return;
    }
    if !((*self_0).pathtarget).is_null() {
        let mut savetarget: *mut libc::c_char = 0 as *mut libc::c_char;
        savetarget = (*self_0).target;
        let ref mut fresh20 = (*self_0).target;
        *fresh20 = (*self_0).pathtarget;
        G_UseTargets(self_0, other);
        let ref mut fresh21 = (*self_0).target;
        *fresh21 = savetarget;
    }
    if !((*self_0).target).is_null() {
        next = G_PickTarget((*self_0).target);
    } else {
        next = 0 as *mut edict_t;
    }
    if !next.is_null() && (*next).spawnflags & 1 as libc::c_int != 0 {
        v[0 as libc::c_int as usize] = (*next).s.origin[0 as libc::c_int as usize];
        v[1 as libc::c_int as usize] = (*next).s.origin[1 as libc::c_int as usize];
        v[2 as libc::c_int as usize] = (*next).s.origin[2 as libc::c_int as usize];
        v[2 as libc::c_int as usize] += (*next).mins[2 as libc::c_int as usize];
        v[2 as libc::c_int as usize] -= (*other).mins[2 as libc::c_int as usize];
        (*other).s.origin[0 as libc::c_int as usize] = v[0 as libc::c_int as usize];
        (*other).s.origin[1 as libc::c_int as usize] = v[1 as libc::c_int as usize];
        (*other).s.origin[2 as libc::c_int as usize] = v[2 as libc::c_int as usize];
        next = G_PickTarget((*next).target);
    }
    let ref mut fresh22 = (*other).movetarget;
    *fresh22 = next;
    let ref mut fresh23 = (*other).goalentity;
    *fresh23 = *fresh22;
    if (*self_0).wait != 0. {
        (*other).monsterinfo.pausetime = level.time + (*self_0).wait;
        ((*other).monsterinfo.stand).expect("non-null function pointer")(other);
        return;
    }
    if ((*other).movetarget).is_null() {
        (*other)
            .monsterinfo
            .pausetime = level.time + 100000000 as libc::c_int as libc::c_float;
        ((*other).monsterinfo.stand).expect("non-null function pointer")(other);
    } else {
        v[0 as libc::c_int
            as usize] = (*(*other).goalentity).s.origin[0 as libc::c_int as usize]
            - (*other).s.origin[0 as libc::c_int as usize];
        v[1 as libc::c_int
            as usize] = (*(*other).goalentity).s.origin[1 as libc::c_int as usize]
            - (*other).s.origin[1 as libc::c_int as usize];
        v[2 as libc::c_int
            as usize] = (*(*other).goalentity).s.origin[2 as libc::c_int as usize]
            - (*other).s.origin[2 as libc::c_int as usize];
        (*other).ideal_yaw = vectoyaw(v.as_mut_ptr());
    };
}
#[no_mangle]
pub unsafe extern "C" fn SP_path_corner(mut self_0: *mut edict_t) {
    if ((*self_0).targetname).is_null() {
        (gi.dprintf)
            .expect(
                "non-null function pointer",
            )(
            b"path_corner with no targetname at %s\n\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            vtos(((*self_0).s.origin).as_mut_ptr()),
        );
        G_FreeEdict(self_0);
        return;
    }
    (*self_0).solid = SOLID_TRIGGER;
    let ref mut fresh24 = (*self_0).touch;
    *fresh24 = Some(
        path_corner_touch
            as unsafe extern "C" fn(
                *mut edict_t,
                *mut edict_t,
                *mut cplane_t,
                *mut csurface_t,
            ) -> (),
    );
    (*self_0).mins[0 as libc::c_int as usize] = -(8 as libc::c_int) as vec_t;
    (*self_0).mins[1 as libc::c_int as usize] = -(8 as libc::c_int) as vec_t;
    (*self_0).mins[2 as libc::c_int as usize] = -(8 as libc::c_int) as vec_t;
    (*self_0).maxs[0 as libc::c_int as usize] = 8 as libc::c_int as vec_t;
    (*self_0).maxs[1 as libc::c_int as usize] = 8 as libc::c_int as vec_t;
    (*self_0).maxs[2 as libc::c_int as usize] = 8 as libc::c_int as vec_t;
    (*self_0).svflags |= 0x1 as libc::c_int;
    (gi.linkentity).expect("non-null function pointer")(self_0);
}
#[no_mangle]
pub unsafe extern "C" fn point_combat_touch(
    mut self_0: *mut edict_t,
    mut other: *mut edict_t,
    mut plane: *mut cplane_t,
    mut surf: *mut csurface_t,
) {
    let mut activator: *mut edict_t = 0 as *mut edict_t;
    if (*other).movetarget != self_0 {
        return;
    }
    if !((*self_0).target).is_null() {
        let ref mut fresh25 = (*other).target;
        *fresh25 = (*self_0).target;
        let ref mut fresh26 = (*other).movetarget;
        *fresh26 = G_PickTarget((*other).target);
        let ref mut fresh27 = (*other).goalentity;
        *fresh27 = *fresh26;
        if ((*other).goalentity).is_null() {
            (gi.dprintf)
                .expect(
                    "non-null function pointer",
                )(
                b"%s at %s target %s does not exist\n\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                (*self_0).classname,
                vtos(((*self_0).s.origin).as_mut_ptr()),
                (*self_0).target,
            );
            let ref mut fresh28 = (*other).movetarget;
            *fresh28 = self_0;
        }
        let ref mut fresh29 = (*self_0).target;
        *fresh29 = 0 as *mut libc::c_char;
    } else if (*self_0).spawnflags & 1 as libc::c_int != 0
        && (*other).flags & (0x2 as libc::c_int | 0x1 as libc::c_int) == 0
    {
        (*other)
            .monsterinfo
            .pausetime = level.time + 100000000 as libc::c_int as libc::c_float;
        (*other).monsterinfo.aiflags |= 0x1 as libc::c_int;
        ((*other).monsterinfo.stand).expect("non-null function pointer")(other);
    }
    if (*other).movetarget == self_0 {
        let ref mut fresh30 = (*other).target;
        *fresh30 = 0 as *mut libc::c_char;
        let ref mut fresh31 = (*other).movetarget;
        *fresh31 = 0 as *mut edict_t;
        let ref mut fresh32 = (*other).goalentity;
        *fresh32 = (*other).enemy;
        (*other).monsterinfo.aiflags &= !(0x1000 as libc::c_int);
    }
    if !((*self_0).pathtarget).is_null() {
        let mut savetarget: *mut libc::c_char = 0 as *mut libc::c_char;
        savetarget = (*self_0).target;
        let ref mut fresh33 = (*self_0).target;
        *fresh33 = (*self_0).pathtarget;
        if !((*other).enemy).is_null() && !((*(*other).enemy).client).is_null() {
            activator = (*other).enemy;
        } else if !((*other).oldenemy).is_null()
            && !((*(*other).oldenemy).client).is_null()
        {
            activator = (*other).oldenemy;
        } else if !((*other).activator).is_null()
            && !((*(*other).activator).client).is_null()
        {
            activator = (*other).activator;
        } else {
            activator = other;
        }
        G_UseTargets(self_0, activator);
        let ref mut fresh34 = (*self_0).target;
        *fresh34 = savetarget;
    }
}
#[no_mangle]
pub unsafe extern "C" fn SP_point_combat(mut self_0: *mut edict_t) {
    if (*deathmatch).value != 0. {
        G_FreeEdict(self_0);
        return;
    }
    (*self_0).solid = SOLID_TRIGGER;
    let ref mut fresh35 = (*self_0).touch;
    *fresh35 = Some(
        point_combat_touch
            as unsafe extern "C" fn(
                *mut edict_t,
                *mut edict_t,
                *mut cplane_t,
                *mut csurface_t,
            ) -> (),
    );
    (*self_0).mins[0 as libc::c_int as usize] = -(8 as libc::c_int) as vec_t;
    (*self_0).mins[1 as libc::c_int as usize] = -(8 as libc::c_int) as vec_t;
    (*self_0).mins[2 as libc::c_int as usize] = -(16 as libc::c_int) as vec_t;
    (*self_0).maxs[0 as libc::c_int as usize] = 8 as libc::c_int as vec_t;
    (*self_0).maxs[1 as libc::c_int as usize] = 8 as libc::c_int as vec_t;
    (*self_0).maxs[2 as libc::c_int as usize] = 16 as libc::c_int as vec_t;
    (*self_0).svflags = 0x1 as libc::c_int;
    (gi.linkentity).expect("non-null function pointer")(self_0);
}
static mut robotron: [libc::c_int; 4] = [0; 4];
#[no_mangle]
pub unsafe extern "C" fn TH_viewthing(mut ent: *mut edict_t) {
    (*ent).s.frame = ((*ent).s.frame + 1 as libc::c_int) % 7 as libc::c_int;
    (*ent).nextthink = (level.time as libc::c_double + 0.1f64) as libc::c_float;
    if (*ent).spawnflags != 0 {
        if (*ent).s.frame == 0 as libc::c_int {
            (*ent)
                .spawnflags = ((*ent).spawnflags + 1 as libc::c_int) % 4 as libc::c_int
                + 1 as libc::c_int;
            (*ent)
                .s
                .modelindex = robotron[((*ent).spawnflags - 1 as libc::c_int) as usize];
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn SP_viewthing(mut ent: *mut edict_t) {
    (gi.dprintf)
        .expect(
            "non-null function pointer",
        )(
        b"viewthing spawned\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    (*ent).movetype = MOVETYPE_NONE as libc::c_int;
    (*ent).solid = SOLID_BBOX;
    (*ent).s.renderfx = 64 as libc::c_int;
    (*ent).mins[0 as libc::c_int as usize] = -(16 as libc::c_int) as vec_t;
    (*ent).mins[1 as libc::c_int as usize] = -(16 as libc::c_int) as vec_t;
    (*ent).mins[2 as libc::c_int as usize] = -(24 as libc::c_int) as vec_t;
    (*ent).maxs[0 as libc::c_int as usize] = 16 as libc::c_int as vec_t;
    (*ent).maxs[1 as libc::c_int as usize] = 16 as libc::c_int as vec_t;
    (*ent).maxs[2 as libc::c_int as usize] = 32 as libc::c_int as vec_t;
    (*ent)
        .s
        .modelindex = (gi.modelindex)
        .expect(
            "non-null function pointer",
        )(
        b"models/objects/banner/tris.md2\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    (gi.linkentity).expect("non-null function pointer")(ent);
    (*ent).nextthink = (level.time as libc::c_double + 0.5f64) as libc::c_float;
    let ref mut fresh36 = (*ent).think;
    *fresh36 = Some(TH_viewthing as unsafe extern "C" fn(*mut edict_t) -> ());
}
#[no_mangle]
pub unsafe extern "C" fn SP_info_null(mut self_0: *mut edict_t) {
    G_FreeEdict(self_0);
}
#[no_mangle]
pub unsafe extern "C" fn SP_info_notnull(mut self_0: *mut edict_t) {
    (*self_0)
        .absmin[0 as libc::c_int
        as usize] = (*self_0).s.origin[0 as libc::c_int as usize];
    (*self_0)
        .absmin[1 as libc::c_int
        as usize] = (*self_0).s.origin[1 as libc::c_int as usize];
    (*self_0)
        .absmin[2 as libc::c_int
        as usize] = (*self_0).s.origin[2 as libc::c_int as usize];
    (*self_0)
        .absmax[0 as libc::c_int
        as usize] = (*self_0).s.origin[0 as libc::c_int as usize];
    (*self_0)
        .absmax[1 as libc::c_int
        as usize] = (*self_0).s.origin[1 as libc::c_int as usize];
    (*self_0)
        .absmax[2 as libc::c_int
        as usize] = (*self_0).s.origin[2 as libc::c_int as usize];
}
unsafe extern "C" fn light_use(
    mut self_0: *mut edict_t,
    mut other: *mut edict_t,
    mut activator: *mut edict_t,
) {
    if (*self_0).spawnflags & 1 as libc::c_int != 0 {
        (gi.configstring)
            .expect(
                "non-null function pointer",
            )(
            32 as libc::c_int + 256 as libc::c_int + 256 as libc::c_int
                + 256 as libc::c_int + (*self_0).style,
            b"m\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        (*self_0).spawnflags &= !(1 as libc::c_int);
    } else {
        (gi.configstring)
            .expect(
                "non-null function pointer",
            )(
            32 as libc::c_int + 256 as libc::c_int + 256 as libc::c_int
                + 256 as libc::c_int + (*self_0).style,
            b"a\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        (*self_0).spawnflags |= 1 as libc::c_int;
    };
}
#[no_mangle]
pub unsafe extern "C" fn SP_light(mut self_0: *mut edict_t) {
    if ((*self_0).targetname).is_null() || (*deathmatch).value != 0. {
        G_FreeEdict(self_0);
        return;
    }
    if (*self_0).style >= 32 as libc::c_int {
        let ref mut fresh37 = (*self_0).use_0;
        *fresh37 = Some(
            light_use
                as unsafe extern "C" fn(*mut edict_t, *mut edict_t, *mut edict_t) -> (),
        );
        if (*self_0).spawnflags & 1 as libc::c_int != 0 {
            (gi.configstring)
                .expect(
                    "non-null function pointer",
                )(
                32 as libc::c_int + 256 as libc::c_int + 256 as libc::c_int
                    + 256 as libc::c_int + (*self_0).style,
                b"a\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
        } else {
            (gi.configstring)
                .expect(
                    "non-null function pointer",
                )(
                32 as libc::c_int + 256 as libc::c_int + 256 as libc::c_int
                    + 256 as libc::c_int + (*self_0).style,
                b"m\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn func_wall_use(
    mut self_0: *mut edict_t,
    mut other: *mut edict_t,
    mut activator: *mut edict_t,
) {
    if (*self_0).solid as libc::c_uint == SOLID_NOT as libc::c_int as libc::c_uint {
        (*self_0).solid = SOLID_BSP;
        (*self_0).svflags &= !(0x1 as libc::c_int);
        KillBox(self_0);
    } else {
        (*self_0).solid = SOLID_NOT;
        (*self_0).svflags |= 0x1 as libc::c_int;
    }
    (gi.linkentity).expect("non-null function pointer")(self_0);
    if (*self_0).spawnflags & 2 as libc::c_int == 0 {
        let ref mut fresh38 = (*self_0).use_0;
        *fresh38 = None;
    }
}
#[no_mangle]
pub unsafe extern "C" fn SP_func_wall(mut self_0: *mut edict_t) {
    (*self_0).movetype = MOVETYPE_PUSH as libc::c_int;
    (gi.setmodel).expect("non-null function pointer")(self_0, (*self_0).model);
    if (*self_0).spawnflags & 8 as libc::c_int != 0 {
        (*self_0).s.effects |= 0x1000 as libc::c_int as libc::c_uint;
    }
    if (*self_0).spawnflags & 16 as libc::c_int != 0 {
        (*self_0).s.effects |= 0x2000 as libc::c_int as libc::c_uint;
    }
    if (*self_0).spawnflags & 7 as libc::c_int == 0 as libc::c_int {
        (*self_0).solid = SOLID_BSP;
        (gi.linkentity).expect("non-null function pointer")(self_0);
        return;
    }
    if (*self_0).spawnflags & 1 as libc::c_int == 0 {
        (*self_0).spawnflags |= 1 as libc::c_int;
    }
    if (*self_0).spawnflags & 4 as libc::c_int != 0 {
        if (*self_0).spawnflags & 2 as libc::c_int == 0 {
            (gi.dprintf)
                .expect(
                    "non-null function pointer",
                )(
                b"func_wall START_ON without TOGGLE\n\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
            );
            (*self_0).spawnflags |= 2 as libc::c_int;
        }
    }
    let ref mut fresh39 = (*self_0).use_0;
    *fresh39 = Some(
        func_wall_use
            as unsafe extern "C" fn(*mut edict_t, *mut edict_t, *mut edict_t) -> (),
    );
    if (*self_0).spawnflags & 4 as libc::c_int != 0 {
        (*self_0).solid = SOLID_BSP;
    } else {
        (*self_0).solid = SOLID_NOT;
        (*self_0).svflags |= 0x1 as libc::c_int;
    }
    (gi.linkentity).expect("non-null function pointer")(self_0);
}
#[no_mangle]
pub unsafe extern "C" fn func_object_touch(
    mut self_0: *mut edict_t,
    mut other: *mut edict_t,
    mut plane: *mut cplane_t,
    mut surf: *mut csurface_t,
) {
    if plane.is_null() {
        return;
    }
    if ((*plane).normal[2 as libc::c_int as usize] as libc::c_double) < 1.0f64 {
        return;
    }
    if (*other).takedamage == DAMAGE_NO as libc::c_int {
        return;
    }
    T_Damage(
        other,
        self_0,
        self_0,
        vec3_origin.as_mut_ptr(),
        ((*self_0).s.origin).as_mut_ptr(),
        vec3_origin.as_mut_ptr(),
        (*self_0).dmg,
        1 as libc::c_int,
        0 as libc::c_int,
        20 as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn func_object_release(mut self_0: *mut edict_t) {
    (*self_0).movetype = MOVETYPE_TOSS as libc::c_int;
    let ref mut fresh40 = (*self_0).touch;
    *fresh40 = Some(
        func_object_touch
            as unsafe extern "C" fn(
                *mut edict_t,
                *mut edict_t,
                *mut cplane_t,
                *mut csurface_t,
            ) -> (),
    );
}
#[no_mangle]
pub unsafe extern "C" fn func_object_use(
    mut self_0: *mut edict_t,
    mut other: *mut edict_t,
    mut activator: *mut edict_t,
) {
    (*self_0).solid = SOLID_BSP;
    (*self_0).svflags &= !(0x1 as libc::c_int);
    let ref mut fresh41 = (*self_0).use_0;
    *fresh41 = None;
    KillBox(self_0);
    func_object_release(self_0);
}
#[no_mangle]
pub unsafe extern "C" fn SP_func_object(mut self_0: *mut edict_t) {
    (gi.setmodel).expect("non-null function pointer")(self_0, (*self_0).model);
    let ref mut fresh42 = (*self_0).mins[0 as libc::c_int as usize];
    *fresh42 += 1 as libc::c_int as libc::c_float;
    let ref mut fresh43 = (*self_0).mins[1 as libc::c_int as usize];
    *fresh43 += 1 as libc::c_int as libc::c_float;
    let ref mut fresh44 = (*self_0).mins[2 as libc::c_int as usize];
    *fresh44 += 1 as libc::c_int as libc::c_float;
    let ref mut fresh45 = (*self_0).maxs[0 as libc::c_int as usize];
    *fresh45 -= 1 as libc::c_int as libc::c_float;
    let ref mut fresh46 = (*self_0).maxs[1 as libc::c_int as usize];
    *fresh46 -= 1 as libc::c_int as libc::c_float;
    let ref mut fresh47 = (*self_0).maxs[2 as libc::c_int as usize];
    *fresh47 -= 1 as libc::c_int as libc::c_float;
    if (*self_0).dmg == 0 {
        (*self_0).dmg = 100 as libc::c_int;
    }
    if (*self_0).spawnflags == 0 as libc::c_int {
        (*self_0).solid = SOLID_BSP;
        (*self_0).movetype = MOVETYPE_PUSH as libc::c_int;
        let ref mut fresh48 = (*self_0).think;
        *fresh48 = Some(func_object_release as unsafe extern "C" fn(*mut edict_t) -> ());
        (*self_0)
            .nextthink = (level.time as libc::c_double
            + 2 as libc::c_int as libc::c_double * 0.1f64) as libc::c_float;
    } else {
        (*self_0).solid = SOLID_NOT;
        (*self_0).movetype = MOVETYPE_PUSH as libc::c_int;
        let ref mut fresh49 = (*self_0).use_0;
        *fresh49 = Some(
            func_object_use
                as unsafe extern "C" fn(*mut edict_t, *mut edict_t, *mut edict_t) -> (),
        );
        (*self_0).svflags |= 0x1 as libc::c_int;
    }
    if (*self_0).spawnflags & 2 as libc::c_int != 0 {
        (*self_0).s.effects |= 0x1000 as libc::c_int as libc::c_uint;
    }
    if (*self_0).spawnflags & 4 as libc::c_int != 0 {
        (*self_0).s.effects |= 0x2000 as libc::c_int as libc::c_uint;
    }
    (*self_0)
        .clipmask = 1 as libc::c_int | 0x20000 as libc::c_int | 2 as libc::c_int
        | 0x2000000 as libc::c_int;
    (gi.linkentity).expect("non-null function pointer")(self_0);
}
#[no_mangle]
pub unsafe extern "C" fn func_explosive_explode(
    mut self_0: *mut edict_t,
    mut inflictor: *mut edict_t,
    mut attacker: *mut edict_t,
    mut damage: libc::c_int,
    mut point: *mut vec_t,
) {
    let mut origin: vec3_t = [0.; 3];
    let mut chunkorigin: vec3_t = [0.; 3];
    let mut size: vec3_t = [0.; 3];
    let mut count: libc::c_int = 0;
    let mut mass: libc::c_int = 0;
    VectorScale(((*self_0).size).as_mut_ptr(), 0.5f64 as vec_t, size.as_mut_ptr());
    origin[0 as libc::c_int
        as usize] = (*self_0).absmin[0 as libc::c_int as usize]
        + size[0 as libc::c_int as usize];
    origin[1 as libc::c_int
        as usize] = (*self_0).absmin[1 as libc::c_int as usize]
        + size[1 as libc::c_int as usize];
    origin[2 as libc::c_int
        as usize] = (*self_0).absmin[2 as libc::c_int as usize]
        + size[2 as libc::c_int as usize];
    (*self_0).s.origin[0 as libc::c_int as usize] = origin[0 as libc::c_int as usize];
    (*self_0).s.origin[1 as libc::c_int as usize] = origin[1 as libc::c_int as usize];
    (*self_0).s.origin[2 as libc::c_int as usize] = origin[2 as libc::c_int as usize];
    (*self_0).takedamage = DAMAGE_NO as libc::c_int;
    if (*self_0).dmg != 0 {
        T_RadiusDamage(
            self_0,
            attacker,
            (*self_0).dmg as libc::c_float,
            0 as *mut edict_t,
            ((*self_0).dmg + 40 as libc::c_int) as libc::c_float,
            25 as libc::c_int,
        );
    }
    (*self_0)
        .velocity[0 as libc::c_int
        as usize] = (*self_0).s.origin[0 as libc::c_int as usize]
        - (*inflictor).s.origin[0 as libc::c_int as usize];
    (*self_0)
        .velocity[1 as libc::c_int
        as usize] = (*self_0).s.origin[1 as libc::c_int as usize]
        - (*inflictor).s.origin[1 as libc::c_int as usize];
    (*self_0)
        .velocity[2 as libc::c_int
        as usize] = (*self_0).s.origin[2 as libc::c_int as usize]
        - (*inflictor).s.origin[2 as libc::c_int as usize];
    VectorNormalize(((*self_0).velocity).as_mut_ptr());
    VectorScale(
        ((*self_0).velocity).as_mut_ptr(),
        150 as libc::c_int as vec_t,
        ((*self_0).velocity).as_mut_ptr(),
    );
    VectorScale(size.as_mut_ptr(), 0.5f64 as vec_t, size.as_mut_ptr());
    mass = (*self_0).mass;
    if mass == 0 {
        mass = 75 as libc::c_int;
    }
    if mass >= 100 as libc::c_int {
        count = mass / 100 as libc::c_int;
        if count > 8 as libc::c_int {
            count = 8 as libc::c_int;
        }
        loop {
            let fresh50 = count;
            count = count - 1;
            if !(fresh50 != 0) {
                break;
            }
            chunkorigin[0 as libc::c_int
                as usize] = (origin[0 as libc::c_int as usize] as libc::c_double
                + 2.0f64
                    * (((rand() & 0x7fff as libc::c_int) as libc::c_float
                        / 0x7fff as libc::c_int as libc::c_float) as libc::c_double
                        - 0.5f64) * size[0 as libc::c_int as usize] as libc::c_double)
                as vec_t;
            chunkorigin[1 as libc::c_int
                as usize] = (origin[1 as libc::c_int as usize] as libc::c_double
                + 2.0f64
                    * (((rand() & 0x7fff as libc::c_int) as libc::c_float
                        / 0x7fff as libc::c_int as libc::c_float) as libc::c_double
                        - 0.5f64) * size[1 as libc::c_int as usize] as libc::c_double)
                as vec_t;
            chunkorigin[2 as libc::c_int
                as usize] = (origin[2 as libc::c_int as usize] as libc::c_double
                + 2.0f64
                    * (((rand() & 0x7fff as libc::c_int) as libc::c_float
                        / 0x7fff as libc::c_int as libc::c_float) as libc::c_double
                        - 0.5f64) * size[2 as libc::c_int as usize] as libc::c_double)
                as vec_t;
            ThrowDebris(
                self_0,
                b"models/objects/debris1/tris.md2\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                1 as libc::c_int as libc::c_float,
                chunkorigin.as_mut_ptr(),
            );
        }
    }
    count = mass / 25 as libc::c_int;
    if count > 16 as libc::c_int {
        count = 16 as libc::c_int;
    }
    loop {
        let fresh51 = count;
        count = count - 1;
        if !(fresh51 != 0) {
            break;
        }
        chunkorigin[0 as libc::c_int
            as usize] = (origin[0 as libc::c_int as usize] as libc::c_double
            + 2.0f64
                * (((rand() & 0x7fff as libc::c_int) as libc::c_float
                    / 0x7fff as libc::c_int as libc::c_float) as libc::c_double - 0.5f64)
                * size[0 as libc::c_int as usize] as libc::c_double) as vec_t;
        chunkorigin[1 as libc::c_int
            as usize] = (origin[1 as libc::c_int as usize] as libc::c_double
            + 2.0f64
                * (((rand() & 0x7fff as libc::c_int) as libc::c_float
                    / 0x7fff as libc::c_int as libc::c_float) as libc::c_double - 0.5f64)
                * size[1 as libc::c_int as usize] as libc::c_double) as vec_t;
        chunkorigin[2 as libc::c_int
            as usize] = (origin[2 as libc::c_int as usize] as libc::c_double
            + 2.0f64
                * (((rand() & 0x7fff as libc::c_int) as libc::c_float
                    / 0x7fff as libc::c_int as libc::c_float) as libc::c_double - 0.5f64)
                * size[2 as libc::c_int as usize] as libc::c_double) as vec_t;
        ThrowDebris(
            self_0,
            b"models/objects/debris2/tris.md2\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            2 as libc::c_int as libc::c_float,
            chunkorigin.as_mut_ptr(),
        );
    }
    G_UseTargets(self_0, attacker);
    if (*self_0).dmg != 0 {
        BecomeExplosion1(self_0);
    } else {
        G_FreeEdict(self_0);
    };
}
#[no_mangle]
pub unsafe extern "C" fn func_explosive_use(
    mut self_0: *mut edict_t,
    mut other: *mut edict_t,
    mut activator: *mut edict_t,
) {
    func_explosive_explode(
        self_0,
        self_0,
        other,
        (*self_0).health,
        vec3_origin.as_mut_ptr(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn func_explosive_spawn(
    mut self_0: *mut edict_t,
    mut other: *mut edict_t,
    mut activator: *mut edict_t,
) {
    (*self_0).solid = SOLID_BSP;
    (*self_0).svflags &= !(0x1 as libc::c_int);
    let ref mut fresh52 = (*self_0).use_0;
    *fresh52 = None;
    KillBox(self_0);
    (gi.linkentity).expect("non-null function pointer")(self_0);
}
#[no_mangle]
pub unsafe extern "C" fn SP_func_explosive(mut self_0: *mut edict_t) {
    if (*deathmatch).value != 0. {
        G_FreeEdict(self_0);
        return;
    }
    (*self_0).movetype = MOVETYPE_PUSH as libc::c_int;
    (gi.modelindex)
        .expect(
            "non-null function pointer",
        )(
        b"models/objects/debris1/tris.md2\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    (gi.modelindex)
        .expect(
            "non-null function pointer",
        )(
        b"models/objects/debris2/tris.md2\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    (gi.setmodel).expect("non-null function pointer")(self_0, (*self_0).model);
    if (*self_0).spawnflags & 1 as libc::c_int != 0 {
        (*self_0).svflags |= 0x1 as libc::c_int;
        (*self_0).solid = SOLID_NOT;
        let ref mut fresh53 = (*self_0).use_0;
        *fresh53 = Some(
            func_explosive_spawn
                as unsafe extern "C" fn(*mut edict_t, *mut edict_t, *mut edict_t) -> (),
        );
    } else {
        (*self_0).solid = SOLID_BSP;
        if !((*self_0).targetname).is_null() {
            let ref mut fresh54 = (*self_0).use_0;
            *fresh54 = Some(
                func_explosive_use
                    as unsafe extern "C" fn(
                        *mut edict_t,
                        *mut edict_t,
                        *mut edict_t,
                    ) -> (),
            );
        }
    }
    if (*self_0).spawnflags & 2 as libc::c_int != 0 {
        (*self_0).s.effects |= 0x1000 as libc::c_int as libc::c_uint;
    }
    if (*self_0).spawnflags & 4 as libc::c_int != 0 {
        (*self_0).s.effects |= 0x2000 as libc::c_int as libc::c_uint;
    }
    if (*self_0).use_0
        != Some(
            func_explosive_use
                as unsafe extern "C" fn(*mut edict_t, *mut edict_t, *mut edict_t) -> (),
        )
    {
        if (*self_0).health == 0 {
            (*self_0).health = 100 as libc::c_int;
        }
        let ref mut fresh55 = (*self_0).die;
        *fresh55 = Some(
            func_explosive_explode
                as unsafe extern "C" fn(
                    *mut edict_t,
                    *mut edict_t,
                    *mut edict_t,
                    libc::c_int,
                    *mut vec_t,
                ) -> (),
        );
        (*self_0).takedamage = DAMAGE_YES as libc::c_int;
    }
    (gi.linkentity).expect("non-null function pointer")(self_0);
}
#[no_mangle]
pub unsafe extern "C" fn barrel_touch(
    mut self_0: *mut edict_t,
    mut other: *mut edict_t,
    mut plane: *mut cplane_t,
    mut surf: *mut csurface_t,
) {
    let mut ratio: libc::c_float = 0.;
    let mut v: vec3_t = [0.; 3];
    if ((*other).groundentity).is_null() || (*other).groundentity == self_0 {
        return;
    }
    ratio = (*other).mass as libc::c_float / (*self_0).mass as libc::c_float;
    v[0 as libc::c_int
        as usize] = (*self_0).s.origin[0 as libc::c_int as usize]
        - (*other).s.origin[0 as libc::c_int as usize];
    v[1 as libc::c_int
        as usize] = (*self_0).s.origin[1 as libc::c_int as usize]
        - (*other).s.origin[1 as libc::c_int as usize];
    v[2 as libc::c_int
        as usize] = (*self_0).s.origin[2 as libc::c_int as usize]
        - (*other).s.origin[2 as libc::c_int as usize];
    M_walkmove(
        self_0,
        vectoyaw(v.as_mut_ptr()),
        ((20 as libc::c_int as libc::c_float * ratio) as libc::c_double * 0.1f64)
            as libc::c_float,
    );
}
#[no_mangle]
pub unsafe extern "C" fn barrel_explode(mut self_0: *mut edict_t) {
    let mut org: vec3_t = [0.; 3];
    let mut spd: libc::c_float = 0.;
    let mut save: vec3_t = [0.; 3];
    T_RadiusDamage(
        self_0,
        (*self_0).activator,
        (*self_0).dmg as libc::c_float,
        0 as *mut edict_t,
        ((*self_0).dmg + 40 as libc::c_int) as libc::c_float,
        26 as libc::c_int,
    );
    save[0 as libc::c_int as usize] = (*self_0).s.origin[0 as libc::c_int as usize];
    save[1 as libc::c_int as usize] = (*self_0).s.origin[1 as libc::c_int as usize];
    save[2 as libc::c_int as usize] = (*self_0).s.origin[2 as libc::c_int as usize];
    VectorMA(
        ((*self_0).absmin).as_mut_ptr(),
        0.5f64 as libc::c_float,
        ((*self_0).size).as_mut_ptr(),
        ((*self_0).s.origin).as_mut_ptr(),
    );
    spd = (1.5f64 * (*self_0).dmg as libc::c_float as libc::c_double / 200.0f64)
        as libc::c_float;
    org[0 as libc::c_int
        as usize] = ((*self_0).s.origin[0 as libc::c_int as usize] as libc::c_double
        + 2.0f64
            * (((rand() & 0x7fff as libc::c_int) as libc::c_float
                / 0x7fff as libc::c_int as libc::c_float) as libc::c_double - 0.5f64)
            * (*self_0).size[0 as libc::c_int as usize] as libc::c_double) as vec_t;
    org[1 as libc::c_int
        as usize] = ((*self_0).s.origin[1 as libc::c_int as usize] as libc::c_double
        + 2.0f64
            * (((rand() & 0x7fff as libc::c_int) as libc::c_float
                / 0x7fff as libc::c_int as libc::c_float) as libc::c_double - 0.5f64)
            * (*self_0).size[1 as libc::c_int as usize] as libc::c_double) as vec_t;
    org[2 as libc::c_int
        as usize] = ((*self_0).s.origin[2 as libc::c_int as usize] as libc::c_double
        + 2.0f64
            * (((rand() & 0x7fff as libc::c_int) as libc::c_float
                / 0x7fff as libc::c_int as libc::c_float) as libc::c_double - 0.5f64)
            * (*self_0).size[2 as libc::c_int as usize] as libc::c_double) as vec_t;
    ThrowDebris(
        self_0,
        b"models/objects/debris1/tris.md2\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        spd,
        org.as_mut_ptr(),
    );
    org[0 as libc::c_int
        as usize] = ((*self_0).s.origin[0 as libc::c_int as usize] as libc::c_double
        + 2.0f64
            * (((rand() & 0x7fff as libc::c_int) as libc::c_float
                / 0x7fff as libc::c_int as libc::c_float) as libc::c_double - 0.5f64)
            * (*self_0).size[0 as libc::c_int as usize] as libc::c_double) as vec_t;
    org[1 as libc::c_int
        as usize] = ((*self_0).s.origin[1 as libc::c_int as usize] as libc::c_double
        + 2.0f64
            * (((rand() & 0x7fff as libc::c_int) as libc::c_float
                / 0x7fff as libc::c_int as libc::c_float) as libc::c_double - 0.5f64)
            * (*self_0).size[1 as libc::c_int as usize] as libc::c_double) as vec_t;
    org[2 as libc::c_int
        as usize] = ((*self_0).s.origin[2 as libc::c_int as usize] as libc::c_double
        + 2.0f64
            * (((rand() & 0x7fff as libc::c_int) as libc::c_float
                / 0x7fff as libc::c_int as libc::c_float) as libc::c_double - 0.5f64)
            * (*self_0).size[2 as libc::c_int as usize] as libc::c_double) as vec_t;
    ThrowDebris(
        self_0,
        b"models/objects/debris1/tris.md2\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        spd,
        org.as_mut_ptr(),
    );
    spd = (1.75f64 * (*self_0).dmg as libc::c_float as libc::c_double / 200.0f64)
        as libc::c_float;
    org[0 as libc::c_int as usize] = (*self_0).absmin[0 as libc::c_int as usize];
    org[1 as libc::c_int as usize] = (*self_0).absmin[1 as libc::c_int as usize];
    org[2 as libc::c_int as usize] = (*self_0).absmin[2 as libc::c_int as usize];
    ThrowDebris(
        self_0,
        b"models/objects/debris3/tris.md2\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        spd,
        org.as_mut_ptr(),
    );
    org[0 as libc::c_int as usize] = (*self_0).absmin[0 as libc::c_int as usize];
    org[1 as libc::c_int as usize] = (*self_0).absmin[1 as libc::c_int as usize];
    org[2 as libc::c_int as usize] = (*self_0).absmin[2 as libc::c_int as usize];
    org[0 as libc::c_int as usize] += (*self_0).size[0 as libc::c_int as usize];
    ThrowDebris(
        self_0,
        b"models/objects/debris3/tris.md2\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        spd,
        org.as_mut_ptr(),
    );
    org[0 as libc::c_int as usize] = (*self_0).absmin[0 as libc::c_int as usize];
    org[1 as libc::c_int as usize] = (*self_0).absmin[1 as libc::c_int as usize];
    org[2 as libc::c_int as usize] = (*self_0).absmin[2 as libc::c_int as usize];
    org[1 as libc::c_int as usize] += (*self_0).size[1 as libc::c_int as usize];
    ThrowDebris(
        self_0,
        b"models/objects/debris3/tris.md2\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        spd,
        org.as_mut_ptr(),
    );
    org[0 as libc::c_int as usize] = (*self_0).absmin[0 as libc::c_int as usize];
    org[1 as libc::c_int as usize] = (*self_0).absmin[1 as libc::c_int as usize];
    org[2 as libc::c_int as usize] = (*self_0).absmin[2 as libc::c_int as usize];
    org[0 as libc::c_int as usize] += (*self_0).size[0 as libc::c_int as usize];
    org[1 as libc::c_int as usize] += (*self_0).size[1 as libc::c_int as usize];
    ThrowDebris(
        self_0,
        b"models/objects/debris3/tris.md2\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        spd,
        org.as_mut_ptr(),
    );
    spd = (2 as libc::c_int * (*self_0).dmg / 200 as libc::c_int) as libc::c_float;
    org[0 as libc::c_int
        as usize] = ((*self_0).s.origin[0 as libc::c_int as usize] as libc::c_double
        + 2.0f64
            * (((rand() & 0x7fff as libc::c_int) as libc::c_float
                / 0x7fff as libc::c_int as libc::c_float) as libc::c_double - 0.5f64)
            * (*self_0).size[0 as libc::c_int as usize] as libc::c_double) as vec_t;
    org[1 as libc::c_int
        as usize] = ((*self_0).s.origin[1 as libc::c_int as usize] as libc::c_double
        + 2.0f64
            * (((rand() & 0x7fff as libc::c_int) as libc::c_float
                / 0x7fff as libc::c_int as libc::c_float) as libc::c_double - 0.5f64)
            * (*self_0).size[1 as libc::c_int as usize] as libc::c_double) as vec_t;
    org[2 as libc::c_int
        as usize] = ((*self_0).s.origin[2 as libc::c_int as usize] as libc::c_double
        + 2.0f64
            * (((rand() & 0x7fff as libc::c_int) as libc::c_float
                / 0x7fff as libc::c_int as libc::c_float) as libc::c_double - 0.5f64)
            * (*self_0).size[2 as libc::c_int as usize] as libc::c_double) as vec_t;
    ThrowDebris(
        self_0,
        b"models/objects/debris2/tris.md2\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        spd,
        org.as_mut_ptr(),
    );
    org[0 as libc::c_int
        as usize] = ((*self_0).s.origin[0 as libc::c_int as usize] as libc::c_double
        + 2.0f64
            * (((rand() & 0x7fff as libc::c_int) as libc::c_float
                / 0x7fff as libc::c_int as libc::c_float) as libc::c_double - 0.5f64)
            * (*self_0).size[0 as libc::c_int as usize] as libc::c_double) as vec_t;
    org[1 as libc::c_int
        as usize] = ((*self_0).s.origin[1 as libc::c_int as usize] as libc::c_double
        + 2.0f64
            * (((rand() & 0x7fff as libc::c_int) as libc::c_float
                / 0x7fff as libc::c_int as libc::c_float) as libc::c_double - 0.5f64)
            * (*self_0).size[1 as libc::c_int as usize] as libc::c_double) as vec_t;
    org[2 as libc::c_int
        as usize] = ((*self_0).s.origin[2 as libc::c_int as usize] as libc::c_double
        + 2.0f64
            * (((rand() & 0x7fff as libc::c_int) as libc::c_float
                / 0x7fff as libc::c_int as libc::c_float) as libc::c_double - 0.5f64)
            * (*self_0).size[2 as libc::c_int as usize] as libc::c_double) as vec_t;
    ThrowDebris(
        self_0,
        b"models/objects/debris2/tris.md2\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        spd,
        org.as_mut_ptr(),
    );
    org[0 as libc::c_int
        as usize] = ((*self_0).s.origin[0 as libc::c_int as usize] as libc::c_double
        + 2.0f64
            * (((rand() & 0x7fff as libc::c_int) as libc::c_float
                / 0x7fff as libc::c_int as libc::c_float) as libc::c_double - 0.5f64)
            * (*self_0).size[0 as libc::c_int as usize] as libc::c_double) as vec_t;
    org[1 as libc::c_int
        as usize] = ((*self_0).s.origin[1 as libc::c_int as usize] as libc::c_double
        + 2.0f64
            * (((rand() & 0x7fff as libc::c_int) as libc::c_float
                / 0x7fff as libc::c_int as libc::c_float) as libc::c_double - 0.5f64)
            * (*self_0).size[1 as libc::c_int as usize] as libc::c_double) as vec_t;
    org[2 as libc::c_int
        as usize] = ((*self_0).s.origin[2 as libc::c_int as usize] as libc::c_double
        + 2.0f64
            * (((rand() & 0x7fff as libc::c_int) as libc::c_float
                / 0x7fff as libc::c_int as libc::c_float) as libc::c_double - 0.5f64)
            * (*self_0).size[2 as libc::c_int as usize] as libc::c_double) as vec_t;
    ThrowDebris(
        self_0,
        b"models/objects/debris2/tris.md2\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        spd,
        org.as_mut_ptr(),
    );
    org[0 as libc::c_int
        as usize] = ((*self_0).s.origin[0 as libc::c_int as usize] as libc::c_double
        + 2.0f64
            * (((rand() & 0x7fff as libc::c_int) as libc::c_float
                / 0x7fff as libc::c_int as libc::c_float) as libc::c_double - 0.5f64)
            * (*self_0).size[0 as libc::c_int as usize] as libc::c_double) as vec_t;
    org[1 as libc::c_int
        as usize] = ((*self_0).s.origin[1 as libc::c_int as usize] as libc::c_double
        + 2.0f64
            * (((rand() & 0x7fff as libc::c_int) as libc::c_float
                / 0x7fff as libc::c_int as libc::c_float) as libc::c_double - 0.5f64)
            * (*self_0).size[1 as libc::c_int as usize] as libc::c_double) as vec_t;
    org[2 as libc::c_int
        as usize] = ((*self_0).s.origin[2 as libc::c_int as usize] as libc::c_double
        + 2.0f64
            * (((rand() & 0x7fff as libc::c_int) as libc::c_float
                / 0x7fff as libc::c_int as libc::c_float) as libc::c_double - 0.5f64)
            * (*self_0).size[2 as libc::c_int as usize] as libc::c_double) as vec_t;
    ThrowDebris(
        self_0,
        b"models/objects/debris2/tris.md2\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        spd,
        org.as_mut_ptr(),
    );
    org[0 as libc::c_int
        as usize] = ((*self_0).s.origin[0 as libc::c_int as usize] as libc::c_double
        + 2.0f64
            * (((rand() & 0x7fff as libc::c_int) as libc::c_float
                / 0x7fff as libc::c_int as libc::c_float) as libc::c_double - 0.5f64)
            * (*self_0).size[0 as libc::c_int as usize] as libc::c_double) as vec_t;
    org[1 as libc::c_int
        as usize] = ((*self_0).s.origin[1 as libc::c_int as usize] as libc::c_double
        + 2.0f64
            * (((rand() & 0x7fff as libc::c_int) as libc::c_float
                / 0x7fff as libc::c_int as libc::c_float) as libc::c_double - 0.5f64)
            * (*self_0).size[1 as libc::c_int as usize] as libc::c_double) as vec_t;
    org[2 as libc::c_int
        as usize] = ((*self_0).s.origin[2 as libc::c_int as usize] as libc::c_double
        + 2.0f64
            * (((rand() & 0x7fff as libc::c_int) as libc::c_float
                / 0x7fff as libc::c_int as libc::c_float) as libc::c_double - 0.5f64)
            * (*self_0).size[2 as libc::c_int as usize] as libc::c_double) as vec_t;
    ThrowDebris(
        self_0,
        b"models/objects/debris2/tris.md2\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        spd,
        org.as_mut_ptr(),
    );
    org[0 as libc::c_int
        as usize] = ((*self_0).s.origin[0 as libc::c_int as usize] as libc::c_double
        + 2.0f64
            * (((rand() & 0x7fff as libc::c_int) as libc::c_float
                / 0x7fff as libc::c_int as libc::c_float) as libc::c_double - 0.5f64)
            * (*self_0).size[0 as libc::c_int as usize] as libc::c_double) as vec_t;
    org[1 as libc::c_int
        as usize] = ((*self_0).s.origin[1 as libc::c_int as usize] as libc::c_double
        + 2.0f64
            * (((rand() & 0x7fff as libc::c_int) as libc::c_float
                / 0x7fff as libc::c_int as libc::c_float) as libc::c_double - 0.5f64)
            * (*self_0).size[1 as libc::c_int as usize] as libc::c_double) as vec_t;
    org[2 as libc::c_int
        as usize] = ((*self_0).s.origin[2 as libc::c_int as usize] as libc::c_double
        + 2.0f64
            * (((rand() & 0x7fff as libc::c_int) as libc::c_float
                / 0x7fff as libc::c_int as libc::c_float) as libc::c_double - 0.5f64)
            * (*self_0).size[2 as libc::c_int as usize] as libc::c_double) as vec_t;
    ThrowDebris(
        self_0,
        b"models/objects/debris2/tris.md2\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        spd,
        org.as_mut_ptr(),
    );
    org[0 as libc::c_int
        as usize] = ((*self_0).s.origin[0 as libc::c_int as usize] as libc::c_double
        + 2.0f64
            * (((rand() & 0x7fff as libc::c_int) as libc::c_float
                / 0x7fff as libc::c_int as libc::c_float) as libc::c_double - 0.5f64)
            * (*self_0).size[0 as libc::c_int as usize] as libc::c_double) as vec_t;
    org[1 as libc::c_int
        as usize] = ((*self_0).s.origin[1 as libc::c_int as usize] as libc::c_double
        + 2.0f64
            * (((rand() & 0x7fff as libc::c_int) as libc::c_float
                / 0x7fff as libc::c_int as libc::c_float) as libc::c_double - 0.5f64)
            * (*self_0).size[1 as libc::c_int as usize] as libc::c_double) as vec_t;
    org[2 as libc::c_int
        as usize] = ((*self_0).s.origin[2 as libc::c_int as usize] as libc::c_double
        + 2.0f64
            * (((rand() & 0x7fff as libc::c_int) as libc::c_float
                / 0x7fff as libc::c_int as libc::c_float) as libc::c_double - 0.5f64)
            * (*self_0).size[2 as libc::c_int as usize] as libc::c_double) as vec_t;
    ThrowDebris(
        self_0,
        b"models/objects/debris2/tris.md2\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        spd,
        org.as_mut_ptr(),
    );
    org[0 as libc::c_int
        as usize] = ((*self_0).s.origin[0 as libc::c_int as usize] as libc::c_double
        + 2.0f64
            * (((rand() & 0x7fff as libc::c_int) as libc::c_float
                / 0x7fff as libc::c_int as libc::c_float) as libc::c_double - 0.5f64)
            * (*self_0).size[0 as libc::c_int as usize] as libc::c_double) as vec_t;
    org[1 as libc::c_int
        as usize] = ((*self_0).s.origin[1 as libc::c_int as usize] as libc::c_double
        + 2.0f64
            * (((rand() & 0x7fff as libc::c_int) as libc::c_float
                / 0x7fff as libc::c_int as libc::c_float) as libc::c_double - 0.5f64)
            * (*self_0).size[1 as libc::c_int as usize] as libc::c_double) as vec_t;
    org[2 as libc::c_int
        as usize] = ((*self_0).s.origin[2 as libc::c_int as usize] as libc::c_double
        + 2.0f64
            * (((rand() & 0x7fff as libc::c_int) as libc::c_float
                / 0x7fff as libc::c_int as libc::c_float) as libc::c_double - 0.5f64)
            * (*self_0).size[2 as libc::c_int as usize] as libc::c_double) as vec_t;
    ThrowDebris(
        self_0,
        b"models/objects/debris2/tris.md2\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        spd,
        org.as_mut_ptr(),
    );
    (*self_0).s.origin[0 as libc::c_int as usize] = save[0 as libc::c_int as usize];
    (*self_0).s.origin[1 as libc::c_int as usize] = save[1 as libc::c_int as usize];
    (*self_0).s.origin[2 as libc::c_int as usize] = save[2 as libc::c_int as usize];
    if !((*self_0).groundentity).is_null() {
        BecomeExplosion2(self_0);
    } else {
        BecomeExplosion1(self_0);
    };
}
#[no_mangle]
pub unsafe extern "C" fn barrel_delay(
    mut self_0: *mut edict_t,
    mut inflictor: *mut edict_t,
    mut attacker: *mut edict_t,
    mut damage: libc::c_int,
    mut point: *mut vec_t,
) {
    (*self_0).takedamage = DAMAGE_NO as libc::c_int;
    (*self_0)
        .nextthink = (level.time as libc::c_double
        + 2 as libc::c_int as libc::c_double * 0.1f64) as libc::c_float;
    let ref mut fresh56 = (*self_0).think;
    *fresh56 = Some(barrel_explode as unsafe extern "C" fn(*mut edict_t) -> ());
    let ref mut fresh57 = (*self_0).activator;
    *fresh57 = attacker;
}
#[no_mangle]
pub unsafe extern "C" fn SP_misc_explobox(mut self_0: *mut edict_t) {
    if (*deathmatch).value != 0. {
        G_FreeEdict(self_0);
        return;
    }
    (gi.modelindex)
        .expect(
            "non-null function pointer",
        )(
        b"models/objects/debris1/tris.md2\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    (gi.modelindex)
        .expect(
            "non-null function pointer",
        )(
        b"models/objects/debris2/tris.md2\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    (gi.modelindex)
        .expect(
            "non-null function pointer",
        )(
        b"models/objects/debris3/tris.md2\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    (*self_0).solid = SOLID_BBOX;
    (*self_0).movetype = MOVETYPE_STEP as libc::c_int;
    let ref mut fresh58 = (*self_0).model;
    *fresh58 = b"models/objects/barrels/tris.md2\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char;
    (*self_0)
        .s
        .modelindex = (gi.modelindex)
        .expect("non-null function pointer")((*self_0).model);
    (*self_0).mins[0 as libc::c_int as usize] = -(16 as libc::c_int) as vec_t;
    (*self_0).mins[1 as libc::c_int as usize] = -(16 as libc::c_int) as vec_t;
    (*self_0).mins[2 as libc::c_int as usize] = 0 as libc::c_int as vec_t;
    (*self_0).maxs[0 as libc::c_int as usize] = 16 as libc::c_int as vec_t;
    (*self_0).maxs[1 as libc::c_int as usize] = 16 as libc::c_int as vec_t;
    (*self_0).maxs[2 as libc::c_int as usize] = 40 as libc::c_int as vec_t;
    if (*self_0).mass == 0 {
        (*self_0).mass = 400 as libc::c_int;
    }
    if (*self_0).health == 0 {
        (*self_0).health = 10 as libc::c_int;
    }
    if (*self_0).dmg == 0 {
        (*self_0).dmg = 150 as libc::c_int;
    }
    let ref mut fresh59 = (*self_0).die;
    *fresh59 = Some(
        barrel_delay
            as unsafe extern "C" fn(
                *mut edict_t,
                *mut edict_t,
                *mut edict_t,
                libc::c_int,
                *mut vec_t,
            ) -> (),
    );
    (*self_0).takedamage = DAMAGE_YES as libc::c_int;
    (*self_0).monsterinfo.aiflags = 0x400 as libc::c_int;
    let ref mut fresh60 = (*self_0).touch;
    *fresh60 = Some(
        barrel_touch
            as unsafe extern "C" fn(
                *mut edict_t,
                *mut edict_t,
                *mut cplane_t,
                *mut csurface_t,
            ) -> (),
    );
    let ref mut fresh61 = (*self_0).think;
    *fresh61 = Some(M_droptofloor as unsafe extern "C" fn(*mut edict_t) -> ());
    (*self_0)
        .nextthink = (level.time as libc::c_double
        + 2 as libc::c_int as libc::c_double * 0.1f64) as libc::c_float;
    (gi.linkentity).expect("non-null function pointer")(self_0);
}
#[no_mangle]
pub unsafe extern "C" fn misc_blackhole_use(
    mut ent: *mut edict_t,
    mut other: *mut edict_t,
    mut activator: *mut edict_t,
) {
    G_FreeEdict(ent);
}
#[no_mangle]
pub unsafe extern "C" fn misc_blackhole_think(mut self_0: *mut edict_t) {
    let ref mut fresh62 = (*self_0).s.frame;
    *fresh62 += 1;
    if *fresh62 < 19 as libc::c_int {
        (*self_0).nextthink = (level.time as libc::c_double + 0.1f64) as libc::c_float;
    } else {
        (*self_0).s.frame = 0 as libc::c_int;
        (*self_0).nextthink = (level.time as libc::c_double + 0.1f64) as libc::c_float;
    };
}
#[no_mangle]
pub unsafe extern "C" fn SP_misc_blackhole(mut ent: *mut edict_t) {
    (*ent).movetype = MOVETYPE_NONE as libc::c_int;
    (*ent).solid = SOLID_NOT;
    (*ent).mins[0 as libc::c_int as usize] = -(64 as libc::c_int) as vec_t;
    (*ent).mins[1 as libc::c_int as usize] = -(64 as libc::c_int) as vec_t;
    (*ent).mins[2 as libc::c_int as usize] = 0 as libc::c_int as vec_t;
    (*ent).maxs[0 as libc::c_int as usize] = 64 as libc::c_int as vec_t;
    (*ent).maxs[1 as libc::c_int as usize] = 64 as libc::c_int as vec_t;
    (*ent).maxs[2 as libc::c_int as usize] = 8 as libc::c_int as vec_t;
    (*ent)
        .s
        .modelindex = (gi.modelindex)
        .expect(
            "non-null function pointer",
        )(
        b"models/objects/black/tris.md2\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    (*ent).s.renderfx = 32 as libc::c_int;
    let ref mut fresh63 = (*ent).use_0;
    *fresh63 = Some(
        misc_blackhole_use
            as unsafe extern "C" fn(*mut edict_t, *mut edict_t, *mut edict_t) -> (),
    );
    let ref mut fresh64 = (*ent).think;
    *fresh64 = Some(misc_blackhole_think as unsafe extern "C" fn(*mut edict_t) -> ());
    (*ent)
        .nextthink = (level.time as libc::c_double
        + 2 as libc::c_int as libc::c_double * 0.1f64) as libc::c_float;
    (gi.linkentity).expect("non-null function pointer")(ent);
}
#[no_mangle]
pub unsafe extern "C" fn misc_eastertank_think(mut self_0: *mut edict_t) {
    let ref mut fresh65 = (*self_0).s.frame;
    *fresh65 += 1;
    if *fresh65 < 293 as libc::c_int {
        (*self_0).nextthink = (level.time as libc::c_double + 0.1f64) as libc::c_float;
    } else {
        (*self_0).s.frame = 254 as libc::c_int;
        (*self_0).nextthink = (level.time as libc::c_double + 0.1f64) as libc::c_float;
    };
}
#[no_mangle]
pub unsafe extern "C" fn SP_misc_eastertank(mut ent: *mut edict_t) {
    (*ent).movetype = MOVETYPE_NONE as libc::c_int;
    (*ent).solid = SOLID_BBOX;
    (*ent).mins[0 as libc::c_int as usize] = -(32 as libc::c_int) as vec_t;
    (*ent).mins[1 as libc::c_int as usize] = -(32 as libc::c_int) as vec_t;
    (*ent).mins[2 as libc::c_int as usize] = -(16 as libc::c_int) as vec_t;
    (*ent).maxs[0 as libc::c_int as usize] = 32 as libc::c_int as vec_t;
    (*ent).maxs[1 as libc::c_int as usize] = 32 as libc::c_int as vec_t;
    (*ent).maxs[2 as libc::c_int as usize] = 32 as libc::c_int as vec_t;
    (*ent)
        .s
        .modelindex = (gi.modelindex)
        .expect(
            "non-null function pointer",
        )(
        b"models/monsters/tank/tris.md2\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    (*ent).s.frame = 254 as libc::c_int;
    let ref mut fresh66 = (*ent).think;
    *fresh66 = Some(misc_eastertank_think as unsafe extern "C" fn(*mut edict_t) -> ());
    (*ent)
        .nextthink = (level.time as libc::c_double
        + 2 as libc::c_int as libc::c_double * 0.1f64) as libc::c_float;
    (gi.linkentity).expect("non-null function pointer")(ent);
}
#[no_mangle]
pub unsafe extern "C" fn misc_easterchick_think(mut self_0: *mut edict_t) {
    let ref mut fresh67 = (*self_0).s.frame;
    *fresh67 += 1;
    if *fresh67 < 247 as libc::c_int {
        (*self_0).nextthink = (level.time as libc::c_double + 0.1f64) as libc::c_float;
    } else {
        (*self_0).s.frame = 208 as libc::c_int;
        (*self_0).nextthink = (level.time as libc::c_double + 0.1f64) as libc::c_float;
    };
}
#[no_mangle]
pub unsafe extern "C" fn SP_misc_easterchick(mut ent: *mut edict_t) {
    (*ent).movetype = MOVETYPE_NONE as libc::c_int;
    (*ent).solid = SOLID_BBOX;
    (*ent).mins[0 as libc::c_int as usize] = -(32 as libc::c_int) as vec_t;
    (*ent).mins[1 as libc::c_int as usize] = -(32 as libc::c_int) as vec_t;
    (*ent).mins[2 as libc::c_int as usize] = 0 as libc::c_int as vec_t;
    (*ent).maxs[0 as libc::c_int as usize] = 32 as libc::c_int as vec_t;
    (*ent).maxs[1 as libc::c_int as usize] = 32 as libc::c_int as vec_t;
    (*ent).maxs[2 as libc::c_int as usize] = 32 as libc::c_int as vec_t;
    (*ent)
        .s
        .modelindex = (gi.modelindex)
        .expect(
            "non-null function pointer",
        )(
        b"models/monsters/bitch/tris.md2\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    (*ent).s.frame = 208 as libc::c_int;
    let ref mut fresh68 = (*ent).think;
    *fresh68 = Some(misc_easterchick_think as unsafe extern "C" fn(*mut edict_t) -> ());
    (*ent)
        .nextthink = (level.time as libc::c_double
        + 2 as libc::c_int as libc::c_double * 0.1f64) as libc::c_float;
    (gi.linkentity).expect("non-null function pointer")(ent);
}
#[no_mangle]
pub unsafe extern "C" fn misc_easterchick2_think(mut self_0: *mut edict_t) {
    let ref mut fresh69 = (*self_0).s.frame;
    *fresh69 += 1;
    if *fresh69 < 287 as libc::c_int {
        (*self_0).nextthink = (level.time as libc::c_double + 0.1f64) as libc::c_float;
    } else {
        (*self_0).s.frame = 248 as libc::c_int;
        (*self_0).nextthink = (level.time as libc::c_double + 0.1f64) as libc::c_float;
    };
}
#[no_mangle]
pub unsafe extern "C" fn SP_misc_easterchick2(mut ent: *mut edict_t) {
    (*ent).movetype = MOVETYPE_NONE as libc::c_int;
    (*ent).solid = SOLID_BBOX;
    (*ent).mins[0 as libc::c_int as usize] = -(32 as libc::c_int) as vec_t;
    (*ent).mins[1 as libc::c_int as usize] = -(32 as libc::c_int) as vec_t;
    (*ent).mins[2 as libc::c_int as usize] = 0 as libc::c_int as vec_t;
    (*ent).maxs[0 as libc::c_int as usize] = 32 as libc::c_int as vec_t;
    (*ent).maxs[1 as libc::c_int as usize] = 32 as libc::c_int as vec_t;
    (*ent).maxs[2 as libc::c_int as usize] = 32 as libc::c_int as vec_t;
    (*ent)
        .s
        .modelindex = (gi.modelindex)
        .expect(
            "non-null function pointer",
        )(
        b"models/monsters/bitch/tris.md2\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    (*ent).s.frame = 248 as libc::c_int;
    let ref mut fresh70 = (*ent).think;
    *fresh70 = Some(misc_easterchick2_think as unsafe extern "C" fn(*mut edict_t) -> ());
    (*ent)
        .nextthink = (level.time as libc::c_double
        + 2 as libc::c_int as libc::c_double * 0.1f64) as libc::c_float;
    (gi.linkentity).expect("non-null function pointer")(ent);
}
#[no_mangle]
pub unsafe extern "C" fn commander_body_think(mut self_0: *mut edict_t) {
    let ref mut fresh71 = (*self_0).s.frame;
    *fresh71 += 1;
    if *fresh71 < 24 as libc::c_int {
        (*self_0).nextthink = (level.time as libc::c_double + 0.1f64) as libc::c_float;
    } else {
        (*self_0).nextthink = 0 as libc::c_int as libc::c_float;
    }
    if (*self_0).s.frame == 22 as libc::c_int {
        (gi.sound)
            .expect(
                "non-null function pointer",
            )(
            self_0,
            4 as libc::c_int,
            (gi.soundindex)
                .expect(
                    "non-null function pointer",
                )(
                b"tank/thud.wav\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            ),
            1 as libc::c_int as libc::c_float,
            1 as libc::c_int as libc::c_float,
            0 as libc::c_int as libc::c_float,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn commander_body_use(
    mut self_0: *mut edict_t,
    mut other: *mut edict_t,
    mut activator: *mut edict_t,
) {
    let ref mut fresh72 = (*self_0).think;
    *fresh72 = Some(commander_body_think as unsafe extern "C" fn(*mut edict_t) -> ());
    (*self_0).nextthink = (level.time as libc::c_double + 0.1f64) as libc::c_float;
    (gi.sound)
        .expect(
            "non-null function pointer",
        )(
        self_0,
        4 as libc::c_int,
        (gi.soundindex)
            .expect(
                "non-null function pointer",
            )(
            b"tank/pain.wav\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ),
        1 as libc::c_int as libc::c_float,
        1 as libc::c_int as libc::c_float,
        0 as libc::c_int as libc::c_float,
    );
}
#[no_mangle]
pub unsafe extern "C" fn commander_body_drop(mut self_0: *mut edict_t) {
    (*self_0).movetype = MOVETYPE_TOSS as libc::c_int;
    let ref mut fresh73 = (*self_0).s.origin[2 as libc::c_int as usize];
    *fresh73 += 2 as libc::c_int as libc::c_float;
}
#[no_mangle]
pub unsafe extern "C" fn SP_monster_commander_body(mut self_0: *mut edict_t) {
    (*self_0).movetype = MOVETYPE_NONE as libc::c_int;
    (*self_0).solid = SOLID_BBOX;
    let ref mut fresh74 = (*self_0).model;
    *fresh74 = b"models/monsters/commandr/tris.md2\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char;
    (*self_0)
        .s
        .modelindex = (gi.modelindex)
        .expect("non-null function pointer")((*self_0).model);
    (*self_0).mins[0 as libc::c_int as usize] = -(32 as libc::c_int) as vec_t;
    (*self_0).mins[1 as libc::c_int as usize] = -(32 as libc::c_int) as vec_t;
    (*self_0).mins[2 as libc::c_int as usize] = 0 as libc::c_int as vec_t;
    (*self_0).maxs[0 as libc::c_int as usize] = 32 as libc::c_int as vec_t;
    (*self_0).maxs[1 as libc::c_int as usize] = 32 as libc::c_int as vec_t;
    (*self_0).maxs[2 as libc::c_int as usize] = 48 as libc::c_int as vec_t;
    let ref mut fresh75 = (*self_0).use_0;
    *fresh75 = Some(
        commander_body_use
            as unsafe extern "C" fn(*mut edict_t, *mut edict_t, *mut edict_t) -> (),
    );
    (*self_0).takedamage = DAMAGE_YES as libc::c_int;
    (*self_0).flags = 0x10 as libc::c_int;
    (*self_0).s.renderfx |= 64 as libc::c_int;
    (gi.linkentity).expect("non-null function pointer")(self_0);
    (gi.soundindex)
        .expect(
            "non-null function pointer",
        )(b"tank/thud.wav\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    (gi.soundindex)
        .expect(
            "non-null function pointer",
        )(b"tank/pain.wav\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    let ref mut fresh76 = (*self_0).think;
    *fresh76 = Some(commander_body_drop as unsafe extern "C" fn(*mut edict_t) -> ());
    (*self_0)
        .nextthink = (level.time as libc::c_double
        + 5 as libc::c_int as libc::c_double * 0.1f64) as libc::c_float;
}
#[no_mangle]
pub unsafe extern "C" fn misc_banner_think(mut ent: *mut edict_t) {
    (*ent).s.frame = ((*ent).s.frame + 1 as libc::c_int) % 16 as libc::c_int;
    (*ent).nextthink = (level.time as libc::c_double + 0.1f64) as libc::c_float;
}
#[no_mangle]
pub unsafe extern "C" fn SP_misc_banner(mut ent: *mut edict_t) {
    (*ent).movetype = MOVETYPE_NONE as libc::c_int;
    (*ent).solid = SOLID_NOT;
    (*ent)
        .s
        .modelindex = (gi.modelindex)
        .expect(
            "non-null function pointer",
        )(
        b"models/objects/banner/tris.md2\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    (*ent).s.frame = rand() % 16 as libc::c_int;
    (gi.linkentity).expect("non-null function pointer")(ent);
    let ref mut fresh77 = (*ent).think;
    *fresh77 = Some(misc_banner_think as unsafe extern "C" fn(*mut edict_t) -> ());
    (*ent).nextthink = (level.time as libc::c_double + 0.1f64) as libc::c_float;
}
#[no_mangle]
pub unsafe extern "C" fn misc_deadsoldier_die(
    mut self_0: *mut edict_t,
    mut inflictor: *mut edict_t,
    mut attacker: *mut edict_t,
    mut damage: libc::c_int,
    mut point: *mut vec_t,
) {
    let mut n: libc::c_int = 0;
    if (*self_0).health > -(80 as libc::c_int) {
        return;
    }
    (gi.sound)
        .expect(
            "non-null function pointer",
        )(
        self_0,
        4 as libc::c_int,
        (gi.soundindex)
            .expect(
                "non-null function pointer",
            )(
            b"misc/udeath.wav\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ),
        1 as libc::c_int as libc::c_float,
        1 as libc::c_int as libc::c_float,
        0 as libc::c_int as libc::c_float,
    );
    n = 0 as libc::c_int;
    while n < 4 as libc::c_int {
        ThrowGib(
            self_0,
            b"models/objects/gibs/sm_meat/tris.md2\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            damage,
            0 as libc::c_int,
        );
        n += 1;
    }
    ThrowHead(
        self_0,
        b"models/objects/gibs/head2/tris.md2\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        damage,
        0 as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn SP_misc_deadsoldier(mut ent: *mut edict_t) {
    if (*deathmatch).value != 0. {
        G_FreeEdict(ent);
        return;
    }
    (*ent).movetype = MOVETYPE_NONE as libc::c_int;
    (*ent).solid = SOLID_BBOX;
    (*ent)
        .s
        .modelindex = (gi.modelindex)
        .expect(
            "non-null function pointer",
        )(
        b"models/deadbods/dude/tris.md2\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    if (*ent).spawnflags & 2 as libc::c_int != 0 {
        (*ent).s.frame = 1 as libc::c_int;
    } else if (*ent).spawnflags & 4 as libc::c_int != 0 {
        (*ent).s.frame = 2 as libc::c_int;
    } else if (*ent).spawnflags & 8 as libc::c_int != 0 {
        (*ent).s.frame = 3 as libc::c_int;
    } else if (*ent).spawnflags & 16 as libc::c_int != 0 {
        (*ent).s.frame = 4 as libc::c_int;
    } else if (*ent).spawnflags & 32 as libc::c_int != 0 {
        (*ent).s.frame = 5 as libc::c_int;
    } else {
        (*ent).s.frame = 0 as libc::c_int;
    }
    (*ent).mins[0 as libc::c_int as usize] = -(16 as libc::c_int) as vec_t;
    (*ent).mins[1 as libc::c_int as usize] = -(16 as libc::c_int) as vec_t;
    (*ent).mins[2 as libc::c_int as usize] = 0 as libc::c_int as vec_t;
    (*ent).maxs[0 as libc::c_int as usize] = 16 as libc::c_int as vec_t;
    (*ent).maxs[1 as libc::c_int as usize] = 16 as libc::c_int as vec_t;
    (*ent).maxs[2 as libc::c_int as usize] = 16 as libc::c_int as vec_t;
    (*ent).deadflag = 2 as libc::c_int;
    (*ent).takedamage = DAMAGE_YES as libc::c_int;
    (*ent).svflags |= 0x4 as libc::c_int | 0x2 as libc::c_int;
    let ref mut fresh78 = (*ent).die;
    *fresh78 = Some(
        misc_deadsoldier_die
            as unsafe extern "C" fn(
                *mut edict_t,
                *mut edict_t,
                *mut edict_t,
                libc::c_int,
                *mut vec_t,
            ) -> (),
    );
    (*ent).monsterinfo.aiflags |= 0x100 as libc::c_int;
    (gi.linkentity).expect("non-null function pointer")(ent);
}
#[no_mangle]
pub unsafe extern "C" fn misc_viper_use(
    mut self_0: *mut edict_t,
    mut other: *mut edict_t,
    mut activator: *mut edict_t,
) {
    (*self_0).svflags &= !(0x1 as libc::c_int);
    let ref mut fresh79 = (*self_0).use_0;
    *fresh79 = Some(
        train_use as unsafe extern "C" fn(*mut edict_t, *mut edict_t, *mut edict_t) -> (),
    );
    train_use(self_0, other, activator);
}
#[no_mangle]
pub unsafe extern "C" fn SP_misc_viper(mut ent: *mut edict_t) {
    if ((*ent).target).is_null() {
        (gi.dprintf)
            .expect(
                "non-null function pointer",
            )(
            b"misc_viper without a target at %s\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            vtos(((*ent).absmin).as_mut_ptr()),
        );
        G_FreeEdict(ent);
        return;
    }
    if (*ent).speed == 0. {
        (*ent).speed = 300 as libc::c_int as libc::c_float;
    }
    (*ent).movetype = MOVETYPE_PUSH as libc::c_int;
    (*ent).solid = SOLID_NOT;
    (*ent)
        .s
        .modelindex = (gi.modelindex)
        .expect(
            "non-null function pointer",
        )(
        b"models/ships/viper/tris.md2\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    (*ent).mins[0 as libc::c_int as usize] = -(16 as libc::c_int) as vec_t;
    (*ent).mins[1 as libc::c_int as usize] = -(16 as libc::c_int) as vec_t;
    (*ent).mins[2 as libc::c_int as usize] = 0 as libc::c_int as vec_t;
    (*ent).maxs[0 as libc::c_int as usize] = 16 as libc::c_int as vec_t;
    (*ent).maxs[1 as libc::c_int as usize] = 16 as libc::c_int as vec_t;
    (*ent).maxs[2 as libc::c_int as usize] = 32 as libc::c_int as vec_t;
    let ref mut fresh80 = (*ent).think;
    *fresh80 = Some(func_train_find as unsafe extern "C" fn(*mut edict_t) -> ());
    (*ent).nextthink = (level.time as libc::c_double + 0.1f64) as libc::c_float;
    let ref mut fresh81 = (*ent).use_0;
    *fresh81 = Some(
        misc_viper_use
            as unsafe extern "C" fn(*mut edict_t, *mut edict_t, *mut edict_t) -> (),
    );
    (*ent).svflags |= 0x1 as libc::c_int;
    let ref mut fresh82 = (*ent).moveinfo.speed;
    *fresh82 = (*ent).speed;
    let ref mut fresh83 = (*ent).moveinfo.decel;
    *fresh83 = *fresh82;
    (*ent).moveinfo.accel = *fresh83;
    (gi.linkentity).expect("non-null function pointer")(ent);
}
#[no_mangle]
pub unsafe extern "C" fn SP_misc_bigviper(mut ent: *mut edict_t) {
    (*ent).movetype = MOVETYPE_NONE as libc::c_int;
    (*ent).solid = SOLID_BBOX;
    (*ent).mins[0 as libc::c_int as usize] = -(176 as libc::c_int) as vec_t;
    (*ent).mins[1 as libc::c_int as usize] = -(120 as libc::c_int) as vec_t;
    (*ent).mins[2 as libc::c_int as usize] = -(24 as libc::c_int) as vec_t;
    (*ent).maxs[0 as libc::c_int as usize] = 176 as libc::c_int as vec_t;
    (*ent).maxs[1 as libc::c_int as usize] = 120 as libc::c_int as vec_t;
    (*ent).maxs[2 as libc::c_int as usize] = 72 as libc::c_int as vec_t;
    (*ent)
        .s
        .modelindex = (gi.modelindex)
        .expect(
            "non-null function pointer",
        )(
        b"models/ships/bigviper/tris.md2\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    (gi.linkentity).expect("non-null function pointer")(ent);
}
#[no_mangle]
pub unsafe extern "C" fn misc_viper_bomb_touch(
    mut self_0: *mut edict_t,
    mut other: *mut edict_t,
    mut plane: *mut cplane_t,
    mut surf: *mut csurface_t,
) {
    G_UseTargets(self_0, (*self_0).activator);
    (*self_0)
        .s
        .origin[2 as libc::c_int
        as usize] = (*self_0).absmin[2 as libc::c_int as usize]
        + 1 as libc::c_int as libc::c_float;
    T_RadiusDamage(
        self_0,
        self_0,
        (*self_0).dmg as libc::c_float,
        0 as *mut edict_t,
        ((*self_0).dmg + 40 as libc::c_int) as libc::c_float,
        27 as libc::c_int,
    );
    BecomeExplosion2(self_0);
}
#[no_mangle]
pub unsafe extern "C" fn misc_viper_bomb_prethink(mut self_0: *mut edict_t) {
    let mut v: vec3_t = [0.; 3];
    let mut diff: libc::c_float = 0.;
    let ref mut fresh84 = (*self_0).groundentity;
    *fresh84 = 0 as *mut edict_t;
    diff = (*self_0).timestamp - level.time;
    if (diff as libc::c_double) < -1.0f64 {
        diff = -1.0f64 as libc::c_float;
    }
    VectorScale(
        ((*self_0).moveinfo.dir).as_mut_ptr(),
        (1.0f64 + diff as libc::c_double) as vec_t,
        v.as_mut_ptr(),
    );
    v[2 as libc::c_int as usize] = diff;
    diff = (*self_0).s.angles[2 as libc::c_int as usize];
    vectoangles(v.as_mut_ptr(), ((*self_0).s.angles).as_mut_ptr());
    (*self_0)
        .s
        .angles[2 as libc::c_int as usize] = diff + 10 as libc::c_int as libc::c_float;
}
#[no_mangle]
pub unsafe extern "C" fn misc_viper_bomb_use(
    mut self_0: *mut edict_t,
    mut other: *mut edict_t,
    mut activator: *mut edict_t,
) {
    let mut viper: *mut edict_t = 0 as *mut edict_t;
    (*self_0).solid = SOLID_BBOX;
    (*self_0).svflags &= !(0x1 as libc::c_int);
    (*self_0).s.effects |= 0x10 as libc::c_int as libc::c_uint;
    let ref mut fresh85 = (*self_0).use_0;
    *fresh85 = None;
    (*self_0).movetype = MOVETYPE_TOSS as libc::c_int;
    let ref mut fresh86 = (*self_0).prethink;
    *fresh86 = Some(
        misc_viper_bomb_prethink as unsafe extern "C" fn(*mut edict_t) -> (),
    );
    let ref mut fresh87 = (*self_0).touch;
    *fresh87 = Some(
        misc_viper_bomb_touch
            as unsafe extern "C" fn(
                *mut edict_t,
                *mut edict_t,
                *mut cplane_t,
                *mut csurface_t,
            ) -> (),
    );
    let ref mut fresh88 = (*self_0).activator;
    *fresh88 = activator;
    viper = G_Find(
        0 as *mut edict_t,
        &mut (*(0 as *mut edict_t)).classname as *mut *mut libc::c_char as libc::c_int,
        b"misc_viper\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    VectorScale(
        ((*viper).moveinfo.dir).as_mut_ptr(),
        (*viper).moveinfo.speed,
        ((*self_0).velocity).as_mut_ptr(),
    );
    (*self_0).timestamp = level.time;
    (*self_0)
        .moveinfo
        .dir[0 as libc::c_int
        as usize] = (*viper).moveinfo.dir[0 as libc::c_int as usize];
    (*self_0)
        .moveinfo
        .dir[1 as libc::c_int
        as usize] = (*viper).moveinfo.dir[1 as libc::c_int as usize];
    (*self_0)
        .moveinfo
        .dir[2 as libc::c_int
        as usize] = (*viper).moveinfo.dir[2 as libc::c_int as usize];
}
#[no_mangle]
pub unsafe extern "C" fn SP_misc_viper_bomb(mut self_0: *mut edict_t) {
    (*self_0).movetype = MOVETYPE_NONE as libc::c_int;
    (*self_0).solid = SOLID_NOT;
    (*self_0).mins[0 as libc::c_int as usize] = -(8 as libc::c_int) as vec_t;
    (*self_0).mins[1 as libc::c_int as usize] = -(8 as libc::c_int) as vec_t;
    (*self_0).mins[2 as libc::c_int as usize] = -(8 as libc::c_int) as vec_t;
    (*self_0).maxs[0 as libc::c_int as usize] = 8 as libc::c_int as vec_t;
    (*self_0).maxs[1 as libc::c_int as usize] = 8 as libc::c_int as vec_t;
    (*self_0).maxs[2 as libc::c_int as usize] = 8 as libc::c_int as vec_t;
    (*self_0)
        .s
        .modelindex = (gi.modelindex)
        .expect(
            "non-null function pointer",
        )(
        b"models/objects/bomb/tris.md2\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    if (*self_0).dmg == 0 {
        (*self_0).dmg = 1000 as libc::c_int;
    }
    let ref mut fresh89 = (*self_0).use_0;
    *fresh89 = Some(
        misc_viper_bomb_use
            as unsafe extern "C" fn(*mut edict_t, *mut edict_t, *mut edict_t) -> (),
    );
    (*self_0).svflags |= 0x1 as libc::c_int;
    (gi.linkentity).expect("non-null function pointer")(self_0);
}
#[no_mangle]
pub unsafe extern "C" fn misc_strogg_ship_use(
    mut self_0: *mut edict_t,
    mut other: *mut edict_t,
    mut activator: *mut edict_t,
) {
    (*self_0).svflags &= !(0x1 as libc::c_int);
    let ref mut fresh90 = (*self_0).use_0;
    *fresh90 = Some(
        train_use as unsafe extern "C" fn(*mut edict_t, *mut edict_t, *mut edict_t) -> (),
    );
    train_use(self_0, other, activator);
}
#[no_mangle]
pub unsafe extern "C" fn SP_misc_strogg_ship(mut ent: *mut edict_t) {
    if ((*ent).target).is_null() {
        (gi.dprintf)
            .expect(
                "non-null function pointer",
            )(
            b"%s without a target at %s\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            (*ent).classname,
            vtos(((*ent).absmin).as_mut_ptr()),
        );
        G_FreeEdict(ent);
        return;
    }
    if (*ent).speed == 0. {
        (*ent).speed = 300 as libc::c_int as libc::c_float;
    }
    (*ent).movetype = MOVETYPE_PUSH as libc::c_int;
    (*ent).solid = SOLID_NOT;
    (*ent)
        .s
        .modelindex = (gi.modelindex)
        .expect(
            "non-null function pointer",
        )(
        b"models/ships/strogg1/tris.md2\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    (*ent).mins[0 as libc::c_int as usize] = -(16 as libc::c_int) as vec_t;
    (*ent).mins[1 as libc::c_int as usize] = -(16 as libc::c_int) as vec_t;
    (*ent).mins[2 as libc::c_int as usize] = 0 as libc::c_int as vec_t;
    (*ent).maxs[0 as libc::c_int as usize] = 16 as libc::c_int as vec_t;
    (*ent).maxs[1 as libc::c_int as usize] = 16 as libc::c_int as vec_t;
    (*ent).maxs[2 as libc::c_int as usize] = 32 as libc::c_int as vec_t;
    let ref mut fresh91 = (*ent).think;
    *fresh91 = Some(func_train_find as unsafe extern "C" fn(*mut edict_t) -> ());
    (*ent).nextthink = (level.time as libc::c_double + 0.1f64) as libc::c_float;
    let ref mut fresh92 = (*ent).use_0;
    *fresh92 = Some(
        misc_strogg_ship_use
            as unsafe extern "C" fn(*mut edict_t, *mut edict_t, *mut edict_t) -> (),
    );
    (*ent).svflags |= 0x1 as libc::c_int;
    let ref mut fresh93 = (*ent).moveinfo.speed;
    *fresh93 = (*ent).speed;
    let ref mut fresh94 = (*ent).moveinfo.decel;
    *fresh94 = *fresh93;
    (*ent).moveinfo.accel = *fresh94;
    (gi.linkentity).expect("non-null function pointer")(ent);
}
#[no_mangle]
pub unsafe extern "C" fn misc_satellite_dish_think(mut self_0: *mut edict_t) {
    let ref mut fresh95 = (*self_0).s.frame;
    *fresh95 += 1;
    if (*self_0).s.frame < 38 as libc::c_int {
        (*self_0).nextthink = (level.time as libc::c_double + 0.1f64) as libc::c_float;
    }
}
#[no_mangle]
pub unsafe extern "C" fn misc_satellite_dish_use(
    mut self_0: *mut edict_t,
    mut other: *mut edict_t,
    mut activator: *mut edict_t,
) {
    (*self_0).s.frame = 0 as libc::c_int;
    let ref mut fresh96 = (*self_0).think;
    *fresh96 = Some(
        misc_satellite_dish_think as unsafe extern "C" fn(*mut edict_t) -> (),
    );
    (*self_0).nextthink = (level.time as libc::c_double + 0.1f64) as libc::c_float;
}
#[no_mangle]
pub unsafe extern "C" fn SP_misc_satellite_dish(mut ent: *mut edict_t) {
    (*ent).movetype = MOVETYPE_NONE as libc::c_int;
    (*ent).solid = SOLID_BBOX;
    (*ent).mins[0 as libc::c_int as usize] = -(64 as libc::c_int) as vec_t;
    (*ent).mins[1 as libc::c_int as usize] = -(64 as libc::c_int) as vec_t;
    (*ent).mins[2 as libc::c_int as usize] = 0 as libc::c_int as vec_t;
    (*ent).maxs[0 as libc::c_int as usize] = 64 as libc::c_int as vec_t;
    (*ent).maxs[1 as libc::c_int as usize] = 64 as libc::c_int as vec_t;
    (*ent).maxs[2 as libc::c_int as usize] = 128 as libc::c_int as vec_t;
    (*ent)
        .s
        .modelindex = (gi.modelindex)
        .expect(
            "non-null function pointer",
        )(
        b"models/objects/satellite/tris.md2\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    let ref mut fresh97 = (*ent).use_0;
    *fresh97 = Some(
        misc_satellite_dish_use
            as unsafe extern "C" fn(*mut edict_t, *mut edict_t, *mut edict_t) -> (),
    );
    (gi.linkentity).expect("non-null function pointer")(ent);
}
#[no_mangle]
pub unsafe extern "C" fn SP_light_mine1(mut ent: *mut edict_t) {
    (*ent).movetype = MOVETYPE_NONE as libc::c_int;
    (*ent).solid = SOLID_BBOX;
    (*ent)
        .s
        .modelindex = (gi.modelindex)
        .expect(
            "non-null function pointer",
        )(
        b"models/objects/minelite/light1/tris.md2\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    (gi.linkentity).expect("non-null function pointer")(ent);
}
#[no_mangle]
pub unsafe extern "C" fn SP_light_mine2(mut ent: *mut edict_t) {
    (*ent).movetype = MOVETYPE_NONE as libc::c_int;
    (*ent).solid = SOLID_BBOX;
    (*ent)
        .s
        .modelindex = (gi.modelindex)
        .expect(
            "non-null function pointer",
        )(
        b"models/objects/minelite/light2/tris.md2\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    (gi.linkentity).expect("non-null function pointer")(ent);
}
#[no_mangle]
pub unsafe extern "C" fn SP_misc_gib_arm(mut ent: *mut edict_t) {
    (gi.setmodel)
        .expect(
            "non-null function pointer",
        )(
        ent,
        b"models/objects/gibs/arm/tris.md2\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    (*ent).solid = SOLID_NOT;
    (*ent).s.effects |= 0x2 as libc::c_int as libc::c_uint;
    (*ent).takedamage = DAMAGE_YES as libc::c_int;
    let ref mut fresh98 = (*ent).die;
    *fresh98 = Some(
        gib_die
            as unsafe extern "C" fn(
                *mut edict_t,
                *mut edict_t,
                *mut edict_t,
                libc::c_int,
                *mut vec_t,
            ) -> (),
    );
    (*ent).movetype = MOVETYPE_TOSS as libc::c_int;
    (*ent).svflags |= 0x4 as libc::c_int;
    (*ent).deadflag = 2 as libc::c_int;
    (*ent)
        .avelocity[0 as libc::c_int
        as usize] = (rand() & 0x7fff as libc::c_int) as libc::c_float
        / 0x7fff as libc::c_int as libc::c_float * 200 as libc::c_int as libc::c_float;
    (*ent)
        .avelocity[1 as libc::c_int
        as usize] = (rand() & 0x7fff as libc::c_int) as libc::c_float
        / 0x7fff as libc::c_int as libc::c_float * 200 as libc::c_int as libc::c_float;
    (*ent)
        .avelocity[2 as libc::c_int
        as usize] = (rand() & 0x7fff as libc::c_int) as libc::c_float
        / 0x7fff as libc::c_int as libc::c_float * 200 as libc::c_int as libc::c_float;
    let ref mut fresh99 = (*ent).think;
    *fresh99 = Some(G_FreeEdict as unsafe extern "C" fn(*mut edict_t) -> ());
    (*ent).nextthink = level.time + 30 as libc::c_int as libc::c_float;
    (gi.linkentity).expect("non-null function pointer")(ent);
}
#[no_mangle]
pub unsafe extern "C" fn SP_misc_gib_leg(mut ent: *mut edict_t) {
    (gi.setmodel)
        .expect(
            "non-null function pointer",
        )(
        ent,
        b"models/objects/gibs/leg/tris.md2\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    (*ent).solid = SOLID_NOT;
    (*ent).s.effects |= 0x2 as libc::c_int as libc::c_uint;
    (*ent).takedamage = DAMAGE_YES as libc::c_int;
    let ref mut fresh100 = (*ent).die;
    *fresh100 = Some(
        gib_die
            as unsafe extern "C" fn(
                *mut edict_t,
                *mut edict_t,
                *mut edict_t,
                libc::c_int,
                *mut vec_t,
            ) -> (),
    );
    (*ent).movetype = MOVETYPE_TOSS as libc::c_int;
    (*ent).svflags |= 0x4 as libc::c_int;
    (*ent).deadflag = 2 as libc::c_int;
    (*ent)
        .avelocity[0 as libc::c_int
        as usize] = (rand() & 0x7fff as libc::c_int) as libc::c_float
        / 0x7fff as libc::c_int as libc::c_float * 200 as libc::c_int as libc::c_float;
    (*ent)
        .avelocity[1 as libc::c_int
        as usize] = (rand() & 0x7fff as libc::c_int) as libc::c_float
        / 0x7fff as libc::c_int as libc::c_float * 200 as libc::c_int as libc::c_float;
    (*ent)
        .avelocity[2 as libc::c_int
        as usize] = (rand() & 0x7fff as libc::c_int) as libc::c_float
        / 0x7fff as libc::c_int as libc::c_float * 200 as libc::c_int as libc::c_float;
    let ref mut fresh101 = (*ent).think;
    *fresh101 = Some(G_FreeEdict as unsafe extern "C" fn(*mut edict_t) -> ());
    (*ent).nextthink = level.time + 30 as libc::c_int as libc::c_float;
    (gi.linkentity).expect("non-null function pointer")(ent);
}
#[no_mangle]
pub unsafe extern "C" fn SP_misc_gib_head(mut ent: *mut edict_t) {
    (gi.setmodel)
        .expect(
            "non-null function pointer",
        )(
        ent,
        b"models/objects/gibs/head/tris.md2\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    (*ent).solid = SOLID_NOT;
    (*ent).s.effects |= 0x2 as libc::c_int as libc::c_uint;
    (*ent).takedamage = DAMAGE_YES as libc::c_int;
    let ref mut fresh102 = (*ent).die;
    *fresh102 = Some(
        gib_die
            as unsafe extern "C" fn(
                *mut edict_t,
                *mut edict_t,
                *mut edict_t,
                libc::c_int,
                *mut vec_t,
            ) -> (),
    );
    (*ent).movetype = MOVETYPE_TOSS as libc::c_int;
    (*ent).svflags |= 0x4 as libc::c_int;
    (*ent).deadflag = 2 as libc::c_int;
    (*ent)
        .avelocity[0 as libc::c_int
        as usize] = (rand() & 0x7fff as libc::c_int) as libc::c_float
        / 0x7fff as libc::c_int as libc::c_float * 200 as libc::c_int as libc::c_float;
    (*ent)
        .avelocity[1 as libc::c_int
        as usize] = (rand() & 0x7fff as libc::c_int) as libc::c_float
        / 0x7fff as libc::c_int as libc::c_float * 200 as libc::c_int as libc::c_float;
    (*ent)
        .avelocity[2 as libc::c_int
        as usize] = (rand() & 0x7fff as libc::c_int) as libc::c_float
        / 0x7fff as libc::c_int as libc::c_float * 200 as libc::c_int as libc::c_float;
    let ref mut fresh103 = (*ent).think;
    *fresh103 = Some(G_FreeEdict as unsafe extern "C" fn(*mut edict_t) -> ());
    (*ent).nextthink = level.time + 30 as libc::c_int as libc::c_float;
    (gi.linkentity).expect("non-null function pointer")(ent);
}
#[no_mangle]
pub unsafe extern "C" fn SP_target_character(mut self_0: *mut edict_t) {
    (*self_0).movetype = MOVETYPE_PUSH as libc::c_int;
    (gi.setmodel).expect("non-null function pointer")(self_0, (*self_0).model);
    (*self_0).solid = SOLID_BSP;
    (*self_0).s.frame = 12 as libc::c_int;
    (gi.linkentity).expect("non-null function pointer")(self_0);
}
#[no_mangle]
pub unsafe extern "C" fn target_string_use(
    mut self_0: *mut edict_t,
    mut other: *mut edict_t,
    mut activator: *mut edict_t,
) {
    let mut e: *mut edict_t = 0 as *mut edict_t;
    let mut n: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut c: libc::c_char = 0;
    l = strlen((*self_0).message) as libc::c_int;
    e = (*self_0).teammaster;
    while !e.is_null() {
        if !((*e).count == 0) {
            n = (*e).count - 1 as libc::c_int;
            if n > l {
                (*e).s.frame = 12 as libc::c_int;
            } else {
                c = *((*self_0).message).offset(n as isize);
                if c as libc::c_int >= '0' as i32 && c as libc::c_int <= '9' as i32 {
                    (*e).s.frame = c as libc::c_int - '0' as i32;
                } else if c as libc::c_int == '-' as i32 {
                    (*e).s.frame = 10 as libc::c_int;
                } else if c as libc::c_int == ':' as i32 {
                    (*e).s.frame = 11 as libc::c_int;
                } else {
                    (*e).s.frame = 12 as libc::c_int;
                }
            }
        }
        e = (*e).teamchain;
    }
}
#[no_mangle]
pub unsafe extern "C" fn SP_target_string(mut self_0: *mut edict_t) {
    if ((*self_0).message).is_null() {
        let ref mut fresh104 = (*self_0).message;
        *fresh104 = b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    let ref mut fresh105 = (*self_0).use_0;
    *fresh105 = Some(
        target_string_use
            as unsafe extern "C" fn(*mut edict_t, *mut edict_t, *mut edict_t) -> (),
    );
}
unsafe extern "C" fn func_clock_reset(mut self_0: *mut edict_t) {
    let ref mut fresh106 = (*self_0).activator;
    *fresh106 = 0 as *mut edict_t;
    if (*self_0).spawnflags & 1 as libc::c_int != 0 {
        (*self_0).health = 0 as libc::c_int;
        (*self_0).wait = (*self_0).count as libc::c_float;
    } else if (*self_0).spawnflags & 2 as libc::c_int != 0 {
        (*self_0).health = (*self_0).count;
        (*self_0).wait = 0 as libc::c_int as libc::c_float;
    }
}
unsafe extern "C" fn func_clock_format_countdown(mut self_0: *mut edict_t) {
    if (*self_0).style == 0 as libc::c_int {
        Com_sprintf(
            (*self_0).message,
            16 as libc::c_int,
            b"%2i\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            (*self_0).health,
        );
        return;
    }
    if (*self_0).style == 1 as libc::c_int {
        Com_sprintf(
            (*self_0).message,
            16 as libc::c_int,
            b"%2i:%2i\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            (*self_0).health / 60 as libc::c_int,
            (*self_0).health % 60 as libc::c_int,
        );
        if *((*self_0).message).offset(3 as libc::c_int as isize) as libc::c_int
            == ' ' as i32
        {
            *((*self_0).message)
                .offset(3 as libc::c_int as isize) = '0' as i32 as libc::c_char;
        }
        return;
    }
    if (*self_0).style == 2 as libc::c_int {
        Com_sprintf(
            (*self_0).message,
            16 as libc::c_int,
            b"%2i:%2i:%2i\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            (*self_0).health / 3600 as libc::c_int,
            ((*self_0).health
                - (*self_0).health / 3600 as libc::c_int * 3600 as libc::c_int)
                / 60 as libc::c_int,
            (*self_0).health % 60 as libc::c_int,
        );
        if *((*self_0).message).offset(3 as libc::c_int as isize) as libc::c_int
            == ' ' as i32
        {
            *((*self_0).message)
                .offset(3 as libc::c_int as isize) = '0' as i32 as libc::c_char;
        }
        if *((*self_0).message).offset(6 as libc::c_int as isize) as libc::c_int
            == ' ' as i32
        {
            *((*self_0).message)
                .offset(6 as libc::c_int as isize) = '0' as i32 as libc::c_char;
        }
        return;
    }
}
#[no_mangle]
pub unsafe extern "C" fn func_clock_think(mut self_0: *mut edict_t) {
    if ((*self_0).enemy).is_null() {
        let ref mut fresh107 = (*self_0).enemy;
        *fresh107 = G_Find(
            0 as *mut edict_t,
            &mut (*(0 as *mut edict_t)).targetname as *mut *mut libc::c_char
                as libc::c_int,
            (*self_0).target,
        );
        if ((*self_0).enemy).is_null() {
            return;
        }
    }
    if (*self_0).spawnflags & 1 as libc::c_int != 0 {
        func_clock_format_countdown(self_0);
        let ref mut fresh108 = (*self_0).health;
        *fresh108 += 1;
    } else if (*self_0).spawnflags & 2 as libc::c_int != 0 {
        func_clock_format_countdown(self_0);
        let ref mut fresh109 = (*self_0).health;
        *fresh109 -= 1;
    } else {
        let mut ltime: *mut tm = 0 as *mut tm;
        let mut gmtime: time_t = 0;
        time(&mut gmtime);
        ltime = localtime(&mut gmtime);
        Com_sprintf(
            (*self_0).message,
            16 as libc::c_int,
            b"%2i:%2i:%2i\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            (*ltime).tm_hour,
            (*ltime).tm_min,
            (*ltime).tm_sec,
        );
        if *((*self_0).message).offset(3 as libc::c_int as isize) as libc::c_int
            == ' ' as i32
        {
            *((*self_0).message)
                .offset(3 as libc::c_int as isize) = '0' as i32 as libc::c_char;
        }
        if *((*self_0).message).offset(6 as libc::c_int as isize) as libc::c_int
            == ' ' as i32
        {
            *((*self_0).message)
                .offset(6 as libc::c_int as isize) = '0' as i32 as libc::c_char;
        }
    }
    let ref mut fresh110 = (*(*self_0).enemy).message;
    *fresh110 = (*self_0).message;
    ((*(*self_0).enemy).use_0)
        .expect("non-null function pointer")((*self_0).enemy, self_0, self_0);
    if (*self_0).spawnflags & 1 as libc::c_int != 0
        && (*self_0).health as libc::c_float > (*self_0).wait
        || (*self_0).spawnflags & 2 as libc::c_int != 0
            && ((*self_0).health as libc::c_float) < (*self_0).wait
    {
        if !((*self_0).pathtarget).is_null() {
            let mut savetarget: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut savemessage: *mut libc::c_char = 0 as *mut libc::c_char;
            savetarget = (*self_0).target;
            savemessage = (*self_0).message;
            let ref mut fresh111 = (*self_0).target;
            *fresh111 = (*self_0).pathtarget;
            let ref mut fresh112 = (*self_0).message;
            *fresh112 = 0 as *mut libc::c_char;
            G_UseTargets(self_0, (*self_0).activator);
            let ref mut fresh113 = (*self_0).target;
            *fresh113 = savetarget;
            let ref mut fresh114 = (*self_0).message;
            *fresh114 = savemessage;
        }
        if (*self_0).spawnflags & 8 as libc::c_int == 0 {
            return;
        }
        func_clock_reset(self_0);
        if (*self_0).spawnflags & 4 as libc::c_int != 0 {
            return;
        }
    }
    (*self_0).nextthink = level.time + 1 as libc::c_int as libc::c_float;
}
#[no_mangle]
pub unsafe extern "C" fn func_clock_use(
    mut self_0: *mut edict_t,
    mut other: *mut edict_t,
    mut activator: *mut edict_t,
) {
    if (*self_0).spawnflags & 8 as libc::c_int == 0 {
        let ref mut fresh115 = (*self_0).use_0;
        *fresh115 = None;
    }
    if !((*self_0).activator).is_null() {
        return;
    }
    let ref mut fresh116 = (*self_0).activator;
    *fresh116 = activator;
    ((*self_0).think).expect("non-null function pointer")(self_0);
}
#[no_mangle]
pub unsafe extern "C" fn SP_func_clock(mut self_0: *mut edict_t) {
    if ((*self_0).target).is_null() {
        (gi.dprintf)
            .expect(
                "non-null function pointer",
            )(
            b"%s with no target at %s\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            (*self_0).classname,
            vtos(((*self_0).s.origin).as_mut_ptr()),
        );
        G_FreeEdict(self_0);
        return;
    }
    if (*self_0).spawnflags & 2 as libc::c_int != 0 && (*self_0).count == 0 {
        (gi.dprintf)
            .expect(
                "non-null function pointer",
            )(
            b"%s with no count at %s\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            (*self_0).classname,
            vtos(((*self_0).s.origin).as_mut_ptr()),
        );
        G_FreeEdict(self_0);
        return;
    }
    if (*self_0).spawnflags & 1 as libc::c_int != 0 && (*self_0).count == 0 {
        (*self_0).count = 60 as libc::c_int * 60 as libc::c_int;
    }
    func_clock_reset(self_0);
    let ref mut fresh117 = (*self_0).message;
    *fresh117 = (gi.TagMalloc)
        .expect("non-null function pointer")(16 as libc::c_int, 766 as libc::c_int)
        as *mut libc::c_char;
    let ref mut fresh118 = (*self_0).think;
    *fresh118 = Some(func_clock_think as unsafe extern "C" fn(*mut edict_t) -> ());
    if (*self_0).spawnflags & 4 as libc::c_int != 0 {
        let ref mut fresh119 = (*self_0).use_0;
        *fresh119 = Some(
            func_clock_use
                as unsafe extern "C" fn(*mut edict_t, *mut edict_t, *mut edict_t) -> (),
        );
    } else {
        (*self_0).nextthink = level.time + 1 as libc::c_int as libc::c_float;
    };
}
#[no_mangle]
pub unsafe extern "C" fn teleporter_touch(
    mut self_0: *mut edict_t,
    mut other: *mut edict_t,
    mut plane: *mut cplane_t,
    mut surf: *mut csurface_t,
) {
    let mut dest: *mut edict_t = 0 as *mut edict_t;
    let mut i: libc::c_int = 0;
    if ((*other).client).is_null() {
        return;
    }
    dest = G_Find(
        0 as *mut edict_t,
        &mut (*(0 as *mut edict_t)).targetname as *mut *mut libc::c_char as libc::c_int,
        (*self_0).target,
    );
    if dest.is_null() {
        (gi.dprintf)
            .expect(
                "non-null function pointer",
            )(
            b"Couldn't find destination\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return;
    }
    CTFPlayerResetGrapple(other);
    (gi.unlinkentity).expect("non-null function pointer")(other);
    (*other)
        .s
        .origin[0 as libc::c_int as usize] = (*dest).s.origin[0 as libc::c_int as usize];
    (*other)
        .s
        .origin[1 as libc::c_int as usize] = (*dest).s.origin[1 as libc::c_int as usize];
    (*other)
        .s
        .origin[2 as libc::c_int as usize] = (*dest).s.origin[2 as libc::c_int as usize];
    (*other)
        .s
        .old_origin[0 as libc::c_int
        as usize] = (*dest).s.origin[0 as libc::c_int as usize];
    (*other)
        .s
        .old_origin[1 as libc::c_int
        as usize] = (*dest).s.origin[1 as libc::c_int as usize];
    (*other)
        .s
        .old_origin[2 as libc::c_int
        as usize] = (*dest).s.origin[2 as libc::c_int as usize];
    let ref mut fresh120 = (*other).s.origin[2 as libc::c_int as usize];
    *fresh120 += 10 as libc::c_int as libc::c_float;
    let ref mut fresh121 = (*other).velocity[2 as libc::c_int as usize];
    *fresh121 = 0 as libc::c_int as vec_t;
    let ref mut fresh122 = (*other).velocity[1 as libc::c_int as usize];
    *fresh122 = *fresh121;
    (*other).velocity[0 as libc::c_int as usize] = *fresh122;
    (*(*other).client)
        .ps
        .pmove
        .pm_time = (160 as libc::c_int >> 3 as libc::c_int) as byte;
    let ref mut fresh123 = (*(*other).client).ps.pmove.pm_flags;
    *fresh123 = (*fresh123 as libc::c_int | 32 as libc::c_int) as byte;
    (*(*self_0).owner).s.event = EV_PLAYER_TELEPORT as libc::c_int;
    (*other).s.event = EV_PLAYER_TELEPORT as libc::c_int;
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        (*(*other).client)
            .ps
            .pmove
            .delta_angles[i
            as usize] = ((((*dest).s.angles[i as usize]
            - (*(*other).client).resp.cmd_angles[i as usize])
            * 65536 as libc::c_int as libc::c_float
            / 360 as libc::c_int as libc::c_float) as libc::c_int & 65535 as libc::c_int)
            as libc::c_short;
        i += 1;
    }
    let ref mut fresh124 = (*other).s.angles[2 as libc::c_int as usize];
    *fresh124 = 0 as libc::c_int as vec_t;
    let ref mut fresh125 = (*other).s.angles[1 as libc::c_int as usize];
    *fresh125 = *fresh124;
    (*other).s.angles[0 as libc::c_int as usize] = *fresh125;
    let ref mut fresh126 = (*(*other).client).ps.viewangles[2 as libc::c_int as usize];
    *fresh126 = 0 as libc::c_int as vec_t;
    let ref mut fresh127 = (*(*other).client).ps.viewangles[1 as libc::c_int as usize];
    *fresh127 = *fresh126;
    (*(*other).client).ps.viewangles[0 as libc::c_int as usize] = *fresh127;
    let ref mut fresh128 = (*(*other).client).v_angle[2 as libc::c_int as usize];
    *fresh128 = 0 as libc::c_int as vec_t;
    let ref mut fresh129 = (*(*other).client).v_angle[1 as libc::c_int as usize];
    *fresh129 = *fresh128;
    (*(*other).client).v_angle[0 as libc::c_int as usize] = *fresh129;
    KillBox(other);
    (gi.linkentity).expect("non-null function pointer")(other);
}
#[no_mangle]
pub unsafe extern "C" fn SP_misc_teleporter(mut ent: *mut edict_t) {
    let mut trig: *mut edict_t = 0 as *mut edict_t;
    if ((*ent).target).is_null() {
        (gi.dprintf)
            .expect(
                "non-null function pointer",
            )(
            b"teleporter without a target.\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        G_FreeEdict(ent);
        return;
    }
    (gi.setmodel)
        .expect(
            "non-null function pointer",
        )(
        ent,
        b"models/objects/dmspot/tris.md2\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    (*ent).s.skinnum = 1 as libc::c_int;
    (*ent).s.effects = 0x20000 as libc::c_int as libc::c_uint;
    (*ent)
        .s
        .sound = (gi.soundindex)
        .expect(
            "non-null function pointer",
        )(b"world/amb10.wav\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    (*ent).solid = SOLID_BBOX;
    (*ent).mins[0 as libc::c_int as usize] = -(32 as libc::c_int) as vec_t;
    (*ent).mins[1 as libc::c_int as usize] = -(32 as libc::c_int) as vec_t;
    (*ent).mins[2 as libc::c_int as usize] = -(24 as libc::c_int) as vec_t;
    (*ent).maxs[0 as libc::c_int as usize] = 32 as libc::c_int as vec_t;
    (*ent).maxs[1 as libc::c_int as usize] = 32 as libc::c_int as vec_t;
    (*ent).maxs[2 as libc::c_int as usize] = -(16 as libc::c_int) as vec_t;
    (gi.linkentity).expect("non-null function pointer")(ent);
    trig = G_Spawn();
    let ref mut fresh130 = (*trig).touch;
    *fresh130 = Some(
        teleporter_touch
            as unsafe extern "C" fn(
                *mut edict_t,
                *mut edict_t,
                *mut cplane_t,
                *mut csurface_t,
            ) -> (),
    );
    (*trig).solid = SOLID_TRIGGER;
    let ref mut fresh131 = (*trig).target;
    *fresh131 = (*ent).target;
    let ref mut fresh132 = (*trig).owner;
    *fresh132 = ent;
    (*trig)
        .s
        .origin[0 as libc::c_int as usize] = (*ent).s.origin[0 as libc::c_int as usize];
    (*trig)
        .s
        .origin[1 as libc::c_int as usize] = (*ent).s.origin[1 as libc::c_int as usize];
    (*trig)
        .s
        .origin[2 as libc::c_int as usize] = (*ent).s.origin[2 as libc::c_int as usize];
    (*trig).mins[0 as libc::c_int as usize] = -(8 as libc::c_int) as vec_t;
    (*trig).mins[1 as libc::c_int as usize] = -(8 as libc::c_int) as vec_t;
    (*trig).mins[2 as libc::c_int as usize] = 8 as libc::c_int as vec_t;
    (*trig).maxs[0 as libc::c_int as usize] = 8 as libc::c_int as vec_t;
    (*trig).maxs[1 as libc::c_int as usize] = 8 as libc::c_int as vec_t;
    (*trig).maxs[2 as libc::c_int as usize] = 24 as libc::c_int as vec_t;
    (gi.linkentity).expect("non-null function pointer")(trig);
}
#[no_mangle]
pub unsafe extern "C" fn SP_misc_teleporter_dest(mut ent: *mut edict_t) {
    (gi.setmodel)
        .expect(
            "non-null function pointer",
        )(
        ent,
        b"models/objects/dmspot/tris.md2\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    (*ent).s.skinnum = 0 as libc::c_int;
    (*ent).solid = SOLID_BBOX;
    (*ent).mins[0 as libc::c_int as usize] = -(32 as libc::c_int) as vec_t;
    (*ent).mins[1 as libc::c_int as usize] = -(32 as libc::c_int) as vec_t;
    (*ent).mins[2 as libc::c_int as usize] = -(24 as libc::c_int) as vec_t;
    (*ent).maxs[0 as libc::c_int as usize] = 32 as libc::c_int as vec_t;
    (*ent).maxs[1 as libc::c_int as usize] = 32 as libc::c_int as vec_t;
    (*ent).maxs[2 as libc::c_int as usize] = -(16 as libc::c_int) as vec_t;
    (gi.linkentity).expect("non-null function pointer")(ent);
}
