<template>
  <div id="main">
    <div id="header">
      <h2>Presupuestos</h2>
			<button @click="toggleBudget()" title="Crear Presupuesto"><svg-icon type="mdi" :path="mdiPlus" /></button>
    </div>
		<component :is="budgetComponent" @destroy="budgetComponent = null"/>
    <ul>
      <p>Presupuestos</p>
    </ul>
  </div>
</template>

<script>                                           
import { ref } from 'vue';                
import CreateBudget from "../components/CreateBudget.vue";
import SvgIcon from '@jamescoyle/vue-icon';
import { mdiPlus } from "@mdi/js";

export default {
	name: 'Budgets',
	components: {CreateBudget, SvgIcon},
	setup() {
		const budgetComponent = ref(null);

		const toggleBudget = () => {
			if (!budgetComponent.value) {
        budgetComponent.value = CreateBudget;
      } else {
        budgetComponent.value = null;
      }
		};
	return {
		toggleBudget,
		budgetComponent,
		mdiPlus,
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
