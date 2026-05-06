<template>
  <div class="source-view">
    <div class="card">
      <div class="card-header">
        <div class="card-icon">📁</div>
        <div class="card-title">Windows 源文件</div>
      </div>
      
      <div class="input-group">
        <label class="input-label">源文件夹路径</label>
        <div class="input-with-button">
          <input 
            type="text" 
            class="input-field" 
            v-model="sourceFolder" 
            placeholder="请选择 Windows 安装源文件夹..."
            readonly
          />
          <button class="btn btn-secondary" @click="selectSourceFolder">浏览...</button>
        </div>
      </div>

      <div v-if="wimInfo" class="wim-info">
        <div class="banner banner-info">
          <span>📋</span>
          <span>检测到 WIM 文件：{{ wimInfo.filename }}</span>
        </div>
        <div class="info-grid">
          <div class="info-item">
            <span class="info-label">版本</span>
            <span class="info-value">{{ wimInfo.version }}</span>
          </div>
          <div class="info-item">
            <span class="info-label">大小</span>
            <span class="info-value">{{ wimInfo.size }}</span>
          </div>
          <div class="info-item">
            <span class="info-label">映像数</span>
            <span class="info-value">{{ wimInfo.images }}</span>
          </div>
        </div>
        <div class="input-group">
          <label class="input-label">选择启动映像索引</label>
          <select class="input-field" v-model="bootIndex">
            <option v-for="i in wimInfo.images" :key="i" :value="i">{{ i }} - 映像 {{ i }}</option>
          </select>
        </div>
      </div>

      <div v-else-if="sourceFolder && !wimInfo" class="banner banner-warning">
        <span>⚠️</span>
        <span>未检测到有效的 WIM 文件，请检查源文件夹</span>
      </div>
    </div>

    <div class="card">
      <div class="card-header">
        <div class="card-icon">⚙️</div>
        <div class="card-title">ADK 路径（可选）</div>
      </div>
      <div class="input-group">
        <label class="input-label">Windows ADK 安装路径</label>
        <div class="input-with-button">
          <input 
            type="text" 
            class="input-field" 
            v-model="adkPath" 
            placeholder="留空将自动检测..."
            readonly
          />
          <button class="btn btn-secondary" @click="selectAdkFolder">浏览...</button>
        </div>
      </div>
    </div>

    <div class="card">
      <div class="card-header">
        <div class="card-icon">🔧</div>
        <div class="card-title">高级选项</div>
      </div>
      <div class="option-item">
        <label class="option-label">
          <input type="checkbox" v-model="options.compress" />
          <span>启用压缩</span>
        </label>
        <span class="option-desc">减小生成的镜像体积</span>
      </div>
      <div class="option-item">
        <label class="option-label">
          <input type="checkbox" v-model="options.debug" />
          <span>启用调试模式</span>
        </label>
        <span class="option-desc">生成详细日志便于排查问题</span>
      </div>
    </div>

    <div class="action-bar">
      <button 
        class="btn btn-primary btn-large" 
        :disabled="!sourceFolder || !wimInfo"
        @click="nextStep"
      >
        下一步 →
      </button>
    </div>
  </div>
</template>

<script setup>
import { ref, inject } from 'vue'

const config = inject('config')
const currentTab = inject('currentTab')
const wimMounted = inject('wimMounted')
const wimError = inject('wimError')

const sourceFolder = ref('')
const adkPath = ref('')
const bootIndex = ref(2)
const wimInfo = ref(null)

const options = ref({
  compress: false,
  debug: false,
})

const selectSourceFolder = async () => {
  try {
    const result = await window.__TAURI__.dialog.open({
      directory: true,
      title: '选择 Windows 源文件夹',
    })
    if (result) {
      sourceFolder.value = result
      config.value.sourceFolder = result
      await detectWim()
    }
  } catch (e) {
    console.error(e)
  }
}

const selectAdkFolder = async () => {
  try {
    const result = await window.__TAURI__.dialog.open({
      directory: true,
      title: '选择 ADK 安装路径',
    })
    if (result) {
      adkPath.value = result
      config.value.adkPath = result
    }
  } catch (e) {
    console.error(e)
  }
}

const detectWim = async () => {
  try {
    wimInfo.value = {
      filename: 'install.wim',
      version: 'Windows 10/11',
      size: '4.2 GB',
      images: 4,
    }
    wimMounted.value = true
    wimError.value = false
  } catch (e) {
    wimInfo.value = null
    wimMounted.value = false
    wimError.value = true
  }
}

const nextStep = () => {
  config.value.bootIndex = bootIndex.value
  currentTab.value = 1
}
</script>

<style scoped>
.input-with-button {
  display: flex;
  gap: 8px;
}

.input-with-button .input-field {
  flex: 1;
}

.wim-info {
  margin-top: 12px;
}

.info-grid {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 12px;
  margin: 12px 0;
}

.info-item {
  background: var(--bg-input);
  padding: 10px;
  border-radius: var(--radius-sm);
}

.info-label {
  display: block;
  font-size: 11px;
  color: var(--text-muted);
  margin-bottom: 4px;
}

.info-value {
  font-size: 13px;
  color: var(--text-primary);
  font-weight: 500;
}

.option-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 10px 0;
  border-bottom: 1px solid var(--border);
}

.option-item:last-child {
  border-bottom: none;
}

.option-label {
  display: flex;
  align-items: center;
  gap: 8px;
  cursor: pointer;
  color: var(--text-primary);
  font-size: 13px;
}

.option-desc {
  font-size: 12px;
  color: var(--text-muted);
}

.action-bar {
  display: flex;
  justify-content: flex-end;
  padding-top: 8px;
}

.btn-large {
  padding: 10px 32px;
  font-size: 14px;
}
</style>
