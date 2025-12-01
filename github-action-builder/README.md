# github-action-builder

安卓 APK 的 Rust 子 crate。入口为 NativeActivity（android-activity），日志输出到 Logcat（android_logger）。
构建由 cargo-apk 完成；Manifest 提供 INTERNET 权限。

环境变量（可选）：
- GITHUB_OWNER：仓库 owner
- GITHUB_REPO：仓库名
- GITHUB_TOKEN：访问 API 的令牌（从 GitHub Secrets 注入）