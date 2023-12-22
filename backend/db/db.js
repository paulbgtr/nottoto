import { drizzle } from "drizzle-orm/postgres-js";
import postgres from "postgres";
import "dotenv/config";

const queryClient = postgres(process.env.DB_URL);
const db = drizzle(queryClient);

