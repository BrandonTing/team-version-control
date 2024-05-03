<script>
	import Footer from './footer.svelte';
	import './styles.css';
</script>

<svelte:head>
	<title>Version Control</title>
	<meta name="description" content="Version control app for VGC teams" />
</svelte:head>

<div class="flex flex-col w-full min-h-screen">
	<main class="box-border flex flex-col self-center flex-1 w-full max-w-5xl p-4">
		<slot />
	</main>
	<Footer />
</div>
