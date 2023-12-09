import {defineStore} from 'pinia';

export const useAppStateStore = defineStore("appState", {
    state: () => ({
        view: '1',
        siteInfo: [] as Array<any>, // 明确指定siteInfo为数组类型
        searchResults: [] as Array<any>, // 明确指定siteInfo为数组类型
        selectedEpisode: {
            type: "mp4",
            episode: {
                episode: "demo",
                link: "https://cdn.plyr.io/static/demo/View_From_A_Blue_Moon_Trailer-720p.mp4",
            },
            currentTime: 0,
            currentVolume: 0.7,
            srcName: "测试",
            episodes:[] as Array<any>
        }, // 明确指定siteInfo为数组类型
        plyrPlayer: {
            captions: { active: true, update: true, language: 'auto' },
            controls: [
                'play-large',// 播放图标
                'rewind',// 后退
                'play',// 播放
                'fast-forward',
                'progress',
                'current-time',
                'duration',
                'mute',// 静音
                'volume',
                'settings',
                'pip',
                'fullscreen',
            ],
            i18n: {
                speed: '速度',
                normal: '正常',
            },
            autoplay: false,
            seekTime: 1,
            // ratio: '16:9',
        },
        playHistoryData: [] as Array<any>

    }),
    getters: {
        getView(state): string {
            return state.view.toString(); // 确保返回一个字符串
        }
    },
    actions: {
        setView(payload: string) {
            this.view = payload; // 设置view的值
        },
        setSiteInfo(payload: Array<any>) {
            this.siteInfo = payload; // 设置siteInfo的值
        },
        setSearchResults(payload: Array<any>) {
            this.searchResults = payload; // 设置searchResults的值
        },
        setSelectedEpisode(payload: any) {
            this.selectedEpisode = payload;
        },
        setSelectedVideoSrc(payload: any) {
            this.selectedVideoSrc = payload;
        }
    }


});
