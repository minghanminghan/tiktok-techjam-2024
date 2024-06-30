import React, {useState, useEffect} from 'react';
import { View, Text, StyleSheet, Image, useWindowDimensions } from 'react-native';
import {SafeAreaView} from 'react-native-safe-area-context';

import { playListData } from '@/assets/songs/playListData';

//import TrackPlayer from 'react-native-track-player';

import MusicCard from '@/components/MusicCard';

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
const SWIPE_VELOCITY = 800;

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

    useEffect(()=> {
      translateX.value = 0;
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
});
