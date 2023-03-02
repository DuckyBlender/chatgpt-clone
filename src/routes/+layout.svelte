<script>
	import '../app.css';
</script>

<div class="bg-gradient-to-l from-blue-600 to-blue-900 min-h-screen">
	<!-- Center the content -->
	<div class="flex justify-center items-center h-full p-8">
		<!-- Set the width of the content -->
		<div class="w-11/12">
			<!-- Set the background color and if the content is too high, make it scrollable-->
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
