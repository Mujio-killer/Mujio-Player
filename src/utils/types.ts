
/**
 * 站点相关
 */
export class SiteInfo {
    id: number;
    key?: string;
    name?: string;
    api: string;
    jiexiUrl?: string;
    group?: string;
    isActive: boolean;
    status?: string;
    reverseOrder?: boolean;
}

/**
 * 分类
 */
export interface ClassInfo {
    tid: string;
    name: string;
}

/**
 *  资源信息
 */
export interface Episode {
    name?: string,
    link?: string,
}

/**
 *  资源信息
 */
export interface Dd {
    flag?: string,
    list?: Episode[],
}

/**
 *  资源信息
 */
export interface VideoInfo {
    last?: string,
    id: string,
    tid: string,
    name: string,
    type_str?: string,
    pic?: string,
    lang?: string,
    area?: string,
    year?: string,
    state?: string,
    note?: string,
    actor?: string,
    director?: string,
    desc?: string,
    dl?: Dd[],
}

/**
 * 查询结果
 */
export interface VideoResource {
    page: number,
    pageCount: number,
    pageSize: number,
    recordCount: number
    list: VideoInfo[]
}

/**
 * 查询参数
 */
export interface QueryOptions {
    api?: string,
    pg?: number, // 页码
    ps?: number, //  每页条数，暂无可用参数
    t?: string, // 资源类型，对应type
    wd?: string,// 搜索关键词
    ct?: 1, // 下载用参数
    ids?: number // 资源ID，下载用参数
}