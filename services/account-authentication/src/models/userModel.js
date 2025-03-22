import { Schema, model } from 'mongoose';
import { hash, compare } from 'bcrypt';
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

userSchema.methods.comparePassword = async function (password) {
    return await compare(password, this.password);
};

userSchema.static.findByUserId = async function(userId) {
    return await this.findOne({ _id: userId });
}

userSchema.statics.findByEmail = async function(email) {
    return await this.findOne({ email });
};

userSchema.statics.findByUsername = async function(username) {
    return await this.findOne({ username });
}

userSchema.statics.findByAuthId = async function(authId) {
    return await this.findOne({ authId });
}

userSchema.statics.deleteByUserId = async function(userId) {
    return await this.deleteMany({ _id: userId });
}

const User = model('User', userSchema);
export default User;