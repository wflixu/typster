import { createApp } from "vue";
import "./style/styles.css";
import "github-markdown-css";
import "./shared/monaco-hook";

import TodayUI from "today-ui";

import App from "./App.vue";
import { pinia } from "./store/store";

const app = createApp(App);
app.use(TodayUI);
app.use(pinia);
app.mount("#app");
