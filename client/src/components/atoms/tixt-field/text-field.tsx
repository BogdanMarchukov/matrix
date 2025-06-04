import React, { useState, FC, ChangeEvent, FocusEvent } from "react";
import styles from './text-field.module.css';

interface FloatingLabelInputProps {
  label: string;
  value: string;
  onChange: (e: ChangeEvent<HTMLInputElement>) => void;
}

const TextField: FC<FloatingLabelInputProps> = ({ label, value, onChange }) => {
  const [focused, setFocused] = useState(false);

  const isActive = focused || value.length > 0;

  const handleFocus = (e: FocusEvent<HTMLInputElement>) => setFocused(true);
  const handleBlur = (e: FocusEvent<HTMLInputElement>) => setFocused(false);

  return (
    <div className={styles.inputWrapper}>
      <label className={`${styles.floatingLabel} ${isActive ? styles.active : ""}`}>
        {label}
      </label>
      <input
        className={styles.floatingInput}
        value={value}
        onChange={onChange}
        onFocus={handleFocus}
        onBlur={handleBlur}
      />
    </div>
  );
};

export default TextField;

