<script setup lang="ts">
import { ref } from 'vue'
import { ElCard, ElButton, ElSwitch, ElTag, ElEmpty, ElMessage } from 'element-plus'

const Message = ElMessage

interface Plugin {
  id: string
  name: string
  description: string
  enabled: boolean
  installed: boolean
}

interface Skill {
  id: string
  name: string
  category: string
  enabled: boolean
  installed: boolean
}

const plugins = ref<Plugin[]>([
  {
    id: 'feishu',
    name: '飞书插件',
    description: '接入飞书平台，实现消息收发',
    enabled: false,
    installed: true,
  },
  {
    id: 'wechat',
    name: '微信插件',
    description: '接入微信平台（需遵守使用规范）',
    enabled: false,
    installed: true,
  },
])

const memorySystems = ref([
  {
    id: 'none',
    name: '不启用记忆系统',
    description: '使用OpenClaw原生记忆功能',
    selected: true,
  },
  {
    id: 'openviking',
    name: 'OpenViking 记忆系统',
    description: '需要配置Embedding模型，用于记忆向量生成与检索',
    selected: false,
  },
  {
    id: 'loseless',
    name: 'Loseless Claw + LanceDB Pro',
    description: '需要配置Embedding+Rerank模型，长上下文精准召回',
    selected: false,
  },
])

const skills = ref<Skill[]>([
  {
    id: 'tavily-search',
    name: 'AI精准搜索',
    category: '办公必备',
    enabled: true,
    installed: true,
  },
  {
    id: 'multi-search',
    name: '多引擎全网搜索',
    category: '办公必备',
    enabled: true,
    installed: true,
  },
  {
    id: 'humanizer',
    name: '文字润色优化',
    category: '办公必备',
    enabled: false,
    installed: false,
  },
  {
    id: 'brainstorming',
    name: '创意头脑风暴',
    category: '办公必备',
    enabled: false,
    installed: false,
  },
  {
    id: 'word-processor',
    name: 'Word文档处理',
    category: 'Office三件套',
    enabled: false,
    installed: false,
  },
  {
    id: 'excel-automation',
    name: 'Excel自动化',
    category: 'Office三件套',
    enabled: false,
    installed: false,
  },
  {
    id: 'ppt-visual',
    name: 'PPT演示制作',
    category: 'Office三件套',
    enabled: false,
    installed: false,
  },
  {
    id: 'coding-agent',
    name: '轻量代码助手',
    category: '开发辅助',
    enabled: false,
    installed: false,
  },
  {
    id: 'self-improving',
    name: '自我进化智能体',
    category: '开发辅助',
    enabled: false,
    installed: false,
  },
])

const selectedMemory = ref('none')

const installPlugin = (plugin: Plugin) => {
  plugin.installed = true
  Message.success(`${plugin.name} 安装成功`)
}

const uninstallPlugin = (plugin: Plugin) => {
  plugin.installed = false
  plugin.enabled = false
  Message.success(`${plugin.name} 已卸载`)
}

const selectMemory = (memory: any) => {
  memorySystems.value.forEach((m) => (m.selected = m.id === memory.id))
  selectedMemory.value = memory.id
  Message.success(`已选择: ${memory.name}`)
}

const installSkill = (skill: Skill) => {
  skill.installed = true
  skill.enabled = true
  Message.success(`${skill.name} 安装并启用成功`)
}

const uninstallSkill = (skill: Skill) => {
  skill.installed = false
  skill.enabled = false
  Message.success(`${skill.name} 已卸载`)
}

const getCategoryColor = (category: string): 'success' | 'warning' | 'info' => {
  const colors: Record<string, 'success' | 'warning' | 'info'> = {
    办公必备: 'success',
    Office三件套: 'warning',
    开发辅助: 'info',
  }
  return colors[category] || 'info'
}
</script>

<template>
  <div class="function-manage">
    <div class="page-header">
      <h1 class="page-title">功能管理</h1>
      <p class="page-subtitle">管理聊天插件、记忆系统、职场Skill包</p>
    </div>

    <!-- 聊天插件管理 -->
    <el-card class="manage-card">
      <template #header>
        <span class="card-title">
          <el-icon><ChatDotRound /></el-icon>
          聊天插件管理
        </span>
      </template>

      <div class="plugin-list">
        <div v-for="plugin in plugins" :key="plugin.id" class="plugin-item">
          <div class="plugin-info">
            <h4>{{ plugin.name }}</h4>
            <p>{{ plugin.description }}</p>
          </div>
          <div class="plugin-actions">
            <el-tag v-if="plugin.installed" type="success" size="small">已安装</el-tag>
            <template v-if="plugin.installed">
              <el-switch v-model="plugin.enabled" />
              <el-button size="small" type="danger" plain @click="uninstallPlugin(plugin)">卸载</el-button>
            </template>
            <el-button v-else type="primary" size="small" @click="installPlugin(plugin)">安装</el-button>
          </div>
        </div>
      </div>

      <el-alert
        v-if="plugins.some((p) => p.id === 'wechat' && p.installed)"
        title="微信插件使用风险提示"
        type="warning"
        description="使用微信插件请遵守微信使用协议，避免账号违规"
        :closable="false"
        style="margin-top: 16px"
      />
    </el-card>

    <!-- 记忆系统管理 -->
    <el-card class="manage-card">
      <template #header>
        <span class="card-title">
          <el-icon><Grid /></el-icon>
          记忆系统管理
        </span>
      </template>

      <el-alert
        title="记忆系统方案互斥，只能选择一个"
        type="info"
        :closable="false"
        style="margin-bottom: 16px"
      />

      <div class="memory-list">
        <div
          v-for="memory in memorySystems"
          :key="memory.id"
          class="memory-item"
          :class="{ selected: memory.selected }"
          @click="selectMemory(memory)"
        >
          <div class="memory-radio">
            <el-radio :model-value="selectedMemory" :label="memory.id">&nbsp;</el-radio>
          </div>
          <div class="memory-info">
            <h4>{{ memory.name }}</h4>
            <p>{{ memory.description }}</p>
          </div>
        </div>
      </div>

      <el-alert
        title="切换记忆系统后，请前往配置中心完成对应的模型配置"
        type="warning"
        :closable="false"
        style="margin-top: 16px"
      />
    </el-card>

    <!-- Skill管理 -->
    <el-card class="manage-card">
      <template #header>
        <span class="card-title">
          <el-icon><Box /></el-icon>
          职场Skill技能包
        </span>
      </template>

      <div class="skill-grid">
        <div v-for="skill in skills" :key="skill.id" class="skill-item">
          <div class="skill-header">
            <el-tag :type="getCategoryColor(skill.category)" size="small">
              {{ skill.category }}
            </el-tag>
            <el-switch v-model="skill.enabled" :disabled="!skill.installed" />
          </div>
          <h4>{{ skill.name }}</h4>
          <div class="skill-actions">
            <template v-if="skill.installed">
              <el-button size="small" type="danger" plain @click="uninstallSkill(skill)">卸载</el-button>
            </template>
            <el-button v-else size="small" type="primary" @click="installSkill(skill)">安装</el-button>
          </div>
        </div>
      </div>

      <el-empty v-if="skills.length === 0" description="暂无可用技能" />
    </el-card>
  </div>
</template>

<style scoped>
.function-manage {
  max-width: 1200px;
}

.manage-card {
  margin-bottom: 24px;
}

.plugin-list,
.memory-list {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.plugin-item,
.memory-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 16px;
  border: 1px solid #ebeef5;
  border-radius: 8px;
  transition: all 0.3s ease;
}

.memory-item {
  cursor: pointer;
}

.memory-item:hover {
  border-color: #667eea;
}

.memory-item.selected {
  border-color: #667eea;
  background: linear-gradient(135deg, rgba(102, 126, 234, 0.05) 0%, rgba(118, 75, 162, 0.05) 100%);
}

.plugin-info h4,
.memory-info h4 {
  margin-bottom: 4px;
  color: #303133;
}

.plugin-info p,
.memory-info p {
  font-size: 13px;
  color: #909399;
}

.plugin-actions,
.skill-actions {
  display: flex;
  align-items: center;
  gap: 12px;
}

.memory-radio {
  margin-right: 12px;
}

.skill-grid {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 16px;
}

.skill-item {
  padding: 16px;
  border: 1px solid #ebeef5;
  border-radius: 8px;
  transition: all 0.3s ease;
}

.skill-item:hover {
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.08);
}

.skill-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 12px;
}

.skill-item h4 {
  margin-bottom: 12px;
  color: #303133;
  font-size: 14px;
}

.skill-actions {
  justify-content: flex-end;
}
</style>
