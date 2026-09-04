import axios from '../../axios.config';
import { uploadFileToS3 } from '../s3/index.js';

export const listConversationsApi = async (page = 0) => {
    const { data } = await axios.get('/timeline/api/chat/conversations', { params: { page } });
    return Array.isArray(data) ? data : [];
};

export const createDirectConversationApi = async (targetPersonUuid) => {
    const { data } = await axios.post('/timeline/api/chat/conversations/direct', { targetPersonUuid });
    return data;
};

export const createTeamGroupConversationApi = async (businessProfileUuid) => {
    const { data } = await axios.post('/timeline/api/chat/conversations/business-team', {
        businessProfileUuid,
    });
    return data;
};

export const createBusinessDirectConversationApi = async (businessProfileUuid, memberPersonUuid) => {
    const { data } = await axios.post('/timeline/api/chat/conversations/business-direct', {
        businessProfileUuid,
        ...(memberPersonUuid ? { memberPersonUuid } : {}),
    });
    return data;
};

export const listMessagesApi = async (conversationUuid, { page = 0, since } = {}) => {
    const { data } = await axios.get(
        `/timeline/api/chat/conversations/${conversationUuid}/messages`,
        { params: since != null ? { since } : { page } },
    );
    return Array.isArray(data) ? data : [];
};

export const sendMessageApi = async (conversationUuid, { body = '', media = [], clientMessageId }) => {
    const { data } = await axios.post(
        `/timeline/api/chat/conversations/${conversationUuid}/messages`,
        { body, media, clientMessageId },
    );
    return data;
};

export const markConversationReadApi = async (conversationUuid, lastReadMessageUuid) => {
    const { data } = await axios.put(
        `/timeline/api/chat/conversations/${conversationUuid}/read`,
        { lastReadMessageUuid },
    );
    return data;
};

/// Uploads image files to S3 (reusing the shared media-upload endpoint) and
/// returns `[{ mediaType: 'Image', objectKey }]` ready for `sendMessageApi`.
export const uploadChatImages = async (files = []) => {
    const uploaded = [];
    for (const file of files) {
        if (!file.type?.startsWith('image/')) continue;
        const { data } = await axios.get('/api/media/upload', {
            params: { mediaType: 'Image', format: file.type, album: 'chat' },
        });
        await uploadFileToS3(file, data.url);
        uploaded.push({ mediaType: 'Image', objectKey: data.objectKey });
    }
    return uploaded;
};
