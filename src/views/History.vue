<template>
  <div id="main">
		<div id="header">
			<h2>Historial</h2>
		</div>
		<Budget
			v-for="(order, index) in history"
			:key="index"
			:data="order"
			:index="index"
			/>
  </div>
</template>

<script>
import { ref, onMounted } from 'vue';
import { invoke } from "@tauri-apps/api/core"
import Budget from "@components/Budget.vue";
import SvgIcon from '@jamescoyle/vue-icon';
import { mdiAccount } from '@mdi/js';

export default {
  name: 'History',
  components: {
    SvgIcon,
		Budget
  },
	setup() {
		const history = ref([]);
		const updateHistory = async() => {
      try {
        const result = await invoke('obtain_history');
        history.value = result.map((entry) => ({
					id: entry.id,
          customer: entry.customer,
          vehicle: entry.vehicle,
          concept: entry.concept,
          kilometrage: entry.kilometrage,
          total: entry.total,
					paid: "none",
					pay_date: entry.pay_date,
        }));
      } catch (error) {
        console.error('Error to fetch history:', error);
      }
    }
    onMounted(updateHistory);
		return {
			history,
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
}
#header {
  display: flex;
  justify-content: space-between;
}
#header > h2 {
  margin-left: 3rem;
}
#header > button {
  margin: .4rem;
  height: 2rem;
  width:  2rem;
  outline: none;
  border: none;
  border-radius: 50%;
  background-color: #3aa;
	cursor: pointer;
}
</style>
