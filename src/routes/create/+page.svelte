<script lang="ts">
	import { goto, preloadData } from '$app/navigation';
	import Button from '$lib/components/ui/button/button.svelte';
	import { FormControl } from '$lib/components/ui/form';
	import FormDescription from '$lib/components/ui/form/form-description.svelte';
	import FormFieldErrors from '$lib/components/ui/form/form-field-errors.svelte';
	import FormField from '$lib/components/ui/form/form-field.svelte';
	import FormLabel from '$lib/components/ui/form/form-label.svelte';
	import Input from '$lib/components/ui/input/input.svelte';
	import Textarea from '$lib/components/ui/textarea/textarea.svelte';
	import { createTeam } from '@/bindings';
	import { Description, Root, Title } from '@/components/ui/alert';
	import { superForm } from 'sveltekit-superforms';
	import { zodClient } from 'sveltekit-superforms/adapters';
	import { createTeamFormSchema } from './schema';

	let { data } = $props();
	let nextUrl = $state('/');
	let failMessage = $state('');

	const form = superForm(data.form, {
		validators: zodClient(createTeamFormSchema),
		SPA: true,
		async onSubmit() {
			try {
				await createTeam($formData.title, $formData.description);
				goto(nextUrl);
			} catch (e) {
				failMessage = e as string;
			}
		}
	});
	const { form: formData, enhance } = form;
</script>

<form method="POST" use:enhance>
	<FormField {form} name="title">
		<FormControl let:attrs>
			<FormLabel>Team Title</FormLabel>
			<Input {...attrs} bind:value={$formData.title} placeholder="title" />
		</FormControl>
		<FormDescription />
		<FormFieldErrors />
	</FormField>
	<FormField {form} name="description">
		<FormControl let:attrs>
			<FormLabel>Description</FormLabel>
			<Textarea {...attrs} bind:value={$formData.description} placeholder="Describe the team" />
		</FormControl>
		<FormDescription />
		<FormFieldErrors />
	</FormField>

	<Button
		on:click={() => {
			goto(nextUrl);
		}}
		on:mouseenter={async () => await preloadData(nextUrl)}
		variant="secondary">cancel</Button
	>
	<Button type="submit">submit</Button>
</form>

{#if failMessage !== ''}
	<Root variant="destructive" class="mt-2">
		<Title>Error!</Title>
		<Description>{failMessage}</Description>
	</Root>
{/if}
