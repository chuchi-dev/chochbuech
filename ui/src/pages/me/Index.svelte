<script module>
	export const requiresLogin = true;
</script>

<script>
	import { getSession } from '@/lib/Session';

	const session = getSession();
</script>

<div class="home">
	<h1>
		Hallo <i>{$session.shortUser?.name ?? ''}</i>
	</h1>
</div>

<style lang="scss">
	.home {
		display: flex;
		height: 100%;
		justify-content: center;
		align-items: center;
	}

	h1 {
		font-family: var(--font-secondary);
		font-weight: 200;
		line-height: 1.25;
		font-size: 3rem;
	}
</style>
