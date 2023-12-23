import { getNotes, getNote, createNote, updateNote } from "../db/queries/notes.js";

export const getAllNotes = async (req, res) => {
  try {
    const notes = await getNotes();

    if (!notes) {
      res.status(404).json({ error: "No notes found" });
    }

    res.status(200).json(notes);
  } catch (err) {
    res.status(500).json({ error: err.message });
  }
}

export const getNoteById = async (req, res) => {
  try {
    const { id } = req.params;

    const [note] = await getNote(id);

    if (!note) {
      res.status(404).json({ error: `Note with id ${id} not found` });
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
      res.status(400).json({ error: "Missing the required title value" });
    }

    const note = await createNote({
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
      res.status(400).json({ error: "Missing title and body values" });
    }

    const note = await updateNote(id, {
      title,
      body,
    });
    res.status(200).json(note);
  } catch (err) {
    res.status(500).json({ error: err.message });
  }
}

