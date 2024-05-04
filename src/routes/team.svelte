<script lang="ts">
	import CardTitle from '$lib/components/ui/card/card-title.svelte';
	import Card from '$lib/components/ui/card/card.svelte';
	import type { Team } from '@/bindings';
	import Button from '@/components/ui/button/button.svelte';
	import CardDescription from '@/components/ui/card/card-description.svelte';

	const {
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
	<Button class="h-5" href={`/team?title=${team.title}`}>Detail</Button>
</Card>
