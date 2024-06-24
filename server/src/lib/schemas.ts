import { Schema, model, Document, type ObjectId } from "mongoose";

enum SongType {
	spotify,
	uploaded
}
const SongSchema = new Schema({
	title: String,
	artist: String,
	type: SongType,
	release: Number,
});

interface SongDocument extends Document {
	_id: ObjectId,
	title: string,
	artist: string,
	type: SongType,
	release: Number,
}

/**
 * A Schema that represents a song
 * @param title: string
 * 	The song title
 * @param artist: string 
 *   	The artist who made the song
 * @param type: SongType
 *   	The enum that describes who the data should be stored
 * @param release: Number
 *   	The year that the song was release in
 */
const Song = model<SongDocument>("Song", SongSchema);

const UserSchema = new Schema({
	username: { type: String, required: true, unique: true },
	password: { type: String, required: true },
	salt: { type: String, required: true },
	email: { type: String, required: true, unique: true },
	liked: { type: [Song], required: true },
	disliked: { type: [Song], required: true },
})

interface UserDocument extends Document {
	_id: ObjectId,
	username: string,
	password: string,
	salt: string,
	email: string,
	liked: [SongDocument],
	disliked: [SongDocument],
}

/**
 * User
 * @param username: string
 *   	The username
 * @param password: string
 *   	The stored hash for the user
 * @param salt: string
 *   	The input used to generate the hash
 * @param email: string
 *   	user email
 * @param liked: [Song]
 *   	List of songs that the user has swiped right on
 * @param disliked: [Song]
 *   	List of songs that the user has swiped left on
 */
const User = model<UserDocument>("User", UserSchema);

export {
	Song,
	User,
	SongType,
	type UserDocument,
	type SongDocument,
}
