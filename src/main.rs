use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

#[derive(Copy, Clone)]
pub struct Position(Vec2);

// Чувачки
#[derive(Copy, Clone)]
struct Unit;

// Сделанная работа
#[derive(Copy, Clone)]
struct Value(f32);

// Чувачок который спавнит сигнал
#[derive(Copy, Clone)]
struct Producer;

// Чувачок который поглощает сигнал и делает работу
#[derive(Copy, Clone)]
struct Consumer;

// Сигнал, который идет между нодами
#[derive(Copy, Clone)]
struct Signal(f32);

// Окраска сигнала - c количеством конкретного вида сигнала, который есть.
// Signal(y + x + z) = UsefulSignal(y) + ChattingSignal(x) + ToxicSignal(z);
#[derive(Copy, Clone)]
struct UsefulSignal(f32);
#[derive(Copy, Clone)]
struct ChattingSignal(f32);
#[derive(Copy, Clone)]
struct ToxicSignal(f32);

// Интенсивность сигнала
#[derive(Copy, Clone)]
struct SignalIntensity(f32);

// Насколько сигнал распространился
#[derive(Copy, Clone)]
struct SignalPropagation(f32);

fn setup_system(mut commands: Commands) {
    let prod_shape = shapes::Circle {
        radius: 50.0,
        center: Vec2::ZERO,
    };
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands
        .spawn_bundle(GeometryBuilder::build_as(
            &prod_shape,
            ShapeColors::new(Color::AZURE),
            DrawMode::Outlined {
                fill_options: FillOptions::default(),
                outline_options: StrokeOptions::default(),
            },
            Transform::default(),
        ))
        .insert(Unit {})
        .insert(Position(Vec2::new(0., 0.)))
        .insert(SignalIntensity(1.0))
        .insert(Producer {});

    let cons_shape = shapes::Circle {
        radius: 50.0,
        center: Vec2::new(200., 100.),
    };

    commands
        .spawn_bundle(GeometryBuilder::build_as(
            &cons_shape,
            ShapeColors::new(Color::RED),
            DrawMode::Outlined {
                fill_options: FillOptions::default(),
                outline_options: StrokeOptions::default(),
            },
            Transform::default(),
        ))
        .insert(Unit {})
        .insert(Position(Vec2::new(100., 100.)))
        .insert(Consumer {});
}

struct OneSecondTimer(Timer);

fn spawn_signal(
    mut commands: Commands,
    query: Query<(&SignalIntensity, &Position), With<Producer>>,
    time: Res<Time>,
    mut timer: ResMut<OneSecondTimer>,
) {
    let default_propagation = SignalPropagation(60.0);
    if timer.0.tick(time.delta()).just_finished() {
        for (intensity, position) in query.iter() {
            let signal_shape = shapes::Circle {
                radius: default_propagation.0,
                center: Vec2::ZERO,
            };

            commands
                .spawn_bundle(GeometryBuilder::build_as(
                    &signal_shape,
                    ShapeColors::new(Color::RED),
                    DrawMode::Stroke(StrokeOptions::default()),
                    Transform::default(),
                ))
                .insert(*position)
                .insert(default_propagation)
                .insert(Signal(intensity.0))
                .insert(UsefulSignal(intensity.0 / 2.))
                .insert(ChattingSignal(intensity.0 / 4.))
                .insert(ToxicSignal(intensity.0 / 4.));
        }
    }
}

// Сигнал гаснет на некотором окружении от кружочка
fn signal_extinction(
    mut commands: Commands,
    query: Query<(Entity, &SignalPropagation)>
) {
    for (entity, ppgtn) in query.iter() {
        if ppgtn.0 > 2000. {
            commands.entity(entity).despawn();
        }
    }
}

// Таймер, который должен триггерить перерисовку экрана для объектов, которым нужна плавная отрисовка
struct FPSTimer(Timer);

fn propagate_signal(
    mut query: Query<(&mut SignalPropagation, &mut Transform)>,
    time: Res<Time>,
    mut timer: ResMut<FPSTimer>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        for (mut ppgtn, mut transform) in query.iter_mut() {
            ppgtn.0 += 10.0;
            transform.scale.x += 0.01;
            transform.scale.y += 0.01;
        }
    }
}

fn main() {
    App::build()
        .insert_resource(Msaa { samples: 8 })
        .insert_resource(OneSecondTimer(Timer::from_seconds(1.0, true)))
        .insert_resource(FPSTimer(Timer::from_seconds(0.016, true)))
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_startup_system(setup_system.system())
        .add_system(spawn_signal.system())
        .add_system(propagate_signal.system())
        .add_system(signal_extinction.system())
        .run();
}
