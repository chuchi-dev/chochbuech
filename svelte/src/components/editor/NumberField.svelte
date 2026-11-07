<script lang="ts">
	import ChipSelect from './ChipSelect.svelte';

	// a labelled number box of the editor with a chip at its end: a unit
	// to choose (units) or a fixed one (suffix)
	let {
		id,
		label,
		value = $bindable(),
		step = 'any',
		min = 0,
		error = '',
		unit = $bindable(''),
		units = null,
		suffix = '',
	}: {
		id: string;
		label: string;
		value: number | null;
		step?: string;
		min?: number;
		error?: string;
		unit?: string;
		units?: [string, string][] | null;
		suffix?: string;
	} = $props();
</script>

<div class="field">
	<label for={id}>{label}</label>
	<div class="box" class:invalid={!!error}>
		<input
			{id}
			type="number"
			inputmode="decimal"
			{step}
			{min}
			bind:value
			aria-invalid={error ? true : undefined}
			aria-describedby={error ? `${id}-error` : undefined}
		/>
		{#if units}
			<ChipSelect
				bind:value={unit}
				options={units}
				label="Unit: {label}"
			/>
		{:else if suffix}
			<span class="chip">{suffix}</span>
		{/if}
	</div>
	{#if error}
		<p class="error" id="{id}-error">{error}</p>
	{/if}
</div>

<style lang="scss">
	label {
		display: block;
		margin-bottom: 1rem;
	}

	.box {
		display: flex;
		height: 3rem;
		align-items: center;
		gap: 1rem;
		padding-inline: 1rem;
		border: 1px solid var(--line);
		background: var(--bg-card);
		transition: border-color 0.3s ease;

		&:focus-within {
			border-color: var(--accent);
		}

		&.invalid {
			border-color: var(--error);
		}
	}

	input {
		width: 100%;
		min-width: 0;
		flex: 1;
		border: 0;
		background: transparent;
		color: inherit;
		font: inherit;
		outline: none;
		appearance: textfield;
		-moz-appearance: textfield;

		&::-webkit-inner-spin-button,
		&::-webkit-outer-spin-button {
			appearance: none;
			margin: 0;
		}
	}

	// like the chip of ChipSelect
	.chip {
		display: flex;
		height: 1.875rem;
		flex-shrink: 0;
		align-items: center;
		padding: 0 0.75rem;
		border: 1px solid var(--accent);
		border-radius: 999px;
		font-size: 0.875rem;
		line-height: 1;
	}

	.error {
		@include small;
		margin-top: 0.5rem;
		color: var(--error);
	}

	@include desktop {
		.box {
			height: 4rem;
			padding-inline: 2rem;
			font-size: 1.25rem;
		}

		.chip {
			height: 2.125rem;
			font-size: 1rem;
		}
	}
</style>
