import React from 'react';

import {useMutation} from '@apollo/client';
import {CREATE_USER} from '../../graphql/user';
import UserForm from '../../components/form/UserForm';

const CreateUser: React.FC = () => {
  const [createUser] = useMutation(CREATE_USER, {
    onError: error => {
      console.error(error);
    },
  });

  const onSubmit = (name: string, email: string) => {
    createUser({variables: {name: name, email: email}});
  };

  return <UserForm onSubmit={onSubmit} />;
};

export default CreateUser;
