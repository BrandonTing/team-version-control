<script lang="ts">
	import { HttpClient } from '@effect/platform';
	import { Console, Effect } from 'effect';
	import type { ComponentProps } from 'svelte';
	import Button from '../ui/button/button.svelte';
	const {
		title,
		variant,
		context
	}: { title: string; context: string; variant: ComponentProps<Button>['variant'] } = $props();

	async function upload() {
		await Effect.gen(function* () {
			const formData = new FormData();
			formData.append('title', title);
			formData.append('paste', context);
			formData.append('author', 'Version Control App');

			const response = yield* HttpClient.request
				.post('https://pokepast.es/create')
				.pipe(
					HttpClient.request.formDataBody(formData),
					HttpClient.client.fetchOk,
					HttpClient.response.json
				);
			yield* Console.log(response);
		}).pipe(Effect.runPromise);
	}
</script>

<Button {variant} on:click={upload}>Upload to PokePaste</Button>
