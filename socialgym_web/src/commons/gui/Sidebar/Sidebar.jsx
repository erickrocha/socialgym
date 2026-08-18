import React from 'react';
import { useNavigate } from 'react-router-dom';
import { useLocation } from 'react-router-dom';
import { useSelector } from 'react-redux';
import { useTranslation } from 'react-i18next';
import PropTypes from 'prop-types';
import Avatar from '../Avatar/Avatar';
import './Sidebar.scss';

const Sidebar = ({ person, isCollapsed = false, onToggle }) => {
  const navigate = useNavigate();
  const location = useLocation();
  const { t } = useTranslation();
  const isBusinessProfileActive = useSelector((state) => !!state.business.selectedProfile);

  const handleAvatarClick = () => {
    navigate('/profile');
  };

  const menuItems = [
    { id: 'home', label: t('sidebar.menu.home'), icon: '🏠', action: () => navigate('/home') },
    { id: 'gallery', label: t('sidebar.menu.gallery'), icon: '🖼️', action: () => navigate('/gallery') },
    { id: 'profile', label: t('sidebar.menu.profile'), icon: '👤', action: () => navigate('/profile') },
    { id: 'workouts', label: t('sidebar.menu.workouts'), icon: '💪', action: () => navigate('/workouts') },
    { id: 'workout-sessions', label: t('sidebar.menu.sessions'), icon: '📊', action: () => navigate('/workout-sessions') },
    // Evolution check-in is personal-only: a business profile has no evolution data of its own.
    ...(!isBusinessProfileActive ? [{ id: 'evolution', label: t('sidebar.menu.evolution'), icon: '📈', action: () => navigate('/evolution') }] : []),
    { id: 'friends', label: t('sidebar.menu.friends'), icon: '👥', action: () => navigate('/friends') },
    ...(person?.hasBusinessProfiles ? [{ id: 'business', label: t('sidebar.menu.business'), icon: '💼', action: () => navigate('/business') }] : []),
    { id: 'settings', label: t('sidebar.menu.settings'), icon: '⚙️', action: () => navigate('/settings') },
  ];

  const avatarSrc = person?.avatar ? person?.avatar : null;
  const name = person ? `${person?.firstname} ${person?.surname}` : t('sidebar.defaultUser');

  return (
    <aside className={`sidebar ${isCollapsed ? 'sidebar--collapsed' : ''}`} role="navigation">
      <div className="sidebar__header">
        <button
          className="sidebar__toggle"
          onClick={onToggle}
          aria-label={t('sidebar.toggleAriaLabel')}
          aria-expanded={!isCollapsed}
        >
          ☰
        </button>
      </div>

      <div className="sidebar__avatar-section">
        <button
          className="sidebar__avatar-button"
          onClick={handleAvatarClick}
          aria-label={t('sidebar.profileAriaLabel')}
          title={t('sidebar.uploadPictureTitle')}
        >
          <Avatar image={avatarSrc} alt={name} size="lg" />
        </button>
        {!isCollapsed && <span className="sidebar__user-name">{name}</span>}
      </div>

      <nav className="sidebar__menu">
        {menuItems.map((item) => (
          <button
            key={item.id}
            className={`sidebar__menu-item ${location.pathname === `/${item.id}` ? 'sidebar__menu-item--active' : ''}`}
            onClick={item.action}
            title={item.label}
            aria-label={item.label}
          >
            <span className="sidebar__menu-icon">{item.icon}</span>
            {!isCollapsed && <span className="sidebar__menu-label">{item.label}</span>}
          </button>
        ))}
      </nav>
    </aside>
  );
};

Sidebar.propTypes = {
  person:PropTypes.object.isRequired,
  isCollapsed: PropTypes.bool,
  onToggle: PropTypes.func,
};

Sidebar.defaultProps = {
  isCollapsed: false,
};

export default Sidebar;

