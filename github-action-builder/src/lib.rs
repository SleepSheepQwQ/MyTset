use android_activity::{AndroidApp, MainEvent};
use log::{info, warn, error};
use std::time::Duration;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

#[no_mangle]
pub extern "C" fn android_main(app: AndroidApp) {
    android_logger::init_once(
        android_logger::Config::default().with_max_level(log::LevelFilter::Info),
    );

    info!("应用启动：GitHub Actions 监控（纯 Rust）");

    // 运行标志，用于优雅停止后台任务
    let running = Arc::new(AtomicBool::new(true));
    let running_clone = running.clone();

    let rt = tokio::runtime::Runtime::new().expect("Tokio 运行时创建失败");
    rt.spawn(async move {
        if let Err(e) = 后台轮询任务(running_clone).await {
            error!("后台轮询任务出错：{e:?}");
        }
    });

    let mut 前台运行 = true;

    while 前台运行 {
        app.poll_events(Some(Duration::from_millis(32)), |event| {
            match event {
                MainEvent::Resume => info!("应用恢复"),
                MainEvent::Pause => info!("应用暂停"),
                MainEvent::Destroy => {
                    warn!("应用销毁，停止后台任务");
                    running.store(false, Ordering::SeqCst);
                    前台运行 = false;
                }
                _ => {}
            }
        });
    }

    info!("退出 android_main");
}

async fn 后台轮询任务(running: Arc<AtomicBool>) -> anyhow::Result<()> {
    use tokio::time::{sleep, Duration};

    let token = std::env::var("GITHUB_TOKEN").unwrap_or_default();
    let octo = if !token.is_empty() {
        octocrab::Octocrab::builder().personal_token(token).build()?
    } else {
        octocrab::Octocrab::builder().build()?
    };

    let owner = std::env::var("GITHUB_OWNER").unwrap_or_else(|_| "SleepSheepQwQ".to_string());
    let repo = std::env::var("GITHUB_REPO").unwrap_or_else(|_| "MyTset".to_string());

    // 简单轮询示例：未来可替换为触发式/退避策略
    while running.load(Ordering::SeqCst) {
        match octo
            .repos(&owner, &repo)
            .actions()
            .list_workflow_runs()
            .per_page(5)
            .send()
            .await
        {
            Ok(runs) => {
                info!("最近 runs: {}", runs.total_count);
            }
            Err(e) => {
                warn!("获取 workflow runs 失败：{e:?}");
            }
        }

        // 合理的轮询间隔，避免过度耗电；错误时也不过度重试
        sleep(Duration::from_secs(15)).await;
    }

    info!("后台轮询任务结束");
    Ok(())
}