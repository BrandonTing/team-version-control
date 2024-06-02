<!-- FIXME refactor to smaller components -->
<script lang="ts">
	import { goto } from '$app/navigation';
	import { page } from '$app/stores';
	import * as Breadcrumb from '$lib/components/ui/breadcrumb';
	import * as Sheet from '$lib/components/ui/sheet/index.js';
	import * as Tabs from '$lib/components/ui/tabs';
	import * as Tooltip from '$lib/components/ui/tooltip';
	import { createBranch, createChange } from '@/bindings';
	import UploadPokePasteButton from '@/components/pokePaste/uploadPokePasteButton.svelte';
	import { Description, Root, Title } from '@/components/ui/alert';
	import BreadcrumbItem from '@/components/ui/breadcrumb/breadcrumb-item.svelte';
	import { buttonVariants } from '@/components/ui/button';
	import Button from '@/components/ui/button/button.svelte';
	import * as DropdownMenu from '@/components/ui/dropdown-menu';
	import {
		FormControl,
		FormDescription,
		FormField,
		FormFieldErrors,
		FormLabel
	} from '@/components/ui/form';
	import { Input } from '@/components/ui/input';
	import Separator from '@/components/ui/separator/separator.svelte';
	import { Textarea } from '@/components/ui/textarea';
	import H4 from '@/components/ui/typography/h4.svelte';
	import { InvokeTauriError } from '@/errors';
	import { cn } from '@/utils';
	import { Effect } from 'effect';
	import { Ellipsis } from 'lucide-svelte';
	import ChevronDown from 'lucide-svelte/icons/chevron-down';
	import { superForm } from 'sveltekit-superforms';
	import { zodClient } from 'sveltekit-superforms/adapters';
	import { getPasteFromPokemons } from 'vgc_data_wrapper';
	import { InvalidBranchTitleError } from './errors';
	import PokemonCard from './pokemonCard.svelte';
	import { createBranchFormSchema, createChangeFormSchema, pokepasteUrlSchema } from './schema';

	const { data } = $props();
	const { title } = data;

	const branchParam = $derived(() => {
		const value = data.branchTitle ?? '';
		return {
			value,
			label: value
		};
	});
	let failMessage = $state('');
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

	const createChangeForm = superForm(data.createChangeForm, {
		validators: zodClient(createChangeFormSchema),
		SPA: true,
		onSubmit: async () => {
			await Effect.tryPromise({
				try: async () => {
					if (!data.branchTitle) {
						throw new InvalidBranchTitleError();
					}
					let context = $createChangeFormData.context;
					const checkIsUrl = pokepasteUrlSchema.safeParse(context);
					if (checkIsUrl.success) {
						const content = await fetch(`${checkIsUrl.data}/json`);
						context = ((await content.json()) as unknown as { paste: string }).paste;
					}
					const changeId = await createChange(
						title,
						data.branchTitle,
						$createChangeFormData.message,
						context
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
	let canSubmit = $derived(
		$createChangeFormData.context !== data.change?.context && !!$createChangeFormData.message
	);
	let pokemonTabValue = $state('');
	function updateContext() {
		const paste = getPasteFromPokemons(data.pokemons);
		createChangeFormData.set({
			context: paste,
			message: $createChangeFormData.message
		});
	}
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
		<Breadcrumb.Separator />
		<BreadcrumbItem>
			<DropdownMenu.Root>
				<DropdownMenu.Trigger class="flex items-center gap-1">
					{branchParam().label}
					<ChevronDown class="w-4 h-4" />
				</DropdownMenu.Trigger>
				<DropdownMenu.Content>
					<Sheet.Root>
						<Sheet.Trigger asChild let:builder>
							<Button variant="ghost" class="w-full text-left" size="sm" builders={[builder]}
								>New</Button
							>
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
					<Separator />
					{#each data.team.branches as branch}
						<DropdownMenu.Item
							class="justify-center cursor-pointer"
							on:click={() => {
								if (branch.title === branchParam().label) {
									return;
								}
								let query = new URLSearchParams($page.url.searchParams.toString());
								query.set('branch', branch.title);
								query.set('change', '');
								goto(`?${query.toString()}`);
							}}
						>
							<Tooltip.Root openDelay={200}>
								<Tooltip.Trigger>{branch.title}</Tooltip.Trigger>
								{#if branch.description}
									<Tooltip.Content>
										{#each branch.description.split('\n') as line}
											<p>{line}</p>
										{/each}
									</Tooltip.Content>
								{/if}
							</Tooltip.Root>
						</DropdownMenu.Item>
					{/each}
				</DropdownMenu.Content>
			</DropdownMenu.Root>
		</BreadcrumbItem>
	</Breadcrumb.List>
</Breadcrumb.Root>

{#if failMessage !== ''}
	<Root variant="destructive" class="mt-2">
		<Title>Error!</Title>
		<Description>{failMessage}</Description>
	</Root>
{/if}

{#if data.branch}
	<H4 content="Branch Description"></H4>
	<p class="text-sm text-muted-foreground">
		{data.branch.description || '(empty)'}
	</p>
	<div class="relative">
		<H4
			content={data.change
				? `Latest message: ${data.change.message}`
				: 'Create your very first version!'}
		></H4>
		<DropdownMenu.Root>
			<DropdownMenu.Trigger
				id="more_options_dropdown"
				class={cn(
					buttonVariants({ variant: 'ghost', size: 'icon' }),
					'absolute right-0 -translate-y-1/2 top-1/2'
				)}
			>
				<Ellipsis class="size-4" />
				<span class="sr-only">More</span>
			</DropdownMenu.Trigger>
			<DropdownMenu.Content align="end">
				<DropdownMenu.Item>
					<Button
						variant="ghost"
						class="w-full"
						href={`/team/history?title=${title}&branch=${branchParam().value}`}>View History</Button
					>
				</DropdownMenu.Item>
				{#if data.change}
					<DropdownMenu.Item>
						<UploadPokePasteButton
							className="w-full"
							variant="ghost"
							{title}
							context={data.change.context}
						/>
					</DropdownMenu.Item>
				{/if}
			</DropdownMenu.Content>
		</DropdownMenu.Root>
	</div>
	<Tabs.Root
		onValueChange={(v) => {
			if (!v) {
				return;
			}
			pokemonTabValue = v;
			if (v === 'paste') {
				updateContext();
			}
		}}
		value={data.pokemons[0]?.name ?? ''}
		class="flex flex-col flex-1 w-full"
	>
		<Tabs.List>
			{#each data.pokemons as pokemon}
				<Tabs.Trigger value={pokemon.name ?? ''}>{pokemon.name}</Tabs.Trigger>
			{/each}
			<Separator orientation="vertical" />
			<Tabs.Trigger value="paste">Full Paste</Tabs.Trigger>
		</Tabs.List>
		{#each data.pokemons as pokemon}
			<Tabs.Content value={pokemon.name ?? ''}>
				<PokemonCard {pokemon} />
			</Tabs.Content>
		{/each}
		{#if pokemonTabValue === 'paste'}
			<Tabs.Content value="paste" class="flex flex-col flex-1">
				<Textarea class="flex-1" disabled value={$createChangeFormData.context ?? ''} />
			</Tabs.Content>
		{/if}
	</Tabs.Root>

	<Sheet.Root
		onOpenChange={(isOpen) => {
			if (isOpen) {
				updateContext();
			}
		}}
	>
		<Sheet.Trigger asChild let:builder>
			<Button builders={[builder]}>New Change</Button>
		</Sheet.Trigger>

		<Sheet.Content side="right" class="flex flex-col">
			<Sheet.Header>
				<Sheet.Title>New Change</Sheet.Title>
				<Sheet.Description>Save your idea with a clear message</Sheet.Description>
			</Sheet.Header>
			<form method="POST" use:createChangeForm.enhance class="flex flex-col flex-1">
				<FormField form={createChangeForm} name="context" class="flex flex-col flex-1">
					<FormControl let:attrs>
						<FormLabel>Current team</FormLabel>
						<Textarea
							{...attrs}
							placeholder="Submit First Version!"
							class="flex-1 "
							bind:value={$createChangeFormData.context}
							on:focus={(e) => {
								(e.target as HTMLTextAreaElement)?.setSelectionRange(0, 0);
							}}
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
						<Button disabled={!canSubmit} builders={[builder]} on:click={createChangeForm.submit}
							>Create</Button
						>
					</Sheet.Close>
				</Sheet.Footer>
			</form>
		</Sheet.Content>
	</Sheet.Root>
{/if}
