#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    fn ceil(_: libc::c_double) -> libc::c_double;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    static mut vec3_origin: vec3_t;
    fn VectorMA(
        veca: *mut vec_t,
        scale: libc::c_float,
        vecb: *mut vec_t,
        vecc: *mut vec_t,
    );
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
    static mut meansOfDeath: libc::c_int;
    static mut deathmatch: *mut cvar_t;
    static mut coop: *mut cvar_t;
    static mut dmflags: *mut cvar_t;
    static mut skill: *mut cvar_t;
    static mut itemlist: [gitem_t; 0];
    fn FindItem(pickup_name: *mut libc::c_char) -> *mut gitem_t;
    fn ArmorIndex(ent: *mut edict_t) -> libc::c_int;
    fn PowerArmorType(ent: *mut edict_t) -> libc::c_int;
    fn GetItemByIndex(index: libc::c_int) -> *mut gitem_t;
    fn findradius(
        from: *mut edict_t,
        org: *mut vec_t,
        rad: libc::c_float,
    ) -> *mut edict_t;
    fn OnSameTeam(ent1: *mut edict_t, ent2: *mut edict_t) -> qboolean;
    static mut ctf: *mut cvar_t;
    fn CTFMatchSetup() -> qboolean;
    fn FoundTarget(self_0: *mut edict_t);
    fn visible(self_0: *mut edict_t, other: *mut edict_t) -> qboolean;
    fn monster_death_use(self_0: *mut edict_t);
    fn CTFCheckHurtCarrier(targ: *mut edict_t, attacker: *mut edict_t);
    fn CTFApplyResistance(ent: *mut edict_t, dmg: libc::c_int) -> libc::c_int;
    fn CTFApplyStrength(ent: *mut edict_t, dmg: libc::c_int) -> libc::c_int;
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
pub struct gitem_armor_t {
    pub base_count: libc::c_int,
    pub max_count: libc::c_int,
    pub normal_protection: libc::c_float,
    pub energy_protection: libc::c_float,
    pub armor: libc::c_int,
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
#[no_mangle]
pub unsafe extern "C" fn CanDamage(
    mut targ: *mut edict_t,
    mut inflictor: *mut edict_t,
) -> qboolean {
    let mut dest: vec3_t = [0.; 3];
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
    if (*targ).movetype == MOVETYPE_PUSH as libc::c_int {
        dest[0 as libc::c_int
            as usize] = (*targ).absmin[0 as libc::c_int as usize]
            + (*targ).absmax[0 as libc::c_int as usize];
        dest[1 as libc::c_int
            as usize] = (*targ).absmin[1 as libc::c_int as usize]
            + (*targ).absmax[1 as libc::c_int as usize];
        dest[2 as libc::c_int
            as usize] = (*targ).absmin[2 as libc::c_int as usize]
            + (*targ).absmax[2 as libc::c_int as usize];
        VectorScale(dest.as_mut_ptr(), 0.5f64 as vec_t, dest.as_mut_ptr());
        trace = (gi.trace)
            .expect(
                "non-null function pointer",
            )(
            ((*inflictor).s.origin).as_mut_ptr(),
            vec3_origin.as_mut_ptr(),
            vec3_origin.as_mut_ptr(),
            dest.as_mut_ptr(),
            inflictor,
            1 as libc::c_int | 2 as libc::c_int,
        );
        if trace.fraction as libc::c_double == 1.0f64 {
            return true_0;
        }
        if trace.ent == targ {
            return true_0;
        }
        return false_0;
    }
    trace = (gi.trace)
        .expect(
            "non-null function pointer",
        )(
        ((*inflictor).s.origin).as_mut_ptr(),
        vec3_origin.as_mut_ptr(),
        vec3_origin.as_mut_ptr(),
        ((*targ).s.origin).as_mut_ptr(),
        inflictor,
        1 as libc::c_int | 2 as libc::c_int,
    );
    if trace.fraction as libc::c_double == 1.0f64 {
        return true_0;
    }
    dest[0 as libc::c_int as usize] = (*targ).s.origin[0 as libc::c_int as usize];
    dest[1 as libc::c_int as usize] = (*targ).s.origin[1 as libc::c_int as usize];
    dest[2 as libc::c_int as usize] = (*targ).s.origin[2 as libc::c_int as usize];
    dest[0 as libc::c_int
        as usize] = (dest[0 as libc::c_int as usize] as libc::c_double + 15.0f64)
        as vec_t;
    dest[1 as libc::c_int
        as usize] = (dest[1 as libc::c_int as usize] as libc::c_double + 15.0f64)
        as vec_t;
    trace = (gi.trace)
        .expect(
            "non-null function pointer",
        )(
        ((*inflictor).s.origin).as_mut_ptr(),
        vec3_origin.as_mut_ptr(),
        vec3_origin.as_mut_ptr(),
        dest.as_mut_ptr(),
        inflictor,
        1 as libc::c_int | 2 as libc::c_int,
    );
    if trace.fraction as libc::c_double == 1.0f64 {
        return true_0;
    }
    dest[0 as libc::c_int as usize] = (*targ).s.origin[0 as libc::c_int as usize];
    dest[1 as libc::c_int as usize] = (*targ).s.origin[1 as libc::c_int as usize];
    dest[2 as libc::c_int as usize] = (*targ).s.origin[2 as libc::c_int as usize];
    dest[0 as libc::c_int
        as usize] = (dest[0 as libc::c_int as usize] as libc::c_double + 15.0f64)
        as vec_t;
    dest[1 as libc::c_int
        as usize] = (dest[1 as libc::c_int as usize] as libc::c_double - 15.0f64)
        as vec_t;
    trace = (gi.trace)
        .expect(
            "non-null function pointer",
        )(
        ((*inflictor).s.origin).as_mut_ptr(),
        vec3_origin.as_mut_ptr(),
        vec3_origin.as_mut_ptr(),
        dest.as_mut_ptr(),
        inflictor,
        1 as libc::c_int | 2 as libc::c_int,
    );
    if trace.fraction as libc::c_double == 1.0f64 {
        return true_0;
    }
    dest[0 as libc::c_int as usize] = (*targ).s.origin[0 as libc::c_int as usize];
    dest[1 as libc::c_int as usize] = (*targ).s.origin[1 as libc::c_int as usize];
    dest[2 as libc::c_int as usize] = (*targ).s.origin[2 as libc::c_int as usize];
    dest[0 as libc::c_int
        as usize] = (dest[0 as libc::c_int as usize] as libc::c_double - 15.0f64)
        as vec_t;
    dest[1 as libc::c_int
        as usize] = (dest[1 as libc::c_int as usize] as libc::c_double + 15.0f64)
        as vec_t;
    trace = (gi.trace)
        .expect(
            "non-null function pointer",
        )(
        ((*inflictor).s.origin).as_mut_ptr(),
        vec3_origin.as_mut_ptr(),
        vec3_origin.as_mut_ptr(),
        dest.as_mut_ptr(),
        inflictor,
        1 as libc::c_int | 2 as libc::c_int,
    );
    if trace.fraction as libc::c_double == 1.0f64 {
        return true_0;
    }
    dest[0 as libc::c_int as usize] = (*targ).s.origin[0 as libc::c_int as usize];
    dest[1 as libc::c_int as usize] = (*targ).s.origin[1 as libc::c_int as usize];
    dest[2 as libc::c_int as usize] = (*targ).s.origin[2 as libc::c_int as usize];
    dest[0 as libc::c_int
        as usize] = (dest[0 as libc::c_int as usize] as libc::c_double - 15.0f64)
        as vec_t;
    dest[1 as libc::c_int
        as usize] = (dest[1 as libc::c_int as usize] as libc::c_double - 15.0f64)
        as vec_t;
    trace = (gi.trace)
        .expect(
            "non-null function pointer",
        )(
        ((*inflictor).s.origin).as_mut_ptr(),
        vec3_origin.as_mut_ptr(),
        vec3_origin.as_mut_ptr(),
        dest.as_mut_ptr(),
        inflictor,
        1 as libc::c_int | 2 as libc::c_int,
    );
    if trace.fraction as libc::c_double == 1.0f64 {
        return true_0;
    }
    return false_0;
}
#[no_mangle]
pub unsafe extern "C" fn Killed(
    mut targ: *mut edict_t,
    mut inflictor: *mut edict_t,
    mut attacker: *mut edict_t,
    mut damage: libc::c_int,
    mut point: *mut vec_t,
) {
    if (*targ).health < -(999 as libc::c_int) {
        (*targ).health = -(999 as libc::c_int);
    }
    let ref mut fresh0 = (*targ).enemy;
    *fresh0 = attacker;
    if (*targ).svflags & 0x4 as libc::c_int != 0 && (*targ).deadflag != 2 as libc::c_int
    {
        if (*targ).monsterinfo.aiflags & 0x100 as libc::c_int == 0 {
            level.killed_monsters += 1;
            if (*coop).value != 0. && !((*attacker).client).is_null() {
                let ref mut fresh1 = (*(*attacker).client).resp.score;
                *fresh1 += 1;
            }
            if strcmp(
                (*attacker).classname,
                b"monster_medic\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
            {
                let ref mut fresh2 = (*targ).owner;
                *fresh2 = attacker;
            }
        }
    }
    if (*targ).movetype == MOVETYPE_PUSH as libc::c_int
        || (*targ).movetype == MOVETYPE_STOP as libc::c_int
        || (*targ).movetype == MOVETYPE_NONE as libc::c_int
    {
        ((*targ).die)
            .expect(
                "non-null function pointer",
            )(targ, inflictor, attacker, damage, point);
        return;
    }
    if (*targ).svflags & 0x4 as libc::c_int != 0 && (*targ).deadflag != 2 as libc::c_int
    {
        let ref mut fresh3 = (*targ).touch;
        *fresh3 = None;
        monster_death_use(targ);
    }
    ((*targ).die)
        .expect("non-null function pointer")(targ, inflictor, attacker, damage, point);
}
#[no_mangle]
pub unsafe extern "C" fn SpawnDamage(
    mut type_0: libc::c_int,
    mut origin: *mut vec_t,
    mut normal: *mut vec_t,
    mut damage: libc::c_int,
) {
    if damage > 255 as libc::c_int {
        damage = 255 as libc::c_int;
    }
    (gi.WriteByte).expect("non-null function pointer")(3 as libc::c_int);
    (gi.WriteByte).expect("non-null function pointer")(type_0);
    (gi.WritePosition).expect("non-null function pointer")(origin);
    (gi.WriteDir).expect("non-null function pointer")(normal);
    (gi.multicast).expect("non-null function pointer")(origin, MULTICAST_PVS);
}
unsafe extern "C" fn CheckPowerArmor(
    mut ent: *mut edict_t,
    mut point: *mut vec_t,
    mut normal: *mut vec_t,
    mut damage: libc::c_int,
    mut dflags: libc::c_int,
) -> libc::c_int {
    let mut client: *mut gclient_t = 0 as *mut gclient_t;
    let mut save: libc::c_int = 0;
    let mut power_armor_type: libc::c_int = 0;
    let mut index: libc::c_int = 0;
    let mut damagePerCell: libc::c_int = 0;
    let mut pa_te_type: libc::c_int = 0;
    let mut power: libc::c_int = 0;
    let mut power_used: libc::c_int = 0;
    if damage == 0 {
        return 0 as libc::c_int;
    }
    client = (*ent).client;
    if dflags & 0x2 as libc::c_int != 0 {
        return 0 as libc::c_int;
    }
    if !client.is_null() {
        power_armor_type = PowerArmorType(ent);
        if power_armor_type != 0 as libc::c_int {
            index = (FindItem(
                b"Cells\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ))
                .offset_from(itemlist.as_mut_ptr()) as libc::c_long as libc::c_int;
            power = (*client).pers.inventory[index as usize];
        }
    } else if (*ent).svflags & 0x4 as libc::c_int != 0 {
        power_armor_type = (*ent).monsterinfo.power_armor_type;
        power = (*ent).monsterinfo.power_armor_power;
    } else {
        return 0 as libc::c_int
    }
    if power_armor_type == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    if power == 0 {
        return 0 as libc::c_int;
    }
    if power_armor_type == 1 as libc::c_int {
        let mut vec: vec3_t = [0.; 3];
        let mut dot: libc::c_float = 0.;
        let mut forward: vec3_t = [0.; 3];
        AngleVectors(
            ((*ent).s.angles).as_mut_ptr(),
            forward.as_mut_ptr(),
            0 as *mut vec_t,
            0 as *mut vec_t,
        );
        vec[0 as libc::c_int
            as usize] = *point.offset(0 as libc::c_int as isize)
            - (*ent).s.origin[0 as libc::c_int as usize];
        vec[1 as libc::c_int
            as usize] = *point.offset(1 as libc::c_int as isize)
            - (*ent).s.origin[1 as libc::c_int as usize];
        vec[2 as libc::c_int
            as usize] = *point.offset(2 as libc::c_int as isize)
            - (*ent).s.origin[2 as libc::c_int as usize];
        VectorNormalize(vec.as_mut_ptr());
        dot = vec[0 as libc::c_int as usize] * forward[0 as libc::c_int as usize]
            + vec[1 as libc::c_int as usize] * forward[1 as libc::c_int as usize]
            + vec[2 as libc::c_int as usize] * forward[2 as libc::c_int as usize];
        if dot as libc::c_double <= 0.3f64 {
            return 0 as libc::c_int;
        }
        damagePerCell = 1 as libc::c_int;
        pa_te_type = TE_SCREEN_SPARKS as libc::c_int;
        damage = damage / 3 as libc::c_int;
    } else {
        damagePerCell = 1 as libc::c_int;
        pa_te_type = TE_SHIELD_SPARKS as libc::c_int;
        damage = 2 as libc::c_int * damage / 3 as libc::c_int;
    }
    save = power * damagePerCell;
    if save == 0 {
        return 0 as libc::c_int;
    }
    if save > damage {
        save = damage;
    }
    SpawnDamage(pa_te_type, point, normal, save);
    (*ent).powerarmor_time = (level.time as libc::c_double + 0.2f64) as libc::c_float;
    power_used = save / damagePerCell;
    if !client.is_null() {
        (*client).pers.inventory[index as usize] -= power_used;
    } else {
        (*ent).monsterinfo.power_armor_power -= power_used;
    }
    return save;
}
unsafe extern "C" fn CheckArmor(
    mut ent: *mut edict_t,
    mut point: *mut vec_t,
    mut normal: *mut vec_t,
    mut damage: libc::c_int,
    mut te_sparks: libc::c_int,
    mut dflags: libc::c_int,
) -> libc::c_int {
    let mut client: *mut gclient_t = 0 as *mut gclient_t;
    let mut save: libc::c_int = 0;
    let mut index: libc::c_int = 0;
    let mut armor: *mut gitem_t = 0 as *mut gitem_t;
    if damage == 0 {
        return 0 as libc::c_int;
    }
    client = (*ent).client;
    if client.is_null() {
        return 0 as libc::c_int;
    }
    if dflags & 0x2 as libc::c_int != 0 {
        return 0 as libc::c_int;
    }
    index = ArmorIndex(ent);
    if index == 0 {
        return 0 as libc::c_int;
    }
    armor = GetItemByIndex(index);
    if dflags & 0x4 as libc::c_int != 0 {
        save = ceil(
            ((*((*armor).info as *mut gitem_armor_t)).energy_protection
                * damage as libc::c_float) as libc::c_double,
        ) as libc::c_int;
    } else {
        save = ceil(
            ((*((*armor).info as *mut gitem_armor_t)).normal_protection
                * damage as libc::c_float) as libc::c_double,
        ) as libc::c_int;
    }
    if save >= (*client).pers.inventory[index as usize] {
        save = (*client).pers.inventory[index as usize];
    }
    if save == 0 {
        return 0 as libc::c_int;
    }
    (*client).pers.inventory[index as usize] -= save;
    SpawnDamage(te_sparks, point, normal, save);
    return save;
}
#[no_mangle]
pub unsafe extern "C" fn M_ReactToDamage(
    mut targ: *mut edict_t,
    mut attacker: *mut edict_t,
) {
    if ((*attacker).client).is_null() && (*attacker).svflags & 0x4 as libc::c_int == 0 {
        return;
    }
    if attacker == targ || attacker == (*targ).enemy {
        return;
    }
    if (*targ).monsterinfo.aiflags & 0x100 as libc::c_int != 0 {
        if !((*attacker).client).is_null()
            || (*attacker).monsterinfo.aiflags & 0x100 as libc::c_int != 0
        {
            return;
        }
    }
    if !((*attacker).client).is_null() {
        if !((*targ).enemy).is_null() && !((*(*targ).enemy).client).is_null() {
            if visible(targ, (*targ).enemy) as u64 != 0 {
                let ref mut fresh4 = (*targ).oldenemy;
                *fresh4 = attacker;
                return;
            }
            let ref mut fresh5 = (*targ).oldenemy;
            *fresh5 = (*targ).enemy;
        }
        let ref mut fresh6 = (*targ).enemy;
        *fresh6 = attacker;
        if (*targ).monsterinfo.aiflags & 0x800 as libc::c_int == 0 {
            FoundTarget(targ);
        }
        return;
    }
    if (*targ).flags & (0x1 as libc::c_int | 0x2 as libc::c_int)
        == (*attacker).flags & (0x1 as libc::c_int | 0x2 as libc::c_int)
        && strcmp((*targ).classname, (*attacker).classname) != 0 as libc::c_int
        && strcmp(
            (*attacker).classname,
            b"monster_tank\0" as *const u8 as *const libc::c_char,
        ) != 0 as libc::c_int
        && strcmp(
            (*attacker).classname,
            b"monster_supertank\0" as *const u8 as *const libc::c_char,
        ) != 0 as libc::c_int
        && strcmp(
            (*attacker).classname,
            b"monster_makron\0" as *const u8 as *const libc::c_char,
        ) != 0 as libc::c_int
        && strcmp(
            (*attacker).classname,
            b"monster_jorg\0" as *const u8 as *const libc::c_char,
        ) != 0 as libc::c_int
    {
        if !((*targ).enemy).is_null() {
            if !((*(*targ).enemy).client).is_null() {
                let ref mut fresh7 = (*targ).oldenemy;
                *fresh7 = (*targ).enemy;
            }
        }
        let ref mut fresh8 = (*targ).enemy;
        *fresh8 = attacker;
        if (*targ).monsterinfo.aiflags & 0x800 as libc::c_int == 0 {
            FoundTarget(targ);
        }
    } else {
        if !((*targ).enemy).is_null() {
            if !((*(*targ).enemy).client).is_null() {
                let ref mut fresh9 = (*targ).oldenemy;
                *fresh9 = (*targ).enemy;
            }
        }
        let ref mut fresh10 = (*targ).enemy;
        *fresh10 = (*attacker).enemy;
        FoundTarget(targ);
    };
}
#[no_mangle]
pub unsafe extern "C" fn CheckTeamDamage(
    mut targ: *mut edict_t,
    mut attacker: *mut edict_t,
) -> qboolean {
    if (*ctf).value != 0. && !((*targ).client).is_null()
        && !((*attacker).client).is_null()
    {
        if (*(*targ).client).resp.ctf_team == (*(*attacker).client).resp.ctf_team
            && targ != attacker
        {
            return true_0;
        }
    }
    return false_0;
}
#[no_mangle]
pub unsafe extern "C" fn T_Damage(
    mut targ: *mut edict_t,
    mut inflictor: *mut edict_t,
    mut attacker: *mut edict_t,
    mut dir: *mut vec_t,
    mut point: *mut vec_t,
    mut normal: *mut vec_t,
    mut damage: libc::c_int,
    mut knockback: libc::c_int,
    mut dflags: libc::c_int,
    mut mod_0: libc::c_int,
) {
    let mut client: *mut gclient_t = 0 as *mut gclient_t;
    let mut take: libc::c_int = 0;
    let mut save: libc::c_int = 0;
    let mut asave: libc::c_int = 0;
    let mut psave: libc::c_int = 0;
    let mut te_sparks: libc::c_int = 0;
    if (*targ).takedamage == 0 {
        return;
    }
    if targ != attacker
        && ((*deathmatch).value != 0.
            && (*dmflags).value as libc::c_int
                & (0x80 as libc::c_int | 0x40 as libc::c_int) != 0
            || (*coop).value != 0.)
    {
        if OnSameTeam(targ, attacker) as u64 != 0 {
            if (*dmflags).value as libc::c_int & 0x100 as libc::c_int != 0 {
                damage = 0 as libc::c_int;
            } else {
                mod_0 |= 0x8000000 as libc::c_int;
            }
        }
    }
    meansOfDeath = mod_0;
    if (*skill).value == 0 as libc::c_int as libc::c_float
        && (*deathmatch).value == 0 as libc::c_int as libc::c_float
        && !((*targ).client).is_null()
    {
        damage = (damage as libc::c_double * 0.5f64) as libc::c_int;
        if damage == 0 {
            damage = 1 as libc::c_int;
        }
    }
    client = (*targ).client;
    if dflags & 0x10 as libc::c_int != 0 {
        te_sparks = TE_BULLET_SPARKS as libc::c_int;
    } else {
        te_sparks = TE_SPARKS as libc::c_int;
    }
    VectorNormalize(dir);
    if dflags & 0x1 as libc::c_int == 0 && (*targ).svflags & 0x4 as libc::c_int != 0
        && !((*attacker).client).is_null() && ((*targ).enemy).is_null()
        && (*targ).health > 0 as libc::c_int
    {
        damage *= 2 as libc::c_int;
    }
    damage = CTFApplyStrength(attacker, damage);
    if (*targ).flags & 0x800 as libc::c_int != 0 {
        knockback = 0 as libc::c_int;
    }
    if dflags & 0x8 as libc::c_int == 0 {
        if knockback != 0 && (*targ).movetype != MOVETYPE_NONE as libc::c_int
            && (*targ).movetype != MOVETYPE_BOUNCE as libc::c_int
            && (*targ).movetype != MOVETYPE_PUSH as libc::c_int
            && (*targ).movetype != MOVETYPE_STOP as libc::c_int
        {
            let mut kvel: vec3_t = [0.; 3];
            let mut mass: libc::c_float = 0.;
            if (*targ).mass < 50 as libc::c_int {
                mass = 50 as libc::c_int as libc::c_float;
            } else {
                mass = (*targ).mass as libc::c_float;
            }
            if !((*targ).client).is_null() && attacker == targ {
                VectorScale(
                    dir,
                    (1600.0f64 * knockback as libc::c_float as libc::c_double
                        / mass as libc::c_double) as vec_t,
                    kvel.as_mut_ptr(),
                );
            } else {
                VectorScale(
                    dir,
                    (500.0f64 * knockback as libc::c_float as libc::c_double
                        / mass as libc::c_double) as vec_t,
                    kvel.as_mut_ptr(),
                );
            }
            (*targ)
                .velocity[0 as libc::c_int
                as usize] = (*targ).velocity[0 as libc::c_int as usize]
                + kvel[0 as libc::c_int as usize];
            (*targ)
                .velocity[1 as libc::c_int
                as usize] = (*targ).velocity[1 as libc::c_int as usize]
                + kvel[1 as libc::c_int as usize];
            (*targ)
                .velocity[2 as libc::c_int
                as usize] = (*targ).velocity[2 as libc::c_int as usize]
                + kvel[2 as libc::c_int as usize];
        }
    }
    take = damage;
    save = 0 as libc::c_int;
    if (*targ).flags & 0x10 as libc::c_int != 0 && dflags & 0x20 as libc::c_int == 0 {
        take = 0 as libc::c_int;
        save = damage;
        SpawnDamage(te_sparks, point, normal, save);
    }
    if !client.is_null()
        && (*client).invincible_framenum > level.framenum as libc::c_float
        && dflags & 0x20 as libc::c_int == 0
    {
        if (*targ).pain_debounce_time < level.time {
            (gi.sound)
                .expect(
                    "non-null function pointer",
                )(
                targ,
                3 as libc::c_int,
                (gi.soundindex)
                    .expect(
                        "non-null function pointer",
                    )(
                    b"items/protect4.wav\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                ),
                1 as libc::c_int as libc::c_float,
                1 as libc::c_int as libc::c_float,
                0 as libc::c_int as libc::c_float,
            );
            (*targ).pain_debounce_time = level.time + 2 as libc::c_int as libc::c_float;
        }
        take = 0 as libc::c_int;
        save = damage;
    }
    if (*ctf).value != 0. && !((*targ).client).is_null()
        && !((*attacker).client).is_null()
        && (*(*targ).client).resp.ctf_team == (*(*attacker).client).resp.ctf_team
        && targ != attacker
        && (*dmflags).value as libc::c_int & 262144 as libc::c_int != 0
    {
        asave = 0 as libc::c_int;
        psave = asave;
    } else {
        psave = CheckPowerArmor(targ, point, normal, take, dflags);
        take -= psave;
        asave = CheckArmor(targ, point, normal, take, te_sparks, dflags);
        take -= asave;
    }
    asave += save;
    take = CTFApplyResistance(targ, take);
    if dflags & 0x20 as libc::c_int == 0
        && CheckTeamDamage(targ, attacker) as libc::c_uint != 0
    {
        return;
    }
    CTFCheckHurtCarrier(targ, attacker);
    if take != 0 {
        if (*targ).svflags & 0x4 as libc::c_int != 0 || !client.is_null() {
            SpawnDamage(TE_BLOOD as libc::c_int, point, normal, take);
        } else {
            SpawnDamage(te_sparks, point, normal, take);
        }
        if CTFMatchSetup() as u64 == 0 {
            (*targ).health = (*targ).health - take;
        }
        if (*targ).health <= 0 as libc::c_int {
            if (*targ).svflags & 0x4 as libc::c_int != 0 || !client.is_null() {
                (*targ).flags |= 0x800 as libc::c_int;
            }
            Killed(targ, inflictor, attacker, take, point);
            return;
        }
    }
    if (*targ).svflags & 0x4 as libc::c_int != 0 {
        M_ReactToDamage(targ, attacker);
        if (*targ).monsterinfo.aiflags & 0x800 as libc::c_int == 0 && take != 0 {
            ((*targ).pain)
                .expect(
                    "non-null function pointer",
                )(targ, attacker, knockback as libc::c_float, take);
            if (*skill).value == 3 as libc::c_int as libc::c_float {
                (*targ)
                    .pain_debounce_time = level.time + 5 as libc::c_int as libc::c_float;
            }
        }
    } else if !client.is_null() {
        if (*targ).flags & 0x10 as libc::c_int == 0 && take != 0
            && CTFMatchSetup() as u64 == 0
        {
            ((*targ).pain)
                .expect(
                    "non-null function pointer",
                )(targ, attacker, knockback as libc::c_float, take);
        }
    } else if take != 0 {
        if ((*targ).pain).is_some() {
            ((*targ).pain)
                .expect(
                    "non-null function pointer",
                )(targ, attacker, knockback as libc::c_float, take);
        }
    }
    if !client.is_null() {
        (*client).damage_parmor += psave;
        (*client).damage_armor += asave;
        (*client).damage_blood += take;
        (*client).damage_knockback += knockback;
        (*client)
            .damage_from[0 as libc::c_int
            as usize] = *point.offset(0 as libc::c_int as isize);
        (*client)
            .damage_from[1 as libc::c_int
            as usize] = *point.offset(1 as libc::c_int as isize);
        (*client)
            .damage_from[2 as libc::c_int
            as usize] = *point.offset(2 as libc::c_int as isize);
    }
}
#[no_mangle]
pub unsafe extern "C" fn T_RadiusDamage(
    mut inflictor: *mut edict_t,
    mut attacker: *mut edict_t,
    mut damage: libc::c_float,
    mut ignore: *mut edict_t,
    mut radius: libc::c_float,
    mut mod_0: libc::c_int,
) {
    let mut points: libc::c_float = 0.;
    let mut ent: *mut edict_t = 0 as *mut edict_t;
    let mut v: vec3_t = [0.; 3];
    let mut dir: vec3_t = [0.; 3];
    loop {
        ent = findradius(ent, ((*inflictor).s.origin).as_mut_ptr(), radius);
        if ent.is_null() {
            break;
        }
        if ent == ignore {
            continue;
        }
        if (*ent).takedamage == 0 {
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
            as usize] = (*inflictor).s.origin[0 as libc::c_int as usize]
            - v[0 as libc::c_int as usize];
        v[1 as libc::c_int
            as usize] = (*inflictor).s.origin[1 as libc::c_int as usize]
            - v[1 as libc::c_int as usize];
        v[2 as libc::c_int
            as usize] = (*inflictor).s.origin[2 as libc::c_int as usize]
            - v[2 as libc::c_int as usize];
        points = (damage as libc::c_double
            - 0.5f64 * VectorLength(v.as_mut_ptr()) as libc::c_double) as libc::c_float;
        if ent == attacker {
            points = (points as libc::c_double * 0.5f64) as libc::c_float;
        }
        if points > 0 as libc::c_int as libc::c_float {
            if CanDamage(ent, inflictor) as u64 != 0 {
                dir[0 as libc::c_int
                    as usize] = (*ent).s.origin[0 as libc::c_int as usize]
                    - (*inflictor).s.origin[0 as libc::c_int as usize];
                dir[1 as libc::c_int
                    as usize] = (*ent).s.origin[1 as libc::c_int as usize]
                    - (*inflictor).s.origin[1 as libc::c_int as usize];
                dir[2 as libc::c_int
                    as usize] = (*ent).s.origin[2 as libc::c_int as usize]
                    - (*inflictor).s.origin[2 as libc::c_int as usize];
                T_Damage(
                    ent,
                    inflictor,
                    attacker,
                    dir.as_mut_ptr(),
                    ((*inflictor).s.origin).as_mut_ptr(),
                    vec3_origin.as_mut_ptr(),
                    points as libc::c_int,
                    points as libc::c_int,
                    0x1 as libc::c_int,
                    mod_0,
                );
            }
        }
    };
}
