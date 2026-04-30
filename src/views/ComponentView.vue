<template>
  <div class="component-view">
    <!-- 顶部工具栏 -->
    <div class="toolbar">
      <div class="search-box">
        <span class="search-icon">🔍</span>
        <input 
          type="text" 
          v-model="searchQuery" 
          placeholder="搜索补丁..." 
          class="search-input"
        >
      </div>
      <div class="toolbar-actions">
        <button class="btn-text" @click="enableAll">全部启用</button>
        <button class="btn-text" @click="disableAll">全部禁用</button>
      </div>
    </div>

    <!-- 补丁列表 -->
    <div class="patches-container">
      <div v-for="cat in filteredCategories" :key="cat.id" class="category-section">
        <!-- 分类标题 -->
        <div class="category-header" @click="cat.expanded = !cat.expanded">
          <div class="category-title">
            <span class="category-icon">{{ cat.icon }}</span>
            <span class="category-name">{{ cat.name }}</span>
            <span class="category-count">{{ cat.enabledCount }}/{{ cat.items.length }}</span>
          </div>
          <span class="expand-icon" :class="{ expanded: cat.expanded }">▼</span>
        </div>

        <!-- 补丁列表 -->
        <div class="patches-list" v-show="cat.expanded">
          <div 
            v-for="patch in cat.items" 
            :key="patch.id" 
            class="patch-item"
            :class="{ 
              'patch-enabled': patch.enabled,
              'patch-selected': selectedPatch === patch.id 
            }"
            @click="selectPatch(patch)"
          >
            <div class="patch-info">
              <div class="patch-name">{{ patch.name }}</div>
              <div class="patch-desc" v-if="patch.description">{{ patch.description }}</div>
            </div>
            <label class="patch-toggle" @click.stop>
              <input 
                type="checkbox" 
                v-model="patch.enabled" 
                @change="saveConfig"
              >
              <span class="toggle-slider"></span>
            </label>
          </div>
        </div>
      </div>
    </div>

    <!-- 右侧详情面板 -->
    <div class="detail-panel" v-if="selectedPatchData">
      <div class="detail-header">
        <h3>{{ selectedPatchData.name }}</h3>
        <p class="detail-desc">{{ selectedPatchData.description }}</p>
      </div>

      <div class="detail-content">
        <!-- 文件操作 -->
        <div class="detail-section" v-if="selectedPatchData.files?.copy?.length">
          <h4>📁 文件操作 ({{ selectedPatchData.files.copy.length }})</h4>
          <div class="file-list">
            <div v-for="(file, idx) in selectedPatchData.files.copy" :key="idx" class="file-item">
              <span class="file-icon">{{ file.optional ? '📄' : '📎' }}</span>
              <span class="file-path">{{ file.source }}</span>
              <span class="file-badge" v-if="file.optional">可选</span>
            </div>
          </div>
        </div>

        <!-- 注册表操作 -->
        <div class="detail-section" v-if="selectedPatchData.registry?.add?.length">
          <h4>📝 注册表 ({{ selectedPatchData.registry.add.length }})</h4>
          <div class="reg-list">
            <div v-for="(reg, idx) in selectedPatchData.registry.add" :key="idx" class="reg-item">
              <div class="reg-hive">{{ reg.hive }}</div>
              <div class="reg-key">{{ reg.key }}</div>
            </div>
          </div>
        </div>

        <!-- 命令 -->
        <div class="detail-section" v-if="selectedPatchData.commands?.length">
          <h4>⚙️ 命令 ({{ selectedPatchData.commands.length }})</h4>
          <div class="cmd-list">
            <div v-for="(cmd, idx) in selectedPatchData.commands" :key="idx" class="cmd-item">
              <code>{{ cmd.tool }} {{ cmd.args.join(' ') }}</code>
            </div>
          </div>
        </div>

        <!-- 依赖关系 -->
        <div class="detail-section" v-if="selectedPatchData.dependencies?.length">
          <h4>🔗 依赖</h4>
          <div class="dep-list">
            <span v-for="dep in selectedPatchData.dependencies" :key="dep" class="dep-badge">
              {{ dep }}
            </span>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, reactive, computed, onMounted, inject } from 'vue'

const config = inject('config')
const searchQuery = ref('')
const selectedPatch = ref(null)
const patches = ref([])

// 分类配置
const categories = reactive([
  { id: 'core', name: '核心配置', icon: '⚙️', expanded: true },
  { id: 'components', name: '功能组件', icon: '🧩', expanded: true },
  { id: 'network', name: '网络支持', icon: '📶', expanded: false },
  { id: 'drivers', name: '驱动支持', icon: '💻', expanded: false },
  { id: 'runtime-kits', name: '运行时包', icon: '📦', expanded: false },
])

// 按分类组织补丁
const categorizedPatches = computed(() => {
  const result = {}
  categories.forEach(cat => {
    result[cat.id] = []
  })
  
  patches.value.forEach(patch => {
    if (result[patch.patch.category]) {
      result[patch.patch.category].push(patch)
    }
  })
  
  // 按 order 排序
  Object.keys(result).forEach(key => {
    result[key].sort((a, b) => a.patch.order - b.patch.order)
  })
  
  return result
})

// 带统计信息的分类
const categoriesWithStats = computed(() => {
  return categories.map(cat => {
    const items = categorizedPatches.value[cat.id] || []
    const enabledCount = items.filter(p => p.enabled).length
    return {
      ...cat,
      items,
      enabledCount
    }
  })
})

// 搜索过滤
const filteredCategories = computed(() => {
  if (!searchQuery.value) {
    return categoriesWithStats.value
  }
  
  const query = searchQuery.value.toLowerCase()
  return categoriesWithStats.value.map(cat => ({
    ...cat,
    items: cat.items.filter(patch => 
      patch.patch.name.toLowerCase().includes(query) ||
      patch.patch.description.toLowerCase().includes(query) ||
      patch.patch.id.toLowerCase().includes(query)
    )
  })).filter(cat => cat.items.length > 0)
})

// 选中的补丁详情
const selectedPatchData = computed(() => {
  if (!selectedPatch.value) return null
  return patches.value.find(p => p.patch.id === selectedPatch.value)?.patch
})

// 从后端加载补丁
async function loadPatches() {
  try {
    if (window.__TAURI__) {
      const patchList = await window.__TAURI__.invoke('patches_get_list')
      patches.value = patchList.map(p => ({
        ...p,
        enabled: true // 默认全部启用
      }))
      
      // 默认选中第一个
      if (patches.value.length > 0) {
        selectedPatch.value = patches.value[0].patch.id
      }
    }
  } catch (e) {
    console.error('加载补丁列表失败:', e)
  }
}

// 选择补丁
function selectPatch(patch) {
  selectedPatch.value = patch.patch.id
}

// 全部启用
function enableAll() {
  patches.value.forEach(p => p.enabled = true)
  saveConfig()
}

// 全部禁用
function disableAll() {
  patches.value.forEach(p => p.enabled = false)
  saveConfig()
}

// 保存配置
function saveConfig() {
  const enabledPatches = patches.value
    .filter(p => p.enabled)
    .map(p => p.patch.id)
  
  config.value.enabledPatches = enabledPatches
}

onMounted(() => {
  loadPatches()
})
</script>

<style scoped>
.component-view {
  display: grid;
  grid-template-columns: 1fr 400px;
  gap: 20px;
  height: calc(100vh - 140px);
}

/* 工具栏 */
.toolbar {
  grid-column: 1 / -1;
  display: flex;
  gap: 16px;
  align-items: center;
  padding: 16px;
  background: var(--bg-secondary);
  border-radius: 12px;
}

.search-box {
  flex: 1;
  position: relative;
  display: flex;
  align-items: center;
}

.search-icon {
  position: absolute;
  left: 12px;
  font-size: 16px;
  pointer-events: none;
}

.search-input {
  width: 100%;
  padding: 10px 12px 10px 40px;
  background: var(--bg-primary);
  border: 1px solid var(--border-color);
  border-radius: 8px;
  color: var(--text-primary);
  font-size: 14px;
}

.search-input:focus {
  outline: none;
  border-color: var(--primary-color);
}

.toolbar-actions {
  display: flex;
  gap: 8px;
}

.btn-text {
  padding: 8px 16px;
  background: transparent;
  border: 1px solid var(--border-color);
  border-radius: 6px;
  color: var(--text-primary);
  font-size: 13px;
  cursor: pointer;
  transition: all 0.2s;
}

.btn-text:hover {
  background: var(--primary-color);
  border-color: var(--primary-color);
  color: white;
}

/* 补丁列表 */
.patches-container {
  overflow-y: auto;
  padding-right: 8px;
}

.category-section {
  margin-bottom: 16px;
  background: var(--bg-secondary);
  border-radius: 12px;
  overflow: hidden;
}

.category-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 16px;
  cursor: pointer;
  user-select: none;
  transition: background 0.2s;
}

.category-header:hover {
  background: var(--bg-hover);
}

.category-title {
  display: flex;
  align-items: center;
  gap: 10px;
  font-size: 15px;
  font-weight: 600;
  color: var(--text-primary);
}

.category-icon {
  font-size: 18px;
}

.category-count {
  font-size: 12px;
  color: var(--text-muted);
  font-weight: 400;
}

.expand-icon {
  font-size: 12px;
  color: var(--text-muted);
  transition: transform 0.2s;
}

.expand-icon.expanded {
  transform: rotate(180deg);
}

.patches-list {
  border-top: 1px solid var(--border-color);
}

.patch-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 14px 16px;
  border-bottom: 1px solid var(--border-color);
  cursor: pointer;
  transition: all 0.2s;
}

.patch-item:last-child {
  border-bottom: none;
}

.patch-item:hover {
  background: var(--bg-hover);
}

.patch-item.patch-selected {
  background: var(--primary-color);
  color: white;
}

.patch-item.patch-selected .patch-desc {
  color: rgba(255, 255, 255, 0.8);
}

.patch-info {
  flex: 1;
  min-width: 0;
}

.patch-name {
  font-size: 14px;
  font-weight: 500;
  margin-bottom: 2px;
}

.patch-desc {
  font-size: 12px;
  color: var(--text-muted);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

/* 开关 */
.patch-toggle {
  position: relative;
  width: 44px;
  height: 24px;
  cursor: pointer;
  flex-shrink: 0;
  margin-left: 12px;
}

.patch-toggle input {
  opacity: 0;
  width: 0;
  height: 0;
}

.toggle-slider {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: var(--border-color);
  border-radius: 24px;
  transition: all 0.3s;
}

.toggle-slider:before {
  content: '';
  position: absolute;
  height: 18px;
  width: 18px;
  left: 3px;
  bottom: 3px;
  background: white;
  border-radius: 50%;
  transition: all 0.3s;
}

.patch-toggle input:checked + .toggle-slider {
  background: var(--success-color);
}

.patch-toggle input:checked + .toggle-slider:before {
  transform: translateX(20px);
}

/* 详情面板 */
.detail-panel {
  background: var(--bg-secondary);
  border-radius: 12px;
  overflow-y: auto;
  padding: 20px;
}

.detail-header {
  margin-bottom: 20px;
  padding-bottom: 16px;
  border-bottom: 2px solid var(--border-color);
}

.detail-header h3 {
  font-size: 18px;
  font-weight: 600;
  color: var(--text-primary);
  margin: 0 0 8px 0;
}

.detail-desc {
  font-size: 13px;
  color: var(--text-muted);
  margin: 0;
  line-height: 1.5;
}

.detail-content {
  display: flex;
  flex-direction: column;
  gap: 20px;
}

.detail-section h4 {
  font-size: 14px;
  font-weight: 600;
  color: var(--text-primary);
  margin: 0 0 12px 0;
}

.file-list,
.reg-list,
.cmd-list {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.file-item,
.reg-item,
.cmd-item {
  padding: 10px;
  background: var(--bg-primary);
  border-radius: 6px;
  font-size: 12px;
}

.file-item {
  display: flex;
  align-items: center;
  gap: 8px;
}

.file-icon {
  font-size: 14px;
}

.file-path {
  flex: 1;
  font-family: 'Courier New', monospace;
  color: var(--text-primary);
  word-break: break-all;
}

.file-badge {
  padding: 2px 6px;
  background: var(--warning-color);
  color: white;
  border-radius: 4px;
  font-size: 10px;
  font-weight: 600;
}

.reg-item {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.reg-hive {
  font-weight: 600;
  color: var(--primary-color);
  font-size: 11px;
}

.reg-key {
  font-family: 'Courier New', monospace;
  color: var(--text-primary);
  word-break: break-all;
}

.cmd-item code {
  font-family: 'Courier New', monospace;
  color: var(--text-primary);
  font-size: 11px;
  word-break: break-all;
}

.dep-list {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
}

.dep-badge {
  padding: 4px 10px;
  background: var(--bg-primary);
  border: 1px solid var(--border-color);
  border-radius: 12px;
  font-size: 12px;
  color: var(--text-primary);
}
</style>
