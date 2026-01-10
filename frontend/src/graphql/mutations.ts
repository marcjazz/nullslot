export const REQUEST_MAGIC_LINK = `
  mutation RequestMagicLink($input: RequestMagicLinkInput!) {
    requestMagicLink(input: $input)
  }
`;

export const LOGIN_WITH_MAGIC_LINK = `
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
