import React from 'react';

import {MockedProvider, MockedResponse} from '@apollo/client/testing';
import {act, render} from '@testing-library/react';

import {CREATE_USER} from '../../graphql/user';
import CreateUser from './CreateUser';

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

test('should create  user', () => {
  const createdUser = {id: '2', name: 'test2', email: 'test2@example.com'};
  const mocks = [
    {
      request: {
        query: CREATE_USER,
        variables: {name: 'test2', email: 'test2@example.com'},
      },
      result: {
        data: {createdUser},
      },
    },
  ];

  prepare(mocks, <CreateUser />);
});
