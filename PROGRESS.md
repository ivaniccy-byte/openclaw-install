# OpenClaw 职场版 开发进度文档

> 最后更新: 2026-04-05

## 项目概述

- **项目名称**: OpenClaw 职场一键版（别名：ClawWork）
- **产品定位**: 永久免费、无捆绑、纯原版内核、全程零代码、一键安装的OpenClaw整合包
- **目标用户**: 国内零技术基础普通上班族

## 技术架构

- **安装包框架**: Tauri 2.0 (包体小、内存低、启动快)
- **前端框架**: Vue 3 + Element Plus + TypeScript
- **后端**: Rust (Tauri原生)
- **OpenClaw核心**: Node.js 24 (最好) / 22+
- **记忆系统Python环境**: 3.12.9
  - OpenViking 0.3.3 (稳定版)
  - Lossless-Claw + LanceDB Pro (最新版)

## 版本要求

- Node.js: v24 (推荐) / v22+ (支持)
- Python: 3.12.9 (必需，用于记忆系统)
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
  - Python 3.12.9检测
  - 端口可用性检测
  - 网络连通性检测
- [x] `src-tauri/src/wrappers.rs` - OpenClaw进程管理
  - Node.js进程启动/停止
  - Python运行时查找
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
  - 主模型/Embedding/Rerank分层配置
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
- [x] `resources/python-runtime/win-x64/` - Python运行时
- [x] `resources/openclaw/` - OpenClaw核心
- [x] `resources/openviking/` - OpenViking 0.3.3
- [x] `resources/lossless-claw/` - Lossless-Claw
- [x] `resources/lancedb-pro/` - LanceDB Pro
- [x] `resources/skills/` - 预封装Skill包
- [x] `resources/openclaw/config.yaml.example` - 配置模板

#### 7. Git版本控制
- [x] Git仓库初始化
- [x] 提交初始代码框架 (commit: ac71def, 59 files, 8045 insertions)

---

## ⏳ 待完成（明天继续）

### 8. 远程仓库与云编译
- [ ] 在GitHub创建远程仓库
- [ ] 添加远程仓库地址
- [ ] 推送代码到远程
- [ ] 创建tag: `v0.8.0` 触发云编译
- [ ] 下载验证构建产物(.exe)

### 9. 离线资源打包
- [ ] 下载 Node.js 24 embeddable 到 resources/
- [ ] 下载 Python 3.12.9 embeddable 到 resources/
- [ ] 预下载 Python 依赖包（pip download）

### 10. OpenClaw核心集成
- [ ] 获取 openclaw-standalone 官方源码包
- [ ] 适配 OpenClaw 入口文件和配置
- [ ] 集成 OpenViking/Lossless-Claw/LanceDB

### 11. 构建与测试
- [ ] 安装包功能测试
- [ ] OpenClaw实际运行测试
- [ ] 健康度监控测试

### 12. 迭代优化
- [ ] V0.9: 插件系统完整适配
- [ ] V1.0: 一键修复功能完善
- [ ] V1.1+: 持续同步上游更新

---

## 当前Git状态

```
Branch: master
Last Commit: d91bf1e
  "更新进度文档"
  60 files changed, 8091 insertions(+)

Remote: 未配置（需要创建GitHub仓库后添加）
```

---

## 今日完成

- [x] 进度文档更新 (d91bf1e)
- [x] 代码框架提交完成，等待明天推送到GitHub

---

## 明日操作步骤

### 步骤1: 创建GitHub仓库并推送
```bash
# 1. 在GitHub创建仓库: openclaw-workplace
# 2. 添加远程仓库
git remote add origin https://github.com/你的用户名/openclaw-workplace.git

# 3. 推送代码
git push -u origin master

# 4. 创建tag触发云编译
git tag v0.8.0
git push origin v0.8.0

# 5. 等待GitHub Actions构建完成
# 6. 下载Release中的.exe安装包
```

### 步骤2: 下载离线资源（可并行）
- Node.js 24: https://nodejs.org/dist/v24.0.0/node-v24.0.0-win-x64.zip
- Python 3.12.9: https://www.python.org/ftp/python/3.12.9/python-3.12.9-embed-amd64.zip

---

## 构建说明

### 本地构建（不推荐，内存不足）
```bash
npm install
export PATH="$HOME/.cargo/bin:/c/ProgramData/mingw64/mingw64/bin:$PATH"
npm run tauri build
```

### 云编译（GIT Actions - 推荐）
```bash
git remote add origin https://github.com/你的用户名/openclaw-workplace.git
git push -u origin master
git tag v0.8.0
git push origin v0.8.0
```

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
├── python-runtime/win-x64/
├── openclaw/
├── openviking/
├── lossless-claw/
├── lancedb-pro/
└── skills/
```

---

## 备注

- 本地构建因内存不足失败（需要16GB+ RAM）
- 建议使用GitHub Actions云编译
- 代码框架已完成 (commit: d91bf1e)
- 明天需推送到GitHub触发云编译
- 离线资源（Node.js 24、Python 3.12.9）待下载
