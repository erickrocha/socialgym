import React from 'react';
import { useTranslation } from 'react-i18next';
import Button from '../../gui/Button/Button.jsx';
import './ExerciseList.scss';

const ExerciseList = ({ exercises, workoutName, onAddExercise }) => {
    const { t } = useTranslation('common');

    if (!exercises || exercises.length === 0) {
        return (
            <div className="exercise-list exercise-list--empty">
                <div className="exercise-list__empty-icon">🏋️</div>
                <p className="exercise-list__empty-message">
                    {t('workout.noExercises')}
                </p>
                <p className="exercise-list__empty-hint">
                    {t('workout.addExerciseHint')}
                </p>
                {onAddExercise && (
                    <Button
                        variant="primary"
                        className="exercise-list__add-button"
                        onClick={onAddExercise}
                    >
                        + {t('workout.addExercise')}
                    </Button>
                )}
            </div>
        );
    }

    return (
        <div className="exercise-list">
            <div className="exercise-list__header">
                <h3 className="exercise-list__title">
                    {t('workout.exercisesFor')} {workoutName}
                </h3>
                <div className="exercise-list__header-actions">
                    <span className="exercise-list__count">
                        {exercises.length} {t('workout.exercises')}
                    </span>
                    {onAddExercise && (
                        <Button
                            variant="primary"
                            className="exercise-list__add-button exercise-list__add-button--small"
                            onClick={onAddExercise}
                        >
                            + {t('workout.addExercise')}
                        </Button>
                    )}
                </div>
            </div>

            <div className="exercise-list__items">
                {exercises.map((exercise, index) => (
                    <div key={exercise.id} className="exercise-item">
                        <div className="exercise-item__number">
                            {index + 1}
                        </div>
                        <div className="exercise-item__content">
                            <div className="exercise-item__header">
                                <h4 className="exercise-item__name">{exercise.name}</h4>
                            </div>
                            <p className="exercise-item__description">
                                {exercise.description}
                            </p>
                            <div className="exercise-item__details">
                                <div className="exercise-item__detail">
                                    <span className="exercise-item__detail-label">
                                        {t('workout.sets')}
                                    </span>
                                    <span className="exercise-item__detail-value">
                                        {exercise.sets}
                                    </span>
                                </div>
                                <div className="exercise-item__detail">
                                    <span className="exercise-item__detail-label">
                                        {t('workout.reps')}
                                    </span>
                                    <span className="exercise-item__detail-value">
                                        {exercise.reps}
                                    </span>
                                </div>
                                {exercise.weight > 0 && (
                                    <div className="exercise-item__detail">
                                        <span className="exercise-item__detail-label">
                                            {t('workout.weight')}
                                        </span>
                                        <span className="exercise-item__detail-value">
                                            {exercise.weight} {t('workout.weightUnit')}
                                        </span>
                                    </div>
                                )}
                            </div>
                        </div>
                    </div>
                ))}
            </div>
        </div>
    );
};

export default ExerciseList;
