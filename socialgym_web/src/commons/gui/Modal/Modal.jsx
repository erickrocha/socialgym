import React from 'react';
import './Modal.scss';
import Button from '../Button/Button';

const Modal = ({ isOpen, onClose, title, children, onConfirm, confirmText = 'Save', cancelText = 'Cancel', isForm = false, isFullScreen = false }) => {
    if (!isOpen) return null;

    const handleOverlayClick = (e) => {
        if (e.target === e.currentTarget && !isFullScreen) {
            onClose();
        }
    };

    return (
        <div className={`modal-overlay ${isFullScreen ? 'modal-overlay--fullscreen' : ''}`} onClick={handleOverlayClick}>
            <div className={`modal-content ${isFullScreen ? 'modal-content--fullscreen' : ''}`}>
                {!isFullScreen && (
                    <div className="modal-header">
                        <h2>{title}</h2>
                        <button
                            className="modal-close-btn"
                            onClick={onClose}
                            aria-label="Close modal"
                            type="button"
                        >
                            ✕
                        </button>
                    </div>
                )}
                <div className={`modal-body ${isFullScreen ? 'modal-body--fullscreen' : ''}`}>
                    {children}
                </div>
                {!isForm && !isFullScreen && (<div className="modal-footer">
                    <Button
                        onClick={onClose}
                        className="modal-btn modal-btn--cancel"
                    >
                        {cancelText}
                    </Button>
                    <Button
                        onClick={onConfirm}
                        className="primary-button"
                    >
                        {confirmText}
                    </Button>
                </div>)}
            </div>
        </div>
    );
};

export default Modal;
