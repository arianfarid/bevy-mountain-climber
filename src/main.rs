use avian2d::{ prelude::* };
use bevy::{input::mouse::MouseMotion, prelude::*, window::{CursorGrabMode, PrimaryWindow}};
use player::{*};
mod player;


pub const GRAVITY: Vec2 = Vec2::new(0., -10000.);
pub const ANG_VEC_DAMP: f32 = 300.;
pub const LIN_VEC_DAMP: f32 = 80.;

// Define the collision layers
#[derive(PhysicsLayer)]
pub enum Layer {
    Platform,
    Hold,
}


fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            PhysicsPlugins::default(),
        ))
        .insert_resource(ClearColor(Color::srgb(0.05, 0.05, 0.1)))
        .insert_resource(SubstepCount(50))
        .insert_resource(Gravity(GRAVITY))
        .init_state::<UserControlState>()
        .add_systems(Startup, (setup_controls, setup, spawn_holds))
        .add_systems(Update, (controls).chain().after(setup))
        .run();
}

fn setup_controls(mut q_windows: Query<&mut Window, With<PrimaryWindow>>,) {
    let mut primary_window = q_windows.single_mut();

    // if you want to use the cursor, but not let it leave the window,
    // use `Confined` mode:
    primary_window.cursor.grab_mode = CursorGrabMode::Confined;

    // for a game that doesn't use the cursor (like a shooter):
    // use `Locked` mode to keep the cursor in one place
    primary_window.cursor.grab_mode = CursorGrabMode::Locked;

    // also hide the cursor
    primary_window.cursor.visible = false;
}

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum UserControlState {
    LeftHand,
    #[default]
    RightHand,
    Pause
}
fn controls(
    mut commands: Commands,
    mut evr_motion: EventReader<MouseMotion>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut q_windows: Query<&mut Window, With<PrimaryWindow>>,
    mut torso_query: Query<(&mut Transform, &TorsoCore, Entity), (With<TorsoCore>, Without<LeftHand>, Without<RightHand>)>,
    mut right_hand_query: Query<(&mut Transform, &RightHand), (With<RightHand>, Without<LeftHand>, Without<TorsoCore>, Without<LeftShoulder>, Without<RightShoulder>)>,
    mut left_hand_query: Query<(&mut Transform, &LeftHand), (With<LeftHand>, Without<RightHand>, Without<TorsoCore>, Without<LeftShoulder>, Without<RightShoulder>)>,
    mut right_shoulder_query: Query<(&mut Transform, &RightShoulder), (With<RightShoulder>, Without<LeftShoulder>, Without<TorsoCore>, Without<LeftHand>, Without<RightHand>)>,
    mut left_shoulder_query: Query<(&mut Transform, &LeftShoulder), (With<LeftShoulder>, Without<RightShoulder>, Without<TorsoCore>, Without<LeftHand>, Without<RightHand>)>,
    mut collider_query: Query<&CollidingEntities>,
    mouse_buttons:Res<ButtonInput<MouseButton>>,
    user_state: Res<State<UserControlState>>,
    mut set_user_state: ResMut<NextState<UserControlState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        let mut primary_window = q_windows.single_mut();
        primary_window.cursor.grab_mode = CursorGrabMode::None;
        primary_window.cursor.visible = true;
        set_user_state.set(UserControlState::Pause)
    }
    if keyboard_input.just_pressed(KeyCode::KeyA) {
        set_user_state.set(UserControlState::LeftHand)
    } else if keyboard_input.just_pressed(KeyCode::KeyS) {
        set_user_state.set(UserControlState::RightHand)
    }
    let (mut torso_transform, _torso, _torso_core_entity) = torso_query.single_mut();
    let (mut left_hand_transform, _left_hand) = left_hand_query.single_mut();
    let (mut right_hand_transform, _right_hand) = right_hand_query.single_mut();
    let (left_shoulder_transform, _left_shoulder) = left_shoulder_query.single_mut();
    let (right_shoulder_transform, _right_shoulder) = right_shoulder_query.single_mut();
    
    // Move selected limb anchor. If mouse is clicked, the torso core is moved to mimick pulling motion.
    for ev in evr_motion.read() {
        if mouse_buttons.pressed(MouseButton::Left) {
            match user_state.get() {
                UserControlState::Pause => {
                    //
                }
                _ => {
                    let new_transform = Vec3::new(torso_transform.translation.x - ev.delta.x, torso_transform.translation.y + ev.delta.y, 1.);
                    let new_lshoulder_transform = Vec3::new(left_shoulder_transform.translation.x - ev.delta.x, left_shoulder_transform.translation.y + ev.delta.y, 1.);
                    let new_rshoulder_transform = Vec3::new(right_shoulder_transform.translation.x - ev.delta.x, right_shoulder_transform.translation.y + ev.delta.y, 1.);
                    let lwing =  (left_hand_transform.translation - new_lshoulder_transform).length();
                    let rwing: f32 =  (right_hand_transform.translation - new_rshoulder_transform).length();
                    if lwing > WINGSPAN || rwing > WINGSPAN {
                        //Do not move
                        println!("LIM");
                         println!("WINGSPAN: {:?}", WINGSPAN);
                        println!("RWINGSPAN: {:?}", rwing);
                        println!("LWINGSPAN: {:?}", lwing);
                    } else {
                        println!("WINGSPAN: {:?}", WINGSPAN);
                        println!("RWINGSPAN: {:?}", rwing);
                        println!("LWINGSPAN: {:?}", lwing);
                        torso_transform.translation = new_transform;
                    }
                }
            }
        } else {
            match user_state.get() {
                UserControlState::LeftHand => {
                    let new_transform = Vec3::new(left_hand_transform.translation.x + ev.delta.x, left_hand_transform.translation.y - ev.delta.y, 1.);
                    let lwing =  (left_shoulder_transform.translation - new_transform).length();
                    if lwing > WINGSPAN {
                        //Do not move
                    } else {
                        println!("WINGSPAN: {:?}", WINGSPAN);
                        println!("LWINGSPAN: {:?}", lwing);
                        left_hand_transform.translation = new_transform;
                    }
                }
                UserControlState::RightHand => {
                    let new_transform = Vec3::new(right_hand_transform.translation.x + ev.delta.x, right_hand_transform.translation.y - ev.delta.y, 1.);
                    let rwing =  (right_shoulder_transform.translation - new_transform).length();
                    if rwing > WINGSPAN {
                        //Do not move
                    } else {
                        println!("WINGSPAN: {:?}", WINGSPAN);
                        println!("RWINGSPAN: {:?}", rwing);
                        right_hand_transform.translation = new_transform;
                    }
                }
                UserControlState::Pause => {
                    //
                }
            }
            }
        
    }
}
#[derive(Component)]
struct Floor;
#[derive(Component)]
struct Hold;

fn spawn_holds(
    mut commands: Commands,
) {
    let square_sprite = Sprite {
        custom_size: Some(Vec2::splat(150.0)),
        ..default()
    };
    let _hold = commands
        .spawn((
            SpriteBundle {
                sprite: {
                    let mut ret = square_sprite.clone();
                    ret.color = Color::srgba(0.2, 0.2, 0.2, 1.);
                    ret.custom_size = Some(Vec2::new(100., 100.));
                    ret
                },
                transform: Transform::from_xyz(0., -0., 10.),
                ..default()
            },
            Hold,
            RigidBody::Static,
            Collider::rectangle(100., 100.,),
            CollisionLayers::new(Layer::Hold, Layer::Hold),
        ))
        .id();
}