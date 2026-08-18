import React from 'react';
import { useTranslation } from 'react-i18next';
import './StartWorkoutButton.scss';

const StartWorkoutButton = ({ onClick, disabled, className = '' }) => {
    const { t } = useTranslation('common');

    return (
        <button
            className={`start-workout-btn ${className} ${disabled ? 'start-workout-btn--disabled' : ''}`}
            onClick={onClick}
            disabled={disabled}
            title={t('workoutExecution.startWorkout')}
        >
            <div className="start-workout-btn__icon">
                {/* Running person with energy icon */}
                <svg viewBox="0 0 64 64" fill="none" xmlns="http://www.w3.org/2000/svg">
                    {/* Runner body */}
                    <circle cx="38" cy="12" r="6" fill="currentColor"/>
                    <path
                        d="M45 22L52 28M45 22L40 30L32 35L38 45L30 52M45 22L35 25L28 35"
                        stroke="currentColor"
                        strokeWidth="4"
                        strokeLinecap="round"
                        strokeLinejoin="round"
                    />
                    <path
                        d="M32 35L22 40M40 30L50 35"
                        stroke="currentColor"
                        strokeWidth="4"
                        strokeLinecap="round"
                        strokeLinejoin="round"
                    />
                    {/* Energy sparks */}
                    <path
                        d="M18 15L20 18L17 20"
                        stroke="#FFD700"
                        strokeWidth="2"
                        strokeLinecap="round"
                        strokeLinejoin="round"
                        className="start-workout-btn__spark"
                    />
                    <path
                        d="M55 18L58 20L56 23"
                        stroke="#FFD700"
                        strokeWidth="2"
                        strokeLinecap="round"
                        strokeLinejoin="round"
                        className="start-workout-btn__spark start-workout-btn__spark--delay"
                    />
                    <circle cx="12" cy="25" r="2" fill="#FFD700" className="start-workout-btn__spark"/>
                    <circle cx="58" cy="30" r="2" fill="#FFD700" className="start-workout-btn__spark start-workout-btn__spark--delay"/>
                </svg>
            </div>
            <span className="start-workout-btn__text">
                {t('workoutExecution.startWorkout')}
            </span>
            <span className="start-workout-btn__emoji">🚀</span>
        </button>
    );
};

export default StartWorkoutButton;

