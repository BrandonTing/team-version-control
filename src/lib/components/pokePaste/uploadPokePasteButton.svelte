<script lang="ts">
	import { upload } from '@/bindings';
	import { InvokeTauriError } from '@/errors';
	import { open } from '@tauri-apps/api/shell';
	import { Effect } from 'effect';
	import type { ComponentProps } from 'svelte';
	import Button from '../ui/button/button.svelte';

	const {
		title,
		variant,
		context,
		className
	}: {
		title: string;
		context: string;
		variant: ComponentProps<Button>['variant'];
		className?: ComponentProps<Button>['class'];
	} = $props();

	async function uploadPaste() {
		await Effect.tryPromise({
			try: async () => {
				const url = await upload(title, context.replaceAll('\n', '\r\n'));
				await open(url);
			},
			catch: (e) => {
				return new InvokeTauriError('upload', e as string);
			}
		}).pipe(Effect.runPromise);
	}
</script>

<Button {variant} class={className} on:click={uploadPaste}>Upload to PokePaste</Button>
