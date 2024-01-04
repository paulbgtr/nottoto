import bcrypt from "bcryptjs";
import { getUserByEmail, createUser } from "../db/queries/users.js";

export const signup = async (req, res) => {
  try {

    const { email, password } = req.body;

    const [user] = await getUserByEmail(email);

    if (user) {
      return res.status(409).json({
        message: "User already exists"
      });
    }

    const hashedPassword = await bcrypt.hash(password, 10);

    const createdUser = await createUser({
      email,
      password: hashedPassword
    })

    res.status(201).json(createdUser);
  } catch (err) {
    res.status(500).json({
      message: "Error creating user"
    });
  }
}

export const signin = async (req, res) => {
  //todo
}

export const signout = async (req, res) => {
  //todo
}
