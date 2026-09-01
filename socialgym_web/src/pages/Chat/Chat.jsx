import { useEffect, useMemo, useRef, useState } from 'react';
import { useDispatch, useSelector } from 'react-redux';
import { useTranslation } from 'react-i18next';
import { AppHeader, Spinner } from '../../commons/gui';
import { decodeToken } from '../../commons/library/tokenUtils';
import {
    fetchConversations,
    fetchMessages,
    sendMessage,
    markConversationRead,
} from '../../redux/reducers/chat/chat.actions';
import { setActiveConversation, clearChatError } from '../../redux/reducers/chat/chat.slice';
import {
    connectChatSocket,
    socketSendMessage,
    socketSendTyping,
    socketSendRead,
} from '../../redux/middleware/chatSocket';
import './Chat.scss';

const conversationTitle = (conversation, myUuid, t) => {
    if (!conversation) return '';
    if (conversation.conversationType === 'BusinessTeamGroup') {
        return conversation.businessProfileName || t('chat.teamGroup');
    }
    if (conversation.conversationType === 'BusinessDirect') {
        return conversation.businessProfileName || t('chat.businessDirect');
    }
    const other = conversation.participants?.find((p) => p.personUuid !== myUuid);
    return conversation.lastMessage?.senderPersonUuid !== myUuid
        ? conversation.lastMessage?.senderDisplayName || other?.personUuid || t('chat.title')
        : other?.personUuid || t('chat.title');
};

export const Chat = () => {
    const { t } = useTranslation();
    const dispatch = useDispatch();
    const person = useSelector((state) => state.person?.person);
    const auth = useSelector((state) => state.auth?.auth);
    const {
        conversations,
        activeConversationUuid,
        messagesByConversation,
        loadingConversations,
        loadingMessages,
        sending,
        socketStatus,
        error,
    } = useSelector((state) => state.chat);

    const myUuid = useMemo(() => {
        if (person?.uuid) return person.uuid;
        const decoded = auth?.accessToken ? decodeToken(auth.accessToken) : null;
        return decoded?.person_uuid || null;
    }, [person, auth]);

    const [draft, setDraft] = useState('');
    const [files, setFiles] = useState([]);
    const fileInputRef = useRef(null);
    const threadEndRef = useRef(null);

    useEffect(() => {
        dispatch(connectChatSocket());
        dispatch(fetchConversations(0));
    }, [dispatch]);

    useEffect(() => {
        if (error) {
            const timer = setTimeout(() => dispatch(clearChatError()), 4000);
            return () => clearTimeout(timer);
        }
    }, [error, dispatch]);

    const activeMessages = activeConversationUuid
        ? messagesByConversation[activeConversationUuid] || []
        : [];

    useEffect(() => {
        if (!activeConversationUuid) return;
        dispatch(fetchMessages({ conversationUuid: activeConversationUuid, page: 0 }));
    }, [activeConversationUuid, dispatch]);

    useEffect(() => {
        threadEndRef.current?.scrollIntoView({ behavior: 'smooth' });
        const last = activeMessages[activeMessages.length - 1];
        if (last && activeConversationUuid && last.senderPersonUuid !== myUuid) {
            dispatch(
                socketSendRead({
                    conversationUuid: activeConversationUuid,
                    lastReadMessageUuid: last.uuid,
                }),
            );
            dispatch(
                markConversationRead({
                    conversationUuid: activeConversationUuid,
                    lastReadMessageUuid: last.uuid,
                }),
            );
        }
    }, [activeMessages, activeConversationUuid, myUuid, dispatch]);

    const selectConversation = (uuid) => dispatch(setActiveConversation(uuid));

    const handleSend = (event) => {
        event.preventDefault();
        if (!activeConversationUuid) return;
        const body = draft.trim();
        if (!body && files.length === 0) return;

        if (socketStatus === 'connected' && files.length === 0) {
            dispatch(socketSendMessage({ conversationUuid: activeConversationUuid, body }));
        } else {
            dispatch(sendMessage({ conversationUuid: activeConversationUuid, body, files }));
        }
        setDraft('');
        setFiles([]);
        if (fileInputRef.current) fileInputRef.current.value = '';
    };

    const handleTyping = (value) => {
        setDraft(value);
        if (activeConversationUuid) {
            dispatch(socketSendTyping(activeConversationUuid));
        }
    };

    const activeConversation = conversations.find((c) => c.uuid === activeConversationUuid);

    return (
        <div className="chat-page">
            <AppHeader person={person} />
            <main className="chat-container">
                <aside className="chat-list">
                    <header className="chat-list__header">
                        <h1>{t('chat.title')}</h1>
                        <span className={`chat-status chat-status--${socketStatus}`}>
                            {socketStatus === 'connected'
                                ? '●'
                                : t('chat.reconnecting')}
                        </span>
                    </header>
                    {loadingConversations && conversations.length === 0 ? (
                        <Spinner />
                    ) : conversations.length === 0 ? (
                        <p className="chat-list__empty">{t('chat.conversationsEmpty')}</p>
                    ) : (
                        <ul>
                            {conversations.map((conversation) => (
                                <li
                                    key={conversation.uuid}
                                    className={`chat-list__item ${
                                        conversation.uuid === activeConversationUuid
                                            ? 'chat-list__item--active'
                                            : ''
                                    } ${conversation.unread ? 'chat-list__item--unread' : ''}`}
                                    onClick={() => selectConversation(conversation.uuid)}
                                >
                                    <span className="chat-list__name">
                                        {conversationTitle(conversation, myUuid, t)}
                                    </span>
                                    <span className="chat-list__preview">
                                        {conversation.lastMessage?.snippet || ''}
                                    </span>
                                </li>
                            ))}
                        </ul>
                    )}
                </aside>

                <section className="chat-thread">
                    {!activeConversationUuid ? (
                        <div className="chat-thread__placeholder">{t('chat.messagePlaceholder')}</div>
                    ) : (
                        <>
                            <header className="chat-thread__header">
                                {conversationTitle(activeConversation, myUuid, t)}
                            </header>
                            <div className="chat-thread__messages">
                                {loadingMessages && activeMessages.length === 0 ? (
                                    <Spinner />
                                ) : (
                                    activeMessages.map((message) => (
                                        <div
                                            key={message.uuid || message.clientMessageId}
                                            className={`chat-bubble ${
                                                message.senderPersonUuid === myUuid
                                                    ? 'chat-bubble--mine'
                                                    : 'chat-bubble--theirs'
                                            }`}
                                        >
                                            {message.senderPersonUuid !== myUuid && (
                                                <span className="chat-bubble__sender">
                                                    {message.senderDisplayName}
                                                    {message.senderKind === 'BusinessProfile' && ' 🏢'}
                                                </span>
                                            )}
                                            {message.body && <p>{message.body}</p>}
                                            {(message.media || []).map((media) => (
                                                <img
                                                    key={media.objectKey}
                                                    src={media.url}
                                                    alt=""
                                                    className="chat-bubble__image"
                                                />
                                            ))}
                                        </div>
                                    ))
                                )}
                                <div ref={threadEndRef} />
                            </div>

                            <form className="chat-composer" onSubmit={handleSend}>
                                <button
                                    type="button"
                                    className="chat-composer__attach"
                                    onClick={() => fileInputRef.current?.click()}
                                    title={t('chat.attachImage')}
                                >
                                    📎
                                </button>
                                <input
                                    ref={fileInputRef}
                                    type="file"
                                    accept="image/*"
                                    multiple
                                    hidden
                                    onChange={(e) => setFiles(Array.from(e.target.files || []))}
                                />
                                {files.length > 0 && (
                                    <span className="chat-composer__files">{files.length} 🖼️</span>
                                )}
                                <input
                                    type="text"
                                    value={draft}
                                    onChange={(e) => handleTyping(e.target.value)}
                                    placeholder={t('chat.messagePlaceholder')}
                                />
                                <button type="submit" disabled={sending}>
                                    {t('chat.send')}
                                </button>
                            </form>
                        </>
                    )}
                    {error && <div className="chat-error">{t('chat.notFriendsError')}</div>}
                </section>
            </main>
        </div>
    );
};

export default Chat;
