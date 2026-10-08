/**
 * The props that connect an input to react-hook-form while keeping a caller's own `onChange`.
 * Spreading `register(...)` and then passing `onChange={onChange}` replaces the form's handler, so the
 * form never sees the change (a required select then fails validation with nothing on screen).
 */
export const registeredProps = (register, name, required, onChange) => {
    const registered = register ? register(name, { required }) : {};
    return {
        ...registered,
        onChange: (event) => {
            registered.onChange?.(event);
            onChange?.(event);
        },
    };
};
