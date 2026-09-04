import { createSlice } from '@reduxjs/toolkit';
import {
    fetchConversations,
    openDirectConversation,
    openTeamGroupConversation,
    openBusinessDirectConversation,
    fetchMessages,
    fetchMissedMessages,
    sendMessage,
    markConversationRead,
} from './chat.actions';

const initialState = {
    conversations: [],
    activeConversationUuid: null,
    messagesByConversation: {},
    socketStatus: 'disconnected', // 'connected' | 'connecting' | 'disconnected'
    typingByConversation: {}, // conversationUuid -> { [personUuid]: expiresAt }
    loadingConversations: false,
    loadingMessages: false,
    sending: false,
    error: null,
};

const upsertConversationFromMessage = (state, conversationUuid, message) => {
    const idx = state.conversations.findIndex((c) => c.uuid === conversationUuid);
    if (idx === -1) return;
    const conversation = state.conversations[idx];
    conversation.lastMessage = {
        messageUuid: message.uuid,
        senderPersonUuid: message.senderPersonUuid,
        senderDisplayName: message.senderDisplayName,
        snippet: message.body || '📷 Photo',
        sentAt: message.sentAt,
        hasMedia: (message.media?.length ?? 0) > 0,
    };
    conversation.updatedAt = message.sentAt;
    // Bubble the conversation to the top.
    state.conversations.splice(idx, 1);
    state.conversations.unshift(conversation);
};

const appendMessage = (state, conversationUuid, message) => {
    const list = state.messagesByConversation[conversationUuid] || [];
    const byUuid = list.some((m) => m.uuid === message.uuid);
    const byClientId =
        message.clientMessageId &&
        list.some((m) => m.clientMessageId && m.clientMessageId === message.clientMessageId);
    if (byUuid || byClientId) {
        state.messagesByConversation[conversationUuid] = list.map((m) =>
            m.clientMessageId && m.clientMessageId === message.clientMessageId ? message : m,
        );
        return;
    }
    state.messagesByConversation[conversationUuid] = [...list, message];
};

const chatSlice = createSlice({
    name: 'chat',
    initialState,
    reducers: {
        setActiveConversation: (state, { payload }) => {
            state.activeConversationUuid = payload;
        },
        socketStatusChanged: (state, { payload }) => {
            state.socketStatus = payload;
        },
        messageReceived: (state, { payload }) => {
            const { conversationUuid, message } = payload;
            appendMessage(state, conversationUuid, message);
            upsertConversationFromMessage(state, conversationUuid, message);
        },
        readReceiptReceived: (state, { payload }) => {
            const { conversationUuid, personUuid, lastReadMessageUuid } = payload;
            const conversation = state.conversations.find((c) => c.uuid === conversationUuid);
            if (!conversation) return;
            const participant = conversation.participants?.find((p) => p.personUuid === personUuid);
            if (participant) {
                participant.lastReadMessageUuid = lastReadMessageUuid;
                participant.lastReadAt = new Date().toISOString();
            }
        },
        typingReceived: (state, { payload }) => {
            const { conversationUuid, personUuid } = payload;
            state.typingByConversation[conversationUuid] = {
                ...(state.typingByConversation[conversationUuid] || {}),
                [personUuid]: Date.now() + 5000,
            };
        },
        clearChatError: (state) => {
            state.error = null;
        },
    },
    extraReducers: (builder) => {
        builder
            .addCase(fetchConversations.pending, (state) => {
                state.loadingConversations = true;
            })
            .addCase(fetchConversations.fulfilled, (state, { payload }) => {
                state.loadingConversations = false;
                state.conversations = Array.isArray(payload) ? payload : [];
            })
            .addCase(fetchConversations.rejected, (state, { payload }) => {
                state.loadingConversations = false;
                state.error = payload;
            });

        const openFulfilled = (state, { payload }) => {
            if (!payload?.uuid) return;
            const idx = state.conversations.findIndex((c) => c.uuid === payload.uuid);
            if (idx === -1) state.conversations.unshift(payload);
            else state.conversations[idx] = payload;
            state.activeConversationUuid = payload.uuid;
        };
        builder
            .addCase(openDirectConversation.fulfilled, openFulfilled)
            .addCase(openTeamGroupConversation.fulfilled, openFulfilled)
            .addCase(openBusinessDirectConversation.fulfilled, openFulfilled)
            .addCase(openDirectConversation.rejected, (state, { payload }) => {
                state.error = payload;
            })
            .addCase(openTeamGroupConversation.rejected, (state, { payload }) => {
                state.error = payload;
            })
            .addCase(openBusinessDirectConversation.rejected, (state, { payload }) => {
                state.error = payload;
            });

        builder
            .addCase(fetchMessages.pending, (state) => {
                state.loadingMessages = true;
            })
            .addCase(fetchMessages.fulfilled, (state, { payload }) => {
                state.loadingMessages = false;
                const { conversationUuid, messages, page } = payload;
                // API returns newest-first; store oldest-first for rendering.
                const ordered = [...messages].reverse();
                if (page === 0) {
                    state.messagesByConversation[conversationUuid] = ordered;
                } else {
                    state.messagesByConversation[conversationUuid] = [
                        ...ordered,
                        ...(state.messagesByConversation[conversationUuid] || []),
                    ];
                }
            })
            .addCase(fetchMessages.rejected, (state, { payload }) => {
                state.loadingMessages = false;
                state.error = payload;
            })
            .addCase(fetchMissedMessages.fulfilled, (state, { payload }) => {
                const { conversationUuid, messages } = payload;
                messages.forEach((m) => appendMessage(state, conversationUuid, m));
            });

        builder
            .addCase(sendMessage.pending, (state) => {
                state.sending = true;
            })
            .addCase(sendMessage.fulfilled, (state, { payload }) => {
                state.sending = false;
                appendMessage(state, payload.conversationUuid, payload.message);
                upsertConversationFromMessage(state, payload.conversationUuid, payload.message);
            })
            .addCase(sendMessage.rejected, (state, { payload }) => {
                state.sending = false;
                state.error = payload;
            });

        builder.addCase(markConversationRead.fulfilled, (state, { payload }) => {
            const conversation = state.conversations.find((c) => c.uuid === payload.conversationUuid);
            if (conversation) conversation.unread = false;
        });
    },
});

export const {
    setActiveConversation,
    socketStatusChanged,
    messageReceived,
    readReceiptReceived,
    typingReceived,
    clearChatError,
} = chatSlice.actions;

export default chatSlice.reducer;
