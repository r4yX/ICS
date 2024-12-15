<template>
  <div id="main">
    <div id="header">
      <h2>Presupuestos</h2>
      <button @click="toggleBudget()" title="Crear Presupuesto"><i class="fa-solid fa-add"></i></button>
    </div>
		<component :is="budgetComponent" @destroy="budgetComponent = null"/>
    <ul>
      <p>Presupuestos</p>
    </ul>
  </div>
</template>

<script>                                           
import { ref } from 'vue';                
import { useRouter } from 'vue-router';            
import CreateBudget from "../components/CreateBudget.vue";

export default {
	name: 'Budgets',
	components: {CreateBudget},
	setup() {
		const budgetComponent = ref(null);

		const toggleBudget = () => {
			if (!budgetComponent.value) {
        budgetComponent.value = CreateBudget; // Crear el componente
      } else {
        budgetComponent.value = null; // Eliminar el componente
      }
		};
	return {
		toggleBudget,
		budgetComponent,
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
