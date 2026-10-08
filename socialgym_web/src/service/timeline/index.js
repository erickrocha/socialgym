import axios from '../../axios.config';
import { uploadFileToS3 } from '../s3/index.js';

// The timeline API names a post by `uuid`; the screens key and address posts by `id`.
const withId = (post) => (post ? { ...post, id: post.id ?? post.uuid } : post);

export const fetchFeedPosts = async ({ page = 0 } = {}) => {
    const { data } = await axios.get('/timeline/api/feed', {
        params: { page },
    });
    return Array.isArray(data) ? data.map(withId) : [];
};

export const addReactionToPost = async (postId, reactionType = 'Like') => {
    await axios.post(`/timeline/api/posts/${postId}/reactions`, { reactionType });
};

export const addCommentToPost = async (postId, content, authorAvatarUrl) => {
    const payload = {
        postId,
        content,
        ...(authorAvatarUrl ? { authorAvatarUrl } : {}),
    };
    // The answer is the whole post with its comments.
    const { data } = await axios.post(`/timeline/api/posts/${postId}/comments`, payload);
    return withId(data);
};

const uploadMediaFiles = async (files = []) => {
    const uploadedMedia = [];

    for (const file of files) {
        const mediaType = file.type.startsWith('video/') ? 'Video' : 'Image';

        const { data } = await axios.get('/workout/api/media/upload', {
            params: {
                mediaType,
                format: file.type,
                album: 'post',
            },
        });

        await uploadFileToS3(file, data.url);

        const uri = new URL(data.url);
        const publicUrl = `${uri.protocol}//${uri.host}${uri.pathname}`;

        uploadedMedia.push({
            mediaType,
            objectKey: data.objectKey,
            url: publicUrl,
        });
    }

    return uploadedMedia;
};

export const createFeedPost = async ({ content, files = [], thirdPartyConsentConfirmed = false }) => {
    const media = await uploadMediaFiles(files);

    const payload = {
        content: content?.trim() || '',
        ...(media.length ? { media } : {}),
        ...(media.length ? { thirdPartyConsentConfirmed } : {}),
    };

    const { data } = await axios.post('/timeline/api/posts', payload);
    return withId(data);
};

export const fetchWorkoutSessions = async ({ startDate, endDate }) => {
    const { data } = await axios.get('/timeline/api/workout-sessions', {
        params: {
            startDate,
            endDate,
        },
    });

    return Array.isArray(data) ? data : [];
};

export const fetchEvolutionCheckins = async ({ startDate, endDate }) => {
    const { data } = await axios.get('/timeline/api/evolution-checkin', {
        params: {
            startDate,
            endDate,
        },
    });

    return Array.isArray(data) ? data : [];
};

export const createEvolutionCheckin = async (payload) => {
    const { data } = await axios.post('/timeline/api/evolution-checkin', payload);
    return data;
};

export const fetchFriendProfile = async (personId) => {
    const { data } = await axios.get(`/workout/api/friends/${personId}`);
    return data;
};
