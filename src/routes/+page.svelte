<script lang="ts">
	import { goto, preloadData } from '$app/navigation';
	import * as Table from '$lib/components/ui/table';
	import { Description, Root, Title } from '@/components/ui/alert';
	import Button from '@/components/ui/button/button.svelte';
	import { Input } from '@/components/ui/input';
	import type { PageData } from './$types';

	const { data }: { data: PageData } = $props();
	const { teamsRequest } = data;

	let searchKw = $state('');
</script>

<section class="flex flex-col w-full gap-2">
	<div class="flex items-center gap-2">
		<Input placeholder="Find a team..." class="w-64" bind:value={searchKw} />
		<Button
			variant="secondary"
			on:click={() => {
				goto('./create');
			}}
			on:mouseenter={async () => await preloadData('/create')}>New</Button
		>
	</div>
	{#await teamsRequest}
		<p>loading...</p>
	{:then teams}
		{#if teams.length === 0}
			<Root variant="destructive">
				<Title>Heads up!</Title>
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
						<Table.Head>Current Branch</Table.Head>
						<Table.Head>Created At</Table.Head>
					</Table.Row>
				</Table.Header>
				<Table.Body>
					{#each teams
						.toSorted((a, b) => b.created_at - a.created_at)
						.filter((team) => team.title.includes(searchKw)) as team}
						<Table.Row>
							<Table.Cell class="font-medium">
								<Button variant="link" class="px-0">
									{team.title}
								</Button>
							</Table.Cell>
							<Table.Cell>{team.description}</Table.Cell>
							<Table.Cell>{team.current_branch_title}</Table.Cell>
							<Table.Cell>{new Date(team.created_at * 1000)}</Table.Cell>
						</Table.Row>
					{/each}
				</Table.Body>
			</Table.Root>
		{/if}
	{/await}
</section>
