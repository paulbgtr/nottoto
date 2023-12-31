import express from "express";
import { getAllNotes, getNoteById, postNote, updateNoteById, deleteNoteById } from "./controllers/notes.controller.js";

const router = express.Router();

router.get("/notes", getAllNotes);
router.get("/notes/:id", getNoteById);
router.post("/notes", postNote);
router.patch("/notes/:id", updateNoteById);
router.delete("/notes/:id", deleteNoteById);

export default router;
