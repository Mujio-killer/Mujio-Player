<template>
  <div>
    <el-form :data="searchForm" inline status-icon>
      <div class="form-row">
        <el-form-item label="站点" class="form-item">
          <el-select v-model="searchForm.site" placeholder="请选择站点" clearable placement="bottom">
            <el-option label="全站" value="all"/>
            <el-option
                v-for="item in siteInfo"
                :key="item.id"
                :label="item.name"
                :value="item.api"
            />
          </el-select>
        </el-form-item>
        <el-form-item label="关键词" class="form-item">
          <el-input v-model="searchForm.keyword" placeholder="请输入关键词"></el-input>
        </el-form-item>

        <el-form-item class="form-item">
          <el-button type="primary" @click="search">搜索</el-button>
        </el-form-item>
      </div>
    </el-form>

    <el-table :data="appState.searchResults"
              v-if="appState.searchResults.length > 0"
              :default-sort="{ prop: 'name', order: 'descending' }"
              highlight-current-row
              style="line-height: 60px;width: 99%; overflow: hidden"
              height="95%"
              fixed
    >
      <!-- 行展开展示详情 -->
      <el-table-column fixed type="expand">
        <template #default="props">

          <div style="margin-left: 5%; margin-right: 5%">
            <el-card class="box-card">
              <template #header>
                <div class="card-header">
                  <span><b>{{ props.row.name }}</b></span>
                </div>
              </template>
              <el-row :gutter="12">
                <el-col :span="8">
                  <el-card shadow="always">
                    <el-image :src="props.row.pic"/>
                  </el-card>
                </el-col>
                <el-col :span="16">
                  <el-card shadow="never">
                    <span><b>类型:  </b>{{ props.row.type_str }}<br></span>
                    <span><b>导演:  </b>{{ props.row.director }}<br></span>
                    <span><b>主演:  </b>{{ props.row.actor }}<br></span>
                  </el-card>
                </el-col>
              </el-row>
              <span><b>简介:  </b></span>
              <span><p v-html="props.row.desc"/></span>

              <div>
                <h3>播放资源</h3>

                <el-tabs type="border-card">
                  <!-- 分资源类型展示 -->
                  <el-tab-pane v-for="videoSrc in props.row.video_source" :key="videoSrc.flag" :label="videoSrc.flag">
                    <el-radio-group v-model="selectedEpisode">
                      <el-radio-button v-for="videoEpisode in videoSrc.episodes" :key="videoEpisode.link"
                                       :label="videoEpisode.episode" border @change="playVideo(videoSrc, videoEpisode)">
                        {{ videoEpisode.episode }}
                      </el-radio-button>
                    </el-radio-group>
                  </el-tab-pane>
                </el-tabs>
              </div>
              <el-button type="primary" @click="toggleExpand(props.row)">全部收起</el-button>
            </el-card>
          </div>
        </template>
      </el-table-column>

      <!-- 行信息 -->
      <el-table-column fixed sortable prop="name" label="名称"></el-table-column>
      <el-table-column fixed prop="lang" label="语言"></el-table-column>
      <el-table-column fixed sortable prop="year" label="上映时间"></el-table-column>
      <el-table-column fixed prop="note" label="备注"></el-table-column>
    </el-table>

    <div v-else>
      <el-empty description="暂无搜索结果"/>
    </div>
  </div>
</template>

<script setup lang="ts">
import {onMounted, ref} from 'vue';
import {invoke} from '@tauri-apps/api/tauri';
import {useAppStateStore} from "../stores";

const appState = useAppStateStore();

/**
 * 获取站点信息
 */
const siteInfo = ref([]);
// 更新站点信息
onMounted(async () => {
  let allSite = appState.siteInfo;
  if (allSite == undefined || allSite.length == 0) {
    await invoke('read_json_file', {filePath: "../dist/resource.json"})
        .then((resp) => {
          allSite = resp.filter(item => item.isActive && item.status == '可用');
        })
        .catch((err) => {
          console.error(err);
        });
  }
  siteInfo.value = allSite;
  appState.setSiteInfo(allSite);
});

/**
 * 搜索资源
 */
const searchForm = ref({
  // 站点
  site: 'all',
  // 关键词
  keyword: '',
});

const search = async () => {
  if (searchForm.value.site == '' || searchForm.value.site == 'all') {
    // const apis: Array<String> = siteInfo.value.filter(item=> item.isActive&&item.status=='可用').map(item => item.api);
    // appState.searchResults = await invoke('search_by_all_site', {siteInfo: apis, keyword: searchForm.value.keyword});

  } else {
  }
  appState.searchResults = await invoke('search_by_site', {
    site: searchForm.value.site,
    keyword: searchForm.value.keyword
  });
};

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

<style>

.form-row {
  display: flex;
  justify-content: space-between;
}

.form-item {
  flex: 1;
  margin-right: 10px;
}

.card-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.el-card {
  height: 100%; /* 设置卡片高度为100% */
  position: relative;
}

.el-image {
  width: 100%; /* 设置图片宽度为100% */
  height: auto; /* 根据宽度自动调整高度，保持图片比例 */
}

span {
  font-size: 14px;
  line-height: 1.5;
}

</style>