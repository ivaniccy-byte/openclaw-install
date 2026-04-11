<script setup lang="ts">
import { ref, computed, watch } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { ElCard, ElButton, ElProgress, ElAlert, ElCheckbox, ElRadio, ElRadioGroup, ElTag, ElMessage, ElMessageBox } from 'element-plus'

const Message = ElMessage

interface EnvCheckResult {
  cpu_avx2: boolean
  windows_version_ok: boolean
  windows_build_number: number
  disk_space_ok: boolean
  disk_space_gb: number
  is_ssd: boolean
  memory_ok: boolean
  memory_gb: number
  nodejs_exists: boolean
  nodejs_version: string | null
  port_available: boolean
  recommended_port: number
  network_ok: boolean
}

const currentStep = ref(1)
const installPath = ref('C:\\OpenClawWorkplace')
const loading = ref(false)
const installing = ref(false)
const installProgress = ref(0)

const envCheck = ref<EnvCheckResult | null>(null)

// 可选组件
const selectedPlugins = ref<string[]>([])
const selectedMemory = ref('none')
const selectedSkills = ref<string[]>([])

const plugins = [
  { id: 'feishu', label: '飞书插件' },
  { id: 'wechat', label: '微信插件' },
]

// 记忆系统：只保留 none 和 lossless-enhanced
const memoryOptions = [
  { id: 'none', label: '不启用（使用OpenClaw原生记忆）' },
  { id: 'lossless-enhanced', label: 'Lossless Claw Enhanced + Memory LanceDB Pro（短期记录和长期记忆）' },
]

const skills = [
  { id: 'tavily-search', label: 'Tavily Search' },
  { id: 'multi-search', label: '多引擎全网搜索' },
  { id: 'humanizer', label: '文字润色优化' },
  { id: 'brainstorming', label: '创意头脑风暴' },
  { id: 'word-processor', label: 'Word文档处理' },
  { id: 'excel-automation', label: 'Excel自动化' },
  { id: 'ppt-visual', label: 'PPT演示制作' },
  { id: 'coding-agent', label: '代码助手' },
  { id: 'self-improving', label: '自我进化智能体' },
]

// 全选逻辑
const checkAllSkills = ref(false)
const isIndeterminate = ref(false)

const handleCheckAllSkillsChange = (val: string | number | boolean) => {
  selectedSkills.value = val ? skills.map(s => s.id) : []
  isIndeterminate.value = false
}

const handleSelectedSkillsChange = (value: (string | number | boolean)[]) => {
  const checkedCount = value.length
  checkAllSkills.value = checkedCount === skills.length
  isIndeterminate.value = checkedCount > 0 && checkedCount < skills.length
}

// 监听 selectedSkills 变化来处理弹窗
watch(selectedSkills, (newVal, oldVal) => {
  // 检查是否新增了 tavily-search
  if (newVal.includes('tavily-search') && !oldVal.includes('tavily-search')) {
    ElMessageBox.alert(
      'Tavily Search 需要申请独立的 API KEY 后才能生效。在安装完成后的“配置中心”中，我们提供了一键申请链接。',
      '功能说明',
      { confirmButtonText: '我知道了', type: 'info' }
    )
  }
})

// 监听记忆系统变化
watch(selectedMemory, (newVal) => {
  if (newVal === 'lossless-enhanced') {
    ElMessageBox.alert(
      'Lossless-Claw Enhanced 方案需要配置多个模型（LLM、Embedding、Rerank）以实现最佳效果。在后续配置界面，我们将为你给出 SiliconFlow（硅基流动）等高性价比模型的推荐配置。',
      '记忆系统说明',
      { confirmButtonText: '我知道了', type: 'info' }
    )
  }
})

const canProceed = computed(() => {
  if (currentStep.value === 1) {
    if (!envCheck.value) return false
    if (!envCheck.value.cpu_avx2) return false
    if (!envCheck.value.disk_space_ok) return false
    if (!envCheck.value.memory_ok) return false
    // 强制检查必须安装在C盘
    if (!installPath.value.toUpperCase().startsWith('C:')) return false
    return true
  }
  return true
})

const checkEnvironment = async () => {
  loading.value = true
  try {
    envCheck.value = await invoke<EnvCheckResult>('check_environment', { installPath: installPath.value })
  } catch (e) {
    console.error('环境检测失败:', e)
    Message.error('环境检测失败')
  } finally {
    loading.value = false
  }
}

const nextStep = () => {
  if (currentStep.value < 5) {
    currentStep.value++
  }
}

const prevStep = () => {
  if (currentStep.value > 1) {
    currentStep.value--
  }
}

const finishInstall = () => {
  window.location.reload()
}

const getStatusIcon = (status: boolean) => (status ? 'success' : 'danger')
const getStatusText = (status: boolean) => (status ? '通过' : '不通过')

// 监听步骤变化，自动触发安装
watch(currentStep, (newStep) => {
  if (newStep === 4) {
    startRealInstall()
  }
})

const startRealInstall = async () => {
  installing.value = true
  installProgress.value = 10 // 初始进度

  try {
    const options = {
      install_path: installPath.value,
      selected_plugins: selectedPlugins.value,
      selected_memory: selectedMemory.value,
      selected_skills: selectedSkills.value
    }

    // 调用真实的后端安装命令
    await invoke('perform_installation', { options })
    
    installProgress.value = 60 // 复制完成
    
    // 模拟最后的零碎配置时间
    const timer = setInterval(() => {
      installProgress.value += 5
      if (installProgress.value >= 100) {
        installProgress.value = 100
        clearInterval(timer)
        setTimeout(() => {
          installing.value = false
          currentStep.value = 5
          localStorage.setItem('openclaw_installed', 'true')
          Message.success('配置完成，安装成功！')
        }, 500)
      }
    }, 200)

  } catch (e) {
    console.error('安装部署失败:', e)
    Message.error(`安装失败: ${e}`)
    installing.value = false
    currentStep.value = 3 // 返回上一步
  }
}
</script>

<template>
  <div class="installer">
    <div class="installer-header">
      <h1>OpenClaw 一键安装版 安装向导</h1>
      <p>OpenClaw 3.28原版 · 零代码 · 一键安装</p>
    </div>

    <!-- 步骤指示器 -->
    <div class="steps-indicator">
      <div v-for="n in 5" :key="n" class="step-dot" :class="{ active: currentStep >= n, current: currentStep === n }">
        <span class="step-number">{{ n }}</span>
        <span class="step-label">
          {{ n === 1 ? '环境检测' : n === 2 ? '选择路径' : n === 3 ? '选择功能' : n === 4 ? '安装中' : '完成' }}
        </span>
      </div>
    </div>

    <!-- 步骤1：环境预检测 -->
    <el-card v-if="currentStep === 1" class="step-card">
      <template #header>
        <span class="card-title">
          <el-icon><Search /></el-icon>
          安装前环境检测
        </span>
      </template>

      <el-button type="primary" :loading="loading" @click="checkEnvironment" style="margin-bottom: 20px">
        开始检测
      </el-button>

      <div v-if="envCheck" class="check-results">
        <el-alert
          v-if="!envCheck.cpu_avx2"
          title="CPU不支持AVX2指令集"
          description="您的电脑CPU不支持AVX2指令集，无法运行OpenClaw，安装已终止。"
          type="error"
          :closable="false"
        />
        <el-alert
          v-else-if="!envCheck.memory_ok"
          title="内存不足"
          description="您的电脑内存小于4GB，无法正常运行OpenClaw。"
          type="error"
          :closable="false"
        />
        <el-alert
          v-else-if="!envCheck.disk_space_ok"
          title="磁盘空间不足"
          description="系统盘(C盘)可用空间小于15GB，无法保证OpenClaw稳定运行，请清理空间后再试。"
          type="error"
          :closable="false"
        />
        <el-alert
          v-else-if="!installPath.toUpperCase().startsWith('C:')"
          title="安装盘符受限"
          description="当前版本仅支持安装在系统盘(C盘)，请修改路径。"
          type="warning"
          :closable="false"
        />
        <el-alert v-else title="环境检测通过" description="您的电脑满足安装要求，可以继续安装" type="success" :closable="false" />

        <div class="check-list">
          <div class="check-item">
            <span>CPU AVX2指令集</span>
            <el-tag :type="getStatusIcon(envCheck.cpu_avx2)">{{ getStatusText(envCheck.cpu_avx2) }}</el-tag>
          </div>
          <div class="check-item">
            <span>Windows版本</span>
            <el-tag :type="getStatusIcon(envCheck.windows_version_ok)">
              {{ envCheck.windows_build_number }} {{ envCheck.windows_version_ok ? '通过' : '需升级' }}
            </el-tag>
          </div>
          <div class="check-item">
            <span>系统盘空间 (需≥15GB)</span>
            <el-tag :type="getStatusIcon(envCheck.disk_space_ok)">
              {{ envCheck.disk_space_gb }}GB {{ envCheck.disk_space_ok ? '通过' : '不足' }}
            </el-tag>
          </div>
          <div class="check-item">
            <span>磁盘类型</span>
            <el-tag :type="envCheck.is_ssd ? 'success' : 'warning'">
              {{ envCheck.is_ssd ? 'SSD固态硬盘' : '机械硬盘(建议使用SSD)' }}
            </el-tag>
          </div>
          <div class="check-item">
            <span>内存 (需≥4GB)</span>
            <el-tag :type="getStatusIcon(envCheck.memory_ok)">
              {{ envCheck.memory_gb }}GB {{ envCheck.memory_ok ? '通过' : '不足' }}
            </el-tag>
          </div>
          <div class="check-item">
            <span>Node.js运行时 (推荐v22+)</span>
            <el-tag :type="envCheck.nodejs_exists ? 'success' : 'info'">
              {{ envCheck.nodejs_exists ? `已安装 ${envCheck.nodejs_version}` : '将使用内置运行时' }}
            </el-tag>
          </div>
          <div class="check-item">
            <span>端口 18789</span>
            <el-tag :type="envCheck.port_available ? 'success' : 'warning'">
              {{ envCheck.port_available ? '可用' : `已被占用，将使用${envCheck.recommended_port}` }}
            </el-tag>
          </div>
          <div class="check-item">
            <span>网络环境</span>
            <el-tag :type="envCheck.network_ok ? 'success' : 'warning'">
              {{ envCheck.network_ok ? '正常' : '可能受影响' }}
            </el-tag>
          </div>
        </div>
      </div>

      <div class="step-actions">
        <el-button type="primary" :disabled="!canProceed" @click="nextStep">下一步</el-button>
      </div>
    </el-card>

    <!-- 步骤2：安装路径 -->
    <el-card v-if="currentStep === 2" class="step-card">
      <template #header>
        <span class="card-title">
          <el-icon><Folder /></el-icon>
          选择安装路径
        </span>
      </template>

      <el-alert title="当前版本仅支持安装在系统盘(C盘)，以确保OpenClaw核心组件的绝对路径引用正确" type="warning" :closable="false" style="margin-bottom: 20px" />
      
      <div class="path-selector">
        <el-input v-model="installPath" placeholder="请输入安装路径" style="margin-bottom: 12px" />
        <p class="path-hint">固定安装在C盘，建议保留默认路径。需要至少15GB可用空间（含运行缓存）</p>
      </div>

      <div class="step-actions">
        <el-button @click="prevStep">上一步</el-button>
        <el-button type="primary" @click="nextStep">下一步</el-button>
      </div>
    </el-card>

    <!-- 步骤3：可选功能 -->
    <el-card v-if="currentStep === 3" class="step-card">
      <template #header>
        <span class="card-title">
          <el-icon><Grid /></el-icon>
          可选功能配置
        </span>
      </template>

      <el-alert title="所有功能均为可选，默认全不勾选" type="info" :closable="false" style="margin-bottom: 20px" />

      <!-- 聊天插件 -->
      <div class="option-section">
        <h4>聊天平台插件</h4>
        <el-checkbox-group v-model="selectedPlugins">
          <el-checkbox v-for="p in plugins" :key="p.id" :value="p.id">{{ p.label }}</el-checkbox>
        </el-checkbox-group>
      </div>

      <!-- 记忆系统 -->
      <div class="option-section">
        <h4>记忆系统方案</h4>
        <el-radio-group v-model="selectedMemory">
          <el-radio v-for="m in memoryOptions" :key="m.id" :value="m.id">{{ m.label }}</el-radio>
        </el-radio-group>
      </div>

      <!-- Skill包 -->
      <div class="option-section">
        <div class="section-header">
          <h4>预封装职场Skill包</h4>
          <el-checkbox
            v-model="checkAllSkills"
            :indeterminate="isIndeterminate"
            @change="handleCheckAllSkillsChange"
          >
            全选
          </el-checkbox>
        </div>
        <el-checkbox-group v-model="selectedSkills" @change="handleSelectedSkillsChange">
          <el-checkbox v-for="s in skills" :key="s.id" :value="s.id">{{ s.label }}</el-checkbox>
        </el-checkbox-group>
      </div>

      <div class="step-actions">
        <el-button @click="prevStep">上一步</el-button>
        <el-button type="primary" @click="nextStep">下一步</el-button>
      </div>
    </el-card>

    <!-- 步骤4：安装中 -->
    <el-card v-if="currentStep === 4" class="step-card">
      <template #header>
        <span class="card-title">
          <el-icon><Download /></el-icon>
          正在安装...
        </span>
      </template>

      <div class="install-progress">
        <el-progress :percentage="Math.round(installProgress)" :stroke-width="20" />
        <p>正在部署 OpenClaw 一键安装版，请稍候...</p>
      </div>
    </el-card>

    <!-- 步骤5：完成 -->
    <el-card v-if="currentStep === 5" class="step-card">
      <template #header>
        <span class="card-title">
          <el-icon><CircleCheck /></el-icon>
          安装完成
        </span>
      </template>

      <div class="complete-content">
        <div class="success-icon">
          <el-icon :size="64" color="#67c23a"><CircleCheck /></el-icon>
        </div>
        <h2>OpenClaw 一键安装版安装成功！</h2>
        <p>感谢您选择 OpenClaw 一键安装版，祝您工作愉快</p>

        <div class="complete-actions">
          <el-button type="primary" size="large" @click="finishInstall">立即启动</el-button>
        </div>
      </div>
    </el-card>
  </div>
</template>

<style scoped>
.installer {
  max-width: 800px;
  margin: 0 auto;
}

.installer-header {
  text-align: center;
  margin-bottom: 40px;
}

.installer-header h1 {
  font-size: 28px;
  color: #303133;
  margin-bottom: 8px;
}

.installer-header p {
  color: #909399;
  font-size: 14px;
}

.steps-indicator {
  display: flex;
  justify-content: space-between;
  margin-bottom: 40px;
  position: relative;
}

.steps-indicator::before {
  content: '';
  position: absolute;
  top: 15px;
  left: 50px;
  right: 50px;
  height: 2px;
  background: #dcdfe6;
}

.step-dot {
  display: flex;
  flex-direction: column;
  align-items: center;
  z-index: 1;
}

.step-number {
  width: 32px;
  height: 32px;
  border-radius: 50%;
  background: #dcdfe6;
  color: white;
  display: flex;
  align-items: center;
  justify-content: center;
  font-weight: 600;
  margin-bottom: 8px;
  transition: all 0.3s ease;
}

.step-dot.active .step-number {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
}

.step-dot.current .step-number {
  width: 40px;
  height: 40px;
  font-size: 16px;
}

.step-label {
  font-size: 13px;
  color: #909399;
}

.step-dot.active .step-label {
  color: #303133;
  font-weight: 500;
}

.step-card {
  margin-bottom: 24px;
}

.check-results {
  margin-top: 20px;
}

.check-list {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: 12px;
  margin-top: 20px;
}

.check-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 12px 16px;
  background: #f5f7fa;
  border-radius: 8px;
}

.check-item span:first-child {
  color: #606266;
  font-size: 14px;
}

.path-selector {
  margin: 20px 0;
}

.path-hint {
  font-size: 13px;
  color: #909399;
}

.section-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 12px;
}

.option-section h4 {
  margin-bottom: 0;
  color: #303133;
}

.install-progress {
  text-align: center;
  padding: 40px 0;
}

.install-progress p {
  margin-top: 20px;
  color: #909399;
}

.complete-content {
  text-align: center;
  padding: 40px 0;
}

.success-icon {
  margin-bottom: 20px;
}

.complete-content h2 {
  color: #303133;
  margin-bottom: 8px;
}

.complete-content p {
  color: #909399;
  margin-bottom: 30px;
}

.step-actions {
  display: flex;
  justify-content: flex-end;
  gap: 12px;
  margin-top: 24px;
}
</style>
