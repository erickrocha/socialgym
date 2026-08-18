import React, {useState} from 'react';

import './Profile.scss';
import {useDispatch, useSelector} from "react-redux";
import {AppHeader, Avatar, Button, CoverPhoto, ProfileSidebar, Spinner, Toast} from "../../commons/gui/index.js";
import * as handler from '../../redux/reducers/person/index.js';
import Modal from "../../commons/gui/Modal/Modal.jsx";
import {useTranslation} from 'react-i18next';
import PersonInfoForm from "./PersonInfoForm/index.js";

const Profile = () => {
    const {t} = useTranslation('common');
    const [activeSection, setActiveSection] = useState('about');

    const {person, error, loading} = useSelector((state) => state.person);
    const name = person ? `${person?.firstname} ${person?.surname}` : 'User';

    const [avatarImage, setAvatarImage] = useState(person?.avatar);
    const [coverImage, setCoverImage] = useState(person?.cover);
    const [mimeType, setMimeType] = useState(null)

    const [isEditModalOpen, setIsEditModalOpen] = useState(false);

    const dispatch = useDispatch();

    const handleCoverConfirm = () => {
        if (coverImage) {
            dispatch(handler.uploadCoverImage(coverImage));
        }
    };

    const handleAvatarConfirm = () => {
        if (avatarImage) {
            const payload = {file: avatarImage, format: mimeType};
            dispatch(handler.uploadAvatar(payload));
        }
    };

    const handleSectionChange = (section) => {
        if (section === 'edit') {
            setIsEditModalOpen(true);
        } else {
            setActiveSection(section);
        }
    };


    return (
        <>
            <AppHeader person={person}/>
            {loading ? <Spinner/> : <>
                <Toast
                    message={error}
                    type="error"
                    autoCloseDuration={5000}
                />
                <div className="profile-page">
                    <CoverPhoto onConfirm={handleCoverConfirm} image={coverImage} setImage={setCoverImage}
                                setMimeType={setMimeType}/>

                    <div className="profile-content-wrapper">
                        <ProfileSidebar
                            activeSection={activeSection}
                            onSectionChange={handleSectionChange}
                        />

                        <div className="profile-main-content">
                            <div className="profile-container">
                                <div className="profile-header">
                                    <Avatar image={avatarImage} gender={person?.gender} setImage={setAvatarImage}
                                            onConfirm={handleAvatarConfirm} size="xl" setMimeType={setMimeType}/>
                                    <div className="profile-info">
                                        <h1>{name}</h1>
                                    </div>
                                    <div className="profile-action-buttons">
                                        <Button
                                            className="secondary-button"
                                        >
                                            {t('profile.createCoachProfile')}
                                        </Button>
                                        <Button
                                            className="primary-button"
                                        >
                                            {t('profile.createBusinessProfile')}
                                        </Button>
                                    </div>
                                </div>

                                {activeSection === 'about' && person?.personInfo && (
                                    <div className="profile-details-box">
                                        <div className="detail-item">
                                            <span className="detail-label">{t('profile.biography')}</span>
                                            <span className="detail-value">{person.personInfo.biography}</span>
                                        </div>
                                        <div className="detail-item">
                                            <span className="detail-label">{t('profile.relationship')}</span>
                                            <span className="detail-value">{person.personInfo.relationship}</span>
                                        </div>
                                        <div className="detail-item">
                                            <span className="detail-label">{t('profile.job')}</span>
                                            <span className="detail-value">{person.personInfo.job}</span>
                                        </div>
                                        <div className="detail-item">
                                            <span className="detail-label">{t('profile.homeTown')}</span>
                                            <span className="detail-value">{person.personInfo.homeTown}</span>
                                        </div>
                                        <div className="detail-item">
                                            <span className="detail-label">{t('profile.currentCity')}</span>
                                            <span className="detail-value">{person.personInfo.currentCity}</span>
                                        </div>
                                        <div className="detail-item">
                                            <span className="detail-label">{t('profile.weight')}</span>
                                            <span className="detail-value">{person.personInfo.weight} kg</span>
                                        </div>
                                        <div className="detail-item">
                                            <span className="detail-label">{t('profile.height')}</span>
                                            <span className="detail-value">{person.personInfo.height} cm</span>
                                        </div>
                                        <div className="detail-item">
                                            <span className="detail-label">{t('profile.dateOfBirth')}</span>
                                            <span className="detail-value">{person.dateOfBirth}</span>
                                        </div>
                                        <div className="detail-item-action">
                                            <Button
                                                className="primary-button"
                                                onClick={() => setIsEditModalOpen(true)}
                                            >
                                                {t('profile.edit')}
                                            </Button>
                                        </div>
                                    </div>
                                )}
                            </div>
                        </div>
                    </div>
                </div>


                <Modal
                    isOpen={isEditModalOpen}
                    onClose={() => setIsEditModalOpen(false)}
                    title={t('modal.editProfileInformation')}
                    isForm={true}
                >
                    <PersonInfoForm
                        personInfo={person?.personInfo}
                        onCancel={() => setIsEditModalOpen(false)}
                        onConfirm={() => setIsEditModalOpen(false)}/>
                </Modal>
            </>
            }
        </>
    );
};

export default Profile;

