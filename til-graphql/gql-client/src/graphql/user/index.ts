import {gql} from '@apollo/client';

export const USERS = gql`
  query {
    users {
      id
      name
      email
    }
  }
`;

export const CREATE_USER = gql`
  mutation CreateUser($name: String!, $email: String!) {
    createUser(user: {name: $name, email: $email}) {
      id
    }
  }
`;
