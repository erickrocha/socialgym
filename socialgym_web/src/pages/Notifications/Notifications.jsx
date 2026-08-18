import { useEffect, useState } from 'react';
import { useTranslation } from 'react-i18next';
import { useDispatch, useSelector } from 'react-redux';
import { useNavigate } from 'react-router';
import { AppHeader, Button, Spinner } from '../../commons/gui';
import { fetchNotifications, markAsRead, markAllAsRead } from '../../redux/reducers/notification/notification.actions';
import './Notifications.scss';

export const Notifications = () => {
    const { t } = useTranslation();
    const dispatch = useDispatch();
    const navigate = useNavigate();
    const person = useSelector((state) => state.person?.person);
    const { notifications, loading } = useSelector((state) => state.notification);

    const [filter, setFilter] = useState('ALL'); // ALL, UNREAD

    useEffect(() => {
        dispatch(fetchNotifications());
    }, [dispatch]);

    const handleItemClick = (item) => {
        if (!item.read) {
            dispatch(markAsRead(item.id));
        }
        if (item.target_url) {
            navigate(item.target_url);
        }
    };

    const handleMarkAllRead = () => {
        dispatch(markAllAsRead());
    };

    const filteredList = notifications.filter(n => {
        if (filter === 'UNREAD') return !n.read;
        return true;
    });

    return (
        <div className="notifications-page">
            <AppHeader person={person} />
            <main className="notifications-container">
                <div className="notifications-header">
                    <div>
                        <h1>{t('notifications.title', 'Notificações')}</h1>
                        <p>{t('notifications.subtitle', 'Acompanhe curtidas, comentários, novos amigos e atualizações')}</p>
                    </div>
                    <Button type="button" variant="secondary" onClick={handleMarkAllRead}>
                        {t('notifications.markAllRead', 'Marcar todas como lidas')}
                    </Button>
                </div>

                <div className="filter-tabs">
                    <button
                        className={`tab-btn ${filter === 'ALL' ? 'active' : ''}`}
                        onClick={() => setFilter('ALL')}
                    >
                        {t('notifications.all', 'Todas')} ({notifications.length})
                    </button>
                    <button
                        className={`tab-btn ${filter === 'UNREAD' ? 'active' : ''}`}
                        onClick={() => setFilter('UNREAD')}
                    >
                        {t('notifications.unread', 'Não lidas')} ({notifications.filter(n => !n.read).length})
                    </button>
                </div>

                {loading ? (
                    <Spinner />
                ) : filteredList.length === 0 ? (
                    <div className="empty-state">
                        <p>{t('notifications.empty', 'Nenhuma notificação para exibir.')}</p>
                    </div>
                ) : (
                    <div className="notifications-list">
                        {filteredList.map((item) => (
                            <div
                                key={item.id}
                                className={`notification-card ${!item.read ? 'unread' : ''}`}
                                onClick={() => handleItemClick(item)}
                            >
                                <div className="notif-icon">🔔</div>
                                <div className="notif-details">
                                    <h4>{item.title || t('notifications.defaultTitle', 'Nova Notificação')}</h4>
                                    <p>{item.message || item.content}</p>
                                    <span className="time">{item.created_at ? new Date(item.created_at).toLocaleString() : ''}</span>
                                </div>
                                {!item.read && <span className="unread-dot" />}
                            </div>
                        ))}
                    </div>
                )}
            </main>
        </div>
    );
};

export default Notifications;
