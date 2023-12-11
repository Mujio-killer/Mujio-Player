<template>
  <div>
    <div>
      <span style="color: #8798aa; float: left"><b>正在播放:</b></span>
      <el-dropdown role="navigation" type="primary">
        <span class="el-dropdown-link">
          {{ selectedEpisode.name }} {{ selectedEpisode.currentEpisode }}
          <el-icon><ArrowDownBold/></el-icon>
        </span>
        <template #dropdown>
            <el-dropdown-menu style="max-height: 200px; overflow: auto; padding-right: 10px ">
              <el-dropdown-item v-for="item in selectedEpisode.episodes" @click="onChange(item)">{{
                  item.name
                }}
              </el-dropdown-item>
            </el-dropdown-menu>
        </template>
      </el-dropdown>
      <span style="color: #8798aa; float: right">【{{ selectedEpisode.siteName }}】</span>
    </div>
    <video id="videoPlayer"
           controls="'play-large', 'play', 'progress', 'current-time', 'mute', 'volume', 'captions', 'settings', 'pip', 'airplay', 'fullscreen']"
           preload
           data-poster="https://forthebadge.com/images/featured/featured-fuck-it-ship-it.svg">
      <source :src="selectedEpisode.url" type="video/mp4"/>
      <source :src="selectedEpisode.url" type="application/x-mpegURL"/>

    </video>
  </div>

</template>

<script setup lang="ts">
import {onMounted, ref} from 'vue';
import {useAppStateStore} from "../stores";
import {ArrowDownBold} from '@element-plus/icons-vue';

const appState = useAppStateStore();
const selectedEpisode = ref(appState.selectedEpisode);

const currentVideoTitle = `【${selectedEpisode.siteName}】${ selectedEpisode.name }`;
onMounted(() => {
  // const player = new Plyr('#videoPlayer', {
  //   ratio: null
  // });
  // appState.selectedEpisode.currentTime = ref(player.value.currentTime);
  // appState.selectedEpisode.currentVolume = ref(player.value.volume);
  // player.volume = appState.selectedEpisode.currentVolume;
  // 添加鼠标移动事件监听器
});
/**
 * m3u8格式type：application/x-mpegURL
 * mp4格式type：video/mp4
 * flv格式type：video/x-flv
 * mov格式type：rtmp/mp4
 * @param episode
 */
const onChange = (episode) => {
  console.log(episode);
  // appState.selectedEpisode.name = episode.name;
  appState.selectedEpisode.currentEpisode = episode.name;
  appState.selectedEpisode.url = episode.link;

  const videoPlayer = document.getElementById('videoPlayer');
  videoPlayer.src = episode.link; // 更新视频链接
  videoPlayer.title = `${selectedEpisode.name}-${episode.name}`; //
  videoPlayer.load(); // 重新加载视频
  videoPlayer.play(); // 播放视频
};

</script>

<style scoped>

.el-dropdown-link {
  cursor: pointer;
  color: #8798aa;
}

.el-dropdown-link:hover {
  cursor: pointer;
  color: #2986e7;
}

video {
  width: 100%;
  height: 95%;
  padding-bottom: 200px;

  object-fit: fill;
}


</style>