import React from "react";
import clsx from "clsx";
import PropTypes from "prop-types";
import "./RadioGroup.scss";

const RadioGroup = ({
  id,
  name,
  className,
  label,
  options = [],
  defaultValue,
  register,
  required,
  onChange,
  "aria-describedby": ariaDescribedby,
}) => (
  <fieldset className={clsx("RadioGroup", className)}>
    {label && <legend id={`${id || name}-label`}>{label}
      {required && <span aria-label="required">*</span>}
    </legend>}
    <div className="radio-options">
      {options.map((option) => (
        <div key={option.value} className="radio-option">
          <input
            type="radio"
            id={`${id || name}-${option.value}`}
            name={name || id}
            value={option.value}
            defaultChecked={defaultValue === option.value}
            {...(register ? register(name || id, { required }) : {})}
            onChange={onChange}
            aria-label={option.label}
            aria-describedby={ariaDescribedby}
            aria-required={required}
          />
          <label htmlFor={`${id || name}-${option.value}`}>
            {option.label}
          </label>
        </div>
      ))}
    </div>
  </fieldset>
);

RadioGroup.propTypes = {
  id: PropTypes.string,
  name: PropTypes.string,
  className: PropTypes.string,
  label: PropTypes.string,
  options: PropTypes.arrayOf(
    PropTypes.shape({
      value: PropTypes.string.isRequired,
      label: PropTypes.string.isRequired,
    })
  ),
  defaultValue: PropTypes.string,
  register: PropTypes.func,
  required: PropTypes.bool,
  onChange: PropTypes.func,
  "aria-describedby": PropTypes.string,
};

export default RadioGroup;

