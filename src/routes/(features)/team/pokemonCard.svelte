<script lang="ts">
	import * as Card from '$lib/components/ui/card';
	import { Input } from '@/components/ui/input';
	import type { Pokemon } from 'vgc_data_wrapper';
	const { pokemon }: { pokemon: Pokemon } = $props();
</script>

<Card.Card class="px-4 py-2">
	<Card.Title>{pokemon.name}</Card.Title>
	<Card.Content class="flex gap-2 p-0">
		<div class="basis-80">Image</div>
		<div class="flex flex-col gap-1 basis-40">
			<div class="flex">
				{#each pokemon.types as type}
					<Input value={type} disabled></Input>
				{/each}
			</div>
			<Input placeholder="Tera types"></Input>
			<Input placeholder="Items"></Input>
			<Input placeholder="Ability"></Input>
		</div>
		<div class="flex-col gap-1 basis-40">
			<Input placeholder="Move"></Input>
			<Input placeholder="Move"></Input>
			<Input placeholder="Move"></Input>
			<Input placeholder="Move"></Input>
		</div>
		<div class="flex flex-col flex-1 gap-1 basis-80">
			<div class="flex gap-1">
				<Input placeholder="HP"></Input>
				<Input placeholder="HP"></Input>
				<p class="py-2 font-semibold">{pokemon.baseStat?.hp}</p>
			</div>
			<div class="flex gap-1">
				<Input placeholder="Atk"></Input>
				<Input placeholder="Atk"></Input>
			</div>
			<div class="flex gap-1">
				<Input placeholder="Def"></Input>
				<Input placeholder="Def"></Input>
			</div>
			<div class="flex gap-1">
				<Input placeholder="Spa"></Input>
				<Input placeholder="Spa"></Input>
			</div>
			<div class="flex gap-1">
				<Input placeholder="Spd"></Input>
				<Input placeholder="Spd"></Input>
			</div>
			<div class="flex gap-1">
				<Input placeholder="Spe"></Input>
				<Input placeholder="Spe"></Input>
			</div>
		</div>
	</Card.Content>
</Card.Card>
