<template>
  <div id="blur" :class="{ hidden: !isBudget }">
  <div id="create-budget" :class="{ hidden: !isBudget }">
		<button @click="toggleBudget()" id="cancel" title="Cancelar"><svg-icon type="mdi" :path="mdiClose" /></button>
    <h2 id="budget-number">N° 0001-0000001</h2>
		<form id="budget-form">
			<div class="rows">
				<div class="in-row">
					<label for="customer">Cliente</label>
					<VueSelect class="vue-select"
						id="customer"
						v-model="customer"
						:options="[
							{ label: 'A Wizard of Earthsea', value: 'wizard_earthsea' },
							{ label: 'The Lord of the Rings', value: 'the_lord_of_the_rings' },
						]" placeholder="Nombre Apellido"/>
				</div>
				<div class="in-row">
					<label for="vehicle">Vehiculo</label>
					<VueSelect class="vue-select"
						id="vehicle"
						v-model="vehicle"
						:options="[
							{ label: 'A Wizard of Earthsea', value: 'wizard_earthsea' },
							{ label: 'The Lord of the Rings', value: 'the_lord_of_the_rings' },
						]" placeholder="ABC123"/>
				</div>
			</div>
			<div class="rows">
				<div class="in-row">
					<label for="concept">Concepto</label>
					<VueSelect class="vue-select"
						id="concept"
						v-model="concept"
						:options="[
							{ label: 'Reparación', value: 'reparacion' },
							{ label: 'Service', value: 'service' },
							{ label: 'Mantenimiento', value: 'mantenimiento' },
							{ label: 'Revisión', value: 'revision' },
							{ label: 'Garantia', value: 'garantia' },
							{ label: 'Otros', value: 'otros' },
						]" placeholder="Reparación"/>
				</div>
				<div class="in-row">
					<label id="kilometrage">Kilometraje</label>
					<input id="kilometrage" type="text" pattern="\d*" placeholder="Km">
				</div>
			</div>
      <div id="table">
				<label for="item">Item</label>
				<label for="price">Importe</label>
				<label for="cant">Cant.</label>
				<label for="tipo">Tipo</label>
				<label for="iva" title="%">IVA</label>
				<label for="save-item"/>
				<VueSelect class="vue-select"
					item="item"
					v-model="item"
					:options="[{label: 'Cambio de Aceite', value: 'oil'}]"
					placeholder="Buscar producto o servicio"
				/>
				<input id="price" v-model="price" placeholder="0.0"></input>
				<input id="cant" v-model="cant" placeholder="0"></input>
				<VueSelect class="vue-select"
					id="tipo"
					v-model="tipo"
					:options="[{label: 'Producto', value: 'prudoduct'},
						{label: 'Servicio', value: 'service'}]"
					placeholder="Producto o Servicio"
				/>
				<input id="iva" v-model="iva" placeholder="21" pattern="\d*"></input>
				<button id="save-item" title="Añadir item" @click="addDetail" type="button"><svg-icon type="mdi" :path="mdiPlus" /></button> 
      </div>
		<div id="details">
			<p>Item</p>
			<p>Precio</p>
			<p>Cant</p>
			<p>Tipo</p>
			<p>Subtotal</p>
			<p>IVA</p>
			<p>Total</p>
		</div>
		<Item
        v-for="(detail, index) in details"
        :key="index"
        :detail="detail"
        :index="index"
        :delDetail="delDetail"
      />
    </form>
		<button id="confirm" title="Crear Presupuesto" @click="createBudget" type="button"><svg-icon type="mdi" :path="mdiCheck"/></button>
  </div>
	</div>
</template>

<script>
import { ref } from 'vue';                
import VueSelect from "vue3-select-component";
import SvgIcon from '@jamescoyle/vue-icon';
import { mdiClose, mdiPlus, mdiCheck } from '@mdi/js';
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
		SvgIcon,
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

		addEventListener('keydown', (e) => {
			if (e.keyCode == 27) {
				let cancelBtn = document.querySelector('#cancel')
				cancelBtn.click()
			}
		})
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
			// Functions
			addDetail,
			delDetail,
			createBudget,
			// Icons
			mdiClose,
			mdiPlus,
			mdiCheck
		};
	},
};
</script>

<style scoped>
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
  border: 2px solid #444;
  border-radius: 9px;
}
#create-budget > button {
  position: absolute;
  display: flex;
  cursor: pointer;
  height: 2rem;
  width: 2rem;
  border: 1px solid #999;
  outline: none;
  border-radius: .4rem;
  background: #333;
  color: white;
  align-items: center;
  justify-content: center;
	transition: background .2s;
}
#cancel {
  top: .6rem;
  left: .6rem;
}
#confirm {
  bottom: .6rem;
  right: .6rem;
}
#cancel:hover {
	background: #543c3c;
}
#confirm:hover {
	background: #434f3b;
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
	height: 78%;
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
	width: 95%;
	margin: 0;
	margin-top: 2rem;
	align-items: center;
	display: grid;
	gap: 10px;
	grid-template-columns: 15rem 5rem 3rem 12rem 3rem 1.8rem;
}
#price, #stotal {width: 4.2rem;}

#cant,
#iva
{width: 2rem;}

#item,
#tipo,
{width: 9rem;}

#save-item {
	display: flex;
	align-items: center;
	justify-content: center;
	cursor: pointer;
	height: 2rem;
	width: 2rem;
	background: #333;
	border: 1px solid #999;
	border-radius: .4rem;
	transition: background .2s;
}
#save-item:hover {
	background: #088;
}
#details {
	width: 98%;
	display: grid;
	gap: 10px;
	grid-template-columns: auto auto auto 110px 130px auto auto auto;
	justify-content: start;
	align-items: center;
}
#details > * {
	text-decoration: underline;
	padding: 0 12px;
	margin-bottom: 8px;
}
/* --  Custom Select  -- */
:deep(.vue-select) {
  --vs-input-bg: #333;
  --vs-border: 1px solid #eee0;
  --vs-border-radius: .4rem;
}
</style>
