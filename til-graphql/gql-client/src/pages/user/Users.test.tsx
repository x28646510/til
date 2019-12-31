import React from 'react';

import {MockedProvider, MockedResponse} from '@apollo/client/testing';
import {act, getByText, render, wait} from '@testing-library/react';

import {USERS} from '../../graphql/user';
import Users from './Users';

const prepare = (
  mocks: Array<MockedResponse>,
  children: JSX.Element
): HTMLElement => {
  const container = document.createElement('div');
  act(() => {
    render(
      <MockedProvider mocks={mocks} addTypename={false}>
        {children}
      </MockedProvider>,
      {container}
    );
  });
  return container;
};

test('renders without error', async () => {
  const mocks = [
    {
      request: {
        query: USERS,
      },
      result: {
        data: {
          users: [{id: '1', name: 'test1', email: 'test1@example.com'}],
        },
      },
    },
  ];

  const container = prepare(mocks, <Users />);
  await wait(
    () => {
      expect(getByText(container, '1')).toBeTruthy();
      expect(getByText(container, 'test1')).toBeTruthy();
      expect(getByText(container, 'test1@example.com')).toBeTruthy();
      expect(getByText(container, 'Refetch!')).toBeTruthy();
    },
    {timeout: 0}
  );
});

test('should render loading state initially', async () => {
  const container = prepare([], <Users />);
  expect(getByText(container, 'Loading...')).toBeDefined();
});

test('should show error UI', async () => {
  const mocks = [
    {
      request: {
        query: USERS,
      },
      error: new Error('error!'),
    },
  ];

  const container = prepare(mocks, <Users />);
  await wait(
    () => {
      expect(getByText(container, /Error!/)).toBeTruthy();
    },
    {timeout: 0}
  );
});
