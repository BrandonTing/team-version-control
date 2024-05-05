<script lang="ts">
	import { preloadData } from '$app/navigation';
	import * as AlertDialog from '$lib/components/ui/alert-dialog';
	import * as Table from '$lib/components/ui/table';
	import { resetStore } from '@/bindings';
	import { Description, Root, Title } from '@/components/ui/alert';
	import { buttonVariants } from '@/components/ui/button';
	import Button from '@/components/ui/button/button.svelte';
	import { Input } from '@/components/ui/input';
	import type { PageData } from './$types';

	const { data }: { data: PageData } = $props();
	const { teamsRequest } = data;

	let searchKw = $state('');

	async function deleteTeam(title: string) {
		console.log('wefwefew');
		// FIXME a infinite loop here
		await deleteTeam(title);
		// TODO invalidate path
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
	{#await teamsRequest}
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
							<Table.Cell>{team.description}</Table.Cell>
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
												on:click={() => deleteTeam(team.title)}
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
