import React from 'react';
import './App.css';
import {Typography, Link, Container, Box} from '@material-ui/core';
import ProTip from './components/ProTip';
import Users from './pages/user/Users';
import CreateUser from './pages/user/CreateUser';

const Copyright: React.FC = () => {
  return (
    <Typography variant="body2" color="textSecondary" align="center">
      {'Copyright © '}
      <Link color="inherit" href="https://material-ui.com/">
        Your Website
      </Link>{' '}
      {new Date().getUTCFullYear()}
      {'.'}
    </Typography>
  );
};

const App: React.FC = () => {
  return (
    <Container>
      <Box my={4}>
        <Typography variant="h4" component="h1" gutterBottom>
          Create React App example with TypeScript
        </Typography>
        <Users />
        <CreateUser />
        <ProTip />
        <Copyright />
      </Box>
    </Container>
  );
};

export default App;
