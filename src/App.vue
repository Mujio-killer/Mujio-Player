<template>
  <div class="common-layout">
    <el-container>
      <el-header>mujio</el-header>
      <el-container>
        <el-aside>
          <Aside />
        </el-aside>
        <el-main>
          <Explorer v-if="needShow('1')"/>
          <Play v-if="needShow('2')" />
          <Star v-if="needShow('3')"/>
          <History v-if="needShow('4')"/>
          <Setting v-if="needShow('5')"/>
        </el-main>
      </el-container>
    </el-container>
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


const appState = useAppStateStore();

const needShow = (idx:string) => {
  return appState.getView == idx;
}

</script>
<style lang="scss">
html, body, #app {
  height: 100%;
  border-radius: 0px;
}
</style>

