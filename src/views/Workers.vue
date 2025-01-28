<template>
  <div id="main">
		<div id="header">
			<h2>Trabajadores</h2>
			<button @click="toggleWorker()" title="Añadir Trabajador"><svg-icon type="mdi" :path="mdiPlus" /></button>
		</div>
		<component :is="workerComponent" @destroy="workerComponent = null" @refresh-workers="updateWorkers"/>
		<ul>
			<Worker
					v-for="(worker, index) in workers"
					:key="index"
					:data="worker"
					:index="index"
			/>
		</ul>
  </div>
</template>

<script>
import { ref, onMounted, shallowRef } from 'vue';
import { invoke } from "@tauri-apps/api/core";
import SvgIcon from '@jamescoyle/vue-icon';
import { mdiPlus } from '@mdi/js';
import CreateWorker from "@components/CreateWorker.vue";
import Worker from "@components/Worker.vue";

export default {
  name: 'Workers',
  components: {
    SvgIcon,
		CreateWorker,
		Worker
  },
	setup() {
		const workerComponent = shallowRef(null);
		const workers = ref([]);

		const toggleWorker = () => {
			if (!workerComponent.value) {
        workerComponent.value = CreateWorker;
      } else {
        workerComponent.value = null;
      }
		};
		const updateWorkers = async() => {
			let log = await invoke('obtain_workers')
			workers.value = log
		}
		onMounted(updateWorkers)
		return {
			workers,
			updateWorkers,
			mdiPlus,
			toggleWorker,
			workerComponent
		}
	},
};
</script>

<style scoped>
#main {
  display: flex;
  flex-direction: column;
	background: #222;
	color: #ddd;
	overflow-y: scroll;
}
#header {
  display: flex;
  justify-content: space-between;
}
#header > h2 {
  margin-left: 2rem;
}
#header > button {
	display: flex;
	align-items: center;
	justify-content: center;
  margin: .4rem;
  height: 2rem;
  width:  2rem;
  outline: none;
  border: none;
  border-radius: .4rem;
	cursor: pointer;
  background: #24c8db20;
	transition: background .3s;
}
#header > button:hover {
  background: #44e8fb80;
}
</style>
