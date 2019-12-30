import {ApolloClient, InMemoryCache, HttpLink} from '@apollo/client';
import {config} from '../config';

export const cache = new InMemoryCache();

export const client = new ApolloClient({
  cache: cache,
  link: new HttpLink({
    uri: config.graphqlEndpoint,
  }),
});

const data = {
  networkStatus: {
    __typename: 'NetworkStatus',
    isConnected: false,
  },
};

cache.writeData({data});

client.onResetStore(async () => cache.writeData({data}));
