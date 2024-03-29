<script lang="ts">
	import CardTitle from '$lib/components/ui/card/card-title.svelte';
	import Card from '$lib/components/ui/card/card.svelte';
	import type { Team } from '@/bindings';
	import CardDescription from '@/components/ui/card/card-description.svelte';

	let {
		team
	}: {
		team: Team;
	} = $props();
</script>

<Card class="px-4 py-2">
	<CardTitle>{team.title}</CardTitle>
	<CardDescription>
		{team.description}
	</CardDescription>
</Card>
