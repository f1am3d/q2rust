#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    fn GetChaseTarget(ent: *mut edict_t);
    fn ChaseNext(ent: *mut edict_t);
    fn UpdateChaseCam(ent: *mut edict_t);
    fn PlayerNoise(who: *mut edict_t, where_0: *mut vec_t, type_0: libc::c_int);
    fn MoveClientToIntermission(client: *mut edict_t);
    fn ClientEndServerFrame(ent: *mut edict_t);
    fn SV_FilterPacket(from: *mut libc::c_char) -> qboolean;
    fn PlayerTrail_LastSpot() -> *mut edict_t;
    fn PlayerTrail_Add(spot: *mut vec_t);
    fn visible(self_0: *mut edict_t, other: *mut edict_t) -> qboolean;
    fn ThrowGib(
        self_0: *mut edict_t,
        gibname: *mut libc::c_char,
        damage: libc::c_int,
        type_0: libc::c_int,
    );
    fn ThrowClientHead(self_0: *mut edict_t, damage: libc::c_int);
    fn atan2(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn atoi(__nptr: *const libc::c_char) -> libc::c_int;
    fn rand() -> libc::c_int;
    fn VectorLength(v: *mut vec_t) -> vec_t;
    fn Q_stricmp(s1: *mut libc::c_char, s2: *mut libc::c_char) -> libc::c_int;
    fn va(format: *mut libc::c_char, _: ...) -> *mut libc::c_char;
    fn Info_ValueForKey(
        s: *mut libc::c_char,
        key: *mut libc::c_char,
    ) -> *mut libc::c_char;
    fn Info_SetValueForKey(
        s: *mut libc::c_char,
        key: *mut libc::c_char,
        value: *mut libc::c_char,
    );
    fn Info_Validate(s: *mut libc::c_char) -> qboolean;
    fn Com_Printf(msg: *mut libc::c_char, _: ...);
    static mut game: game_locals_t;
    static mut level: level_locals_t;
    static mut gi: game_import_t;
    static mut meansOfDeath: libc::c_int;
    static mut g_edicts: *mut edict_t;
    static mut deathmatch: *mut cvar_t;
    static mut coop: *mut cvar_t;
    static mut dmflags: *mut cvar_t;
    static mut password: *mut cvar_t;
    static mut spectator_password: *mut cvar_t;
    static mut sv_gravity: *mut cvar_t;
    static mut maxclients: *mut cvar_t;
    static mut maxspectators: *mut cvar_t;
    static mut itemlist: [gitem_t; 0];
    fn Cmd_Help_f(ent: *mut edict_t);
    fn FindItem(pickup_name: *mut libc::c_char) -> *mut gitem_t;
    fn FindItemByClassname(classname: *mut libc::c_char) -> *mut gitem_t;
    fn Drop_Item(ent: *mut edict_t, item: *mut gitem_t) -> *mut edict_t;
    fn ChangeWeapon(ent: *mut edict_t);
    fn Think_Weapon(ent: *mut edict_t);
    fn Touch_Item(
        ent: *mut edict_t,
        other: *mut edict_t,
        plane: *mut cplane_t,
        surf: *mut csurface_t,
    );
    fn KillBox(ent: *mut edict_t) -> qboolean;
    fn G_Find(
        from: *mut edict_t,
        fieldofs: libc::c_int,
        match_0: *mut libc::c_char,
    ) -> *mut edict_t;
    fn G_InitEdict(e: *mut edict_t);
    fn G_Spawn() -> *mut edict_t;
    fn G_FreeEdict(e: *mut edict_t);
    fn G_TouchTriggers(ent: *mut edict_t);
    fn SP_misc_teleporter_dest(ent: *mut edict_t);
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
pub const EV_OTHER_TELEPORT: C2RustUnnamed = 7;
pub const EV_PLAYER_TELEPORT: C2RustUnnamed = 6;
pub const EV_FALLFAR: C2RustUnnamed = 5;
pub const EV_FALL: C2RustUnnamed = 4;
pub const EV_FALLSHORT: C2RustUnnamed = 3;
pub const EV_FOOTSTEP: C2RustUnnamed = 2;
pub const EV_ITEM_RESPAWN: C2RustUnnamed = 1;
pub const EV_NONE: C2RustUnnamed = 0;
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
unsafe extern "C" fn SP_FixCoopSpots(mut self_0: *mut edict_t) {
    let mut spot: *mut edict_t = 0 as *mut edict_t;
    let mut d: vec3_t = [0.; 3];
    spot = 0 as *mut edict_t;
    loop {
        spot = G_Find(
            spot,
            &mut (*(0 as *mut edict_t)).classname as *mut *mut libc::c_char
                as libc::c_int,
            b"info_player_start\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        if spot.is_null() {
            return;
        }
        if ((*spot).targetname).is_null() {
            continue;
        }
        d[0 as libc::c_int
            as usize] = (*self_0).s.origin[0 as libc::c_int as usize]
            - (*spot).s.origin[0 as libc::c_int as usize];
        d[1 as libc::c_int
            as usize] = (*self_0).s.origin[1 as libc::c_int as usize]
            - (*spot).s.origin[1 as libc::c_int as usize];
        d[2 as libc::c_int
            as usize] = (*self_0).s.origin[2 as libc::c_int as usize]
            - (*spot).s.origin[2 as libc::c_int as usize];
        if VectorLength(d.as_mut_ptr()) < 384 as libc::c_int as libc::c_float {
            if ((*self_0).targetname).is_null()
                || Q_stricmp((*self_0).targetname, (*spot).targetname)
                    != 0 as libc::c_int
            {
                let ref mut fresh0 = (*self_0).targetname;
                *fresh0 = (*spot).targetname;
            }
            return;
        }
    };
}
unsafe extern "C" fn SP_CreateCoopSpots(mut self_0: *mut edict_t) {
    let mut spot: *mut edict_t = 0 as *mut edict_t;
    if Q_stricmp(
        (level.mapname).as_mut_ptr(),
        b"security\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) == 0 as libc::c_int
    {
        spot = G_Spawn();
        let ref mut fresh1 = (*spot).classname;
        *fresh1 = b"info_player_coop\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char;
        (*spot)
            .s
            .origin[0 as libc::c_int
            as usize] = (188 as libc::c_int - 64 as libc::c_int) as vec_t;
        (*spot).s.origin[1 as libc::c_int as usize] = -(164 as libc::c_int) as vec_t;
        (*spot).s.origin[2 as libc::c_int as usize] = 80 as libc::c_int as vec_t;
        let ref mut fresh2 = (*spot).targetname;
        *fresh2 = b"jail3\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        (*spot).s.angles[1 as libc::c_int as usize] = 90 as libc::c_int as vec_t;
        spot = G_Spawn();
        let ref mut fresh3 = (*spot).classname;
        *fresh3 = b"info_player_coop\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char;
        (*spot)
            .s
            .origin[0 as libc::c_int
            as usize] = (188 as libc::c_int + 64 as libc::c_int) as vec_t;
        (*spot).s.origin[1 as libc::c_int as usize] = -(164 as libc::c_int) as vec_t;
        (*spot).s.origin[2 as libc::c_int as usize] = 80 as libc::c_int as vec_t;
        let ref mut fresh4 = (*spot).targetname;
        *fresh4 = b"jail3\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        (*spot).s.angles[1 as libc::c_int as usize] = 90 as libc::c_int as vec_t;
        spot = G_Spawn();
        let ref mut fresh5 = (*spot).classname;
        *fresh5 = b"info_player_coop\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char;
        (*spot)
            .s
            .origin[0 as libc::c_int
            as usize] = (188 as libc::c_int + 128 as libc::c_int) as vec_t;
        (*spot).s.origin[1 as libc::c_int as usize] = -(164 as libc::c_int) as vec_t;
        (*spot).s.origin[2 as libc::c_int as usize] = 80 as libc::c_int as vec_t;
        let ref mut fresh6 = (*spot).targetname;
        *fresh6 = b"jail3\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        (*spot).s.angles[1 as libc::c_int as usize] = 90 as libc::c_int as vec_t;
        return;
    }
}
#[no_mangle]
pub unsafe extern "C" fn SP_info_player_start(mut self_0: *mut edict_t) {
    if (*coop).value == 0. {
        return;
    }
    if Q_stricmp(
        (level.mapname).as_mut_ptr(),
        b"security\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) == 0 as libc::c_int
    {
        let ref mut fresh7 = (*self_0).think;
        *fresh7 = Some(SP_CreateCoopSpots as unsafe extern "C" fn(*mut edict_t) -> ());
        (*self_0).nextthink = (level.time as libc::c_double + 0.1f64) as libc::c_float;
    }
}
#[no_mangle]
pub unsafe extern "C" fn SP_info_player_deathmatch(mut self_0: *mut edict_t) {
    if (*deathmatch).value == 0. {
        G_FreeEdict(self_0);
        return;
    }
    SP_misc_teleporter_dest(self_0);
}
#[no_mangle]
pub unsafe extern "C" fn SP_info_player_coop(mut self_0: *mut edict_t) {
    if (*coop).value == 0. {
        G_FreeEdict(self_0);
        return;
    }
    if Q_stricmp(
        (level.mapname).as_mut_ptr(),
        b"jail2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) == 0 as libc::c_int
        || Q_stricmp(
            (level.mapname).as_mut_ptr(),
            b"jail4\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) == 0 as libc::c_int
        || Q_stricmp(
            (level.mapname).as_mut_ptr(),
            b"mine1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) == 0 as libc::c_int
        || Q_stricmp(
            (level.mapname).as_mut_ptr(),
            b"mine2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) == 0 as libc::c_int
        || Q_stricmp(
            (level.mapname).as_mut_ptr(),
            b"mine3\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) == 0 as libc::c_int
        || Q_stricmp(
            (level.mapname).as_mut_ptr(),
            b"mine4\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) == 0 as libc::c_int
        || Q_stricmp(
            (level.mapname).as_mut_ptr(),
            b"lab\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) == 0 as libc::c_int
        || Q_stricmp(
            (level.mapname).as_mut_ptr(),
            b"boss1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) == 0 as libc::c_int
        || Q_stricmp(
            (level.mapname).as_mut_ptr(),
            b"fact3\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) == 0 as libc::c_int
        || Q_stricmp(
            (level.mapname).as_mut_ptr(),
            b"biggun\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) == 0 as libc::c_int
        || Q_stricmp(
            (level.mapname).as_mut_ptr(),
            b"space\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) == 0 as libc::c_int
        || Q_stricmp(
            (level.mapname).as_mut_ptr(),
            b"command\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) == 0 as libc::c_int
        || Q_stricmp(
            (level.mapname).as_mut_ptr(),
            b"power2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) == 0 as libc::c_int
        || Q_stricmp(
            (level.mapname).as_mut_ptr(),
            b"strike\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) == 0 as libc::c_int
    {
        let ref mut fresh8 = (*self_0).think;
        *fresh8 = Some(SP_FixCoopSpots as unsafe extern "C" fn(*mut edict_t) -> ());
        (*self_0).nextthink = (level.time as libc::c_double + 0.1f64) as libc::c_float;
    }
}
#[no_mangle]
pub unsafe extern "C" fn SP_info_player_intermission() {}
#[no_mangle]
pub unsafe extern "C" fn player_pain(
    mut self_0: *mut edict_t,
    mut other: *mut edict_t,
    mut kick: libc::c_float,
    mut damage: libc::c_int,
) {}
#[no_mangle]
pub unsafe extern "C" fn IsFemale(mut ent: *mut edict_t) -> qboolean {
    let mut info: *mut libc::c_char = 0 as *mut libc::c_char;
    if ((*ent).client).is_null() {
        return false_0;
    }
    info = Info_ValueForKey(
        ((*(*ent).client).pers.userinfo).as_mut_ptr(),
        b"gender\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if *info.offset(0 as libc::c_int as isize) as libc::c_int == 'f' as i32
        || *info.offset(0 as libc::c_int as isize) as libc::c_int == 'F' as i32
    {
        return true_0;
    }
    return false_0;
}
#[no_mangle]
pub unsafe extern "C" fn IsNeutral(mut ent: *mut edict_t) -> qboolean {
    let mut info: *mut libc::c_char = 0 as *mut libc::c_char;
    if ((*ent).client).is_null() {
        return false_0;
    }
    info = Info_ValueForKey(
        ((*(*ent).client).pers.userinfo).as_mut_ptr(),
        b"gender\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if *info.offset(0 as libc::c_int as isize) as libc::c_int != 'f' as i32
        && *info.offset(0 as libc::c_int as isize) as libc::c_int != 'F' as i32
        && *info.offset(0 as libc::c_int as isize) as libc::c_int != 'm' as i32
        && *info.offset(0 as libc::c_int as isize) as libc::c_int != 'M' as i32
    {
        return true_0;
    }
    return false_0;
}
#[no_mangle]
pub unsafe extern "C" fn ClientObituary(
    mut self_0: *mut edict_t,
    mut inflictor: *mut edict_t,
    mut attacker: *mut edict_t,
) {
    let mut mod_0: libc::c_int = 0;
    let mut message: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut message2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ff: qboolean = false_0;
    if (*coop).value != 0. && !((*attacker).client).is_null() {
        meansOfDeath |= 0x8000000 as libc::c_int;
    }
    if (*deathmatch).value != 0. || (*coop).value != 0. {
        ff = (meansOfDeath & 0x8000000 as libc::c_int) as qboolean;
        mod_0 = meansOfDeath & !(0x8000000 as libc::c_int);
        message = 0 as *mut libc::c_char;
        message2 = b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        match mod_0 {
            23 => {
                message = b"suicides\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char;
            }
            22 => {
                message = b"cratered\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char;
            }
            20 => {
                message = b"was squished\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char;
            }
            17 => {
                message = b"sank like a rock\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char;
            }
            18 => {
                message = b"melted\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char;
            }
            19 => {
                message = b"does a back flip into the lava\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char;
            }
            25 | 26 => {
                message = b"blew up\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char;
            }
            28 => {
                message = b"found a way out\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char;
            }
            30 => {
                message = b"saw the light\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char;
            }
            33 => {
                message = b"got blasted\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char;
            }
            27 | 29 | 31 => {
                message = b"was in the wrong place\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char;
            }
            _ => {}
        }
        if attacker == self_0 {
            match mod_0 {
                24 => {
                    message = b"tried to put the pin back in\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char;
                }
                16 | 7 => {
                    if IsNeutral(self_0) as u64 != 0 {
                        message = b"tripped on its own grenade\0" as *const u8
                            as *const libc::c_char as *mut libc::c_char;
                    } else if IsFemale(self_0) as u64 != 0 {
                        message = b"tripped on her own grenade\0" as *const u8
                            as *const libc::c_char as *mut libc::c_char;
                    } else {
                        message = b"tripped on his own grenade\0" as *const u8
                            as *const libc::c_char as *mut libc::c_char;
                    }
                }
                9 => {
                    if IsNeutral(self_0) as u64 != 0 {
                        message = b"blew itself up\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char;
                    } else if IsFemale(self_0) as u64 != 0 {
                        message = b"blew herself up\0" as *const u8
                            as *const libc::c_char as *mut libc::c_char;
                    } else {
                        message = b"blew himself up\0" as *const u8
                            as *const libc::c_char as *mut libc::c_char;
                    }
                }
                13 => {
                    message = b"should have used a smaller gun\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char;
                }
                _ => {
                    if IsNeutral(self_0) as u64 != 0 {
                        message = b"killed itself\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char;
                    } else if IsFemale(self_0) as u64 != 0 {
                        message = b"killed herself\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char;
                    } else {
                        message = b"killed himself\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char;
                    }
                }
            }
        }
        if !message.is_null() {
            (gi.bprintf)
                .expect(
                    "non-null function pointer",
                )(
                1 as libc::c_int,
                b"%s %s.\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                ((*(*self_0).client).pers.netname).as_mut_ptr(),
                message,
            );
            if (*deathmatch).value != 0. {
                let ref mut fresh9 = (*(*self_0).client).resp.score;
                *fresh9 -= 1;
            }
            let ref mut fresh10 = (*self_0).enemy;
            *fresh10 = 0 as *mut edict_t;
            return;
        }
        let ref mut fresh11 = (*self_0).enemy;
        *fresh11 = attacker;
        if !attacker.is_null() && !((*attacker).client).is_null() {
            match mod_0 {
                1 => {
                    message = b"was blasted by\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char;
                }
                2 => {
                    message = b"was gunned down by\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char;
                }
                3 => {
                    message = b"was blown away by\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char;
                    message2 = b"'s super shotgun\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char;
                }
                4 => {
                    message = b"was machinegunned by\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char;
                }
                5 => {
                    message = b"was cut in half by\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char;
                    message2 = b"'s chaingun\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char;
                }
                6 => {
                    message = b"was popped by\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char;
                    message2 = b"'s grenade\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char;
                }
                7 => {
                    message = b"was shredded by\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char;
                    message2 = b"'s shrapnel\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char;
                }
                8 => {
                    message = b"ate\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char;
                    message2 = b"'s rocket\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char;
                }
                9 => {
                    message = b"almost dodged\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char;
                    message2 = b"'s rocket\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char;
                }
                10 => {
                    message = b"was melted by\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char;
                    message2 = b"'s hyperblaster\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char;
                }
                11 => {
                    message = b"was railed by\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char;
                }
                12 => {
                    message = b"saw the pretty lights from\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char;
                    message2 = b"'s BFG\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char;
                }
                13 => {
                    message = b"was disintegrated by\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char;
                    message2 = b"'s BFG blast\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char;
                }
                14 => {
                    message = b"couldn't hide from\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char;
                    message2 = b"'s BFG\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char;
                }
                15 => {
                    message = b"caught\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char;
                    message2 = b"'s handgrenade\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char;
                }
                16 => {
                    message = b"didn't see\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char;
                    message2 = b"'s handgrenade\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char;
                }
                24 => {
                    message = b"feels\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char;
                    message2 = b"'s pain\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char;
                }
                21 => {
                    message = b"tried to invade\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char;
                    message2 = b"'s personal space\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char;
                }
                _ => {}
            }
            if !message.is_null() {
                (gi.bprintf)
                    .expect(
                        "non-null function pointer",
                    )(
                    1 as libc::c_int,
                    b"%s %s %s%s\n\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    ((*(*self_0).client).pers.netname).as_mut_ptr(),
                    message,
                    ((*(*attacker).client).pers.netname).as_mut_ptr(),
                    message2,
                );
                if (*deathmatch).value != 0. {
                    if ff as u64 != 0 {
                        let ref mut fresh12 = (*(*attacker).client).resp.score;
                        *fresh12 -= 1;
                    } else {
                        let ref mut fresh13 = (*(*attacker).client).resp.score;
                        *fresh13 += 1;
                    }
                }
                return;
            }
        }
    }
    (gi.bprintf)
        .expect(
            "non-null function pointer",
        )(
        1 as libc::c_int,
        b"%s died.\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ((*(*self_0).client).pers.netname).as_mut_ptr(),
    );
    if (*deathmatch).value != 0. {
        let ref mut fresh14 = (*(*self_0).client).resp.score;
        *fresh14 -= 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn TossClientWeapon(mut self_0: *mut edict_t) {
    let mut item: *mut gitem_t = 0 as *mut gitem_t;
    let mut drop_0: *mut edict_t = 0 as *mut edict_t;
    let mut quad: qboolean = false_0;
    let mut spread: libc::c_float = 0.;
    if (*deathmatch).value == 0. {
        return;
    }
    item = (*(*self_0).client).pers.weapon;
    if (*(*self_0).client).pers.inventory[(*(*self_0).client).ammo_index as usize] == 0 {
        item = 0 as *mut gitem_t;
    }
    if !item.is_null()
        && strcmp((*item).pickup_name, b"Blaster\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
    {
        item = 0 as *mut gitem_t;
    }
    if (*dmflags).value as libc::c_int & 0x4000 as libc::c_int == 0 {
        quad = false_0;
    } else {
        quad = ((*(*self_0).client).quad_framenum
            > (level.framenum + 10 as libc::c_int) as libc::c_float) as libc::c_int
            as qboolean;
    }
    if !item.is_null() && quad as libc::c_uint != 0 {
        spread = 22.5f64 as libc::c_float;
    } else {
        spread = 0.0f64 as libc::c_float;
    }
    if !item.is_null() {
        let ref mut fresh15 = (*(*self_0).client).v_angle[1 as libc::c_int as usize];
        *fresh15 -= spread;
        drop_0 = Drop_Item(self_0, item);
        let ref mut fresh16 = (*(*self_0).client).v_angle[1 as libc::c_int as usize];
        *fresh16 += spread;
        (*drop_0).spawnflags = 0x20000 as libc::c_int;
    }
    if quad as u64 != 0 {
        let ref mut fresh17 = (*(*self_0).client).v_angle[1 as libc::c_int as usize];
        *fresh17 += spread;
        drop_0 = Drop_Item(
            self_0,
            FindItemByClassname(
                b"item_quad\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ),
        );
        let ref mut fresh18 = (*(*self_0).client).v_angle[1 as libc::c_int as usize];
        *fresh18 -= spread;
        (*drop_0).spawnflags |= 0x20000 as libc::c_int;
        let ref mut fresh19 = (*drop_0).touch;
        *fresh19 = Some(
            Touch_Item
                as unsafe extern "C" fn(
                    *mut edict_t,
                    *mut edict_t,
                    *mut cplane_t,
                    *mut csurface_t,
                ) -> (),
        );
        (*drop_0)
            .nextthink = (level.time as libc::c_double
            + ((*(*self_0).client).quad_framenum - level.framenum as libc::c_float)
                as libc::c_double * 0.1f64) as libc::c_float;
        let ref mut fresh20 = (*drop_0).think;
        *fresh20 = Some(G_FreeEdict as unsafe extern "C" fn(*mut edict_t) -> ());
    }
}
#[no_mangle]
pub unsafe extern "C" fn LookAtKiller(
    mut self_0: *mut edict_t,
    mut inflictor: *mut edict_t,
    mut attacker: *mut edict_t,
) {
    let mut dir: vec3_t = [0.; 3];
    if !attacker.is_null()
        && attacker != &mut *g_edicts.offset(0 as libc::c_int as isize) as *mut edict_t
        && attacker != self_0
    {
        dir[0 as libc::c_int
            as usize] = (*attacker).s.origin[0 as libc::c_int as usize]
            - (*self_0).s.origin[0 as libc::c_int as usize];
        dir[1 as libc::c_int
            as usize] = (*attacker).s.origin[1 as libc::c_int as usize]
            - (*self_0).s.origin[1 as libc::c_int as usize];
        dir[2 as libc::c_int
            as usize] = (*attacker).s.origin[2 as libc::c_int as usize]
            - (*self_0).s.origin[2 as libc::c_int as usize];
    } else if !inflictor.is_null()
        && inflictor != &mut *g_edicts.offset(0 as libc::c_int as isize) as *mut edict_t
        && inflictor != self_0
    {
        dir[0 as libc::c_int
            as usize] = (*inflictor).s.origin[0 as libc::c_int as usize]
            - (*self_0).s.origin[0 as libc::c_int as usize];
        dir[1 as libc::c_int
            as usize] = (*inflictor).s.origin[1 as libc::c_int as usize]
            - (*self_0).s.origin[1 as libc::c_int as usize];
        dir[2 as libc::c_int
            as usize] = (*inflictor).s.origin[2 as libc::c_int as usize]
            - (*self_0).s.origin[2 as libc::c_int as usize];
    } else {
        (*(*self_0).client).killer_yaw = (*self_0).s.angles[1 as libc::c_int as usize];
        return;
    }
    if dir[0 as libc::c_int as usize] != 0. {
        (*(*self_0).client)
            .killer_yaw = (180 as libc::c_int as libc::c_double
            / 3.14159265358979323846f64
            * atan2(
                dir[1 as libc::c_int as usize] as libc::c_double,
                dir[0 as libc::c_int as usize] as libc::c_double,
            )) as libc::c_float;
    } else {
        (*(*self_0).client).killer_yaw = 0 as libc::c_int as libc::c_float;
        if dir[1 as libc::c_int as usize] > 0 as libc::c_int as libc::c_float {
            (*(*self_0).client).killer_yaw = 90 as libc::c_int as libc::c_float;
        } else if dir[1 as libc::c_int as usize] < 0 as libc::c_int as libc::c_float {
            (*(*self_0).client).killer_yaw = -(90 as libc::c_int) as libc::c_float;
        }
    }
    if (*(*self_0).client).killer_yaw < 0 as libc::c_int as libc::c_float {
        (*(*self_0).client).killer_yaw += 360 as libc::c_int as libc::c_float;
    }
}
#[no_mangle]
pub unsafe extern "C" fn player_die(
    mut self_0: *mut edict_t,
    mut inflictor: *mut edict_t,
    mut attacker: *mut edict_t,
    mut damage: libc::c_int,
    mut point: *mut vec_t,
) {
    let mut n: libc::c_int = 0;
    let ref mut fresh21 = (*self_0).avelocity[2 as libc::c_int as usize];
    *fresh21 = 0 as libc::c_int as vec_t;
    let ref mut fresh22 = (*self_0).avelocity[1 as libc::c_int as usize];
    *fresh22 = *fresh21;
    (*self_0).avelocity[0 as libc::c_int as usize] = *fresh22;
    (*self_0).takedamage = DAMAGE_YES as libc::c_int;
    (*self_0).movetype = MOVETYPE_TOSS as libc::c_int;
    (*self_0).s.modelindex2 = 0 as libc::c_int;
    (*self_0).s.angles[0 as libc::c_int as usize] = 0 as libc::c_int as vec_t;
    (*self_0).s.angles[2 as libc::c_int as usize] = 0 as libc::c_int as vec_t;
    (*self_0).s.sound = 0 as libc::c_int;
    (*(*self_0).client).weapon_sound = 0 as libc::c_int;
    (*self_0).maxs[2 as libc::c_int as usize] = -(8 as libc::c_int) as vec_t;
    (*self_0).svflags |= 0x2 as libc::c_int;
    if (*self_0).deadflag == 0 {
        (*(*self_0).client)
            .respawn_time = (level.time as libc::c_double + 1.0f64) as libc::c_float;
        LookAtKiller(self_0, inflictor, attacker);
        (*(*self_0).client).ps.pmove.pm_type = PM_DEAD;
        ClientObituary(self_0, inflictor, attacker);
        TossClientWeapon(self_0);
        if (*deathmatch).value != 0. {
            Cmd_Help_f(self_0);
        }
        n = 0 as libc::c_int;
        while n < game.num_items {
            if (*coop).value != 0.
                && (*itemlist.as_mut_ptr().offset(n as isize)).flags & 16 as libc::c_int
                    != 0
            {
                (*(*self_0).client)
                    .resp
                    .coop_respawn
                    .inventory[n
                    as usize] = (*(*self_0).client).pers.inventory[n as usize];
            }
            (*(*self_0).client).pers.inventory[n as usize] = 0 as libc::c_int;
            n += 1;
        }
    }
    (*(*self_0).client).quad_framenum = 0 as libc::c_int as libc::c_float;
    (*(*self_0).client).invincible_framenum = 0 as libc::c_int as libc::c_float;
    (*(*self_0).client).breather_framenum = 0 as libc::c_int as libc::c_float;
    (*(*self_0).client).enviro_framenum = 0 as libc::c_int as libc::c_float;
    (*self_0).flags &= !(0x1000 as libc::c_int);
    if (*self_0).health < -(40 as libc::c_int) {
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
                b"misc/udeath.wav\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            ),
            1 as libc::c_int as libc::c_float,
            1 as libc::c_int as libc::c_float,
            0 as libc::c_int as libc::c_float,
        );
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
        ThrowClientHead(self_0, damage);
        (*self_0).takedamage = DAMAGE_NO as libc::c_int;
    } else if (*self_0).deadflag == 0 {
        static mut i: libc::c_int = 0;
        i = (i + 1 as libc::c_int) % 3 as libc::c_int;
        (*(*self_0).client).anim_priority = 5 as libc::c_int;
        if (*(*self_0).client).ps.pmove.pm_flags as libc::c_int & 1 as libc::c_int != 0 {
            (*self_0).s.frame = 173 as libc::c_int - 1 as libc::c_int;
            (*(*self_0).client).anim_end = 177 as libc::c_int;
        } else {
            match i {
                0 => {
                    (*self_0).s.frame = 178 as libc::c_int - 1 as libc::c_int;
                    (*(*self_0).client).anim_end = 183 as libc::c_int;
                }
                1 => {
                    (*self_0).s.frame = 184 as libc::c_int - 1 as libc::c_int;
                    (*(*self_0).client).anim_end = 189 as libc::c_int;
                }
                2 => {
                    (*self_0).s.frame = 190 as libc::c_int - 1 as libc::c_int;
                    (*(*self_0).client).anim_end = 197 as libc::c_int;
                }
                _ => {}
            }
        }
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
                va(
                    b"*death%i.wav\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    rand() % 4 as libc::c_int + 1 as libc::c_int,
                ),
            ),
            1 as libc::c_int as libc::c_float,
            1 as libc::c_int as libc::c_float,
            0 as libc::c_int as libc::c_float,
        );
    }
    (*self_0).deadflag = 2 as libc::c_int;
    (gi.linkentity).expect("non-null function pointer")(self_0);
}
#[no_mangle]
pub unsafe extern "C" fn InitClientPersistant(mut client: *mut gclient_t) {
    let mut item: *mut gitem_t = 0 as *mut gitem_t;
    memset(
        &mut (*client).pers as *mut client_persistant_t as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<client_persistant_t>() as libc::c_ulong,
    );
    item = FindItem(
        b"Blaster\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    (*client)
        .pers
        .selected_item = item.offset_from(itemlist.as_mut_ptr()) as libc::c_long
        as libc::c_int;
    (*client).pers.inventory[(*client).pers.selected_item as usize] = 1 as libc::c_int;
    let ref mut fresh23 = (*client).pers.weapon;
    *fresh23 = item;
    (*client).pers.health = 100 as libc::c_int;
    (*client).pers.max_health = 100 as libc::c_int;
    (*client).pers.max_bullets = 200 as libc::c_int;
    (*client).pers.max_shells = 100 as libc::c_int;
    (*client).pers.max_rockets = 50 as libc::c_int;
    (*client).pers.max_grenades = 50 as libc::c_int;
    (*client).pers.max_cells = 200 as libc::c_int;
    (*client).pers.max_slugs = 50 as libc::c_int;
    (*client).pers.connected = true_0;
}
#[no_mangle]
pub unsafe extern "C" fn InitClientResp(mut client: *mut gclient_t) {
    memset(
        &mut (*client).resp as *mut client_respawn_t as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<client_respawn_t>() as libc::c_ulong,
    );
    (*client).resp.enterframe = level.framenum;
    (*client).resp.coop_respawn = (*client).pers;
}
#[no_mangle]
pub unsafe extern "C" fn SaveClientData() {
    let mut i: libc::c_int = 0;
    let mut ent: *mut edict_t = 0 as *mut edict_t;
    i = 0 as libc::c_int;
    while i < game.maxclients {
        ent = &mut *g_edicts.offset((1 as libc::c_int + i) as isize) as *mut edict_t;
        if !((*ent).inuse as u64 == 0) {
            (*(game.clients).offset(i as isize)).pers.health = (*ent).health;
            (*(game.clients).offset(i as isize)).pers.max_health = (*ent).max_health;
            (*(game.clients).offset(i as isize))
                .pers
                .savedFlags = (*ent).flags
                & (0x10 as libc::c_int | 0x20 as libc::c_int | 0x1000 as libc::c_int);
            if (*coop).value != 0. {
                (*(game.clients).offset(i as isize))
                    .pers
                    .score = (*(*ent).client).resp.score;
            }
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn FetchClientEntData(mut ent: *mut edict_t) {
    (*ent).health = (*(*ent).client).pers.health;
    (*ent).max_health = (*(*ent).client).pers.max_health;
    (*ent).flags |= (*(*ent).client).pers.savedFlags;
    if (*coop).value != 0. {
        (*(*ent).client).resp.score = (*(*ent).client).pers.score;
    }
}
#[no_mangle]
pub unsafe extern "C" fn PlayersRangeFromSpot(mut spot: *mut edict_t) -> libc::c_float {
    let mut player: *mut edict_t = 0 as *mut edict_t;
    let mut bestplayerdistance: libc::c_float = 0.;
    let mut v: vec3_t = [0.; 3];
    let mut n: libc::c_int = 0;
    let mut playerdistance: libc::c_float = 0.;
    bestplayerdistance = 9999999 as libc::c_int as libc::c_float;
    n = 1 as libc::c_int;
    while n as libc::c_float <= (*maxclients).value {
        player = &mut *g_edicts.offset(n as isize) as *mut edict_t;
        if !((*player).inuse as u64 == 0) {
            if !((*player).health <= 0 as libc::c_int) {
                v[0 as libc::c_int
                    as usize] = (*spot).s.origin[0 as libc::c_int as usize]
                    - (*player).s.origin[0 as libc::c_int as usize];
                v[1 as libc::c_int
                    as usize] = (*spot).s.origin[1 as libc::c_int as usize]
                    - (*player).s.origin[1 as libc::c_int as usize];
                v[2 as libc::c_int
                    as usize] = (*spot).s.origin[2 as libc::c_int as usize]
                    - (*player).s.origin[2 as libc::c_int as usize];
                playerdistance = VectorLength(v.as_mut_ptr());
                if playerdistance < bestplayerdistance {
                    bestplayerdistance = playerdistance;
                }
            }
        }
        n += 1;
    }
    return bestplayerdistance;
}
#[no_mangle]
pub unsafe extern "C" fn SelectRandomDeathmatchSpawnPoint() -> *mut edict_t {
    let mut spot: *mut edict_t = 0 as *mut edict_t;
    let mut spot1: *mut edict_t = 0 as *mut edict_t;
    let mut spot2: *mut edict_t = 0 as *mut edict_t;
    let mut count: libc::c_int = 0 as libc::c_int;
    let mut selection: libc::c_int = 0;
    let mut range: libc::c_float = 0.;
    let mut range1: libc::c_float = 0.;
    let mut range2: libc::c_float = 0.;
    spot = 0 as *mut edict_t;
    range2 = 99999 as libc::c_int as libc::c_float;
    range1 = range2;
    spot2 = 0 as *mut edict_t;
    spot1 = spot2;
    loop {
        spot = G_Find(
            spot,
            &mut (*(0 as *mut edict_t)).classname as *mut *mut libc::c_char
                as libc::c_int,
            b"info_player_deathmatch\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        if spot.is_null() {
            break;
        }
        count += 1;
        range = PlayersRangeFromSpot(spot);
        if range < range1 {
            range1 = range;
            spot1 = spot;
        } else if range < range2 {
            range2 = range;
            spot2 = spot;
        }
    }
    if count == 0 {
        return 0 as *mut edict_t;
    }
    if count <= 2 as libc::c_int {
        spot2 = 0 as *mut edict_t;
        spot1 = spot2;
    } else {
        count -= 2 as libc::c_int;
    }
    selection = rand() % count;
    spot = 0 as *mut edict_t;
    loop {
        spot = G_Find(
            spot,
            &mut (*(0 as *mut edict_t)).classname as *mut *mut libc::c_char
                as libc::c_int,
            b"info_player_deathmatch\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        if spot == spot1 || spot == spot2 {
            selection += 1;
        }
        let fresh24 = selection;
        selection = selection - 1;
        if !(fresh24 != 0) {
            break;
        }
    }
    return spot;
}
#[no_mangle]
pub unsafe extern "C" fn SelectFarthestDeathmatchSpawnPoint() -> *mut edict_t {
    let mut bestspot: *mut edict_t = 0 as *mut edict_t;
    let mut bestdistance: libc::c_float = 0.;
    let mut bestplayerdistance: libc::c_float = 0.;
    let mut spot: *mut edict_t = 0 as *mut edict_t;
    spot = 0 as *mut edict_t;
    bestspot = 0 as *mut edict_t;
    bestdistance = 0 as libc::c_int as libc::c_float;
    loop {
        spot = G_Find(
            spot,
            &mut (*(0 as *mut edict_t)).classname as *mut *mut libc::c_char
                as libc::c_int,
            b"info_player_deathmatch\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        if spot.is_null() {
            break;
        }
        bestplayerdistance = PlayersRangeFromSpot(spot);
        if bestplayerdistance > bestdistance {
            bestspot = spot;
            bestdistance = bestplayerdistance;
        }
    }
    if !bestspot.is_null() {
        return bestspot;
    }
    spot = G_Find(
        0 as *mut edict_t,
        &mut (*(0 as *mut edict_t)).classname as *mut *mut libc::c_char as libc::c_int,
        b"info_player_deathmatch\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    return spot;
}
#[no_mangle]
pub unsafe extern "C" fn SelectDeathmatchSpawnPoint() -> *mut edict_t {
    if (*dmflags).value as libc::c_int & 0x200 as libc::c_int != 0 {
        return SelectFarthestDeathmatchSpawnPoint()
    } else {
        return SelectRandomDeathmatchSpawnPoint()
    };
}
#[no_mangle]
pub unsafe extern "C" fn SelectCoopSpawnPoint(mut ent: *mut edict_t) -> *mut edict_t {
    let mut index: libc::c_int = 0;
    let mut spot: *mut edict_t = 0 as *mut edict_t;
    let mut target: *mut libc::c_char = 0 as *mut libc::c_char;
    index = ((*ent).client).offset_from(game.clients) as libc::c_long as libc::c_int;
    if index == 0 {
        return 0 as *mut edict_t;
    }
    spot = 0 as *mut edict_t;
    loop {
        spot = G_Find(
            spot,
            &mut (*(0 as *mut edict_t)).classname as *mut *mut libc::c_char
                as libc::c_int,
            b"info_player_coop\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        if spot.is_null() {
            return 0 as *mut edict_t;
        }
        target = (*spot).targetname;
        if target.is_null() {
            target = b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        }
        if Q_stricmp((game.spawnpoint).as_mut_ptr(), target) == 0 as libc::c_int {
            index -= 1;
            if index == 0 {
                return spot;
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn SelectSpawnPoint(
    mut ent: *mut edict_t,
    mut origin: *mut vec_t,
    mut angles: *mut vec_t,
) {
    let mut spot: *mut edict_t = 0 as *mut edict_t;
    if (*deathmatch).value != 0. {
        spot = SelectDeathmatchSpawnPoint();
    } else if (*coop).value != 0. {
        spot = SelectCoopSpawnPoint(ent);
    }
    if spot.is_null() {
        loop {
            spot = G_Find(
                spot,
                &mut (*(0 as *mut edict_t)).classname as *mut *mut libc::c_char
                    as libc::c_int,
                b"info_player_start\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
            if spot.is_null() {
                break;
            }
            if game.spawnpoint[0 as libc::c_int as usize] == 0
                && ((*spot).targetname).is_null()
            {
                break;
            }
            if game.spawnpoint[0 as libc::c_int as usize] == 0
                || ((*spot).targetname).is_null()
            {
                continue;
            }
            if Q_stricmp((game.spawnpoint).as_mut_ptr(), (*spot).targetname)
                == 0 as libc::c_int
            {
                break;
            }
        }
        if spot.is_null() {
            if game.spawnpoint[0 as libc::c_int as usize] == 0 {
                spot = G_Find(
                    spot,
                    &mut (*(0 as *mut edict_t)).classname as *mut *mut libc::c_char
                        as libc::c_int,
                    b"info_player_start\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                );
            }
            if spot.is_null() {
                (gi.error)
                    .expect(
                        "non-null function pointer",
                    )(
                    b"Couldn't find spawn point %s\n\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    (game.spawnpoint).as_mut_ptr(),
                );
            }
        }
    }
    *origin
        .offset(0 as libc::c_int as isize) = (*spot).s.origin[0 as libc::c_int as usize];
    *origin
        .offset(1 as libc::c_int as isize) = (*spot).s.origin[1 as libc::c_int as usize];
    *origin
        .offset(2 as libc::c_int as isize) = (*spot).s.origin[2 as libc::c_int as usize];
    let ref mut fresh25 = *origin.offset(2 as libc::c_int as isize);
    *fresh25 += 9 as libc::c_int as libc::c_float;
    *angles
        .offset(0 as libc::c_int as isize) = (*spot).s.angles[0 as libc::c_int as usize];
    *angles
        .offset(1 as libc::c_int as isize) = (*spot).s.angles[1 as libc::c_int as usize];
    *angles
        .offset(2 as libc::c_int as isize) = (*spot).s.angles[2 as libc::c_int as usize];
}
#[no_mangle]
pub unsafe extern "C" fn InitBodyQue() {
    let mut i: libc::c_int = 0;
    let mut ent: *mut edict_t = 0 as *mut edict_t;
    level.body_que = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        ent = G_Spawn();
        let ref mut fresh26 = (*ent).classname;
        *fresh26 = b"bodyque\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn body_die(
    mut self_0: *mut edict_t,
    mut inflictor: *mut edict_t,
    mut attacker: *mut edict_t,
    mut damage: libc::c_int,
    mut point: *mut vec_t,
) {
    let mut n: libc::c_int = 0;
    if (*self_0).health < -(40 as libc::c_int) {
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
                b"misc/udeath.wav\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            ),
            1 as libc::c_int as libc::c_float,
            1 as libc::c_int as libc::c_float,
            0 as libc::c_int as libc::c_float,
        );
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
        let ref mut fresh27 = (*self_0).s.origin[2 as libc::c_int as usize];
        *fresh27 -= 48 as libc::c_int as libc::c_float;
        ThrowClientHead(self_0, damage);
        (*self_0).takedamage = DAMAGE_NO as libc::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn CopyToBodyQue(mut ent: *mut edict_t) {
    let mut body: *mut edict_t = 0 as *mut edict_t;
    body = &mut *g_edicts
        .offset(
            ((*maxclients).value as libc::c_int + level.body_que + 1 as libc::c_int)
                as isize,
        ) as *mut edict_t;
    level.body_que = (level.body_que + 1 as libc::c_int) % 8 as libc::c_int;
    (gi.unlinkentity).expect("non-null function pointer")(ent);
    (gi.unlinkentity).expect("non-null function pointer")(body);
    (*body).s = (*ent).s;
    (*body).s.number = body.offset_from(g_edicts) as libc::c_long as libc::c_int;
    (*body).svflags = (*ent).svflags;
    (*body).mins[0 as libc::c_int as usize] = (*ent).mins[0 as libc::c_int as usize];
    (*body).mins[1 as libc::c_int as usize] = (*ent).mins[1 as libc::c_int as usize];
    (*body).mins[2 as libc::c_int as usize] = (*ent).mins[2 as libc::c_int as usize];
    (*body).maxs[0 as libc::c_int as usize] = (*ent).maxs[0 as libc::c_int as usize];
    (*body).maxs[1 as libc::c_int as usize] = (*ent).maxs[1 as libc::c_int as usize];
    (*body).maxs[2 as libc::c_int as usize] = (*ent).maxs[2 as libc::c_int as usize];
    (*body).absmin[0 as libc::c_int as usize] = (*ent).absmin[0 as libc::c_int as usize];
    (*body).absmin[1 as libc::c_int as usize] = (*ent).absmin[1 as libc::c_int as usize];
    (*body).absmin[2 as libc::c_int as usize] = (*ent).absmin[2 as libc::c_int as usize];
    (*body).absmax[0 as libc::c_int as usize] = (*ent).absmax[0 as libc::c_int as usize];
    (*body).absmax[1 as libc::c_int as usize] = (*ent).absmax[1 as libc::c_int as usize];
    (*body).absmax[2 as libc::c_int as usize] = (*ent).absmax[2 as libc::c_int as usize];
    (*body).size[0 as libc::c_int as usize] = (*ent).size[0 as libc::c_int as usize];
    (*body).size[1 as libc::c_int as usize] = (*ent).size[1 as libc::c_int as usize];
    (*body).size[2 as libc::c_int as usize] = (*ent).size[2 as libc::c_int as usize];
    (*body).solid = (*ent).solid;
    (*body).clipmask = (*ent).clipmask;
    let ref mut fresh28 = (*body).owner;
    *fresh28 = (*ent).owner;
    (*body).movetype = (*ent).movetype;
    let ref mut fresh29 = (*body).die;
    *fresh29 = Some(
        body_die
            as unsafe extern "C" fn(
                *mut edict_t,
                *mut edict_t,
                *mut edict_t,
                libc::c_int,
                *mut vec_t,
            ) -> (),
    );
    (*body).takedamage = DAMAGE_YES as libc::c_int;
    (gi.linkentity).expect("non-null function pointer")(body);
}
#[no_mangle]
pub unsafe extern "C" fn respawn(mut self_0: *mut edict_t) {
    if (*deathmatch).value != 0. || (*coop).value != 0. {
        if (*self_0).movetype != MOVETYPE_NOCLIP as libc::c_int {
            CopyToBodyQue(self_0);
        }
        (*self_0).svflags &= !(0x1 as libc::c_int);
        PutClientInServer(self_0);
        (*self_0).s.event = EV_PLAYER_TELEPORT as libc::c_int;
        (*(*self_0).client).ps.pmove.pm_flags = 32 as libc::c_int as byte;
        (*(*self_0).client).ps.pmove.pm_time = 14 as libc::c_int as byte;
        (*(*self_0).client).respawn_time = level.time;
        return;
    }
    (gi.AddCommandString)
        .expect(
            "non-null function pointer",
        )(b"menu_loadgame\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn spectator_respawn(mut ent: *mut edict_t) {
    let mut i: libc::c_int = 0;
    let mut numspec: libc::c_int = 0;
    if (*(*ent).client).pers.spectator as u64 != 0 {
        let mut value: *mut libc::c_char = Info_ValueForKey(
            ((*(*ent).client).pers.userinfo).as_mut_ptr(),
            b"spectator\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        if *(*spectator_password).string as libc::c_int != 0
            && strcmp(
                (*spectator_password).string,
                b"none\0" as *const u8 as *const libc::c_char,
            ) != 0 && strcmp((*spectator_password).string, value) != 0
        {
            (gi.cprintf)
                .expect(
                    "non-null function pointer",
                )(
                ent,
                2 as libc::c_int,
                b"Spectator password incorrect.\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
            (*(*ent).client).pers.spectator = false_0;
            (gi.WriteByte).expect("non-null function pointer")(11 as libc::c_int);
            (gi.WriteString)
                .expect(
                    "non-null function pointer",
                )(
                b"spectator 0\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
            (gi.unicast).expect("non-null function pointer")(ent, true_0);
            return;
        }
        i = 1 as libc::c_int;
        numspec = 0 as libc::c_int;
        while i as libc::c_float <= (*maxclients).value {
            if (*g_edicts.offset(i as isize)).inuse as libc::c_uint != 0
                && (*(*g_edicts.offset(i as isize)).client).pers.spectator
                    as libc::c_uint != 0
            {
                numspec += 1;
            }
            i += 1;
        }
        if numspec as libc::c_float >= (*maxspectators).value {
            (gi.cprintf)
                .expect(
                    "non-null function pointer",
                )(
                ent,
                2 as libc::c_int,
                b"Server spectator limit is full.\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
            (*(*ent).client).pers.spectator = false_0;
            (gi.WriteByte).expect("non-null function pointer")(11 as libc::c_int);
            (gi.WriteString)
                .expect(
                    "non-null function pointer",
                )(
                b"spectator 0\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
            (gi.unicast).expect("non-null function pointer")(ent, true_0);
            return;
        }
    } else {
        let mut value_0: *mut libc::c_char = Info_ValueForKey(
            ((*(*ent).client).pers.userinfo).as_mut_ptr(),
            b"password\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        if *(*password).string as libc::c_int != 0
            && strcmp((*password).string, b"none\0" as *const u8 as *const libc::c_char)
                != 0 && strcmp((*password).string, value_0) != 0
        {
            (gi.cprintf)
                .expect(
                    "non-null function pointer",
                )(
                ent,
                2 as libc::c_int,
                b"Password incorrect.\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
            (*(*ent).client).pers.spectator = true_0;
            (gi.WriteByte).expect("non-null function pointer")(11 as libc::c_int);
            (gi.WriteString)
                .expect(
                    "non-null function pointer",
                )(
                b"spectator 1\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
            (gi.unicast).expect("non-null function pointer")(ent, true_0);
            return;
        }
    }
    let ref mut fresh30 = (*(*ent).client).resp.score;
    *fresh30 = 0 as libc::c_int;
    (*(*ent).client).pers.score = *fresh30;
    (*ent).svflags &= !(0x1 as libc::c_int);
    PutClientInServer(ent);
    if (*(*ent).client).pers.spectator as u64 == 0 {
        (gi.WriteByte).expect("non-null function pointer")(1 as libc::c_int);
        (gi.WriteShort)
            .expect(
                "non-null function pointer",
            )(ent.offset_from(g_edicts) as libc::c_long as libc::c_int);
        (gi.WriteByte).expect("non-null function pointer")(9 as libc::c_int);
        (gi.multicast)
            .expect(
                "non-null function pointer",
            )(((*ent).s.origin).as_mut_ptr(), MULTICAST_PVS);
        (*(*ent).client).ps.pmove.pm_flags = 32 as libc::c_int as byte;
        (*(*ent).client).ps.pmove.pm_time = 14 as libc::c_int as byte;
    }
    (*(*ent).client).respawn_time = level.time;
    if (*(*ent).client).pers.spectator as u64 != 0 {
        (gi.bprintf)
            .expect(
                "non-null function pointer",
            )(
            2 as libc::c_int,
            b"%s has moved to the sidelines\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            ((*(*ent).client).pers.netname).as_mut_ptr(),
        );
    } else {
        (gi.bprintf)
            .expect(
                "non-null function pointer",
            )(
            2 as libc::c_int,
            b"%s joined the game\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            ((*(*ent).client).pers.netname).as_mut_ptr(),
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn PutClientInServer(mut ent: *mut edict_t) {
    let mut mins: vec3_t = [
        -(16 as libc::c_int) as vec_t,
        -(16 as libc::c_int) as vec_t,
        -(24 as libc::c_int) as vec_t,
    ];
    let mut maxs: vec3_t = [
        16 as libc::c_int as vec_t,
        16 as libc::c_int as vec_t,
        32 as libc::c_int as vec_t,
    ];
    let mut index: libc::c_int = 0;
    let mut spawn_origin: vec3_t = [0.; 3];
    let mut spawn_angles: vec3_t = [0.; 3];
    let mut client: *mut gclient_t = 0 as *mut gclient_t;
    let mut i: libc::c_int = 0;
    let mut saved: client_persistant_t = client_persistant_t {
        userinfo: [0; 512],
        netname: [0; 16],
        hand: 0,
        connected: false_0,
        health: 0,
        max_health: 0,
        savedFlags: 0,
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
        game_helpchanged: 0,
        helpchanged: 0,
        spectator: false_0,
    };
    let mut resp: client_respawn_t = client_respawn_t {
        coop_respawn: client_persistant_t {
            userinfo: [0; 512],
            netname: [0; 16],
            hand: 0,
            connected: false_0,
            health: 0,
            max_health: 0,
            savedFlags: 0,
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
            game_helpchanged: 0,
            helpchanged: 0,
            spectator: false_0,
        },
        enterframe: 0,
        score: 0,
        cmd_angles: [0.; 3],
        spectator: false_0,
    };
    SelectSpawnPoint(ent, spawn_origin.as_mut_ptr(), spawn_angles.as_mut_ptr());
    index = (ent.offset_from(g_edicts) as libc::c_long
        - 1 as libc::c_int as libc::c_long) as libc::c_int;
    client = (*ent).client;
    if (*deathmatch).value != 0. {
        let mut userinfo: [libc::c_char; 512] = [0; 512];
        resp = (*client).resp;
        memcpy(
            userinfo.as_mut_ptr() as *mut libc::c_void,
            ((*client).pers.userinfo).as_mut_ptr() as *const libc::c_void,
            ::std::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong,
        );
        InitClientPersistant(client);
        ClientUserinfoChanged(ent, userinfo.as_mut_ptr());
    } else if (*coop).value != 0. {
        let mut userinfo_0: [libc::c_char; 512] = [0; 512];
        resp = (*client).resp;
        memcpy(
            userinfo_0.as_mut_ptr() as *mut libc::c_void,
            ((*client).pers.userinfo).as_mut_ptr() as *const libc::c_void,
            ::std::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong,
        );
        resp.coop_respawn.game_helpchanged = (*client).pers.game_helpchanged;
        resp.coop_respawn.helpchanged = (*client).pers.helpchanged;
        (*client).pers = resp.coop_respawn;
        ClientUserinfoChanged(ent, userinfo_0.as_mut_ptr());
        if resp.score > (*client).pers.score {
            (*client).pers.score = resp.score;
        }
    } else {
        memset(
            &mut resp as *mut client_respawn_t as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<client_respawn_t>() as libc::c_ulong,
        );
    }
    saved = (*client).pers;
    memset(
        client as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<gclient_t>() as libc::c_ulong,
    );
    (*client).pers = saved;
    if (*client).pers.health <= 0 as libc::c_int {
        InitClientPersistant(client);
    }
    (*client).resp = resp;
    FetchClientEntData(ent);
    let ref mut fresh31 = (*ent).groundentity;
    *fresh31 = 0 as *mut edict_t;
    let ref mut fresh32 = (*ent).client;
    *fresh32 = &mut *(game.clients).offset(index as isize) as *mut gclient_t;
    (*ent).takedamage = DAMAGE_AIM as libc::c_int;
    (*ent).movetype = MOVETYPE_WALK as libc::c_int;
    (*ent).viewheight = 22 as libc::c_int;
    (*ent).inuse = true_0;
    let ref mut fresh33 = (*ent).classname;
    *fresh33 = b"player\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    (*ent).mass = 200 as libc::c_int;
    (*ent).solid = SOLID_BBOX;
    (*ent).deadflag = 0 as libc::c_int;
    (*ent).air_finished = level.time + 12 as libc::c_int as libc::c_float;
    (*ent)
        .clipmask = 1 as libc::c_int | 0x10000 as libc::c_int | 2 as libc::c_int
        | 0x2000000 as libc::c_int;
    let ref mut fresh34 = (*ent).model;
    *fresh34 = b"players/male/tris.md2\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char;
    let ref mut fresh35 = (*ent).pain;
    *fresh35 = Some(
        player_pain
            as unsafe extern "C" fn(
                *mut edict_t,
                *mut edict_t,
                libc::c_float,
                libc::c_int,
            ) -> (),
    );
    let ref mut fresh36 = (*ent).die;
    *fresh36 = Some(
        player_die
            as unsafe extern "C" fn(
                *mut edict_t,
                *mut edict_t,
                *mut edict_t,
                libc::c_int,
                *mut vec_t,
            ) -> (),
    );
    (*ent).waterlevel = 0 as libc::c_int;
    (*ent).watertype = 0 as libc::c_int;
    (*ent).flags &= !(0x800 as libc::c_int);
    (*ent).svflags &= !(0x2 as libc::c_int);
    (*ent).mins[0 as libc::c_int as usize] = mins[0 as libc::c_int as usize];
    (*ent).mins[1 as libc::c_int as usize] = mins[1 as libc::c_int as usize];
    (*ent).mins[2 as libc::c_int as usize] = mins[2 as libc::c_int as usize];
    (*ent).maxs[0 as libc::c_int as usize] = maxs[0 as libc::c_int as usize];
    (*ent).maxs[1 as libc::c_int as usize] = maxs[1 as libc::c_int as usize];
    (*ent).maxs[2 as libc::c_int as usize] = maxs[2 as libc::c_int as usize];
    let ref mut fresh37 = (*ent).velocity[2 as libc::c_int as usize];
    *fresh37 = 0 as libc::c_int as vec_t;
    let ref mut fresh38 = (*ent).velocity[1 as libc::c_int as usize];
    *fresh38 = *fresh37;
    (*ent).velocity[0 as libc::c_int as usize] = *fresh38;
    memset(
        &mut (*(*ent).client).ps as *mut player_state_t as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<player_state_t>() as libc::c_ulong,
    );
    (*client)
        .ps
        .pmove
        .origin[0 as libc::c_int
        as usize] = (spawn_origin[0 as libc::c_int as usize]
        * 8 as libc::c_int as libc::c_float) as libc::c_short;
    (*client)
        .ps
        .pmove
        .origin[1 as libc::c_int
        as usize] = (spawn_origin[1 as libc::c_int as usize]
        * 8 as libc::c_int as libc::c_float) as libc::c_short;
    (*client)
        .ps
        .pmove
        .origin[2 as libc::c_int
        as usize] = (spawn_origin[2 as libc::c_int as usize]
        * 8 as libc::c_int as libc::c_float) as libc::c_short;
    if (*deathmatch).value != 0.
        && (*dmflags).value as libc::c_int & 0x8000 as libc::c_int != 0
    {
        (*client).ps.fov = 90 as libc::c_int as libc::c_float;
    } else {
        (*client)
            .ps
            .fov = atoi(
            Info_ValueForKey(
                ((*client).pers.userinfo).as_mut_ptr(),
                b"fov\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ),
        ) as libc::c_float;
        if (*client).ps.fov < 1 as libc::c_int as libc::c_float {
            (*client).ps.fov = 90 as libc::c_int as libc::c_float;
        } else if (*client).ps.fov > 160 as libc::c_int as libc::c_float {
            (*client).ps.fov = 160 as libc::c_int as libc::c_float;
        }
    }
    (*client)
        .ps
        .gunindex = (gi.modelindex)
        .expect("non-null function pointer")((*(*client).pers.weapon).view_model);
    (*ent).s.effects = 0 as libc::c_int as libc::c_uint;
    (*ent).s.modelindex = 255 as libc::c_int;
    (*ent).s.modelindex2 = 255 as libc::c_int;
    (*ent)
        .s
        .skinnum = (ent.offset_from(g_edicts) as libc::c_long
        - 1 as libc::c_int as libc::c_long) as libc::c_int;
    (*ent).s.frame = 0 as libc::c_int;
    (*ent).s.origin[0 as libc::c_int as usize] = spawn_origin[0 as libc::c_int as usize];
    (*ent).s.origin[1 as libc::c_int as usize] = spawn_origin[1 as libc::c_int as usize];
    (*ent).s.origin[2 as libc::c_int as usize] = spawn_origin[2 as libc::c_int as usize];
    let ref mut fresh39 = (*ent).s.origin[2 as libc::c_int as usize];
    *fresh39 += 1 as libc::c_int as libc::c_float;
    (*ent)
        .s
        .old_origin[0 as libc::c_int
        as usize] = (*ent).s.origin[0 as libc::c_int as usize];
    (*ent)
        .s
        .old_origin[1 as libc::c_int
        as usize] = (*ent).s.origin[1 as libc::c_int as usize];
    (*ent)
        .s
        .old_origin[2 as libc::c_int
        as usize] = (*ent).s.origin[2 as libc::c_int as usize];
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        (*client)
            .ps
            .pmove
            .delta_angles[i
            as usize] = (((spawn_angles[i as usize]
            - (*client).resp.cmd_angles[i as usize])
            * 65536 as libc::c_int as libc::c_float
            / 360 as libc::c_int as libc::c_float) as libc::c_int & 65535 as libc::c_int)
            as libc::c_short;
        i += 1;
    }
    (*ent).s.angles[0 as libc::c_int as usize] = 0 as libc::c_int as vec_t;
    (*ent).s.angles[1 as libc::c_int as usize] = spawn_angles[1 as libc::c_int as usize];
    (*ent).s.angles[2 as libc::c_int as usize] = 0 as libc::c_int as vec_t;
    (*client)
        .ps
        .viewangles[0 as libc::c_int
        as usize] = (*ent).s.angles[0 as libc::c_int as usize];
    (*client)
        .ps
        .viewangles[1 as libc::c_int
        as usize] = (*ent).s.angles[1 as libc::c_int as usize];
    (*client)
        .ps
        .viewangles[2 as libc::c_int
        as usize] = (*ent).s.angles[2 as libc::c_int as usize];
    (*client)
        .v_angle[0 as libc::c_int as usize] = (*ent).s.angles[0 as libc::c_int as usize];
    (*client)
        .v_angle[1 as libc::c_int as usize] = (*ent).s.angles[1 as libc::c_int as usize];
    (*client)
        .v_angle[2 as libc::c_int as usize] = (*ent).s.angles[2 as libc::c_int as usize];
    if (*client).pers.spectator as u64 != 0 {
        let ref mut fresh40 = (*client).chase_target;
        *fresh40 = 0 as *mut edict_t;
        (*client).resp.spectator = true_0;
        (*ent).movetype = MOVETYPE_NOCLIP as libc::c_int;
        (*ent).solid = SOLID_NOT;
        (*ent).svflags |= 0x1 as libc::c_int;
        (*(*ent).client).ps.gunindex = 0 as libc::c_int;
        (gi.linkentity).expect("non-null function pointer")(ent);
        return;
    } else {
        (*client).resp.spectator = false_0;
    }
    KillBox(ent) as u64 == 0;
    (gi.linkentity).expect("non-null function pointer")(ent);
    let ref mut fresh41 = (*client).newweapon;
    *fresh41 = (*client).pers.weapon;
    ChangeWeapon(ent);
}
#[no_mangle]
pub unsafe extern "C" fn ClientBeginDeathmatch(mut ent: *mut edict_t) {
    G_InitEdict(ent);
    InitClientResp((*ent).client);
    PutClientInServer(ent);
    (gi.WriteByte).expect("non-null function pointer")(1 as libc::c_int);
    (gi.WriteShort)
        .expect(
            "non-null function pointer",
        )(ent.offset_from(g_edicts) as libc::c_long as libc::c_int);
    (gi.WriteByte).expect("non-null function pointer")(9 as libc::c_int);
    (gi.multicast)
        .expect(
            "non-null function pointer",
        )(((*ent).s.origin).as_mut_ptr(), MULTICAST_PVS);
    (gi.bprintf)
        .expect(
            "non-null function pointer",
        )(
        2 as libc::c_int,
        b"%s entered the game\n\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        ((*(*ent).client).pers.netname).as_mut_ptr(),
    );
    ClientEndServerFrame(ent);
}
#[no_mangle]
pub unsafe extern "C" fn ClientBegin(mut ent: *mut edict_t) {
    let mut i: libc::c_int = 0;
    let ref mut fresh42 = (*ent).client;
    *fresh42 = (game.clients)
        .offset(
            (ent.offset_from(g_edicts) as libc::c_long
                - 1 as libc::c_int as libc::c_long) as isize,
        );
    if (*deathmatch).value != 0. {
        ClientBeginDeathmatch(ent);
        return;
    }
    if (*ent).inuse as libc::c_uint == true_0 as libc::c_int as libc::c_uint {
        i = 0 as libc::c_int;
        while i < 3 as libc::c_int {
            (*(*ent).client)
                .ps
                .pmove
                .delta_angles[i
                as usize] = (((*(*ent).client).ps.viewangles[i as usize]
                * 65536 as libc::c_int as libc::c_float
                / 360 as libc::c_int as libc::c_float) as libc::c_int
                & 65535 as libc::c_int) as libc::c_short;
            i += 1;
        }
    } else {
        G_InitEdict(ent);
        let ref mut fresh43 = (*ent).classname;
        *fresh43 = b"player\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        InitClientResp((*ent).client);
        PutClientInServer(ent);
    }
    if level.intermissiontime != 0. {
        MoveClientToIntermission(ent);
    } else if game.maxclients > 1 as libc::c_int {
        (gi.WriteByte).expect("non-null function pointer")(1 as libc::c_int);
        (gi.WriteShort)
            .expect(
                "non-null function pointer",
            )(ent.offset_from(g_edicts) as libc::c_long as libc::c_int);
        (gi.WriteByte).expect("non-null function pointer")(9 as libc::c_int);
        (gi.multicast)
            .expect(
                "non-null function pointer",
            )(((*ent).s.origin).as_mut_ptr(), MULTICAST_PVS);
        (gi.bprintf)
            .expect(
                "non-null function pointer",
            )(
            2 as libc::c_int,
            b"%s entered the game\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            ((*(*ent).client).pers.netname).as_mut_ptr(),
        );
    }
    ClientEndServerFrame(ent);
}
#[no_mangle]
pub unsafe extern "C" fn ClientUserinfoChanged(
    mut ent: *mut edict_t,
    mut userinfo: *mut libc::c_char,
) {
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut playernum: libc::c_int = 0;
    if Info_Validate(userinfo) as u64 == 0 {
        strcpy(
            userinfo,
            b"\\name\\badinfo\\skin\\male/grunt\0" as *const u8 as *const libc::c_char,
        );
    }
    s = Info_ValueForKey(
        userinfo,
        b"name\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    strncpy(
        ((*(*ent).client).pers.netname).as_mut_ptr(),
        s,
        (::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
    );
    s = Info_ValueForKey(
        userinfo,
        b"spectator\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if (*deathmatch).value != 0. && *s as libc::c_int != 0
        && strcmp(s, b"0\0" as *const u8 as *const libc::c_char) != 0
    {
        (*(*ent).client).pers.spectator = true_0;
    } else {
        (*(*ent).client).pers.spectator = false_0;
    }
    s = Info_ValueForKey(
        userinfo,
        b"skin\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    playernum = (ent.offset_from(g_edicts) as libc::c_long
        - 1 as libc::c_int as libc::c_long) as libc::c_int;
    (gi.configstring)
        .expect(
            "non-null function pointer",
        )(
        32 as libc::c_int + 256 as libc::c_int + 256 as libc::c_int + 256 as libc::c_int
            + 256 as libc::c_int + 256 as libc::c_int + playernum,
        va(
            b"%s\\%s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ((*(*ent).client).pers.netname).as_mut_ptr(),
            s,
        ),
    );
    if (*deathmatch).value != 0.
        && (*dmflags).value as libc::c_int & 0x8000 as libc::c_int != 0
    {
        (*(*ent).client).ps.fov = 90 as libc::c_int as libc::c_float;
    } else {
        (*(*ent).client)
            .ps
            .fov = atoi(
            Info_ValueForKey(
                userinfo,
                b"fov\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ),
        ) as libc::c_float;
        if (*(*ent).client).ps.fov < 1 as libc::c_int as libc::c_float {
            (*(*ent).client).ps.fov = 90 as libc::c_int as libc::c_float;
        } else if (*(*ent).client).ps.fov > 160 as libc::c_int as libc::c_float {
            (*(*ent).client).ps.fov = 160 as libc::c_int as libc::c_float;
        }
    }
    s = Info_ValueForKey(
        userinfo,
        b"hand\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if strlen(s) != 0 {
        (*(*ent).client).pers.hand = atoi(s);
    }
    strncpy(
        ((*(*ent).client).pers.userinfo).as_mut_ptr(),
        userinfo,
        (::std::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
    );
}
#[no_mangle]
pub unsafe extern "C" fn ClientConnect(
    mut ent: *mut edict_t,
    mut userinfo: *mut libc::c_char,
) -> qboolean {
    let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
    value = Info_ValueForKey(
        userinfo,
        b"ip\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if SV_FilterPacket(value) as u64 != 0 {
        Info_SetValueForKey(
            userinfo,
            b"rejmsg\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"Banned.\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        return false_0;
    }
    value = Info_ValueForKey(
        userinfo,
        b"spectator\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if (*deathmatch).value != 0. && *value as libc::c_int != 0
        && strcmp(value, b"0\0" as *const u8 as *const libc::c_char) != 0
    {
        let mut i: libc::c_int = 0;
        let mut numspec: libc::c_int = 0;
        if *(*spectator_password).string as libc::c_int != 0
            && strcmp(
                (*spectator_password).string,
                b"none\0" as *const u8 as *const libc::c_char,
            ) != 0 && strcmp((*spectator_password).string, value) != 0
        {
            Info_SetValueForKey(
                userinfo,
                b"rejmsg\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                b"Spectator password required or incorrect.\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
            );
            return false_0;
        }
        numspec = 0 as libc::c_int;
        i = numspec;
        while (i as libc::c_float) < (*maxclients).value {
            if (*g_edicts.offset((i + 1 as libc::c_int) as isize)).inuse as libc::c_uint
                != 0
                && (*(*g_edicts.offset((i + 1 as libc::c_int) as isize)).client)
                    .pers
                    .spectator as libc::c_uint != 0
            {
                numspec += 1;
            }
            i += 1;
        }
        if numspec as libc::c_float >= (*maxspectators).value {
            Info_SetValueForKey(
                userinfo,
                b"rejmsg\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                b"Server spectator limit is full.\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
            return false_0;
        }
    } else {
        value = Info_ValueForKey(
            userinfo,
            b"password\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        if *(*password).string as libc::c_int != 0
            && strcmp((*password).string, b"none\0" as *const u8 as *const libc::c_char)
                != 0 && strcmp((*password).string, value) != 0
        {
            Info_SetValueForKey(
                userinfo,
                b"rejmsg\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                b"Password required or incorrect.\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
            return false_0;
        }
    }
    let ref mut fresh44 = (*ent).client;
    *fresh44 = (game.clients)
        .offset(
            (ent.offset_from(g_edicts) as libc::c_long
                - 1 as libc::c_int as libc::c_long) as isize,
        );
    if (*ent).inuse as libc::c_uint == false_0 as libc::c_int as libc::c_uint {
        InitClientResp((*ent).client);
        if game.autosaved as u64 == 0 || ((*(*ent).client).pers.weapon).is_null() {
            InitClientPersistant((*ent).client);
        }
    }
    ClientUserinfoChanged(ent, userinfo);
    if game.maxclients > 1 as libc::c_int {
        (gi.dprintf)
            .expect(
                "non-null function pointer",
            )(
            b"%s connected\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ((*(*ent).client).pers.netname).as_mut_ptr(),
        );
    }
    (*(*ent).client).pers.connected = true_0;
    return true_0;
}
#[no_mangle]
pub unsafe extern "C" fn ClientDisconnect(mut ent: *mut edict_t) {
    let mut playernum: libc::c_int = 0;
    if ((*ent).client).is_null() {
        return;
    }
    (gi.bprintf)
        .expect(
            "non-null function pointer",
        )(
        2 as libc::c_int,
        b"%s disconnected\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ((*(*ent).client).pers.netname).as_mut_ptr(),
    );
    (gi.WriteByte).expect("non-null function pointer")(1 as libc::c_int);
    (gi.WriteShort)
        .expect(
            "non-null function pointer",
        )(ent.offset_from(g_edicts) as libc::c_long as libc::c_int);
    (gi.WriteByte).expect("non-null function pointer")(10 as libc::c_int);
    (gi.multicast)
        .expect(
            "non-null function pointer",
        )(((*ent).s.origin).as_mut_ptr(), MULTICAST_PVS);
    (gi.unlinkentity).expect("non-null function pointer")(ent);
    (*ent).s.modelindex = 0 as libc::c_int;
    (*ent).solid = SOLID_NOT;
    (*ent).inuse = false_0;
    let ref mut fresh45 = (*ent).classname;
    *fresh45 = b"disconnected\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char;
    (*(*ent).client).pers.connected = false_0;
    playernum = (ent.offset_from(g_edicts) as libc::c_long
        - 1 as libc::c_int as libc::c_long) as libc::c_int;
    (gi.configstring)
        .expect(
            "non-null function pointer",
        )(
        32 as libc::c_int + 256 as libc::c_int + 256 as libc::c_int + 256 as libc::c_int
            + 256 as libc::c_int + 256 as libc::c_int + playernum,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
}
#[no_mangle]
pub static mut pm_passent: *mut edict_t = 0 as *const edict_t as *mut edict_t;
#[no_mangle]
pub unsafe extern "C" fn PM_trace(
    mut start: *mut vec_t,
    mut mins: *mut vec_t,
    mut maxs: *mut vec_t,
    mut end: *mut vec_t,
) -> trace_t {
    if (*pm_passent).health > 0 as libc::c_int {
        return (gi.trace)
            .expect(
                "non-null function pointer",
            )(
            start,
            mins,
            maxs,
            end,
            pm_passent,
            1 as libc::c_int | 0x10000 as libc::c_int | 2 as libc::c_int
                | 0x2000000 as libc::c_int,
        )
    } else {
        return (gi.trace)
            .expect(
                "non-null function pointer",
            )(
            start,
            mins,
            maxs,
            end,
            pm_passent,
            1 as libc::c_int | 0x10000 as libc::c_int | 2 as libc::c_int,
        )
    };
}
#[no_mangle]
pub unsafe extern "C" fn CheckBlock(
    mut b: *mut libc::c_void,
    mut c: libc::c_int,
) -> libc::c_uint {
    let mut v: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    v = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < c {
        v += *(b as *mut byte).offset(i as isize) as libc::c_int;
        i += 1;
    }
    return v as libc::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn PrintPmove(mut pm: *mut pmove_t) {
    let mut c1: libc::c_uint = 0;
    let mut c2: libc::c_uint = 0;
    c1 = CheckBlock(
        &mut (*pm).s as *mut pmove_state_t as *mut libc::c_void,
        ::std::mem::size_of::<pmove_state_t>() as libc::c_ulong as libc::c_int,
    );
    c2 = CheckBlock(
        &mut (*pm).cmd as *mut usercmd_t as *mut libc::c_void,
        ::std::mem::size_of::<usercmd_t>() as libc::c_ulong as libc::c_int,
    );
    Com_Printf(
        b"sv %3i:%i %i\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        (*pm).cmd.impulse as libc::c_int,
        c1,
        c2,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ClientThink(mut ent: *mut edict_t, mut ucmd: *mut usercmd_t) {
    let mut client: *mut gclient_t = 0 as *mut gclient_t;
    let mut other: *mut edict_t = 0 as *mut edict_t;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut pm: pmove_t = pmove_t {
        s: pmove_state_t {
            pm_type: PM_NORMAL,
            origin: [0; 3],
            velocity: [0; 3],
            pm_flags: 0,
            pm_time: 0,
            gravity: 0,
            delta_angles: [0; 3],
        },
        cmd: usercmd_t {
            msec: 0,
            buttons: 0,
            angles: [0; 3],
            forwardmove: 0,
            sidemove: 0,
            upmove: 0,
            impulse: 0,
            lightlevel: 0,
        },
        snapinitial: false_0,
        numtouch: 0,
        touchents: [0 as *mut edict_s; 32],
        viewangles: [0.; 3],
        viewheight: 0.,
        mins: [0.; 3],
        maxs: [0.; 3],
        groundentity: 0 as *mut edict_s,
        watertype: 0,
        waterlevel: 0,
        trace: None,
        pointcontents: None,
    };
    level.current_entity = ent;
    client = (*ent).client;
    if level.intermissiontime != 0. {
        (*client).ps.pmove.pm_type = PM_FREEZE;
        if level.time as libc::c_double
            > level.intermissiontime as libc::c_double + 5.0f64
            && (*ucmd).buttons as libc::c_int & 128 as libc::c_int != 0
        {
            level.exitintermission = true_0 as libc::c_int;
        }
        return;
    }
    pm_passent = ent;
    if !((*(*ent).client).chase_target).is_null() {
        (*client)
            .resp
            .cmd_angles[0 as libc::c_int
            as usize] = ((*ucmd).angles[0 as libc::c_int as usize] as libc::c_int
            as libc::c_double * (360.0f64 / 65536 as libc::c_int as libc::c_double))
            as vec_t;
        (*client)
            .resp
            .cmd_angles[1 as libc::c_int
            as usize] = ((*ucmd).angles[1 as libc::c_int as usize] as libc::c_int
            as libc::c_double * (360.0f64 / 65536 as libc::c_int as libc::c_double))
            as vec_t;
        (*client)
            .resp
            .cmd_angles[2 as libc::c_int
            as usize] = ((*ucmd).angles[2 as libc::c_int as usize] as libc::c_int
            as libc::c_double * (360.0f64 / 65536 as libc::c_int as libc::c_double))
            as vec_t;
    } else {
        memset(
            &mut pm as *mut pmove_t as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<pmove_t>() as libc::c_ulong,
        );
        if (*ent).movetype == MOVETYPE_NOCLIP as libc::c_int {
            (*client).ps.pmove.pm_type = PM_SPECTATOR;
        } else if (*ent).s.modelindex != 255 as libc::c_int {
            (*client).ps.pmove.pm_type = PM_GIB;
        } else if (*ent).deadflag != 0 {
            (*client).ps.pmove.pm_type = PM_DEAD;
        } else {
            (*client).ps.pmove.pm_type = PM_NORMAL;
        }
        (*client).ps.pmove.gravity = (*sv_gravity).value as libc::c_short;
        pm.s = (*client).ps.pmove;
        i = 0 as libc::c_int;
        while i < 3 as libc::c_int {
            pm
                .s
                .origin[i
                as usize] = ((*ent).s.origin[i as usize]
                * 8 as libc::c_int as libc::c_float) as libc::c_short;
            pm
                .s
                .velocity[i
                as usize] = ((*ent).velocity[i as usize]
                * 8 as libc::c_int as libc::c_float) as libc::c_short;
            i += 1;
        }
        if memcmp(
            &mut (*client).old_pmove as *mut pmove_state_t as *const libc::c_void,
            &mut pm.s as *mut pmove_state_t as *const libc::c_void,
            ::std::mem::size_of::<pmove_state_t>() as libc::c_ulong,
        ) != 0
        {
            pm.snapinitial = true_0;
        }
        pm.cmd = *ucmd;
        pm
            .trace = Some(
            PM_trace
                as unsafe extern "C" fn(
                    *mut vec_t,
                    *mut vec_t,
                    *mut vec_t,
                    *mut vec_t,
                ) -> trace_t,
        );
        pm.pointcontents = gi.pointcontents;
        (gi.Pmove).expect("non-null function pointer")(&mut pm);
        (*client).ps.pmove = pm.s;
        (*client).old_pmove = pm.s;
        i = 0 as libc::c_int;
        while i < 3 as libc::c_int {
            (*ent)
                .s
                .origin[i
                as usize] = (pm.s.origin[i as usize] as libc::c_int as libc::c_double
                * 0.125f64) as vec_t;
            (*ent)
                .velocity[i
                as usize] = (pm.s.velocity[i as usize] as libc::c_int as libc::c_double
                * 0.125f64) as vec_t;
            i += 1;
        }
        (*ent).mins[0 as libc::c_int as usize] = pm.mins[0 as libc::c_int as usize];
        (*ent).mins[1 as libc::c_int as usize] = pm.mins[1 as libc::c_int as usize];
        (*ent).mins[2 as libc::c_int as usize] = pm.mins[2 as libc::c_int as usize];
        (*ent).maxs[0 as libc::c_int as usize] = pm.maxs[0 as libc::c_int as usize];
        (*ent).maxs[1 as libc::c_int as usize] = pm.maxs[1 as libc::c_int as usize];
        (*ent).maxs[2 as libc::c_int as usize] = pm.maxs[2 as libc::c_int as usize];
        (*client)
            .resp
            .cmd_angles[0 as libc::c_int
            as usize] = ((*ucmd).angles[0 as libc::c_int as usize] as libc::c_int
            as libc::c_double * (360.0f64 / 65536 as libc::c_int as libc::c_double))
            as vec_t;
        (*client)
            .resp
            .cmd_angles[1 as libc::c_int
            as usize] = ((*ucmd).angles[1 as libc::c_int as usize] as libc::c_int
            as libc::c_double * (360.0f64 / 65536 as libc::c_int as libc::c_double))
            as vec_t;
        (*client)
            .resp
            .cmd_angles[2 as libc::c_int
            as usize] = ((*ucmd).angles[2 as libc::c_int as usize] as libc::c_int
            as libc::c_double * (360.0f64 / 65536 as libc::c_int as libc::c_double))
            as vec_t;
        if !((*ent).groundentity).is_null() && (pm.groundentity).is_null()
            && pm.cmd.upmove as libc::c_int >= 10 as libc::c_int
            && pm.waterlevel == 0 as libc::c_int
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
                    b"*jump1.wav\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                ),
                1 as libc::c_int as libc::c_float,
                1 as libc::c_int as libc::c_float,
                0 as libc::c_int as libc::c_float,
            );
            PlayerNoise(ent, ((*ent).s.origin).as_mut_ptr(), 0 as libc::c_int);
        }
        (*ent).viewheight = pm.viewheight as libc::c_int;
        (*ent).waterlevel = pm.waterlevel;
        (*ent).watertype = pm.watertype;
        let ref mut fresh46 = (*ent).groundentity;
        *fresh46 = pm.groundentity;
        if !(pm.groundentity).is_null() {
            (*ent).groundentity_linkcount = (*pm.groundentity).linkcount;
        }
        if (*ent).deadflag != 0 {
            (*client)
                .ps
                .viewangles[2 as libc::c_int as usize] = 40 as libc::c_int as vec_t;
            (*client)
                .ps
                .viewangles[0 as libc::c_int as usize] = -(15 as libc::c_int) as vec_t;
            (*client).ps.viewangles[1 as libc::c_int as usize] = (*client).killer_yaw;
        } else {
            (*client)
                .v_angle[0 as libc::c_int
                as usize] = pm.viewangles[0 as libc::c_int as usize];
            (*client)
                .v_angle[1 as libc::c_int
                as usize] = pm.viewangles[1 as libc::c_int as usize];
            (*client)
                .v_angle[2 as libc::c_int
                as usize] = pm.viewangles[2 as libc::c_int as usize];
            (*client)
                .ps
                .viewangles[0 as libc::c_int
                as usize] = pm.viewangles[0 as libc::c_int as usize];
            (*client)
                .ps
                .viewangles[1 as libc::c_int
                as usize] = pm.viewangles[1 as libc::c_int as usize];
            (*client)
                .ps
                .viewangles[2 as libc::c_int
                as usize] = pm.viewangles[2 as libc::c_int as usize];
        }
        (gi.linkentity).expect("non-null function pointer")(ent);
        if (*ent).movetype != MOVETYPE_NOCLIP as libc::c_int {
            G_TouchTriggers(ent);
        }
        i = 0 as libc::c_int;
        while i < pm.numtouch {
            other = pm.touchents[i as usize];
            j = 0 as libc::c_int;
            while j < i {
                if pm.touchents[j as usize] == other {
                    break;
                }
                j += 1;
            }
            if !(j != i) {
                if !((*other).touch).is_none() {
                    ((*other).touch)
                        .expect(
                            "non-null function pointer",
                        )(other, ent, 0 as *mut cplane_t, 0 as *mut csurface_t);
                }
            }
            i += 1;
        }
    }
    (*client).oldbuttons = (*client).buttons;
    (*client).buttons = (*ucmd).buttons as libc::c_int;
    (*client).latched_buttons |= (*client).buttons & !(*client).oldbuttons;
    (*ent).light_level = (*ucmd).lightlevel as libc::c_int;
    if (*client).latched_buttons & 1 as libc::c_int != 0 {
        if (*client).resp.spectator as u64 != 0 {
            (*client).latched_buttons = 0 as libc::c_int;
            if !((*client).chase_target).is_null() {
                let ref mut fresh47 = (*client).chase_target;
                *fresh47 = 0 as *mut edict_t;
                let ref mut fresh48 = (*client).ps.pmove.pm_flags;
                *fresh48 = (*fresh48 as libc::c_int & !(64 as libc::c_int)) as byte;
            } else {
                GetChaseTarget(ent);
            }
        } else if (*client).weapon_thunk as u64 == 0 {
            (*client).weapon_thunk = true_0;
            Think_Weapon(ent);
        }
    }
    if (*client).resp.spectator as u64 != 0 {
        if (*ucmd).upmove as libc::c_int >= 10 as libc::c_int {
            if (*client).ps.pmove.pm_flags as libc::c_int & 2 as libc::c_int == 0 {
                let ref mut fresh49 = (*client).ps.pmove.pm_flags;
                *fresh49 = (*fresh49 as libc::c_int | 2 as libc::c_int) as byte;
                if !((*client).chase_target).is_null() {
                    ChaseNext(ent);
                } else {
                    GetChaseTarget(ent);
                }
            }
        } else {
            let ref mut fresh50 = (*client).ps.pmove.pm_flags;
            *fresh50 = (*fresh50 as libc::c_int & !(2 as libc::c_int)) as byte;
        }
    }
    i = 1 as libc::c_int;
    while i as libc::c_float <= (*maxclients).value {
        other = g_edicts.offset(i as isize);
        if (*other).inuse as libc::c_uint != 0 && (*(*other).client).chase_target == ent
        {
            UpdateChaseCam(other);
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ClientBeginServerFrame(mut ent: *mut edict_t) {
    let mut client: *mut gclient_t = 0 as *mut gclient_t;
    let mut buttonMask: libc::c_int = 0;
    if level.intermissiontime != 0. {
        return;
    }
    client = (*ent).client;
    if (*deathmatch).value != 0.
        && (*client).pers.spectator as libc::c_uint
            != (*client).resp.spectator as libc::c_uint
        && level.time - (*client).respawn_time >= 5 as libc::c_int as libc::c_float
    {
        spectator_respawn(ent);
        return;
    }
    if (*client).weapon_thunk as u64 == 0 && (*client).resp.spectator as u64 == 0 {
        Think_Weapon(ent);
    } else {
        (*client).weapon_thunk = false_0;
    }
    if (*ent).deadflag != 0 {
        if level.time > (*client).respawn_time {
            if (*deathmatch).value != 0. {
                buttonMask = 1 as libc::c_int;
            } else {
                buttonMask = -(1 as libc::c_int);
            }
            if (*client).latched_buttons & buttonMask != 0
                || (*deathmatch).value != 0.
                    && (*dmflags).value as libc::c_int & 0x400 as libc::c_int != 0
            {
                respawn(ent);
                (*client).latched_buttons = 0 as libc::c_int;
            }
        }
        return;
    }
    if (*deathmatch).value == 0. {
        if visible(ent, PlayerTrail_LastSpot()) as u64 == 0 {
            PlayerTrail_Add(((*ent).s.old_origin).as_mut_ptr());
        }
    }
    (*client).latched_buttons = 0 as libc::c_int;
}
