import {gql} from "../../../__generated__";
import {useMutation, useQuery} from "@apollo/client";
import {useUserStore} from "../../../common/store/userStore";

const GET_USER_INFO = gql(/* GraphQl */ `
  query GetUserInfo($userId: UUID!) {
    userInfo {
      fundByUserId(userId: $userId) {
        userInfoId
        city
        dateOfBirth
        hourOfBirth
        minOfBirth
        userId
      }
    }
  }
`);

const UPDATE_USER_INFO = gql(/* GraphQl */ `
  mutation UpdateUserInfo($data: UserInfoUpdateInput!, $userInfoId: UUID!) {
    userInfo {
      updateOne(data: $data, userInfoId: $userInfoId) {
        userId
      }
    }
  }
`);

export const useProfile = () => {
  const {userInfo} = useUserStore((state) => state);
  const {userId, avatarUrl, firstName} = userInfo;
  const {data, loading} = useQuery(GET_USER_INFO, {
    variables: {
      userId,
    },
    skip: !userId
  });

  const [updateUserInfo] = useMutation(UPDATE_USER_INFO)

  return {
    loading,
    data: data?.userInfo?.fundByUserId || {},
    updateUserInfo,
    avatarUrl: avatarUrl || '',
    firstName: firstName || ''
  }
}