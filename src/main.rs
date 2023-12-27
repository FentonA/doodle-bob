use bevy::prelude::*;


fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_startup_system(spawn_cam)
    .add_startup_system(spawn_background)
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

fn spawn_background(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let texture_handle = asset_server.load("Fantasy Swamp Forest/Free/BG_1/BG_1.png");
    commands.spawn(SpriteBundle {
        texture: texture_handle,
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, -1.0)),
        ..Default::default()
    });
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
        8, 1, None, None
    );
    commands.spawn(SpriteSheetBundle { 
        texture_atlas: texture_atlas.add(atlas),
        sprite: TextureAtlasSprite { index: 0, ..Default::default() },
        ..Default::default()
    })
    .insert(Player)
    .insert(SpriteAnimation {
        len: 8,
        frame_time: 1./20.
    })
    .insert(FrameTime(0.0));
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



const JUMP_SPEED: f32 = 180.0; // Adjust this value for jump height
const FALL_SPEED: f32 = 300.0; // Adjust this value for falling speed
const MOVE_SPEED: f32 = 220.0; // Adjust this value for movement speed

// Modified move_player function to include jump initiation
fn move_player(
    mut commands: Commands,
    mut player_query: Query<(Entity, &mut Transform), With<Player>>,
    jump_query: Query<Entity, With<Jump>>,
    time: Res<Time>,
    input: Res<Input<KeyCode>>,
) {
    if let Ok((player_entity, mut transform)) = player_query.get_single_mut() {
        // Existing movement code...
        if input.any_pressed([KeyCode::A, KeyCode::Left]) {
            transform.translation.x -= MOVE_SPEED * time.delta_seconds();
        } else if input.any_pressed([KeyCode::D, KeyCode::Right]) {
            transform.translation.x += MOVE_SPEED * time.delta_seconds();
        }

        // Check if the player entity already has the Jump component
        let player_has_jump = jump_query.get(player_entity).is_ok();

        // Jump initiation
        if (input.just_pressed(KeyCode::Space) || input.just_pressed(KeyCode::W) || input.just_pressed(KeyCode::Up))
            && !player_has_jump
        {
            commands.entity(player_entity).insert(Jump(JUMP_SPEED));
        }
    }
}

// player_jump function for handling jump physics
fn player_jump(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Transform, &mut Jump), With<Player>>,
) {
    for (entity, mut transform, mut jump) in query.iter_mut() {
        transform.translation.y += jump.0 * time.delta_seconds();
        jump.0 -= FALL_SPEED * time.delta_seconds();

        if jump.0 <= 0.0 {
            commands.entity(entity).remove::<Jump>();
        }
    }
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
    if  input.any_just_released([KeyCode::A, KeyCode::Left, KeyCode::D, KeyCode::Right, KeyCode::Up])
    && !input.any_pressed([KeyCode::A, KeyCode::Left, KeyCode::D, KeyCode::Right]) {
        animation.len = 8; 
        *atlas = texture_atlas.add(TextureAtlas::from_grid(
            asset_server.load("dogpack_assets/dogpack_spritesheets/dog_idle_strip8.png"),
        Vec2::splat(60.),
        8,1,None,None
        ));
    }
    else if input.any_just_pressed([KeyCode::Up]) {
        animation.len = 8; 
        *atlas = texture_atlas.add(TextureAtlas::from_grid(
        asset_server.load("dogpack_assets/dogpack_spritesheets/dog_jump_strip8.png"),
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

fn player_fall (
    mut player: Query<&mut Transform, (With<Player>, Without<Jump>)>,
    time:Res<Time>,) { 
    let Ok(mut player) = player.get_single_mut() else {return;};
    if player.translation.y > 0.0 {
        player.translation.y -= time.delta_seconds() * FALL_SPEED; 
        if player.translation.y < 0.0 {player.translation.y = 0.0} {
        }
    }
}
