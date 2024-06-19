import { Elysia } from "elysia";
import * as auth from "../lib/auth";

function userRoutes(app: Elysia) {
	app.post("/api/v1/login", async (req) => {
		return await userLogin(req.body);
	})

	app.post("/api/v1/register", async (req) => {
		return await userLogin(req.body);
	})
}

async function userRegister(body: any): Promise<string> {
	if (!body) {
		return "Empty body"
	}
	if (!auth.isRegistrationInput) {
		return "Invalid input: requires body properties 'username', 'email', and 'password' as string";
	}
	return await auth.registerUser(
		body.username,
		body.email,
		body.password
	);
}

async function userLogin(body: any): Promise<string> {
		if (!body) {
			return "Empty body";
		}
		if (!auth.isLoginInput) {
			return "Invalid input: requires body properties 'identifier' and 'password' as string";
		}
		const { identifier, password }: auth.LoginInput = body as auth.LoginInput;

		try {
			const authToken = await auth.loginUser({identifier: identifier, password: password});
			//return authToken;
			return "Password token not implemented yet sorry :)";
		} catch (err) {
			if (err instanceof auth.IncorrectPasswordError) return err.message;
			else if (err instanceof auth.UserNotFoundError) return err.message;
			else {
				console.log(`Unknown error unable to login ${err}`);
				return `Unknown error unable to login ${err}`;
			}
		}
}

export default userRoutes;
