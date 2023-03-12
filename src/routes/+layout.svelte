<script>
	import '../app.css';
</script>

<!-- Center the content -->
<div class="flex h-full items-center justify-center p-4">
	<!-- Set the width of the content -->
	<div class="w-full">
		<!-- Header -->
		<div
			class="mb-4 flex flex-row justify-between overflow-auto rounded-lg bg-gradient-to-bl from-slate-600 to-slate-700 p-4 shadow-xl"
		>
			<div class="text-left">
				<!-- Title -->
				<h1 class="text-2xl font-bold text-white">ChatGPT</h1>
				<!-- Subtitle -->
				<p class="text-sm text-gray-300">An advanced AI chatbot powered by GPT-3.5</p>
			</div>
			<div class="text-right">
				<!-- Title -->
				<h1 class="text-md mb-1 font-light text-white">Website by DuckyBlender</h1>
				<!-- Subtitle -->
				<p class="text-sm font-light text-gray-300">Public Beta 1.1</p>
			</div>
		</div>

		<!-- Main content -->
		<div
			class="overflow-y-auto rounded-lg bg-gradient-to-bl from-slate-600 to-slate-700 p-4 shadow-md"
		>
			<!-- Set the content -->
			<slot />
		</div>
	</div>
</div>
