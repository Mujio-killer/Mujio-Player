<!-- FilmDetail.vue -->
<template>
  <div>
    <div style="margin-left: 40px">
      <div style="display: flex; align-items: center;">
        <img :src="rowData.pic" style="width: 15%; margin-right: 20px;">
        <div style="padding-top: 5%">
          <span class="video-info"><b>名称:</b> {{ rowData.name }}<br/></span>
          <span class="video-info"><b>类型:</b> {{ rowData.type_str }}<br/></span>
          <span class="video-info"><b>语言:</b> {{ rowData.lang }}<br/></span>
          <span class="video-info"><b>地区:</b> {{ rowData.area }}<br/></span>
          <span class="video-info"><b>上映时间:</b> {{ rowData.year }}<br/></span>
          <span class="video-info"><b>导演:</b> {{ rowData.director }}<br/></span>
          <span class="video-info"><b>主演:</b> {{ rowData.actor }}<br/></span>
        </div>
      </div>
      <div>
        <span><b>描述:</b> <p v-html="rowData.desc"></p></span>
      </div>
      <div>
        <h3>播放资源</h3>
        <el-tabs type="border-card">
          <el-tab-pane v-for="videoSrc in rowData.video_source" :key="videoSrc.flag" :label="videoSrc.flag">
            <el-radio-group v-model="selectedEpisode">
              <el-radio-button v-for="videoEpisode in videoSrc.episodes" :key="videoEpisode.link" :label="videoEpisode.episode" border @change="playVideo(videoSrc, videoEpisode)">
                {{ videoEpisode.episode }}
              </el-radio-button>
            </el-radio-group>
          </el-tab-pane>
        </el-tabs>
      </div>
    </div>
    <div style="position: sticky; bottom: 0; background-color: rgba(255, 255, 255, 0.5); padding: 10px; border-radius: 5px; z-index: 1;">
      <button @click="toggleExpand" style="background-color: transparent; border: none;">合起</button>
    </div>
  </div>
</template>

<script setup lang="ts">
import {ref, defineProps} from 'vue';
import {useAppStateStore} from "../stores";

const appState = useAppStateStore();
const props = defineProps({
  rowData: {
    type: Object,
    required: true,
  },
});

// 动态设置选中的资源
const toggleExpand = (row: any) => {
  row.expanded = !row.expanded;
};

const selectedEpisode = ref();
// 播放选中的资源
const playVideo = (videoSrc: any, episode: any) => {
  appState.selectedVideoSrc = videoSrc;
  appState.selectedEpisode = episode;
  appState.view = '2';
  console.log(episode);
}

</script>