<script lang="ts">
	import CardContent from '$lib/components/ui/card/card-content.svelte';
	import CardTitle from '$lib/components/ui/card/card-title.svelte';
	import Card from '$lib/components/ui/card/card.svelte';
	import type { Team } from '../bindings';

	export let team: Team;
</script>

<Card>
	<CardTitle>{team.title}</CardTitle>
	<CardContent>
		{team.description}
	</CardContent>
</Card>
