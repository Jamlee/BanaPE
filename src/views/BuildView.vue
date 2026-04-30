<template>
  <div class="build-layout">
    <div class="progress-section">
      <div class="progress-header">
        <div class="progress-percent">{{ progressPercent }}%</div>
        <div class="progress-step-info">
          <div class="progress-current-step">{{ currentStepName }}</div>
          <div class="progress-step-detail">{{ currentStepDetail }}</div>
        </div>
      </div>
      <div class="progress-bar-bg">
        <div
          class="progress-bar-fill"
          :class="{ animating: isBuilding }"
          :style="{ width: progressPercent + '%' }"
        ></div>
      </div>

      <div class="step-list">
        <div
          v-for="(step, idx) in buildSteps"
          :key="idx"
          class="step-row"
          :class="getStepClass(idx)"
        >
          <div class="step-status-icon">{{ getStepIcon(idx) }}</div>
          <span>{{ step.name }}</span>
          <span class="step-weight">{{ step.weight }}%</span>
        </div>
      </div>
    </div>

    <div class="build-actions">
      <button class="btn btn-primary" :disabled="isBuilding" @click="startBuild">
        &#9654; 开始构建
      </button>
      <button class="btn btn-danger" :disabled="!isBuilding" @click="cancelBuild">
        &#9632; 取消
      </button>
      <button class="btn btn-secondary" :disabled="isBuilding" @click="cleanup">
        &#128465; 清理
      </button>
      <button class="btn btn-secondary" :disabled="isBuilding" @click="makeISO">
        &#128190; 制作 ISO
      </button>
    </div>

    <div class="log-section">
      <div class="log-header">
        <span class="log-title">&#128220; 构建日志</span>
        <button class="btn btn-browse" @click="clearLog" style="font-size:12px;padding:4px 10px;">清空</button>
      </div>
      <div class="log-body" ref="logBody">
        <div v-for="(line, idx) in logLines" :key="idx" class="log-line">
          <span class="log-time">{{ line.time }}</span>
          <span class="log-msg" :class="line.level">{{ line.message }}</span>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, reactive, inject, nextTick, onMounted, onUnmounted } from 'vue'

const config = inject('config')
const wimMounted = inject('wimMounted')
const logBody = ref(null)

const isBuilding = ref(false)
const progressPercent = ref(0)
const currentStepIdx = ref(-1)
const currentStepName = ref('准备就绪')
const currentStepDetail = ref('点击"开始构建"以开始')
const logLines = reactive([])

const buildSteps = reactive([
  { name: '准备工作目录', weight: 2, status: 'pending' },
  { name: '复制 boot.wim', weight: 3, status: 'pending' },
  { name: '挂载 WIM 映像', weight: 10, status: 'pending' },
  { name: '映射驱动器 (SUBST)', weight: 1, status: 'pending' },
  { name: '加载注册表', weight: 2, status: 'pending' },
  { name: '应用补丁', weight: 60, status: 'pending' },
  { name: '卸载注册表', weight: 2, status: 'pending' },
  { name: '清理日志文件', weight: 1, status: 'pending' },
  { name: '提交 WIM 映像', weight: 10, status: 'pending' },
  { name: '导出优化 WIM', weight: 5, status: 'pending' },
  { name: '创建启动 ISO', weight: 4, status: 'pending' },
])

function getStepClass(idx) {
  const step = buildSteps[idx]
  if (step.status === 'completed') return 'completed'
  if (step.status === 'active') return 'active'
  return 'pending'
}

function getStepIcon(idx) {
  const step = buildSteps[idx]
  if (step.status === 'completed') return '\u2713'
  if (step.status === 'active') return '\u25CF'
  return '\u25CB'
}

function addLog(level, message) {
  const time = new Date().toLocaleTimeString('zh-CN', { hour12: false })
  logLines.push({ time, level, message })
  nextTick(() => {
    if (logBody.value) logBody.value.scrollTop = logBody.value.scrollHeight
  })
}

function clearLog() {
  logLines.length = 0
  addLog('info', '日志已清空')
}

async function startBuild() {
  if (isBuilding.value) return
  isBuilding.value = true
  progressPercent.value = 0
  currentStepIdx.value = -1

  // Reset all steps
  buildSteps.forEach(s => s.status = 'pending')

  addLog('info', '构建开始...')

  if (window.banape) {
    // Use real build engine
    try {
      const result = await window.banape.startBuild(config.value)
      addLog('success', '构建完成!')
    } catch (e) {
      addLog('error', `构建失败: ${e.message}`)
    }
    isBuilding.value = false
  } else {
    // Simulate build for dev mode
    simulateBuild(0)
  }
}

function simulateBuild(stepIdx) {
  if (!isBuilding.value || stepIdx >= buildSteps.length) {
    if (isBuilding.value) {
      progressPercent.value = 100
      currentStepName.value = '构建完成'
      currentStepDetail.value = 'WinPE 已成功构建'
      addLog('success', '构建完成!')
      isBuilding.value = false
      wimMounted.value = false
    }
    return
  }

  const step = buildSteps[stepIdx]
  step.status = 'active'
  currentStepName.value = step.name
  currentStepDetail.value = '正在执行...'
  addLog('info', `[${stepIdx + 1}/${buildSteps.length}] ${step.name}...`)

  if (stepIdx === 2) wimMounted.value = true
  if (stepIdx === 8) wimMounted.value = false

  // Calculate total progress
  let completedWeight = 0
  for (let i = 0; i < stepIdx; i++) completedWeight += buildSteps[i].weight

  const duration = stepIdx === 5 ? 3000 : (stepIdx === 2 ? 2000 : 1000)
  const interval = 100
  let subProgress = 0

  const timer = setInterval(() => {
    if (!isBuilding.value) { clearInterval(timer); return }
    subProgress += interval / duration
    if (subProgress > 1) subProgress = 1
    progressPercent.value = Math.round(completedWeight + step.weight * subProgress)
    if (subProgress >= 1) {
      clearInterval(timer)
      step.status = 'completed'
      addLog('success', `[${stepIdx + 1}/${buildSteps.length}] ${step.name} - 完成`)
      setTimeout(() => simulateBuild(stepIdx + 1), 200)
    }
  }, interval)
}

function cancelBuild() {
  isBuilding.value = false
  addLog('error', '构建已取消')
  currentStepName.value = '已取消'
  currentStepDetail.value = '构建已被用户取消'
}

async function cleanup() {
  addLog('warn', '执行清理...')
  wimMounted.value = false
  if (window.banape) {
    try {
      await window.banape.cleanup(config.value)
      addLog('success', '清理完成')
    } catch (e) {
      addLog('error', `清理失败: ${e.message}`)
    }
  } else {
    setTimeout(() => addLog('success', '清理完成'), 500)
  }
}

async function makeISO() {
  addLog('info', '开始制作 ISO...')
  if (window.banape) {
    try {
      await window.banape.makeISO(config.value)
      addLog('success', 'ISO 创建成功')
    } catch (e) {
      addLog('error', `ISO 创建失败: ${e.message}`)
    }
  } else {
    setTimeout(() => addLog('success', 'ISO 创建成功: _Factory_/BOOTPE.iso'), 1500)
  }
}

// Listen for build events from Electron
onMounted(() => {
  if (window.banape) {
    window.banape.onProgress((data) => {
      progressPercent.value = data.percent
      currentStepName.value = data.message
      currentStepDetail.value = data.detail || ''
      if (data.stepIndex !== undefined) {
        buildSteps.forEach((s, i) => {
          if (i < data.stepIndex) s.status = 'completed'
          else if (i === data.stepIndex) s.status = 'active'
          else s.status = 'pending'
        })
      }
    })
    window.banape.onLog((data) => {
      addLog(data.level, data.message)
    })
  }
})
</script>
