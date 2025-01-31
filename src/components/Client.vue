<template>
	<div id="card" class="closed">
		<button id="closeBtn" @click="toggleCard($event)"><svg-icon type="mdi" :path="mdiArrowExpandDown"/></button>
		<div id="header">
			<p title="Nombre"><svg-icon type="mdi" :path="mdiAccount"/>{{ data.name }}</p>
			<p title="Telefono"><svg-icon type="mdi" :path="mdiPhone"/>{{ data. phone }}</p>
			<p title="CUIL/CUIT"><svg-icon type="mdi" :path="mdiCardAccountDetails"/>{{ data.cuil }}</p>
			<p title="DNI"><svg-icon type="mdi" :path="mdiAccountCreditCard"/>{{ data.dni }}</p>
			<p title="Tipo de cuenta"><svg-icon type="mdi" :path="mdiAccountQuestion"/>{{ data.tipo }}</p>
		</div>
		<button title="Editar cliente" id="checkBtn" @click="editCustomer(data)"><svg-icon type="mdi" :path="mdiPencil"/></button>
	</div>
</template>

<script>
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import SvgIcon from '@jamescoyle/vue-icon';
import { mdiArrowExpandDown, mdiAccount, mdiPhone, mdiCardAccountDetails, mdiAccountCreditCard,
	mdiAccountQuestion, mdiPencil} from '@mdi/js';

export default {
	name: "Client",
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

		const editCustomer = async(data) => {
			// obtain vehicles
			let log  = await invoke('obtain_vehicles')
			let domains = log.filter((vehicle) => vehicle.owner == data.name)
				.map((vehicle) => vehicle.domain)
			data.vehicles = domains
			emit("select-customer", data)
			emit("edit")
		}

		return {
			toggleCard,
			editCustomer,
			// Icons
			mdiArrowExpandDown, mdiAccount, mdiPhone, mdiCardAccountDetails, mdiAccountCreditCard,
			mdiAccountQuestion, mdiPencil
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
	height: 200px;
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
