#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn floor(_: libc::c_double) -> libc::c_double;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn atof(__nptr: *const libc::c_char) -> libc::c_double;
    fn atoi(__nptr: *const libc::c_char) -> libc::c_int;
    fn COM_Parse(data_p: *mut *mut libc::c_char) -> *mut libc::c_char;
    fn Q_stricmp(s1: *mut libc::c_char, s2: *mut libc::c_char) -> libc::c_int;
    fn va(format: *mut libc::c_char, _: ...) -> *mut libc::c_char;
    static mut game: game_locals_t;
    static mut level: level_locals_t;
    static mut gi: game_import_t;
    static mut globals: game_export_t;
    static mut st: spawn_temp_t;
    static mut sm_meat_index: libc::c_int;
    static mut snd_fry: libc::c_int;
    static mut g_edicts: *mut edict_t;
    static mut deathmatch: *mut cvar_t;
    static mut skill: *mut cvar_t;
    static mut maxclients: *mut cvar_t;
    static mut fields: [field_t; 0];
    static mut itemlist: [gitem_t; 0];
    fn PrecacheItem(it: *mut gitem_t);
    fn SetItemNames();
    fn FindItem(pickup_name: *mut libc::c_char) -> *mut gitem_t;
    fn SpawnItem(ent: *mut edict_t, item: *mut gitem_t);
    fn G_Spawn() -> *mut edict_t;
    fn G_FreeEdict(e: *mut edict_t);
    fn PlayerTrail_Init();
    fn InitBodyQue();
    fn SaveClientData();
    fn SP_item_health(self_0: *mut edict_t);
    fn SP_item_health_small(self_0: *mut edict_t);
    fn SP_item_health_large(self_0: *mut edict_t);
    fn SP_item_health_mega(self_0: *mut edict_t);
    fn SP_info_player_start(ent: *mut edict_t);
    fn SP_info_player_deathmatch(ent: *mut edict_t);
    fn SP_info_player_coop(ent: *mut edict_t);
    fn SP_info_player_intermission(ent: *mut edict_t);
    fn SP_func_plat(ent: *mut edict_t);
    fn SP_func_rotating(ent: *mut edict_t);
    fn SP_func_button(ent: *mut edict_t);
    fn SP_func_door(ent: *mut edict_t);
    fn SP_func_door_secret(ent: *mut edict_t);
    fn SP_func_door_rotating(ent: *mut edict_t);
    fn SP_func_water(ent: *mut edict_t);
    fn SP_func_train(ent: *mut edict_t);
    fn SP_func_conveyor(self_0: *mut edict_t);
    fn SP_func_wall(self_0: *mut edict_t);
    fn SP_func_object(self_0: *mut edict_t);
    fn SP_func_explosive(self_0: *mut edict_t);
    fn SP_func_timer(self_0: *mut edict_t);
    fn SP_func_areaportal(ent: *mut edict_t);
    fn SP_func_clock(ent: *mut edict_t);
    fn SP_func_killbox(ent: *mut edict_t);
    fn SP_trigger_always(ent: *mut edict_t);
    fn SP_trigger_once(ent: *mut edict_t);
    fn SP_trigger_multiple(ent: *mut edict_t);
    fn SP_trigger_relay(ent: *mut edict_t);
    fn SP_trigger_push(ent: *mut edict_t);
    fn SP_trigger_hurt(ent: *mut edict_t);
    fn SP_trigger_key(ent: *mut edict_t);
    fn SP_trigger_counter(ent: *mut edict_t);
    fn SP_trigger_elevator(ent: *mut edict_t);
    fn SP_trigger_gravity(ent: *mut edict_t);
    fn SP_trigger_monsterjump(ent: *mut edict_t);
    fn SP_target_temp_entity(ent: *mut edict_t);
    fn SP_target_speaker(ent: *mut edict_t);
    fn SP_target_explosion(ent: *mut edict_t);
    fn SP_target_changelevel(ent: *mut edict_t);
    fn SP_target_secret(ent: *mut edict_t);
    fn SP_target_goal(ent: *mut edict_t);
    fn SP_target_splash(ent: *mut edict_t);
    fn SP_target_spawner(ent: *mut edict_t);
    fn SP_target_blaster(ent: *mut edict_t);
    fn SP_target_crosslevel_trigger(ent: *mut edict_t);
    fn SP_target_crosslevel_target(ent: *mut edict_t);
    fn SP_target_laser(self_0: *mut edict_t);
    fn SP_target_help(ent: *mut edict_t);
    fn SP_target_actor(ent: *mut edict_t);
    fn SP_target_lightramp(self_0: *mut edict_t);
    fn SP_target_earthquake(ent: *mut edict_t);
    fn SP_target_character(ent: *mut edict_t);
    fn SP_target_string(ent: *mut edict_t);
    fn SP_viewthing(ent: *mut edict_t);
    fn SP_light(self_0: *mut edict_t);
    fn SP_light_mine1(ent: *mut edict_t);
    fn SP_light_mine2(ent: *mut edict_t);
    fn SP_info_null(self_0: *mut edict_t);
    fn SP_info_notnull(self_0: *mut edict_t);
    fn SP_path_corner(self_0: *mut edict_t);
    fn SP_point_combat(self_0: *mut edict_t);
    fn SP_misc_explobox(self_0: *mut edict_t);
    fn SP_misc_banner(self_0: *mut edict_t);
    fn SP_misc_satellite_dish(self_0: *mut edict_t);
    fn SP_misc_actor(self_0: *mut edict_t);
    fn SP_misc_gib_arm(self_0: *mut edict_t);
    fn SP_misc_gib_leg(self_0: *mut edict_t);
    fn SP_misc_gib_head(self_0: *mut edict_t);
    fn SP_misc_insane(self_0: *mut edict_t);
    fn SP_misc_deadsoldier(self_0: *mut edict_t);
    fn SP_misc_viper(self_0: *mut edict_t);
    fn SP_misc_viper_bomb(self_0: *mut edict_t);
    fn SP_misc_bigviper(self_0: *mut edict_t);
    fn SP_misc_strogg_ship(self_0: *mut edict_t);
    fn SP_misc_teleporter(self_0: *mut edict_t);
    fn SP_misc_teleporter_dest(self_0: *mut edict_t);
    fn SP_misc_blackhole(self_0: *mut edict_t);
    fn SP_misc_eastertank(self_0: *mut edict_t);
    fn SP_misc_easterchick(self_0: *mut edict_t);
    fn SP_misc_easterchick2(self_0: *mut edict_t);
    fn SP_monster_berserk(self_0: *mut edict_t);
    fn SP_monster_gladiator(self_0: *mut edict_t);
    fn SP_monster_gunner(self_0: *mut edict_t);
    fn SP_monster_infantry(self_0: *mut edict_t);
    fn SP_monster_soldier_light(self_0: *mut edict_t);
    fn SP_monster_soldier(self_0: *mut edict_t);
    fn SP_monster_soldier_ss(self_0: *mut edict_t);
    fn SP_monster_tank(self_0: *mut edict_t);
    fn SP_monster_medic(self_0: *mut edict_t);
    fn SP_monster_flipper(self_0: *mut edict_t);
    fn SP_monster_chick(self_0: *mut edict_t);
    fn SP_monster_parasite(self_0: *mut edict_t);
    fn SP_monster_flyer(self_0: *mut edict_t);
    fn SP_monster_brain(self_0: *mut edict_t);
    fn SP_monster_floater(self_0: *mut edict_t);
    fn SP_monster_hover(self_0: *mut edict_t);
    fn SP_monster_mutant(self_0: *mut edict_t);
    fn SP_monster_supertank(self_0: *mut edict_t);
    fn SP_monster_boss2(self_0: *mut edict_t);
    fn SP_monster_jorg(self_0: *mut edict_t);
    fn SP_monster_boss3_stand(self_0: *mut edict_t);
    fn SP_monster_commander_body(self_0: *mut edict_t);
    fn SP_turret_breach(self_0: *mut edict_t);
    fn SP_turret_base(self_0: *mut edict_t);
    fn SP_turret_driver(self_0: *mut edict_t);
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
pub type fieldtype_t = libc::c_uint;
pub const F_IGNORE: fieldtype_t = 11;
pub const F_MMOVE: fieldtype_t = 10;
pub const F_FUNCTION: fieldtype_t = 9;
pub const F_CLIENT: fieldtype_t = 8;
pub const F_ITEM: fieldtype_t = 7;
pub const F_EDICT: fieldtype_t = 6;
pub const F_ANGLEHACK: fieldtype_t = 5;
pub const F_VECTOR: fieldtype_t = 4;
pub const F_GSTRING: fieldtype_t = 3;
pub const F_LSTRING: fieldtype_t = 2;
pub const F_FLOAT: fieldtype_t = 1;
pub const F_INT: fieldtype_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct field_t {
    pub name: *mut libc::c_char,
    pub ofs: libc::c_int,
    pub type_0: fieldtype_t,
    pub flags: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spawn_t {
    pub name: *mut libc::c_char,
    pub spawn: Option::<unsafe extern "C" fn(*mut edict_t) -> ()>,
}
#[no_mangle]
pub static mut spawns: [spawn_t; 109] = unsafe {
    [
        {
            let mut init = spawn_t {
                name: b"item_health\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                spawn: Some(SP_item_health as unsafe extern "C" fn(*mut edict_t) -> ()),
            };
            init
        },
        {
            let mut init = spawn_t {
                name: b"item_health_small\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                spawn: Some(
                    SP_item_health_small as unsafe extern "C" fn(*mut edict_t) -> (),
                ),
            };
            init
        },
        {
            let mut init = spawn_t {
                name: b"item_health_large\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                spawn: Some(
                    SP_item_health_large as unsafe extern "C" fn(*mut edict_t) -> (),
                ),
            };
            init
        },
        {
            let mut init = spawn_t {
                name: b"item_health_mega\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                spawn: Some(
                    SP_item_health_mega as unsafe extern "C" fn(*mut edict_t) -> (),
                ),
            };
            init
        },
        {
            let mut init = spawn_t {
                name: b"info_player_start\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                spawn: Some(
                    SP_info_player_start as unsafe extern "C" fn(*mut edict_t) -> (),
                ),
            };
            init
        },
        {
            let mut init = spawn_t {
                name: b"info_player_deathmatch\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                spawn: Some(
                    SP_info_player_deathmatch as unsafe extern "C" fn(*mut edict_t) -> (),
                ),
            };
            init
        },
        {
            let mut init = spawn_t {
                name: b"info_player_coop\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                spawn: Some(
                    SP_info_player_coop as unsafe extern "C" fn(*mut edict_t) -> (),
                ),
            };
            init
        },
        {
            let mut init = spawn_t {
                name: b"info_player_intermission\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                spawn: Some(
                    SP_info_player_intermission
                        as unsafe extern "C" fn(*mut edict_t) -> (),
                ),
            };
            init
        },
        {
            let mut init = spawn_t {
                name: b"func_plat\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                spawn: Some(SP_func_plat as unsafe extern "C" fn(*mut edict_t) -> ()),
            };
            init
        },
        {
            let mut init = spawn_t {
                name: b"func_button\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                spawn: Some(SP_func_button as unsafe extern "C" fn(*mut edict_t) -> ()),
            };
            init
        },
        {
            let mut init = spawn_t {
                name: b"func_door\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                spawn: Some(SP_func_door as unsafe extern "C" fn(*mut edict_t) -> ()),
            };
            init
        },
        {
            let mut init = spawn_t {
                name: b"func_door_secret\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                spawn: Some(
                    SP_func_door_secret as unsafe extern "C" fn(*mut edict_t) -> (),
                ),
            };
            init
        },
        {
            let mut init = spawn_t {
                name: b"func_door_rotating\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                spawn: Some(
                    SP_func_door_rotating as unsafe extern "C" fn(*mut edict_t) -> (),
                ),
            };
            init
        },
        {
            let mut init = spawn_t {
                name: b"func_rotating\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                spawn: Some(SP_func_rotating as unsafe extern "C" fn(*mut edict_t) -> ()),
            };
            init
        },
        {
            let mut init = spawn_t {
                name: b"func_train\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                spawn: Some(SP_func_train as unsafe extern "C" fn(*mut edict_t) -> ()),
            };
            init
        },
        {
            let mut init = spawn_t {
                name: b"func_water\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                spawn: Some(SP_func_water as unsafe extern "C" fn(*mut edict_t) -> ()),
            };
            init
        },
        {
            let mut init = spawn_t {
                name: b"func_conveyor\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                spawn: Some(SP_func_conveyor as unsafe extern "C" fn(*mut edict_t) -> ()),
            };
            init
        },
        {
            let mut init = spawn_t {
                name: b"func_areaportal\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                spawn: Some(
                    SP_func_areaportal as unsafe extern "C" fn(*mut edict_t) -> (),
                ),
            };
            init
        },
        {
            let mut init = spawn_t {
                name: b"func_clock\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                spawn: Some(SP_func_clock as unsafe extern "C" fn(*mut edict_t) -> ()),
            };
            init
        },
        {
            let mut init = spawn_t {
                name: b"func_wall\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                spawn: Some(SP_func_wall as unsafe extern "C" fn(*mut edict_t) -> ()),
            };
            init
        },
        {
            let mut init = spawn_t {
                name: b"func_object\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                spawn: Some(SP_func_object as unsafe extern "C" fn(*mut edict_t) -> ()),
            };
            init
        },
        {
            let mut init = spawn_t {
                name: b"func_timer\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                spawn: Some(SP_func_timer as unsafe extern "C" fn(*mut edict_t) -> ()),
            };
            init
        },
        {
            let mut init = spawn_t {
                name: b"func_explosive\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                spawn: Some(
                    SP_func_explosive as unsafe extern "C" fn(*mut edict_t) -> (),
                ),
            };
            init
        },
        {
            let mut init = spawn_t {
                name: b"func_killbox\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                spawn: Some(SP_func_killbox as unsafe extern "C" fn(*mut edict_t) -> ()),
            };
            init
        },
        {
            let mut init = spawn_t {
                name: b"trigger_always\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                spawn: Some(
                    SP_trigger_always as unsafe extern "C" fn(*mut edict_t) -> (),
                ),
            };
            init
        },
        {
            let mut init = spawn_t {
                name: b"trigger_once\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                spawn: Some(SP_trigger_once as unsafe extern "C" fn(*mut edict_t) -> ()),
            };
            init
        },
        {
            let mut init = spawn_t {
                name: b"trigger_multiple\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                spawn: Some(
                    SP_trigger_multiple as unsafe extern "C" fn(*mut edict_t) -> (),
                ),
            };
            init
        },
        {
            let mut init = spawn_t {
                name: b"trigger_relay\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                spawn: Some(SP_trigger_relay as unsafe extern "C" fn(*mut edict_t) -> ()),
            };
            init
        },
        {
            let mut init = spawn_t {
                name: b"trigger_push\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                spawn: Some(SP_trigger_push as unsafe extern "C" fn(*mut edict_t) -> ()),
            };
            init
        },
        {
            let mut init = spawn_t {
                name: b"trigger_hurt\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                spawn: Some(SP_trigger_hurt as unsafe extern "C" fn(*mut edict_t) -> ()),
            };
            init
        },
        {
            let mut init = spawn_t {
                name: b"trigger_key\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                spawn: Some(SP_trigger_key as unsafe extern "C" fn(*mut edict_t) -> ()),
            };
            init
        },
        {
            let mut init = spawn_t {
                name: b"trigger_counter\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                spawn: Some(
                    SP_trigger_counter as unsafe extern "C" fn(*mut edict_t) -> (),
                ),
            };
            init
        },
        {
            let mut init = spawn_t {
                name: b"trigger_elevator\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                spawn: Some(
                    SP_trigger_elevator as unsafe extern "C" fn(*mut edict_t) -> (),
                ),
            };
            init
        },
        {
            let mut init = spawn_t {
                name: b"trigger_gravity\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                spawn: Some(
                    SP_trigger_gravity as unsafe extern "C" fn(*mut edict_t) -> (),
                ),
            };
            init
        },
        {
            let mut init = spawn_t {
                name: b"trigger_monsterjump\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                spawn: Some(
                    SP_trigger_monsterjump as unsafe extern "C" fn(*mut edict_t) -> (),
                ),
            };
            init
        },
        {
            let mut init = spawn_t {
                name: b"target_temp_entity\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                spawn: Some(
                    SP_target_temp_entity as unsafe extern "C" fn(*mut edict_t) -> (),
                ),
            };
            init
        },
        {
            let mut init = spawn_t {
                name: b"target_speaker\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                spawn: Some(
                    SP_target_speaker as unsafe extern "C" fn(*mut edict_t) -> (),
                ),
            };
            init
        },
        {
            let mut init = spawn_t {
                name: b"target_explosion\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                spawn: Some(
                    SP_target_explosion as unsafe extern "C" fn(*mut edict_t) -> (),
                ),
            };
            init
        },
        {
            let mut init = spawn_t {
                name: b"target_changelevel\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                spawn: Some(
                    SP_target_changelevel as unsafe extern "C" fn(*mut edict_t) -> (),
                ),
            };
            init
        },
        {
            let mut init = spawn_t {
                name: b"target_secret\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                spawn: Some(SP_target_secret as unsafe extern "C" fn(*mut edict_t) -> ()),
            };
            init
        },
        {
            let mut init = spawn_t {
                name: b"target_goal\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                spawn: Some(SP_target_goal as unsafe extern "C" fn(*mut edict_t) -> ()),
            };
            init
        },
        {
            let mut init = spawn_t {
                name: b"target_splash\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                spawn: Some(SP_target_splash as unsafe extern "C" fn(*mut edict_t) -> ()),
            };
            init
        },
        {
            let mut init = spawn_t {
                name: b"target_spawner\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                spawn: Some(
                    SP_target_spawner as unsafe extern "C" fn(*mut edict_t) -> (),
                ),
            };
            init
        },
        {
            let mut init = spawn_t {
                name: b"target_blaster\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                spawn: Some(
                    SP_target_blaster as unsafe extern "C" fn(*mut edict_t) -> (),
                ),
            };
            init
        },
        {
            let mut init = spawn_t {
                name: b"target_crosslevel_trigger\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                spawn: Some(
                    SP_target_crosslevel_trigger
                        as unsafe extern "C" fn(*mut edict_t) -> (),
                ),
            };
            init
        },
        {
            let mut init = spawn_t {
                name: b"target_crosslevel_target\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                spawn: Some(
                    SP_target_crosslevel_target
                        as unsafe extern "C" fn(*mut edict_t) -> (),
                ),
            };
            init
        },
        {
            let mut init = spawn_t {
                name: b"target_laser\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                spawn: Some(SP_target_laser as unsafe extern "C" fn(*mut edict_t) -> ()),
            };
            init
        },
        {
            let mut init = spawn_t {
                name: b"target_help\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                spawn: Some(SP_target_help as unsafe extern "C" fn(*mut edict_t) -> ()),
            };
            init
        },
        {
            let mut init = spawn_t {
                name: b"target_actor\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                spawn: Some(SP_target_actor as unsafe extern "C" fn(*mut edict_t) -> ()),
            };
            init
        },
        {
            let mut init = spawn_t {
                name: b"target_lightramp\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                spawn: Some(
                    SP_target_lightramp as unsafe extern "C" fn(*mut edict_t) -> (),
                ),
            };
            init
        },
        {
            let mut init = spawn_t {
                name: b"target_earthquake\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                spawn: Some(
                    SP_target_earthquake as unsafe extern "C" fn(*mut edict_t) -> (),
                ),
            };
            init
        },
        {
            let mut init = spawn_t {
                name: b"target_character\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                spawn: Some(
                    SP_target_character as unsafe extern "C" fn(*mut edict_t) -> (),
                ),
            };
            init
        },
        {
            let mut init = spawn_t {
                name: b"target_string\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                spawn: Some(SP_target_string as unsafe extern "C" fn(*mut edict_t) -> ()),
            };
            init
        },
        {
            let mut init = spawn_t {
                name: b"worldspawn\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                spawn: Some(SP_worldspawn as unsafe extern "C" fn(*mut edict_t) -> ()),
            };
            init
        },
        {
            let mut init = spawn_t {
                name: b"viewthing\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                spawn: Some(SP_viewthing as unsafe extern "C" fn(*mut edict_t) -> ()),
            };
            init
        },
        {
            let mut init = spawn_t {
                name: b"light\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                spawn: Some(SP_light as unsafe extern "C" fn(*mut edict_t) -> ()),
            };
            init
        },
        {
            let mut init = spawn_t {
                name: b"light_mine1\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                spawn: Some(SP_light_mine1 as unsafe extern "C" fn(*mut edict_t) -> ()),
            };
            init
        },
        {
            let mut init = spawn_t {
                name: b"light_mine2\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                spawn: Some(SP_light_mine2 as unsafe extern "C" fn(*mut edict_t) -> ()),
            };
            init
        },
        {
            let mut init = spawn_t {
                name: b"info_null\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                spawn: Some(SP_info_null as unsafe extern "C" fn(*mut edict_t) -> ()),
            };
            init
        },
        {
            let mut init = spawn_t {
                name: b"func_group\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                spawn: Some(SP_info_null as unsafe extern "C" fn(*mut edict_t) -> ()),
            };
            init
        },
        {
            let mut init = spawn_t {
                name: b"info_notnull\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                spawn: Some(SP_info_notnull as unsafe extern "C" fn(*mut edict_t) -> ()),
            };
            init
        },
        {
            let mut init = spawn_t {
                name: b"path_corner\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                spawn: Some(SP_path_corner as unsafe extern "C" fn(*mut edict_t) -> ()),
            };
            init
        },
        {
            let mut init = spawn_t {
                name: b"point_combat\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                spawn: Some(SP_point_combat as unsafe extern "C" fn(*mut edict_t) -> ()),
            };
            init
        },
        {
            let mut init = spawn_t {
                name: b"misc_explobox\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                spawn: Some(SP_misc_explobox as unsafe extern "C" fn(*mut edict_t) -> ()),
            };
            init
        },
        {
            let mut init = spawn_t {
                name: b"misc_banner\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                spawn: Some(SP_misc_banner as unsafe extern "C" fn(*mut edict_t) -> ()),
            };
            init
        },
        {
            let mut init = spawn_t {
                name: b"misc_satellite_dish\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                spawn: Some(
                    SP_misc_satellite_dish as unsafe extern "C" fn(*mut edict_t) -> (),
                ),
            };
            init
        },
        {
            let mut init = spawn_t {
                name: b"misc_actor\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                spawn: Some(SP_misc_actor as unsafe extern "C" fn(*mut edict_t) -> ()),
            };
            init
        },
        {
            let mut init = spawn_t {
                name: b"misc_gib_arm\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                spawn: Some(SP_misc_gib_arm as unsafe extern "C" fn(*mut edict_t) -> ()),
            };
            init
        },
        {
            let mut init = spawn_t {
                name: b"misc_gib_leg\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                spawn: Some(SP_misc_gib_leg as unsafe extern "C" fn(*mut edict_t) -> ()),
            };
            init
        },
        {
            let mut init = spawn_t {
                name: b"misc_gib_head\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                spawn: Some(SP_misc_gib_head as unsafe extern "C" fn(*mut edict_t) -> ()),
            };
            init
        },
        {
            let mut init = spawn_t {
                name: b"misc_insane\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                spawn: Some(SP_misc_insane as unsafe extern "C" fn(*mut edict_t) -> ()),
            };
            init
        },
        {
            let mut init = spawn_t {
                name: b"misc_deadsoldier\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                spawn: Some(
                    SP_misc_deadsoldier as unsafe extern "C" fn(*mut edict_t) -> (),
                ),
            };
            init
        },
        {
            let mut init = spawn_t {
                name: b"misc_viper\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                spawn: Some(SP_misc_viper as unsafe extern "C" fn(*mut edict_t) -> ()),
            };
            init
        },
        {
            let mut init = spawn_t {
                name: b"misc_viper_bomb\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                spawn: Some(
                    SP_misc_viper_bomb as unsafe extern "C" fn(*mut edict_t) -> (),
                ),
            };
            init
        },
        {
            let mut init = spawn_t {
                name: b"misc_bigviper\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                spawn: Some(SP_misc_bigviper as unsafe extern "C" fn(*mut edict_t) -> ()),
            };
            init
        },
        {
            let mut init = spawn_t {
                name: b"misc_strogg_ship\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                spawn: Some(
                    SP_misc_strogg_ship as unsafe extern "C" fn(*mut edict_t) -> (),
                ),
            };
            init
        },
        {
            let mut init = spawn_t {
                name: b"misc_teleporter\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                spawn: Some(
                    SP_misc_teleporter as unsafe extern "C" fn(*mut edict_t) -> (),
                ),
            };
            init
        },
        {
            let mut init = spawn_t {
                name: b"misc_teleporter_dest\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                spawn: Some(
                    SP_misc_teleporter_dest as unsafe extern "C" fn(*mut edict_t) -> (),
                ),
            };
            init
        },
        {
            let mut init = spawn_t {
                name: b"misc_blackhole\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                spawn: Some(
                    SP_misc_blackhole as unsafe extern "C" fn(*mut edict_t) -> (),
                ),
            };
            init
        },
        {
            let mut init = spawn_t {
                name: b"misc_eastertank\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                spawn: Some(
                    SP_misc_eastertank as unsafe extern "C" fn(*mut edict_t) -> (),
                ),
            };
            init
        },
        {
            let mut init = spawn_t {
                name: b"misc_easterchick\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                spawn: Some(
                    SP_misc_easterchick as unsafe extern "C" fn(*mut edict_t) -> (),
                ),
            };
            init
        },
        {
            let mut init = spawn_t {
                name: b"misc_easterchick2\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                spawn: Some(
                    SP_misc_easterchick2 as unsafe extern "C" fn(*mut edict_t) -> (),
                ),
            };
            init
        },
        {
            let mut init = spawn_t {
                name: b"monster_berserk\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                spawn: Some(
                    SP_monster_berserk as unsafe extern "C" fn(*mut edict_t) -> (),
                ),
            };
            init
        },
        {
            let mut init = spawn_t {
                name: b"monster_gladiator\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                spawn: Some(
                    SP_monster_gladiator as unsafe extern "C" fn(*mut edict_t) -> (),
                ),
            };
            init
        },
        {
            let mut init = spawn_t {
                name: b"monster_gunner\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                spawn: Some(
                    SP_monster_gunner as unsafe extern "C" fn(*mut edict_t) -> (),
                ),
            };
            init
        },
        {
            let mut init = spawn_t {
                name: b"monster_infantry\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                spawn: Some(
                    SP_monster_infantry as unsafe extern "C" fn(*mut edict_t) -> (),
                ),
            };
            init
        },
        {
            let mut init = spawn_t {
                name: b"monster_soldier_light\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                spawn: Some(
                    SP_monster_soldier_light as unsafe extern "C" fn(*mut edict_t) -> (),
                ),
            };
            init
        },
        {
            let mut init = spawn_t {
                name: b"monster_soldier\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                spawn: Some(
                    SP_monster_soldier as unsafe extern "C" fn(*mut edict_t) -> (),
                ),
            };
            init
        },
        {
            let mut init = spawn_t {
                name: b"monster_soldier_ss\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                spawn: Some(
                    SP_monster_soldier_ss as unsafe extern "C" fn(*mut edict_t) -> (),
                ),
            };
            init
        },
        {
            let mut init = spawn_t {
                name: b"monster_tank\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                spawn: Some(SP_monster_tank as unsafe extern "C" fn(*mut edict_t) -> ()),
            };
            init
        },
        {
            let mut init = spawn_t {
                name: b"monster_tank_commander\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                spawn: Some(SP_monster_tank as unsafe extern "C" fn(*mut edict_t) -> ()),
            };
            init
        },
        {
            let mut init = spawn_t {
                name: b"monster_medic\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                spawn: Some(SP_monster_medic as unsafe extern "C" fn(*mut edict_t) -> ()),
            };
            init
        },
        {
            let mut init = spawn_t {
                name: b"monster_flipper\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                spawn: Some(
                    SP_monster_flipper as unsafe extern "C" fn(*mut edict_t) -> (),
                ),
            };
            init
        },
        {
            let mut init = spawn_t {
                name: b"monster_chick\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                spawn: Some(SP_monster_chick as unsafe extern "C" fn(*mut edict_t) -> ()),
            };
            init
        },
        {
            let mut init = spawn_t {
                name: b"monster_parasite\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                spawn: Some(
                    SP_monster_parasite as unsafe extern "C" fn(*mut edict_t) -> (),
                ),
            };
            init
        },
        {
            let mut init = spawn_t {
                name: b"monster_flyer\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                spawn: Some(SP_monster_flyer as unsafe extern "C" fn(*mut edict_t) -> ()),
            };
            init
        },
        {
            let mut init = spawn_t {
                name: b"monster_brain\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                spawn: Some(SP_monster_brain as unsafe extern "C" fn(*mut edict_t) -> ()),
            };
            init
        },
        {
            let mut init = spawn_t {
                name: b"monster_floater\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                spawn: Some(
                    SP_monster_floater as unsafe extern "C" fn(*mut edict_t) -> (),
                ),
            };
            init
        },
        {
            let mut init = spawn_t {
                name: b"monster_hover\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                spawn: Some(SP_monster_hover as unsafe extern "C" fn(*mut edict_t) -> ()),
            };
            init
        },
        {
            let mut init = spawn_t {
                name: b"monster_mutant\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                spawn: Some(
                    SP_monster_mutant as unsafe extern "C" fn(*mut edict_t) -> (),
                ),
            };
            init
        },
        {
            let mut init = spawn_t {
                name: b"monster_supertank\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                spawn: Some(
                    SP_monster_supertank as unsafe extern "C" fn(*mut edict_t) -> (),
                ),
            };
            init
        },
        {
            let mut init = spawn_t {
                name: b"monster_boss2\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                spawn: Some(SP_monster_boss2 as unsafe extern "C" fn(*mut edict_t) -> ()),
            };
            init
        },
        {
            let mut init = spawn_t {
                name: b"monster_boss3_stand\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                spawn: Some(
                    SP_monster_boss3_stand as unsafe extern "C" fn(*mut edict_t) -> (),
                ),
            };
            init
        },
        {
            let mut init = spawn_t {
                name: b"monster_jorg\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                spawn: Some(SP_monster_jorg as unsafe extern "C" fn(*mut edict_t) -> ()),
            };
            init
        },
        {
            let mut init = spawn_t {
                name: b"monster_commander_body\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                spawn: Some(
                    SP_monster_commander_body as unsafe extern "C" fn(*mut edict_t) -> (),
                ),
            };
            init
        },
        {
            let mut init = spawn_t {
                name: b"turret_breach\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                spawn: Some(SP_turret_breach as unsafe extern "C" fn(*mut edict_t) -> ()),
            };
            init
        },
        {
            let mut init = spawn_t {
                name: b"turret_base\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                spawn: Some(SP_turret_base as unsafe extern "C" fn(*mut edict_t) -> ()),
            };
            init
        },
        {
            let mut init = spawn_t {
                name: b"turret_driver\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                spawn: Some(SP_turret_driver as unsafe extern "C" fn(*mut edict_t) -> ()),
            };
            init
        },
        {
            let mut init = spawn_t {
                name: 0 as *const libc::c_char as *mut libc::c_char,
                spawn: None,
            };
            init
        },
    ]
};
#[no_mangle]
pub unsafe extern "C" fn ED_CallSpawn(mut ent: *mut edict_t) {
    let mut s: *mut spawn_t = 0 as *mut spawn_t;
    let mut item: *mut gitem_t = 0 as *mut gitem_t;
    let mut i: libc::c_int = 0;
    if ((*ent).classname).is_null() {
        (gi.dprintf)
            .expect(
                "non-null function pointer",
            )(
            b"ED_CallSpawn: NULL classname\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return;
    }
    i = 0 as libc::c_int;
    item = itemlist.as_mut_ptr();
    while i < game.num_items {
        if !((*item).classname).is_null() {
            if strcmp((*item).classname, (*ent).classname) == 0 {
                SpawnItem(ent, item);
                return;
            }
        }
        i += 1;
        item = item.offset(1);
    }
    s = spawns.as_mut_ptr();
    while !((*s).name).is_null() {
        if strcmp((*s).name, (*ent).classname) == 0 {
            ((*s).spawn).expect("non-null function pointer")(ent);
            return;
        }
        s = s.offset(1);
    }
    (gi.dprintf)
        .expect(
            "non-null function pointer",
        )(
        b"%s doesn't have a spawn function\n\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        (*ent).classname,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ED_NewString(
    mut string: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut newb: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut new_p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    l = (strlen(string)).wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int;
    newb = (gi.TagMalloc).expect("non-null function pointer")(l, 766 as libc::c_int)
        as *mut libc::c_char;
    new_p = newb;
    i = 0 as libc::c_int;
    while i < l {
        if *string.offset(i as isize) as libc::c_int == '\\' as i32
            && i < l - 1 as libc::c_int
        {
            i += 1;
            if *string.offset(i as isize) as libc::c_int == 'n' as i32 {
                let fresh0 = new_p;
                new_p = new_p.offset(1);
                *fresh0 = '\n' as i32 as libc::c_char;
            } else {
                let fresh1 = new_p;
                new_p = new_p.offset(1);
                *fresh1 = '\\' as i32 as libc::c_char;
            }
        } else {
            let fresh2 = new_p;
            new_p = new_p.offset(1);
            *fresh2 = *string.offset(i as isize);
        }
        i += 1;
    }
    return newb;
}
#[no_mangle]
pub unsafe extern "C" fn ED_ParseField(
    mut key: *mut libc::c_char,
    mut value: *mut libc::c_char,
    mut ent: *mut edict_t,
) {
    let mut f: *mut field_t = 0 as *mut field_t;
    let mut b: *mut byte = 0 as *mut byte;
    let mut v: libc::c_float = 0.;
    let mut vec: vec3_t = [0.; 3];
    f = fields.as_mut_ptr();
    while !((*f).name).is_null() {
        if (*f).flags & 2 as libc::c_int == 0 && Q_stricmp((*f).name, key) == 0 {
            if (*f).flags & 1 as libc::c_int != 0 {
                b = &mut st as *mut spawn_temp_t as *mut byte;
            } else {
                b = ent as *mut byte;
            }
            match (*f).type_0 as libc::c_uint {
                2 => {
                    let ref mut fresh3 = *(b.offset((*f).ofs as isize)
                        as *mut *mut libc::c_char);
                    *fresh3 = ED_NewString(value);
                }
                4 => {
                    sscanf(
                        value,
                        b"%f %f %f\0" as *const u8 as *const libc::c_char,
                        &mut *vec.as_mut_ptr().offset(0 as libc::c_int as isize)
                            as *mut vec_t,
                        &mut *vec.as_mut_ptr().offset(1 as libc::c_int as isize)
                            as *mut vec_t,
                        &mut *vec.as_mut_ptr().offset(2 as libc::c_int as isize)
                            as *mut vec_t,
                    );
                    *(b.offset((*f).ofs as isize) as *mut libc::c_float)
                        .offset(
                            0 as libc::c_int as isize,
                        ) = vec[0 as libc::c_int as usize];
                    *(b.offset((*f).ofs as isize) as *mut libc::c_float)
                        .offset(
                            1 as libc::c_int as isize,
                        ) = vec[1 as libc::c_int as usize];
                    *(b.offset((*f).ofs as isize) as *mut libc::c_float)
                        .offset(
                            2 as libc::c_int as isize,
                        ) = vec[2 as libc::c_int as usize];
                }
                0 => {
                    *(b.offset((*f).ofs as isize) as *mut libc::c_int) = atoi(value);
                }
                1 => {
                    *(b.offset((*f).ofs as isize)
                        as *mut libc::c_float) = atof(value) as libc::c_float;
                }
                5 => {
                    v = atof(value) as libc::c_float;
                    *(b.offset((*f).ofs as isize) as *mut libc::c_float)
                        .offset(
                            0 as libc::c_int as isize,
                        ) = 0 as libc::c_int as libc::c_float;
                    *(b.offset((*f).ofs as isize) as *mut libc::c_float)
                        .offset(1 as libc::c_int as isize) = v;
                    *(b.offset((*f).ofs as isize) as *mut libc::c_float)
                        .offset(
                            2 as libc::c_int as isize,
                        ) = 0 as libc::c_int as libc::c_float;
                }
                11 | _ => {}
            }
            return;
        }
        f = f.offset(1);
    }
    (gi.dprintf)
        .expect(
            "non-null function pointer",
        )(
        b"%s is not a field\n\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        key,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ED_ParseEdict(
    mut data: *mut libc::c_char,
    mut ent: *mut edict_t,
) -> *mut libc::c_char {
    let mut init: qboolean = false_0;
    let mut keyname: [libc::c_char; 256] = [0; 256];
    let mut com_token: *mut libc::c_char = 0 as *mut libc::c_char;
    init = false_0;
    memset(
        &mut st as *mut spawn_temp_t as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<spawn_temp_t>() as libc::c_ulong,
    );
    loop {
        com_token = COM_Parse(&mut data);
        if *com_token.offset(0 as libc::c_int as isize) as libc::c_int == '}' as i32 {
            break;
        }
        if data.is_null() {
            (gi.error)
                .expect(
                    "non-null function pointer",
                )(
                b"ED_ParseEntity: EOF without closing brace\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
            );
        }
        strncpy(
            keyname.as_mut_ptr(),
            com_token,
            (::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        );
        com_token = COM_Parse(&mut data);
        if data.is_null() {
            (gi.error)
                .expect(
                    "non-null function pointer",
                )(
                b"ED_ParseEntity: EOF without closing brace\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
            );
        }
        if *com_token.offset(0 as libc::c_int as isize) as libc::c_int == '}' as i32 {
            (gi.error)
                .expect(
                    "non-null function pointer",
                )(
                b"ED_ParseEntity: closing brace without data\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
            );
        }
        init = true_0;
        if keyname[0 as libc::c_int as usize] as libc::c_int == '_' as i32 {
            continue;
        }
        ED_ParseField(keyname.as_mut_ptr(), com_token, ent);
    }
    if init as u64 == 0 {
        memset(
            ent as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<edict_t>() as libc::c_ulong,
        );
    }
    return data;
}
#[no_mangle]
pub unsafe extern "C" fn G_FindTeams() {
    let mut e: *mut edict_t = 0 as *mut edict_t;
    let mut e2: *mut edict_t = 0 as *mut edict_t;
    let mut chain: *mut edict_t = 0 as *mut edict_t;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut c2: libc::c_int = 0;
    c = 0 as libc::c_int;
    c2 = 0 as libc::c_int;
    i = 1 as libc::c_int;
    e = g_edicts.offset(i as isize);
    while i < globals.num_edicts {
        if !((*e).inuse as u64 == 0) {
            if !((*e).team).is_null() {
                if !((*e).flags & 0x400 as libc::c_int != 0) {
                    chain = e;
                    let ref mut fresh4 = (*e).teammaster;
                    *fresh4 = e;
                    c += 1;
                    c2 += 1;
                    j = i + 1 as libc::c_int;
                    e2 = e.offset(1 as libc::c_int as isize);
                    while j < globals.num_edicts {
                        if !((*e2).inuse as u64 == 0) {
                            if !((*e2).team).is_null() {
                                if !((*e2).flags & 0x400 as libc::c_int != 0) {
                                    if strcmp((*e).team, (*e2).team) == 0 {
                                        c2 += 1;
                                        let ref mut fresh5 = (*chain).teamchain;
                                        *fresh5 = e2;
                                        let ref mut fresh6 = (*e2).teammaster;
                                        *fresh6 = e;
                                        chain = e2;
                                        (*e2).flags |= 0x400 as libc::c_int;
                                    }
                                }
                            }
                        }
                        j += 1;
                        e2 = e2.offset(1);
                    }
                }
            }
        }
        i += 1;
        e = e.offset(1);
    }
    (gi.dprintf)
        .expect(
            "non-null function pointer",
        )(
        b"%i teams with %i entities\n\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        c,
        c2,
    );
}
#[no_mangle]
pub unsafe extern "C" fn SpawnEntities(
    mut mapname: *mut libc::c_char,
    mut entities: *mut libc::c_char,
    mut spawnpoint: *mut libc::c_char,
) {
    let mut ent: *mut edict_t = 0 as *mut edict_t;
    let mut inhibit: libc::c_int = 0;
    let mut com_token: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    let mut skill_level: libc::c_float = 0.;
    skill_level = floor((*skill).value as libc::c_double) as libc::c_float;
    if skill_level < 0 as libc::c_int as libc::c_float {
        skill_level = 0 as libc::c_int as libc::c_float;
    }
    if skill_level > 3 as libc::c_int as libc::c_float {
        skill_level = 3 as libc::c_int as libc::c_float;
    }
    if (*skill).value != skill_level {
        (gi.cvar_forceset)
            .expect(
                "non-null function pointer",
            )(
            b"skill\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            va(
                b"%f\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                skill_level as libc::c_double,
            ),
        );
    }
    SaveClientData();
    (gi.FreeTags).expect("non-null function pointer")(766 as libc::c_int);
    memset(
        &mut level as *mut level_locals_t as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<level_locals_t>() as libc::c_ulong,
    );
    memset(
        g_edicts as *mut libc::c_void,
        0 as libc::c_int,
        (game.maxentities as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<edict_t>() as libc::c_ulong),
    );
    strncpy(
        (level.mapname).as_mut_ptr(),
        mapname,
        (::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
    );
    strncpy(
        (game.spawnpoint).as_mut_ptr(),
        spawnpoint,
        (::std::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
    );
    i = 0 as libc::c_int;
    while i < game.maxclients {
        let ref mut fresh7 = (*g_edicts.offset((i + 1 as libc::c_int) as isize)).client;
        *fresh7 = (game.clients).offset(i as isize);
        i += 1;
    }
    ent = 0 as *mut edict_t;
    inhibit = 0 as libc::c_int;
    loop {
        com_token = COM_Parse(&mut entities);
        if entities.is_null() {
            break;
        }
        if *com_token.offset(0 as libc::c_int as isize) as libc::c_int != '{' as i32 {
            (gi.error)
                .expect(
                    "non-null function pointer",
                )(
                b"ED_LoadFromFile: found %s when expecting {\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                com_token,
            );
        }
        if ent.is_null() {
            ent = g_edicts;
        } else {
            ent = G_Spawn();
        }
        entities = ED_ParseEdict(entities, ent);
        if Q_stricmp(
            (level.mapname).as_mut_ptr(),
            b"command\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) == 0
            && Q_stricmp(
                (*ent).classname,
                b"trigger_once\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            ) == 0
            && Q_stricmp(
                (*ent).model,
                b"*27\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ) == 0
        {
            (*ent).spawnflags &= !(0x400 as libc::c_int);
        }
        if ent != g_edicts {
            if (*deathmatch).value != 0. {
                if (*ent).spawnflags & 0x800 as libc::c_int != 0 {
                    G_FreeEdict(ent);
                    inhibit += 1;
                    continue;
                }
            } else if (*skill).value == 0 as libc::c_int as libc::c_float
                && (*ent).spawnflags & 0x100 as libc::c_int != 0
                || (*skill).value == 1 as libc::c_int as libc::c_float
                    && (*ent).spawnflags & 0x200 as libc::c_int != 0
                || ((*skill).value == 2 as libc::c_int as libc::c_float
                    || (*skill).value == 3 as libc::c_int as libc::c_float)
                    && (*ent).spawnflags & 0x400 as libc::c_int != 0
            {
                G_FreeEdict(ent);
                inhibit += 1;
                continue;
            }
            (*ent).spawnflags
                &= !(0x100 as libc::c_int | 0x200 as libc::c_int | 0x400 as libc::c_int
                    | 0x1000 as libc::c_int | 0x800 as libc::c_int);
        }
        ED_CallSpawn(ent);
    }
    (gi.dprintf)
        .expect(
            "non-null function pointer",
        )(
        b"%i entities inhibited\n\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        inhibit,
    );
    G_FindTeams();
    PlayerTrail_Init();
}
#[no_mangle]
pub static mut single_statusbar: *mut libc::c_char = b"yb\t-24 xv\t0 hnum xv\t50 pic 0 if 2 \txv\t100 \tanum \txv\t150 \tpic 2 endif if 4 \txv\t200 \trnum \txv\t250 \tpic 4 endif if 6 \txv\t296 \tpic 6 endif yb\t-50 if 7 \txv\t0 \tpic 7 \txv\t26 \tyb\t-42 \tstat_string 8 \tyb\t-50 endif if 9 \txv\t262 \tnum\t2\t10 \txv\t296 \tpic\t9 endif if 11 \txv\t148 \tpic\t11 endif \0"
    as *const u8 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub static mut dm_statusbar: *mut libc::c_char = b"yb\t-24 xv\t0 hnum xv\t50 pic 0 if 2 \txv\t100 \tanum \txv\t150 \tpic 2 endif if 4 \txv\t200 \trnum \txv\t250 \tpic 4 endif if 6 \txv\t296 \tpic 6 endif yb\t-50 if 7 \txv\t0 \tpic 7 \txv\t26 \tyb\t-42 \tstat_string 8 \tyb\t-50 endif if 9 \txv\t246 \tnum\t2\t10 \txv\t296 \tpic\t9 endif if 11 \txv\t148 \tpic\t11 endif xr\t-50 yt 2 num 3 14 if 17 xv 0 yb -58 string2 \"SPECTATOR MODE\" endif if 16 xv 0 yb -68 string \"Chasing\" xv 64 stat_string 16 endif \0"
    as *const u8 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub unsafe extern "C" fn SP_worldspawn(mut ent: *mut edict_t) {
    (*ent).movetype = MOVETYPE_PUSH as libc::c_int;
    (*ent).solid = SOLID_BSP;
    (*ent).inuse = true_0;
    (*ent).s.modelindex = 1 as libc::c_int;
    InitBodyQue();
    SetItemNames();
    if !(st.nextmap).is_null() {
        strcpy((level.nextmap).as_mut_ptr(), st.nextmap);
    }
    if !((*ent).message).is_null()
        && *((*ent).message).offset(0 as libc::c_int as isize) as libc::c_int != 0
    {
        (gi.configstring)
            .expect("non-null function pointer")(0 as libc::c_int, (*ent).message);
        strncpy(
            (level.level_name).as_mut_ptr(),
            (*ent).message,
            ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong,
        );
    } else {
        strncpy(
            (level.level_name).as_mut_ptr(),
            (level.mapname).as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong,
        );
    }
    if !(st.sky).is_null()
        && *(st.sky).offset(0 as libc::c_int as isize) as libc::c_int != 0
    {
        (gi.configstring).expect("non-null function pointer")(2 as libc::c_int, st.sky);
    } else {
        (gi.configstring)
            .expect(
                "non-null function pointer",
            )(
            2 as libc::c_int,
            b"unit1_\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
    (gi.configstring)
        .expect(
            "non-null function pointer",
        )(
        4 as libc::c_int,
        va(
            b"%f\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            st.skyrotate as libc::c_double,
        ),
    );
    (gi.configstring)
        .expect(
            "non-null function pointer",
        )(
        3 as libc::c_int,
        va(
            b"%f %f %f\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            st.skyaxis[0 as libc::c_int as usize] as libc::c_double,
            st.skyaxis[1 as libc::c_int as usize] as libc::c_double,
            st.skyaxis[2 as libc::c_int as usize] as libc::c_double,
        ),
    );
    (gi.configstring)
        .expect(
            "non-null function pointer",
        )(
        1 as libc::c_int,
        va(
            b"%i\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            (*ent).sounds,
        ),
    );
    (gi.configstring)
        .expect(
            "non-null function pointer",
        )(
        30 as libc::c_int,
        va(
            b"%i\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            (*maxclients).value as libc::c_int,
        ),
    );
    if (*deathmatch).value != 0. {
        (gi.configstring)
            .expect("non-null function pointer")(5 as libc::c_int, dm_statusbar);
    } else {
        (gi.configstring)
            .expect("non-null function pointer")(5 as libc::c_int, single_statusbar);
    }
    (gi.imageindex)
        .expect(
            "non-null function pointer",
        )(b"i_help\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    level
        .pic_health = (gi.imageindex)
        .expect(
            "non-null function pointer",
        )(b"i_health\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    (gi.imageindex)
        .expect(
            "non-null function pointer",
        )(b"help\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    (gi.imageindex)
        .expect(
            "non-null function pointer",
        )(b"field_3\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    if (st.gravity).is_null() {
        (gi.cvar_set)
            .expect(
                "non-null function pointer",
            )(
            b"sv_gravity\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"800\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    } else {
        (gi.cvar_set)
            .expect(
                "non-null function pointer",
            )(
            b"sv_gravity\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            st.gravity,
        );
    }
    snd_fry = (gi.soundindex)
        .expect(
            "non-null function pointer",
        )(b"player/fry.wav\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    PrecacheItem(
        FindItem(b"Blaster\0" as *const u8 as *const libc::c_char as *mut libc::c_char),
    );
    (gi.soundindex)
        .expect(
            "non-null function pointer",
        )(
        b"player/lava1.wav\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    (gi.soundindex)
        .expect(
            "non-null function pointer",
        )(
        b"player/lava2.wav\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    (gi.soundindex)
        .expect(
            "non-null function pointer",
        )(b"misc/pc_up.wav\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    (gi.soundindex)
        .expect(
            "non-null function pointer",
        )(b"misc/talk1.wav\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    (gi.soundindex)
        .expect(
            "non-null function pointer",
        )(b"misc/udeath.wav\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    (gi.soundindex)
        .expect(
            "non-null function pointer",
        )(
        b"items/respawn1.wav\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    (gi.soundindex)
        .expect(
            "non-null function pointer",
        )(b"*death1.wav\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    (gi.soundindex)
        .expect(
            "non-null function pointer",
        )(b"*death2.wav\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    (gi.soundindex)
        .expect(
            "non-null function pointer",
        )(b"*death3.wav\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    (gi.soundindex)
        .expect(
            "non-null function pointer",
        )(b"*death4.wav\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    (gi.soundindex)
        .expect(
            "non-null function pointer",
        )(b"*fall1.wav\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    (gi.soundindex)
        .expect(
            "non-null function pointer",
        )(b"*fall2.wav\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    (gi.soundindex)
        .expect(
            "non-null function pointer",
        )(b"*gurp1.wav\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    (gi.soundindex)
        .expect(
            "non-null function pointer",
        )(b"*gurp2.wav\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    (gi.soundindex)
        .expect(
            "non-null function pointer",
        )(b"*jump1.wav\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    (gi.soundindex)
        .expect(
            "non-null function pointer",
        )(b"*pain25_1.wav\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    (gi.soundindex)
        .expect(
            "non-null function pointer",
        )(b"*pain25_2.wav\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    (gi.soundindex)
        .expect(
            "non-null function pointer",
        )(b"*pain50_1.wav\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    (gi.soundindex)
        .expect(
            "non-null function pointer",
        )(b"*pain50_2.wav\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    (gi.soundindex)
        .expect(
            "non-null function pointer",
        )(b"*pain75_1.wav\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    (gi.soundindex)
        .expect(
            "non-null function pointer",
        )(b"*pain75_2.wav\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    (gi.soundindex)
        .expect(
            "non-null function pointer",
        )(b"*pain100_1.wav\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    (gi.soundindex)
        .expect(
            "non-null function pointer",
        )(b"*pain100_2.wav\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    (gi.modelindex)
        .expect(
            "non-null function pointer",
        )(b"#w_blaster.md2\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    (gi.modelindex)
        .expect(
            "non-null function pointer",
        )(b"#w_shotgun.md2\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    (gi.modelindex)
        .expect(
            "non-null function pointer",
        )(b"#w_sshotgun.md2\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    (gi.modelindex)
        .expect(
            "non-null function pointer",
        )(
        b"#w_machinegun.md2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    (gi.modelindex)
        .expect(
            "non-null function pointer",
        )(b"#w_chaingun.md2\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    (gi.modelindex)
        .expect(
            "non-null function pointer",
        )(b"#a_grenades.md2\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    (gi.modelindex)
        .expect(
            "non-null function pointer",
        )(
        b"#w_glauncher.md2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    (gi.modelindex)
        .expect(
            "non-null function pointer",
        )(
        b"#w_rlauncher.md2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    (gi.modelindex)
        .expect(
            "non-null function pointer",
        )(
        b"#w_hyperblaster.md2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    (gi.modelindex)
        .expect(
            "non-null function pointer",
        )(b"#w_railgun.md2\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    (gi.modelindex)
        .expect(
            "non-null function pointer",
        )(b"#w_bfg.md2\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    (gi.soundindex)
        .expect(
            "non-null function pointer",
        )(
        b"player/gasp1.wav\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    (gi.soundindex)
        .expect(
            "non-null function pointer",
        )(
        b"player/gasp2.wav\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    (gi.soundindex)
        .expect(
            "non-null function pointer",
        )(
        b"player/watr_in.wav\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    (gi.soundindex)
        .expect(
            "non-null function pointer",
        )(
        b"player/watr_out.wav\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    (gi.soundindex)
        .expect(
            "non-null function pointer",
        )(
        b"player/watr_un.wav\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    (gi.soundindex)
        .expect(
            "non-null function pointer",
        )(
        b"player/u_breath1.wav\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    (gi.soundindex)
        .expect(
            "non-null function pointer",
        )(
        b"player/u_breath2.wav\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    (gi.soundindex)
        .expect(
            "non-null function pointer",
        )(b"items/pkup.wav\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    (gi.soundindex)
        .expect(
            "non-null function pointer",
        )(b"world/land.wav\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    (gi.soundindex)
        .expect(
            "non-null function pointer",
        )(
        b"misc/h2ohit1.wav\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    (gi.soundindex)
        .expect(
            "non-null function pointer",
        )(
        b"items/damage.wav\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    (gi.soundindex)
        .expect(
            "non-null function pointer",
        )(
        b"items/protect.wav\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    (gi.soundindex)
        .expect(
            "non-null function pointer",
        )(
        b"items/protect4.wav\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    (gi.soundindex)
        .expect(
            "non-null function pointer",
        )(
        b"weapons/noammo.wav\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    (gi.soundindex)
        .expect(
            "non-null function pointer",
        )(
        b"infantry/inflies1.wav\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    sm_meat_index = (gi.modelindex)
        .expect(
            "non-null function pointer",
        )(
        b"models/objects/gibs/sm_meat/tris.md2\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    (gi.modelindex)
        .expect(
            "non-null function pointer",
        )(
        b"models/objects/gibs/arm/tris.md2\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    (gi.modelindex)
        .expect(
            "non-null function pointer",
        )(
        b"models/objects/gibs/bone/tris.md2\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    (gi.modelindex)
        .expect(
            "non-null function pointer",
        )(
        b"models/objects/gibs/bone2/tris.md2\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    (gi.modelindex)
        .expect(
            "non-null function pointer",
        )(
        b"models/objects/gibs/chest/tris.md2\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    (gi.modelindex)
        .expect(
            "non-null function pointer",
        )(
        b"models/objects/gibs/skull/tris.md2\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    (gi.modelindex)
        .expect(
            "non-null function pointer",
        )(
        b"models/objects/gibs/head2/tris.md2\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    (gi.configstring)
        .expect(
            "non-null function pointer",
        )(
        32 as libc::c_int + 256 as libc::c_int + 256 as libc::c_int + 256 as libc::c_int
            + 0 as libc::c_int,
        b"m\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    (gi.configstring)
        .expect(
            "non-null function pointer",
        )(
        32 as libc::c_int + 256 as libc::c_int + 256 as libc::c_int + 256 as libc::c_int
            + 1 as libc::c_int,
        b"mmnmmommommnonmmonqnmmo\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    (gi.configstring)
        .expect(
            "non-null function pointer",
        )(
        32 as libc::c_int + 256 as libc::c_int + 256 as libc::c_int + 256 as libc::c_int
            + 2 as libc::c_int,
        b"abcdefghijklmnopqrstuvwxyzyxwvutsrqponmlkjihgfedcba\0" as *const u8
            as *const libc::c_char as *mut libc::c_char,
    );
    (gi.configstring)
        .expect(
            "non-null function pointer",
        )(
        32 as libc::c_int + 256 as libc::c_int + 256 as libc::c_int + 256 as libc::c_int
            + 3 as libc::c_int,
        b"mmmmmaaaaammmmmaaaaaabcdefgabcdefg\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    (gi.configstring)
        .expect(
            "non-null function pointer",
        )(
        32 as libc::c_int + 256 as libc::c_int + 256 as libc::c_int + 256 as libc::c_int
            + 4 as libc::c_int,
        b"mamamamamama\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    (gi.configstring)
        .expect(
            "non-null function pointer",
        )(
        32 as libc::c_int + 256 as libc::c_int + 256 as libc::c_int + 256 as libc::c_int
            + 5 as libc::c_int,
        b"jklmnopqrstuvwxyzyxwvutsrqponmlkj\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    (gi.configstring)
        .expect(
            "non-null function pointer",
        )(
        32 as libc::c_int + 256 as libc::c_int + 256 as libc::c_int + 256 as libc::c_int
            + 6 as libc::c_int,
        b"nmonqnmomnmomomno\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    (gi.configstring)
        .expect(
            "non-null function pointer",
        )(
        32 as libc::c_int + 256 as libc::c_int + 256 as libc::c_int + 256 as libc::c_int
            + 7 as libc::c_int,
        b"mmmaaaabcdefgmmmmaaaammmaamm\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    (gi.configstring)
        .expect(
            "non-null function pointer",
        )(
        32 as libc::c_int + 256 as libc::c_int + 256 as libc::c_int + 256 as libc::c_int
            + 8 as libc::c_int,
        b"mmmaaammmaaammmabcdefaaaammmmabcdefmmmaaaa\0" as *const u8
            as *const libc::c_char as *mut libc::c_char,
    );
    (gi.configstring)
        .expect(
            "non-null function pointer",
        )(
        32 as libc::c_int + 256 as libc::c_int + 256 as libc::c_int + 256 as libc::c_int
            + 9 as libc::c_int,
        b"aaaaaaaazzzzzzzz\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    (gi.configstring)
        .expect(
            "non-null function pointer",
        )(
        32 as libc::c_int + 256 as libc::c_int + 256 as libc::c_int + 256 as libc::c_int
            + 10 as libc::c_int,
        b"mmamammmmammamamaaamammma\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    (gi.configstring)
        .expect(
            "non-null function pointer",
        )(
        32 as libc::c_int + 256 as libc::c_int + 256 as libc::c_int + 256 as libc::c_int
            + 11 as libc::c_int,
        b"abcdefghijklmnopqrrqponmlkjihgfedcba\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    (gi.configstring)
        .expect(
            "non-null function pointer",
        )(
        32 as libc::c_int + 256 as libc::c_int + 256 as libc::c_int + 256 as libc::c_int
            + 63 as libc::c_int,
        b"a\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
}
