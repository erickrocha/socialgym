import {Button, CheckBoxGroup, TextField, VisibilityDropdown} from "../../../commons/gui/index.js";
import {useTranslation} from "react-i18next";
import {Controller, useForm} from "react-hook-form";
import './WorkoutForm.scss';
import React, {useState} from "react";
import {updateObject} from "../../../commons/library/utility.js";
import {useDispatch} from "react-redux";
import * as handler from "../../../redux/reducers/workout/index.js";
import {DifficultyDropdown} from "../../../commons/components/index.js";

const WorkoutForm = ({workout, person, onCancel, onConfirm}) => {
    const {t} = useTranslation('common');
    const {register, handleSubmit, control} = useForm({
        defaultValues: {
            name: workout?.name || '',
            description: workout?.description || '',
            difficulty: workout?.difficulty || 'soft',
            muscleGroup: workout?.muscleGroup || [],
            visibility: workout?.visibility || 'Private',
        }
    });

    const [muscleGroups, setMuscleGroups] = useState([])


    const muscleGroupOptions = [
        {value: 'chest', label: t('workout.muscleGroups.chest')},
        {value: 'legs', label: t('workout.muscleGroups.legs')},
        {value: 'back', label: t('workout.muscleGroups.back')},
        {value: 'core', label: t('workout.muscleGroups.core')},
        {value: 'full_body', label: t('workout.muscleGroups.full_body')},
        {value: 'shoulders', label: t('workout.muscleGroups.shoulders')},
        {value: 'arms', label: t('workout.muscleGroups.arms')},
    ];

    const handleCheckboxChange = (items) => {
        setMuscleGroups(items);
    }

    const dispatch = useDispatch();

    const onSubmit = (data) => {
        const payload = updateObject(data,{
            muscleGroup: muscleGroups.join("|"),
            ownerId: person.id,
        })
        dispatch(handler.addWorkout(payload))
        onConfirm();
    }

    const handleCancel = () => {
        onCancel();
    }

    return (
        <form className="workout-form" onSubmit={handleSubmit(onSubmit)}>
            <TextField
                name="name"
                label={t('workout.form.name')}
                placeholder={t('workout.form.namePlaceholder')}
                register={register}
                required
            />
            <TextField
                name="description"
                label={t('workout.form.description')}
                placeholder={t('workout.form.descriptionPlaceholder')}
                register={register}
            />
            <div className="workout-form__row">
                <Controller
                    name="difficulty"
                    control={control}
                    render={({ field: { onChange, value, name } }) => (
                        <DifficultyDropdown
                            name={name}
                            value={value}
                            onChange={onChange}
                        />
                    )}
                />
                <Controller
                    name="visibility"
                    control={control}
                    render={({ field: { onChange, value, name } }) => (
                        <VisibilityDropdown
                            name={name}
                            value={value}
                            onChange={onChange}
                        />
                    )}
                />
            </div>
           <CheckBoxGroup
               name="muscle-groups"
               items={muscleGroupOptions}
               title={t('workout.form.muscleGroups')}
               initialValues={workout?.muscleGroups || []}
               onChange={handleCheckboxChange} />
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

export default WorkoutForm;