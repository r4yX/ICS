<template>
  <div id="blur">
		<div id="create-customer">
			<button @click="toggleCustomer()" id="cancel" title="Salir"><svg-icon type="mdi" :path="mdiClose" /></button>
    <form id="customer-form">
			<div class="rows">
				<div class="cols">
					<label for="customer">Nombre *</label>
					<input v-model="customer" id="customer" placeholder="Jhon Doe" />
				</div>
				<div class="cols">
					<label for="phone">Telefono *</label>
					<input v-model="phone" id="phone" placeholder="341-012345" /> 
				</div>
			</div>
			<div class="rows">
				<div class="cols">
					<label for="tipo">Tipo de cuenta *</label>
					<VueSelect class="vue-select"
						id="tipo"
						v-model="tipo"
						:options="[
							{ label: 'Particular', value: 'particular' },
							{ label: 'Corriente', value: 'current' },
						]" placeholder="Partiuclar"/>
				</div>
				<div class="cols">
					<label id="cuit">CUIT / CUIL *</label>
					<input v-model="cuit" id="cuit" type="text" placeholder="00-12346789-0">
				</div>
			</div>
			<div class="rows">
				<div class="cols">
					<label for="vehicle">Vehiculo</label>
					<div id="vehicle-row">
						<VueSelect class="vue-select"
							id="vehicle"
							v-model="vehicle"
							:options="domains"
							placeholder="AAA000"/>
						<button type="button" @click="addCar" :disabled="vehicle == '' || vehicle == null"><svg-icon type="mdi" :path="mdiPlus" /></button>
					</div>
					<div class="item" v-for="(plate, index) in vehicles">
						<p title="Patente">{{ plate }}</p>
						<button id="del-btn" @click="delCar(index)" type="button">
							<svg-icon type="mdi" :path="mdiDelete"></svg-icon>
						</button>
					</div>
				</div>
				<div class="cols">
					<label id="dni">DNI</label>
					<input v-model="dni" id="dni" type="text" placeholder="123456789">
				</div>
			</div>
    </form>
		<button id="confirm" title="Añadir cuenta" @click="addCustomer" type="button"><svg-icon type="mdi" :path="mdiCheck" /></button>
		</div>
		</div>
</template>

<script>
import { ref, onMounted } from 'vue';                
import { invoke } from "@tauri-apps/api/core";
import VueSelect from "vue3-select-component";
import SvgIcon from "@jamescoyle/vue-icon";
import { mdiClose, mdiPlus, mdiCheck, mdiDelete } from "@mdi/js"

const customer = ref("");
const phone = ref("");
const tipo = ref("");
const cuit = ref("");
const vehicle = ref("");
const dni = ref("");
const vehicles = ref([]); 
const domains = ref([]);

export default {
	methods: {
		toggleCustomer() {
			const userConfirmed = confirm("¿Seguro de cerrar? Los cambios no se guardaran")
			if (!userConfirmed) {return 0}
			this.$emit('destroy');
		}
	},
	name: 'CreateCustomer',
	components: {
		VueSelect,
		SvgIcon,
	},
	setup() {
		const updateCars = async() => {
			let res = await invoke('obtain_vehicles')
			for (let r in res) {
				domains.value.push({"label": res[r].domain, "value":res[r].domain})
			}
		}
    const isCustomer = ref(true);

		const delCar = (index) => {vehicles.value.splice(index, 1);};

		const addCar = () => {
			vehicles.value.push(vehicle.value)
		};

		const addCustomer = async() => {
			let log = await invoke('create_customer', {'name': customer.value, 'phone': phone.value,
			'cuit': cuit.value, 'dni': dni.value, 'tipo': tipo.value, 'vehicles': vehicles.value})
			alert(log)
		}
		onMounted(updateCars)
    // -- Return
		return {
			isCustomer,
			domains,
			// Input vars
			customer,
			phone,
			tipo,
			cuit,
			vehicle,
			vehicles,
			dni,
			// Functions
			addCar,
			delCar,
			addCustomer,
			updateCars,
			// Icons
			mdiClose,	mdiPlus, mdiCheck,mdiDelete
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
	z-index: 90;
}
#create-customer {
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
	z-index: 100;
}
#create-customer > button {
  position: absolute;
  display: flex;
  cursor: pointer;
	height: 2rem;
	width: 2rem;
  outline: none;
	background: #333;
  border: 1px solid #999;
  border-radius: .4rem;
  color: white;
  align-items: center;
  justify-content: center;
}
#cancel {
  top: .6rem;
  left: .6rem;
	transition: background .2s;
}
#confirm {
  bottom: .6rem;
  right: .6rem;
	transition: background .2s;
}
#cancel:hover {
	background: #543c3c;
}
#confirm:hover {
	background: #434f3b;
}
#customer-form {
	height: 90%;
  width: 90%;
	display: flex;
	flex-direction: column;
  align-items: center;
	overflow-y: scroll;
	overflow-x: scroll;	
}
#customer-form input {
	background: #333;
	border: none;
	border-radius: .4rem;
	padding: 6px 0.5rem;
	font-size: 18px;
	font-weight: 400;
}
#customer-form input::placeholder {
	color: #52525b;
}
#customer-form input:focus {
	outline: 2px solid #777;
}
.rows {
	margin-top: 2rem;
	width: 95%;
  display: flex;
	justify-content: space-between;
}
.cols {
	display: flex;
	width: 40%;
	flex-direction: column;
	align-items: space-between;
}
.cols > label {
  margin: 0 0 .4rem 0;
}
#vehicle-row {
	display: flex;
	align-items: center;
}
#vehicle-row > button {
	display: flex;
	align-items: center;
	justify-content: center;
	height: 2rem;
	width: 2rem;
	margin-left: .5rem;
	background: #333;
	color: white;
	border: 1px solid #999;
	border-radius: .4rem;
	transition: background .3s;
}
#vehicle-row > button:disabled {
	border: none;
	color: #777;
	cursor: not-allowed;
}
#vehicle-row > button:not([disabled]):hover {
	cursor: pointer;
  background: #3d5859;
}
/* -- Vehilce plate item -- */
.item {
	width: 100%;
	display: grid;
	gap: 10px;
	grid-template-columns: 64px 74px 63px 100px 130px 64px 54px auto;
	justify-content: start;
	align-items: center;
}
.item > * {
	padding: 0 10px;
}
#del-btn {
	padding: 2px 6px 3px 6px;
	display: flex;
	align-items: center;
	justify-content: center;
	height: 2rem;
	width: 2rem;
	background: #333;
	cursor: pointer;
	border: 1px solid #999;
	border-radius: .4rem;
	color: white;
	margin-left: 1.4rem;
	transition: background .2s;
}
#del-btn:hover {
	background: #543c3c;
}
/* --  Custom Select  -- */
:deep(.vue-select) {
  --vs-input-bg: #333;
  --vs-border: 1px solid #eee0;
  --vs-border-radius: .4rem;
	--vs-input-outline: #777;
}
</style>
