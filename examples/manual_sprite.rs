use bevy::{prelude::*, render::camera::ScalingMode};
use bevy_mutate_image::MutableImageHandle;
use image::{GenericImage, GenericImageView, Rgba};

const WIDTH: f32 = 640.0;
const HEIGHT: f32 = 360.0;
const VIEWPORT_WIDTH: f32 = 16.0;
const VIEWPORT_HEIGHT: f32 = 9.0;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: format!(
                            "{} - v{}",
                            env!("CARGO_PKG_NAME"),
                            env!("CARGO_PKG_VERSION")
                        ),
                        resolution: Vec2::new(WIDTH, HEIGHT).into(),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_systems(Startup, setup)
        .add_systems(Update, update_mutable_image)
        .run();
}

fn setup(mut commands: Commands, mut assets: ResMut<Assets<Image>>) {
    // Spawn camera
    commands.spawn(Camera2dBundle {
        projection: OrthographicProjection {
            near: -1000.,
            far: 1000.,
            scaling_mode: ScalingMode::AutoMin {
                min_width: VIEWPORT_WIDTH,
                min_height: VIEWPORT_HEIGHT,
            },
            ..default()
        },
        ..default()
    });

    // Spawn sprite using a mutable image as texture
    let component =
        MutableImageHandle::new(VIEWPORT_WIDTH as u32, VIEWPORT_HEIGHT as u32, &mut assets);
    commands
        .spawn(SpriteBundle {
            texture: component.image_handle(),
            ..default()
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
