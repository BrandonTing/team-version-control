<!-- FIXME refactor to smaller components -->
<script lang="ts">
	import { goto } from '$app/navigation';
	import { page } from '$app/stores';
	import * as Breadcrumb from '$lib/components/ui/breadcrumb';
	import * as Select from '$lib/components/ui/select/index.js';
	import * as Sheet from '$lib/components/ui/sheet/index.js';
	import Separator from '@/components/ui/separator/separator.svelte';

	import { createBranch, createChange } from '@/bindings';
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
	import Label from '@/components/ui/label/label.svelte';
	import ScrollArea from '@/components/ui/scroll-area/scroll-area.svelte';
	import { Textarea } from '@/components/ui/textarea';
	import { InvokeTauriError } from '@/errors';
	import { Effect } from 'effect';
	import { superForm } from 'sveltekit-superforms';
	import { zodClient } from 'sveltekit-superforms/adapters';
	import type { PageData } from './$types';
	import { InvalidBranchTitleError } from './error';
	import { createBranchFormSchema, createChangeFormSchema } from './schema';
	const { data }: { data: PageData } = $props();
	const { title } = data;

	const branchParam = $derived(() => {
		const value = data.branchTitle ?? '';
		return {
			value,
			label: value
		};
	});
	let failMessage = $state('');
	let selectOpen = $state(false);
	const createBranchForm = superForm(data.createBranchForm, {
		validators: zodClient(createBranchFormSchema),
		SPA: true,
		onSubmit: async () => {
			await Effect.tryPromise({
				try: async () => {
					await createBranch(title, $formData.title, $formData.description);
				},
				catch: (e) => {
					return new InvokeTauriError('createBranch', e as string);
				}
			}).pipe(
				Effect.catchTag('InvokeTauriError', (e) => {
					failMessage = e.message;
					return Effect.succeed(null);
				}),
				Effect.runPromise
			);
		},
		invalidateAll: true
	});
	const { form: formData, enhance, submit } = createBranchForm;
	let edited = $state(false);
	function checkEdited(e: Event) {
		edited = (e.target as HTMLTextAreaElement).value !== (data.change?.context ?? '');
	}

	const createChangeForm = superForm(data.createChangeForm, {
		validators: zodClient(createChangeFormSchema),
		SPA: true,
		onSubmit: async () => {
			await Effect.tryPromise({
				try: async () => {
					if (!data.branchTitle) {
						throw new InvalidBranchTitleError();
					}
					const changeId = await createChange(
						title,
						data.branchTitle,
						$createChangeFormData.message,
						$createChangeFormData.context
					);
					let query = new URLSearchParams($page.url.searchParams.toString());
					query.set('change', changeId);
					goto(`?${query.toString()}`);
				},
				catch: (e) => {
					if (e instanceof InvalidBranchTitleError) {
						goto('/');
						return;
					}
					return new InvokeTauriError('createChange', e as string);
				}
			}).pipe(
				Effect.catchTag('InvokeTauriError', (e) => {
					failMessage = e.message;
					return Effect.succeed(null);
				}),
				Effect.runPromise
			);
		},

		invalidateAll: false
	});
	const { form: createChangeFormData } = createChangeForm;
</script>

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

<p class="text-sm text-muted-foreground">{data.team.team.description}</p>

<div>
	<Label for="context">Branch</Label>
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
			<ScrollArea class="h-40">
				{#each data.team.branches as branch}
					<Select.Item value={branch.title} label={branch.title}>{branch.title}</Select.Item>
				{/each}
			</ScrollArea>
			<Separator />
			<Sheet.Root
				onOpenChange={(value) => {
					if (!value) {
						selectOpen = false;
					}
				}}
			>
				<Sheet.Trigger asChild let:builder>
					<Button variant="ghost" class="w-full" size="sm" builders={[builder]}>New</Button>
				</Sheet.Trigger>
				<Sheet.Content side="right">
					<Sheet.Header>
						<Sheet.Title>New Branch</Sheet.Title>
						<Sheet.Description>
							Want to test a new idea? Let's create a new branch to prevent confusions!
						</Sheet.Description>
					</Sheet.Header>
					<form method="POST" use:enhance>
						<FormField form={createBranchForm} name="title">
							<FormControl let:attrs>
								<FormLabel>Branch Title</FormLabel>
								<Input {...attrs} bind:value={$formData.title} placeholder="title" />
							</FormControl>
							<FormDescription />
							<FormFieldErrors />
						</FormField>
						<FormField form={createBranchForm} name="description">
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
	{#if failMessage !== ''}
		<Root variant="destructive" class="mt-2">
			<Title>Error!</Title>
			<Description>{failMessage}</Description>
		</Root>
	{/if}
</div>

{#if data.branch}
	<p class="text-sm text-muted-foreground">
		{data.branch.description}
	</p>
	<div class="relative">
		<h2
			class="pb-2 text-3xl font-semibold tracking-tight transition-colors border-b scroll-m-20 first:mt-0"
		>
			{#if data.change}
				{data.change.message}
			{:else}
				Create your very first version!
			{/if}
		</h2>
		<Sheet.Root
			onOpenChange={(value) => {
				if (!value) {
					selectOpen = false;
				}
			}}
		>
			<Sheet.Trigger asChild let:builder>
				<Button
					class="absolute right-0 -translate-y-1/2 top-1/2"
					variant="secondary"
					builders={[builder]}
					size="icon"
				>
					<svg
						xmlns="http://www.w3.org/2000/svg"
						width="24"
						height="24"
						viewBox="0 0 24 24"
						fill="none"
						stroke="currentColor"
						stroke-width="2"
						stroke-linecap="round"
						stroke-linejoin="round"
						class="lucide lucide-plus"><path d="M5 12h14" /><path d="M12 5v14" /></svg
					>
				</Button>
			</Sheet.Trigger>
			<Sheet.Content side="right">
				<Sheet.Header>
					<Sheet.Title>New Change</Sheet.Title>
					<Sheet.Description>Save your idea with a clear message</Sheet.Description>
				</Sheet.Header>
				<form method="POST" use:createChangeForm.enhance>
					<FormField form={createChangeForm} name="context">
						<FormControl let:attrs>
							<FormLabel>Current team</FormLabel>
							<Textarea
								{...attrs}
								placeholder="Submit First Version!"
								class="resize-none"
								bind:value={$createChangeFormData.context}
								on:change={checkEdited}
							/>
							<FormDescription>Update the paste</FormDescription>
						</FormControl>
						<FormFieldErrors />
					</FormField>
					<FormField form={createChangeForm} name="message">
						<FormControl let:attrs>
							<FormLabel>Message</FormLabel>
							<Input
								{...attrs}
								placeholder="Leave some message!"
								bind:value={$createChangeFormData.message}
							/>
							<FormDescription>Describe why you update the team</FormDescription>
						</FormControl>
						<FormFieldErrors />
					</FormField>

					<Sheet.Footer>
						<Sheet.Close asChild let:builder>
							<Button disabled={!edited} builders={[builder]} on:click={createChangeForm.submit}
								>Create</Button
							>
						</Sheet.Close>
					</Sheet.Footer>
				</form>
			</Sheet.Content>
		</Sheet.Root>
	</div>
	<Textarea disabled value={data.change?.context ?? ''} />
{/if}
