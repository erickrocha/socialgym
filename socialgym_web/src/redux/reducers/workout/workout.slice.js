import {updateObject} from "../../../commons/library/utility";
import {createSlice} from "@reduxjs/toolkit";
import {addWorkout, addExercise, addExercises, getWorkouts} from "./workout.action";


const initialState = {
    workouts: [],
    loading: false,
    error: null,
    selectedWorkout: null,
}
const workoutSlice = createSlice({
    name: 'workout',
    initialState,
    reducers: {
        setSelectedWorkout: (state, {payload}) => {
            return updateObject(state, {selectedWorkout: payload})
        },
        clearSelectedWorkout: (state) => {
            return updateObject(state, {selectedWorkout: null})
        },
        clearError: (state) => {
            return updateObject(state, {error: null})
        }
    },
    extraReducers: (builder) => {
        builder
            .addCase(getWorkouts.fulfilled, (state, {payload}) => {
                return updateObject(state, {
                    workouts: payload,
                    loading: false
                })
            })
            .addCase(getWorkouts.pending, (state) => (
                {
                    ...state,
                    loading: true,
                }
            ))
            .addCase(getWorkouts.rejected, (state, {payload}) => (
                {
                    ...state,
                    loading: false,
                    error: payload
                }
            ))

            .addCase(addWorkout.pending, (state) => (
                {
                    ...state,
                    loading: true,
                }
            )).addCase(addWorkout.rejected, (state, {payload}) => (
            {
                ...state,
                loading: false,
                error: payload
            }
        ))
            .addCase(addWorkout.fulfilled, (state, {payload}) => {
                return updateObject(state, {
                    loading: false,
                    error: null,
                    workouts: [...state.workouts, payload],
                })
            })

            .addCase(addExercise.pending, (state) => ({
                ...state,
                loading: true,
            }))
            .addCase(addExercise.rejected, (state, {payload}) => ({
                ...state,
                loading: false,
                error: payload
            }))
            .addCase(addExercise.fulfilled, (state, {payload}) => {
                const updatedWorkouts = state.workouts.map(workout => {
                    if (workout.id === payload.workoutId) {
                        return {
                            ...workout,
                            exercises: [...(workout.exercises || []), payload]
                        };
                    }
                    return workout;
                });

                const updatedSelectedWorkout = state.selectedWorkout?.id === payload.workoutId
                    ? {
                        ...state.selectedWorkout,
                        exercises: [...(state.selectedWorkout.exercises || []), payload]
                    }
                    : state.selectedWorkout;

                return updateObject(state, {
                    loading: false,
                    error: null,
                    workouts: updatedWorkouts,
                    selectedWorkout: updatedSelectedWorkout,
                });
            })

            .addCase(addExercises.pending, (state) => ({
                ...state,
                loading: true,
            }))
            .addCase(addExercises.rejected, (state, {payload}) => ({
                ...state,
                loading: false,
                error: payload
            }))
            .addCase(addExercises.fulfilled, (state, {payload}) => {
                const updatedWorkouts = state.workouts.map(workout => {
                    if (workout.id === payload.workoutId) {
                        return {
                            ...workout,
                            exercises: [...(workout.exercises || []), ...payload.exercises]
                        };
                    }
                    return workout;
                });

                const updatedSelectedWorkout = state.selectedWorkout?.id === payload.workoutId
                    ? {
                        ...state.selectedWorkout,
                        exercises: [...(state.selectedWorkout.exercises || []), ...payload.exercises]
                    }
                    : state.selectedWorkout;

                return updateObject(state, {
                    loading: false,
                    error: null,
                    workouts: updatedWorkouts,
                    selectedWorkout: updatedSelectedWorkout,
                });
            })
    }
})
export default workoutSlice.reducer;
export const {setSelectedWorkout, clearSelectedWorkout, clearError} = workoutSlice.actions;