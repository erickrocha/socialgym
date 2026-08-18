import React, {useEffect, useState} from 'react';
import {useDispatch, useSelector} from 'react-redux';
import {AppHeader, Sidebar,Toast,Spinner} from '../../commons/gui/index.js';
import Feed from '../../commons/gui/Feed/Feed';
import defaultAvatarFemale from '../../assets/img/avatar_female.png';
import defaultAvatarMale from '../../assets/img/avatar_male.png';
import './Home.scss';
import * as handler from '../../redux/reducers/person/index.js';

const Home = () => {
    const [isSidebarCollapsed, setIsSidebarCollapsed] = useState(false);

    const {person, error, loading} = useSelector((state) => state.person);

    const handleSidebarToggle = () => {
        setIsSidebarCollapsed(!isSidebarCollapsed);
    };

    const dispatch = useDispatch();

    useEffect(() => {
        dispatch(handler.getMe())
    }, [])

    const defaultAvatar = person?.gender === 'male' ? defaultAvatarMale : defaultAvatarFemale;
    const avatarImage = person?.avatar ? `data:image/jpeg;base64,${person?.avatar}` : defaultAvatar;

    return (
        <div className="home-layout">
            <AppHeader person={person}/>
            {loading && <Spinner />}
            <Toast
                message={error}
                type="error"
                autoCloseDuration={5000}
            />
            <div className="home-container">
                <Sidebar
                    person={person}
                    isCollapsed={isSidebarCollapsed}
                    onToggle={handleSidebarToggle}
                />
                <Feed
                    person={person}
                    userAvatar={avatarImage}
                    defaultAvatar={defaultAvatar}
                />
            </div>
        </div>
    );
};

export default Home;