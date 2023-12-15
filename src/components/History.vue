<!--
<template>
  <el-table v-if="playHistoryData.length > 0" :data="playHistoryData" >
    <el-table-column prop="date" label="观看时间" width="180" />
    <el-table-column prop="site" label="来源" width="180" />
    <el-table-column prop="name" label="名称" width="180" />
    <el-table-column prop="address" label="链接" />
  </el-table>
  <el-empty v-else :image-size="200" image="https://forthebadge.com/images/featured/featured-fuck-it-ship-it.svg" />
</template>

<script lang="ts" setup>
import {onMounted, ref} from 'vue';
import {invoke} from '@tauri-apps/api/tauri';
import {useAppStateStore} from "../stores";

const appState = useAppStateStore();
const playHistoryData = ref([]);

onMounted(async () => {
  let appHistoryData = appState.playHistoryData;
  if (appHistoryData.length == 0) {
    appHistoryData = await invoke('read_json_file', {filePath: "../dist/history.json"});
  }
  playHistoryData.value = appHistoryData;
});


</script>

<style>
</style>
-->
