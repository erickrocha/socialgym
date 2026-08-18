import React from "react";
import clsx from "clsx";
import PropTypes from "prop-types";
import "./TextField.scss";

const TextField = ({
    id,
    name,
    className,
    type = 'text',
    label,
    placeholder,
    defaultValue,
    value,
    register,
    required,
    "aria-describedby": ariaDescribedby,
    onChange
}) => (
    <div className={clsx("TextField", className)}>
        {label && (
            <label htmlFor={id || name} id={`${id || name}-label`}>
                {label}
                {required && <span aria-label="required">*</span>}
            </label>
        )}
        <input
            id={id || name}
            name={name || id}
            type={type}
            placeholder={placeholder}
            defaultValue={defaultValue}
            value={value}
            {...(register ? register(name || id, { required }) : {})}
            onChange={onChange}
            aria-label={label || placeholder}
            aria-describedby={ariaDescribedby}
            aria-required={required}
        />
    </div>
);

TextField.propTypes = {
    id: PropTypes.string,
    name: PropTypes.string,
    className: PropTypes.string,
    type: PropTypes.string,
    label: PropTypes.string,
    placeholder: PropTypes.string,
    defaultValue: PropTypes.string,
    value: PropTypes.string,
    register: PropTypes.func,
    required: PropTypes.bool,
    "aria-describedby": PropTypes.string,
    onChange: PropTypes.func
}

export default TextField;