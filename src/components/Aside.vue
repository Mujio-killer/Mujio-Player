<template>
    <el-row class="tac">
      <el-col :span="12">
        <el-menu
        active-text-color="#ffd04b"
        background-color="#545c64"
        class="el-menu-vertical-demo"
        default-active="1"
        text-color="#fff"
          @select="changeView"
        >
            <el-menu-item index="1">
              <el-icon><Search /></el-icon>
              <span>搜索</span>
            </el-menu-item>

            <el-menu-item index="2">
              <el-icon><VideoPlay /></el-icon>
              <span>播放</span>
            </el-menu-item>

            <el-menu-item index="3">
              <el-icon><Clock /></el-icon>
               <span>历史</span>
            </el-menu-item>

            <el-menu-item index="4">
                <el-icon><Star /></el-icon>
               <span>收藏</span>
            </el-menu-item>

            <el-menu-item index="5">
              <el-icon><Setting /></el-icon>
              <span>设置</span>
            </el-menu-item>

        </el-menu>
      </el-col>
    </el-row>
</template>


<script lang="ts" setup>
import {Search, VideoPlay, Clock, Star, Setting } from '@element-plus/icons-vue'
import {useAppStateStore} from "./../stores";
import {ref} from 'vue';

const menuIndex = ref("1");
const appState = useAppStateStore();

const changeView = (idx:string) => {
  console.log("current idx="+idx);
  menuIndex.value = idx;
  setView(idx);
}

const setView = (idx: string) => {
    appState.setView(idx);
}

</script>
<style>
.el-menu-vertical-demo:not(.el-menu--collapse) {
  width: 100px;
  min-height: 400px;
}
</style>