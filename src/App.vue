<template>
  <div class="container">
    <Aside />
    <Explorer />
      <Play v-show="view === 'Play'" />
      <Star v-show="view === 'Star'" />
      <History v-show="view === 'History'" />
      <Setting v-show="view === 'Setting'" />
  </div>
</template>

<script setup lang="ts">
import Aside from "./components/Aside.vue";
import Explorer from "./components/Explorer.vue";
import Play from "./components/Play.vue";
import Star from "./components/Star.vue";
import History from "./components/History.vue";
import Setting from "./components/Setting.vue";

import { useAppStateStore } from "./stores";
import { computed } from "vue";


const appState = useAppStateStore();
const view = computed({
  get () {
    return appState.getView
  },
  set (val) {
    appState.setView(val);
  }
});
</script>
<style lang="scss">
@import './assets/scss/theme.scss';
html, body, #app{
  height: 100%;
  border-radius: 0px;
}
#app {
  font-family: 'Helvetica Neue', Helvetica, 'PingFang SC', 'Hiragino Sans GB', 'Microsoft YaHei', SimSun, sans-serif;
  -webkit-font-smoothing: antialiased;
  -webkit-tap-highlight-color: transparent;
  width: 100%;
  height: 100%;
  overflow: hidden;
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  .zy-body{
    flex: 1;
    height: 100%;
    display: flex;
    justify-content: flex-start;
    align-items: flex-start;
    flex-direction: column;
    padding: 0 20px 20px;
  }
}
</style>

