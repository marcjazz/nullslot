import { gql } from 'urql';

export const ME_QUERY = gql`
  query Me {
    me {
      id
      email
      role
    }
  }
`;

export const MY_WORKSPACES = gql`
  query MyWorkspaces {
    myWorkspaces {
      id
      name
      ownerId
    }
  }
`;
