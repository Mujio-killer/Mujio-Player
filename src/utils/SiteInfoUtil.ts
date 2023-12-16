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


