<template>
  <div class="source-view">
    <!-- 步骤指示器 -->
    <div class="steps-indicator">
      <div class="step active">
        <div class="step-number">1</div>
        <div class="step-label">选择源</div>
      </div>
      <div class="step-line"></div>
      <div class="step">
        <div class="step-number">2</div>
        <div class="step-label">配置组件</div>
      </div>
      <div class="step-line"></div>
      <div class="step">
        <div class="step-number">3</div>
        <div class="step-label">构建</div>
      </div>
    </div>

    <!-- 主操作区 -->
    <div class="main-action">
      <div class="action-card" @click="selectSource">
        <div class="action-icon">📁</div>
        <div class="action-content">
          <div class="action-title">选择 Windows 源</div>
          <div class="action-desc" v-if="!config.sourceFolder">点击选择 ISO 文件夹或挂载的驱动器</div>
          <div class="action-value" v-else>{{ config.sourceFolder }}</div>
        </div>
        <div class="action-arrow">→</div>
      </div>
    </div>

    <!-- WIM 信息 (检测到后显示) -->
    <div class="info-section" v-if="wimInfo">
      <div class="section-header">
        <span class="section-icon">✅</span>
        <span class="section-title">检测到 WIM 映像</span>
      </div>
      
      <div class="wim-details">
        <div class="detail-item">
          <div class="detail-label">版本</div>
          <div class="detail-value">{{ wimInfo.version || '-' }}</div>
        </div>
        <div class="detail-item">
          <div class="detail-label">架构</div>
          <div class="detail-value">{{ wimInfo.arch || '-' }}</div>
        </div>
        <div class="detail-item">
          <div class="detail-label">语言</div>
          <div class="detail-value">{{ wimInfo.lang || '-' }}</div>
        </div>
        <div class="detail-item">
          <div class="detail-label">索引</div>
          <div class="detail-value">
            <select v-model="config.bootIndex" class="index-select">
              <option value="1">1 - Windows PE</option>
              <option value="2">2 - Windows PE (含网络)</option>
            </select>
          </div>
        </div>
      </div>
    </div>

    <!-- 高级选项 (折叠) -->
    <details class="advanced-options">
      <summary>高级选项</summary>
      <div class="option-group">
        <label class="option-label">ADK 路径 (可选)</label>
        <div class="input-with-btn">
          <input 
            type="text" 
            v-model="config.adkPath" 
            placeholder="留空将使用内置工具"
            class="input-field"
          >
          <button class="btn-icon" @click="browseADK" title="浏览">📂</button>
        </div>
        <div class="option-hint">如未安装 ADK，将使用 wimlib-imagex</div>
      </div>
    </details>

    <!-- 状态提示 -->
    <div class="status-bar" v-if="config.sourceFolder">
      <div class="status-ready" v-if="wimInfo">
        <span class="status-icon">✓</span>
        <span>已就绪，点击下一步配置组件</span>
      </div>
      <div class="status-loading" v-else>
        <span class="loading-spinner"></span>
        <span>正在检测 WIM 文件...</span>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, computed, inject, watch } from 'vue'

const config = inject('config')
const wimInfo = ref(null)
const detecting = ref(false)

// 监听源文件夹变化
watch(() => config.sourceFolder, (newVal) => {
  if (newVal) {
    detectWim()
  } else {
    wimInfo.value = null
  }
})

async function selectSource() {
  if (window.__TAURI__) {
    const result = await window.__TAURI__.dialog.open({
      directory: true,
      title: '选择 Windows ISO 文件夹或驱动器'
    })
    if (result) {
      config.sourceFolder = result
    }
  }
}

async function detectWim() {
  if (!config.sourceFolder) return
  
  detecting.value = true
  try {
    if (window.__TAURI__) {
      const info = await window.__TAURI__.invoke('wim_get_info', {
        wimPath: `${config.sourceFolder}\\sources\\boot.wim`
      })
      wimInfo.value = info
    }
  } catch (e) {
    console.error('检测 WIM 失败:', e)
  } finally {
    detecting.value = false
  }
}

async function browseADK() {
  if (window.__TAURI__) {
    const result = await window.__TAURI__.dialog.open({
      directory: true,
      title: '选择 ADK 路径'
    })
    if (result) {
      config.adkPath = result
    }
  }
}
</script>

<style scoped>
.source-view {
  max-width: 800px;
  margin: 0 auto;
  padding: 20px;
}

/* 步骤指示器 */
.steps-indicator {
  display: flex;
  align-items: center;
  justify-content: center;
  margin-bottom: 40px;
  padding: 20px;
}

.step {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 8px;
}

.step-number {
  width: 40px;
  height: 40px;
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  background: var(--bg-secondary);
  color: var(--text-muted);
  font-weight: 600;
  font-size: 16px;
  transition: all 0.3s;
}

.step.active .step-number {
  background: var(--primary-color);
  color: white;
}

.step-label {
  font-size: 13px;
  color: var(--text-muted);
}

.step.active .step-label {
  color: var(--text-primary);
  font-weight: 500;
}

.step-line {
  width: 60px;
  height: 2px;
  background: var(--border-color);
  margin: 0 10px;
  margin-bottom: 24px;
}

/* 主操作区 */
.main-action {
  margin-bottom: 24px;
}

.action-card {
  display: flex;
  align-items: center;
  gap: 16px;
  padding: 24px;
  background: var(--bg-secondary);
  border: 2px solid var(--border-color);
  border-radius: 12px;
  cursor: pointer;
  transition: all 0.2s;
}

.action-card:hover {
  border-color: var(--primary-color);
  background: var(--bg-hover);
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
}

.action-icon {
  font-size: 32px;
  flex-shrink: 0;
}

.action-content {
  flex: 1;
  min-width: 0;
}

.action-title {
  font-size: 18px;
  font-weight: 600;
  color: var(--text-primary);
  margin-bottom: 4px;
}

.action-desc {
  font-size: 14px;
  color: var(--text-muted);
}

.action-value {
  font-size: 14px;
  color: var(--text-primary);
  font-family: 'Courier New', monospace;
  word-break: break-all;
}

.action-arrow {
  font-size: 24px;
  color: var(--text-muted);
  flex-shrink: 0;
}

/* 信息区域 */
.info-section {
  background: var(--bg-secondary);
  border-radius: 12px;
  padding: 20px;
  margin-bottom: 20px;
  border-left: 4px solid var(--success-color);
}

.section-header {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-bottom: 16px;
  font-size: 16px;
  font-weight: 600;
  color: var(--text-primary);
}

.section-icon {
  font-size: 18px;
}

.wim-details {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(150px, 1fr));
  gap: 16px;
}

.detail-item {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.detail-label {
  font-size: 12px;
  color: var(--text-muted);
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.detail-value {
  font-size: 15px;
  color: var(--text-primary);
  font-weight: 500;
}

.index-select {
  width: 100%;
  padding: 8px 12px;
  background: var(--bg-primary);
  border: 1px solid var(--border-color);
  border-radius: 6px;
  color: var(--text-primary);
  font-size: 14px;
  cursor: pointer;
}

.index-select:focus {
  outline: none;
  border-color: var(--primary-color);
}

/* 高级选项 */
.advanced-options {
  background: var(--bg-secondary);
  border-radius: 12px;
  padding: 16px;
  margin-bottom: 20px;
}

.advanced-options summary {
  cursor: pointer;
  font-size: 15px;
  font-weight: 500;
  color: var(--text-primary);
  user-select: none;
}

.advanced-options[open] summary {
  margin-bottom: 16px;
  padding-bottom: 16px;
  border-bottom: 1px solid var(--border-color);
}

.option-group {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.option-label {
  font-size: 14px;
  color: var(--text-primary);
  font-weight: 500;
}

.input-with-btn {
  display: flex;
  gap: 8px;
}

.input-field {
  flex: 1;
  padding: 10px 12px;
  background: var(--bg-primary);
  border: 1px solid var(--border-color);
  border-radius: 6px;
  color: var(--text-primary);
  font-size: 14px;
}

.input-field:focus {
  outline: none;
  border-color: var(--primary-color);
}

.btn-icon {
  padding: 10px 16px;
  background: var(--bg-hover);
  border: 1px solid var(--border-color);
  border-radius: 6px;
  cursor: pointer;
  font-size: 16px;
  transition: all 0.2s;
}

.btn-icon:hover {
  background: var(--primary-color);
  border-color: var(--primary-color);
}

.option-hint {
  font-size: 12px;
  color: var(--text-muted);
  margin-top: 4px;
}

/* 状态栏 */
.status-bar {
  padding: 16px;
  background: var(--bg-secondary);
  border-radius: 8px;
  margin-top: 20px;
}

.status-ready {
  display: flex;
  align-items: center;
  gap: 8px;
  color: var(--success-color);
  font-size: 14px;
  font-weight: 500;
}

.status-icon {
  font-size: 16px;
}

.status-loading {
  display: flex;
  align-items: center;
  gap: 12px;
  color: var(--text-muted);
  font-size: 14px;
}

.loading-spinner {
  width: 16px;
  height: 16px;
  border: 2px solid var(--border-color);
  border-top-color: var(--primary-color);
  border-radius: 50%;
  animation: spin 0.8s linear infinite;
}

@keyframes spin {
  to { transform: rotate(360deg); }
}
</style>
