import {
    socketStatusChanged,
    messageReceived,
    readReceiptReceived,
    typingReceived,
} from '../reducers/chat/chat.slice';
import { fetchConversations, fetchMissedMessages } from '../reducers/chat/chat.actions';

export const CONNECT_CHAT_SOCKET = 'chat/connectSocket';
export const DISCONNECT_CHAT_SOCKET = 'chat/disconnectSocket';
export const SOCKET_SEND_MESSAGE = 'chat/socketSendMessage';
export const SOCKET_SEND_TYPING = 'chat/socketSendTyping';
export const SOCKET_SEND_READ = 'chat/socketSendRead';

export const connectChatSocket = () => ({ type: CONNECT_CHAT_SOCKET });
export const disconnectChatSocket = () => ({ type: DISCONNECT_CHAT_SOCKET });
export const socketSendMessage = (payload) => ({ type: SOCKET_SEND_MESSAGE, payload });
export const socketSendTyping = (conversationUuid) => ({
    type: SOCKET_SEND_TYPING,
    payload: { conversationUuid },
});
export const socketSendRead = (payload) => ({ type: SOCKET_SEND_READ, payload });

const wsBaseUrl = () => {
    const base = import.meta.env.VITE_API_BASE_URL || 'https://localhost';
    return base.replace(/^http/i, 'ws');
};

const readToken = () => {
    try {
        const auth = JSON.parse(localStorage.getItem('auth'));
        return auth?.accessToken || null;
    } catch {
        return null;
    }
};

export const chatSocketMiddleware = (store) => {
    let socket = null;
    let reconnectDelay = 1000;
    let reconnectTimer = null;
    let manualClose = false;
    let lastSeenSentAtByConversation = {};

    const clearReconnect = () => {
        if (reconnectTimer) {
            clearTimeout(reconnectTimer);
            reconnectTimer = null;
        }
    };

    const scheduleReconnect = () => {
        clearReconnect();
        if (manualClose) return;
        reconnectTimer = setTimeout(connect, reconnectDelay);
        reconnectDelay = Math.min(reconnectDelay * 2, 30000);
    };

    const handleFrame = (frame) => {
        const { dispatch, getState } = store;
        switch (frame.type) {
            case 'message.new': {
                const conversationUuid = frame.conversationUuid;
                const message = frame.message;
                lastSeenSentAtByConversation[conversationUuid] = message.sentAt;
                const known = getState().chat.conversations.some((c) => c.uuid === conversationUuid);
                dispatch(messageReceived({ conversationUuid, message }));
                if (!known) dispatch(fetchConversations(0));
                break;
            }
            case 'message.read':
                dispatch(
                    readReceiptReceived({
                        conversationUuid: frame.conversationUuid,
                        personUuid: frame.personUuid,
                        lastReadMessageUuid: frame.lastReadMessageUuid,
                    }),
                );
                break;
            case 'typing':
                dispatch(
                    typingReceived({
                        conversationUuid: frame.conversationUuid,
                        personUuid: frame.personUuid,
                    }),
                );
                break;
            case 'pong':
            case 'error':
            default:
                break;
        }
    };

    const connect = () => {
        clearReconnect();
        const token = readToken();
        if (!token) return;
        manualClose = false;

        try {
            socket = new WebSocket(
                `${wsBaseUrl()}/timeline/api/chat/ws?access_token=${encodeURIComponent(token)}`,
            );
        } catch {
            scheduleReconnect();
            return;
        }

        store.dispatch(socketStatusChanged('connecting'));

        socket.onopen = () => {
            reconnectDelay = 1000;
            store.dispatch(socketStatusChanged('connected'));
            // Replay anything missed while disconnected.
            Object.entries(lastSeenSentAtByConversation).forEach(([conversationUuid, sentAt]) => {
                const since = Date.parse(sentAt);
                if (!Number.isNaN(since)) {
                    store.dispatch(fetchMissedMessages({ conversationUuid, since }));
                }
            });
            store.dispatch(fetchConversations(0));
        };

        socket.onmessage = (event) => {
            try {
                handleFrame(JSON.parse(event.data));
            } catch {
                /* ignore malformed frame */
            }
        };

        socket.onclose = () => {
            store.dispatch(socketStatusChanged('disconnected'));
            socket = null;
            scheduleReconnect();
        };

        socket.onerror = () => {
            socket?.close();
        };
    };

    const disconnect = () => {
        manualClose = true;
        clearReconnect();
        lastSeenSentAtByConversation = {};
        if (socket) {
            socket.close();
            socket = null;
        }
        store.dispatch(socketStatusChanged('disconnected'));
    };

    const send = (obj) => {
        if (socket && socket.readyState === WebSocket.OPEN) {
            socket.send(JSON.stringify(obj));
            return true;
        }
        return false;
    };

    return (next) => (action) => {
        switch (action.type) {
            case CONNECT_CHAT_SOCKET:
                connect();
                return;
            case DISCONNECT_CHAT_SOCKET:
                disconnect();
                return;
            case 'auth/logout':
                disconnect();
                break;
            case SOCKET_SEND_MESSAGE: {
                const ok = send({ type: 'send', ...action.payload });
                if (!ok) {
                    // Fall back to REST when the socket is down.
                    import('../reducers/chat/chat.actions').then(({ sendMessage }) => {
                        store.dispatch(sendMessage(action.payload));
                    });
                }
                return;
            }
            case SOCKET_SEND_TYPING:
                send({ type: 'typing', ...action.payload });
                return;
            case SOCKET_SEND_READ:
                if (!send({ type: 'read', ...action.payload })) {
                    import('../reducers/chat/chat.actions').then(({ markConversationRead }) => {
                        store.dispatch(markConversationRead(action.payload));
                    });
                }
                return;
            default:
                break;
        }
        return next(action);
    };
};

export default chatSocketMiddleware;
