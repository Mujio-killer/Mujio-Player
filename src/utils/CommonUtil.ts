import {invoke} from "@tauri-apps/api";

/**
 * 读取本地json文件
 */
export const fetchJsonData = async (fileName: string) => {
    try {
        const response = await fetch(fileName);
        if (response.ok) {
            const jsonData = await response.json();
            return jsonData;
        } else {
            console.error('Failed to fetch JSON data from', fileName);
        }
    } catch (error) {
        console.error('Error fetching JSON data:', error);
    }
}

// 在Vue3中调用Tauri函数获取XML信息
export const fetchRss = async (url: string) => {
    try {
        // 调用Tauri函数fetch_rss，传入指定的RSS资源网址
        return await invoke("fetch_rss", {url: url});
    } catch (error) {
        console.error('Failed to fetch RSS data:', error);
    }
}

