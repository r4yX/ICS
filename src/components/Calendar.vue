<template>
	<div id="calendar">
		<div class="controls">
			<VueSelect 
				 v-model="selectedYear"
				 @update:modelValue="updateDays"
				 class="vue-select"
				 :options="years.map(year => ({ label: year, value: year }))"
				 placeholder="Año"
			/>
			<VueSelect 
				v-model="selectedMonth" 
				@update:modelValue="updateDays"
				class="vue-select"
				:options="months.map((month, i) => ({ label: month, value: i }))"
				placeholder="Mes"
			/>
		</div>
		<div class="month">
			<div class="week">
				<div v-for="day in weekDays" :key="day" class="day header">{{ day }}</div>
			</div>
			<div class="week" v-for="(week, index) in weeks" :key="index">
				<div
					v-for="day in week"
					:key="day.date"
					:class="['day', { empty: !day.isCurrentMonth }]"
					@click="selectDay(day)"
				> <p>{{ day.isCurrentMonth ? day.date.getDate() : '' }}</p>
				</div>
			</div>
		</div>
	</div>
</template>

<script>
import { ref } from 'vue';                
import VueSelect from "vue3-select-component";

export default {
	components: {
		VueSelect,
	},
  data() {
    return {
      selectedYear: new Date().getFullYear(),
      selectedMonth: new Date().getMonth(),
      years: Array.from({ length: 10 }, (_, i) => new Date().getFullYear() - 5 + i),
      months: ["Enero", "Febrero", "Marzo", "Abril", "Mayo", "Junio", "Julio", "Agosto", "Septiembre", "Octubre", "Noviembre", "Diciembre"],
      weekDays: ["L", "M", "M", "J", "V", "S", "D"],
      weeks: [],
    };
  },
  methods: {
    updateDays() {
			// Prevent undefined year or month
			if (typeof this.selectedYear == "undefined") {return 0}
			if (typeof this.selectedMonth == "undefined") {return 0}

      const firstDay = new Date(this.selectedYear, this.selectedMonth, 1);
      const lastDay = new Date(this.selectedYear, this.selectedMonth + 1, 0);

      const days = [];
      let currentDay = new Date(firstDay);
      currentDay.setDate(currentDay.getDate() - (currentDay.getDay() || 7) + 1);

      while (currentDay <= lastDay || currentDay.getDay() !== 1) {
        days.push({
          date: new Date(currentDay),
          isCurrentMonth: currentDay.getMonth() === this.selectedMonth,
        });
        currentDay.setDate(currentDay.getDate() + 1);
      }

      this.weeks = [];
      for (let i = 0; i < days.length; i += 7) {
        this.weeks.push(days.slice(i, i + 7));
      }
    },
    selectDay(day) {
      if (day.isCurrentMonth) {
			let approve_date = day.date.toLocaleDateString('en-GB', {
				month: '2-digit', day: '2-digit', year: 'numeric'});
				this.$emit("selectDay", approve_date);
      }
    },
  },
  mounted() {
    this.updateDays();
  },
};
</script>

<style scoped>
#calendar {
	display: flex;
	flex-direction: column;
	justify-content: center;
	margin: 0px auto;
	width: 90%;
	max-width: 28rem;
}
.controls {
  display: flex;
  gap: 1rem;
  margin-bottom: 1rem;
}
.month {
	margin: 0 auto;
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}
.week {
  display: flex;
	gap: 0.7rem;
}
.day {
  width: 2rem;
  height: 2rem;
	display: flex;
	align-items: center;
	justify-content: center;
  cursor: pointer;
	background: #333;
  border: 1px solid #ccc;
  margin: 0.2rem;
	border-radius: .4rem;
	transition: background .2s;
}
.day.header {
  font-weight: bold;
  background-color: #465d5e;
	cursor: default;
}
.day.empty {
  background-color: #222;
	border: 1px solid #222;
  cursor: default;
}
.day:not(.empty):not(.header):hover {
  background: #777;
}
</style>
