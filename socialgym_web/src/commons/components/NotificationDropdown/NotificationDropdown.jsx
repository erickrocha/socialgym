import { useEffect } from 'react';
import { useTranslation } from 'react-i18next';
import { useDispatch, useSelector } from 'react-redux';
import { useNavigate } from 'react-router';
import { fetchNotifications, markAsRead, markAllAsRead } from '../../../redux/reducers/notification/notification.actions';
import './NotificationDropdown.scss';

export const NotificationDropdown = ({ isOpen, onClose }) => {
    const { t } = useTranslation();
    const dispatch = useDispatch();
    const navigate = useNavigate();
    const { notifications, loading } = useSelector((state) => state.notification);

    useEffect(() => {
        if (isOpen) {
            dispatch(fetchNotifications());
        }
    }, [isOpen, dispatch]);

    if (!isOpen) return null;

    const handleNotificationClick = (item) => {
        if (!item.read) {
            dispatch(markAsRead(item.id));
        }
        onClose();
        if (item.target_url) {
            navigate(item.target_url);
        } else {
            navigate('/notifications');
        }
    };

    const handleMarkAllRead = () => {
        dispatch(markAllAsRead());
    };

    return (
        <div className="notification-dropdown">
            <div className="dropdown-header">
                <h3>{t('notifications.title', 'Notificações')}</h3>
                <button type="button" className="btn-mark-all" onClick={handleMarkAllRead}>
                    {t('notifications.markAllRead', 'Marcar todas como lidas')}
                </button>
            </div>
            <div className="dropdown-body">
                {loading ? (
                    <div className="loading-state">{t('application.loading', 'Carregando...')}</div>
                ) : notifications.length === 0 ? (
                    <div className="empty-state">{t('notifications.empty', 'Nenhuma notificação encontrada.')}</div>
                ) : (
                    notifications.slice(0, 5).map((item) => (
                        <div
                            key={item.id}
                            className={`notification-item ${!item.read ? 'unread' : ''}`}
                            onClick={() => handleNotificationClick(item)}
                        >
                            <div className="notif-content">
                                <p className="notif-title">{item.title || item.message}</p>
                                <span className="notif-time">{item.created_at ? new Date(item.created_at).toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' }) : ''}</span>
                            </div>
                        </div>
                    ))
                )}
            </div>
            <div className="dropdown-footer">
                <button
                    type="button"
                    onClick={() => {
                        onClose();
                        navigate('/notifications');
                    }}
                >
                    {t('notifications.viewAll', 'Ver todas as notificações')}
                </button>
            </div>
        </div>
    );
};

export default NotificationDropdown;
