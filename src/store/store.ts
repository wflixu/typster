import { createPinia, defineStore } from "pinia";
import { MaybeRef, reactive, ref, toValue, unref } from "vue";

const pinia = createPinia();
const EDITING_FILE = "EDITING_FILE";
const DEVICE_key = "ICAMERA_DEVICE_ID";

const useSystemStoreHook = defineStore("system", () => {
  const editingFilePath = ref(window.localStorage.getItem(EDITING_FILE) ?? "");
  const setEditingFilePath = (val: string) => {
    editingFilePath.value = val;
    window.localStorage.setItem(EDITING_FILE, val);
  };

  const dirs = reactive([]);

  const imgBedToken = ref<string>(
    "Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJuYW1lIjoibHgiLCJpZCI6MSwiaWF0IjoxNzA5ODg3MjUwLCJleHAiOjE3MTAwNjAwNTB9.oq5FJrAWN7HsNBqibFWY3ZPd0w6odM2W5j_Vr6UruwA"
  );
  const imgBedUrl = ref("http://127.0.0.1:8443/chunk/upload");
  const imgBedShowBaseUrl = ref("http://127.0.0.1:8443/chunk/show");

  return {
    editingFilePath,
    setEditingFilePath,
    dirs,
    imgBedUrl,
    imgBedShowBaseUrl,
    imgBedToken,
  };
});

export { pinia, useSystemStoreHook };
