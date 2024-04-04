<script lang="ts">
	import * as Breadcrumb from '$lib/components/ui/breadcrumb';

	import { page } from '$app/stores';
	const teamTitle = decodeURIComponent($page.url.searchParams.get('title') ?? '');
</script>

<Breadcrumb.Root>
	<Breadcrumb.List>
		<Breadcrumb.Item>
			<Breadcrumb.Link href="/">Home</Breadcrumb.Link>
		</Breadcrumb.Item>
		<Breadcrumb.Separator />
		<Breadcrumb.Item>
			<Breadcrumb.Page>{teamTitle}</Breadcrumb.Page>
		</Breadcrumb.Item>
	</Breadcrumb.List>
</Breadcrumb.Root>
