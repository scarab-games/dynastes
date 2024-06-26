use bevy::prelude::*;
use dynastes::{
    bevy::{
        BevyASM, BevyFrameSource, DynastesAnimationBundle, MaybeBevyStateInstance,
        SpriteAnimationPlugin, TextureAtlasGridMetadata,
    },
    state_machine::StateID,
    states::index::IndexState,
};

const COMMON_MSPF: f64 = 1000. / 8.;

fn main() {
    env_logger::init();

    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin {
            default_sampler: bevy::render::texture::ImageSampler::nearest_descriptor(),
        }))
        .add_plugins(SpriteAnimationPlugin)
        .add_systems(Startup, setup_animations)
        .run()
}

fn setup_animations(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut sprites: ResMut<Assets<TextureAtlas>>,
    mut state_machines: ResMut<Assets<BevyASM>>,
) {
    commands.spawn(Camera2dBundle::default());

    let fs = BevyFrameSource {
        path: "sprite-sheet.png".into(),
        metadata: TextureAtlasGridMetadata {
            tile_size: [128., 128.].into(),
            columns: 26,
            rows: 2,
            padding: None,
            offset: None,
        },
    };

    let texture_handle = asset_server.load(fs.path.clone());
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        fs.metadata.tile_size,
        fs.metadata.columns,
        fs.metadata.rows,
        fs.metadata.padding,
        fs.metadata.offset,
    );
    let texture_atlas_handle = sprites.add(texture_atlas);

    walk_animation_with_fluidity(
        &mut commands,
        &mut state_machines,
        texture_atlas_handle.clone(),
        (-60., 0., 0.).into(),
        None,
    );
    walk_animation_with_fluidity(
        &mut commands,
        &mut state_machines,
        texture_atlas_handle.clone(),
        (60., 0., 0.).into(),
        Some(1. / 2.),
    );
}

fn walk_animation_with_fluidity(
    commands: &mut Commands,
    state_machines: &mut ResMut<Assets<BevyASM>>,
    texture_atlas_handle: Handle<TextureAtlas>,
    position: Vec3,
    fluidity: Option<f64>,
) {
    let walk_id: StateID = "walk".to_string().into();
    let idle_id: StateID = "idle".to_string().into();

    let walk_state: IndexState<TextureAtlasSprite> =
        IndexState::new(0, 9, COMMON_MSPF, Some(idle_id.clone()), None, fluidity);
    let idle_state: IndexState<TextureAtlasSprite> =
        IndexState::new(26, 51, COMMON_MSPF, Some(walk_id.clone()), None, fluidity);

    let mut asm = BevyASM::new(texture_atlas_handle.clone(), idle_id, idle_state);
    asm.0.add_states(vec![(walk_id, walk_state)]);

    let asm_handle = state_machines.add(asm);
    let scale = 4.;

    commands.spawn(DynastesAnimationBundle {
        state_machine: asm_handle,
        animation_state: MaybeBevyStateInstance::default(),
        sprite_sheet: SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(0),
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_translation(position).with_scale(Vec3::splat(scale)),
            ..Default::default()
        },
    });
}
