import React, { useEffect, useState } from 'react';
import { useTranslation } from 'react-i18next';
import { useDispatch } from 'react-redux';
import { Button, TextField, VisibilityDropdown } from '../../../commons/gui/index.js';
import * as handler from '../../../redux/reducers/workout/index.js';
import './ExerciseForm.scss';

const ExerciseForm = ({ workoutId, workoutVisibility, onCancel, onConfirm }) => {
    const { t } = useTranslation('common');
    const dispatch = useDispatch();

    const [exercises, setExercises] = useState([]);
    const [name, setName] = useState('');
    const [description, setDescription] = useState('');
    const [sets, setSets] = useState('');
    const [reps, setReps] = useState('');
    const [visibility, setVisibility] = useState(workoutVisibility || 'Private');

    // Keep the exercise visibility in sync with the workout's own visibility,
    // e.g. if the workout is changed to Public, new exercises default to Public too.
    // The user can still override it per exercise via the dropdown below.
    useEffect(() => {
        if (workoutVisibility) {
            setVisibility(workoutVisibility);
        }
    }, [workoutVisibility]);

    const handleAddExercise = () => {
        if (!name || !sets || !reps) return;

        const newExercise = {
            id: Date.now(),
            name,
            description,
            sets: parseInt(sets),
            reps: parseInt(reps),
            visibility,
        };

        setExercises([...exercises, newExercise]);

        // Clear form
        setName('');
        setDescription('');
        setSets('');
        setReps('');
        setVisibility(workoutVisibility || 'Private');
    };

    const handleRemoveExercise = (id) => {
        setExercises(exercises.filter(ex => ex.id !== id));
    };

    const handleSave = () => {
        if (exercises.length === 0) return;
        const payload = {
            workoutId,
            exercises: exercises.map( item => ({
                name: item.name,
                description: item.description,
                sets: item.sets,
                reps: item.reps,
                visibility: item.visibility,
                workoutId: workoutId,
            }))
        };
        dispatch(handler.addExercises(payload));
        onConfirm();
    };

    const handleCancel = () => {
        onCancel();
    };

    const isAddDisabled = !name || !sets || !reps;

    return (
        <div className="exercise-form">
            <div className="exercise-form__input-section">
                <TextField
                    name="name"
                    label={t('workout.exercise.name')}
                    placeholder={t('workout.exercise.namePlaceholder')}
                    value={name}
                    onChange={(e) => setName(e.target.value)}
                    required
                />
                <TextField
                    name="description"
                    label={t('workout.exercise.description')}
                    placeholder={t('workout.exercise.descriptionPlaceholder')}
                    value={description}
                    onChange={(e) => setDescription(e.target.value)}
                />
                <div className="exercise-form__inline-row">
                    <TextField
                        name="sets"
                        label={t('workout.sets')}
                        placeholder={t('workout.exercise.setsPlaceholder')}
                        type="number"
                        value={sets}
                        onChange={(e) => setSets(e.target.value)}
                        required
                    />
                    <TextField
                        name="reps"
                        label={t('workout.reps')}
                        placeholder={t('workout.exercise.repsPlaceholder')}
                        type="number"
                        value={reps}
                        onChange={(e) => setReps(e.target.value)}
                        required
                    />
                    <button
                        type="button"
                        className="exercise-form__add-btn"
                        onClick={handleAddExercise}
                        disabled={isAddDisabled}
                        title={t('workout.addExercise')}
                    >
                        <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
                            <line x1="12" y1="5" x2="12" y2="19"></line>
                            <line x1="5" y1="12" x2="19" y2="12"></line>
                        </svg>
                    </button>
                </div>
                <VisibilityDropdown
                    name="exerciseVisibility"
                    value={visibility}
                    onChange={setVisibility}
                />
            </div>

            {exercises.length > 0 && (
                <div className="exercise-form__list">
                    <h4 className="exercise-form__list-title">
                        {t('workout.exercisesList')} ({exercises.length})
                    </h4>
                    <ul className="exercise-form__items">
                        {exercises.map((exercise) => (
                            <li key={exercise.id} className="exercise-form__item">
                                <div className="exercise-form__item-info">
                                    <span className="exercise-form__item-name">{exercise.name}</span>
                                    {exercise.description && (
                                        <span className="exercise-form__item-desc">{exercise.description}</span>
                                    )}
                                    <span className="exercise-form__item-details">
                                        {exercise.sets} {t('workout.sets')} × {exercise.reps} {t('workout.reps')}
                                    </span>
                                </div>
                                <button
                                    type="button"
                                    className="exercise-form__remove-btn"
                                    onClick={() => handleRemoveExercise(exercise.id)}
                                    title={t('application.button.cancel')}
                                >
                                    ✕
                                </button>
                            </li>
                        ))}
                    </ul>
                </div>
            )}

            <div className="exercise-form__actions">
                <Button
                    type="button"
                    variant="secondary"
                    onClick={handleCancel}
                >
                    {t('application.button.cancel')}
                </Button>
                <Button
                    type="button"
                    variant="primary"
                    onClick={handleSave}
                    disabled={exercises.length === 0}
                >
                    {t('modal.save')}
                </Button>
            </div>
        </div>
    );
};

export default ExerciseForm;
