import {fetchJsonData} from "./CommonUtil";
import {SiteInfo} from "./types";

/**
 * 读取站点信息
 */
export async function getSiteInfo(): Promise<Array<SiteInfo>> {
    const siteInfoFileName = "../dist/resource.json";
    try {
        const resp = await fetchJsonData(siteInfoFileName);
        return resp as SiteInfo[];
    } catch (err) {
        console.error('Failed to fetch SiteInfoUtil', err);
        return [];
    }
}

// 根据站点ID，修改站点信息
export async function updateSiteInfo(siteId: number, siteInfo: SiteInfo) {

}

// 根据站点ID，删除resource.json中对应的站点数据
export async function deleteSiteInfo(siteId: number) {

}

