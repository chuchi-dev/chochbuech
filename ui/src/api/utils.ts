import { Api } from 'chuchi/api';

/**
 * Create a new API instance
 * @param uri The base URI of the API
 * needs to start with /
 */
export async function newApi(uri: string) {
	if (!uri.startsWith('/')) throw new Error('uri needs to start with /');
	if (import.meta.env.DEV) {
		return new Api('http://localhost:4986/api' + uri);
	}

	return new Api('/api' + uri);
}
