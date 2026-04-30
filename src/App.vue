<template>
  <div class="app-layout">
    <!-- Sidebar -->
    <aside class="sidebar">
      <div class="sidebar-header">
        <div class="sidebar-logo">B</div>
        <div>
          <div class="sidebar-title">BanaPE</div>
          <div class="sidebar-subtitle">WinPE Builder v1.0</div>
        </div>
      </div>

      <nav class="step-nav">
        <div
          v-for="(step, idx) in steps"
          :key="idx"
          class="step-item"
          :class="{ active: currentStep === idx + 1, completed: currentStep > idx + 1 }"
          @click="currentStep = idx + 1"
        >
          <div class="step-number">{{ idx + 1 }}</div>
          <div class="step-label">{{ step }}</div>
        </div>
      </nav>

      <div class="wim-status">
        <div class="wim-status-title">WIM 挂载状态</div>
        <div class="wim-status-indicator">
          <div class="status-dot" :class="{ mounted: wimMounted, error: wimError }"></div>
          <span>{{ wimMounted ? 'boot.wim 已挂载' : wimError ? '挂载异常' : '未挂载' }}</span>
        </div>
      </div>
    </aside>

    <!-- Main -->
    <div class="main-content">
      <div class="topbar">
        <div class="topbar-title">{{ steps[currentStep - 1] }}</div>
        <div class="topbar-actions">
          <button class="btn btn-secondary" @click="toggleTheme">
            {{ isDark ? '浅色' : '深色' }}
          </button>
        </div>
      </div>

      <div class="content-area">
        <SourceView v-if="currentStep === 1" />
        <ComponentView v-if="currentStep === 2" />
        <BuildView v-if="currentStep === 3" />
      </div>

      <div class="nav-footer">
        <button class="btn btn-secondary" :disabled="currentStep === 1" @click="currentStep--">
          ◀ 上一步
        </button>
        <button class="btn btn-primary" v-if="currentStep < 3" @click="currentStep++">
          下一步 ▶
        </button>
        <button class="btn btn-primary" v-if="currentStep === 2" @click="currentStep = 3">
          构建 ▶
        </button>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, provide } from 'vue'
import SourceView from './views/SourceView.vue'
import ComponentView from './views/ComponentView.vue'
import BuildView from './views/BuildView.vue'

const steps = ['选择 Windows 源文件', '组件配置', '构建']
const currentStep = ref(1)
const wimMounted = ref(false)
const wimError = ref(false)
const isDark = ref(true)

const config = ref({
  srcFolder: '',
  bootIndex: 2,
  adkPath: '',
  components: {},
  options: {},
})

provide('config', config)
provide('wimMounted', wimMounted)
provide('wimError', wimError)

function toggleTheme() {
  isDark.value = !isDark.value
  const root = document.documentElement
  if (isDark.value) {
    root.style.setProperty('--bg-primary', '#1a1a2e')
    root.style.setProperty('--bg-secondary', '#16213e')
    root.style.setProperty('--bg-card', '#1e2a4a')
    root.style.setProperty('--bg-card-hover', '#253356')
    root.style.setProperty('--bg-input', '#0f1729')
    root.style.setProperty('--text-primary', '#e8eaf6')
    root.style.setProperty('--text-secondary', '#9fa8da')
    root.style.setProperty('--text-muted', '#5c6bc0')
    root.style.setProperty('--border', '#2a3a5c')
  } else {
    root.style.setProperty('--bg-primary', '#f5f5f5')
    root.style.setProperty('--bg-secondary', '#ffffff')
    root.style.setProperty('--bg-card', '#ffffff')
    root.style.setProperty('--bg-card-hover', '#f0f0f0')
    root.style.setProperty('--bg-input', '#f0f0f0')
    root.style.setProperty('--text-primary', '#1a1a2e')
    root.style.setProperty('--text-secondary', '#5c6bc0')
    root.style.setProperty('--text-muted', '#9fa8da')
    root.style.setProperty('--border', '#e0e0e0')
  }
}
</script>
