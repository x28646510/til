import React, {useState} from 'react';
import {makeStyles, Theme} from '@material-ui/core/styles';
import TextField from '@material-ui/core/TextField';
import Button from '@material-ui/core/Button';

const userStyles = makeStyles((theme: Theme) => ({
  root: {
    '& > *': {
      margin: theme.spacing(1),
      width: 200,
    },
  },
}));

interface Props {
  onSubmit: (name: string, email: string) => void;
}

const UserForm = (props: Props) => {
  const [formValue, setFormValue] = useState({name: '', email: ''});

  const handleChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    const id = event.target.id;
    const value = event.target.value;
    if (id === 'user-name') {
      setFormValue({name: value, email: formValue.email});
    } else if (id === 'user-email') {
      setFormValue({name: formValue.name, email: value});
    }
  };

  const handleSubmit = (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    props.onSubmit(formValue.name, formValue.email);
    setFormValue({name: '', email: ''});
  };

  const classes = userStyles();
  return (
    <form
      className={classes.root}
      noValidate
      autoComplete="off"
      onSubmit={handleSubmit}
    >
      <TextField
        id="user-name"
        label="Name"
        value={formValue.name}
        onChange={handleChange}
        variant="outlined"
        required
      />
      <TextField
        id="user-email"
        label="Email"
        value={formValue.email}
        onChange={handleChange}
        variant="outlined"
        required
      />
      <Button variant="contained" type="submit">
        Create
      </Button>
    </form>
  );
};

export default UserForm;
