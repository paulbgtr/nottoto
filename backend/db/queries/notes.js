import { eq } from "drizzle-orm";
import db from "../db.js";
import { notes } from "../schemas.js";

export const getNotes = async () => {
  const res = await db
    .select()
    .from(notes);
  return res;
}

export const getNote = async (id) => {
  const res = await db
    .select()
    .from(notes)
    .where(eq(notes.id, id));
  return res;
}

export const createNote = async (note) => {
  const { title, body } = note;

  const res = await db
    .insert(notes)
    .values({
      title,
      body,
    })
    .returning();
  return res;
}

export const updateNote = async (id, note) => {
  const { title, body } = note;

  const res = await db
    .update(notes)
    .set({
      title,
      body,
    })
    .where(eq(notes.id, id))
    .returning();
  return res;
}

export const deleteNote = async (id) => {
  const res = await db
    .delete(notes)
    .where(eq(notes.id, id))
    .returning();

  return res;
}

