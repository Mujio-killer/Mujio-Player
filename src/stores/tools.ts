/**
 * 公共方法
 */
import { useAppStateStore } from "../stores";

const appState = useAppStateStore();
/**
 * 播放选择资源
 * @param videoSrc
 * @param episode
 */
const playVideo = (videoSrc: any, episode: any) => {
    appState.selectedEpisode.type = videoSrc.flag;
    appState.selectedEpisode.episode = episode;
    appState.selectedEpisode.episodes = videoSrc.episodes;
    appState.view = '2';
}

