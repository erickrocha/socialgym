import React from "react";
import clsx from "clsx";
import PropTypes from "prop-types";
import "./Select.scss";

const Select = ({
  id,
  name,
  className,
  label,
  placeholder,
  options = [],
  defaultValue,
  register,
  required,
  ariaLabel,
  ariaDescribedby,
  onChange
}) => (
  <div className={clsx("Select", className)}>
    {label && (
      <label htmlFor={id || name} id={`${id || name}-label`}>
        {label}
        {required && <span aria-label="required">*</span>}
      </label>
    )}
    <select
      id={id || name}
      name={name || id}
      defaultValue={defaultValue || ""}
      {...(register ? register(name || id, { required }) : {})}
      onChange={onChange}
      aria-label={ariaLabel || label}
      aria-describedby={ariaDescribedby}
      aria-required={required}
    >
      {placeholder && <option value="">{placeholder}</option>}
      {options.map((option) => (
        <option key={option.value} value={option.value}>
          {option.label}
        </option>
      ))}
    </select>
  </div>
);

Select.propTypes = {
  id: PropTypes.string,
  name: PropTypes.string,
  className: PropTypes.string,
  label: PropTypes.string,
  placeholder: PropTypes.string,
  options: PropTypes.arrayOf(
    PropTypes.shape({
      value: PropTypes.string.isRequired,
      label: PropTypes.string.isRequired,
    })
  ),
  defaultValue: PropTypes.string,
  register: PropTypes.func,
  required: PropTypes.bool,
  "aria-label": PropTypes.string,
  "aria-describedby": PropTypes.string,
  onChange: PropTypes.func,
};

export default Select;

