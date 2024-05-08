<script lang="ts">
	import { goto } from '$app/navigation';
	import { createBranchFromChange } from '$lib/bindings';
	import * as Breadcrumb from '$lib/components/ui/breadcrumb';
	import * as Dialog from '$lib/components/ui/dialog';
	import * as Sheet from '$lib/components/ui/sheet/index.js';
	import * as Table from '$lib/components/ui/table';
	import { InvokeTauriError } from '$lib/errors';
	import UploadPokePasteButton from '@/components/pokePaste/uploadPokePasteButton.svelte';
	import { Description, Root, Title } from '@/components/ui/alert';
	import Button from '@/components/ui/button/button.svelte';
	import { buttonVariants } from '@/components/ui/button/index.js';
	import {
		FormControl,
		FormDescription,
		FormField,
		FormFieldErrors,
		FormLabel
	} from '@/components/ui/form';
	import { Input } from '@/components/ui/input';
	import ScrollArea from '@/components/ui/scroll-area/scroll-area.svelte';
	import { Textarea } from '@/components/ui/textarea';
	import { Effect } from 'effect';
	import { superForm } from 'sveltekit-superforms';
	import { zodClient } from 'sveltekit-superforms/adapters';
	import { createBranchFromChangeFormSchema } from './schema';
	const { data } = $props();
	const { teamTitle, branchTitle } = data.data;
	let failMessage = $state('');

	let chosenChangeId = $state('');
	const createBranchForm = superForm(data.form, {
		validators: zodClient(createBranchFromChangeFormSchema),
		SPA: true,
		onSubmit: async () => {
			await Effect.tryPromise({
				try: async () => {
					await createBranchFromChange(
						teamTitle,
						$formData.title,
						$formData.description,
						branchTitle,
						chosenChangeId
					);
					await goto(`/team?title=${teamTitle}&branch=${$formData.title}&change=${chosenChangeId}`);
				},
				catch: (e) => {
					return new InvokeTauriError('createBranchFromChange', e as string);
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
</script>

<Breadcrumb.Root>
	<Breadcrumb.List>
		<Breadcrumb.Item>
			<Breadcrumb.Link href="/">Home</Breadcrumb.Link>
		</Breadcrumb.Item>
		<Breadcrumb.Separator />
		<Breadcrumb.Item>
			<Breadcrumb.Link href={`/team?title=${teamTitle}&branch=${branchTitle}`}
				>{teamTitle}</Breadcrumb.Link
			>
		</Breadcrumb.Item>
		<Breadcrumb.Separator />
		<Breadcrumb.Item>
			<Breadcrumb.Page>{branchTitle}</Breadcrumb.Page>
		</Breadcrumb.Item>
	</Breadcrumb.List>
</Breadcrumb.Root>

<Table.Root>
	<Table.Caption>Change History of Branch {branchTitle} of Team {teamTitle}</Table.Caption>
	<Table.Header>
		<Table.Row>
			<Table.Head class="text-center">Message</Table.Head>
			<Table.Head></Table.Head>
		</Table.Row>
	</Table.Header>
	<Table.Body class="text-center">
		{#each data.data.history as change, i (i)}
			<Table.Row>
				<Table.Cell class="font-medium">{change.message}</Table.Cell>
				<Table.Cell>
					<Dialog.Root>
						<Dialog.Trigger class={buttonVariants({ variant: 'default' })}>
							Show Full Context
						</Dialog.Trigger>
						<Dialog.Content class="flex flex-col w-3/5 h-4/5">
							<Dialog.Header>
								<Dialog.Title class="text-2xl text-center">
									{change.message}
								</Dialog.Title>
								<!-- <Dialog.Description>
									This action cannot be undone. This will permanently delete your account and remove
									your data from our servers.
								</Dialog.Description> -->
							</Dialog.Header>
							<ScrollArea class="flex-1">
								<p class="whitespace-pre-line">{change.context}</p>
							</ScrollArea>
							<div class="flex justify-end gap-2">
								<UploadPokePasteButton
									variant="outline"
									title={teamTitle}
									context={change.context}
								/>
								<Button
									on:click={() => {
										chosenChangeId = change.id;
									}}>Create Branch from this change</Button
								>
							</div>
							{#if failMessage !== ''}
								<Root variant="destructive" class="mt-2">
									<Title>Error!</Title>
									<Description>{failMessage}</Description>
								</Root>
							{/if}
						</Dialog.Content>
					</Dialog.Root>
				</Table.Cell>
			</Table.Row>
		{/each}
	</Table.Body>
</Table.Root>

<Sheet.Root open={Boolean(chosenChangeId)}>
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
					<Textarea {...attrs} bind:value={$formData.description} placeholder="Describe the idea" />
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
