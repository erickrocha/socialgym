import React, { useState } from "react";
import clsx from "clsx";
import PropTypes from "prop-types";
import "./PasswordField.scss";

const PasswordField = ({
  id,
  name,
  className,
  label,
  placeholder,
  defaultValue,
  register,
  required,
  "aria-describedby": ariaDescribedby,
  error,
}) => {
  const [showPassword, setShowPassword] = useState(false);

  const togglePasswordVisibility = () => {
    setShowPassword(!showPassword);
  };

  return (
    <div className={clsx("PasswordField", className)}>
      {label && (
        <label htmlFor={id || name} id={`${id || name}-label`}>
          {label}
          {required && <span aria-label="required">*</span>}
        </label>
      )}
      <div className="password-input-wrapper">
        <input
          type={showPassword ? "text" : "password"}
          id={id || name}
          name={name || id}
          placeholder={placeholder}
          defaultValue={defaultValue}
          {...(register ? register(name || id, { required }) : {})}
          aria-label={label || "password"}
          aria-describedby={ariaDescribedby}
          aria-required={required}
          className={clsx({ "has-error": error })}
        />
        <button
          type="button"
          className="toggle-password-btn"
          onClick={togglePasswordVisibility}
          aria-label={showPassword ? "Hide password" : "Show password"}
          aria-pressed={showPassword}
          title={showPassword ? "Hide password" : "Show password"}
        >
          {showPassword ? (
            <svg
              xmlns="http://www.w3.org/2000/svg"
              viewBox="0 0 24 24"
              fill="none"
              stroke="currentColor"
              strokeWidth="2"
              strokeLinecap="round"
              strokeLinejoin="round"
              aria-hidden="true"
            >
              <path d="M1 12s4-8 11-8 11 8 11 8-4 8-11 8-11-8-11-8z"></path>
              <circle cx="12" cy="12" r="3"></circle>
            </svg>
          ) : (
            <svg
              xmlns="http://www.w3.org/2000/svg"
              viewBox="0 0 24 24"
              fill="none"
              stroke="currentColor"
              strokeWidth="2"
              strokeLinecap="round"
              strokeLinejoin="round"
              aria-hidden="true"
            >
              <path d="M17.94 17.94A10.07 10.07 0 0 1 12 20c-7 0-11-8-11-8a18.45 18.45 0 0 1 5.06-5.94M9.9 4.24A9.12 9.12 0 0 1 12 4c7 0 11 8 11 8a18.5 18.5 0 0 1-2.16 3.19m-6.72-1.07a3 3 0 1 1-4.24-4.24"></path>
              <line x1="1" y1="1" x2="23" y2="23"></line>
            </svg>
          )}
        </button>
      </div>
      {error && (
        <span className="password-error" role="alert">
          {error}
        </span>
      )}
    </div>
  );
};

PasswordField.propTypes = {
  id: PropTypes.string,
  name: PropTypes.string,
  className: PropTypes.string,
  label: PropTypes.string,
  placeholder: PropTypes.string,
  defaultValue: PropTypes.string,
  register: PropTypes.func,
  required: PropTypes.bool,
  "aria-describedby": PropTypes.string,
  error: PropTypes.string,
};

export default PasswordField;

