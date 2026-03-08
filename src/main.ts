import { createApp } from "vue";
import 'primeicons/primeicons.css'

// 样式导入顺序很重要
import "./styles/reset.css";              // 1. 全局重置
import "./styles/tokens/base.css";       // 2. Design Tokens 基础
import "./styles/tokens/github-light.css"; // 3. 默认主题
import "./styles/primevue-overrides.css"; // 4. PrimeVue 覆盖
import "./styles/components.css";         // 5. 组件样式
import "./styles/legacy.css";             // 6. 原有样式（保持兼容）


import TodayUI from "today-ui";

import PrimeVue from 'primevue/config';
import Aura from '@primevue/themes/aura';

import App from "./App.vue";

import { pinia } from "./store/store";
import { router } from "./router";
import { setupUILib } from "./shared/uilib";

const app = createApp(App);

app.use(TodayUI);
app.use(PrimeVue, {
    theme: {
        preset: Aura,
        options: {
            cssLayer: false,  // 禁用 CSS 层，便于覆盖
        }
    }
});
app.use(setupUILib)
app.use(router);
app.use(pinia);

app.mount("#app");
