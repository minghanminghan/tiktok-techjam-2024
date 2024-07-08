import React, {useState, useEffect} from 'react';
import { View, Text, StyleSheet, Image, useWindowDimensions } from 'react-native';
import {SafeAreaView} from 'react-native-safe-area-context';

import { playListData } from '@/assets/songs/playListData'; //change this to db

import MusicCard from '@/components/MusicCard';
import WebPlayback from '@/components/WebPlayback';
import { SpotifyApi } from '@spotify/web-api-ts-sdk'; 

import Animated, {
  useSharedValue,
  useAnimatedStyle,
  withSpring,
  withTiming,
  useDerivedValue,
  interpolate,
  runOnJS
} from 'react-native-reanimated';

import {
  Gesture,
  GestureDetector,
  GestureHandlerRootView,
} from 'react-native-gesture-handler';

const ROTATION = 35;
const SWIPE_VELOCITY = 1000;

export default function TabTwoScreen() {
  //to handle the current index of the song being played
  const [currentIndex, setCurrentIndex] = useState(0);
  const [nextIndex, setNextIndex] = useState(currentIndex+1); 
  const currentSong = playListData[currentIndex];
  const nextSong = playListData[nextIndex];

  const {width: screenWidth} = useWindowDimensions();
  const hiddenTranslateX = 2*screenWidth;

  const translateX = useSharedValue<number>(0);
  const rotate = useDerivedValue(
    () => interpolate(translateX.value, [0, screenWidth], [0,ROTATION]) + 'deg')

  const pressed = useSharedValue<boolean>(false);

  const pan = Gesture.Pan()
    .onBegin(() => {
      pressed.value = true;
    })
    .onChange((event) => {
      translateX.value = event.translationX;
    })
    .onFinalize((event) => {
      if (Math.abs(event.velocityX) < SWIPE_VELOCITY) {
        translateX.value = withSpring(0);
      }
      else {
        translateX.value = withSpring(
          event.velocityX > 0 ? hiddenTranslateX: -hiddenTranslateX,
          {},
          ()=>{
            runOnJS(setCurrentIndex)(currentIndex+1);
          }
        )
      }
      pressed.value = false;
      
    });

/*
    const spotifyConfig: ApiConfig = {
        clientID: "577571ab029843d58e72fb448a256c58",
        redirectURL: "SPOTIFY_REDIRECT_URL", //fill these urls out
        tokenRefreshURL: "SPOTIFY_TOKEN_REFRESH_URL",
        tokenSwapURL: "SPOTIFY_TOKEN_SWAP_URL",
        scope: ApiScope.AppRemoteControlScope | ApiScope.UserFollowReadScope
    }

    async function playEpicSong() {
      try {
        //]const session = await SpotifyAuth.authorize(spotifyConfig);
        //await SpotifyRemote.connect(session.accessToken);
        //await SpotifyRemote.playUri("spotify:track:6IA8E2Q5ttcpbuahIejO74");
        //await SpotifyRemote.seek(58000);
      } catch (err) {
        console.error("Couldn't authorize with or connect to Spotify", err);
    }}
*/


async function callback(ctx) {
  const CLIENT_ID = '577571ab029843d58e72fb448a256c58';
  const CLIENT_SECRET = 'f1ddbccf106b4923a8c92334c1b08f99';
  const code = ctx['code'] || null;
  const authToken = btoa(`${CLIENT_ID}:${CLIENT_SECRET}`);

  const response = await fetch('https://accounts.spotify.com/api/token', {
    method: 'POST',
    mode: 'cors',
    cache: 'no-cache',
    headers: {
      'Authorization': `Basic ${authToken}`,
      'Content-Type': 'application/x-www-form-urlencoded'
    },
    body: JSON.stringify({
      code: code,
      redirect_uri: '/explore',
      grant_type: 'authorization_code'
    })
  });

  if (response.status !== 200) {
    console.log(`Bad response: ${response.statusText}`);
    ctx.response.body = JSON.stringify({
      'type': 'error',
      'error': 'Error while authorizing Spotify'
    });
    ctx.response.type = 'application/json';
    return;
  }

  const { access_token } = await response.json();

  ctx.cookies.set('access_token', access_token);
  ctx.response.redirect(`http://localhost:${PORT}/`);
}

    useEffect(()=> {
      translateX.value = 0;
      console.log(callback(null))
      setNextIndex(currentIndex+ 1);
    }, [currentIndex, translateX])

    const CardStyles = useAnimatedStyle(() => ({
      transform: [
        { translateX: translateX.value },
        { scale: withTiming(pressed.value ? 1.1 : 1) },
        { rotate: rotate.value}
      ],
    }));

    const nextCardStyles = useAnimatedStyle(() => ({
      transform: [
        { scale: interpolate(
          translateX.value, [-hiddenTranslateX,0,hiddenTranslateX], [1,0.8,1]) },
      ],
      opacity: interpolate(
        translateX.value, [-hiddenTranslateX,0,hiddenTranslateX], [1,0.6,1]
      )
    }));

  return (
    <View style = {{flex:1}}>
        {nextSong && <Animated.View style = {[nextCardStyles,styles.nextCardContainer]}>
        <MusicCard 
          image={nextSong.artwork}
          title={nextSong.title}
          artist={nextSong.artist}
          />
          </Animated.View> }
        {currentSong && <GestureHandlerRootView style = {styles.root}>
      <GestureDetector gesture={pan}>
        <Animated.View style = {[CardStyles,styles.currentCardContainer]}> 
        <MusicCard 
          image={currentSong.artwork}
          title={currentSong.title}
          artist={currentSong.artist}/>
        </Animated.View>
      </GestureDetector>
    </GestureHandlerRootView>  }
    </View>
  );
}

const styles = StyleSheet.create({
  root:{
    flex:1,
    alignItems: "center",
    padding: 20,
    justifyContent:'flex-start'
  },
  currentCardContainer:{
    justifyContent: 'center',
    alignItems:'center',
    flex:1,
  },
  nextCardContainer:{
    ...StyleSheet.absoluteFillObject,
    justifyContent: 'center',
    alignItems:'center',
  }
})