import { useState, useEffect } from 'react';
import { useParams, useNavigate } from 'react-router-dom';
import { useTranslation } from 'react-i18next';
import { useSelector } from 'react-redux';
import { AppHeader, Button, Modal, Spinner } from '../../commons/gui';
import axios from '../../axios.config';
import './WorkoutExecution.scss';

export const WorkoutExecution = () => {
    const { id } = useParams();
    const { t } = useTranslation();
    const navigate = useNavigate();
    const person = useSelector((state) => state.person?.person);

    const [workout, setWorkout] = useState(null);
    const [loading, setLoading] = useState(true);

    // Active session state
    const [elapsedSeconds, setElapsedSeconds] = useState(0);
    const [completedSets, setCompletedSets] = useState({}); // { exerciseIdx-setIdx: true }
    const [exerciseLogs, setExerciseLogs] = useState({}); // { exerciseIdx-setIdx: { weight, reps } }
    const [restSeconds, setRestSeconds] = useState(0);
    const [isRestActive, setIsRestActive] = useState(false);
    const [showCompleteModal, setShowCompleteModal] = useState(false);
    const [notes, setNotes] = useState('');
    const [submitting, setSubmitting] = useState(false);

    // Fetch workout detail
    useEffect(() => {
        const loadWorkout = async () => {
            try {
                const { data } = await axios.get(`/workout/api/workouts/${id}`);
                setWorkout(data);
            } catch (err) {
                console.error(err);
            } finally {
                setLoading(false);
            }
        };
        loadWorkout();
    }, [id]);

    // Timer elapsed stopwatch
    useEffect(() => {
        const interval = setInterval(() => {
            setElapsedSeconds(prev => prev + 1);
        }, 1000);
        return () => clearInterval(interval);
    }, []);

    // Rest countdown timer
    useEffect(() => {
        let timer;
        if (isRestActive && restSeconds > 0) {
            timer = setInterval(() => {
                setRestSeconds(prev => prev - 1);
            }, 1000);
        } else if (restSeconds === 0) {
            setIsRestActive(false);
        }
        return () => clearInterval(timer);
    }, [isRestActive, restSeconds]);

    const formatTime = (secs) => {
        const m = Math.floor(secs / 60);
        const s = secs % 60;
        return `${m.toString().padStart(2, '0')}:${s.toString().padStart(2, '0')}`;
    };

    const handleToggleSet = (exIdx, setIdx) => {
        const key = `${exIdx}-${setIdx}`;
        const isNowCompleted = !completedSets[key];

        setCompletedSets(prev => ({
            ...prev,
            [key]: isNowCompleted
        }));

        if (isNowCompleted) {
            // Trigger 60s rest timer
            setRestSeconds(60);
            setIsRestActive(true);
        }
    };

    const handleLogChange = (exIdx, setIdx, field, value) => {
        const key = `${exIdx}-${setIdx}`;
        setExerciseLogs(prev => ({
            ...prev,
            [key]: {
                ...(prev[key] || {}),
                [field]: value
            }
        }));
    };

    const handleFinishWorkout = async () => {
        setSubmitting(true);
        try {
            const totalSets = Object.keys(completedSets).filter(k => completedSets[k]).length;
            const sessionPayload = {
                workout_id: workout.id,
                workout_name: workout.name,
                duration_seconds: elapsedSeconds,
                completed_sets_count: totalSets,
                notes: notes,
                date: new Date().toISOString()
            };

            await axios.post('/timeline/api/workout-sessions', sessionPayload);
            navigate('/workout-sessions');
        } catch (err) {
            console.error('Error saving workout session', err);
        } finally {
            setSubmitting(false);
        }
    };

    if (loading) {
        return (
            <div className="workout-execution-page">
                <AppHeader person={person} />
                <Spinner />
            </div>
        );
    }

    if (!workout) {
        return (
            <div className="workout-execution-page">
                <AppHeader person={person} />
                <div className="error-container">
                    <h2>{t('workoutExecution.notFound', 'Treino não encontrado')}</h2>
                    <Button onClick={() => navigate('/workouts')}>{t('workoutExecution.backToWorkouts', 'Voltar aos Treinos')}</Button>
                </div>
            </div>
        );
    }

    return (
        <div className="workout-execution-page">
            <AppHeader person={person} />

            <div className="execution-header-bar">
                <div className="header-info">
                    <h2>{workout.name}</h2>
                    <span className="badge">{workout.difficulty || 'Normal'}</span>
                </div>
                <div className="timer-box">
                    <span className="timer-label">{t('workoutExecution.timer', 'Tempo Decorrido')}</span>
                    <span className="timer-val">{formatTime(elapsedSeconds)}</span>
                </div>
                <Button type="button" variant="primary" onClick={() => setShowCompleteModal(true)}>
                    🏁 {t('workoutExecution.finishBtn', 'Finalizar Treino')}
                </Button>
            </div>

            {isRestActive && (
                <div className="rest-timer-banner">
                    <span>⏱️ {t('workoutExecution.restTimer', 'Descanso')}: <strong>{restSeconds}s</strong></span>
                    <button type="button" onClick={() => setIsRestActive(false)}>✕ {t('application.button.cancel', 'Pular')}</button>
                </div>
            )}

            <main className="execution-container">
                {(!workout.exercises || workout.exercises.length === 0) ? (
                    <div className="empty-exercises">
                        <p>{t('workoutExecution.noExercises', 'Este treino não possui exercícios cadastrados.')}</p>
                    </div>
                ) : (
                    workout.exercises.map((ex, exIdx) => (
                        <div key={exIdx} className="execution-exercise-card">
                            <div className="ex-header">
                                <h3>{ex.name}</h3>
                                <span className="ex-cat">{ex.category || 'Geral'}</span>
                            </div>

                            <div className="sets-table">
                                <div className="table-header">
                                    <span>{t('workoutExecution.set', 'Série')}</span>
                                    <span>{t('workoutExecution.weight', 'Carga (kg)')}</span>
                                    <span>{t('workoutExecution.reps', 'Reps')}</span>
                                    <span>{t('workoutExecution.status', 'Concluído')}</span>
                                </div>

                                {Array.from({ length: ex.sets || 3 }).map((_, setIdx) => {
                                    const key = `${exIdx}-${setIdx}`;
                                    const isDone = !!completedSets[key];
                                    const log = exerciseLogs[key] || {};

                                    return (
                                        <div key={setIdx} className={`table-row ${isDone ? 'completed' : ''}`}>
                                            <span className="set-num">#{setIdx + 1}</span>
                                            <input
                                                type="number"
                                                placeholder="0"
                                                value={log.weight || ''}
                                                onChange={(e) => handleLogChange(exIdx, setIdx, 'weight', e.target.value)}
                                            />
                                            <input
                                                type="number"
                                                placeholder={ex.reps_or_duration || 10}
                                                value={log.reps || ''}
                                                onChange={(e) => handleLogChange(exIdx, setIdx, 'reps', e.target.value)}
                                            />
                                            <button
                                                type="button"
                                                className={`check-btn ${isDone ? 'checked' : ''}`}
                                                onClick={() => handleToggleSet(exIdx, setIdx)}
                                            >
                                                {isDone ? '✓' : ''}
                                            </button>
                                        </div>
                                    );
                                })}
                            </div>
                        </div>
                    ))
                )}
            </main>

            {showCompleteModal && (
                <Modal
                    isOpen={showCompleteModal}
                    onClose={() => setShowCompleteModal(false)}
                    title={t('workoutExecution.modalTitle', 'Parabéns! Treino Concluído 🎉')}
                >
                    <div className="completion-modal-content">
                        <p><strong>{t('workoutExecution.duration', 'Duração total')}:</strong> {formatTime(elapsedSeconds)}</p>
                        <p><strong>{t('workoutExecution.totalSets', 'Séries completadas')}:</strong> {Object.keys(completedSets).filter(k => completedSets[k]).length}</p>

                        <div className="form-group">
                            <label>{t('workoutExecution.notes', 'Anotações sobre a sessão')}</label>
                            <textarea
                                value={notes}
                                onChange={(e) => setNotes(e.target.value)}
                                placeholder={t('workoutExecution.notesPlaceholder', 'Ex: Aumentei carga no supino, ótima energia!')}
                                rows={3}
                            />
                        </div>

                        <div className="modal-actions">
                            <Button type="button" variant="secondary" onClick={() => setShowCompleteModal(false)}>
                                {t('application.button.cancel', 'Continuar Treinando')}
                            </Button>
                            <Button type="button" onClick={handleFinishWorkout} disabled={submitting}>
                                {submitting ? t('application.loading', 'Salvando...') : t('workoutExecution.saveSession', 'Salvar Sessão')}
                            </Button>
                        </div>
                    </div>
                </Modal>
            )}
        </div>
    );
};

export default WorkoutExecution;
