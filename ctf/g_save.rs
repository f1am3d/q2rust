#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(extern_types, register_tool)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fread(
        _: *mut libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn fwrite(
        _: *const libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn Com_sprintf(
        dest: *mut libc::c_char,
        size: libc::c_int,
        fmt: *mut libc::c_char,
        _: ...
    );
    static mut game: game_locals_t;
    static mut level: level_locals_t;
    static mut gi: game_import_t;
    static mut globals: game_export_t;
    static mut g_edicts: *mut edict_t;
    static mut maxentities: *mut cvar_t;
    static mut deathmatch: *mut cvar_t;
    static mut coop: *mut cvar_t;
    static mut dmflags: *mut cvar_t;
    static mut skill: *mut cvar_t;
    static mut fraglimit: *mut cvar_t;
    static mut timelimit: *mut cvar_t;
    static mut capturelimit: *mut cvar_t;
    static mut instantweap: *mut cvar_t;
    static mut password: *mut cvar_t;
    static mut g_select_empty: *mut cvar_t;
    static mut dedicated: *mut cvar_t;
    static mut sv_gravity: *mut cvar_t;
    static mut sv_maxvelocity: *mut cvar_t;
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
    static mut sv_cheats: *mut cvar_t;
    static mut maxclients: *mut cvar_t;
    static mut flood_msgs: *mut cvar_t;
    static mut flood_persecond: *mut cvar_t;
    static mut flood_waitdelay: *mut cvar_t;
    static mut sv_maplist: *mut cvar_t;
    static mut itemlist: [gitem_t; 0];
    fn InitItems();
    fn SaveClientData();
    fn CTFInit();
}
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
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
pub type fieldtype_t = libc::c_uint;
pub const F_IGNORE: fieldtype_t = 9;
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
#[no_mangle]
pub static mut fields: [field_t; 47] = [field_t {
    name: 0 as *mut libc::c_char,
    ofs: 0,
    type_0: F_INT,
    flags: 0,
}; 47];
#[no_mangle]
pub static mut savefields: [field_t; 27] = [field_t {
    name: 0 as *mut libc::c_char,
    ofs: 0,
    type_0: F_INT,
    flags: 0,
}; 27];
#[no_mangle]
pub static mut levelfields: [field_t; 6] = [field_t {
    name: 0 as *mut libc::c_char,
    ofs: 0,
    type_0: F_INT,
    flags: 0,
}; 6];
#[no_mangle]
pub static mut clientfields: [field_t; 4] = [field_t {
    name: 0 as *mut libc::c_char,
    ofs: 0,
    type_0: F_INT,
    flags: 0,
}; 4];
#[no_mangle]
pub unsafe extern "C" fn InitGame() {
    (gi.dprintf)
        .expect(
            "non-null function pointer",
        )(
        b"==== InitGame ====\n\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    gun_x = (gi.cvar)
        .expect(
            "non-null function pointer",
        )(
        b"gun_x\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    gun_y = (gi.cvar)
        .expect(
            "non-null function pointer",
        )(
        b"gun_y\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    gun_z = (gi.cvar)
        .expect(
            "non-null function pointer",
        )(
        b"gun_z\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    sv_rollspeed = (gi.cvar)
        .expect(
            "non-null function pointer",
        )(
        b"sv_rollspeed\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"200\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    sv_rollangle = (gi.cvar)
        .expect(
            "non-null function pointer",
        )(
        b"sv_rollangle\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    sv_maxvelocity = (gi.cvar)
        .expect(
            "non-null function pointer",
        )(
        b"sv_maxvelocity\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"2000\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    sv_gravity = (gi.cvar)
        .expect(
            "non-null function pointer",
        )(
        b"sv_gravity\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"800\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    dedicated = (gi.cvar)
        .expect(
            "non-null function pointer",
        )(
        b"dedicated\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        8 as libc::c_int,
    );
    sv_cheats = (gi.cvar)
        .expect(
            "non-null function pointer",
        )(
        b"cheats\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        4 as libc::c_int | 16 as libc::c_int,
    );
    (gi.cvar)
        .expect(
            "non-null function pointer",
        )(
        b"gamename\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"baseq2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        4 as libc::c_int | 16 as libc::c_int,
    );
    (gi.cvar)
        .expect(
            "non-null function pointer",
        )(
        b"gamedate\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"Jan  2 2023\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        4 as libc::c_int | 16 as libc::c_int,
    );
    maxclients = (gi.cvar)
        .expect(
            "non-null function pointer",
        )(
        b"maxclients\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"4\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        4 as libc::c_int | 16 as libc::c_int,
    );
    deathmatch = (gi.cvar)
        .expect(
            "non-null function pointer",
        )(
        b"deathmatch\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        16 as libc::c_int,
    );
    coop = (gi.cvar)
        .expect(
            "non-null function pointer",
        )(
        b"coop\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        16 as libc::c_int,
    );
    skill = (gi.cvar)
        .expect(
            "non-null function pointer",
        )(
        b"skill\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        16 as libc::c_int,
    );
    maxentities = (gi.cvar)
        .expect(
            "non-null function pointer",
        )(
        b"maxentities\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"1024\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        16 as libc::c_int,
    );
    if (*deathmatch).value == 0. {
        (gi.dprintf)
            .expect(
                "non-null function pointer",
            )(
            b"Forcing deathmatch.\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        (gi.cvar_set)
            .expect(
                "non-null function pointer",
            )(
            b"deathmatch\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
    if (*coop).value != 0. {
        (gi.cvar_set)
            .expect(
                "non-null function pointer",
            )(
            b"coop\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
    dmflags = (gi.cvar)
        .expect(
            "non-null function pointer",
        )(
        b"dmflags\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        4 as libc::c_int,
    );
    fraglimit = (gi.cvar)
        .expect(
            "non-null function pointer",
        )(
        b"fraglimit\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        4 as libc::c_int,
    );
    timelimit = (gi.cvar)
        .expect(
            "non-null function pointer",
        )(
        b"timelimit\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        4 as libc::c_int,
    );
    capturelimit = (gi.cvar)
        .expect(
            "non-null function pointer",
        )(
        b"capturelimit\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        4 as libc::c_int,
    );
    instantweap = (gi.cvar)
        .expect(
            "non-null function pointer",
        )(
        b"instantweap\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        4 as libc::c_int,
    );
    password = (gi.cvar)
        .expect(
            "non-null function pointer",
        )(
        b"password\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        2 as libc::c_int,
    );
    g_select_empty = (gi.cvar)
        .expect(
            "non-null function pointer",
        )(
        b"g_select_empty\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        1 as libc::c_int,
    );
    run_pitch = (gi.cvar)
        .expect(
            "non-null function pointer",
        )(
        b"run_pitch\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"0.002\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    run_roll = (gi.cvar)
        .expect(
            "non-null function pointer",
        )(
        b"run_roll\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"0.005\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    bob_up = (gi.cvar)
        .expect(
            "non-null function pointer",
        )(
        b"bob_up\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"0.005\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    bob_pitch = (gi.cvar)
        .expect(
            "non-null function pointer",
        )(
        b"bob_pitch\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"0.002\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    bob_roll = (gi.cvar)
        .expect(
            "non-null function pointer",
        )(
        b"bob_roll\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"0.002\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    flood_msgs = (gi.cvar)
        .expect(
            "non-null function pointer",
        )(
        b"flood_msgs\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"4\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    flood_persecond = (gi.cvar)
        .expect(
            "non-null function pointer",
        )(
        b"flood_persecond\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"4\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    flood_waitdelay = (gi.cvar)
        .expect(
            "non-null function pointer",
        )(
        b"flood_waitdelay\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"10\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    sv_maplist = (gi.cvar)
        .expect(
            "non-null function pointer",
        )(
        b"sv_maplist\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    InitItems();
    Com_sprintf(
        (game.helpmessage1).as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    Com_sprintf(
        (game.helpmessage2).as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    game.maxentities = (*maxentities).value as libc::c_int;
    g_edicts = (gi.TagMalloc)
        .expect(
            "non-null function pointer",
        )(
        (game.maxentities as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<edict_t>() as libc::c_ulong)
            as libc::c_int,
        765 as libc::c_int,
    ) as *mut edict_t;
    globals.edicts = g_edicts;
    globals.max_edicts = game.maxentities;
    game.maxclients = (*maxclients).value as libc::c_int;
    game
        .clients = (gi.TagMalloc)
        .expect(
            "non-null function pointer",
        )(
        (game.maxclients as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<gclient_t>() as libc::c_ulong)
            as libc::c_int,
        765 as libc::c_int,
    ) as *mut gclient_t;
    globals.num_edicts = game.maxclients + 1 as libc::c_int;
    CTFInit();
}
#[no_mangle]
pub unsafe extern "C" fn WriteField1(
    mut f: *mut FILE,
    mut field: *mut field_t,
    mut base: *mut byte,
) {
    let mut p: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut len: libc::c_int = 0;
    let mut index: libc::c_int = 0;
    p = base.offset((*field).ofs as isize) as *mut libc::c_void;
    match (*field).type_0 as libc::c_uint {
        0 | 1 | 5 | 4 | 9 => {}
        2 | 3 => {
            if !(*(p as *mut *mut libc::c_char)).is_null() {
                len = (strlen(*(p as *mut *mut libc::c_char)))
                    .wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int;
            } else {
                len = 0 as libc::c_int;
            }
            *(p as *mut libc::c_int) = len;
        }
        6 => {
            if (*(p as *mut *mut edict_t)).is_null() {
                index = -(1 as libc::c_int);
            } else {
                index = (*(p as *mut *mut edict_t)).offset_from(g_edicts) as libc::c_long
                    as libc::c_int;
            }
            *(p as *mut libc::c_int) = index;
        }
        8 => {
            if (*(p as *mut *mut gclient_t)).is_null() {
                index = -(1 as libc::c_int);
            } else {
                index = (*(p as *mut *mut gclient_t)).offset_from(game.clients)
                    as libc::c_long as libc::c_int;
            }
            *(p as *mut libc::c_int) = index;
        }
        7 => {
            if (*(p as *mut *mut edict_t)).is_null() {
                index = -(1 as libc::c_int);
            } else {
                index = (*(p as *mut *mut gitem_t)).offset_from(itemlist.as_mut_ptr())
                    as libc::c_long as libc::c_int;
            }
            *(p as *mut libc::c_int) = index;
        }
        _ => {
            (gi.error)
                .expect(
                    "non-null function pointer",
                )(
                b"WriteEdict: unknown field type\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn WriteField2(
    mut f: *mut FILE,
    mut field: *mut field_t,
    mut base: *mut byte,
) {
    let mut len: libc::c_int = 0;
    let mut p: *mut libc::c_void = 0 as *mut libc::c_void;
    p = base.offset((*field).ofs as isize) as *mut libc::c_void;
    match (*field).type_0 as libc::c_uint {
        2 | 3 => {
            if !(*(p as *mut *mut libc::c_char)).is_null() {
                len = (strlen(*(p as *mut *mut libc::c_char)))
                    .wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int;
                fwrite(
                    *(p as *mut *mut libc::c_char) as *const libc::c_void,
                    len as libc::c_ulong,
                    1 as libc::c_int as libc::c_ulong,
                    f,
                );
            }
        }
        _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn ReadField(
    mut f: *mut FILE,
    mut field: *mut field_t,
    mut base: *mut byte,
) {
    let mut p: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut len: libc::c_int = 0;
    let mut index: libc::c_int = 0;
    p = base.offset((*field).ofs as isize) as *mut libc::c_void;
    match (*field).type_0 as libc::c_uint {
        0 | 1 | 5 | 4 | 9 => {}
        2 => {
            len = *(p as *mut libc::c_int);
            if len == 0 {
                let ref mut fresh0 = *(p as *mut *mut libc::c_char);
                *fresh0 = 0 as *mut libc::c_char;
            } else {
                let ref mut fresh1 = *(p as *mut *mut libc::c_char);
                *fresh1 = (gi.TagMalloc)
                    .expect("non-null function pointer")(len, 766 as libc::c_int)
                    as *mut libc::c_char;
                fread(
                    *(p as *mut *mut libc::c_char) as *mut libc::c_void,
                    len as libc::c_ulong,
                    1 as libc::c_int as libc::c_ulong,
                    f,
                );
            }
        }
        3 => {
            len = *(p as *mut libc::c_int);
            if len == 0 {
                let ref mut fresh2 = *(p as *mut *mut libc::c_char);
                *fresh2 = 0 as *mut libc::c_char;
            } else {
                let ref mut fresh3 = *(p as *mut *mut libc::c_char);
                *fresh3 = (gi.TagMalloc)
                    .expect("non-null function pointer")(len, 765 as libc::c_int)
                    as *mut libc::c_char;
                fread(
                    *(p as *mut *mut libc::c_char) as *mut libc::c_void,
                    len as libc::c_ulong,
                    1 as libc::c_int as libc::c_ulong,
                    f,
                );
            }
        }
        6 => {
            index = *(p as *mut libc::c_int);
            if index == -(1 as libc::c_int) {
                let ref mut fresh4 = *(p as *mut *mut edict_t);
                *fresh4 = 0 as *mut edict_t;
            } else {
                let ref mut fresh5 = *(p as *mut *mut edict_t);
                *fresh5 = &mut *g_edicts.offset(index as isize) as *mut edict_t;
            }
        }
        8 => {
            index = *(p as *mut libc::c_int);
            if index == -(1 as libc::c_int) {
                let ref mut fresh6 = *(p as *mut *mut gclient_t);
                *fresh6 = 0 as *mut gclient_t;
            } else {
                let ref mut fresh7 = *(p as *mut *mut gclient_t);
                *fresh7 = &mut *(game.clients).offset(index as isize) as *mut gclient_t;
            }
        }
        7 => {
            index = *(p as *mut libc::c_int);
            if index == -(1 as libc::c_int) {
                let ref mut fresh8 = *(p as *mut *mut gitem_t);
                *fresh8 = 0 as *mut gitem_t;
            } else {
                let ref mut fresh9 = *(p as *mut *mut gitem_t);
                *fresh9 = &mut *itemlist.as_mut_ptr().offset(index as isize)
                    as *mut gitem_t;
            }
        }
        _ => {
            (gi.error)
                .expect(
                    "non-null function pointer",
                )(
                b"ReadEdict: unknown field type\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn WriteClient(mut f: *mut FILE, mut client: *mut gclient_t) {
    let mut field: *mut field_t = 0 as *mut field_t;
    let mut temp: gclient_t = gclient_t {
        ps: player_state_t {
            pmove: pmove_state_t {
                pm_type: PM_NORMAL,
                origin: [0; 3],
                velocity: [0; 3],
                pm_flags: 0,
                pm_time: 0,
                gravity: 0,
                delta_angles: [0; 3],
            },
            viewangles: [0.; 3],
            viewoffset: [0.; 3],
            kick_angles: [0.; 3],
            gunangles: [0.; 3],
            gunoffset: [0.; 3],
            gunindex: 0,
            gunframe: 0,
            blend: [0.; 4],
            fov: 0.,
            rdflags: 0,
            stats: [0; 32],
        },
        ping: 0,
        pers: client_persistant_t {
            userinfo: [0; 512],
            netname: [0; 16],
            hand: 0,
            connected: false_0,
            health: 0,
            max_health: 0,
            powerArmorActive: false_0,
            selected_item: 0,
            inventory: [0; 256],
            max_bullets: 0,
            max_shells: 0,
            max_rockets: 0,
            max_grenades: 0,
            max_cells: 0,
            max_slugs: 0,
            weapon: 0 as *mut gitem_t,
            lastweapon: 0 as *mut gitem_t,
            power_cubes: 0,
            score: 0,
        },
        resp: client_respawn_t {
            coop_respawn: client_persistant_t {
                userinfo: [0; 512],
                netname: [0; 16],
                hand: 0,
                connected: false_0,
                health: 0,
                max_health: 0,
                powerArmorActive: false_0,
                selected_item: 0,
                inventory: [0; 256],
                max_bullets: 0,
                max_shells: 0,
                max_rockets: 0,
                max_grenades: 0,
                max_cells: 0,
                max_slugs: 0,
                weapon: 0 as *mut gitem_t,
                lastweapon: 0 as *mut gitem_t,
                power_cubes: 0,
                score: 0,
            },
            enterframe: 0,
            score: 0,
            ctf_team: 0,
            ctf_state: 0,
            ctf_lasthurtcarrier: 0.,
            ctf_lastreturnedflag: 0.,
            ctf_flagsince: 0.,
            ctf_lastfraggedcarrier: 0.,
            id_state: false_0,
            voted: false_0,
            ready: false_0,
            admin: false_0,
            ghost: 0 as *mut ghost_s,
            cmd_angles: [0.; 3],
            game_helpchanged: 0,
            helpchanged: 0,
        },
        old_pmove: pmove_state_t {
            pm_type: PM_NORMAL,
            origin: [0; 3],
            velocity: [0; 3],
            pm_flags: 0,
            pm_time: 0,
            gravity: 0,
            delta_angles: [0; 3],
        },
        showscores: false_0,
        inmenu: false_0,
        menu: 0 as *mut pmenuhnd_t,
        showinventory: false_0,
        showhelp: false_0,
        showhelpicon: false_0,
        ammo_index: 0,
        buttons: 0,
        oldbuttons: 0,
        latched_buttons: 0,
        weapon_thunk: false_0,
        newweapon: 0 as *mut gitem_t,
        damage_armor: 0,
        damage_parmor: 0,
        damage_blood: 0,
        damage_knockback: 0,
        damage_from: [0.; 3],
        killer_yaw: 0.,
        weaponstate: WEAPON_READY,
        kick_angles: [0.; 3],
        kick_origin: [0.; 3],
        v_dmg_roll: 0.,
        v_dmg_pitch: 0.,
        v_dmg_time: 0.,
        fall_time: 0.,
        fall_value: 0.,
        damage_alpha: 0.,
        bonus_alpha: 0.,
        damage_blend: [0.; 3],
        v_angle: [0.; 3],
        bobtime: 0.,
        oldviewangles: [0.; 3],
        oldvelocity: [0.; 3],
        next_drown_time: 0.,
        old_waterlevel: 0,
        breather_sound: 0,
        machinegun_shots: 0,
        anim_end: 0,
        anim_priority: 0,
        anim_duck: false_0,
        anim_run: false_0,
        quad_framenum: 0.,
        invincible_framenum: 0.,
        breather_framenum: 0.,
        enviro_framenum: 0.,
        grenade_blew_up: false_0,
        grenade_time: 0.,
        silencer_shots: 0,
        weapon_sound: 0,
        pickup_msg_time: 0.,
        flood_locktill: 0.,
        flood_when: [0.; 10],
        flood_whenhead: 0,
        respawn_time: 0.,
        ctf_grapple: 0 as *mut libc::c_void,
        ctf_grapplestate: 0,
        ctf_grapplereleasetime: 0.,
        ctf_regentime: 0.,
        ctf_techsndtime: 0.,
        ctf_lasttechmsg: 0.,
        chase_target: 0 as *mut edict_t,
        update_chase: false_0,
        menutime: 0.,
        menudirty: false_0,
    };
    temp = *client;
    field = clientfields.as_mut_ptr();
    while !((*field).name).is_null() {
        WriteField1(f, field, &mut temp as *mut gclient_t as *mut byte);
        field = field.offset(1);
    }
    fwrite(
        &mut temp as *mut gclient_t as *const libc::c_void,
        ::std::mem::size_of::<gclient_t>() as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        f,
    );
    field = clientfields.as_mut_ptr();
    while !((*field).name).is_null() {
        WriteField2(f, field, client as *mut byte);
        field = field.offset(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn ReadClient(mut f: *mut FILE, mut client: *mut gclient_t) {
    let mut field: *mut field_t = 0 as *mut field_t;
    fread(
        client as *mut libc::c_void,
        ::std::mem::size_of::<gclient_t>() as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        f,
    );
    field = clientfields.as_mut_ptr();
    while !((*field).name).is_null() {
        ReadField(f, field, client as *mut byte);
        field = field.offset(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn WriteGame(
    mut filename: *mut libc::c_char,
    mut autosave: qboolean,
) {
    let mut f: *mut FILE = 0 as *mut FILE;
    let mut i: libc::c_int = 0;
    let mut str: [libc::c_char; 16] = [0; 16];
    if autosave as u64 == 0 {
        SaveClientData();
    }
    f = fopen(filename, b"wb\0" as *const u8 as *const libc::c_char);
    if f.is_null() {
        (gi.error)
            .expect(
                "non-null function pointer",
            )(
            b"Couldn't open %s\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            filename,
        );
    }
    memset(
        str.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong,
    );
    strcpy(str.as_mut_ptr(), b"Jan  2 2023\0" as *const u8 as *const libc::c_char);
    fwrite(
        str.as_mut_ptr() as *const libc::c_void,
        ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        f,
    );
    game.autosaved = autosave;
    fwrite(
        &mut game as *mut game_locals_t as *const libc::c_void,
        ::std::mem::size_of::<game_locals_t>() as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        f,
    );
    game.autosaved = false_0;
    i = 0 as libc::c_int;
    while i < game.maxclients {
        WriteClient(f, &mut *(game.clients).offset(i as isize));
        i += 1;
    }
    fclose(f);
}
#[no_mangle]
pub unsafe extern "C" fn ReadGame(mut filename: *mut libc::c_char) {
    let mut f: *mut FILE = 0 as *mut FILE;
    let mut i: libc::c_int = 0;
    let mut str: [libc::c_char; 16] = [0; 16];
    (gi.FreeTags).expect("non-null function pointer")(765 as libc::c_int);
    f = fopen(filename, b"rb\0" as *const u8 as *const libc::c_char);
    if f.is_null() {
        (gi.error)
            .expect(
                "non-null function pointer",
            )(
            b"Couldn't open %s\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            filename,
        );
    }
    fread(
        str.as_mut_ptr() as *mut libc::c_void,
        ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        f,
    );
    if strcmp(str.as_mut_ptr(), b"Jan  2 2023\0" as *const u8 as *const libc::c_char)
        != 0
    {
        fclose(f);
        (gi.error)
            .expect(
                "non-null function pointer",
            )(
            b"Savegame from an older version.\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    g_edicts = (gi.TagMalloc)
        .expect(
            "non-null function pointer",
        )(
        (game.maxentities as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<edict_t>() as libc::c_ulong)
            as libc::c_int,
        765 as libc::c_int,
    ) as *mut edict_t;
    globals.edicts = g_edicts;
    fread(
        &mut game as *mut game_locals_t as *mut libc::c_void,
        ::std::mem::size_of::<game_locals_t>() as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        f,
    );
    game
        .clients = (gi.TagMalloc)
        .expect(
            "non-null function pointer",
        )(
        (game.maxclients as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<gclient_t>() as libc::c_ulong)
            as libc::c_int,
        765 as libc::c_int,
    ) as *mut gclient_t;
    i = 0 as libc::c_int;
    while i < game.maxclients {
        ReadClient(f, &mut *(game.clients).offset(i as isize));
        i += 1;
    }
    fclose(f);
}
#[no_mangle]
pub unsafe extern "C" fn WriteEdict(mut f: *mut FILE, mut ent: *mut edict_t) {
    let mut field: *mut field_t = 0 as *mut field_t;
    let mut temp: edict_t = edict_t {
        s: entity_state_t {
            number: 0,
            origin: [0.; 3],
            angles: [0.; 3],
            old_origin: [0.; 3],
            modelindex: 0,
            modelindex2: 0,
            modelindex3: 0,
            modelindex4: 0,
            frame: 0,
            skinnum: 0,
            effects: 0,
            renderfx: 0,
            solid: 0,
            sound: 0,
            event: 0,
        },
        client: 0 as *mut gclient_s,
        inuse: false_0,
        linkcount: 0,
        area: link_t {
            prev: 0 as *mut link_s,
            next: 0 as *mut link_s,
        },
        num_clusters: 0,
        clusternums: [0; 16],
        headnode: 0,
        areanum: 0,
        areanum2: 0,
        svflags: 0,
        mins: [0.; 3],
        maxs: [0.; 3],
        absmin: [0.; 3],
        absmax: [0.; 3],
        size: [0.; 3],
        solid: SOLID_NOT,
        clipmask: 0,
        owner: 0 as *mut edict_t,
        movetype: 0,
        flags: 0,
        model: 0 as *mut libc::c_char,
        freetime: 0.,
        message: 0 as *mut libc::c_char,
        classname: 0 as *mut libc::c_char,
        spawnflags: 0,
        timestamp: 0.,
        angle: 0.,
        target: 0 as *mut libc::c_char,
        targetname: 0 as *mut libc::c_char,
        killtarget: 0 as *mut libc::c_char,
        team: 0 as *mut libc::c_char,
        pathtarget: 0 as *mut libc::c_char,
        deathtarget: 0 as *mut libc::c_char,
        combattarget: 0 as *mut libc::c_char,
        target_ent: 0 as *mut edict_t,
        speed: 0.,
        accel: 0.,
        decel: 0.,
        movedir: [0.; 3],
        pos1: [0.; 3],
        pos2: [0.; 3],
        velocity: [0.; 3],
        avelocity: [0.; 3],
        mass: 0,
        air_finished: 0.,
        gravity: 0.,
        goalentity: 0 as *mut edict_t,
        movetarget: 0 as *mut edict_t,
        yaw_speed: 0.,
        ideal_yaw: 0.,
        nextthink: 0.,
        prethink: None,
        think: None,
        blocked: None,
        touch: None,
        use_0: None,
        pain: None,
        die: None,
        touch_debounce_time: 0.,
        pain_debounce_time: 0.,
        damage_debounce_time: 0.,
        fly_sound_debounce_time: 0.,
        last_move_time: 0.,
        health: 0,
        max_health: 0,
        gib_health: 0,
        deadflag: 0,
        show_hostile: false_0,
        powerarmor_time: 0.,
        map: 0 as *mut libc::c_char,
        viewheight: 0,
        takedamage: 0,
        dmg: 0,
        radius_dmg: 0,
        dmg_radius: 0.,
        sounds: 0,
        count: 0,
        chain: 0 as *mut edict_t,
        enemy: 0 as *mut edict_t,
        oldenemy: 0 as *mut edict_t,
        activator: 0 as *mut edict_t,
        groundentity: 0 as *mut edict_t,
        groundentity_linkcount: 0,
        teamchain: 0 as *mut edict_t,
        teammaster: 0 as *mut edict_t,
        mynoise: 0 as *mut edict_t,
        mynoise2: 0 as *mut edict_t,
        noise_index: 0,
        noise_index2: 0,
        volume: 0.,
        attenuation: 0.,
        wait: 0.,
        delay: 0.,
        random: 0.,
        teleport_time: 0.,
        watertype: 0,
        waterlevel: 0,
        move_origin: [0.; 3],
        move_angles: [0.; 3],
        light_level: 0,
        style: 0,
        item: 0 as *mut gitem_t,
        moveinfo: moveinfo_t {
            start_origin: [0.; 3],
            start_angles: [0.; 3],
            end_origin: [0.; 3],
            end_angles: [0.; 3],
            sound_start: 0,
            sound_middle: 0,
            sound_end: 0,
            accel: 0.,
            speed: 0.,
            decel: 0.,
            distance: 0.,
            wait: 0.,
            state: 0,
            dir: [0.; 3],
            current_speed: 0.,
            move_speed: 0.,
            next_speed: 0.,
            remaining_distance: 0.,
            decel_distance: 0.,
            endfunc: None,
        },
        monsterinfo: monsterinfo_t {
            currentmove: 0 as *mut mmove_t,
            aiflags: 0,
            nextframe: 0,
            scale: 0.,
            stand: None,
            idle: None,
            search: None,
            walk: None,
            run: None,
            dodge: None,
            attack: None,
            melee: None,
            sight: None,
            checkattack: None,
            pausetime: 0.,
            attack_finished: 0.,
            saved_goal: [0.; 3],
            search_time: 0.,
            trail_time: 0.,
            last_sighting: [0.; 3],
            attack_state: 0,
            lefty: 0,
            idle_time: 0.,
            linkcount: 0,
            power_armor_type: 0,
            power_armor_power: 0,
        },
    };
    temp = *ent;
    field = savefields.as_mut_ptr();
    while !((*field).name).is_null() {
        WriteField1(f, field, &mut temp as *mut edict_t as *mut byte);
        field = field.offset(1);
    }
    fwrite(
        &mut temp as *mut edict_t as *const libc::c_void,
        ::std::mem::size_of::<edict_t>() as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        f,
    );
    field = savefields.as_mut_ptr();
    while !((*field).name).is_null() {
        WriteField2(f, field, ent as *mut byte);
        field = field.offset(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn WriteLevelLocals(mut f: *mut FILE) {
    let mut field: *mut field_t = 0 as *mut field_t;
    let mut temp: level_locals_t = level_locals_t {
        framenum: 0,
        time: 0.,
        level_name: [0; 64],
        mapname: [0; 64],
        nextmap: [0; 64],
        forcemap: [0; 64],
        intermissiontime: 0.,
        changemap: 0 as *const libc::c_char as *mut libc::c_char,
        exitintermission: 0,
        intermission_origin: [0.; 3],
        intermission_angle: [0.; 3],
        sight_client: 0 as *const edict_t as *mut edict_t,
        sight_entity: 0 as *const edict_t as *mut edict_t,
        sight_entity_framenum: 0,
        sound_entity: 0 as *const edict_t as *mut edict_t,
        sound_entity_framenum: 0,
        sound2_entity: 0 as *const edict_t as *mut edict_t,
        sound2_entity_framenum: 0,
        pic_health: 0,
        total_secrets: 0,
        found_secrets: 0,
        total_goals: 0,
        found_goals: 0,
        total_monsters: 0,
        killed_monsters: 0,
        current_entity: 0 as *const edict_t as *mut edict_t,
        body_que: 0,
        power_cubes: 0,
    };
    temp = level;
    field = levelfields.as_mut_ptr();
    while !((*field).name).is_null() {
        WriteField1(f, field, &mut temp as *mut level_locals_t as *mut byte);
        field = field.offset(1);
    }
    fwrite(
        &mut temp as *mut level_locals_t as *const libc::c_void,
        ::std::mem::size_of::<level_locals_t>() as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        f,
    );
    field = levelfields.as_mut_ptr();
    while !((*field).name).is_null() {
        WriteField2(f, field, &mut level as *mut level_locals_t as *mut byte);
        field = field.offset(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn ReadEdict(mut f: *mut FILE, mut ent: *mut edict_t) {
    let mut field: *mut field_t = 0 as *mut field_t;
    fread(
        ent as *mut libc::c_void,
        ::std::mem::size_of::<edict_t>() as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        f,
    );
    field = savefields.as_mut_ptr();
    while !((*field).name).is_null() {
        ReadField(f, field, ent as *mut byte);
        field = field.offset(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn ReadLevelLocals(mut f: *mut FILE) {
    let mut field: *mut field_t = 0 as *mut field_t;
    fread(
        &mut level as *mut level_locals_t as *mut libc::c_void,
        ::std::mem::size_of::<level_locals_t>() as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        f,
    );
    field = levelfields.as_mut_ptr();
    while !((*field).name).is_null() {
        ReadField(f, field, &mut level as *mut level_locals_t as *mut byte);
        field = field.offset(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn WriteLevel(mut filename: *mut libc::c_char) {
    let mut i: libc::c_int = 0;
    let mut ent: *mut edict_t = 0 as *mut edict_t;
    let mut f: *mut FILE = 0 as *mut FILE;
    let mut base: *mut libc::c_void = 0 as *mut libc::c_void;
    f = fopen(filename, b"wb\0" as *const u8 as *const libc::c_char);
    if f.is_null() {
        (gi.error)
            .expect(
                "non-null function pointer",
            )(
            b"Couldn't open %s\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            filename,
        );
    }
    i = ::std::mem::size_of::<edict_t>() as libc::c_ulong as libc::c_int;
    fwrite(
        &mut i as *mut libc::c_int as *const libc::c_void,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        f,
    );
    base = ::std::mem::transmute::<
        Option::<unsafe extern "C" fn() -> ()>,
        *mut libc::c_void,
    >(Some(InitGame as unsafe extern "C" fn() -> ()));
    fwrite(
        &mut base as *mut *mut libc::c_void as *const libc::c_void,
        ::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        f,
    );
    WriteLevelLocals(f);
    i = 0 as libc::c_int;
    while i < globals.num_edicts {
        ent = &mut *g_edicts.offset(i as isize) as *mut edict_t;
        if !((*ent).inuse as u64 == 0) {
            fwrite(
                &mut i as *mut libc::c_int as *const libc::c_void,
                ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                1 as libc::c_int as libc::c_ulong,
                f,
            );
            WriteEdict(f, ent);
        }
        i += 1;
    }
    i = -(1 as libc::c_int);
    fwrite(
        &mut i as *mut libc::c_int as *const libc::c_void,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        f,
    );
    fclose(f);
}
#[no_mangle]
pub unsafe extern "C" fn ReadLevel(mut filename: *mut libc::c_char) {
    let mut entnum: libc::c_int = 0;
    let mut f: *mut FILE = 0 as *mut FILE;
    let mut i: libc::c_int = 0;
    let mut base: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut ent: *mut edict_t = 0 as *mut edict_t;
    f = fopen(filename, b"rb\0" as *const u8 as *const libc::c_char);
    if f.is_null() {
        (gi.error)
            .expect(
                "non-null function pointer",
            )(
            b"Couldn't open %s\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            filename,
        );
    }
    (gi.FreeTags).expect("non-null function pointer")(766 as libc::c_int);
    memset(
        g_edicts as *mut libc::c_void,
        0 as libc::c_int,
        (game.maxentities as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<edict_t>() as libc::c_ulong),
    );
    globals
        .num_edicts = ((*maxclients).value + 1 as libc::c_int as libc::c_float)
        as libc::c_int;
    fread(
        &mut i as *mut libc::c_int as *mut libc::c_void,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        f,
    );
    if i as libc::c_ulong != ::std::mem::size_of::<edict_t>() as libc::c_ulong {
        fclose(f);
        (gi.error)
            .expect(
                "non-null function pointer",
            )(
            b"ReadLevel: mismatched edict size\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    fread(
        &mut base as *mut *mut libc::c_void as *mut libc::c_void,
        ::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        f,
    );
    if base
        != ::std::mem::transmute::<
            Option::<unsafe extern "C" fn() -> ()>,
            *mut libc::c_void,
        >(Some(InitGame as unsafe extern "C" fn() -> ()))
    {
        fclose(f);
        (gi.error)
            .expect(
                "non-null function pointer",
            )(
            b"ReadLevel: function pointers have moved\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        );
    }
    ReadLevelLocals(f);
    loop {
        if fread(
            &mut entnum as *mut libc::c_int as *mut libc::c_void,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
            1 as libc::c_int as libc::c_ulong,
            f,
        ) != 1 as libc::c_int as libc::c_ulong
        {
            fclose(f);
            (gi.error)
                .expect(
                    "non-null function pointer",
                )(
                b"ReadLevel: failed to read entnum\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
        }
        if entnum == -(1 as libc::c_int) {
            break;
        }
        if entnum >= globals.num_edicts {
            globals.num_edicts = entnum + 1 as libc::c_int;
        }
        ent = &mut *g_edicts.offset(entnum as isize) as *mut edict_t;
        ReadEdict(f, ent);
        memset(
            &mut (*ent).area as *mut link_t as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<link_t>() as libc::c_ulong,
        );
        (gi.linkentity).expect("non-null function pointer")(ent);
    }
    fclose(f);
    i = 0 as libc::c_int;
    while (i as libc::c_float) < (*maxclients).value {
        ent = &mut *g_edicts.offset((i + 1 as libc::c_int) as isize) as *mut edict_t;
        let ref mut fresh10 = (*ent).client;
        *fresh10 = (game.clients).offset(i as isize);
        (*(*ent).client).pers.connected = false_0;
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < globals.num_edicts {
        ent = &mut *g_edicts.offset(i as isize) as *mut edict_t;
        if !((*ent).inuse as u64 == 0) {
            if !((*ent).classname).is_null() {
                if strcmp(
                    (*ent).classname,
                    b"target_crosslevel_target\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    (*ent).nextthink = level.time + (*ent).delay;
                }
            }
        }
        i += 1;
    }
}
unsafe extern "C" fn run_static_initializers() {
    fields = [
        {
            let mut init = field_t {
                name: b"classname\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ofs: &mut (*(0 as *mut edict_t)).classname as *mut *mut libc::c_char
                    as libc::c_int,
                type_0: F_LSTRING,
                flags: 0,
            };
            init
        },
        {
            let mut init = field_t {
                name: b"origin\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ofs: &mut (*(0 as *mut edict_t)).s.origin as *mut vec3_t as libc::c_int,
                type_0: F_VECTOR,
                flags: 0,
            };
            init
        },
        {
            let mut init = field_t {
                name: b"model\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ofs: &mut (*(0 as *mut edict_t)).model as *mut *mut libc::c_char
                    as libc::c_int,
                type_0: F_LSTRING,
                flags: 0,
            };
            init
        },
        {
            let mut init = field_t {
                name: b"spawnflags\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ofs: &mut (*(0 as *mut edict_t)).spawnflags as *mut libc::c_int
                    as libc::c_int,
                type_0: F_INT,
                flags: 0,
            };
            init
        },
        {
            let mut init = field_t {
                name: b"speed\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ofs: &mut (*(0 as *mut edict_t)).speed as *mut libc::c_float
                    as libc::c_int,
                type_0: F_FLOAT,
                flags: 0,
            };
            init
        },
        {
            let mut init = field_t {
                name: b"accel\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ofs: &mut (*(0 as *mut edict_t)).accel as *mut libc::c_float
                    as libc::c_int,
                type_0: F_FLOAT,
                flags: 0,
            };
            init
        },
        {
            let mut init = field_t {
                name: b"decel\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ofs: &mut (*(0 as *mut edict_t)).decel as *mut libc::c_float
                    as libc::c_int,
                type_0: F_FLOAT,
                flags: 0,
            };
            init
        },
        {
            let mut init = field_t {
                name: b"target\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ofs: &mut (*(0 as *mut edict_t)).target as *mut *mut libc::c_char
                    as libc::c_int,
                type_0: F_LSTRING,
                flags: 0,
            };
            init
        },
        {
            let mut init = field_t {
                name: b"targetname\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ofs: &mut (*(0 as *mut edict_t)).targetname as *mut *mut libc::c_char
                    as libc::c_int,
                type_0: F_LSTRING,
                flags: 0,
            };
            init
        },
        {
            let mut init = field_t {
                name: b"pathtarget\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ofs: &mut (*(0 as *mut edict_t)).pathtarget as *mut *mut libc::c_char
                    as libc::c_int,
                type_0: F_LSTRING,
                flags: 0,
            };
            init
        },
        {
            let mut init = field_t {
                name: b"deathtarget\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ofs: &mut (*(0 as *mut edict_t)).deathtarget as *mut *mut libc::c_char
                    as libc::c_int,
                type_0: F_LSTRING,
                flags: 0,
            };
            init
        },
        {
            let mut init = field_t {
                name: b"killtarget\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ofs: &mut (*(0 as *mut edict_t)).killtarget as *mut *mut libc::c_char
                    as libc::c_int,
                type_0: F_LSTRING,
                flags: 0,
            };
            init
        },
        {
            let mut init = field_t {
                name: b"combattarget\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ofs: &mut (*(0 as *mut edict_t)).combattarget as *mut *mut libc::c_char
                    as libc::c_int,
                type_0: F_LSTRING,
                flags: 0,
            };
            init
        },
        {
            let mut init = field_t {
                name: b"message\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ofs: &mut (*(0 as *mut edict_t)).message as *mut *mut libc::c_char
                    as libc::c_int,
                type_0: F_LSTRING,
                flags: 0,
            };
            init
        },
        {
            let mut init = field_t {
                name: b"team\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                ofs: &mut (*(0 as *mut edict_t)).team as *mut *mut libc::c_char
                    as libc::c_int,
                type_0: F_LSTRING,
                flags: 0,
            };
            init
        },
        {
            let mut init = field_t {
                name: b"wait\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                ofs: &mut (*(0 as *mut edict_t)).wait as *mut libc::c_float
                    as libc::c_int,
                type_0: F_FLOAT,
                flags: 0,
            };
            init
        },
        {
            let mut init = field_t {
                name: b"delay\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ofs: &mut (*(0 as *mut edict_t)).delay as *mut libc::c_float
                    as libc::c_int,
                type_0: F_FLOAT,
                flags: 0,
            };
            init
        },
        {
            let mut init = field_t {
                name: b"random\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ofs: &mut (*(0 as *mut edict_t)).random as *mut libc::c_float
                    as libc::c_int,
                type_0: F_FLOAT,
                flags: 0,
            };
            init
        },
        {
            let mut init = field_t {
                name: b"move_origin\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ofs: &mut (*(0 as *mut edict_t)).move_origin as *mut vec3_t
                    as libc::c_int,
                type_0: F_VECTOR,
                flags: 0,
            };
            init
        },
        {
            let mut init = field_t {
                name: b"move_angles\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ofs: &mut (*(0 as *mut edict_t)).move_angles as *mut vec3_t
                    as libc::c_int,
                type_0: F_VECTOR,
                flags: 0,
            };
            init
        },
        {
            let mut init = field_t {
                name: b"style\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ofs: &mut (*(0 as *mut edict_t)).style as *mut libc::c_int
                    as libc::c_int,
                type_0: F_INT,
                flags: 0,
            };
            init
        },
        {
            let mut init = field_t {
                name: b"count\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ofs: &mut (*(0 as *mut edict_t)).count as *mut libc::c_int
                    as libc::c_int,
                type_0: F_INT,
                flags: 0,
            };
            init
        },
        {
            let mut init = field_t {
                name: b"health\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ofs: &mut (*(0 as *mut edict_t)).health as *mut libc::c_int
                    as libc::c_int,
                type_0: F_INT,
                flags: 0,
            };
            init
        },
        {
            let mut init = field_t {
                name: b"sounds\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ofs: &mut (*(0 as *mut edict_t)).sounds as *mut libc::c_int
                    as libc::c_int,
                type_0: F_INT,
                flags: 0,
            };
            init
        },
        {
            let mut init = field_t {
                name: b"light\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ofs: 0 as libc::c_int,
                type_0: F_IGNORE,
                flags: 0,
            };
            init
        },
        {
            let mut init = field_t {
                name: b"dmg\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                ofs: &mut (*(0 as *mut edict_t)).dmg as *mut libc::c_int as libc::c_int,
                type_0: F_INT,
                flags: 0,
            };
            init
        },
        {
            let mut init = field_t {
                name: b"angles\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ofs: &mut (*(0 as *mut edict_t)).s.angles as *mut vec3_t as libc::c_int,
                type_0: F_VECTOR,
                flags: 0,
            };
            init
        },
        {
            let mut init = field_t {
                name: b"angle\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ofs: &mut (*(0 as *mut edict_t)).s.angles as *mut vec3_t as libc::c_int,
                type_0: F_ANGLEHACK,
                flags: 0,
            };
            init
        },
        {
            let mut init = field_t {
                name: b"mass\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                ofs: &mut (*(0 as *mut edict_t)).mass as *mut libc::c_int as libc::c_int,
                type_0: F_INT,
                flags: 0,
            };
            init
        },
        {
            let mut init = field_t {
                name: b"volume\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ofs: &mut (*(0 as *mut edict_t)).volume as *mut libc::c_float
                    as libc::c_int,
                type_0: F_FLOAT,
                flags: 0,
            };
            init
        },
        {
            let mut init = field_t {
                name: b"attenuation\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ofs: &mut (*(0 as *mut edict_t)).attenuation as *mut libc::c_float
                    as libc::c_int,
                type_0: F_FLOAT,
                flags: 0,
            };
            init
        },
        {
            let mut init = field_t {
                name: b"map\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                ofs: &mut (*(0 as *mut edict_t)).map as *mut *mut libc::c_char
                    as libc::c_int,
                type_0: F_LSTRING,
                flags: 0,
            };
            init
        },
        {
            let mut init = field_t {
                name: b"lip\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                ofs: &mut (*(0 as *mut spawn_temp_t)).lip as *mut libc::c_int
                    as libc::c_int,
                type_0: F_INT,
                flags: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = field_t {
                name: b"distance\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ofs: &mut (*(0 as *mut spawn_temp_t)).distance as *mut libc::c_int
                    as libc::c_int,
                type_0: F_INT,
                flags: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = field_t {
                name: b"height\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ofs: &mut (*(0 as *mut spawn_temp_t)).height as *mut libc::c_int
                    as libc::c_int,
                type_0: F_INT,
                flags: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = field_t {
                name: b"noise\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ofs: &mut (*(0 as *mut spawn_temp_t)).noise as *mut *mut libc::c_char
                    as libc::c_int,
                type_0: F_LSTRING,
                flags: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = field_t {
                name: b"pausetime\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ofs: &mut (*(0 as *mut spawn_temp_t)).pausetime as *mut libc::c_float
                    as libc::c_int,
                type_0: F_FLOAT,
                flags: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = field_t {
                name: b"item\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                ofs: &mut (*(0 as *mut spawn_temp_t)).item as *mut *mut libc::c_char
                    as libc::c_int,
                type_0: F_LSTRING,
                flags: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = field_t {
                name: b"gravity\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ofs: &mut (*(0 as *mut spawn_temp_t)).gravity as *mut *mut libc::c_char
                    as libc::c_int,
                type_0: F_LSTRING,
                flags: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = field_t {
                name: b"sky\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                ofs: &mut (*(0 as *mut spawn_temp_t)).sky as *mut *mut libc::c_char
                    as libc::c_int,
                type_0: F_LSTRING,
                flags: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = field_t {
                name: b"skyrotate\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ofs: &mut (*(0 as *mut spawn_temp_t)).skyrotate as *mut libc::c_float
                    as libc::c_int,
                type_0: F_FLOAT,
                flags: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = field_t {
                name: b"skyaxis\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ofs: &mut (*(0 as *mut spawn_temp_t)).skyaxis as *mut vec3_t
                    as libc::c_int,
                type_0: F_VECTOR,
                flags: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = field_t {
                name: b"minyaw\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ofs: &mut (*(0 as *mut spawn_temp_t)).minyaw as *mut libc::c_float
                    as libc::c_int,
                type_0: F_FLOAT,
                flags: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = field_t {
                name: b"maxyaw\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ofs: &mut (*(0 as *mut spawn_temp_t)).maxyaw as *mut libc::c_float
                    as libc::c_int,
                type_0: F_FLOAT,
                flags: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = field_t {
                name: b"minpitch\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ofs: &mut (*(0 as *mut spawn_temp_t)).minpitch as *mut libc::c_float
                    as libc::c_int,
                type_0: F_FLOAT,
                flags: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = field_t {
                name: b"maxpitch\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ofs: &mut (*(0 as *mut spawn_temp_t)).maxpitch as *mut libc::c_float
                    as libc::c_int,
                type_0: F_FLOAT,
                flags: 1 as libc::c_int,
            };
            init
        },
        {
            let mut init = field_t {
                name: b"nextmap\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ofs: &mut (*(0 as *mut spawn_temp_t)).nextmap as *mut *mut libc::c_char
                    as libc::c_int,
                type_0: F_LSTRING,
                flags: 1 as libc::c_int,
            };
            init
        },
    ];
    savefields = [
        {
            let mut init = field_t {
                name: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                ofs: &mut (*(0 as *mut edict_t)).classname as *mut *mut libc::c_char
                    as libc::c_int,
                type_0: F_LSTRING,
                flags: 0,
            };
            init
        },
        {
            let mut init = field_t {
                name: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                ofs: &mut (*(0 as *mut edict_t)).target as *mut *mut libc::c_char
                    as libc::c_int,
                type_0: F_LSTRING,
                flags: 0,
            };
            init
        },
        {
            let mut init = field_t {
                name: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                ofs: &mut (*(0 as *mut edict_t)).targetname as *mut *mut libc::c_char
                    as libc::c_int,
                type_0: F_LSTRING,
                flags: 0,
            };
            init
        },
        {
            let mut init = field_t {
                name: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                ofs: &mut (*(0 as *mut edict_t)).killtarget as *mut *mut libc::c_char
                    as libc::c_int,
                type_0: F_LSTRING,
                flags: 0,
            };
            init
        },
        {
            let mut init = field_t {
                name: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                ofs: &mut (*(0 as *mut edict_t)).team as *mut *mut libc::c_char
                    as libc::c_int,
                type_0: F_LSTRING,
                flags: 0,
            };
            init
        },
        {
            let mut init = field_t {
                name: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                ofs: &mut (*(0 as *mut edict_t)).pathtarget as *mut *mut libc::c_char
                    as libc::c_int,
                type_0: F_LSTRING,
                flags: 0,
            };
            init
        },
        {
            let mut init = field_t {
                name: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                ofs: &mut (*(0 as *mut edict_t)).deathtarget as *mut *mut libc::c_char
                    as libc::c_int,
                type_0: F_LSTRING,
                flags: 0,
            };
            init
        },
        {
            let mut init = field_t {
                name: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                ofs: &mut (*(0 as *mut edict_t)).combattarget as *mut *mut libc::c_char
                    as libc::c_int,
                type_0: F_LSTRING,
                flags: 0,
            };
            init
        },
        {
            let mut init = field_t {
                name: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                ofs: &mut (*(0 as *mut edict_t)).model as *mut *mut libc::c_char
                    as libc::c_int,
                type_0: F_LSTRING,
                flags: 0,
            };
            init
        },
        {
            let mut init = field_t {
                name: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                ofs: &mut (*(0 as *mut edict_t)).map as *mut *mut libc::c_char
                    as libc::c_int,
                type_0: F_LSTRING,
                flags: 0,
            };
            init
        },
        {
            let mut init = field_t {
                name: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                ofs: &mut (*(0 as *mut edict_t)).message as *mut *mut libc::c_char
                    as libc::c_int,
                type_0: F_LSTRING,
                flags: 0,
            };
            init
        },
        {
            let mut init = field_t {
                name: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                ofs: &mut (*(0 as *mut edict_t)).client as *mut *mut gclient_s
                    as libc::c_int,
                type_0: F_CLIENT,
                flags: 0,
            };
            init
        },
        {
            let mut init = field_t {
                name: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                ofs: &mut (*(0 as *mut edict_t)).item as *mut *mut gitem_t
                    as libc::c_int,
                type_0: F_ITEM,
                flags: 0,
            };
            init
        },
        {
            let mut init = field_t {
                name: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                ofs: &mut (*(0 as *mut edict_t)).goalentity as *mut *mut edict_t
                    as libc::c_int,
                type_0: F_EDICT,
                flags: 0,
            };
            init
        },
        {
            let mut init = field_t {
                name: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                ofs: &mut (*(0 as *mut edict_t)).movetarget as *mut *mut edict_t
                    as libc::c_int,
                type_0: F_EDICT,
                flags: 0,
            };
            init
        },
        {
            let mut init = field_t {
                name: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                ofs: &mut (*(0 as *mut edict_t)).enemy as *mut *mut edict_t
                    as libc::c_int,
                type_0: F_EDICT,
                flags: 0,
            };
            init
        },
        {
            let mut init = field_t {
                name: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                ofs: &mut (*(0 as *mut edict_t)).oldenemy as *mut *mut edict_t
                    as libc::c_int,
                type_0: F_EDICT,
                flags: 0,
            };
            init
        },
        {
            let mut init = field_t {
                name: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                ofs: &mut (*(0 as *mut edict_t)).activator as *mut *mut edict_t
                    as libc::c_int,
                type_0: F_EDICT,
                flags: 0,
            };
            init
        },
        {
            let mut init = field_t {
                name: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                ofs: &mut (*(0 as *mut edict_t)).groundentity as *mut *mut edict_t
                    as libc::c_int,
                type_0: F_EDICT,
                flags: 0,
            };
            init
        },
        {
            let mut init = field_t {
                name: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                ofs: &mut (*(0 as *mut edict_t)).teamchain as *mut *mut edict_t
                    as libc::c_int,
                type_0: F_EDICT,
                flags: 0,
            };
            init
        },
        {
            let mut init = field_t {
                name: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                ofs: &mut (*(0 as *mut edict_t)).teammaster as *mut *mut edict_t
                    as libc::c_int,
                type_0: F_EDICT,
                flags: 0,
            };
            init
        },
        {
            let mut init = field_t {
                name: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                ofs: &mut (*(0 as *mut edict_t)).owner as *mut *mut edict_t
                    as libc::c_int,
                type_0: F_EDICT,
                flags: 0,
            };
            init
        },
        {
            let mut init = field_t {
                name: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                ofs: &mut (*(0 as *mut edict_t)).mynoise as *mut *mut edict_t
                    as libc::c_int,
                type_0: F_EDICT,
                flags: 0,
            };
            init
        },
        {
            let mut init = field_t {
                name: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                ofs: &mut (*(0 as *mut edict_t)).mynoise2 as *mut *mut edict_t
                    as libc::c_int,
                type_0: F_EDICT,
                flags: 0,
            };
            init
        },
        {
            let mut init = field_t {
                name: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                ofs: &mut (*(0 as *mut edict_t)).target_ent as *mut *mut edict_t
                    as libc::c_int,
                type_0: F_EDICT,
                flags: 0,
            };
            init
        },
        {
            let mut init = field_t {
                name: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                ofs: &mut (*(0 as *mut edict_t)).chain as *mut *mut edict_t
                    as libc::c_int,
                type_0: F_EDICT,
                flags: 0,
            };
            init
        },
        {
            let mut init = field_t {
                name: 0 as *mut libc::c_char,
                ofs: 0 as libc::c_int,
                type_0: F_INT,
                flags: 0,
            };
            init
        },
    ];
    levelfields = [
        {
            let mut init = field_t {
                name: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                ofs: &mut (*(0 as *mut level_locals_t)).changemap
                    as *mut *mut libc::c_char as libc::c_int,
                type_0: F_LSTRING,
                flags: 0,
            };
            init
        },
        {
            let mut init = field_t {
                name: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                ofs: &mut (*(0 as *mut level_locals_t)).sight_client as *mut *mut edict_t
                    as libc::c_int,
                type_0: F_EDICT,
                flags: 0,
            };
            init
        },
        {
            let mut init = field_t {
                name: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                ofs: &mut (*(0 as *mut level_locals_t)).sight_entity as *mut *mut edict_t
                    as libc::c_int,
                type_0: F_EDICT,
                flags: 0,
            };
            init
        },
        {
            let mut init = field_t {
                name: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                ofs: &mut (*(0 as *mut level_locals_t)).sound_entity as *mut *mut edict_t
                    as libc::c_int,
                type_0: F_EDICT,
                flags: 0,
            };
            init
        },
        {
            let mut init = field_t {
                name: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                ofs: &mut (*(0 as *mut level_locals_t)).sound2_entity
                    as *mut *mut edict_t as libc::c_int,
                type_0: F_EDICT,
                flags: 0,
            };
            init
        },
        {
            let mut init = field_t {
                name: 0 as *mut libc::c_char,
                ofs: 0 as libc::c_int,
                type_0: F_INT,
                flags: 0,
            };
            init
        },
    ];
    clientfields = [
        {
            let mut init = field_t {
                name: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                ofs: &mut (*(0 as *mut gclient_t)).pers.weapon as *mut *mut gitem_t
                    as libc::c_int,
                type_0: F_ITEM,
                flags: 0,
            };
            init
        },
        {
            let mut init = field_t {
                name: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                ofs: &mut (*(0 as *mut gclient_t)).pers.lastweapon as *mut *mut gitem_t
                    as libc::c_int,
                type_0: F_ITEM,
                flags: 0,
            };
            init
        },
        {
            let mut init = field_t {
                name: b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                ofs: &mut (*(0 as *mut gclient_t)).newweapon as *mut *mut gitem_t
                    as libc::c_int,
                type_0: F_ITEM,
                flags: 0,
            };
            init
        },
        {
            let mut init = field_t {
                name: 0 as *mut libc::c_char,
                ofs: 0 as libc::c_int,
                type_0: F_INT,
                flags: 0,
            };
            init
        },
    ];
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
