import React, { useState, useEffect } from 'react';
import { useTranslation } from 'react-i18next';
import { useDispatch, useSelector } from 'react-redux';
import { useNavigate } from 'react-router';
import Button from '../../../commons/gui/Button/Button.jsx';
import {
    confirmSet as confirmSetAction,
    skipSet as skipSetAction,
    finishExecution,
    endExecution,
} from '../../../redux/reducers/workoutExecution/index.js';
import './WorkoutExecution.scss';

const WorkoutExecution = () => {
    const { t } = useTranslation('common');
    const dispatch = useDispatch();
    const navigate = useNavigate();

    const {
        workout,
        currentExerciseIndex,
        currentSetIndex,
        executedSets,
        startedAt,
    } = useSelector((state) => state.workoutExecution);

    const [isEditMode, setIsEditMode] = useState(false);
    const [editValues, setEditValues] = useState({
        weight: 0,
        reps: 0
    });
    const [elapsedTime, setElapsedTime] = useState(0);

    const exercises = workout?.exercises || [];
    const currentExercise = exercises[currentExerciseIndex];
    const totalSets = currentExercise?.sets || 0;
    const isLastExercise = currentExerciseIndex === exercises.length - 1;
    const isLastSet = currentSetIndex === totalSets - 1;

    useEffect(() => {
        const timer = setInterval(() => {
            if (startedAt) {
                setElapsedTime(Math.floor((Date.now() - startedAt) / 1000));
            }
        }, 1000);

        return () => clearInterval(timer);
    }, [startedAt]);

    useEffect(() => {
        if (currentExercise) {
            setEditValues({
                weight: currentExercise.weight || 0,
                reps: currentExercise.reps || 0
            });
        }
    }, [currentExercise, currentSetIndex]);

    const formatTime = (seconds) => {
        const mins = Math.floor(seconds / 60);
        const secs = seconds % 60;
        return `${mins.toString().padStart(2, '0')}:${secs.toString().padStart(2, '0')}`;
    };

    const getProgress = () => {
        if (exercises.length === 0) return 0;
        const totalSetsAll = exercises.reduce((acc, ex) => acc + (ex.sets || 0), 0);
        const completedSets = executedSets.length;
        return Math.round((completedSets / totalSetsAll) * 100);
    };

    const buildSession = (finalSets) => ({
        workoutId: workout.id,
        workoutName: workout.name,
        startTime: startedAt,
        endTime: Date.now(),
        duration: elapsedTime,
        executedSets: finalSets
    });

    const handleConfirmSet = () => {
        const executedSet = {
            exerciseId: currentExercise.id,
            exerciseName: currentExercise.name,
            category: currentExercise.category,
            setNumber: currentSetIndex + 1,
            weight: editValues.weight,
            reps: editValues.reps,
            completedAt: new Date().toISOString()
        };

        dispatch(confirmSetAction(executedSet));
        setIsEditMode(false);

        if (isLastSet && isLastExercise) {
            dispatch(finishExecution(buildSession([...executedSets, executedSet])));
        }
    };

    const handleSkipSet = () => {
        if (isLastSet && isLastExercise) {
            dispatch(finishExecution(buildSession(executedSets)));
        } else {
            dispatch(skipSetAction());
        }
    };

    const handleClose = () => {
        dispatch(endExecution());
        navigate('/workouts');
    };

    const handleEditToggle = () => {
        setIsEditMode(!isEditMode);
    };

    const handleInputChange = (field, value) => {
        setEditValues(prev => ({
            ...prev,
            [field]: parseFloat(value) || 0
        }));
    };

    if (!workout || exercises.length === 0) {
        return (
            <div className="workout-execution">
                <div className="workout-execution__empty">
                    <span className="workout-execution__empty-icon">🏋️</span>
                    <p>{t('workoutExecution.noExercises')}</p>
                    <Button variant="secondary" onClick={handleClose}>
                        {t('application.button.cancel')}
                    </Button>
                </div>
            </div>
        );
    }

    return (
        <div className="workout-execution">
            {/* Header with timer and progress */}
            <div className="workout-execution__header">
                <button className="workout-execution__close" onClick={handleClose}>
                    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
                        <path d="M19 12H5M12 19l-7-7 7-7" />
                    </svg>
                </button>
                <div className="workout-execution__title">
                    <h2>{workout.name}</h2>
                    <span className="workout-execution__timer">⏱️ {formatTime(elapsedTime)}</span>
                </div>
                <div className="workout-execution__progress-wrapper">
                    <div className="workout-execution__progress-bar">
                        <div
                            className="workout-execution__progress-fill"
                            style={{ width: `${getProgress()}%` }}
                        />
                    </div>
                    <span className="workout-execution__progress-text">{getProgress()}%</span>
                </div>
            </div>

            {/* Exercise indicator */}
            <div className="workout-execution__exercise-nav">
                {exercises.map((ex, idx) => (
                    <div
                        key={ex.id}
                        className={`workout-execution__exercise-dot ${
                            idx < currentExerciseIndex ? 'workout-execution__exercise-dot--completed' : ''
                        } ${idx === currentExerciseIndex ? 'workout-execution__exercise-dot--active' : ''}`}
                    />
                ))}
            </div>

            {/* Current exercise card */}
            <div className="workout-execution__current-exercise">
                <div className="workout-execution__exercise-header">
                    <span className="workout-execution__exercise-number">
                        {t('workoutExecution.exercise')} {currentExerciseIndex + 1}/{exercises.length}
                    </span>
                    <h3 className="workout-execution__exercise-name">{currentExercise.name}</h3>
                    <p className="workout-execution__exercise-description">{currentExercise.description}</p>
                </div>

                {/* Set indicator */}
                <div className="workout-execution__sets-indicator">
                    {Array.from({ length: totalSets }).map((_, idx) => (
                        <div
                            key={idx}
                            className={`workout-execution__set-dot ${
                                idx < currentSetIndex ? 'workout-execution__set-dot--completed' : ''
                            } ${idx === currentSetIndex ? 'workout-execution__set-dot--active' : ''}`}
                        >
                            {idx + 1}
                        </div>
                    ))}
                </div>

                {/* Current set details */}
                <div className="workout-execution__set-card">
                    <div className="workout-execution__set-header">
                        <span className="workout-execution__set-title">
                            {t('workoutExecution.set')} {currentSetIndex + 1} {t('workoutExecution.of')} {totalSets}
                        </span>
                        <button
                            className="workout-execution__edit-btn"
                            onClick={handleEditToggle}
                            title={t('workoutExecution.editValues')}
                        >
                            {isEditMode ? '✓' : '✏️'}
                        </button>
                    </div>

                    <div className="workout-execution__values">
                        {/* Weight */}
                        <div className="workout-execution__value-box">
                            <span className="workout-execution__value-icon">🏋️</span>
                            <span className="workout-execution__value-label">{t('workout.weight')}</span>
                            {isEditMode ? (
                                <input
                                    type="number"
                                    className="workout-execution__value-input"
                                    value={editValues.weight}
                                    onChange={(e) => handleInputChange('weight', e.target.value)}
                                    min="0"
                                    step="0.5"
                                />
                            ) : (
                                <span className="workout-execution__value-number">
                                    {editValues.weight} {t('workout.weightUnit')}
                                </span>
                            )}
                        </div>

                        {/* Reps */}
                        <div className="workout-execution__value-box">
                            <span className="workout-execution__value-icon">🔄</span>
                            <span className="workout-execution__value-label">{t('workout.reps')}</span>
                            {isEditMode ? (
                                <input
                                    type="number"
                                    className="workout-execution__value-input"
                                    value={editValues.reps}
                                    onChange={(e) => handleInputChange('reps', e.target.value)}
                                    min="0"
                                />
                            ) : (
                                <span className="workout-execution__value-number">
                                    {editValues.reps}
                                </span>
                            )}
                        </div>
                    </div>

                    {/* Action buttons */}
                    <div className="workout-execution__actions">
                        <Button
                            variant="secondary"
                            className="workout-execution__skip-btn"
                            onClick={handleSkipSet}
                        >
                            {t('workoutExecution.skip')}
                        </Button>
                        <Button
                            variant="primary"
                            className="workout-execution__confirm-btn"
                            onClick={handleConfirmSet}
                        >
                            <span className="workout-execution__confirm-icon">✓</span>
                            {t('workoutExecution.confirmSet')}
                        </Button>
                    </div>
                </div>

                {/* Motivational message */}
                <div className="workout-execution__motivation">
                    <span className="workout-execution__motivation-icon">💪</span>
                    <span className="workout-execution__motivation-text">
                        {isLastSet && isLastExercise
                            ? t('workoutExecution.almostDone')
                            : t('workoutExecution.keepGoing')
                        }
                    </span>
                </div>
            </div>

            {/* Completed sets summary — tap to see all, grouped by exercise */}
            {executedSets.length > 0 && (
                <button
                    type="button"
                    className="workout-execution__completed"
                    onClick={() => navigate(`/workouts/execution/${workout.id}/completed`)}
                >
                    <h4 className="workout-execution__completed-title">
                        <span>{t('workoutExecution.completedSets')} ({executedSets.length})</span>
                        {executedSets.length > 3 && (
                            <span className="workout-execution__completed-viewall">
                                {t('workoutExecution.viewAllSets')} →
                            </span>
                        )}
                    </h4>
                    <div className="workout-execution__completed-list">
                        {executedSets.slice(-3).map((set, idx) => (
                            <div key={idx} className="workout-execution__completed-item">
                                <span className="workout-execution__completed-check">✓</span>
                                <span className="workout-execution__completed-name">{set.exerciseName}</span>
                                <span className="workout-execution__completed-details">
                                    {t('workoutExecution.set')} {set.setNumber} • {set.weight}{t('workout.weightUnit')} × {set.reps}
                                </span>
                            </div>
                        ))}
                    </div>
                </button>
            )}
        </div>
    );
};

export default WorkoutExecution;
