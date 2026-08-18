import React, {useState} from 'react';
import PropTypes from 'prop-types';
import clsx from 'clsx';
import './Avatar.scss';
import defaultAvatarMale from "../../../assets/img/avatar_male.png";
import defaultAvatarFemale from "../../../assets/img/avatar_female.png";
import {CameraIcon, CancelIcon, SaveIcon} from '../Icons/Icons.tsx';

const Avatar = ({image, setImage, setMimeType, onConfirm, gender = 'Male', size = 'md', className = ''}) => {

    const defaultAvatar = gender === 'Male' ? defaultAvatarMale : defaultAvatarFemale;
    const [preview, setPreview] = useState(image ? image : defaultAvatar);
    const [imageUpdated, setImageUpdated] = useState(false);

    const handleAvatarImageChange = (e) => {
        const file = e.target.files[0];
        if (file) {
            const reader = new FileReader();
            reader.onloadend = () => {
                setPreview(reader.result);
                setImage(file);
            };
            reader.readAsDataURL(file);
            setMimeType(file.type);
            setImageUpdated(true);
        }
    };

    const handleConfirm = () => {
        onConfirm();
        setImageUpdated(false);
    }

    const handleCancel = () => {
        setPreview(null);
        setImageUpdated(false);
    }

    return (
        <>
            <div className="avatar-container">
                <div className={clsx('avatar', `avatar--${size}`, className)}>
                    {preview ? (
                        <img src={preview} alt="avatar"/>
                    ) : (
                        <span className="avatar-placeholder">Avatar</span>
                    )}
                </div>
                {onConfirm && (
                    <>
                        {imageUpdated ? (
                            <>
                                <button
                                    type="button"
                                    className="avatar-action-btn avatar-cancel-btn"
                                    onClick={handleCancel}
                                    aria-label="Cancel image change"
                                >
                                    <CancelIcon/>
                                </button>
                                <button
                                    type="button"
                                    className="avatar-action-btn avatar-save-btn"
                                    onClick={handleConfirm}
                                    aria-label="Save image"
                                >
                                    <SaveIcon/>
                                </button>
                            </>
                        ) : (
                            <>
                                <label htmlFor="avatar-input" className="avatar-upload-label"
                                       aria-label="Upload profile picture">
                                    <CameraIcon/>
                                </label>
                                <input
                                    id="avatar-input"
                                    type="file"
                                    accept="image/*"
                                    onChange={handleAvatarImageChange}
                                    className="profile-input hidden"
                                    aria-label="Upload profile picture"
                                />
                            </>
                        )}
                    </>
                )}
            </div>
        </>
    );
};

Avatar.propTypes = {
    src: PropTypes.string,
    alt: PropTypes.string,
    size: PropTypes.oneOf(['sm', 'md', 'lg', 'xl']),
    className: PropTypes.string,
};

export default Avatar;

