# OpenClaw 职场版 开发进度文档

> 最后更新: 2026-04-07

## 项目概述

- **项目名称**: OpenClaw 职场一键版（别名：ClawWork）
- **产品定位**: 永久免费、无捆绑、纯原版内核、全程零代码、一键安装的OpenClaw整合包
- **目标用户**: 国内零技术基础普通上班族

## 技术架构

- **安装包框架**: Tauri 2.0 (包体小、内存低、启动快)
- **前端框架**: Vue 3 + Element Plus + TypeScript
- **后端**: Rust (Tauri原生)
- **OpenClaw核心**: Node.js 22+ LTS (OpenClaw 3.28 稳定版)
- **记忆系统**: Loseless-Claw + LanceDB Pro (可选，需配置Rerank模型)

## 版本要求

- Node.js: v22+ LTS (OpenClaw 3.28 兼容)
- CPU: 支持 AVX2 指令集
- 内存: ≥4GB (推荐 8GB+)
- 磁盘空间: ≥10GB (SSD 推荐)

---

## 开发进度

### ✅ 已完成

#### 1. 项目脚手架
- [x] Tauri 2.0 + Vue 3 + TypeScript 项目初始化
- [x] `package.json` - 前端依赖配置
- [x] `Cargo.toml` - Rust后端依赖配置
- [x] `tauri.conf.json` - Tauri构建配置
- [x] `vite.config.ts` - Vite构建配置

#### 2. Rust后端模块
- [x] `src-tauri/src/detectors.rs` - 环境检测模块
  - CPU AVX2指令集检测
  - Windows版本检测
  - 磁盘空间/类型检测
  - 内存检测
  - Node.js检测
  - 端口可用性检测
  - 网络连通性检测
- [x] `src-tauri/src/wrappers.rs` - OpenClaw进程管理
  - Node.js进程启动/停止
  - 模型API连通性测试
  - 配置文件读写
- [x] `src-tauri/src/telemetry.rs` - 健康度评分引擎
  - 0-100分评分体系
  - 权重计算逻辑
  - 告警生成
- [x] `src-tauri/src/fixes.rs` - 一键修复引擎
  - 6种修复脚本
- [x] `src-tauri/src/lib.rs` - Tauri命令接口
  - check_environment
  - start_openclaw / stop_openclaw
  - get_openclaw_status
  - get_health_score
  - get_config / save_config
  - auto_fix
  - test_model_connection
  - get_available_port

#### 3. Vue前端页面
- [x] `src/views/InstallerView.vue` - 5步安装向导
  - 步骤1: 环境预检测（AVX2/磁盘/内存/运行时/端口）
  - 步骤2: 安装路径选择
  - 步骤3: 可选功能配置（插件/记忆系统/Skill）
  - 步骤4: 安装进度
  - 步骤5: 完成页面
- [x] `src/views/DashboardView.vue` - 启动控制仪表盘
  - 一键启动/停止按钮
  - 健康度评分显示
  - 实时运行信息
  - 快捷入口
- [x] `src/views/ConfigCenterView.vue` - 配置中心
  - 大模型API配置（与记忆系统联动显示）
  - 主模型/Rerank分层配置
  - 基础运行配置
  - 连通性测试
- [x] `src/views/FunctionManageView.vue` - 功能管理
  - 插件管理（飞书/微信）
  - 记忆系统管理
  - Skill管理
- [x] `src/views/HealthCenterView.vue` - 健康中心
  - 健康度仪表盘
  - 异常告警
  - 一键修复
  - 运行日志
- [x] `src/views/HelpView.vue` - 帮助页
  - 3步快速上手
  - FAQ常见问题
  - 反馈与支持
  - 版权声明

#### 4. 前端公共组件和状态管理
- [x] `src/App.vue` - 主应用布局
- [x] `src/main.ts` - 应用入口
- [x] `src/router/index.ts` - 路由配置
- [x] `src/stores/openclaw.ts` - OpenClaw状态
- [x] `src/stores/health.ts` - 健康状态
- [x] `src/stores/config.ts` - 配置状态
- [x] `src/styles/main.css` - 全局样式

#### 5. 项目配置
- [x] `index.html` - 中文标题
- [x] `.github/workflows/build-windows.yml` - GIT云编译配置
- [x] `README.md` - 项目说明文档

#### 6. 离线资源目录结构
- [x] `resources/node-runtime/win-x64/` - Node.js运行时
- [x] `resources/openclaw/` - OpenClaw核心
- [x] `resources/lossless-claw/` - Lossless-Claw
- [x] `resources/lancedb-pro/` - LanceDB Pro
- [x] `resources/skills/` - 预封装Skill包
- [x] `resources/openclaw/config.yaml.example` - 配置模板

#### 7. Git版本控制
- [x] Git仓库初始化
- [x] 提交初始代码框架

---

## ⏳ 进行中

### 8. 远程仓库与云编译
- [x] 在GitHub创建远程仓库
- [x] 添加远程仓库地址
- [x] 推送代码到远程
- [x] 创建tag触发云编译 (v0.8.0 ~ v0.9.0)
- [ ] 修复GitHub Actions Windows打包超时问题

### 9. 离线资源打包
- [ ] 下载 Node.js 22 LTS embeddable 到 resources/
- [ ] 预下载 Lossless-Claw / LanceDB 依赖包

### 10. OpenClaw核心集成
- [ ] 获取 openclaw-standalone 3.28 官方源码包
- [ ] 适配 OpenClaw 入口文件和配置
- [ ] 集成 Lossless-Claw / LanceDB

### 11. 构建与测试
- [ ] 安装包功能测试
- [ ] OpenClaw实际运行测试
- [ ] 健康度监控测试

---

## ❌ 当前问题

### Windows打包超时
- **错误信息**: `failed to bundle project: timeout: global`
- **根本原因**: Tauri在打包时下载NSIS/WiX工具超时
- **影响**: GitHub Actions Windows构建在打包步骤失败
- **已尝试方案**:
  - v0.8.6: 跳过NSIS，只用MSI打包 → 仍然超时
  - v0.8.7: 完全禁用打包，只生成exe → 构建成功
  - v0.8.8: 重新启用打包，添加Tauri工具缓存 → 仍然超时
- **结论**: 代码无问题，本地exe构建成功，只是打包工具下载失败

---

## 当前Git状态

```
分支: master
远程: origin (https://github.com/ivaniccy-byte/openclaw-install.git)
最新Tag: v0.9.0
最新提交: 69c027f - fix: 添加Tauri打包工具缓存，优化下载
```

## 构建历史

| 版本 | 日期 | 说明 |
|------|------|------|
| v0.9.0 | 2026-04-07 | 添加Tauri打包工具缓存 |
| v0.8.9 | 2026-04-06 | 使用MSI打包，跳过NSIS |
| v0.8.8 | 2026-04-06 | 重新启用打包 |
| v0.8.7 | 2026-04-06 | 禁用打包，只生成exe（构建成功） |
| v0.8.6 | 2026-04-06 | 使用MSI打包（仍超时） |
| v0.8.5 | 2026-04-06 | 使用npm run tauri build |
| v0.8.4 | 2026-04-06 | 改用tauri-action |
| v0.8.3 | 2026-04-06 | 修复sysinfo 0.33 API兼容性 |
| v0.8.2 | 2026-04-06 | 修复tauri.conf.json resources配置错误 |
| v0.8.1 | 2026-04-06 | 修复workflow PowerShell语法 |
| v0.8.0 | 2026-04-06 | 初始版本，删除OpenViking，锁定OpenClaw 3.28 |

## 本地构建状态

```
src-tauri/target/release/openclaw-workplace.exe ✅ 构建成功
```

---

## 构建说明

### 云编译（GIT Actions）
```bash
git push -u origin master
git tag v0.x.x
git push origin v0.x.x
```

### 本地构建
```bash
npm install
npm run tauri build
```

### 下载离线资源
- Node.js 22 LTS: https://nodejs.org/dist/v22.12.0/node-v22.12.0-win-x64.zip

---

## 文件清单

### 核心代码文件
```
src-tauri/
├── Cargo.toml
├── build.rs
├── tauri.conf.json
├── capabilities/default.json
└── src/
    ├── main.rs
    ├── lib.rs (命令接口)
    ├── detectors.rs (环境检测)
    ├── wrappers.rs (进程管理)
    ├── telemetry.rs (健康评分)
    └── fixes.rs (修复引擎)

src/
├── main.ts
├── App.vue
├── index.html
├── router/index.ts
├── stores/
│   ├── openclaw.ts
│   ├── health.ts
│   └── config.ts
├── styles/main.css
└── views/
    ├── InstallerView.vue
    ├── DashboardView.vue
    ├── ConfigCenterView.vue
    ├── FunctionManageView.vue
    ├── HealthCenterView.vue
    └── HelpView.vue

.github/workflows/
└── build-windows.yml

resources/
├── node-runtime/win-x64/
├── openclaw/
├── lossless-claw/
├── lancedb-pro/
└── skills/
```

---

## 备注

- OpenClaw 版本锁定为 3.28（稳定版，新版本有BUG不推荐）
- 移除 OpenViking 记忆系统（资源占用高，普通电脑带不动）
- 移除 Python 3.12.9 依赖（因 OpenViking 移除）
- 记忆系统仅保留 Loseless-Claw + LanceDB Pro
- 本地构建exe成功，GitHub Actions打包步骤超时待解决
