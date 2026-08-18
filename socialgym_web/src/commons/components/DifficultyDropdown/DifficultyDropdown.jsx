import React, { useState, useRef, useEffect } from 'react';
import ReactDOM from 'react-dom';
import PropTypes from 'prop-types';
import { useTranslation } from 'react-i18next';
import './DifficultyDropdown.scss';

const DifficultyDropdown = ({ value, onChange, name }) => {
    const { t } = useTranslation('common');
    const [isOpen, setIsOpen] = useState(false);
    const [menuPosition, setMenuPosition] = useState({ top: 0, left: 0, width: 0 });
    const dropdownRef = useRef(null);
    const triggerRef = useRef(null);
    const menuRef = useRef(null);

    const difficultyOptions = [
        {
            value: 'Soft',
            label: t('workout.difficulty.soft'),
            icon: '🌱',
            description: t('difficulty.softDescription')
        },
        {
            value: 'Easy',
            label: t('workout.difficulty.easy'),
            icon: '🟢',
            description: t('difficulty.easyDescription')
        },
        {
            value: 'Medium',
            label: t('workout.difficulty.medium'),
            icon: '🟡',
            description: t('difficulty.mediumDescription')
        },
        {
            value: 'Hard',
            label: t('workout.difficulty.hard'),
            icon: '🔴',
            description: t('difficulty.hardDescription')
        },
        {
            value: 'Strong',
            label: t('workout.difficulty.strong'),
            icon: '💪',
            description: t('difficulty.strongDescription')
        }
    ];

    const selectedOption = difficultyOptions.find(opt => opt.value === value) || difficultyOptions[0];

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
            className="difficulty-dropdown__menu"
            role="listbox"
            style={{
                position: 'fixed',
                top: menuPosition.top,
                left: menuPosition.left,
                width: menuPosition.width,
                zIndex: 9999
            }}
        >
            {difficultyOptions.map((option) => (
                <li
                    key={option.value}
                    className={`difficulty-dropdown__item ${value === option.value ? 'difficulty-dropdown__item--selected' : ''}`}
                    role="option"
                    aria-selected={value === option.value}
                    onClick={() => handleSelect(option)}
                >
                    <span className="difficulty-dropdown__item-icon">{option.icon}</span>
                    <div className="difficulty-dropdown__item-content">
                        <span className="difficulty-dropdown__item-label">{option.label}</span>
                        <span className="difficulty-dropdown__item-description">{option.description}</span>
                    </div>
                    {value === option.value && (
                        <span className="difficulty-dropdown__item-check">✓</span>
                    )}
                </li>
            ))}
        </ul>,
        document.body
    ) : null;

    return (
        <div className="difficulty-dropdown" ref={dropdownRef}>
            <button
                type="button"
                ref={triggerRef}
                className="difficulty-dropdown__trigger"
                onClick={toggleDropdown}
                aria-haspopup="listbox"
                aria-expanded={isOpen}
                aria-label={t('difficulty.selectDifficulty')}
            >
                <span className="difficulty-dropdown__icon">{selectedOption.icon}</span>
                <span className="difficulty-dropdown__label">{selectedOption.label}</span>
                <span className="difficulty-dropdown__arrow">{isOpen ? '▲' : '▼'}</span>
            </button>

            {menu}

            <input type="hidden" name={name} value={value} />
        </div>
    );
};

DifficultyDropdown.propTypes = {
    value: PropTypes.string,
    onChange: PropTypes.func.isRequired,
    name: PropTypes.string
};

DifficultyDropdown.defaultProps = {
    value: 'medium',
    name: 'difficulty'
};

export default DifficultyDropdown;

