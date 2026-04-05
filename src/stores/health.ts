import { defineStore } from 'pinia'
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'

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

export const useHealthStore = defineStore('health', () => {
  const healthScore = ref<HealthScore>({
    score: 100,
    level: '优秀',
    details: {},
  })

  const alerts = ref<Alert[]>([])
  const loading = ref(false)

  const fetchHealthScore = async () => {
    try {
      healthScore.value = await invoke<HealthScore>('get_health_score')
    } catch (e) {
      console.error('获取健康度失败:', e)
    }
  }

  const executeFix = async (fixType: string) => {
    loading.value = true
    try {
      await invoke('auto_fix', { fixType })
      await fetchHealthScore()
    } catch (e) {
      console.error('修复失败:', e)
      throw e
    } finally {
      loading.value = false
    }
  }

  return {
    healthScore,
    alerts,
    loading,
    fetchHealthScore,
    executeFix,
  }
})
