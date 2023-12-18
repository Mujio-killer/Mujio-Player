<template>
  <div id="app" class="common-layout">
    <el-container class="app-media">
      <el-aside class="media-side">
        <Aside/>
      </el-aside>
      <el-main class="media-content">
        <KeepAlive>
          <Explorer class="media-content media-content-explorer" v-if="needShow('1')"/>
        </KeepAlive>
        <KeepAlive>
          <PlayPlyr style="width: 100%; height: 100%" class="media-content media-content-playPlyr"
                    v-if="needShow('2')"/>
        </KeepAlive>

        <!--        <History style="width: 100%; height: 100%" class="media-content media-content-history" v-if="needShow('3')"/>-->
        <Setting class="media-content media-content-setting" v-if="needShow('4')"/>
      </el-main>
    </el-container>
  </div>
</template>

<script setup lang="ts">
import Aside from "./components/Aside.vue";
import Explorer from "./components/Explorer.vue";
import PlayPlyr from "./components/PlayPlyr.vue";
import Setting from "./components/Setting.vue";
import {useAppStateStore} from "./stores";


const appState = useAppStateStore();

const needShow = (idx: string) => {
  return appState.getView == idx;
}

</script>
<style lang="scss">
html, body, #app {
  //border-radius: 10px;
  //position: relative;
  //display: flex;
  height: 100%;
  width: 100%;
  margin: 0;
  padding: 0;
  border: 0;
}

.app-media {
  background: #e9eae9;
  height: 100%;
  width: 100%;
  border: 0;
}

.media-side {
  width: 70px;
  height: 100%;
  background: #ffffff;
  padding-top: 20%;
  border: 0;
}

.media-content {
  background: #e9eae9;
  border: 0;
  height: 100%;
  width: 100%;
  overflow: hidden;
}
</style>

