import { useMutation } from '@apollo/client';
import { ChangeEvent, FocusEvent, forwardRef, InputHTMLAttributes, useEffect, useState } from 'react';
import { gql } from '../../../__generated__';
import { useUserStore } from '../../../common/store/userStore';

interface DateInputProps extends InputHTMLAttributes<HTMLInputElement> {
  className?: string;
  placeholder?: string;
  onFocus?: (e: FocusEvent<HTMLInputElement>) => void;
  onBlur?: (e: FocusEvent<HTMLInputElement>) => void;
  onChange?: (e: ChangeEvent<HTMLInputElement>) => void;
}

export const DateInput = forwardRef<HTMLInputElement, DateInputProps>((props, ref) => {
  const UPDATE_USER_INFO = gql(/* GraphQl */ `
      mutation UpdateOne($userInfoId: UUID!, $data: UserInfoUpdateInput!) {
        userInfo {
          updateOne(userInfoId: $userInfoId, data: $data) {
            dateOfBirth
            userInfoId
          }
        }
      }
`);
  const {
    className,
    placeholder = 'ДД.ММ.ГГГГ',
    onFocus = () => { },
    onBlur = (e) => {
      if (!date) {
        setDate(new Date(e.target.value))
      }
    },
    onChange = () => { },
    ...rest
  } = props;
  const [inputType, setInputType] = useState<'text' | 'date'>('text');
  const [isEmpty, setIsEmpty] = useState(true);
  const [date, setDate] = useState<Date | null>(null);
  const { userInfo: { dateOfBirth, userInfoId }, setUserInfo } = useUserStore((state) => state);
  const [updateUserInfo] = useMutation(UPDATE_USER_INFO);

  useEffect(() => {
    if (date) {
      console.log('date', date.toISOString())
      updateUserInfo({
        variables: {
          userInfoId,
          data: {
            dateOfBirth: date.toISOString().split('T')[0]
          }
        }
      }).then((result) => {
        if (!result.errors && result.data?.userInfo?.updateOne) {
          setUserInfo({ dateOfBirth: result.data.userInfo.updateOne.dateOfBirth, userInfoId })
        }
      })
    }
  }, [date, userInfoId, updateUserInfo, setUserInfo])

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
    !dateOfBirth && userInfoId ?
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
      : null
  );
})

DateInput.displayName = 'DateInput';
