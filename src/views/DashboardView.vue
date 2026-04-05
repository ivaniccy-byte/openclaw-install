<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { ElButton, ElCard, ElProgress, ElTag } from 'element-plus'

interface Status {
  running: boolean
  port: number
  uptime_seconds: number
  memory_mb: number
  cpu_percent: number
}

interface HealthScore {
  score: number
  level: string
  details: any
}

const status = ref<Status>({
  running: false,
  port: 18789,
  uptime_seconds: 0,
  memory_mb: 0,
  cpu_percent: 0,
})

const healthScore = ref<HealthScore>({
  score: 100,
  level: '优秀',
  details: {},
})

const loading = ref(false)
let pollTimer: number | null = null

const fetchStatus = async () => {
  try {
    status.value = await invoke<Status>('get_openclaw_status')
    healthScore.value = await invoke<HealthScore>('get_health_score')
  } catch (e) {
    console.error('获取状态失败:', e)
  }
}

const startOpenClaw = async () => {
  loading.value = true
  try {
    await invoke('start_openclaw', { installPath: '', port: 18789 })
    await fetchStatus()
  } catch (e) {
    console.error('启动失败:', e)
  } finally {
    loading.value = false
  }
}

const stopOpenClaw = async () => {
  loading.value = true
  try {
    await invoke('stop_openclaw')
    await fetchStatus()
  } catch (e) {
    console.error('停止失败:', e)
  } finally {
    loading.value = false
  }
}

const formatUptime = (seconds: number): string => {
  const h = Math.floor(seconds / 3600)
  const m = Math.floor((seconds % 3600) / 60)
  const s = seconds % 60
  if (h > 0) return `${h}小时${m}分钟`
  if (m > 0) return `${m}分钟${s}秒`
  return `${s}秒`
}

const getHealthColor = (level: string): string => {
  const colors: Record<string, string> = {
    优秀: '#67c23a',
    正常: '#85ce61',
    警告: '#e6a23c',
    异常: '#f56c6c',
  }
  return colors[level] || '#909399'
}

const openWebPanel = () => {
  window.open(`http://localhost:${status.value.port}`, '_blank')
}

onMounted(() => {
  fetchStatus()
  pollTimer = window.setInterval(fetchStatus, 5000)
})

onUnmounted(() => {
  if (pollTimer) {
    clearInterval(pollTimer)
  }
})
</script>

<template>
  <div class="dashboard">
    <div class="page-header">
      <h1 class="page-title">控制面板</h1>
      <p class="page-subtitle">一键启动/停止OpenClaw服务，查看运行状态</p>
    </div>

    <!-- 核心状态卡片 -->
    <div class="grid grid-2" style="margin-bottom: 24px">
      <!-- 启动控制卡片 -->
      <el-card class="control-card">
        <div class="control-content">
          <div class="status-display">
            <div
              class="status-circle"
              :class="{
                running: status.running,
                stopped: !status.running,
              }"
            >
              <el-icon :size="48">
                <VideoPlay v-if="!status.running" />
                <VideoPause v-else />
              </el-icon>
            </div>
            <div class="status-text">
              <h2>{{ status.running ? '运行中' : '已停止' }}</h2>
              <p>端口: {{ status.port }}</p>
            </div>
          </div>

          <div class="control-buttons">
            <el-button
              type="success"
              size="large"
              :loading="loading"
              :disabled="status.running"
              @click="startOpenClaw"
            >
              <el-icon><VideoPlay /></el-icon>
              一键启动
            </el-button>
            <el-button
              type="danger"
              size="large"
              :loading="loading"
              :disabled="!status.running"
              @click="stopOpenClaw"
            >
              <el-icon><VideoPause /></el-icon>
              一键停止
            </el-button>
          </div>
        </div>
      </el-card>

      <!-- 健康度卡片 -->
      <el-card class="health-card">
        <div class="health-content">
          <div class="health-score">
            <el-progress
              type="dashboard"
              :percentage="healthScore.score"
              :color="getHealthColor(healthScore.level)"
              :width="120"
            >
              <template #default>
                <span class="score-value">{{ healthScore.score }}</span>
                <span class="score-label">{{ healthScore.level }}</span>
              </template>
            </el-progress>
          </div>
          <div class="health-info">
            <h3>健康度评分</h3>
            <p v-if="status.running">运行时长: {{ formatUptime(status.uptime_seconds) }}</p>
            <p v-else>服务未启动</p>
            <el-tag :type="healthScore.score >= 70 ? 'success' : 'danger'" size="small">
              {{ healthScore.score >= 90 ? '优秀' : healthScore.score >= 70 ? '正常' : '需修复' }}
            </el-tag>
          </div>
        </div>
      </el-card>
    </div>

    <!-- 运行信息 -->
    <el-card v-if="status.running" class="info-card">
      <template #header>
        <div class="card-header">
          <span class="card-title">
            <el-icon><Monitor /></el-icon>
            实时运行信息
          </span>
        </div>
      </template>
      <div class="info-grid">
        <div class="info-item">
          <span class="info-label">内存占用</span>
          <span class="info-value">{{ status.memory_mb }} MB</span>
        </div>
        <div class="info-item">
          <span class="info-label">CPU占用</span>
          <span class="info-value">{{ status.cpu_percent.toFixed(1) }}%</span>
        </div>
        <div class="info-item">
          <span class="info-label">监听端口</span>
          <span class="info-value">{{ status.port }}</span>
        </div>
        <div class="info-item">
          <span class="info-label">运行时长</span>
          <span class="info-value">{{ formatUptime(status.uptime_seconds) }}</span>
        </div>
      </div>
    </el-card>

    <!-- 快捷入口 -->
    <div class="quick-actions">
      <el-card class="action-card" @click="openWebPanel">
        <div class="action-content">
          <el-icon :size="32"><Browser /></el-icon>
          <span>打开Web管理面板</span>
        </div>
      </el-card>
      <el-card class="action-card" @click="$router.push('/config')">
        <div class="action-content">
          <el-icon :size="32"><Setting /></el-icon>
          <span>快速配置向导</span>
        </div>
      </el-card>
      <el-card class="action-card" @click="$router.push('/health')">
        <div class="action-content">
          <el-icon :size="32"><FirstAidKit /></el-icon>
          <span>常见问题修复</span>
        </div>
      </el-card>
    </div>
  </div>
</template>

<style scoped>
.dashboard {
  max-width: 1200px;
}

.control-card,
.health-card {
  height: 280px;
}

.control-content,
.health-content {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  height: 100%;
  gap: 24px;
}

.status-display {
  display: flex;
  align-items: center;
  gap: 20px;
}

.status-circle {
  width: 80px;
  height: 80px;
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.3s ease;
}

.status-circle.running {
  background: linear-gradient(135deg, #67c23a 0%, #5daf34 100%);
  color: white;
  box-shadow: 0 4px 20px rgba(103, 194, 58, 0.4);
}

.status-circle.stopped {
  background: linear-gradient(135deg, #909399 0%, #767676 100%);
  color: white;
  box-shadow: 0 4px 20px rgba(144, 147, 153, 0.4);
}

.status-text h2 {
  font-size: 24px;
  margin-bottom: 8px;
  color: #303133;
}

.status-text p {
  color: #909399;
  font-size: 14px;
}

.control-buttons {
  display: flex;
  gap: 16px;
}

.health-content {
  flex-direction: row;
  gap: 40px;
}

.score-value {
  font-size: 32px;
  font-weight: 600;
  color: #303133;
}

.score-label {
  font-size: 14px;
  color: #909399;
}

.health-info h3 {
  font-size: 16px;
  margin-bottom: 8px;
  color: #303133;
}

.health-info p {
  font-size: 14px;
  color: #909399;
  margin-bottom: 12px;
}

.info-card {
  margin-bottom: 24px;
}

.card-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.info-grid {
  display: grid;
  grid-template-columns: repeat(4, 1fr);
  gap: 20px;
}

.info-item {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 8px;
}

.info-label {
  font-size: 12px;
  color: #909399;
}

.info-value {
  font-size: 20px;
  font-weight: 600;
  color: #303133;
}

.quick-actions {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 20px;
}

.action-card {
  cursor: pointer;
  transition: all 0.3s ease;
}

.action-card:hover {
  transform: translateY(-4px);
  box-shadow: 0 8px 24px rgba(0, 0, 0, 0.1);
}

.action-content {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 12px;
  padding: 20px;
  color: #667eea;
}

.action-content span {
  font-size: 14px;
  color: #303133;
}
</style>
