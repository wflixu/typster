import { ref } from 'vue'

// 注意：@tauri-apps/plugin-updater 可能在构建时才可用
// 在开发环境中，这个插件可能不工作
let check: (() => Promise<any>) | null = null
let ask: ((message: string, options?: any) => Promise<boolean>) | null = null
let relaunch: (() => Promise<void>) | null = null

// 动态导入 Tauri 插件（仅在构建环境可用）
const loadUpdaterPlugin = async () => {
  try {
    const updater = await import('@tauri-apps/plugin-updater')
    check = updater.check
    const dialog = await import('@tauri-apps/plugin-dialog')
    ask = dialog.ask
    const process = await import('@tauri-apps/api/process')
    relaunch = process.relaunch
    return true
  } catch (e) {
    console.warn('Updater plugin not available:', e)
    return false
  }
}

export interface UpdateInfo {
  available: boolean
  version?: string
  date?: string
  body?: string
}

export function useAutoUpdate() {
  const isChecking = ref(false)
  const updateAvailable = ref(false)
  const updateInfo = ref<UpdateInfo | null>(null)
  const isDownloading = ref(false)
  const isDownloaded = ref(false)

  // 检查更新
  const checkForUpdates = async (showNotification = false): Promise<UpdateInfo | null> => {
    if (!check) {
      const loaded = await loadUpdaterPlugin()
      if (!loaded) {
        console.warn('Updater plugin not loaded')
        return null
      }
    }

    if (isChecking.value) return null

    isChecking.value = true

    try {
      const update = await check()

      if (update?.available) {
        updateAvailable.value = true
        updateInfo.value = {
          available: true,
          version: update.version,
          date: update.date,
          body: update.body,
        }

        console.log('Update available:', updateInfo.value)

        // 如果需要显示通知，询问用户
        if (showNotification && ask) {
          const shouldUpdate = await ask(
            `发现新版本 ${update.version}，是否立即下载并安装？`,
            { title: '更新可用', kind: 'info' }
          )

          if (shouldUpdate) {
            await downloadAndInstallUpdate(update)
          }
        }
      } else {
        if (showNotification) {
          console.log('No updates available')
        }
      }

      return updateInfo.value
    } catch (error) {
      console.error('更新检查失败:', error)
      return null
    } finally {
      isChecking.value = false
    }
  }

  // 下载并安装更新
  const downloadAndInstallUpdate = async (update: any) => {
    if (!relaunch) {
      await loadUpdaterPlugin()
    }

    isDownloading.value = true

    try {
      // 下载并安装更新
      await update.downloadAndInstall()
      isDownloaded.value = true

      // 提示用户重启
      if (ask) {
        const shouldRestart = await ask(
          '更新已下载完成，是否立即重启应用？',
          { title: '更新就绪', kind: 'info' }
        )

        if (shouldRestart && relaunch) {
          await relaunch()
        }
      }
    } catch (error) {
      console.error('更新安装失败:', error)
    } finally {
      isDownloading.value = false
    }
  }

  // 初始化自动更新检查
  const initAutoUpdate = (delaySeconds = 5) => {
    // 延迟检查更新，避免启动时卡顿
    setTimeout(() => {
      checkForUpdates(false)
    }, delaySeconds * 1000)
  }

  return {
    isChecking,
    updateAvailable,
    updateInfo,
    isDownloading,
    isDownloaded,
    checkForUpdates,
    downloadAndInstallUpdate,
    initAutoUpdate,
  }
}
