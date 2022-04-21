#![feature(is_some_with)]
use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_terrain::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(TerrainPlugin)
        .add_startup_system(setup)
        .add_system(camera)
        .run();
}

fn setup(mut commands: Commands, ass: Res<AssetServer>) {
    commands.spawn_bundle(TerrainBundle {
        terrain: Terrain::new(
            GenerationType::WaveCollapse,
            UVec2::splat(50),
            Vec2::splat(16.0),
        )
        .with_module(TerrainModule {
            generation_rule: |adjacents| {
                adjacents
                    .e
                    .map(|module| [2, 3, 66, 86].contains(&module.id))
                    .unwrap_or(true)
                    && adjacents
                        .s
                        .map(|module| [22, 43, 66, 86].contains(&module.id))
                        .unwrap_or(true)
                    && adjacents
                        .n
                        .map(|module| [21, 42, 43, 44, 45].contains(&module.id))
                        .unwrap_or(true)
                    && adjacents
                        .w
                        .map(|module| [3, 21, 24, 42, 45].contains(&module.id))
                        .unwrap_or(true)
            },
            id: 1,
            image: ass.load("2d/tile001.png"),
        })
        .with_module(TerrainModule {
            generation_rule: |adjacents| {
                adjacents
                    .e
                    .map(|module| [2, 3, 66, 86].contains(&module.id))
                    .unwrap_or(true)
                    && adjacents
                        .s
                        .map(|module| [4, 23, 25, 44, 46, 64, 65].contains(&module.id))
                        .unwrap_or(true)
                    && adjacents
                        .n
                        .map(|module| [21, 42, 43, 44, 45].contains(&module.id))
                        .unwrap_or(true)
                    && adjacents
                        .w
                        .map(|module| [1, 2, 85, 87].contains(&module.id))
                        .unwrap_or(true)
            },
            id: 2,
            image: ass.load("2d/tile002.png"),
        })
        .with_module(TerrainModule {
            generation_rule: |adjacents| {
                adjacents
                    .e
                    .map(|module| [1, 21, 22, 42, 43].contains(&module.id))
                    .unwrap_or(true)
                    && adjacents
                        .s
                        .map(|module| [24, 45, 85, 87].contains(&module.id))
                        .unwrap_or(true)
                    && adjacents
                        .n
                        .map(|module| [21, 42, 43, 44, 45].contains(&module.id))
                        .unwrap_or(true)
                    && adjacents
                        .w
                        .map(|module| [1, 2, 85, 87].contains(&module.id))
                        .unwrap_or(true)
            },
            id: 3,
            image: ass.load("2d/tile003.png"),
        })
        .with_module(TerrainModule {
            generation_rule: |adjacents| {
                adjacents
                    .e
                    .map(|module| [4, 23, 24, 25, 46, 64, 85].contains(&module.id))
                    .unwrap_or(true)
                    && adjacents
                        .s
                        .map(|module| [4, 23, 25, 44, 46, 64, 65].contains(&module.id))
                        .unwrap_or(true)
                    && adjacents
                        .n
                        .map(|module| [2, 4, 23, 25, 46, 85, 86].contains(&module.id))
                        .unwrap_or(true)
                    && adjacents
                        .w
                        .map(|module| [4, 22, 23, 25, 46, 65, 86].contains(&module.id))
                        .unwrap_or(true)
            },
            id: 4,
            image: ass.load("2d/tile004.png"),
        })
        .with_module(TerrainModule {
            generation_rule: |adjacents| {
                adjacents
                    .e
                    .map(|module| [1, 21, 22, 42, 43].contains(&module.id))
                    .unwrap_or(true)
                    && adjacents
                        .s
                        .map(|module| [1, 2, 3, 21, 42].contains(&module.id))
                        .unwrap_or(true)
                    && adjacents
                        .n
                        .map(|module| [21, 42, 43, 44, 45].contains(&module.id))
                        .unwrap_or(true)
                    && adjacents
                        .w
                        .map(|module| [1, 21, 24, 42, 45].contains(&module.id))
                        .unwrap_or(true)
            },
            id: 21,
            image: ass.load("2d/tile021.png"),
        })
        .with_module(TerrainModule {
            generation_rule: |adjacents| {
                adjacents
                    .e
                    .map(|module| [4, 23, 24, 25, 46, 64, 85].contains(&module.id))
                    .unwrap_or(true)
                    && adjacents
                        .s
                        .map(|module| [22, 43, 66, 86].contains(&module.id))
                        .unwrap_or(true)
                    && adjacents
                        .n
                        .map(|module| [1, 22, 65, 87].contains(&module.id))
                        .unwrap_or(true)
                    && adjacents
                        .w
                        .map(|module| [3, 21, 24, 42, 45].contains(&module.id))
                        .unwrap_or(true)
            },
            id: 22,
            image: ass.load("2d/tile022.png"),
        })
        .with_module(TerrainModule {
            generation_rule: |adjacents| {
                adjacents
                    .e
                    .map(|module| [4, 23, 24, 25, 46].contains(&module.id))
                    .unwrap_or(true)
                    && adjacents
                        .s
                        .map(|module| [4, 23, 25, 44, 46].contains(&module.id))
                        .unwrap_or(true)
                    && adjacents
                        .n
                        .map(|module| [2, 4, 23, 25, 2, 46].contains(&module.id))
                        .unwrap_or(true)
                    && adjacents
                        .w
                        .map(|module| [4, 22, 23, 25, 46].contains(&module.id))
                        .unwrap_or(true)
            },
            id: 23,
            image: ass.load("2d/tile023.png"),
        })
        .with_module(TerrainModule {
            generation_rule: |adjacents| {
                adjacents
                    .e
                    .map(|module| [1, 21, 22, 42, 43].contains(&module.id))
                    .unwrap_or(true)
                    && adjacents
                        .s
                        .map(|module| [24, 45, 85, 87].contains(&module.id))
                        .unwrap_or(true)
                    && adjacents
                        .n
                        .map(|module| [1, 22, 65, 87].contains(&module.id))
                        .unwrap_or(true)
                    && adjacents
                        .w
                        .map(|module| [4, 22, 23, 25, 46, 65, 86].contains(&module.id))
                        .unwrap_or(true)
            },
            id: 24,
            image: ass.load("2d/tile024.png"),
        })
        .with_module(TerrainModule {
            generation_rule: |adjacents| {
                adjacents
                    .e
                    .map(|module| [44, 45, 65, 87].contains(&module.id))
                    .unwrap_or(true)
                    && adjacents
                        .s
                        .map(|module| [1, 2, 3, 21, 42].contains(&module.id))
                        .unwrap_or(true)
                    && adjacents
                        .n
                        .map(|module| [1, 22, 65, 87].contains(&module.id))
                        .unwrap_or(true)
                    && adjacents
                        .w
                        .map(|module| [3, 21, 24, 42, 45].contains(&module.id))
                        .unwrap_or(true)
            },
            id: 43,
            image: ass.load("2d/tile043.png"),
        })
        .with_module(TerrainModule {
            generation_rule: |adjacents| {
                adjacents
                    .e
                    .map(|module| [44, 45, 65, 87].contains(&module.id))
                    .unwrap_or(true)
                    && adjacents
                        .s
                        .map(|module| [1, 2, 3, 21, 42].contains(&module.id))
                        .unwrap_or(true)
                    && adjacents
                        .n
                        .map(|module| [2, 4, 23, 25, 46, 85, 86].contains(&module.id))
                        .unwrap_or(true)
                    && adjacents
                        .w
                        .map(|module| [43, 44, 64, 66].contains(&module.id))
                        .unwrap_or(true)
            },
            id: 44,
            image: ass.load("2d/tile044.png"),
        })
        .with_module(TerrainModule {
            generation_rule: |adjacents| {
                adjacents
                    .e
                    .map(|module| [1, 21, 22, 42, 43].contains(&module.id))
                    .unwrap_or(true)
                    && adjacents
                        .s
                        .map(|module| [1, 2, 3, 21, 42].contains(&module.id))
                        .unwrap_or(true)
                    && adjacents
                        .n
                        .map(|module| [1, 22, 65, 87].contains(&module.id))
                        .unwrap_or(true)
                    && adjacents
                        .w
                        .map(|module| [43, 44, 64, 66].contains(&module.id))
                        .unwrap_or(true)
            },
            id: 45,
            image: ass.load("2d/tile045.png"),
        })
        .with_module(TerrainModule {
            generation_rule: |adjacents| {
                adjacents
                    .e
                    .map(|module| [44, 45, 65, 87].contains(&module.id))
                    .unwrap_or(true)
                    && adjacents
                        .s
                        .map(|module| [24, 45, 85, 87].contains(&module.id))
                        .unwrap_or(true)
                    && adjacents
                        .n
                        .map(|module| [2, 4, 85, 86].contains(&module.id))
                        .unwrap_or(true)
                    && adjacents
                        .w
                        .map(|module| [4, 22, 65, 86].contains(&module.id))
                        .unwrap_or(true)
            },
            id: 64,
            image: ass.load("2d/tile064.png"),
        })
        .with_module(TerrainModule {
            generation_rule: |adjacents| {
                adjacents
                    .e
                    .map(|module| [4, 24, 64, 85].contains(&module.id))
                    .unwrap_or(true)
                    && adjacents
                        .s
                        .map(|module| [22, 43, 66, 86].contains(&module.id))
                        .unwrap_or(true)
                    && adjacents
                        .n
                        .map(|module| [2, 4, 85, 86].contains(&module.id))
                        .unwrap_or(true)
                    && adjacents
                        .w
                        .map(|module| [43, 44, 64, 66].contains(&module.id))
                        .unwrap_or(true)
            },
            id: 65,
            image: ass.load("2d/tile065.png"),
        })
        .with_module(TerrainModule {
            generation_rule: |adjacents| {
                adjacents
                    .e
                    .map(|module| [44, 45, 65, 87].contains(&module.id))
                    .unwrap_or(true)
                    && adjacents
                        .s
                        .map(|module| [24, 45, 85, 87].contains(&module.id))
                        .unwrap_or(true)
                    && adjacents
                        .n
                        .map(|module| [1, 22, 65, 87].contains(&module.id))
                        .unwrap_or(true)
                    && adjacents
                        .w
                        .map(|module| [1, 2, 85, 87].contains(&module.id))
                        .unwrap_or(true)
            },
            id: 66,
            image: ass.load("2d/tile066.png"),
        })
        .with_module(TerrainModule {
            generation_rule: |adjacents| {
                adjacents
                    .e
                    .map(|module| [2, 3, 66, 86].contains(&module.id))
                    .unwrap_or(true)
                    && adjacents
                        .s
                        .map(|module| [4, 44, 64, 65].contains(&module.id))
                        .unwrap_or(true)
                    && adjacents
                        .n
                        .map(|module| [3, 24, 64, 66].contains(&module.id))
                        .unwrap_or(true)
                    && adjacents
                        .w
                        .map(|module| [4, 22, 65, 86].contains(&module.id))
                        .unwrap_or(true)
            },
            id: 85,
            image: ass.load("2d/tile085.png"),
        })
        .with_module(TerrainModule {
            generation_rule: |adjacents| {
                adjacents
                    .e
                    .map(|module| [4, 24, 64, 85].contains(&module.id))
                    .unwrap_or(true)
                    && adjacents
                        .s
                        .map(|module| [4, 44, 64, 65].contains(&module.id))
                        .unwrap_or(true)
                    && adjacents
                        .n
                        .map(|module| [1, 22, 65, 87].contains(&module.id))
                        .unwrap_or(true)
                    && adjacents
                        .w
                        .map(|module| [1, 2, 85, 87].contains(&module.id))
                        .unwrap_or(true)
            },
            id: 86,
            image: ass.load("2d/tile086.png"),
        })
        .with_module(TerrainModule {
            generation_rule: |adjacents| {
                adjacents
                    .e
                    .map(|module| [2, 3, 66, 86].contains(&module.id))
                    .unwrap_or(true)
                    && adjacents
                        .s
                        .map(|module| [22, 43, 66, 86].contains(&module.id))
                        .unwrap_or(true)
                    && adjacents
                        .n
                        .map(|module| [3, 24, 64, 66].contains(&module.id))
                        .unwrap_or(true)
                    && adjacents
                        .w
                        .map(|module| [43, 44, 64, 66].contains(&module.id))
                        .unwrap_or(true)
            },
            id: 87,
            image: ass.load("2d/tile087.png"),
        }),
        transform: Transform::from_xyz(-25.0 * 16.0 * 4.0, 25.0 * 16.0 * 4.0, 0.0)
            .with_scale(Vec2::splat(4.0).extend(0.0)),
        ..default()
    });

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn camera(
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut camera: Query<&mut Transform, With<Camera>>,
) {
    let mut direction = Vec2::splat(0.0);
    if keys.pressed(KeyCode::A) {
        direction.x = -1.0;
    }
    if keys.pressed(KeyCode::E) {
        direction.x = 1.0;
    }
    if keys.pressed(KeyCode::O) {
        direction.y = -1.0;
    }
    if keys.pressed(KeyCode::Comma) {
        direction.y = 1.0;
    }
    if direction.length() != 0.0 {
        let direction = direction.normalize() * time.delta_seconds() * 500.0;
        camera.single_mut().translation += direction.extend(0.0);
    }
}
