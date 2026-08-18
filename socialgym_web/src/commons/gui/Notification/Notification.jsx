import {Button} from "../Button/index.jsx";
import React from "react";
import {useTranslation} from "react-i18next";
import './Notification.scss';


const Notification = ({show = false, onCancel, onConfirm}) => {
    const {t} = useTranslation('common');

    return show && (
        <div className="notification">
            <div className="notification-content">
                {/*<span className="notification-message">{t('profile.unsavedChanges')}</span>*/}
                <div className="notification-buttons">
                    <Button
                        type="button"
                        onClick={onCancel}
                        className="notification-btn notification-btn--cancel"

                    >
                        {t('application.button.cancel')}
                    </Button>
                    <Button
                        type="button"
                        onClick={onConfirm}
                        className="notification-btn notification-btn--confirm"
                    >
                        {t('application.button.confirm')}
                    </Button>
                </div>
            </div>
        </div>
    )
}

export default Notification;