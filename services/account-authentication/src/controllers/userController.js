import User from "../models/userModel.js";
import Session from "../models/sessionModel.js";

export const deleteUser = async (req, res) => {
    const { userId } = req.user;

    try {
        const result = await User.deleteByUserId(userId);
        if (result?.deletedCount) {
            res.status(200).json({ message: 'User is deleted.' });
        }
        else {
            return res.status(400).json({ error: 'User Id is invalid.' });
        }
        await Session.deleteByUserId(userId);
    } catch (error) {
        res.status(400).json({ error: 'User Id is invalid.' });
    }
};