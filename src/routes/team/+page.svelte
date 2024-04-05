<script lang="ts">
	import { goto } from '$app/navigation';
	import { page } from '$app/stores';
	import * as Breadcrumb from '$lib/components/ui/breadcrumb';
	import * as Select from '$lib/components/ui/select/index.js';
	import * as Sheet from '$lib/components/ui/sheet/index.js';
	import Separator from '@/components/ui/separator/separator.svelte';

	import { createBranch } from '@/bindings';
	import { Description, Root, Title } from '@/components/ui/alert';
	import Button from '@/components/ui/button/button.svelte';
	import {
		FormControl,
		FormDescription,
		FormField,
		FormFieldErrors,
		FormLabel
	} from '@/components/ui/form';
	import { Input } from '@/components/ui/input';
	import { Textarea } from '@/components/ui/textarea';
	import { superForm } from 'sveltekit-superforms';
	import { zodClient } from 'sveltekit-superforms/adapters';
	import type { PageData } from './$types';
	import { createBranchFormSchema } from './schema';
	const { data }: { data: PageData } = $props();
	const { title, team } = data;
	if (!$page.url.searchParams.get('branch')) {
		let query = new URLSearchParams($page.url.searchParams.toString());
		query.set('branch', team.current_branch_title);
		goto(`?${query.toString()}`);
	}
	const branchParam = $derived(() => {
		const value = decodeURIComponent($page.url.searchParams.get('branch') ?? '');
		return {
			value,
			label: value
		};
	});
	let failMessage = $state('');
	let selectOpen = $state(false);
	let sheetOpen = $state(false);
	const form = superForm(data.form, {
		validators: zodClient(createBranchFormSchema),
		SPA: true,
		async onSubmit() {
			try {
				await createBranch(title, $formData.title, $formData.description);
			} catch (e) {
				failMessage = e as string;
			}
		}
	});
	const { form: formData, enhance, submit } = form;
</script>

<div class="flex flex-col gap-2">
	<Breadcrumb.Root>
		<Breadcrumb.List>
			<Breadcrumb.Item>
				<Breadcrumb.Link href="/">Home</Breadcrumb.Link>
			</Breadcrumb.Item>
			<Breadcrumb.Separator />
			<Breadcrumb.Item>
				<Breadcrumb.Page>{title}</Breadcrumb.Page>
			</Breadcrumb.Item>
		</Breadcrumb.List>
	</Breadcrumb.Root>
	<Select.Root
		portal={null}
		selected={branchParam()}
		onSelectedChange={(selected) => {
			if (selected) {
				let query = new URLSearchParams($page.url.searchParams.toString());
				query.set('branch', selected.value);
				goto(`?${query.toString()}`);
			}
		}}
		open={selectOpen}
	>
		<Select.Trigger class="w-[180px]" on:click={() => (selectOpen = true)}>
			<Select.Value placeholder="Select a Branch" />
		</Select.Trigger>
		<Select.Content>
			{#each team.branches as branch}
				<Select.Item value={branch.title} label={branch.title}>{branch.title}</Select.Item>
			{/each}
			<Separator />
			<Sheet.Root
				onOpenChange={(value) => {
					if (!value) {
						selectOpen = false;
					}
				}}
			>
				<Sheet.Trigger asChild let:builder>
					<Button variant="ghost" class="w-full" size="sm" builders={[builder]}>Open</Button>
				</Sheet.Trigger>
				<Sheet.Content side="right">
					<Sheet.Header>
						<Sheet.Title>New Branch</Sheet.Title>
						<Sheet.Description>
							Want to test a new idea? Let's create a new branch to prevent confusions!
						</Sheet.Description>
					</Sheet.Header>
					<form method="POST" use:enhance>
						<FormField {form} name="title">
							<FormControl let:attrs>
								<FormLabel>Branch Title</FormLabel>
								<Input {...attrs} bind:value={$formData.title} placeholder="title" />
							</FormControl>
							<FormDescription />
							<FormFieldErrors />
						</FormField>
						<FormField {form} name="description">
							<FormControl let:attrs>
								<FormLabel>Description</FormLabel>
								<Textarea
									{...attrs}
									bind:value={$formData.description}
									placeholder="Describe the idea"
								/>
							</FormControl>
							<FormDescription />
							<FormFieldErrors />
						</FormField>

						<Sheet.Footer>
							<Sheet.Close asChild let:builder>
								<Button builders={[builder]} on:click={submit}>Create</Button>
							</Sheet.Close>
						</Sheet.Footer>
					</form>
				</Sheet.Content>
			</Sheet.Root>
		</Select.Content>
	</Select.Root>
</div>
{#if failMessage !== ''}
	<Root variant="destructive" class="mt-2">
		<Title>Error!</Title>
		<Description>{failMessage}</Description>
	</Root>
{/if}
