<script lang="ts">
	import Button from '@/components/Button.svelte';
	import { errorMessage, uploadImage } from '@/lib/members';
	import { getToasts } from '@/lib/toasts.svelte';
	import type { FormImage } from './form';

	// the photo of a recipe: a preview and a button that uploads another
	// one at once (POST /members/assets), the field keeps its id
	let {
		image = $bindable(),
		error = '',
	}: { image: FormImage | null; error?: string } = $props();

	const toasts = getToasts();

	let input: HTMLInputElement;
	let uploading = $state(false);

	async function upload() {
		const file = input.files?.[0];
		input.value = '';
		if (!file) return;
		uploading = true;
		try {
			const up = await uploadImage(file);
			image = {
				id: up.id,
				url: up.url,
				width: up.width,
				height: up.height,
			};
		} catch (e) {
			toasts.error(errorMessage(e));
		} finally {
			uploading = false;
		}
	}
</script>

<div class="field">
	<span class="label" id="photo-label">Photo:</span>
	<div class="box" class:invalid={!!error}>
		<div class="preview">
			{#if image?.url}
				<img
					src={image.url}
					width={image.width}
					height={image.height}
					alt="Your dish"
				/>
			{/if}
		</div>
		<div class="side">
			<p class="hint">
				{uploading
					? 'Uploading …'
					: image
						? 'The photo of your dish, shown on the card and at the top of the recipe.'
						: 'Add a photo of your dish: JPG, PNG or WebP, at most 8 MB.'}
			</p>
			<input
				bind:this={input}
				id="photo"
				class="file"
				type="file"
				accept="image/*"
				onchange={upload}
				aria-labelledby="photo-label"
				aria-describedby={error ? 'photo-error' : undefined}
			/>
			<Button
				look={image ? 'secondary' : 'primary'}
				size="sm"
				disabled={uploading}
				onclick={() => input.click()}
			>
				{image ? 'Replace Photo' : 'Add Photo'}
			</Button>
		</div>
	</div>
	{#if error}
		<p class="error" id="photo-error">{error}</p>
	{/if}
</div>

<style lang="scss">
	.label {
		display: block;
		margin-bottom: 1rem;
	}

	.box {
		display: flex;
		gap: 1.25rem;
		padding: 1rem;
		border: 1px solid var(--line);
		background: var(--bg-card);
		transition: border-color 0.3s ease;

		&.invalid {
			border-color: var(--error);
		}
	}

	.preview {
		flex: none;
		width: 6rem;
		aspect-ratio: 3 / 4;
		overflow: hidden;
		background: var(--line);

		img {
			display: block;
			width: 100%;
			height: 100%;
			object-fit: cover;
		}
	}

	.side {
		display: flex;
		flex-direction: column;
		align-items: flex-start;
		justify-content: space-between;
		gap: 1rem;
	}

	.hint {
		color: var(--accent-soft);
	}

	.file {
		position: absolute;
		width: 1px;
		height: 1px;
		opacity: 0;
		pointer-events: none;
	}

	.error {
		@include small;
		margin-top: 0.5rem;
		color: var(--error);
	}

	@include desktop {
		.box {
			gap: 2rem;
			padding: 1.5rem 2rem;
		}

		.preview {
			width: 8rem;
		}

		.hint {
			@include lead;
		}
	}
</style>
