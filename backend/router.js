import express from "express";
import {
  getAllNotes,
  getNoteById,
  postNote,
  updateNoteById,
  deleteNoteById
} from "./controllers/notes.controller.js";
import {
  signup,
  signin,
  signout,
  verify
} from "./controllers/auth.controller.js"

const router = express.Router();

router.get("/notes", getAllNotes);
router.get("/notes/:id", getNoteById);
router.post("/notes", postNote);
router.patch("/notes/:id", updateNoteById);
router.delete("/notes/:id", deleteNoteById);

router.post("/users/signup", signup);
router.post("/users/signin", signin);
router.post("/users/signout", signout);
router.post("/users/verify", verify);

export default router;
