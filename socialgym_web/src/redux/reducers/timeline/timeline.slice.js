import { createSlice } from '@reduxjs/toolkit';
import {
    addEvolutionCheckin,
    commentOnPost,
    createTimelinePost,
    getEvolutionCheckins,
    getFeedPosts,
    getWorkoutSessions,
    reactToPost,
} from './timeline.actions.js';

const initialState = {
    posts: [],
    postsLoading: false,
    postsLoadingMore: false,
    postsSubmitting: false,
    postsError: null,
    postsPage: 0,
    postsHasMore: true,

    sessions: [],
    sessionsLoading: false,
    sessionsError: null,

    checkins: [],
    checkinsLoading: false,
    checkinsSubmitting: false,
    checkinsError: null,
};

const normalizeError = (payload) => {
    if (!payload) return 'Unexpected error';
    if (typeof payload === 'string') return payload;
    if (payload.message) return payload.message;
    return 'Unexpected error';
};

const mergePosts = (existingPosts, incomingPosts) => {
    const seenIds = new Set(existingPosts.map((post) => post.id));
    return [...existingPosts, ...incomingPosts.filter((post) => !seenIds.has(post.id))];
};

const timelineSlice = createSlice({
    name: 'timeline',
    initialState,
    reducers: {
        clearTimelineErrors: (state) => {
            state.postsError = null;
            state.sessionsError = null;
            state.checkinsError = null;
        },
    },
    extraReducers: (builder) => {
        builder
            .addCase(getFeedPosts.pending, (state, action) => {
                const append = Boolean(action.meta.arg?.append);
                state.postsLoading = !append;
                state.postsLoadingMore = append;
                state.postsError = null;
            })
            .addCase(getFeedPosts.fulfilled, (state, { payload }) => {
                state.postsLoading = false;
                state.postsLoadingMore = false;
                state.postsPage = payload.page;
                state.postsHasMore = payload.posts.length === 20;
                state.posts = payload.append ? mergePosts(state.posts, payload.posts) : payload.posts;
            })
            .addCase(getFeedPosts.rejected, (state, { payload }) => {
                state.postsLoading = false;
                state.postsLoadingMore = false;
                state.postsError = normalizeError(payload);
            })

            .addCase(createTimelinePost.pending, (state) => {
                state.postsSubmitting = true;
                state.postsError = null;
            })
            .addCase(createTimelinePost.fulfilled, (state, { payload }) => {
                state.postsSubmitting = false;
                state.posts = [payload, ...state.posts];
            })
            .addCase(createTimelinePost.rejected, (state, { payload }) => {
                state.postsSubmitting = false;
                state.postsError = normalizeError(payload);
            })

            .addCase(reactToPost.pending, (state) => {
                state.postsError = null;
            })
            .addCase(reactToPost.rejected, (state, { payload }) => {
                state.postsError = normalizeError(payload);
            })

            .addCase(commentOnPost.pending, (state) => {
                state.postsError = null;
            })
            .addCase(commentOnPost.fulfilled, (state, { payload }) => {
                state.posts = state.posts.map((post) => {
                    if (post.id !== payload.postId) return post;
                    const comments = Array.isArray(post.comments) ? post.comments : [];
                    return {
                        ...post,
                        comments: [...comments, payload.createdComment],
                    };
                });
            })
            .addCase(commentOnPost.rejected, (state, { payload }) => {
                state.postsError = normalizeError(payload);
            })

            .addCase(getWorkoutSessions.pending, (state) => {
                state.sessionsLoading = true;
                state.sessionsError = null;
            })
            .addCase(getWorkoutSessions.fulfilled, (state, { payload }) => {
                state.sessionsLoading = false;
                state.sessions = payload;
            })
            .addCase(getWorkoutSessions.rejected, (state, { payload }) => {
                state.sessionsLoading = false;
                state.sessionsError = normalizeError(payload);
            })

            .addCase(getEvolutionCheckins.pending, (state) => {
                state.checkinsLoading = true;
                state.checkinsError = null;
            })
            .addCase(getEvolutionCheckins.fulfilled, (state, { payload }) => {
                state.checkinsLoading = false;
                state.checkins = payload;
            })
            .addCase(getEvolutionCheckins.rejected, (state, { payload }) => {
                state.checkinsLoading = false;
                state.checkinsError = normalizeError(payload);
            })

            .addCase(addEvolutionCheckin.pending, (state) => {
                state.checkinsSubmitting = true;
                state.checkinsError = null;
            })
            .addCase(addEvolutionCheckin.fulfilled, (state, { payload }) => {
                state.checkinsSubmitting = false;
                state.checkins = [payload, ...state.checkins];
            })
            .addCase(addEvolutionCheckin.rejected, (state, { payload }) => {
                state.checkinsSubmitting = false;
                state.checkinsError = normalizeError(payload);
            });
    },
});

export const { clearTimelineErrors } = timelineSlice.actions;
export default timelineSlice.reducer;
