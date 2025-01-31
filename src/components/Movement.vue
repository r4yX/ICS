<template>
	<div id="card" class="closed">
		<button id="closeBtn" @click="toggleCard($event)"><svg-icon type="mdi" :path="mdiArrowExpandDown"/></button>
		<div id="header">
			<p title="Fecha"><svg-icon type="mdi" :path="mdiCalendar"/>{{ data.date }}</p>
			<p title="Motivo"><svg-icon type="mdi" :path="mdiAccount"/>{{ data.name }}</p>
			<p title="Tipo"><svg-icon type="mdi" :path="mdiHelp"/>{{ data.tipo }}</p>
			<p title="Monto"><svg-icon type="mdi" :path="mdiCurrencyUsd"/>{{ data.amount }}</p>
		</div>
	</div>
</template>

<script>
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import SvgIcon from '@jamescoyle/vue-icon';
import { mdiArrowExpandDown, mdiCalendar, mdiHelp, mdiCurrencyUsd, mdiAccount } from '@mdi/js';

export default {
	name: "Movement",
	props: {
		data: {
			type: Object,
			required: true
		}, 
	},
	components: {
		SvgIcon,
	},
	setup() {
		const toggleCard = async(e) => {
			let cardParent = e.target.parentElement;
			while (cardParent.tagName.toLowerCase() != 'div') {
				cardParent = cardParent.parentElement
			}
			cardParent.classList.toggle('closed');
		};

		return {
			toggleCard,
			// Icons
			mdiArrowExpandDown, mdiCalendar, mdiHelp, mdiCurrencyUsd, mdiAccount
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
	grid-template-columns: 1fr 1fr;
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
