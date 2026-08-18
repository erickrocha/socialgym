import React, {useEffect} from 'react';
import {useDispatch, useSelector} from 'react-redux';
import {useNavigate} from 'react-router-dom';
import {useTranslation} from 'react-i18next';
import {AppHeader, Sidebar, Toast, Spinner} from '../../commons/gui/index.js';
import * as businessHandler from '../../redux/reducers/business/index.js';
import './Business.scss';

const Business = () => {
    const {t} = useTranslation('common');
    const dispatch = useDispatch();
    const navigate = useNavigate();

    const {person} = useSelector((state) => state.person);
    const {businessProfiles, loading, error} = useSelector((state) => state.business);

    useEffect(() => {
        dispatch(businessHandler.getBusinessProfiles());
    }, [dispatch]);

    const handleCardClick = (profileId) => {
        navigate(`/business/${profileId}`);
    };

    return (
        <div className="business-layout">
            <AppHeader person={person}/>
            {loading && <Spinner/>}
            <Toast
                message={error}
                type="error"
                autoCloseDuration={5000}
            />
            <div className="business-container">
                <Sidebar person={person}/>
                <main className="business-content">
                    <header className="business-header">
                        <h1 className="business-title">{t('business.title')}</h1>
                        <p className="business-subtitle">{t('business.subtitle')}</p>
                    </header>

                    <div className="business-profiles-grid">
                        {businessProfiles && businessProfiles.length > 0 ? (
                            businessProfiles.map((profile) => (
                                <div
                                    key={profile.id}
                                    className="business-card business-card--clickable"
                                    onClick={() => handleCardClick(profile.id)}
                                    role="button"
                                    tabIndex={0}
                                    onKeyPress={(e) => e.key === 'Enter' && handleCardClick(profile.id)}
                                >
                                    <div className="business-card__header">
                                        {profile.logoUrl ? (
                                            <img
                                                src={profile.logoUrl}
                                                alt={profile.businessName}
                                                className="business-card__logo"
                                            />
                                        ) : (
                                            <div className="business-card__logo-placeholder">
                                                💼
                                            </div>
                                        )}
                                        <div className="business-card__info">
                                            <h2 className="business-card__name">{profile.businessName}</h2>
                                            <span className="business-card__type">{profile.businessType}</span>
                                        </div>
                                    </div>

                                    <div className="business-card__body">
                                        <div className="business-card__detail">
                                            <span className="business-card__label">{t('business.socialName')}:</span>
                                            <span className="business-card__value">{profile.socialName || '-'}</span>
                                        </div>
                                        <div className="business-card__detail">
                                            <span className="business-card__label">{t('business.taxId')}:</span>
                                            <span className="business-card__value">{profile.taxId || '-'}</span>
                                        </div>
                                    </div>

                                    {profile.addresses && profile.addresses.length > 0 && (
                                        <div className="business-card__addresses">
                                            <h3 className="business-card__addresses-title">{t('business.addresses')}</h3>
                                            {profile.addresses.map((address) => (
                                                <div key={address.id} className="business-card__address">
                                                    <p className="business-card__address-line">
                                                        {address.street}, {address.addressNumber}
                                                        {address.complement && ` - ${address.complement}`}
                                                    </p>
                                                    <p className="business-card__address-line">
                                                        {address.neighborhood}, {address.city}
                                                    </p>
                                                    <p className="business-card__address-line">
                                                        {t('business.zipcode')}: {address.zipcode}
                                                    </p>
                                                </div>
                                            ))}
                                        </div>
                                    )}
                                </div>
                            ))
                        ) : (
                            !loading && (
                                <div className="business-empty">
                                    <span className="business-empty__icon">💼</span>
                                    <p className="business-empty__text">{t('business.noProfiles')}</p>
                                </div>
                            )
                        )}
                    </div>
                </main>
            </div>
        </div>
    );
};

export default Business;

