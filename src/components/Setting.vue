<template>
  <div class="settings">
    <el-table
        :data="appState.siteInfo"
        style="width: 100%">
      <el-table-column prop="name" label="播放源名称"></el-table-column>
      <el-table-column prop="group" label="视频"></el-table-column>
      <el-table-column prop="isActive" label="启用"></el-table-column>
      <el-table-column prop="status" label="可用"></el-table-column>
      <el-table-column
          label="操作">
        <template #default="{ row }">
          <el-button
              type="danger"
              size="mini"
              @click="deletePlaySource(row)">
            删除
          </el-button>
        </template>
      </el-table-column>
    </el-table>
    <el-button
        type="primary"
        @click="addPlaySource">
      添加播放源
    </el-button>
  </div>
</template>

<script setup lang="ts">
import {useAppStateStore} from "../stores";

const appState = useAppStateStore();

const addPlaySource = () => {
  // 弹出对话框或跳转到添加播放源页面
}
const deletePlaySource = (row) => {
  // 从playSources数组中删除指定的播放源
  // const index = playSources.indexOf(row);
  // if (index !== -1) {
  //   playSources.splice(index, 1);
  // }
}
</script>