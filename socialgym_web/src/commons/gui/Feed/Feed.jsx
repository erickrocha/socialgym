import React, { useEffect, useMemo, useState } from 'react';
import { useTranslation } from 'react-i18next';
import { useDispatch, useSelector } from 'react-redux';
import PostCreator from '../PostCreator/PostCreator';
import Post from '../Post/Post';
import {
    commentOnPost,
    createTimelinePost,
    getFeedPosts,
    reactToPost,
} from '../../../redux/reducers/timeline/index.js';
import './Feed.scss';
import axios from '../../../axios.config.js';

const Feed = ({ person, userAvatar, defaultAvatar }) => {
    const { t } = useTranslation('common');
    const dispatch = useDispatch();
    const { posts, postsLoading, postsLoadingMore, postsSubmitting, postsError, postsPage, postsHasMore } = useSelector(
        (state) => state.timeline,
    );
    const [error, setError] = useState(postsError);

    const userName = person ? `${person?.firstname} ${person?.surname}` : 'User';

    const normalizedPosts = useMemo(
        () =>
            posts.map((post) => ({
                ...post,
                reactions: Array.isArray(post.reactions) ? post.reactions : [],
                comments: Array.isArray(post.comments) ? post.comments : [],
                media: Array.isArray(post.media) ? post.media : [],
            })),
        [posts],
    );

    useEffect(() => {
        dispatch(getFeedPosts({ page: 0 }));
    }, [dispatch]);

    useEffect(() => {
        setError(postsError);
    }, [postsError]);

    const refreshLoadedPages = async () => {
        await dispatch(getFeedPosts({ page: 0 })).unwrap();

        for (let page = 1; page <= postsPage; page += 1) {
            await dispatch(getFeedPosts({ page, append: true })).unwrap();
        }
    };

    const handlePostCreate = async (newPost) => {
        setError(null);
        try {
            await dispatch(createTimelinePost(newPost)).unwrap();
        } catch (err) {
            setError(err?.message || t('feed.errors.create'));
        }
    };

    const handleReact = async (postId, reactionType) => {
        try {
            await dispatch(reactToPost({ postId, reactionType })).unwrap();
            await refreshLoadedPages();
        } catch (err) {
            setError(err?.message || t('feed.errors.react'));
        }
    };

    const handleLoadMore = async () => {
        try {
            await dispatch(getFeedPosts({ page: postsPage + 1, append: true })).unwrap();
        } catch (err) {
            setError(err?.message || t('feed.errors.loadMore', { defaultValue: 'Unable to load more posts' }));
        }
    };

    const handleComment = async (postId, content) => {
        try {
            await dispatch(
                commentOnPost({
                    postId,
                    content,
                    authorAvatarUrl: person?.avatar || null,
                }),
            ).unwrap();
        } catch (err) {
            setError(err?.message || t('feed.errors.comment'));
        }
    };

    const handleReport = async ({ targetType, targetId, postId }) => {
        const reason = window.prompt('Motivo da denúncia (ex.: harassment, intimate_image, illegal_content, other):', 'other');
        if (!reason) return;
        const details = window.prompt('Descreva o problema (opcional):', '') || null;
        try {
            await axios.post('/timeline/api/reports', { targetType, targetId, postId, reason, details });
            setError('Denúncia recebida. Nossa equipe fará a análise.');
        } catch (err) { setError(err?.response?.data?.message || 'Não foi possível enviar a denúncia.'); }
    };

    return (
        <div className="feed">
            <div className="feed__container">
                <PostCreator
                    userAvatar={userAvatar || defaultAvatar}
                    userName={userName}
                    onPostCreate={handlePostCreate}
                    loading={postsSubmitting}
                />

                {error && (
                    <div className="feed__empty">
                        <p>{error}</p>
                    </div>
                )}

                <div className="feed__posts">
                    {postsLoading ? (
                        <div className="feed__empty">
                            <p>{t('feed.loading')}</p>
                        </div>
                    ) : normalizedPosts.length === 0 ? (
                        <div className="feed__empty">
                            <p>{t('feed.noPostsYet')}</p>
                        </div>
                    ) : (
                        <>
                            {normalizedPosts.map((post) => (
                                <Post
                                    key={post.id}
                                    post={post}
                                    userAvatar={userAvatar || defaultAvatar}
                                    userName={userName}
                                    userId={person?.id}
                                    onReact={handleReact}
                                    onComment={handleComment}
                                    onReport={handleReport}
                                />
                            ))}

                            {postsHasMore && (
                                <div className="feed__pagination">
                                    <button
                                        type="button"
                                        className="feed__load-more"
                                        onClick={handleLoadMore}
                                        disabled={postsLoadingMore}
                                    >
                                        {postsLoadingMore
                                            ? t('feed.loadingMore', { defaultValue: 'Loading more...' })
                                            : t('feed.loadMore', { defaultValue: 'Load more' })}
                                    </button>
                                </div>
                            )}
                        </>
                    )}
                </div>
            </div>
        </div>
    );
};

export default Feed;
