# Mujio-Player

#### 介绍
尝试使用vue3重写zy
> 工具链：Vue3 + TypeScript + Pinia + Vite + Tauri
#### 软件架构
#### 功能点设计




#### 安装教程

1.  xxxx
2.  xxxx
3.  xxxx

#### 使用说明

1.  xxxx
2.  xxxx
3.  xxxx

#### 参与贡献

1.  Fork 本仓库
2.  新建 Feat_xxx 分支
3.  提交代码
4.  新建 Pull Request


#### 特技

1.  使用 Readme\_XXX.md 来支持不同的语言，例如 Readme\_en.md, Readme\_zh.md
2.  Gitee 官方博客 [blog.gitee.com](https://blog.gitee.com)
3.  你可以 [https://gitee.com/explore](https://gitee.com/explore) 这个地址来了解 Gitee 上的优秀开源项目
4.  [GVP](https://gitee.com/gvp) 全称是 Gitee 最有价值开源项目，是综合评定出的优秀开源项目
5.  Gitee 官方提供的使用手册 [https://gitee.com/help](https://gitee.com/help)
6.  Gitee 封面人物是一档用来展示 Gitee 会员风采的栏目 [https://gitee.com/gitee-stars/](https://gitee.com/gitee-stars/)

#### commit log

2023-09-01 尝试rust读写sqlite； 读取db文件失败解决方案：
1.  改依赖为bundled；
2.  检查文件路径
3.  修改文件夹读写权限(修改db文件依然报错，修改file文件夹并递归设为777后修复- chmod -R 777 file)