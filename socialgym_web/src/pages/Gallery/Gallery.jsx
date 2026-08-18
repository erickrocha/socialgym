import React, { useEffect, useMemo, useState } from 'react';
import { useDispatch, useSelector } from 'react-redux';
import { useTranslation } from 'react-i18next';
import { AppHeader, Sidebar } from '../../commons/gui/index.js';
import { getFeedPosts } from '../../redux/reducers/timeline/index.js';
import './Gallery.scss';

const Gallery = () => {
    const { t } = useTranslation('common');
    const dispatch = useDispatch();
    const { person } = useSelector((state) => state.person);
    const { posts, postsLoading, postsError } = useSelector((state) => state.timeline);
    const [isSidebarCollapsed, setIsSidebarCollapsed] = useState(false);

    useEffect(() => {
        dispatch(getFeedPosts());
    }, [dispatch]);

    const mediaPosts = useMemo(
        () => posts.filter((post) => Array.isArray(post.media) && post.media.length > 0),
        [posts],
    );

    return (
        <div className="gallery-page">
            <AppHeader person={person} />
            <div className="gallery-page__layout">
                <Sidebar
                    person={person}
                    isCollapsed={isSidebarCollapsed}
                    onToggle={() => setIsSidebarCollapsed((prev) => !prev)}
                />
                <main className="gallery-page__content">
                    <header className="gallery-page__header">
                        <h1>{t('gallery.title')}</h1>
                        <p>{t('gallery.subtitle')}</p>
                    </header>

                    {postsLoading && <div className="gallery-page__empty">{t('gallery.loading')}</div>}
                    {postsError && <div className="gallery-page__error">{postsError}</div>}

                    {!postsLoading && !postsError && mediaPosts.length === 0 && (
                        <div className="gallery-page__empty">{t('gallery.empty')}</div>
                    )}

                    <section className="gallery-page__feed">
                        {mediaPosts.map((post) => (
                            <article key={post.id} className="gallery-post">
                                <div className="gallery-post__meta">
                                    <strong>{post.authorName || t('gallery.unknownUser')}</strong>
                                    <span>{new Date(post.createdAt || Date.now()).toLocaleString()}</span>
                                </div>

                                <div className={`gallery-post__media gallery-post__media--${post.media.length > 1 ? 'grid' : 'single'}`}>
                                    {post.media.map((item, index) => (
                                        <div key={`${post.id}-${index}`} className="gallery-post__item">
                                            {item.mediaType?.toLowerCase() === 'video' ? (
                                                <video src={item.url} controls />
                                            ) : (
                                                <img src={item.url} alt={`media-${index}`} loading="lazy" />
                                            )}
                                        </div>
                                    ))}
                                </div>

                                {post.content && <p className="gallery-post__caption">{post.content}</p>}
                            </article>
                        ))}
                    </section>
                </main>
            </div>
        </div>
    );
};

export default Gallery;
