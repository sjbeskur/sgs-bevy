

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RSOPlugin)
        //.add_startup_system(add_rsos)
        //.add_system(ack_rsos)
        //.add_system(hello_world)
        .run();
}

fn hello_world() {
    println!("Hello \"virtual\" World");
}

#[derive(Component)]
struct RSO;

#[derive(Component)]
struct Name(String);

fn add_rsos(mut commands: Commands){
    commands.spawn((RSO, Name("hubble".to_string())));
    commands.spawn((RSO, Name("james web".to_string())));
    commands.spawn((RSO, Name("sputnik".to_string())));
}

fn ack_rsos(time: Res<Time>, mut timer: ResMut<PingTimer>, query: Query<&Name, With<RSO>>){

    if timer.0.tick(time.delta()).just_finished() {
        for name in query.iter(){
            println!("targeting:  {}", name.0);
        }
    }

}


#[derive(Resource)]
struct PingTimer(Timer);

pub struct RSOPlugin;
impl Plugin for RSOPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PingTimer(Timer::from_seconds(2.0,TimerMode::Repeating)))
            .add_startup_system(add_rsos)
            .add_system(ack_rsos);
    }
}
