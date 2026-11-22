<p align="center">
  <img src="../../resources/icon.png" width="120" alt="AirDrop Bridge Logo" />
</p>

<h1 align="center">AirDrop Bridge</h1>

<p align="center">
  <strong>从 iPhone 和 iPad 到 Windows 与 Linux 的原生超快文件传输。</strong><br>
  无需在 iOS 上安装第三方 App，无需云端账户，完全免费无订阅。
</p>

<p align="center">
  <a href="../../README.md">English</a> |
  <a href="README.es.md">Español</a> |
  <strong>简体中文</strong> |
  <a href="README.pt.md">Português</a> |
  <a href="README.fr.md">Français</a> |
  <a href="README.de.md">Deutsch</a> |
  <a href="README.it.md">Italiano</a> |
  <a href="README.ja.md">日本語</a>
</p>

<p align="center">
  <a href="https://github.com/jorjanion/airdrop-bridge/releases"><img src="https://img.shields.io/github/v/release/jorjanion/airdrop-bridge?color=38bdf8&style=flat-square" alt="Release" /></a>
  <a href="https://github.com/jorjanion/airdrop-bridge/blob/main/LICENSE"><img src="https://img.shields.io/badge/license-MIT%20%2F%20Apache--2.0-blue?style=flat-square" alt="许可证" /></a>
  <img src="https://img.shields.io/badge/平台-Windows%20%7C%20Linux-emerald?style=flat-square" alt="平台" />
  <img src="https://img.shields.io/badge/语言-内置%208%20种-purple?style=flat-square" alt="语言" />
  <img src="https://img.shields.io/badge/rust-2024%20edition-orange?style=flat-square" alt="Rust" />
</p>

> ⭐ **喜欢 AirDrop Bridge 吗？** 欢迎在 GitHub 上点亮一颗星！这能让更多人发现这个完全免费的开源工具，也是我们持续维护的最大动力。

---

## 概述

**AirDrop Bridge** 让你能够通过局域网 Wi-Fi，将 iPhone 或 iPad 上的照片、4K 视频和文件直接秒传至电脑。

无需在 App Store 下载任何第三方软件，也不必把私密文件上传到外部云端服务器。AirDrop Bridge 将电脑与 Apple 官方快捷指令无缝对接，直接集成在原生 iOS 分享菜单、操作按钮或轻点背面手势中。

<p align="center">
  <img src="../../assets/screenshots/pairing-window.png" width="280" alt="电脑端配对窗口" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="../../assets/screenshots/mobile-setup.png" width="240" alt="手机 Safari 设置页面" />
  &nbsp;&nbsp;&nbsp;&nbsp;
  <img src="../../assets/screenshots/ios-share-sheet.png" width="240" alt="iOS 分享面板" />
</p>

---

## 核心特性

* **100% 本地与私密**: 点对点局域网传输，照片与文件绝不离开您的家庭网络。
* **iOS 端免装任何第三方 App**: 采用 Apple 官方签名快捷指令，安全透明。
* **极速传输**: 分块磁盘流式写入，内存占用极低（常驻内存低于 15 MB）。即使传输超大体积的 4K 视频也不会爆内存。
* **零配置自动发现**: 局域网 mDNS 自动响应，搭配 IP 直连备份方案。
* **默认安全机制**: 采用加密 Bearer 令牌鉴权，防止公共 Wi-Fi 网络下的未授权传输。
* **多语言支持**: 完整本地化覆盖 8 种语言（English、Español、简体中文、Português、Français、Deutsch、Italiano、日本語）。
* **原生桌面体验**: 原生暗黑模式配对窗口、系统托盘右键菜单、提示音以及 Windows 交互式通知。
* **剪贴板同步**: 点击即可快速在设备之间互传复制的文本。
* **静默自动更新**: 内置后台检查器，自动平滑升级至最新版本。

<p align="center">
  <img src="../../assets/screenshots/tray-menus.png" width="620" alt="托盘菜单与多语言设置" />
</p>

---

## 快速上手指南

只需不到一分钟即可完成配置。

### 1. 电脑端下载并启动
* 在 [GitHub Releases](https://github.com/jorjanion/airdrop-bridge/releases/latest) 下载 `airdrop-bridge-windows-x64.exe`。
* 双击运行。程序会在系统托盘中静默运行，并弹出配对二维码窗口。

### 2. 使用 iPhone 相机扫码
* 打开 iPhone 或 iPad 的自带相机，对准电脑屏幕上的二维码扫描。
* 轻点出现的黄色提示条，在 Safari 中打开配置页面。

### 3. 安装 Apple 官方快捷指令
* 在网页中轻点 **"1. 下载并安装 AirDrop Bridge 快捷指令"**。
* Safari 将提示在“快捷指令”中打开，轻点 **打开** 并选择 **添加快捷指令**。

### 4. 配对电脑
* 返回 Safari 页面，轻点 **"2. 配对并配置此电脑"**。
* 快捷指令将自动接收安全令牌与电脑 IP，无需手动输入任何配置。

### 5. 开始分享文件与双向剪贴板！

#### 发送照片、视频与文件：
* 在 iPhone 上打开 **照片** 或 **文件** App。
* 选择任意照片、视频或文档，轻点 **分享** 按钮，选中 **AirDrop Bridge**。
* 文件将瞬间传输至电脑的下载文件夹（`Downloads\AirDropBridge`），并伴有提示音和桌面通知！

#### 双向剪贴板同步：
* 直接轻点运行 **AirDrop Bridge** 快捷指令（可通过快捷指令 App、主屏幕小组件、操作按钮或轻点背面手势触发）。
* 弹出菜单将提供两个实用选项：
  * 📤 **Send clipboard（发送剪贴板）**: 将 iPhone 当前复制的文本秒传至电脑剪贴板。
  * 📥 **Get clipboard（获取剪贴板）**: 将电脑当前剪贴板中的文本直接读取并复制到 iPhone。
* 您还可以在 iPhone 的 Safari、备忘录或信息中选中任何文字，轻点 **共享** 并选择 **AirDrop Bridge**，直接发送到电脑！

---

## 关于 Windows SmartScreen 提示

首次在 Windows 上启动 AirDrop Bridge 时，Windows Defender SmartScreen 可能会弹出蓝色提示框：

> *"Windows 已保护你的电脑"*

### 为什么会出现此提示？
微软的代码签名商业证书每年需花费数百美元。作为独立、免费且开源的非商业项目，我们并未购买昂贵的技术证书。Windows 会对新发布的独立可执行文件展示该安全提示，直到下载量积累达到一定规模。

### 如何继续运行：
1. 点击提示框中的 **"更多信息"**。
2. 点击出现的 **"仍要运行"** 按钮。

本项目所有源代码 100% 开源、透明且在 GitHub 上可随时审查。如果您希望，也可以通过 Cargo 自行源码编译。

---

## 系统托盘设置说明

右键单击 Windows 任务栏通知区域的 AirDrop Bridge 图标，即可使用各项实用功能：

* **打开 AirDrop Bridge**: 随时再次唤出二维码配对窗口。
* **打开下载文件夹**: 一键直达接收文件的本地存储目录。
* **复制令牌**: 快速复制安全令牌到系统剪贴板。
* **设置**:
  * **开机自启动**: 随 Windows 登录自动在后台就绪。
  * **系统通知**: 开启或静音接收时的桌面通知。
  * **接收时播放提示音**: 文件到达时播放清脆提示音。
  * **接收后打开文件夹**: 文件接收完成后自动弹出所在目录。
  * **剪贴板同步**: 开启或关闭跨设备文本快传。
  * **自动更新**: 后台静默检测并升级最新功能。
  * **语言**: 即时在 8 种语言间自由切换。

---

## 从源码编译

环境要求: [Rust](https://www.rust-lang.org/) (2024 edition 或更新版本)。

```bash
git clone https://github.com/jorjanion/airdrop-bridge.git
cd airdrop-bridge
cargo build --release
```

编译生成的高性能执行文件位于 `target/release/airdrop-bridge.exe`。

---

## 参与贡献、提交 Issue 与 PR

我们由衷欢迎来自社区的任何 Bug 反馈、功能建议与代码贡献！

### 报告问题或提出建议
如果您在使用中遇到了异常，或者有更好的功能想法：
1. 前往项目的 [Issues 页面](https://github.com/jorjanion/airdrop-bridge/issues)。
2. 查看是否已有相似主题。
3. 点击 **New Issue** 并注明：
   * 发生了什么现象以及您预期的效果。
   * 您的操作系统版本（如 Windows 11 或具体 Linux 发行版）。
   * 您的 iOS/iPadOS 版本（如 iOS 18.1）。
   * 复现该问题的具体操作步骤。

### 提交 Pull Request (PR)
1. Fork 本仓库并创建分支：
   ```bash
   git checkout -b feature/my-new-feature
   ```
2. 编写代码并通过所有单元测试：
   ```bash
   cargo test
   ```
3. 遵循常规 Commit 格式提交（如 `feat: ...`, `fix: ...`）。
4. Push 至您的 Fork 仓库，向 `main` 分支发起 Pull Request。
5. 简要说明您的改动意图，我们会积极审查并合并优秀代码！

---

## 开源许可证

AirDrop Bridge 遵循以下双重开源协议，您可任选其一使用：

* Apache License, Version 2.0 ([LICENSE-APACHE](../../LICENSE) 或 http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](../../LICENSE) 或 http://opensource.org/licenses/MIT)
