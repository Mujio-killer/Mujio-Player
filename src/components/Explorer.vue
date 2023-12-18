<template>
  <div class="explorer-container">
    <el-form inline status-icon style="display: block">
      <div class="form-row">
        <el-form-item label="站点" class="form-item">
          <el-select v-model="queryOptions.api" value-key="id" placeholder="请选择站点"
                     default-first-option
                     clearable placement="bottom"
                     @change="changeSite"
          >
            <el-option
                v-for="item in appState.siteList"
                :key="item.api"
                :label="item.name"
                :value="item.api"
            />
          </el-select>
        </el-form-item>
        <el-form-item label="关键词" class="form-item">
          <el-input v-model="queryOptions.wd" placeholder="请输入关键词"></el-input>
        </el-form-item>

        <el-form-item class="form-item">
          <el-button type="primary" @click="search">搜索</el-button>
        </el-form-item>
      </div>
    </el-form>
    <div style="height: 100%; display: block">
      <el-table :data="searchResult.list"
                :default-sort="{ prop: 'name', order: 'descending' }"
                highlight-current-row
                style="line-height: 60px; width: 100%; border: 0; overflow: auto;"
                max-height="80%"
                table-layout="fixed">
        <!-- 行展开展示详情 -->
        <el-table-column type="expand">
          <template #default="props">

            <div style="margin-left: 15px; margin-right: 15px; border: 0">
              <el-card class="box-card">
                <div style="display: flex">
                  <div>
                    <img :src="props.row.pic" class="image" style="width: 200px; height: 300px"/>
                  </div>
                  <div style="margin-left: 20px">
                  <span>
                    <p><b>名称:  </b>{{ props.row.name }}</p>
                    <p><b>类型:  </b>{{ props.row.type_str }}</p>
                    <p><b>导演:  </b>{{ props.row.director }}</p>
                    <p><b>主演:  </b>{{ props.row.actor }}</p>
                  </span>
                  </div>
                </div>
                <div>
                  <span><p><b>简介:  </b></p><p v-html="props.row.desc"/></span>
                </div>

                <div>
                  <span><h3>播放资源</h3></span>

                  <el-tabs type="border-card" style="margin-top: 10px">
                    <!-- 分资源类型展示 -->
                    <el-tab-pane v-for="dd in props.row.dl" :key="dd.flag" :label="dd.flag">
                      <el-radio-group v-model="appState.currentSrc.dd.flag">
                        <el-radio-button v-for="episode in dd.list" :key="episode.link"
                                         :label="episode.name" border
                                         @click="playVideo(props.row.name, dd, episode)">
                          {{ episode.name }}
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

      <!-- 分页组件 -->
      <el-pagination
          style="min-height: 10%"
          background
          layout="prev, pager, next"
          :page-size="queryOptions.ps??20"
          :current-page="queryOptions.pg??1"
          :total="searchResult.recordCount??0"
          class="mt-4"
          @current-change="pageChange"
      />
    </div>
  </div>

</template>

<script setup lang="ts">
import {onMounted, ref} from 'vue';
import {useAppStateStore} from "../stores";
import {Dd, Episode} from "../utils/types";
import {doSearch} from "../utils/SiteUtil";

const appState = useAppStateStore();

// 查询参数
const queryOptions = ref(appState.queryOption);
// 查询结果
const searchResult = ref(appState.searchResult);

onMounted(() => {
  console.log('ALlsiteList:', appState.siteList)
  search();
});

const search = () => {
  doSearch(queryOptions.value).then(res => {
    if (res) {
      searchResult.value = res;
    }
    console.log('搜索结果：', res)
  }).catch(err => {
    console.error('搜索出现错误：', err);
  });
}

// 动态设置选中的资源
const toggleExpand = (row: any) => {
  row.expanded = !row.expanded;
};

// 播放选中的资源
const playVideo = (name: string, dd: Dd, episode: Episode) => {
  appState.currentSrc.name = name;
  appState.currentSrc.dd = dd;
  appState.currentSrc.episode = episode;
  appState.view = '2';
}

const changeSite = () => {
  queryOptions.value.pg = 1;
  search();
}
const pageChange = (page: number) => {
  queryOptions.value.pg = page;
  search();
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

.el-image {
  width: 100%; /* 设置图片宽度为100% */
  height: auto; /* 根据宽度自动调整高度，保持图片比例 */
}

span {
  font-size: 14px;
  line-height: 1.5;
}


</style>