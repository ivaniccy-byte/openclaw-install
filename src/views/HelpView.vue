<script setup lang="ts">
import { ref } from 'vue'
import { ElCard, ElCollapse, ElCollapseItem, ElLink, ElTag } from 'element-plus'

const faqs = ref([
  {
    title: 'OpenClaw职场版支持哪些操作系统？',
    content:
      '目前支持 Windows 10/11 x64 (推荐22H2及以上版本)，macOS 12+ (Intel/Apple Silicon) 正在适配中。',
  },
  {
    title: '为什么需要AVX2指令集支持？',
    content:
      'OpenClaw依赖的底层运行库需要AVX2指令集支持。经实测，不支持AVX2的设备运行时会出现核心依赖报错，无法正常启动。这是硬件层面的要求，无法通过软件方式解决。',
  },
  {
    title: '安装后如何配置大模型？',
    content:
      '点击左侧「配置中心」，选择您使用的大模型服务商（如火山引擎、腾讯云等），填写API密钥和模型名称即可。支持国内主流大模型服务：火山引擎、腾讯云、阿里百炼、百度千帆、智谱GLM、京东云、Minimax。',
  },
  {
    title: '什么是记忆系统？如何选择？',
    content:
      '记忆系统让OpenClaw能够记住您的使用习惯和对话历史。Lossless Claw Enhanced+LanceDB方案需要Rerank模型，提供精准的长上下文记忆召回。如果您不需要高级记忆功能，可以选择「不启用」。',
  },
  {
    title: '低资源办公模式是什么？',
    content:
      '开启后会自动限制OpenClaw的CPU和内存占用上限，避免影响您同时使用Word、Excel等办公软件。推荐在配置较低的电脑（8GB以下内存）上开启。',
  },
  {
    title: '绿色单文件版和安装版的区别？',
    content:
      '安装版需要管理员权限安装到电脑，提供完整的开始菜单和桌面快捷方式；绿色单文件版无需安装，直接双击即可运行，适合公司电脑没有安装权限的用户。',
  },
  {
    title: '如何卸载OpenClaw职场版？',
    content:
      '如果是安装版，可以在Windows「设置」→「应用」中找到OpenClaw职场版进行卸载。卸载后会保留配置文件和数据目录，如需完全清除可手动删除安装目录。绿色单文件版直接删除文件即可。',
  },
  {
    title: '连接大模型API失败怎么办？',
    content:
      '请检查：1) API密钥是否正确；2) 账户余额是否充足；3) 网络是否能访问大模型服务商；4) 确认Endpoint地址是否正确。如果仍有问题，可以尝试在配置中心点击「一键测试连通性」进行诊断。',
  },
])

const steps = [
  {
    title: '第一步：配置大模型API',
    desc: '打开「配置中心」，选择您的大模型服务商，一键申请API密钥并填写配置',
    icon: 'Setting',
  },
  {
    title: '第二步：一键启动服务',
    desc: '回到「控制面板」，点击「一键启动OpenClaw」，等待服务启动完成',
    icon: 'VideoPlay',
  },
  {
    title: '第三步：开始使用',
    desc: '点击「打开Web管理面板」，在浏览器中使用OpenClaw的全部AI能力',
    icon: 'Browser',
  },
]

const version = '0.8.3'
const kernelVersion = 'openclaw-standalone latest'
</script>

<template>
  <div class="help-page">
    <div class="page-header">
      <h1 class="page-title">帮助与反馈</h1>
      <p class="page-subtitle">新手入门指南、常见问题解答</p>
    </div>

    <!-- 新手入门 -->
    <el-card class="guide-card">
      <template #header>
        <span class="card-title">
          <el-icon><Reading /></el-icon>
          3步快速上手
        </span>
      </template>
      <div class="steps">
        <div v-for="(step, index) in steps" :key="index" class="step-item">
          <div class="step-number">{{ index + 1 }}</div>
          <div class="step-content">
            <h4>{{ step.title }}</h4>
            <p>{{ step.desc }}</p>
          </div>
        </div>
      </div>
    </el-card>

    <!-- 常见问题 -->
    <el-card class="faq-card">
      <template #header>
        <span class="card-title">
          <el-icon><QuestionFilled /></el-icon>
          常见问题FAQ
        </span>
      </template>
      <el-collapse>
        <el-collapse-item v-for="(faq, index) in faqs" :key="index" :title="faq.title">
          <p>{{ faq.content }}</p>
        </el-collapse-item>
      </el-collapse>
    </el-card>

    <!-- 反馈与支持 -->
    <el-card class="feedback-card">
      <template #header>
        <span class="card-title">
          <el-icon><ChatLineRound /></el-icon>
          反馈与支持
        </span>
      </template>
      <div class="feedback-content">
        <div class="feedback-item">
          <h4>开源项目仓库</h4>
          <p>欢迎提交Issue和Pull Request</p>
          <el-link type="primary" href="https://github.com/openclaw/workplace" target="_blank">
            GitHub仓库
          </el-link>
        </div>
        <div class="feedback-item">
          <h4>版本信息</h4>
          <p>
            当前版本：<el-tag size="small">{{ version }}</el-tag>
          </p>
          <p>
            内核版本：<el-tag size="small" type="info">{{ kernelVersion }}</el-tag>
          </p>
        </div>
      </div>
    </el-card>

    <!-- 版权声明 -->
    <el-card class="copyright-card">
      <template #header>
        <span class="card-title">
          <el-icon><Document /></el-icon>
          开源与版权
        </span>
      </template>
      <div class="copyright-content">
        <p>
          <strong>OpenClaw职场版</strong> 基于
          <el-link type="primary" href="https://github.com/openclaw/standalone" target="_blank">
            openclaw-standalone
          </el-link>
          开发，GUI架构参考
          <el-link type="primary" href="https://github.com/openclaw/launcher" target="_blank">
            OpenClaw-Launcher
          </el-link>
          和
          <el-link type="primary" href="https://github.com/openclaw/sifu" target="_blank">
            OpenClaw-Sifu
          </el-link>
          ，交互设计借鉴
          <el-link type="primary" href="https://github.com/openclaw/squire" target="_blank">
            ClawSquire
          </el-link>
          。
        </p>
        <p style="margin-top: 12px">
          本项目采用 MIT 开源协议，永久免费开源，无广告、无捆绑、不收集用户数据。
        </p>
        <el-alert
          title="免责声明"
          type="warning"
          description="本项目仅为开源工具整合包，不提供任何大模型服务。用户需遵守大模型服务商的使用协议和相关法律法规。"
          :closable="false"
          style="margin-top: 16px"
        />
      </div>
    </el-card>
  </div>
</template>

<style scoped>
.help-page {
  max-width: 900px;
}

.guide-card,
.faq-card,
.feedback-card,
.copyright-card {
  margin-bottom: 24px;
}

.steps {
  display: flex;
  flex-direction: column;
  gap: 20px;
}

.step-item {
  display: flex;
  gap: 16px;
  align-items: flex-start;
}

.step-number {
  width: 40px;
  height: 40px;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 18px;
  font-weight: 600;
  flex-shrink: 0;
}

.step-content h4 {
  color: #303133;
  margin-bottom: 4px;
}

.step-content p {
  color: #909399;
  font-size: 14px;
}

.feedback-content {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: 24px;
}

.feedback-item h4 {
  color: #303133;
  margin-bottom: 8px;
}

.feedback-item p {
  color: #606266;
  font-size: 14px;
  margin-bottom: 8px;
}

.copyright-content p {
  color: #606266;
  font-size: 14px;
  line-height: 1.8;
}
</style>
