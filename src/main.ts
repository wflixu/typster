import { createApp } from "vue";
import "./style/styles.css";
import "github-markdown-css";
import "ant-design-vue/dist/reset.css";
import "./shared/monaco-hook";

import Antd from "ant-design-vue";

import TodayUI from "today-ui";

import App from "./App.vue";
import { pinia } from "./store/store";
import { router } from "./router";

const app = createApp(App);

app.use(TodayUI);
app.use(Antd);
app.use(router);
app.use(pinia);

app.mount("#app");
