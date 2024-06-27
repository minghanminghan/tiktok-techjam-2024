import jwt from "jsonwebtoken";
import { Elysia, type Context } from "elysia"

function authenticateToken(ctx: Context, next: Next) {
    const authHeader = req.headers["authorization"];
    const token
}
