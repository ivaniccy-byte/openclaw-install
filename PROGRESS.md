# OpenClaw 职场一键版 开发进度文档

> 最后更新: 2026-04-23
> 当前版本: v1.0.0 (Release Candidate)

## 项目概述

- **项目名称**: OpenClaw 职场一键版 (整合包 V1.0)
- **产品定位**: 全量离线、零配置、一键安装、纯净原版内核
- **核心内核**: OpenClaw 3.28 稳定版

## 技术规格 (V1.0)

- **安装路径**: 锁定 `%USERPROFILE%\.openclaw` (对齐官方内核逻辑)
- **内置运行时**:
  - Node.js v22.12.0 (Portable) → `resources/node-runtime/`
  - Python 3.10.11 (Embeddable) → `resources/python-runtime/`
- **OpenClaw 核心**: `resources/openclaw/` (node_modules.tar.gz + start.js)
- **职场技能包**: `resources/skills/`
- **记忆系统**: `resources/memories/`
- **CLI 工具**: `resources/bin/openclaw.cmd`
- **环境变量**: 自动配置 `OPENCLAW_HOME` 并安全追加系统 `PATH`
- **一键卸载**: 支持可选卸载各依赖组件，干净移除残留文件与环境变量

---

## 开发进度

### 🔄 v1.0.1 (进行中)
- [ ] **卸载器重构**:
  - 添加可选卸载界面，用户可选择保留哪些组件
  - 依次卸载：Node.js 运行时、Python 运行时、OpenClaw 核心、Skills、Memories
  - 保留用户配置文件选项
- [ ] **安装问题修复**:
  - 修复 NSIS 钩子导致安装卡住的问题
  - 使用 nsExec 替代 ExecWait 执行 tar 解压

### ✅ v1.0.0 (全量离线版 Final)
- [x] **CI/CD 全量打包**:
  - 自动化下载并封包 Node/Python 运行时
  - 预执行 `npm install` 封装完整 `node_modules`
  - 打包全量职场 Skill (Excel, Word, Humanizer等)
- [x] **安装器逻辑重构**:
  - 对齐 `%USERPROFILE%\.openclaw` 安装路径
  - 实现 NSIS 环境变量自动化注册
  - 完善清理卸载流程
- [x] **UI/UX 深度优化**:
  - 动态展示本命路径提示
  - 修复 Skill 全选逻辑与布局
  - 整合模型-记忆联动提示

### ✅ 已完成 (v0.9.x 阶段)
- [x] CPU AVX2 指令集全自动检测
- [x] 物理安装引擎 `perform_install`
- [x] 健康度评分与一键修复
- [x] 云编译缓存优化

---

## 构建历史

| 版本 | 日期 | 说明 |
|------|------|------|
| **v1.0.1** | 2026-04-23 | 修复安装卡住问题，重构卸载器支持可选卸载 |
| **v1.0.0** | 2026-04-11 | **全量离线版发布**: 整合内置运行时，对齐本命路径，环境变量一键配置 |
| v0.9.3 | 2026-04-11 | 修正4GB内存检测，修复产物丢失 |
| v0.9.2 | 2026-04-08 | 核心物理安装引擎落地 |

---

## 安装目录结构

```
%USERPROFILE%\.openclaw\
├── openclaw.json              # 配置文件
├── resources\
│   ├── node-runtime\          # Node.js v22.12.0 运行时 (~80MB)
│   ├── python-runtime\        # Python 3.10.11 运行时 (~15MB)
│   ├── openclaw\              # OpenClaw 核心
│   │   ├── node_modules\      # 解压后的依赖
│   │   └── start.js           # 启动脚本
│   ├── skills\                # 职场技能包
│   ├── memories\              # 记忆系统
│   └── bin\
│       └── openclaw.cmd       # CLI 工具
└── workspace\                 # 用户工作空间
```

---

## 备注
- 最终安装包体积由于包含离线运行时，约 172MB
