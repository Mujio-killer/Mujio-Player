import { SiteInfo } from "./types";
import { BaseDirectory, readTextFile, writeTextFile } from "@tauri-apps/api/fs";

const filePath = `_up_/data/resource.json`;

/**
 * 读取站点信息
 */
export async function getSiteInfo(): Promise<Array<SiteInfo>> {
    try {
        const resp = await readTextFile(filePath, { dir: BaseDirectory.Resource });
        const jsonData = JSON.parse(resp) as SiteInfo[];
        console.log("queryList:", resp);
        console.log("queryList2json:", jsonData);

        return jsonData;
    } catch (err) {
        console.error('Failed to fetch SiteInfoUtil', err);
        throw new Error("查询失败：" + err);
    }
}

export const saveSiteInfo = async (jsonData: any): Promise<any> => {
    try {
        await writeTextFile(filePath, jsonData);
        return jsonData;
    } catch (err) {
        console.error("写入JSON文件时出错:", err);
        throw new Error("查询失败：" + err);
    }
}