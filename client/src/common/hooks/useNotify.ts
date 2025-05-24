import { useMutation, useQuery } from "@apollo/client";
import { gql } from "../../__generated__";
import {
  GqlOrder,
  NotifyByUserIdFilter,
  NotifyOrderBy
} from "../../__generated__/graphql";
import { useCallback } from 'react';

const NOTIFY = gql(/* GraphQl */ `
  query GetAllNotify($data: NotifyByUserIdFilter!, $sort: Sort) {
    notify {
      findByUserId(data: $data, sort: $sort) {
        notifyId
        title
        payload
        isRead
      }
    }
  }
`);

const UPDATE_NOTIFY = gql(/* GraphQl */ `
  mutation SetNotifyIsRead($data: NotifyUpdateData!, $notifyId: UUID!) {
    notify {
      updateOne(data: $data, notifyId: $notifyId) {
        notifyId
      }
    }
  }
`);

export const useNotify = (notifyFilter: NotifyByUserIdFilter) => {
  const { error, loading, data, refetch } = useQuery(NOTIFY, {
    variables: {
      data: notifyFilter,
      sort: {
        limit: 1,
        order: GqlOrder.Desc,
        orderBy: NotifyOrderBy.CreatedAt
      }
    },
    skip: !notifyFilter.userId
  });

  const [updateNotify] = useMutation(UPDATE_NOTIFY);

  const setNotifyIsRead = useCallback((notifyId: string) => {
    if (notifyId) {
      updateNotify({
        variables: {
          notifyId,
          data: {
            isRead: true,
          }
        }
      }).then((res) => {
        if (res?.data?.notify?.updateOne?.notifyId) {
          refetch()
        }
      })
    }
  }, [refetch, updateNotify])

  return {
    error,
    loading,
    data,
    setNotifyIsRead,
  };
};
