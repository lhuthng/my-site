import { body, validationResult } from 'express-validator';
import { AUTH_METHODS } from '../configs/db.js';
import jwt from 'jsonwebtoken';
import Session from '../models/sessionModel.js';
import User from '../models/userModel.js';

const { JWT_SECRET } = process.env;

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

export const validateLogin = [
    body('authMethod')
        .notEmpty()
        .withMessage('Authentication method is required.')
        .isIn(AUTH_METHODS)
        .withMessage('Invalid authentication method.'),
    body('email')
        .if(body('authMethod').equals('local'))
        .notEmpty()
        .withMessage('Email is required for login.')
        .isEmail()
        .withMessage('Invalid email.')
        .normalizeEmail(),
    body('password')
        .if(body('authMethod').equals('local'))
        .notEmpty()
        .withMessage('Password is required for login.'),
    body('authId')
        .if(body('authMethod').not().equals('local'))
        .notEmpty()
        .withMessage('Auth ID is required for OAuth login')
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
]

export const validateSession = async (req, res, next) => {
    const token = req.headers.authorization?.split(' ')[1];
    if (!token) {
        return res.status(401).json({ error: "Access denied. No token provided."});
    }

    try {
        const decoded = jwt.verify(token, JWT_SECRET);

        const session = await Session.findValidSession(decoded.userId, token);
        if (!session) {
            return res.status(401).json({ error: "Access denued. Invalid or expired token."});
        }
        req.user = { userId: decoded.userId };
        next();
    } catch (err) {
        console.error('Session validation error: ', err);
        res.status(401).json({ error: 'Invalid token.'});
    }
};