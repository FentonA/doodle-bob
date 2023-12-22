use bevy::prelude::*;



fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_plugin(EditorPlugin)
    .add_startup 
    .run()
}

fn spawn_cam(
    mut commands: Commands,
) { 
    commands.spawn(Camera2dBundle::default());
}

#[derive(Component)]
struct Player;

fn spawn_player(
    mut commands: Commands,
    mut texture_atlas: ResMut<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
 ) {
        let atlas = TextureAtlas::from_grid(
        asset_server.load("dogpack_spritesheets/dog_idle_strip8.png"),
        Vec2::splat(60.),
        8, 1, None, None);
    commands.spawn((SpriteSheetBundle { 
        texture_atlas: texture_atlas.add(atlas),
        sprite: TextureAtlasSprite {index: 0, ..Default::default()},
        ..Default::default()
    }, Player));
}