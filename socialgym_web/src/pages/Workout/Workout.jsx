import React, {useEffect, useState} from 'react';
import {useDispatch, useSelector} from 'react-redux';
import {useTranslation} from 'react-i18next';
import {useNavigate} from 'react-router';
import WorkoutCard from '../../commons/components/WorkoutCard/WorkoutCard';
import ExerciseList from '../../commons/components/ExerciseList/ExerciseList';
import * as handler from '../../redux/reducers/workout/index.js';
import {startExecution} from '../../redux/reducers/workoutExecution/index.js';
import './Workout.scss';
import {AppHeader, Modal, Sidebar, Spinner, Toast} from "../../commons/gui/index.js";
import WorkoutForm from "./WorkoutForm/index.js";
import ExerciseForm from "./ExerciseForm/index.js";
import StartWorkoutButton from "./StartWorkoutButton/index.js";

// Custom hook to detect mobile screen
const useIsMobile = (breakpoint = 768) => {
    const [isMobile, setIsMobile] = useState(window.innerWidth <= breakpoint);

    useEffect(() => {
        const handleResize = () => {
            setIsMobile(window.innerWidth <= breakpoint);
        };

        window.addEventListener('resize', handleResize);
        return () => window.removeEventListener('resize', handleResize);
    }, [breakpoint]);

    return isMobile;
};


const Workout = () => {
    const {t} = useTranslation('common');
    const dispatch = useDispatch();
    const navigate = useNavigate();
    const [isSidebarCollapsed, setIsSidebarCollapsed] = useState(false);
    const isMobile = useIsMobile();

    const [isShowModal, setShowModal] = useState(false);
    const [isAddingExercises, setIsAddingExercises] = useState(false);
    const [exerciseWorkoutId, setExerciseWorkoutId] = useState(null);

    const {person} = useSelector((state) => state.person);
    const {workouts,selectedWorkout, error, loading } = useSelector((state) => state.workout);

    useEffect(() => {
        if (person) {
            dispatch(handler.getWorkouts(person?.id));
        }
    }, [dispatch]);

    const handleSidebarToggle = () => {
        setIsSidebarCollapsed(!isSidebarCollapsed);
    };

    const handleWorkoutSelect = (workout) => {
        if (selectedWorkout?.id === workout.id) {
            dispatch(handler.clearSelectedWorkout());
            setIsAddingExercises(false);
        } else {
            dispatch(handler.setSelectedWorkout(workout));
            setIsAddingExercises(false);
        }
    };

    const handleAddExercise = (workoutId) => {
        setExerciseWorkoutId(workoutId || selectedWorkout?.id);
        setIsAddingExercises(true);
    };

    const handleExerciseFormClose = () => {
        setIsAddingExercises(false);
        setExerciseWorkoutId(null);
    };

    const handleStartWorkout = (workout) => {
        dispatch(startExecution(workout));
        navigate(`/workouts/execution/${workout.id}`);
    };

    // Determine if we should show exercise form inline (mobile) or modal (desktop)
    const showExerciseModal = !isMobile && isAddingExercises;
    const showExerciseInline = isMobile && isAddingExercises;
    const exerciseWorkout = workouts.find((w) => w.id === exerciseWorkoutId);

    return (
        <div className="workout-layout">
            <AppHeader person={person}/>
            {loading && <Spinner/>}
            <Toast
                message={error}
                type="error"
                autoCloseDuration={5000}
            />
            <div className="workout-container">
                <Sidebar
                    person={person}
                    isCollapsed={isSidebarCollapsed}
                    onToggle={handleSidebarToggle}
                />
                <main className="workout-content">
                    <div className="workout-header">
                        <div className="workout-header__text">
                            <h1>{t('workout.title')}</h1>
                            <p>{t('workout.description')}</p>
                        </div>
                        <button
                            className="workout-header__add-btn"
                            onClick={() => {
                                setShowModal(true)
                            }}
                            aria-label={t('workout.addWorkout')}
                            title={t('workout.addWorkout')}
                        >
                            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
                                <line x1="12" y1="5" x2="12" y2="19"></line>
                                <line x1="5" y1="12" x2="19" y2="12"></line>
                            </svg>
                        </button>
                    </div>
                    <Modal isOpen={isShowModal}
                           isForm={true}
                           onClose={() => setShowModal(false)}
                           title={t('workout.addWorkout')}>
                        <WorkoutForm
                            workout={selectedWorkout}
                            person={person}
                            onCancel={() => setShowModal(false)}
                            onConfirm={() => setShowModal(false)}
                        />
                    </Modal>

                    {/* Exercise Form Modal for Desktop */}
                    <Modal isOpen={showExerciseModal}
                           isForm={true}
                           onClose={handleExerciseFormClose}
                           title={t('workout.addExercise')}>
                        <ExerciseForm
                            workoutId={exerciseWorkoutId}
                            workoutVisibility={exerciseWorkout?.visibility}
                            onCancel={handleExerciseFormClose}
                            onConfirm={handleExerciseFormClose}
                        />
                    </Modal>


                    {workouts.length === 0 && !loading ? (
                        <div className="workout-empty">
                            <span className="workout-empty__icon">🏋️</span>
                            <p className="workout-empty__text">{t('workout.noWorkouts')}</p>
                        </div>
                    ) : (
                        <div className="workout-grid">
                            {workouts.map((workout) => (
                                <div key={workout.id} className="workout-grid__item">
                                    <WorkoutCard
                                        workout={workout}
                                        isSelected={selectedWorkout?.id === workout.id}
                                        onSelect={handleWorkoutSelect}
                                    />
                                    {selectedWorkout?.id === workout.id && (
                                        <>
                                            {workout.exercises && workout.exercises.length > 0 ? (
                                                <>
                                                    {/* Start Workout Button */}
                                                    <div className="workout-start-section">
                                                        <StartWorkoutButton
                                                            onClick={() => handleStartWorkout(workout)}
                                                            disabled={!workout.exercises || workout.exercises.length === 0}
                                                        />
                                                    </div>
                                                    <ExerciseList
                                                        exercises={workout.exercises}
                                                        workoutName={workout.name}
                                                        onAddExercise={() => handleAddExercise(workout.id)}
                                                    />
                                                    {/* Inline Exercise Form for Mobile */}
                                                    {showExerciseInline && exerciseWorkoutId === workout.id && (
                                                        <ExerciseForm
                                                            workoutId={workout.id}
                                                            workoutVisibility={workout.visibility}
                                                            onCancel={handleExerciseFormClose}
                                                            onConfirm={handleExerciseFormClose}
                                                        />
                                                    )}
                                                </>
                                            ) : (
                                                isMobile ? (
                                                    <ExerciseForm
                                                        workoutId={workout.id}
                                                        workoutVisibility={workout.visibility}
                                                        onCancel={() => dispatch(handler.clearSelectedWorkout())}
                                                        onConfirm={handleExerciseFormClose}
                                                    />
                                                ) : (
                                                    <div className="workout-no-exercises">
                                                        <p>{t('workout.noExercises')}</p>
                                                        <button
                                                            className="workout-no-exercises__btn"
                                                            onClick={() => handleAddExercise(workout.id)}
                                                        >
                                                            + {t('workout.addExercise')}
                                                        </button>
                                                    </div>
                                                )
                                            )}
                                        </>
                                    )}
                                </div>
                            ))}
                        </div>
                    )}
                </main>
            </div>
        </div>
    );
};

export default Workout;
