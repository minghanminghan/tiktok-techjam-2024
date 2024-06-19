import bcrypt from "bcrypt";
import { User } from "./schemas";

/**
 * The identifier can be either email or username
 * The password is plaintext bruh
 */
interface LoginInput {
	identifier: string,
	password: string,
}

class IncorrectPasswordError extends Error {
	constructor(message: string) {
		super(message);
		this.name = "IncorrectPasswordError";
	}
}

class UserNotFoundError extends Error {
	constructor(message: string) {
		super(message);
		this.name = "UserNotFoundError";
	}
}

class IdentifierTakenError extends Error {
	constructor(message: string) {
		super(message);
		this.name = "IdentifierTakenError";
	}
}

async function comparePassword(
	password: string,
	hash: string,
	salt: string
): Promise<boolean>{
	const inputHash = await bcrypt.hash(password, salt);
	const passwordMatch = await bcrypt.compare(inputHash, hash)
	if (!passwordMatch) {
		return false;
	}
	return true;
}

/**
 * @param LoginInput
 * An interface with either username or password as identifier and the plain-text password for the user
 * @returns
 * Returns the authentication token. Don't know what that looks like yet
 * @example
	const login: LoginInput = {identifier: username, password: password}
	const authToken = await loginUser(login);
 */
async function loginUser({ identifier, password }: LoginInput): Promise<string> {
	const user = await User.findOne({
		$or: [{ username:identifier }, { email:identifier }]
	});

	if (!user) {
		throw new UserNotFoundError("User not found");
	}

	const passwordMatch = await comparePassword(password, user.password, user.salt);
	if (!passwordMatch) {
		throw new IncorrectPasswordError("Password does not match");
	}
	//@TODO: Include authentication token generation
	return "";
}

/**
 * @param
 * password - plaintext eleg
 * @throws
 * username email in use bullshit 
 * @returns
 * The auth token from loginUser
 */
async function registerUser (
	username: string, 
	password: string, 
	email: string
): Promise<string> {
	const avaliable = await checkAvaliability(username, email);
	if (!avaliable) {
		throw new IdentifierTakenError("Username or email already in use!");
	}

	const [salt, hash] = await generateHash(password);
	const newUser = new User({
		username: username,
		password: hash,
		salt: salt,
		email: email,
		liked: [],
		disliked: [],
	});
	await newUser.save();

	const login: LoginInput = {identifier: username, password: password}
	return await loginUser(login);
}

async function generateHash(password: string): Promise<[string, string]> {
	const salt = await bcrypt.genSalt(10);
	const hash = await bcrypt.hash(password, salt);
	return [salt, hash];
}

async function checkAvaliability(username: string, email: string): Promise<boolean> {
	const user = await User.findOne({
		$or: [{ username:username }, { email:email }]
	});
	if (user) return false;
	else return true;
}

export {
	type LoginInput,
	loginUser,
	registerUser,
}
