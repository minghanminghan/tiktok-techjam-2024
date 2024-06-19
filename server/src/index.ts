import { Elysia } from "elysia";
import { config } from "dotenv";
import userRoutes from "./routes/userRoutes";

//NECESSARY TO IMPORT ENV FILES
config();

const app = new Elysia()

userRoutes(app);

app.get("/api/v1/", () => {
	return "This is the api for tiktok app";
})

app.post("/api/v1/echo", ({ body }) => {
	return `${ body }`;
});

app.listen(process.env.PORT ?? 8080, () => {
	console.log(`Server is running on port ${app.server?.port}...`);
})
