import Session from "../models/sessionModel.js";
import User from "../models/userModel.js";
import jwt from 'jsonwebtoken';

const { JWT_SECRET } = process.env;

export const createUser = async (req, res) => {
    const { username, email, password, authMethod, authId } = req.body;
    try {
        let user = undefined;
        let detail = "";
        if (authMethod === 'local') {
            if (await User.findByEmail(email)) {
                detail = "Email is already used.";
            }
            else if (await User.findByUsername(username)) {
                detail = "Username is taken.";
            }
            else {
                user = new User({
                    username,
                    email,
                    password,
                    authMethod
                })
            }
        }
        else {
            if (await User.findByAuthId(authId)) {
                detail = "AuthID is already used.";
            }
            else if (await User.findByUserName(username)) {
                detail = "Username is taken.";
            }
            else {
                user = new User({
                    username,
                    authId,
                    authMethod
                })
            }
        }
        if (user) {
            await user.save();
            const userResponse = user.toObject();
            delete userResponse.password;
            res.status(201).json(userResponse);
        }
        else {
            res.status(400).json({
                message: "User already exists.",
                detail
            })
        }
    } catch (err) {
        res.status(500).json({ message: err.message });
    }
};

export const login = async (req, res) => {
    const { email, password, authMethod, authId } = req.body;
    try {
        if (authMethod === 'local') {
            const user = await User.findByEmail(email);
            if (!user) {
                return res.status(401).json({ errors: "Invalid email or password."});
            }
            const isValid = await user.comparePassword(password);
            if (!isValid) {
                return res.status(401).json({ errors: "Invalid email or password."});
            }
            const token = jwt.sign(
                { userId: user._id },
                JWT_SECRET,
                { expiresIn: '1h' }
            )
            await Session.createSession(user._id, token);
            res.json({ token });
        }
        else {
            // TODO: WRITE LOGIN FOR OAUTH
        }

    } catch (err) {
        console.error('Login error: ', err);
        res.status(500).json({ message: 'Login failed' });
    }
};