<template>
  <div>
    <video id="player" playsinline
           data-poster="https://forthebadge.com/images/featured/featured-fuck-it-ship-it.svg">
      <source :src="appState.selectedEpisode.link" type="video/mp4" />
      <source :src="appState.selectedEpisode.link" type="video/m3u8" />
    </video>

    <el-button type="primary" @click="show">选集</el-button>
    <PlayList v-model="showPalyList"/>
  </div>
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue';
import Plyr from 'plyr';
import 'plyr/dist/plyr.css';
import { useAppStateStore } from "../stores";
import PlayList from './PlayList.vue';

const appState = useAppStateStore();
const selectedEpisode = appState.selectedEpisode;

const player = ref(null);
const showPalyList = ref(false);

onMounted(() => {
   player.value = new Plyr('#player', appState.plyrPlayer);
   appState.selectedEpisode.currentTime = ref(player.value.currentTime);
   appState.selectedEpisode.currentVolume = ref(player.value.volume);
   // player.volume = appState.selectedEpisode.currentVolume;
  // 添加鼠标移动事件监听器
});

const show = () => {
  showPalyList.value = !showPalyList.value;
}
</script>

<style scoped>
.custom-playlist {
  background: #d7b9b9;
}
</style>