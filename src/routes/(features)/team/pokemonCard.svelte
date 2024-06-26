<script lang="ts">
	import * as Card from '$lib/components/ui/card';
	import * as Select from '$lib/components/ui/select';

	import { Input } from '@/components/ui/input';
	import Label from '@/components/ui/label/label.svelte';
	import ScrollArea from '@/components/ui/scroll-area/scroll-area.svelte';
	import { getTypedObjKeys } from '@/utils';
	import { types, type Pokemon } from 'vgc_data_wrapper';
	import abilities from 'vgc_data_wrapper/sd/abilities.json';
	import items from 'vgc_data_wrapper/sd/items.json';
	import moves from 'vgc_data_wrapper/sd/moves.json';
	import InfoComboBox from './infoComboBox.svelte';
	const { pokemon }: { pokemon: Pokemon } = $props();
	let teraType = $state({ value: pokemon.teraType, label: pokemon.teraType });
	$effect(() => {
		teraType = {
			value: pokemon.teraType,
			label: pokemon.teraType
		};
	});
	let stats = $state(pokemon.getStats());
</script>

<Card.Card class="px-4 py-2">
	<Card.Content class="flex justify-center gap-2 p-0 pt-2">
		<div>
			<img src={pokemon.sprite} alt="" />
		</div>
		<div class="flex flex-col gap-1 basis-40">
			<div>
				<Label>Types</Label>
				<div class="flex gap-1">
					{#each pokemon.types as type}
						<Input value={type} disabled></Input>
					{/each}
				</div>
			</div>
			<div>
				<Label>Tera Type</Label>
				<Select.Root
					bind:selected={teraType}
					onSelectedChange={(v) => {
						if (v?.value) pokemon.teraType = v.value;
					}}
				>
					<Select.Trigger>
						<Select.Value placeholder="Tera Type" />
					</Select.Trigger>

					<Select.Content>
						<ScrollArea class="h-48">
							{#each types as type}
								<Select.Item value={type}>{type}</Select.Item>
							{/each}
							<Select.Item value="Stellar">Stellar</Select.Item>
						</ScrollArea>
					</Select.Content>
				</Select.Root>
			</div>
			<div>
				<Label>Item</Label>
				<InfoComboBox
					options={Object.values(items).map((item) => item.name)}
					inputValue={pokemon.originalItem}
				/>
			</div>
			<div>
				<Label>Ability</Label>
				<InfoComboBox
					options={Object.values(abilities).map((item) => item.name)}
					inputValue={pokemon.ability}
				/>
			</div>
		</div>
		<div class="flex flex-col gap-1 basis-40">
			<Label>Move</Label>
			{#each Array(4) as _, i}
				<InfoComboBox
					options={Object.values(moves).map((move) => move.name)}
					inputValue={pokemon.moves[i]}
				/>
			{/each}
		</div>
		<div class="grid grid-cols-4 gap-1">
			<span>Base Stat</span>
			<span class="col-span-2">EV</span>
			<span>Stat</span>
			{#each getTypedObjKeys(pokemon.baseStat) as statKey}
				<p class="py-2">{pokemon.baseStat[statKey]}</p>
				<Input
					type="number"
					step={4}
					max={252}
					min={0}
					bind:value={pokemon.effortValues[statKey]}
					class="col-span-2"
					on:change={() => {
						stats[statKey] = pokemon.getStat(statKey);
					}}
				></Input>
				<p class="py-2 font-semibold">{stats[statKey]}</p>
			{/each}
		</div>
	</Card.Content>
</Card.Card>
