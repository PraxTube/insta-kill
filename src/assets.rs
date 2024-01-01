use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_ecs_ldtk::prelude::LdtkProject;
// use bevy_kira_audio::AudioSource;
use bevy_trickfilm::prelude::*;

#[derive(AssetCollection, Resource)]
pub struct GameAssets {
    // --- PLAYER ---
    #[asset(texture_atlas(tile_size_x = 80.0, tile_size_y = 80.0, columns = 23, rows = 5))]
    #[asset(path = "player/player.png")]
    pub player: Handle<TextureAtlas>,
    #[asset(
        paths(
            "player/player.trickfilm#idle",
            "player/player.trickfilm#moving",
            "player/player.trickfilm#dashing",
            "player/player.trickfilm#hooking",
            "player/player.trickfilm#sliding",
        ),
        collection(typed)
    )]
    pub player_animations: Vec<Handle<AnimationClip2D>>,

    #[asset(texture_atlas(tile_size_x = 80.0, tile_size_y = 80.0, columns = 3, rows = 1))]
    #[asset(path = "player/player_strike.png")]
    pub player_strike: Handle<TextureAtlas>,
    #[asset(paths("player/player_strike.trickfilm#main"), collection(typed))]
    pub player_strike_animations: Vec<Handle<AnimationClip2D>>,

    #[asset(texture_atlas(tile_size_x = 320.0, tile_size_y = 10.0, columns = 1, rows = 5))]
    #[asset(path = "player/player_hook.png")]
    pub player_hook: Handle<TextureAtlas>,
    #[asset(paths("player/player_hook.trickfilm#main"), collection(typed))]
    pub player_hook_animations: Vec<Handle<AnimationClip2D>>,

    #[asset(path = "player/player_shadow.png")]
    pub player_shadow: Handle<Image>,

    // --- ENEMY ---
    #[asset(texture_atlas(tile_size_x = 34.0, tile_size_y = 34.0, columns = 7, rows = 1))]
    #[asset(path = "enemy/enemy_hit.png")]
    pub enemy_hit: Handle<TextureAtlas>,
    #[asset(paths("enemy/enemy_hit.trickfilm#main"), collection(typed))]
    pub enemy_hit_animations: Vec<Handle<AnimationClip2D>>,

    #[asset(texture_atlas(tile_size_x = 96.0, tile_size_y = 80.0, columns = 8, rows = 2))]
    #[asset(path = "enemy/bat/bat.png")]
    pub enemy_bat: Handle<TextureAtlas>,
    #[asset(
        paths("enemy/bat/bat.trickfilm#flying", "enemy/bat/bat.trickfilm#dying",),
        collection(typed)
    )]
    pub enemy_bat_animations: Vec<Handle<AnimationClip2D>>,

    #[asset(path = "enemy/bat/bat_shadow.png")]
    pub enemy_bat_shadow: Handle<Image>,

    // --- MAP ---
    #[asset(path = "map/level.ldtk")]
    pub level: Handle<LdtkProject>,

    // --- EFFECTS ---
    #[asset(texture_atlas(tile_size_x = 64.0, tile_size_y = 64.0, columns = 9, rows = 1))]
    #[asset(path = "effects/super_sonic.png")]
    pub super_sonic: Handle<TextureAtlas>,
    #[asset(paths("effects/super_sonic.trickfilm#main"), collection(typed))]
    pub super_sonic_animations: Vec<Handle<AnimationClip2D>>,

    // --- UI ---
    #[asset(path = "ui/white_pixel.png")]
    pub white_pixel: Handle<Image>,

    // --- FONT ---
    #[asset(path = "fonts/PressStart2P.ttf")]
    pub font: Handle<Font>,
}
