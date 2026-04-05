# OpenClaw 职场版

永久免费、无捆绑、纯原版内核、全程零代码、一键安装的 OpenClaw 整合包，专为国内零技术基础普通上班族设计。

## 技术架构

- **安装包框架**: Tauri 2.0 (包体小、内存低、启动快)
- **前端框架**: Vue 3 + Element Plus
- **后端**: Rust (Tauri)
- **OpenClaw核心**: Node.js 24 (最好) / 22+
- **记忆系统**:
  - OpenViking 0.3.3 (稳定版, Python 3.12.9)
  - Lossless-Claw + LanceDB Pro (最新版, Python 3.12.9)

## 版本要求

- **Node.js**: v24 (推荐) / v22+ (支持)
- **Python**: 3.12.9 (必需，用于记忆系统)
- **CPU**: 支持 AVX2 指令集
- **内存**: ≥4GB (推荐 8GB+)
- **磁盘空间**: ≥10GB (SSD 推荐)

## 离线打包

安装包包含所有运行时和依赖，无需网络即可安装：
- Node.js 24 运行时
- Python 3.12.9 运行时
- OpenClaw 核心包
- 记忆系统组件 (OpenViking / Lossless-Claw / LanceDB)
- 预封装职场Skill包

## 功能特点

- 一键安装，零命令行操作
- 安装前环境预检测 (AVX2/磁盘/内存/运行时/端口)
- 可视化配置中心 (与记忆系统联动)
- 健康度监控与一键修复
- 可选插件 (飞书/微信)
- 可选记忆系统 (OpenViking / Lossless+LanceDB)
- 预封装职场Skill包
- 全程中文界面

## 开发

```bash
# 安装前端依赖
npm install

# 开发模式
npm run tauri dev

# 构建
npm run tauri build
```

## 开源协议

MIT License - 永久免费开源

## 致谢

- OpenClaw 官方开源项目
- openclaw-standalone 社区项目
- OpenClaw-Launcher / OpenClaw-Sifu / ClawSquire
- OpenViking / Lossless-Claw / LanceDB Pro
