import { getNotes, getNote, getNoteByTitle, createNote, updateNote, deleteNote } from "../db/queries/notes.js";
import { getUserId } from "../utils/getUserId.js";

export const getAllNotes = async (req, res) => {
  try {
    if (!req.headers.authorization) {
      return res.status(401).json({ error: "Missing Authorization Header" });
    }

    const headerParts = req.headers.authorization.split(" ");

    if (headerParts.length !== 2 || headerParts[0] !== "Bearer") {
      return res.status(401).json({ error: "Invalid Authorization Header" });
    }

    const token = headerParts[1];

    const userId = getUserId(token);

    const notes = await getNotes(userId);

    if (!notes) {
      return res.status(404).json({ error: "No notes found" });
    }

    res.status(200).json(notes);
  } catch (err) {
    res.status(500).json({ error: err.message });
  }
}

export const getNoteById = async (req, res) => {
  try {
    const { id } = req.params;

    if (!req.headers.authorization) {
      return res.status(401).json({ error: "Missing Authorization Header" });
    }

    const headerParts = req.headers.authorization.split(" ");

    if (headerParts.length !== 2 || headerParts[0] !== "Bearer") {
      return res.status(401).json({ error: "Invalid Authorization Header" });
    }

    const token = headerParts[1];

    const userId = getUserId(token);

    const [note] = await getNote(userId, id);

    if (!note) {
      return res.status(404).json({ error: `Note with id ${id} is not found` });
    }

    res.status(200).json(note);
  } catch (err) {
    res.status(500).json({ error: err.message });
  }
}

export const postNote = async (req, res) => {
  try {
    const { title, body } = req.body;

    if (!title) {
      return res.status(400).json({ error: "Missing the required title value" });
    }

    const [noteExists] = await getNoteByTitle(title);

    if (noteExists) {
      return res.status(400).json({ error: `Note with title ${title} already exists` });
    }

    if (!req.headers.authorization) {
      return res.status(401).json({ error: "Missing Authorization Header" });
    }

    const headerParts = req.headers.authorization.split(" ");

    if (headerParts.length !== 2 || headerParts[0] !== "Bearer") {
      return res.status(401).json({ error: "Invalid Authorization Header" });
    }

    const token = headerParts[1];

    const userId = getUserId(token);

    const note = await createNote(userId, {
      title,
      body,
    });
    res.status(201).json(note);
  } catch (err) {
    res.status(500).json({ error: err.message });
  }
}

export const updateNoteById = async (req, res) => {
  try {
    const { id } = req.params;
    const { title, body } = req.body;

    if (!title && !body) {
      return res.status(400).json({ error: "Missing title and body values" });
    }

    if (!req.headers.authorization) {
      return res.status(401).json({ error: "Missing Authorization Header" });
    }

    const headerParts = req.headers.authorization.split(" ");

    if (headerParts.length !== 2 || headerParts[0] !== "Bearer") {
      return res.status(401).json({ error: "Invalid Authorization Header" });
    }

    const token = headerParts[1];

    const userId = getUserId(token);

    const note = await updateNote(id, userId, {
      title,
      body,
    });
    res.status(200).json(note);
  } catch (err) {
    res.status(500).json({ error: err.message });
  }
}

export const deleteNoteById = async (req, res) => {
  try {
    const { id } = req.params;

    const [note] = await getNote(id);

    if (!note) {
      return res.status(404).json({ error: `Note with id ${id} is not found` });
    }

    if (!req.headers.authorization) {
      return res.status(401).json({ error: "Missing Authorization Header" });
    }

    const headerParts = req.headers.authorization.split(" ");

    if (headerParts.length !== 2 || headerParts[0] !== "Bearer") {
      return res.status(401).json({ error: "Invalid Authorization Header" });
    }

    const token = headerParts[1];

    const userId = getUserId(token);

    const deletedNote = await deleteNote(id, userId);
    res.status(200).json({
      message: `Note with id ${id} was deleted`,
      deletedNote
    });
  } catch (err) {
    res.status(500).json({ error: err.message });
  }
}

