<template>
  <div class="build-view">
    <div class="card progress-card">
      <div class="progress-header">
        <div class="progress-icon">🔨</div>
        <div class="progress-title">构建进度</div>
      </div>
      
      <div class="progress-display">
        <div class="progress-percent">{{ progress }}%</div>
        <div class="progress-bar-bg">
          <div class="progress-bar-fill" :style="{ width: progress + '%' }"></div>
        </div>
        <div class="progress-status">{{ currentStep }}</div>
      </div>
    </div>

    <div class="card steps-card">
      <div class="card-header">
        <div class="card-icon">📝</div>
        <div class="card-title">构建步骤</div>
      </div>
      
      <div class="steps-list">
        <div 
          v-for="(step, idx) in steps" 
          :key="idx" 
          class="step-item"
          :class="{ 
            active: currentStepIndex === idx, 
            completed: idx < currentStepIndex,
            pending: idx > currentStepIndex 
          }"
        >
          <div class="step-number">{{ idx + 1 }}</div>
          <div class="step-content">
            <div class="step-name">{{ step.name }}</div>
            <div class="step-desc">{{ step.desc }}</div>
          </div>
          <div class="step-status">
            <span v-if="idx < currentStepIndex">✓</span>
            <span v-else-if="idx === currentStepIndex" class="spinner">◐</span>
            <span v-else>-</span>
          </div>
        </div>
      </div>
    </div>

    <div class="card log-card">
      <div class="card-header">
        <div class="card-icon">📜</div>
        <div class="card-title">输出日志</div>
        <div class="card-actions">
          <button class="btn btn-secondary btn-xs" @click="clearLog">清空</button>
          <button class="btn btn-secondary btn-xs" @click="copyLog">复制</button>
        </div>
      </div>
      
      <div class="log-content" ref="logContainer">
        <div 
          v-for="(log, idx) in logs" 
          :key="idx" 
          class="log-item"
          :class="log.type"
        >
          <span class="log-time">{{ log.time }}</span>
          <span class="log-text">{{ log.text }}</span>
        </div>
      </div>
    </div>

    <div class="action-bar">
      <button class="btn btn-secondary" @click="prevStep">← 返回</button>
      <button 
        v-if="!isBuilding" 
        class="btn btn-primary btn-large" 
        @click="startBuild"
      >
        开始构建
      </button>
      <button 
        v-else 
        class="btn btn-danger" 
        @click="stopBuild"
      >
        停止构建
      </button>
    </div>
  </div>
</template>

<script setup>
import { ref, inject, nextTick } from 'vue'

const currentTab = inject('currentTab')

const progress = ref(0)
const currentStep = ref('准备开始...')
const currentStepIndex = ref(-1)
const isBuilding = ref(false)
const logs = ref([])

const logContainer = ref(null)

const steps = [
  { name: '挂载 WIM 文件', desc: '正在挂载 Windows 映像文件' },
  { name: '应用组件', desc: '正在安装选定的组件' },
  { name: '配置系统', desc: '正在配置系统设置' },
  { name: '生成 ISO', desc: '正在生成启动镜像' },
  { name: '完成', desc: '构建完成' },
]

const addLog = (text, type = 'info') => {
  const now = new Date()
  const time = `${now.getHours().toString().padStart(2, '0')}:${now.getMinutes().toString().padStart(2, '0')}:${now.getSeconds().toString().padStart(2, '0')}`
  logs.value.push({ time, text, type })
  nextTick(() => {
    if (logContainer.value) {
      logContainer.value.scrollTop = logContainer.value.scrollHeight
    }
  })
}

const startBuild = async () => {
  isBuilding.value = true
  progress.value = 0
  currentStepIndex.value = 0
  logs.value = []
  
  addLog('开始构建 WinPE 镜像...', 'info')
  
  for (let i = 0; i < steps.length && isBuilding.value; i++) {
    currentStepIndex.value = i
    currentStep.value = steps[i].name
    
    addLog(`[步骤 ${i + 1}] ${steps[i].name}`, 'info')
    
    for (let j = 0; j < 20 && isBuilding.value; j++) {
      await new Promise(resolve => setTimeout(resolve, 100))
      progress.value = Math.min(100, ((i * 20) + j + 1))
    }
    
    if (i === steps.length - 1) {
      addLog('构建完成！', 'success')
      currentStep.value = '构建完成'
    }
  }
  
  isBuilding.value = false
}

const stopBuild = () => {
  isBuilding.value = false
  addLog('用户取消构建', 'warning')
  currentStep.value = '已取消'
}

const clearLog = () => {
  logs.value = []
}

const copyLog = async () => {
  const text = logs.value.map(l => `${l.time} ${l.text}`).join('\n')
  await window.__TAURI__.clipboard.writeText(text)
  addLog('日志已复制到剪贴板', 'success')
}

const prevStep = () => {
  currentTab.value = 1
}
</script>

<style scoped>
.progress-card {
  text-align: center;
}

.progress-header {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
  margin-bottom: 20px;
}

.progress-icon {
  font-size: 20px;
}

.progress-title {
  font-size: 14px;
  font-weight: 600;
  color: var(--text-primary);
}

.progress-display {
  padding: 20px;
}

.progress-percent {
  font-size: 48px;
  font-weight: bold;
  color: var(--primary-light);
  margin-bottom: 16px;
}

.progress-status {
  font-size: 13px;
  color: var(--text-secondary);
  margin-top: 10px;
}

.steps-list {
  margin-top: 8px;
}

.step-item {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 10px 12px;
  border-radius: var(--radius-sm);
  margin-bottom: 4px;
  transition: all var(--transition-fast);
}

.step-item.completed {
  background: rgba(34, 197, 94, 0.1);
}

.step-item.active {
  background: rgba(37, 99, 235, 0.15);
  border: 1px solid var(--primary);
}

.step-item.pending {
  opacity: 0.5;
}

.step-number {
  width: 24px;
  height: 24px;
  border-radius: 50%;
  background: var(--bg-hover);
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 12px;
  font-weight: bold;
  color: var(--text-secondary);
}

.step-item.completed .step-number {
  background: var(--success);
  color: white;
}

.step-item.active .step-number {
  background: var(--primary);
  color: white;
}

.step-content {
  flex: 1;
}

.step-name {
  font-size: 13px;
  font-weight: 500;
  color: var(--text-primary);
}

.step-desc {
  font-size: 11px;
  color: var(--text-muted);
  margin-top: 2px;
}

.step-status {
  font-size: 14px;
  color: var(--text-muted);
}

.step-item.completed .step-status {
  color: var(--success);
}

.step-item.active .step-status {
  color: var(--primary);
}

.spinner {
  animation: spin 1s linear infinite;
}

@keyframes spin {
  from { transform: rotate(0deg); }
  to { transform: rotate(360deg); }
}

.log-content {
  max-height: 250px;
  overflow-y: auto;
  background: var(--bg-input);
  border-radius: var(--radius-sm);
  padding: 10px;
  font-family: 'Consolas', 'Monaco', monospace;
  font-size: 12px;
}

.log-item {
  padding: 4px 0;
  border-bottom: 1px solid rgba(255,255,255,0.05);
}

.log-item:last-child {
  border-bottom: none;
}

.log-time {
  color: var(--text-muted);
  margin-right: 10px;
}

.log-text {
  color: var(--text-primary);
}

.log-item.success .log-text {
  color: var(--success);
}

.log-item.warning .log-text {
  color: var(--warning);
}

.log-item.error .log-text {
  color: var(--error);
}

.action-bar {
  display: flex;
  justify-content: flex-end;
  gap: 10px;
  padding-top: 8px;
}

.btn-large {
  padding: 10px 32px;
  font-size: 14px;
}
</style>
