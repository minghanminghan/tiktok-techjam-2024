import React, {useEffect, useState} from 'react';
import { View, Text, StyleSheet, Image, useWindowDimensions, ImageBackground } from 'react-native';
import TrackPlayer from 'react-native-track-player';
import { playbackService, setUpPlayer, addTracks } from './PlayBack';

import { playListData } from '@/assets/songs/playListData';

// AppRegistry.registerComponent(...);
TrackPlayer.registerPlaybackService(() => playbackService);

// service.js
module.exports = async function() {
  // This service needs to be registered for the module to work
  // but it will be used later in the "Receiving Events" section
}

const MusicCard = (props) => {

    const [playerReady, setPlayerReady] = useState(false)

    useEffect

    const {title, artist, image} = props;
    return (
        <View style={styles.rectangle}>
            <Image source={image}style={styles.image}></Image>
            <Text style={styles.title}> {title} </Text>
            <Text style={styles.artist}> {artist} </Text>

        </View>
        
    );
}

const styles = StyleSheet.create({
    rectangle: {
        width: 350,
        height: 600,
        backgroundColor:'white',
        borderRadius: 5,
        shadowColor: "#000",
        shadowOffset: {
            width: 0,
            height: 2,
        },
        shadowOpacity: 0.25,
        shadowRadius: 3.84,
        elevation: 5,
        padding: 15,
    },
    image: {
        width: '100%',
        height: '60%',
        display: 'flex',
        resizeMode: 'contain',
        marginHorizontal: 'auto',
        padding:10,
    },
    title: {
        marginHorizontal: 'auto',
        fontWeight: '400',
        marginVertical:5,
        fontSize:20,
    },
    artist: {
        marginHorizontal: 'auto',
        fontWeight: '200',
        fontSize:15
    }
})

export default MusicCard;