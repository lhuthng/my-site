import { body, validationResult } from 'express-validator';
import { AUTH_METHODS } from '../configs/db.js';

export const validateUser = [
    body('authMethod')
        .notEmpty()
        .withMessage('Authentication method is required.')
        .isIn(AUTH_METHODS)
        .withMessage('Invalid authentication method.'),
    body('username')
        .notEmpty()
        .withMessage('Username is required.')
        .isString()
        .withMessage('Username must be a string.')
        .trim()
        .escape(),
    body('email')
        .if(body('authMethod').equals('local'))
        .notEmpty()
        .withMessage('Email is required for local authentication.')
        .isEmail()
        .withMessage('Invalid email.')
        .normalizeEmail(),
    body('password')
        .if(body('authMethod').equals('local'))
        .notEmpty()
        .withMessage('Password is required for local authentication.')
        .isLength({ min: 8, max: 20 })
        .withMessage('Password must be at least 8 and at most 20 characters.'),
    body('authId')
        .if(body('authMethod').not().equals('local'))
        .notEmpty()
        .withMessage('Auth ID is required for OAuth authentication')
        .isString()
        .withMessage('Auth ID must be a string')
        .trim()
        .escape(),
    (req, res, next) => {
        const errors = validationResult(req);
        if (!errors.isEmpty()) {
            return res.status(400).json({ errors: errors.array() });
        }
        next();
    }
];