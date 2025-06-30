/* eslint-disable */
import * as types from './graphql';
import { TypedDocumentNode as DocumentNode } from '@graphql-typed-document-node/core';

/**
 * Map of all GraphQL operations in the project.
 *
 * This map has several performance disadvantages:
 * 1. It is not tree-shakeable, so it will include all operations in the project.
 * 2. It is not minifiable, so the string of a GraphQL query will be multiple times inside the bundle.
 * 3. It does not support dead code elimination, so it will add unused operations.
 *
 * Therefore it is highly recommended to use the babel or swc plugin for production.
 */
const documents = {
    "\n  mutation userLogin ($data: LoginInput!) {\n    auth {\n      login(data: $data) {\n        jwt\n        user {\n          userId\n          firstName\n          lastName\n          photoUrl\n            userInfo {\n              dateOfBirth\n              userInfoId\n            }\n        }\n      }\n    }\n  }\n": types.UserLoginDocument,
    "\n  mutation userDevLogin ($data: DevLoginInput!) {\n    auth {\n      devLogin(data: $data) {\n        jwt\n        user {\n          userId\n          firstName\n          lastName\n          photoUrl\n            userInfo {\n              dateOfBirth\n              userInfoId\n            }\n        }\n      }\n    }\n  }\n": types.UserDevLoginDocument,
    "\n  query GetAllNotify($data: NotifyByUserIdFilter!, $sort: Sort) {\n    notify {\n      findByUserId(data: $data, sort: $sort) {\n        notifyId\n        title\n        payload\n        isRead\n      }\n    }\n  }\n": types.GetAllNotifyDocument,
    "\n  mutation SetNotifyIsRead($data: NotifyUpdateData!, $notifyId: UUID!) {\n    notify {\n      updateOne(data: $data, notifyId: $notifyId) {\n        notifyId\n      }\n    }\n  }\n": types.SetNotifyIsReadDocument,
    "\n      mutation UpdateOne($userInfoId: UUID!, $data: UserInfoUpdateInput!) {\n        userInfo {\n          updateOne(userInfoId: $userInfoId, data: $data) {\n            dateOfBirth\n            userInfoId\n          }\n        }\n      }\n": types.UpdateOneDocument,
    "\n  subscription NotifyDelay {\n  notifyDelay {\n    notifyId\n   }\n  }\n": types.NotifyDelayDocument,
    "\n  query FindTariffPlanByOffer($offerId: UUID!) {\n    offer {\n      findById(offerId: $offerId) {\n        tariffs {\n          price\n          tariffPlanId\n          title\n          description\n          expiryDays\n        }\n        offerId\n      }\n    }\n  }\n": types.FindTariffPlanByOfferDocument,
    "\n    query OfferFindById($offerId: UUID!) {\n      offer {\n        findById(offerId: $offerId) {\n          img\n          isActive\n          offerId\n          tariffIds\n        }\n      }\n    }\n": types.OfferFindByIdDocument,
    "\n    query UserFindById($userId: UUID!) {\n      user {\n        findByPk(userId: $userId) {\n          userId\n          userTariffPlan {\n            tariffPlanId\n          }\n        }\n      }\n    }\n": types.UserFindByIdDocument,
    "\n  query FindMany {\n    offer {\n      findMany {\n        img\n        offerId\n        title\n        tariffIds\n        description\n      }\n    }\n  }\n": types.FindManyDocument,
    "query GetUserInfoByUserId($userId: UUID!) {\n    userInfo {\n      fundByUserId(userId: $userId) {\n        city\n        dateOfBirth\n        hourOfBirth\n        minOfBirth\n      }\n    }\n  }\n": types.GetUserInfoByUserIdDocument,
    "mutation UserInfoUpdateOne($userInfoId: UUID!, $data: UserInfoUpdateInput!) {\n    userInfo {\n      updateOne(userInfoId: $userInfoId, data: $data) {\n        city\n        dateOfBirth\n        hourOfBirth\n        minOfBirth\n    }\n  }\n}": types.UserInfoUpdateOneDocument,
};

/**
 * The gql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 *
 *
 * @example
 * ```ts
 * const query = gql(`query GetUser($id: ID!) { user(id: $id) { name } }`);
 * ```
 *
 * The query argument is unknown!
 * Please regenerate the types.
 */
export function gql(source: string): unknown;

/**
 * The gql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function gql(source: "\n  mutation userLogin ($data: LoginInput!) {\n    auth {\n      login(data: $data) {\n        jwt\n        user {\n          userId\n          firstName\n          lastName\n          photoUrl\n            userInfo {\n              dateOfBirth\n              userInfoId\n            }\n        }\n      }\n    }\n  }\n"): (typeof documents)["\n  mutation userLogin ($data: LoginInput!) {\n    auth {\n      login(data: $data) {\n        jwt\n        user {\n          userId\n          firstName\n          lastName\n          photoUrl\n            userInfo {\n              dateOfBirth\n              userInfoId\n            }\n        }\n      }\n    }\n  }\n"];
/**
 * The gql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function gql(source: "\n  mutation userDevLogin ($data: DevLoginInput!) {\n    auth {\n      devLogin(data: $data) {\n        jwt\n        user {\n          userId\n          firstName\n          lastName\n          photoUrl\n            userInfo {\n              dateOfBirth\n              userInfoId\n            }\n        }\n      }\n    }\n  }\n"): (typeof documents)["\n  mutation userDevLogin ($data: DevLoginInput!) {\n    auth {\n      devLogin(data: $data) {\n        jwt\n        user {\n          userId\n          firstName\n          lastName\n          photoUrl\n            userInfo {\n              dateOfBirth\n              userInfoId\n            }\n        }\n      }\n    }\n  }\n"];
/**
 * The gql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function gql(source: "\n  query GetAllNotify($data: NotifyByUserIdFilter!, $sort: Sort) {\n    notify {\n      findByUserId(data: $data, sort: $sort) {\n        notifyId\n        title\n        payload\n        isRead\n      }\n    }\n  }\n"): (typeof documents)["\n  query GetAllNotify($data: NotifyByUserIdFilter!, $sort: Sort) {\n    notify {\n      findByUserId(data: $data, sort: $sort) {\n        notifyId\n        title\n        payload\n        isRead\n      }\n    }\n  }\n"];
/**
 * The gql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function gql(source: "\n  mutation SetNotifyIsRead($data: NotifyUpdateData!, $notifyId: UUID!) {\n    notify {\n      updateOne(data: $data, notifyId: $notifyId) {\n        notifyId\n      }\n    }\n  }\n"): (typeof documents)["\n  mutation SetNotifyIsRead($data: NotifyUpdateData!, $notifyId: UUID!) {\n    notify {\n      updateOne(data: $data, notifyId: $notifyId) {\n        notifyId\n      }\n    }\n  }\n"];
/**
 * The gql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function gql(source: "\n      mutation UpdateOne($userInfoId: UUID!, $data: UserInfoUpdateInput!) {\n        userInfo {\n          updateOne(userInfoId: $userInfoId, data: $data) {\n            dateOfBirth\n            userInfoId\n          }\n        }\n      }\n"): (typeof documents)["\n      mutation UpdateOne($userInfoId: UUID!, $data: UserInfoUpdateInput!) {\n        userInfo {\n          updateOne(userInfoId: $userInfoId, data: $data) {\n            dateOfBirth\n            userInfoId\n          }\n        }\n      }\n"];
/**
 * The gql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function gql(source: "\n  subscription NotifyDelay {\n  notifyDelay {\n    notifyId\n   }\n  }\n"): (typeof documents)["\n  subscription NotifyDelay {\n  notifyDelay {\n    notifyId\n   }\n  }\n"];
/**
 * The gql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function gql(source: "\n  query FindTariffPlanByOffer($offerId: UUID!) {\n    offer {\n      findById(offerId: $offerId) {\n        tariffs {\n          price\n          tariffPlanId\n          title\n          description\n          expiryDays\n        }\n        offerId\n      }\n    }\n  }\n"): (typeof documents)["\n  query FindTariffPlanByOffer($offerId: UUID!) {\n    offer {\n      findById(offerId: $offerId) {\n        tariffs {\n          price\n          tariffPlanId\n          title\n          description\n          expiryDays\n        }\n        offerId\n      }\n    }\n  }\n"];
/**
 * The gql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function gql(source: "\n    query OfferFindById($offerId: UUID!) {\n      offer {\n        findById(offerId: $offerId) {\n          img\n          isActive\n          offerId\n          tariffIds\n        }\n      }\n    }\n"): (typeof documents)["\n    query OfferFindById($offerId: UUID!) {\n      offer {\n        findById(offerId: $offerId) {\n          img\n          isActive\n          offerId\n          tariffIds\n        }\n      }\n    }\n"];
/**
 * The gql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function gql(source: "\n    query UserFindById($userId: UUID!) {\n      user {\n        findByPk(userId: $userId) {\n          userId\n          userTariffPlan {\n            tariffPlanId\n          }\n        }\n      }\n    }\n"): (typeof documents)["\n    query UserFindById($userId: UUID!) {\n      user {\n        findByPk(userId: $userId) {\n          userId\n          userTariffPlan {\n            tariffPlanId\n          }\n        }\n      }\n    }\n"];
/**
 * The gql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function gql(source: "\n  query FindMany {\n    offer {\n      findMany {\n        img\n        offerId\n        title\n        tariffIds\n        description\n      }\n    }\n  }\n"): (typeof documents)["\n  query FindMany {\n    offer {\n      findMany {\n        img\n        offerId\n        title\n        tariffIds\n        description\n      }\n    }\n  }\n"];
/**
 * The gql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function gql(source: "query GetUserInfoByUserId($userId: UUID!) {\n    userInfo {\n      fundByUserId(userId: $userId) {\n        city\n        dateOfBirth\n        hourOfBirth\n        minOfBirth\n      }\n    }\n  }\n"): (typeof documents)["query GetUserInfoByUserId($userId: UUID!) {\n    userInfo {\n      fundByUserId(userId: $userId) {\n        city\n        dateOfBirth\n        hourOfBirth\n        minOfBirth\n      }\n    }\n  }\n"];
/**
 * The gql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function gql(source: "mutation UserInfoUpdateOne($userInfoId: UUID!, $data: UserInfoUpdateInput!) {\n    userInfo {\n      updateOne(userInfoId: $userInfoId, data: $data) {\n        city\n        dateOfBirth\n        hourOfBirth\n        minOfBirth\n    }\n  }\n}"): (typeof documents)["mutation UserInfoUpdateOne($userInfoId: UUID!, $data: UserInfoUpdateInput!) {\n    userInfo {\n      updateOne(userInfoId: $userInfoId, data: $data) {\n        city\n        dateOfBirth\n        hourOfBirth\n        minOfBirth\n    }\n  }\n}"];

export function gql(source: string) {
  return (documents as any)[source] ?? {};
}

export type DocumentType<TDocumentNode extends DocumentNode<any, any>> = TDocumentNode extends DocumentNode<  infer TType,  any>  ? TType  : never;