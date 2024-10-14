<script module>
	export async function loadProps(props, lp) {
		if (lp.session.isLoggedIn()) {
			lp.setRedirect('/me');
		}
	}
</script>

<script>
	import { login } from '@/api/users';
	import { toast } from '@/layout/Toasts.svelte';
	import { getSession } from '@/lib/Session';
	import { getRouter } from '@/main';
	import { timeout } from 'chuchi-utils';

	const session = getSession();
	const router = getRouter();
	const req = router.currentRequest;

	let email = $state('');
	let password = $state('');

	async function onsubmit(e) {
		e.preventDefault();
		console.log('submit');

		const ref = toast({
			message: 'Signing in...',
			status: 'success',
		});

		try {
			const auth = await login(email, password);
			session.setAuthed(auth);

			// nows lets either redirect to url or to /me
			const url = $req?.search.get('url') ?? '/me';
			router.open(url);
		} catch (e) {
			console.error(e);
			ref.update({ message: e.message, status: 'error' });
			return;
		}

		ref.update({ message: 'Signed in!', status: 'success' });

		setTimeout(() => {
			ref.remove();
		}, 3000);
	}
</script>

<form {onsubmit}>
	<input
		type="email"
		name="email"
		placeholder="E-Mail"
		bind:value={email}
		required
	/>
	<input
		type="password"
		name="password"
		placeholder="Password"
		bind:value={password}
		required
	/>

	<button type="submit" class="btn">Sign In</button>
</form>
