import React from 'react';
import PropTypes from 'prop-types';
import {useTranslation} from 'react-i18next';
import Avatar from '../../gui/Avatar/Avatar.jsx';
import Button from '../../gui/Button/Button.jsx';
import defaultAvatarFemale from '../../../assets/img/avatar_female.png';
import defaultAvatarMale from '../../../assets/img/avatar_male.png';
import './FriendCard.scss';

const FriendCard = ({
                        friend,
                        variant = 'friend',
                        onAddFriend,
                        onRemoveFriend,
                        onAcceptRequest,
                        onDeclineRequest,
                        onCancelRequest,
                        onViewProfile,
                        loading = false
                    }) => {
    const {t} = useTranslation('common');

    const defaultAvatar = friend?.gender === 'Female' ? defaultAvatarFemale : defaultAvatarMale;
    const avatarSrc = friend?.avatar
        ? `data:image/jpeg;base64,${friend.avatar}`
        : defaultAvatar;

    const name = friend ? `${friend.firstname} ${friend.surname}` : t('friends.unknownUser');
    const mutualFriends = friend?.mutualFriends || 0;

    const renderActions = () => {
        switch (variant) {
            case 'suggestion':
                return (
                    <Button
                        variant="primary"
                        size="sm"
                        onClick={() => onAddFriend?.(friend)}
                        disabled={loading}
                    >
                        {t('friends.addFriend')}
                    </Button>
                );
            case 'receivedRequest':
                return (
                    <>
                        <Button
                            variant="primary"
                            size="sm"
                            onClick={() => onAcceptRequest?.(friend)}
                            disabled={loading}
                        >
                            {t('friends.accept')}
                        </Button>
                        <Button
                            variant="secondary"
                            size="sm"
                            onClick={() => onDeclineRequest?.(friend)}
                            disabled={loading}
                        >
                            {t('friends.decline')}
                        </Button>
                    </>
                );
            case 'sentRequest':
                return (
                    <Button
                        variant="secondary"
                        size="sm"
                        onClick={() => onCancelRequest?.(friend)}
                        disabled={loading}
                    >
                        {t('friends.cancelRequest')}
                    </Button>
                );
            case 'friend':
            default:
                return (
                    <>
                        <Button
                            variant="secondary"
                            size="sm"
                            onClick={() => onViewProfile?.(friend)}
                        >
                            {t('friends.viewProfile')}
                        </Button>
                        <Button
                            variant="secondary"
                            size="sm"
                            onClick={() => onRemoveFriend?.(friend)}
                            disabled={loading}
                        >
                            {t('friends.remove')}
                        </Button>
                    </>
                );
        }
    };

    return (
        <div className={`friend-card friend-card--${variant}`}>
            <div className="friend-card__avatar" onClick={() => onViewProfile?.(friend)}>
                <Avatar image={avatarSrc} size="md"/>
            </div>
            <div className="friend-card__info">
                <h4
                    className="friend-card__name"
                    onClick={() => onViewProfile?.(friend)}
                >
                    {name}
                </h4>
                {mutualFriends > 0 && (
                    <span className="friend-card__mutual">
            {t('friends.mutualFriends', {count: mutualFriends})}
          </span>
                )}
                {friend?.currentCity && (
                    <span className="friend-card__location">
            📍 {friend.currentCity}
          </span>
                )}
            </div>
            <div className="friend-card__actions">
                {renderActions()}
            </div>
        </div>
    );
};

FriendCard.propTypes = {
    friend: PropTypes.shape({
        id: PropTypes.number,
        firstname: PropTypes.string,
        surname: PropTypes.string,
        avatar: PropTypes.string,
        gender: PropTypes.string,
        mutualFriends: PropTypes.number,
        currentCity: PropTypes.string,
    }),
    variant: PropTypes.oneOf(['friend', 'suggestion', 'receivedRequest', 'sentRequest']),
    onAddFriend: PropTypes.func,
    onRemoveFriend: PropTypes.func,
    onAcceptRequest: PropTypes.func,
    onDeclineRequest: PropTypes.func,
    onCancelRequest: PropTypes.func,
    onViewProfile: PropTypes.func,
    loading: PropTypes.bool,
};

export default FriendCard;
