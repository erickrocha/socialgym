import React, { useEffect, useState } from 'react';
import './Toast.scss';

const Toast = ({ message, type = 'error', autoCloseDuration = 5000 }) => {
    const [isVisible, setIsVisible] = useState(message !== undefined && message !== null);

    useEffect(() => {
        if (autoCloseDuration > 0) {
            const timer = setTimeout(() => {
                setIsVisible(false);
            }, autoCloseDuration);

            return () => clearTimeout(timer);
        }
    }, [autoCloseDuration]);

    const handleClose = () => {
        setIsVisible(false);
    };

    if (!isVisible) return null;

    return (
        <div className={`toast toast--${type}`}>
            <div className="toast__content">
                <span className="toast__message">{message}</span>
            </div>
            <button
                className="toast__close-btn"
                onClick={handleClose}
                aria-label="Close notification"
                type="button"
            >
                ✕
            </button>
        </div>
    );
};

export default Toast;
