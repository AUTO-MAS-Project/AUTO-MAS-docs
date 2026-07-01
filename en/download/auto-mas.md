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
    <div class="download-card-title">GitHub Repository</div>
    <div class="download-card-desc">Download the latest version from GitHub Releases<br>For users who can access GitHub directly</div>
    <a href="https://github.com/AUTO-MAS-Project/AUTO-MAS/releases/latest" target="_blank">Download</a>
  </div>

  <div class="download-card">
    <picture>
      <img src="https://mirrorchyan.com/favicon.ico" alt="MirrorChyan icon" style="width:48px;height:48px;">
    </picture>
    <div class="download-card-title">MirrorChyan</div>
    <div class="download-card-desc">High-speed latest-version downloads through MirrorChyan<br>For users with a MirrorChyan CDK</div>
    <a href="https://mirrorchyan.com/zh/projects?rid=AUTO_MAS&scource=AUTO-MAS-Web" target="_blank">High-Speed Download</a>
  </div>

  <div class="download-card">
    <picture>
      <img src="https://pan.quark.cn/favicon.ico" alt="Quark Cloud Drive icon" style="width:48px;height:48px;">
    </picture>
    <div class="download-card-title">Quark Cloud Drive</div>
    <div class="download-card-desc">Download the latest version from Quark Cloud Drive for free<br>Your save action can provide a small benefit to the project team</div>
    <a href="https://pan.quark.cn/s/5003e30dc69b" target="_blank">Open Drive</a>
  </div>

  <div class="download-card">
    <picture>
      <img src="https://mall.bilibili.com/favicon.ico" alt="Bilibili Shop icon" style="width:48px;height:48px;">
    </picture>
    <div class="download-card-title">Bilibili Shop</div>
    <div class="download-card-desc">Purchase and download the stable version from Bilibili Shop<br>For users who want to additionally support the developers</div>
    <a href="https://mall.bilibili.com/neul-next/detailuniversal/detail.html?itemsId=11306389" target="_blank">Open Product Page</a>
  </div>

</div>
