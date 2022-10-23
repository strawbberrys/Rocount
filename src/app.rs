use druid::AppLauncher;

mod ui;

pub fn run() {
    AppLauncher::with_window(ui::main_window())
        .launch(())
        .expect("App failed to launch.");
}