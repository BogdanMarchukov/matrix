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
  news: NewsMutation;
  newsletter: NewsletterMutation;
  notify: NotifyMutation;
  offer: OfferMutation;
  tariffPlan: TariffPlanMutation;
  userInfo: UserInfoMutation;
  userNews: UserNewsMutation;
  userTariffPlan: UserTariffPlanMutation;
};

export type News = {
  __typename?: 'News';
  createdAt: Scalars['DateTime']['output'];
  img?: Maybe<Scalars['String']['output']>;
  isPublish: Scalars['Boolean']['output'];
  newsId: Scalars['UUID']['output'];
  payload: Scalars['String']['output'];
  publishAt: Scalars['DateTime']['output'];
  title: Scalars['String']['output'];
  updatedAt: Scalars['DateTime']['output'];
};

export type NewsCreateInput = {
  isPublish: Scalars['Boolean']['input'];
  payload: Scalars['String']['input'];
  publishAt: Scalars['NaiveDateTime']['input'];
  title: Scalars['String']['input'];
};

export type NewsLetterCreateInput = {
  payload: Scalars['String']['input'];
  publishAt: Scalars['NaiveDateTime']['input'];
  title: Scalars['String']['input'];
};

export type NewsMutation = {
  __typename?: 'NewsMutation';
  createOne: News;
  updateOne: News;
};


export type NewsMutationCreateOneArgs = {
  data: NewsCreateInput;
};


export type NewsMutationUpdateOneArgs = {
  data: NewsUpdateInput;
  newsId: Scalars['UUID']['input'];
};

export type NewsUpdateInput = {
  isPublish?: InputMaybe<Scalars['Boolean']['input']>;
  payload?: InputMaybe<Scalars['String']['input']>;
  title?: InputMaybe<Scalars['String']['input']>;
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

export type Offer = {
  __typename?: 'Offer';
  createdAt: Scalars['DateTime']['output'];
  description?: Maybe<Scalars['String']['output']>;
  img?: Maybe<Scalars['String']['output']>;
  isActive: Scalars['Boolean']['output'];
  offerId: Scalars['UUID']['output'];
  tariffIds: Array<Scalars['UUID']['output']>;
  tariffs: Array<TariffPlan>;
  title: Scalars['String']['output'];
};

export type OfferCreateData = {
  description?: InputMaybe<Scalars['String']['input']>;
  img?: InputMaybe<Scalars['String']['input']>;
  isActive: Scalars['Boolean']['input'];
  tariffIds: Array<Scalars['UUID']['input']>;
  title: Scalars['String']['input'];
};

export type OfferMutation = {
  __typename?: 'OfferMutation';
  createOne: Offer;
};


export type OfferMutationCreateOneArgs = {
  data: OfferCreateData;
};

export type OfferQuery = {
  __typename?: 'OfferQuery';
  findById: Offer;
  findMany: Array<Offer>;
};


export type OfferQueryFindByIdArgs = {
  offerId: Scalars['UUID']['input'];
};

export type Query = {
  __typename?: 'Query';
  notify: NotifyQuery;
  offer: OfferQuery;
  user: UserQuery;
  userInfo: UserInfoQuery;
  userNews: UserNewsQuery;
};

export type Sort = {
  limit?: InputMaybe<Scalars['Int']['input']>;
  order?: InputMaybe<GqlOrder>;
  orderBy?: InputMaybe<NotifyOrderBy>;
};

export type Subscription = {
  __typename?: 'Subscription';
  notifyDelay: NotifySub;
  userNews: UserNewsSub;
};

export type TariffPlan = {
  __typename?: 'TariffPlan';
  description?: Maybe<Scalars['String']['output']>;
  expiryDays: Scalars['Int']['output'];
  price: Scalars['Float']['output'];
  tariffPlanId: Scalars['UUID']['output'];
  title: Scalars['String']['output'];
};

export type TariffPlanCreateData = {
  description?: InputMaybe<Scalars['String']['input']>;
  expiryDays: Scalars['Int']['input'];
  price: Scalars['Float']['input'];
  title: Scalars['String']['input'];
};

export type TariffPlanMutation = {
  __typename?: 'TariffPlanMutation';
  createOne: TariffPlan;
};


export type TariffPlanMutationCreateOneArgs = {
  data: TariffPlanCreateData;
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
  userTariffPlan: Array<UserTariffPlan>;
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

export type UserNews = {
  __typename?: 'UserNews';
  createdAt: Scalars['DateTime']['output'];
  img?: Maybe<Scalars['String']['output']>;
  newsId: Scalars['UUID']['output'];
  payload: Scalars['String']['output'];
  readingAt?: Maybe<Scalars['DateTime']['output']>;
  readingCount: Scalars['Int']['output'];
  title: Scalars['String']['output'];
  userId: Scalars['UUID']['output'];
  userNewsId: Scalars['UUID']['output'];
};

export type UserNewsMutation = {
  __typename?: 'UserNewsMutation';
  readIncrement: UserNews;
};


export type UserNewsMutationReadIncrementArgs = {
  userNewsId: Scalars['UUID']['input'];
};

export type UserNewsQuery = {
  __typename?: 'UserNewsQuery';
  findByPk: UserNews;
  findByUserId: Array<UserNews>;
};


export type UserNewsQueryFindByPkArgs = {
  userNewsId: Scalars['UUID']['input'];
};


export type UserNewsQueryFindByUserIdArgs = {
  userId: Scalars['UUID']['input'];
};

export type UserNewsSub = {
  __typename?: 'UserNewsSub';
  userNewsId: Scalars['UUID']['output'];
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

export type UserTariffPlan = {
  __typename?: 'UserTariffPlan';
  createdAt: Scalars['DateTime']['output'];
  expiresAt: Scalars['DateTime']['output'];
  tariffPlanId: Scalars['UUID']['output'];
  tariffPlanPaymentId?: Maybe<Scalars['UUID']['output']>;
  userId: Scalars['UUID']['output'];
  userTariffPlanId: Scalars['UUID']['output'];
};

export type UserTariffPlanMutation = {
  __typename?: 'UserTariffPlanMutation';
  buyTariffPlan: UserTariffPlan;
};


export type UserTariffPlanMutationBuyTariffPlanArgs = {
  tariffPlanId: Scalars['UUID']['input'];
};

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

export type UpdateOneMutationVariables = Exact<{
  userInfoId: Scalars['UUID']['input'];
  data: UserInfoUpdateInput;
}>;


export type UpdateOneMutation = { __typename?: 'Mutation', userInfo: { __typename?: 'UserInfoMutation', updateOne: { __typename?: 'UserInfo', dateOfBirth?: any | null, userInfoId: any } } };

export type NotifyDelaySubscriptionVariables = Exact<{ [key: string]: never; }>;


export type NotifyDelaySubscription = { __typename?: 'Subscription', notifyDelay: { __typename?: 'NotifySub', notifyId: any } };

export type FindTariffPlanByOfferQueryVariables = Exact<{
  offerId: Scalars['UUID']['input'];
}>;


export type FindTariffPlanByOfferQuery = { __typename?: 'Query', offer: { __typename?: 'OfferQuery', findById: { __typename?: 'Offer', offerId: any, tariffs: Array<{ __typename?: 'TariffPlan', price: number, tariffPlanId: any, title: string, description?: string | null, expiryDays: number }> } } };

export type BuyTariffPlanMutationVariables = Exact<{
  tariffPlanId: Scalars['UUID']['input'];
}>;


export type BuyTariffPlanMutation = { __typename?: 'Mutation', userTariffPlan: { __typename?: 'UserTariffPlanMutation', buyTariffPlan: { __typename?: 'UserTariffPlan', tariffPlanId: any, userId: any } } };

export type OfferFindByIdQueryVariables = Exact<{
  offerId: Scalars['UUID']['input'];
}>;


export type OfferFindByIdQuery = { __typename?: 'Query', offer: { __typename?: 'OfferQuery', findById: { __typename?: 'Offer', img?: string | null, isActive: boolean, offerId: any, tariffIds: Array<any> } } };

export type UserFindByIdQueryVariables = Exact<{
  userId: Scalars['UUID']['input'];
}>;


export type UserFindByIdQuery = { __typename?: 'Query', user: { __typename?: 'UserQuery', findByPk: { __typename?: 'User', userId: any, userTariffPlan: Array<{ __typename?: 'UserTariffPlan', tariffPlanId: any }> } } };

export type FindByUserIdQueryVariables = Exact<{
  userId: Scalars['UUID']['input'];
}>;


export type FindByUserIdQuery = { __typename?: 'Query', userNews: { __typename?: 'UserNewsQuery', findByUserId: Array<{ __typename?: 'UserNews', img?: string | null, newsId: any, payload: string, title: string, userNewsId: any }> } };

export type FindManyQueryVariables = Exact<{ [key: string]: never; }>;


export type FindManyQuery = { __typename?: 'Query', offer: { __typename?: 'OfferQuery', findMany: Array<{ __typename?: 'Offer', img?: string | null, offerId: any, title: string, tariffIds: Array<any>, description?: string | null }> } };

export type GetUserInfoByUserIdQueryVariables = Exact<{
  userId: Scalars['UUID']['input'];
}>;


export type GetUserInfoByUserIdQuery = { __typename?: 'Query', userInfo: { __typename?: 'UserInfoQuery', fundByUserId: { __typename?: 'UserInfo', city?: string | null, dateOfBirth?: any | null, hourOfBirth?: number | null, minOfBirth?: number | null } } };

export type UserInfoUpdateOneMutationVariables = Exact<{
  userInfoId: Scalars['UUID']['input'];
  data: UserInfoUpdateInput;
}>;


export type UserInfoUpdateOneMutation = { __typename?: 'Mutation', userInfo: { __typename?: 'UserInfoMutation', updateOne: { __typename?: 'UserInfo', city?: string | null, dateOfBirth?: any | null, hourOfBirth?: number | null, minOfBirth?: number | null } } };


export const UserLoginDocument = {"kind":"Document","definitions":[{"kind":"OperationDefinition","operation":"mutation","name":{"kind":"Name","value":"userLogin"},"variableDefinitions":[{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"data"}},"type":{"kind":"NonNullType","type":{"kind":"NamedType","name":{"kind":"Name","value":"LoginInput"}}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"auth"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"login"},"arguments":[{"kind":"Argument","name":{"kind":"Name","value":"data"},"value":{"kind":"Variable","name":{"kind":"Name","value":"data"}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"jwt"}},{"kind":"Field","name":{"kind":"Name","value":"user"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"userId"}},{"kind":"Field","name":{"kind":"Name","value":"firstName"}},{"kind":"Field","name":{"kind":"Name","value":"lastName"}},{"kind":"Field","name":{"kind":"Name","value":"photoUrl"}},{"kind":"Field","name":{"kind":"Name","value":"userInfo"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"dateOfBirth"}},{"kind":"Field","name":{"kind":"Name","value":"userInfoId"}}]}}]}}]}}]}}]}}]} as unknown as DocumentNode<UserLoginMutation, UserLoginMutationVariables>;
export const UserDevLoginDocument = {"kind":"Document","definitions":[{"kind":"OperationDefinition","operation":"mutation","name":{"kind":"Name","value":"userDevLogin"},"variableDefinitions":[{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"data"}},"type":{"kind":"NonNullType","type":{"kind":"NamedType","name":{"kind":"Name","value":"DevLoginInput"}}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"auth"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"devLogin"},"arguments":[{"kind":"Argument","name":{"kind":"Name","value":"data"},"value":{"kind":"Variable","name":{"kind":"Name","value":"data"}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"jwt"}},{"kind":"Field","name":{"kind":"Name","value":"user"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"userId"}},{"kind":"Field","name":{"kind":"Name","value":"firstName"}},{"kind":"Field","name":{"kind":"Name","value":"lastName"}},{"kind":"Field","name":{"kind":"Name","value":"photoUrl"}},{"kind":"Field","name":{"kind":"Name","value":"userInfo"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"dateOfBirth"}},{"kind":"Field","name":{"kind":"Name","value":"userInfoId"}}]}}]}}]}}]}}]}}]} as unknown as DocumentNode<UserDevLoginMutation, UserDevLoginMutationVariables>;
export const GetAllNotifyDocument = {"kind":"Document","definitions":[{"kind":"OperationDefinition","operation":"query","name":{"kind":"Name","value":"GetAllNotify"},"variableDefinitions":[{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"data"}},"type":{"kind":"NonNullType","type":{"kind":"NamedType","name":{"kind":"Name","value":"NotifyByUserIdFilter"}}}},{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"sort"}},"type":{"kind":"NamedType","name":{"kind":"Name","value":"Sort"}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"notify"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"findByUserId"},"arguments":[{"kind":"Argument","name":{"kind":"Name","value":"data"},"value":{"kind":"Variable","name":{"kind":"Name","value":"data"}}},{"kind":"Argument","name":{"kind":"Name","value":"sort"},"value":{"kind":"Variable","name":{"kind":"Name","value":"sort"}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"notifyId"}},{"kind":"Field","name":{"kind":"Name","value":"title"}},{"kind":"Field","name":{"kind":"Name","value":"payload"}},{"kind":"Field","name":{"kind":"Name","value":"isRead"}}]}}]}}]}}]} as unknown as DocumentNode<GetAllNotifyQuery, GetAllNotifyQueryVariables>;
export const SetNotifyIsReadDocument = {"kind":"Document","definitions":[{"kind":"OperationDefinition","operation":"mutation","name":{"kind":"Name","value":"SetNotifyIsRead"},"variableDefinitions":[{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"data"}},"type":{"kind":"NonNullType","type":{"kind":"NamedType","name":{"kind":"Name","value":"NotifyUpdateData"}}}},{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"notifyId"}},"type":{"kind":"NonNullType","type":{"kind":"NamedType","name":{"kind":"Name","value":"UUID"}}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"notify"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"updateOne"},"arguments":[{"kind":"Argument","name":{"kind":"Name","value":"data"},"value":{"kind":"Variable","name":{"kind":"Name","value":"data"}}},{"kind":"Argument","name":{"kind":"Name","value":"notifyId"},"value":{"kind":"Variable","name":{"kind":"Name","value":"notifyId"}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"notifyId"}}]}}]}}]}}]} as unknown as DocumentNode<SetNotifyIsReadMutation, SetNotifyIsReadMutationVariables>;
export const UpdateOneDocument = {"kind":"Document","definitions":[{"kind":"OperationDefinition","operation":"mutation","name":{"kind":"Name","value":"UpdateOne"},"variableDefinitions":[{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"userInfoId"}},"type":{"kind":"NonNullType","type":{"kind":"NamedType","name":{"kind":"Name","value":"UUID"}}}},{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"data"}},"type":{"kind":"NonNullType","type":{"kind":"NamedType","name":{"kind":"Name","value":"UserInfoUpdateInput"}}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"userInfo"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"updateOne"},"arguments":[{"kind":"Argument","name":{"kind":"Name","value":"userInfoId"},"value":{"kind":"Variable","name":{"kind":"Name","value":"userInfoId"}}},{"kind":"Argument","name":{"kind":"Name","value":"data"},"value":{"kind":"Variable","name":{"kind":"Name","value":"data"}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"dateOfBirth"}},{"kind":"Field","name":{"kind":"Name","value":"userInfoId"}}]}}]}}]}}]} as unknown as DocumentNode<UpdateOneMutation, UpdateOneMutationVariables>;
export const NotifyDelayDocument = {"kind":"Document","definitions":[{"kind":"OperationDefinition","operation":"subscription","name":{"kind":"Name","value":"NotifyDelay"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"notifyDelay"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"notifyId"}}]}}]}}]} as unknown as DocumentNode<NotifyDelaySubscription, NotifyDelaySubscriptionVariables>;
export const FindTariffPlanByOfferDocument = {"kind":"Document","definitions":[{"kind":"OperationDefinition","operation":"query","name":{"kind":"Name","value":"FindTariffPlanByOffer"},"variableDefinitions":[{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"offerId"}},"type":{"kind":"NonNullType","type":{"kind":"NamedType","name":{"kind":"Name","value":"UUID"}}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"offer"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"findById"},"arguments":[{"kind":"Argument","name":{"kind":"Name","value":"offerId"},"value":{"kind":"Variable","name":{"kind":"Name","value":"offerId"}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"tariffs"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"price"}},{"kind":"Field","name":{"kind":"Name","value":"tariffPlanId"}},{"kind":"Field","name":{"kind":"Name","value":"title"}},{"kind":"Field","name":{"kind":"Name","value":"description"}},{"kind":"Field","name":{"kind":"Name","value":"expiryDays"}}]}},{"kind":"Field","name":{"kind":"Name","value":"offerId"}}]}}]}}]}}]} as unknown as DocumentNode<FindTariffPlanByOfferQuery, FindTariffPlanByOfferQueryVariables>;
export const BuyTariffPlanDocument = {"kind":"Document","definitions":[{"kind":"OperationDefinition","operation":"mutation","name":{"kind":"Name","value":"BuyTariffPlan"},"variableDefinitions":[{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"tariffPlanId"}},"type":{"kind":"NonNullType","type":{"kind":"NamedType","name":{"kind":"Name","value":"UUID"}}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"userTariffPlan"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"buyTariffPlan"},"arguments":[{"kind":"Argument","name":{"kind":"Name","value":"tariffPlanId"},"value":{"kind":"Variable","name":{"kind":"Name","value":"tariffPlanId"}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"tariffPlanId"}},{"kind":"Field","name":{"kind":"Name","value":"userId"}}]}}]}}]}}]} as unknown as DocumentNode<BuyTariffPlanMutation, BuyTariffPlanMutationVariables>;
export const OfferFindByIdDocument = {"kind":"Document","definitions":[{"kind":"OperationDefinition","operation":"query","name":{"kind":"Name","value":"OfferFindById"},"variableDefinitions":[{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"offerId"}},"type":{"kind":"NonNullType","type":{"kind":"NamedType","name":{"kind":"Name","value":"UUID"}}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"offer"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"findById"},"arguments":[{"kind":"Argument","name":{"kind":"Name","value":"offerId"},"value":{"kind":"Variable","name":{"kind":"Name","value":"offerId"}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"img"}},{"kind":"Field","name":{"kind":"Name","value":"isActive"}},{"kind":"Field","name":{"kind":"Name","value":"offerId"}},{"kind":"Field","name":{"kind":"Name","value":"tariffIds"}}]}}]}}]}}]} as unknown as DocumentNode<OfferFindByIdQuery, OfferFindByIdQueryVariables>;
export const UserFindByIdDocument = {"kind":"Document","definitions":[{"kind":"OperationDefinition","operation":"query","name":{"kind":"Name","value":"UserFindById"},"variableDefinitions":[{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"userId"}},"type":{"kind":"NonNullType","type":{"kind":"NamedType","name":{"kind":"Name","value":"UUID"}}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"user"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"findByPk"},"arguments":[{"kind":"Argument","name":{"kind":"Name","value":"userId"},"value":{"kind":"Variable","name":{"kind":"Name","value":"userId"}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"userId"}},{"kind":"Field","name":{"kind":"Name","value":"userTariffPlan"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"tariffPlanId"}}]}}]}}]}}]}}]} as unknown as DocumentNode<UserFindByIdQuery, UserFindByIdQueryVariables>;
export const FindByUserIdDocument = {"kind":"Document","definitions":[{"kind":"OperationDefinition","operation":"query","name":{"kind":"Name","value":"FindByUserId"},"variableDefinitions":[{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"userId"}},"type":{"kind":"NonNullType","type":{"kind":"NamedType","name":{"kind":"Name","value":"UUID"}}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"userNews"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"findByUserId"},"arguments":[{"kind":"Argument","name":{"kind":"Name","value":"userId"},"value":{"kind":"Variable","name":{"kind":"Name","value":"userId"}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"img"}},{"kind":"Field","name":{"kind":"Name","value":"newsId"}},{"kind":"Field","name":{"kind":"Name","value":"payload"}},{"kind":"Field","name":{"kind":"Name","value":"title"}},{"kind":"Field","name":{"kind":"Name","value":"userNewsId"}}]}}]}}]}}]} as unknown as DocumentNode<FindByUserIdQuery, FindByUserIdQueryVariables>;
export const FindManyDocument = {"kind":"Document","definitions":[{"kind":"OperationDefinition","operation":"query","name":{"kind":"Name","value":"FindMany"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"offer"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"findMany"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"img"}},{"kind":"Field","name":{"kind":"Name","value":"offerId"}},{"kind":"Field","name":{"kind":"Name","value":"title"}},{"kind":"Field","name":{"kind":"Name","value":"tariffIds"}},{"kind":"Field","name":{"kind":"Name","value":"description"}}]}}]}}]}}]} as unknown as DocumentNode<FindManyQuery, FindManyQueryVariables>;
export const GetUserInfoByUserIdDocument = {"kind":"Document","definitions":[{"kind":"OperationDefinition","operation":"query","name":{"kind":"Name","value":"GetUserInfoByUserId"},"variableDefinitions":[{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"userId"}},"type":{"kind":"NonNullType","type":{"kind":"NamedType","name":{"kind":"Name","value":"UUID"}}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"userInfo"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"fundByUserId"},"arguments":[{"kind":"Argument","name":{"kind":"Name","value":"userId"},"value":{"kind":"Variable","name":{"kind":"Name","value":"userId"}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"city"}},{"kind":"Field","name":{"kind":"Name","value":"dateOfBirth"}},{"kind":"Field","name":{"kind":"Name","value":"hourOfBirth"}},{"kind":"Field","name":{"kind":"Name","value":"minOfBirth"}}]}}]}}]}}]} as unknown as DocumentNode<GetUserInfoByUserIdQuery, GetUserInfoByUserIdQueryVariables>;
export const UserInfoUpdateOneDocument = {"kind":"Document","definitions":[{"kind":"OperationDefinition","operation":"mutation","name":{"kind":"Name","value":"UserInfoUpdateOne"},"variableDefinitions":[{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"userInfoId"}},"type":{"kind":"NonNullType","type":{"kind":"NamedType","name":{"kind":"Name","value":"UUID"}}}},{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"data"}},"type":{"kind":"NonNullType","type":{"kind":"NamedType","name":{"kind":"Name","value":"UserInfoUpdateInput"}}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"userInfo"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"updateOne"},"arguments":[{"kind":"Argument","name":{"kind":"Name","value":"userInfoId"},"value":{"kind":"Variable","name":{"kind":"Name","value":"userInfoId"}}},{"kind":"Argument","name":{"kind":"Name","value":"data"},"value":{"kind":"Variable","name":{"kind":"Name","value":"data"}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"city"}},{"kind":"Field","name":{"kind":"Name","value":"dateOfBirth"}},{"kind":"Field","name":{"kind":"Name","value":"hourOfBirth"}},{"kind":"Field","name":{"kind":"Name","value":"minOfBirth"}}]}}]}}]}}]} as unknown as DocumentNode<UserInfoUpdateOneMutation, UserInfoUpdateOneMutationVariables>;