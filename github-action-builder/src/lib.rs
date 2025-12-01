use android_activity::{AndroidApp, MainEvent};
use log::{info, warn, error};
use std::time::Duration;

#[no_mangle]
pub extern "C" fn android_main(app: AndroidApp) {
    android_logger::init_once(
        android_logger::Config::default().with_max_level(log::LevelFilter::Info),
    );

    info!("应用启动：GitHub Actions 监控（纯 Rust）");

    let mut 运行中 = true;

    let rt = tokio::runtime::Runtime::new().expect("Tokio 运行时创建失败");
    rt.spawn(async {
        if let Err(e) = 后台轮询任务().await {
            error!("后台轮询任务出错：{e:?}");
        }
    });

    while 运行中 {
        app.poll_events(Some(Duration::from_millis(16)), |event| {
            match event {
                MainEvent::Resume => info!("应用恢复"),
                MainEvent::Pause => info!("应用暂停"),
                MainEvent::Destroy => {
                    warn!("应用销毁");
                    运行中 = false;
                }
                _ => {}
            }
        });
    }
}

async fn 后台轮询任务() -> anyhow::Result<()> {
    use tokio::time::sleep;

    let token = std::env::var("GITHUB_TOKEN").unwrap_or_default();
    let octo = if !token.is_empty() {
        octocrab::Octocrab::builder().personal_token(token).build()?
    } else {
        octocrab::Octocrab::builder().build()?
    };

    let owner = std::env::var("GITHUB_OWNER").unwrap_or_else(|_| "SleepSheepQwQ".to_string());
    let repo = std::env::var("GITHUB_REPO").unwrap_or_else(|_| "MyTset".to_string());

    loop {