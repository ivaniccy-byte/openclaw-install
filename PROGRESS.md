# OpenClaw 职场版 开发进度文档

> 最后更新: 2026-04-08
> 当前版本: v0.9.2

## 项目概述

- **项目名称**: OpenClaw 职场一键版（别名：ClawWork）
- **产品定位**: 永久免费、无捆绑、纯原版内核、全程零代码、一键安装的OpenClaw整合包
- **目标用户**: 国内零技术基础普通上班族

## 技术架构

- **安装包框架**: Tauri 2.0 (包体小、内存低、启动快)
- **前端框架**: Vue 3 + Element Plus + TypeScript
- **后端**: Rust (Tauri原生)
- **OpenClaw核心**: Node.js 22+ LTS (OpenClaw 3.28 稳定版)
- **记忆系统**: Lossless-Claw Enhanced + LanceDB Pro

## 版本要求

- **Node.js**: v22+ LTS (OpenClaw 3.28 兼容)
- **CPU架构限制**:
  - **x86_64 (Intel/AMD)**: 必须支持 **AVX2 指令集**。
  - **AArch64 (Apple Silicon)**: 原生支持，无AVX2硬性限制。
- **内存**: ≥4GB (推荐 8GB+)
- **磁盘空间**: ≥10GB (SSD 推荐)

---

## 开发进度

### ✅ 已完成 (v0.9.2 核心全量实现)

#### 1. 项目脚手架
- [x] Tauri 2.0 + Vue 3 + TypeScript 项目初始化
- [x] `package.json` / `Cargo.toml` / `tauri.conf.json` 配置

#### 2. Rust后端模块
- [x] `src-tauri/src/detectors.rs` - 环境检测模块
  - [x] CPU AVX2指令集检测 (已适配 ARM/Apple Silicon)
  - [x] Node.js检测 (已适配跨平台路径)
  - [x] 磁盘空间/内存/端口检测
- [x] `src-tauri/src/wrappers.rs` - **物理安装引擎与进程管理**
  - [x] **perform_install**: 真正的物理安装逻辑（文件复制、组件可选安装、配置自动生成）
  - [x] OpenClaw 进程管理与状态监控
  - [x] 跨平台 Node.js 运行时寻找逻辑 (Win/Mac)
- [x] `src-tauri/src/telemetry.rs` - 健康度评分引擎 (0-100分评分体系)
- [x] `src-tauri/src/fixes.rs` - 一键修复引擎 (实现端口、配置、重启等基础修复)
- [x] `src-tauri/src/lib.rs` - Tauri命令接口全量封装

#### 3. Vue前端页面
- [x] `src/views/InstallerView.vue` - **5步安装向导 (已对接物理安装引擎)**
- [x] `src/views/DashboardView.vue` - 启动控制仪表盘
- [x] `src/views/ConfigCenterView.vue` - 配置中心 (模型-记忆联动)
- [x] `src/views/HealthCenterView.vue` - 健康中心 (实时监控与一键修复)
- [x] `src/views/FunctionManageView.vue` - 组件管理 (插件/Skill)

#### 4. CI/CD 云编译
- [x] `.github/workflows/build-windows.yml`: **修正缓存路径，解决打包超时问题**
- [x] `tauri.conf.json`: **启用 NSIS (EXE) 打包，优化一键安装体验**

---

## ✅ 已解决问题

### 1. 云编译打包超时
- **方案**: 修正 GitHub Actions 缓存路径至 `~\AppData\Local\tauri`，避免重复下载巨型工具包。

### 2. 环境检测拦截过严
- **方案**: 识别 ARM 架构（Apple Silicon），跳过 AVX2 强制校验。

### 3. 安装包“空壳化”
- **方案**: 废弃前端模拟逻辑，开发了真正的 Rust 后端安装引擎 `perform_install`。

---

## 构建历史

| 版本 | 日期 | 说明 |
|------|------|------|
| v0.9.2 | 2026-04-08 | **核心功能全量落地**: 实现物理安装引擎，适配 ARM 架构，修复空壳代码 |
| v0.9.1 | 2026-04-08 | **编译修复**: 修复云编译超时，启用 NSIS 一键安装包 |
| v0.8.0 | 2026-04-06 | 初始版本，锁定 OpenClaw 3.28 |

---

## 备注

- OpenClaw 版本锁定为 3.28（稳定版）
- 移除 OpenViking 与 Python 运行时依赖，提升安装包纯净度
