import { Elysia } from "elysia";
import { config } from "dotenv";

//NECESSARY TO IMPORT ENV FILES
config();

const app = new Elysia()

app.get("/api/v1/", () => {
	return "This is the api for tiktok app";
})

app.post("/api/v1/echo", ({ body }) => {
	return `${ body }`;
});

app.listen(process.env.PORT ?? 8080, () => {
	console.log(`Server is running on port ${app.server?.port}...`);
})
