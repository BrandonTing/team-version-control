<script lang="ts">
	import * as Card from '$lib/components/ui/card';
	import { Input } from '@/components/ui/input';
	import { getTypedObjKeys } from '@/utils';
	import type { Pokemon } from 'vgc_data_wrapper';
	const { pokemon }: { pokemon: Pokemon } = $props();
	console.log(pokemon.moves);
</script>

<Card.Card class="px-4 py-2">
	<Card.Content class="flex justify-center gap-2 p-0 pt-2">
		<div>
			<img src={pokemon.sprite} alt="" />
		</div>
		<div class="flex flex-col gap-1 basis-40">
			<div class="flex gap-1">
				{#each pokemon.types as type}
					<Input value={type} disabled></Input>
				{/each}
			</div>
			<Input placeholder="Tera types" bind:value={pokemon.teraType}></Input>
			<Input placeholder="Items" bind:value={pokemon.originalItem}></Input>
			<Input placeholder="Ability" bind:value={pokemon.ability}></Input>
		</div>
		<div class="flex flex-col gap-1 basis-40">
			{#each Array(4) as _, i}
				<Input placeholder="Move" value={pokemon.moves[i]} />
			{/each}
		</div>
		<div class="grid grid-cols-4 gap-1">
			<span>Base Stat</span>
			<span class="col-span-2">EV</span>
			<span>Stat</span>
			{#each getTypedObjKeys(pokemon.baseStat) as statKey}
				<p class="py-2">{pokemon.baseStat[statKey]}</p>
				<Input placeholder="HP" bind:value={pokemon.effortValues[statKey]} class="col-span-2"
				></Input>
				<p class="py-2 font-semibold">{pokemon.getStat(statKey)}</p>
			{/each}
		</div>
	</Card.Content>
</Card.Card>
