import React, {useEffect, useState} from 'react';
import {useDispatch, useSelector} from 'react-redux';
import {useNavigate} from 'react-router-dom';
import {useTranslation} from 'react-i18next';
import {AppHeader, FriendCard, Sidebar, Spinner, Toast} from '../../commons/gui/index.js';
import {
    acceptFriendRequest,
    denyFriendRequest,
    getFriends,
    sendFriendRequest
} from '../../redux/reducers/friend/index.js';
import './Friends.scss';

const Friends = () => {
    const {t} = useTranslation('common');
    const dispatch = useDispatch();
    const navigate = useNavigate();

    const {person} = useSelector((state) => state.person);
    const {friends, suggestions, receiveRequests, sentRequests, loading, error} = useSelector((state) => state.friend);

    const [isSidebarCollapsed, setIsSidebarCollapsed] = useState(false);
    const [toastMessage, setToastMessage] = useState(null);
    const [activeTab, setActiveTab] = useState('all');

    useEffect(() => {
        dispatch(getFriends());
    }, [dispatch]);

    useEffect(() => {
        if (error) {
            setToastMessage({type: 'error', message: error.message || t('friends.errorLoading')});
        }
    }, [error, t]);

    const handleToggleSidebar = () => {
        setIsSidebarCollapsed(!isSidebarCollapsed);
    };

    const handleAddFriend = async (friend) => {
        try {
            await dispatch(sendFriendRequest(friend.id)).unwrap();
            setToastMessage({type: 'success', message: t('friends.friendAdded')});
            dispatch(getFriends());
        } catch (err) {
            setToastMessage({type: 'error', message: err.message || t('friends.errorAddingFriend')});
        }
    };

    const handleAcceptRequest = async (friend) => {
        try {
            await dispatch(acceptFriendRequest(friend.id)).unwrap();
            setToastMessage({type: 'success', message: t('friends.requestAccepted')});
            dispatch(getFriends());
        } catch (err) {
            setToastMessage({type: 'error', message: err.message || t('friends.errorAcceptingRequest')});
        }
    };

    const handleDeclineRequest = async (friend) => {
        try {
            await dispatch(denyFriendRequest(friend.id)).unwrap();
            setToastMessage({type: 'info', message: t('friends.requestDeclined')});
            dispatch(getFriends());
        } catch (err) {
            setToastMessage({type: 'error', message: err.message || t('friends.errorDecliningRequest')});
        }
    };

    const handleCancelRequest = async (friend) => {
        try {
            // TODO: Implement cancel friend request action
            setToastMessage({type: 'info', message: t('friends.requestCancelled')});
            dispatch(getFriends());
        } catch (err) {
            setToastMessage({type: 'error', message: err.message || t('friends.errorCancellingRequest')});
        }
    };

    const handleRemoveFriend = async (friend) => {
        try {
            // TODO: Implement remove friend action
            setToastMessage({type: 'info', message: t('friends.friendRemoved')});
            dispatch(getFriends());
        } catch (err) {
            setToastMessage({type: 'error', message: err.message || t('friends.errorRemovingFriend')});
        }
    };

    const handleViewProfile = (friend) => {
        navigate(`/profile/${friend.id}`);
    };

    const handleCloseToast = () => {
        setToastMessage(null);
    };

    const tabs = [
        {id: 'all', label: t('friends.tabs.all'), icon: '👥', count: friends?.length || 0},
        {id: 'suggestions', label: t('friends.tabs.suggestions'), icon: '✨', count: suggestions?.length || 0},
        {id: 'received', label: t('friends.tabs.received'), icon: '📩', count: receiveRequests?.length || 0},
        {id: 'sent', label: t('friends.tabs.sent'), icon: '📤', count: sentRequests?.length || 0},
    ];

    return (
        <div className="friends-page">
            <AppHeader person={person}/>

            <div className="friends-page__layout">
                <Sidebar
                    person={person}
                    isCollapsed={isSidebarCollapsed}
                    onToggle={handleToggleSidebar}
                />

                <main className="friends-page__content">
                    <div className="friends-page__header">
                        <h1 className="friends-page__title">{t('friends.title')}</h1>
                        <p className="friends-page__subtitle">{t('friends.subtitle')}</p>
                    </div>

                    {/* Tabs Navigation */}
                    <div className="friends-page__tabs">
                        {tabs.map((tab) => (
                            <button
                                key={tab.id}
                                className={`friends-page__tab ${activeTab === tab.id ? 'friends-page__tab--active' : ''}`}
                                onClick={() => setActiveTab(tab.id)}
                            >
                                <span className="friends-page__tab-icon">{tab.icon}</span>
                                <span className="friends-page__tab-label">{tab.label}</span>
                                {tab.count > 0 && (
                                    <span className="friends-page__tab-count">{tab.count}</span>
                                )}
                            </button>
                        ))}
                    </div>

                    <div className="friends-page__sections">
                        {/* Friend Suggestions Box */}
                        {(activeTab === 'all' || activeTab === 'suggestions') && (
                            <section className="friends-box friends-box--suggestions">
                                <div className="friends-box__header">
                                    <h2 className="friends-box__title">
                                        <span className="friends-box__icon">✨</span>
                                        {t('friends.peopleYouMayKnow')}
                                    </h2>
                                    {suggestions.length > 0 && (
                                        <span className="friends-box__count">{suggestions.length}</span>
                                    )}
                                </div>

                                <div className="friends-box__content">
                                    {loading ? (
                                        <div className="friends-box__loading">
                                            <Spinner/>
                                        </div>
                                    ) : suggestions.length > 0 ? (
                                        <div className="friends-box__grid">
                                            {suggestions.map((suggestion) => (
                                                <FriendCard
                                                    key={suggestion.id}
                                                    friend={suggestion}
                                                    variant="suggestion"
                                                    onAddFriend={handleAddFriend}
                                                    onViewProfile={handleViewProfile}
                                                    loading={loading}
                                                />
                                            ))}
                                        </div>
                                    ) : (
                                        <div className="friends-box__empty">
                                            <span className="friends-box__empty-icon">🔍</span>
                                            <p>{t('friends.noSuggestions')}</p>
                                        </div>
                                    )}
                                </div>
                            </section>
                        )}

                        {/* Received Friend Requests Box */}
                        {(activeTab === 'all' || activeTab === 'received') && (
                            <section className="friends-box friends-box--received">
                                <div className="friends-box__header">
                                    <h2 className="friends-box__title">
                                        <span className="friends-box__icon">📩</span>
                                        {t('friends.receivedRequests')}
                                    </h2>
                                    {receiveRequests.length > 0 && (
                                        <span className="friends-box__count">{receiveRequests.length}</span>
                                    )}
                                </div>

                                <div className="friends-box__content">
                                    {loading ? (
                                        <div className="friends-box__loading">
                                            <Spinner/>
                                        </div>
                                    ) : receiveRequests.length > 0 ? (
                                        <div className="friends-box__grid">
                                            {receiveRequests.map((request) => (
                                                <FriendCard
                                                    key={request.id}
                                                    friend={request}
                                                    variant="receivedRequest"
                                                    onAcceptRequest={handleAcceptRequest}
                                                    onDeclineRequest={handleDeclineRequest}
                                                    onViewProfile={handleViewProfile}
                                                    loading={loading}
                                                />
                                            ))}
                                        </div>
                                    ) : (
                                        <div className="friends-box__empty">
                                            <span className="friends-box__empty-icon">📭</span>
                                            <p>{t('friends.noReceivedRequests')}</p>
                                        </div>
                                    )}
                                </div>
                            </section>
                        )}

                        {/* Sent Friend Requests Box */}
                        {(activeTab === 'all' || activeTab === 'sent') && (
                            <section className="friends-box friends-box--sent">
                                <div className="friends-box__header">
                                    <h2 className="friends-box__title">
                                        <span className="friends-box__icon">📤</span>
                                        {t('friends.sentRequests')}
                                    </h2>
                                    {sentRequests.length > 0 && (
                                        <span className="friends-box__count">{sentRequests.length}</span>
                                    )}
                                </div>

                                <div className="friends-box__content">
                                    {loading ? (
                                        <div className="friends-box__loading">
                                            <Spinner/>
                                        </div>
                                    ) : sentRequests.length > 0 ? (
                                        <div className="friends-box__grid">
                                            {sentRequests.map((request) => (
                                                <FriendCard
                                                    key={request.id}
                                                    friend={request}
                                                    variant="sentRequest"
                                                    onCancelRequest={handleCancelRequest}
                                                    onViewProfile={handleViewProfile}
                                                    loading={loading}
                                                />
                                            ))}
                                        </div>
                                    ) : (
                                        <div className="friends-box__empty">
                                            <span className="friends-box__empty-icon">📮</span>
                                            <p>{t('friends.noSentRequests')}</p>
                                        </div>
                                    )}
                                </div>
                            </section>
                        )}

                        {/* My Friends Box */}
                        {activeTab === 'all' && (
                            <section className="friends-box friends-box--friends">
                                <div className="friends-box__header">
                                    <h2 className="friends-box__title">
                                        <span className="friends-box__icon">👥</span>
                                        {t('friends.myFriends')}
                                    </h2>
                                    {friends.length > 0 && (
                                        <span className="friends-box__count">{friends.length}</span>
                                    )}
                                </div>

                                <div className="friends-box__content">
                                    {loading ? (
                                        <div className="friends-box__loading">
                                            <Spinner/>
                                        </div>
                                    ) : friends.length > 0 ? (
                                        <div className="friends-box__grid">
                                            {friends.map((friend) => (
                                                <FriendCard
                                                    key={friend.id}
                                                    friend={friend}
                                                    variant="friend"
                                                    onRemoveFriend={handleRemoveFriend}
                                                    onViewProfile={handleViewProfile}
                                                    loading={loading}
                                                />
                                            ))}
                                        </div>
                                    ) : (
                                        <div className="friends-box__empty">
                                            <span className="friends-box__empty-icon">👋</span>
                                            <p>{t('friends.noFriends')}</p>
                                            <span
                                                className="friends-box__empty-hint">{t('friends.noFriendsHint')}</span>
                                        </div>
                                    )}
                                </div>
                            </section>
                        )}
                    </div>
                </main>
            </div>

            {toastMessage && (
                <Toast
                    type={toastMessage.type}
                    message={toastMessage.message}
                    onClose={handleCloseToast}
                />
            )}
        </div>
    );
};

export default Friends;
