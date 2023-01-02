#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn rand() -> libc::c_int;
    fn VectorMA(
        veca: *mut vec_t,
        scale: libc::c_float,
        vecb: *mut vec_t,
        vecc: *mut vec_t,
    );
    fn VectorLength(v: *mut vec_t) -> vec_t;
    fn AngleVectors(
        angles: *mut vec_t,
        forward: *mut vec_t,
        right: *mut vec_t,
        up: *mut vec_t,
    );
    static mut monster_flash_offset: [vec3_t; 0];
    static mut level: level_locals_t;
    static mut gi: game_import_t;
    static mut g_edicts: *mut edict_t;
    static mut deathmatch: *mut cvar_t;
    static mut skill: *mut cvar_t;
    fn G_ProjectSource(
        point: *mut vec_t,
        distance: *mut vec_t,
        forward: *mut vec_t,
        right: *mut vec_t,
        result: *mut vec_t,
    );
    fn findradius(
        from: *mut edict_t,
        org: *mut vec_t,
        rad: libc::c_float,
    ) -> *mut edict_t;
    fn G_FreeEdict(e: *mut edict_t);
    fn vectoangles(vec: *mut vec_t, angles: *mut vec_t);
    fn monster_fire_blaster(
        self_0: *mut edict_t,
        start: *mut vec_t,
        dir: *mut vec_t,
        damage: libc::c_int,
        speed: libc::c_int,
        flashtype: libc::c_int,
        effect: libc::c_int,
    );
    fn walkmonster_start(self_0: *mut edict_t);
    fn M_CheckAttack(self_0: *mut edict_t) -> qboolean;
    fn ThrowHead(
        self_0: *mut edict_t,
        gibname: *mut libc::c_char,
        damage: libc::c_int,
        type_0: libc::c_int,
    );
    fn ThrowGib(
        self_0: *mut edict_t,
        gibname: *mut libc::c_char,
        damage: libc::c_int,
        type_0: libc::c_int,
    );
    fn ai_stand(self_0: *mut edict_t, dist: libc::c_float);
    fn ai_move(self_0: *mut edict_t, dist: libc::c_float);
    fn ai_walk(self_0: *mut edict_t, dist: libc::c_float);
    fn ai_run(self_0: *mut edict_t, dist: libc::c_float);
    fn ai_charge(self_0: *mut edict_t, dist: libc::c_float);
    fn FoundTarget(self_0: *mut edict_t);
    fn visible(self_0: *mut edict_t, other: *mut edict_t) -> qboolean;
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
pub const DAMAGE_AIM: C2RustUnnamed_0 = 2;
pub const DAMAGE_YES: C2RustUnnamed_0 = 1;
pub const DAMAGE_NO: C2RustUnnamed_0 = 0;
pub type C2RustUnnamed_1 = libc::c_uint;
pub const MOVETYPE_BOUNCE: C2RustUnnamed_1 = 9;
pub const MOVETYPE_FLYMISSILE: C2RustUnnamed_1 = 8;
pub const MOVETYPE_TOSS: C2RustUnnamed_1 = 7;
pub const MOVETYPE_FLY: C2RustUnnamed_1 = 6;
pub const MOVETYPE_STEP: C2RustUnnamed_1 = 5;
pub const MOVETYPE_WALK: C2RustUnnamed_1 = 4;
pub const MOVETYPE_STOP: C2RustUnnamed_1 = 3;
pub const MOVETYPE_PUSH: C2RustUnnamed_1 = 2;
pub const MOVETYPE_NOCLIP: C2RustUnnamed_1 = 1;
pub const MOVETYPE_NONE: C2RustUnnamed_1 = 0;
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
static mut sound_idle1: libc::c_int = 0;
static mut sound_pain1: libc::c_int = 0;
static mut sound_pain2: libc::c_int = 0;
static mut sound_die: libc::c_int = 0;
static mut sound_sight: libc::c_int = 0;
static mut sound_search: libc::c_int = 0;
static mut sound_hook_launch: libc::c_int = 0;
static mut sound_hook_hit: libc::c_int = 0;
static mut sound_hook_heal: libc::c_int = 0;
static mut sound_hook_retract: libc::c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn medic_FindDeadMonster(
    mut self_0: *mut edict_t,
) -> *mut edict_t {
    let mut ent: *mut edict_t = 0 as *mut edict_t;
    let mut best: *mut edict_t = 0 as *mut edict_t;
    loop {
        ent = findradius(
            ent,
            ((*self_0).s.origin).as_mut_ptr(),
            1024 as libc::c_int as libc::c_float,
        );
        if ent.is_null() {
            break;
        }
        if ent == self_0 {
            continue;
        }
        if (*ent).svflags & 0x4 as libc::c_int == 0 {
            continue;
        }
        if (*ent).monsterinfo.aiflags & 0x100 as libc::c_int != 0 {
            continue;
        }
        if !((*ent).owner).is_null() {
            continue;
        }
        if (*ent).health > 0 as libc::c_int {
            continue;
        }
        if (*ent).nextthink != 0. {
            continue;
        }
        if visible(self_0, ent) as u64 == 0 {
            continue;
        }
        if best.is_null() {
            best = ent;
        } else {
            if (*ent).max_health <= (*best).max_health {
                continue;
            }
            best = ent;
        }
    }
    return best;
}
#[no_mangle]
pub unsafe extern "C" fn medic_idle(mut self_0: *mut edict_t) {
    let mut ent: *mut edict_t = 0 as *mut edict_t;
    (gi.sound)
        .expect(
            "non-null function pointer",
        )(
        self_0,
        2 as libc::c_int,
        sound_idle1,
        1 as libc::c_int as libc::c_float,
        2 as libc::c_int as libc::c_float,
        0 as libc::c_int as libc::c_float,
    );
    ent = medic_FindDeadMonster(self_0);
    if !ent.is_null() {
        let ref mut fresh0 = (*self_0).enemy;
        *fresh0 = ent;
        let ref mut fresh1 = (*(*self_0).enemy).owner;
        *fresh1 = self_0;
        (*self_0).monsterinfo.aiflags |= 0x2000 as libc::c_int;
        FoundTarget(self_0);
    }
}
#[no_mangle]
pub unsafe extern "C" fn medic_search(mut self_0: *mut edict_t) {
    let mut ent: *mut edict_t = 0 as *mut edict_t;
    (gi.sound)
        .expect(
            "non-null function pointer",
        )(
        self_0,
        2 as libc::c_int,
        sound_search,
        1 as libc::c_int as libc::c_float,
        2 as libc::c_int as libc::c_float,
        0 as libc::c_int as libc::c_float,
    );
    if ((*self_0).oldenemy).is_null() {
        ent = medic_FindDeadMonster(self_0);
        if !ent.is_null() {
            let ref mut fresh2 = (*self_0).oldenemy;
            *fresh2 = (*self_0).enemy;
            let ref mut fresh3 = (*self_0).enemy;
            *fresh3 = ent;
            let ref mut fresh4 = (*(*self_0).enemy).owner;
            *fresh4 = self_0;
            (*self_0).monsterinfo.aiflags |= 0x2000 as libc::c_int;
            FoundTarget(self_0);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn medic_sight(mut self_0: *mut edict_t, mut other: *mut edict_t) {
    (gi.sound)
        .expect(
            "non-null function pointer",
        )(
        self_0,
        2 as libc::c_int,
        sound_sight,
        1 as libc::c_int as libc::c_float,
        1 as libc::c_int as libc::c_float,
        0 as libc::c_int as libc::c_float,
    );
}
#[no_mangle]
pub static mut medic_frames_stand: [mframe_t; 90] = unsafe {
    [
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_stand as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: Some(medic_idle as unsafe extern "C" fn(*mut edict_t) -> ()),
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_stand as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_stand as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_stand as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_stand as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_stand as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_stand as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_stand as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_stand as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_stand as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_stand as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_stand as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_stand as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_stand as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_stand as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_stand as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_stand as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_stand as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_stand as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_stand as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_stand as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_stand as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_stand as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_stand as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_stand as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_stand as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_stand as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_stand as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_stand as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_stand as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_stand as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_stand as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_stand as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_stand as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_stand as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_stand as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_stand as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_stand as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_stand as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_stand as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_stand as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_stand as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_stand as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_stand as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_stand as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_stand as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_stand as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_stand as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_stand as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_stand as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_stand as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_stand as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_stand as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_stand as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_stand as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_stand as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_stand as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_stand as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_stand as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_stand as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_stand as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_stand as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_stand as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_stand as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_stand as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_stand as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_stand as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_stand as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_stand as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_stand as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_stand as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_stand as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_stand as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_stand as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_stand as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_stand as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_stand as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_stand as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_stand as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_stand as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_stand as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_stand as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_stand as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_stand as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_stand as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_stand as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_stand as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_stand as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_stand as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_stand as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
    ]
};
#[no_mangle]
pub static mut medic_move_stand: mmove_t = unsafe {
    {
        let mut init = mmove_t {
            firstframe: 12 as libc::c_int,
            lastframe: 101 as libc::c_int,
            frame: medic_frames_stand.as_ptr() as *mut _,
            endfunc: None,
        };
        init
    }
};
#[no_mangle]
pub unsafe extern "C" fn medic_stand(mut self_0: *mut edict_t) {
    let ref mut fresh5 = (*self_0).monsterinfo.currentmove;
    *fresh5 = &mut medic_move_stand;
}
#[no_mangle]
pub static mut medic_frames_walk: [mframe_t; 12] = unsafe {
    [
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_walk as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 6.2f64 as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_walk as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 18.1f64 as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_walk as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 1 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_walk as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 9 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_walk as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 10 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_walk as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 9 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_walk as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 11 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_walk as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 11.6f64 as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_walk as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 2 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_walk as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 9.9f64 as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_walk as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 14 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_walk as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 9.3f64 as libc::c_float,
                thinkfunc: None,
            };
            init
        },
    ]
};
#[no_mangle]
pub static mut medic_move_walk: mmove_t = unsafe {
    {
        let mut init = mmove_t {
            firstframe: 0 as libc::c_int,
            lastframe: 11 as libc::c_int,
            frame: medic_frames_walk.as_ptr() as *mut _,
            endfunc: None,
        };
        init
    }
};
#[no_mangle]
pub unsafe extern "C" fn medic_walk(mut self_0: *mut edict_t) {
    let ref mut fresh6 = (*self_0).monsterinfo.currentmove;
    *fresh6 = &mut medic_move_walk;
}
#[no_mangle]
pub static mut medic_frames_run: [mframe_t; 6] = unsafe {
    [
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_run as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 18 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_run as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 22.5f64 as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_run as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 25.4f64 as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_run as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 23.4f64 as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_run as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 24 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_run as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 35.6f64 as libc::c_float,
                thinkfunc: None,
            };
            init
        },
    ]
};
#[no_mangle]
pub static mut medic_move_run: mmove_t = unsafe {
    {
        let mut init = mmove_t {
            firstframe: 102 as libc::c_int,
            lastframe: 107 as libc::c_int,
            frame: medic_frames_run.as_ptr() as *mut _,
            endfunc: None,
        };
        init
    }
};
#[no_mangle]
pub unsafe extern "C" fn medic_run(mut self_0: *mut edict_t) {
    if (*self_0).monsterinfo.aiflags & 0x2000 as libc::c_int == 0 {
        let mut ent: *mut edict_t = 0 as *mut edict_t;
        ent = medic_FindDeadMonster(self_0);
        if !ent.is_null() {
            let ref mut fresh7 = (*self_0).oldenemy;
            *fresh7 = (*self_0).enemy;
            let ref mut fresh8 = (*self_0).enemy;
            *fresh8 = ent;
            let ref mut fresh9 = (*(*self_0).enemy).owner;
            *fresh9 = self_0;
            (*self_0).monsterinfo.aiflags |= 0x2000 as libc::c_int;
            FoundTarget(self_0);
            return;
        }
    }
    if (*self_0).monsterinfo.aiflags & 0x1 as libc::c_int != 0 {
        let ref mut fresh10 = (*self_0).monsterinfo.currentmove;
        *fresh10 = &mut medic_move_stand;
    } else {
        let ref mut fresh11 = (*self_0).monsterinfo.currentmove;
        *fresh11 = &mut medic_move_run;
    };
}
#[no_mangle]
pub static mut medic_frames_pain1: [mframe_t; 8] = unsafe {
    [
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_move as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_move as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_move as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_move as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_move as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_move as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_move as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_move as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
    ]
};
#[no_mangle]
pub static mut medic_move_pain1: mmove_t = unsafe {
    {
        let mut init = mmove_t {
            firstframe: 108 as libc::c_int,
            lastframe: 115 as libc::c_int,
            frame: medic_frames_pain1.as_ptr() as *mut _,
            endfunc: Some(medic_run as unsafe extern "C" fn(*mut edict_t) -> ()),
        };
        init
    }
};
#[no_mangle]
pub static mut medic_frames_pain2: [mframe_t; 15] = unsafe {
    [
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_move as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_move as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_move as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_move as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_move as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_move as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_move as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_move as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_move as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_move as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_move as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_move as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_move as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_move as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_move as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
    ]
};
#[no_mangle]
pub static mut medic_move_pain2: mmove_t = unsafe {
    {
        let mut init = mmove_t {
            firstframe: 116 as libc::c_int,
            lastframe: 130 as libc::c_int,
            frame: medic_frames_pain2.as_ptr() as *mut _,
            endfunc: Some(medic_run as unsafe extern "C" fn(*mut edict_t) -> ()),
        };
        init
    }
};
#[no_mangle]
pub unsafe extern "C" fn medic_pain(
    mut self_0: *mut edict_t,
    mut other: *mut edict_t,
    mut kick: libc::c_float,
    mut damage: libc::c_int,
) {
    if (*self_0).health < (*self_0).max_health / 2 as libc::c_int {
        (*self_0).s.skinnum = 1 as libc::c_int;
    }
    if level.time < (*self_0).pain_debounce_time {
        return;
    }
    (*self_0).pain_debounce_time = level.time + 3 as libc::c_int as libc::c_float;
    if (*skill).value == 3 as libc::c_int as libc::c_float {
        return;
    }
    if (((rand() & 0x7fff as libc::c_int) as libc::c_float
        / 0x7fff as libc::c_int as libc::c_float) as libc::c_double) < 0.5f64
    {
        let ref mut fresh12 = (*self_0).monsterinfo.currentmove;
        *fresh12 = &mut medic_move_pain1;
        (gi.sound)
            .expect(
                "non-null function pointer",
            )(
            self_0,
            2 as libc::c_int,
            sound_pain1,
            1 as libc::c_int as libc::c_float,
            1 as libc::c_int as libc::c_float,
            0 as libc::c_int as libc::c_float,
        );
    } else {
        let ref mut fresh13 = (*self_0).monsterinfo.currentmove;
        *fresh13 = &mut medic_move_pain2;
        (gi.sound)
            .expect(
                "non-null function pointer",
            )(
            self_0,
            2 as libc::c_int,
            sound_pain2,
            1 as libc::c_int as libc::c_float,
            1 as libc::c_int as libc::c_float,
            0 as libc::c_int as libc::c_float,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn medic_fire_blaster(mut self_0: *mut edict_t) {
    let mut start: vec3_t = [0.; 3];
    let mut forward: vec3_t = [0.; 3];
    let mut right: vec3_t = [0.; 3];
    let mut end: vec3_t = [0.; 3];
    let mut dir: vec3_t = [0.; 3];
    let mut effect: libc::c_int = 0;
    if (*self_0).s.frame == 185 as libc::c_int || (*self_0).s.frame == 188 as libc::c_int
    {
        effect = 0x8 as libc::c_int;
    } else if (*self_0).s.frame == 195 as libc::c_int
        || (*self_0).s.frame == 198 as libc::c_int
        || (*self_0).s.frame == 201 as libc::c_int
        || (*self_0).s.frame == 204 as libc::c_int
    {
        effect = 0x40 as libc::c_int;
    } else {
        effect = 0 as libc::c_int;
    }
    AngleVectors(
        ((*self_0).s.angles).as_mut_ptr(),
        forward.as_mut_ptr(),
        right.as_mut_ptr(),
        0 as *mut vec_t,
    );
    G_ProjectSource(
        ((*self_0).s.origin).as_mut_ptr(),
        (*monster_flash_offset.as_mut_ptr().offset(60 as libc::c_int as isize))
            .as_mut_ptr(),
        forward.as_mut_ptr(),
        right.as_mut_ptr(),
        start.as_mut_ptr(),
    );
    end[0 as libc::c_int
        as usize] = (*(*self_0).enemy).s.origin[0 as libc::c_int as usize];
    end[1 as libc::c_int
        as usize] = (*(*self_0).enemy).s.origin[1 as libc::c_int as usize];
    end[2 as libc::c_int
        as usize] = (*(*self_0).enemy).s.origin[2 as libc::c_int as usize];
    end[2 as libc::c_int as usize] += (*(*self_0).enemy).viewheight as libc::c_float;
    dir[0 as libc::c_int
        as usize] = end[0 as libc::c_int as usize] - start[0 as libc::c_int as usize];
    dir[1 as libc::c_int
        as usize] = end[1 as libc::c_int as usize] - start[1 as libc::c_int as usize];
    dir[2 as libc::c_int
        as usize] = end[2 as libc::c_int as usize] - start[2 as libc::c_int as usize];
    monster_fire_blaster(
        self_0,
        start.as_mut_ptr(),
        dir.as_mut_ptr(),
        2 as libc::c_int,
        1000 as libc::c_int,
        60 as libc::c_int,
        effect,
    );
}
#[no_mangle]
pub unsafe extern "C" fn medic_dead(mut self_0: *mut edict_t) {
    (*self_0).mins[0 as libc::c_int as usize] = -(16 as libc::c_int) as vec_t;
    (*self_0).mins[1 as libc::c_int as usize] = -(16 as libc::c_int) as vec_t;
    (*self_0).mins[2 as libc::c_int as usize] = -(24 as libc::c_int) as vec_t;
    (*self_0).maxs[0 as libc::c_int as usize] = 16 as libc::c_int as vec_t;
    (*self_0).maxs[1 as libc::c_int as usize] = 16 as libc::c_int as vec_t;
    (*self_0).maxs[2 as libc::c_int as usize] = -(8 as libc::c_int) as vec_t;
    (*self_0).movetype = MOVETYPE_TOSS as libc::c_int;
    (*self_0).svflags |= 0x2 as libc::c_int;
    (*self_0).nextthink = 0 as libc::c_int as libc::c_float;
    (gi.linkentity).expect("non-null function pointer")(self_0);
}
#[no_mangle]
pub static mut medic_frames_death: [mframe_t; 30] = unsafe {
    [
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_move as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_move as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_move as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_move as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_move as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_move as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_move as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_move as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_move as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_move as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_move as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_move as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_move as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_move as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_move as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_move as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_move as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_move as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_move as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_move as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_move as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_move as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_move as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_move as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_move as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_move as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_move as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_move as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_move as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_move as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
    ]
};
#[no_mangle]
pub static mut medic_move_death: mmove_t = unsafe {
    {
        let mut init = mmove_t {
            firstframe: 147 as libc::c_int,
            lastframe: 176 as libc::c_int,
            frame: medic_frames_death.as_ptr() as *mut _,
            endfunc: Some(medic_dead as unsafe extern "C" fn(*mut edict_t) -> ()),
        };
        init
    }
};
#[no_mangle]
pub unsafe extern "C" fn medic_die(
    mut self_0: *mut edict_t,
    mut inflictor: *mut edict_t,
    mut attacker: *mut edict_t,
    mut damage: libc::c_int,
    mut point: *mut vec_t,
) {
    let mut n: libc::c_int = 0;
    if !((*self_0).enemy).is_null() && (*(*self_0).enemy).owner == self_0 {
        let ref mut fresh14 = (*(*self_0).enemy).owner;
        *fresh14 = 0 as *mut edict_t;
    }
    if (*self_0).health <= (*self_0).gib_health {
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
                b"misc/udeath.wav\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            ),
            1 as libc::c_int as libc::c_float,
            1 as libc::c_int as libc::c_float,
            0 as libc::c_int as libc::c_float,
        );
        n = 0 as libc::c_int;
        while n < 2 as libc::c_int {
            ThrowGib(
                self_0,
                b"models/objects/gibs/bone/tris.md2\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                damage,
                0 as libc::c_int,
            );
            n += 1;
        }
        n = 0 as libc::c_int;
        while n < 4 as libc::c_int {
            ThrowGib(
                self_0,
                b"models/objects/gibs/sm_meat/tris.md2\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
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
        (*self_0).deadflag = 2 as libc::c_int;
        return;
    }
    if (*self_0).deadflag == 2 as libc::c_int {
        return;
    }
    (gi.sound)
        .expect(
            "non-null function pointer",
        )(
        self_0,
        2 as libc::c_int,
        sound_die,
        1 as libc::c_int as libc::c_float,
        1 as libc::c_int as libc::c_float,
        0 as libc::c_int as libc::c_float,
    );
    (*self_0).deadflag = 2 as libc::c_int;
    (*self_0).takedamage = DAMAGE_YES as libc::c_int;
    let ref mut fresh15 = (*self_0).monsterinfo.currentmove;
    *fresh15 = &mut medic_move_death;
}
#[no_mangle]
pub unsafe extern "C" fn medic_duck_down(mut self_0: *mut edict_t) {
    if (*self_0).monsterinfo.aiflags & 0x800 as libc::c_int != 0 {
        return;
    }
    (*self_0).monsterinfo.aiflags |= 0x800 as libc::c_int;
    let ref mut fresh16 = (*self_0).maxs[2 as libc::c_int as usize];
    *fresh16 -= 32 as libc::c_int as libc::c_float;
    (*self_0).takedamage = DAMAGE_YES as libc::c_int;
    (*self_0).monsterinfo.pausetime = level.time + 1 as libc::c_int as libc::c_float;
    (gi.linkentity).expect("non-null function pointer")(self_0);
}
#[no_mangle]
pub unsafe extern "C" fn medic_duck_hold(mut self_0: *mut edict_t) {
    if level.time >= (*self_0).monsterinfo.pausetime {
        (*self_0).monsterinfo.aiflags &= !(0x80 as libc::c_int);
    } else {
        (*self_0).monsterinfo.aiflags |= 0x80 as libc::c_int;
    };
}
#[no_mangle]
pub unsafe extern "C" fn medic_duck_up(mut self_0: *mut edict_t) {
    (*self_0).monsterinfo.aiflags &= !(0x800 as libc::c_int);
    let ref mut fresh17 = (*self_0).maxs[2 as libc::c_int as usize];
    *fresh17 += 32 as libc::c_int as libc::c_float;
    (*self_0).takedamage = DAMAGE_AIM as libc::c_int;
    (gi.linkentity).expect("non-null function pointer")(self_0);
}
#[no_mangle]
pub static mut medic_frames_duck: [mframe_t; 16] = unsafe {
    [
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_move as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: -(1 as libc::c_int) as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_move as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: -(1 as libc::c_int) as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_move as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: -(1 as libc::c_int) as libc::c_float,
                thinkfunc: Some(
                    medic_duck_down as unsafe extern "C" fn(*mut edict_t) -> (),
                ),
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_move as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: -(1 as libc::c_int) as libc::c_float,
                thinkfunc: Some(
                    medic_duck_hold as unsafe extern "C" fn(*mut edict_t) -> (),
                ),
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_move as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: -(1 as libc::c_int) as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_move as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: -(1 as libc::c_int) as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_move as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: -(1 as libc::c_int) as libc::c_float,
                thinkfunc: Some(
                    medic_duck_up as unsafe extern "C" fn(*mut edict_t) -> (),
                ),
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_move as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: -(1 as libc::c_int) as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_move as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: -(1 as libc::c_int) as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_move as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: -(1 as libc::c_int) as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_move as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: -(1 as libc::c_int) as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_move as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: -(1 as libc::c_int) as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_move as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: -(1 as libc::c_int) as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_move as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: -(1 as libc::c_int) as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_move as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: -(1 as libc::c_int) as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_move as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: -(1 as libc::c_int) as libc::c_float,
                thinkfunc: None,
            };
            init
        },
    ]
};
#[no_mangle]
pub static mut medic_move_duck: mmove_t = unsafe {
    {
        let mut init = mmove_t {
            firstframe: 131 as libc::c_int,
            lastframe: 146 as libc::c_int,
            frame: medic_frames_duck.as_ptr() as *mut _,
            endfunc: Some(medic_run as unsafe extern "C" fn(*mut edict_t) -> ()),
        };
        init
    }
};
#[no_mangle]
pub unsafe extern "C" fn medic_dodge(
    mut self_0: *mut edict_t,
    mut attacker: *mut edict_t,
    mut eta: libc::c_float,
) {
    if ((rand() & 0x7fff as libc::c_int) as libc::c_float
        / 0x7fff as libc::c_int as libc::c_float) as libc::c_double > 0.25f64
    {
        return;
    }
    if ((*self_0).enemy).is_null() {
        let ref mut fresh18 = (*self_0).enemy;
        *fresh18 = attacker;
    }
    let ref mut fresh19 = (*self_0).monsterinfo.currentmove;
    *fresh19 = &mut medic_move_duck;
}
#[no_mangle]
pub static mut medic_frames_attackHyperBlaster: [mframe_t; 16] = unsafe {
    [
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_charge as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_charge as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_charge as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_charge as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_charge as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: Some(
                    medic_fire_blaster as unsafe extern "C" fn(*mut edict_t) -> (),
                ),
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_charge as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: Some(
                    medic_fire_blaster as unsafe extern "C" fn(*mut edict_t) -> (),
                ),
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_charge as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: Some(
                    medic_fire_blaster as unsafe extern "C" fn(*mut edict_t) -> (),
                ),
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_charge as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: Some(
                    medic_fire_blaster as unsafe extern "C" fn(*mut edict_t) -> (),
                ),
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_charge as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: Some(
                    medic_fire_blaster as unsafe extern "C" fn(*mut edict_t) -> (),
                ),
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_charge as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: Some(
                    medic_fire_blaster as unsafe extern "C" fn(*mut edict_t) -> (),
                ),
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_charge as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: Some(
                    medic_fire_blaster as unsafe extern "C" fn(*mut edict_t) -> (),
                ),
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_charge as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: Some(
                    medic_fire_blaster as unsafe extern "C" fn(*mut edict_t) -> (),
                ),
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_charge as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: Some(
                    medic_fire_blaster as unsafe extern "C" fn(*mut edict_t) -> (),
                ),
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_charge as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: Some(
                    medic_fire_blaster as unsafe extern "C" fn(*mut edict_t) -> (),
                ),
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_charge as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: Some(
                    medic_fire_blaster as unsafe extern "C" fn(*mut edict_t) -> (),
                ),
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_charge as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: Some(
                    medic_fire_blaster as unsafe extern "C" fn(*mut edict_t) -> (),
                ),
            };
            init
        },
    ]
};
#[no_mangle]
pub static mut medic_move_attackHyperBlaster: mmove_t = unsafe {
    {
        let mut init = mmove_t {
            firstframe: 191 as libc::c_int,
            lastframe: 206 as libc::c_int,
            frame: medic_frames_attackHyperBlaster.as_ptr() as *mut _,
            endfunc: Some(medic_run as unsafe extern "C" fn(*mut edict_t) -> ()),
        };
        init
    }
};
#[no_mangle]
pub unsafe extern "C" fn medic_continue(mut self_0: *mut edict_t) {
    if visible(self_0, (*self_0).enemy) as u64 != 0 {
        if ((rand() & 0x7fff as libc::c_int) as libc::c_float
            / 0x7fff as libc::c_int as libc::c_float) as libc::c_double <= 0.95f64
        {
            let ref mut fresh20 = (*self_0).monsterinfo.currentmove;
            *fresh20 = &mut medic_move_attackHyperBlaster;
        }
    }
}
#[no_mangle]
pub static mut medic_frames_attackBlaster: [mframe_t; 14] = unsafe {
    [
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_charge as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_charge as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 5 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_charge as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 5 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_charge as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 3 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_charge as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 2 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_charge as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_charge as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_charge as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_charge as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: Some(
                    medic_fire_blaster as unsafe extern "C" fn(*mut edict_t) -> (),
                ),
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_charge as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_charge as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_charge as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: Some(
                    medic_fire_blaster as unsafe extern "C" fn(*mut edict_t) -> (),
                ),
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_charge as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_charge as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: Some(
                    medic_continue as unsafe extern "C" fn(*mut edict_t) -> (),
                ),
            };
            init
        },
    ]
};
#[no_mangle]
pub static mut medic_move_attackBlaster: mmove_t = unsafe {
    {
        let mut init = mmove_t {
            firstframe: 177 as libc::c_int,
            lastframe: 190 as libc::c_int,
            frame: medic_frames_attackBlaster.as_ptr() as *mut _,
            endfunc: Some(medic_run as unsafe extern "C" fn(*mut edict_t) -> ()),
        };
        init
    }
};
#[no_mangle]
pub unsafe extern "C" fn medic_hook_launch(mut self_0: *mut edict_t) {
    (gi.sound)
        .expect(
            "non-null function pointer",
        )(
        self_0,
        1 as libc::c_int,
        sound_hook_launch,
        1 as libc::c_int as libc::c_float,
        1 as libc::c_int as libc::c_float,
        0 as libc::c_int as libc::c_float,
    );
}
static mut medic_cable_offsets: [vec3_t; 10] = [
    [45.0f64 as vec_t, -9.2f64 as vec_t, 15.5f64 as vec_t],
    [48.4f64 as vec_t, -9.7f64 as vec_t, 15.2f64 as vec_t],
    [47.8f64 as vec_t, -9.8f64 as vec_t, 15.8f64 as vec_t],
    [47.3f64 as vec_t, -9.3f64 as vec_t, 14.3f64 as vec_t],
    [45.4f64 as vec_t, -10.1f64 as vec_t, 13.1f64 as vec_t],
    [41.9f64 as vec_t, -12.7f64 as vec_t, 12.0f64 as vec_t],
    [37.8f64 as vec_t, -15.8f64 as vec_t, 11.2f64 as vec_t],
    [34.3f64 as vec_t, -18.4f64 as vec_t, 10.7f64 as vec_t],
    [32.7f64 as vec_t, -19.7f64 as vec_t, 10.4f64 as vec_t],
    [32.7f64 as vec_t, -19.7f64 as vec_t, 10.4f64 as vec_t],
];
#[no_mangle]
pub unsafe extern "C" fn medic_cable_attack(mut self_0: *mut edict_t) {
    let mut offset: vec3_t = [0.; 3];
    let mut start: vec3_t = [0.; 3];
    let mut end: vec3_t = [0.; 3];
    let mut f: vec3_t = [0.; 3];
    let mut r: vec3_t = [0.; 3];
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
    let mut angles: vec3_t = [0.; 3];
    let mut distance: libc::c_float = 0.;
    if (*(*self_0).enemy).inuse as u64 == 0 {
        return;
    }
    AngleVectors(
        ((*self_0).s.angles).as_mut_ptr(),
        f.as_mut_ptr(),
        r.as_mut_ptr(),
        0 as *mut vec_t,
    );
    offset[0 as libc::c_int
        as usize] = medic_cable_offsets[((*self_0).s.frame - 218 as libc::c_int)
        as usize][0 as libc::c_int as usize];
    offset[1 as libc::c_int
        as usize] = medic_cable_offsets[((*self_0).s.frame - 218 as libc::c_int)
        as usize][1 as libc::c_int as usize];
    offset[2 as libc::c_int
        as usize] = medic_cable_offsets[((*self_0).s.frame - 218 as libc::c_int)
        as usize][2 as libc::c_int as usize];
    G_ProjectSource(
        ((*self_0).s.origin).as_mut_ptr(),
        offset.as_mut_ptr(),
        f.as_mut_ptr(),
        r.as_mut_ptr(),
        start.as_mut_ptr(),
    );
    dir[0 as libc::c_int
        as usize] = start[0 as libc::c_int as usize]
        - (*(*self_0).enemy).s.origin[0 as libc::c_int as usize];
    dir[1 as libc::c_int
        as usize] = start[1 as libc::c_int as usize]
        - (*(*self_0).enemy).s.origin[1 as libc::c_int as usize];
    dir[2 as libc::c_int
        as usize] = start[2 as libc::c_int as usize]
        - (*(*self_0).enemy).s.origin[2 as libc::c_int as usize];
    distance = VectorLength(dir.as_mut_ptr());
    if distance > 256 as libc::c_int as libc::c_float {
        return;
    }
    vectoangles(dir.as_mut_ptr(), angles.as_mut_ptr());
    if angles[0 as libc::c_int as usize] < -(180 as libc::c_int) as libc::c_float {
        angles[0 as libc::c_int as usize] += 360 as libc::c_int as libc::c_float;
    }
    if fabs(angles[0 as libc::c_int as usize] as libc::c_double)
        > 45 as libc::c_int as libc::c_double
    {
        return;
    }
    tr = (gi.trace)
        .expect(
            "non-null function pointer",
        )(
        start.as_mut_ptr(),
        0 as *mut vec_t,
        0 as *mut vec_t,
        ((*(*self_0).enemy).s.origin).as_mut_ptr(),
        self_0,
        1 as libc::c_int | 0x2000000 as libc::c_int | 2 as libc::c_int
            | 0x4000000 as libc::c_int,
    );
    if tr.fraction as libc::c_double != 1.0f64 && tr.ent != (*self_0).enemy {
        return;
    }
    if (*self_0).s.frame == 219 as libc::c_int {
        (gi.sound)
            .expect(
                "non-null function pointer",
            )(
            (*self_0).enemy,
            0 as libc::c_int,
            sound_hook_hit,
            1 as libc::c_int as libc::c_float,
            1 as libc::c_int as libc::c_float,
            0 as libc::c_int as libc::c_float,
        );
        (*(*self_0).enemy).monsterinfo.aiflags |= 0x4000 as libc::c_int;
    } else if (*self_0).s.frame == 226 as libc::c_int {
        (*(*self_0).enemy).spawnflags = 0 as libc::c_int;
        (*(*self_0).enemy).monsterinfo.aiflags = 0 as libc::c_int;
        let ref mut fresh21 = (*(*self_0).enemy).target;
        *fresh21 = 0 as *mut libc::c_char;
        let ref mut fresh22 = (*(*self_0).enemy).targetname;
        *fresh22 = 0 as *mut libc::c_char;
        let ref mut fresh23 = (*(*self_0).enemy).combattarget;
        *fresh23 = 0 as *mut libc::c_char;
        let ref mut fresh24 = (*(*self_0).enemy).deathtarget;
        *fresh24 = 0 as *mut libc::c_char;
        let ref mut fresh25 = (*(*self_0).enemy).owner;
        *fresh25 = self_0;
        ED_CallSpawn((*self_0).enemy);
        let ref mut fresh26 = (*(*self_0).enemy).owner;
        *fresh26 = 0 as *mut edict_t;
        if ((*(*self_0).enemy).think).is_some() {
            (*(*self_0).enemy).nextthink = level.time;
            ((*(*self_0).enemy).think)
                .expect("non-null function pointer")((*self_0).enemy);
        }
        (*(*self_0).enemy).monsterinfo.aiflags |= 0x4000 as libc::c_int;
        if !((*self_0).oldenemy).is_null() && !((*(*self_0).oldenemy).client).is_null() {
            let ref mut fresh27 = (*(*self_0).enemy).enemy;
            *fresh27 = (*self_0).oldenemy;
            FoundTarget((*self_0).enemy);
        }
    } else if (*self_0).s.frame == 220 as libc::c_int {
        (gi.sound)
            .expect(
                "non-null function pointer",
            )(
            self_0,
            1 as libc::c_int,
            sound_hook_heal,
            1 as libc::c_int as libc::c_float,
            1 as libc::c_int as libc::c_float,
            0 as libc::c_int as libc::c_float,
        );
    }
    VectorMA(
        start.as_mut_ptr(),
        8 as libc::c_int as libc::c_float,
        f.as_mut_ptr(),
        start.as_mut_ptr(),
    );
    end[0 as libc::c_int
        as usize] = (*(*self_0).enemy).s.origin[0 as libc::c_int as usize];
    end[1 as libc::c_int
        as usize] = (*(*self_0).enemy).s.origin[1 as libc::c_int as usize];
    end[2 as libc::c_int
        as usize] = (*(*self_0).enemy).s.origin[2 as libc::c_int as usize];
    end[2 as libc::c_int
        as usize] = (*(*self_0).enemy).absmin[2 as libc::c_int as usize]
        + (*(*self_0).enemy).size[2 as libc::c_int as usize]
            / 2 as libc::c_int as libc::c_float;
    (gi.WriteByte).expect("non-null function pointer")(3 as libc::c_int);
    (gi.WriteByte)
        .expect("non-null function pointer")(TE_MEDIC_CABLE_ATTACK as libc::c_int);
    (gi.WriteShort)
        .expect(
            "non-null function pointer",
        )(self_0.offset_from(g_edicts) as libc::c_long as libc::c_int);
    (gi.WritePosition).expect("non-null function pointer")(start.as_mut_ptr());
    (gi.WritePosition).expect("non-null function pointer")(end.as_mut_ptr());
    (gi.multicast)
        .expect(
            "non-null function pointer",
        )(((*self_0).s.origin).as_mut_ptr(), MULTICAST_PVS);
}
#[no_mangle]
pub unsafe extern "C" fn medic_hook_retract(mut self_0: *mut edict_t) {
    (gi.sound)
        .expect(
            "non-null function pointer",
        )(
        self_0,
        1 as libc::c_int,
        sound_hook_retract,
        1 as libc::c_int as libc::c_float,
        1 as libc::c_int as libc::c_float,
        0 as libc::c_int as libc::c_float,
    );
    (*(*self_0).enemy).monsterinfo.aiflags &= !(0x4000 as libc::c_int);
}
#[no_mangle]
pub static mut medic_frames_attackCable: [mframe_t; 28] = unsafe {
    [
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_move as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 2 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_move as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 3 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_move as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 5 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_move as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 4.4f64 as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_charge as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 4.7f64 as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_charge as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 5 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_charge as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 6 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_charge as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 4 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_charge as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_move as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: Some(
                    medic_hook_launch as unsafe extern "C" fn(*mut edict_t) -> (),
                ),
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_move as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: Some(
                    medic_cable_attack as unsafe extern "C" fn(*mut edict_t) -> (),
                ),
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_move as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: Some(
                    medic_cable_attack as unsafe extern "C" fn(*mut edict_t) -> (),
                ),
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_move as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: Some(
                    medic_cable_attack as unsafe extern "C" fn(*mut edict_t) -> (),
                ),
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_move as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: Some(
                    medic_cable_attack as unsafe extern "C" fn(*mut edict_t) -> (),
                ),
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_move as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: Some(
                    medic_cable_attack as unsafe extern "C" fn(*mut edict_t) -> (),
                ),
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_move as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: Some(
                    medic_cable_attack as unsafe extern "C" fn(*mut edict_t) -> (),
                ),
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_move as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: Some(
                    medic_cable_attack as unsafe extern "C" fn(*mut edict_t) -> (),
                ),
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_move as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: Some(
                    medic_cable_attack as unsafe extern "C" fn(*mut edict_t) -> (),
                ),
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_move as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: Some(
                    medic_cable_attack as unsafe extern "C" fn(*mut edict_t) -> (),
                ),
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_move as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: -(15 as libc::c_int) as libc::c_float,
                thinkfunc: Some(
                    medic_hook_retract as unsafe extern "C" fn(*mut edict_t) -> (),
                ),
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_move as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: -1.5f64 as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_move as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: -1.2f64 as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_move as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: -(3 as libc::c_int) as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_move as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: -(2 as libc::c_int) as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_move as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0.3f64 as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_move as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 0.7f64 as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_move as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 1.2f64 as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_move as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 1.3f64 as libc::c_float,
                thinkfunc: None,
            };
            init
        },
    ]
};
#[no_mangle]
pub static mut medic_move_attackCable: mmove_t = unsafe {
    {
        let mut init = mmove_t {
            firstframe: 209 as libc::c_int,
            lastframe: 236 as libc::c_int,
            frame: medic_frames_attackCable.as_ptr() as *mut _,
            endfunc: Some(medic_run as unsafe extern "C" fn(*mut edict_t) -> ()),
        };
        init
    }
};
#[no_mangle]
pub unsafe extern "C" fn medic_attack(mut self_0: *mut edict_t) {
    if (*self_0).monsterinfo.aiflags & 0x2000 as libc::c_int != 0 {
        let ref mut fresh28 = (*self_0).monsterinfo.currentmove;
        *fresh28 = &mut medic_move_attackCable;
    } else {
        let ref mut fresh29 = (*self_0).monsterinfo.currentmove;
        *fresh29 = &mut medic_move_attackBlaster;
    };
}
#[no_mangle]
pub unsafe extern "C" fn medic_checkattack(mut self_0: *mut edict_t) -> qboolean {
    if (*self_0).monsterinfo.aiflags & 0x2000 as libc::c_int != 0 {
        medic_attack(self_0);
        return true_0;
    }
    return M_CheckAttack(self_0);
}
#[no_mangle]
pub unsafe extern "C" fn SP_monster_medic(mut self_0: *mut edict_t) {
    if (*deathmatch).value != 0. {
        G_FreeEdict(self_0);
        return;
    }
    sound_idle1 = (gi.soundindex)
        .expect(
            "non-null function pointer",
        )(b"medic/idle.wav\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    sound_pain1 = (gi.soundindex)
        .expect(
            "non-null function pointer",
        )(
        b"medic/medpain1.wav\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    sound_pain2 = (gi.soundindex)
        .expect(
            "non-null function pointer",
        )(
        b"medic/medpain2.wav\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    sound_die = (gi.soundindex)
        .expect(
            "non-null function pointer",
        )(
        b"medic/meddeth1.wav\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    sound_sight = (gi.soundindex)
        .expect(
            "non-null function pointer",
        )(
        b"medic/medsght1.wav\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    sound_search = (gi.soundindex)
        .expect(
            "non-null function pointer",
        )(
        b"medic/medsrch1.wav\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    sound_hook_launch = (gi.soundindex)
        .expect(
            "non-null function pointer",
        )(
        b"medic/medatck2.wav\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    sound_hook_hit = (gi.soundindex)
        .expect(
            "non-null function pointer",
        )(
        b"medic/medatck3.wav\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    sound_hook_heal = (gi.soundindex)
        .expect(
            "non-null function pointer",
        )(
        b"medic/medatck4.wav\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    sound_hook_retract = (gi.soundindex)
        .expect(
            "non-null function pointer",
        )(
        b"medic/medatck5.wav\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    (gi.soundindex)
        .expect(
            "non-null function pointer",
        )(
        b"medic/medatck1.wav\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    (*self_0).movetype = MOVETYPE_STEP as libc::c_int;
    (*self_0).solid = SOLID_BBOX;
    (*self_0)
        .s
        .modelindex = (gi.modelindex)
        .expect(
            "non-null function pointer",
        )(
        b"models/monsters/medic/tris.md2\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    (*self_0).mins[0 as libc::c_int as usize] = -(24 as libc::c_int) as vec_t;
    (*self_0).mins[1 as libc::c_int as usize] = -(24 as libc::c_int) as vec_t;
    (*self_0).mins[2 as libc::c_int as usize] = -(24 as libc::c_int) as vec_t;
    (*self_0).maxs[0 as libc::c_int as usize] = 24 as libc::c_int as vec_t;
    (*self_0).maxs[1 as libc::c_int as usize] = 24 as libc::c_int as vec_t;
    (*self_0).maxs[2 as libc::c_int as usize] = 32 as libc::c_int as vec_t;
    (*self_0).health = 300 as libc::c_int;
    (*self_0).gib_health = -(130 as libc::c_int);
    (*self_0).mass = 400 as libc::c_int;
    let ref mut fresh30 = (*self_0).pain;
    *fresh30 = Some(
        medic_pain
            as unsafe extern "C" fn(
                *mut edict_t,
                *mut edict_t,
                libc::c_float,
                libc::c_int,
            ) -> (),
    );
    let ref mut fresh31 = (*self_0).die;
    *fresh31 = Some(
        medic_die
            as unsafe extern "C" fn(
                *mut edict_t,
                *mut edict_t,
                *mut edict_t,
                libc::c_int,
                *mut vec_t,
            ) -> (),
    );
    let ref mut fresh32 = (*self_0).monsterinfo.stand;
    *fresh32 = Some(medic_stand as unsafe extern "C" fn(*mut edict_t) -> ());
    let ref mut fresh33 = (*self_0).monsterinfo.walk;
    *fresh33 = Some(medic_walk as unsafe extern "C" fn(*mut edict_t) -> ());
    let ref mut fresh34 = (*self_0).monsterinfo.run;
    *fresh34 = Some(medic_run as unsafe extern "C" fn(*mut edict_t) -> ());
    let ref mut fresh35 = (*self_0).monsterinfo.dodge;
    *fresh35 = Some(
        medic_dodge
            as unsafe extern "C" fn(*mut edict_t, *mut edict_t, libc::c_float) -> (),
    );
    let ref mut fresh36 = (*self_0).monsterinfo.attack;
    *fresh36 = Some(medic_attack as unsafe extern "C" fn(*mut edict_t) -> ());
    let ref mut fresh37 = (*self_0).monsterinfo.melee;
    *fresh37 = None;
    let ref mut fresh38 = (*self_0).monsterinfo.sight;
    *fresh38 = Some(
        medic_sight as unsafe extern "C" fn(*mut edict_t, *mut edict_t) -> (),
    );
    let ref mut fresh39 = (*self_0).monsterinfo.idle;
    *fresh39 = Some(medic_idle as unsafe extern "C" fn(*mut edict_t) -> ());
    let ref mut fresh40 = (*self_0).monsterinfo.search;
    *fresh40 = Some(medic_search as unsafe extern "C" fn(*mut edict_t) -> ());
    let ref mut fresh41 = (*self_0).monsterinfo.checkattack;
    *fresh41 = Some(medic_checkattack as unsafe extern "C" fn(*mut edict_t) -> qboolean);
    (gi.linkentity).expect("non-null function pointer")(self_0);
    let ref mut fresh42 = (*self_0).monsterinfo.currentmove;
    *fresh42 = &mut medic_move_stand;
    (*self_0).monsterinfo.scale = 1.000000f64 as libc::c_float;
    walkmonster_start(self_0);
}
