<template>
  <div id="blur">
		<div id="create-worker">
			<button @click="toggleCustomer()" id="cancel" title="Salir"><svg-icon type="mdi" :path="mdiClose" /></button>
    <form id="worker-form">
			<div>
				<label for="name">Nombre *</label>
				<input v-model="name" id="name" placeholder="Jhon Doe" />
			</div>
			<div>
				<label id="dni">N° de documento *</label>
				<input v-model="dni" id="dni" type="text" placeholder="123456789">
			</div>
			<div>
				<label for="phone">Telefono *</label>
				<input v-model="phone" id="phone" placeholder="341-012345" /> 
			</div>
			<div>
				<label for="salary">Paga *</label>
				<input v-model="salary" id="salary" placeholder="0.0" /> 
			</div>
			<div>
				<label for="address">Dirección</label>
				<input v-model="address" id="address" placeholder="742 Evergreen Terrace" /> 
			</div>
    </form>
		<button id="confirm" title="Añadir cuenta" @click="addWorker" type="button"><svg-icon type="mdi" :path="mdiCheck" /></button>
		</div>
		</div>
</template>

<script>
import { ref, onMounted } from 'vue';                
import { invoke } from "@tauri-apps/api/core";
import VueSelect from "vue3-select-component";
import SvgIcon from "@jamescoyle/vue-icon";
import { mdiClose, mdiCheck } from "@mdi/js"


export default {
	name: 'CreateWorker',
	components: {
		VueSelect,
		SvgIcon,
	},
	setup(props, { emit }) {
		const name = ref(null);
		const dni = ref(null);
		const phone = ref(null);
		const salary = ref(null);
		const address = ref(null);
		
		const toggleCustomer = () => {
			if (name.value == null || dni.value == null) {emit('destroy'); return 0}
			const userConfirmed = confirm("¿Seguro de cerrar? Los cambios no se guardaran")
			if (!userConfirmed) {return 0}
			emit('destroy');
		}

		const addWorker = async() => {
			if (address.value == "") {address.value = "none"}
			let log = await invoke('create_worker', {'name': name.value, 'dni': dni.value,
			'phone': phone.value, 'salary': parseFloat(salary.value), 'address': address.value})
			alert(log)

			emit('refresh-workers');
			emit('destroy');
		}

		return {
			name,
			dni,
			phone,
			salary,
			address,
			// Functions
			toggleCustomer,
			addWorker,
			// Icons
			mdiClose,	mdiCheck
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
#create-worker {
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
#create-worker > button {
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
#worker-form {
	height: 90%;
  width: 90%;
	display: grid;
	grid-template-columns: 40% 40%; 
	column-gap: 20%;
  align-items: center;
	overflow-y: scroll;
	overflow-x: scroll;	
}
#worker-form input {
	background: #333;
	border: none;
	border-radius: .4rem;
	padding: 6px 0.5rem;
	font-size: 18px;
	font-weight: 400;
}
#worker-form > * {
	display: flex;
	flex-direction: column;
}
#worker-form input::placeholder {
	color: #52525b;
}
#worker-form input:focus {
	outline: 2px solid #777;
}
/* --  Custom Select  -- */
:deep(.vue-select) {
  --vs-input-bg: #333;
  --vs-border: 1px solid #eee0;
  --vs-border-radius: .4rem;
	--vs-input-outline: #777;
}
</style>
