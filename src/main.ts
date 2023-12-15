import {createApp} from "vue";
import "./styles.css";
import App from "./App.vue";
import {createPinia} from 'pinia';
import {useAppStateStore} from "./stores";
//初始化App
createApp(App)
    .use(createPinia())// 使用pinia替代vuex来管理全局变量
    .mount("#app");

// 获取站点信息
const appState = useAppStateStore();
appState.initializeGlobalVariable();
