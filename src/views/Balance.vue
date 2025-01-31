<template>
  <div id="main">
    <div id="header">
      <h2>Balance</h2>
    </div>
		<Calendar id="calendar" @selectDay="updateDayMovements"/>
		<p v-if="selectedDay != null">Movimientos de {{ selectedDay }}</p>
		<ul id="day-movements">
			<Movement 
        v-for="(movement, index) in movements"
        :key="index"
        :data="movement"
        :index="index"
			/>
		</ul>	
  </div>
</template>

<script>                                           
import { ref, onMounted } from 'vue';
import { invoke } from "@tauri-apps/api/core";
import SvgIcon from '@jamescoyle/vue-icon';
import VueSelect from "vue3-select-component";
import Calendar from "@components/Calendar.vue";
import Movement from "@components/Movement.vue";

export default {
	name: 'Balance',
	components: {VueSelect, SvgIcon, Calendar, Movement},
	setup() {
		const selectedDay = ref(null);
		const movements = ref([]);
		const year = ref(new Date().getFullYear());
		const month = ref(new Date().getMonth());

		const updateDayMovements = async(data) => {
			if (data == undefined) {return 0}
			let log = await invoke('obtain_balance', {'date': data})
			selectedDay.value = data
			movements.value = log
		}
		onMounted(updateDayMovements)
		return {
			selectedDay,
			updateDayMovements,
			movements,
			years: Array.from({ length: 10 }, (_, i) => new Date().getFullYear() - 5 + i),
			months: ["Enero", "Febrero", "Marzo", "Abril", "Mayo", "Junio", "Julio", "Agosto", "Septiembre", "Octubre", "Noviembre", "Diciembre"],
			year,
			month,
		};
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
	align-items: center;
}
#header > h2 {
  margin-left: 2rem;
}
#main > p {
	margin: 0 0 0 2rem;
}
/* --  Custom Select  -- */
:deep(.vue-select) {
  --vs-input-bg: #333;
  --vs-border: 1px solid #eee0;
  --vs-border-radius: .4rem;
	--vs-input-outline: #777;
}
</style>
