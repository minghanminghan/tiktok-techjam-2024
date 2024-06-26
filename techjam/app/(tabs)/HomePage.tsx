import { Text, View, StyleSheet, ScrollView } from "react-native";
import { Stack } from "expo-router";
import {router} from 'expo-router';
import {SafeAreaView} from 'react-native-safe-area-context';

import CustomButton from '../../components/CustomButton'

const onSignOutPressed = () => {
  router.back()
}

export default function HomePage() {
    return (
      <SafeAreaView style={{flex:1}}>
        <ScrollView showsVerticalScrollIndicator={false}>
        <View style = {styles.root}>
            <Text style = {styles.title}>Home Page</Text>
            <CustomButton text={"Sign Out"} onPress={onSignOutPressed} />
        </View>
        </ScrollView>
      </SafeAreaView>
    );
};

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
  });