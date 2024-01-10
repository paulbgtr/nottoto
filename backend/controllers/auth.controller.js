import bcrypt from "bcryptjs";
import jwt from "jsonwebtoken";
import {
  getUserById,
  getUserByEmail,
  createUser
} from "../db/queries/users.js";

export const signup = async (req, res) => {
  try {
    const { email, password } = req.body;

    if (!email || !password) {
      return res.status(400).json({
        message: "Missing email or password"
      });
    }

    const [user] = await getUserByEmail(email);

    console.log("a");

    if (user) {
      return res.status(409).json({
        message: "User already exists"
      });
    }

    const hashedPassword = await bcrypt.hash(password, 10);

    const createdUser = await createUser({
      email,
      password: hashedPassword
    });

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
      userId: user.id,
    }, "secret", { expiresIn: "7d" });

    res.cookie("jwt", token, {
      httpOnly: true,
      maxAge: 7 * 24 * 60 * 60 * 1000,
    })
  } catch (err) {
    res.status(500).json({
      message: "Error signing in"
    });
  }
}

export const verify = async (req, res) => {
  const { jwt: token } = req.cookies;

  if (!token) {
    return res.status(401).json({
      message: "No token"
    });
  }

  const decoded = jwt.verify(token, "secret");

  const { userId } = decoded;

  const [user] = await getUserById(userId);

  if (!user) {
    return res.status(404).json({
      message: "User not found"
    });
  }

  res.status(200).json(user);
}

export const signout = async (req, res) => {
  //todo
}
