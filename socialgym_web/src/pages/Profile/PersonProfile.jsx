import React, { useEffect, useState } from 'react';
import { useSelector } from 'react-redux';
import { useNavigate, useParams } from 'react-router-dom';
import { useTranslation } from 'react-i18next';
import { AppHeader, Sidebar } from '../../commons/gui/index.js';
import { fetchFriendProfile } from '../../service/timeline/index.js';
import defaultAvatarFemale from '../../assets/img/avatar_female.png';
import defaultAvatarMale from '../../assets/img/avatar_male.png';
import defaultCover from '../../assets/img/cover_foto.png';
import './PersonProfile.scss';

const PersonProfile = () => {
    const { t } = useTranslation('common');
    const { person } = useSelector((state) => state.person);
    const navigate = useNavigate();
    const { id } = useParams();

    const [isSidebarCollapsed, setIsSidebarCollapsed] = useState(false);
    const [friend, setFriend] = useState(null);
    const [loading, setLoading] = useState(true);
    const [error, setError] = useState(null);

    useEffect(() => {
        const load = async () => {
            setLoading(true);
            setError(null);
            try {
                const data = await fetchFriendProfile(id);
                setFriend(data);
            } catch (err) {
                setError(err?.response?.data?.message || err?.message || t('personProfile.errors.load'));
            } finally {
                setLoading(false);
            }
        };

        if (id) {
            load();
        }
    }, [id]);

    const avatar = friend?.avatar ? `data:image/jpeg;base64,${friend.avatar}` : (friend?.gender === 'Female' ? defaultAvatarFemale : defaultAvatarMale);
    const cover = friend?.cover ? `data:image/jpeg;base64,${friend.cover}` : defaultCover;

    return (
        <div className="person-profile-page">
            <AppHeader person={person} />
            <div className="person-profile-page__layout">
                <Sidebar person={person} isCollapsed={isSidebarCollapsed} onToggle={() => setIsSidebarCollapsed((prev) => !prev)} />
                <main className="person-profile-page__content">
                    <button className="person-profile-page__back" onClick={() => navigate(-1)}>{t('personProfile.back')}</button>

                    {loading && <p>{t('personProfile.loading')}</p>}
                    {error && <p className="person-profile-page__error">{error}</p>}

                    {!loading && !error && friend && (
                        <article className="person-card">
                            <div className="person-card__cover">
                                <img src={cover} alt="cover" />
                            </div>
                            <div className="person-card__head">
                                <img className="person-card__avatar" src={avatar} alt="avatar" />
                                <div>
                                    <h1>{friend.firstname} {friend.surname}</h1>
                                    <p>{t('personProfile.memberSince')} {new Date(friend.createdAt || Date.now()).toLocaleDateString()}</p>
                                </div>
                            </div>

                            <div className="person-card__grid">
                                <section>
                                    <h3>{t('personProfile.personalInfo')}</h3>
                                    <p><strong>{t('personProfile.biography')}:</strong> {friend?.personInfo?.biography || '-'}</p>
                                    <p><strong>{t('personProfile.job')}:</strong> {friend?.personInfo?.job || '-'}</p>
                                    <p><strong>{t('personProfile.relationship')}:</strong> {friend?.personInfo?.relationship || '-'}</p>
                                    <p><strong>{t('personProfile.homeTown')}:</strong> {friend?.personInfo?.homeTown || '-'}</p>
                                    <p><strong>{t('personProfile.currentCity')}:</strong> {friend?.personInfo?.currentCity || '-'}</p>
                                </section>
                                <section>
                                    <h3>{t('personProfile.physicalStats')}</h3>
                                    <p><strong>{t('personProfile.weight')}:</strong> {friend?.personInfo?.weight || '-'} kg</p>
                                    <p><strong>{t('personProfile.height')}:</strong> {friend?.personInfo?.height || '-'} cm</p>
                                    <p><strong>{t('personProfile.dateOfBirth')}:</strong> {friend?.dateOfBirth || '-'}</p>
                                    <p><strong>{t('personProfile.gender')}:</strong> {friend?.gender || '-'}</p>
                                </section>
                            </div>
                        </article>
                    )}
                </main>
            </div>
        </div>
    );
};

export default PersonProfile;
