import React from 'react';
import PropTypes from "prop-types";
import clsx from "clsx";

const Logo = ({src, alt = 'logo', className = ''}) => (
    <div className={clsx("logo", className)}>
        <img src={src} alt={alt}/>
    </div>
);

Logo.propTypes = {
    src: PropTypes.string.isRequired,
    alt: PropTypes.string,
    className: PropTypes.string
}

export default Logo;

