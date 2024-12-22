<template>
	<div id="card">
		<p title="Nombre"><svg-icon type="mdi" :path="mdiAccount"/>Jhon Doe</p>
		<p title="DNI"><svg-icon type="mdi" :path="mdiCardAccountDetails"/>12345678</p>
		<p title="Telefono"><svg-icon type="mdi" :path="mdiPhone"/>+54-341-000000</p>
		<p title="Domicilio"><svg-icon type="mdi" :path="mdiHome"/>742 Evergreen Terrace</p>
		<p title="Sueldo"><svg-icon type="mdi" :path="mdiCurrencyUsd"/>200.000</p>
		<button @click="showPays()">Pagos</button>
	</div>
</template>

<script>
import { ref } from "vue";
import SvgIcon from '@jamescoyle/vue-icon';
import { mdiAccount, mdiCardAccountDetails, mdiPhone, mdiHome, mdiPencil, mdiCurrencyUsd } from '@mdi/js';

export default {
	components: {
		SvgIcon,
	},
	setup() {
		const toggleCard = (e) => {
			let cardParent = e.target.parentElement;
			while (cardParent.tagName.toLowerCase() != 'div') {
				cardParent = cardParent.parentElement
			}
			cardParent.classList.toggle('closed');
		};
		const showPays = () => {}

		return {
			showPays,
			toggleCard,
			// Icons
			mdiAccount,
			mdiCardAccountDetails,
			mdiPhone,
			mdiHome,
			mdiPencil,
			mdiCurrencyUsd,
		}
	},
};

</script>

<style scoped>
#card {
	position: relative;
	display: grid;
	margin: 0 auto;
	grid-column-gap: 8rem;
	grid-template-columns: 1fr 1fr;
	justify-items: center;
	align-items: center;
	width: 90%;
	height: 360px;
	border-radius: .5rem;
	background: #333;
	overflow-x: hidden;
	overflow-y: scroll; 
	transition: height .7s ease;
}
#card > *:not(button) {
	display: flex;
	align-items: center;
	justify-content: space-between;
	cursor: default;
}
#card > button {
	font-size: 18px;
	padding: .5rem 1rem;
	outline: none;
	color: white;
	background: #333;
	border: 1px solid #999;
	border-radius: .4rem;
	cursor: pointer;
	transition: background .2s;
}
#card > button:hover {
	background: #555;
}
</style>
