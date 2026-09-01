import monitorReducersEnhancer from '../../commons/enhancers/monitorReducer'
import loggerMiddleware from '../../commons/middleware/logger'

import {configureStore} from "@reduxjs/toolkit";
import authReducer from "../reducers/auth/auth.slice.js";
import personReducer from "../reducers/person/person.slice.js";
import friendReducer from "../reducers/friend/friend.slice.js";
import workoutReducer from "../reducers/workout/workout.slice.js";
import workoutExecutionReducer from "../reducers/workoutExecution/workoutExecution.slice.js";
import uploadReducer from "../reducers/upload/upload.slice.js";
import businessReducer from "../reducers/business/business.slice.js";
import timelineReducer from "../reducers/timeline/timeline.slice.js";
import exerciseReducer from "../reducers/exercise/exercise.slice.js";
import settingsReducer from "../reducers/settings/settings.slice.js";
import notificationReducer from "../reducers/notification/notification.slice.js";
import evolutionReducer from "../reducers/evolution/evolution.slice.js";
import chatReducer from "../reducers/chat/chat.slice.js";
import chatSocketMiddleware from "../middleware/chatSocket.js";

export const store = configureStore({
    reducer: {
        auth: authReducer,
        person: personReducer,
        friend: friendReducer,
        workout: workoutReducer,
        workoutExecution: workoutExecutionReducer,
        upload: uploadReducer,
        business: businessReducer,
        timeline: timelineReducer,
        exercise: exerciseReducer,
        settings: settingsReducer,
        notification: notificationReducer,
        evolution: evolutionReducer,
        chat: chatReducer
    },
    middleware: (getDefaultMiddleware) =>
        getDefaultMiddleware().concat(loggerMiddleware, chatSocketMiddleware),
    devTools: true,
    enhancers: (getDefaultEnhancers) => getDefaultEnhancers().concat(monitorReducersEnhancer)
});
