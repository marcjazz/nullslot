import { createClient, cacheExchange, fetchExchange, Client } from 'urql';

const client: Client = createClient({
  url: '/graphql',
  exchanges: [cacheExchange, fetchExchange],
  fetchOptions: () => {
    const token = localStorage.getItem('token');
    const workspaceId = localStorage.getItem('workspace_id');
    const headers: Record<string, string> = {
      authorization: token ? `Bearer ${token}` : '',
    };
    if (workspaceId) {
      headers['X-Workspace-ID'] = workspaceId;
    }
    return { headers };
  },
});

export default client;
