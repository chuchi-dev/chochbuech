import { newApi } from './utils';

const api = newApi('/waitlist');

export async function addWaitlist(email: string) {
	await api.request('POST', '/add', { email });
}
