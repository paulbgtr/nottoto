import { eq } from "drizzle-orm";
import db from "../db.js";
import { users } from "../schemas.js";

export const getUserById = async (id) => {
  const res = await db
    .select()
    .from(users)
    .where(eq(users.id, id));
  return res;
}

export const getUserByEmail = async (email) => {
  const res = await db
    .select()
    .from(users)
    .where(eq(users.email, email));

  return res;
}

export const createUser = async (user) => {
  const { email, password } = user;

  const res = await db
    .insert(users)
    .values({
      email,
      password
    }).returning();

  return res;
}

export const updateUser = async (user) => {
  const { id, email, password } = user;

  const res = await db
    .update(users)
    .set({
      email,
      password
    })
    .where(eq("id", id))
    .returning();

  return res;
}
