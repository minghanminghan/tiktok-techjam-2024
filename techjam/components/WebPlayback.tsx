import { auth as SpotifyAuth, remote as SpotifyRemote, ApiScope, ApiConfig, SpotifyRemoteApi } from 'react-native-spotify-remote';
import React, { useState, useEffect } from 'react';
import { View, Text, StyleSheet, Image, useWindowDimensions } from 'react-native';

const client_id="577571ab029843d58e72fb448a256c58";
const client_secret="f1ddbccf106b4923a8c92334c1b08f99";
/*
    async function playEpicSong(){
        try{
            const token = await SpotifyAuth.initialize(spotifyConfig);
            await SpotifyRemote.connect(token);
            SpotifyRemote.playUri("spotify:track:6IA8E2Q5ttcpbuahIejO74#0:38");
        }catch(err){
            console.error("Couldn't authorize with or connect to Spotify",err);
        }
    }
*/
    //let listener = SpotifyRemote.addListener;

    //fields: json from web api
    //this component needs the web playback sdk
    //render json fields and also have the web player

    const WebPlayback = (props) => {
        const spotifyConfig: ApiConfig = {
            clientID: "577571ab029843d58e72fb448a256c58",
            redirectURL: "SPOTIFY_REDIRECT_URL", //fill these urls out
            tokenRefreshURL: "SPOTIFY_TOKEN_REFRESH_URL",
            tokenSwapURL: "SPOTIFY_TOKEN_SWAP_URL",
            //scope: ApiScope.AppRemoteControlScope | ApiScope.UserFollowReadScope
        }
    
    async function playEpicSong() {
        try {
          const session = await SpotifyAuth.authorize(spotifyConfig);
          await SpotifyRemote.connect(session.accessToken);
          await SpotifyRemote.playUri("spotify:track:6IA8E2Q5ttcpbuahIejO74");
          await SpotifyRemote.seek(58000);
        } catch (err) {
          console.error("Couldn't authorize with or connect to Spotify", err);
        }
      }

    //playEpicSong()

    return(
        <View>
            {}
        </View>
    )
}
  
/*
    const get_refresh_token = async () => {
        var refresh_token //need to call this from back end
        const response = await fetch('https://accounts.spotify.com/api/token', {
            method: 'POST',
            headers: {'content-type': 'application/x-www-form-urlencoded',
                'Authorization': 'Basic ' + (new Buffer.from(client_id + ':' + client_secret).toString('base64'))},
            body: JSON.stringify({
                grant_type: 'refresh_token',
                refresh_token: refresh_token
            }),
        })
        .then(response => response.json())
        .then(data => {
            console.log(`api token refreshed`)
            console.log(data)
            //post new auth token to back end
        })
        .catch(error => {
            console.log(`Error: ${error}`);
        });
    }
*/
/*
    var authOptions = {
        url: 'https://accounts.spotify.com/api/token',
        headers: {
        'content-type': 'application/x-www-form-urlencoded',
        'Authorization': 'Basic ' + (new Buffer.from(client_id + ':' + client_secret).toString('base64'))
        },
        form: {
        grant_type: 'refresh_token',
        refresh_token: refresh_token
        },
        json: true
    };

    request.post(authOptions, function(error, response, body) {
        if (!error && response.statusCode === 200) {
        var access_token = body.access_token,
            refresh_token = body.refresh_token;
        res.send({
            'access_token': access_token,
            'refresh_token': refresh_token
        });
        }
    });
    }
*/

export default WebPlayback