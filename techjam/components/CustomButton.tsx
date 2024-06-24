import React from 'react'
import {View, Text, TextInput, StyleSheet, Pressable} from 'react-native'

const CustomButton = ({onPress, text, type='PRIMARY'}) => {
    return (
        <Pressable onPress = {onPress} style = {[styles.container, styles[`container_${type}`]]}>
            <Text style = {[styles.text, styles[`text_${type}`]]}> {text} </Text> 
        </Pressable>
    ) 
}

const styles = StyleSheet.create({
    container: {
        padding: 10,
        width: '100%',
        marginVertical: 5,
        alignItems: 'center',
        borderRadius: 5,
    },
    container_PRIMARY: {
        backgroundColor: '#3B71F3',
    },
    container_TERTIARY: {

    },
    text: {
        fontWeight: 'bold',
        color: 'white',
    },
    text_TERTIARY: {
        color: 'gray',
    }
})

export default CustomButton