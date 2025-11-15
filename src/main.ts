import { createApp } from "vue";
import "ant-design-vue/dist/reset.css";
import 'primeicons/primeicons.css'

import "./style/styles.css";

import "./shared/monaco-hook";

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
        preset: Aura
    }
});
app.use(setupUILib)
app.use(router);
app.use(pinia);

app.mount("#app");
