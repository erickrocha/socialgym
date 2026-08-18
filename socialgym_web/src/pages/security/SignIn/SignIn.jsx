import "./SignIn.scss"
import socialgym from '../../../assets/img/logo.png';
import {useNavigate} from "react-router";
import {useEffect} from "react";
import {useForm} from "react-hook-form";
import * as handler from '../../../redux/reducers/auth/index.js';
import {useTranslation} from 'react-i18next';
import {useDispatch, useSelector} from "react-redux";

import Header from '../../../commons/gui/Header/Header.jsx';
import Footer from '../../../commons/gui/Footer/Footer.jsx';
import Logo from '../../../commons/gui/Logo/Logo.jsx';
import Divider from '../../../commons/gui/Divider/Divider.jsx';
import {Button, TextField} from "../../../commons/gui/index.js";
import {NavLink} from "react-router-dom";
import {ButtonLink} from "../../../commons/gui/ButtonLink/index.js";


const SignIn = () => {

    const {t} = useTranslation('common');

    const {auth, user, loading} = useSelector((state) => state.auth);

    const {register, handleSubmit, reset} = useForm();


    const navigate = useNavigate()

    useEffect(() => {
        if (auth) {
            navigate('/home');
        }
    }, [navigate, auth]);

    // keep form email in sync with user (avoid controlled/uncontrolled)
    useEffect(() => {
        reset({ email: user?.email || '' });
    }, [user, reset]);

    const dispatch = useDispatch();

    const onSubmit = (data) => {
        const payload = {email: data.email, password: data.password};
        dispatch(handler.authentication(payload));
    }

    return (
        <div className="SignIn">
            <Header>
            </Header>
            <div className="body">
                <Logo src={socialgym} alt="Social Gym Logo"/>
                <div className="form-container">
                    <form onSubmit={handleSubmit(onSubmit)}>
                        <TextField id="email" name="email" className="input-group" type="email" placeholder={t('signIn.emailPlaceholder')} register={register} />
                        <TextField id="password" name="password" className="input-group" type="password" placeholder={t('signIn.passwordPlaceholder')} register={register} />
                        <Button className="primary-button" disabled={loading} type="submit"
                                id="sign-in-button">{t('signIn.submit')}</Button>
                    </form>
                    <NavLink to="/recovery">{t('signIn.forgotPassword')}</NavLink>
                    <Divider/>
                    <div className={"box-create-account"}>
                        <ButtonLink id="secondary-button" disabled={loading} className="secondary-button" to="/signup" >
                            {t('signIn.createAccount')}
                        </ButtonLink>
                    </div>
                </div>
            </div>
            <Footer showLanguageSwitcher={true} />
        </div>
    )
}

export default SignIn;