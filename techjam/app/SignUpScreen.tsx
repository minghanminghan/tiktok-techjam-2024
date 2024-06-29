import React, {useState} from 'react'
import {router} from 'expo-router'
import Animated, {useSharedValue, withTiming, useAnimatedStyle, Easing} from "react-native-reanimated"
import { Text, View, StyleSheet,  ScrollView } from "react-native";
import {SafeAreaView} from 'react-native-safe-area-context';

import CustomInput from '../components/CustomInput'
import CustomButton from '../components/CustomButton'




const onHaveAnAccountPressed = () => {
    router.back()
  }

export default function SignUpScreen() {
  const [username, setUsername] = useState('');
  const [email, setEmail] = useState('');
  const [password, setPassword] = useState('');
  const [confirmPassword, setConfirmPassword] = useState('');
  const [SuccessMessage, setSuccessMessage] = useState('');
  
  const onRegisterPressed = async () => {

    setSuccessMessage(`button pressed`)
/*
    const response = await fetch('https://18.218.123.255:8080/api/v1/register', {
      method: 'POST', 
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({
        username,
        email,
        password,
      }),
    })
      .then(response => response.json())
      .then(data => {
        setSuccessMessage(`Registered successfully!`)
        router.navigate('/(tabs)/HomePage')
      })
      .catch(error => {
        setSuccessMessage(`Error: ${error}`);
      });
    */
}
  return (
    <SafeAreaView style={{flex:1}}>
    <ScrollView showsVerticalScrollIndicator={false}>
    <View style = {styles.root}>
        <Text style = {styles.title}>
            Create an Account!
        </Text>

    { SuccessMessage ? (
      <Text style = {styles.successMessage}>{SuccessMessage} </Text>
    ) : null}

      <CustomInput 
        placeholder={"Username"} 
        value = {username} 
        setValue = {setUsername} 
        secureTextEntry={false}/>

      <CustomInput
        placeholder={"Email"} 
        value = {email} 
        setValue = {setEmail}
        secureTextEntry={false}/>

      <CustomInput
        placeholder={"Password"} 
        value = {password} 
        setValue = {setPassword} 
        secureTextEntry={true}/>

      <CustomInput
        placeholder={"Confirm Password"} 
        value = {confirmPassword} 
        setValue = {setConfirmPassword} 
        secureTextEntry={true}/>

      <CustomButton 
        text={"Register"} 
        onPress={()=>{onRegisterPressed(username,email,password,confirmPassword)}}/>
      <CustomButton 
        text={"Have an Account?"} 
        onPress={onHaveAnAccountPressed}
        type={"TERTIARY"}/>
    </View>
    </ScrollView>
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
    margin: 10,
  },
  successMessage:{
    fontSize: 16,
    color: 'gray',
    padding:5
  }
});