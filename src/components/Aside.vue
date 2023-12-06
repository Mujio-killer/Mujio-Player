<template>
  <el-menu
      active-text-color="#ff804b"
      class="el-menu-vertical-demo"
      :default-active="appState.view"
      text-color="#7b1199"
      @select="changeView"
  >
    <el-menu-item index="1">
      <el-icon size="25">
        <Search/>
      </el-icon>
    </el-menu-item>
    <el-menu-item index="2">
      <el-icon size="25">
        <VideoPlay/>
      </el-icon>
    </el-menu-item>
    <el-menu-item index="3">
      <el-icon size="25">
        <Clock/>
      </el-icon>
    </el-menu-item>
    <el-menu-item index="4">
      <el-icon size="25">
        <Setting/>
      </el-icon>
    </el-menu-item>
  </el-menu>
</template>

<script lang="ts" setup>
import {Clock, Search, Setting, VideoPlay} from '@element-plus/icons-vue'
import {useAppStateStore} from "./../stores";

const appState = useAppStateStore();

const changeView = (idx: string) => {
  appState.view = idx;
}
</script>
<style>
.el-menu-vertical-demo .el-menu-item.is-active {
  background: #e8e9e8;
}
.el-menu-vertical-demo {
  display: flex;
  flex-direction: column;
  height: 100%; /* 设置菜单高度为100% */
}

</style>