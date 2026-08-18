import {NavLink} from "react-router-dom";
import {clsx} from "clsx";
import PropTypes from "prop-types";
import './ButtonLink.scss';


const ButtonLink = ({children, to, className}) => {
    return (
        <NavLink to={to} className={clsx("ButtonLink", className)}>{children}</NavLink>
    );
}

ButtonLink.propTypes = {
    children: PropTypes.element,
    to: PropTypes.string,
    className: PropTypes.string
};

export default ButtonLink;