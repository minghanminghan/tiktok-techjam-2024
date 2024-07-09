import joblib

import pandas as pd
import numpy as np

import spotipy
from spotipy.oauth2 import SpotifyClientCredentials

#outward facing function at the very bottom
#figure out how to not expose the client_id and client_secret

#main function is called get_recs()
#input:     list of uris (user like list), can be any size (will trim in function)
#output:    list of uris (recommendations), size = 0-10









#big_data = joblib.load('python\\9_million_songs.JSON')
#mini_pipeline = joblib.load('python\\20_clusters_pipeline.JSON')

sp = spotipy.Spotify(auth_manager=SpotifyClientCredentials(client_id="577571ab029843d58e72fb448a256c58", client_secret="f1ddbccf106b4923a8c92334c1b08f99"))

#sp.audio_features caps at 100, so len(results) <= 100 (very max)
def get_track_features(track_ids:list[str]) -> pd.DataFrame: #takes list of uris, outputs dataframe
    results = pd.DataFrame(sp.audio_features(track_ids[:(10 if len(track_ids) < 10 else len(track_ids))])).drop(["type", "uri", "track_href", "analysis_url", "time_signature"], axis=1)
    return results

def get_rec_info(results:pd.DataFrame) -> pd.DataFrame:
    

    #is there a way to make this live all the time?
    mini_pipeline = joblib.load('python\\20_clusters_pipeline.JSON')


    #generate cluster for each song
    results['cluster'] = mini_pipeline.predict(results.select_dtypes(np.number)) #this line is a bit buggy
        
    #pick the mode cluster and a random song in the mode cluster
    cluster_mode = results['cluster'].mode()
    results = results[results['cluster'] == cluster_mode.iat[0]]
    return results.sample(1) #there might be an issue here

def find_song_recs(song_info:pd.DataFrame) -> pd.DataFrame:


    #make this more efficient
    rec_data = joblib.load('python\\9_million_songs.JSON')
    
    
    #slice data to cluster
    rec_data = rec_data[rec_data['cluster'] == song_info['cluster'].iat[0]] #438799 left (/20)
    
    #drop songs not in the same key and mode 
    rec_data = rec_data[rec_data['key'] == song_info['key'].iat[0]] #40369 left (/11)
    #rec_data = rec_data[rec_data['mode'] == song_info['mode'].loc[0]] #did nothing so not doing this
    
    #drop songs +-15s from query duration
    rec_data = rec_data[rec_data['duration_ms'] > song_info['duration_ms'].iat[0] - 15*1000]
    rec_data = rec_data[rec_data['duration_ms'] < song_info['duration_ms'].iat[0] + 15*1000] #2626

    #could do further filtering here
    rec_data = rec_data[round(rec_data['tempo'], 0) == round(song_info['tempo'].iat[0], 0)]
    rec_data = rec_data[round(rec_data['valence'], 0) == round(song_info['valence'].iat[0], 0)]
    rec_data = rec_data[round(rec_data['danceability'], 0) == round(song_info['danceability'].iat[0], 0)]
    rec_data = rec_data[round(rec_data['energy'], 0) == round(song_info['energy'].iat[0], 0)]
    rec_data = rec_data[round(rec_data['loudness'], -1) == round(song_info['loudness'].iat[0], -1)]
    if len(rec_data.index) > 2000:
        rec_data = rec_data[round(rec_data['speechiness'], 0) == round(song_info['speechiness'].iat[0], 0)]        
        rec_data = rec_data[round(rec_data['acousticness'], 0) == round(song_info['acousticness'].iat[0], 0)]
        rec_data = rec_data[round(rec_data['instrumentalness'], 0) == round(song_info['instrumentalness'].iat[0], 0)]
        rec_data = rec_data[round(rec_data['liveness'], 0) == round(song_info['liveness'].iat[0], 0)]

    print('songs matching filter:', len(rec_data.index))
    return rec_data['track_id'][:10].tolist()



#outward-facing function: takes in user's like list and outputs 10 recommended songs
def get_recs(uris:list[str]) -> list[str]:
    uri_features = get_track_features(uris)
    rec_profile = get_rec_info(uri_features)

    print(rec_profile.to_string())
    print('target song:',rec_profile['id'].iat[0])
    #print(sp.track(rec_profile['id'].get(0))['external_urls'])
    
    recs = find_song_recs(rec_profile)
    return recs


uris = ["7Far7FhCkXCQTsovPTzrmH",
       "6lYY2HktYKpV1pUamfRlU1",
       "4b82tXj35SycILuILcgBQ6"]

playlist = ['0sjxRg1VlYfx4YG7uxurrq',
 '3HMY0r2BAdpasXMY8rseR0',
 '03tk2dwP2r4xgg4kKY3X2X',
 '7vUNVm5CpZLHDlkRiOJJc5',
 '7b4iOHMngSAFhgN120hbWB',
 '6Rqn2GFlmvmV4w9Ala0I1e',
 '1L2v0m8OZthSoBHGpOwpV2',
 '1kPBT8S2wJFNAyBMnGVZgL',
 '08IytaL8esBeRlKzb61bJ3',
 '4v47bWal8JY2kQgYSnvsOm',
 '3ArhAv6XnxAnGK8s6et1E1',
 '6zE2xJvFmHEPECsvLki2CE',
 '5mUuJ9ho28WRUv03odBCBb',
 '0zGLlXbHlrAyBN1x6sY0rb',
 '6rqUOLt9dpfMANLMKTzoWj',
 '5lmGgOaixbwKHtpTzjJoqx',
 '4feXcsElKIVsGwkbnTHAfV',
 '61YzdCCBPM5Pc7lIiD5i8C',
 '00ko9WaS4jOX1kEk3gvHjf',
 '5Tf1BtJ5v5PebDYSwQXGFv',
 '3fAgliAn20lwENJdk0qfpW',
 '1gKmnrz6SzJlGMNRKBLx1V',
 '3EEr6l5PYelwkrvvKX7N0X',
 '0p9kp28VdoiuIxgzkzhZLU',
 '1XR1X1VTyTOVdPgH18RdME',
 '3HlK8txWAdtKMrbsqX40pl',
 '4o0VXG1CHe6gWVzsLxmcCT',
 '2qvToeBdYliw6n0nEsKJQa',
 '0GegHVxeozw3rdjte45Bfx']


#recs = get_recs(uris)
recs = get_recs(playlist)
print('recommendations',recs)