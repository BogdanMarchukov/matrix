import {ChangeEvent, SelectHTMLAttributes, forwardRef, Ref} from 'react';

interface SelectProps<T> extends SelectHTMLAttributes <HTMLSelectElement> {
  className?: string;
  options?: T[];
  optionKey?: string;
  placeholder?: string;
  onChange?: (e: ChangeEvent<HTMLSelectElement>) => void;
}

export const Select = forwardRef<HTMLSelectElement, SelectProps<unknown>>(
  <T, >(props: SelectProps<T>, ref: Ref<HTMLSelectElement>) => {
    const {
      className,
      options = [],
      optionKey,
      placeholder = '',
      onChange = () => {
      },
      ...rest
    } = props;

    const handleChange = (e: ChangeEvent<HTMLSelectElement>) => {
      onChange(e)
    };

    return (
      <select
        ref={ref}
        className={className}
        onChange={handleChange}
        {...rest}
      >
        {placeholder && <option value="">{placeholder}</option>}
        {options.map((value, index) => (
          <option key={index}>{String(optionKey ? value?.[optionKey as keyof T] : value)}</option>
        ))}
      </select>
    );
  })

Select.displayName = 'select';
