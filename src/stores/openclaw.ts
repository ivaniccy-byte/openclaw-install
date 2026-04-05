import { defineStore } from 'pinia'
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'

interface Status {
  running: boolean
  port: number
  uptime_seconds: number
  memory_mb: number
  cpu_percent: number
}

export const useOpenClawStore = defineStore('openclaw', () => {
  const status = ref<Status>({
    running: false,
    port: 18789,
    uptime_seconds: 0,
    memory_mb: 0,
    cpu_percent: 0,
  })

  const loading = ref(false)

  const fetchStatus = async () => {
    try {
      status.value = await invoke<Status>('get_openclaw_status')
    } catch (e) {
      console.error('获取状态失败:', e)
    }
  }

  const start = async (installPath: string, port: number) => {
    loading.value = true
    try {
      await invoke('start_openclaw', { installPath, port })
      await fetchStatus()
    } catch (e) {
      console.error('启动失败:', e)
      throw e
    } finally {
      loading.value = false
    }
  }

  const stop = async () => {
    loading.value = true
    try {
      await invoke('stop_openclaw')
      await fetchStatus()
    } catch (e) {
      console.error('停止失败:', e)
      throw e
    } finally {
      loading.value = false
    }
  }

  return {
    status,
    loading,
    fetchStatus,
    start,
    stop,
  }
})
