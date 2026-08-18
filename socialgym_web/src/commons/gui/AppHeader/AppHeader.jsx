import React, { useState } from 'react';
import {useLocation, useNavigate} from 'react-router-dom';
import {useDispatch, useSelector} from 'react-redux';
import {useTranslation} from 'react-i18next';
import {logout} from '../../../redux/reducers/auth';
import Logo from '../Logo/Logo';
import Avatar from '../Avatar/Avatar';
import Dropdown from '../Dropdown/Dropdown';
import NotificationDropdown from '../../components/NotificationDropdown/NotificationDropdown';
import logoImg from '../../../assets/img/logo.png';
import './AppHeader.scss';

const AppHeader = ({person}) => {
    const {t} = useTranslation('common');
    const navigate = useNavigate();
    const dispatch = useDispatch();
    const location = useLocation();
    const [notifOpen, setNotifOpen] = useState(false);
    const unreadCount = useSelector((state) => state.notification?.unreadCount || 0);
    const isBusinessProfileActive = useSelector((state) => !!state.business.selectedProfile);

    const handleLogout = () => {
        dispatch(logout());
        navigate('/login');
    };

    const handleProfileClick = () => {
        navigate('/profile');
    };

    const handleSettingsClick = () => {
        navigate('/settings');
    };

    const avatarSrc = person?.avatar ? person?.avatar : null;
    const name = person ? `${person?.firstname} ${person?.surname}` : 'User';

    return (
        <header className="app-header" role="banner">
            <div className="app-header__content">
                <div className="app-header__logo-section" onClick={() => navigate('/home')} style={{ cursor: 'pointer' }}>
                    <Logo src={logoImg} alt="SocialGym Logo" className="app-header__logo"/>
                    <span className="app-header__name">SocialGym</span>
                </div>
                <nav className="app-header__nav-section" role="navigation" aria-label="Main navigation">
                    <button
                        className={`app-header__nav-tab ${location.pathname === '/home' ? 'app-header__nav-tab--active' : ''}`}
                        onClick={() => navigate('/home')}
                        title={t('header.homeTitle', 'Início')}
                    >
                        <span className="app-header__nav-icon">🏠</span>
                    </button>
                    <button
                        className={`app-header__nav-tab ${location.pathname === '/gallery' ? 'app-header__nav-tab--active' : ''}`}
                        onClick={() => navigate('/gallery')}
                        title={t('header.galleryTitle', 'Galeria')}
                    >
                        <span className="app-header__nav-icon">🖼️</span>
                    </button>
                    <button
                        className={`app-header__nav-tab ${location.pathname === '/workouts' ? 'app-header__nav-tab--active' : ''}`}
                        onClick={() => navigate('/workouts')}
                        title={t('header.workoutTitle', 'Treinos')}
                    >
                        <span className="app-header__nav-icon">💪</span>
                    </button>
                    <button
                        className={`app-header__nav-tab ${location.pathname === '/exercises' ? 'app-header__nav-tab--active' : ''}`}
                        onClick={() => navigate('/exercises')}
                        title={t('header.exercisesTitle', 'Exercícios')}
                    >
                        <span className="app-header__nav-icon">🏋️</span>
                    </button>
                    <button
                        className={`app-header__nav-tab ${location.pathname === '/friends' ? 'app-header__nav-tab--active' : ''}`}
                        onClick={() => navigate('/friends')}
                        title={t('header.friendsTitle', 'Amigos')}
                    >
                        <span className="app-header__nav-icon">👥</span>
                    </button>
                    <button
                        className={`app-header__nav-tab ${location.pathname === '/workout-sessions' ? 'app-header__nav-tab--active' : ''}`}
                        onClick={() => navigate('/workout-sessions')}
                        title={t('header.sessionsTitle', 'Sessões')}
                    >
                        <span className="app-header__nav-icon">📊</span>
                    </button>
                    {!isBusinessProfileActive && (
                        <button
                            className={`app-header__nav-tab ${location.pathname === '/evolution' ? 'app-header__nav-tab--active' : ''}`}
                            onClick={() => navigate('/evolution')}
                            title={t('header.evolutionTitle', 'Evolução')}
                        >
                            <span className="app-header__nav-icon">📈</span>
                        </button>
                    )}
                </nav>
                <div className="app-header__user-section" style={{ display: 'flex', alignItems: 'center', gap: '1rem', position: 'relative' }}>
                    <button
                        type="button"
                        className="app-header__notif-btn"
                        onClick={() => setNotifOpen(!notifOpen)}
                        style={{ background: 'transparent', border: 'none', cursor: 'pointer', fontSize: '1.2rem', position: 'relative' }}
                    >
                        🔔
                        {unreadCount > 0 && (
                            <span className="notif-badge">{unreadCount}</span>
                        )}
                    </button>
                    <NotificationDropdown isOpen={notifOpen} onClose={() => setNotifOpen(false)} />

                    <Dropdown
                        trigger={
                            <div className="app-header__avatar-wrapper">
                                <Avatar image={avatarSrc} alt={name} size="sm"/>
                                <span className="app-header__user-name">{name}</span>
                            </div>
                        }
                    >
                        <button onClick={handleProfileClick}>{t('header.profile', 'Perfil')}</button>
                        <button onClick={handleSettingsClick}>{t('header.settings', 'Configurações')}</button>
                        <button onClick={handleLogout}>{t('header.logout', 'Sair')}</button>
                    </Dropdown>
                </div>
            </div>
        </header>
    );
};

export default AppHeader;

