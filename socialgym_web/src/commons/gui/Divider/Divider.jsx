import React from 'react';
import './Divider.scss';
import clsx from "clsx";
import PropTypes from "prop-types";

const Divider = ({className = ''}) => <div className={clsx("Divider", className)}/>;

Divider.propTypes = {
    className: PropTypes.string,
}

export default Divider;

