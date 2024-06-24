import React, {useState} from 'react'
import { Text, View, Image, StyleSheet, useWindowDimensions } from "react-native";
import Logo from '../../techjam/assets/images/music-app.png';
import CustomInput from '../../techjam/components/CustomInput'
import CustomButton from '../../techjam/components/CustomButton'

const onSignInPressed = () => {
  console.warn("Signed in");
}

const onForgotPasswordPressed = () => {
  console.warn("Forgot Password");
}

export default function index() {
  const [username, setUsername] = useState('');
  const [password, setPassword] = useState('');
  const {height} = useWindowDimensions();
  return (
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

      <CustomButton text={"Sign In"} onPress={onSignInPressed}/>
      <CustomButton 
        text={"Forgot Password?"} 
        onPress={onForgotPasswordPressed}
        type = "TERTIARY"/>
    </View>
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
    height: '30%',
    //maxHeight: 200,
    marginVertical: 10,
  },
});