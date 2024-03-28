<script lang="ts">
	import Button from '$lib/components/ui/button/button.svelte';
	import CardContent from '$lib/components/ui/card/card-content.svelte';
	import Card from '$lib/components/ui/card/card.svelte';
	import { FormControl } from '$lib/components/ui/form';
	import FormDescription from '$lib/components/ui/form/form-description.svelte';
	import FormFieldErrors from '$lib/components/ui/form/form-field-errors.svelte';
	import FormField from '$lib/components/ui/form/form-field.svelte';
	import FormLabel from '$lib/components/ui/form/form-label.svelte';
	import Input from '$lib/components/ui/input/input.svelte';
	import { superForm } from 'sveltekit-superforms';
	import { zodClient } from 'sveltekit-superforms/adapters';
	import type { PageData } from './$types';
	import { formSchema } from './schema';
	import Team from './team.svelte';

	export let data: PageData;

	const form = superForm(data.form, {
		validators: zodClient(formSchema),
		SPA: true,
		onSubmit() {
			// TODO
			message = 'message';
		}
	});
	const { form: formData, enhance } = form;
	let message = '';
</script>

<svelte:head>
	<title>Version Control</title>
	<meta name="description" content="Version control app for VGC teams" />
</svelte:head>

<section>
	Teams
	<Card>
		<CardContent>
			No team yet? Let's create your first team!
			<form method="POST" use:enhance>
				<FormField {form} name="username">
					<FormControl let:attrs>
						<FormLabel>User name</FormLabel>
						<Input {...attrs} bind:value={$formData.username} />
					</FormControl>
					<FormDescription />
					<FormFieldErrors />
				</FormField>
				<Button type="submit">submit</Button>
			</form>
		</CardContent>
	</Card>
	{#each data.teams as team}
		<Team {team} />
	{/each}
</section>
