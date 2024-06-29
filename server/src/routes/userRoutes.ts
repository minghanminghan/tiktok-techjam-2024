import { Elysia, type Context } from "elysia";
import * as auth from "../lib/auth";

function userRoutes(app: Elysia) {
	app.post("/api/v1/login", async (cxt: Context) => {
		return await userLogin(cxt);
	})

	app.post("/api/v1/register", async (req) => {
		return await userRegister(req.body);
	})
}


async function userRegister(ctx: Context): Promise<string> {

    const body = await ctx.request.json;
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

async function parseBody<T>(request: Request): Promise<T> {
    const body = await request.json();
    return body as T;
}

async function userLogin(ctx: Context): Promise<string> {
        const body = ctx.request.body;
		if (!body) {
			return "Empty body";
		}
		if (!auth.isLoginInput) {
			return "Invalid input: requires body properties 'identifier' and 'password' as string";
		}

		const { identifier, password }: auth.LoginInput = body as auth.LoginInput;

		try {
			const authToken = await auth.loginUser({identifier: identifier, password: password});
			return authToken;
		} catch (err) {
			if (err instanceof auth.IncorrectPasswordError) {
				return err.message;
			}
			else if (err instanceof auth.UserNotFoundError) {
				return err.message;
			}
			else if (err instanceof auth.PrivateKeyNotFound) {
                res.status(500).send(err.message);
				return err.message;
			}
			else if (err instanceof auth.PublicKeyNotFound) {
                res.status(500).send(err.message);
                return err.message;
			}
			else {
				console.log(`Unknown error unable to login ${err}`);
				return `Unknown error unable to login ${err}`;
			}
		}
}

export default userRoutes;
