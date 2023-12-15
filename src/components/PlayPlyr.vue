<template>
  <div>
    <div>
      <span style="color: #8798aa; float: left"><b>正在播放:</b></span>
      <el-dropdown role="navigation" type="primary">
        <span class="el-dropdown-link">
          {{ appState.currentSrc.name }} {{ appState.currentSrc.episode.name }}
          <el-icon><ArrowDownBold/></el-icon>
        </span>
        <template #dropdown>
          <el-dropdown-menu style="max-height: 200px; overflow: auto; padding-right: 10px ">
            <el-dropdown-item v-for="item in appState.currentSrc.dd.list" @click="onChange(item)">{{
                item.name
              }}
            </el-dropdown-item>
          </el-dropdown-menu>
        </template>
      </el-dropdown>
      <span style="color: #8798aa; float: right">【{{ appState.currentSrc.site.name }}】</span>
    </div>
    <video id="videoPlayer"
           controls
           preload="auto"
           poster="https://forthebadge.com/images/featured/featured-fuck-it-ship-it.svg">
      <source :src="appState.currentSrc.episode.link" type="video/mp4"/>
      <source :src="appState.currentSrc.episode.link" type="video/m3u8"/>
      <source :src="appState.currentSrc.episode.link" type="video/ogg"/>
      <source :src="appState.currentSrc.episode.link" type="application/x-mpegURL"/>

    </video>
  </div>

</template>

<script setup lang="ts">
import {useAppStateStore} from "../stores";
import {ArrowDownBold} from '@element-plus/icons-vue';

const appState = useAppStateStore();

// 切换剧集
const onChange = (episode) => {
  appState.currentSrc.episode = episode;
  reLoadPlayer(); // 切换剧集时重新加载播放器
};



// 重新加载播放器
const reLoadPlayer = () => {
  const videoPlayer = document.getElementById('videoPlayer') as HTMLVideoElement;
  videoPlayer.src = appState.currentSrc.episode.link;
  videoPlayer.load(); // 重新加载视频
  videoPlayer.play(); // 播放视频
}
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