import { createApp } from "vue";
import "./styles.css";
import App from "./App.vue";
import { createPinia } from 'pinia';
import './lib/element/index'

createApp(App)
    .use(createPinia)// 使用pinia替代vuex来管理全局变量
    .mount("#app");

