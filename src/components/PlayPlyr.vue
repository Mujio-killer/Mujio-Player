<template>
  <el-card class="box-card">
      <el-dropdown>
        <span class="el-dropdown-link">
          <b>正在播放：</b>【{{ selectedEpisode.srcName }}】{{selectedEpisode.episode.episode}}<el-icon><ArrowDownBold /></el-icon>
        </span>
        <template #dropdown>
          <el-dropdown-menu>
            <el-dropdown-item v-for="item in selectedEpisode.episodes" @change="onChange">{{ item.episode }}</el-dropdown-item>
          </el-dropdown-menu>
        </template>
      </el-dropdown>
      <video id="player" controls
             data-poster="https://forthebadge.com/images/featured/featured-fuck-it-ship-it.svg">
        <source :src="appState.selectedEpisode.link" type="video/mp4" />
        <source :src="appState.selectedEpisode.link" type="video/m3u8" />
      </video>
  </el-card>

</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue';
import Plyr from 'plyr';
import 'plyr/dist/plyr.css';
import { useAppStateStore } from "../stores";
import { ArrowDownBold } from '@element-plus/icons-vue';

const appState = useAppStateStore();
const selectedEpisode = appState.selectedEpisode;

const player = ref(null);

onMounted(() => {
   player.value = new Plyr('#player', appState.plyrPlayer);
   appState.selectedEpisode.currentTime = ref(player.value.currentTime);
   appState.selectedEpisode.currentVolume = ref(player.value.volume);
   // player.volume = appState.selectedEpisode.currentVolume;
  // 添加鼠标移动事件监听器
});

const onChange = (status: boolean) => {

}
// 播放内容
selectedEpisode.link
</script>

<style scoped>
.box-card {
  width: 100%;
  height: 100%;
}

#player {
  width: 100%;
  height: 100%;
  display: flex;
  /*flex: 1;*/
}

.el-dropdown-link {
  cursor: pointer;
  color: #8798aa;
}

.el-dropdown-link:hover {
  cursor: pointer;
  color: #2986e7;
}


</style>