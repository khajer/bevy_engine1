use bevy::{
    // input::mouse::{AccumulatedMouseMotion, AccumulatedMouseScroll},
    prelude::*,
    window::WindowResolution,
};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.2, 0.2, 0.2))) // Set background color
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(1280.0, 720.0),
                title: "Cozy Corner".to_string(),
                resizable: true, // Disable resizing
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_systems(Update, mouse_click_system)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn((SpriteBundle {
        texture: asset_server.load("backgrounds/0.png"),
        transform: Transform::from_xyz(0., 0., 0.),
        ..default()
    },));
}

fn mouse_click_system(
    mut mouse_button_input: Res<ButtonInput<MouseButton>>,
    mut mouse_button_input_events: EventReader<MouseButtonInput>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut cursor_moved_events: EventReader<CursorMoved>,

    // cursor: Res<Cursor>,
) {
    if mouse_button_input.pressed(MouseButton::Left) {
        info!("left mouse currently pressed");
        
    }

    if mouse_button_input.just_pressed(MouseButton::Left) {
        info!("left mouse just pressed");
    }

    if mouse_button_input.just_released(MouseButton::Left) {
        info!("left mouse just released");
    }
}

//! Prints mouse button events.
//! Prints all mouse events to the console.

// use bevy::{
//     input::{
//         gestures::*,
//         mouse::{MouseButtonInput, MouseMotion, MouseWheel},
//     },
//     prelude::*,
// };

// fn main() {
//     App::new()
//         .add_plugins(DefaultPlugins)
//         .add_systems(Update, print_mouse_events_system)
//         .run();
// }

// /// This system prints out all mouse events as they come in
// fn print_mouse_events_system(
//     mut mouse_button_input_events: EventReader<MouseButtonInput>,
//     mut mouse_motion_events: EventReader<MouseMotion>,
//     mut cursor_moved_events: EventReader<CursorMoved>,
//     mut mouse_wheel_events: EventReader<MouseWheel>,
//     mut pinch_gesture_events: EventReader<PinchGesture>,
//     mut rotation_gesture_events: EventReader<RotationGesture>,
//     mut double_tap_gesture_events: EventReader<DoubleTapGesture>,
// ) {
//     for event in mouse_button_input_events.read() {
//         info!("{:?}", event);
//     }

//     for event in mouse_motion_events.read() {
//         info!("{:?}", event);
//     }

//     for event in cursor_moved_events.read() {
//         info!("{:?}", event);
//     }

//     for event in mouse_wheel_events.read() {
//         info!("{:?}", event);
//     }

//     // This event will only fire on macOS
//     for event in pinch_gesture_events.read() {
//         info!("{:?}", event);
//     }

//     // This event will only fire on macOS
//     for event in rotation_gesture_events.read() {
//         info!("{:?}", event);
//     }

//     // This event will only fire on macOS
//     for event in double_tap_gesture_events.read() {
//         info!("{:?}", event);
//     }
// }
