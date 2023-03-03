<script>
	import '../app.css';
</script>

<div class="bg-gradient-to-l from-blue-900 to-red-900 min-h-screen -z-10">
	<!-- Center the content -->
	<div class="flex justify-center items-center h-full p-8">
		<!-- Set the width of the content -->
		<div class="w-11/12">
			<!-- Header -->
			<div
				class="bg-gradient-to-bl from-slate-600 to-slate-700 rounded-lg p-4 shadow-md mb-4 flex flex-row justify-between"
			>
				<div>
					<!-- Title -->
					<h1 class="text-2xl text-white font-bold">ChatGPT</h1>
					<!-- Subtitle -->
					<p class="text-sm text-gray-300">An advanced AI chatbot powered by GPT-3.5</p>
				</div>
				<div class="text-right">
					<!-- Title -->
					<h1 class="text-md text-white font-light">Website by: DuckyBlender</h1>
					<!-- Subtitle -->
					<p class="text-sm font-light text-gray-300">Version 1.0.0</p>
				</div>
			</div>

			<!-- Main content -->
			<div
				class="bg-gradient-to-bl from-slate-600 to-slate-700 rounded-lg p-4 overflow-y-auto shadow-md"
			>
				<!-- Set the content -->
				<slot />
			</div>
		</div>
	</div>
</div>
<!-- slot is a placeholder for the content of the page -->
