#![allow(unused)]
use std::collections::HashMap;

use bevy::{log::LogPlugin, prelude::*};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(LogPlugin {
            filter: "warn,skalla-salong=trace".into(),
            ..default()
        }))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut cmd: Commands, asset_server: Res<AssetServer>) {
    cmd.spawn(Camera2dBundle::default());

    use HairState::*;

    let hair_images = [
        (Neutral, "n"),
        (Left1, "l1"),
        (Left2, "l2"),
        (Left3, "l3"),
        (Right1, "r1"),
        (Right2, "r2"),
        (Right3, "r3"),
        (Horizontal1, "h1"),
        (Horizontal2, "h2"),
        (Horizontal3, "h3"),
        (Vertical1, "v1"),
        (Vertical2, "v2"),
        (Vertical3, "v3"),
    ]
    .into_iter()
    .map(|(k, v)| (k, asset_server.load("hår-1-".to_owned() + v + ".png")))
    .collect::<HashMap<HairState, Handle<Image>>>();

    cmd.spawn(SpriteBundle {
        texture: asset_server.load("kunde-1.png"),
        transform: Transform {
            scale: [0.5, 0.5, 1.].into(),
            ..default()
        },
        ..default()
    })
    .with_children(|parent| {
        parent.spawn(SpriteBundle {
            texture: hair_images[&Neutral].clone(),
            transform: Transform {
                translation: [0., 0., 1.].into(),
                ..default()
            },
            ..default()
        });
    });
    cmd.insert_resource(HairImages(hair_images));
}

#[derive(PartialEq, Eq, Hash)]
enum HairState {
    Neutral,
    Left1,
    Left2,
    Left3,
    Right1,
    Right2,
    Right3,
    Horizontal1,
    Horizontal2,
    Horizontal3,
    Vertical1,
    Vertical2,
    Vertical3,
}

#[derive(Resource)]
struct HairImages(HashMap<HairState, Handle<Image>>);
