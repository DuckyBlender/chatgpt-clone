<script lang="ts">
	import '../app.css';
	import { onMount } from 'svelte';

	let commitCount = 'Loading...';
	onMount(() => {
		getCommitNumber();
	});
	async function getCommitNumber() {
		let res = await fetch(
			'https://api.github.com/repos/DuckyBlender/chatgpt-clone/commits?per_page=1000'
		);
		let res_json = await res.json();
		commitCount = `Commit ${res_json.length}`;
		return commitCount;
	}
</script>

<svelte:head>
	<!-- Meta tags for discord embed -->
	<meta property="og:title" content="ChatGPT" />
	<meta
		property="og:description"
		content="An advanced AI chatbot powered by GPT-3.5, website made by DuckyBlender"
	/>
	<meta property="og:image" content="https://duckys3.s3.eu-central-1.amazonaws.com/embed.webp" />
	<meta property="og:url" content="https://chatgpt.ducky.pics" />
	<meta property="og:site_name" content="DuckyBlender" />
	<meta property="og:type" content="website" />
	<meta property="og:locale" content="en_US" />
</svelte:head>

<!-- Center the content -->
<div class="flex h-full items-center justify-center p-4">
	<!-- Set the width of the content -->
	<div class="w-full">
		<!-- Header -->
		<div
			class="mb-4 flex flex-row justify-between overflow-auto rounded-lg bg-gradient-to-bl from-slate-100 to-slate-200 p-4 text-slate-800 shadow-xl  dark:from-slate-700 dark:to-slate-600 dark:text-slate-100"
		>
			<div class="text-left ">
				<!-- Title -->
				<h1 class="text-2xl font-bold">ChatGPT</h1>
				<!-- Subtitle -->

				<p class="text-sm ">An advanced AI chatbot powered by GPT-3.5</p>
			</div>
			<div class="text-right">
				<h1 class="text-md mb-1 font-light">Website by DuckyBlender</h1>
				<!-- To get the exact number of commits we need to use the GitHub API -->
				<!-- https://api.github.com/repos/DuckyBlender/chatgpt-clone/commits -->
				<p class="text-sm font-light">
					<a href="https://github.com/DuckyBlender/chatgpt-clone" class="text-blue-400">
						{commitCount}
					</a>
					|
					<a href="/changelog" class="text-blue-400">Changelog</a>
				</p>
			</div>
		</div>

		<!-- Main content -->
		<div
			class="overflow-y-auto rounded-lg bg-gradient-to-bl from-slate-100 to-slate-200 p-4 text-slate-900 shadow-md dark:from-slate-700 dark:to-slate-600 dark:text-slate-100"
		>
			<!-- Set the content -->
			<slot />
		</div>
	</div>
</div>
