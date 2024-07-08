import React, {useState} from 'react'
import {router} from 'expo-router'
import Animated, {useSharedValue, withTiming, useAnimatedStyle, Easing} from "react-native-reanimated"

import {SafeAreaView} from 'react-native-safe-area-context';
import { Text, View, Image, StyleSheet, useWindowDimensions, ScrollView } from "react-native";
import Logo from '../assets/images/music-app.png';
import CustomInput from '../components/CustomInput';
import CustomButton from '../components/CustomButton';

import TrackPlayer from 'react-native-track-player';

// this is just hardcoded for now so that when you input username and password
// it logs in to the home page

const hardUsername = "Username"
const hardPassword = "Password"
//const [SuccessMessage, setSuccessMessage] = useState('');

const onSignInPressed = async (username:string, password:string) => {
  console.log(`button pressed`)
  const response = await fetch('https://https://harmonizeapp.net/api/v1/login', {
    method: 'POST', 
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({
      'username':username,
      'password':password,
    }),
  })
  .then(response => response.json())
  .then(data => {
    console.log(`Successful Login`)
    console.log(data)
    router.navigate('/explore')
  })
  .catch(error => {
    console.log(`Error: ${error}`);
    router.navigate("/SignInError");
  });

  // temporary sign in, remove when api's get fixed
  router.navigate('/explore');
}

const onForgotPasswordPressed = () => {
  console.warn("Forgot Password");
}

const onNoAccountPressed = () => {
  console.warn("User has no account");
  router.push("/SignUpScreen");
}

export default function index() {
  const [username, setUsername] = useState('');
  const [password, setPassword] = useState('');
  const {height} = useWindowDimensions();
  return (
    <SafeAreaView style = {{flex:1}}>
      <ScrollView showsVerticalScrollIndicator={false} >
    <View style = {styles.root}>
    <Text style = {styles.title}>
        Welcome! Sign in to your account. 
      </Text>
      <Image 
        source = {Logo} 
        style={styles.logo}>
      </Image>

      <CustomInput 
        placeholder={"Username"} 
        value = {username} 
        setValue = {setUsername} 
        secureTextEntry={false}/>

      <CustomInput
        placeholder={"Password"} 
        value = {password} 
        setValue = {setPassword} 
        secureTextEntry={true}/>

      <CustomButton 
        text={"Sign In"} 
        onPress={()=>{onSignInPressed(username, password)}}/>
        
      <CustomButton 
        text={"Don't have an account?"} 
        onPress={onNoAccountPressed}
        type = "TERTIARY"/>
    </View>
    </ScrollView>
    </SafeAreaView>
  );
}

const styles = StyleSheet.create({
  root:{
    alignItems: 'center',
    padding: 20,
    flex: 1,
    backgroundColor: 'F9FBFC'
  },
  logo:{
    width: '60%',
    //maxWidth: 300,
    height: '40%',
    //maxHeight: 200,
    marginVertical: 15,
    resizeMode:'contain',
  },
  title:{
    fontSize:24,
    fontWeight:'bold',
    color:'#051c60',
    margin:10,
  },
});