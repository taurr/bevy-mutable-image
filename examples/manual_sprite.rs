use bevy::{
    prelude::*,
    render::{camera::ScalingMode, texture::ImageSettings},
};
use bevy_mutate_image::MutableImageHandle;
use image::{GenericImage, GenericImageView, Rgba};

const WIDTH: f32 = 640.0;
const HEIGHT: f32 = 360.0;
const VIEWPORT_WIDTH: f32 = 16.0;
const VIEWPORT_HEIGHT: f32 = 9.0;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: format!(
                "{} - v{}",
                env!("CARGO_PKG_NAME"),
                env!("CARGO_PKG_VERSION")
            ),
            width: WIDTH,
            height: HEIGHT,
            ..default()
        })
        .insert_resource(ImageSettings::default_nearest())
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(update_mutable_image)
        .run();
}

fn setup(mut commands: Commands, mut assets: ResMut<Assets<Image>>) {
    // Spawn camera
    commands.spawn_bundle(Camera2dBundle {
        projection: OrthographicProjection {
            scaling_mode: ScalingMode::Auto {
                min_width: VIEWPORT_WIDTH,
                min_height: VIEWPORT_HEIGHT,
            },
            ..Default::default()
        },
        ..Default::default()
    });

    // Spawn sprite using a mutable image as texture
    let component =
        MutableImageHandle::new(VIEWPORT_WIDTH as u32, VIEWPORT_HEIGHT as u32, &mut assets);
    commands
        .spawn_bundle(SpriteBundle {
            texture: component.image_handle(),
            ..Default::default()
        })
        .insert(component);
}

fn update_mutable_image(
    query: Query<&MutableImageHandle>,
    mut assets: ResMut<Assets<Image>>,
    mut blue: Local<i16>,
    mut blue_decreasing: Local<bool>,
) {
    if *blue_decreasing {
        *blue -= 1;
    } else {
        *blue += 3;
    };
    if *blue > 255 {
        *blue = 255;
        *blue_decreasing = true;
    } else if *blue < 0 {
        *blue = 0;
        *blue_decreasing = false;
    }

    for image_component in &query {
        if let Some(mut image) = image_component.image(&mut assets) {
            let (left, top, right, bottom) = image.bounds();
            for x in left..right {
                let red = (x * 255 / image.width()) as u8;
                for y in top..bottom {
                    let green = (y * 255 / image.height()) as u8;
                    image.put_pixel(x, y, Rgba([red, green, *blue as u8, 255]));
                }
            }
        }
    }
}
