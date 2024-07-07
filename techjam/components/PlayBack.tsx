import TrackPlayer, { Event, RepeatMode } from 'react-native-track-player';

import { playListData } from '@/assets/songs/playListData';

export async function setUpPlayer() {
    let isSetup = false;
    try {
        await TrackPlayer.getActiveTrackIndex();
        isSetup = true;
    } catch (error) {
        await TrackPlayer.setupPlayer();
        isSetup = true;
    } finally {
        return isSetup;
    }
}

export async function addTracks() {
    await TrackPlayer.add(playListData)
    await TrackPlayer.setRepeatMode(RepeatMode.Queue)
}

export async function playbackService () {
    TrackPlayer.addEventListener(Event.RemotePause, () => {
        TrackPlayer.pause()
    })
    TrackPlayer.addEventListener(Event.RemotePlay, () => {
        TrackPlayer.play()
    })
    TrackPlayer.addEventListener(Event.RemoteNext, () => {
        TrackPlayer.skipToNext()
    })
    TrackPlayer.addEventListener(Event.RemotePrevious, () => {
        TrackPlayer.skipToPrevious()
    })

}