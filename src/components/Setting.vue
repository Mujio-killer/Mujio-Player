<template>
  <div class="settings">
    <el-form inline status-icon style="display: block">
      <div class="form-row">
        <el-form-item>
          <el-tooltip placement="top" content="检测资源是否有效">
            <el-button type="primary" @click="check">检测</el-button>
          </el-tooltip>
        </el-form-item>
        <el-form-item class="form-item" style="margin-left: 20px">
          <el-tooltip placement="top" content="导入json格式站点配置">
            <el-button type="primary" @click="importDialogVisible = true">导入</el-button>
          </el-tooltip>
        </el-form-item>
        <el-form-item class="form-item" style="position: relative; float: right">
          <el-tooltip placement="top" content="点击查看全部站点JSON配置">
            <el-button class="m-4" @click="queryAll">查看全部</el-button>
          </el-tooltip>
        </el-form-item>
      </div>
    </el-form>
    <div>
      <el-dialog
          v-model="importDialogVisible"
          style="min-width: 45%;"
          draggable
      >
        <el-collapse>
          <el-collapse-item title="点击查看示例" name="json">
            <el-card><pre><code>{{ siteFormat }}</code></pre></el-card>
          </el-collapse-item>
        </el-collapse>

        <el-input
            v-model="addSite"
            :autosize="{ minRows: 4, maxRows: 4 }"
            type="textarea"
            placeholder="请输入JSON数组"
        />
        <template #footer>
          <span class="dialog-footer">
            <el-button @click="importDialogVisible = false">取消</el-button>
            <el-button type="primary" @click="doImport">
              保存
            </el-button>
          </span>
        </template>
      </el-dialog>

      <el-dialog
          v-model="exportDialogVisible"
          style="min-width: 30%; overflow-x: scroll"
          draggable
          :show-header="false"
      >
            <el-card style="overflow-y: auto"><pre><code>{{ appState.siteList }}</code></pre></el-card>
      </el-dialog>
    </div>
    <div style="height: 100%; display: block">
        <el-table
            :data="siteList"
            style="width: 100%; height: 95%">
          <el-table-column prop="name" label="站点名称"></el-table-column>
          <el-table-column prop="api" label="链接"></el-table-column>
          <el-table-column prop="group" label="视频"></el-table-column>
          <el-table-column prop="isActive" label="启用">
            <template #default="{ row }">
              <el-switch v-model="row.isActive" @change="updateSiteList"/>
            </template>
          </el-table-column>

          <el-table-column prop="status" label="状态">
            <template #default="{ row }">
              <el-text class="mx-1" :type="getStatus(row)">{{ row.status }}</el-text>
            </template>
          </el-table-column>

          <el-table-column
              label="操作">
            <template #default="{ row }">
              <el-button
                  type="danger"
                  size="mini"
                  @click="deleteSite(row)">
                删除
              </el-button>
            </template>
          </el-table-column>
        </el-table>
    </div>
  </div>
</template>

<script setup lang="ts">
import {useAppStateStore} from "../stores";
import {ref} from "vue";
import {getSiteInfo, saveSiteInfo} from "../utils/SiteInfoUtil";
import { ElMessage } from 'element-plus';
import {checkChannel} from "../utils/SiteUtil";
import {SiteInfo} from "../utils/types";

const appState = useAppStateStore();

const siteList = ref(appState.siteList);
const importDialogVisible = ref(false);
const exportDialogVisible = ref(false);
const addSite = ref([] as SiteInfo[])

const siteFormat = `[
  {
    "id": 1,
    "key": "39kan",
    "name": "39影视",
    "api": "https://www.39kan.com/api.php/provide/vod/at/xml",
    "jiexiUrl": "",
    "group": "影视",
    "isActive": true,
    "status": "可用",
    "reverseOrder": false
  },
  {
    "id": 9,
    "key": "apibdzy",
    "name": "百度云资源",
    "api": "https://api.apibdzy.com/api.php/provide/vod/at/xml",
    "jiexiUrl": "",
    "group": "影视",
    "isActive": true,
    "status": "可用",
    "reverseOrder": false
  }]`;
const doImport = () => {
  appState.siteList=[...appState.siteList,...addSite.value];
  addSite.value = [];
  try {
    saveSiteInfo(appState.siteList);
  }catch (err) {
    console.log(err);
    ElMessage.error('保存失败:', err);
  }

  importDialogVisible.value = false;
  ElMessage({
    message: '保存成功',
    type: 'success',
  })

}

const checkingSite = ref('');
const getStatus = (site)=> {
  if(checkingSite.value === site.api){
    return 'warning';
  }
  return site.status==="可用"?"success":"danger";
}


const check = () => {
  appState.siteList.forEach(site=>{
    const status = checkChannel(site.api)?"可用":"失效";
    console.log("name:{},status:{}", site.name, site.status)
    site.status = status;
  });
  updateSiteList();
  ElMessage({
    message: '已更新表格状态',
    type: 'success',
  })
}

const deleteSite = (row) => {
  try {
    appState.siteList = appState.siteList.filter((item) => item.api !== row.api);
    updateSiteList();
  }catch (err) {
    console.log(err);
    ElMessage.error('保存失败:', err);
  }
  ElMessage({message: '操作成功', type: 'success'});
}

const updateSiteList = async () => {
  await saveSiteInfo(appState.siteList);
  appState.siteList = await getSiteInfo();
}

const queryAll = async () => {
  appState.siteList = await getSiteInfo();
  exportDialogVisible.value = true
}
</script>

<style>
.el-popper.is-customized {
  /* Set padding to ensure the height is 32px */
  padding: 6px 12px;
  background: linear-gradient(90deg, rgb(159, 229, 151), rgb(204, 229, 129));
}

.el-popper.is-customized .el-popper__arrow::before {
  background: linear-gradient(45deg, #b2e68d, #bce689);
  right: 0;
}

</style>
