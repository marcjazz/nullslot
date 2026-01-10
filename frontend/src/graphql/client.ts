import { createClient, cacheExchange, fetchExchange, Client } from 'urql';

const client: Client = createClient({
  url: '/graphql',
  exchanges: [cacheExchange, fetchExchange],
  fetchOptions: () => {
    const token = localStorage.getItem('token');
    return {
      headers: { authorization: token ? `Bearer ${token}` : '' },
    };
  },
});

export default client;
