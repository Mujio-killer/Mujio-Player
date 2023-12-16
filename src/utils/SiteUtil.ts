import {fetchRss} from './CommonUtil';
import {ClassInfo, Dd, Episode, QueryOptions, VideoInfo, VideoResource} from './types';

/**
 * 读取网站分类、分页信息
 * @param siteApi
 */
export async function getSiteClass(siteApi: string): Promise<ClassInfo[]> {
    const xmlData: string = await fetchRss(siteApi);
    const parser = new DOMParser();
    const xmlDoc = parser.parseFromString(xmlData, 'text/xml');

    const siteInfoData = xmlDoc.querySelector('rss') ? xmlDoc.querySelector('rss') : xmlDoc;

    // console.log("siteInfoData", siteInfoData);
    // 先解析分类信息
    return Array.from(siteInfoData.querySelectorAll('ty'))
        .map(item => {
            return {
                tid: item.getAttribute('id'),
                name: item?.textContent ?? ''
            }
        });
}

/**
 * 搜索
 * @param options
 */
export async function doSearch(options: QueryOptions): Promise<VideoResource> {
    var queryUrl = getQueryUrl(options);
    // console.log("queryUrl:", queryUrl);
    const xmlData = await fetchRss(queryUrl);
    const parser = new DOMParser();
    const xmlDoc = parser.parseFromString(xmlData, 'text/xml');

    const siteInfoData = xmlDoc.querySelector('rss') ? xmlDoc.querySelector('rss') : xmlDoc;

    const list = siteInfoData.querySelector('list');
    if (!list) {
        return null;
    }

    const videoList: VideoInfo[] = Array.from(list.querySelectorAll('video')).map(item => {
        // 解析资源信息
        const ddTage = item.querySelectorAll("dl > dd");
        const ddList: Dd[] = Array.from(ddTage).map(ddTag => {
            const dd: Dd = {
                flag: ddTag.getAttribute('flag'),
                list: parseEpisodes(ddTag?.textContent ?? '')
            }
            return dd;
        });

        // 完整video信息
        const videoInfo: VideoInfo = {
            last: item.querySelector('last')?.textContent ?? '',
            id: item.querySelector('id')?.textContent ?? '',
            tid: item.querySelector('tid')?.textContent ?? '',
            name: parseStr(item.querySelector('name')?.textContent ?? ''),
            type_str: item.querySelector('type')?.textContent ?? '',
            pic: item.querySelector('pic')?.textContent ?? '',
            lang: item.querySelector('lang')?.textContent ?? '',
            area: parseStr(item.querySelector('area')?.textContent ?? ''),
            year: item.querySelector('year')?.textContent ?? '',
            state: item.querySelector('state')?.textContent ?? '',
            note: parseStr(item.querySelector('note')?.textContent ?? ''),
            actor: parseStr(item.querySelector('actor')?.textContent ?? ''),
            director: parseStr(item.querySelector('director')?.textContent ?? ''),
            desc: parseStr(item.querySelector('des')?.textContent ?? ''),
            dl: ddList,
        }
        return videoInfo;
    });

    // 完整结果
    return {
        page: Number(list.getAttribute('page')),
        pageCount: Number(list.getAttribute('pagecount')),
        pageSize: Number(list.getAttribute('pagesize')),
        recordCount: Number(list.getAttribute('recordcount')),
        list: videoList
    };
}

/**
 * 根据参数构造请求URL
 * @param options
 */
function getQueryUrl(options: QueryOptions): string {
    let url = `${options.api}?ac=videolist`;

    if (options.pg) {
        url += `&pg=${options.pg}`;
    }

    if (options.ps) {
        url += `&ps=${options.ps}`;
    }

    if (options.t) {
        url += `&t=${options.t}`;
    }

    if (options.wd) {
        url += `&wd=${options.wd}`;
    }
/*
    if (options.ct) {
        url += `&ct=${options.ct}`;
    }*/


    if (options.ids) {
        url += `&ids=${options.ids}`;
    }

    return url;
}

function parseEpisodes(episodeStr: string): Episode[] {
    // 去掉注释
    episodeStr = parseStr(episodeStr);

    // 截取出剧集信息
    return episodeStr.split('#').map(item => {
        const episodeInfoArr = item.split('$');
        const episode: Episode = {
            name: episodeInfoArr[0],
            link: episodeInfoArr[1]
        }
        return episode;
    });
}

/**
 * 解析字符串中的内容，去掉注释和两端空格
 * @param str
 */
function parseStr(str: string): string {
    return str
        // 去掉描述中的特殊标签
        .replace("<p-->", '')
        // 去掉注释
        .replace("<!--[CDATA[", '')
        .replace("]]", '')
        .replace("-->", '')
        // 去掉特殊html标签
        .replace(/<[^>]+>/g, '')
        // 去掉html的&gt;等特殊字符
        .replace(/&[a-zA-Z]+;/g, '')
        .replace("]]>", '')
        .trim();
}





