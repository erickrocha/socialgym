import {useForm} from "react-hook-form";
import {Button, TextField, Divider} from "../../../commons/gui/index.js";
import {useTranslation} from "react-i18next";
import {useDispatch} from "react-redux";
import * as handler from "../../../redux/reducers/person/index.js";
import React from "react";
import './PersonInfoForm.scss';
import {updateObject} from "../../../commons/library/utility.js";

const PersonInfoForm = ({personInfo, onCancel, onConfirm}) => {
    const {t} = useTranslation('common');
    const {register, handleSubmit, watch, formState: {errors}} = useForm({
        defaultValues: {
            biography: personInfo?.biography || '',
            relationship: personInfo?.relationship || '',
            job: personInfo?.job || '',
            homeTown: personInfo?.homeTown || '',
            currentCity: personInfo?.currentCity || '',
            weight: personInfo?.weight || '',
            height: personInfo?.height || '',
        }
    });

    const dispatch = useDispatch();

    const onSubmit = (data) => {
        const payload = updateObject(personInfo,{
            ...data
        })
        dispatch(handler.updatePersonInfo(payload));
        onConfirm();
    }

    const handleCancel = () => {
        onCancel();
    }

    return (
        <form onSubmit={handleSubmit(onSubmit)} className="PersonInfoForm">
            <TextField
                id="biography"
                name="biography"
                label={t('profile.biography')}
                placeholder={t('profile.enterBiography')}
                register={register}
            />
            <TextField
                id="relationship"
                name="relationship"
                label={t('profile.relationship')}
                placeholder={t('profile.enterRelationship')}
                register={register}
            />
            <TextField
                id="job"
                name="job"
                label={t('profile.job')}
                placeholder={t('profile.enterJobTitle')}
                register={register}
            />
            <TextField
                id="homeTown"
                name="homeTown"
                label={t('profile.homeTown')}
                placeholder={t('profile.enterHomeTown')}
                register={register}
            />
            <TextField
                id="currentCity"
                name="currentCity"
                label={t('profile.currentCity')}
                placeholder={t('profile.enterCurrentCity')}
                register={register}
            />
            <TextField
                id="weight"
                name="weight"
                label={t('profile.weight')}
                placeholder={t('profile.enterWeight')}
                register={register}
                type="number"
            />
            <TextField
                id="height"
                name="height"
                label={t('profile.height')}
                placeholder={t('profile.enterHeight')}
                register={register}
                type="number"
            />
            <Divider />
            <div className="action-buttons">
                <Button variant="secondary" onClick={handleCancel}>
                    {t('application.button.cancel')}
                </Button>
                <Button variant="primary" type="submit">
                    {t('application.button.confirm')}
                </Button>
            </div>
        </form>
    )
}

export default PersonInfoForm;