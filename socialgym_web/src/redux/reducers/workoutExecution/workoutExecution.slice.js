import {createSlice} from "@reduxjs/toolkit";

const initialState = {
    active: false,
    workout: null,
    currentExerciseIndex: 0,
    currentSetIndex: 0,
    executedSets: [],
    startedAt: null,
    // Set when the workout finishes so the screen can show <WorkoutComplete/>.
    completedSession: null,
};

// Advances currentSetIndex/currentExerciseIndex the same way the old local-state
// runner did (handleConfirmSet / handleSkipSet). The final set of the final
// exercise leaves the indices untouched — completion is handled via finishExecution.
const advance = (state) => {
    const exercises = state.workout?.exercises || [];
    const totalSets = exercises[state.currentExerciseIndex]?.sets || 0;
    const isLastSet = state.currentSetIndex >= totalSets - 1;
    const isLastExercise = state.currentExerciseIndex >= exercises.length - 1;

    if (isLastSet) {
        if (!isLastExercise) {
            state.currentExerciseIndex += 1;
            state.currentSetIndex = 0;
        }
    } else {
        state.currentSetIndex += 1;
    }
};

const workoutExecutionSlice = createSlice({
    name: 'workoutExecution',
    initialState,
    reducers: {
        startExecution: (state, {payload}) => {
            state.active = true;
            state.workout = payload;
            state.currentExerciseIndex = 0;
            state.currentSetIndex = 0;
            state.executedSets = [];
            state.startedAt = Date.now();
            state.completedSession = null;
        },
        confirmSet: (state, {payload}) => {
            state.executedSets.push(payload);
            advance(state);
        },
        skipSet: (state) => {
            advance(state);
        },
        finishExecution: (state, {payload}) => {
            state.completedSession = payload;
            state.active = false;
        },
        endExecution: () => initialState,
    },
});

export default workoutExecutionSlice.reducer;
export const {
    startExecution,
    confirmSet,
    skipSet,
    finishExecution,
    endExecution,
} = workoutExecutionSlice.actions;
