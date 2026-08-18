import React, {useState} from 'react';
import {useTranslation} from 'react-i18next';
import PropTypes from 'prop-types';
import './BusinessSidebar.scss';

const BusinessSidebar = ({activeSection, onSectionChange, onBack}) => {
    const {t} = useTranslation('common');
    const [isCollapsed, setIsCollapsed] = useState(false);

    const sectionItems = [
        {id: 'about', icon: 'ℹ️', label: t('business.sidebarSections.about')},
        {id: 'services', icon: '🏋️', label: t('business.sidebarSections.services')},
        {id: 'schedule', icon: '📅', label: t('business.sidebarSections.schedule')},
        {id: 'team', icon: '👥', label: t('business.sidebarSections.team')},
        {id: 'photos', icon: '📷', label: t('business.sidebarSections.photos')},
    ];

    const actionItems = [
        {id: 'edit', icon: '✏️', label: t('business.sidebarActions.edit'), action: 'edit'},
        {id: 'settings', icon: '⚙️', label: t('business.sidebarActions.settings'), action: 'settings'},
    ];

    const toggleCollapse = () => {
        setIsCollapsed(!isCollapsed);
    };

    return (
        <aside className={`business-sidebar ${isCollapsed ? 'business-sidebar--collapsed' : ''}`}>
            <button
                className="business-sidebar__toggle"
                onClick={toggleCollapse}
                aria-label={t('business.toggleMenu')}
            >
                {isCollapsed ? '→' : '←'}
            </button>

            {!isCollapsed && (
                <button className="business-sidebar__back-btn" onClick={onBack}>
                    ← {t('business.backToList')}
                </button>
            )}

            {isCollapsed && (
                <button
                    className="business-sidebar__back-btn business-sidebar__back-btn--icon"
                    onClick={onBack}
                    title={t('business.backToList')}
                >
                    ←
                </button>
            )}

            <div className="business-sidebar__section">
                <h3 className="business-sidebar__title">
                    {!isCollapsed && t('business.sidebarSections.title')}
                </h3>
                <nav className="business-sidebar__nav">
                    {sectionItems.map((item) => (
                        <button
                            key={item.id}
                            className={`business-sidebar__item ${activeSection === item.id ? 'business-sidebar__item--active' : ''}`}
                            onClick={() => onSectionChange(item.id)}
                            title={isCollapsed ? item.label : undefined}
                        >
                            <span className="business-sidebar__icon">{item.icon}</span>
                            {!isCollapsed && (
                                <span className="business-sidebar__label">{item.label}</span>
                            )}
                        </button>
                    ))}
                </nav>
            </div>

            <div className="business-sidebar__divider" />

            <div className="business-sidebar__section">
                <h3 className="business-sidebar__title">
                    {!isCollapsed && t('business.sidebarActions.title')}
                </h3>
                <nav className="business-sidebar__nav">
                    {actionItems.map((item) => (
                        <button
                            key={item.id}
                            className="business-sidebar__item"
                            onClick={() => onSectionChange(item.action)}
                            title={isCollapsed ? item.label : undefined}
                        >
                            <span className="business-sidebar__icon">{item.icon}</span>
                            {!isCollapsed && (
                                <span className="business-sidebar__label">{item.label}</span>
                            )}
                        </button>
                    ))}
                </nav>
            </div>
        </aside>
    );
};

BusinessSidebar.propTypes = {
    activeSection: PropTypes.string.isRequired,
    onSectionChange: PropTypes.func.isRequired,
    onBack: PropTypes.func.isRequired,
};

export default BusinessSidebar;

