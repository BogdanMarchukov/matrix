import {useState, FocusEvent, ChangeEvent, InputHTMLAttributes, forwardRef, Ref} from 'react';

interface DateInputProps extends InputHTMLAttributes < HTMLInputElement > {
  className? : string;
  placeholder?: string;
  onFocus?: (e: FocusEvent<HTMLInputElement>) => void;
  onBlur?: (e: FocusEvent<HTMLInputElement>) => void;
  onChange?: (e: ChangeEvent<HTMLInputElement>) => void;
}

export const DateInput = forwardRef<HTMLInputElement, DateInputProps>((props, ref) => {
  const {
    className,
    placeholder = 'ДД.ММ.ГГГГ',
    onFocus = () => {},
    onBlur = () => {},
    onChange = () => {},
    ...rest
  } = props;
  const [inputType, setInputType] = useState<'text' | 'date'>('text');
  const [isEmpty, setIsEmpty] = useState(true);

  const handleFocus = (e: FocusEvent<HTMLInputElement>) => {
    setInputType('date');
    onFocus(e)
  };

  const handleBlur = (e: FocusEvent<HTMLInputElement>) => {
    if (isEmpty) {
      setInputType('text');
    }
    onBlur(e)
  };

  const handleChange = (e: ChangeEvent<HTMLInputElement>) => {
    setIsEmpty(!e.target.value);
    onChange(e)
  };

  return (
    <input
      ref={ref}
      className={className}
      type={inputType}
      placeholder={placeholder}
      onFocus={handleFocus}
      onBlur={handleBlur}
      onChange={handleChange}
      {...rest}
    />
  );
})

DateInput.displayName = 'DateInput';
