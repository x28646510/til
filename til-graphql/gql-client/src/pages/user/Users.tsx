import React, {useState, useEffect} from 'react';
import {USERS} from '../../graphql/user';
import {useQuery} from '@apollo/client';
import {Typography, Button} from '@material-ui/core';

const Users = () => {
  const [users, setUsers] = useState([]);
  const {loading, error, data, refetch} = useQuery(USERS, {
    fetchPolicy: 'network-only',
  });

  useEffect(() => {
    if (data && data.users) {
      setUsers(data.users);
    }
  }, [data]);

  if (loading) {
    return <Typography color="textSecondary">{'Loading...'}</Typography>;
  }

  if (error) {
    return <Typography color="error">{`Error! ${error.message}`}</Typography>;
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

export default Users;
