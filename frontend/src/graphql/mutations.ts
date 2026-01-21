import { gql } from 'urql';

export const REQUEST_MAGIC_LINK = gql`
  mutation RequestMagicLink($input: RequestMagicLinkInput!) {
    requestMagicLink(input: $input)
  }
`;

export const LOGIN_WITH_MAGIC_LINK = gql`
  mutation LoginWithMagicLink($input: LoginWithMagicLinkInput!) {
    loginWithMagicLink(input: $input) {
      token
      user {
        id
        email
        role
      }
    }
  }
`;

export const CREATE_WORKSPACE = gql`
  mutation CreateWorkspace($input: CreateWorkspaceInput!) {
    createWorkspace(input: $input) {
      id
      name
      ownerId
    }
  }
`;

export const CREATE_INVITE = gql`
  mutation CreateInvite($input: CreateInviteInput!) {
    createInvite(input: $input)
  }
`;

export const ACCEPT_INVITE = gql`
  mutation AcceptInvite($token: String!) {
    acceptInvite(token: $token)
  }
`;
