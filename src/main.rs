use std::f32::consts::*;
use bevy::prelude::*;
use bevy::gltf::Gltf;

fn main() {
    App::new()
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 1.0 / 5.0f32,
        })    
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup_gltf)
        //.add_systems(Startup, setup)
        //.add_startup_system(spawn_gltf_objects)        
        .add_systems(Update, animate_light_direction)
        //.add_systems(Update,spin_cubes)
        .run();
}

fn setup_gltf(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.7, 0.7, 1.0).looking_at(Vec3::new(0.0, 0.3, 0.0), Vec3::Y),
        ..default()
    });
    const HALF_SIZE: f32 = 1.0;
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            // shadow_projection: OrthographicProjection {
            //     left: -HALF_SIZE,
            //     right: HALF_SIZE,
            //     bottom: -HALF_SIZE,
            //     top: HALF_SIZE,
            //     near: -10.0 * HALF_SIZE,
            //     far: 10.0 * HALF_SIZE,
            //     ..default()
            // },
            shadows_enabled: true,
            ..default()
        },
        ..default()
    });
    commands.spawn(SceneBundle {
        scene: asset_server.load("FlightHelmet/FlightHelmet.gltf#Scene0"),
        //scene: asset_server.load("Satellite01/untitled.gltf#Scene0"),
        ..default()
    });
}

fn animate_light_direction(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<DirectionalLight>>,
) {
    for mut transform in &mut query {
        transform.rotation = Quat::from_euler(
            EulerRot::ZYX,
            0.0,
            time.elapsed_seconds() * PI / 5.0,
            -FRAC_PI_4,
        );
    }
}






/// set up a simple 3D scene with a cube and a floor 
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    
    // plane (floor)
    // commands.spawn(PbrBundle {
    //     mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 , subdivisions: 4 })),
    //     material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
    //     ..default()
    // });
     
    // cube
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(-3.0, 0.0, -2.0),
        ..default()
    });

    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(-1.0, 0.0, -1.0),
        ..default()
    });

    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(1.0, 0.0, -0.5),
        ..default()
    });

    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(3.0, 0.0, 0.0),
        ..default()
    });


    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(5.0, 8.0, 4.0),
        ..default()
    });
    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}


fn spin_cubes(
    time: Res<Time>,
    mut query: Query<&mut Transform>,
) {
    for mut transform in &mut query {
        transform.rotation = Quat::from_euler(
            EulerRot::ZYX,
            0.4,
            time.elapsed_seconds() * PI / 6.0,
            -FRAC_PI_4,
        );
    }
}
