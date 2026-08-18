import React, {useEffect, useState} from 'react';
import {useDispatch, useSelector} from 'react-redux';
import {useParams, useNavigate} from 'react-router-dom';
import {useTranslation} from 'react-i18next';
import {AppHeader, Toast, Spinner, Button, CoverPhoto} from '../../../commons/gui/index.js';
import Modal from '../../../commons/gui/Modal/Modal.jsx';
import * as businessHandler from '../../../redux/reducers/business/index.js';
import {clearSelectedProfile} from '../../../redux/reducers/business/business.slice.js';
import BusinessProfileForm from './BusinessProfileForm.jsx';
import BusinessSidebar from './BusinessSidebar.jsx';
import './BusinessProfile.scss';

const BusinessProfile = () => {
    const {t} = useTranslation('common');
    const {id} = useParams();
    const navigate = useNavigate();
    const dispatch = useDispatch();

    const {person} = useSelector((state) => state.person);
    const {selectedProfile, loading, error} = useSelector((state) => state.business);

    const [activeSection, setActiveSection] = useState('about');
    const [isEditModalOpen, setIsEditModalOpen] = useState(false);
    const [coverImage, setCoverImage] = useState(null);
    const [mimeType, setMimeType] = useState(null);

    useEffect(() => {
        if (id) {
            dispatch(businessHandler.getBusinessProfileById(id));
        }
        return () => {
            dispatch(clearSelectedProfile());
        };
    }, [dispatch, id]);

    useEffect(() => {
        if (selectedProfile?.coverImageUrl) {
            setCoverImage(selectedProfile.coverImageUrl);
        }
    }, [selectedProfile]);

    const handleSectionChange = (section) => {
        if (section === 'edit') {
            setIsEditModalOpen(true);
        } else if (section === 'team') {
            navigate(`/business/${id}/team`);
        } else {
            setActiveSection(section);
        }
    };

    const handleBack = () => {
        navigate('/business');
    };

    const handleCoverConfirm = () => {
        // TODO: Implement cover upload for business profile
        console.log('Cover upload for business profile');
    };

    if (loading && !selectedProfile) {
        return (
            <div className="business-profile-layout">
                <AppHeader person={person}/>
                <Spinner/>
            </div>
        );
    }

    return (
        <>
            <AppHeader person={person}/>
            {loading && <Spinner/>}
            <Toast
                message={error}
                type="error"
                autoCloseDuration={5000}
            />
            <div className="business-profile-page">
                <CoverPhoto
                    image={coverImage}
                    setImage={setCoverImage}
                    setMimeType={setMimeType}
                    onConfirm={handleCoverConfirm}
                />

                <div className="business-profile-content-wrapper">
                    <BusinessSidebar
                        activeSection={activeSection}
                        onSectionChange={handleSectionChange}
                        onBack={handleBack}
                    />

                    <div className="business-profile-main-content">
                        <div className="business-profile-container">
                            <div className="business-profile-header">
                                <div className="business-logo-wrapper">
                                    {selectedProfile?.logoUrl ? (
                                        <img
                                            src={selectedProfile.logoUrl}
                                            alt={selectedProfile.businessName}
                                            className="business-logo"
                                        />
                                    ) : (
                                        <div className="business-logo-placeholder">
                                            💼
                                        </div>
                                    )}
                                </div>
                                <div className="business-profile-info">
                                    <h1>{selectedProfile?.businessName}</h1>
                                    <span className="business-type-badge">{selectedProfile?.businessType}</span>
                                </div>
                                <div className="business-profile-actions">
                                    <Button
                                        className="secondary-button"
                                        onClick={handleBack}
                                    >
                                        {t('business.backToList')}
                                    </Button>
                                    <Button
                                        className="primary-button"
                                        onClick={() => setIsEditModalOpen(true)}
                                    >
                                        {t('business.editProfile')}
                                    </Button>
                                </div>
                            </div>

                            {activeSection === 'about' && selectedProfile && (
                                <div className="business-details-box">
                                    <h2 className="section-title">{t('business.businessInfo')}</h2>
                                    <div className="detail-item">
                                        <span className="detail-label">{t('business.businessName')}</span>
                                        <span className="detail-value">{selectedProfile.businessName || '-'}</span>
                                    </div>
                                    <div className="detail-item">
                                        <span className="detail-label">{t('business.socialName')}</span>
                                        <span className="detail-value">{selectedProfile.socialName || '-'}</span>
                                    </div>
                                    <div className="detail-item">
                                        <span className="detail-label">{t('business.businessType')}</span>
                                        <span className="detail-value">{selectedProfile.businessType || '-'}</span>
                                    </div>
                                    <div className="detail-item">
                                        <span className="detail-label">{t('business.taxId')}</span>
                                        <span className="detail-value">{selectedProfile.taxId || '-'}</span>
                                    </div>
                                </div>
                            )}

                            {activeSection === 'about' && selectedProfile?.addresses && selectedProfile.addresses.length > 0 && (
                                <div className="business-details-box">
                                    <h2 className="section-title">{t('business.addresses')}</h2>
                                    {selectedProfile.addresses.map((address) => (
                                        <div key={address.id} className="address-card">
                                            <div className="address-icon">📍</div>
                                            <div className="address-content">
                                                <p className="address-line address-main">
                                                    {address.street}, {address.addressNumber}
                                                    {address.complement && ` - ${address.complement}`}
                                                </p>
                                                <p className="address-line">
                                                    {address.neighborhood}
                                                </p>
                                                <p className="address-line">
                                                    {address.city}
                                                </p>
                                                <p className="address-line address-zip">
                                                    {t('business.zipcode')}: {address.zipcode}
                                                </p>
                                            </div>
                                        </div>
                                    ))}
                                </div>
                            )}

                            {activeSection === 'services' && (
                                <div className="business-details-box">
                                    <h2 className="section-title">{t('business.services')}</h2>
                                    <p className="empty-state">{t('business.noServicesYet')}</p>
                                </div>
                            )}

                            {activeSection === 'schedule' && (
                                <div className="business-details-box">
                                    <h2 className="section-title">{t('business.schedule')}</h2>
                                    <p className="empty-state">{t('business.noScheduleYet')}</p>
                                </div>
                            )}

                        </div>
                    </div>
                </div>
            </div>

            <Modal
                isOpen={isEditModalOpen}
                onClose={() => setIsEditModalOpen(false)}
                title={t('business.editBusinessProfile')}
                isForm={true}
            >
                <BusinessProfileForm
                    profile={selectedProfile}
                    onCancel={() => setIsEditModalOpen(false)}
                    onSuccess={() => setIsEditModalOpen(false)}
                />
            </Modal>
        </>
    );
};

export default BusinessProfile;

