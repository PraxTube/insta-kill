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
        paths("player/player.trickfilm#idle", "player/player.trickfilm#moving",),
        collection(typed)
    )]
    pub player_animations: Vec<Handle<AnimationClip2D>>,

    #[asset(texture_atlas(tile_size_x = 80.0, tile_size_y = 80.0, columns = 3, rows = 1))]
    #[asset(path = "player/player_strike.png")]
    pub player_strike: Handle<TextureAtlas>,
    #[asset(paths("player/player_strike.trickfilm#main"), collection(typed))]
    pub player_strike_animations: Vec<Handle<AnimationClip2D>>,

    #[asset(path = "player/player_shadow.png")]
    pub player_shadow: Handle<Image>,

    // --- ENEMY ---
    #[asset(texture_atlas(tile_size_x = 96.0, tile_size_y = 80.0, columns = 8, rows = 2))]
    #[asset(path = "enemy/enemy.png")]
    pub enemy: Handle<TextureAtlas>,
    #[asset(
        paths("enemy/enemy.trickfilm#flying", "enemy/enemy.trickfilm#dying",),
        collection(typed)
    )]
    pub enemy_animations: Vec<Handle<AnimationClip2D>>,

    #[asset(path = "enemy/enemy_shadow.png")]
    pub enemy_shadow: Handle<Image>,

    // --- MAP ---
    #[asset(path = "map/level.ldtk")]
    pub level: Handle<LdtkProject>,

    // --- UI ---
    #[asset(path = "ui/white_pixel.png")]
    pub white_pixel: Handle<Image>,

    // --- FONT ---
    #[asset(path = "fonts/PressStart2P.ttf")]
    pub font: Handle<Font>,
}
