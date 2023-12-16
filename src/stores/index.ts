import {defineStore} from 'pinia';
import {getSiteInfo} from "../utils/SiteInfoUtil";
import {Dd, Episode, QueryOptions, SiteInfo, VideoResource} from "../utils/types";

export const useAppStateStore = defineStore("appState", {
    state: () => ({
        view: '1',
        queryOption: {
            api: '',
            ct: 1,
            ids: 0,
            pg: 1,
            ps: 20,
            t: '',
            wd: '',
        } as QueryOptions,
        siteList: [] as SiteInfo[],
        selectedSite: "" as string,
        searchResult: {
            recordCount:0,
            list: []
        } as VideoResource,
        currentSrc: {
            site: {} as SiteInfo,// 站点信息
            dd: {} as Dd,// 当前选择的资源
            episode: {} as Episode,// 当前播放的剧集
            name: "",// 当前资源名称
        },
        playerState: {
            currentTime: 0,
            currentVolume: 0.7
        },
        playHistoryData: [] as Array<any>

    }),
    getters: {
        getView(state): string {
            return state.view.toString(); // 确保返回一个字符串
        }
    },
    actions: {
        async initializeGlobalVariable() {
            // 在这里进行全局变量的初始化
            this.siteList = await getSiteInfo();
            this.queryOption.api = this.siteList[0].api;
            this.selectedSite = this.siteList[0].name;
        },
    },


});
