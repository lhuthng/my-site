import express from 'express';
import { validateSession } from '../middleware/validation.js';
import { deleteUser } from '../controllers/userController.js';

const router = express.Router();

router.delete('/me', validateSession, deleteUser);

export default router;