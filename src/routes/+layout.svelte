<script>
	import Footer from './Footer.svelte';
	import './styles.css';
</script>

<div class="flex flex-col min-h-screen">
	<main class="box-border flex flex-col flex-grow flex-shrink w-full max-w-5xl p-4">
		<slot />
	</main>
	<Footer />
</div>
