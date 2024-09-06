import { useQuery } from "@apollo/client";
import { gql } from "../../__generated__";
import { NotifyByUserIdFilter } from "../../__generated__/graphql";

const NOTIFY = gql(/* GraphQl */ `
query GetAllNotify($data: NotifyByUserIdFilter!) {
  notify {
    findByUserId(data: $data) {
      notifyId
      title
      payload
    }
  }
}
 `);

export const useNotify = (notifyFilter: NotifyByUserIdFilter) => {
  const {client, error, loading, data } = useQuery(NOTIFY, {
    variables: {
      data: notifyFilter,
    },
  });


  return {
    error,
    loading,
    data,
  };
};
