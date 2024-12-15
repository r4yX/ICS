
<template>
  <div id="blur" :class="{ hidden: !isBudget }">
  <div id="create-budget" :class="{ hidden: !isBudget }">
    <button @click="toggleBudget()" id="cancel" title="Cancelar"><i class="fa-solid fa-xmark"></i></button>
    <form id="budget-form">
			<label for="name">Nombre</label>
			<label for="phone">Telefono</label>
			<label for="cuit">CUIT</label>
			<label for="tipo">Tipo</label>
			<label for="iva" title="%">IVA</label>
			<label for="save-item"/>

			<input id="name" v-model="price" placeholder="0.0"></input>
			<input id="phone" v-model="cant" placeholder="0"></input>
			<input id="cuit" v-model="iva" placeholder="21" pattern="\d*"></input>
			<VueSelect class="vue-select"
				item="item"
				v-model="item"
				:options="[{label: 'Cambio de Aceite', value: 'oil'}]"
				placeholder="Buscar producto o servicio"
			/>
			<button id="save-item" title="Añadir item" @click="addDetail" type="button"><i class="fa-solid fa-add"></i></button> 
    </form>
    <button id="confirm" title="Crear Presupuesto" @click="createBudget" type="button"><i class="fa-solid fa-check"></i></button>
  </div>
	</div>
</template>

<script>
import { ref } from 'vue';                
import VueSelect from "vue3-select-component";
import Item from "../components/Item.vue";

const customer = ref("");
const vehicle = ref("");
const concept = ref("");
const kilometrage = ref("");
const details = ref([]);

const item = ref("");
const price = ref(0);
const cant = ref(0);
const tipo = ref("");
const iva = ref(0);

export default {
	methods: {
		toggleBudget() {
			const userConfirmed = confirm("¿Seguro de cerrar? Los cambios no se guardaran")
			if (!userConfirmed) {return 0}
			this.$emit('destroy');
		}
	},
	name: 'CreateBudget',
	components: {
		VueSelect,
		Item,
	},
	setup() {
    const isBudget = ref(true);
		const createBudget = () => {}

		const delDetail = (index) => {details.value.splice(index, 1);};

		const addDetail = () => {
			let stotal = price.value*cant.value;
			let ivaPrice = stotal*iva.value/100
			let total = stotal+ivaPrice;
			details.value.push({
				item: item.value,
				price: price.value,
				cant: cant.value,
				tipo: tipo.value,
				stotal,
				iva: ivaPrice,
				total});
		};
    // -- Return
		return {
			isBudget,
			// Input vars
			customer,
			vehicle,
			concept,
			kilometrage,
			details,
			// Inside detail
			item, price, cant, tipo, iva,
			// Funtions
			addDetail,
			delDetail,
			createBudget,
		};
	},
};
</script>

<style>
/* --  Create budget box  -- */
#blur {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background-color: #0005;
}
#blur.hidden {
  display: none;
}
#create-budget {
  display: flex;
  position: absolute;
  align-items: center;
  justify-content: center;
  width: 90%;
  height: 60%;
  min-width: 315px;
  overflow-y: hidden;
  overflow-x: hidden;
  color: #ddd;
  top: 20%;
  left: 5%;
  display: flex;
  background-color: #202020;
  border: 2px solid #668076;
  border-radius: 9px;
}
#create-budget > button {
  position: absolute;
  display: flex;
  cursor: pointer;
  height: 30px;
  width: 30px;
  border: none;
  outline: none;
  border-radius: 50%;
  color: white;
  align-items: center;
  justify-content: center;
}
#cancel {
  top: .6rem;
  left: .6rem;
  background: #800;
}
#confirm {
  bottom: .6rem;
  right: .6rem;
  background: #080;
}
#budget-number {
  margin: 0px;
  position: absolute;
  top: .6rem;
  right: .6rem;
}
#create-budget.hidden {
  display: none;
}
#create-budget.test {
  background-color: red;
}
#budget-form {
	height: 90%;
  width: 90%;
  display: flex;
  flex-direction: column;
  align-items: center;
	overflow-y: scroll;
	overflow-x: scroll;	
}
#budget-form input {
	background: #333;
	border: none;
	border-radius: .4rem;
	padding: 6px 0.5rem;
	font-size: 18px;
	font-weight: 400;
}
#budget-form input::placeholder {
	color: #52525b;
}
.rows {
	margin-top: 2rem;
	width: 95%;
  display: flex;
	justify-content: space-between;
}
.in-row {
	display: flex;
	width: 40%;
	flex-direction: column;
	align-items: space-between;
}
.in-row > label {
  margin: 0 0 .4rem 0;
}
/* --  Table  -- */
#table {
	margin-top: 2rem;
	display: grid;
	gap: 10px;
	grid-template-columns: 15rem auto 3rem 12rem auto 1.8rem;
}
#cant,
#iva
{width: 2rem;}
#price, #stotal {width: 4.2rem;}
#item,
#tipo,
{width: 9rem;}
#save-item {
	cursor: pointer;
	border: none;
	height: 2rem;
	width: 2rem;
	background-color: #3aa;
	border-radius: .3rem;
}
#details {
	width: 100%;
	display: grid;
	gap: 10px;
	grid-template-columns: auto auto auto 110px 130px auto auto auto;
	justify-content: start;
	align-items: center;
}
#details > * {
	text-decoration: underline;
	padding: 0 12px;
}
/* --  Custom Select  -- */
:deep(.vue-select) {
  --vs-input-bg: #333;
  --vs-border: 1px solid #eee0;
  --vs-border-radius: .4rem;
}
</style>
