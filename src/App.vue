<template>
  <div class="app-container">
    <!-- 顶部工具栏 -->
    <header class="toolbar">
      <div class="toolbar-brand">
        <div class="brand-logo">B</div>
        <div class="brand-text">
          <span class="brand-name">BanaPE</span>
          <span class="brand-subtitle">WinPE Builder</span>
        </div>
      </div>

      <!-- 工作区状态面板 -->
      <div class="workspace-status">
        <div class="status-item">
          <div class="status-indicator-dot" :class="{ ready: wimMounted, error: wimError, building: isBuilding }"></div>
          <div>
            <div class="status-label">{{ $t('status.status') }}</div>
            <div class="status-value" :class="{ highlight: isBuilding }">{{ $t(`status.${isBuilding ? 'building' : wimError ? 'error' : wimMounted ? 'ready' : 'idle'}`) }}</div>
          </div>
        </div>
        <div class="status-item">
          <div>
            <div class="status-label">{{ $t('status.source') }}</div>
            <div class="status-value highlight">{{ sourceVersion || $t('status.notSet') }}</div>
          </div>
        </div>
        <div class="status-item">
          <div>
            <div class="status-label">{{ $t('status.drive') }}</div>
            <div class="status-value">{{ targetDrive || 'X:' }}</div>
          </div>
        </div>
        <div class="status-item">
          <div>
            <div class="status-label">{{ $t('status.type') }}</div>
            <div class="status-value">{{ buildType || 'WinPE' }}</div>
          </div>
        </div>
        <div class="status-item">
          <div>
            <div class="status-label">{{ $t('status.components') }}</div>
            <div class="status-value highlight">{{ enabledCount }}/{{ totalCount }}</div>
          </div>
        </div>
      </div>

      <div class="toolbar-actions">
        <button class="btn-tool" @click="startBuild" :disabled="!canBuild">
          <span class="btn-tool-icon">▶</span>
          <span class="btn-tool-label">{{ $t('toolbar.build') }}</span>
        </button>
        <button class="btn-tool" @click="refreshSource">
          <span class="btn-tool-icon">↻</span>
          <span class="btn-tool-label">{{ $t('toolbar.refresh') }}</span>
        </button>
        <button class="btn-tool" @click="currentView = 'settings'">
          <span class="btn-tool-icon">⚙</span>
          <span class="btn-tool-label">{{ $t('toolbar.settings') }}</span>
        </button>
        <button class="btn-tool" @click="currentView = 'logs'">
          <span class="btn-tool-icon">📋</span>
          <span class="btn-tool-label">{{ $t('toolbar.logs') }}</span>
        </button>
        
        <!-- 语言切换 -->
        <div style="display:flex;align-items:center;margin-left:8px;padding:0 6px;background:rgba(0,0,0,0.15);border-radius:4px;">
          <select :value="locale" @change="setLocale($event.target.value)" 
            style="background:transparent;border:none;color:white;font-size:10px;cursor:pointer;outline:none;padding:2px 4px;">
            <option v-for="(name, code) in localeNames" :key="code" :value="code" 
              style="background:#3730a3;color:white;">{{ name }}</option>
          </select>
        </div>

        <button class="btn-tool">
          <span class="btn-tool-icon">?</span>
          <span class="btn-tool-label">{{ $t('toolbar.about') }}</span>
        </button>
      </div>
    </header>

    <!-- 主内容区域 -->
    <div class="main-area">
      <!-- 左侧边栏 - 树形菜单 -->
      <aside class="sidebar">
        <div class="sidebar-header">
          <span class="sidebar-toggle" @click="toggleAllGroups">{{ allExpanded ? '▼' : '▶' }}</span>
          <span class="sidebar-title">{{ $t('sidebar.title') }}</span>
          <span style="margin-left:auto;font-size:10px;color:var(--text-muted);">{{ enabledCount }}/{{ totalCount }}</span>
        </div>

        <div class="sidebar-tree">
          <!-- Configures 组 (00-Configures) -->
          <div class="tree-group">
            <div class="tree-group-header" @click="toggleGroup('configures')">
              <span class="tree-arrow" :class="{ expanded: expandedGroups.includes('configures') }">▶</span>
              <input type="checkbox" class="tree-checkbox" v-model="groupStates.configures" />
              <span>⚙️ {{ catName('configures') }}</span>
            </div>
            <div v-show="expandedGroups.includes('configures')">
              <div v-for="item in getComponentsByCategory('configures')" :key="item.id"
                class="tree-item" :class="{ selected: isSelected(item) }"
                @click="toggleAndSelect(item)">
                <input type="checkbox" class="tree-checkbox" v-model="item.enabled" />
                <span class="tree-icon">{{ item.icon }}</span>
                <span class="tree-label">{{ compName(item.id) }}</span>
              </div>
            </div>
          </div>

          <!-- ADK OCs 组 (01-ADK_OCs) -->
          <div class="tree-group">
            <div class="tree-group-header" @click="toggleGroup('adk_ocs')">
              <span class="tree-arrow" :class="{ expanded: expandedGroups.includes('adk_ocs') }">▶</span>
              <input type="checkbox" class="tree-checkbox" v-model="groupStates.adk_ocs" />
              <span>📦 {{ catName('adk_ocs') }}</span>
            </div>
            <div v-show="expandedGroups.includes('adk_ocs')">
              <div v-for="item in getComponentsByCategory('adk_ocs')" :key="item.id"
                class="tree-item" :class="{ selected: isSelected(item) }"
                @click="toggleAndSelect(item)">
                <input type="checkbox" class="tree-checkbox" v-model="item.enabled" />
                <span class="tree-icon">{{ item.icon }}</span>
                <span class="tree-label">{{ compName(item.id) }}</span>
              </div>
            </div>
          </div>

          <!-- Shell 组 (01-Components/00-Shell) -->
          <div class="tree-group">
            <div class="tree-group-header" @click="toggleGroup('components_shell')">
              <span class="tree-arrow" :class="{ expanded: expandedGroups.includes('components_shell') }">▶</span>
              <input type="checkbox" class="tree-checkbox" v-model="groupStates.components_shell" />
              <span>🖥️ {{ catName('components_shell') }}</span>
            </div>
            <div v-show="expandedGroups.includes('components_shell')">
              <div v-for="item in getComponentsByCategory('components_shell')" :key="item.id"
                class="tree-item" :class="{ selected: isSelected(item) }"
                @click="toggleAndSelect(item)">
                <input type="checkbox" class="tree-checkbox" v-model="item.enabled" />
                <span class="tree-icon">{{ item.icon }}</span>
                <span class="tree-label">{{ compName(item.id) }}</span>
              </div>
            </div>
          </div>

          <!-- Network 组 (01-Components/02-Network) -->
          <div class="tree-group">
            <div class="tree-group-header" @click="toggleGroup('components_network')">
              <span class="tree-arrow" :class="{ expanded: expandedGroups.includes('components_network') }">▶</span>
              <input type="checkbox" class="tree-checkbox" v-model="groupStates.components_network" />
              <span>🌐 {{ catName('components_network') }}</span>
            </div>
            <div v-show="expandedGroups.includes('components_network')">
              <div v-for="item in getComponentsByCategory('components_network')" :key="item.id"
                class="tree-item" :class="{ selected: isSelected(item) }"
                @click="toggleAndSelect(item)">
                <input type="checkbox" class="tree-checkbox" v-model="item.enabled" />
                <span class="tree-icon">{{ item.icon }}</span>
                <span class="tree-label">{{ compName(item.id) }}</span>
              </div>
            </div>
          </div>

          <!-- Audio 组 (01-Components/03-Audio) -->
          <div class="tree-group">
            <div class="tree-group-header" @click="toggleGroup('components_audio')">
              <span class="tree-arrow" :class="{ expanded: expandedGroups.includes('components_audio') }">▶</span>
              <input type="checkbox" class="tree-checkbox" v-model="groupStates.components_audio" />
              <span>🔊 {{ catName('components_audio') }}</span>
            </div>
            <div v-show="expandedGroups.includes('components_audio')">
              <div v-for="item in getComponentsByCategory('components_audio')" :key="item.id"
                class="tree-item" :class="{ selected: isSelected(item) }"
                @click="toggleAndSelect(item)">
                <input type="checkbox" class="tree-checkbox" v-model="item.enabled" />
                <span class="tree-icon">{{ item.icon }}</span>
                <span class="tree-label">{{ compName(item.id) }}</span>
              </div>
            </div>
          </div>

          <!-- System Components 组 (MMC/DWM/IME/BitLocker/UWP/Runtime/Accessories 合并) -->
          <div class="tree-group">
            <div class="tree-group-header" @click="toggleGroup('components_system')">
              <span class="tree-arrow" :class="{ expanded: expandedGroups.includes('components_system') }">▶</span>
              <input type="checkbox" class="tree-checkbox" v-model="groupStates.components_system" />
              <span>⚙️ {{ catName('components_system') }}</span>
            </div>
            <div v-show="expandedGroups.includes('components_system')">
              <div v-for="item in getComponentsByCategory('components_system')" :key="item.id"
                class="tree-item" :class="{ selected: isSelected(item) }"
                @click="toggleAndSelect(item)">
                <input type="checkbox" class="tree-checkbox" v-model="item.enabled" />
                <span class="tree-icon">{{ item.icon }}</span>
                <span class="tree-label">{{ compName(item.id) }}</span>
              </div>
            </div>
          </div>

          <!-- Drivers 组 (03-Drivers) -->
          <div class="tree-group">
            <div class="tree-group-header" @click="toggleGroup('drivers')">
              <span class="tree-arrow" :class="{ expanded: expandedGroups.includes('drivers') }">▶</span>
              <input type="checkbox" class="tree-checkbox" v-model="groupStates.drivers" />
              <span>🔧 {{ catName('drivers') }}</span>
            </div>
            <div v-show="expandedGroups.includes('drivers')">
              <div v-for="item in getComponentsByCategory('drivers')" :key="item.id"
                class="tree-item" :class="{ selected: isSelected(item) }"
                @click="toggleAndSelect(item)">
                <input type="checkbox" class="tree-checkbox" v-model="item.enabled" />
                <span class="tree-icon">{{ item.icon }}</span>
                <span class="tree-label">{{ compName(item.id) }}</span>
              </div>
            </div>
          </div>

          <!-- Applications 组 (02-Apps) -->
          <div class="tree-group">
            <div class="tree-group-header" @click="toggleGroup('apps')">
              <span class="tree-arrow" :class="{ expanded: expandedGroups.includes('apps') }">▶</span>
              <input type="checkbox" class="tree-checkbox" v-model="groupStates.apps" />
              <span>💼 {{ catName('apps') }}</span>
            </div>
            <div v-show="expandedGroups.includes('apps')">
              <div v-for="item in getComponentsByCategory('apps')" :key="item.id"
                class="tree-item" :class="{ selected: isSelected(item) }"
                @click="toggleAndSelect(item)">
                <input type="checkbox" class="tree-checkbox" v-model="item.enabled" />
                <span class="tree-icon">{{ item.icon }}</span>
                <span class="tree-label">{{ compName(item.id) }}</span>
              </div>
            </div>
          </div>

          <!-- PE Material 组 (02-PEMaterial + Media Creation) -->
          <div class="tree-group">
            <div class="tree-group-header" @click="toggleGroup('pematerial')">
              <span class="tree-arrow" :class="{ expanded: expandedGroups.includes('pematerial') }">▶</span>
              <input type="checkbox" class="tree-checkbox" v-model="groupStates.pematerial" />
              <span>📚 {{ catName('pematerial') }}</span>
            </div>
            <div v-show="expandedGroups.includes('pematerial')">
              <div v-for="item in getComponentsByCategory('pematerial')" :key="item.id"
                class="tree-item" :class="{ selected: isSelected(item) }"
                @click="toggleAndSelect(item)">
                <input type="checkbox" class="tree-checkbox" v-model="item.enabled" />
                <span class="tree-icon">{{ item.icon }}</span>
                <span class="tree-label">{{ compName(item.id) }}</span>
              </div>
            </div>
          </div>
        </div>
      </aside>

      <!-- 右侧内容区 -->
      <main class="content-area">
        <div class="content-header">
          <div class="content-title-section">
            <div class="content-logo">
              <svg viewBox="0 0 44 44" fill="none" xmlns="http://www.w3.org/2000/svg">
                <circle cx="22" cy="22" r="20" fill="#4f46e5"/>
                <text x="22" y="28" text-anchor="middle" fill="white" font-size="18" font-weight="900">B</text>
              </svg>
            </div>
            <div class="content-title">
              <span class="content-name">{{ $t('app.name') }} ✓</span>
              <span class="content-status">{{ contentStatusText }}</span>
            </div>
          </div>
          <span class="content-version">v1.0.0</span>
        </div>

        <div class="content-body">
          <!-- 欢迎页面 -->
          <div v-if="currentView === 'welcome'" class="welcome-page">
            <div class="welcome-hero">
              <div class="hero-icon">🛠️</div>
              <h1>{{ $t('pages.welcome.title') }}</h1>
              <p class="hero-desc">{{ $t('pages.welcome.intro') }}</p>
            </div>

            <div class="feature-grid">
              <div class="feature-card">
                <div class="feature-icon">💿</div>
                <h3>{{ $t('pages.welcome.featWinpe') }}</h3>
                <p>{{ $t('pages.welcome.featWinpeDesc') }}</p>
              </div>
              <div class="feature-card">
                <div class="feature-icon">🧩</div>
                <h3>{{ $t('pages.welcome.featComponents') }}</h3>
                <p>{{ $t('pages.welcome.featComponentsDesc') }}</p>
              </div>
              <div class="feature-card">
                <div class="feature-icon">🌐</div>
                <h3>{{ $t('pages.welcome.featMultiLang') }}</h3>
                <p>{{ $t('pages.welcome.featMultiLangDesc') }}</p>
              </div>
              <div class="feature-card">
                <div class="feature-icon">⚡</div>
                <h3>{{ $t('pages.welcome.featFast') }}</h3>
                <p>{{ $t('pages.welcome.featFastDesc') }}</p>
              </div>
            </div>

            <div class="card">
              <div class="card-header"><span class="card-title">{{ $t('pages.welcome.guideTitle') }}</span></div>
              <div class="card-body">
                <div class="guide-steps">
                  <div class="guide-step" v-for="(step, idx) in t.pages.welcome.steps" :key="idx">
                    <div class="guide-step-num">{{ idx + 1 }}</div>
                    <div class="guide-step-text" v-html="step"></div>
                  </div>
                </div>
              </div>
            </div>

            <div class="card support-card">
              <div class="card-body" style="display:flex;align-items:center;gap:16px;">
                <div style="font-size:32px;">📋</div>
                <div>
                  <h3 style="font-size:14px;margin-bottom:4px;">{{ $t('pages.welcome.supportTitle') }}</h3>
                  <p style="color:var(--text-secondary);font-size:12px;line-height:1.5;">{{ $t('pages.welcome.supportDesc') }}</p>
                </div>
              </div>
            </div>
          </div>

          <!-- Source 配置页面 -->
          <div v-if="currentView === 'source'" class="config-page">
            <div class="card">
              <div class="card-header"><span class="card-title">📁 {{ $t('pages.source.title') }}</span></div>
              <div class="card-body">
                <div class="form-group">
                  <label class="form-label">{{ $t('pages.source.pathLabel') }}</label>
                  <div class="input-with-btn">
                    <input type="text" class="form-input" v-model="config.sourceFolder" :placeholder="$t('pages.source.pathPlaceholder')" readonly />
                    <button class="btn btn-secondary" @click="selectSourceFolder">{{ $t('pages.source.browse') }}</button>
                  </div>
                </div>
                <div v-if="wimInfo" class="info-banner banner-info"><span>✓</span><span>{{ $t('pages.source.detected', { filename: wimInfo.filename, version: wimInfo.version, size: wimInfo.size }) }}</span></div>
                <div v-if="wimInfo" class="info-grid">
                  <div class="info-box"><span class="info-box-label">{{ $t('pages.source.version') }}</span><span class="info-box-value">{{ wimInfo.version }}</span></div>
                  <div class="info-box"><span class="info-box-label">{{ $t('pages.source.size') }}</span><span class="info-box-value">{{ wimInfo.size }}</span></div>
                  <div class="info-box"><span class="info-box-label">{{ $t('pages.source.images') }}</span><span class="info-box-value">{{ wimInfo.images }}</span></div>
                </div>
                <div v-if="wimInfo" class="form-group">
                  <label class="form-label">{{ $t('pages.source.bootIndex') }}</label>
                  <select class="form-input" v-model="config.bootIndex">
                    <option v-for="i in wimInfo.images" :key="i" :value="i">{{ $t('pages.source.image') }} {{ i }}</option>
                  </select>
                </div>
              </div>
            </div>
            <div class="card">
              <div class="card-header"><span class="card-title">📦 {{ $t('pages.source.adkTitle') }}</span></div>
              <div class="card-body">
                <div class="form-group">
                  <label class="form-label">{{ $t('pages.source.adkPathLabel') }}</label>
                  <div class="input-with-btn">
                    <input type="text" class="form-input" v-model="config.adkPath" :placeholder="$t('pages.source.adkPlaceholder')" readonly />
                    <button class="btn btn-secondary" @click="selectAdkFolder">{{ $t('pages.source.browse') }}</button>
                  </div>
                </div>
              </div>
            </div>
          </div>

          <!-- 组件详情页面 -->
          <div v-if="currentView === 'component' && selectedComponentData" class="component-detail">
            <div class="comp-detail-header">
              <div class="comp-detail-icon">{{ selectedComponentData.icon }}</div>
              <div class="comp-detail-info">
                <h2 class="comp-detail-name">{{ compName(selectedComponentData.id) }}</h2>
                <span class="comp-detail-cat-badge">{{ catName(selectedComponentData.category) }}</span>
              </div>
              <div class="comp-detail-status" :class="{ on: selectedComponentData.enabled }">
                {{ selectedComponentData.enabled ? $t('pages.component.enabled') : $t('pages.component.disabled') }}
              </div>
            </div>

            <div class="card">
              <div class="card-header"><span class="card-title">📝 {{ $t('pages.component.descTitle') }}</span></div>
              <div class="card-body">
                <p class="comp-desc-text">{{ compDesc(selectedComponentData.id) }}</p>
              </div>
            </div>

            <div class="card">
              <div class="card-header"><span class="card-title">⚙️ {{ $t('pages.component.infoTitle') }}</span></div>
              <div class="card-body">
                <div class="comp-info-table">
                  <div class="comp-info-row">
                    <span class="comp-info-key">{{ $t('pages.component.idLabel') }}</span>
                    <span class="comp-info-val mono">{{ selectedComponentData.id }}</span>
                  </div>
                  <div class="comp-info-row">
                    <span class="comp-info-key">{{ $t('pages.component.categoryLabel') }}</span>
                    <span class="comp-info-val">{{ catName(selectedComponentData.category) }}</span>
                  </div>
                  <div class="comp-info-row">
                    <span class="comp-info-key">{{ $t('pages.component.statusLabel') }}</span>
                    <span class="comp-info-val" :class="{ 'text-success': selectedComponentData.enabled, 'text-muted': !selectedComponentData.enabled }">
                      {{ selectedComponentData.enabled ? $t('pages.component.enabled') : $t('pages.component.disabled') }}
                    </span>
                  </div>
                  <div class="comp-info-row">
                    <span class="comp-info-key">{{ $t('pages.component.wimSource') }}</span>
                    <span class="comp-info-val">{{ getWimSource(selectedComponentData.category) }}</span>
                  </div>
                </div>
              </div>
            </div>

            <div class="card">
              <div class="card-header"><span class="card-title">📦 {{ $t('pages.component.whatItDoes') }}</span></div>
              <div class="card-body">
                <ul class="comp-feature-list">
                  <li v-for="(feat, idx) in getComponentFeatures(selectedComponentData.id)" :key="idx">✦ {{ feat }}</li>
                </ul>
              </div>
            </div>

            <div class="comp-actions">
              <button class="btn btn-primary btn-lg" :class="{ 'btn-danger': selectedComponentData.enabled }" @click="selectedComponentData.enabled = !selectedComponentData.enabled">
                <span>{{ selectedComponentData.enabled ? '✕ ' + $t('pages.component.disableBtn') : '✓ ' + $t('pages.component.enableBtn') }}</span>
              </button>
            </div>
          </div>

          <!-- 构建进度页面 -->
          <div v-if="currentView === 'build' || isBuilding" class="config-page">
            <div class="card progress-card">
              <div class="card-header"><span class="card-title">🔨 {{ $t('pages.build.title') }}</span></div>
              <div class="card-body">
                <div class="progress-container">
                  <div class="progress-percent">{{ buildProgress }}%</div>
                  <div class="progress-bar-wrapper"><div class="progress-bar-fill" :style="{ width: buildProgress + '%' }"></div></div>
                  <div class="progress-status">{{ currentBuildStep }}</div>
                </div>
                <ul class="step-list" style="margin-top:16px;">
                  <li v-for="(step, idx) in buildSteps" :key="idx" class="step-list-item"
                    :class="{ active: buildStepIndex === idx, completed: idx < buildStepIndex, pending: idx > buildStepIndex }">
                    <div class="step-num">{{ idx + 1 }}</div>
                    <div class="step-info"><div class="step-name">{{ stepName(idx) }}</div></div>
                    <div class="step-status-icon">
                      <span v-if="idx < buildStepIndex">✓</span>
                      <span v-else-if="idx === buildStepIndex && isBuilding" class="spinner">◐</span>
                      <span v-else>-</span>
                    </div>
                  </li>
                </ul>
              </div>
            </div>
            <div class="card log-card">
              <div class="card-header">
                <span class="card-title">📜 {{ $t('pages.build.logTitle') }}</span>
                <div style="margin-left:auto;display:flex;gap:4px;">
                  <button class="btn btn-xs btn-secondary" @click="clearLog">{{ $t('pages.build.clear') }}</button>
                  <button class="btn btn-xs btn-secondary" @click="copyLog">{{ $t('pages.build.copy') }}</button>
                </div>
              </div>
              <div class="card-body">
                <div class="log-container" ref="logContainer">
                  <div v-for="(log, idx) in logs" :key="idx" class="log-line" :class="log.type">
                    <span class="log-time">{{ log.time }}</span><span class="log-msg">{{ log.text }}</span>
                  </div>
                  <div v-if="logs.length === 0" style="color:#666;padding:10px;">{{ $t('pages.build.noLog') }}</div>
                </div>
              </div>
            </div>
          </div>

          <!-- Logs 页面 -->
          <div v-if="currentView === 'logs' && !isBuilding" class="config-page">
            <div class="card">
              <div class="card-header">
                <span class="card-title">📜 {{ $t('pages.logs.title') }}</span>
                <div style="margin-left:auto;display:flex;gap:4px;">
                  <button class="btn btn-xs btn-secondary" @click="clearLog">{{ $t('pages.build.clear') }}</button>
                  <button class="btn btn-xs btn-secondary" @click="copyLog">{{ $t('pages.build.copyAll') }}</button>
                </div>
              </div>
              <div class="card-body">
                <div class="log-container" ref="logContainer" style="max-height:400px;">
                  <div v-for="(log, idx) in logs" :key="idx" class="log-line" :class="log.type">
                    <span class="log-time">{{ log.time }}</span><span class="log-msg">{{ log.text }}</span>
                  </div>
                  <div v-if="logs.length === 0" style="color:#666;padding:10px;">{{ $t('pages.logs.noLog') }}</div>
                </div>
              </div>
            </div>
          </div>
        </div>
      </main>
    </div>

    <!-- 底部状态栏 -->
    <footer class="statusbar">
      <div class="statusbar-left">
        <div class="status-indicator">
          <div class="status-dot" :class="{ ready: wimMounted && !isBuilding, error: wimError, building: isBuilding }"></div>
          <span>{{ $t(`status.${isBuilding ? 'building' : wimError ? 'error' : wimMounted ? 'ready' : 'idle'}`) }}</span>
        </div>
        <span>{{ $t('status.selected', { count: enabledCount, total: totalCount }) }}</span>
      </div>
      <div class="statusbar-right">
        <span v-if="isBuilding">{{ $t('status.building') }}... {{ buildProgress }}%</span>
        <span v-else>{{ $t('app.name') }} {{ $t('app.version') }}</span>
      </div>
    </footer>
  </div>
</template>

<script setup>
import { ref, computed, nextTick, reactive, onMounted } from 'vue'
import { useI18n } from './i18n'

const { $t, t, compName, compDesc, catName, stepName, stepDesc, setLocale, locale, localeNames, initLocale } = useI18n()

onMounted(() => {
  initLocale()
})

// 组件数据定义 - 完全对应 WimBuilder2 WIN10XPE 结构
const componentData = reactive([
  // ===== Configures (00-Configures) =====
  { id: 'cfg_build_catalog', name: 'Build Catalog', category: 'configures', icon: '📋', desc: '完整目录文件与 TTS 语音', enabled: false },
  { id: 'cfg_loader_network', name: 'Loader Network', category: 'configures', icon: '🔄', desc: '加载时初始化网络连接', enabled: true },
  { id: 'cfg_system_audio', name: 'Audio Driver Patch', category: 'configures', icon: '🔊', desc: '音频驱动补丁', enabled: true },
  { id: 'cfg_system_computername', name: 'Computer Name', category: 'configures', icon: '💻', desc: '计算机名称设置', enabled: true },
  { id: 'cfg_system_peversion', name: 'PE Version', category: 'configures', icon: '🏷️', desc: 'PE 版本信息配置', enabled: true },
  { id: 'cfg_system_productoptions', name: 'Product Options', category: 'configures', icon: '📝', desc: '产品选项配置 (SafeMode等)', enabled: true },
  { id: 'cfg_system_fbwf', name: 'FBWF (Filter)', category: 'configures', icon: '🛡️', desc: '基于文件的写入过滤器', enabled: false },
  { id: 'cfg_system_textassoc', name: 'Text Association', category: 'configures', icon: '📄', desc: '文本文件关联', enabled: true },
  { id: 'cfg_account_admin', name: 'Admin Account', category: 'configures', icon: '👤', desc: '内置管理员账户配置', enabled: true },
  { id: 'cfg_customization_theme', name: 'Customization Theme', category: 'configures', icon: '🎨', desc: '主题自定义 (透明度等)', enabled: false },

  // ===== ADK OCs (01-ADK_OCs) =====
  { id: 'adk_settings', name: 'ADK Settings', category: 'adk_ocs', icon: '⚙️', desc: 'ADK 基础设置', enabled: true },
  { id: 'adk_fonts', name: 'Fonts', category: 'adk_ocs', icon: '🔤', desc: '字体支持', enabled: true },
  { id: 'adk_network', name: 'Network Components', category: 'adk_ocs', icon: '🌐', desc: 'ADK 网络组件', enabled: true },
  { id: 'adk_powershell', name: 'PowerShell', category: 'adk_ocs', icon: '>', desc: 'PowerShell 环境', enabled: true },
  { id: 'adk_setup', name: 'Setup Components', category: 'adk_ocs', icon: '📦', desc: '安装程序组件', enabled: false },
  { id: 'adk_misc', name: 'Miscellaneous', category: 'adk_ocs', icon: '📌', desc: '其他 ADK 组件', enabled: false },

  // ===== Shell (01-Components/00-Shell) =====
  { id: 'boot2winre', name: 'Boot to WinRE', category: 'components_shell', icon: '🔄', desc: '启动到 WinRE 环境 (含 WiFi)', enabled: false },
  { id: 'shell_explorer', name: 'Explorer', category: 'components_shell', icon: '📁', desc: 'Windows 资源管理器 (主题/设置)', enabled: true },
  { id: 'shell_winxshell', name: 'WinXShell', category: 'components_shell', icon: '🪟', desc: '轻量级 Shell 替代方案', enabled: false },
  { id: 'shell_startmenu_classic', name: 'Classic Start Menu', category: 'components_shell', icon: '📋', desc: '经典开始菜单 (Classic Shell / StartIsBack)', enabled: false },
  { id: 'shell_settings', name: 'Shell Settings', category: 'components_shell', icon: '▬', desc: '任务栏/资源管理器详细设置', enabled: true },

  // ===== Network (01-Components/02-Network) =====
  { id: 'network_full', name: 'Full Network Support', category: 'components_network', icon: '🌐', desc: '完整网络支持 (TCP/IP/DNS/DHCP/WLAN)', enabled: true },
  { id: 'wifi_package', name: 'WinPE WiFi Package', category: 'components_network', icon: '📶', desc: 'WinPE 无线网络包', enabled: true },
  { id: 'rndis', name: 'RNDIS Driver', category: 'components_network', icon: '🔗', desc: 'USB 网络共享驱动 (手机USB共享)', enabled: false },
  { id: 'pppoe', name: 'PPPoE Support', category: 'components_network', icon: '📞', desc: 'PPPoE 拨号连接支持', enabled: false },
  { id: 'patch_drvinst', name: 'DrvInst Patch', category: 'components_network', icon: '🔧', desc: '驱动安装程序补丁', enabled: true },

  // ===== Audio (01-Components/03-Audio) =====
  { id: 'audio_core', name: 'Audio Core', category: 'components_audio', icon: '🔊', desc: '核心音频支持 (WASAPI/MMCSS)', enabled: true },
  { id: 'media_player', name: 'Windows Media Player', category: 'components_audio', icon: '🎵', desc: 'Windows Media Player', enabled: false },

  // ===== System Components (MMC/DWM/IME/BitLocker etc.) =====
  { id: 'bitlocker', name: 'BitLocker', category: 'components_system', icon: '🔐', desc: 'BitLocker 磁盘加密支持', enabled: false },
  { id: 'cred_dialog', name: 'Credential Dialog', category: 'components_system', icon: '🔑', desc: '凭据对话框 (UAC提示)', enabled: false },
  { id: 'dwm', name: 'DWM (Desktop Window Manager)', category: 'components_system', icon: '✨', desc: '桌面窗口管理器 (Aero效果)', enabled: true },
  { id: 'devices_bluetooth', name: 'Bluetooth Support', category: 'components_system', icon: '📶', desc: '蓝牙设备支持', enabled: false },
  { id: 'devices_camera', name: 'Camera Support', category: 'components_system', icon: '📷', desc: '摄像头设备支持', enabled: false },
  { id: 'devices_dsmsvc', name: 'DSM Service', category: 'components_system', icon: '🖨️', desc: '设备设置管理器服务', enabled: false },
  { id: 'devices_printer', name: 'Printer Support', category: 'components_system', icon: '🖨️', desc: '打印机和扫描仪支持', enabled: false },
  { id: 'ime_zhcn', name: 'IME Chinese (Simplified)', category: 'components_system', icon: '中', desc: '简体中文输入法 (微软拼音/五笔等)', enabled: true },
  { id: 'ime_zhtw', name: 'IME Chinese (Traditional)', category: 'components_system', icon: '繁', desc: '繁体中文输入法', enabled: false },
  { id: 'ime_ja', name: 'IME Japanese', category: 'components_system', icon: 'あ', desc: '日语输入法', enabled: false },
  { id: 'ime_ko', name: 'IME Korean', category: 'components_system', icon: '한', desc: '韩语输入法', enabled: false },
  { id: 'ie_browser', name: 'Internet Explorer', category: 'components_system', icon: '🌐', desc: 'IE 浏览器', enabled: false },
  { id: 'mmc', name: 'MMC Snap-ins', category: 'components_system', icon: '⚙️', desc: 'Microsoft 管理控制台 (磁盘管理等)', enabled: true },
  { id: 'msi', name: 'MSI Installer', category: 'components_system', icon: '📦', desc: 'Windows Installer 服务', enabled: true },
  { id: 'mtp_support', name: 'MTP Support', category: 'components_system', icon: '📱', desc: 'MTP 设备支持 (Android手机连接)', enabled: false },
  { id: 'netfx', name: '.NET Framework', category: 'components_system', icon: '🔄', desc: '.NET Framework 3.5 运行时', enabled: false },
  { id: 'remote_desktop', name: 'Remote Desktop', category: 'components_system', icon: '🖥️', desc: '远程桌面 (RDP) 支持', enabled: false },
  { id: 'search', name: 'Windows Search', category: 'components_system', icon: '🔍', desc: 'Windows 索引服务', enabled: false },
  { id: 'syswow64_basic', name: 'SysWOW64 Basic', category: 'components_system', icon: 'x64', desc: 'WoW64 32位兼容层基础支持', enabled: true },
  { id: 'vcruntime', name: 'VC++ Runtime', category: 'components_system', icon: 'C++', desc: 'Visual C++ 运行时库', enabled: true },

  // ===== Accessories (za-Accessories) =====
  { id: 'acc_notepad', name: 'Notepad', category: 'components_system', icon: '📝', desc: '记事本', enabled: true },
  { id: 'acc_mspaint', name: 'Paint', category: 'components_system', icon: '🎨', desc: '画图', enabled: false },
  { id: 'acc_snippingtool', name: 'Snipping Tool', category: 'components_system', icon: '✂️', desc: '截图工具', enabled: false },
  { id: 'acc_winphoto', name: 'Photo Viewer', category: 'components_system', icon: '🖼️', desc: 'Windows 照片查看器', enabled: false },
  { id: 'acc_accessibility', name: 'Accessibility', category: 'components_system', icon: '♿', desc: '辅助功能支持', enabled: false },
  { id: 'acc_taskmgr', name: 'Task Manager', category: 'components_system', icon: '📊', desc: '任务管理器', enabled: true },
  { id: 'acc_regedit', name: 'Registry Editor', category: 'components_system', icon: '🗄️', desc: '注册表编辑器', enabled: true },

  // ===== Runtime Kits (zh-RuntimeKits) =====
  { id: 'rt_appcompat', name: 'App Compatibility', category: 'components_system', icon: '✔', desc: '应用兼容性层', enabled: false },
  { id: 'rt_arm64_support', name: 'ARM64 x86_64 Support', category: 'components_system', icon: '🔄', desc: 'ARM64 架构 x86_64 兼容层', enabled: false },
  { id: 'rt_compatibility', name: 'Compatibility Mode', category: 'components_system', icon: '🔗', desc: '兼容性模式', enabled: false },
  { id: 'rt_directx', name: 'DirectX', category: 'components_system', icon: '🎮', desc: 'DirectX 运行时', enabled: false },
  { id: 'rt_speech_onecore', name: 'Speech OneCore', category: 'components_system', icon: '🗣️', desc: '语音识别运行时', enabled: false },
  { id: 'rt_syswow_sapi', name: 'SysWOW64 SAPI', category: 'components_system', icon: '🔊', desc: 'WoW64 SAPI 支持', enabled: false },

  // ===== UWP/AppX (zp-UWPAppX) =====
  { id: 'uwp_taskbar', name: 'UWP Taskbar', category: 'components_system', icon: '▬', desc: 'UWP 任务栏设置 (隐藏搜索框等)', enabled: false },
  { id: 'uwp_startmenu', name: 'UWP Start Menu', category: 'components_system', icon: '🏠', desc: 'UWP 开始菜单', enabled: false },
  { id: 'uwp_settings', name: 'UWP Settings', category: 'components_system', icon: '⚙️', desc: 'UWP 设置应用', enabled: false },
  { id: 'uwp_ime', name: 'UWP IME', category: 'components_system', icon: '⌨️', desc: 'UWP 输入法面板', enabled: false },
  { id: 'uwp_taskmgr', name: 'UWP Task Manager', category: 'components_system', icon: '📈', desc: 'UWP 任务管理器', enabled: false },
  { id: 'uwp_appsrvs', name: 'UWP App Services', category: 'components_system', icon: '📱', desc: 'UWP 应用服务框架', enabled: false },
  { id: 'uwp_appxapps', name: 'UWP AppX Apps', category: 'components_system', icon: '📲', desc: '预装 UWP 应用', enabled: false },

  // ===== Drivers (03-Drivers) =====
  { id: 'drv_system', name: 'System Drivers', category: 'drivers', icon: '🔧', desc: '系统驱动 (存储/显示/音频等)', enabled: true },

  // ===== Applications (02-Apps) =====
  { id: 'app_7zip', name: '7-Zip', category: 'apps', icon: '📦', desc: '7-Zip 压缩工具', enabled: true },
  { id: 'app_explorerpp', name: 'Explorer++', category: 'apps', icon: '📂', desc: '增强型资源管理器', enabled: false },
  { id: 'app_defraggler', name: 'Defraggler', category: 'apps', icon: '💿', desc: '磁盘碎片整理工具', enabled: false },
  { id: 'app_imdisk', name: 'ImDisk', category: 'apps', icon: '💽', desc: '虚拟磁盘工具', enabled: false },
  { id: 'app_hotswap', name: 'HotSwap!', category: 'apps', icon: '🔥', desc: '硬盘热插拔工具', enabled: false },
  { id: 'app_penetwork', name: 'PENetwork', category: 'apps', icon: '🌍', desc: 'PE 网络管理工具', enabled: true },
  { id: 'app_yong_ime', name: 'Yong IME', category: 'apps', icon: '🀄', desc: '勇哥输入法 (中文)', enabled: false },
  { id: 'app_chrome', name: 'Chrome Browser', category: 'apps', icon: '🌐', desc: 'Google Chrome 浏览器', enabled: false },

  // ===== PE Material (02-PEMaterial) =====
  { id: 'mat_office2007', name: 'Office 2007', category: 'pematerial', icon: '📊', desc: 'Microsoft Office 2007', enabled: false },
  { id: 'mat_potplayer', name: 'PotPlayer', category: 'pematerial', icon: '🎬', desc: 'PotPlayer 视频播放器', enabled: false },
  { id: 'mat_dismpp', name: 'Dism++', category: 'pematerial', icon: '🛠️', desc: 'Dism++ 系统镜像工具', enabled: false },
  { id: 'mat_everything', name: 'Everything', category: 'pematerial', icon: '🔍', desc: 'Everything 文件搜索', enabled: false },
  { id: 'mat_sumatrapdf', name: 'SumatraPDF', category: 'pematerial', icon: '📕', desc: 'SumatraPDF PDF阅读器', enabled: false },
  { id: 'mat_winntsetup', name: 'WinNTSetup', category: 'pematerial', icon: '💾', desc: 'WinNTSetup 系统安装工具', enabled: false },
  { id: 'mat_mpcbe', name: 'MPC-BE', category: 'pematerial', icon: '🎥', desc: 'MPC-BE 媒体播放器', enabled: false },
  { id: 'mat_libreoffice', name: 'LibreOffice', category: 'pematerial', icon: '📝', desc: 'LibreOffice 办公套件', enabled: false },

  // ===== Media Creation =====
  { id: 'build_iso', name: 'Build ISO', category: 'pematerial', icon: '💿', desc: '生成可启动 ISO 镜像', enabled: true },
  { id: 'build_usb', name: 'Create USB', category: 'pematerial', icon: '💾', desc: '制作 USB 启动盘', enabled: false },
])

// 状态变量
const currentView = ref('welcome')
const selectedComponent = ref(null)
const selectedComponentData = computed(() => componentData.find(c => c.id === selectedComponent.value))
const wimMounted = ref(false)
const wimError = ref(false)
const wimInfo = ref(null)
const isBuilding = ref(false)
const logs = ref([])
const logContainer = ref(null)

const buildProgress = ref(0)
const buildStepIndex = ref(-1)
const currentBuildStep = ref($t('contentStatus.configure'))

const expandedGroups = ref(['configures', 'adk_ocs', 'components_shell', 'components_network'])
const groupStates = reactive({
  configures: true, adk_ocs: true, components_shell: true, components_network: true,
  components_audio: false, components_system: true, drivers: true,
  apps: false, pematerial: true
})

const config = reactive({
  sourceFolder: '',
  bootIndex: 2,
  adkPath: '',
})

const targetDrive = ref('X:')
const buildType = ref('WinPE')
const sourceVersion = ref('')

const buildSteps = [
  { name: stepName(0), desc: stepDesc(0) },
  { name: stepName(1), desc: stepDesc(1) },
  { name: stepName(2), desc: stepDesc(2) },
  { name: stepName(3), desc: stepDesc(3) },
  { name: stepName(4), desc: stepDesc(4) },
  { name: stepName(5), desc: stepDesc(5) },
  { name: stepName(6), desc: stepDesc(6) },
  { name: stepName(7), desc: stepDesc(7) },
]

// 计算属性
const allExpanded = computed(() => expandedGroups.value.length >= 9)

const totalCount = computed(() => componentData.length)
const enabledCount = computed(() => componentData.filter(c => c.enabled).length)

const canBuild = computed(() => config.sourceFolder && wimInfo.value)

const statusText = computed(() => {
  if (isBuilding.value) return $t('status.building')
  if (wimError.value) return $t('status.error')
  if (wimMounted.value) return $t('status.ready')
  return $t('status.idle')
})

const contentStatusText = computed(() => {
  if (isBuilding.value) return $t('contentStatus.building')
  if (wimMounted.value) return $t('contentStatus.ready', { count: enabledCount.value })
  return $t('contentStatus.configure')
})

// 方法
function toggleGroup(name) {
  const idx = expandedGroups.value.indexOf(name)
  if (idx > -1) expandedGroups.value.splice(idx, 1)
  else expandedGroups.value.push(name)
}

function toggleAllGroups() {
  if (allExpanded.value) expandedGroups.value = []
  else expandedGroups.value = ['configures','adk_ocs','components_shell','components_network','components_audio','components_system','drivers','apps','pematerial']
}

function getComponentsByCategory(category) {
  return componentData.filter(c => c.category === category)
}

function getWimSource(category) {
  const map = {
    configures: '00-Configures',
    adk_ocs: '01-ADK_OCs',
    components_shell: '01-Components\\00-Shell',
    components_network: '01-Components\\02-Network',
    components_audio: '01-Components\\03-Audio',
    components_system: '01-Components (MMC/DWM/IME/Accessories/Runtime/UWP)',
    drivers: '03-Drivers',
    apps: '02-Apps',
    pematerial: '02-PEMaterial',
  }
  return map[category] || category
}

const componentFeatures = {
  cfg_build_catalog: ['添加完整目录结构文件', '包含 TTS 语音合成引擎支持', '提供 PE 环境下的完整文件浏览体验'],
  cfg_loader_network: ['PE 启动时自动初始化网络栈', '支持 TCP/IP 协议初始化', '为需要网络的组件提供基础'],
  cfg_system_audio: ['修补音频驱动在 WinPE 下的兼容性', '修复 HD Audio 驱动加载问题', '确保 WASAPI/MMCSS 正常工作'],
  cfg_system_computername: ['设置 PE 环境计算机名称', '自定义主机名配置', '网络环境中便于识别'],
  cfg_system_peversion: ['配置 PE 版本信息显示', '设置系统版本号和构建信息', '让 PE 环境看起来更像完整 Windows'],
  cfg_system_productoptions: ['启用安全模式等启动选项', '配置产品选项注册表项', '高级系统行为控制'],
  cfg_system_fbwf: ['基于文件的写入过滤驱动', '保护 PE 运行时文件不被修改', '适用于只读介质运行的 PE'],
  cfg_system_textassoc: ['关联文本文件到默认编辑器', '.txt/.log/.ini 文件双击打开', '提升使用体验'],
  cfg_account_admin: ['启用内置管理员账户', '配置默认管理员权限', '无需 UAC 提示直接操作'],
  cfg_customization_theme: ['自定义窗口主题透明度', 'DWM 桌面效果微调', 'Aero Glass 效果参数调整'],
  adk_settings: ['基础 ADK 组件安装与配置', 'WinPE 核心运行环境依赖', 'PE 启动的必要条件'],
  adk_fonts: ['中/日/韩等多语言字体支持', '包含 Segoe UI、微软雅黑等字体', '确保 UI 正确渲染多语言文本'],
  adk_network: ['ADK 基础网络协议栈', 'TCP/IP、DNS、DHCP 客户端', '网络功能的基础层'],
  adk_powershell: ['PowerShell 5.1/7.x 运行环境', '脚本自动化管理能力', '系统管理和排错利器'],
  adk_setup: ['Windows 安装程序组件', 'DISM 工具链支持', '镜像操作和管理工具'],
  adk_misc: ['其他 ADK 可选功能包', '辅助工具和运行库', '扩展 PE 功能边界'],
  boot2winre: ['启动进入 WinRE 恢复环境', '内置 WiFi 网络连接支持', '基于 WinRE 的轻量 Shell 方案'],
  shell_explorer: ['Windows 资源管理器完整版', '主题支持和外观定制', '文件复制/移动/删除等核心操作'],
  shell_winxshell: ['轻量级 Shell 替代方案', '低资源占用的文件管理器', '适合内存受限的 PE 环境'],
  shell_startmenu_classic: ['经典风格开始菜单', '兼容 Classic Shell / StartIsBack', '传统 Windows 操作习惯'],
  shell_settings: ['任务栏详细配置选项', '资源管理器行为设置', 'Shell 外观深度定制'],
  network_full: ['完整的 TCP/IP 网络协议栈', 'DNS 解析 + DHCP 自动获取 IP', 'WLAN 无线网卡支持'],
  wifi_package: ['WinPE 专用无线网络驱动包', 'WiFi 连接管理界面', '移动热点和网络共享'],
  rndis: ['USB RNDIS 网络驱动', '手机 USB 共享网络接入', 'Android/iPhone USB Tethering 支持'],
  pppoe: ['PPPoE 拨号连接协议支持', '宽带拨号上网能力', 'ADSL/光纤直连场景'],
  patch_drvinst: ['设备驱动安装程序补丁', '修复 PE 下驱动签名强制问题', '允许安装未签名驱动程序'],
  audio_core: ['WASAPI 音频子系统', 'MMCSS 多媒体调度服务', '音频播放和录音基础支持'],
  media_player: ['Windows Media Player', '多媒体文件播放能力', '音频/视频回放功能'],
  bitlocker: ['BitLocker 磁盘加密驱动', '加密分区解锁和访问', '企业安全合规需求'],
  cred_dialog: ['UAC 凭据提示对话框', '管理员权限提升提示', '安全上下文切换界面'],
  dwm: ['桌面窗口管理器 (DWM)', 'Aero 毛玻璃视觉效果', '窗口动画和桌面合成'],
  devices_bluetooth: ['蓝牙协议栈和驱动', '蓝牙外设连接支持', '耳机/键盘/鼠标等设备'],
  devices_camera: ['摄像头设备驱动 (UVC)', '图像采集设备支持', '视频通话和拍照功能'],
  devices_dsmsvc: ['设备设置管理器服务', '硬件设备属性配置', '设备管理增强体验'],
  devices_printer: ['打印机和扫描仪驱动', '本地/网络打印机支持', '文档输出功能'],
  ime_zhcn: ['简体中文输入法引擎', '微软拼音/五笔/郑码等', '中文打字和文字输入'],
  ime_zhtw: ['繁体中文输入法引擎', '注音/速成/仓颉等输入法', '港台地区用户支持'],
  ime_ja: ['日语 IME 输入法', 'MS-IME / ATOK 兼容', '平假名/片假名/汉字转换'],
  ie_browser: ['Internet Explorer 浏览器', 'Web 页面浏览能力', '旧版网站兼容性访问'],
  mmc: ['Microsoft 管理控制台', '磁盘管理/服务管理/事件查看器', '系统管理核心工具集'],
  msi: ['Windows Installer 服务', '.msi 安装包执行能力', '软件安装和卸载支持'],
  mtp_support: ['MTP 媒体传输协议', 'Android 手机文件传输', 'USB 连接手机访问文件'],
  netfx: ['.NET Framework 3.5 运行时', 'C#/VB.NET 应用运行基础', '大量 Windows 工具的依赖'],
  remote_desktop: ['远程桌面 (RDP) 客户端', '远程连接其他计算机', '运维和技术支持场景'],
  search: ['Windows Search 索引服务', '文件内容快速搜索', '开始菜单即时搜索'],
  syswow64_basic: ['WoW64 32 位兼容层', '运行 32 位应用程序的基础', 'x64 PE 上的 32 位软件支持'],
  vcruntime: ['Visual C++ 运行时库 (x86+x64)', 'VC++ 应用程序依赖', '大量第三方工具的前置要求'],
  acc_notepad: ['Windows 记事本', '纯文本编辑器', '配置文件查看和修改'],
  acc_mspaint: ['Windows 画图', '简单图像编辑工具', '截图标注和基本绘图'],
  acc_snippingtool: ['截图工具', '屏幕区域捕获', '快速截图保存分享'],
  acc_winphoto: ['Windows 照片查看器', '图片预览和幻灯播放', '照片浏览体验'],
  acc_accessibility: ['Windows 辅助功能', '放大镜/讲述人/高对比度', '无障碍使用支持'],
  acc_taskmgr: ['任务管理器', '进程监控和资源查看', '系统性能诊断工具'],
  acc_regedit: ['注册表编辑器', '系统注册表查看和修改', '高级配置和排错'],
  rt_appcompat: ['应用兼容性层', '旧版应用在 PE 上运行', '兼容性 shimming 机制'],
  rt_arm64_support: ['ARM64 架构 x86_64 兼容', '在 ARM 设备上运行 x64 应用', '跨架构模拟执行层'],
  rt_compatibility: ['程序兼容模式', '以特定 Windows 版本模式运行', '解决旧软件兼容问题'],
  rt_directx: ['DirectX 运行时库', '游戏和图形应用基础', 'GPU 加速渲染支持'],
  rt_speech_onecore: ['OneCore 语音识别引擎', '语音输入和命令控制', '离线语音识别能力'],
  rt_syswow_sapi: ['WoW64 SAPI 语音接口', '32 位环境语音合成', 'TTS 多平台支持'],
  uwp_taskbar: ['UWP 任务栏定制', '隐藏搜索框/Cortana 等', '精简任务栏外观'],
  uwp_startmenu: ['UWP 开始菜单', '磁贴式现代开始菜单', 'Windows 10/11 风格交互'],
  uwp_settings: ['UWP 设置应用', '现代风格系统设置面板', '触控友好的设置界面'],
  uwp_ime: ['UWP 触控输入面板', '虚拟触摸键盘', '平板和触摸屏设备支持'],
  uwp_taskmgr: ['UWP 任务管理器', '现代风格进程管理器', '简洁的系统监控视图'],
  uwp_appsrvs: ['UWP 应用服务框架', '后台任务和推送通知', 'UWP 应用运行基础设施'],
  uwp_appxapps: ['预装 UWP 应用', '计算器/商店等现代应用', '开箱即用的应用集合'],
  drv_system: ['系统基础驱动集合', '存储/显示/音频/网络驱动', 'PE 硬件兼容性基础保障'],
  app_7zip: ['7-Zip 压缩解压工具', '支持 7z/zip/rar/tar/gz 等格式', '高压缩比文件归档'],
  app_explorerpp: ['Explorer++ 增强文件管理器', '标签页/双窗格/书签功能', '专业级文件操作体验'],
  app_defraggler: ['Defraggler 磁盘碎片整理', '文件/卷级别碎片分析', '磁盘性能优化工具'],
  app_imdisk: ['ImDisk 虚拟磁盘工具', '创建 RAM 磁盘和镜像挂载', 'ISO/WIM/VHD 即时挂载'],
  app_hotswap: ['HotSwap! 硬盘热插拔', 'SATA/eSATA 热拔插支持', '无需重启更换硬盘'],
  app_penetwork: ['PENetwork 网络管理工具', 'IP 配置/DNS 设置/网卡诊断', 'PE 网络一站式管理'],
  app_yong_ime: ['勇哥输入法', '专为 PE 优化的中文输入法', '小巧高效无需 .NET'],
  app_chrome: ['Google Chrome 浏览器', '现代化 Web 浏览体验', '开发者工具和扩展生态'],
  mat_office2007: ['Microsoft Office 2007', 'Word/Excel/PowerPoint 办公套件', '文档处理和表格计算'],
  mat_potplayer: ['PotPlayer 视频播放器', '全格式视频解码支持', '硬件加速高清播放'],
  mat_dismpp: ['Dism++ 系统镜像工具', 'WIM 编辑/清理/优化', '镜像体积精简和定制'],
  mat_everything: ['Everything 文件搜索', '实时文件名索引引擎', '毫秒级文件定位'],
  mat_sumatrapdf: ['SumatraPDF PDF 阅读器', '轻量高速 PDF 查看', 'PDF/EPUB/MOBI/XPS 阅读'],
  mat_winntsetup: ['WinNTSetup 系统安装器', 'Windows 安装部署工具', '磁盘分区/引导修复/系统迁移'],
  mat_mpcbe: ['MPC-BE 媒体播放器', 'MPC-HC 增强分支', '全格式媒体播放方案'],
  mat_libreoffice: ['LibreOffice 办公套件', '开源免费 Office 替代品', 'Writer/Calc/Impress 完整办公'],
  build_iso: ['生成可启动 ISO 镜像', '使用 oscdimg 创建标准 ISO', '可直接写入 U 盘或光盘'],
  build_usb: ['制作 USB 启动盘', '一键写入 PE 到 USB 设备', '即插即用的便携 PE 环境'],
}

function getComponentFeatures(id) {
  return componentFeatures[id] || [compDesc(id)]
}

function selectComponent(item) {
  selectedComponent.value = item.id
  currentView.value = 'component'
}

function isSelected(item) {
  return currentView.value !== 'welcome' && currentView.value !== 'logs' && selectedComponent.value === item.id
}

function toggleAndSelect(item) {
  item.enabled = !item.enabled
  selectedComponent.value = item.id
  currentView.value = 'component'
}

async function selectSourceFolder() {
  try {
    const result = await window.__TAURI__.dialog.open({ directory: true, title: $t('pages.source.title') })
    if (result) {
      config.sourceFolder = result
      await detectWim()
    }
  } catch (e) { console.error(e) }
}

async function selectAdkFolder() {
  try {
    const result = await window.__TAURI__.dialog.open({ directory: true, title: $t('pages.source.adkTitle') })
    if (result) config.adkPath = result
  } catch (e) { console.error(e) }
}

async function detectWim() {
  try {
    wimInfo.value = { filename: 'install.wim', version: 'Windows 11 23H2 x64', size: '4.7 GB', images: 5 }
    wimMounted.value = true
    wimError.value = false
    sourceVersion.value = $t('status.source') || 'Win11 x64'
    addLog($t('log.wimDetected'), 'success')
  } catch (e) {
    wimInfo.value = null
    wimMounted.value = false
    wimError.value = true
    addLog($t('log.wimFailed'), 'error')
  }
}

function refreshSource() { if (config.sourceFolder) detectWim() }

function addLog(text, type = 'info') {
  const now = new Date()
  const time = `${now.getHours().toString().padStart(2,'0')}:${now.getMinutes().toString().padStart(2,'0')}:${now.getSeconds().toString().padStart(2,'0')}`
  logs.value.push({ time, text, type })
  nextTick(() => { if (logContainer.value) logContainer.value.scrollTop = logContainer.value.scrollHeight })
}

async function startBuild() {
  if (!canBuild.value) return
  currentView.value = 'build'
  isBuilding.value = true
  buildProgress.value = 0
  buildStepIndex.value = 0
  logs.value = []
  
  addLog($t('log.buildStarted'), 'info')
  addLog($t('log.sourceInfo', { path: config.sourceFolder }), 'info')
  addLog($t('log.componentInfo', { count: enabledCount.value }), 'info')
  addLog($t('log.targetInfo', { drive: targetDrive.value, type: buildType.value }), 'info')
  
  for (let i = 0; i < buildSteps.length && isBuilding.value; i++) {
    buildStepIndex.value = i
    currentBuildStep.value = stepName(i)
    addLog($t('log.step', { current: i + 1, total: buildSteps.length, name: stepName(i) }), 'info')
    
    for (let j = 0; j < 20 && isBuilding.value; j++) {
      await new Promise(r => setTimeout(r, 80))
      buildProgress.value = Math.min(100, ((i * 20) + j + 1))
    }
    if (i === buildSteps.length - 1) { addLog($t('log.buildComplete'), 'success'); currentBuildStep.value = stepName(7) }
  }
  isBuilding.value = false
}

function clearLog() { logs.value = [] }

async function copyLog() {
  const text = logs.value.map(l => `${l.time} ${l.text}`).join('\n')
  await window.__TAURI__.clipboard.writeText(text)
  addLog($t('log.logsCopied'), 'success')
}
</script>
