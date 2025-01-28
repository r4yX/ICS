<template>
	<div id="card" class="closed">
		<button id="closeBtn" @click="toggleCard($event)"><svg-icon type="mdi" :path="mdiArrowExpandDown"/></button>
		<div id="header">
			<p title="Patente"><svg-icon type="mdi" :path="mdiCar"/>{{ data.domain }}</p>
			<p title="Fabricante"><svg-icon type="mdi" :path="mdiCarCog"/>{{ data.maker }}</p>
			<p title="Modelo"><svg-icon type="mdi" :path="mdiCarInfo"/>{{ data.model }}</p>
			<p title="Tipo"><svg-icon type="mdi" :path="mdiCarSide"/>{{ data.tipo }}</p>
			<p title="Color"><svg-icon type="mdi" :path="mdiBrush"/>{{ data.colour }}</p>
			<p title="Año"><svg-icon type="mdi" :path="mdiCalendarBlank"/>{{ data.year }}</p>
			<p title="Dueño"><svg-icon type="mdi" :path="mdiAccount"/>{{ data.owner }}</p>
		</div>
		<button v-if="data.owner == 'none' " title="Añadir dueño" id="checkBtn" @click="addOwner()"><svg-icon type="mdi" :path="mdiAccountPlus"/></button>
	</div>
</template>

<script>
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import SvgIcon from '@jamescoyle/vue-icon';
import { mdiArrowExpandDown, mdiCar, mdiCarCog, mdiCarInfo, mdiCarSide, mdiBrush,
	mdiCalendarBlank, mdiAccount, mdiAccountPlus} from '@mdi/js';

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
	setup(props) {
		const toggleCard = async(e) => {
			let cardParent = e.target.parentElement;
			while (cardParent.tagName.toLowerCase() != 'div') {
				cardParent = cardParent.parentElement
			}
			cardParent.classList.toggle('closed');
		};

		const addOwner = async() => {
			console.log(props.data.domain)
		}

		return {
			toggleCard,
			addOwner,
			// Icons
			mdiArrowExpandDown, mdiCar, mdiCarCog, mdiCarInfo, mdiCarSide, mdiBrush,
			mdiCalendarBlank, mdiAccount, mdiAccountPlus
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
	height: 240px;
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
