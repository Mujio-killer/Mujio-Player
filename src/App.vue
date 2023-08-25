<template>
  <div class="container">
    <Aside/>
    <Explorer/>
    <Play/>
    <Star/>
    <History/>
    <Setting/>
  </div>
</template>

<script setup lang="ts">
import Aside from "./components/Aside.vue";
import Explorer from "./components/Explorer.vue";
import Play from "./components/Play.vue";
import Star from "./components/Star.vue";
import History from "./components/History.vue";
import Setting from "./components/Setting.vue";

import {useAppStateStore} from "./stores";
import {computed} from "vue";


const appState = useAppStateStore();
const view = computed({
  get() {
    return appState.getView
  },
  set(val) {
    appState.setView(val);
  }
});
</script>
<style lang="scss">
html, body, #app {
  height: 100%;
  border-radius: 0px;
}
</style>

