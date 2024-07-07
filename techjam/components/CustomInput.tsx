import React from 'react'
import {View, Text, TextInput, StyleSheet} from 'react-native'

const CustomInput = ({value, setValue, placeholder, secureTextEntry}) => {
    return (
        <View style = {styles.container}>
            <TextInput 
                value = {value}
                onChangeText={setValue}
                placeholder={placeholder} 
                secureTextEntry={secureTextEntry}
                style={styles.input}>

            </TextInput>
        </View>
    )
}

const styles = StyleSheet.create({
    container: {
        padding: 10,
        backgroundColor: 'white',
        width: '100%',

        borderColor: '#e8e8e8',
        borderWidth: 1,
        borderRadius: 5,

        paddingHorizontal: 10,
        marginVertical: 5,
    },
    input: {},
})

export default CustomInput