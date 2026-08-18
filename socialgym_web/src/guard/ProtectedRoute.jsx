import {useSelector, useDispatch} from 'react-redux'
import {Outlet, useNavigate} from 'react-router-dom'
import {useEffect, useState} from "react";
import {isAuthValid} from "../commons/library/tokenUtils";
import {logout} from "../redux/reducers/auth";
import {Spinner} from "../commons/gui";

const ProtectedRoute = () => {
    const {auth} = useSelector((state) => state.auth)
    const dispatch = useDispatch();
    const navigate = useNavigate();
    const [isValidating, setIsValidating] = useState(true);

    useEffect(() => {
        const validateToken = () => {
            // Check if auth exists
            if (!auth) {
                navigate('/login');
                return;
            }

            // Validate if token is not expired
            if (!isAuthValid(auth)) {
                // Token is expired, clear auth and redirect to login
                dispatch(logout());
                navigate('/login');
                return;
            }

            setIsValidating(false);
        };

        validateToken();
    }, [auth, navigate, dispatch]);

    // Show loading spinner while validating
    if (isValidating) {
        return (
            <div style={{
                display: 'flex',
                justifyContent: 'center',
                alignItems: 'center',
                height: '100vh'
            }}>
                <Spinner />
            </div>
        );
    }

    return <Outlet/>
}
export default ProtectedRoute