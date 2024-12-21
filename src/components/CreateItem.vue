<template>
  <div id="blur">
		<div id="create-item">
			<button @click="toggleItem()" id="cancel" title="Cancelar"><svg-icon type="mdi" :path="mdiClose" /></button>
    <form id="item-form">
				<div class="cols">
					<label for="name">Nombre *</label>
					<input id="name" placeholder="Cambio de Aceite" />
				</div>
				<div class="cols">
					<label id="idn">ID</label>
					<input id="idn" type="text" placeholder="0000 or F0000">
				</div>
				<div class="cols">
					<label for="price">Precio *</label>
					<input id="price" placeholder="0.000,00" />
				</div>
				<div class="cols">
					<label for="tipo">Tipo de Item *</label>
					<VueSelect class="vue-select"
						id="tipo"
						v-model="tipo"
						:options="[
							{ label: 'Producto', value: 'product' },
							{ label: 'Servicio', value: 'service' },
						]" placeholder="Producto"/>
				</div>
				<div :class="{cols:true, hidden: tipo != 'product'}">
					<label for="brand">Marca</label>
					<input id="brand" placeholder="Denso" />
				</div>
				<div :class="{cols:true, hidden: tipo != 'product'}">
					<label id="model">Modelo</label>
					<input id="model" type="text" placeholder="Inyector Common Rail">
				</div>
				<div :class="{cols:true, hidden: tipo != 'product'}">
					<label id="supplier">Proveedor</label>
					<input id="supplier" type="text" placeholder="Rosario Filtros">
				</div>
				<div :class="{cols:true, hidden: tipo != 'product'}">
					<label id="stock">Stock</label>
					<input id="stock" type="text" placeholder="99">
				</div>
    </form>
		<button id="confirm" title="Añadir Item" @click="addItem" type="button"><svg-icon type="mdi" :path="mdiCheck" /></button>
		</div>
		</div>
</template>

<script>
import { ref } from 'vue';                
import VueSelect from "vue3-select-component";
import SvgIcon from "@jamescoyle/vue-icon";
import { mdiClose, mdiCheck } from "@mdi/js"

const name = ref("");
const idn = ref("");
const price = ref("");
const tipo = ref("");
const brand = ref("");
const model = ref("");
const supplier = ref("");
const stock = ref(0);

export default {
	methods: {
		toggleItem() {
			const userConfirmed = confirm("¿Seguro de cerrar? Los cambios no se guardaran")
			if (!userConfirmed) {return 0}
			this.$emit('destroy');
		}
	},
	name: 'CreateItem',
	components: {
		VueSelect,
		SvgIcon,
	},
	setup() {
    const isItem = ref(true);

		const addItem = () => {}
    // -- Return
		return {
			isItem,
			// Input vars
			name,
			idn,
			price,
			tipo,
			brand,
			model,
			supplier,
			stock,
			// Functions
			addItem,
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
#create-item {
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
#create-item > button {
  position: absolute;
  display: flex;
  cursor: pointer;
  height: 30px;
  width: 30px;
  outline: none;
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
#item-form {
	height: 90%;
	margin: auto;
  width: 100%;
	display: grid;
	grid-template-columns: 50% 50%;
	grid-template-rows: repeat(4, 1fr);
	justify-items: center;
	overflow-y: scroll;
	overflow-x: scroll;
}
#item-form input {
	background: #333;
	border: none;
	border-radius: .4rem;
	padding: 6px 0.5rem;
	font-size: 18px;
	font-weight: 400;
}
#item-form input::placeholder {
	color: #52525b;
}
#item-form input:focus {
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
.cols.hidden {
	display: none;
}
/* --  Custom Select  -- */
:deep(.vue-select) {
  --vs-input-bg: #333;
  --vs-border: 1px solid #eee0;
  --vs-border-radius: .4rem;
	--vs-input-outline: #777;
}
</style>
