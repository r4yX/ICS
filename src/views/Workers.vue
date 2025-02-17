<template>
  <div id="main">
		<div id="header">
			<h2>Trabajadores</h2>
			<button @click="toggleWorker()" title="Añadir Trabajador"><svg-icon type="mdi" :path="mdiPlus" /></button>
		</div>
		<component
			:is="workerComponent"
			@destroy="workerComponent = null"
			@refresh-workers="updateWorkers"/>
		<ul>
			<Worker
				v-for="(worker, index) in workers"
				:key="index"
				:data="worker"
				@selectWorker="selectWorker"
				@refresh-workers="updateWorkers" />
		</ul>
		<p v-if="selectedWorker != null">Pagos de {{ selectedWorker.name }}:</p>
		<ul id="pays">
			<Payment
					v-if="selectedWorker"
					v-for="(payment, index) in payments"
					:key="index"
					:data="payment"
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
import Payment from "@components/Payment.vue";

export default {
  name: 'Workers',
  components: {
    SvgIcon,
		CreateWorker,
		Payment,
		Worker
  },
	setup() {
		const selectedWorker = ref(null);
		const workerComponent = shallowRef(null);
		const workers = ref([]);
		const payments = ref([]);

		const toggleWorker = () => {
			if (!workerComponent.value) {
        workerComponent.value = CreateWorker;
      } else {
        workerComponent.value = null;
      }
		};

		const selectWorker = (data) => {
			selectedWorker.value = data
			updatePayments(data)
		}

		const updateWorkers = async() => {
			let log = await invoke('obtain_workers')
			workers.value = log
		}
		const updatePayments = async(data) => {
			if (!data) {return 0}
			let log = await invoke('obtain_payments', {'dni':data.dni})
			payments.value = log
		}
		onMounted(updateWorkers)
		return {
			toggleWorker,
			workerComponent,
			workers,
			updateWorkers,
			selectWorker,
			selectedWorker,
			payments,
			mdiPlus
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
#main > p {
	margin: 0 0 0 2rem;
}
#pays {
	display: grid;
	grid-template-columns: 1fr 1fr; 
	width: 90%;
	overflow-y: scroll;
	overflow-x: scroll;
}
@media (min-width: 800px) {
	#pays { grid-template-columns: repeat(3, 1fr) }
}
@media (min-width: 1000px) {
	#pays { grid-template-columns: repeat(4, 1fr) }
}
@media (min-width: 1300px) {
	#pays { grid-template-columns: repeat(5, 1fr) }
}

</style>
