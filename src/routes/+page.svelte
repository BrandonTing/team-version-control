<script lang="ts">
	import { goto, invalidateAll, preloadData } from '$app/navigation';
	import * as AlertDialog from '$lib/components/ui/alert-dialog';
	import * as Table from '$lib/components/ui/table';
	import * as Tooltip from '$lib/components/ui/tooltip';
	import { deleteTeam, resetStore } from '@/bindings';
	import { Description, Root, Title } from '@/components/ui/alert';
	import { buttonVariants } from '@/components/ui/button';
	import Button from '@/components/ui/button/button.svelte';
	import { Input } from '@/components/ui/input';
	import { InvokeTauriError, RedirectError } from '@/errors';
	import { getEllipsisText } from '@/utils';
	import { Effect, Either } from 'effect';
	import type { PageData } from './$types';

	const { data }: { data: PageData } = $props();

	let searchKw = $state('');

	async function deleteTeamHandler(title: string) {
		const possibleErrors = await Effect.gen(function* () {
			yield* Effect.tryPromise({
				try: async () => {
					await deleteTeam(title);
				},
				catch: (e) => {
					return new InvokeTauriError('deleteTeam', e as string);
				}
			});
			yield* Effect.tryPromise({
				try: async () => {
					await invalidateAll();
				},
				catch: (e) => {
					return new RedirectError({ path: '/' });
				}
			});
		}).pipe(
			Effect.catchTag('InvokeTauriError', (e) => {
				return Effect.succeed(undefined);
			}),
			Effect.either,
			Effect.runPromise
		);
		if (Either.isLeft(possibleErrors)) {
			if (possibleErrors.left instanceof RedirectError) {
				await goto(possibleErrors.left.path);
			}
		}
	}
</script>

<section class="flex flex-col w-full gap-2">
	<div class="flex items-center gap-2">
		<Input placeholder="Find a team..." class="w-64" bind:value={searchKw} />
		<Button
			variant="secondary"
			href="/create"
			on:mouseenter={async () => await preloadData('/create')}>New</Button
		>
		<Button variant="outline" on:click={resetStore}>Remove all teams</Button>
	</div>
	{#await data.teamsRequest}
		<p>loading...</p>
	{:then teams}
		{#if teams.length === 0}
			<Root variant="destructive">
				<Title>Empty Teambuilder!</Title>
				<Description>You can add team by click the New button.</Description>
			</Root>
		{:else}
			<!-- use table -->
			<Table.Root>
				<Table.Caption>A list of your recent teams.</Table.Caption>
				<Table.Header>
					<Table.Row>
						<Table.Head class="w-[100px]">Title</Table.Head>
						<Table.Head>Description</Table.Head>
						<Table.Head>Created At</Table.Head>
						<Table.Head></Table.Head>
					</Table.Row>
				</Table.Header>
				<Table.Body>
					{#each teams
						.toSorted((a, b) => b.created_at - a.created_at)
						.filter((team) => team.title.includes(searchKw)) as team}
						<Table.Row>
							<Table.Cell class="font-medium">
								<Button
									variant="link"
									class="px-0"
									href={`./team?title=${encodeURIComponent(team.title)}`}
								>
									{team.title}
								</Button>
							</Table.Cell>
							<Table.Cell>
								<Tooltip.Root openDelay={200}>
									<Tooltip.Trigger>{getEllipsisText(team.description, 30)}</Tooltip.Trigger>
									<Tooltip.Content>
										{#each team.description.split('\n') as line}
											<p>{line}</p>
										{/each}
									</Tooltip.Content>
								</Tooltip.Root>
							</Table.Cell>
							<Table.Cell>{new Date(team.created_at * 1000).toLocaleString()}</Table.Cell>
							<Table.Cell>
								<AlertDialog.Root>
									<AlertDialog.Trigger>
										<Button variant="destructive">Delete</Button>
									</AlertDialog.Trigger>
									<AlertDialog.Content>
										<AlertDialog.Header>
											<AlertDialog.Title>Are you absolutely sure?</AlertDialog.Title>
											<AlertDialog.Description>
												This action cannot be undone. This will permanently delete your teams,
												branches and all changes.
											</AlertDialog.Description>
										</AlertDialog.Header>
										<AlertDialog.Footer>
											<AlertDialog.Cancel>Cancel</AlertDialog.Cancel>
											<AlertDialog.Action
												class={buttonVariants({ variant: 'destructive' })}
												on:click={() => deleteTeamHandler(team.title)}
											>
												Confirm
											</AlertDialog.Action>
										</AlertDialog.Footer>
									</AlertDialog.Content>
								</AlertDialog.Root>
							</Table.Cell>
						</Table.Row>
					{/each}
				</Table.Body>
			</Table.Root>
		{/if}
	{/await}
</section>
