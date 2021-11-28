use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

pub struct Position {
    x: f32,
    y: f32
}

// Чувачки
struct Unit;

// Сделанная работа
struct Value(f32);

// Чувачок который спавнит сигнал
struct Producer;

// Чувачок который поглощает сигнал и делает работу
struct Consumer;

// Сигнал, который идет между нодами
struct Signal;

// Интенсивность сигнала
#[derive(Copy, Clone)]
struct SignalIntensity (f32);

fn setup_system(mut commands: Commands) {
    let prod_shape = shapes::Circle {
        radius: 50.0,
        center: Vec2::ZERO,
    };
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands
        .spawn_bundle(GeometryBuilder::build_as (
            &prod_shape,
            ShapeColors::new(Color::AZURE),
            DrawMode::Outlined {
                fill_options: FillOptions::default(),
                outline_options: StrokeOptions::default(),
            },
            Transform::default(),
        ))
        .insert(Unit{})
        .insert(Position { x: 0.0, y:0.0 })
        .insert(SignalIntensity(1.0))
        .insert(Producer{});

    let cons_shape = shapes::Circle {
        radius: 50.0,
        center: Vec2::new(200., 100.),
    };

    commands
        .spawn_bundle(GeometryBuilder::build_as (
            &cons_shape,
            ShapeColors::new(Color::RED),
            DrawMode::Outlined {
                fill_options: FillOptions::default(),
                outline_options: StrokeOptions::default(),
            },
            Transform::default(),
        ))
        .insert(Unit{})
        .insert(Position { x: 100.0, y:100.0 })
        .insert(Consumer{});
}

fn spawn_signal(
    mut commands: Commands,
    query: Query<&SignalIntensity, With<Producer>>,
    time: Res<Time>,
    mut timer: ResMut<Timer>,
) {
    if timer.tick(time.delta()).just_finished() {
        for intensity in query.iter () {
            commands
                .spawn()
                .insert(Signal{})
                .insert(*intensity);
        }
    }}



fn main() {
    App::build()
        .insert_resource(Msaa { samples: 8 })
        .insert_resource(Timer::from_seconds(1.0, true))
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_startup_system(setup_system.system())
        .add_system(spawn_signal.system())
        .run();
}
