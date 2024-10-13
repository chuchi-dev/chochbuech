<script lang="ts">
	import { Writable } from 'chuchi/stores';
	import './app.scss';
	import Footer from './layout/Footer.svelte';
	import Header from './layout/Header.svelte';
	import Toasts from './layout/Toasts.svelte';

	const { page }: { page: Writable<any> } = $props();
</script>

<Toasts />

<div id="app">
	<Header />

	<main>
		<svelte:component this={$page.component} {...$page.props} />
	</main>

	<Footer />
</div>

<style lang="scss">
	#app {
		display: grid;
		grid-template-rows: auto 1fr auto;
		min-height: 100vh;
	}
</style>
