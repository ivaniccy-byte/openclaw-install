import { createRouter, createWebHistory } from 'vue-router'
import DashboardView from '@/views/DashboardView.vue'
import ConfigCenterView from '@/views/ConfigCenterView.vue'
import FunctionManageView from '@/views/FunctionManageView.vue'
import HealthCenterView from '@/views/HealthCenterView.vue'
import HelpView from '@/views/HelpView.vue'
import InstallerView from '@/views/InstallerView.vue'

const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: '/',
      name: 'dashboard',
      component: DashboardView,
    },
    {
      path: '/config',
      name: 'config',
      component: ConfigCenterView,
    },
    {
      path: '/functions',
      name: 'functions',
      component: FunctionManageView,
    },
    {
      path: '/health',
      name: 'health',
      component: HealthCenterView,
    },
    {
      path: '/help',
      name: 'help',
      component: HelpView,
    },
    {
      path: '/installer',
      name: 'installer',
      component: InstallerView,
    },
  ],
})

export default router
