<script lang="ts">
	import { goto } from '$app/navigation';
	import * as Breadcrumb from '$lib/components/ui/breadcrumb';
	import Button from '$lib/components/ui/button/button.svelte';
	import {
		FormControl,
		FormDescription,
		FormField,
		FormFieldErrors,
		FormLabel
	} from '$lib/components/ui/form';
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
				await createTeam($formData.title, $formData.description, $formData.mainBranchTitle);
				goto(nextUrl);
			} catch (e) {
				failMessage = e as string;
			}
		}
	});
	const { form: formData, enhance } = form;
</script>

<Breadcrumb.Root>
	<Breadcrumb.List>
		<Breadcrumb.Item>
			<Breadcrumb.Link href="/">Home</Breadcrumb.Link>
		</Breadcrumb.Item>
		<Breadcrumb.Separator />
		<Breadcrumb.Item>
			<Breadcrumb.Page>create new team</Breadcrumb.Page>
		</Breadcrumb.Item>
	</Breadcrumb.List>
</Breadcrumb.Root>

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
	<FormField {form} name="mainBranchTitle">
		<FormControl let:attrs>
			<FormLabel>Main Branch</FormLabel>
			<Input {...attrs} bind:value={$formData.mainBranchTitle} />
		</FormControl>
		<FormDescription />
		<FormFieldErrors />
	</FormField>

	<Button type="submit">submit</Button>
</form>

{#if failMessage !== ''}
	<Root variant="destructive" class="mt-2">
		<Title>Error!</Title>
		<Description>{failMessage}</Description>
	</Root>
{/if}
