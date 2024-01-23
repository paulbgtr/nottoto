import { eq, and } from "drizzle-orm";
import db from "../db.js";
import { notes } from "../schemas.js";

export const getNotes = async (userId) => {
  const res = await db
    .select()
    .from(notes)
    .where(eq(notes.userId, userId));
  return res;
}

export const getNote = async (noteId, userId) => {
  const res = await db
    .select()
    .from(notes)
    .where(
      and(
        eq(notes.userId, userId),
        eq(notes.id, noteId),
      )
    );
  console.log(res)
  return res;
}

export const getNoteByTitle = async (title) => {
  const res = await db
    .select()
    .from(notes)
    .where(eq(notes.title, title));

  return res;
}

export const createNote = async (userId, note) => {
  const { title, body } = note;

  const res = await db
    .insert(notes)
    .values({
      userId,
      title,
      body,
    })
    .returning();
  return res;
}

export const updateNote = async (noteId, userId, note) => {
  const { title, body } = note;

  const res = await db
    .update(notes)
    .set({
      title,
      body,
    })
    .where(
      and(
        eq(notes.id, noteId),
        eq(notes.userId, userId)
      )
    )
    .returning();
  return res;
}

export const deleteNote = async (noteId, userId) => {
  const res = await db
    .delete(notes)
    .where(
      and(
        eq(notes.id, noteId),
        eq(notes.userId, userId)
      ))
    .returning();

  return res;
}

