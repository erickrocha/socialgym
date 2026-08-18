import { useState, useEffect } from 'react';
import { useTranslation } from 'react-i18next';
import { useDispatch, useSelector } from 'react-redux';
import { AppHeader, Button, TextField, Spinner } from '../../commons/gui';
import { ExerciseModal } from '../../commons/components/ExerciseModal';
import { fetchExercises, createExercise, updateExercise, deleteExercise } from '../../redux/reducers/exercise/exercise.actions';
import './Exercises.scss';

export const Exercises = () => {
    const { t } = useTranslation();
    const dispatch = useDispatch();
    const person = useSelector((state) => state.person?.person);
    const { exercises, loading } = useSelector((state) => state.exercise);

    const [selectedCategory, setSelectedCategory] = useState('ALL');
    const [searchTerm, setSearchTerm] = useState('');
    const [isModalOpen, setIsModalOpen] = useState(false);
    const [editingExercise, setEditingExercise] = useState(null);

    useEffect(() => {
        dispatch(fetchExercises());
    }, [dispatch]);

    const handleCreateOrUpdate = async (exerciseData) => {
        if (editingExercise) {
            await dispatch(updateExercise({ id: editingExercise.id, data: exerciseData }));
        } else {
            await dispatch(createExercise(exerciseData));
        }
        setIsModalOpen(false);
        setEditingExercise(null);
    };

    const handleDelete = async (id) => {
        if (window.confirm(t('exercises.deleteConfirm', 'Tem certeza que deseja excluir este exercício?'))) {
            await dispatch(deleteExercise(id));
        }
    };

    const categories = [
        { key: 'ALL', label: t('exercises.categories.all', 'Todos') },
        { key: 'Chest', label: t('exercises.categories.chest', 'Peito') },
        { key: 'Back', label: t('exercises.categories.back', 'Costas') },
        { key: 'Legs', label: t('exercises.categories.legs', 'Pernas') },
        { key: 'Shoulders', label: t('exercises.categories.shoulders', 'Ombros') },
        { key: 'Arms', label: t('exercises.categories.arms', 'Braços') },
        { key: 'Core', label: t('exercises.categories.core', 'Abdômen') },
        { key: 'Cardio', label: t('exercises.categories.cardio', 'Cardio') },
    ];

    const filteredExercises = exercises.filter(ex => {
        const matchesCategory = selectedCategory === 'ALL' || ex.category === selectedCategory;
        const matchesSearch = ex.name?.toLowerCase().includes(searchTerm.toLowerCase()) ||
            ex.description?.toLowerCase().includes(searchTerm.toLowerCase());
        return matchesCategory && matchesSearch;
    });

    return (
        <div className="exercises-page">
            <AppHeader person={person} />
            <main className="exercises-container">
                <div className="exercises-header">
                    <div>
                        <h1>{t('exercises.pageTitle', 'Biblioteca de Exercícios')}</h1>
                        <p>{t('exercises.pageSubtitle', 'Explore, crie e gerencie os exercícios da sua rotina')}</p>
                    </div>
                    <Button
                        type="button"
                        variant="primary"
                        onClick={() => {
                            setEditingExercise(null);
                            setIsModalOpen(true);
                        }}
                    >
                        + {t('exercises.newExerciseBtn', 'Novo Exercício')}
                    </Button>
                </div>

                <div className="exercises-filter-bar">
                    <div className="category-pills">
                        {categories.map(cat => (
                            <button
                                key={cat.key}
                                className={`category-pill ${selectedCategory === cat.key ? 'active' : ''}`}
                                onClick={() => setSelectedCategory(cat.key)}
                            >
                                {cat.label}
                            </button>
                        ))}
                    </div>
                    <div className="search-wrapper">
                        <TextField
                            placeholder={t('exercises.searchPlaceholder', 'Buscar exercício por nome...')}
                            value={searchTerm}
                            onChange={(e) => setSearchTerm(e.target.value)}
                        />
                    </div>
                </div>

                {loading ? (
                    <Spinner />
                ) : filteredExercises.length === 0 ? (
                    <div className="empty-state">
                        <p>{t('exercises.emptyList', 'Nenhum exercício encontrado.')}</p>
                    </div>
                ) : (
                    <div className="exercises-grid">
                        {filteredExercises.map(ex => (
                            <div key={ex.id} className="exercise-card">
                                <div className="card-top">
                                    <span className="category-badge">{ex.category || 'Geral'}</span>
                                    <span className="visibility-badge">{ex.visibility || 'Público'}</span>
                                </div>
                                <h3>{ex.name}</h3>
                                {ex.description && <p className="description">{ex.description}</p>}
                                <div className="stats-row">
                                    <div className="stat">
                                        <span className="label">{t('exercises.sets', 'Séries')}</span>
                                        <span className="val">{ex.sets || 3}</span>
                                    </div>
                                    <div className="stat">
                                        <span className="label">{t('exercises.repsOrDuration', 'Reps/Dur.')}</span>
                                        <span className="val">{ex.reps_or_duration || ex.repsOrDuration || 10}</span>
                                    </div>
                                </div>
                                <div className="card-actions">
                                    <button
                                        type="button"
                                        className="btn-edit"
                                        onClick={() => {
                                            setEditingExercise(ex);
                                            setIsModalOpen(true);
                                        }}
                                    >
                                        ✏️ {t('application.edit', 'Editar')}
                                    </button>
                                    <button
                                        type="button"
                                        className="btn-delete"
                                        onClick={() => handleDelete(ex.id)}
                                    >
                                        🗑️ {t('application.delete', 'Excluir')}
                                    </button>
                                </div>
                            </div>
                        ))}
                    </div>
                )}
            </main>

            <ExerciseModal
                isOpen={isModalOpen}
                onClose={() => {
                    setIsModalOpen(false);
                    setEditingExercise(null);
                }}
                onSubmit={handleCreateOrUpdate}
                initialData={editingExercise}
                loading={loading}
            />
        </div>
    );
};

export default Exercises;
