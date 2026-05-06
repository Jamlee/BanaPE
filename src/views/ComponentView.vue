<template>
  <div class="component-view">
    <div class="card">
      <div class="card-header">
        <div class="card-icon">📦</div>
        <div class="card-title">组件选择</div>
        <div class="card-actions">
          <button class="btn btn-secondary btn-xs" @click="selectAll">全选</button>
          <button class="btn btn-secondary btn-xs" @click="clearAll">全不选</button>
        </div>
      </div>

      <div class="input-group">
        <input 
          type="text" 
          class="input-field" 
          v-model="searchText" 
          placeholder="搜索组件..."
        />
      </div>

      <div class="category-list">
        <div v-for="category in categories" :key="category.name" class="category">
          <div 
            class="category-header" 
            @click="toggleCategory(category.name)"
          >
            <span class="category-icon">{{ category.icon }}</span>
            <span class="category-name">{{ category.name }}</span>
            <span class="category-count">{{ getCategoryCount(category.name) }}/{{ category.items.length }}</span>
            <span class="category-arrow">{{ expandedCategories.includes(category.name) ? '▼' : '▶' }}</span>
          </div>
          
          <div v-show="expandedCategories.includes(category.name)" class="category-content">
            <div 
              v-for="item in getFilteredItems(category.items)" 
              :key="item.id" 
              class="component-item"
              :class="{ selected: enabledPatches.includes(item.id) }"
              @click="toggleComponent(item.id)"
            >
              <label class="component-checkbox">
                <input 
                  type="checkbox" 
                  :checked="enabledPatches.includes(item.id)"
                  @change="toggleComponent(item.id)"
                />
              </label>
              <div class="component-info">
                <div class="component-name">{{ item.name }}</div>
                <div class="component-desc">{{ item.desc }}</div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>

    <div class="card">
      <div class="card-header">
        <div class="card-icon">📋</div>
        <div class="card-title">已选组件 ({{ enabledPatches.length }})</div>
      </div>
      
      <div v-if="enabledPatches.length > 0" class="selected-list">
        <div 
          v-for="patchId in enabledPatches" 
          :key="patchId"
          class="selected-item"
        >
          <span class="selected-name">{{ getPatchName(patchId) }}</span>
          <button class="btn btn-danger btn-xs" @click="removePatch(patchId)">移除</button>
        </div>
      </div>
      <div v-else class="empty-state">
        <span>暂无选择的组件</span>
      </div>
    </div>

    <div class="action-bar">
      <button class="btn btn-secondary" @click="prevStep">← 上一步</button>
      <button class="btn btn-primary btn-large" @click="nextStep">下一步 →</button>
    </div>
  </div>
</template>

<script setup>
import { ref, inject } from 'vue'

const config = inject('config')
const currentTab = inject('currentTab')

const searchText = ref('')
const expandedCategories = ref(['系统工具', '网络工具'])
const enabledPatches = ref(['explorer', 'network', 'notepad'])

const categories = [
  {
    name: '系统工具',
    icon: '⚙️',
    items: [
      { id: 'explorer', name: '文件资源管理器', desc: 'Windows 文件管理器' },
      { id: 'cmd', name: '命令提示符', desc: 'CMD 命令行工具' },
      { id: 'powershell', name: 'PowerShell', desc: 'PowerShell 脚本环境' },
      { id: 'notepad', name: '记事本', desc: '简单文本编辑器' },
      { id: 'regedit', name: '注册表编辑器', desc: '系统注册表管理' },
    ]
  },
  {
    name: '网络工具',
    icon: '🌐',
    items: [
      { id: 'network', name: '网络组件', desc: '网络适配器支持' },
      { id: 'tcpip', name: 'TCP/IP 协议', desc: '网络协议栈' },
      { id: 'wifi', name: '无线网卡', desc: 'WiFi 支持' },
      { id: 'lan', name: '有线网卡', desc: '有线网络支持' },
    ]
  },
  {
    name: '硬件驱动',
    icon: '🔧',
    items: [
      { id: 'storage', name: '存储驱动', desc: '磁盘存储支持' },
      { id: 'usb', name: 'USB 驱动', desc: 'USB 设备支持' },
      { id: 'video', name: '显卡驱动', desc: '显示适配器支持' },
      { id: 'audio', name: '音频驱动', desc: '声音设备支持' },
    ]
  },
  {
    name: '安全工具',
    icon: '🛡️',
    items: [
      { id: 'antivirus', name: '杀毒软件', desc: '病毒防护工具' },
      { id: 'firewall', name: '防火墙', desc: '系统防火墙' },
      { id: 'bitlocker', name: 'BitLocker', desc: '磁盘加密工具' },
    ]
  },
]

const toggleCategory = (name) => {
  const idx = expandedCategories.value.indexOf(name)
  if (idx > -1) {
    expandedCategories.value.splice(idx, 1)
  } else {
    expandedCategories.value.push(name)
  }
}

const getFilteredItems = (items) => {
  if (!searchText.value) return items
  const query = searchText.value.toLowerCase()
  return items.filter(item => 
    item.name.toLowerCase().includes(query) || 
    item.desc.toLowerCase().includes(query)
  )
}

const getCategoryCount = (name) => {
  const category = categories.find(c => c.name === name)
  return category.items.filter(item => enabledPatches.value.includes(item.id)).length
}

const toggleComponent = (id) => {
  const idx = enabledPatches.value.indexOf(id)
  if (idx > -1) {
    enabledPatches.value.splice(idx, 1)
  } else {
    enabledPatches.value.push(id)
  }
  config.value.enabledPatches = [...enabledPatches.value]
}

const getPatchName = (id) => {
  for (const category of categories) {
    const item = category.items.find(i => i.id === id)
    if (item) return item.name
  }
  return id
}

const removePatch = (id) => {
  const idx = enabledPatches.value.indexOf(id)
  if (idx > -1) {
    enabledPatches.value.splice(idx, 1)
    config.value.enabledPatches = [...enabledPatches.value]
  }
}

const selectAll = () => {
  const allIds = []
  categories.forEach(cat => {
    cat.items.forEach(item => allIds.push(item.id))
  })
  enabledPatches.value = allIds
  config.value.enabledPatches = [...enabledPatches.value]
}

const clearAll = () => {
  enabledPatches.value = []
  config.value.enabledPatches = []
}

const prevStep = () => {
  currentTab.value = 0
}

const nextStep = () => {
  currentTab.value = 2
}
</script>

<style scoped>
.card-actions {
  display: flex;
  gap: 6px;
  margin-left: auto;
}

.btn-xs {
  padding: 4px 10px;
  font-size: 12px;
}

.category-list {
  margin-top: 8px;
}

.category {
  border: 1px solid var(--border);
  border-radius: var(--radius-sm);
  margin-bottom: 6px;
  overflow: hidden;
}

.category-header {
  display: flex;
  align-items: center;
  padding: 10px 12px;
  background: var(--bg-hover);
  cursor: pointer;
  gap: 8px;
}

.category-header:hover {
  background: var(--bg-input);
}

.category-icon {
  font-size: 14px;
}

.category-name {
  flex: 1;
  font-size: 13px;
  font-weight: 500;
  color: var(--text-primary);
}

.category-count {
  font-size: 11px;
  color: var(--text-muted);
  background: var(--bg-input);
  padding: 2px 8px;
  border-radius: 10px;
}

.category-arrow {
  font-size: 10px;
  color: var(--text-muted);
}

.category-content {
  padding: 6px;
}

.component-item {
  display: flex;
  align-items: center;
  padding: 8px;
  gap: 10px;
  cursor: pointer;
  border-radius: var(--radius-sm);
  margin-bottom: 3px;
}

.component-item:hover {
  background: var(--bg-hover);
}

.component-item.selected {
  background: rgba(37, 99, 235, 0.15);
  border: 1px solid var(--primary);
}

.component-checkbox input {
  width: 16px;
  height: 16px;
}

.component-info {
  flex: 1;
}

.component-name {
  font-size: 13px;
  color: var(--text-primary);
}

.component-desc {
  font-size: 11px;
  color: var(--text-muted);
  margin-top: 2px;
}

.selected-list {
  max-height: 200px;
  overflow-y: auto;
}

.selected-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 8px 10px;
  background: var(--bg-input);
  border-radius: var(--radius-sm);
  margin-bottom: 4px;
}

.selected-name {
  font-size: 12px;
  color: var(--text-primary);
}

.empty-state {
  text-align: center;
  padding: 30px;
  color: var(--text-muted);
  font-size: 13px;
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
