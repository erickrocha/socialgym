import React, { useState } from 'react';
import { useTranslation } from 'react-i18next';
import Avatar from '../Avatar/Avatar';
import './Post.scss';

const Post = ({ post, userAvatar, userName, onReact, onComment }) => {
    const { t } = useTranslation('common');
    const [showComments, setShowComments] = useState(false);
    const [commentText, setCommentText] = useState('');

    const authorName = post.authorName || 'Unknown User';
    const authorAvatar = post.authorAvatarUrl || userAvatar;
    const createdAt = post.createdAt ? new Date(post.createdAt) : new Date();
    const likes = Array.isArray(post.reactions) ? post.reactions.length : 0;
    const comments = Array.isArray(post.comments) ? post.comments : [];

    const handleLike = async () => {
        if (onReact) {
            await onReact(post.id, 'Like');
        }
    };

    const handleAddComment = async () => {
        if (commentText.trim()) {
            if (onComment) {
                await onComment(post.id, commentText.trim());
                setCommentText('');
            }
        }
    };

    const formatTime = (date) => {
        const now = new Date();
        const diff = now - date;
        const minutes = Math.floor(diff / 60000);
        const hours = Math.floor(diff / 3600000);
        const days = Math.floor(diff / 86400000);

        if (minutes < 1) return t('feed.now');
        if (minutes < 60) return `${minutes}m ${t('feed.ago')}`;
        if (hours < 24) return `${hours}h ${t('feed.ago')}`;
        if (days < 7) return `${days}d ${t('feed.ago')}`;
        return date.toLocaleDateString();
    };

    return (
        <div className="post">
            <div className="post__header">
                <Avatar image={authorAvatar} size="sm" />
                <div className="post__header-info">
                    <p className="post__author">{authorName}</p>
                    <p className="post__timestamp">{formatTime(createdAt)}</p>
                </div>
            </div>

            {post.content && (
                <div className="post__content">
                    <p>{post.content}</p>
                </div>
            )}

            {post.media && post.media.length > 0 && (
                <div className={`post__media post__media--${post.media.length > 1 ? 'multiple' : 'single'}`}>
                    {post.media.map((attachment, index) => (
                        <div key={index} className="post__media-item">
                            {attachment.mediaType?.toLowerCase() === 'image' ? (
                                <img src={attachment.url} alt={`Post media ${index + 1}`} />
                            ) : (
                                <video src={attachment.url} controls />
                            )}
                        </div>
                    ))}
                </div>
            )}

            <div className="post__stats">
                {likes > 0 && (
                    <div className="post__likes">
                        <span>👍 {likes} {likes === 1 ? t('feed.like') : t('feed.likes')}</span>
                    </div>
                )}
                {comments.length > 0 && (
                    <div className="post__comment-count">
                        <span>{comments.length} {comments.length === 1 ? t('feed.comment') : t('feed.comments')}</span>
                    </div>
                )}
            </div>

            <div className="post__actions">
                <button
                    className="post__action-btn"
                    onClick={handleLike}
                >
                    <span>👍</span>
                    <span>{t('feed.like')}</span>
                </button>
                <button
                    className="post__action-btn"
                    onClick={() => setShowComments(!showComments)}
                >
                    <span>💬</span>
                    <span>{t('feed.comment')}</span>
                </button>
                <button className="post__action-btn">
                    <span>↗️</span>
                    <span>{t('feed.share')}</span>
                </button>
            </div>

            {showComments && (
                <div className="post__comments-section">
                    <div className="post__comments-list">
                        {comments.map((comment) => (
                            <div key={comment.id} className="post__comment">
                                <Avatar image={comment.authorAvatarUrl || userAvatar} size="xs" />
                                <div className="post__comment-content">
                                    <p className="post__comment-author">{comment.authorName || 'User'}</p>
                                    <p className="post__comment-text">{comment.content}</p>
                                    <p className="post__comment-time">{formatTime(new Date(comment.createdAt || Date.now()))}</p>
                                </div>
                            </div>
                        ))}
                    </div>

                    <div className="post__comment-input-wrapper">
                        <Avatar image={userAvatar} size="xs" />
                        <input
                            type="text"
                            placeholder={t('feed.writeComment')}
                            value={commentText}
                            onChange={(e) => setCommentText(e.target.value)}
                            onKeyDown={(e) => {
                                if (e.key === 'Enter') {
                                    handleAddComment();
                                }
                            }}
                            className="post__comment-input"
                        />
                        {commentText.trim() && (
                            <button
                                className="post__comment-submit"
                                onClick={handleAddComment}
                            >
                                ↗️
                            </button>
                        )}
                    </div>
                </div>
            )}
        </div>
    );
};

export default Post;
