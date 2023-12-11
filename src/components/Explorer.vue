<template>
  <div class="explorer-container">
    <el-form :data="searchForm" inline status-icon style="height: 6%; display: block">
      <div class="form-row">
        <el-form-item label="站点" class="form-item">
          <el-select v-model="searchForm.site" value-key="id" placeholder="请选择站点" clearable placement="bottom">
            <el-option label="全站" value="all"/>
            <el-option
                v-for="item in siteInfo"
                :key="item.id"
                :label="item.name"
                :value="item"
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
              style="line-height: 60px; width: 100%; border: 0; overflow: auto; margin-top: 20px "
              height="95%"
              table-layout="fixed"
    >
      <!-- 行展开展示详情 -->
      <el-table-column type="expand">
        <template #default="props">

          <div style="margin-left: 5%; margin-right: 5%; border: 0">
            <el-card class="box-card">
              <el-row :gutter="12" style="border: #e8e9e8 1px solid">
                <el-col :span="8">
                    <el-image style="width: 200px" :src="props.row.pic"/>
                </el-col>
                <el-col :span="16">
                    <p><b>名称:  </b>{{ props.row.name }}</p>
                    <p><b>类型:  </b>{{ props.row.type_str }}</p>
                    <p><b>导演:  </b>{{ props.row.director }}</p>
                    <p><b>主演:  </b>{{ props.row.actor }}</p>
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
                                       :label="videoEpisode.name" border @click="playVideo(props.row.name, videoSrc, videoEpisode)">
                        {{ videoEpisode.name }}
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
      <el-table-column sortable prop="name" label="名称"></el-table-column>
      <el-table-column prop="lang" label="语言"></el-table-column>
      <el-table-column sortable prop="year" label="上映时间"></el-table-column>
      <el-table-column prop="note" label="备注"></el-table-column>
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
  site: {
    id: 0,
    name: 'all',
    api:  ""
  },
  // 关键词
  keyword: '',
});

const search = async () => {
  appState.searchResults = await invoke('search_by_site', {
    site: searchForm.value.site.api,
    keyword: searchForm.value.keyword
  });
};

// 动态设置选中的资源
const toggleExpand = (row: any) => {
  row.expanded = !row.expanded;
};

const selectedEpisode = ref();
// 播放选中的资源
const playVideo = (name: String, videoSrc: any, episode: any) => {
  appState.selectedEpisode.type = videoSrc.flag;
  appState.selectedEpisode.episodes = videoSrc.episodes;
  appState.selectedEpisode.name = name;
  appState.selectedEpisode.currentEpisode = episode.name;
  appState.selectedEpisode.url = episode.link;

  console.log(searchForm.value.site)
  console.log(searchForm.value.site.name)
  appState.selectedEpisode.siteName = searchForm.value.site.name;
  appState.view = '2';
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