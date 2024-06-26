import React, {useState} from 'react'
import {router} from 'expo-router'
import Animated, {useSharedValue, withTiming, useAnimatedStyle, Easing} from "react-native-reanimated"
import { Text, View, Image, StyleSheet, useWindowDimensions, ScrollView } from "react-native";
import Logo from '../assets/images/music-app.png';
import CustomInput from '../components/CustomInput'
import CustomButton from '../components/CustomButton'

const onSignInPressed = () => {
  console.warn("Signed in");
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
    <ScrollView showsVerticalScrollIndicator={false}>
    <View style = {styles.root}>
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
        onPress={onSignInPressed}/>
        
      <CustomButton 
        text={"Don't have an account?"} 
        onPress={onNoAccountPressed}
        type = "TERTIARY"/>
    </View>
    </ScrollView>
  );
}

const styles = StyleSheet.create({
  root:{
    alignItems: "center",
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
});