import { createAsyncThunk } from '@reduxjs/toolkit';
import {
    addCommentToPost,
    addReactionToPost,
    createEvolutionCheckin,
    createFeedPost,
    fetchEvolutionCheckins,
    fetchFeedPosts,
    fetchWorkoutSessions,
} from '../../../service/timeline/index.js';

export const getFeedPosts = createAsyncThunk(
    'timeline/getFeedPosts',
    async ({ page = 0, append = false } = {}, { rejectWithValue }) => {
        try {
            const posts = await fetchFeedPosts({ page });
            return { posts, page, append };
        } catch (error) {
            return rejectWithValue(error?.response?.data || error?.message || 'Failed to fetch feed posts');
        }
    },
);

export const createTimelinePost = createAsyncThunk(
    'timeline/createTimelinePost',
    async (payload, { rejectWithValue }) => {
        try {
            return await createFeedPost(payload);
        } catch (error) {
            return rejectWithValue(error?.response?.data || error?.message || 'Failed to create post');
        }
    },
);

export const reactToPost = createAsyncThunk(
    'timeline/reactToPost',
    async ({ postId, reactionType = 'Like' }, { rejectWithValue }) => {
        try {
            await addReactionToPost(postId, reactionType);
            return { postId, reactionType };
        } catch (error) {
            return rejectWithValue(error?.response?.data || error?.message || 'Failed to react to post');
        }
    },
);

export const commentOnPost = createAsyncThunk(
    'timeline/commentOnPost',
    async ({ postId, content, authorAvatarUrl }, { rejectWithValue }) => {
        try {
            const createdComment = await addCommentToPost(postId, content, authorAvatarUrl);
            return { postId, createdComment };
        } catch (error) {
            return rejectWithValue(error?.response?.data || error?.message || 'Failed to add comment');
        }
    },
);

export const getWorkoutSessions = createAsyncThunk(
    'timeline/getWorkoutSessions',
    async (dateRange, { rejectWithValue }) => {
        try {
            return await fetchWorkoutSessions(dateRange);
        } catch (error) {
            return rejectWithValue(error?.response?.data || error?.message || 'Failed to fetch workout sessions');
        }
    },
);

export const getEvolutionCheckins = createAsyncThunk(
    'timeline/getEvolutionCheckins',
    async (dateRange, { rejectWithValue }) => {
        try {
            return await fetchEvolutionCheckins(dateRange);
        } catch (error) {
            return rejectWithValue(error?.response?.data || error?.message || 'Failed to fetch evolution check-ins');
        }
    },
);

export const addEvolutionCheckin = createAsyncThunk(
    'timeline/addEvolutionCheckin',
    async (payload, { rejectWithValue }) => {
        try {
            return await createEvolutionCheckin(payload);
        } catch (error) {
            return rejectWithValue(error?.response?.data || error?.message || 'Failed to create evolution check-in');
        }
    },
);
