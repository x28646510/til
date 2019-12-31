import React from 'react';
import {fireEvent, render} from '@testing-library/react';

import UserForm from './UserForm';

const onSubmit = (name: string, email: string) => {};

test('renders without error', () => {
  const {getByText, getByLabelText} = render(<UserForm onSubmit={onSubmit} />);
  expect(getByLabelText(/Name/)).toBeTruthy();
  expect(getByLabelText(/Email/)).toBeTruthy();
  expect(getByText('Create')).toBeTruthy();
});

test('should input name value is changed', async () => {
  const {getByDisplayValue, getByLabelText} = render(
    <UserForm onSubmit={onSubmit} />
  );
  const nameInput = getByLabelText(/Name/);
  fireEvent.change(nameInput, {
    target: {id: 'user-name', value: 'test-user-1'},
  });
  expect(getByDisplayValue('test-user-1')).toBeTruthy();
});

test('should input email value is changed', async () => {
  const {getByDisplayValue, getByLabelText} = render(
    <UserForm onSubmit={onSubmit} />
  );
  const emailInput = getByLabelText(/Email/);
  fireEvent.change(emailInput, {
    target: {id: 'user-email', value: 'test1@example.com'},
  });
  expect(getByDisplayValue('test1@example.com')).toBeTruthy();
});

test('should clear input values after submit', async () => {
  const {
    getByDisplayValue,
    getByLabelText,
    getByText,
    queryAllByDisplayValue,
  } = render(<UserForm onSubmit={onSubmit} />);
  const nameInput = getByLabelText(/Name/);
  fireEvent.change(nameInput, {
    target: {id: 'user-name', value: 'test-user-1'},
  });
  expect(getByDisplayValue('test-user-1')).toBeTruthy();
  const emailInput = getByLabelText(/Email/);
  fireEvent.change(emailInput, {
    target: {id: 'user-email', value: 'test1@example.com'},
  });
  expect(getByDisplayValue('test1@example.com')).toBeTruthy();

  const submitButton = getByText('Create');
  expect(submitButton).toBeTruthy();
  fireEvent.submit(submitButton);

  expect(queryAllByDisplayValue('').length).toBe(2);
});
