# 纯 Rust 安卓 APK：GitHub Actions 监控与日志过滤

本项目使用 Rust 构建安卓 APK，通过简洁前端展示 GitHub Actions 的运行状态并支持日志过滤。
构建在 GitHub Actions 上自动完成，无需本地环境。

## 功能
- 展示最近的 workflow runs
- 查看运行状态（进行中/成功/失败）
- 下载并查看构建日志（支持关键字过滤）

## 构建方式
- 推送到 main 后，GitHub Actions 会自动：
  1. 安装 Rust 工具链、cargo-apk、Java/Android SDK/NDK
  2. 使用 `cargo apk build --release` 构建 APK
  3. 上传产物到 artifacts