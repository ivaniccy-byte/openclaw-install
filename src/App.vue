<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useRouter, useRoute } from 'vue-router'

const router = useRouter()
const route = useRoute()

const isInstalled = ref(false)
const checkInstallStatus = async () => {
  // 检查是否已完成安装
  // 从本地存储读取安装状态
  const installed = localStorage.getItem('openclaw_installed')
  isInstalled.value = installed === 'true'
}

const navItems = [
  { path: '/', name: '首页', icon: 'HomeFilled' },
  { path: '/config', name: '配置中心', icon: 'Setting' },
  { path: '/health', name: '健康中心', icon: 'FirstAidKit' },
  { path: '/help', name: '帮助', icon: 'QuestionFilled' },
]

onMounted(() => {
  checkInstallStatus()
})

const goToInstaller = () => {
  router.push('/installer')
}
</script>

<template>
  <div class="app-container">
    <!-- 左侧导航 -->
    <aside class="sidebar">
      <div class="logo-area">
        <div class="logo-icon">
          <el-icon :size="32"><Operation /></el-icon>
        </div>
        <div class="logo-text">
          <span class="title">OpenClaw</span>
          <span class="subtitle">职场版</span>
        </div>
      </div>

      <nav class="nav-menu">
        <router-link
          v-for="item in navItems"
          :key="item.path"
          :to="item.path"
          class="nav-item"
          :class="{ active: route.path === item.path }"
        >
          <el-icon><component :is="item.icon" /></el-icon>
          <span>{{ item.name }}</span>
        </router-link>
      </nav>

      <div class="sidebar-footer">
        <div class="version-info">v0.9.3</div>
      </div>
    </aside>

    <!-- 主内容区 -->
    <main class="main-content">
      <router-view v-if="isInstalled || route.path === '/installer'" />
      <div v-else class="not-installed">
        <el-empty description="尚未安装 OpenClaw 职场版">
          <el-button type="primary" @click="goToInstaller">前往安装</el-button>
        </el-empty>
      </div>
    </main>
  </div>
</template>

<style scoped>
.app-container {
  display: flex;
  height: 100vh;
  background-color: #f5f7fa;
}

.sidebar {
  width: 220px;
  background: linear-gradient(180deg, #1a1a2e 0%, #16213e 100%);
  display: flex;
  flex-direction: column;
  padding: 20px 0;
}

.logo-area {
  display: flex;
  align-items: center;
  padding: 0 20px 30px;
  border-bottom: 1px solid rgba(255, 255, 255, 0.1);
}

.logo-icon {
  width: 40px;
  height: 40px;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  border-radius: 10px;
  display: flex;
  align-items: center;
  justify-content: center;
  color: white;
}

.logo-text {
  margin-left: 12px;
  display: flex;
  flex-direction: column;
}

.logo-text .title {
  color: white;
  font-size: 16px;
  font-weight: 600;
}

.logo-text .subtitle {
  color: rgba(255, 255, 255, 0.6);
  font-size: 12px;
}

.nav-menu {
  flex: 1;
  padding: 20px 10px;
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.nav-item {
  display: flex;
  align-items: center;
  padding: 12px 16px;
  border-radius: 8px;
  color: rgba(255, 255, 255, 0.7);
  text-decoration: none;
  transition: all 0.3s ease;
  font-size: 14px;
}

.nav-item:hover {
  background: rgba(255, 255, 255, 0.1);
  color: white;
}

.nav-item.active {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
}

.nav-item .el-icon {
  margin-right: 10px;
  font-size: 18px;
}

.sidebar-footer {
  padding: 20px;
  border-top: 1px solid rgba(255, 255, 255, 0.1);
}

.version-info {
  color: rgba(255, 255, 255, 0.4);
  font-size: 12px;
  text-align: center;
}

.main-content {
  flex: 1;
  overflow-y: auto;
  padding: 24px;
}

.not-installed {
  display: flex;
  align-items: center;
  justify-content: center;
  height: 100%;
}
</style>
