<script lang="ts">
	import { goto, preloadData } from '$app/navigation';
	import { Description, Root, Title } from '@/components/ui/alert';
	import Button from '@/components/ui/button/button.svelte';
	import type { PageData } from './$types';
	import Team from './team.svelte';
	export let data: PageData;
	const { teams } = data;
</script>

<section class="w-full">
	<div class="flex items-center gap-2">
		<span>Teams</span>
		<Button
			variant="secondary"
			on:click={() => {
				goto('./create');
			}}
			on:mouseenter={async () => await preloadData('/create')}>New</Button
		>
	</div>
	{#if teams.length === 0}
		<Root>
			<Title>Heads up!</Title>
			<Description>You can add team by click the New button.</Description>
		</Root>
	{/if}

	<div class="grid grid-cols-4 gap-4 py-2">
		{#each teams as team}
			<Team {team} />
		{/each}
	</div>
</section>
