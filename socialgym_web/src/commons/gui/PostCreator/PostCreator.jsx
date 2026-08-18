import React, { useState } from 'react';
import { useTranslation } from 'react-i18next';
import Avatar from '../Avatar/Avatar';
import Button from '../Button/Button';
import './PostCreator.scss';

const PostCreator = ({ userAvatar, userName, onPostCreate, loading = false }) => {
    const { t } = useTranslation('common');
    const [isExpanded, setIsExpanded] = useState(false);
    const [postContent, setPostContent] = useState('');
    const [attachments, setAttachments] = useState([]);
    const [previewUrls, setPreviewUrls] = useState([]);

    const handleTextChange = (e) => {
        setPostContent(e.target.value);
    };

    const handleFileChange = (e) => {
        const files = Array.from(e.target.files);
        const newAttachments = [...attachments, ...files];
        setAttachments(newAttachments);

        // Create preview URLs for images and videos
        files.forEach(file => {
            const reader = new FileReader();
            reader.onloadend = () => {
                setPreviewUrls(prev => [...prev, {
                    url: reader.result,
                    type: file.type.startsWith('image') ? 'image' : 'video',
                    name: file.name
                }]);
            };
            reader.readAsDataURL(file);
        });
    };

    const handleRemoveAttachment = (index) => {
        setAttachments(prev => prev.filter((_, i) => i !== index));
        setPreviewUrls(prev => prev.filter((_, i) => i !== index));
    };

    const handlePostSubmit = () => {
        if (postContent.trim() || attachments.length > 0) {
            onPostCreate({
                content: postContent,
                files: attachments,
            });
            setPostContent('');
            setAttachments([]);
            setPreviewUrls([]);
            setIsExpanded(false);
        }
    };

    const handleCancel = () => {
        setPostContent('');
        setAttachments([]);
        setPreviewUrls([]);
        setIsExpanded(false);
    };

    return (
        <div className="post-creator">
            <div className="post-creator__header">
                <Avatar image={userAvatar} size="sm" />
                <div
                    className="post-creator__input-wrapper"
                    onClick={() => setIsExpanded(true)}
                >
                    <input
                        type="text"
                        placeholder={t('feed.whatAreYouThinking')}
                        className="post-creator__input"
                        value={postContent}
                        onChange={handleTextChange}
                        readOnly={!isExpanded}
                    />
                </div>
            </div>

            {isExpanded && (
                <div className="post-creator__expanded">
                    <textarea
                        className="post-creator__textarea"
                        placeholder={t('feed.whatAreYouThinking')}
                        value={postContent}
                        onChange={handleTextChange}
                    />

                    {previewUrls.length > 0 && (
                        <div className="post-creator__attachments">
                            {previewUrls.map((preview, index) => (
                                <div key={index} className="post-creator__attachment-item">
                                    {preview.type === 'image' ? (
                                        <img src={preview.url} alt={preview.name} />
                                    ) : (
                                        <video src={preview.url} controls />
                                    )}
                                    <button
                                        className="post-creator__remove-btn"
                                        onClick={() => handleRemoveAttachment(index)}
                                        aria-label={t('feed.removeAttachment')}
                                    >
                                        ×
                                    </button>
                                </div>
                            ))}
                        </div>
                    )}

                    <div className="post-creator__actions">
                        <div className="post-creator__attachment-buttons">
                            <label className="post-creator__attachment-btn">
                                <input
                                    type="file"
                                    multiple
                                    accept="image/*,video/*"
                                    onChange={handleFileChange}
                                    style={{ display: 'none' }}
                                />
                                <span>📷 {t('feed.addPhoto')}</span>
                            </label>
                            <label className="post-creator__attachment-btn">
                                <input
                                    type="file"
                                    multiple
                                    accept="video/*"
                                    onChange={handleFileChange}
                                    style={{ display: 'none' }}
                                />
                                <span>🎥 {t('feed.addVideo')}</span>
                            </label>
                        </div>
                        <div className="post-creator__action-buttons">
                            <Button
                                variant="secondary"
                                onClick={handleCancel}
                            >
                                {t('feed.cancel')}
                            </Button>
                            <Button
                                variant="primary"
                                onClick={handlePostSubmit}
                                disabled={loading || (!postContent.trim() && attachments.length === 0)}
                            >
                                {loading ? 'Posting...' : t('feed.post')}
                            </Button>
                        </div>
                    </div>
                </div>
            )}

            {!isExpanded && (
                <div className="post-creator__footer">
                    <label className="post-creator__footer-btn">
                        <input
                            type="file"
                            accept="image/*"
                            onChange={handleFileChange}
                            style={{ display: 'none' }}
                        />
                        <span>📷</span>
                    </label>
                    <label className="post-creator__footer-btn">
                        <input
                            type="file"
                            accept="video/*"
                            onChange={handleFileChange}
                            style={{ display: 'none' }}
                        />
                        <span>🎥</span>
                    </label>
                </div>
            )}
        </div>
    );
};

export default PostCreator;
