import React, {useState} from 'react';
import { View, Text, StyleSheet, Image, useWindowDimensions } from 'react-native';
import {SafeAreaView} from 'react-native-safe-area-context';

import { playListData } from '@/assets/songs/playListData';

import TrackPlayer from 'react-native-track-player';

import MusicCard from '@/components/MusicCard';

import Animated, {
  useSharedValue,
  useAnimatedStyle,
  withSpring,
  withTiming,
  useDerivedValue,
  interpolate
} from 'react-native-reanimated';

import {
  Gesture,
  GestureDetector,
  GestureHandlerRootView,
} from 'react-native-gesture-handler';

const ROTATION = 35;

export default function TabTwoScreen() {
  //to handle the current index of the song being played
  const [currentIndex, setCurrentIndex] = useState(0);
  const currentSong = playListData[currentIndex];
  const {width: screenWidth} = useWindowDimensions();
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
    .onFinalize(() => {
      translateX.value = withSpring(0);
      pressed.value = false;
    });

    const animatedStyles = useAnimatedStyle(() => ({
      transform: [
        { translateX: translateX.value },
        { scale: withTiming(pressed.value ? 1.1 : 1) },
        { rotate: rotate.value}
      ],
    }));

//<Animated.Image source={ncslogo} style={[styles.image, animatedStyles]} />
  return (
    <SafeAreaView style = {{flex:1}}>
    <GestureHandlerRootView style = {styles.root}>
      <Text style = {styles.title}> Explore Music! </Text>
      <GestureDetector gesture={pan}>
        <Animated.View style = {animatedStyles}> 
        <MusicCard 
          style = {animatedStyles} 
          image={currentSong.artwork}
          title={currentSong.title}
          artist={currentSong.artist}/>
        </Animated.View>
      </GestureDetector>
    </GestureHandlerRootView>
    </SafeAreaView>
  );
}

const styles = StyleSheet.create({
  root:{
    flex:1,
    alignItems: "center",
    padding: 20,
    backgroundColor: 'F9FBFC',
    justifyContent:'flex-start'
  },
  title:{
    fontSize:24,
    fontWeight:'bold',
    color:'#051c60',
    margin: 10,
  }
});
