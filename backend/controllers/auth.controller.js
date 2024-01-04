import bcrypt from "bcryptjs";
import jwt from "jsonwebtoken";
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
  try {
    const { email, password } = req.body;

    const [user] = await getUserByEmail(email);

    if (!user || !(await bcrypt.compare(password, user.password))) {
      return res.status(401).json({
        message: "Invalid credentials"
      });
    }

    const token = jwt.sign({
      id: user.id,
    }, "secret", { expiresIn: "7d" });

    res.cookie("jwt", jwt, {
      httpOnly: true,
      maxAge: 7 * 24 * 60 * 60 * 1000,
    })
  } catch (err) {
    res.status(500).json({
      message: "Error signing in"
    });
  }
}

export const signout = async (req, res) => {
  //todo
}
