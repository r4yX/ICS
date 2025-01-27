<template>
  <div id="main">
    <div id="header">
      <h2>Presupuestos</h2>
			<button @click="toggleBudget()" title="Crear Presupuesto"><svg-icon type="mdi" :path="mdiPlus" /></button>
    </div>
		<component :is="budgetComponent" @destroy="budgetComponent = null" @refresh-budgets="updateBudgets"/>
    <ul>
		<Budget
        v-for="(budget, index) in budgets"
        :key="index"
        :data="budget"
        :index="index"
				@refresh-budgets="updateBudgets"
				/>
    </ul>
  </div>
</template>

<script>                                           
import { ref, onMounted, shallowRef } from 'vue';                
import { invoke } from "@tauri-apps/api/core";
import CreateBudget from "@components/CreateBudget.vue";
import Budget from "@components/Budget.vue"
import SvgIcon from '@jamescoyle/vue-icon';
import { mdiPlus } from "@mdi/js";

export default {
	name: 'Budgets',
	components: {CreateBudget, Budget, SvgIcon},
	setup() {
		const budgets = ref([]);
		const budgetComponent = shallowRef(null);

		const toggleBudget = () => {
			if (!budgetComponent.value) {
        budgetComponent.value = CreateBudget;
      } else {
        budgetComponent.value = null;
      }
		};
		const updateBudgets = async() => {
      try {
        const result = await invoke('obtain_budgets');
        budgets.value = result.map((budget) => ({
					id: budget.id,
          customer: budget.customer,
          vehicle: budget.vehicle,
          concept: budget.concept,
          kilometrage: budget.kilometrage,
          total: budget.total,
					paid: "none",
					pay_date: "none",
        }));
      } catch (error) {
        console.error('Error al cargar presupuestos:', error);
      }
    }
    onMounted(updateBudgets);
		return {
			toggleBudget,
			budgetComponent,
			mdiPlus,
			budgets,
			updateBudgets
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
