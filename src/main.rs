use avian2d::{ prelude::* };
use bevy::{prelude::*, window::PrimaryWindow};


pub const HEAD_SIZE: Vec2 = Vec2::new(20.0, 25.);
pub const HAND_SIZE: Vec2 = Vec2::new(15.0, 20.);
pub const BICEP_SIZE: Vec2 = Vec2::new(12.5, 25.);
pub const ARM_SIZE: Vec2 = Vec2::new(12.5, 25.,);
pub const TORSO_SIZE: Vec2 = Vec2::new(30.0, 50.);
pub const THIGH_SIZE: Vec2 = Vec2::new(20.,50.);
pub const SHIN_SIZE: Vec2 = Vec2::new(17.5, 50.);
pub const FOOT_SIZE: Vec2 = Vec2::new(20., 30.);

pub const GRAVITY: Vec2 = Vec2::new(0., -10000.);
pub const ANG_VEC_DAMP: f32 = 300.;

// Define the collision layers
#[derive(PhysicsLayer)]
enum Layer {
    Platform,
    // Red,
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
        .add_systems(Startup, setup)
        .add_systems(Update, (move_right_arm, collision).chain())
        .run();
}
#[derive(Component)]
pub struct MainCamera;
fn setup(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), MainCamera));

    let square_sprite = Sprite {
        color: Color::srgb(0.2, 0.2, 0.9),
        custom_size: Some(Vec2::splat(150.0)),
        ..default()
    };


    let head = commands
        .spawn((
            SpriteBundle {
                sprite: {
                    let mut ret = square_sprite.clone();
                    ret.color = Color::srgba(0.9, 0.2, 0.2, 0.5);
                    ret.custom_size = Some(HEAD_SIZE);
                    ret
                },
                transform: Transform::from_xyz(0.0, 100.0, 0.0),
                ..default()
            },
            Head,
            RigidBody::Dynamic,
            MassPropertiesBundle::new_computed(&Collider::rectangle(HEAD_SIZE.x, HEAD_SIZE.y), 1.0),
        ))
        .id();

    let right_hand = commands
        .spawn((
            SpriteBundle {
                sprite: {
                    let mut ret = square_sprite.clone();
                    ret.color = Color::srgba(0.2, 0.7, 0.2, 0.5);
                    ret.custom_size = Some(HAND_SIZE);
                    ret
                },
                ..default()
            },
            RightHand,
            RigidBody::Kinematic,
            MassPropertiesBundle::new_computed(&Collider::rectangle(HAND_SIZE.x, HAND_SIZE.y), 1.0),
        ))
        .id();
    let left_hand = commands
        .spawn((
            SpriteBundle {
                sprite: {
                    let mut ret = square_sprite.clone();
                    ret.color = Color::srgba(0.2, 0.7, 0.2, 0.5);
                    ret.custom_size = Some(HAND_SIZE);
                    ret
                },
                ..default()
            },
            LeftHand,
            RigidBody::Dynamic,
            MassPropertiesBundle::new_computed(&Collider::rectangle(HAND_SIZE.x, HAND_SIZE.y), 1.0),
        ))
        .id();
    

    let right_arm = commands
        .spawn((
            SpriteBundle {
                sprite: {
                    let mut ret = square_sprite.clone();
                    ret.color = Color::srgba(0.2, 0.7, 0.2, 0.5);
                    ret.custom_size = Some(ARM_SIZE);
                    ret
                },
                ..default()
            },
            RightArm,
            RigidBody::Dynamic,
            MassPropertiesBundle::new_computed(&Collider::rectangle(ARM_SIZE.x, ARM_SIZE.y), 1.0),
        ))
        .id();
    let left_arm = commands
    .spawn((
        SpriteBundle {
            sprite: {
                let mut ret = square_sprite.clone();
                ret.color = Color::srgba(0.2, 0.7, 0.2, 0.5);
                ret.custom_size = Some(ARM_SIZE);
                ret
            },
            ..default()
        },
        LeftArm,
        RigidBody::Dynamic,
        Collider::rectangle(ARM_SIZE.x, ARM_SIZE.y),
        MassPropertiesBundle::new_computed(&Collider::rectangle(ARM_SIZE.x, ARM_SIZE.y), 1.0),
    ))
    .id();

    
    let right_bicep = commands
        .spawn((
            SpriteBundle {
                sprite: {
                    let mut ret = square_sprite.clone();
                    ret.color = Color::srgba(0.7, 0.5, 0.1, 0.5);
                    ret.custom_size = Some(BICEP_SIZE);
                    ret
                },
                ..default()
            },
            RightBicep,
            RigidBody::Dynamic,
            MassPropertiesBundle::new_computed(&Collider::rectangle(BICEP_SIZE.x, BICEP_SIZE.y), 1.0),
        ))
        .id();
    let left_bicep = commands
        .spawn((
            SpriteBundle {
                sprite: {
                    let mut ret = square_sprite.clone();
                    ret.color = Color::srgba(0.7, 0.5, 0.1, 0.5);
                    ret.custom_size = Some(BICEP_SIZE);
                    ret
                },
                ..default()
            },
            LeftBicep,
            RigidBody::Dynamic,
            MassPropertiesBundle::new_computed(&Collider::rectangle(BICEP_SIZE.x, BICEP_SIZE.y), 1.0),
        ))
        .id();

    let torso = commands
        .spawn((
            SpriteBundle {
                sprite: {
                    let mut ret = square_sprite.clone();
                    ret.color = Color::srgba(0.1, 0.1, 0.9, 0.5);
                    ret.custom_size = Some(TORSO_SIZE);
                    ret
                },
                transform: Transform::from_xyz(0.0, -100.0, 0.0),
                ..default()
            },
            RigidBody::Dynamic,
            MassPropertiesBundle::new_computed(&Collider::rectangle(TORSO_SIZE.x, TORSO_SIZE.y), 1.0),
        ))
        .id();

    let right_thigh = commands
        .spawn((
            SpriteBundle {
                sprite: {
                    let mut ret = square_sprite.clone();
                    ret.color = Color::srgba(0.1, 0.5, 0.5, 0.5);
                    ret.custom_size = Some(THIGH_SIZE);
                    ret
                },
                ..default()
            },
            RightThigh,
            RigidBody::Dynamic,
            MassPropertiesBundle::new_computed(&Collider::rectangle(BICEP_SIZE.x, BICEP_SIZE.y), 1.0),
        ))
        .id();
    let left_thigh = commands
        .spawn((
            SpriteBundle {
                sprite: {
                    let mut ret = square_sprite.clone();
                    ret.color = Color::srgba(0.1, 0.5, 0.5, 0.5);
                    ret.custom_size = Some(THIGH_SIZE);
                    ret
                },
                ..default()
            },
            LeftThigh,
            RigidBody::Dynamic,
            MassPropertiesBundle::new_computed(&Collider::rectangle(BICEP_SIZE.x, BICEP_SIZE.y), 1.0),
        ))
        .id();
    let right_shin = commands
        .spawn((
            SpriteBundle {
                sprite: {
                    let mut ret = square_sprite.clone();
                    ret.color = Color::srgba(0.1, 0.5, 0.5, 0.5);
                    ret.custom_size = Some(SHIN_SIZE);
                    ret
                },
                ..default()
            },
            RightShin,
            RigidBody::Dynamic,
            MassPropertiesBundle::new_computed(&Collider::rectangle(BICEP_SIZE.x, BICEP_SIZE.y), 1.0),
        ))
        .id();
    let left_shin = commands
        .spawn((
            SpriteBundle {
                sprite: {
                    let mut ret = square_sprite.clone();
                    ret.color = Color::srgba(0.1, 0.5, 0.5, 0.5);
                    ret.custom_size = Some(SHIN_SIZE);
                    ret
                },
                ..default()
            },
            LeftShin,
            RigidBody::Dynamic,
            MassPropertiesBundle::new_computed(&Collider::rectangle(BICEP_SIZE.x, BICEP_SIZE.y), 1.0),
        ))
        .id();
    let right_foot = commands
        .spawn((
            SpriteBundle {
                sprite: {
                    let mut ret = square_sprite.clone();
                    ret.color = Color::srgba(0.1, 0.5, 0.5, 0.5);
                    ret.custom_size = Some(FOOT_SIZE);
                    ret
                },
                ..default()
            },
            RightFoot,
            RigidBody::Dynamic,
            Collider::rectangle(FOOT_SIZE.x, FOOT_SIZE.y),
            CollisionLayers::new(Layer::Platform, Layer::Platform),
            // MassPropertiesBundle::new_computed(&Collider::rectangle(FOOT_SIZE.x, FOOT_SIZE.y), 1.0),
        ))
        .id();
    let left_foot = commands
        .spawn((
            SpriteBundle {
                sprite: {
                    let mut ret = square_sprite.clone();
                    ret.color = Color::srgba(0.1, 0.5, 0.5, 0.5);
                    ret.custom_size = Some(FOOT_SIZE);
                    ret
                },
                ..default()
            },
            LeftFoot,
            RigidBody::Dynamic,
            Collider::rectangle(FOOT_SIZE.x, FOOT_SIZE.y),
            CollisionLayers::new(Layer::Platform, Layer::Platform),
            // MassPropertiesBundle::new_computed(&Collider::rectangle(FOOT_SIZE.x, FOOT_SIZE.y), 1.0),
        ))
        .id();


        let floor = commands
        .spawn((
            SpriteBundle {
                sprite: {
                    let mut ret = square_sprite.clone();
                    ret.color = Color::srgba(0.2, 0.2, 0.2, 0.5);
                    ret.custom_size = Some(Vec2::new(500., 10.));
                    ret
                },
                transform: Transform::from_xyz(0., -200., 10.),
                ..default()
            },
            Floor,
            RigidBody::Kinematic,
            Collider::rectangle(500., 10.,),
            CollisionLayers::new(Layer::Platform, Layer::Platform),
            // MassPropertiesBundle::new_computed(&Collider::rectangle(0., -200.,), 10.0),
        ))
        .id();
    
    commands.spawn(
        DistanceJoint::new(head, torso)
        .with_local_anchor_1(Vec2::new(0., -HEAD_SIZE.y / 2.))
        .with_local_anchor_2(Vec2::new(0., TORSO_SIZE.y / 2.))
        .with_rest_length(0.0)
        .with_linear_velocity_damping(10.)
        .with_angular_velocity_damping(ANG_VEC_DAMP)
        .with_compliance(0.00000001),
    );

    //Connects right arm
    commands.spawn(
        DistanceJoint::new(right_hand, right_arm)
        .with_local_anchor_1(Vec2::new(0., -HAND_SIZE.y / 2.))
        .with_local_anchor_2(Vec2::new(0. , ARM_SIZE.y / 2.))
        .with_rest_length(0.0)
        .with_linear_velocity_damping(10.)
        .with_angular_velocity_damping(ANG_VEC_DAMP)
        .with_compliance(0.00000005),
    );
    commands.spawn(
        DistanceJoint::new(right_arm, right_bicep)
        .with_local_anchor_1(Vec2::new(0., -ARM_SIZE.y / 2.))
        .with_local_anchor_2(Vec2::new(0., BICEP_SIZE.y / 2.))
        .with_rest_length(0.0)
        .with_linear_velocity_damping(10.)
        .with_angular_velocity_damping(ANG_VEC_DAMP)
        .with_compliance(0.00000005),
    );

    commands.spawn(
        DistanceJoint::new(right_bicep, torso)
        .with_local_anchor_1(Vec2::new(-ARM_SIZE.x / 2., -ARM_SIZE.y / 2.))
        .with_local_anchor_2(Vec2::new(TORSO_SIZE.x / 2., TORSO_SIZE.y / 2.))
        .with_rest_length(1.0)
        .with_linear_velocity_damping(10.)
        .with_angular_velocity_damping(ANG_VEC_DAMP)
        .with_compliance(0.00000005),
    );

    //connect left arm
    commands.spawn(
        DistanceJoint::new(left_hand, left_arm)
        .with_local_anchor_1(Vec2::new(0., -HAND_SIZE.y / 2.))
        .with_local_anchor_2(Vec2::new(0. , ARM_SIZE.y / 2.))
        .with_rest_length(0.0)
        .with_linear_velocity_damping(10.)
        .with_angular_velocity_damping(ANG_VEC_DAMP)
        .with_compliance(0.00000005),
    );
    commands.spawn(
        DistanceJoint::new(left_arm, left_bicep)
        .with_local_anchor_1(Vec2::new(0., -ARM_SIZE.y / 2.))
        .with_local_anchor_2(Vec2::new(0., BICEP_SIZE.y / 2.))
        .with_rest_length(0.0)
        .with_linear_velocity_damping(10.)
        .with_angular_velocity_damping(ANG_VEC_DAMP)
        .with_compliance(0.00000005),
    );
    commands.spawn(
        DistanceJoint::new(left_bicep, torso)
        .with_local_anchor_1(Vec2::new(ARM_SIZE.x / 2., -ARM_SIZE.y / 2.))
        .with_local_anchor_2(Vec2::new(-TORSO_SIZE.x / 2., TORSO_SIZE.y / 2.))
        .with_rest_length(1.0)
        .with_linear_velocity_damping(10.)
        .with_angular_velocity_damping(ANG_VEC_DAMP)
        .with_compliance(0.00000005),
    );

    //Connects right leg
    commands.spawn(
        DistanceJoint::new(right_foot, right_shin)
        .with_local_anchor_1(Vec2::new(0., FOOT_SIZE.y / 2.))
        .with_local_anchor_2(Vec2::new(0. , -SHIN_SIZE.y / 2.))
        .with_rest_length(0.0)
        .with_linear_velocity_damping(10.)
        .with_angular_velocity_damping(ANG_VEC_DAMP)
        .with_compliance(0.00000005),
    );
    commands.spawn(
        DistanceJoint::new(right_thigh, right_shin)
        .with_local_anchor_1(Vec2::new(0., -THIGH_SIZE.y / 2.))
        .with_local_anchor_2(Vec2::new(0., SHIN_SIZE.y / 2.))
        .with_rest_length(0.0)
        .with_linear_velocity_damping(10.)
        .with_angular_velocity_damping(ANG_VEC_DAMP)
        .with_compliance(0.00000005),
    );

    commands.spawn(
        DistanceJoint::new(right_thigh, torso)
        .with_local_anchor_1(Vec2::new(THIGH_SIZE.x / 2., THIGH_SIZE.y / 2.))
        .with_local_anchor_2(Vec2::new(TORSO_SIZE.x / 2., -TORSO_SIZE.y / 2.))
        .with_rest_length(1.0)
        .with_linear_velocity_damping(10.)
        .with_angular_velocity_damping(ANG_VEC_DAMP)
        .with_compliance(0.00000005),
    );

    //connect left leg
    commands.spawn(
        DistanceJoint::new(left_foot, left_shin)
        .with_local_anchor_1(Vec2::new(0., FOOT_SIZE.y / 2.))
        .with_local_anchor_2(Vec2::new(0. , -SHIN_SIZE.y / 2.))
        .with_rest_length(0.0)
        .with_linear_velocity_damping(10.)
        .with_angular_velocity_damping(ANG_VEC_DAMP)
        .with_compliance(0.00000005),
    );
    commands.spawn(
        DistanceJoint::new(left_thigh, left_shin)
        .with_local_anchor_1(Vec2::new(0., -THIGH_SIZE.y / 2.))
        .with_local_anchor_2(Vec2::new(0., SHIN_SIZE.y / 2.))
        .with_rest_length(0.0)
        .with_linear_velocity_damping(10.)
        .with_angular_velocity_damping(ANG_VEC_DAMP)
        .with_compliance(0.00000005),
    );
    commands.spawn(
        DistanceJoint::new(left_thigh, torso)
        .with_local_anchor_1(Vec2::new(-THIGH_SIZE.x / 2., THIGH_SIZE.y / 2.))
        .with_local_anchor_2(Vec2::new(-TORSO_SIZE.x / 2., -TORSO_SIZE.y / 2.))
        .with_rest_length(1.0)
        .with_linear_velocity_damping(10.)
        .with_angular_velocity_damping(ANG_VEC_DAMP)
        .with_compliance(0.00000005),
    );

    
}

#[derive(Component)]
struct Head;
#[derive(Component)]
struct RightArm;
#[derive(Component)]
struct RightBicep;
#[derive(Component)]
struct RightHand;
#[derive(Component)]
struct LeftArm;
#[derive(Component)]
struct LeftBicep;
#[derive(Component)]
struct LeftHand;
#[derive(Component)]
struct RightThigh;
#[derive(Component)]
struct RightShin;
#[derive(Component)]
struct RightFoot;
#[derive(Component)]
struct LeftThigh;
#[derive(Component)]
struct LeftShin;
#[derive(Component)]
struct LeftFoot;
#[derive(Component)]
struct Floor;

fn move_right_arm (
    mut query: Query<(&mut Transform, &RightHand), With<RightHand>>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let (camera, camera_transform) = q_camera.single();
    let cursor_pos = q_windows.single().cursor_position()
    .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
    .map(|ray| ray.origin.truncate());
    for (mut transform, _hand) in query.iter_mut() {
        match cursor_pos {
            Some(v) => {
                transform.translation = Vec3::new(v.x, v.y, 1.);
            },
            None => {

            },
        }
    }
}

fn collision (
    mut query: Query<(&CollidingEntities)>,
) {
    for collider in query.iter_mut() {
        println!("COLLIDER: {:?}", collider.0);
    }
}