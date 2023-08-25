import {defineStore} from 'pinia'

// 存放共享变量，记录当前状态等
// 这里的appState可以理解为命名空间
export const useAppStateStore = defineStore("appState", {
    state: () => ({
        view: 'Film',
        setting: {
            theme: 'light',
            site: 'zuidazy',
            view: 'picture',
            shortcut: true
        },
        detail: {
            show: false,
            key: '',
            info: {}
        },
        share: {
            show: false,
            key: '',
            info: {}
        },
        video: {
            key: '',
            info: {}
        },
        winState: {
            windowIsOnTop: false
        },
        detailCache: {},
    }),

    getters: {
        getView(state): string {
            return state.view
        },
        getSetting(state): any {
            return state.setting
        },
        getDetail(state): any {
            return state.detail
        },
        getShare(state): any {
            return state.share
        },
        getVideo(state): any {
            return state.video
        },
        getAppState(state): any {
            return state.winState
        },
        getDetailCache(state): any {
            return state.detailCache
        }
    },
    actions: {
        setView(payload: string) {
            this.view = payload
        },
        setSetting(payload: any) {
            this.setting = payload
        },
        setDetail(payload: any) {
            this.detail = payload
        },
        setShare(payload: any) {
            this.share = payload
        },
        setVideo(payload: any) {
            this.video = payload
        },
        setWinState(payload: any) {
            this.winState = payload
        },
        setDetailCache(payload: any) {
            this.detailCache = payload
        }
    }

})

