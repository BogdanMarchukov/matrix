import {FocusEvent, ChangeEvent, InputHTMLAttributes, forwardRef} from 'react';

interface TextInputProps extends InputHTMLAttributes < HTMLInputElement > {
  className? : string;
  placeholder?: string;
  onFocus?: (e: FocusEvent<HTMLInputElement>) => void;
  onBlur?: (e: FocusEvent<HTMLInputElement>) => void;
  onChange?: (e: ChangeEvent<HTMLInputElement>) => void;
}

export const TextInput = forwardRef<HTMLInputElement, TextInputProps>((props, ref) => {
  const {
    className,
    placeholder = '',
    onChange = () => {},
    ...rest
  } = props;

  const handleChange = (e: ChangeEvent<HTMLInputElement>) => {
    onChange(e)
  };

  return (
    <input
      ref={ref}
      className={className}
      type="text"
      placeholder={placeholder}
      onChange={handleChange}
      {...rest}
    />
  );
})

TextInput.displayName = 'textInput';
