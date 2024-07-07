import { Track } from "react-native-track-player";

export const playListData: Track[] = [
    {
        id: 1,
        title: 'FE!N',
        artist: 'Travis Scott',
        artwork: require('../images/fe!nlogo.png'),
        url: require('./FE!N.mp3'),
    },
    {
        id: 2,
        title: 'seasons',
        artist: 'wave to earth',
        artwork: require('../images/seasonslogo.png'),
        url: require('./seasons.mp3'),
    },
    {
        id: 3,
        title: 'LIKE THAT',
        artist: 'BABYMONSTER',
        artwork: require('../images/likethatlogo.png'),
        url: require('./likethat.mp3'),
    },
]