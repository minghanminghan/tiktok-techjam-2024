import { Elysia } from "elysia";
import * as auth from "../lib/spotify/auth";


function spotifyRoutes(app: Elysia) {

	app.post("/api/v1/spotify/login", async (req: any, res: any) => {
        return "not implemented yet :)";
	})

}

export default spotifyRoutes;
