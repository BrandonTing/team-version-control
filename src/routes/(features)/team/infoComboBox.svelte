<script lang="ts">
	import { flyAndScale } from '@/utils';
	import { Combobox } from 'bits-ui';
	import { Check, ChevronsUpDown } from 'lucide-svelte';

	let { options, inputValue = $bindable('') }: { options: Array<string>; inputValue?: string } =
		$props();
	let touchedInput = $state(false);
	const filteredOptions = $derived(
		options
			.filter((option) => option.toLowerCase().includes(inputValue.toLowerCase()))
			.map((option) => ({ value: option }))
	);
	$effect(() => {});
</script>

<Combobox.Root items={filteredOptions} bind:inputValue bind:touchedInput>
	<div class="relative">
		<Combobox.Input
			class="flex w-full h-10 px-3 py-2 text-sm border rounded-md border-input bg-background ring-offset-background file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50"
			placeholder="Search an option"
			aria-label="Search an option"
		/>
		<ChevronsUpDown class="absolute -translate-y-1/2 end-3 top-1/2 size-4 text-muted-foreground" />
	</div>

	<Combobox.Content
		class="w-full px-1 py-1 border outline-none rounded-xl border-muted bg-background shadow-popover"
		transition={flyAndScale}
		sideOffset={8}
	>
		{#each filteredOptions as option (option.value)}
			<Combobox.Item
				class="flex  w-full select-none items-center rounded-button py-3 pl-5 pr-1.5 text-sm capitalize outline-none transition-all duration-75 data-[highlighted]:bg-muted"
				value={option.value}
				label={option.value}
			>
				{option.value}
				<Combobox.ItemIndicator class="ml-auto" asChild={false}>
					<Check />
				</Combobox.ItemIndicator>
			</Combobox.Item>
		{:else}
			<span class="block px-5 py-2 text-sm text-muted-foreground"> No results found </span>
		{/each}
	</Combobox.Content>
	<Combobox.HiddenInput name="favoriteFruit" />
</Combobox.Root>
