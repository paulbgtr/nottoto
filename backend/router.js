import express from "express";
import { getAllNotes, getNoteById, postNote } from "./controllers/notes.controller.js";

const router = express.Router();

router.get("/notes", getAllNotes);
router.get("/notes/:id", getNoteById);
router.post("/notes", postNote);

export default router;
