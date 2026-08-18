import React from 'react';
import { useTranslation } from 'react-i18next';
import Button from '../../../commons/gui/Button/Button.jsx';
import './WorkoutComplete.scss';

const WorkoutComplete = ({ workoutSession, onClose, onSave }) => {
    const { t } = useTranslation('common');

    const formatDuration = (seconds) => {
        const mins = Math.floor(seconds / 60);
        const secs = seconds % 60;
        return `${mins}:${secs.toString().padStart(2, '0')}`;
    };

    const getTotalWeight = () => {
        return workoutSession?.executedSets?.reduce((acc, set) => {
            return acc + (set.weight * set.reps);
        }, 0) || 0;
    };

    const getCompletedSets = () => {
        return workoutSession?.executedSets?.length || 0;
    };

    const handleSave = () => {
        if (onSave) {
            onSave(workoutSession);
        }
        onClose();
    };

    return (
        <div className="workout-complete">
            <div className="workout-complete__content">
                {/* Celebration animation */}
                <div className="workout-complete__celebration">
                    <span className="workout-complete__trophy">🏆</span>
                    <div className="workout-complete__confetti">
                        <span>🎉</span>
                        <span>💪</span>
                        <span>⭐</span>
                        <span>🔥</span>
                    </div>
                </div>

                <h2 className="workout-complete__title">
                    {t('workoutExecution.workoutComplete')}
                </h2>

                <p className="workout-complete__workout-name">
                    {workoutSession?.workoutName}
                </p>

                {/* Stats */}
                <div className="workout-complete__stats">
                    <div className="workout-complete__stat">
                        <span className="workout-complete__stat-icon">⏱️</span>
                        <span className="workout-complete__stat-value">
                            {formatDuration(workoutSession?.duration || 0)}
                        </span>
                        <span className="workout-complete__stat-label">
                            {t('workoutExecution.duration')}
                        </span>
                    </div>

                    <div className="workout-complete__stat">
                        <span className="workout-complete__stat-icon">✓</span>
                        <span className="workout-complete__stat-value">
                            {getCompletedSets()}
                        </span>
                        <span className="workout-complete__stat-label">
                            {t('workoutExecution.setsCompleted')}
                        </span>
                    </div>

                    <div className="workout-complete__stat">
                        <span className="workout-complete__stat-icon">🏋️</span>
                        <span className="workout-complete__stat-value">
                            {getTotalWeight().toLocaleString()}
                        </span>
                        <span className="workout-complete__stat-label">
                            {t('workoutExecution.totalVolume')} ({t('workout.weightUnit')})
                        </span>
                    </div>
                </div>

                {/* Motivational message */}
                <div className="workout-complete__message">
                    <p>{t('workoutExecution.greatJob')}</p>
                </div>

                {/* Actions */}
                <div className="workout-complete__actions">
                    <Button
                        variant="secondary"
                        onClick={onClose}
                        className="workout-complete__btn"
                    >
                        {t('workoutExecution.close')}
                    </Button>
                    <Button
                        variant="primary"
                        onClick={handleSave}
                        className="workout-complete__btn workout-complete__btn--primary"
                    >
                        <span>💾</span>
                        {t('workoutExecution.saveSession')}
                    </Button>
                </div>
            </div>
        </div>
    );
};

export default WorkoutComplete;

