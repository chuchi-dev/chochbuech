import { newApi } from './utils';

const api = newApi('/users');

export class ShortSession {
	token!: string;
	timeout!: Date;

	constructor(data: any) {
		Object.assign(this, data);
		this.timeout = new Date(data.timeout);
	}
}

export class ShortUser {
	id!: string;
	oauth!: any;
	name!: string;
	email!: string;
	createdOn!: Date;

	constructor(data: any) {
		Object.assign(this, data);
		this.createdOn = new Date(data.createdOn);
	}
}

export class Authenticated {
	session!: ShortSession;
	user!: ShortUser;

	constructor(data: any) {
		this.session = new ShortSession(data.session);
		this.user = new ShortUser(data.user);
	}
}

export async function login(email: string, password: string) {
	const d = await api.request('POST', '/login', { email, password });

	return new Authenticated(d);
}

export async function tokenAuth(token: string): Promise<Authenticated> {
	const d = await api.request('POST', '/tokenauth', null, {
		'session-token': token,
	});

	return new Authenticated(d);
}
