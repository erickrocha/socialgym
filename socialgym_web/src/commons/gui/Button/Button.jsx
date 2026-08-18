import './Button.scss';
import PropTypes from "prop-types";
import clsx from "clsx";

const Button = ({className, children, onClick, type = 'button', disabled = false, variant = ''}) => {


    const chooseVariant = (variant) => {
        switch (variant) {
            case 'primary':
                return 'primary-button';
            case 'secondary':
                return 'secondary-button';
            default:
                return "";
        }
    }

    return (
        <button
            onClick={onClick}
            disabled={disabled}
            type={type}
            className={clsx("Button", className, chooseVariant(variant))}
        >
            {children}
        </button>
    );
}

Button.propTypes = {
    className: PropTypes.string,
    onClick: PropTypes.func,
    type: PropTypes.string,
    disabled: PropTypes.bool
};

export default Button;