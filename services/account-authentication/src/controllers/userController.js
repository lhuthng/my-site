import User from "../models/userModel.js";

export const createUser = async (req, res) => {
    const { username, email, password, authMethod, authId } = req.body;
    try {
        let user = undefined;
        let detail = "";
        if (authMethod === 'local') {
            if (await User.findOne({ email })) {
                detail = "Email is already used.";
            }
            else if (await User.findOne({ username })) {
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
            if (await User.findOne({ authId })) {
                detail = "AuthID is already used.";
            }
            else if (await User.findOne({ username })) {
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