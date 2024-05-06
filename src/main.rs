use bevy::prelude::*;
mod tag;

fn main() {
    App::new()
    .set_runner(my_runner)
    .add_systems(Update, hello_world_system)
    .run();
}

fn hello_world_system() {
    println!("hello world");
}

fn my_runner(mut app: App)  {
    println!("Type stuff into the console");
    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
        app.update();
    }
}
