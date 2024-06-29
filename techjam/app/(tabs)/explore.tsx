import Ionicons from '@expo/vector-icons/Ionicons';
import { View, Text, StyleSheet, Image, Platform } from 'react-native';
import {SafeAreaView} from 'react-native-safe-area-context';
import ncslogo from '../../assets/images/ncslogo'

import { Collapsible } from '@/components/Collapsible';
import { ExternalLink } from '@/components/ExternalLink';
import ParallaxScrollView from '@/components/ParallaxScrollView';
import { ThemedText } from '@/components/ThemedText';
import { ThemedView } from '@/components/ThemedView';

export default function TabTwoScreen() {
  return (
    <SafeAreaView style = {{flex:1}}>
    <View style = {styles.root}>
      <Text style = {styles.title}> Explore Music! </Text>
      <Image source={ncslogo} style={styles.image}></Image>
    </View>
    </SafeAreaView>
  );
}

const styles = StyleSheet.create({
  root:{
    alignItems: "center",
    padding: 20,
    flex: 1,
    backgroundColor: 'F9FBFC'
  },
  title:{
    fontSize:24,
    fontWeight:'bold',
    color:'#051c60',
    margin: 10,
  },
  image:{
    width: '60%',
    height: '40%',
    marginVertical: 15,
    resizeMode:'contain',
  }
});
