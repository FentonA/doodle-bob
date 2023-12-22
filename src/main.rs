use bevy::prelude::*;


fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_startup_system(spawn_cam)
    .add_startup_system(spawn_player)
    .add_system(animate_sprite)
    .add_system(move_player)
    .add_system(change_player_animation)
    .add_system(player_jump)
    .add_system(player_fall)
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
        asset_server.load("dogpack_assets/dogpack_spritesheets/dog_idle_strip8.png"),
        Vec2::splat(60.),
        8, 1, None, None);
    commands.spawn((SpriteSheetBundle { 
        texture_atlas: texture_atlas.add(atlas),
        sprite: TextureAtlasSprite {index: 0, ..Default::default()},
        ..Default::default()
    }, 
    Player,
    SpriteAnimation{
        len: 8,
        frame_time: 1./20.
        },
        FrameTime(0.0)
    ));
}

#[derive(Component)]
struct FrameTime(f32);
# [derive(Component)]   
struct SpriteAnimation {
    len: usize, 
    frame_time: f32,
}



fn animate_sprite(
    mut query: Query<(&mut TextureAtlasSprite, &SpriteAnimation, &mut FrameTime)>,
    time: Res<Time>,
) {
    for (mut sprite, animation, mut frame_time) in query.iter_mut() {
        frame_time.0 += time.delta_seconds();
        if frame_time.0 > animation.frame_time {
            let frames = (frame_time.0/animation.frame_time) as usize;
            sprite.index += frames;
            if sprite.index >= animation.len {sprite.index %= animation.len; }
            frame_time.0 -= animation.frame_time * frames as f32;
        }
    }
}



const MOVE_SPEED: f32 = 100.; 
fn move_player (
    mut player: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
    input: Res<Input<KeyCode>>,
) {
    let mut player = player.single_mut();
    if input.any_pressed([KeyCode::A, KeyCode::Left]) {
        player.translation.x -= MOVE_SPEED * time.delta_seconds();
    } else if input.any_pressed ([KeyCode::D, KeyCode::Right]) {
        player.translation.x += MOVE_SPEED * time.delta_seconds();
    }
    if input.any_pressed([KeyCode::W, KeyCode::Up]) {
        player.translation.y += MOVE_SPEED * time.delta_seconds();
    } else if input.any_pressed([KeyCode::S, KeyCode::Down]) {
        player.translation.y -= MOVE_SPEED * time.delta_seconds();
    }   player.translation.y += MOVE_SPEED * time.delta_seconds();
}





fn change_player_animation(
    mut player: Query<(&mut Handle<TextureAtlas>, &mut SpriteAnimation, &mut TextureAtlasSprite), With <Player>>,
    input: Res<Input<KeyCode>>,
    mut texture_atlas: ResMut<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
) {
    let (mut atlas, mut animation, mut sprite) = player.single_mut();

    if input.any_just_pressed([KeyCode::A, KeyCode::Left, KeyCode::D, KeyCode::Right]) {
        animation.len = 8; 
        *atlas = texture_atlas.add(TextureAtlas::from_grid(
        asset_server.load("dogpack_assets/dogpack_spritesheets/dog_run_strip8.png"),
        Vec2::splat(60.),
        8,1,None,None
        ));
    }
    if  input.any_just_released([KeyCode::A, KeyCode::Left, KeyCode::D, KeyCode::Right])
    && !input.any_pressed([KeyCode::A, KeyCode::Left, KeyCode::D, KeyCode::Right]) {
        animation.len = 8; 
        *atlas = texture_atlas.add(TextureAtlas::from_grid(
            asset_server.load("dogpack_assets/dogpack_spritesheets/dog_idle_strip8.png"),
        Vec2::splat(60.),
        8,1,None,None
        ));
    }
    if input.any_just_pressed([KeyCode::A, KeyCode::Left]) {
        sprite.flip_x = true; 
    } else if input.any_just_pressed([KeyCode::D, KeyCode::Right])
    && !input.any_pressed([KeyCode::A, KeyCode::Left]) {
        sprite.flip_x = false; 
    } else if input.any_just_released([KeyCode::A, KeyCode::Left])
    && !input.any_pressed([KeyCode::A, KeyCode::Left])
    && input.any_pressed([KeyCode::D,KeyCode::Right]) {
        sprite.flip_x = false;
    }
}







#[derive(Component)]
struct Jump(f32);       
const FALL_SPEED:f32 = 130.;
fn player_jump(
    mut commands: Commands, time:Res<Time>,
    mut player: Query<(Entity, &mut Transform, &mut Jump), With <Player>>,
) {
    let Ok((player, mut transform, mut jump)) = player.get_single_mut() else {return;};
    let jump_power = (time.delta_seconds() * FALL_SPEED * 2.).min(jump.0);
    jump.0 -= jump_power; 
    transform.translation.y += jump_power; 
    if jump.0 == 0. {commands.entity(player).remove::<Jump>();}
}
fn player_fall (
    mut player: Query<&mut Transform, (With<Player>, Without<Jump>)>,
    time:Res<Time>,) { 
    let Ok(mut player) = player.get_single_mut() else {return;};
    if player.translation.y > 0.0 {
        player.translation.y -= time.delta_seconds() * FALL_SPEED; 
        if player.translation.y < 0.0 {player.translation.y = 0.0}
    }
    }
