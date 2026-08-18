import React from 'react';
import {useForm} from 'react-hook-form';
import {useDispatch} from 'react-redux';
import {useTranslation} from 'react-i18next';
import {Button, TextField, Divider} from '../../../commons/gui/index.js';
import * as businessHandler from '../../../redux/reducers/business/index.js';
import {updateObject} from '../../../commons/library/utility.js';
import './BusinessProfileForm.scss';

const BusinessProfileForm = ({profile, onCancel, onSuccess}) => {
    const {t} = useTranslation('common');
    const dispatch = useDispatch();

    const {register, handleSubmit, formState: {errors}} = useForm({
        defaultValues: {
            businessName: profile?.businessName || '',
            socialName: profile?.socialName || '',
            businessType: profile?.businessType || '',
            taxId: profile?.taxId || '',
        }
    });

    const onSubmit = (data) => {
        const payload = updateObject(profile, data);
        dispatch(businessHandler.updateBusinessProfile(payload))
            .unwrap()
            .then(() => {
                onSuccess();
            })
            .catch((error) => {
                console.error('Error updating business profile:', error);
            });
    };

    return (
        <form onSubmit={handleSubmit(onSubmit)} className="business-profile-form">
            <TextField
                id="businessName"
                name="businessName"
                label={t('business.form.businessName')}
                placeholder={t('business.form.businessNamePlaceholder')}
                register={register}
                required
                error={errors.businessName?.message}
            />
            <TextField
                id="socialName"
                name="socialName"
                label={t('business.form.socialName')}
                placeholder={t('business.form.socialNamePlaceholder')}
                register={register}
            />
            <TextField
                id="businessType"
                name="businessType"
                label={t('business.form.businessType')}
                placeholder={t('business.form.businessTypePlaceholder')}
                register={register}
                required
                error={errors.businessType?.message}
            />
            <TextField
                id="taxId"
                name="taxId"
                label={t('business.form.taxId')}
                placeholder={t('business.form.taxIdPlaceholder')}
                register={register}
            />

            <Divider />

            <div className="action-buttons">
                <Button variant="secondary" type="button" onClick={onCancel}>
                    {t('application.button.cancel')}
                </Button>
                <Button variant="primary" type="submit">
                    {t('application.button.confirm')}
                </Button>
            </div>
        </form>
    );
};

export default BusinessProfileForm;

