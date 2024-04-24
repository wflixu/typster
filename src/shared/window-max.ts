
import { ref } from "vue";
import {
  PhysicalPosition,
  PhysicalSize,
  appWindow,
  currentMonitor
} from "@tauri-apps/api/window";

export function useToggleWindowMax() {
  let curSize: PhysicalSize
  let curPosition: PhysicalPosition
  let isMax = ref(false);
  const toggleWindowMax = async () => {
    if (isMax.value) {
      if (curSize && curPosition) {
        await appWindow.setSize(curSize)
        await appWindow.setPosition(curPosition)
        isMax.value = false
      }
    } else {
      const monitor = await currentMonitor()
      curSize = await appWindow.outerSize()
      curPosition = await appWindow.outerPosition()
      if (monitor) {
        await appWindow.setSize(monitor.size)
        await appWindow.setPosition(monitor.position)
        isMax.value = true
      }
    }
  }

  return {
    isMax,
    toggleWindowMax
  };
}
