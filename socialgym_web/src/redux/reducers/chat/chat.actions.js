import { createAsyncThunk } from '@reduxjs/toolkit';
import {
    listConversationsApi,
    createDirectConversationApi,
    createTeamGroupConversationApi,
    createBusinessDirectConversationApi,
    listMessagesApi,
    sendMessageApi,
    markConversationReadApi,
    uploadChatImages,
} from '../../../service/chat/chat.service';

const rejected = (error, { rejectWithValue }) =>
    rejectWithValue(error.response?.data || error.message);

const newClientMessageId = () =>
    (crypto?.randomUUID ? crypto.randomUUID() : `cm-${Date.now()}-${Math.random()}`);

export const fetchConversations = createAsyncThunk(
    'chat/fetchConversations',
    async (page = 0, thunkApi) => {
        try {
            return await listConversationsApi(page);
        } catch (error) {
            return rejected(error, thunkApi);
        }
    },
);

export const openDirectConversation = createAsyncThunk(
    'chat/openDirectConversation',
    async (targetPersonUuid, thunkApi) => {
        try {
            return await createDirectConversationApi(targetPersonUuid);
        } catch (error) {
            return rejected(error, thunkApi);
        }
    },
);

export const openTeamGroupConversation = createAsyncThunk(
    'chat/openTeamGroupConversation',
    async (businessProfileUuid, thunkApi) => {
        try {
            return await createTeamGroupConversationApi(businessProfileUuid);
        } catch (error) {
            return rejected(error, thunkApi);
        }
    },
);

export const openBusinessDirectConversation = createAsyncThunk(
    'chat/openBusinessDirectConversation',
    async ({ businessProfileUuid, memberPersonUuid } = {}, thunkApi) => {
        try {
            return await createBusinessDirectConversationApi(businessProfileUuid, memberPersonUuid);
        } catch (error) {
            return rejected(error, thunkApi);
        }
    },
);

export const fetchMessages = createAsyncThunk(
    'chat/fetchMessages',
    async ({ conversationUuid, page = 0 }, thunkApi) => {
        try {
            const messages = await listMessagesApi(conversationUuid, { page });
            return { conversationUuid, messages, page };
        } catch (error) {
            return rejected(error, thunkApi);
        }
    },
);

export const fetchMissedMessages = createAsyncThunk(
    'chat/fetchMissedMessages',
    async ({ conversationUuid, since }, thunkApi) => {
        try {
            const messages = await listMessagesApi(conversationUuid, { since });
            return { conversationUuid, messages };
        } catch (error) {
            return rejected(error, thunkApi);
        }
    },
);

export const sendMessage = createAsyncThunk(
    'chat/sendMessage',
    async ({ conversationUuid, body = '', files = [] }, thunkApi) => {
        try {
            const media = files.length ? await uploadChatImages(files) : [];
            const clientMessageId = newClientMessageId();
            const message = await sendMessageApi(conversationUuid, { body, media, clientMessageId });
            return { conversationUuid, message };
        } catch (error) {
            return rejected(error, thunkApi);
        }
    },
);

export const markConversationRead = createAsyncThunk(
    'chat/markConversationRead',
    async ({ conversationUuid, lastReadMessageUuid }, thunkApi) => {
        try {
            await markConversationReadApi(conversationUuid, lastReadMessageUuid);
            return { conversationUuid, lastReadMessageUuid };
        } catch (error) {
            return rejected(error, thunkApi);
        }
    },
);
