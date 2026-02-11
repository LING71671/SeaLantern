<script setup lang="ts">
import { ref, onMounted, onBeforeMount } from "vue";
import { openUrl } from "@tauri-apps/plugin-opener";
import SLCard from "../components/common/SLCard.vue";
import SLButton from "../components/common/SLButton.vue";
import { contributors as contributorsList } from "../data/contributors";
import { checkUpdate, openDownloadUrl, type UpdateInfo } from "../api/update";

console.log('[AboutView] 脚本开始执行');

const version = "0.1.1";
const buildDate = "2026";

const contributors = ref(contributorsList);

// 更新相关状态
const isCheckingUpdate = ref(false);
const updateInfo = ref<UpdateInfo | null>(null);
const updateError = ref<string | null>(null);

onBeforeMount(() => {
  console.log('[AboutView] onBeforeMount - 组件即将挂载');
});

onMounted(() => {
  console.log('[AboutView] onMounted - 组件已挂载');
  console.log('[AboutView] contributors:', contributors.value);
});

// 打开外部链接
async function openLink(url: string) {
  console.log('[AboutView] 尝试打开URL:', url);
  if (!url) return;
  try {
    await openUrl(url);
    console.log('[AboutView] URL打开成功');
  } catch (e) {
    console.error("[AboutView] 打开URL失败:", e);
    alert(`无法打开链接: ${e}`);
  }
}

// 检查更新
async function handleCheckUpdate() {
  console.log('[AboutView] 开始检查更新');
  isCheckingUpdate.value = true;
  updateError.value = null;
  updateInfo.value = null;

  try {
    // 替换为你的Gitee用户名和仓库名
    const info = await checkUpdate("fps_z", "SeaLantern");
    console.log('[AboutView] 更新信息:', info);
    updateInfo.value = info;
  } catch (error) {
    console.error('[AboutView] 检查更新失败:', error);
    updateError.value = error as string;
  } finally {
    isCheckingUpdate.value = false;
  }
}

// 下载更新
async function handleDownloadUpdate() {
  console.log('[AboutView] 开始下载更新');
  if (updateInfo.value?.download_url) {
    try {
      await openDownloadUrl(updateInfo.value.download_url);
      console.log('[AboutView] 下载链接已打开');
    } catch (error) {
      console.error('[AboutView] 打开下载链接失败:', error);
      alert(`打开下载链接失败: ${error}`);
    }
  }
}

console.log('[AboutView] 脚本执行完成');
</script>

<template>
  <div class="about-view animate-fade-in-up">
    <!-- Hero Section -->
    <div class="hero-section">
      <div class="hero-logo">
        <img src="../assets/logo.svg" alt="Sea Lantern" width="72" height="72" />
      </div>
      <h1 class="hero-title">Sea Lantern</h1>
      <p class="hero-subtitle">Minecraft 服务器管理工具</p>
      <div class="hero-badges">
        <span class="version-badge">v{{ version }}</span>
        <span class="tech-badge">Tauri 2 + Vue 3</span>
        <span class="license-badge">GPLv3</span>
      </div>
      <p class="hero-desc">
        一个由社区共同打造的 Minecraft 开服器。<br/>
        不仅代码开源，连灵魂都由你们定义。
      </p>
    </div>

    <!-- Manifesto -->
    <SLCard>
      <div class="manifesto">
        <h3 class="manifesto-title">为什么叫 Sea Lantern？</h3>
        <p class="manifesto-text">
          海晶灯（Sea Lantern）是 Minecraft 中一种发光方块——它由无数碎片组合而成，却能发出柔和而持久的光。
        </p>
        <p class="manifesto-text">
          就像这个项目一样，每一位贡献者都是一片海晶碎片。<br/>
          当我们聚在一起，就能照亮整个社区。
        </p>
      </div>
    </SLCard>

    <!-- 此处缺一段代码 -->
    <!-- 点击加入开发 -->
    
    <!-- Contributor Wall -->
    <div class="contributor-section">
      <div class="section-header">
        <h2 class="section-title">贡献者墙</h2>
        <p class="section-desc">每一个让这个项目变得更好的人</p>
      </div>

      <div class="contributor-grid">
        <div
          v-for="c in contributors"
          :key="c.name"
          class="contributor-card glass-card"
        >
          <img :src="c.avatar" :alt="c.name" class="contributor-avatar" />
          <div class="contributor-info">
            <span class="contributor-name">{{ c.name }}</span>
            <span class="contributor-role">{{ c.role }}</span>
          </div>
        </div>

        <!-- Join Card -->
        <div class="contributor-card glass-card join-card">
          <div class="join-icon">
            <svg width="40" height="40" viewBox="0 0 24 24" fill="none" stroke="var(--sl-primary)" stroke-width="1.5" stroke-linecap="round">
              <path d="M12 4v16m8-8H4" />
            </svg>
          </div>
          <div class="contributor-info">
            <span class="contributor-name join-text">你的名字</span>
            <span class="contributor-role">参与贡献，加入我们</span>
          </div>
        </div>
      </div>
    </div>

    <!-- Project Info -->
    <div class="info-grid">
      <SLCard title="项目信息">
        <div class="info-list">
          <div class="info-item">
            <span class="info-label">版本</span>
            <span class="info-value">{{ version }}</span>
          </div>
          <div class="info-item">
            <span class="info-label">构建年份</span>
            <span class="info-value">{{ buildDate }}</span>
          </div>
          <div class="info-item">
            <span class="info-label">前端</span>
            <span class="info-value">Vue 3 + TypeScript + Vite</span>
          </div>
          <div class="info-item">
            <span class="info-label">后端</span>
            <span class="info-value">Rust + Tauri 2</span>
          </div>
          <div class="info-item">
            <span class="info-label">许可证</span>
            <span class="info-value">GNU GPLv3</span>
          </div>
        </div>

        <!-- 检查更新按钮 -->
        <div class="update-section">
          <SLButton
            variant="secondary"
            size="sm"
            @click="handleCheckUpdate"
            :disabled="isCheckingUpdate"
            style="width: 100%"
          >
            {{ isCheckingUpdate ? "检查中..." : "检查更新" }}
          </SLButton>

          <!-- 更新信息 -->
          <div v-if="updateInfo" class="update-info">
            <div v-if="updateInfo.has_update" class="update-available">
              <div class="update-message">
                <div class="update-icon">
                  <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                    <polyline points="17 1 21 5 17 9"></polyline>
                    <path d="M3 11V9a4 4 0 0 1 4-4h14"></path>
                    <polyline points="7 23 3 19 7 15"></polyline>
                    <path d="M21 13v2a4 4 0 0 1-4 4H3"></path>
                  </svg>
                </div>
                <div>
                  <div class="update-title">发现新版本 v{{ updateInfo.latest_version }}</div>
                  <div class="update-desc">当前版本: v{{ updateInfo.current_version }}</div>
                </div>
              </div>
              <div v-if="updateInfo.release_notes" class="release-notes">
                <div class="notes-title">更新内容:</div>
                <div class="notes-content">{{ updateInfo.release_notes }}</div>
              </div>
              <SLButton
                v-if="updateInfo.download_url"
                variant="primary"
                size="sm"
                @click="handleDownloadUpdate"
                style="width: 100%; margin-top: 8px"
              >
                下载更新
              </SLButton>
            </div>
            <div v-else class="update-latest">
              <div class="update-icon">
                <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
                  <polyline points="20 6 9 17 4 12"></polyline>
                </svg>
              </div>
              <span>已是最新版本</span>
            </div>
          </div>

          <!-- 错误信息 -->
          <div v-if="updateError" class="update-error">
            <div class="error-icon">
              <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
                <circle cx="12" cy="12" r="10"></circle>
                <line x1="12" y1="8" x2="12" y2="12"></line>
                <line x1="12" y1="16" x2="12.01" y2="16"></line>
              </svg>
            </div>
            <span>{{ updateError }}</span>
          </div>
        </div>
      </SLCard>

      <SLCard title="参与方式">
        <div class="contribute-ways">
          <div class="way-item">
            <div class="way-icon">
              <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <polyline points="16 18 22 12 16 6"></polyline>
                <polyline points="8 6 2 12 8 18"></polyline>
              </svg>
            </div>
            <div class="way-info">
              <span class="way-title">写代码</span>
              <span class="way-desc">提交 PR，修 Bug 或加新功能</span>
            </div>
          </div>
          <div class="way-item">
            <div class="way-icon">
              <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <path d="M12 19l7 2-7-18-7 18 7-2zm0 0v-8"></path>
              </svg>
            </div>
            <div class="way-info">
              <span class="way-title">做设计</span>
              <span class="way-desc">设计 UI、图标、主题皮肤</span>
            </div>
          </div>
          <div class="way-item">
            <div class="way-icon">
              <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <circle cx="12" cy="12" r="10"></circle>
                <path d="M9.09 9a3 3 0 0 1 5.83 1c0 2-3 3-3 3"></path>
                <line x1="12" y1="17" x2="12.01" y2="17"></line>
              </svg>
            </div>
            <div class="way-info">
              <span class="way-title">提建议</span>
              <span class="way-desc">在 Issues 里提出你的想法</span>
            </div>
          </div>
          <div class="way-item">
            <div class="way-icon">
              <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <path d="M4 19.5A2.5 2.5 0 0 1 6.5 17H20"></path>
                <path d="M6.5 2H20v20H6.5A2.5 2.5 0 0 1 4 19.5v-15A2.5 2.5 0 0 1 6.5 2z"></path>
              </svg>
            </div>
            <div class="way-info">
              <span class="way-title">写文档</span>
              <span class="way-desc">完善教程和使用说明</span>
            </div>
          </div>
          <div class="way-item">
            <div class="way-icon">
              <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <circle cx="12" cy="12" r="10"></circle>
                <line x1="2" y1="12" x2="22" y2="12"></line>
                <path d="M12 2a15.3 15.3 0 0 1 4 10 15.3 15.3 0 0 1-4 10 15.3 15.3 0 0 1-4-10 15.3 15.3 0 0 1 4-10z"></path>
              </svg>
            </div>
            <div class="way-info">
              <span class="way-title">翻译</span>
              <span class="way-desc">帮助翻译成其他语言</span>
            </div>
          </div>
          <div class="way-item">
            <div class="way-icon">
              <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <path d="M4 12v8a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2v-8"></path>
                <polyline points="16 6 12 2 8 6"></polyline>
                <line x1="12" y1="2" x2="12" y2="15"></line>
              </svg>
            </div>
            <div class="way-info">
              <span class="way-title">推广</span>
              <span class="way-desc">分享给更多 MC 服主</span>
            </div>
          </div>
        </div>
      </SLCard>
    </div>

    <!-- Links -->
    <div class="links-section">
      <SLButton variant="primary" size="lg" @click="openLink('https://gitee.com/fps_z/SeaLantern')">
        Gitee 仓库
      </SLButton>
      <SLButton variant="secondary" size="lg" @click="openLink('https://space.bilibili.com/3706927622130406?spm_id_from=333.1387.0.0')">
        B站主页
      </SLButton>
    </div>

    <!-- Footer -->
    <div class="about-footer">
      <p class="footer-text">
        Sea Lantern 是一个开源项目，遵循 GPLv3 协议。
      </p>
      <p class="footer-text">
        Minecraft 是 Mojang Studios 的注册商标。本项目与 Mojang 无关。
      </p>
      <p class="footer-quote">
        "我们搭建了骨架，而灵魂，交给你们。"
      </p>
    </div>
  </div>
</template>

<style scoped>
.about-view {
  display: flex;
  flex-direction: column;
  gap: var(--sl-space-xl);
  max-width: 900px;
  margin: 0 auto;
  padding-bottom: var(--sl-space-2xl);
}

/* Hero */
.hero-section {
  display: flex;
  flex-direction: column;
  align-items: center;
  text-align: center;
  padding: var(--sl-space-2xl) 0;
}

.hero-logo {
  margin-bottom: var(--sl-space-md);
  animation: sl-fade-in-up 0.6s ease forwards;
}

.hero-title {
  font-size: 2.5rem;
  font-weight: 800;
  color: var(--sl-text-primary);
  letter-spacing: -0.03em;
  margin-bottom: var(--sl-space-xs);
  animation: sl-fade-in-up 0.6s ease 0.1s both;
}

.hero-subtitle {
  font-size: 1.125rem;
  color: var(--sl-text-secondary);
  margin-bottom: var(--sl-space-md);
  animation: sl-fade-in-up 0.6s ease 0.2s both;
}

.hero-badges {
  display: flex;
  gap: var(--sl-space-sm);
  margin-bottom: var(--sl-space-lg);
  animation: sl-fade-in-up 0.6s ease 0.3s both;
}

.version-badge, .tech-badge, .license-badge {
  padding: 4px 14px;
  border-radius: var(--sl-radius-full);
  font-size: 0.8125rem;
  font-weight: 500;
}

.version-badge {
  background: var(--sl-primary-bg);
  color: var(--sl-primary);
}

.tech-badge {
  background: rgba(34, 197, 94, 0.1);
  color: var(--sl-success);
}

.license-badge {
  background: rgba(168, 85, 247, 0.1);
  color: #a855f7;
}

.hero-desc {
  font-size: 1rem;
  color: var(--sl-text-secondary);
  line-height: 1.8;
  animation: sl-fade-in-up 0.6s ease 0.4s both;
}

/* Manifesto */
.manifesto {
  text-align: center;
  padding: var(--sl-space-lg);
}

.manifesto-title {
  font-size: 1.25rem;
  font-weight: 700;
  color: var(--sl-text-primary);
  margin-bottom: var(--sl-space-md);
}

.manifesto-text {
  font-size: 0.9375rem;
  color: var(--sl-text-secondary);
  line-height: 1.8;
  margin-bottom: var(--sl-space-sm);
}

/* Contributor Wall */
.contributor-section {
  display: flex;
  flex-direction: column;
  gap: var(--sl-space-md);
}

.section-header {
  text-align: center;
}

.section-title {
  font-size: 1.5rem;
  font-weight: 700;
  color: var(--sl-text-primary);
  margin-bottom: var(--sl-space-xs);
}

.section-desc {
  font-size: 0.9375rem;
  color: var(--sl-text-tertiary);
}

.contributor-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(220px, 1fr));
  gap: var(--sl-space-md);
}

.contributor-card {
  display: flex;
  align-items: center;
  gap: var(--sl-space-md);
  padding: var(--sl-space-md);
  transition: all var(--sl-transition-normal);
}

.contributor-card.clickable {
  cursor: pointer;
}

.contributor-card.clickable:hover {
  transform: translateY(-4px);
  box-shadow: var(--sl-shadow-lg);
}

.contributor-avatar {
  width: 48px;
  height: 48px;
  border-radius: var(--sl-radius-md);
  flex-shrink: 0;
  background: var(--sl-bg-tertiary);
  image-rendering: pixelated;
}

.contributor-info {
  display: flex;
  flex-direction: column;
  gap: 2px;
  min-width: 0;
}

.contributor-name {
  font-size: 0.9375rem;
  font-weight: 600;
  color: var(--sl-text-primary);
}

.contributor-role {
  font-size: 0.75rem;
  color: var(--sl-text-tertiary);
}

.join-card {
  border: 2px dashed var(--sl-border);
  background: transparent;
  cursor: default;
}

.join-card:hover {
  border-color: var(--sl-primary-light);
  background: var(--sl-primary-bg);
  transform: none;
  box-shadow: none;
}

.join-icon {
  width: 48px;
  height: 48px;
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
}

.join-text {
  color: var(--sl-primary);
}

/* Info Grid */
.info-grid {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: var(--sl-space-md);
}

.info-list {
  display: flex;
  flex-direction: column;
  gap: 0;
}

.info-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 10px 0;
  border-bottom: 1px solid var(--sl-border-light);
}

.info-item:last-child { border-bottom: none; }

.info-label {
  font-size: 0.875rem;
  color: var(--sl-text-tertiary);
}

.info-value {
  font-size: 0.875rem;
  font-weight: 500;
  color: var(--sl-text-primary);
  font-family: var(--sl-font-mono);
}

/* Contribute Ways */
.contribute-ways {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: var(--sl-space-sm);
}

.way-item {
  display: flex;
  align-items: center;
  gap: var(--sl-space-sm);
  padding: var(--sl-space-sm);
  border-radius: var(--sl-radius-md);
  transition: background var(--sl-transition-fast);
}

.way-item:hover {
  background: var(--sl-bg-secondary);
}

.way-icon {
  flex-shrink: 0;
  width: 40px;
  height: 40px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: var(--sl-primary-bg);
  color: var(--sl-primary);
  border-radius: var(--sl-radius-md);
  transition: all var(--sl-transition-fast);
}

.way-item:hover .way-icon {
  background: var(--sl-primary);
  color: white;
  transform: scale(1.05);
}

.way-info {
  display: flex;
  flex-direction: column;
  gap: 1px;
}

.way-title {
  font-size: 0.875rem;
  font-weight: 600;
  color: var(--sl-text-primary);
}

.way-desc {
  font-size: 0.75rem;
  color: var(--sl-text-tertiary);
}

/* Links */
.links-section {
  display: flex;
  justify-content: center;
  gap: var(--sl-space-md);
}

/* Footer */
.about-footer {
  text-align: center;
  padding: var(--sl-space-xl) 0;
  border-top: 1px solid var(--sl-border-light);
}

.footer-text {
  font-size: 0.8125rem;
  color: var(--sl-text-tertiary);
  margin-bottom: var(--sl-space-xs);
}

.footer-quote {
  font-size: 1rem;
  font-weight: 500;
  color: var(--sl-primary);
  font-style: italic;
  margin-top: var(--sl-space-md);
}

/* Update Section */
.update-section {
  margin-top: var(--sl-space-md);
  padding-top: var(--sl-space-md);
  border-top: 1px solid var(--sl-border-light);
}

.update-info {
  margin-top: var(--sl-space-sm);
  padding: var(--sl-space-sm);
  border-radius: var(--sl-radius-md);
  font-size: 0.875rem;
}

.update-available {
  background: var(--sl-primary-bg);
  border: 1px solid var(--sl-primary-light);
  padding: var(--sl-space-sm);
  border-radius: var(--sl-radius-md);
}

.update-message {
  display: flex;
  align-items: flex-start;
  gap: var(--sl-space-sm);
}

.update-icon {
  flex-shrink: 0;
  width: 28px;
  height: 28px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: var(--sl-primary);
  color: white;
  border-radius: var(--sl-radius-sm);
}

.update-latest .update-icon {
  background: var(--sl-success);
}

.update-title {
  font-weight: 600;
  color: var(--sl-primary);
  margin-bottom: 2px;
}

.update-desc {
  font-size: 0.75rem;
  color: var(--sl-text-tertiary);
}

.release-notes {
  margin-top: var(--sl-space-sm);
  padding-top: var(--sl-space-sm);
  border-top: 1px solid var(--sl-border-light);
}

.notes-title {
  font-size: 0.75rem;
  font-weight: 600;
  color: var(--sl-text-secondary);
  margin-bottom: 4px;
}

.notes-content {
  font-size: 0.8125rem;
  color: var(--sl-text-secondary);
  line-height: 1.6;
  max-height: 120px;
  overflow-y: auto;
  white-space: pre-wrap;
}

.update-latest {
  display: flex;
  align-items: center;
  gap: var(--sl-space-xs);
  padding: var(--sl-space-sm);
  background: rgba(34, 197, 94, 0.1);
  border: 1px solid rgba(34, 197, 94, 0.2);
  border-radius: var(--sl-radius-md);
  color: var(--sl-success);
  font-weight: 500;
}

.update-error {
  display: flex;
  align-items: center;
  gap: var(--sl-space-xs);
  padding: var(--sl-space-sm);
  background: rgba(239, 68, 68, 0.1);
  border: 1px solid rgba(239, 68, 68, 0.2);
  border-radius: var(--sl-radius-md);
  color: var(--sl-danger);
  font-size: 0.8125rem;
}

.error-icon {
  flex-shrink: 0;
  width: 22px;
  height: 22px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: var(--sl-danger);
  color: white;
  border-radius: 50%;
}

@media (max-width: 768px) {
  .info-grid { grid-template-columns: 1fr; }
  .contribute-ways { grid-template-columns: 1fr; }
  .contributor-grid { grid-template-columns: repeat(auto-fill, minmax(180px, 1fr)); }
}
</style>
