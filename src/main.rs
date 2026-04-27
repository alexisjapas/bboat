use avian3d::prelude::*;
use bevy::prelude::*;

fn main() {
    let mut app = App::new();

    app.add_plugins((DefaultPlugins, PhysicsPlugins::default()));
    app.add_systems(Startup, setup);
    app.add_systems(Update, move_player);

    // Debug
    app.add_systems(Startup, debug_gamepad);

    app.run();
}

#[derive(Component)]
struct Player;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            color: Color::srgba(1., 1., 1., 1.),
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));
    // Floor
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(8., 8.))),
        MeshMaterial3d(materials.add(Color::WHITE)),
        Transform::default(),
        RigidBody::Static,
        Collider::half_space(Vec3::Y),
    ));
    // Player
    commands.spawn((
        Player,
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::srgba(1., 1., 1., 1.))),
        Transform::from_xyz(0.0, 5., 0.0),
        RigidBody::Dynamic,
        Collider::cuboid(1., 1., 1.),
        LockedAxes::ROTATION_LOCKED,
    ));
    // Object
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::srgba(1., 1., 1., 1.))),
        Transform::from_xyz(0.0, 2., 0.0),
        RigidBody::Dynamic,
        Collider::cuboid(1., 1., 1.),
    ));

    // Camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0., 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

fn move_player(
    gamepads: Query<&Gamepad>,
    mut query: Query<(&mut LinearVelocity, &mut Transform), With<Player>>,
) {
    let Ok((mut velocity, mut transform)) = query.single_mut() else {
        return;
    };

    // Get first connected gamepad
    let Some(gamepad) = gamepads.iter().next() else {
        return;
    };

    // Parameters
    let max_speed = 5.0;
    let deadzone = 0.1;

    // Left stick
    let axis_x = gamepad.get(GamepadAxis::LeftStickX).unwrap_or(0.0);
    let axis_y = gamepad.get(GamepadAxis::LeftStickY).unwrap_or(0.0);

    let input_dir = Vec2::new(axis_x, axis_y);

    // Apply deadzone
    if input_dir.length() > deadzone {
        let dir = input_dir.normalize_or_zero();
        let move_dir = Vec3::new(dir.x, 0.0, -dir.y); // Y stick → -Z monde

        // Rotate player to face movement direction
        transform.rotation =
            Quat::from_rotation_y((-move_dir.z).atan2(move_dir.x) - std::f32::consts::FRAC_PI_2);

        // Compute velocity based on input direction
        let speed = max_speed * input_dir.length();
        velocity.x = move_dir.x * speed;
        velocity.z = move_dir.z * speed;
    } else {
        velocity.x = 0.0;
        velocity.z = 0.0;
    }
}

fn debug_gamepad(gamepads: Query<(Entity, &Gamepad)>) {
    for (entity, _) in &gamepads {
        println!("Manette connectée : {:?}", entity);
    }
}
