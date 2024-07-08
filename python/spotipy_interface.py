import pandas as pd
import spotipy
from spotipy.oauth2 import SpotifyClientCredentials

#hard coded api key, not good
sp = spotipy.Spotify(auth_manager=SpotifyClientCredentials(client_id="577571ab029843d58e72fb448a256c58",
                                                           client_secret="f1ddbccf106b4923a8c92334c1b08f99"))

test_like_list = [ #configure this such that last liked song = index 0
    "7Far7FhCkXCQTsovPTzrmH",
    "6lYY2HktYKpV1pUamfRlU1",
    "4b82tXj35SycILuILcgBQ6"
]

def get_recs(track_ids:str) -> pd.DataFrame:
    #aggregating feature data from liked list
    features = pd.DataFrame(sp.audio_features(track_ids))[["duration_ms", "key", "time_signature", "mode", "tempo", "loudness", "danceability", "energy",  "speechiness", "acousticness", "instrumentalness", "liveness", "valence"]]
    features["duration_ms"] = features["duration_ms"].apply(lambda x: x//(30)) #rounds down, so duration_30s: 8 = 3.5 - 4 minutes
    features = features.round({
        'tempo':0, 
        'loudness':0, 
        'danceability':1, 
        'energy':1, 
        'speechiness':1, 
        'acousticness':1, 
        'instrumentalness':1, 
        'liveness':1, 
        'valence':1
        }).mode()

    features = features.loc[0] #if there are multiple modes, the first mode is returned

    recs = sp.recommendations(  #technically not supposed to use this; do the kmeans cluster instead
        seed_tracks=track_ids[:(5 if len(track_ids) > 5 else len(track_ids))], #using last 5 tracks at most
        limit=10,
        type='track',
        target_tempo = features["tempo"]
    )['tracks']

    output = [x['uri'] for x in recs]

    return output

'''
example_uri = "spotify:track:20JcU4ZiYAuKE4oIal8WoV"
example_id = "20JcU4ZiYAuKE4oIal8WoV"

spotify_response = get_recs(test_like_list)

print(spotify_response)
'''