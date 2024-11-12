import {gql} from "../../../__generated__";
import {useMutation, useQuery} from "@apollo/client";

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

interface UseProfileProps {
  userId?: string
}


export const useProfile = ({userId}: UseProfileProps) => {
  const {data} = useQuery(GET_USER_INFO, {
    variables: {
      userId,
    },
    skip: !userId
  });

  const [updateUserInfo] = useMutation(UPDATE_USER_INFO)

  return {
    data,
    updateUserInfo
  }
}