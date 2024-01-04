import { serial, text, timestamp, pgTable } from "drizzle-orm/pg-core";

export const notes = pgTable("notes", {
  id: serial("id").primaryKey(),
  title: text("title"),
  body: text("body"),
  createdAt: timestamp("created_at", { withTimezone: true }).default(),
});

export const users = pgTable("users", {
  id: serial("id").primaryKey(),
  email: text("email").notNull(),
  password: text("password").notNull(),
  createdAt: timestamp("created_at", { withTimezone: true }).default(),
});

