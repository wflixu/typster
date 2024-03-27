import { LogicalPosition, appWindow } from "@tauri-apps/api/window";
import { ref } from "vue";

export function useWinMove() {
  const moving = ref(false);
  const lastPos = ref<number[]>([]);

  const move = async (evt: MouseEvent) => {
    const { screenX, screenY, pageX, pageY } = evt;
    const [sX, sY] = lastPos.value;
    const ph = new LogicalPosition(
      screenX - pageX + (screenX - sX),
      screenY - pageY + (screenY - sY)
    );
    await appWindow.setPosition(ph);
    lastPos.value = [screenX, screenY];
  };

  const mousedownHandler = async (evt: MouseEvent) => {
    const { screenX, screenY } = evt;
    moving.value = true;
    lastPos.value = [screenX, screenY];
    console.log("mousedown poiont", lastPos.value);
  };

  const mouseupHandler = async (evt: MouseEvent) => {
    console.log(" mouseup");
    moving.value = false;
    await move(evt);
  };

  const mousemoveHandler = async (evt: MouseEvent) => {
    if (moving.value) {
      console.log("mousemove");
      await move(evt);
    }
  };

  const mouseleaveHandler = async (evt: MouseEvent) => {
    if (moving.value) {
      moving.value = false;
      await move(evt);
    }
  };

  return {
    moving,
    lastPos,
    mousedownHandler,
    mouseupHandler,
    mousemoveHandler,
    mouseleaveHandler,
  };
}
