import { Schema, model } from 'mongoose';

const sessionDuration = 60; // minutes

const sessionSchema = new Schema({
    userId: {
        type: Schema.Types.ObjectId,
        ref: 'User',
        required: true
    },
    token: {
        type: String,
        required: true
    },
    expiredAt: {
        type: Date,
        required: true
    }
})

sessionSchema.statics.findValidSession = async function(userId, token) {
    return await this.findOne({
        userId,
        token,
        expiredAt: { $gt: new Date() }
    });
}

sessionSchema.statics.createSession = async function(userId, token) {
    await this.deleteMany({ userId });
    return await this.create({ userId, token, expiredAt: Date.now() + sessionDuration * 60000});
};

sessionSchema.statics.deleteByUserId = async function(userId) {
    return await this.deleteMany({ userId });
}

const Session = model('Session', sessionSchema);
export default Session;