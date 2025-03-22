import express from 'express';
import { createUser, login } from '../controllers/authController.js';
import { validateLogin, validateSession, validateUser } from '../middleware/validation.js';

const router = express.Router();

router.post('/register', validateUser, createUser);
router.post('/login', validateLogin, login);

export default router;