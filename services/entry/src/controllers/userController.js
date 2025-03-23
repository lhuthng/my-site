import User from "../models/userModel.js";
import Session from "../models/sessionModel.js";
import jwt from 'jsonwebtoken';

const { JWT_SECRET } = process.env;

export const verifyUser = async (req, res) => {
    const { token } = req.params;

    try {
        const decoded = jwt.verify(token, JWT_SECRET);
        const { email, type } = decoded;

        if (type !== 'verification') {
            return res.status(400).json({ error: `Invalid token type. Expected verification but got ${type}.`});
        }

        const user = await User.findByEmail(email);
        if (!user) {
            return res.status(400).json({ error: 'Invalid email.'});
        }

        const isVerified = await user.isVerified();
        if (isVerified) {
            return res.status(400).json({ error: 'User is already verified.'});
        }

        user.verified = true;
        await user.save();

        res.status(201).json({ message: 'User is verifed.'});
    }
    catch (error) {
        console.error('Error: ', error.message);
        res.status(500).json({ message: 'Failed to verify a user.' });
    }
};

export const requestDeletion = async (req, res) => {
    const { userId } = req.user;

    try {
        const user = await User.findByUserId(userId);
        if (!user) {
            return res.status(400).json({ error: 'User Id is invalid.' });
        }
        const deletionToken = jwt.sign(
            { 
                userId,
                type: 'deletion'
            },
            JWT_SECRET,
        )
        console.log(deletionToken);
        // TODO: SEND DELETION TOKEN
        res.status(200).json({ message: 'Deletion token is generated.' });
    } catch (error) {
        console.error('Error: ', error.message);
        res.status(400).json({ message: 'Failed to request a deletion.' });
    }
};

export const confirmDeletion = async (req, res) => {
    const { token } = req.params;

    try {
        const decoded = jwt.verify(token, JWT_SECRET);
        const { userId, type } = decoded;

        if (type !== 'deletion') {
            return res.status(400).json({ error: `Invalid token type. Expected deletion but got ${type}.`});
        }

        const result = await User.deleteByUserId(userId);
        if (!result?.deletedCount) {
            return res.status(400).json({ error: 'User Id is invalid or user is already deleted.' });
        }
        res.status(200).json({ message: 'User is deleted.'});
        
    } catch (error) {
        console.error('Error: ', error.message);
        return res.status(400).json({ message: 'Failed to delete.'});
    }
}