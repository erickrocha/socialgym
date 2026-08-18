import React, { useState, useRef, useEffect } from 'react';
import ReactDOM from 'react-dom';
import PropTypes from 'prop-types';
import { useTranslation } from 'react-i18next';
import './VisibilityDropdown.scss';

const VisibilityDropdown = ({ value, onChange, name }) => {
    const { t } = useTranslation('common');
    const [isOpen, setIsOpen] = useState(false);
    const [menuPosition, setMenuPosition] = useState({ top: 0, left: 0, width: 0 });
    const dropdownRef = useRef(null);
    const triggerRef = useRef(null);
    const menuRef = useRef(null);

    const visibilityOptions = [
        {
            value: 'Private',
            label: t('workout.visibility.Private'),
            icon: '🔒',
            description: t('visibility.privateDescription')
        },
        {
            value: 'Friends',
            label: t('workout.visibility.Friends'),
            icon: '👥',
            description: t('visibility.friendsDescription')
        },
        {
            value: 'Public',
            label: t('workout.visibility.Public'),
            icon: '🌍',
            description: t('visibility.publicDescription')
        }
    ];

    const selectedOption = visibilityOptions.find(opt => opt.value === value) || visibilityOptions[0];

    const handleSelect = (option) => {
        onChange(option.value);
        setIsOpen(false);
    };

    const toggleDropdown = () => {
        if (!isOpen && triggerRef.current) {
            const rect = triggerRef.current.getBoundingClientRect();
            setMenuPosition({
                top: rect.bottom + 4,
                left: rect.left,
                width: rect.width
            });
        }
        setIsOpen(!isOpen);
    };

    useEffect(() => {
        const handleClickOutside = (event) => {
            const isOutsideDropdown = dropdownRef.current && !dropdownRef.current.contains(event.target);
            const isOutsideMenu = menuRef.current && !menuRef.current.contains(event.target);

            if (isOutsideDropdown && isOutsideMenu) {
                setIsOpen(false);
            }
        };

        document.addEventListener('mousedown', handleClickOutside);
        return () => {
            document.removeEventListener('mousedown', handleClickOutside);
        };
    }, []);

    const menu = isOpen ? ReactDOM.createPortal(
        <ul
            ref={menuRef}
            className="visibility-dropdown__menu"
            role="listbox"
            style={{
                position: 'fixed',
                top: menuPosition.top,
                left: menuPosition.left,
                width: menuPosition.width,
                zIndex: 9999
            }}
        >
            {visibilityOptions.map((option) => (
                <li
                    key={option.value}
                    className={`visibility-dropdown__item ${value === option.value ? 'visibility-dropdown__item--selected' : ''}`}
                    role="option"
                    aria-selected={value === option.value}
                    onClick={() => handleSelect(option)}
                >
                    <span className="visibility-dropdown__item-icon">{option.icon}</span>
                    <div className="visibility-dropdown__item-content">
                        <span className="visibility-dropdown__item-label">{option.label}</span>
                        <span className="visibility-dropdown__item-description">{option.description}</span>
                    </div>
                    {value === option.value && (
                        <span className="visibility-dropdown__item-check">✓</span>
                    )}
                </li>
            ))}
        </ul>,
        document.body
    ) : null;

    return (
        <div className="visibility-dropdown" ref={dropdownRef}>
            <button
                type="button"
                ref={triggerRef}
                className="visibility-dropdown__trigger"
                onClick={toggleDropdown}
                aria-haspopup="listbox"
                aria-expanded={isOpen}
                aria-label={t('visibility.selectVisibility')}
            >
                <span className="visibility-dropdown__icon">{selectedOption.icon}</span>
                <span className="visibility-dropdown__label">{selectedOption.label}</span>
                <span className="visibility-dropdown__arrow">{isOpen ? '▲' : '▼'}</span>
            </button>

            {menu}

            <input type="hidden" name={name} value={value} />
        </div>
    );
};

VisibilityDropdown.propTypes = {
    value: PropTypes.string,
    onChange: PropTypes.func.isRequired,
    name: PropTypes.string
};

VisibilityDropdown.defaultProps = {
    value: 'Private',
    name: 'visibility'
};

export default VisibilityDropdown;

