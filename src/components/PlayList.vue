<template>
  <el-drawer v-model="drawer" size="15%" title="播放列表" :with-header="true" :show-close="true">
        <el-check-tag checked style="margin-right: 8px"
        v-for="item in playList" @change="onChange">{{ item.episode }}</el-check-tag>

  </el-drawer>
</template>

<script lang="ts" setup>
import { ref } from 'vue';
import { useAppStateStore } from "../stores";

const appState = useAppStateStore();
const playList = appState.selectedVideoSrc;

const drawer = ref(true);

const checked = ref(false)

const onChange = (status: boolean) => {
  checked.value = status
}
</script>

<style scoped>
#playlist-btn {
  position: fixed;
  right: 0;
  bottom: 50%;
  transform: translateY(50%);
  background: transparent;
  border: none;
  font-size: 24px;
  cursor: pointer;
  transition: transform 0.3s;
}

#playlist-btn:hover {
  transform: translateX(-100%);
}
</style>
