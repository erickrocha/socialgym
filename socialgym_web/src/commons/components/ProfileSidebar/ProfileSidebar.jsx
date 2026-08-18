import React, { useState } from 'react';
import { useTranslation } from 'react-i18next';
import { useNavigate } from 'react-router-dom';
import './ProfileSidebar.scss';

const ProfileSidebar = ({ activeSection, onSectionChange }) => {
    const { t } = useTranslation('common');
    const navigate = useNavigate();
    const [isCollapsed, setIsCollapsed] = useState(false);

    const menuItems = [
        {
            id: 'about',
            icon: '👤',
            label: t('profileSidebar.about'),
            section: 'about'
        },
        {
            id: 'photos',
            icon: '📷',
            label: t('profileSidebar.photos'),
            section: 'photos'
        },
        {
            id: 'friends',
            icon: '👥',
            label: t('profileSidebar.friends'),
            section: 'friends'
        },
        {
            id: 'workouts',
            icon: '🏋️',
            label: t('profileSidebar.workouts'),
            section: 'workouts'
        },
        {
            id: 'achievements',
            icon: '🏆',
            label: t('profileSidebar.achievements'),
            section: 'achievements'
        },
        {
            id: 'stats',
            icon: '📊',
            label: t('profileSidebar.stats'),
            section: 'stats'
        }
    ];

    const actionItems = [
        {
            id: 'editProfile',
            icon: '✏️',
            label: t('profileSidebar.editProfile'),
            action: 'edit'
        },
        {
            id: 'privacy',
            icon: '🔒',
            label: t('profileSidebar.privacy'),
            action: 'privacy'
        },
        {
            id: 'activityLog',
            icon: '📋',
            label: t('profileSidebar.activityLog'),
            action: 'activity'
        },
        {
            id: 'business',
            icon: '💼',
            label: t('profileSidebar.business'),
            action: 'business'
        }
    ];

    const handleSectionClick = (section) => {
        if (onSectionChange) {
            onSectionChange(section);
        }
    };

    const handleActionClick = (action) => {
        switch (action) {
            case 'edit':
                if (onSectionChange) {
                    onSectionChange('edit');
                }
                break;
            case 'privacy':
                navigate('/settings/privacy');
                break;
            case 'activity':
                navigate('/activity');
                break;
            case 'business':
                navigate('/business');
                break;
            default:
                break;
        }
    };

    const toggleCollapse = () => {
        setIsCollapsed(!isCollapsed);
    };

    return (
        <aside className={`profile-sidebar ${isCollapsed ? 'profile-sidebar--collapsed' : ''}`}>
            <button
                className="profile-sidebar__toggle"
                onClick={toggleCollapse}
                aria-label={t('profileSidebar.toggleMenu')}
            >
                {isCollapsed ? '→' : '←'}
            </button>

            <nav className="profile-sidebar__nav">
                <div className="profile-sidebar__section">
                    <h3 className="profile-sidebar__section-title">
                        {!isCollapsed && t('profileSidebar.sections')}
                    </h3>
                    <ul className="profile-sidebar__menu">
                        {menuItems.map((item) => (
                            <li key={item.id}>
                                <button
                                    className={`profile-sidebar__item ${activeSection === item.section ? 'profile-sidebar__item--active' : ''}`}
                                    onClick={() => handleSectionClick(item.section)}
                                    title={isCollapsed ? item.label : undefined}
                                >
                                    <span className="profile-sidebar__icon">{item.icon}</span>
                                    {!isCollapsed && (
                                        <span className="profile-sidebar__label">{item.label}</span>
                                    )}
                                </button>
                            </li>
                        ))}
                    </ul>
                </div>

                <div className="profile-sidebar__divider" />

                <div className="profile-sidebar__section">
                    <h3 className="profile-sidebar__section-title">
                        {!isCollapsed && t('profileSidebar.actions')}
                    </h3>
                    <ul className="profile-sidebar__menu">
                        {actionItems.map((item) => (
                            <li key={item.id}>
                                <button
                                    className="profile-sidebar__item"
                                    onClick={() => handleActionClick(item.action)}
                                    title={isCollapsed ? item.label : undefined}
                                >
                                    <span className="profile-sidebar__icon">{item.icon}</span>
                                    {!isCollapsed && (
                                        <span className="profile-sidebar__label">{item.label}</span>
                                    )}
                                </button>
                            </li>
                        ))}
                    </ul>
                </div>
            </nav>
        </aside>
    );
};

export default ProfileSidebar;
