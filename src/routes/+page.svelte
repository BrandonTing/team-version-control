<script lang="ts">
	import type { PageData } from './$types';
	export let data: PageData;
</script>

<svelte:head>
	<title>Version Control</title>
	<meta name="description" content="Version control app for VGC teams" />
</svelte:head>

<section>
	Teams
	{#each data.teams as team}
		<li>{team.title}</li>
	{/each}
</section>
