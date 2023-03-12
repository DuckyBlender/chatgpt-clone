<script>
	import '../app.css';
</script>

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
				<!-- Title -->
				<h1 class="text-md mb-1 font-light">Website by DuckyBlender</h1>
				<!-- Subtitle -->
				<p class="text-sm font-light ">Public Beta 1.1</p>
			</div>
		</div>

		<!-- Main content -->
		<div
			class="overflow-y-auto rounded-lg bg-gradient-to-bl from-slate-100 to-slate-200  p-4  text-slate-900 shadow-md dark:from-slate-700 dark:to-slate-600 dark:text-slate-100"
		>
			<!-- Set the content -->
			<slot />
		</div>
	</div>
</div>
