<template>
  <div id="blur">
		<div id="create-vehicle">
			<button @click="toggleVehicle()" id="cancel" title="Cancelar"><svg-icon type="mdi" :path="mdiClose" /></button>
    <form id="vehicle-form">
				<div class="cols">
					<label for="plate">Patente *</label>
					<input v-model="plate" id="plate" placeholder="AAA000" />
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
					<label for="color">Color</label>
					<input v-model="colour" id="colour" type="text" placeholder="Rojo">
				</div>
				<div class="cols">
					<label for="model">Modelo</label>
					<input v-model="model" id="model" placeholder="Astra" /> 
				</div>
				<div class="cols">
					<label for="year">Año</label>
					<input v-model="year" id="year" type="text" placeholder="1999">
				</div>
    </form>
		<button id="confirm" title="Añadir Vehiculo" @click="addVehicle" type="button"><svg-icon type="mdi" :path="mdiCheck" /></button>
		</div>
		</div>
</template>

<script>
import { ref } from 'vue';
import { invoke } from "@tauri-apps/api/core";
import VueSelect from "vue3-select-component";
import SvgIcon from "@jamescoyle/vue-icon";
import { mdiClose, mdiCheck } from "@mdi/js"

const plate = ref("");
const maker = ref("");
const model = ref("");
const tipo = ref("");
const colour = ref("");
const year = ref(0);

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
	setup({ emit }) {
    const isVehicle = ref(true);

		const addVehicle = async() => {
			let log = await invoke('create_vehicle', {
			'domain':plate.value, 'maker': maker.value, 'model':model.value, 'tipo':tipo.value,
			'colour':colour.value, 'year':parseInt(year.value), 'owner': ""})
			alert(log)
		}

		return {
			isVehicle,
			// Input vars
			plate,
			maker,
			model,
			tipo,
			colour,
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
  outline: none;
  height: 2rem;
  width: 2rem;
  border: 1px solid #999;
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
