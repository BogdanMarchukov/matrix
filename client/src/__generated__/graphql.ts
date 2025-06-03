/* eslint-disable */
import { TypedDocumentNode as DocumentNode } from '@graphql-typed-document-node/core';
export type Maybe<T> = T | null;
export type InputMaybe<T> = Maybe<T>;
export type Exact<T extends { [key: string]: unknown }> = { [K in keyof T]: T[K] };
export type MakeOptional<T, K extends keyof T> = Omit<T, K> & { [SubKey in K]?: Maybe<T[SubKey]> };
export type MakeMaybe<T, K extends keyof T> = Omit<T, K> & { [SubKey in K]: Maybe<T[SubKey]> };
export type MakeEmpty<T extends { [key: string]: unknown }, K extends keyof T> = { [_ in K]?: never };
export type Incremental<T> = T | { [P in keyof T]?: P extends ' $fragmentName' | '__typename' ? T[P] : never };
/** All built-in and custom scalars, mapped to their actual values */
export type Scalars = {
  ID: { input: string; output: string; }
  String: { input: string; output: string; }
  Boolean: { input: boolean; output: boolean; }
  Int: { input: number; output: number; }
  Float: { input: number; output: number; }
  /**
   * Implement the DateTime<Utc> scalar
   *
   * The input/output is a string in RFC3339 format.
   */
  DateTime: { input: any; output: any; }
  /**
   * ISO 8601 calendar date without timezone.
   * Format: %Y-%m-%d
   *
   * # Examples
   *
   * * `1994-11-13`
   * * `2000-02-24`
   */
  NaiveDate: { input: any; output: any; }
  /**
   * ISO 8601 combined date and time without timezone.
   *
   * # Examples
   *
   * * `2015-07-01T08:59:60.123`,
   */
  NaiveDateTime: { input: any; output: any; }
  /**
   * A UUID is a unique 128-bit number, stored as 16 octets. UUIDs are parsed as
   * Strings within GraphQL. UUIDs are used to assign unique identifiers to
   * entities without requiring a central allocating authority.
   *
   * # References
   *
   * * [Wikipedia: Universally Unique Identifier](http://en.wikipedia.org/wiki/Universally_unique_identifier)
   * * [RFC4122: A Universally Unique IDentifier (UUID) URN Namespace](http://tools.ietf.org/html/rfc4122)
   */
  UUID: { input: any; output: any; }
};

export type AuthMutation = {
  __typename?: 'AuthMutation';
  devLogin: LoginResult;
  login: LoginResult;
};


export type AuthMutationDevLoginArgs = {
  data: DevLoginInput;
};


export type AuthMutationLoginArgs = {
  data: LoginInput;
};

export type DevLoginInput = {
  userId: Scalars['UUID']['input'];
};

export enum GqlOrder {
  Asc = 'ASC',
  Desc = 'DESC'
}

export type LoginInput = {
  initData: Scalars['String']['input'];
};

export type LoginResult = {
  __typename?: 'LoginResult';
  jwt: Scalars['String']['output'];
  user: User;
};

export type Mutation = {
  __typename?: 'Mutation';
  auth: AuthMutation;
  newsletter: NewsletterMutation;
  notify: NotifyMutation;
  userInfo: UserInfoMutation;
};

export type NewsLetterCreateInput = {
  payload: Scalars['String']['input'];
  publishAt: Scalars['NaiveDateTime']['input'];
  title: Scalars['String']['input'];
};

export type Newsletter = {
  __typename?: 'Newsletter';
  createdAt: Scalars['DateTime']['output'];
  isPublished: Scalars['Boolean']['output'];
  newsletterId: Scalars['UUID']['output'];
  payload: Scalars['String']['output'];
  publishAt: Scalars['DateTime']['output'];
  title: Scalars['String']['output'];
};

export type NewsletterMutation = {
  __typename?: 'NewsletterMutation';
  createOne: Newsletter;
};


export type NewsletterMutationCreateOneArgs = {
  data: NewsLetterCreateInput;
};

export type Notify = {
  __typename?: 'Notify';
  createdAt: Scalars['DateTime']['output'];
  isRead: Scalars['Boolean']['output'];
  notifyId: Scalars['UUID']['output'];
  notifyType: NotifyType;
  payload: Scalars['String']['output'];
  title: Scalars['String']['output'];
  userId: Scalars['UUID']['output'];
};

export type NotifyByUserIdFilter = {
  isRead?: InputMaybe<Scalars['Boolean']['input']>;
  notifyType?: InputMaybe<NotifyType>;
  userId: Scalars['UUID']['input'];
};

export type NotifyMutation = {
  __typename?: 'NotifyMutation';
  updateOne: Notify;
};


export type NotifyMutationUpdateOneArgs = {
  data: NotifyUpdateData;
  notifyId: Scalars['UUID']['input'];
};

export enum NotifyOrderBy {
  CreatedAt = 'CREATED_AT'
}

export type NotifyQuery = {
  __typename?: 'NotifyQuery';
  findByPk: Notify;
  findByUserId: Array<Notify>;
};


export type NotifyQueryFindByPkArgs = {
  notifyId: Scalars['UUID']['input'];
};


export type NotifyQueryFindByUserIdArgs = {
  data: NotifyByUserIdFilter;
  sort?: InputMaybe<Sort>;
};

export type NotifySub = {
  __typename?: 'NotifySub';
  notifyId: Scalars['UUID']['output'];
};

export enum NotifyType {
  Daly = 'DALY'
}

export type NotifyUpdateData = {
  isRead: Scalars['Boolean']['input'];
};

export type Query = {
  __typename?: 'Query';
  notify: NotifyQuery;
  user: UserQuery;
  userInfo: UserInfoQuery;
};

export type Sort = {
  limit?: InputMaybe<Scalars['Int']['input']>;
  order?: InputMaybe<GqlOrder>;
  orderBy?: InputMaybe<NotifyOrderBy>;
};

export type Subscription = {
  __typename?: 'Subscription';
  notifyDelay: NotifySub;
};

export type User = {
  __typename?: 'User';
  firstName?: Maybe<Scalars['String']['output']>;
  isPremium?: Maybe<Scalars['Boolean']['output']>;
  languageCode?: Maybe<Scalars['String']['output']>;
  lastName?: Maybe<Scalars['String']['output']>;
  photoUrl?: Maybe<Scalars['String']['output']>;
  role: UserRoleType;
  telegramId: Scalars['Int']['output'];
  userId: Scalars['UUID']['output'];
  userInfo: UserInfo;
  username?: Maybe<Scalars['String']['output']>;
};

export type UserInfo = {
  __typename?: 'UserInfo';
  city?: Maybe<Scalars['String']['output']>;
  dateOfBirth?: Maybe<Scalars['NaiveDate']['output']>;
  hourOfBirth?: Maybe<Scalars['Int']['output']>;
  minOfBirth?: Maybe<Scalars['Int']['output']>;
  userId: Scalars['UUID']['output'];
  userInfoId: Scalars['UUID']['output'];
};

export type UserInfoMutation = {
  __typename?: 'UserInfoMutation';
  updateOne: UserInfo;
};


export type UserInfoMutationUpdateOneArgs = {
  data: UserInfoUpdateInput;
  userInfoId: Scalars['UUID']['input'];
};

export type UserInfoQuery = {
  __typename?: 'UserInfoQuery';
  fundByUserId: UserInfo;
};


export type UserInfoQueryFundByUserIdArgs = {
  userId: Scalars['UUID']['input'];
};

export type UserInfoUpdateInput = {
  city?: InputMaybe<Scalars['String']['input']>;
  dateOfBirth?: InputMaybe<Scalars['NaiveDate']['input']>;
  hourOfBirth?: InputMaybe<Scalars['Int']['input']>;
  minOfBirth?: InputMaybe<Scalars['Int']['input']>;
};

export type UserQuery = {
  __typename?: 'UserQuery';
  findByPk: User;
};


export type UserQueryFindByPkArgs = {
  userId: Scalars['UUID']['input'];
};

export enum UserRoleType {
  Admin = 'ADMIN',
  Member = 'MEMBER',
  Owner = 'OWNER'
}

export type UserLoginMutationVariables = Exact<{
  data: LoginInput;
}>;


export type UserLoginMutation = { __typename?: 'Mutation', auth: { __typename?: 'AuthMutation', login: { __typename?: 'LoginResult', jwt: string, user: { __typename?: 'User', userId: any, firstName?: string | null, lastName?: string | null, photoUrl?: string | null, userInfo: { __typename?: 'UserInfo', dateOfBirth?: any | null, userInfoId: any } } } } };

export type UserDevLoginMutationVariables = Exact<{
  data: DevLoginInput;
}>;


export type UserDevLoginMutation = { __typename?: 'Mutation', auth: { __typename?: 'AuthMutation', devLogin: { __typename?: 'LoginResult', jwt: string, user: { __typename?: 'User', userId: any, firstName?: string | null, lastName?: string | null, photoUrl?: string | null, userInfo: { __typename?: 'UserInfo', dateOfBirth?: any | null, userInfoId: any } } } } };

export type GetAllNotifyQueryVariables = Exact<{
  data: NotifyByUserIdFilter;
  sort?: InputMaybe<Sort>;
}>;


export type GetAllNotifyQuery = { __typename?: 'Query', notify: { __typename?: 'NotifyQuery', findByUserId: Array<{ __typename?: 'Notify', notifyId: any, title: string, payload: string, isRead: boolean }> } };

export type SetNotifyIsReadMutationVariables = Exact<{
  data: NotifyUpdateData;
  notifyId: Scalars['UUID']['input'];
}>;


export type SetNotifyIsReadMutation = { __typename?: 'Mutation', notify: { __typename?: 'NotifyMutation', updateOne: { __typename?: 'Notify', notifyId: any } } };

export type NotifyDelaySubscriptionVariables = Exact<{ [key: string]: never; }>;


export type NotifyDelaySubscription = { __typename?: 'Subscription', notifyDelay: { __typename?: 'NotifySub', notifyId: any } };


export const UserLoginDocument = {"kind":"Document","definitions":[{"kind":"OperationDefinition","operation":"mutation","name":{"kind":"Name","value":"userLogin"},"variableDefinitions":[{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"data"}},"type":{"kind":"NonNullType","type":{"kind":"NamedType","name":{"kind":"Name","value":"LoginInput"}}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"auth"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"login"},"arguments":[{"kind":"Argument","name":{"kind":"Name","value":"data"},"value":{"kind":"Variable","name":{"kind":"Name","value":"data"}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"jwt"}},{"kind":"Field","name":{"kind":"Name","value":"user"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"userId"}},{"kind":"Field","name":{"kind":"Name","value":"firstName"}},{"kind":"Field","name":{"kind":"Name","value":"lastName"}},{"kind":"Field","name":{"kind":"Name","value":"photoUrl"}},{"kind":"Field","name":{"kind":"Name","value":"userInfo"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"dateOfBirth"}},{"kind":"Field","name":{"kind":"Name","value":"userInfoId"}}]}}]}}]}}]}}]}}]} as unknown as DocumentNode<UserLoginMutation, UserLoginMutationVariables>;
export const UserDevLoginDocument = {"kind":"Document","definitions":[{"kind":"OperationDefinition","operation":"mutation","name":{"kind":"Name","value":"userDevLogin"},"variableDefinitions":[{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"data"}},"type":{"kind":"NonNullType","type":{"kind":"NamedType","name":{"kind":"Name","value":"DevLoginInput"}}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"auth"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"devLogin"},"arguments":[{"kind":"Argument","name":{"kind":"Name","value":"data"},"value":{"kind":"Variable","name":{"kind":"Name","value":"data"}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"jwt"}},{"kind":"Field","name":{"kind":"Name","value":"user"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"userId"}},{"kind":"Field","name":{"kind":"Name","value":"firstName"}},{"kind":"Field","name":{"kind":"Name","value":"lastName"}},{"kind":"Field","name":{"kind":"Name","value":"photoUrl"}},{"kind":"Field","name":{"kind":"Name","value":"userInfo"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"dateOfBirth"}},{"kind":"Field","name":{"kind":"Name","value":"userInfoId"}}]}}]}}]}}]}}]}}]} as unknown as DocumentNode<UserDevLoginMutation, UserDevLoginMutationVariables>;
export const GetAllNotifyDocument = {"kind":"Document","definitions":[{"kind":"OperationDefinition","operation":"query","name":{"kind":"Name","value":"GetAllNotify"},"variableDefinitions":[{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"data"}},"type":{"kind":"NonNullType","type":{"kind":"NamedType","name":{"kind":"Name","value":"NotifyByUserIdFilter"}}}},{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"sort"}},"type":{"kind":"NamedType","name":{"kind":"Name","value":"Sort"}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"notify"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"findByUserId"},"arguments":[{"kind":"Argument","name":{"kind":"Name","value":"data"},"value":{"kind":"Variable","name":{"kind":"Name","value":"data"}}},{"kind":"Argument","name":{"kind":"Name","value":"sort"},"value":{"kind":"Variable","name":{"kind":"Name","value":"sort"}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"notifyId"}},{"kind":"Field","name":{"kind":"Name","value":"title"}},{"kind":"Field","name":{"kind":"Name","value":"payload"}},{"kind":"Field","name":{"kind":"Name","value":"isRead"}}]}}]}}]}}]} as unknown as DocumentNode<GetAllNotifyQuery, GetAllNotifyQueryVariables>;
export const SetNotifyIsReadDocument = {"kind":"Document","definitions":[{"kind":"OperationDefinition","operation":"mutation","name":{"kind":"Name","value":"SetNotifyIsRead"},"variableDefinitions":[{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"data"}},"type":{"kind":"NonNullType","type":{"kind":"NamedType","name":{"kind":"Name","value":"NotifyUpdateData"}}}},{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"notifyId"}},"type":{"kind":"NonNullType","type":{"kind":"NamedType","name":{"kind":"Name","value":"UUID"}}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"notify"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"updateOne"},"arguments":[{"kind":"Argument","name":{"kind":"Name","value":"data"},"value":{"kind":"Variable","name":{"kind":"Name","value":"data"}}},{"kind":"Argument","name":{"kind":"Name","value":"notifyId"},"value":{"kind":"Variable","name":{"kind":"Name","value":"notifyId"}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"notifyId"}}]}}]}}]}}]} as unknown as DocumentNode<SetNotifyIsReadMutation, SetNotifyIsReadMutationVariables>;
export const NotifyDelayDocument = {"kind":"Document","definitions":[{"kind":"OperationDefinition","operation":"subscription","name":{"kind":"Name","value":"NotifyDelay"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"notifyDelay"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"notifyId"}}]}}]}}]} as unknown as DocumentNode<NotifyDelaySubscription, NotifyDelaySubscriptionVariables>;