import {invoke} from "@tauri-apps/api";


// 在Vue3中调用Tauri函数获取XML信息
export const fetchRss = async (url: string): Promise<string> => {
    try {
        // 调用Tauri函数fetch_rss，传入指定的RSS资源网址
        return await invoke("fetch_rss", {url: url});
    } catch (error) {
        console.error('Failed to fetch RSS data:', error);
    }
}

