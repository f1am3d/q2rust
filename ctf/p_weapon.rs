#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    fn cos(_: libc::c_double) -> libc::c_double;
    fn sin(_: libc::c_double) -> libc::c_double;
    fn rand() -> libc::c_int;
    static mut vec3_origin: vec3_t;
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
    static mut g_edicts: *mut edict_t;
    static mut deathmatch: *mut cvar_t;
    static mut coop: *mut cvar_t;
    static mut dmflags: *mut cvar_t;
    static mut instantweap: *mut cvar_t;
    static mut g_select_empty: *mut cvar_t;
    static mut is_quad: qboolean;
    static mut itemlist: [gitem_t; 0];
    fn FindItem(pickup_name: *mut libc::c_char) -> *mut gitem_t;
    fn Drop_Item(ent: *mut edict_t, item: *mut gitem_t) -> *mut edict_t;
    fn SetRespawn(ent: *mut edict_t, delay: libc::c_float);
    fn fire_grenade2(
        self_0: *mut edict_t,
        start: *mut vec_t,
        aimdir: *mut vec_t,
        damage: libc::c_int,
        speed: libc::c_int,
        timer: libc::c_float,
        damage_radius: libc::c_float,
        held: qboolean,
    );
    fn G_ProjectSource(
        point: *mut vec_t,
        distance: *mut vec_t,
        forward: *mut vec_t,
        right: *mut vec_t,
        result: *mut vec_t,
    );
    fn Add_Ammo(ent: *mut edict_t, item: *mut gitem_t, count: libc::c_int) -> qboolean;
    fn G_Spawn() -> *mut edict_t;
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
    fn fire_blaster(
        self_0: *mut edict_t,
        start: *mut vec_t,
        aimdir: *mut vec_t,
        damage: libc::c_int,
        speed: libc::c_int,
        effect: libc::c_int,
        hyper: qboolean,
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
    fn fire_rocket(
        self_0: *mut edict_t,
        start: *mut vec_t,
        dir: *mut vec_t,
        damage: libc::c_int,
        speed: libc::c_int,
        damage_radius: libc::c_float,
        radius_damage: libc::c_int,
    );
    fn fire_rail(
        self_0: *mut edict_t,
        start: *mut vec_t,
        aimdir: *mut vec_t,
        damage: libc::c_int,
        kick: libc::c_int,
    );
    fn fire_bfg(
        self_0: *mut edict_t,
        start: *mut vec_t,
        dir: *mut vec_t,
        damage: libc::c_int,
        speed: libc::c_int,
        damage_radius: libc::c_float,
    );
    fn CTFApplyHasteSound(ent: *mut edict_t);
    fn CTFApplyStrengthSound(ent: *mut edict_t) -> qboolean;
    fn CTFApplyHaste(ent: *mut edict_t) -> qboolean;
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
static mut is_silenced: byte = 0;
#[no_mangle]
pub unsafe extern "C" fn P_ProjectSource(
    mut client: *mut gclient_t,
    mut point: *mut vec_t,
    mut distance: *mut vec_t,
    mut forward: *mut vec_t,
    mut right: *mut vec_t,
    mut result: *mut vec_t,
) {
    let mut _distance: vec3_t = [0.; 3];
    _distance[0 as libc::c_int as usize] = *distance.offset(0 as libc::c_int as isize);
    _distance[1 as libc::c_int as usize] = *distance.offset(1 as libc::c_int as isize);
    _distance[2 as libc::c_int as usize] = *distance.offset(2 as libc::c_int as isize);
    if (*client).pers.hand == 1 as libc::c_int {
        _distance[1 as libc::c_int as usize] *= -(1 as libc::c_int) as libc::c_float;
    } else if (*client).pers.hand == 2 as libc::c_int {
        _distance[1 as libc::c_int as usize] = 0 as libc::c_int as vec_t;
    }
    G_ProjectSource(point, _distance.as_mut_ptr(), forward, right, result);
}
#[no_mangle]
pub unsafe extern "C" fn PlayerNoise(
    mut who: *mut edict_t,
    mut where_0: *mut vec_t,
    mut type_0: libc::c_int,
) {
    let mut noise: *mut edict_t = 0 as *mut edict_t;
    if type_0 == 1 as libc::c_int {
        if (*(*who).client).silencer_shots != 0 {
            let ref mut fresh0 = (*(*who).client).silencer_shots;
            *fresh0 -= 1;
            return;
        }
    }
    if (*deathmatch).value != 0. {
        return;
    }
    if (*who).flags & 0x20 as libc::c_int != 0 {
        return;
    }
    if ((*who).mynoise).is_null() {
        noise = G_Spawn();
        let ref mut fresh1 = (*noise).classname;
        *fresh1 = b"player_noise\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char;
        (*noise).mins[0 as libc::c_int as usize] = -(8 as libc::c_int) as vec_t;
        (*noise).mins[1 as libc::c_int as usize] = -(8 as libc::c_int) as vec_t;
        (*noise).mins[2 as libc::c_int as usize] = -(8 as libc::c_int) as vec_t;
        (*noise).maxs[0 as libc::c_int as usize] = 8 as libc::c_int as vec_t;
        (*noise).maxs[1 as libc::c_int as usize] = 8 as libc::c_int as vec_t;
        (*noise).maxs[2 as libc::c_int as usize] = 8 as libc::c_int as vec_t;
        let ref mut fresh2 = (*noise).owner;
        *fresh2 = who;
        (*noise).svflags = 0x1 as libc::c_int;
        let ref mut fresh3 = (*who).mynoise;
        *fresh3 = noise;
        noise = G_Spawn();
        let ref mut fresh4 = (*noise).classname;
        *fresh4 = b"player_noise\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char;
        (*noise).mins[0 as libc::c_int as usize] = -(8 as libc::c_int) as vec_t;
        (*noise).mins[1 as libc::c_int as usize] = -(8 as libc::c_int) as vec_t;
        (*noise).mins[2 as libc::c_int as usize] = -(8 as libc::c_int) as vec_t;
        (*noise).maxs[0 as libc::c_int as usize] = 8 as libc::c_int as vec_t;
        (*noise).maxs[1 as libc::c_int as usize] = 8 as libc::c_int as vec_t;
        (*noise).maxs[2 as libc::c_int as usize] = 8 as libc::c_int as vec_t;
        let ref mut fresh5 = (*noise).owner;
        *fresh5 = who;
        (*noise).svflags = 0x1 as libc::c_int;
        let ref mut fresh6 = (*who).mynoise2;
        *fresh6 = noise;
    }
    if type_0 == 0 as libc::c_int || type_0 == 1 as libc::c_int {
        noise = (*who).mynoise;
        level.sound_entity = noise;
        level.sound_entity_framenum = level.framenum;
    } else {
        noise = (*who).mynoise2;
        level.sound2_entity = noise;
        level.sound2_entity_framenum = level.framenum;
    }
    (*noise)
        .s
        .origin[0 as libc::c_int as usize] = *where_0.offset(0 as libc::c_int as isize);
    (*noise)
        .s
        .origin[1 as libc::c_int as usize] = *where_0.offset(1 as libc::c_int as isize);
    (*noise)
        .s
        .origin[2 as libc::c_int as usize] = *where_0.offset(2 as libc::c_int as isize);
    (*noise)
        .absmin[0 as libc::c_int
        as usize] = *where_0.offset(0 as libc::c_int as isize)
        - (*noise).maxs[0 as libc::c_int as usize];
    (*noise)
        .absmin[1 as libc::c_int
        as usize] = *where_0.offset(1 as libc::c_int as isize)
        - (*noise).maxs[1 as libc::c_int as usize];
    (*noise)
        .absmin[2 as libc::c_int
        as usize] = *where_0.offset(2 as libc::c_int as isize)
        - (*noise).maxs[2 as libc::c_int as usize];
    (*noise)
        .absmax[0 as libc::c_int
        as usize] = *where_0.offset(0 as libc::c_int as isize)
        + (*noise).maxs[0 as libc::c_int as usize];
    (*noise)
        .absmax[1 as libc::c_int
        as usize] = *where_0.offset(1 as libc::c_int as isize)
        + (*noise).maxs[1 as libc::c_int as usize];
    (*noise)
        .absmax[2 as libc::c_int
        as usize] = *where_0.offset(2 as libc::c_int as isize)
        + (*noise).maxs[2 as libc::c_int as usize];
    (*noise).teleport_time = level.time;
    (gi.linkentity).expect("non-null function pointer")(noise);
}
#[no_mangle]
pub unsafe extern "C" fn Pickup_Weapon(
    mut ent: *mut edict_t,
    mut other: *mut edict_t,
) -> qboolean {
    let mut index: libc::c_int = 0;
    let mut ammo: *mut gitem_t = 0 as *mut gitem_t;
    index = ((*ent).item).offset_from(itemlist.as_mut_ptr()) as libc::c_long
        as libc::c_int;
    if ((*dmflags).value as libc::c_int & 0x4 as libc::c_int != 0 || (*coop).value != 0.)
        && (*(*other).client).pers.inventory[index as usize] != 0
    {
        if (*ent).spawnflags & (0x10000 as libc::c_int | 0x20000 as libc::c_int) == 0 {
            return false_0;
        }
    }
    let ref mut fresh7 = (*(*other).client).pers.inventory[index as usize];
    *fresh7 += 1;
    if (*ent).spawnflags & 0x10000 as libc::c_int == 0 {
        ammo = FindItem((*(*ent).item).ammo);
        if (*dmflags).value as libc::c_int & 0x2000 as libc::c_int != 0 {
            Add_Ammo(other, ammo, 1000 as libc::c_int);
        } else {
            Add_Ammo(other, ammo, (*ammo).quantity);
        }
        if (*ent).spawnflags & 0x20000 as libc::c_int == 0 {
            if (*deathmatch).value != 0. {
                if (*dmflags).value as libc::c_int & 0x4 as libc::c_int != 0 {
                    let ref mut fresh8 = (*ent).flags;
                    *fresh8 = (*fresh8 as libc::c_uint | 0x80000000 as libc::c_uint)
                        as libc::c_int;
                } else {
                    SetRespawn(ent, 30 as libc::c_int as libc::c_float);
                }
            }
            if (*coop).value != 0. {
                let ref mut fresh9 = (*ent).flags;
                *fresh9 = (*fresh9 as libc::c_uint | 0x80000000 as libc::c_uint)
                    as libc::c_int;
            }
        }
    }
    if (*(*other).client).pers.weapon != (*ent).item
        && (*(*other).client).pers.inventory[index as usize] == 1 as libc::c_int
        && ((*deathmatch).value == 0.
            || (*(*other).client).pers.weapon
                == FindItem(
                    b"blaster\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                ))
    {
        let ref mut fresh10 = (*(*other).client).newweapon;
        *fresh10 = (*ent).item;
    }
    return true_0;
}
#[no_mangle]
pub unsafe extern "C" fn ChangeWeapon(mut ent: *mut edict_t) {
    let mut i: libc::c_int = 0;
    if (*(*ent).client).grenade_time != 0. {
        (*(*ent).client).grenade_time = level.time;
        (*(*ent).client).weapon_sound = 0 as libc::c_int;
        weapon_grenade_fire(ent, false_0);
        (*(*ent).client).grenade_time = 0 as libc::c_int as libc::c_float;
    }
    let ref mut fresh11 = (*(*ent).client).pers.lastweapon;
    *fresh11 = (*(*ent).client).pers.weapon;
    let ref mut fresh12 = (*(*ent).client).pers.weapon;
    *fresh12 = (*(*ent).client).newweapon;
    let ref mut fresh13 = (*(*ent).client).newweapon;
    *fresh13 = 0 as *mut gitem_t;
    (*(*ent).client).machinegun_shots = 0 as libc::c_int;
    if (*ent).s.modelindex == 255 as libc::c_int {
        if !((*(*ent).client).pers.weapon).is_null() {
            i = ((*(*(*ent).client).pers.weapon).weapmodel & 0xff as libc::c_int)
                << 8 as libc::c_int;
        } else {
            i = 0 as libc::c_int;
        }
        (*ent)
            .s
            .skinnum = (ent.offset_from(g_edicts) as libc::c_long
            - 1 as libc::c_int as libc::c_long | i as libc::c_long) as libc::c_int;
    }
    if !((*(*ent).client).pers.weapon).is_null()
        && !((*(*(*ent).client).pers.weapon).ammo).is_null()
    {
        (*(*ent).client)
            .ammo_index = (FindItem((*(*(*ent).client).pers.weapon).ammo))
            .offset_from(itemlist.as_mut_ptr()) as libc::c_long as libc::c_int;
    } else {
        (*(*ent).client).ammo_index = 0 as libc::c_int;
    }
    if ((*(*ent).client).pers.weapon).is_null() {
        (*(*ent).client).ps.gunindex = 0 as libc::c_int;
        return;
    }
    (*(*ent).client).weaponstate = WEAPON_ACTIVATING;
    (*(*ent).client).ps.gunframe = 0 as libc::c_int;
    (*(*ent).client)
        .ps
        .gunindex = (gi.modelindex)
        .expect("non-null function pointer")((*(*(*ent).client).pers.weapon).view_model);
    (*(*ent).client).anim_priority = 3 as libc::c_int;
    if (*(*ent).client).ps.pmove.pm_flags as libc::c_int & 1 as libc::c_int != 0 {
        (*ent).s.frame = 169 as libc::c_int;
        (*(*ent).client).anim_end = 172 as libc::c_int;
    } else {
        (*ent).s.frame = 62 as libc::c_int;
        (*(*ent).client).anim_end = 65 as libc::c_int;
    };
}
#[no_mangle]
pub unsafe extern "C" fn NoAmmoWeaponChange(mut ent: *mut edict_t) {
    if (*(*ent).client)
        .pers
        .inventory[(FindItem(
        b"slugs\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ))
        .offset_from(itemlist.as_mut_ptr()) as libc::c_long as usize] != 0
        && (*(*ent).client)
            .pers
            .inventory[(FindItem(
            b"railgun\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ))
            .offset_from(itemlist.as_mut_ptr()) as libc::c_long as usize] != 0
    {
        let ref mut fresh14 = (*(*ent).client).newweapon;
        *fresh14 = FindItem(
            b"railgun\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        return;
    }
    if (*(*ent).client)
        .pers
        .inventory[(FindItem(
        b"cells\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ))
        .offset_from(itemlist.as_mut_ptr()) as libc::c_long as usize] != 0
        && (*(*ent).client)
            .pers
            .inventory[(FindItem(
            b"hyperblaster\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ))
            .offset_from(itemlist.as_mut_ptr()) as libc::c_long as usize] != 0
    {
        let ref mut fresh15 = (*(*ent).client).newweapon;
        *fresh15 = FindItem(
            b"hyperblaster\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        return;
    }
    if (*(*ent).client)
        .pers
        .inventory[(FindItem(
        b"bullets\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ))
        .offset_from(itemlist.as_mut_ptr()) as libc::c_long as usize] != 0
        && (*(*ent).client)
            .pers
            .inventory[(FindItem(
            b"chaingun\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ))
            .offset_from(itemlist.as_mut_ptr()) as libc::c_long as usize] != 0
    {
        let ref mut fresh16 = (*(*ent).client).newweapon;
        *fresh16 = FindItem(
            b"chaingun\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        return;
    }
    if (*(*ent).client)
        .pers
        .inventory[(FindItem(
        b"bullets\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ))
        .offset_from(itemlist.as_mut_ptr()) as libc::c_long as usize] != 0
        && (*(*ent).client)
            .pers
            .inventory[(FindItem(
            b"machinegun\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ))
            .offset_from(itemlist.as_mut_ptr()) as libc::c_long as usize] != 0
    {
        let ref mut fresh17 = (*(*ent).client).newweapon;
        *fresh17 = FindItem(
            b"machinegun\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        return;
    }
    if (*(*ent).client)
        .pers
        .inventory[(FindItem(
        b"shells\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ))
        .offset_from(itemlist.as_mut_ptr()) as libc::c_long as usize] > 1 as libc::c_int
        && (*(*ent).client)
            .pers
            .inventory[(FindItem(
            b"super shotgun\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ))
            .offset_from(itemlist.as_mut_ptr()) as libc::c_long as usize] != 0
    {
        let ref mut fresh18 = (*(*ent).client).newweapon;
        *fresh18 = FindItem(
            b"super shotgun\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        return;
    }
    if (*(*ent).client)
        .pers
        .inventory[(FindItem(
        b"shells\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ))
        .offset_from(itemlist.as_mut_ptr()) as libc::c_long as usize] != 0
        && (*(*ent).client)
            .pers
            .inventory[(FindItem(
            b"shotgun\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ))
            .offset_from(itemlist.as_mut_ptr()) as libc::c_long as usize] != 0
    {
        let ref mut fresh19 = (*(*ent).client).newweapon;
        *fresh19 = FindItem(
            b"shotgun\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        return;
    }
    let ref mut fresh20 = (*(*ent).client).newweapon;
    *fresh20 = FindItem(
        b"blaster\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
}
#[no_mangle]
pub unsafe extern "C" fn Think_Weapon(mut ent: *mut edict_t) {
    if (*ent).health < 1 as libc::c_int {
        let ref mut fresh21 = (*(*ent).client).newweapon;
        *fresh21 = 0 as *mut gitem_t;
        ChangeWeapon(ent);
    }
    if !((*(*ent).client).pers.weapon).is_null()
        && ((*(*(*ent).client).pers.weapon).weaponthink).is_some()
    {
        is_quad = ((*(*ent).client).quad_framenum > level.framenum as libc::c_float)
            as libc::c_int as qboolean;
        if (*(*ent).client).silencer_shots != 0 {
            is_silenced = 128 as libc::c_int as byte;
        } else {
            is_silenced = 0 as libc::c_int as byte;
        }
        ((*(*(*ent).client).pers.weapon).weaponthink)
            .expect("non-null function pointer")(ent);
    }
}
#[no_mangle]
pub unsafe extern "C" fn Use_Weapon(mut ent: *mut edict_t, mut item: *mut gitem_t) {
    let mut ammo_index: libc::c_int = 0;
    let mut ammo_item: *mut gitem_t = 0 as *mut gitem_t;
    if item == (*(*ent).client).pers.weapon {
        return;
    }
    if !((*item).ammo).is_null() && (*g_select_empty).value == 0.
        && (*item).flags & 2 as libc::c_int == 0
    {
        ammo_item = FindItem((*item).ammo);
        ammo_index = ammo_item.offset_from(itemlist.as_mut_ptr()) as libc::c_long
            as libc::c_int;
        if (*(*ent).client).pers.inventory[ammo_index as usize] == 0 {
            (gi.cprintf)
                .expect(
                    "non-null function pointer",
                )(
                ent,
                2 as libc::c_int,
                b"No %s for %s.\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                (*ammo_item).pickup_name,
                (*item).pickup_name,
            );
            return;
        }
        if (*(*ent).client).pers.inventory[ammo_index as usize] < (*item).quantity {
            (gi.cprintf)
                .expect(
                    "non-null function pointer",
                )(
                ent,
                2 as libc::c_int,
                b"Not enough %s for %s.\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                (*ammo_item).pickup_name,
                (*item).pickup_name,
            );
            return;
        }
    }
    let ref mut fresh22 = (*(*ent).client).newweapon;
    *fresh22 = item;
}
#[no_mangle]
pub unsafe extern "C" fn Drop_Weapon(mut ent: *mut edict_t, mut item: *mut gitem_t) {
    let mut index: libc::c_int = 0;
    if (*dmflags).value as libc::c_int & 0x4 as libc::c_int != 0 {
        return;
    }
    index = item.offset_from(itemlist.as_mut_ptr()) as libc::c_long as libc::c_int;
    if (item == (*(*ent).client).pers.weapon || item == (*(*ent).client).newweapon)
        && (*(*ent).client).pers.inventory[index as usize] == 1 as libc::c_int
    {
        (gi.cprintf)
            .expect(
                "non-null function pointer",
            )(
            ent,
            2 as libc::c_int,
            b"Can't drop current weapon\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return;
    }
    Drop_Item(ent, item);
    let ref mut fresh23 = (*(*ent).client).pers.inventory[index as usize];
    *fresh23 -= 1;
}
unsafe extern "C" fn Weapon_Generic2(
    mut ent: *mut edict_t,
    mut FRAME_ACTIVATE_LAST: libc::c_int,
    mut FRAME_FIRE_LAST: libc::c_int,
    mut FRAME_IDLE_LAST: libc::c_int,
    mut FRAME_DEACTIVATE_LAST: libc::c_int,
    mut pause_frames: *mut libc::c_int,
    mut fire_frames: *mut libc::c_int,
    mut fire: Option::<unsafe extern "C" fn(*mut edict_t) -> ()>,
) {
    let mut n: libc::c_int = 0;
    if (*ent).deadflag != 0 || (*ent).s.modelindex != 255 as libc::c_int {
        return;
    }
    if (*(*ent).client).weaponstate as libc::c_uint
        == WEAPON_DROPPING as libc::c_int as libc::c_uint
    {
        if (*(*ent).client).ps.gunframe == FRAME_DEACTIVATE_LAST {
            ChangeWeapon(ent);
            return;
        } else {
            if FRAME_DEACTIVATE_LAST - (*(*ent).client).ps.gunframe == 4 as libc::c_int {
                (*(*ent).client).anim_priority = 6 as libc::c_int;
                if (*(*ent).client).ps.pmove.pm_flags as libc::c_int & 1 as libc::c_int
                    != 0
                {
                    (*ent).s.frame = 172 as libc::c_int + 1 as libc::c_int;
                    (*(*ent).client).anim_end = 169 as libc::c_int;
                } else {
                    (*ent).s.frame = 65 as libc::c_int + 1 as libc::c_int;
                    (*(*ent).client).anim_end = 62 as libc::c_int;
                }
            }
        }
        let ref mut fresh24 = (*(*ent).client).ps.gunframe;
        *fresh24 += 1;
        return;
    }
    if (*(*ent).client).weaponstate as libc::c_uint
        == WEAPON_ACTIVATING as libc::c_int as libc::c_uint
    {
        if (*(*ent).client).ps.gunframe == FRAME_ACTIVATE_LAST
            || (*instantweap).value != 0.
        {
            (*(*ent).client).weaponstate = WEAPON_READY;
            (*(*ent).client).ps.gunframe = FRAME_FIRE_LAST + 1 as libc::c_int;
            Weapon_Generic2(
                ent,
                FRAME_ACTIVATE_LAST,
                FRAME_FIRE_LAST,
                FRAME_IDLE_LAST,
                FRAME_DEACTIVATE_LAST,
                pause_frames,
                fire_frames,
                fire,
            );
            return;
        }
        let ref mut fresh25 = (*(*ent).client).ps.gunframe;
        *fresh25 += 1;
        return;
    }
    if !((*(*ent).client).newweapon).is_null()
        && (*(*ent).client).weaponstate as libc::c_uint
            != WEAPON_FIRING as libc::c_int as libc::c_uint
    {
        (*(*ent).client).weaponstate = WEAPON_DROPPING;
        if (*instantweap).value != 0. {
            ChangeWeapon(ent);
            return;
        } else {
            (*(*ent).client).ps.gunframe = FRAME_IDLE_LAST + 1 as libc::c_int;
        }
        if FRAME_DEACTIVATE_LAST - (FRAME_IDLE_LAST + 1 as libc::c_int)
            < 4 as libc::c_int
        {
            (*(*ent).client).anim_priority = 6 as libc::c_int;
            if (*(*ent).client).ps.pmove.pm_flags as libc::c_int & 1 as libc::c_int != 0
            {
                (*ent).s.frame = 172 as libc::c_int + 1 as libc::c_int;
                (*(*ent).client).anim_end = 169 as libc::c_int;
            } else {
                (*ent).s.frame = 65 as libc::c_int + 1 as libc::c_int;
                (*(*ent).client).anim_end = 62 as libc::c_int;
            }
        }
        return;
    }
    if (*(*ent).client).weaponstate as libc::c_uint
        == WEAPON_READY as libc::c_int as libc::c_uint
    {
        if ((*(*ent).client).latched_buttons | (*(*ent).client).buttons)
            & 1 as libc::c_int != 0
        {
            (*(*ent).client).latched_buttons &= !(1 as libc::c_int);
            if (*(*ent).client).ammo_index == 0
                || (*(*ent).client).pers.inventory[(*(*ent).client).ammo_index as usize]
                    >= (*(*(*ent).client).pers.weapon).quantity
            {
                (*(*ent).client).ps.gunframe = FRAME_ACTIVATE_LAST + 1 as libc::c_int;
                (*(*ent).client).weaponstate = WEAPON_FIRING;
                (*(*ent).client).anim_priority = 4 as libc::c_int;
                if (*(*ent).client).ps.pmove.pm_flags as libc::c_int & 1 as libc::c_int
                    != 0
                {
                    (*ent).s.frame = 160 as libc::c_int - 1 as libc::c_int;
                    (*(*ent).client).anim_end = 168 as libc::c_int;
                } else {
                    (*ent).s.frame = 46 as libc::c_int - 1 as libc::c_int;
                    (*(*ent).client).anim_end = 53 as libc::c_int;
                }
            } else {
                if level.time >= (*ent).pain_debounce_time {
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
                            b"weapons/noammo.wav\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char,
                        ),
                        1 as libc::c_int as libc::c_float,
                        1 as libc::c_int as libc::c_float,
                        0 as libc::c_int as libc::c_float,
                    );
                    (*ent)
                        .pain_debounce_time = level.time
                        + 1 as libc::c_int as libc::c_float;
                }
                NoAmmoWeaponChange(ent);
            }
        } else {
            if (*(*ent).client).ps.gunframe == FRAME_IDLE_LAST {
                (*(*ent).client).ps.gunframe = FRAME_FIRE_LAST + 1 as libc::c_int;
                return;
            }
            if !pause_frames.is_null() {
                n = 0 as libc::c_int;
                while *pause_frames.offset(n as isize) != 0 {
                    if (*(*ent).client).ps.gunframe == *pause_frames.offset(n as isize) {
                        if rand() & 15 as libc::c_int != 0 {
                            return;
                        }
                    }
                    n += 1;
                }
            }
            let ref mut fresh26 = (*(*ent).client).ps.gunframe;
            *fresh26 += 1;
            return;
        }
    }
    if (*(*ent).client).weaponstate as libc::c_uint
        == WEAPON_FIRING as libc::c_int as libc::c_uint
    {
        n = 0 as libc::c_int;
        while *fire_frames.offset(n as isize) != 0 {
            if (*(*ent).client).ps.gunframe == *fire_frames.offset(n as isize) {
                if CTFApplyStrengthSound(ent) as u64 == 0 {
                    if (*(*ent).client).quad_framenum > level.framenum as libc::c_float {
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
                                b"items/damage3.wav\0" as *const u8 as *const libc::c_char
                                    as *mut libc::c_char,
                            ),
                            1 as libc::c_int as libc::c_float,
                            1 as libc::c_int as libc::c_float,
                            0 as libc::c_int as libc::c_float,
                        );
                    }
                }
                CTFApplyHasteSound(ent);
                fire.expect("non-null function pointer")(ent);
                break;
            } else {
                n += 1;
            }
        }
        if *fire_frames.offset(n as isize) == 0 {
            let ref mut fresh27 = (*(*ent).client).ps.gunframe;
            *fresh27 += 1;
        }
        if (*(*ent).client).ps.gunframe
            == FRAME_FIRE_LAST + 1 as libc::c_int + 1 as libc::c_int
        {
            (*(*ent).client).weaponstate = WEAPON_READY;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn Weapon_Generic(
    mut ent: *mut edict_t,
    mut FRAME_ACTIVATE_LAST: libc::c_int,
    mut FRAME_FIRE_LAST: libc::c_int,
    mut FRAME_IDLE_LAST: libc::c_int,
    mut FRAME_DEACTIVATE_LAST: libc::c_int,
    mut pause_frames: *mut libc::c_int,
    mut fire_frames: *mut libc::c_int,
    mut fire: Option::<unsafe extern "C" fn(*mut edict_t) -> ()>,
) {
    let mut oldstate: libc::c_int = (*(*ent).client).weaponstate as libc::c_int;
    Weapon_Generic2(
        ent,
        FRAME_ACTIVATE_LAST,
        FRAME_FIRE_LAST,
        FRAME_IDLE_LAST,
        FRAME_DEACTIVATE_LAST,
        pause_frames,
        fire_frames,
        fire,
    );
    if stricmp(
        (*(*(*ent).client).pers.weapon).pickup_name,
        b"Grapple\0" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int
        && (*(*ent).client).weaponstate as libc::c_uint
            == WEAPON_FIRING as libc::c_int as libc::c_uint
    {
        return;
    }
    if (CTFApplyHaste(ent) as libc::c_uint != 0
        || Q_stricmp(
            (*(*(*ent).client).pers.weapon).pickup_name,
            b"Grapple\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) == 0 as libc::c_int
            && (*(*ent).client).weaponstate as libc::c_uint
                != WEAPON_FIRING as libc::c_int as libc::c_uint)
        && oldstate as libc::c_uint == (*(*ent).client).weaponstate as libc::c_uint
    {
        Weapon_Generic2(
            ent,
            FRAME_ACTIVATE_LAST,
            FRAME_FIRE_LAST,
            FRAME_IDLE_LAST,
            FRAME_DEACTIVATE_LAST,
            pause_frames,
            fire_frames,
            fire,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn weapon_grenade_fire(mut ent: *mut edict_t, mut held: qboolean) {
    let mut offset: vec3_t = [0.; 3];
    let mut forward: vec3_t = [0.; 3];
    let mut right: vec3_t = [0.; 3];
    let mut start: vec3_t = [0.; 3];
    let mut damage: libc::c_int = 125 as libc::c_int;
    let mut timer: libc::c_float = 0.;
    let mut speed: libc::c_int = 0;
    let mut radius: libc::c_float = 0.;
    radius = (damage + 40 as libc::c_int) as libc::c_float;
    if is_quad as u64 != 0 {
        damage *= 4 as libc::c_int;
    }
    offset[0 as libc::c_int as usize] = 8 as libc::c_int as vec_t;
    offset[1 as libc::c_int as usize] = 8 as libc::c_int as vec_t;
    offset[2 as libc::c_int as usize] = ((*ent).viewheight - 8 as libc::c_int) as vec_t;
    AngleVectors(
        ((*(*ent).client).v_angle).as_mut_ptr(),
        forward.as_mut_ptr(),
        right.as_mut_ptr(),
        0 as *mut vec_t,
    );
    P_ProjectSource(
        (*ent).client,
        ((*ent).s.origin).as_mut_ptr(),
        offset.as_mut_ptr(),
        forward.as_mut_ptr(),
        right.as_mut_ptr(),
        start.as_mut_ptr(),
    );
    timer = (*(*ent).client).grenade_time - level.time;
    speed = (400 as libc::c_int as libc::c_double
        + (3.0f64 - timer as libc::c_double)
            * ((800 as libc::c_int - 400 as libc::c_int) as libc::c_double / 3.0f64))
        as libc::c_int;
    fire_grenade2(
        ent,
        start.as_mut_ptr(),
        forward.as_mut_ptr(),
        damage,
        speed,
        timer,
        radius,
        held,
    );
    if (*dmflags).value as libc::c_int & 0x2000 as libc::c_int == 0 {
        let ref mut fresh28 = (*(*ent).client)
            .pers
            .inventory[(*(*ent).client).ammo_index as usize];
        *fresh28 -= 1;
    }
    (*(*ent).client)
        .grenade_time = (level.time as libc::c_double + 1.0f64) as libc::c_float;
    if (*ent).deadflag != 0 || (*ent).s.modelindex != 255 as libc::c_int {
        return;
    }
    if (*(*ent).client).ps.pmove.pm_flags as libc::c_int & 1 as libc::c_int != 0 {
        (*(*ent).client).anim_priority = 4 as libc::c_int;
        (*ent).s.frame = 160 as libc::c_int - 1 as libc::c_int;
        (*(*ent).client).anim_end = 162 as libc::c_int;
    } else {
        (*(*ent).client).anim_priority = 6 as libc::c_int;
        (*ent).s.frame = 119 as libc::c_int;
        (*(*ent).client).anim_end = 112 as libc::c_int;
    };
}
#[no_mangle]
pub unsafe extern "C" fn Weapon_Grenade(mut ent: *mut edict_t) {
    if !((*(*ent).client).newweapon).is_null()
        && (*(*ent).client).weaponstate as libc::c_uint
            == WEAPON_READY as libc::c_int as libc::c_uint
    {
        ChangeWeapon(ent);
        return;
    }
    if (*(*ent).client).weaponstate as libc::c_uint
        == WEAPON_ACTIVATING as libc::c_int as libc::c_uint
    {
        (*(*ent).client).weaponstate = WEAPON_READY;
        (*(*ent).client).ps.gunframe = 16 as libc::c_int;
        return;
    }
    if (*(*ent).client).weaponstate as libc::c_uint
        == WEAPON_READY as libc::c_int as libc::c_uint
    {
        if ((*(*ent).client).latched_buttons | (*(*ent).client).buttons)
            & 1 as libc::c_int != 0
        {
            (*(*ent).client).latched_buttons &= !(1 as libc::c_int);
            if (*(*ent).client).pers.inventory[(*(*ent).client).ammo_index as usize] != 0
            {
                (*(*ent).client).ps.gunframe = 1 as libc::c_int;
                (*(*ent).client).weaponstate = WEAPON_FIRING;
                (*(*ent).client).grenade_time = 0 as libc::c_int as libc::c_float;
            } else {
                if level.time >= (*ent).pain_debounce_time {
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
                            b"weapons/noammo.wav\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char,
                        ),
                        1 as libc::c_int as libc::c_float,
                        1 as libc::c_int as libc::c_float,
                        0 as libc::c_int as libc::c_float,
                    );
                    (*ent)
                        .pain_debounce_time = level.time
                        + 1 as libc::c_int as libc::c_float;
                }
                NoAmmoWeaponChange(ent);
            }
            return;
        }
        if (*(*ent).client).ps.gunframe == 29 as libc::c_int
            || (*(*ent).client).ps.gunframe == 34 as libc::c_int
            || (*(*ent).client).ps.gunframe == 39 as libc::c_int
            || (*(*ent).client).ps.gunframe == 48 as libc::c_int
        {
            if rand() & 15 as libc::c_int != 0 {
                return;
            }
        }
        let ref mut fresh29 = (*(*ent).client).ps.gunframe;
        *fresh29 += 1;
        if *fresh29 > 48 as libc::c_int {
            (*(*ent).client).ps.gunframe = 16 as libc::c_int;
        }
        return;
    }
    if (*(*ent).client).weaponstate as libc::c_uint
        == WEAPON_FIRING as libc::c_int as libc::c_uint
    {
        if (*(*ent).client).ps.gunframe == 5 as libc::c_int {
            (gi.sound)
                .expect(
                    "non-null function pointer",
                )(
                ent,
                1 as libc::c_int,
                (gi.soundindex)
                    .expect(
                        "non-null function pointer",
                    )(
                    b"weapons/hgrena1b.wav\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                ),
                1 as libc::c_int as libc::c_float,
                1 as libc::c_int as libc::c_float,
                0 as libc::c_int as libc::c_float,
            );
        }
        if (*(*ent).client).ps.gunframe == 11 as libc::c_int {
            if (*(*ent).client).grenade_time == 0. {
                (*(*ent).client)
                    .grenade_time = (level.time as libc::c_double + 3.0f64 + 0.2f64)
                    as libc::c_float;
                (*(*ent).client)
                    .weapon_sound = (gi.soundindex)
                    .expect(
                        "non-null function pointer",
                    )(
                    b"weapons/hgrenc1b.wav\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                );
            }
            if (*(*ent).client).grenade_blew_up as u64 == 0
                && level.time >= (*(*ent).client).grenade_time
            {
                (*(*ent).client).weapon_sound = 0 as libc::c_int;
                weapon_grenade_fire(ent, true_0);
                (*(*ent).client).grenade_blew_up = true_0;
            }
            if (*(*ent).client).buttons & 1 as libc::c_int != 0 {
                return;
            }
            if (*(*ent).client).grenade_blew_up as u64 != 0 {
                if level.time >= (*(*ent).client).grenade_time {
                    (*(*ent).client).ps.gunframe = 15 as libc::c_int;
                    (*(*ent).client).grenade_blew_up = false_0;
                } else {
                    return
                }
            }
        }
        if (*(*ent).client).ps.gunframe == 12 as libc::c_int {
            (*(*ent).client).weapon_sound = 0 as libc::c_int;
            weapon_grenade_fire(ent, false_0);
        }
        if (*(*ent).client).ps.gunframe == 15 as libc::c_int
            && level.time < (*(*ent).client).grenade_time
        {
            return;
        }
        let ref mut fresh30 = (*(*ent).client).ps.gunframe;
        *fresh30 += 1;
        if (*(*ent).client).ps.gunframe == 16 as libc::c_int {
            (*(*ent).client).grenade_time = 0 as libc::c_int as libc::c_float;
            (*(*ent).client).weaponstate = WEAPON_READY;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn weapon_grenadelauncher_fire(mut ent: *mut edict_t) {
    let mut offset: vec3_t = [0.; 3];
    let mut forward: vec3_t = [0.; 3];
    let mut right: vec3_t = [0.; 3];
    let mut start: vec3_t = [0.; 3];
    let mut damage: libc::c_int = 120 as libc::c_int;
    let mut radius: libc::c_float = 0.;
    radius = (damage + 40 as libc::c_int) as libc::c_float;
    if is_quad as u64 != 0 {
        damage *= 4 as libc::c_int;
    }
    offset[0 as libc::c_int as usize] = 8 as libc::c_int as vec_t;
    offset[1 as libc::c_int as usize] = 8 as libc::c_int as vec_t;
    offset[2 as libc::c_int as usize] = ((*ent).viewheight - 8 as libc::c_int) as vec_t;
    AngleVectors(
        ((*(*ent).client).v_angle).as_mut_ptr(),
        forward.as_mut_ptr(),
        right.as_mut_ptr(),
        0 as *mut vec_t,
    );
    P_ProjectSource(
        (*ent).client,
        ((*ent).s.origin).as_mut_ptr(),
        offset.as_mut_ptr(),
        forward.as_mut_ptr(),
        right.as_mut_ptr(),
        start.as_mut_ptr(),
    );
    VectorScale(
        forward.as_mut_ptr(),
        -(2 as libc::c_int) as vec_t,
        ((*(*ent).client).kick_origin).as_mut_ptr(),
    );
    (*(*ent).client)
        .kick_angles[0 as libc::c_int as usize] = -(1 as libc::c_int) as vec_t;
    fire_grenade(
        ent,
        start.as_mut_ptr(),
        forward.as_mut_ptr(),
        damage,
        600 as libc::c_int,
        2.5f64 as libc::c_float,
        radius,
    );
    (gi.WriteByte).expect("non-null function pointer")(1 as libc::c_int);
    (gi.WriteShort)
        .expect(
            "non-null function pointer",
        )(ent.offset_from(g_edicts) as libc::c_long as libc::c_int);
    (gi.WriteByte)
        .expect(
            "non-null function pointer",
        )(8 as libc::c_int | is_silenced as libc::c_int);
    (gi.multicast)
        .expect(
            "non-null function pointer",
        )(((*ent).s.origin).as_mut_ptr(), MULTICAST_PVS);
    let ref mut fresh31 = (*(*ent).client).ps.gunframe;
    *fresh31 += 1;
    PlayerNoise(ent, start.as_mut_ptr(), 1 as libc::c_int);
    if (*dmflags).value as libc::c_int & 0x2000 as libc::c_int == 0 {
        let ref mut fresh32 = (*(*ent).client)
            .pers
            .inventory[(*(*ent).client).ammo_index as usize];
        *fresh32 -= 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn Weapon_GrenadeLauncher(mut ent: *mut edict_t) {
    static mut pause_frames: [libc::c_int; 4] = [
        34 as libc::c_int,
        51 as libc::c_int,
        59 as libc::c_int,
        0 as libc::c_int,
    ];
    static mut fire_frames: [libc::c_int; 2] = [6 as libc::c_int, 0 as libc::c_int];
    Weapon_Generic(
        ent,
        5 as libc::c_int,
        16 as libc::c_int,
        59 as libc::c_int,
        64 as libc::c_int,
        pause_frames.as_mut_ptr(),
        fire_frames.as_mut_ptr(),
        Some(weapon_grenadelauncher_fire as unsafe extern "C" fn(*mut edict_t) -> ()),
    );
}
#[no_mangle]
pub unsafe extern "C" fn Weapon_RocketLauncher_Fire(mut ent: *mut edict_t) {
    let mut offset: vec3_t = [0.; 3];
    let mut start: vec3_t = [0.; 3];
    let mut forward: vec3_t = [0.; 3];
    let mut right: vec3_t = [0.; 3];
    let mut damage: libc::c_int = 0;
    let mut damage_radius: libc::c_float = 0.;
    let mut radius_damage: libc::c_int = 0;
    damage = 100 as libc::c_int
        + (((rand() & 0x7fff as libc::c_int) as libc::c_float
            / 0x7fff as libc::c_int as libc::c_float) as libc::c_double * 20.0f64)
            as libc::c_int;
    radius_damage = 120 as libc::c_int;
    damage_radius = 120 as libc::c_int as libc::c_float;
    if is_quad as u64 != 0 {
        damage *= 4 as libc::c_int;
        radius_damage *= 4 as libc::c_int;
    }
    AngleVectors(
        ((*(*ent).client).v_angle).as_mut_ptr(),
        forward.as_mut_ptr(),
        right.as_mut_ptr(),
        0 as *mut vec_t,
    );
    VectorScale(
        forward.as_mut_ptr(),
        -(2 as libc::c_int) as vec_t,
        ((*(*ent).client).kick_origin).as_mut_ptr(),
    );
    (*(*ent).client)
        .kick_angles[0 as libc::c_int as usize] = -(1 as libc::c_int) as vec_t;
    offset[0 as libc::c_int as usize] = 8 as libc::c_int as vec_t;
    offset[1 as libc::c_int as usize] = 8 as libc::c_int as vec_t;
    offset[2 as libc::c_int as usize] = ((*ent).viewheight - 8 as libc::c_int) as vec_t;
    P_ProjectSource(
        (*ent).client,
        ((*ent).s.origin).as_mut_ptr(),
        offset.as_mut_ptr(),
        forward.as_mut_ptr(),
        right.as_mut_ptr(),
        start.as_mut_ptr(),
    );
    fire_rocket(
        ent,
        start.as_mut_ptr(),
        forward.as_mut_ptr(),
        damage,
        650 as libc::c_int,
        damage_radius,
        radius_damage,
    );
    (gi.WriteByte).expect("non-null function pointer")(1 as libc::c_int);
    (gi.WriteShort)
        .expect(
            "non-null function pointer",
        )(ent.offset_from(g_edicts) as libc::c_long as libc::c_int);
    (gi.WriteByte)
        .expect(
            "non-null function pointer",
        )(7 as libc::c_int | is_silenced as libc::c_int);
    (gi.multicast)
        .expect(
            "non-null function pointer",
        )(((*ent).s.origin).as_mut_ptr(), MULTICAST_PVS);
    let ref mut fresh33 = (*(*ent).client).ps.gunframe;
    *fresh33 += 1;
    PlayerNoise(ent, start.as_mut_ptr(), 1 as libc::c_int);
    if (*dmflags).value as libc::c_int & 0x2000 as libc::c_int == 0 {
        let ref mut fresh34 = (*(*ent).client)
            .pers
            .inventory[(*(*ent).client).ammo_index as usize];
        *fresh34 -= 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn Weapon_RocketLauncher(mut ent: *mut edict_t) {
    static mut pause_frames: [libc::c_int; 5] = [
        25 as libc::c_int,
        33 as libc::c_int,
        42 as libc::c_int,
        50 as libc::c_int,
        0 as libc::c_int,
    ];
    static mut fire_frames: [libc::c_int; 2] = [5 as libc::c_int, 0 as libc::c_int];
    Weapon_Generic(
        ent,
        4 as libc::c_int,
        12 as libc::c_int,
        50 as libc::c_int,
        54 as libc::c_int,
        pause_frames.as_mut_ptr(),
        fire_frames.as_mut_ptr(),
        Some(Weapon_RocketLauncher_Fire as unsafe extern "C" fn(*mut edict_t) -> ()),
    );
}
#[no_mangle]
pub unsafe extern "C" fn Blaster_Fire(
    mut ent: *mut edict_t,
    mut g_offset: *mut vec_t,
    mut damage: libc::c_int,
    mut hyper: qboolean,
    mut effect: libc::c_int,
) {
    let mut forward: vec3_t = [0.; 3];
    let mut right: vec3_t = [0.; 3];
    let mut start: vec3_t = [0.; 3];
    let mut offset: vec3_t = [0.; 3];
    if is_quad as u64 != 0 {
        damage *= 4 as libc::c_int;
    }
    AngleVectors(
        ((*(*ent).client).v_angle).as_mut_ptr(),
        forward.as_mut_ptr(),
        right.as_mut_ptr(),
        0 as *mut vec_t,
    );
    offset[0 as libc::c_int as usize] = 24 as libc::c_int as vec_t;
    offset[1 as libc::c_int as usize] = 8 as libc::c_int as vec_t;
    offset[2 as libc::c_int as usize] = ((*ent).viewheight - 8 as libc::c_int) as vec_t;
    offset[0 as libc::c_int
        as usize] = offset[0 as libc::c_int as usize]
        + *g_offset.offset(0 as libc::c_int as isize);
    offset[1 as libc::c_int
        as usize] = offset[1 as libc::c_int as usize]
        + *g_offset.offset(1 as libc::c_int as isize);
    offset[2 as libc::c_int
        as usize] = offset[2 as libc::c_int as usize]
        + *g_offset.offset(2 as libc::c_int as isize);
    P_ProjectSource(
        (*ent).client,
        ((*ent).s.origin).as_mut_ptr(),
        offset.as_mut_ptr(),
        forward.as_mut_ptr(),
        right.as_mut_ptr(),
        start.as_mut_ptr(),
    );
    VectorScale(
        forward.as_mut_ptr(),
        -(2 as libc::c_int) as vec_t,
        ((*(*ent).client).kick_origin).as_mut_ptr(),
    );
    (*(*ent).client)
        .kick_angles[0 as libc::c_int as usize] = -(1 as libc::c_int) as vec_t;
    fire_blaster(
        ent,
        start.as_mut_ptr(),
        forward.as_mut_ptr(),
        damage,
        1000 as libc::c_int,
        effect,
        hyper,
    );
    (gi.WriteByte).expect("non-null function pointer")(1 as libc::c_int);
    (gi.WriteShort)
        .expect(
            "non-null function pointer",
        )(ent.offset_from(g_edicts) as libc::c_long as libc::c_int);
    if hyper as u64 != 0 {
        (gi.WriteByte)
            .expect(
                "non-null function pointer",
            )(14 as libc::c_int | is_silenced as libc::c_int);
    } else {
        (gi.WriteByte)
            .expect(
                "non-null function pointer",
            )(0 as libc::c_int | is_silenced as libc::c_int);
    }
    (gi.multicast)
        .expect(
            "non-null function pointer",
        )(((*ent).s.origin).as_mut_ptr(), MULTICAST_PVS);
    PlayerNoise(ent, start.as_mut_ptr(), 1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn Weapon_Blaster_Fire(mut ent: *mut edict_t) {
    let mut damage: libc::c_int = 0;
    if (*deathmatch).value != 0. {
        damage = 15 as libc::c_int;
    } else {
        damage = 10 as libc::c_int;
    }
    Blaster_Fire(ent, vec3_origin.as_mut_ptr(), damage, false_0, 0x8 as libc::c_int);
    let ref mut fresh35 = (*(*ent).client).ps.gunframe;
    *fresh35 += 1;
}
#[no_mangle]
pub unsafe extern "C" fn Weapon_Blaster(mut ent: *mut edict_t) {
    static mut pause_frames: [libc::c_int; 3] = [
        19 as libc::c_int,
        32 as libc::c_int,
        0 as libc::c_int,
    ];
    static mut fire_frames: [libc::c_int; 2] = [5 as libc::c_int, 0 as libc::c_int];
    Weapon_Generic(
        ent,
        4 as libc::c_int,
        8 as libc::c_int,
        52 as libc::c_int,
        55 as libc::c_int,
        pause_frames.as_mut_ptr(),
        fire_frames.as_mut_ptr(),
        Some(Weapon_Blaster_Fire as unsafe extern "C" fn(*mut edict_t) -> ()),
    );
}
#[no_mangle]
pub unsafe extern "C" fn Weapon_HyperBlaster_Fire(mut ent: *mut edict_t) {
    let mut rotation: libc::c_float = 0.;
    let mut offset: vec3_t = [0.; 3];
    let mut effect: libc::c_int = 0;
    let mut damage: libc::c_int = 0;
    (*(*ent).client)
        .weapon_sound = (gi.soundindex)
        .expect(
            "non-null function pointer",
        )(
        b"weapons/hyprbl1a.wav\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    if (*(*ent).client).buttons & 1 as libc::c_int == 0 {
        let ref mut fresh36 = (*(*ent).client).ps.gunframe;
        *fresh36 += 1;
    } else {
        if (*(*ent).client).pers.inventory[(*(*ent).client).ammo_index as usize] == 0 {
            if level.time >= (*ent).pain_debounce_time {
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
                        b"weapons/noammo.wav\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    ),
                    1 as libc::c_int as libc::c_float,
                    1 as libc::c_int as libc::c_float,
                    0 as libc::c_int as libc::c_float,
                );
                (*ent)
                    .pain_debounce_time = level.time + 1 as libc::c_int as libc::c_float;
            }
            NoAmmoWeaponChange(ent);
        } else {
            rotation = ((((*(*ent).client).ps.gunframe - 5 as libc::c_int)
                * 2 as libc::c_int) as libc::c_double * 3.14159265358979323846f64
                / 6 as libc::c_int as libc::c_double) as libc::c_float;
            offset[0 as libc::c_int
                as usize] = (-(4 as libc::c_int) as libc::c_double
                * sin(rotation as libc::c_double)) as vec_t;
            offset[1 as libc::c_int as usize] = 0 as libc::c_int as vec_t;
            offset[2 as libc::c_int
                as usize] = (4 as libc::c_int as libc::c_double
                * cos(rotation as libc::c_double)) as vec_t;
            if (*(*ent).client).ps.gunframe == 6 as libc::c_int
                || (*(*ent).client).ps.gunframe == 9 as libc::c_int
            {
                effect = 0x40 as libc::c_int;
            } else {
                effect = 0 as libc::c_int;
            }
            if (*deathmatch).value != 0. {
                damage = 15 as libc::c_int;
            } else {
                damage = 20 as libc::c_int;
            }
            Blaster_Fire(ent, offset.as_mut_ptr(), damage, true_0, effect);
            if (*dmflags).value as libc::c_int & 0x2000 as libc::c_int == 0 {
                let ref mut fresh37 = (*(*ent).client)
                    .pers
                    .inventory[(*(*ent).client).ammo_index as usize];
                *fresh37 -= 1;
            }
            (*(*ent).client).anim_priority = 4 as libc::c_int;
            if (*(*ent).client).ps.pmove.pm_flags as libc::c_int & 1 as libc::c_int != 0
            {
                (*ent).s.frame = 160 as libc::c_int - 1 as libc::c_int;
                (*(*ent).client).anim_end = 168 as libc::c_int;
            } else {
                (*ent).s.frame = 46 as libc::c_int - 1 as libc::c_int;
                (*(*ent).client).anim_end = 53 as libc::c_int;
            }
        }
        let ref mut fresh38 = (*(*ent).client).ps.gunframe;
        *fresh38 += 1;
        if (*(*ent).client).ps.gunframe == 12 as libc::c_int
            && (*(*ent).client).pers.inventory[(*(*ent).client).ammo_index as usize] != 0
        {
            (*(*ent).client).ps.gunframe = 6 as libc::c_int;
        }
    }
    if (*(*ent).client).ps.gunframe == 12 as libc::c_int {
        (gi.sound)
            .expect(
                "non-null function pointer",
            )(
            ent,
            0 as libc::c_int,
            (gi.soundindex)
                .expect(
                    "non-null function pointer",
                )(
                b"weapons/hyprbd1a.wav\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            ),
            1 as libc::c_int as libc::c_float,
            1 as libc::c_int as libc::c_float,
            0 as libc::c_int as libc::c_float,
        );
        (*(*ent).client).weapon_sound = 0 as libc::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn Weapon_HyperBlaster(mut ent: *mut edict_t) {
    static mut pause_frames: [libc::c_int; 1] = [0 as libc::c_int];
    static mut fire_frames: [libc::c_int; 7] = [
        6 as libc::c_int,
        7 as libc::c_int,
        8 as libc::c_int,
        9 as libc::c_int,
        10 as libc::c_int,
        11 as libc::c_int,
        0 as libc::c_int,
    ];
    Weapon_Generic(
        ent,
        5 as libc::c_int,
        20 as libc::c_int,
        49 as libc::c_int,
        53 as libc::c_int,
        pause_frames.as_mut_ptr(),
        fire_frames.as_mut_ptr(),
        Some(Weapon_HyperBlaster_Fire as unsafe extern "C" fn(*mut edict_t) -> ()),
    );
}
#[no_mangle]
pub unsafe extern "C" fn Machinegun_Fire(mut ent: *mut edict_t) {
    let mut i: libc::c_int = 0;
    let mut start: vec3_t = [0.; 3];
    let mut forward: vec3_t = [0.; 3];
    let mut right: vec3_t = [0.; 3];
    let mut angles: vec3_t = [0.; 3];
    let mut damage: libc::c_int = 8 as libc::c_int;
    let mut kick: libc::c_int = 2 as libc::c_int;
    let mut offset: vec3_t = [0.; 3];
    if (*(*ent).client).buttons & 1 as libc::c_int == 0 {
        (*(*ent).client).machinegun_shots = 0 as libc::c_int;
        let ref mut fresh39 = (*(*ent).client).ps.gunframe;
        *fresh39 += 1;
        return;
    }
    if (*(*ent).client).ps.gunframe == 5 as libc::c_int {
        (*(*ent).client).ps.gunframe = 4 as libc::c_int;
    } else {
        (*(*ent).client).ps.gunframe = 5 as libc::c_int;
    }
    if (*(*ent).client).pers.inventory[(*(*ent).client).ammo_index as usize]
        < 1 as libc::c_int
    {
        (*(*ent).client).ps.gunframe = 6 as libc::c_int;
        if level.time >= (*ent).pain_debounce_time {
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
                    b"weapons/noammo.wav\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                ),
                1 as libc::c_int as libc::c_float,
                1 as libc::c_int as libc::c_float,
                0 as libc::c_int as libc::c_float,
            );
            (*ent).pain_debounce_time = level.time + 1 as libc::c_int as libc::c_float;
        }
        NoAmmoWeaponChange(ent);
        return;
    }
    if is_quad as u64 != 0 {
        damage *= 4 as libc::c_int;
        kick *= 4 as libc::c_int;
    }
    i = 1 as libc::c_int;
    while i < 3 as libc::c_int {
        (*(*ent).client)
            .kick_origin[i
            as usize] = (2.0f64
            * (((rand() & 0x7fff as libc::c_int) as libc::c_float
                / 0x7fff as libc::c_int as libc::c_float) as libc::c_double - 0.5f64)
            * 0.35f64) as vec_t;
        (*(*ent).client)
            .kick_angles[i
            as usize] = (2.0f64
            * (((rand() & 0x7fff as libc::c_int) as libc::c_float
                / 0x7fff as libc::c_int as libc::c_float) as libc::c_double - 0.5f64)
            * 0.7f64) as vec_t;
        i += 1;
    }
    (*(*ent).client)
        .kick_origin[0 as libc::c_int
        as usize] = (2.0f64
        * (((rand() & 0x7fff as libc::c_int) as libc::c_float
            / 0x7fff as libc::c_int as libc::c_float) as libc::c_double - 0.5f64)
        * 0.35f64) as vec_t;
    (*(*ent).client)
        .kick_angles[0 as libc::c_int
        as usize] = ((*(*ent).client).machinegun_shots as libc::c_double * -1.5f64)
        as vec_t;
    if (*deathmatch).value == 0. {
        let ref mut fresh40 = (*(*ent).client).machinegun_shots;
        *fresh40 += 1;
        if (*(*ent).client).machinegun_shots > 9 as libc::c_int {
            (*(*ent).client).machinegun_shots = 9 as libc::c_int;
        }
    }
    angles[0 as libc::c_int
        as usize] = (*(*ent).client).v_angle[0 as libc::c_int as usize]
        + (*(*ent).client).kick_angles[0 as libc::c_int as usize];
    angles[1 as libc::c_int
        as usize] = (*(*ent).client).v_angle[1 as libc::c_int as usize]
        + (*(*ent).client).kick_angles[1 as libc::c_int as usize];
    angles[2 as libc::c_int
        as usize] = (*(*ent).client).v_angle[2 as libc::c_int as usize]
        + (*(*ent).client).kick_angles[2 as libc::c_int as usize];
    AngleVectors(
        angles.as_mut_ptr(),
        forward.as_mut_ptr(),
        right.as_mut_ptr(),
        0 as *mut vec_t,
    );
    offset[0 as libc::c_int as usize] = 0 as libc::c_int as vec_t;
    offset[1 as libc::c_int as usize] = 8 as libc::c_int as vec_t;
    offset[2 as libc::c_int as usize] = ((*ent).viewheight - 8 as libc::c_int) as vec_t;
    P_ProjectSource(
        (*ent).client,
        ((*ent).s.origin).as_mut_ptr(),
        offset.as_mut_ptr(),
        forward.as_mut_ptr(),
        right.as_mut_ptr(),
        start.as_mut_ptr(),
    );
    fire_bullet(
        ent,
        start.as_mut_ptr(),
        forward.as_mut_ptr(),
        damage,
        kick,
        300 as libc::c_int,
        500 as libc::c_int,
        4 as libc::c_int,
    );
    (gi.WriteByte).expect("non-null function pointer")(1 as libc::c_int);
    (gi.WriteShort)
        .expect(
            "non-null function pointer",
        )(ent.offset_from(g_edicts) as libc::c_long as libc::c_int);
    (gi.WriteByte)
        .expect(
            "non-null function pointer",
        )(1 as libc::c_int | is_silenced as libc::c_int);
    (gi.multicast)
        .expect(
            "non-null function pointer",
        )(((*ent).s.origin).as_mut_ptr(), MULTICAST_PVS);
    PlayerNoise(ent, start.as_mut_ptr(), 1 as libc::c_int);
    if (*dmflags).value as libc::c_int & 0x2000 as libc::c_int == 0 {
        let ref mut fresh41 = (*(*ent).client)
            .pers
            .inventory[(*(*ent).client).ammo_index as usize];
        *fresh41 -= 1;
    }
    (*(*ent).client).anim_priority = 4 as libc::c_int;
    if (*(*ent).client).ps.pmove.pm_flags as libc::c_int & 1 as libc::c_int != 0 {
        (*ent)
            .s
            .frame = 160 as libc::c_int
            - (((rand() & 0x7fff as libc::c_int) as libc::c_float
                / 0x7fff as libc::c_int as libc::c_float) as libc::c_double + 0.25f64)
                as libc::c_int;
        (*(*ent).client).anim_end = 168 as libc::c_int;
    } else {
        (*ent)
            .s
            .frame = 46 as libc::c_int
            - (((rand() & 0x7fff as libc::c_int) as libc::c_float
                / 0x7fff as libc::c_int as libc::c_float) as libc::c_double + 0.25f64)
                as libc::c_int;
        (*(*ent).client).anim_end = 53 as libc::c_int;
    };
}
#[no_mangle]
pub unsafe extern "C" fn Weapon_Machinegun(mut ent: *mut edict_t) {
    static mut pause_frames: [libc::c_int; 3] = [
        23 as libc::c_int,
        45 as libc::c_int,
        0 as libc::c_int,
    ];
    static mut fire_frames: [libc::c_int; 3] = [
        4 as libc::c_int,
        5 as libc::c_int,
        0 as libc::c_int,
    ];
    Weapon_Generic(
        ent,
        3 as libc::c_int,
        5 as libc::c_int,
        45 as libc::c_int,
        49 as libc::c_int,
        pause_frames.as_mut_ptr(),
        fire_frames.as_mut_ptr(),
        Some(Machinegun_Fire as unsafe extern "C" fn(*mut edict_t) -> ()),
    );
}
#[no_mangle]
pub unsafe extern "C" fn Chaingun_Fire(mut ent: *mut edict_t) {
    let mut i: libc::c_int = 0;
    let mut shots: libc::c_int = 0;
    let mut start: vec3_t = [0.; 3];
    let mut forward: vec3_t = [0.; 3];
    let mut right: vec3_t = [0.; 3];
    let mut up: vec3_t = [0.; 3];
    let mut r: libc::c_float = 0.;
    let mut u: libc::c_float = 0.;
    let mut offset: vec3_t = [0.; 3];
    let mut damage: libc::c_int = 0;
    let mut kick: libc::c_int = 2 as libc::c_int;
    if (*deathmatch).value != 0. {
        damage = 6 as libc::c_int;
    } else {
        damage = 8 as libc::c_int;
    }
    if (*(*ent).client).ps.gunframe == 5 as libc::c_int {
        (gi.sound)
            .expect(
                "non-null function pointer",
            )(
            ent,
            0 as libc::c_int,
            (gi.soundindex)
                .expect(
                    "non-null function pointer",
                )(
                b"weapons/chngnu1a.wav\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            ),
            1 as libc::c_int as libc::c_float,
            2 as libc::c_int as libc::c_float,
            0 as libc::c_int as libc::c_float,
        );
    }
    if (*(*ent).client).ps.gunframe == 14 as libc::c_int
        && (*(*ent).client).buttons & 1 as libc::c_int == 0
    {
        (*(*ent).client).ps.gunframe = 32 as libc::c_int;
        (*(*ent).client).weapon_sound = 0 as libc::c_int;
        return;
    } else {
        if (*(*ent).client).ps.gunframe == 21 as libc::c_int
            && (*(*ent).client).buttons & 1 as libc::c_int != 0
            && (*(*ent).client).pers.inventory[(*(*ent).client).ammo_index as usize] != 0
        {
            (*(*ent).client).ps.gunframe = 15 as libc::c_int;
        } else {
            let ref mut fresh42 = (*(*ent).client).ps.gunframe;
            *fresh42 += 1;
        }
    }
    if (*(*ent).client).ps.gunframe == 22 as libc::c_int {
        (*(*ent).client).weapon_sound = 0 as libc::c_int;
        (gi.sound)
            .expect(
                "non-null function pointer",
            )(
            ent,
            0 as libc::c_int,
            (gi.soundindex)
                .expect(
                    "non-null function pointer",
                )(
                b"weapons/chngnd1a.wav\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            ),
            1 as libc::c_int as libc::c_float,
            2 as libc::c_int as libc::c_float,
            0 as libc::c_int as libc::c_float,
        );
    } else {
        (*(*ent).client)
            .weapon_sound = (gi.soundindex)
            .expect(
                "non-null function pointer",
            )(
            b"weapons/chngnl1a.wav\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    (*(*ent).client).anim_priority = 4 as libc::c_int;
    if (*(*ent).client).ps.pmove.pm_flags as libc::c_int & 1 as libc::c_int != 0 {
        (*ent)
            .s
            .frame = 160 as libc::c_int
            - ((*(*ent).client).ps.gunframe & 1 as libc::c_int);
        (*(*ent).client).anim_end = 168 as libc::c_int;
    } else {
        (*ent)
            .s
            .frame = 46 as libc::c_int
            - ((*(*ent).client).ps.gunframe & 1 as libc::c_int);
        (*(*ent).client).anim_end = 53 as libc::c_int;
    }
    if (*(*ent).client).ps.gunframe <= 9 as libc::c_int {
        shots = 1 as libc::c_int;
    } else if (*(*ent).client).ps.gunframe <= 14 as libc::c_int {
        if (*(*ent).client).buttons & 1 as libc::c_int != 0 {
            shots = 2 as libc::c_int;
        } else {
            shots = 1 as libc::c_int;
        }
    } else {
        shots = 3 as libc::c_int;
    }
    if (*(*ent).client).pers.inventory[(*(*ent).client).ammo_index as usize] < shots {
        shots = (*(*ent).client).pers.inventory[(*(*ent).client).ammo_index as usize];
    }
    if shots == 0 {
        if level.time >= (*ent).pain_debounce_time {
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
                    b"weapons/noammo.wav\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                ),
                1 as libc::c_int as libc::c_float,
                1 as libc::c_int as libc::c_float,
                0 as libc::c_int as libc::c_float,
            );
            (*ent).pain_debounce_time = level.time + 1 as libc::c_int as libc::c_float;
        }
        NoAmmoWeaponChange(ent);
        return;
    }
    if is_quad as u64 != 0 {
        damage *= 4 as libc::c_int;
        kick *= 4 as libc::c_int;
    }
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        (*(*ent).client)
            .kick_origin[i
            as usize] = (2.0f64
            * (((rand() & 0x7fff as libc::c_int) as libc::c_float
                / 0x7fff as libc::c_int as libc::c_float) as libc::c_double - 0.5f64)
            * 0.35f64) as vec_t;
        (*(*ent).client)
            .kick_angles[i
            as usize] = (2.0f64
            * (((rand() & 0x7fff as libc::c_int) as libc::c_float
                / 0x7fff as libc::c_int as libc::c_float) as libc::c_double - 0.5f64)
            * 0.7f64) as vec_t;
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < shots {
        AngleVectors(
            ((*(*ent).client).v_angle).as_mut_ptr(),
            forward.as_mut_ptr(),
            right.as_mut_ptr(),
            up.as_mut_ptr(),
        );
        r = (7 as libc::c_int as libc::c_double
            + 2.0f64
                * (((rand() & 0x7fff as libc::c_int) as libc::c_float
                    / 0x7fff as libc::c_int as libc::c_float) as libc::c_double - 0.5f64)
                * 4 as libc::c_int as libc::c_double) as libc::c_float;
        u = (2.0f64
            * (((rand() & 0x7fff as libc::c_int) as libc::c_float
                / 0x7fff as libc::c_int as libc::c_float) as libc::c_double - 0.5f64)
            * 4 as libc::c_int as libc::c_double) as libc::c_float;
        offset[0 as libc::c_int as usize] = 0 as libc::c_int as vec_t;
        offset[1 as libc::c_int as usize] = r;
        offset[2 as libc::c_int
            as usize] = u + (*ent).viewheight as libc::c_float
            - 8 as libc::c_int as libc::c_float;
        P_ProjectSource(
            (*ent).client,
            ((*ent).s.origin).as_mut_ptr(),
            offset.as_mut_ptr(),
            forward.as_mut_ptr(),
            right.as_mut_ptr(),
            start.as_mut_ptr(),
        );
        fire_bullet(
            ent,
            start.as_mut_ptr(),
            forward.as_mut_ptr(),
            damage,
            kick,
            300 as libc::c_int,
            500 as libc::c_int,
            5 as libc::c_int,
        );
        i += 1;
    }
    (gi.WriteByte).expect("non-null function pointer")(1 as libc::c_int);
    (gi.WriteShort)
        .expect(
            "non-null function pointer",
        )(ent.offset_from(g_edicts) as libc::c_long as libc::c_int);
    (gi.WriteByte)
        .expect(
            "non-null function pointer",
        )(3 as libc::c_int + shots - 1 as libc::c_int | is_silenced as libc::c_int);
    (gi.multicast)
        .expect(
            "non-null function pointer",
        )(((*ent).s.origin).as_mut_ptr(), MULTICAST_PVS);
    PlayerNoise(ent, start.as_mut_ptr(), 1 as libc::c_int);
    if (*dmflags).value as libc::c_int & 0x2000 as libc::c_int == 0 {
        (*(*ent).client).pers.inventory[(*(*ent).client).ammo_index as usize] -= shots;
    }
}
#[no_mangle]
pub unsafe extern "C" fn Weapon_Chaingun(mut ent: *mut edict_t) {
    static mut pause_frames: [libc::c_int; 5] = [
        38 as libc::c_int,
        43 as libc::c_int,
        51 as libc::c_int,
        61 as libc::c_int,
        0 as libc::c_int,
    ];
    static mut fire_frames: [libc::c_int; 18] = [
        5 as libc::c_int,
        6 as libc::c_int,
        7 as libc::c_int,
        8 as libc::c_int,
        9 as libc::c_int,
        10 as libc::c_int,
        11 as libc::c_int,
        12 as libc::c_int,
        13 as libc::c_int,
        14 as libc::c_int,
        15 as libc::c_int,
        16 as libc::c_int,
        17 as libc::c_int,
        18 as libc::c_int,
        19 as libc::c_int,
        20 as libc::c_int,
        21 as libc::c_int,
        0 as libc::c_int,
    ];
    Weapon_Generic(
        ent,
        4 as libc::c_int,
        31 as libc::c_int,
        61 as libc::c_int,
        64 as libc::c_int,
        pause_frames.as_mut_ptr(),
        fire_frames.as_mut_ptr(),
        Some(Chaingun_Fire as unsafe extern "C" fn(*mut edict_t) -> ()),
    );
}
#[no_mangle]
pub unsafe extern "C" fn weapon_shotgun_fire(mut ent: *mut edict_t) {
    let mut start: vec3_t = [0.; 3];
    let mut forward: vec3_t = [0.; 3];
    let mut right: vec3_t = [0.; 3];
    let mut offset: vec3_t = [0.; 3];
    let mut damage: libc::c_int = 4 as libc::c_int;
    let mut kick: libc::c_int = 8 as libc::c_int;
    if (*(*ent).client).ps.gunframe == 9 as libc::c_int {
        let ref mut fresh43 = (*(*ent).client).ps.gunframe;
        *fresh43 += 1;
        return;
    }
    AngleVectors(
        ((*(*ent).client).v_angle).as_mut_ptr(),
        forward.as_mut_ptr(),
        right.as_mut_ptr(),
        0 as *mut vec_t,
    );
    VectorScale(
        forward.as_mut_ptr(),
        -(2 as libc::c_int) as vec_t,
        ((*(*ent).client).kick_origin).as_mut_ptr(),
    );
    (*(*ent).client)
        .kick_angles[0 as libc::c_int as usize] = -(2 as libc::c_int) as vec_t;
    offset[0 as libc::c_int as usize] = 0 as libc::c_int as vec_t;
    offset[1 as libc::c_int as usize] = 8 as libc::c_int as vec_t;
    offset[2 as libc::c_int as usize] = ((*ent).viewheight - 8 as libc::c_int) as vec_t;
    P_ProjectSource(
        (*ent).client,
        ((*ent).s.origin).as_mut_ptr(),
        offset.as_mut_ptr(),
        forward.as_mut_ptr(),
        right.as_mut_ptr(),
        start.as_mut_ptr(),
    );
    if is_quad as u64 != 0 {
        damage *= 4 as libc::c_int;
        kick *= 4 as libc::c_int;
    }
    if (*deathmatch).value != 0. {
        fire_shotgun(
            ent,
            start.as_mut_ptr(),
            forward.as_mut_ptr(),
            damage,
            kick,
            500 as libc::c_int,
            500 as libc::c_int,
            12 as libc::c_int,
            2 as libc::c_int,
        );
    } else {
        fire_shotgun(
            ent,
            start.as_mut_ptr(),
            forward.as_mut_ptr(),
            damage,
            kick,
            500 as libc::c_int,
            500 as libc::c_int,
            12 as libc::c_int,
            2 as libc::c_int,
        );
    }
    (gi.WriteByte).expect("non-null function pointer")(1 as libc::c_int);
    (gi.WriteShort)
        .expect(
            "non-null function pointer",
        )(ent.offset_from(g_edicts) as libc::c_long as libc::c_int);
    (gi.WriteByte)
        .expect(
            "non-null function pointer",
        )(2 as libc::c_int | is_silenced as libc::c_int);
    (gi.multicast)
        .expect(
            "non-null function pointer",
        )(((*ent).s.origin).as_mut_ptr(), MULTICAST_PVS);
    let ref mut fresh44 = (*(*ent).client).ps.gunframe;
    *fresh44 += 1;
    PlayerNoise(ent, start.as_mut_ptr(), 1 as libc::c_int);
    if (*dmflags).value as libc::c_int & 0x2000 as libc::c_int == 0 {
        let ref mut fresh45 = (*(*ent).client)
            .pers
            .inventory[(*(*ent).client).ammo_index as usize];
        *fresh45 -= 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn Weapon_Shotgun(mut ent: *mut edict_t) {
    static mut pause_frames: [libc::c_int; 4] = [
        22 as libc::c_int,
        28 as libc::c_int,
        34 as libc::c_int,
        0 as libc::c_int,
    ];
    static mut fire_frames: [libc::c_int; 3] = [
        8 as libc::c_int,
        9 as libc::c_int,
        0 as libc::c_int,
    ];
    Weapon_Generic(
        ent,
        7 as libc::c_int,
        18 as libc::c_int,
        36 as libc::c_int,
        39 as libc::c_int,
        pause_frames.as_mut_ptr(),
        fire_frames.as_mut_ptr(),
        Some(weapon_shotgun_fire as unsafe extern "C" fn(*mut edict_t) -> ()),
    );
}
#[no_mangle]
pub unsafe extern "C" fn weapon_supershotgun_fire(mut ent: *mut edict_t) {
    let mut start: vec3_t = [0.; 3];
    let mut forward: vec3_t = [0.; 3];
    let mut right: vec3_t = [0.; 3];
    let mut offset: vec3_t = [0.; 3];
    let mut v: vec3_t = [0.; 3];
    let mut damage: libc::c_int = 6 as libc::c_int;
    let mut kick: libc::c_int = 12 as libc::c_int;
    AngleVectors(
        ((*(*ent).client).v_angle).as_mut_ptr(),
        forward.as_mut_ptr(),
        right.as_mut_ptr(),
        0 as *mut vec_t,
    );
    VectorScale(
        forward.as_mut_ptr(),
        -(2 as libc::c_int) as vec_t,
        ((*(*ent).client).kick_origin).as_mut_ptr(),
    );
    (*(*ent).client)
        .kick_angles[0 as libc::c_int as usize] = -(2 as libc::c_int) as vec_t;
    offset[0 as libc::c_int as usize] = 0 as libc::c_int as vec_t;
    offset[1 as libc::c_int as usize] = 8 as libc::c_int as vec_t;
    offset[2 as libc::c_int as usize] = ((*ent).viewheight - 8 as libc::c_int) as vec_t;
    P_ProjectSource(
        (*ent).client,
        ((*ent).s.origin).as_mut_ptr(),
        offset.as_mut_ptr(),
        forward.as_mut_ptr(),
        right.as_mut_ptr(),
        start.as_mut_ptr(),
    );
    if is_quad as u64 != 0 {
        damage *= 4 as libc::c_int;
        kick *= 4 as libc::c_int;
    }
    v[0 as libc::c_int as usize] = (*(*ent).client).v_angle[0 as libc::c_int as usize];
    v[1 as libc::c_int
        as usize] = (*(*ent).client).v_angle[1 as libc::c_int as usize]
        - 5 as libc::c_int as libc::c_float;
    v[2 as libc::c_int as usize] = (*(*ent).client).v_angle[2 as libc::c_int as usize];
    AngleVectors(v.as_mut_ptr(), forward.as_mut_ptr(), 0 as *mut vec_t, 0 as *mut vec_t);
    fire_shotgun(
        ent,
        start.as_mut_ptr(),
        forward.as_mut_ptr(),
        damage,
        kick,
        1000 as libc::c_int,
        500 as libc::c_int,
        20 as libc::c_int / 2 as libc::c_int,
        3 as libc::c_int,
    );
    v[1 as libc::c_int
        as usize] = (*(*ent).client).v_angle[1 as libc::c_int as usize]
        + 5 as libc::c_int as libc::c_float;
    AngleVectors(v.as_mut_ptr(), forward.as_mut_ptr(), 0 as *mut vec_t, 0 as *mut vec_t);
    fire_shotgun(
        ent,
        start.as_mut_ptr(),
        forward.as_mut_ptr(),
        damage,
        kick,
        1000 as libc::c_int,
        500 as libc::c_int,
        20 as libc::c_int / 2 as libc::c_int,
        3 as libc::c_int,
    );
    (gi.WriteByte).expect("non-null function pointer")(1 as libc::c_int);
    (gi.WriteShort)
        .expect(
            "non-null function pointer",
        )(ent.offset_from(g_edicts) as libc::c_long as libc::c_int);
    (gi.WriteByte)
        .expect(
            "non-null function pointer",
        )(13 as libc::c_int | is_silenced as libc::c_int);
    (gi.multicast)
        .expect(
            "non-null function pointer",
        )(((*ent).s.origin).as_mut_ptr(), MULTICAST_PVS);
    let ref mut fresh46 = (*(*ent).client).ps.gunframe;
    *fresh46 += 1;
    PlayerNoise(ent, start.as_mut_ptr(), 1 as libc::c_int);
    if (*dmflags).value as libc::c_int & 0x2000 as libc::c_int == 0 {
        (*(*ent).client).pers.inventory[(*(*ent).client).ammo_index as usize]
            -= 2 as libc::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn Weapon_SuperShotgun(mut ent: *mut edict_t) {
    static mut pause_frames: [libc::c_int; 4] = [
        29 as libc::c_int,
        42 as libc::c_int,
        57 as libc::c_int,
        0 as libc::c_int,
    ];
    static mut fire_frames: [libc::c_int; 2] = [7 as libc::c_int, 0 as libc::c_int];
    Weapon_Generic(
        ent,
        6 as libc::c_int,
        17 as libc::c_int,
        57 as libc::c_int,
        61 as libc::c_int,
        pause_frames.as_mut_ptr(),
        fire_frames.as_mut_ptr(),
        Some(weapon_supershotgun_fire as unsafe extern "C" fn(*mut edict_t) -> ()),
    );
}
#[no_mangle]
pub unsafe extern "C" fn weapon_railgun_fire(mut ent: *mut edict_t) {
    let mut start: vec3_t = [0.; 3];
    let mut forward: vec3_t = [0.; 3];
    let mut right: vec3_t = [0.; 3];
    let mut offset: vec3_t = [0.; 3];
    let mut damage: libc::c_int = 0;
    let mut kick: libc::c_int = 0;
    if (*deathmatch).value != 0. {
        damage = 100 as libc::c_int;
        kick = 200 as libc::c_int;
    } else {
        damage = 150 as libc::c_int;
        kick = 250 as libc::c_int;
    }
    if is_quad as u64 != 0 {
        damage *= 4 as libc::c_int;
        kick *= 4 as libc::c_int;
    }
    AngleVectors(
        ((*(*ent).client).v_angle).as_mut_ptr(),
        forward.as_mut_ptr(),
        right.as_mut_ptr(),
        0 as *mut vec_t,
    );
    VectorScale(
        forward.as_mut_ptr(),
        -(3 as libc::c_int) as vec_t,
        ((*(*ent).client).kick_origin).as_mut_ptr(),
    );
    (*(*ent).client)
        .kick_angles[0 as libc::c_int as usize] = -(3 as libc::c_int) as vec_t;
    offset[0 as libc::c_int as usize] = 0 as libc::c_int as vec_t;
    offset[1 as libc::c_int as usize] = 7 as libc::c_int as vec_t;
    offset[2 as libc::c_int as usize] = ((*ent).viewheight - 8 as libc::c_int) as vec_t;
    P_ProjectSource(
        (*ent).client,
        ((*ent).s.origin).as_mut_ptr(),
        offset.as_mut_ptr(),
        forward.as_mut_ptr(),
        right.as_mut_ptr(),
        start.as_mut_ptr(),
    );
    fire_rail(ent, start.as_mut_ptr(), forward.as_mut_ptr(), damage, kick);
    (gi.WriteByte).expect("non-null function pointer")(1 as libc::c_int);
    (gi.WriteShort)
        .expect(
            "non-null function pointer",
        )(ent.offset_from(g_edicts) as libc::c_long as libc::c_int);
    (gi.WriteByte)
        .expect(
            "non-null function pointer",
        )(6 as libc::c_int | is_silenced as libc::c_int);
    (gi.multicast)
        .expect(
            "non-null function pointer",
        )(((*ent).s.origin).as_mut_ptr(), MULTICAST_PVS);
    let ref mut fresh47 = (*(*ent).client).ps.gunframe;
    *fresh47 += 1;
    PlayerNoise(ent, start.as_mut_ptr(), 1 as libc::c_int);
    if (*dmflags).value as libc::c_int & 0x2000 as libc::c_int == 0 {
        let ref mut fresh48 = (*(*ent).client)
            .pers
            .inventory[(*(*ent).client).ammo_index as usize];
        *fresh48 -= 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn Weapon_Railgun(mut ent: *mut edict_t) {
    static mut pause_frames: [libc::c_int; 2] = [56 as libc::c_int, 0 as libc::c_int];
    static mut fire_frames: [libc::c_int; 2] = [4 as libc::c_int, 0 as libc::c_int];
    Weapon_Generic(
        ent,
        3 as libc::c_int,
        18 as libc::c_int,
        56 as libc::c_int,
        61 as libc::c_int,
        pause_frames.as_mut_ptr(),
        fire_frames.as_mut_ptr(),
        Some(weapon_railgun_fire as unsafe extern "C" fn(*mut edict_t) -> ()),
    );
}
#[no_mangle]
pub unsafe extern "C" fn weapon_bfg_fire(mut ent: *mut edict_t) {
    let mut offset: vec3_t = [0.; 3];
    let mut start: vec3_t = [0.; 3];
    let mut forward: vec3_t = [0.; 3];
    let mut right: vec3_t = [0.; 3];
    let mut damage: libc::c_int = 0;
    let mut damage_radius: libc::c_float = 1000 as libc::c_int as libc::c_float;
    if (*deathmatch).value != 0. {
        damage = 200 as libc::c_int;
    } else {
        damage = 500 as libc::c_int;
    }
    if (*(*ent).client).ps.gunframe == 9 as libc::c_int {
        (gi.WriteByte).expect("non-null function pointer")(1 as libc::c_int);
        (gi.WriteShort)
            .expect(
                "non-null function pointer",
            )(ent.offset_from(g_edicts) as libc::c_long as libc::c_int);
        (gi.WriteByte)
            .expect(
                "non-null function pointer",
            )(12 as libc::c_int | is_silenced as libc::c_int);
        (gi.multicast)
            .expect(
                "non-null function pointer",
            )(((*ent).s.origin).as_mut_ptr(), MULTICAST_PVS);
        let ref mut fresh49 = (*(*ent).client).ps.gunframe;
        *fresh49 += 1;
        PlayerNoise(ent, ((*ent).s.origin).as_mut_ptr(), 1 as libc::c_int);
        return;
    }
    if (*(*ent).client).pers.inventory[(*(*ent).client).ammo_index as usize]
        < 50 as libc::c_int
    {
        let ref mut fresh50 = (*(*ent).client).ps.gunframe;
        *fresh50 += 1;
        return;
    }
    if is_quad as u64 != 0 {
        damage *= 4 as libc::c_int;
    }
    AngleVectors(
        ((*(*ent).client).v_angle).as_mut_ptr(),
        forward.as_mut_ptr(),
        right.as_mut_ptr(),
        0 as *mut vec_t,
    );
    VectorScale(
        forward.as_mut_ptr(),
        -(2 as libc::c_int) as vec_t,
        ((*(*ent).client).kick_origin).as_mut_ptr(),
    );
    (*(*ent).client).v_dmg_pitch = -(40 as libc::c_int) as libc::c_float;
    (*(*ent).client)
        .v_dmg_roll = (2.0f64
        * (((rand() & 0x7fff as libc::c_int) as libc::c_float
            / 0x7fff as libc::c_int as libc::c_float) as libc::c_double - 0.5f64)
        * 8 as libc::c_int as libc::c_double) as libc::c_float;
    (*(*ent).client)
        .v_dmg_time = (level.time as libc::c_double + 0.5f64) as libc::c_float;
    offset[0 as libc::c_int as usize] = 8 as libc::c_int as vec_t;
    offset[1 as libc::c_int as usize] = 8 as libc::c_int as vec_t;
    offset[2 as libc::c_int as usize] = ((*ent).viewheight - 8 as libc::c_int) as vec_t;
    P_ProjectSource(
        (*ent).client,
        ((*ent).s.origin).as_mut_ptr(),
        offset.as_mut_ptr(),
        forward.as_mut_ptr(),
        right.as_mut_ptr(),
        start.as_mut_ptr(),
    );
    fire_bfg(
        ent,
        start.as_mut_ptr(),
        forward.as_mut_ptr(),
        damage,
        400 as libc::c_int,
        damage_radius,
    );
    let ref mut fresh51 = (*(*ent).client).ps.gunframe;
    *fresh51 += 1;
    PlayerNoise(ent, start.as_mut_ptr(), 1 as libc::c_int);
    if (*dmflags).value as libc::c_int & 0x2000 as libc::c_int == 0 {
        (*(*ent).client).pers.inventory[(*(*ent).client).ammo_index as usize]
            -= 50 as libc::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn Weapon_BFG(mut ent: *mut edict_t) {
    static mut pause_frames: [libc::c_int; 5] = [
        39 as libc::c_int,
        45 as libc::c_int,
        50 as libc::c_int,
        55 as libc::c_int,
        0 as libc::c_int,
    ];
    static mut fire_frames: [libc::c_int; 3] = [
        9 as libc::c_int,
        17 as libc::c_int,
        0 as libc::c_int,
    ];
    Weapon_Generic(
        ent,
        8 as libc::c_int,
        32 as libc::c_int,
        55 as libc::c_int,
        58 as libc::c_int,
        pause_frames.as_mut_ptr(),
        fire_frames.as_mut_ptr(),
        Some(weapon_bfg_fire as unsafe extern "C" fn(*mut edict_t) -> ()),
    );
}
