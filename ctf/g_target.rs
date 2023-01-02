#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn rand() -> libc::c_int;
    static mut vec3_origin: vec3_t;
    fn VectorMA(
        veca: *mut vec_t,
        scale: libc::c_float,
        vecb: *mut vec_t,
        vecc: *mut vec_t,
    );
    fn VectorCompare(v1: *mut vec_t, v2: *mut vec_t) -> libc::c_int;
    fn VectorNormalize(v: *mut vec_t) -> vec_t;
    fn VectorScale(in_0: *mut vec_t, scale: vec_t, out: *mut vec_t);
    fn Com_sprintf(
        dest: *mut libc::c_char,
        size: libc::c_int,
        fmt: *mut libc::c_char,
        _: ...
    );
    fn G_SetMovedir(angles: *mut vec_t, movedir: *mut vec_t);
    fn G_Spawn() -> *mut edict_t;
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
    fn T_RadiusDamage(
        inflictor: *mut edict_t,
        attacker: *mut edict_t,
        damage: libc::c_float,
        ignore: *mut edict_t,
        radius: libc::c_float,
        mod_0: libc::c_int,
    );
    fn G_UseTargets(ent: *mut edict_t, activator: *mut edict_t);
    fn G_Find(
        from: *mut edict_t,
        fieldofs: libc::c_int,
        match_0: *mut libc::c_char,
    ) -> *mut edict_t;
    fn KillBox(ent: *mut edict_t) -> qboolean;
    fn fire_blaster(
        self_0: *mut edict_t,
        start: *mut vec_t,
        aimdir: *mut vec_t,
        damage: libc::c_int,
        speed: libc::c_int,
        effect: libc::c_int,
        hyper: qboolean,
    );
    static mut dmflags: *mut cvar_t;
    static mut coop: *mut cvar_t;
    static mut deathmatch: *mut cvar_t;
    static mut g_edicts: *mut edict_t;
    static mut st: spawn_temp_t;
    static mut globals: game_export_t;
    static mut gi: game_import_t;
    static mut level: level_locals_t;
    static mut game: game_locals_t;
    fn BeginIntermission(targ: *mut edict_t);
    fn ED_CallSpawn(ent: *mut edict_t);
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
pub unsafe extern "C" fn Use_Target_Tent(
    mut ent: *mut edict_t,
    mut other: *mut edict_t,
    mut activator: *mut edict_t,
) {
    (gi.WriteByte).expect("non-null function pointer")(3 as libc::c_int);
    (gi.WriteByte).expect("non-null function pointer")((*ent).style);
    (gi.WritePosition)
        .expect("non-null function pointer")(((*ent).s.origin).as_mut_ptr());
    (gi.multicast)
        .expect(
            "non-null function pointer",
        )(((*ent).s.origin).as_mut_ptr(), MULTICAST_PVS);
}
#[no_mangle]
pub unsafe extern "C" fn SP_target_temp_entity(mut ent: *mut edict_t) {
    let ref mut fresh0 = (*ent).use_0;
    *fresh0 = Some(
        Use_Target_Tent
            as unsafe extern "C" fn(*mut edict_t, *mut edict_t, *mut edict_t) -> (),
    );
}
#[no_mangle]
pub unsafe extern "C" fn Use_Target_Speaker(
    mut ent: *mut edict_t,
    mut other: *mut edict_t,
    mut activator: *mut edict_t,
) {
    let mut chan: libc::c_int = 0;
    if (*ent).spawnflags & 3 as libc::c_int != 0 {
        if (*ent).s.sound != 0 {
            (*ent).s.sound = 0 as libc::c_int;
        } else {
            (*ent).s.sound = (*ent).noise_index;
        }
    } else {
        if (*ent).spawnflags & 4 as libc::c_int != 0 {
            chan = 2 as libc::c_int | 16 as libc::c_int;
        } else {
            chan = 2 as libc::c_int;
        }
        (gi.positioned_sound)
            .expect(
                "non-null function pointer",
            )(
            ((*ent).s.origin).as_mut_ptr(),
            ent,
            chan,
            (*ent).noise_index,
            (*ent).volume,
            (*ent).attenuation,
            0 as libc::c_int as libc::c_float,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn SP_target_speaker(mut ent: *mut edict_t) {
    let mut buffer: [libc::c_char; 64] = [0; 64];
    if (st.noise).is_null() {
        (gi.dprintf)
            .expect(
                "non-null function pointer",
            )(
            b"target_speaker with no noise set at %s\n\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            vtos(((*ent).s.origin).as_mut_ptr()),
        );
        return;
    }
    if (strstr(st.noise, b".wav\0" as *const u8 as *const libc::c_char)).is_null() {
        Com_sprintf(
            buffer.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
            b"%s.wav\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            st.noise,
        );
    } else {
        strncpy(
            buffer.as_mut_ptr(),
            st.noise,
            ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong,
        );
    }
    (*ent)
        .noise_index = (gi.soundindex)
        .expect("non-null function pointer")(buffer.as_mut_ptr());
    if (*ent).volume == 0. {
        (*ent).volume = 1.0f64 as libc::c_float;
    }
    if (*ent).attenuation == 0. {
        (*ent).attenuation = 1.0f64 as libc::c_float;
    } else if (*ent).attenuation == -(1 as libc::c_int) as libc::c_float {
        (*ent).attenuation = 0 as libc::c_int as libc::c_float;
    }
    if (*ent).spawnflags & 1 as libc::c_int != 0 {
        (*ent).s.sound = (*ent).noise_index;
    }
    let ref mut fresh1 = (*ent).use_0;
    *fresh1 = Some(
        Use_Target_Speaker
            as unsafe extern "C" fn(*mut edict_t, *mut edict_t, *mut edict_t) -> (),
    );
    (gi.linkentity).expect("non-null function pointer")(ent);
}
#[no_mangle]
pub unsafe extern "C" fn Use_Target_Help(
    mut ent: *mut edict_t,
    mut other: *mut edict_t,
    mut activator: *mut edict_t,
) {
    if (*ent).spawnflags & 1 as libc::c_int != 0 {
        strncpy(
            (game.helpmessage1).as_mut_ptr(),
            (*ent).message,
            (::std::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        );
    } else {
        strncpy(
            (game.helpmessage2).as_mut_ptr(),
            (*ent).message,
            (::std::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        );
    }
    game.helpchanged += 1;
}
#[no_mangle]
pub unsafe extern "C" fn SP_target_help(mut ent: *mut edict_t) {
    if (*deathmatch).value != 0. {
        G_FreeEdict(ent);
        return;
    }
    if ((*ent).message).is_null() {
        (gi.dprintf)
            .expect(
                "non-null function pointer",
            )(
            b"%s with no message at %s\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            (*ent).classname,
            vtos(((*ent).s.origin).as_mut_ptr()),
        );
        G_FreeEdict(ent);
        return;
    }
    let ref mut fresh2 = (*ent).use_0;
    *fresh2 = Some(
        Use_Target_Help
            as unsafe extern "C" fn(*mut edict_t, *mut edict_t, *mut edict_t) -> (),
    );
}
#[no_mangle]
pub unsafe extern "C" fn use_target_secret(
    mut ent: *mut edict_t,
    mut other: *mut edict_t,
    mut activator: *mut edict_t,
) {
    (gi.sound)
        .expect(
            "non-null function pointer",
        )(
        ent,
        2 as libc::c_int,
        (*ent).noise_index,
        1 as libc::c_int as libc::c_float,
        1 as libc::c_int as libc::c_float,
        0 as libc::c_int as libc::c_float,
    );
    level.found_secrets += 1;
    G_UseTargets(ent, activator);
    G_FreeEdict(ent);
}
#[no_mangle]
pub unsafe extern "C" fn SP_target_secret(mut ent: *mut edict_t) {
    if (*deathmatch).value != 0. {
        G_FreeEdict(ent);
        return;
    }
    let ref mut fresh3 = (*ent).use_0;
    *fresh3 = Some(
        use_target_secret
            as unsafe extern "C" fn(*mut edict_t, *mut edict_t, *mut edict_t) -> (),
    );
    if (st.noise).is_null() {
        st
            .noise = b"misc/secret.wav\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char;
    }
    (*ent).noise_index = (gi.soundindex).expect("non-null function pointer")(st.noise);
    (*ent).svflags = 0x1 as libc::c_int;
    level.total_secrets += 1;
    if stricmp(
        (level.mapname).as_mut_ptr(),
        b"mine3\0" as *const u8 as *const libc::c_char,
    ) == 0
        && (*ent).s.origin[0 as libc::c_int as usize]
            == 280 as libc::c_int as libc::c_float
        && (*ent).s.origin[1 as libc::c_int as usize]
            == -(2048 as libc::c_int) as libc::c_float
        && (*ent).s.origin[2 as libc::c_int as usize]
            == -(624 as libc::c_int) as libc::c_float
    {
        let ref mut fresh4 = (*ent).message;
        *fresh4 = b"You have found a secret area.\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char;
    }
}
#[no_mangle]
pub unsafe extern "C" fn use_target_goal(
    mut ent: *mut edict_t,
    mut other: *mut edict_t,
    mut activator: *mut edict_t,
) {
    (gi.sound)
        .expect(
            "non-null function pointer",
        )(
        ent,
        2 as libc::c_int,
        (*ent).noise_index,
        1 as libc::c_int as libc::c_float,
        1 as libc::c_int as libc::c_float,
        0 as libc::c_int as libc::c_float,
    );
    level.found_goals += 1;
    if level.found_goals == level.total_goals {
        (gi.configstring)
            .expect(
                "non-null function pointer",
            )(
            1 as libc::c_int,
            b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
    G_UseTargets(ent, activator);
    G_FreeEdict(ent);
}
#[no_mangle]
pub unsafe extern "C" fn SP_target_goal(mut ent: *mut edict_t) {
    if (*deathmatch).value != 0. {
        G_FreeEdict(ent);
        return;
    }
    let ref mut fresh5 = (*ent).use_0;
    *fresh5 = Some(
        use_target_goal
            as unsafe extern "C" fn(*mut edict_t, *mut edict_t, *mut edict_t) -> (),
    );
    if (st.noise).is_null() {
        st
            .noise = b"misc/secret.wav\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char;
    }
    (*ent).noise_index = (gi.soundindex).expect("non-null function pointer")(st.noise);
    (*ent).svflags = 0x1 as libc::c_int;
    level.total_goals += 1;
}
#[no_mangle]
pub unsafe extern "C" fn target_explosion_explode(mut self_0: *mut edict_t) {
    let mut save: libc::c_float = 0.;
    (gi.WriteByte).expect("non-null function pointer")(3 as libc::c_int);
    (gi.WriteByte).expect("non-null function pointer")(TE_EXPLOSION1 as libc::c_int);
    (gi.WritePosition)
        .expect("non-null function pointer")(((*self_0).s.origin).as_mut_ptr());
    (gi.multicast)
        .expect(
            "non-null function pointer",
        )(((*self_0).s.origin).as_mut_ptr(), MULTICAST_PHS);
    T_RadiusDamage(
        self_0,
        (*self_0).activator,
        (*self_0).dmg as libc::c_float,
        0 as *mut edict_t,
        ((*self_0).dmg + 40 as libc::c_int) as libc::c_float,
        25 as libc::c_int,
    );
    save = (*self_0).delay;
    (*self_0).delay = 0 as libc::c_int as libc::c_float;
    G_UseTargets(self_0, (*self_0).activator);
    (*self_0).delay = save;
}
#[no_mangle]
pub unsafe extern "C" fn use_target_explosion(
    mut self_0: *mut edict_t,
    mut other: *mut edict_t,
    mut activator: *mut edict_t,
) {
    let ref mut fresh6 = (*self_0).activator;
    *fresh6 = activator;
    if (*self_0).delay == 0. {
        target_explosion_explode(self_0);
        return;
    }
    let ref mut fresh7 = (*self_0).think;
    *fresh7 = Some(target_explosion_explode as unsafe extern "C" fn(*mut edict_t) -> ());
    (*self_0).nextthink = level.time + (*self_0).delay;
}
#[no_mangle]
pub unsafe extern "C" fn SP_target_explosion(mut ent: *mut edict_t) {
    let ref mut fresh8 = (*ent).use_0;
    *fresh8 = Some(
        use_target_explosion
            as unsafe extern "C" fn(*mut edict_t, *mut edict_t, *mut edict_t) -> (),
    );
    (*ent).svflags = 0x1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn use_target_changelevel(
    mut self_0: *mut edict_t,
    mut other: *mut edict_t,
    mut activator: *mut edict_t,
) {
    if level.intermissiontime != 0. {
        return;
    }
    if (*deathmatch).value == 0. && (*coop).value == 0. {
        if (*g_edicts.offset(1 as libc::c_int as isize)).health <= 0 as libc::c_int {
            return;
        }
    }
    if (*deathmatch).value != 0.
        && (*dmflags).value as libc::c_int & 0x1000 as libc::c_int == 0
        && other != &mut *g_edicts.offset(0 as libc::c_int as isize) as *mut edict_t
    {
        T_Damage(
            other,
            self_0,
            self_0,
            vec3_origin.as_mut_ptr(),
            ((*other).s.origin).as_mut_ptr(),
            vec3_origin.as_mut_ptr(),
            10 as libc::c_int * (*other).max_health,
            1000 as libc::c_int,
            0 as libc::c_int,
            28 as libc::c_int,
        );
        return;
    }
    if (*deathmatch).value != 0. {
        if !activator.is_null() && !((*activator).client).is_null() {
            (gi.bprintf)
                .expect(
                    "non-null function pointer",
                )(
                2 as libc::c_int,
                b"%s exited the level.\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ((*(*activator).client).pers.netname).as_mut_ptr(),
            );
        }
    }
    if !(strstr((*self_0).map, b"*\0" as *const u8 as *const libc::c_char)).is_null() {
        game.serverflags &= !(0xff as libc::c_int);
    }
    BeginIntermission(self_0);
}
#[no_mangle]
pub unsafe extern "C" fn SP_target_changelevel(mut ent: *mut edict_t) {
    if ((*ent).map).is_null() {
        (gi.dprintf)
            .expect(
                "non-null function pointer",
            )(
            b"target_changelevel with no map at %s\n\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            vtos(((*ent).s.origin).as_mut_ptr()),
        );
        G_FreeEdict(ent);
        return;
    }
    if stricmp(
        (level.mapname).as_mut_ptr(),
        b"fact1\0" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int
        && stricmp((*ent).map, b"fact3\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
    {
        let ref mut fresh9 = (*ent).map;
        *fresh9 = b"fact3$secret1\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char;
    }
    let ref mut fresh10 = (*ent).use_0;
    *fresh10 = Some(
        use_target_changelevel
            as unsafe extern "C" fn(*mut edict_t, *mut edict_t, *mut edict_t) -> (),
    );
    (*ent).svflags = 0x1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn use_target_splash(
    mut self_0: *mut edict_t,
    mut other: *mut edict_t,
    mut activator: *mut edict_t,
) {
    (gi.WriteByte).expect("non-null function pointer")(3 as libc::c_int);
    (gi.WriteByte).expect("non-null function pointer")(TE_SPLASH as libc::c_int);
    (gi.WriteByte).expect("non-null function pointer")((*self_0).count);
    (gi.WritePosition)
        .expect("non-null function pointer")(((*self_0).s.origin).as_mut_ptr());
    (gi.WriteDir).expect("non-null function pointer")(((*self_0).movedir).as_mut_ptr());
    (gi.WriteByte).expect("non-null function pointer")((*self_0).sounds);
    (gi.multicast)
        .expect(
            "non-null function pointer",
        )(((*self_0).s.origin).as_mut_ptr(), MULTICAST_PVS);
    if (*self_0).dmg != 0 {
        T_RadiusDamage(
            self_0,
            activator,
            (*self_0).dmg as libc::c_float,
            0 as *mut edict_t,
            ((*self_0).dmg + 40 as libc::c_int) as libc::c_float,
            29 as libc::c_int,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn SP_target_splash(mut self_0: *mut edict_t) {
    let ref mut fresh11 = (*self_0).use_0;
    *fresh11 = Some(
        use_target_splash
            as unsafe extern "C" fn(*mut edict_t, *mut edict_t, *mut edict_t) -> (),
    );
    G_SetMovedir(((*self_0).s.angles).as_mut_ptr(), ((*self_0).movedir).as_mut_ptr());
    if (*self_0).count == 0 {
        (*self_0).count = 32 as libc::c_int;
    }
    (*self_0).svflags = 0x1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn use_target_spawner(
    mut self_0: *mut edict_t,
    mut other: *mut edict_t,
    mut activator: *mut edict_t,
) {
    let mut ent: *mut edict_t = 0 as *mut edict_t;
    ent = G_Spawn();
    let ref mut fresh12 = (*ent).classname;
    *fresh12 = (*self_0).target;
    (*ent)
        .s
        .origin[0 as libc::c_int
        as usize] = (*self_0).s.origin[0 as libc::c_int as usize];
    (*ent)
        .s
        .origin[1 as libc::c_int
        as usize] = (*self_0).s.origin[1 as libc::c_int as usize];
    (*ent)
        .s
        .origin[2 as libc::c_int
        as usize] = (*self_0).s.origin[2 as libc::c_int as usize];
    (*ent)
        .s
        .angles[0 as libc::c_int
        as usize] = (*self_0).s.angles[0 as libc::c_int as usize];
    (*ent)
        .s
        .angles[1 as libc::c_int
        as usize] = (*self_0).s.angles[1 as libc::c_int as usize];
    (*ent)
        .s
        .angles[2 as libc::c_int
        as usize] = (*self_0).s.angles[2 as libc::c_int as usize];
    ED_CallSpawn(ent);
    (gi.unlinkentity).expect("non-null function pointer")(ent);
    KillBox(ent);
    (gi.linkentity).expect("non-null function pointer")(ent);
    if (*self_0).speed != 0. {
        (*ent)
            .velocity[0 as libc::c_int
            as usize] = (*self_0).movedir[0 as libc::c_int as usize];
        (*ent)
            .velocity[1 as libc::c_int
            as usize] = (*self_0).movedir[1 as libc::c_int as usize];
        (*ent)
            .velocity[2 as libc::c_int
            as usize] = (*self_0).movedir[2 as libc::c_int as usize];
    }
}
#[no_mangle]
pub unsafe extern "C" fn SP_target_spawner(mut self_0: *mut edict_t) {
    let ref mut fresh13 = (*self_0).use_0;
    *fresh13 = Some(
        use_target_spawner
            as unsafe extern "C" fn(*mut edict_t, *mut edict_t, *mut edict_t) -> (),
    );
    (*self_0).svflags = 0x1 as libc::c_int;
    if (*self_0).speed != 0. {
        G_SetMovedir(
            ((*self_0).s.angles).as_mut_ptr(),
            ((*self_0).movedir).as_mut_ptr(),
        );
        VectorScale(
            ((*self_0).movedir).as_mut_ptr(),
            (*self_0).speed,
            ((*self_0).movedir).as_mut_ptr(),
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn use_target_blaster(
    mut self_0: *mut edict_t,
    mut other: *mut edict_t,
    mut activator: *mut edict_t,
) {
    let mut effect: libc::c_int = 0;
    if (*self_0).spawnflags & 2 as libc::c_int != 0 {
        effect = 0 as libc::c_int;
    } else if (*self_0).spawnflags & 1 as libc::c_int != 0 {
        effect = 0x40 as libc::c_int;
    } else {
        effect = 0x8 as libc::c_int;
    }
    fire_blaster(
        self_0,
        ((*self_0).s.origin).as_mut_ptr(),
        ((*self_0).movedir).as_mut_ptr(),
        (*self_0).dmg,
        (*self_0).speed as libc::c_int,
        0x8 as libc::c_int,
        33 as qboolean,
    );
    (gi.sound)
        .expect(
            "non-null function pointer",
        )(
        self_0,
        2 as libc::c_int,
        (*self_0).noise_index,
        1 as libc::c_int as libc::c_float,
        1 as libc::c_int as libc::c_float,
        0 as libc::c_int as libc::c_float,
    );
}
#[no_mangle]
pub unsafe extern "C" fn SP_target_blaster(mut self_0: *mut edict_t) {
    let ref mut fresh14 = (*self_0).use_0;
    *fresh14 = Some(
        use_target_blaster
            as unsafe extern "C" fn(*mut edict_t, *mut edict_t, *mut edict_t) -> (),
    );
    G_SetMovedir(((*self_0).s.angles).as_mut_ptr(), ((*self_0).movedir).as_mut_ptr());
    (*self_0)
        .noise_index = (gi.soundindex)
        .expect(
            "non-null function pointer",
        )(
        b"weapons/laser2.wav\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if (*self_0).dmg == 0 {
        (*self_0).dmg = 15 as libc::c_int;
    }
    if (*self_0).speed == 0. {
        (*self_0).speed = 1000 as libc::c_int as libc::c_float;
    }
    (*self_0).svflags = 0x1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn trigger_crosslevel_trigger_use(
    mut self_0: *mut edict_t,
    mut other: *mut edict_t,
    mut activator: *mut edict_t,
) {
    game.serverflags |= (*self_0).spawnflags;
    G_FreeEdict(self_0);
}
#[no_mangle]
pub unsafe extern "C" fn SP_target_crosslevel_trigger(mut self_0: *mut edict_t) {
    (*self_0).svflags = 0x1 as libc::c_int;
    let ref mut fresh15 = (*self_0).use_0;
    *fresh15 = Some(
        trigger_crosslevel_trigger_use
            as unsafe extern "C" fn(*mut edict_t, *mut edict_t, *mut edict_t) -> (),
    );
}
#[no_mangle]
pub unsafe extern "C" fn target_crosslevel_target_think(mut self_0: *mut edict_t) {
    if (*self_0).spawnflags
        == game.serverflags & 0xff as libc::c_int & (*self_0).spawnflags
    {
        G_UseTargets(self_0, self_0);
        G_FreeEdict(self_0);
    }
}
#[no_mangle]
pub unsafe extern "C" fn SP_target_crosslevel_target(mut self_0: *mut edict_t) {
    if (*self_0).delay == 0. {
        (*self_0).delay = 1 as libc::c_int as libc::c_float;
    }
    (*self_0).svflags = 0x1 as libc::c_int;
    let ref mut fresh16 = (*self_0).think;
    *fresh16 = Some(
        target_crosslevel_target_think as unsafe extern "C" fn(*mut edict_t) -> (),
    );
    (*self_0).nextthink = level.time + (*self_0).delay;
}
#[no_mangle]
pub unsafe extern "C" fn target_laser_think(mut self_0: *mut edict_t) {
    let mut ignore: *mut edict_t = 0 as *mut edict_t;
    let mut start: vec3_t = [0.; 3];
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
    let mut point: vec3_t = [0.; 3];
    let mut last_movedir: vec3_t = [0.; 3];
    let mut count: libc::c_int = 0;
    if (*self_0).spawnflags as libc::c_uint & 0x80000000 as libc::c_uint != 0 {
        count = 8 as libc::c_int;
    } else {
        count = 4 as libc::c_int;
    }
    if !((*self_0).enemy).is_null() {
        last_movedir[0 as libc::c_int
            as usize] = (*self_0).movedir[0 as libc::c_int as usize];
        last_movedir[1 as libc::c_int
            as usize] = (*self_0).movedir[1 as libc::c_int as usize];
        last_movedir[2 as libc::c_int
            as usize] = (*self_0).movedir[2 as libc::c_int as usize];
        VectorMA(
            ((*(*self_0).enemy).absmin).as_mut_ptr(),
            0.5f64 as libc::c_float,
            ((*(*self_0).enemy).size).as_mut_ptr(),
            point.as_mut_ptr(),
        );
        (*self_0)
            .movedir[0 as libc::c_int
            as usize] = point[0 as libc::c_int as usize]
            - (*self_0).s.origin[0 as libc::c_int as usize];
        (*self_0)
            .movedir[1 as libc::c_int
            as usize] = point[1 as libc::c_int as usize]
            - (*self_0).s.origin[1 as libc::c_int as usize];
        (*self_0)
            .movedir[2 as libc::c_int
            as usize] = point[2 as libc::c_int as usize]
            - (*self_0).s.origin[2 as libc::c_int as usize];
        VectorNormalize(((*self_0).movedir).as_mut_ptr());
        if VectorCompare(((*self_0).movedir).as_mut_ptr(), last_movedir.as_mut_ptr())
            == 0
        {
            let ref mut fresh17 = (*self_0).spawnflags;
            *fresh17 = (*fresh17 as libc::c_uint | 0x80000000 as libc::c_uint)
                as libc::c_int;
        }
    }
    ignore = self_0;
    start[0 as libc::c_int as usize] = (*self_0).s.origin[0 as libc::c_int as usize];
    start[1 as libc::c_int as usize] = (*self_0).s.origin[1 as libc::c_int as usize];
    start[2 as libc::c_int as usize] = (*self_0).s.origin[2 as libc::c_int as usize];
    VectorMA(
        start.as_mut_ptr(),
        2048 as libc::c_int as libc::c_float,
        ((*self_0).movedir).as_mut_ptr(),
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
        if (*tr.ent).takedamage != 0 && (*tr.ent).flags & 0x4 as libc::c_int == 0 {
            T_Damage(
                tr.ent,
                self_0,
                (*self_0).activator,
                ((*self_0).movedir).as_mut_ptr(),
                (tr.endpos).as_mut_ptr(),
                vec3_origin.as_mut_ptr(),
                (*self_0).dmg,
                1 as libc::c_int,
                0x4 as libc::c_int,
                30 as libc::c_int,
            );
        }
        if (*tr.ent).svflags & 0x4 as libc::c_int == 0 && ((*tr.ent).client).is_null() {
            if (*self_0).spawnflags as libc::c_uint & 0x80000000 as libc::c_uint != 0 {
                let ref mut fresh18 = (*self_0).spawnflags;
                *fresh18 = (*fresh18 as libc::c_uint & !(0x80000000 as libc::c_uint))
                    as libc::c_int;
                (gi.WriteByte).expect("non-null function pointer")(3 as libc::c_int);
                (gi.WriteByte)
                    .expect("non-null function pointer")(TE_LASER_SPARKS as libc::c_int);
                (gi.WriteByte).expect("non-null function pointer")(count);
                (gi.WritePosition)
                    .expect("non-null function pointer")((tr.endpos).as_mut_ptr());
                (gi.WriteDir)
                    .expect("non-null function pointer")((tr.plane.normal).as_mut_ptr());
                (gi.WriteByte).expect("non-null function pointer")((*self_0).s.skinnum);
                (gi.multicast)
                    .expect(
                        "non-null function pointer",
                    )((tr.endpos).as_mut_ptr(), MULTICAST_PVS);
            }
            break;
        } else {
            ignore = tr.ent;
            start[0 as libc::c_int as usize] = tr.endpos[0 as libc::c_int as usize];
            start[1 as libc::c_int as usize] = tr.endpos[1 as libc::c_int as usize];
            start[2 as libc::c_int as usize] = tr.endpos[2 as libc::c_int as usize];
        }
    }
    (*self_0)
        .s
        .old_origin[0 as libc::c_int as usize] = tr.endpos[0 as libc::c_int as usize];
    (*self_0)
        .s
        .old_origin[1 as libc::c_int as usize] = tr.endpos[1 as libc::c_int as usize];
    (*self_0)
        .s
        .old_origin[2 as libc::c_int as usize] = tr.endpos[2 as libc::c_int as usize];
    (*self_0).nextthink = (level.time as libc::c_double + 0.1f64) as libc::c_float;
}
#[no_mangle]
pub unsafe extern "C" fn target_laser_on(mut self_0: *mut edict_t) {
    if ((*self_0).activator).is_null() {
        let ref mut fresh19 = (*self_0).activator;
        *fresh19 = self_0;
    }
    let ref mut fresh20 = (*self_0).spawnflags;
    *fresh20 = (*fresh20 as libc::c_uint | 0x80000001 as libc::c_uint) as libc::c_int;
    (*self_0).svflags &= !(0x1 as libc::c_int);
    target_laser_think(self_0);
}
#[no_mangle]
pub unsafe extern "C" fn target_laser_off(mut self_0: *mut edict_t) {
    (*self_0).spawnflags &= !(1 as libc::c_int);
    (*self_0).svflags |= 0x1 as libc::c_int;
    (*self_0).nextthink = 0 as libc::c_int as libc::c_float;
}
#[no_mangle]
pub unsafe extern "C" fn target_laser_use(
    mut self_0: *mut edict_t,
    mut other: *mut edict_t,
    mut activator: *mut edict_t,
) {
    let ref mut fresh21 = (*self_0).activator;
    *fresh21 = activator;
    if (*self_0).spawnflags & 1 as libc::c_int != 0 {
        target_laser_off(self_0);
    } else {
        target_laser_on(self_0);
    };
}
#[no_mangle]
pub unsafe extern "C" fn target_laser_start(mut self_0: *mut edict_t) {
    let mut ent: *mut edict_t = 0 as *mut edict_t;
    (*self_0).movetype = MOVETYPE_NONE as libc::c_int;
    (*self_0).solid = SOLID_NOT;
    (*self_0).s.renderfx |= 128 as libc::c_int | 32 as libc::c_int;
    (*self_0).s.modelindex = 1 as libc::c_int;
    if (*self_0).spawnflags & 64 as libc::c_int != 0 {
        (*self_0).s.frame = 16 as libc::c_int;
    } else {
        (*self_0).s.frame = 4 as libc::c_int;
    }
    if (*self_0).spawnflags & 2 as libc::c_int != 0 {
        (*self_0).s.skinnum = 0xf2f2f0f0 as libc::c_uint as libc::c_int;
    } else if (*self_0).spawnflags & 4 as libc::c_int != 0 {
        (*self_0).s.skinnum = 0xd0d1d2d3 as libc::c_uint as libc::c_int;
    } else if (*self_0).spawnflags & 8 as libc::c_int != 0 {
        (*self_0).s.skinnum = 0xf3f3f1f1 as libc::c_uint as libc::c_int;
    } else if (*self_0).spawnflags & 16 as libc::c_int != 0 {
        (*self_0).s.skinnum = 0xdcdddedf as libc::c_uint as libc::c_int;
    } else if (*self_0).spawnflags & 32 as libc::c_int != 0 {
        (*self_0).s.skinnum = 0xe0e1e2e3 as libc::c_uint as libc::c_int;
    }
    if ((*self_0).enemy).is_null() {
        if !((*self_0).target).is_null() {
            ent = G_Find(
                0 as *mut edict_t,
                &mut (*(0 as *mut edict_t)).targetname as *mut *mut libc::c_char
                    as libc::c_int,
                (*self_0).target,
            );
            if ent.is_null() {
                (gi.dprintf)
                    .expect(
                        "non-null function pointer",
                    )(
                    b"%s at %s: %s is a bad target\n\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    (*self_0).classname,
                    vtos(((*self_0).s.origin).as_mut_ptr()),
                    (*self_0).target,
                );
            }
            let ref mut fresh22 = (*self_0).enemy;
            *fresh22 = ent;
        } else {
            G_SetMovedir(
                ((*self_0).s.angles).as_mut_ptr(),
                ((*self_0).movedir).as_mut_ptr(),
            );
        }
    }
    let ref mut fresh23 = (*self_0).use_0;
    *fresh23 = Some(
        target_laser_use
            as unsafe extern "C" fn(*mut edict_t, *mut edict_t, *mut edict_t) -> (),
    );
    let ref mut fresh24 = (*self_0).think;
    *fresh24 = Some(target_laser_think as unsafe extern "C" fn(*mut edict_t) -> ());
    if (*self_0).dmg == 0 {
        (*self_0).dmg = 1 as libc::c_int;
    }
    (*self_0).mins[0 as libc::c_int as usize] = -(8 as libc::c_int) as vec_t;
    (*self_0).mins[1 as libc::c_int as usize] = -(8 as libc::c_int) as vec_t;
    (*self_0).mins[2 as libc::c_int as usize] = -(8 as libc::c_int) as vec_t;
    (*self_0).maxs[0 as libc::c_int as usize] = 8 as libc::c_int as vec_t;
    (*self_0).maxs[1 as libc::c_int as usize] = 8 as libc::c_int as vec_t;
    (*self_0).maxs[2 as libc::c_int as usize] = 8 as libc::c_int as vec_t;
    (gi.linkentity).expect("non-null function pointer")(self_0);
    if (*self_0).spawnflags & 1 as libc::c_int != 0 {
        target_laser_on(self_0);
    } else {
        target_laser_off(self_0);
    };
}
#[no_mangle]
pub unsafe extern "C" fn SP_target_laser(mut self_0: *mut edict_t) {
    let ref mut fresh25 = (*self_0).think;
    *fresh25 = Some(target_laser_start as unsafe extern "C" fn(*mut edict_t) -> ());
    (*self_0).nextthink = level.time + 1 as libc::c_int as libc::c_float;
}
#[no_mangle]
pub unsafe extern "C" fn target_lightramp_think(mut self_0: *mut edict_t) {
    let mut style: [libc::c_char; 2] = [0; 2];
    style[0 as libc::c_int
        as usize] = (('a' as i32 as libc::c_float
        + (*self_0).movedir[0 as libc::c_int as usize]) as libc::c_double
        + (level.time - (*self_0).timestamp) as libc::c_double / 0.1f64
            * (*self_0).movedir[2 as libc::c_int as usize] as libc::c_double)
        as libc::c_char;
    style[1 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    (gi.configstring)
        .expect(
            "non-null function pointer",
        )(
        32 as libc::c_int + 256 as libc::c_int + 256 as libc::c_int + 256 as libc::c_int
            + (*(*self_0).enemy).style,
        style.as_mut_ptr(),
    );
    if level.time - (*self_0).timestamp < (*self_0).speed {
        (*self_0).nextthink = (level.time as libc::c_double + 0.1f64) as libc::c_float;
    } else if (*self_0).spawnflags & 1 as libc::c_int != 0 {
        let mut temp: libc::c_char = 0;
        temp = (*self_0).movedir[0 as libc::c_int as usize] as libc::c_char;
        (*self_0)
            .movedir[0 as libc::c_int
            as usize] = (*self_0).movedir[1 as libc::c_int as usize];
        (*self_0).movedir[1 as libc::c_int as usize] = temp as vec_t;
        let ref mut fresh26 = (*self_0).movedir[2 as libc::c_int as usize];
        *fresh26 *= -(1 as libc::c_int) as libc::c_float;
    }
}
#[no_mangle]
pub unsafe extern "C" fn target_lightramp_use(
    mut self_0: *mut edict_t,
    mut other: *mut edict_t,
    mut activator: *mut edict_t,
) {
    if ((*self_0).enemy).is_null() {
        let mut e: *mut edict_t = 0 as *mut edict_t;
        e = 0 as *mut edict_t;
        loop {
            e = G_Find(
                e,
                &mut (*(0 as *mut edict_t)).targetname as *mut *mut libc::c_char
                    as libc::c_int,
                (*self_0).target,
            );
            if e.is_null() {
                break;
            }
            if strcmp((*e).classname, b"light\0" as *const u8 as *const libc::c_char)
                != 0 as libc::c_int
            {
                (gi.dprintf)
                    .expect(
                        "non-null function pointer",
                    )(
                    b"%s at %s \0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    (*self_0).classname,
                    vtos(((*self_0).s.origin).as_mut_ptr()),
                );
                (gi.dprintf)
                    .expect(
                        "non-null function pointer",
                    )(
                    b"target %s (%s at %s) is not a light\n\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    (*self_0).target,
                    (*e).classname,
                    vtos(((*e).s.origin).as_mut_ptr()),
                );
            } else {
                let ref mut fresh27 = (*self_0).enemy;
                *fresh27 = e;
            }
        }
        if ((*self_0).enemy).is_null() {
            (gi.dprintf)
                .expect(
                    "non-null function pointer",
                )(
                b"%s target %s not found at %s\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                (*self_0).classname,
                (*self_0).target,
                vtos(((*self_0).s.origin).as_mut_ptr()),
            );
            G_FreeEdict(self_0);
            return;
        }
    }
    (*self_0).timestamp = level.time;
    target_lightramp_think(self_0);
}
#[no_mangle]
pub unsafe extern "C" fn SP_target_lightramp(mut self_0: *mut edict_t) {
    if ((*self_0).message).is_null()
        || strlen((*self_0).message) != 2 as libc::c_int as libc::c_ulong
        || (*((*self_0).message).offset(0 as libc::c_int as isize) as libc::c_int)
            < 'a' as i32
        || *((*self_0).message).offset(0 as libc::c_int as isize) as libc::c_int
            > 'z' as i32
        || (*((*self_0).message).offset(1 as libc::c_int as isize) as libc::c_int)
            < 'a' as i32
        || *((*self_0).message).offset(1 as libc::c_int as isize) as libc::c_int
            > 'z' as i32
        || *((*self_0).message).offset(0 as libc::c_int as isize) as libc::c_int
            == *((*self_0).message).offset(1 as libc::c_int as isize) as libc::c_int
    {
        (gi.dprintf)
            .expect(
                "non-null function pointer",
            )(
            b"target_lightramp has bad ramp (%s) at %s\n\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            (*self_0).message,
            vtos(((*self_0).s.origin).as_mut_ptr()),
        );
        G_FreeEdict(self_0);
        return;
    }
    if (*deathmatch).value != 0. {
        G_FreeEdict(self_0);
        return;
    }
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
    (*self_0).svflags |= 0x1 as libc::c_int;
    let ref mut fresh28 = (*self_0).use_0;
    *fresh28 = Some(
        target_lightramp_use
            as unsafe extern "C" fn(*mut edict_t, *mut edict_t, *mut edict_t) -> (),
    );
    let ref mut fresh29 = (*self_0).think;
    *fresh29 = Some(target_lightramp_think as unsafe extern "C" fn(*mut edict_t) -> ());
    (*self_0)
        .movedir[0 as libc::c_int
        as usize] = (*((*self_0).message).offset(0 as libc::c_int as isize)
        as libc::c_int - 'a' as i32) as vec_t;
    (*self_0)
        .movedir[1 as libc::c_int
        as usize] = (*((*self_0).message).offset(1 as libc::c_int as isize)
        as libc::c_int - 'a' as i32) as vec_t;
    (*self_0)
        .movedir[2 as libc::c_int
        as usize] = (((*self_0).movedir[1 as libc::c_int as usize]
        - (*self_0).movedir[0 as libc::c_int as usize]) as libc::c_double
        / ((*self_0).speed as libc::c_double / 0.1f64)) as vec_t;
}
#[no_mangle]
pub unsafe extern "C" fn target_earthquake_think(mut self_0: *mut edict_t) {
    let mut i: libc::c_int = 0;
    let mut e: *mut edict_t = 0 as *mut edict_t;
    if (*self_0).last_move_time < level.time {
        (gi.positioned_sound)
            .expect(
                "non-null function pointer",
            )(
            ((*self_0).s.origin).as_mut_ptr(),
            self_0,
            0 as libc::c_int,
            (*self_0).noise_index,
            1.0f64 as libc::c_float,
            0 as libc::c_int as libc::c_float,
            0 as libc::c_int as libc::c_float,
        );
        (*self_0)
            .last_move_time = (level.time as libc::c_double + 0.5f64) as libc::c_float;
    }
    i = 1 as libc::c_int;
    e = g_edicts.offset(i as isize);
    while i < globals.num_edicts {
        if !((*e).inuse as u64 == 0) {
            if !((*e).client).is_null() {
                if !((*e).groundentity).is_null() {
                    let ref mut fresh30 = (*e).groundentity;
                    *fresh30 = 0 as *mut edict_t;
                    let ref mut fresh31 = (*e).velocity[0 as libc::c_int as usize];
                    *fresh31 = (*fresh31 as libc::c_double
                        + 2.0f64
                            * (((rand() & 0x7fff as libc::c_int) as libc::c_float
                                / 0x7fff as libc::c_int as libc::c_float) as libc::c_double
                                - 0.5f64) * 150 as libc::c_int as libc::c_double) as vec_t;
                    let ref mut fresh32 = (*e).velocity[1 as libc::c_int as usize];
                    *fresh32 = (*fresh32 as libc::c_double
                        + 2.0f64
                            * (((rand() & 0x7fff as libc::c_int) as libc::c_float
                                / 0x7fff as libc::c_int as libc::c_float) as libc::c_double
                                - 0.5f64) * 150 as libc::c_int as libc::c_double) as vec_t;
                    (*e)
                        .velocity[2 as libc::c_int
                        as usize] = ((*self_0).speed as libc::c_double
                        * (100.0f64 / (*e).mass as libc::c_double)) as vec_t;
                }
            }
        }
        i += 1;
        e = e.offset(1);
    }
    if level.time < (*self_0).timestamp {
        (*self_0).nextthink = (level.time as libc::c_double + 0.1f64) as libc::c_float;
    }
}
#[no_mangle]
pub unsafe extern "C" fn target_earthquake_use(
    mut self_0: *mut edict_t,
    mut other: *mut edict_t,
    mut activator: *mut edict_t,
) {
    (*self_0).timestamp = level.time + (*self_0).count as libc::c_float;
    (*self_0).nextthink = (level.time as libc::c_double + 0.1f64) as libc::c_float;
    let ref mut fresh33 = (*self_0).activator;
    *fresh33 = activator;
    (*self_0).last_move_time = 0 as libc::c_int as libc::c_float;
}
#[no_mangle]
pub unsafe extern "C" fn SP_target_earthquake(mut self_0: *mut edict_t) {
    if ((*self_0).targetname).is_null() {
        (gi.dprintf)
            .expect(
                "non-null function pointer",
            )(
            b"untargeted %s at %s\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            (*self_0).classname,
            vtos(((*self_0).s.origin).as_mut_ptr()),
        );
    }
    if (*self_0).count == 0 {
        (*self_0).count = 5 as libc::c_int;
    }
    if (*self_0).speed == 0. {
        (*self_0).speed = 200 as libc::c_int as libc::c_float;
    }
    (*self_0).svflags |= 0x1 as libc::c_int;
    let ref mut fresh34 = (*self_0).think;
    *fresh34 = Some(target_earthquake_think as unsafe extern "C" fn(*mut edict_t) -> ());
    let ref mut fresh35 = (*self_0).use_0;
    *fresh35 = Some(
        target_earthquake_use
            as unsafe extern "C" fn(*mut edict_t, *mut edict_t, *mut edict_t) -> (),
    );
    (*self_0)
        .noise_index = (gi.soundindex)
        .expect(
            "non-null function pointer",
        )(b"world/quake.wav\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
}
