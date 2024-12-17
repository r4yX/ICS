<template>
  <div id="blur">
		<div id="create-vehicle">
			<button @click="toggleVehicle()" id="cancel" title="Cancelar"><svg-icon type="mdi" :path="mdiClose" /></button>
    <form id="vehicle-form">
				<div class="cols">
					<label for="plate">Patente *</label>
					<input id="plate" placeholder="AAA000" />
				</div>
				<div class="cols">
					<label for="tipo">Tipo</label>
					<VueSelect class="vue-select"
						id="tipo"
						v-model="tipo"
						:options="[
							{ label: 'Sedan', value: 'sedan' },
							{ label: 'Coupe', value: 'coupe' },
						]" placeholder="Sedan"/>
				</div>
				<div class="cols">
					<label for="maker">Fabricante *</label>
					<VueSelect class="vue-select"
						id="maker"
						v-model="maker"
						:options="[
							{ label: 'Chevrolet', value: 'chevrolet' },
							{ label: 'BMW', value: 'bmw' },
						]" placeholder="Chevrolet"/>
				</div>
				<div class="cols">
					<label id="color">Color</label>
					<input id="color" type="text" placeholder="Rojo">
				</div>
				<div class="cols">
					<label for="model">Modelo</label>
					<input id="model" placeholder="Astra" /> 
				</div>
				<div class="cols">
					<label id="year">Año</label>
					<input id="year" type="text" placeholder="1999">
				</div>
    </form>
		<button id="confirm" title="Añadir Vehiculo" @click="addVehicle" type="button"><svg-icon type="mdi" :path="mdiCheck" /></button>
		</div>
		</div>
</template>

<script>
import { ref } from 'vue';                
import VueSelect from "vue3-select-component";
import SvgIcon from "@jamescoyle/vue-icon";
import { mdiClose, mdiCheck } from "@mdi/js"

const plate = ref("");
const maker = ref("");
const model = ref("");
const tipo = ref("");
const color = ref("");
const year = ref("");

export default {
	methods: {
		toggleVehicle() {
			const userConfirmed = confirm("¿Seguro de cerrar? Los cambios no se guardaran")
			if (!userConfirmed) {return 0}
			this.$emit('destroy');
		}
	},
	name: 'CreateVehicle',
	components: {
		VueSelect,
		SvgIcon,
	},
	setup() {
    const isVehicle = ref(true);

		const addVehicle = () => {}
    // -- Return
		return {
			isVehicle,
			// Input vars
			plate,
			maker,
			model,
			tipo,
			color,
			year,
			// Functions
			addVehicle,
			// Icons
			mdiClose,
			mdiCheck,
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
#create-vehicle {
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
  background-color: #202020;
  border: 2px solid #668076;
  border-radius: 9px;
}
#create-vehicle > button {
  position: absolute;
  display: flex;
  cursor: pointer;
  height: 30px;
  width: 30px;
  border: none;
  outline: none;
  border-radius: .4rem;
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
#vehicle-form {
	height: 90%;
	margin: auto;
  width: 90%;
	display: grid;
	grid-template-columns: 1fr 1fr;
	justify-items: center;
	overflow-y: scroll;
	overflow-x: scroll;	
}
#vehicle-form input {
	background: #333;
	border: none;
	border-radius: .4rem;
	padding: 6px 0.5rem;
	font-size: 18px;
	font-weight: 400;
}
#vehicle-form input::placeholder {
	color: #52525b;
}
#vehicle-form input:focus {
	outline: 2px solid #777;
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
/* --  Custom Select  -- */
:deep(.vue-select) {
  --vs-input-bg: #333;
  --vs-border: 1px solid #eee0;
  --vs-border-radius: .4rem;
	--vs-input-outline: #777;
}
</style>
