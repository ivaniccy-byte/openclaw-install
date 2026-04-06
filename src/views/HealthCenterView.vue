<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import {
  ElCard,
  ElProgress,
  ElButton,
  ElTag,
  ElTimeline,
  ElTimelineItem,
  ElAlert,
  ElEmpty,
} from 'element-plus'

interface HealthScore {
  score: number
  level: string
  details: any
}

interface Alert {
  id: string
  title: string
  description: string
  severity: string
  fix_type: string | null
}

const healthScore = ref<HealthScore>({
  score: 100,
  level: '优秀',
  details: {},
})

const alerts = ref<Alert[]>([])
const logs = ref<{ time: string; message: string; level: string }[]>([])
const loading = ref(false)
let pollTimer: number | null = null

const fixScripts = [
  { id: 'restart', name: '进程崩溃重启', desc: '当OpenClaw进程意外退出时自动重启', severity: 'high' },
  { id: 'port', name: '端口占用修复', desc: '当默认端口被占用时自动更换到可用端口', severity: 'medium' },
  { id: 'config', name: '配置损坏还原', desc: '当配置文件损坏时自动还原为默认配置', severity: 'high' },
  { id: 'dependency', name: '依赖缺失修复', desc: '当检测到缺少运行时依赖时自动重新安装', severity: 'high' },
  { id: 'network', name: '网络异常检测', desc: '检测并修复网络连接问题', severity: 'low' },
  { id: 'model_connection', name: '模型连通性排查', desc: '检测并修复大模型API连接问题', severity: 'high' },
]

const getHealthColor = (level: string): string => {
  const colors: Record<string, string> = {
    优秀: '#67c23a',
    正常: '#85ce61',
    警告: '#e6a23c',
    异常: '#f56c6c',
  }
  return colors[level] || '#909399'
}

const getSeverityColor = (severity: string): 'danger' | 'warning' | 'info' => {
  const colors: Record<string, 'danger' | 'warning' | 'info'> = {
    high: 'danger',
    medium: 'warning',
    low: 'info',
  }
  return colors[severity] || 'info'
}

const getSeverityText = (severity: string): string => {
  const texts: Record<string, string> = {
    high: '高',
    medium: '中',
    low: '低',
  }
  return texts[severity] || '低'
}

const fetchHealthScore = async () => {
  try {
    healthScore.value = await invoke<HealthScore>('get_health_score')
    // 生成模拟告警
    alerts.value = []
    if (!healthScore.value.details.process_alive) {
      alerts.value.push({
        id: 'process',
        title: '核心进程未运行',
        description: 'OpenClaw服务进程未启动，请尝试重启服务',
        severity: 'high',
        fix_type: 'restart',
      })
    }
    if (!healthScore.value.details.main_model_ok) {
      alerts.value.push({
        id: 'model',
        title: '主模型未配置',
        description: '请在配置中心配置大模型API密钥',
        severity: 'high',
        fix_type: null,
      })
    }
  } catch (e) {
    console.error('获取健康度失败:', e)
  }
}

const executeFix = async (fixType: string) => {
  loading.value = true
  try {
    await invoke('auto_fix', { fixType })
  } catch (e) {
    console.error('修复失败:', e)
  } finally {
    loading.value = false
  }
}

const clearLogs = () => {
  logs.value = []
}

onMounted(() => {
  fetchHealthScore()
  pollTimer = window.setInterval(fetchHealthScore, 10000)
})

onUnmounted(() => {
  if (pollTimer) {
    clearInterval(pollTimer)
  }
})
</script>

<template>
  <div class="health-center">
    <div class="page-header">
      <h1 class="page-title">健康中心</h1>
      <p class="page-subtitle">实时监控系统运行状态，一键修复常见问题</p>
    </div>

    <!-- 健康度总览 -->
    <el-card class="health-overview">
      <div class="health-display">
        <div class="health-score-circle">
          <el-progress
            type="dashboard"
            :percentage="healthScore.score"
            :color="getHealthColor(healthScore.level)"
            :width="160"
          >
            <template #default>
              <span class="score-value">{{ healthScore.score }}</span>
              <span class="score-label">{{ healthScore.level }}</span>
            </template>
          </el-progress>
        </div>
        <div class="health-details">
          <h3>健康度详情</h3>
          <div class="detail-grid">
            <div class="detail-item">
              <span class="label">核心进程</span>
              <el-tag :type="healthScore.details.process_alive ? 'success' : 'danger'" size="small">
                {{ healthScore.details.process_alive ? '正常' : '异常' }}
              </el-tag>
            </div>
            <div class="detail-item">
              <span class="label">端口监听</span>
              <el-tag :type="healthScore.details.port_listening ? 'success' : 'danger'" size="small">
                {{ healthScore.details.port_listening ? '正常' : '异常' }}
              </el-tag>
            </div>
            <div class="detail-item">
              <span class="label">主模型</span>
              <el-tag :type="healthScore.details.main_model_ok ? 'success' : 'warning'" size="small">
                {{ healthScore.details.main_model_ok ? '正常' : '未配置' }}
              </el-tag>
            </div>
            <div class="detail-item">
              <span class="label">记忆系统</span>
              <el-tag :type="healthScore.details.memory_system_ok ? 'success' : 'warning'" size="small">
                {{ healthScore.details.memory_system_ok ? '正常' : '异常' }}
              </el-tag>
            </div>
            <div class="detail-item">
              <span class="label">配置文件</span>
              <el-tag :type="healthScore.details.config_valid ? 'success' : 'danger'" size="small">
                {{ healthScore.details.config_valid ? '正常' : '异常' }}
              </el-tag>
            </div>
          </div>
          <div class="resource-info">
            <div class="resource-item">
              <span>CPU占用</span>
              <el-progress :percentage="healthScore.details.cpu_usage || 0" :show-text="false" :stroke-width="6" />
              <span class="value">{{ (healthScore.details.cpu_usage || 0).toFixed(1) }}%</span>
            </div>
            <div class="resource-item">
              <span>内存占用</span>
              <el-progress :percentage="healthScore.details.memory_usage || 0" :show-text="false" :stroke-width="6" />
              <span class="value">{{ (healthScore.details.memory_usage || 0).toFixed(1) }}%</span>
            </div>
          </div>
        </div>
      </div>
    </el-card>

    <!-- 异常告警 -->
    <el-card v-if="alerts.length > 0" class="alerts-card">
      <template #header>
        <span class="card-title">
          <el-icon><Warning /></el-icon>
          异常告警 ({{ alerts.length }})
        </span>
      </template>
      <el-alert
        v-for="alert in alerts"
        :key="alert.id"
        :title="alert.title"
        :description="alert.description"
        :type="alert.severity === 'high' ? 'error' : 'warning'"
        show-icon
        :closable="false"
        style="margin-bottom: 12px"
      >
        <template #extra>
          <el-button
            v-if="alert.fix_type"
            type="primary"
            size="small"
            :loading="loading"
            @click="executeFix(alert.fix_type)"
          >
            一键修复
          </el-button>
        </template>
      </el-alert>
    </el-card>

    <!-- 一键修复 -->
    <el-card class="fix-card">
      <template #header>
        <span class="card-title">
          <el-icon><Tools /></el-icon>
          常见问题一键修复
        </span>
      </template>
      <div class="fix-grid">
        <div v-for="fix in fixScripts" :key="fix.id" class="fix-item">
          <div class="fix-header">
            <h4>{{ fix.name }}</h4>
            <el-tag :type="getSeverityColor(fix.severity)" size="small">
              优先级: {{ getSeverityText(fix.severity) }}
            </el-tag>
          </div>
          <p>{{ fix.desc }}</p>
          <el-button type="primary" :loading="loading" @click="executeFix(fix.id)">执行修复</el-button>
        </div>
      </div>
    </el-card>

    <!-- 运行日志 -->
    <el-card class="logs-card">
      <template #header>
        <div class="card-header">
          <span class="card-title">
            <el-icon><Document /></el-icon>
            运行日志
          </span>
          <el-button size="small" @click="clearLogs">清空日志</el-button>
        </div>
      </template>
      <el-empty v-if="logs.length === 0" description="暂无日志" />
      <el-timeline v-else>
        <el-timeline-item v-for="(log, index) in logs" :key="index" :type="log.level === 'error' ? 'danger' : 'info'">
          <p>
            <span class="log-time">{{ log.time }}</span>
            <span class="log-message">{{ log.message }}</span>
          </p>
        </el-timeline-item>
      </el-timeline>
    </el-card>
  </div>
</template>

<style scoped>
.health-center {
  max-width: 1200px;
}

.health-overview {
  margin-bottom: 24px;
}

.health-display {
  display: flex;
  gap: 40px;
}

.health-score-circle {
  flex-shrink: 0;
}

.score-value {
  font-size: 40px;
  font-weight: 600;
  color: #303133;
}

.score-label {
  font-size: 16px;
  color: #909399;
}

.health-details h3 {
  margin-bottom: 16px;
  color: #303133;
}

.detail-grid {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 12px;
  margin-bottom: 24px;
}

.detail-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 8px 12px;
  background: #f5f7fa;
  border-radius: 4px;
}

.detail-item .label {
  font-size: 13px;
  color: #606266;
}

.resource-info {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.resource-item {
  display: flex;
  align-items: center;
  gap: 12px;
}

.resource-item span:first-child {
  width: 60px;
  font-size: 13px;
  color: #606266;
}

.resource-item .el-progress {
  flex: 1;
}

.resource-item .value {
  width: 50px;
  text-align: right;
  font-size: 13px;
  color: #303133;
}

.alerts-card,
.fix-card,
.logs-card {
  margin-bottom: 24px;
}

.fix-grid {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 16px;
}

.fix-item {
  padding: 16px;
  border: 1px solid #ebeef5;
  border-radius: 8px;
}

.fix-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 8px;
}

.fix-item h4 {
  color: #303133;
  font-size: 14px;
}

.fix-item p {
  font-size: 13px;
  color: #909399;
  margin-bottom: 12px;
}

.log-time {
  color: #909399;
  margin-right: 12px;
}

.log-message {
  color: #303133;
}
</style>
