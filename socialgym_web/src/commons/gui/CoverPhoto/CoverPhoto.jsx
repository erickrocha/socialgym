import React, {useState} from 'react';
import {useTranslation} from 'react-i18next';
import './CoverPhoto.scss';
import {Notification} from '../index';
import {CameraIcon} from '../Icons/Icons.tsx';
import defaultCover from '../../../assets/img/cover_foto.png';

const CoverPhoto = ({image, setImage,setMimeType, onConfirm}) => {
    const {t} = useTranslation('common');

    const coverImage = image ? image : defaultCover;
    const [coverPreview, setCoverPreview] = useState(coverImage);
    const [imageUpdated, setImageUpdated] = useState(false);

    const handleCoverChange = (e) => {
        const file = e.target.files[0];
        if (file) {
            const reader = new FileReader();
            reader.onloadend = () => {
                setCoverPreview(reader.result);
                setImage(file)
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
        setCoverPreview(defaultCover);
        setImageUpdated(false);
    }

    return (
        <>
            <Notification show={imageUpdated} onConfirm={handleConfirm} onCancel={handleCancel} />
            <div className="cover-photo">
                <div className="cover-photo-cover" style={{backgroundImage: `url(${coverPreview})`}}>
                    {!imageUpdated && (<> <label htmlFor="cover-input" className="cover-photo-cover-upload-label">
                        <span className="camera-icon"><CameraIcon/></span>
                        <span className="label-text">{t('cover-photo.changeCoverPhoto')}</span>
                    </label>
                        <input
                            id="cover-input"
                            type="file"
                            accept="image/*"
                            onChange={handleCoverChange}
                            className="cover-input hidden"
                            aria-label="Upload cover photo"
                        /></>)}
                </div>
            </div>
        </>
    )
}

CoverPhoto.prototype = {
    image: String,
    setImage: Function,
    onCancel: Function,
    onConfirm: Function
}

export default CoverPhoto;