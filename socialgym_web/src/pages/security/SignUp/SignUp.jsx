import "./SignUp.scss";
import socialgym from '../../../assets/img/logo.png';
import {useEffect, useState} from "react";
import {useForm} from "react-hook-form";
import {useTranslation} from 'react-i18next';
import Logo from '../../../commons/gui/Logo/Logo.jsx';
import Divider from '../../../commons/gui/Divider/Divider.jsx';
import {Button, PasswordField, RadioGroup, Select, TextField} from "../../../commons/gui/index.js";
import {useDispatch, useSelector} from "react-redux";
import * as handler from '../../../redux/reducers/auth/index.js';
import {useNavigate} from "react-router";

const SignUp = () => {
    const { t } = useTranslation('common');

    const { register, handleSubmit, watch, formState: { errors } } = useForm();
    const [showCustomGender, setShowCustomGender] = useState(false);
    const selectedGender = watch("gender");
    const termsAccepted = watch("termsAccepted");
    const privacyAccepted = watch("privacyAccepted");

    // Handle gender change to show/hide custom gender field
    const handleGenderChange = (e) => {
        setShowCustomGender(e.target.value === "Custom");
    };

    const {auth} = useSelector((state) => state.auth);
    const navigate = useNavigate()

    useEffect(() => {
        if (auth) {
            navigate('/home');
        }
    }, [navigate, auth]);

    // ...existing code...
    const monthOptions = [
        { value: "01", label: t('months.January') },
        { value: "02", label: t('months.February') },
        { value: "03", label: t('months.March') },
        { value: "04", label: t('months.April') },
        { value: "05", label: t('months.May') },
        { value: "06", label: t('months.June') },
        { value: "07", label: t('months.July') },
        { value: "08", label: t('months.August') },
        { value: "09", label: t('months.September') },
        { value: "10", label: t('months.October') },
        { value: "11", label: t('months.November') },
        { value: "12", label: t('months.December') },
    ];

    // Generate day options (1-31)
    const dayOptions = Array.from({ length: 31 }, (_, i) => ({
        value: String(i + 1).padStart(2, '0'),
        label: String(i + 1),
    }));

    // Generate year options (from current year back 100 years)
    const currentYear = new Date().getFullYear();
    const yearOptions = Array.from({ length: 100 }, (_, i) => {
        const year = currentYear - i;
        return { value: String(year), label: String(year) };
    });

    // Gender options
    const genderOptions = [
        { value: "Female", label: t('signUp.genderFemale') },
        { value: "Male", label: t('signUp.genderMale') },
        { value: "Custom", label: t('signUp.genderCustom') },
    ];

    useEffect(() => {
        setShowCustomGender(selectedGender === "Custom");
    }, [selectedGender]);

    const dispatch = useDispatch();

    const onSubmit = (data) => {
        // Handle custom gender
        if (data.gender === "Custom") {
            data.gender = data.customGender;
        }
        data.dateOfBirth = `${data.birthdayYear}-${data.birthdayMonth}-${data.birthdayDay}`;
        const birthDate = new Date(`${data.dateOfBirth}T00:00:00`);
        const cutoff = new Date();
        cutoff.setFullYear(cutoff.getFullYear() - 18);
        if (birthDate > cutoff) return;
        data.termsVersion = import.meta.env.VITE_TERMS_VERSION || '1.0.0';
        data.privacyVersion = import.meta.env.VITE_PRIVACY_VERSION || '1.0.0';
        // Clean up unused fields
        delete data.birthdayYear;
        delete data.birthdayMonth;
        delete data.birthdayDay;
        delete data.customGender;

        // Dispatch signup action here
        dispatch(handler.signUp(data));
    };

    return (
        <div className="SignUp">
            <div className="body">
                <Logo src={socialgym} alt="Social Gym Logo" />
                <div className="form-container">
                    <h1>{t('signUp.title')}</h1>
                    <form onSubmit={handleSubmit(onSubmit)}>
                        {/* First Name */}
                        <TextField
                            id="firstname"
                            name="firstname"
                            placeholder={t('signUp.firstName')}
                            register={register}
                            required
                        />

                        {/* Surname */}
                        <TextField
                            id="surname"
                            name="surname"
                            placeholder={t('signUp.surname')}
                            register={register}
                            required
                        />

                        {/* Birthday */}
                        <fieldset className="birthday-group">
                            <div className="birthday-inputs">
                                <Select
                                    id="birthdayMonth"
                                    name="birthdayMonth"
                                    label={t('signUp.birthdayMonth')}
                                    placeholder={t('signUp.birthdayMonth')}
                                    options={monthOptions}
                                    register={register}
                                    required
                                />
                                <Select
                                    id="birthdayDay"
                                    name="birthdayDay"
                                    label={t('signUp.birthdayDay')}
                                    placeholder={t('signUp.birthdayDay')}
                                    options={dayOptions}
                                    register={register}
                                    required
                                />
                                <Select
                                    id="birthdayYear"
                                    name="birthdayYear"
                                    label={t('signUp.birthdayYear')}
                                    placeholder={t('signUp.birthdayYear')}
                                    options={yearOptions}
                                    register={register}
                                    required
                                />
                            </div>
                        </fieldset>

                        {/* Gender */}
                        <RadioGroup
                            id="gender"
                            name="gender"
                            options={genderOptions}
                            register={register}
                            onChange={handleGenderChange}
                            required
                        />

                        {/* Custom Gender Field */}
                        {showCustomGender && (
                            <TextField
                                id="customGender"
                                name="customGender"
                                label={t('signUp.genderCustomPlaceholder')}
                                placeholder={t('signUp.genderCustomPlaceholder')}
                                register={register}
                                required
                            />
                        )}

                        {/* Mobile or Email */}
                        <TextField
                            id="email"
                            name="email"
                            label={t('signUp.mobileOrEmail')}
                            placeholder={t('signUp.mobileOrEmail')}
                            type="text"
                            register={register}
                            required
                        />

                        {/* Password */}
                        <PasswordField
                            id="password"
                            name="password"
                            label={t('signUp.password')}
                            placeholder={t('signUp.password')}
                            register={register}
                            required
                            aria-describedby="password-hint"
                        />
                        <p id="password-hint" className="field-hint">{t('signUp.passwordHint')}</p>

                        <label className="legal-acceptance">
                            <input type="checkbox" {...register('termsAccepted', { required: true })} />
                            <span>Li e aceito os <a href="/terms" target="_blank" rel="noreferrer">Termos de Uso</a>.</span>
                        </label>
                        <label className="legal-acceptance">
                            <input type="checkbox" {...register('privacyAccepted', { required: true })} />
                            <span>Li e aceito a <a href="/privacy" target="_blank" rel="noreferrer">Política de Privacidade</a>.</span>
                        </label>

                        {/* Submit Button */}
                        <Button className="primary-button" id="sign-up-button" type="submit" disabled={!termsAccepted || !privacyAccepted}>
                            {t('signUp.submit')}
                        </Button>
                    </form>

                    <Divider />

                    <div className="sign-in-link">
                        <a href="/login">{t('signUp.alreadyHaveAccount')}</a>
                    </div>
                </div>
            </div>
        </div>
    );
};

export default SignUp;
