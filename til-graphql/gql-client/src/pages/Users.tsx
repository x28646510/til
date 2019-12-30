import React, {useState, useEffect} from 'react';
import {ALL_USERS, CREATE_USER} from '../graphql/user';
import {useQuery, useMutation} from '@apollo/client';
import {Typography, Button} from '@material-ui/core';
import UserForm from '../components/form/UserForm';

export const AllUsers = () => {
  const [users, setUsers] = useState([]);
  const {loading, error, data, refetch} = useQuery(ALL_USERS, {
    fetchPolicy: "network-only"
  });

  useEffect(() => {
    if (data && data.users) {
      setUsers(data.users);
    }
  }, [data]);

  if (loading) {
    return <Typography color="textSecondary">Loading...</Typography>;
  }

  if (error) {
    console.error(error);
    return <Typography color="error">`Error! ${error.message}`</Typography>;
  }

  return (
    <>
      {users.map(
        ({id, name, email}: {id: string; name: string; email: string}) => (
          <div key={id}>
            <Typography color="textSecondary">{id}</Typography>
            <Typography color="textSecondary">{name}</Typography>
            <Typography color="textSecondary">{email}</Typography>
          </div>
        )
      )}
      <Button variant="contained" onClick={() => refetch()}>
        Refetch!
      </Button>
    </>
  );
};

export const CreateUser: React.FC = () => {
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
