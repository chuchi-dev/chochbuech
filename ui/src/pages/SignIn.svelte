<script>
	import { toast } from '@/layout/Toasts.svelte';

	let email = $state('');
	let password = $state('');

	function onsubmit(e) {
		e.preventDefault();
		console.log('submit');

		toast({
			message: 'Signing in...',
			status: 'success',
		});
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
