<script setup lang="ts">
import { ref, computed, watch } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import {
  ElCard,
  ElForm,
  ElFormItem,
  ElInput,
  ElButton,
  ElSelect,
  ElOption,
  ElSwitch,
  ElDivider,
  ElAlert,
  ElTag,
  ElMessage,
} from 'element-plus'

const Message = ElMessage

interface ModelConfig {
  provider: string
  api_key: string
  endpoint: string
  model_name: string
}

interface AppConfig {
  port: number
  auto_start: boolean
  auto_restart: boolean
  low_power_mode: boolean
  memory_system: string
  main_model: ModelConfig | null
  embedding_model: ModelConfig | null
  rerank_model: ModelConfig | null
}

const config = ref<AppConfig>({
  port: 18789,
  auto_start: false,
  auto_restart: true,
  low_power_mode: false,
  memory_system: 'none',
  main_model: null,
  embedding_model: null,
  rerank_model: null,
})

const providers = [
  { value: 'volcengine', label: '火山引擎', url: 'https://ark.cn-beijing.volces.com/api/v3' },
  { value: 'tencent', label: '腾讯云', url: 'https://api.larliga.cn/v1' },
  { value: 'aliyun', label: '阿里百炼', url: 'https://dashscope.aliyuncs.com/api/v1' },
  { value: 'baidu', label: '百度千帆', url: 'https://qianfan.baidubce.com/v2' },
  { value: 'glm', label: '智谱GLM', url: 'https://open.bigmodel.cn/api/paas/v4' },
  { value: 'jd', label: '京东云', url: 'https://api.jdcloud.com/v1' },
  { value: 'minimax', label: 'MiniMax', url: 'https://api.minimax.chat/v1' },
  { value: 'custom', label: '自定义', url: '' },
]

const loading = ref(false)
const testingModel = ref<string | null>(null)

const showEmbeddingConfig = computed(() => {
  return config.value.memory_system === 'loseless'
})

const showRerankConfig = computed(() => {
  return config.value.memory_system === 'loseless'
})

watch(
  () => config.value.main_model?.provider,
  (newProvider) => {
    const provider = providers.find((p) => p.value === newProvider)
    if (provider && provider.value !== 'custom' && config.value.main_model) {
      config.value.main_model.endpoint = provider.url
    }
  }
)

const loadConfig = async () => {
  try {
    config.value = await invoke<AppConfig>('get_config')
  } catch (e) {
    console.error('加载配置失败:', e)
  }
}

const saveConfig = async () => {
  loading.value = true
  try {
    await invoke('save_config', { config: config.value })
    localStorage.setItem('openclaw_installed', 'true')
    Message.success('配置保存成功')
  } catch (e) {
    console.error('保存配置失败:', e)
    Message.error('保存配置失败')
  } finally {
    loading.value = false
  }
}

const testConnection = async (modelType: 'main' | 'embedding' | 'rerank') => {
  const model =
    modelType === 'main'
      ? config.value.main_model
      : modelType === 'embedding'
        ? config.value.embedding_model
        : config.value.rerank_model

  if (!model) {
    Message.warning('请先填写模型配置')
    return
  }

  testingModel.value = modelType
  try {
    const result = await invoke<boolean>('test_model_connection', { model })
    if (result) {
      Message.success(`${modelType === 'main' ? '主' : modelType === 'embedding' ? 'Embedding' : 'Rerank'}模型连接成功`)
    } else {
      Message.error('模型连接失败，请检查配置')
    }
  } catch (e) {
    Message.error(`连接失败: ${e}`)
  } finally {
    testingModel.value = null
  }
}

const initMainModel = () => {
  config.value.main_model = {
    provider: 'volcengine',
    api_key: '',
    endpoint: 'https://ark.cn-beijing.volces.com/api/v3',
    model_name: '',
  }
}

const initEmbeddingModel = () => {
  config.value.embedding_model = {
    provider: 'volcengine',
    api_key: '',
    endpoint: 'https://ark.cn-beijing.volces.com/api/v3',
    model_name: '',
  }
}

const initRerankModel = () => {
  config.value.rerank_model = {
    provider: 'volcengine',
    api_key: '',
    endpoint: 'https://ark.cn-beijing.volces.com/api/v3',
    model_name: '',
  }
}

const openProviderUrl = (provider: string) => {
  const urls: Record<string, string> = {
    volcengine: 'https://console.volcengine.com/ark',
    tencent: 'https://console.cloud.tencent.com',
    aliyun: 'https://dashscope.console.aliyun.com',
    baidu: 'https://console.bce.baidu.com',
    glm: 'https://open.bigmodel.cn',
    jd: 'https://www.jdcloud.com',
    minimax: 'https://www.minimax.chat',
  }
  if (urls[provider]) {
    window.open(urls[provider], '_blank')
  }
}

loadConfig()
</script>

<template>
  <div class="config-center">
    <div class="page-header">
      <h1 class="page-title">配置中心</h1>
      <p class="page-subtitle">配置大模型API、运行参数，与记忆系统联动</p>
    </div>

    <!-- 大模型配置 -->
    <el-card class="config-card">
      <template #header>
        <div class="card-header">
          <span class="card-title">
            <el-icon><Connection /></el-icon>
            大模型API配置
          </span>
          <el-tag type="success">必填</el-tag>
        </div>
      </template>

      <el-alert
        title="请选择您的大模型服务提供商，申请API密钥"
        type="info"
        :closable="false"
        style="margin-bottom: 20px"
      />

      <!-- 主模型配置 -->
      <div class="model-section">
        <div class="section-title">
          <span>主模型配置</span>
          <el-button size="small" @click="openProviderUrl(config.main_model?.provider || 'volcengine')">
            一键申请API密钥
          </el-button>
        </div>

        <el-form label-width="120px" v-if="config.main_model">
          <el-form-item label="服务商">
            <el-select v-model="config.main_model.provider" placeholder="请选择服务商" style="width: 100%">
              <el-option v-for="p in providers" :key="p.value" :label="p.label" :value="p.value" />
            </el-select>
          </el-form-item>
          <el-form-item label="API密钥">
            <el-input
              v-model="config.main_model.api_key"
              type="password"
              placeholder="请输入API密钥"
              show-password
            />
          </el-form-item>
          <el-form-item label="Endpoint">
            <el-input v-model="config.main_model.endpoint" placeholder="API地址" />
          </el-form-item>
          <el-form-item label="模型名称">
            <el-input v-model="config.main_model.model_name" placeholder="如: doubao-pro-32k" />
          </el-form-item>
          <el-form-item>
            <el-button type="primary" :loading="testingModel === 'main'" @click="testConnection('main')">
              一键测试连通性
            </el-button>
          </el-form-item>
        </el-form>

        <el-button v-else type="primary" plain @click="initMainModel">配置主模型</el-button>
      </div>

      <el-divider v-if="showEmbeddingConfig" />

      <!-- Embedding模型配置 -->
      <div class="model-section" v-if="showEmbeddingConfig">
        <div class="section-title">
          <span>Embedding模型配置</span>
          <el-tag type="warning">Loseless记忆系统必填</el-tag>
        </div>
        <p class="section-desc">用于记忆向量的生成与检索</p>

        <el-form label-width="120px" v-if="config.embedding_model">
          <el-form-item label="服务商">
            <el-select v-model="config.embedding_model.provider" placeholder="请选择服务商" style="width: 100%">
              <el-option v-for="p in providers" :key="p.value" :label="p.label" :value="p.value" />
            </el-select>
          </el-form-item>
          <el-form-item label="API密钥">
            <el-input
              v-model="config.embedding_model.api_key"
              type="password"
              placeholder="请输入API密钥"
              show-password
            />
          </el-form-item>
          <el-form-item label="Endpoint">
            <el-input v-model="config.embedding_model.endpoint" placeholder="API地址" />
          </el-form-item>
          <el-form-item label="模型名称">
            <el-input v-model="config.embedding_model.model_name" placeholder="如: _embedding-v1" />
          </el-form-item>
          <el-form-item>
            <el-button type="primary" :loading="testingModel === 'embedding'" @click="testConnection('embedding')">
              一键测试连通性
            </el-button>
          </el-form-item>
        </el-form>

        <el-button v-else type="warning" plain @click="initEmbeddingModel">配置Embedding模型</el-button>
      </div>

      <el-divider v-if="showRerankConfig" />

      <!-- Rerank模型配置 -->
      <div class="model-section" v-if="showRerankConfig">
        <div class="section-title">
          <span>Rerank模型配置</span>
          <el-tag type="warning">Loseless记忆系统必填</el-tag>
        </div>
        <p class="section-desc">用于长上下文记忆的精准召回，重排序优化</p>

        <el-form label-width="120px" v-if="config.rerank_model">
          <el-form-item label="服务商">
            <el-select v-model="config.rerank_model.provider" placeholder="请选择服务商" style="width: 100%">
              <el-option v-for="p in providers" :key="p.value" :label="p.label" :value="p.value" />
            </el-select>
          </el-form-item>
          <el-form-item label="API密钥">
            <el-input
              v-model="config.rerank_model.api_key"
              type="password"
              placeholder="请输入API密钥"
              show-password
            />
          </el-form-item>
          <el-form-item label="Endpoint">
            <el-input v-model="config.rerank_model.endpoint" placeholder="API地址" />
          </el-form-item>
          <el-form-item label="模型名称">
            <el-input v-model="config.rerank_model.model_name" placeholder="如: rerank-01" />
          </el-form-item>
          <el-form-item>
            <el-button type="primary" :loading="testingModel === 'rerank'" @click="testConnection('rerank')">
              一键测试连通性
            </el-button>
          </el-form-item>
        </el-form>

        <el-button v-else type="warning" plain @click="initRerankModel">配置Rerank模型</el-button>
      </div>
    </el-card>

    <!-- 基础运行配置 -->
    <el-card class="config-card">
      <template #header>
        <span class="card-title">
          <el-icon><Setting /></el-icon>
          基础运行配置
        </span>
      </template>

      <el-form label-width="140px">
        <el-form-item label="服务端口">
          <el-input-number v-model="config.port" :min="1024" :max="65535" />
          <span style="margin-left: 12px; color: #909399">默认18789</span>
        </el-form-item>
        <el-form-item label="开机自启">
          <el-switch v-model="config.auto_start" />
        </el-form-item>
        <el-form-item label="崩溃自动重启">
          <el-switch v-model="config.auto_restart" />
        </el-form-item>
        <el-form-item label="低资源办公模式">
          <el-switch v-model="config.low_power_mode" />
          <span style="margin-left: 12px; color: #909399">开启后限制CPU/内存占用</span>
        </el-form-item>
      </el-form>
    </el-card>

    <!-- 保存按钮 -->
    <div class="save-actions">
      <el-button type="primary" size="large" :loading="loading" @click="saveConfig">
        保存配置
      </el-button>
    </div>
  </div>
</template>

<style scoped>
.config-center {
  max-width: 900px;
}

.config-card {
  margin-bottom: 24px;
}

.card-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.model-section {
  padding: 16px 0;
}

.section-title {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 12px;
  font-size: 16px;
  font-weight: 600;
  color: #303133;
}

.section-desc {
  font-size: 13px;
  color: #909399;
  margin-bottom: 16px;
}

.save-actions {
  display: flex;
  justify-content: center;
  padding: 24px 0;
}
</style>
