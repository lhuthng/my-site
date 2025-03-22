import { Schema, model } from 'mongoose';
import { hash } from 'bcrypt';
import { AUTH_METHODS } from '../configs/db.js';

const saltRounds = 10;

const userSchema = new Schema({
    username: {
        type: String,
        unique: true,
        required: true
    },
    email: {
        type: String,
        unique: true,
        sparse: true,
        lowercase: true,
        trim: true,
        required: function() { return this.authMethod == 'local'; }
    },
    password: {
        type: String,
        required: function() { return this.authMethod == 'local'; }
    },
    authId: {
        type: String,
        unique: true,
        sparse: true
    },
    authMethod: {
        type: String,
        enum: AUTH_METHODS,
        required: true,
        default: 'local'
    },
    createAt: {
        type: Date,
        default: Date.now
    }
});

userSchema.pre('save', async function(next) {
    if (this.authMethod !== 'local' || !this.isModified('password')) {
        return next();
    }
    try {
        this.password = await hash(this.password, saltRounds);
        next();
    } catch (err) {
        next(err);
    }
});

userSchema.statics.findByEmail = async function (email) {
    return this.findOne({ email });
};

const User = model('User', userSchema);
export default User;