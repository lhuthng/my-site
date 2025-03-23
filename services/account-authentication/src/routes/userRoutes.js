import express from 'express';
import { validateSession } from '../middleware/validation.js';
import { requestDeletion, confirmDeletion, verifyUser } from '../controllers/userController.js';

const router = express.Router();

router.post('/me', validateSession, requestDeletion);
router.delete('/confirm-deletion/:token', confirmDeletion);
router.put('/verify-user/:token', verifyUser);

export default router;