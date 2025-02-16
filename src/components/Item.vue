<template>
	<div id="card" class="closed">
		<button id="closeBtn" @click="toggleCard($event)"><svg-icon type="mdi" :path="mdiArrowExpandDown"/></button>
		<div id="header">
			<p title="ID"><svg-icon type="mdi" :path="mdiIdentifier"/>{{ data.id }}</p>
			<p title="Nombre"><svg-icon type="mdi" :path="mdiInformationVariant"/>{{ data.name }}</p>
			<p title="Precio"><svg-icon type="mdi" :path="mdiCurrencyUsd"/>{{ data.price }}</p>
			<p title="Tipo"><svg-icon type="mdi" :path="mdiHelp"/>{{ data.tipo }}</p>
			<p v-if="data.tipo == 'product'" title="Fabricante"><svg-icon type="mdi" :path="mdiDomain"/>{{ data.manufacturer }}</p>
			<p v-if="data.tipo == 'product'" title="Modelo"><svg-icon type="mdi" :path="mdiInformation"/>{{ data.model }}</p>
			<p v-if="data.tipo == 'product'" title="Proveedor"><svg-icon type="mdi" :path="mdiAccountTag"/>{{ data.supplier}}</p>
			<p v-if="data.tipo == 'product'" title="Stock"><svg-icon type="mdi" :path="mdiFormatListBulleted"/>{{ data.stock }}</p>
		</div>
		<button title="Eliminar" id="checkBtn" @click="editItem(data)"><svg-icon type="mdi" :path="mdiPencil"/></button>
	</div>
</template>

<script>
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import SvgIcon from '@jamescoyle/vue-icon';
import { mdiArrowExpandDown, mdiIdentifier, mdiInformationVariant, mdiCurrencyUsd, mdiHelp,
	mdiDomain, mdiInformation, mdiAccountTag, mdiFormatListBulleted, mdiPencil} from '@mdi/js';

export default {
	props: {
		data: {
			type: Object,
			required: true
		}, 
	},
	components: {
		SvgIcon,
	},
	setup(props, { emit }) {
		const toggleCard = async(e) => {
			let cardParent = e.target.parentElement;
			while (cardParent.tagName.toLowerCase() != 'div') {
				cardParent = cardParent.parentElement
			}
			cardParent.classList.toggle('closed');
		};

		const editItem = async(data) => {
			emit("select-item", data)
			emit("edit")
		}

		return {
			editItem,
			toggleCard,
			// Icons
			mdiArrowExpandDown, mdiIdentifier, mdiInformationVariant, mdiCurrencyUsd, mdiHelp,
			mdiDomain, mdiInformation, mdiAccountTag, mdiFormatListBulleted, mdiPencil
		}
	},
};

</script>

<style scoped>
#card {
	margin: 2rem 0;
	position: relative;
	background: #333;
	display: flex;
	flex-direction: column;
	width: 90%;
	height: 260px;
	justify-content: start;
	align-items: center;
	align-self: center;
	border-radius: .5rem;
	overflow-x: hidden;
	overflow-y: scroll; 
	transition: height .7s ease;
}
#card.closed {
	height: 3.2rem;
	overflow-y: hidden; 
}
#closeBtn {
	display: flex;
	align-items: center;
	justify-content: center;
	height: 2.2rem;
	width: 2.2rem;
	position: absolute;
	right: .6rem;
	margin-top: .5rem;
	border: none;
	cursor: pointer;
	border-radius: .5rem;
	transition: transform .3s;
	transform: rotateZ(180deg);
	z-index: 2;
}
#card.closed > #closeBtn {
	transform: rotateZ(0deg);
}
#closeBtn:focus {
	all: none
}
#header {
	display: grid;
	grid-column-gap: 2rem;
	grid-template-columns: 1fr 1fr 1fr;
	justify-items: center;
	align-items: center;
}
#header > p {
	cursor: default;
	display: flex;
	justify-items: center;
	align-items: center;
}
table {
	margin: 2rem 1rem;
}
td {
	width: auto;
	border-radius: .2rem;
	padding: 3px 6px;
}
tr:first-child {
	background-color: #7f7f7f;
}
tr {
	background-color: #555;
}
tr:nth-child(even) {
	background-color: #232323;
}
#checkBtn {
	display: flex;
	align-items: center;
	justify-content: center;
	height: 2.2rem;
	width: 2.2rem;
	position: absolute;
	right: .6rem;
	bottom: .5rem;
	border: none;
	cursor: pointer;
	border-radius: .5rem;
}
</style>
