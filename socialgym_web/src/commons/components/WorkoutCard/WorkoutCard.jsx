import React from 'react';
import { useTranslation } from 'react-i18next';
import './WorkoutCard.scss';

const WorkoutCard = ({ workout, isSelected, onSelect }) => {
    const { t } = useTranslation('common');

    const getDifficultyClass = (difficulty) => {
        switch (difficulty?.toLowerCase()) {
            case 'soft':
                return 'workout-card__difficulty--soft';
            case 'easy':
                return 'workout-card__difficulty--easy';
            case 'medium':
                return 'workout-card__difficulty--medium';
            case 'hard':
                return 'workout-card__difficulty--hard';
            case 'strong':
                return 'workout-card__difficulty--strong';
            default:
                return '';
        }
    };

    const getVisibilityIcon = (visibility) => {
        switch (visibility) {
            case 'Private':
                return '🔒';
            case 'Friends':
                return '👥';
            case 'Public':
                return '🌍';
            default:
                return '🔒';
        }
    };

    const getVisibilityTooltip = (visibility) => {
        switch (visibility) {
            case 'Private':
                return t('visibility.privateDescription');
            case 'Friends':
                return t('visibility.friendsDescription');
            case 'Public':
                return t('visibility.publicDescription');
            default:
                return '';
        }
    };

    const getMuscleGroupIcon = (muscleGroup) => {
        switch (muscleGroup?.toLowerCase()) {
            case 'chest':
                return '';
            case 'legs':
                return '🦵';
            case 'back':
                return '🔙';
            case 'core':
                return '🎯';
            case 'full_body':
                return '🏃';
            case 'arms':
                return '💪'
            default:
                return '🏋️';
        }
    };

    return (
        <div
            className={`workout-card ${isSelected ? 'workout-card--selected' : ''}`}
            onClick={() => onSelect(workout)}
            role="button"
            tabIndex={0}
            onKeyDown={(e) => e.key === 'Enter' && onSelect(workout)}
        >
            <div className="workout-card__header">
                <span className="workout-card__icon">
                    {getMuscleGroupIcon(workout.muscleGroup)}
                </span>
                <div className="workout-card__title-wrapper">
                    <h3 className="workout-card__title">{workout.name}</h3>
                    <span className={`workout-card__difficulty ${getDifficultyClass(workout.difficulty)}`}>
                        {t(`workout.difficulty.${workout.difficulty?.toLowerCase()}`)}
                    </span>
                </div>
                <span
                    className="workout-card__visibility"
                    title={getVisibilityTooltip(workout.visibility)}
                >
                    {getVisibilityIcon(workout.visibility)}
                </span>
            </div>

            <p className="workout-card__description">{workout.description}</p>

            <div className="workout-card__meta">
                {workout.muscleGroup && workout.muscleGroup.split("|").map(item => (
                    <div key={item} className="workout-card__meta-item">
                        <span className="workout-card__meta-icon">🎯</span>
                        <span className="workout-card__meta-text">
                        {t(`workout.muscleGroups.${item}`)}
                    </span>
                    </div>
                ))}
                <div className="workout-card__meta-item">
                    <span className="workout-card__meta-icon">📋</span>
                    <span className="workout-card__meta-text">
                        {workout.exercises?.length || 0} {t('workout.exercises')}
                    </span>
                </div>
            </div>

            {isSelected && (
                <div className="workout-card__expand-indicator">
                    <span>▼</span>
                </div>
            )}
        </div>
    );
};

export default WorkoutCard;
