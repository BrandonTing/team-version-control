<script lang="ts">
	import { Button } from '$lib/components/ui/button/index.js';
	import * as Command from '$lib/components/ui/command/index.js';
	import * as Popover from '$lib/components/ui/popover/index.js';
	import { cn } from '$lib/utils.js';
	import { ScrollArea } from '@/components/ui/scroll-area';
	import Check from 'lucide-svelte/icons/check';
	import ChevronsUpDown from 'lucide-svelte/icons/chevrons-up-down';
	import { tick } from 'svelte';

	const { options, defaultValue }: { options: Array<string>; defaultValue?: string } = $props();
	let open = $state(false);
	let value = $state(defaultValue?.trim() ?? '');
	const selectedValue = $derived(
		options.find((option) => option.toLowerCase() === value.toLowerCase()) ?? 'Select a option...'
	);
	function closeAndFocusTrigger(triggerId: string) {
		open = false;
		tick().then(() => {
			document.getElementById(triggerId)?.focus();
		});
	}
</script>

<Popover.Root bind:open let:ids>
	<Popover.Trigger asChild let:builder>
		<Button
			builders={[builder]}
			variant="outline"
			role="combobox"
			aria-expanded={open}
			class="w-[200px] justify-between"
		>
			{selectedValue}
			<ChevronsUpDown class="w-4 h-4 ml-2 opacity-50 shrink-0" />
		</Button>
	</Popover.Trigger>
	<Popover.Content class="w-[200px] p-0">
		<Command.Root>
			<Command.Input placeholder="Search..." />
			<Command.Empty>No option found.</Command.Empty>
			<Command.Group>
				<ScrollArea class="h-48">
					{#each options as option}
						<Command.Item
							value={option}
							onSelect={(currentValue) => {
								value = currentValue;
								closeAndFocusTrigger(ids.trigger);
							}}
						>
							<Check class={cn('mr-2 h-4 w-4', value !== option && 'text-transparent')} />
							{option}
						</Command.Item>
					{/each}
				</ScrollArea>
			</Command.Group>
		</Command.Root>
	</Popover.Content>
</Popover.Root>
