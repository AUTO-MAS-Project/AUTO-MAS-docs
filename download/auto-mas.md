<script setup>
import { onMounted, onBeforeUnmount } from 'vue'

function updateGithubIcon() {
  const isDark = document.documentElement.classList.contains('dark')
  const githubImg = document.querySelector('img[data-icon="github"]')
  if (githubImg) {
    githubImg.src = isDark ? '/icons/github-dark.svg' : '/icons/github.svg'
  }
}

let observer
onMounted(() => {
  updateGithubIcon()
  observer = new MutationObserver(updateGithubIcon)
  observer.observe(document.documentElement, { attributes: true, attributeFilter: ['class'] })
})
onBeforeUnmount(() => {
  observer && observer.disconnect()
})
</script>
<style>
.download-cards {
  display: flex;
  gap: 2rem;
  flex-wrap: wrap;
  margin-bottom: 2rem;
  justify-content: center;
}
.download-card {
  flex: 1 1 280px;
  max-width: 340px;
  background: var(--vp-c-bg);
  border-radius: 18px;
  box-shadow: 0 4px 24px 0 rgba(60,60,60,0.08), 0 1.5px 6px 0 rgba(60,60,60,0.04);
  padding: 2rem 1.2rem 1.5rem 1.2rem;
  text-align: center;
  border: 1.5px solid var(--vp-c-divider);
  transition: box-shadow 0.25s, border-color 0.25s, transform 0.18s;
  position: relative;
  overflow: hidden;
}
.download-card:hover {
  box-shadow: 0 8px 32px 0 rgba(60,60,60,0.16), 0 2px 8px 0 rgba(60,60,60,0.08);
  border-color: var(--vp-c-brand);
  transform: translateY(-4px) scale(1.025);
}
.download-card picture {
  display: flex;
  justify-content: center;
  align-items: center;
  margin-bottom: 1.2rem;
}
.download-card img {
  width: 54px;
  height: 54px;
  border-radius: 12px;
  background: var(--vp-c-bg-soft);
  box-shadow: 0 1px 4px 0 rgba(60,60,60,0.08);
}
.download-card-title {
  font-size: 1.18rem;
  font-weight: 700;
  margin-bottom: 0.6rem;
  letter-spacing: 0.01em;
}
.download-card-desc {
  color: var(--vp-c-text-2);
  font-size: 1rem;
  margin-bottom: 1.2rem;
  min-height: 2.5em;
}
.download-card a {
  display: inline-block;
  padding: 0.5em 1.5em;
    background: var(--vp-c-brand);
    color: var(--vp-c-bg);
  border-radius: 8px;
  text-decoration: none;
  font-weight: 600;
  font-size: 1.02rem;
  box-shadow: 0 1px 4px 0 rgba(60,60,60,0.08);
    transition: none;
}
  .download-card a:hover {
  background: var(--vp-c-brand);
  color: var(--vp-c-bg);
  box-shadow: 0 1px 4px 0 rgba(60,60,60,0.08);
  transform: none;
}
@media (max-width: 900px) {
  .download-cards {
    flex-direction: column;
    gap: 1.2rem;
    align-items: center;
  }
  .download-card {
    max-width: 98vw;
    width: 100%;
  }
}
</style>

<div class="download-cards">

  <div class="download-card">
    <div style="display:flex;justify-content:center;align-items:center;margin-bottom:1.2rem;">
      <img data-icon="github" src="/icons/github.svg" alt="GitHub" style="width:48px;height:48px;">
    </div>
    <div class="download-card-title">GitHub 仓库</div>
    <div class="download-card-desc">从 GitHub Release 直接下载最新版本<br>适用于能够直接访问 GitHub 的用户</div>
    <a href="https://github.com/DLmaster361/AUTO_MAA/releases/latest" target="_blank">前往下载</a>
  </div>

  <div class="download-card">
    <picture>
      <img src="https://mirrorchyan.com/favicon.ico" alt="Mirror酱图标" style="width:48px;height:48px;">
    </picture>
    <div class="download-card-title">Mirror 酱</div>
    <div class="download-card-desc">从 Mirror 酱高速下载最新版本<br>适用于已有 Mirror 酱 CDK 的富裕用户</div>
    <a href="https://mirrorchyan.com/zh/projects?rid=AUTO_MAA&scource=AUTO_MAA-Web" target="_blank">高速下载</a>
  </div>

  <div class="download-card">
    <picture>
      <img src="https://pan.quark.cn/favicon.ico" alt="夸克网盘图标" style="width:48px;height:48px;">
    </picture>
    <div class="download-card-title">夸克网盘</div>
    <div class="download-card-desc">从夸克网盘免费下载最新版本<br>您的转存行为能为项目组带来少量收益</div>
    <a href="https://pan.quark.cn/s/5003e30dc69b" target="_blank">访问网盘</a>
  </div>

  <div class="download-card">
    <picture>
      <img src="https://mall.bilibili.com/favicon.ico" alt="B站小店图标" style="width:48px;height:48px;">
    </picture>
    <div class="download-card-title">B 站小店</div>
    <div class="download-card-desc">从 B 站小店购买并下载稳定版本<br>适用于希望额外打赏开发者的富裕用户</div>
    <a href="https://mall.bilibili.com/neul-next/detailuniversal/detail.html?isMerchant=1&page=detailuniversal_detail&saleType=10&itemsId=11306389&loadingShow=1&noTitleBar=1&msource=merchant_share" target="_blank">前往商品页</a>
  </div>

</div>