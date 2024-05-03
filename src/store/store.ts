import { createPinia, defineStore } from "pinia";
import { reactive, ref, } from "vue";
import { IProject } from "../pages/project/interface";
import { IMode } from "../pages/typst/interface";

const pinia = createPinia();
const EDITING_FILE = "EDITING_FILE";
const PROJECTS_KEY = "PROJECTS_KEY";
const EDITING_PROJECT = "EDITING_PROJECT";

const useSystemStoreHook = defineStore("system", () => {
  const editingFilePath = ref(window.localStorage.getItem(EDITING_FILE) ?? "");
  const setEditingFilePath = (val: string) => {
    editingFilePath.value = val;
    window.localStorage.setItem(EDITING_FILE, val);
  };

  const dirs = reactive([]);

  const projects = reactive<IProject[]>(
    JSON.parse(window.localStorage.getItem(PROJECTS_KEY) ?? "[]")
  );
  const addProject = (p: IProject) => {
    projects.push(p);
    window.localStorage.setItem(PROJECTS_KEY, JSON.stringify(projects));
  };
  const deleteProject = (project: IProject) => {
    let index = projects.findIndex((item) => {
      return item.title == project.title;
    });
    projects.splice(index, 1);
    window.localStorage.setItem(PROJECTS_KEY, JSON.stringify(projects));
  };

  const editingProject = ref<IProject | null>(
    JSON.parse(window.localStorage.getItem(EDITING_PROJECT) ?? "null")
  );
  const selectProject = (pr: IProject | null) => {
    editingProject.value = pr;
    window.localStorage.setItem(EDITING_PROJECT, JSON.stringify(pr));
    if (pr) {
      setEditingFilePath(pr.path + '/main.typ');
    } else {
      setEditingFilePath('');
    }
  };

  const mode = ref<IMode>("all");
  const setMode = (m: IMode) => {
    mode.value = m;
  };

  const showSidebar = ref(true);
  const toggleShowSidebar = (show?: boolean) => {
    showSidebar.value = show ?? !showSidebar.value;
  };

  const loading = ref(false);

  const setLoading = (state: boolean) => {
    loading.value = state;
  };

  return {
    loading,
    setLoading,
    mode,
    setMode,

    showSidebar,
    toggleShowSidebar,

    editingProject,
    selectProject,
    projects,
    addProject,
    deleteProject,
    editingFilePath,
    setEditingFilePath,
    dirs,
  };
});

export { pinia, useSystemStoreHook };
