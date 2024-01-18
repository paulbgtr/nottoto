import jwt from 'jsonwebtoken';

export const getUserId = (token) => {
  const { userId } = jwt.verify(token, "secret");
  return userId;
}
