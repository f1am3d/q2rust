#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    fn fire_hit(
        self_0: *mut edict_t,
        aim: *mut vec_t,
        damage: libc::c_int,
        kick: libc::c_int,
    ) -> qboolean;
    fn range(self_0: *mut edict_t, other: *mut edict_t) -> libc::c_int;
    fn ai_charge(self_0: *mut edict_t, dist: libc::c_float);
    fn ai_run(self_0: *mut edict_t, dist: libc::c_float);
    fn ai_walk(self_0: *mut edict_t, dist: libc::c_float);
    fn ai_move(self_0: *mut edict_t, dist: libc::c_float);
    fn ai_stand(self_0: *mut edict_t, dist: libc::c_float);
    fn rand() -> libc::c_int;
    fn VectorMA(
        veca: *mut vec_t,
        scale: libc::c_float,
        vecb: *mut vec_t,
        vecc: *mut vec_t,
    );
    fn VectorNormalize(v: *mut vec_t) -> vec_t;
    fn AngleVectors(
        angles: *mut vec_t,
        forward: *mut vec_t,
        right: *mut vec_t,
        up: *mut vec_t,
    );
    static mut monster_flash_offset: [vec3_t; 0];
    static mut level: level_locals_t;
    static mut gi: game_import_t;
    static mut deathmatch: *mut cvar_t;
    static mut skill: *mut cvar_t;
    fn G_ProjectSource(
        point: *mut vec_t,
        distance: *mut vec_t,
        forward: *mut vec_t,
        right: *mut vec_t,
        result: *mut vec_t,
    );
    fn G_FreeEdict(e: *mut edict_t);
    fn monster_fire_bullet(
        self_0: *mut edict_t,
        start: *mut vec_t,
        dir: *mut vec_t,
        damage: libc::c_int,
        kick: libc::c_int,
        hspread: libc::c_int,
        vspread: libc::c_int,
        flashtype: libc::c_int,
    );
    fn walkmonster_start(self_0: *mut edict_t);
    fn M_FlyCheck(self_0: *mut edict_t);
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
static mut sound_pain1: libc::c_int = 0;
static mut sound_pain2: libc::c_int = 0;
static mut sound_die1: libc::c_int = 0;
static mut sound_die2: libc::c_int = 0;
static mut sound_gunshot: libc::c_int = 0;
static mut sound_weapon_cock: libc::c_int = 0;
static mut sound_punch_swing: libc::c_int = 0;
static mut sound_punch_hit: libc::c_int = 0;
static mut sound_sight: libc::c_int = 0;
static mut sound_search: libc::c_int = 0;
static mut sound_idle: libc::c_int = 0;
#[no_mangle]
pub static mut infantry_frames_stand: [mframe_t; 22] = unsafe {
    [
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
pub static mut infantry_move_stand: mmove_t = unsafe {
    {
        let mut init = mmove_t {
            firstframe: 50 as libc::c_int,
            lastframe: 71 as libc::c_int,
            frame: infantry_frames_stand.as_ptr() as *mut _,
            endfunc: None,
        };
        init
    }
};
#[no_mangle]
pub unsafe extern "C" fn infantry_stand(mut self_0: *mut edict_t) {
    let ref mut fresh0 = (*self_0).monsterinfo.currentmove;
    *fresh0 = &mut infantry_move_stand;
}
#[no_mangle]
pub static mut infantry_frames_fidget: [mframe_t; 49] = unsafe {
    [
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_stand as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 1 as libc::c_int as libc::c_float,
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
                dist: 1 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_stand as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 3 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_stand as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 6 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_stand as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 3 as libc::c_int as libc::c_float,
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
                dist: 1 as libc::c_int as libc::c_float,
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
                dist: 1 as libc::c_int as libc::c_float,
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
                dist: -(1 as libc::c_int) as libc::c_float,
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
                dist: 1 as libc::c_int as libc::c_float,
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
                dist: -(2 as libc::c_int) as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_stand as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 1 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_stand as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 1 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_stand as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 1 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_stand as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: -(1 as libc::c_int) as libc::c_float,
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
                dist: -(1 as libc::c_int) as libc::c_float,
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
                dist: -(1 as libc::c_int) as libc::c_float,
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
                dist: 1 as libc::c_int as libc::c_float,
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
                dist: -(1 as libc::c_int) as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_stand as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: -(1 as libc::c_int) as libc::c_float,
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
                dist: -(3 as libc::c_int) as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_stand as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: -(2 as libc::c_int) as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_stand as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: -(3 as libc::c_int) as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_stand as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: -(3 as libc::c_int) as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_stand as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: -(2 as libc::c_int) as libc::c_float,
                thinkfunc: None,
            };
            init
        },
    ]
};
#[no_mangle]
pub static mut infantry_move_fidget: mmove_t = unsafe {
    {
        let mut init = mmove_t {
            firstframe: 1 as libc::c_int,
            lastframe: 49 as libc::c_int,
            frame: infantry_frames_fidget.as_ptr() as *mut _,
            endfunc: Some(infantry_stand as unsafe extern "C" fn(*mut edict_t) -> ()),
        };
        init
    }
};
#[no_mangle]
pub unsafe extern "C" fn infantry_fidget(mut self_0: *mut edict_t) {
    let ref mut fresh1 = (*self_0).monsterinfo.currentmove;
    *fresh1 = &mut infantry_move_fidget;
    (gi.sound)
        .expect(
            "non-null function pointer",
        )(
        self_0,
        2 as libc::c_int,
        sound_idle,
        1 as libc::c_int as libc::c_float,
        2 as libc::c_int as libc::c_float,
        0 as libc::c_int as libc::c_float,
    );
}
#[no_mangle]
pub static mut infantry_frames_walk: [mframe_t; 12] = unsafe {
    [
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_walk as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 5 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_walk as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 4 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_walk as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 4 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_walk as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 5 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_walk as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 4 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_walk as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 5 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_walk as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 6 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_walk as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 4 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_walk as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 4 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_walk as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 4 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_walk as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 4 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_walk as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 5 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
    ]
};
#[no_mangle]
pub static mut infantry_move_walk: mmove_t = unsafe {
    {
        let mut init = mmove_t {
            firstframe: 74 as libc::c_int,
            lastframe: 85 as libc::c_int,
            frame: infantry_frames_walk.as_ptr() as *mut _,
            endfunc: None,
        };
        init
    }
};
#[no_mangle]
pub unsafe extern "C" fn infantry_walk(mut self_0: *mut edict_t) {
    let ref mut fresh2 = (*self_0).monsterinfo.currentmove;
    *fresh2 = &mut infantry_move_walk;
}
#[no_mangle]
pub static mut infantry_frames_run: [mframe_t; 8] = unsafe {
    [
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_run as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 10 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_run as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 20 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_run as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 5 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_run as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 7 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_run as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 30 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_run as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 35 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_run as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 2 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_run as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 6 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
    ]
};
#[no_mangle]
pub static mut infantry_move_run: mmove_t = unsafe {
    {
        let mut init = mmove_t {
            firstframe: 92 as libc::c_int,
            lastframe: 99 as libc::c_int,
            frame: infantry_frames_run.as_ptr() as *mut _,
            endfunc: None,
        };
        init
    }
};
#[no_mangle]
pub unsafe extern "C" fn infantry_run(mut self_0: *mut edict_t) {
    if (*self_0).monsterinfo.aiflags & 0x1 as libc::c_int != 0 {
        let ref mut fresh3 = (*self_0).monsterinfo.currentmove;
        *fresh3 = &mut infantry_move_stand;
    } else {
        let ref mut fresh4 = (*self_0).monsterinfo.currentmove;
        *fresh4 = &mut infantry_move_run;
    };
}
#[no_mangle]
pub static mut infantry_frames_pain1: [mframe_t; 10] = unsafe {
    [
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
                dist: 1 as libc::c_int as libc::c_float,
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
                dist: 1 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_move as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 6 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
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
    ]
};
#[no_mangle]
pub static mut infantry_move_pain1: mmove_t = unsafe {
    {
        let mut init = mmove_t {
            firstframe: 100 as libc::c_int,
            lastframe: 109 as libc::c_int,
            frame: infantry_frames_pain1.as_ptr() as *mut _,
            endfunc: Some(infantry_run as unsafe extern "C" fn(*mut edict_t) -> ()),
        };
        init
    }
};
#[no_mangle]
pub static mut infantry_frames_pain2: [mframe_t; 10] = unsafe {
    [
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
                dist: 2 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
    ]
};
#[no_mangle]
pub static mut infantry_move_pain2: mmove_t = unsafe {
    {
        let mut init = mmove_t {
            firstframe: 110 as libc::c_int,
            lastframe: 119 as libc::c_int,
            frame: infantry_frames_pain2.as_ptr() as *mut _,
            endfunc: Some(infantry_run as unsafe extern "C" fn(*mut edict_t) -> ()),
        };
        init
    }
};
#[no_mangle]
pub unsafe extern "C" fn infantry_pain(
    mut self_0: *mut edict_t,
    mut other: *mut edict_t,
    mut kick: libc::c_float,
    mut damage: libc::c_int,
) {
    let mut n: libc::c_int = 0;
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
    n = rand() % 2 as libc::c_int;
    if n == 0 as libc::c_int {
        let ref mut fresh5 = (*self_0).monsterinfo.currentmove;
        *fresh5 = &mut infantry_move_pain1;
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
        let ref mut fresh6 = (*self_0).monsterinfo.currentmove;
        *fresh6 = &mut infantry_move_pain2;
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
pub static mut aimangles: [vec3_t; 12] = [
    [0.0f64 as vec_t, 5.0f64 as vec_t, 0.0f64 as vec_t],
    [10.0f64 as vec_t, 15.0f64 as vec_t, 0.0f64 as vec_t],
    [20.0f64 as vec_t, 25.0f64 as vec_t, 0.0f64 as vec_t],
    [25.0f64 as vec_t, 35.0f64 as vec_t, 0.0f64 as vec_t],
    [30.0f64 as vec_t, 40.0f64 as vec_t, 0.0f64 as vec_t],
    [30.0f64 as vec_t, 45.0f64 as vec_t, 0.0f64 as vec_t],
    [25.0f64 as vec_t, 50.0f64 as vec_t, 0.0f64 as vec_t],
    [20.0f64 as vec_t, 40.0f64 as vec_t, 0.0f64 as vec_t],
    [15.0f64 as vec_t, 35.0f64 as vec_t, 0.0f64 as vec_t],
    [40.0f64 as vec_t, 35.0f64 as vec_t, 0.0f64 as vec_t],
    [70.0f64 as vec_t, 35.0f64 as vec_t, 0.0f64 as vec_t],
    [90.0f64 as vec_t, 35.0f64 as vec_t, 0.0f64 as vec_t],
];
#[no_mangle]
pub unsafe extern "C" fn InfantryMachineGun(mut self_0: *mut edict_t) {
    let mut start: vec3_t = [0.; 3];
    let mut target: vec3_t = [0.; 3];
    let mut forward: vec3_t = [0.; 3];
    let mut right: vec3_t = [0.; 3];
    let mut vec: vec3_t = [0.; 3];
    let mut flash_number: libc::c_int = 0;
    if (*self_0).s.frame == 194 as libc::c_int {
        flash_number = 26 as libc::c_int;
        AngleVectors(
            ((*self_0).s.angles).as_mut_ptr(),
            forward.as_mut_ptr(),
            right.as_mut_ptr(),
            0 as *mut vec_t,
        );
        G_ProjectSource(
            ((*self_0).s.origin).as_mut_ptr(),
            (*monster_flash_offset.as_mut_ptr().offset(flash_number as isize))
                .as_mut_ptr(),
            forward.as_mut_ptr(),
            right.as_mut_ptr(),
            start.as_mut_ptr(),
        );
        if !((*self_0).enemy).is_null() {
            VectorMA(
                ((*(*self_0).enemy).s.origin).as_mut_ptr(),
                -0.2f64 as libc::c_float,
                ((*(*self_0).enemy).velocity).as_mut_ptr(),
                target.as_mut_ptr(),
            );
            target[2 as libc::c_int as usize]
                += (*(*self_0).enemy).viewheight as libc::c_float;
            forward[0 as libc::c_int
                as usize] = target[0 as libc::c_int as usize]
                - start[0 as libc::c_int as usize];
            forward[1 as libc::c_int
                as usize] = target[1 as libc::c_int as usize]
                - start[1 as libc::c_int as usize];
            forward[2 as libc::c_int
                as usize] = target[2 as libc::c_int as usize]
                - start[2 as libc::c_int as usize];
            VectorNormalize(forward.as_mut_ptr());
        } else {
            AngleVectors(
                ((*self_0).s.angles).as_mut_ptr(),
                forward.as_mut_ptr(),
                right.as_mut_ptr(),
                0 as *mut vec_t,
            );
        }
    } else {
        flash_number = 27 as libc::c_int + ((*self_0).s.frame - 155 as libc::c_int);
        AngleVectors(
            ((*self_0).s.angles).as_mut_ptr(),
            forward.as_mut_ptr(),
            right.as_mut_ptr(),
            0 as *mut vec_t,
        );
        G_ProjectSource(
            ((*self_0).s.origin).as_mut_ptr(),
            (*monster_flash_offset.as_mut_ptr().offset(flash_number as isize))
                .as_mut_ptr(),
            forward.as_mut_ptr(),
            right.as_mut_ptr(),
            start.as_mut_ptr(),
        );
        vec[0 as libc::c_int
            as usize] = (*self_0).s.angles[0 as libc::c_int as usize]
            - aimangles[(flash_number - 27 as libc::c_int)
                as usize][0 as libc::c_int as usize];
        vec[1 as libc::c_int
            as usize] = (*self_0).s.angles[1 as libc::c_int as usize]
            - aimangles[(flash_number - 27 as libc::c_int)
                as usize][1 as libc::c_int as usize];
        vec[2 as libc::c_int
            as usize] = (*self_0).s.angles[2 as libc::c_int as usize]
            - aimangles[(flash_number - 27 as libc::c_int)
                as usize][2 as libc::c_int as usize];
        AngleVectors(
            vec.as_mut_ptr(),
            forward.as_mut_ptr(),
            0 as *mut vec_t,
            0 as *mut vec_t,
        );
    }
    monster_fire_bullet(
        self_0,
        start.as_mut_ptr(),
        forward.as_mut_ptr(),
        3 as libc::c_int,
        4 as libc::c_int,
        300 as libc::c_int,
        500 as libc::c_int,
        flash_number,
    );
}
#[no_mangle]
pub unsafe extern "C" fn infantry_sight(
    mut self_0: *mut edict_t,
    mut other: *mut edict_t,
) {
    (gi.sound)
        .expect(
            "non-null function pointer",
        )(
        self_0,
        4 as libc::c_int,
        sound_sight,
        1 as libc::c_int as libc::c_float,
        1 as libc::c_int as libc::c_float,
        0 as libc::c_int as libc::c_float,
    );
}
#[no_mangle]
pub unsafe extern "C" fn infantry_dead(mut self_0: *mut edict_t) {
    (*self_0).mins[0 as libc::c_int as usize] = -(16 as libc::c_int) as vec_t;
    (*self_0).mins[1 as libc::c_int as usize] = -(16 as libc::c_int) as vec_t;
    (*self_0).mins[2 as libc::c_int as usize] = -(24 as libc::c_int) as vec_t;
    (*self_0).maxs[0 as libc::c_int as usize] = 16 as libc::c_int as vec_t;
    (*self_0).maxs[1 as libc::c_int as usize] = 16 as libc::c_int as vec_t;
    (*self_0).maxs[2 as libc::c_int as usize] = -(8 as libc::c_int) as vec_t;
    (*self_0).movetype = MOVETYPE_TOSS as libc::c_int;
    (*self_0).svflags |= 0x2 as libc::c_int;
    (gi.linkentity).expect("non-null function pointer")(self_0);
    M_FlyCheck(self_0);
}
#[no_mangle]
pub static mut infantry_frames_death1: [mframe_t; 20] = unsafe {
    [
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_move as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: -(4 as libc::c_int) as libc::c_float,
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
                dist: -(4 as libc::c_int) as libc::c_float,
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
                dist: 1 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_move as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 1 as libc::c_int as libc::c_float,
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
                dist: 9 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_move as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 9 as libc::c_int as libc::c_float,
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
                dist: -(3 as libc::c_int) as libc::c_float,
                thinkfunc: None,
            };
            init
        },
    ]
};
#[no_mangle]
pub static mut infantry_move_death1: mmove_t = unsafe {
    {
        let mut init = mmove_t {
            firstframe: 125 as libc::c_int,
            lastframe: 144 as libc::c_int,
            frame: infantry_frames_death1.as_ptr() as *mut _,
            endfunc: Some(infantry_dead as unsafe extern "C" fn(*mut edict_t) -> ()),
        };
        init
    }
};
#[no_mangle]
pub static mut infantry_frames_death2: [mframe_t; 25] = unsafe {
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
                dist: 1 as libc::c_int as libc::c_float,
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
                dist: 1 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_move as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 1 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_move as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 4 as libc::c_int as libc::c_float,
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
                dist: -(2 as libc::c_int) as libc::c_float,
                thinkfunc: Some(
                    InfantryMachineGun as unsafe extern "C" fn(*mut edict_t) -> (),
                ),
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_move as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: -(2 as libc::c_int) as libc::c_float,
                thinkfunc: Some(
                    InfantryMachineGun as unsafe extern "C" fn(*mut edict_t) -> (),
                ),
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_move as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: -(3 as libc::c_int) as libc::c_float,
                thinkfunc: Some(
                    InfantryMachineGun as unsafe extern "C" fn(*mut edict_t) -> (),
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
                    InfantryMachineGun as unsafe extern "C" fn(*mut edict_t) -> (),
                ),
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_move as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: -(2 as libc::c_int) as libc::c_float,
                thinkfunc: Some(
                    InfantryMachineGun as unsafe extern "C" fn(*mut edict_t) -> (),
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
                    InfantryMachineGun as unsafe extern "C" fn(*mut edict_t) -> (),
                ),
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_move as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 2 as libc::c_int as libc::c_float,
                thinkfunc: Some(
                    InfantryMachineGun as unsafe extern "C" fn(*mut edict_t) -> (),
                ),
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_move as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 2 as libc::c_int as libc::c_float,
                thinkfunc: Some(
                    InfantryMachineGun as unsafe extern "C" fn(*mut edict_t) -> (),
                ),
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_move as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 3 as libc::c_int as libc::c_float,
                thinkfunc: Some(
                    InfantryMachineGun as unsafe extern "C" fn(*mut edict_t) -> (),
                ),
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_move as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: -(10 as libc::c_int) as libc::c_float,
                thinkfunc: Some(
                    InfantryMachineGun as unsafe extern "C" fn(*mut edict_t) -> (),
                ),
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_move as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: -(7 as libc::c_int) as libc::c_float,
                thinkfunc: Some(
                    InfantryMachineGun as unsafe extern "C" fn(*mut edict_t) -> (),
                ),
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_move as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: -(8 as libc::c_int) as libc::c_float,
                thinkfunc: Some(
                    InfantryMachineGun as unsafe extern "C" fn(*mut edict_t) -> (),
                ),
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_move as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: -(6 as libc::c_int) as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_move as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 4 as libc::c_int as libc::c_float,
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
pub static mut infantry_move_death2: mmove_t = unsafe {
    {
        let mut init = mmove_t {
            firstframe: 145 as libc::c_int,
            lastframe: 169 as libc::c_int,
            frame: infantry_frames_death2.as_ptr() as *mut _,
            endfunc: Some(infantry_dead as unsafe extern "C" fn(*mut edict_t) -> ()),
        };
        init
    }
};
#[no_mangle]
pub static mut infantry_frames_death3: [mframe_t; 9] = unsafe {
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
                dist: -(6 as libc::c_int) as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_move as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: -(11 as libc::c_int) as libc::c_float,
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
                dist: -(11 as libc::c_int) as libc::c_float,
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
pub static mut infantry_move_death3: mmove_t = unsafe {
    {
        let mut init = mmove_t {
            firstframe: 170 as libc::c_int,
            lastframe: 178 as libc::c_int,
            frame: infantry_frames_death3.as_ptr() as *mut _,
            endfunc: Some(infantry_dead as unsafe extern "C" fn(*mut edict_t) -> ()),
        };
        init
    }
};
#[no_mangle]
pub unsafe extern "C" fn infantry_die(
    mut self_0: *mut edict_t,
    mut inflictor: *mut edict_t,
    mut attacker: *mut edict_t,
    mut damage: libc::c_int,
    mut point: *mut vec_t,
) {
    let mut n: libc::c_int = 0;
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
    (*self_0).deadflag = 2 as libc::c_int;
    (*self_0).takedamage = DAMAGE_YES as libc::c_int;
    n = rand() % 3 as libc::c_int;
    if n == 0 as libc::c_int {
        let ref mut fresh7 = (*self_0).monsterinfo.currentmove;
        *fresh7 = &mut infantry_move_death1;
        (gi.sound)
            .expect(
                "non-null function pointer",
            )(
            self_0,
            2 as libc::c_int,
            sound_die2,
            1 as libc::c_int as libc::c_float,
            1 as libc::c_int as libc::c_float,
            0 as libc::c_int as libc::c_float,
        );
    } else if n == 1 as libc::c_int {
        let ref mut fresh8 = (*self_0).monsterinfo.currentmove;
        *fresh8 = &mut infantry_move_death2;
        (gi.sound)
            .expect(
                "non-null function pointer",
            )(
            self_0,
            2 as libc::c_int,
            sound_die1,
            1 as libc::c_int as libc::c_float,
            1 as libc::c_int as libc::c_float,
            0 as libc::c_int as libc::c_float,
        );
    } else {
        let ref mut fresh9 = (*self_0).monsterinfo.currentmove;
        *fresh9 = &mut infantry_move_death3;
        (gi.sound)
            .expect(
                "non-null function pointer",
            )(
            self_0,
            2 as libc::c_int,
            sound_die2,
            1 as libc::c_int as libc::c_float,
            1 as libc::c_int as libc::c_float,
            0 as libc::c_int as libc::c_float,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn infantry_duck_down(mut self_0: *mut edict_t) {
    if (*self_0).monsterinfo.aiflags & 0x800 as libc::c_int != 0 {
        return;
    }
    (*self_0).monsterinfo.aiflags |= 0x800 as libc::c_int;
    let ref mut fresh10 = (*self_0).maxs[2 as libc::c_int as usize];
    *fresh10 -= 32 as libc::c_int as libc::c_float;
    (*self_0).takedamage = DAMAGE_YES as libc::c_int;
    (*self_0).monsterinfo.pausetime = level.time + 1 as libc::c_int as libc::c_float;
    (gi.linkentity).expect("non-null function pointer")(self_0);
}
#[no_mangle]
pub unsafe extern "C" fn infantry_duck_hold(mut self_0: *mut edict_t) {
    if level.time >= (*self_0).monsterinfo.pausetime {
        (*self_0).monsterinfo.aiflags &= !(0x80 as libc::c_int);
    } else {
        (*self_0).monsterinfo.aiflags |= 0x80 as libc::c_int;
    };
}
#[no_mangle]
pub unsafe extern "C" fn infantry_duck_up(mut self_0: *mut edict_t) {
    (*self_0).monsterinfo.aiflags &= !(0x800 as libc::c_int);
    let ref mut fresh11 = (*self_0).maxs[2 as libc::c_int as usize];
    *fresh11 += 32 as libc::c_int as libc::c_float;
    (*self_0).takedamage = DAMAGE_AIM as libc::c_int;
    (gi.linkentity).expect("non-null function pointer")(self_0);
}
#[no_mangle]
pub static mut infantry_frames_duck: [mframe_t; 5] = unsafe {
    [
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_move as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: -(2 as libc::c_int) as libc::c_float,
                thinkfunc: Some(
                    infantry_duck_down as unsafe extern "C" fn(*mut edict_t) -> (),
                ),
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_move as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: -(5 as libc::c_int) as libc::c_float,
                thinkfunc: Some(
                    infantry_duck_hold as unsafe extern "C" fn(*mut edict_t) -> (),
                ),
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
                dist: 4 as libc::c_int as libc::c_float,
                thinkfunc: Some(
                    infantry_duck_up as unsafe extern "C" fn(*mut edict_t) -> (),
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
                thinkfunc: None,
            };
            init
        },
    ]
};
#[no_mangle]
pub static mut infantry_move_duck: mmove_t = unsafe {
    {
        let mut init = mmove_t {
            firstframe: 120 as libc::c_int,
            lastframe: 124 as libc::c_int,
            frame: infantry_frames_duck.as_ptr() as *mut _,
            endfunc: Some(infantry_run as unsafe extern "C" fn(*mut edict_t) -> ()),
        };
        init
    }
};
#[no_mangle]
pub unsafe extern "C" fn infantry_dodge(
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
        let ref mut fresh12 = (*self_0).enemy;
        *fresh12 = attacker;
    }
    let ref mut fresh13 = (*self_0).monsterinfo.currentmove;
    *fresh13 = &mut infantry_move_duck;
}
#[no_mangle]
pub unsafe extern "C" fn infantry_cock_gun(mut self_0: *mut edict_t) {
    let mut n: libc::c_int = 0;
    (gi.sound)
        .expect(
            "non-null function pointer",
        )(
        self_0,
        1 as libc::c_int,
        sound_weapon_cock,
        1 as libc::c_int as libc::c_float,
        1 as libc::c_int as libc::c_float,
        0 as libc::c_int as libc::c_float,
    );
    n = (rand() & 15 as libc::c_int) + 3 as libc::c_int + 7 as libc::c_int;
    (*self_0)
        .monsterinfo
        .pausetime = (level.time as libc::c_double + n as libc::c_double * 0.1f64)
        as libc::c_float;
}
#[no_mangle]
pub unsafe extern "C" fn infantry_fire(mut self_0: *mut edict_t) {
    InfantryMachineGun(self_0);
    if level.time >= (*self_0).monsterinfo.pausetime {
        (*self_0).monsterinfo.aiflags &= !(0x80 as libc::c_int);
    } else {
        (*self_0).monsterinfo.aiflags |= 0x80 as libc::c_int;
    };
}
#[no_mangle]
pub static mut infantry_frames_attack1: [mframe_t; 15] = unsafe {
    [
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
                dist: -(1 as libc::c_int) as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_charge as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: -(1 as libc::c_int) as libc::c_float,
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
                    infantry_cock_gun as unsafe extern "C" fn(*mut edict_t) -> (),
                ),
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_charge as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: -(1 as libc::c_int) as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_charge as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 1 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_charge as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 1 as libc::c_int as libc::c_float,
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
                dist: -(2 as libc::c_int) as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_charge as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: -(3 as libc::c_int) as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_charge as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 1 as libc::c_int as libc::c_float,
                thinkfunc: Some(
                    infantry_fire as unsafe extern "C" fn(*mut edict_t) -> (),
                ),
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
                dist: -(1 as libc::c_int) as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_charge as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: -(2 as libc::c_int) as libc::c_float,
                thinkfunc: None,
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_charge as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: -(3 as libc::c_int) as libc::c_float,
                thinkfunc: None,
            };
            init
        },
    ]
};
#[no_mangle]
pub static mut infantry_move_attack1: mmove_t = unsafe {
    {
        let mut init = mmove_t {
            firstframe: 184 as libc::c_int,
            lastframe: 198 as libc::c_int,
            frame: infantry_frames_attack1.as_ptr() as *mut _,
            endfunc: Some(infantry_run as unsafe extern "C" fn(*mut edict_t) -> ()),
        };
        init
    }
};
#[no_mangle]
pub unsafe extern "C" fn infantry_swing(mut self_0: *mut edict_t) {
    (gi.sound)
        .expect(
            "non-null function pointer",
        )(
        self_0,
        1 as libc::c_int,
        sound_punch_swing,
        1 as libc::c_int as libc::c_float,
        1 as libc::c_int as libc::c_float,
        0 as libc::c_int as libc::c_float,
    );
}
#[no_mangle]
pub unsafe extern "C" fn infantry_smack(mut self_0: *mut edict_t) {
    let mut aim: vec3_t = [0.; 3];
    aim[0 as libc::c_int as usize] = 80 as libc::c_int as vec_t;
    aim[1 as libc::c_int as usize] = 0 as libc::c_int as vec_t;
    aim[2 as libc::c_int as usize] = 0 as libc::c_int as vec_t;
    if fire_hit(
        self_0,
        aim.as_mut_ptr(),
        5 as libc::c_int + rand() % 5 as libc::c_int,
        50 as libc::c_int,
    ) as u64 != 0
    {
        (gi.sound)
            .expect(
                "non-null function pointer",
            )(
            self_0,
            1 as libc::c_int,
            sound_punch_hit,
            1 as libc::c_int as libc::c_float,
            1 as libc::c_int as libc::c_float,
            0 as libc::c_int as libc::c_float,
        );
    }
}
#[no_mangle]
pub static mut infantry_frames_attack2: [mframe_t; 8] = unsafe {
    [
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
                dist: 0 as libc::c_int as libc::c_float,
                thinkfunc: Some(
                    infantry_swing as unsafe extern "C" fn(*mut edict_t) -> (),
                ),
            };
            init
        },
        {
            let mut init = mframe_t {
                aifunc: Some(
                    ai_charge as unsafe extern "C" fn(*mut edict_t, libc::c_float) -> (),
                ),
                dist: 8 as libc::c_int as libc::c_float,
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
                dist: 8 as libc::c_int as libc::c_float,
                thinkfunc: Some(
                    infantry_smack as unsafe extern "C" fn(*mut edict_t) -> (),
                ),
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
                dist: 3 as libc::c_int as libc::c_float,
                thinkfunc: None,
            };
            init
        },
    ]
};
#[no_mangle]
pub static mut infantry_move_attack2: mmove_t = unsafe {
    {
        let mut init = mmove_t {
            firstframe: 199 as libc::c_int,
            lastframe: 206 as libc::c_int,
            frame: infantry_frames_attack2.as_ptr() as *mut _,
            endfunc: Some(infantry_run as unsafe extern "C" fn(*mut edict_t) -> ()),
        };
        init
    }
};
#[no_mangle]
pub unsafe extern "C" fn infantry_attack(mut self_0: *mut edict_t) {
    if range(self_0, (*self_0).enemy) == 0 as libc::c_int {
        let ref mut fresh14 = (*self_0).monsterinfo.currentmove;
        *fresh14 = &mut infantry_move_attack2;
    } else {
        let ref mut fresh15 = (*self_0).monsterinfo.currentmove;
        *fresh15 = &mut infantry_move_attack1;
    };
}
#[no_mangle]
pub unsafe extern "C" fn SP_monster_infantry(mut self_0: *mut edict_t) {
    if (*deathmatch).value != 0. {
        G_FreeEdict(self_0);
        return;
    }
    sound_pain1 = (gi.soundindex)
        .expect(
            "non-null function pointer",
        )(
        b"infantry/infpain1.wav\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    sound_pain2 = (gi.soundindex)
        .expect(
            "non-null function pointer",
        )(
        b"infantry/infpain2.wav\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    sound_die1 = (gi.soundindex)
        .expect(
            "non-null function pointer",
        )(
        b"infantry/infdeth1.wav\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    sound_die2 = (gi.soundindex)
        .expect(
            "non-null function pointer",
        )(
        b"infantry/infdeth2.wav\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    sound_gunshot = (gi.soundindex)
        .expect(
            "non-null function pointer",
        )(
        b"infantry/infatck1.wav\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    sound_weapon_cock = (gi.soundindex)
        .expect(
            "non-null function pointer",
        )(
        b"infantry/infatck3.wav\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    sound_punch_swing = (gi.soundindex)
        .expect(
            "non-null function pointer",
        )(
        b"infantry/infatck2.wav\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    sound_punch_hit = (gi.soundindex)
        .expect(
            "non-null function pointer",
        )(
        b"infantry/melee2.wav\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    sound_sight = (gi.soundindex)
        .expect(
            "non-null function pointer",
        )(
        b"infantry/infsght1.wav\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    sound_search = (gi.soundindex)
        .expect(
            "non-null function pointer",
        )(
        b"infantry/infsrch1.wav\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    sound_idle = (gi.soundindex)
        .expect(
            "non-null function pointer",
        )(
        b"infantry/infidle1.wav\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    (*self_0).movetype = MOVETYPE_STEP as libc::c_int;
    (*self_0).solid = SOLID_BBOX;
    (*self_0)
        .s
        .modelindex = (gi.modelindex)
        .expect(
            "non-null function pointer",
        )(
        b"models/monsters/infantry/tris.md2\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    (*self_0).mins[0 as libc::c_int as usize] = -(16 as libc::c_int) as vec_t;
    (*self_0).mins[1 as libc::c_int as usize] = -(16 as libc::c_int) as vec_t;
    (*self_0).mins[2 as libc::c_int as usize] = -(24 as libc::c_int) as vec_t;
    (*self_0).maxs[0 as libc::c_int as usize] = 16 as libc::c_int as vec_t;
    (*self_0).maxs[1 as libc::c_int as usize] = 16 as libc::c_int as vec_t;
    (*self_0).maxs[2 as libc::c_int as usize] = 32 as libc::c_int as vec_t;
    (*self_0).health = 100 as libc::c_int;
    (*self_0).gib_health = -(40 as libc::c_int);
    (*self_0).mass = 200 as libc::c_int;
    let ref mut fresh16 = (*self_0).pain;
    *fresh16 = Some(
        infantry_pain
            as unsafe extern "C" fn(
                *mut edict_t,
                *mut edict_t,
                libc::c_float,
                libc::c_int,
            ) -> (),
    );
    let ref mut fresh17 = (*self_0).die;
    *fresh17 = Some(
        infantry_die
            as unsafe extern "C" fn(
                *mut edict_t,
                *mut edict_t,
                *mut edict_t,
                libc::c_int,
                *mut vec_t,
            ) -> (),
    );
    let ref mut fresh18 = (*self_0).monsterinfo.stand;
    *fresh18 = Some(infantry_stand as unsafe extern "C" fn(*mut edict_t) -> ());
    let ref mut fresh19 = (*self_0).monsterinfo.walk;
    *fresh19 = Some(infantry_walk as unsafe extern "C" fn(*mut edict_t) -> ());
    let ref mut fresh20 = (*self_0).monsterinfo.run;
    *fresh20 = Some(infantry_run as unsafe extern "C" fn(*mut edict_t) -> ());
    let ref mut fresh21 = (*self_0).monsterinfo.dodge;
    *fresh21 = Some(
        infantry_dodge
            as unsafe extern "C" fn(*mut edict_t, *mut edict_t, libc::c_float) -> (),
    );
    let ref mut fresh22 = (*self_0).monsterinfo.attack;
    *fresh22 = Some(infantry_attack as unsafe extern "C" fn(*mut edict_t) -> ());
    let ref mut fresh23 = (*self_0).monsterinfo.melee;
    *fresh23 = None;
    let ref mut fresh24 = (*self_0).monsterinfo.sight;
    *fresh24 = Some(
        infantry_sight as unsafe extern "C" fn(*mut edict_t, *mut edict_t) -> (),
    );
    let ref mut fresh25 = (*self_0).monsterinfo.idle;
    *fresh25 = Some(infantry_fidget as unsafe extern "C" fn(*mut edict_t) -> ());
    (gi.linkentity).expect("non-null function pointer")(self_0);
    let ref mut fresh26 = (*self_0).monsterinfo.currentmove;
    *fresh26 = &mut infantry_move_stand;
    (*self_0).monsterinfo.scale = 1.000000f64 as libc::c_float;
    walkmonster_start(self_0);
}
