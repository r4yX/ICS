<template>
  <div id="main">
    <div id="header">
      <h2>Balance</h2>
    </div>
		<Calendar id="calendar" @selectDay="updateSelectedDay"/>
		<div id="day-movements">
			<p>Pagos de: {{ selectedDay }} </p>
		</div>	
  </div>
</template>

<script>                                           
import { ref } from 'vue';
import SvgIcon from '@jamescoyle/vue-icon';
import VueSelect from "vue3-select-component";
import Calendar from "@components/Calendar.vue";

export default {
	name: 'Balance',
	components: {VueSelect, SvgIcon, Calendar},
	setup() {
		let selectedDay = ref("Lunes");
		const year = ref(new Date().getFullYear());
		const month = ref(new Date().getMonth());

		const updateSelectedDay = (day) => {
			selectedDay.value = day
		}

		return {
			years: Array.from({ length: 10 }, (_, i) => new Date().getFullYear() - 5 + i),
			months: ["Enero", "Febrero", "Marzo", "Abril", "Mayo", "Junio", "Julio", "Agosto", "Septiembre", "Octubre", "Noviembre", "Diciembre"],
			year,
			month,
			selectedDay,
			updateSelectedDay,
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
/* --  Custom Select  -- */
:deep(.vue-select) {
  --vs-input-bg: #333;
  --vs-border: 1px solid #eee0;
  --vs-border-radius: .4rem;
	--vs-input-outline: #777;
}
</style>
