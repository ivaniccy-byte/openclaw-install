import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'

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

export const useConfigStore = defineStore('config', () => {
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

  const loading = ref(false)

  const showEmbeddingConfig = computed(() => {
    return config.value.memory_system === 'lossless-enhanced'
  })

  const showRerankConfig = computed(() => {
    return config.value.memory_system === 'lossless-enhanced'
  })

  const loadConfig = async () => {
    loading.value = true
    try {
      config.value = await invoke<AppConfig>('get_config')
    } catch (e) {
      console.error('加载配置失败:', e)
    } finally {
      loading.value = false
    }
  }

  const saveConfig = async () => {
    loading.value = true
    try {
      await invoke('save_config', { config: config.value })
      localStorage.setItem('openclaw_installed', 'true')
    } catch (e) {
      console.error('保存配置失败:', e)
      throw e
    } finally {
      loading.value = false
    }
  }

  const testModelConnection = async (model: ModelConfig): Promise<boolean> => {
    try {
      return await invoke<boolean>('test_model_connection', { model })
    } catch (e) {
      console.error('测试连接失败:', e)
      throw e
    }
  }

  return {
    config,
    loading,
    showEmbeddingConfig,
    showRerankConfig,
    loadConfig,
    saveConfig,
    testModelConnection,
  }
})
