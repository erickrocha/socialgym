import React from 'react';
import {useDispatch, useSelector} from 'react-redux';
import {Navigate, useNavigate} from 'react-router';
import WorkoutExecution from './WorkoutExecution.jsx';
import WorkoutComplete from '../WorkoutComplete/index.js';
import {endExecution} from '../../../redux/reducers/workoutExecution/index.js';
import './WorkoutExecution.scss';

/**
 * Route-level host for the workout runner. Keeps the in-progress session in the
 * `workoutExecution` slice so navigating to `/workouts/execution/:id/completed`
 * (and back) does not lose it. Shows <WorkoutComplete/> once the session ends.
 */
const WorkoutExecutionScreen = () => {
    const dispatch = useDispatch();
    const navigate = useNavigate();
    const {active, completedSession} = useSelector((state) => state.workoutExecution);

    const handleCloseComplete = () => {
        dispatch(endExecution());
        navigate('/workouts');
    };

    const handleSaveSession = (session) => {
        // TODO: Dispatch an action to persist the workout session to the backend
        console.log('Saving workout session:', session);
        dispatch(endExecution());
        navigate('/workout-sessions');
    };

    if (completedSession) {
        return (
            <WorkoutComplete
                workoutSession={completedSession}
                onClose={handleCloseComplete}
                onSave={handleSaveSession}
            />
        );
    }

    if (!active) {
        return <Navigate to="/workouts" replace/>;
    }

    return (
        <div className="workout-execution-screen">
            <WorkoutExecution/>
        </div>
    );
};

export default WorkoutExecutionScreen;
