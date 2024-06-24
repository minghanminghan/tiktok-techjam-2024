import bcrypt from "bcrypt";
import jwt from "jsonwebtoken";
import { type ObjectId } from "mongoose";
import { User, type UserDocument } from "./schemas";

/**
 * The identifier can be either email or username
 * The password is plaintext bruh
 */
interface LoginInput {
	identifier: string,
	password: string,
}

const SALT_ROUNDS = 10;

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

class PrivateKeyNotFound extends Error {
	constructor(message: string) {
		super(message);
		this.name = "PrivateKeyNotFound";
	}
}

class PublicKeyNotFound extends Error {
	constructor(message: string) {
		super(message);
		this.name = "PublicKeyNotFound";
	}
}

async function comparePassword(
	password: string,
	hash: string,
	salt: string
) {
	const inputHash = await bcrypt.hash(password, salt);
	const passwordMatch = await bcrypt.compare(inputHash, hash)
	if (!passwordMatch) {
		throw new IncorrectPasswordError("Password does not match");
	}
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
	const user = await findUser(identifier);
	await comparePassword(password, user.password, user.salt);
	const authToken = generateToken(user._id)
    return authToken;
}

function generateToken(userId: ObjectId): string {
	if (!process.env.PRIVATE_KEY) {
		throw new PrivateKeyNotFound("Unable to load PRIVATE_KEY from .env file")
	}
	const token = jwt.sign({ userId }, process.env.PRIVATE_KEY, { algorithm: "RS256", expiresIn: "1h"});
	return token;
}

function authenticateToken(req: any, res: any, next: any) {
	const token = req.header("Authorization")?.split(' ')[1]
	if (!token) {
		return res.status(401).send('Access denied');
	}

    if (!process.env.PUBLIC_KEY) {
		return res.status(500).send('Internal Server Error: Unable to load public key from .env');
    }

	try {
		const verified = jwt.verify(token, process.env.PUBLIC_KEY);
		req.user = verified;
		next();
	} catch (error) {
		if (error instanceof jwt.TokenExpiredError) {
			return res.status(401).send('Token expired');
		}
		return res.status(400).send('Invalid token');
	}
}

async function findUser(identifier: string): Promise<UserDocument> {
	const user = await User.findOne({
		$or: [{ username:identifier }, { email:identifier }]
	});

	if (!user) {
		throw new UserNotFoundError("User not found");
	}
	return user;
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
	await checkAvaliability(username, email);
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
	const salt = await bcrypt.genSalt(SALT_ROUNDS);
	const hash = await bcrypt.hash(password, salt);
	return [salt, hash];
}

async function checkAvaliability(username: string, email: string) {
	const user = await User.findOne({
		$or: [{ username:username }, { email:email }]
	});
	if (user) {
		throw new IdentifierTakenError("Username or email already in use!");
	}
}

function isLoginInput(body: any): boolean {
	if (!(body.identifier && body.password)) {
		return false;
	}
	return (
	    typeof body === 'object' &&
	    typeof body.identifier === 'string' &&
	    typeof body.password === 'string'
	);
}

function isRegistrationInput(body: any): boolean {
	if (!(body.username && body.email && body.password)) {
		return false;
	}
	return (
	    typeof body === 'object' &&
	    typeof body.username === 'string' &&
	    typeof body.email === 'string' &&
	    typeof body.password === 'string'
	);
}

export {
	type LoginInput,
	loginUser,
	registerUser,
	isLoginInput,
	isRegistrationInput,
	authenticateToken,
	IdentifierTakenError,
	IncorrectPasswordError,
	UserNotFoundError,
	PrivateKeyNotFound,
    PublicKeyNotFound
}
