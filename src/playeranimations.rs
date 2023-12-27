

// #[derive(Resource)]
// struct PlyaerAnimations { 
//     map: HashMap<Animation, (Handle<TextureAtlas>, SpriteAnimation)>,
// }
// impl FromWorld for PlayerAnimations {
//     fn from_world(World: &mut World) -> Self {
//         let mut map = PlayerAnimations {map: HashMap::new()};
//         let asset_server = world.resource::<AssetServer>();
//         let idel_atlas =  texture_atlas.add(TextureAtlas::from_grid(
//             asset_server.load("dogpack_assets/dogpack_spritesheets/dog_run_strip8.png"),
//             Vec2::splat(60.),
//             8,1,None,None));
//         let run_atlas =  texture_atlas.add(TextureAtlas::from_grid(
//             asset_server.load("dogpack_assets/dogpack_spritesheets/dog_dash_strip8.png"),
//             Vec2::splat(60.),
//             8,1,None,None));
//         let jump_atlas = texture_atlas.add(TextureAtlas::from_grid(
//             asset_server.load("dogpack_assets/dogpack_spritesheets/dog_jump_strip8.png"),
//             Vec2::splat(60.),
//             8,1,None,None));
//         let fall_atlas = texture_atlas.add(TextureAtlas::from_grid(
//             asset_server.load("dogpack_assets/dogpack_spritesheets/dog_fall_strip5.png"),
//             Vec2::splat(60.),
//             5,1,None,None));
//         let mut texture_atles = world.resource_mut::<Assets<TextureAtlas>>();
//         map.add(Animation::Idle, texture_atles.add(idel_atlas),
//         SpriteAnimation{ len: 8, frame_time: 1./20.});
//         map.add(Animation::Idle, texture_atles.add(run_atlas),
//         SpriteAnimation{ len: 8, frame_time: 1./20.});
//         map.add(Animation::Idle, texture_atles.add(jump_atlas),
//         SpriteAnimation{ len: 8, frame_time: 1./20.});
//         map.add(Animation::Idle, texture_atles.add(fall_atlas),
//         SpriteAnimation{ len: 5, frame_time: 1./20.});
//
//     }
// }
//
//
