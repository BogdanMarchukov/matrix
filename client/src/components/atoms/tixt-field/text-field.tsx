import { FC, FocusEvent, InputHTMLAttributes, ChangeEvent, useState } from "react";
import styles from './text-field.module.css';

interface FloatingLabelInputProps {
  label: string;
  type: InputHTMLAttributes<HTMLInputElement>['type'];
  disable: boolean;
  value: string;
  setValue: (state: string) => void;
  min?: number;
  max?: number;
}

const TextField: FC<FloatingLabelInputProps> = ({ label, type, value, setValue, min = 0, max = 60, disable }) => {
  const [focused, setFocused] = useState(false);
  const handleChange = (e: ChangeEvent<HTMLInputElement>) => {
    if (type === 'number') {
      if (e.target.value === '') {
        return setValue('');
      }
      let input = parseInt(e.target.value);
      let targetValue = min;
      if (e.target.value.length > 1 && +e.target.value[0] === 0) {
        input = parseInt(e.target.value.slice(1));
      }
      if (input > min && input > max) {
        targetValue = max;
      }
      if (input >= min && input <= max) {
        targetValue = input;
      }
      setValue(targetValue.toString());
    } else {
      setValue(e.target.value);
    }
  }

  const isActive = focused || value !== '';

  const handleFocus = (e: FocusEvent<HTMLInputElement>) => setFocused(true);
  const handleBlur = (e: FocusEvent<HTMLInputElement>) => setFocused(false);

  return (
    <div className={styles.inputWrapper}>
      <label className={`${styles.floatingLabel} ${isActive ? styles.active : ""}`}>
        {label}
      </label>
      <input
        className={styles.floatingInput}
        type={type === 'date' ? isActive ? type : 'text' : type}
        value={value}
        disabled={disable}
        onChange={handleChange}
        onFocus={handleFocus}
        onBlur={handleBlur}
      />
    </div>
  );
};

export default TextField;

