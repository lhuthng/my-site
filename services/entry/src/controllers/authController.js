import Session from "../models/sessionModel.js";
import User from "../models/userModel.js";
import jwt from 'jsonwebtoken';

const { JWT_SECRET } = process.env;

export const createUser = async (req, res) => {
    const { username, email, password, authMethod, authId } = req.body;
    try {
        let user;
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
                    authMethod,
                    verified: false
                });
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
                    authMethod,
                    verified: true
                })
            }
        }
        if (user) {
            await user.save();
            const userResponse = user.toObject();
            delete userResponse.password;
            if (authMethod === 'local') {
                // TODO: send this token to the email to verify the user
                const token = jwt.sign(
                    {
                        email,
                        type: 'verification'
                    },
                    JWT_SECRET
                )
                console.log(token);
            }
            res.status(201).json(userResponse);
        }
        else {
            res.status(400).json({
                message: "User already exists.",
                detail
            })
        }
    } catch (error) {
        console.error('Error: ', error.message);
        res.status(500).json({ message: 'Failed to create a user' });
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
            const isVerified = await user.isVerified();
            if (!isVerified) {
                return res.status(401).json({ errors: "The account is not verified."});
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

    } catch (error) {
        console.error('Error: ', error);
        res.status(500).json({ message: 'Failed to login' });
    }
};