import { Text, View, StyleSheet, ScrollView } from "react-native";
import { Stack } from "expo-router";
import {router} from 'expo-router';
import {SafeAreaView} from 'react-native-safe-area-context';

import CustomButton from '../components/CustomButton'

const onBackPressed = () => {
    router.back();
}

export default function SignInError() {
    return (
        <SafeAreaView style = {{flex:1}}>
        <View style = {styles.root}>
            <Text style = {styles.title}> Sorry! There was an error signing in. Please try again. </Text>
            <CustomButton onPress={onBackPressed} text={"back"} type={"TERTIARY"}/>
        </View>
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
    title:{
      fontSize:24,
      fontWeight:'bold',
      color:'#051c60',
      margin:10,
    },
  });