<script>
	function onSubmit(e) {
		e.preventDefault();
	}
</script>

<form on:submit={onSubmit} class="input-form get-notified">
	<input type="email" name="email" placeholder="Deine E-Mail-Adresse" />
	<button>→</button>
</form>

<style lang="scss">
	form {
		display: block;
		width: 100%;
		max-width: 32rem;
		margin: 0 auto;
	}
</style>
