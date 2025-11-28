use android_activity::{AndroidApp, MainEvent};
use log::info;

#[no_mangle]
fn android_main(app: AndroidApp) {
    android_logger::init_once(
        android_logger::Config::default().with_max_level(log::LevelFilter::Info)
    );

    info!("🚀 GitHub Action Builder Started");

    let mut should_run = true;
    
    while should_run {
        app.poll_events(Some(std::time::Duration::from_millis(16)), |event| {
            match event {
                MainEvent::Resume => {
                    info!("App resumed - ready to manage GitHub Actions");
                }
                MainEvent::Destroy => {
                    info!("App destroyed");
                    should_run = false;
                }
                _ => {}
            }
        });
    }
}