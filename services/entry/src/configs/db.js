import mongoose from 'mongoose';

const { MONGO_URI } = process.env;

if (!MONGO_URI) {
    console.error("MONGO_URI is not defined in the environment variables.");
    process.exit(1);
}

export const AUTH_METHODS = ['local', 'google', 'facebook'];

export const connectDB = async () => {
    try {
        await mongoose.connect(MONGO_URI);
        console.log("MongoDB connected!");
    } catch(err) {
        console.log("MongoDB connection error: ", err);
        process.exit(1);
    }
};