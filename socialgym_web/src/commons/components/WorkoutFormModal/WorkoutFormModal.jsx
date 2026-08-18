import { useState, useEffect } from 'react';
import { useTranslation } from 'react-i18next';
import { Modal, TextField, Button, VisibilityDropdown, DifficultyDropdown } from '../../gui';
import './WorkoutFormModal.scss';

export const WorkoutFormModal = ({
    isOpen,
    onClose,
    onSubmit,
    availableExercises = [],
    initialData = null,
    loading = false
}) => {
    const { t } = useTranslation();
    const [name, setName] = useState('');
    const [description, setDescription] = useState('');
    const [difficulty, setDifficulty] = useState('Medium');
    const [visibility, setVisibility] = useState('Public');
    const [muscleGroup, setMuscleGroup] = useState('Full Body');
    const [selectedExercises, setSelectedExercises] = useState([]);
    const [exercisePickerOpen, setExercisePickerOpen] = useState(false);

    useEffect(() => {
        if (initialData) {
            setName(initialData.name || '');
            setDescription(initialData.description || '');
            setDifficulty(initialData.difficulty || 'Medium');
            setVisibility(initialData.visibility || 'Public');
            setMuscleGroup(initialData.muscleGroup || initialData.muscle_group || 'Full Body');
            setSelectedExercises(initialData.exercises || []);
        } else {
            setName('');
            setDescription('');
            setDifficulty('Medium');
            setVisibility('Public');
            setMuscleGroup('Full Body');
            setSelectedExercises([]);
        }
    }, [initialData, isOpen]);

    const handleAddExerciseToWorkout = (ex) => {
        const newItem = {
            ...ex,
            sets: ex.sets || 3,
            reps_or_duration: ex.reps_or_duration || 10
        };
        setSelectedExercises(prev => [...prev, newItem]);
        setExercisePickerOpen(false);
    };

    const handleRemoveExercise = (index) => {
        setSelectedExercises(prev => prev.filter((_, i) => i !== index));
    };

    const handleUpdateExerciseParam = (index, field, val) => {
        setSelectedExercises(prev => {
            const next = [...prev];
            next[index] = { ...next[index], [field]: Number(val) };
            return next;
        });
    };

    const handleSubmit = (e) => {
        e.preventDefault();
        if (!name.trim()) return;
        onSubmit({
            name,
            description,
            difficulty,
            visibility,
            muscle_group: muscleGroup,
            exercises: selectedExercises
        });
    };

    if (!isOpen) return null;

    return (
        <Modal
            isOpen={isOpen}
            onClose={onClose}
            title={initialData ? t('workout.editTitle', 'Editar Treino') : t('workout.createTitle', 'Criar Novo Treino')}
        >
            <form onSubmit={handleSubmit} className="workout-form-modal">
                <TextField
                    label={t('workout.nameLabel', 'Nome do Treino')}
                    value={name}
                    onChange={(e) => setName(e.target.value)}
                    required
                    placeholder={t('workout.namePlaceholder', 'Ex: Treino Hipertrofia A')}
                />
                <TextField
                    label={t('workout.descriptionLabel', 'Descrição')}
                    value={description}
                    onChange={(e) => setDescription(e.target.value)}
                    placeholder={t('workout.descriptionPlaceholder', 'Foco em peito e tríceps')}
                />

                <div className="form-row">
                    <div className="form-group">
                        <label>{t('workout.muscleGroupLabel', 'Grupo Muscular')}</label>
                        <select
                            className="gui-select-custom"
                            value={muscleGroup}
                            onChange={(e) => setMuscleGroup(e.target.value)}
                        >
                            <option value="Full Body">{t('workout.groups.fullBody', 'Corpo Inteiro')}</option>
                            <option value="Chest">{t('workout.groups.chest', 'Peito')}</option>
                            <option value="Back">{t('workout.groups.back', 'Costas')}</option>
                            <option value="Legs">{t('workout.groups.legs', 'Pernas')}</option>
                            <option value="Shoulders">{t('workout.groups.shoulders', 'Ombros')}</option>
                            <option value="Arms">{t('workout.groups.arms', 'Braços')}</option>
                            <option value="Core">{t('workout.groups.core', 'Abdômen')}</option>
                        </select>
                    </div>
                    <div className="form-group">
                        <label>{t('workout.difficultyLabel', 'Dificuldade')}</label>
                        <DifficultyDropdown
                            value={difficulty}
                            onChange={setDifficulty}
                        />
                    </div>
                </div>

                <div className="form-group">
                    <label>{t('workout.visibilityLabel', 'Visibilidade')}</label>
                    <VisibilityDropdown
                        value={visibility}
                        onChange={setVisibility}
                    />
                </div>

                <div className="exercises-section">
                    <div className="exercises-header">
                        <h4>{t('workout.exercisesListTitle', 'Exercícios do Treino')} ({selectedExercises.length})</h4>
                        <Button
                            type="button"
                            variant="secondary"
                            onClick={() => setExercisePickerOpen(true)}
                        >
                            + {t('workout.addExerciseBtn', 'Adicionar Exercício')}
                        </Button>
                    </div>

                    {selectedExercises.length === 0 ? (
                        <p className="no-exercises-text">
                            {t('workout.noExercisesSelected', 'Nenhum exercício adicionado a este treino ainda.')}
                        </p>
                    ) : (
                        <div className="workout-exercises-list">
                            {selectedExercises.map((ex, idx) => (
                                <div key={idx} className="workout-exercise-item">
                                    <div className="ex-info">
                                        <span className="ex-name">{ex.name}</span>
                                        <span className="ex-category">({ex.category || 'Geral'})</span>
                                    </div>
                                    <div className="ex-inputs">
                                        <label>{t('exercises.sets', 'Séries')}:</label>
                                        <input
                                            type="number"
                                            value={ex.sets || 3}
                                            min={1}
                                            onChange={(e) => handleUpdateExerciseParam(idx, 'sets', e.target.value)}
                                        />
                                        <label>{t('exercises.reps', 'Reps')}:</label>
                                        <input
                                            type="number"
                                            value={ex.reps_or_duration || ex.repsOrDuration || 10}
                                            min={1}
                                            onChange={(e) => handleUpdateExerciseParam(idx, 'reps_or_duration', e.target.value)}
                                        />
                                        <button
                                            type="button"
                                            className="btn-remove-ex"
                                            onClick={() => handleRemoveExercise(idx)}
                                        >
                                            ✕
                                        </button>
                                    </div>
                                </div>
                            ))}
                        </div>
                    )}
                </div>

                {exercisePickerOpen && (
                    <Modal
                        isOpen={exercisePickerOpen}
                        onClose={() => setExercisePickerOpen(false)}
                        title={t('workout.pickerTitle', 'Selecionar Exercício')}
                    >
                        <div className="exercise-picker-list">
                            {availableExercises.length === 0 ? (
                                <p className="no-data">{t('exercises.noAvailable', 'Nenhum exercício cadastrado na biblioteca.')}</p>
                            ) : (
                                availableExercises.map(ex => (
                                    <div key={ex.id} className="picker-item" onClick={() => handleAddExerciseToWorkout(ex)}>
                                        <div className="picker-info">
                                            <strong>{ex.name}</strong>
                                            <span>{ex.category} • {ex.sets}x{ex.reps_or_duration}</span>
                                        </div>
                                        <Button type="button" variant="primary" size="small">{t('workout.select', 'Selecionar')}</Button>
                                    </div>
                                ))
                            )}
                        </div>
                    </Modal>
                )}

                <div className="modal-actions">
                    <Button type="button" variant="secondary" onClick={onClose}>
                        {t('application.button.cancel', 'Cancelar')}
                    </Button>
                    <Button type="submit" disabled={loading || !name.trim()}>
                        {loading ? t('application.loading', 'Salvando...') : t('application.button.confirm', 'Salvar Treino')}
                    </Button>
                </div>
            </form>
        </Modal>
    );
};

export default WorkoutFormModal;
