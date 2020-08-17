use crate::app_builder::AppBuilder;
use bevy_ecs::{DefaultExecutor, Resources, Schedule, World};

/// Containers of app logic and data
///
/// App store the ECS World, Resources, Schedule, and Executor. They also store the "run" function of the App, which
/// by default executes the App schedule once. Apps are constructed using the builder pattern.
///
/// ## Example
/// Here is a simple "Hello World" Bevy app:
/// ```
///use bevy_app::prelude::*;
///use bevy_ecs::prelude::*;
///
///fn main() {
///    App::build()
///        .add_system(hello_world_system.system())
///        .run();
///}
///
///fn hello_world_system() {
///    println!("hello world");
///}
/// ```
pub struct App {
    pub world: World,
    pub resources: Resources,
    pub runner: Box<dyn Fn(App)>,
    pub schedule: Schedule,
    pub executor: Option<DefaultExecutor>,
    pub startup_schedule: Schedule,
    pub startup_executor: Option<DefaultExecutor>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            world: Default::default(),
            resources: Default::default(),
            schedule: Default::default(),
            executor: None, //DefaultExecutor::new("App", true),
            startup_schedule: Default::default(),
            startup_executor: None, //DefaultExecutor::new("Startup", false),
            runner: Box::new(run_once),
        }
    }
}

fn run_once(mut app: App) {
    app.update();
}

impl App {
    pub fn build() -> AppBuilder {
        AppBuilder::default()
    }

    pub fn update(&mut self) {
        self.schedule.initialize(&mut self.resources, &mut self.executor, "Startup", false);
        self.executor
            .as_mut()
            .unwrap()
            .run(&mut self.schedule, &mut self.world, &mut self.resources);
    }

    pub fn run(mut self) {
        self.startup_schedule.initialize(&mut self.resources, &mut self.startup_executor, "App", true);
        self.startup_executor
            .as_mut()
            .unwrap()
            .run(
            &mut self.startup_schedule,
            &mut self.world,
            &mut self.resources,
        );

        let runner = std::mem::replace(&mut self.runner, Box::new(run_once));
        (runner)(self);
    }
}

/// An event that indicates the app should exit. This will fully exit the app process.
pub struct AppExit;
