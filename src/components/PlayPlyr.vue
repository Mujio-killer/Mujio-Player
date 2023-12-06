<template>
  <div>
    <video id="player" playsinline
           data-poster="https://forthebadge.com/images/featured/featured-fuck-it-ship-it.svg">
      <source :src="appState.selectedEpisode.link" type="video/mp4" />
      <source :src="appState.selectedEpisode.link" type="video/m3u8" />
    </video>

    <div v-if="showPlaylist" class="custom-playlist">
      <PlayList/>
    </div>
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
const showPlaylist = ref(true);

onMounted(() => {
   player.value = new Plyr('#player', appState.plyrPlayer);
   appState.selectedEpisode.currentTime = ref(player.value.currentTime);
   appState.selectedEpisode.currentVolume = ref(player.value.volume);
   // player.volume = appState.selectedEpisode.currentVolume;
  // 添加鼠标移动事件监听器
});


</script>

<style scoped>
.custom-playlist {
  background: #d7b9b9;
}
</style>