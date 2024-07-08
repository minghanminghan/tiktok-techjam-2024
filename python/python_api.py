from flask import Flask, request, jsonify

import spotipy_interface as spi

app = Flask(__name__)

@app.route('/recommend-song', methods=['POST'])
def run_python_script():
    # Receive data from the request
    data = request.json

    # Extract parameters as needed
    like_list = data.get('like_list')

    # Run your Python script here
    # Example: Call a function that executes your script
    result = spi.get_recs(like_list)

    # Return the result as JSON
    return jsonify({'result': result})

if __name__ == '__main__':
    app.run(debug=True)